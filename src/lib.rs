mod utils;

use std::{
    collections::VecDeque,
    fs, future,
    sync::{Arc, Mutex},
    usize,
};
use tinyaudio::run_output_device;
use tinyaudio::BaseAudioOutputDevice;
use tinyaudio::OutputDeviceParameters;

use wasm_bindgen::prelude::*;

mod spu;

use spu::{AudioBitDepth, Nds, Spu};

mod drums;
mod ins;
mod split;

use drums::drum_instructions;
use ins::instrument_instructions;
use split::*;

static mut DEVICE: Option<Box<dyn BaseAudioOutputDevice>> = None;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wahdio!");
}

struct Timing {
    tiny_tick: usize,
    loop_times: usize,
}

fn play_stuff(
    spu: Arc<Mutex<Spu>>,
    multiplier: usize,
    timing: &mut Timing,
    channel_manager: &mut ChannelManager,
    queued_notes: &[QueuedNote],
    queued_drums: &[QueuedDrum],
    instruments: &[Instrument],
    rhythm_sections: &[RhythmSection; RHYTHM_SECTION_COUNT],
    previous_notes: &mut [Option<u8>; 4],
) -> Vec<i16> {
    let mut output = Vec::new();
    let mut spu = spu.lock().unwrap();

    for _ in 0..multiplier {
        for _ in 0..1024 {
            if timing.tiny_tick == timing.loop_times * 32 * NOTE_RATE {
                for t in 0..16 {
                    channel_manager.release_tracks(t, timing.tiny_tick);
                }
            }
            for note in queued_notes
                .iter()
                .filter(|note| note.time as usize * NOTE_RATE == timing.tiny_tick)
            {
                match &instruments[note.instrument as usize].instructions {
                    InstrumentInstructions::Adsr(adsr) => {
                        channel_manager.release_tracks(note.track, timing.tiny_tick);

                        add_adsr_to_channel(
                            channel_manager,
                            &mut spu,
                            &note,
                            note.note as u32,
                            24,
                            &instruments,
                            &adsr,
                            VecDeque::new(),
                            0,
                            previous_notes,
                        );
                    }
                    InstrumentInstructions::Dual(adsr_pair) => {
                        channel_manager.release_tracks(note.track, timing.tiny_tick);
                        for adsr in adsr_pair.iter() {
                            add_adsr_to_channel(
                                channel_manager,
                                &mut spu,
                                &note,
                                note.note as u32,
                                24,
                                &instruments,
                                &adsr,
                                VecDeque::new(),
                                0,
                                previous_notes,
                            );
                        }

                        println!("CHANNELS: {:?}", channel_manager);
                    }
                    InstrumentInstructions::Ranged(ranged_adsr) => {
                        channel_manager.release_tracks(note.track, timing.tiny_tick);
                        let adsr = ranged_adsr
                            .iter()
                            .find(|r| note.note >= r.low && note.note <= r.high)
                            .unwrap();

                        let note_offset = (note.note - adsr.low) as u32;

                        add_adsr_to_channel(
                            channel_manager,
                            &mut spu,
                            &note,
                            note_offset,
                            (adsr.high - adsr.low) as u32,
                            &instruments,
                            &adsr.adsr,
                            VecDeque::new(),
                            0,
                            previous_notes,
                        );
                    }
                    InstrumentInstructions::TimedMultiple(timed_adsr) => {
                        channel_manager.release_tracks(note.track, timing.tiny_tick);

                        let (_, adsr) = timed_adsr.iter().find(|(time, _)| *time == 0).unwrap();

                        let remaining: VecDeque<_> = timed_adsr
                            .iter()
                            .filter(|(time, _)| *time != 0)
                            .cloned()
                            .collect();

                        let channel_id = add_adsr_to_channel(
                            channel_manager,
                            &mut spu,
                            &note,
                            note.note as u32,
                            24,
                            &instruments,
                            adsr,
                            remaining,
                            0,
                            previous_notes,
                        );
                    }
                }
            }

            // TEST NOTE

            let drums_now: Vec<_> = queued_drums
                .iter()
                .filter(|drum| drum.time as usize * NOTE_RATE == timing.tiny_tick)
                .cloned()
                .collect();

            fn play_drum_on_spu(
                spu: &mut Spu,
                channel_id: usize,
                sample: &DrumSample,
                pan: u8,
                initial_volume: u32,
                volume_multiplier: f32,
            ) {
                {
                    spu.set_channel_pan(channel_id, pan);
                    spu.set_adjusted_channel_volume(channel_id, initial_volume, volume_multiplier);
                    spu.set_channel_timer_reload(channel_id, sample.timer_reload as u32);
                    spu.set_channel_loop_pos(channel_id, sample.loop_pos);
                    spu.set_channel_length(channel_id, sample.length);
                    spu.set_channel_src_address(channel_id, sample.src_address);
                    spu.channel_play_note(channel_id, sample.is_repeating);
                }
            }

            for drum in drums_now.into_iter() {
                let pan_addition = drum.pan_addition;
                let volume_multiplier = drum.volume_multiplier;
                let drum_set = drum.section;
                let adjust_pan = |pan: u8| -> u8 {
                    (pan as i32 + pan_addition).max(0).min(u8::MAX as i32) as u8
                };

                channel_manager.release_tracks(drum.pretend_track, timing.tiny_tick);
                let channel_id = match &rhythm_sections[drum_set].instructions[drum.id] {
                    DrumInstructions::Noise {
                        sample,
                        attack,
                        decay,
                        sustain,
                        release,
                    } => channel_manager.request_channel_noise(drum.pretend_track),
                    _ => channel_manager.request_channel_pcm(drum.pretend_track),
                }
                .unwrap();

                match &rhythm_sections[drum_set].instructions[drum.id] {
                    DrumInstructions::Dsr {
                        sample,
                        decay,
                        sustain,
                        release,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let this_sustain = sustain.volume;
                        let decay_constant = (1.0 / decay.duration as f32)
                            * (this_sustain as f32 / initial_volume as f32).ln();

                        let envelope = Some(Envelope {
                            attack: None,
                            initial_volume,
                            decay: Some(DecayEnvelope::Exponential {
                                constant: decay_constant,
                                duration: decay.duration,
                            }),
                            sustain: Some(SustainEnvelope {
                                volume: this_sustain,
                                duration: sustain.duration,
                            }),
                            release: Some(release.clone()),
                        });

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            Vec::new(),
                            Vec::new(),
                        );

                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                    DrumInstructions::Sr {
                        sample,
                        sustain,
                        release,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let envelope = Some(Envelope {
                            attack: None,
                            initial_volume,
                            decay: None,
                            sustain: Some(SustainEnvelope {
                                volume: sustain.volume,
                                duration: sustain.duration,
                            }),
                            release: Some(release.clone()),
                        });

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            Vec::new(),
                            Vec::new(),
                        );

                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                    DrumInstructions::Decay {
                        sample,
                        decay,
                        final_volume,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let this_sustain = (*final_volume as f32).max(0.1);
                        let decay_constant = (1.0 / decay.duration as f32)
                            * (this_sustain as f32 / initial_volume as f32).ln();

                        let decay = match decay {
                            DecayInstructions {
                                duration,
                                kind: DecayKind::Exponential,
                            } => DecayEnvelope::Exponential {
                                constant: decay_constant,
                                duration: *duration,
                            },
                            DecayInstructions {
                                duration,
                                kind: DecayKind::Linear,
                            } => DecayEnvelope::Linear {
                                duration: *duration,
                            },
                        };

                        let envelope = Some(Envelope {
                            attack: None,
                            initial_volume,
                            decay: Some(decay),
                            sustain: None,
                            release: None,
                        });

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            Vec::new(),
                            Vec::new(),
                        );

                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                    DrumInstructions::ExactVolumeAdjust {
                        sample,
                        adjustments,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let envelope = Some(Envelope {
                            attack: Some(AttackEnvelope::Exact {
                                adjustments: adjustments.clone(),
                            }),
                            initial_volume,
                            decay: None,
                            sustain: None,
                            release: None,
                        });

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            Vec::new(),
                            Vec::new(),
                        );

                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                    DrumInstructions::PitchThenRelease {
                        sample,
                        sustain,
                        release,
                        adjustments,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let envelope = Some(Envelope {
                            attack: None,
                            initial_volume,
                            decay: None,
                            sustain: Some(SustainEnvelope {
                                volume: sustain.volume,
                                duration: sustain.duration,
                            }),
                            release: Some(release.clone()),
                        });

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            adjustments.clone(),
                            Vec::new(),
                        );

                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                    DrumInstructions::DsrPitchAdjust {
                        sample,
                        decay,
                        sustain,
                        release,
                        adjustments,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let this_sustain = sustain.volume;
                        let decay_constant = (1.0 / decay.duration as f32)
                            * (this_sustain as f32 / initial_volume as f32).ln();

                        let decay = match decay {
                            DecayInstructions {
                                duration,
                                kind: DecayKind::Exponential,
                            } => DecayEnvelope::Exponential {
                                constant: decay_constant,
                                duration: *duration,
                            },
                            DecayInstructions {
                                duration,
                                kind: DecayKind::Linear,
                            } => DecayEnvelope::Linear {
                                duration: *duration,
                            },
                        };

                        let envelope = Some(Envelope {
                            attack: None,
                            initial_volume,
                            decay: Some(decay),
                            sustain: Some(SustainEnvelope {
                                volume: sustain.volume,
                                duration: sustain.duration,
                            }),
                            release: Some(release.clone()),
                        });

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            adjustments.clone(),
                            Vec::new(),
                        );

                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                    DrumInstructions::RepeatOnce {
                        sample,
                        repeat_time,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let envelope = None;
                        let queued_samples = vec![(sample.clone(), *repeat_time)];
                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            Vec::new(),
                            queued_samples,
                        );
                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                    DrumInstructions::Multiple { samples } => {
                        //assert_eq!(samples[0].0.time, 0);
                        //let sample = samples[0].0.sample.clone;

                        //let initial_volume = sample.volume;
                        //let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let envelope = None;
                        //repeat_sample = Some((sample.clone(), *repeat_time));
                        let mut queued_samples = Vec::new();
                        for sample in samples {
                            // TODO: sample.time amount ahead of original time
                            queued_samples.push((sample.sample.clone(), sample.time));
                        }
                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            samples[0].sample.volume,
                            Vec::new(),
                            queued_samples,
                        );
                    }
                    DrumInstructions::Noise {
                        sample,
                        attack,
                        decay,
                        sustain,
                        release,
                    } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let this_sustain = sustain.volume;
                        let decay_constant = (1.0 / decay.duration as f32)
                            * (this_sustain as f32 / initial_volume as f32).ln();

                        let attack = match attack.clone() {
                            AttackInstructions::Exact { adjustments } => {
                                AttackEnvelope::Exact { adjustments }
                            }
                            AttackInstructions::Linear { volume, duration } => {
                                AttackEnvelope::Linear { volume, duration }
                            }
                        };

                        let envelope = Some(Envelope {
                            attack: Some(attack),
                            initial_volume,
                            decay: Some(DecayEnvelope::Exponential {
                                constant: decay_constant,
                                duration: decay.duration,
                            }),
                            sustain: Some(SustainEnvelope {
                                volume: sustain.volume,
                                duration: sustain.duration,
                            }),
                            release: Some(release.clone()),
                        });

                        {
                            spu.set_channel_pan(channel_id, adjust_pan(sample.base_pan));
                            spu.set_adjusted_channel_volume(
                                channel_id,
                                initial_volume,
                                volume_multiplier,
                            );
                            spu.set_channel_timer_reload(channel_id, timer_reload as u32);
                            spu.channel_play_noise(channel_id);
                        }

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            Vec::new(),
                            Vec::new(),
                        );
                    }
                    DrumInstructions::Simple { sample } => {
                        let initial_volume = sample.volume;
                        let timer_reload = sample.timer_reload;
                        //let src_address = sample

                        let envelope = None;

                        channel_manager.allocate_drum(
                            drum.clone(),
                            envelope,
                            initial_volume,
                            Vec::new(),
                            Vec::new(),
                        );

                        play_drum_on_spu(
                            &mut spu,
                            channel_id,
                            sample,
                            adjust_pan(sample.base_pan),
                            initial_volume,
                            volume_multiplier,
                        );
                    }
                }
            }

            timing.tiny_tick += 1;

            let mut play_next = Vec::new();
            for (channel_id, channel) in channel_manager.channels.iter_mut().enumerate() {
                match channel {
                    Channel::Used {
                        sound,
                        envelope,
                        volume: channel_volume,
                        pitch_adjustments,
                        future_adsr,
                        start_tick,
                        queued_samples,
                    } => {
                        if (timing.tiny_tick - sound.time() as usize * NOTE_RATE) % 171 == 0 {
                            let mut should_be_freed = false;
                            let tick = ((timing.tiny_tick - sound.time() as usize * NOTE_RATE)
                                / 171) as u32
                                - *start_tick;
                            if let Some(adj) = pitch_adjustments
                                .iter()
                                .filter(|adj| adj.time <= tick)
                                .last()
                            {
                                println!("TIMER RELOAD: {}", adj.timer_reload);
                                spu.set_channel_timer_reload(channel_id, adj.timer_reload as u32);
                            }
                            for (sample, _) in
                                queued_samples.iter().filter(|(_, time)| *time == tick)
                            {
                                let initial_volume = sample.volume;
                                let timer_reload = sample.timer_reload;

                                {
                                    // TODO: NEEDED FOR DRUMS
                                    spu.set_adjusted_channel_pan(
                                        channel_id,
                                        sample.base_pan,
                                        sound.pan_addition(),
                                    );
                                    spu.set_adjusted_channel_volume(
                                        channel_id,
                                        initial_volume,
                                        sound.volume_multiplier(),
                                    );
                                    spu.set_channel_timer_reload(channel_id, timer_reload as u32);
                                    spu.set_channel_loop_pos(channel_id, sample.loop_pos);
                                    spu.set_channel_length(channel_id, sample.length);
                                    spu.set_channel_src_address(channel_id, sample.src_address);
                                    spu.channel_play_note(channel_id, sample.is_repeating);
                                }
                            }
                            match &envelope {
                                Some(Envelope {
                                    attack: Some(AttackEnvelope::Exact { adjustments }),
                                    initial_volume: _,
                                    decay: None,
                                    sustain: None,
                                    release: None,
                                }) => {
                                    if let Some(attack) =
                                        adjustments.iter().filter(|atk| atk.time == tick).last()
                                    {
                                        spu.set_adjusted_channel_volume(
                                            channel_id,
                                            attack.volume,
                                            sound.volume_multiplier(),
                                        );
                                    }
                                }
                                Some(Envelope {
                                    attack,
                                    initial_volume,
                                    decay,
                                    sustain,
                                    release,
                                }) => {
                                    let attack_volume = attack
                                        .as_ref()
                                        .map(|atk| atk.last_volume())
                                        .unwrap_or(*initial_volume);
                                    let attack_duration =
                                        attack.as_ref().map(|atk| atk.duration()).unwrap_or(0);
                                    let sustain = sustain.unwrap_or(SustainEnvelope {
                                        volume: 0,
                                        duration: 0,
                                    });
                                    let decay =
                                        decay.unwrap_or(DecayEnvelope::Linear { duration: 0 });
                                    let volume = if tick < attack_duration {
                                        match attack {
                                            Some(AttackEnvelope::Linear { volume, duration }) => {
                                                interp_val_until(
                                                    tick,
                                                    (*initial_volume, *volume),
                                                    *duration,
                                                )
                                                    as f32
                                            }
                                            Some(AttackEnvelope::Exact { adjustments }) => {
                                                if let Some(adj) = adjustments
                                                    .iter()
                                                    .filter(|atk| atk.time <= tick)
                                                    .last()
                                                {
                                                    adj.volume as f32
                                                } else {
                                                    *initial_volume as f32
                                                }
                                            }
                                            None => unreachable!(),
                                        }
                                    } else if tick - attack_duration < decay.duration() {
                                        //println!(
                                        //    "{}, {}, {}, {}",
                                        //    attack_volume, decay.constant, tick, attack_duration,
                                        //);
                                        let decay_tick = tick - attack_duration;
                                        match decay {
                                            DecayEnvelope::Exponential { constant, duration } => {
                                                attack_volume as f32
                                                    * (constant * decay_tick as f32).exp()
                                            }
                                            DecayEnvelope::Linear { duration } => interp_val_until(
                                                decay_tick,
                                                (attack_volume, sustain.volume),
                                                duration,
                                            )
                                                as f32,
                                        }
                                    } else if tick - attack_duration - decay.duration()
                                        <= sustain.duration
                                    {
                                        sustain.volume as f32
                                    } else {
                                        let release_tick = (tick
                                            - attack_duration
                                            - decay.duration()
                                            - sustain.duration)
                                            as f32;
                                        //1408_f32 * (-0.014 * tick as f32).exp()
                                        let new_volume = match &release {
                                            Some(ReleaseInstructions::Exponential { duration }) => {
                                                let release_constant = (1.0 / *duration as f32)
                                                    * (0.08 / sustain.volume as f32).ln();
                                                sustain.volume as f32
                                                    * (release_constant * release_tick).exp()
                                            }
                                            Some(ReleaseInstructions::Geometric { ratio })
                                            | Some(ReleaseInstructions::GeometricStopBlowing {
                                                ratio,
                                            }) => sustain.volume as f32 * ratio.powf(release_tick),
                                            Some(ReleaseInstructions::Basic) => {
                                                let release_constant = -0.17;
                                                sustain.volume as f32
                                                    * (release_constant * release_tick).exp()
                                            }
                                            Some(ReleaseInstructions::Static(value)) => {
                                                should_be_freed = true;
                                                *value as f32
                                            }
                                            None => 1.0,
                                        };
                                        if new_volume <= 1.5 {
                                            should_be_freed = true;
                                        }
                                        new_volume
                                    } as u32;

                                    println!("{} :: {}, {}", channel_id, tick, volume);

                                    *channel_volume =
                                        (volume as f32 * sound.volume_multiplier()) as u32;

                                    spu.set_adjusted_channel_volume(
                                        channel_id,
                                        volume as u32,
                                        sound.volume_multiplier(),
                                    );

                                    if should_be_freed {
                                        if let Some((next_time, adsr)) =
                                            future_adsr.front().cloned()
                                        {
                                            let tick = ((timing.tiny_tick
                                                - sound.time() as usize * NOTE_RATE)
                                                / 171)
                                                as u32;
                                            if tick >= next_time {
                                                play_next.push(channel_id);
                                            }
                                        } else {
                                            spu.set_adjusted_channel_volume(
                                                channel_id,
                                                0,
                                                sound.volume_multiplier(),
                                            );
                                            *channel = Channel::Open;
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                    Channel::Freeing {
                        sound,
                        volume,
                        initial_release_volume,
                        kill_tick,
                        release,
                    } => {
                        if (timing.tiny_tick - sound.time() as usize * NOTE_RATE) % 171 == 0 {
                            let tick = ((timing.tiny_tick - sound.time() as usize * NOTE_RATE)
                                / 171) as u32;
                            let release_constant = -0.17;
                            let release_tick = tick - *kill_tick;
                            match release {
                                Some(ReleaseInstructions::Basic)
                                | Some(ReleaseInstructions::Exponential { .. }) => {
                                    *volume = (*initial_release_volume as f32
                                        * (release_constant * release_tick as f32).exp())
                                        as u32;
                                    println!("FR {}, {}", tick, volume);
                                }
                                //Some(ReleaseInstructions::GeometricFastExit) => {
                                //    let initial_release_volume = *initial_release_volume / 2;
                                //    let release_constant = -0.6;
                                //    *volume = (initial_release_volume as f32
                                //        * (release_constant * release_tick as f32)
                                //            .exp())
                                //        as u32;
                                //    println!("FE {}, {}", tick, volume);
                                //}
                                Some(ReleaseInstructions::Geometric { ratio }) => {
                                    *volume = (*initial_release_volume as f32
                                        * ratio.powf(release_tick as f32))
                                        as u32;
                                    println!("FG {}, {}", tick, volume);
                                }
                                Some(ReleaseInstructions::GeometricStopBlowing { .. })
                                | Some(ReleaseInstructions::Static(_)) => *volume = 1,
                                None => {
                                    *volume = (*initial_release_volume as f32
                                        * 0.5_f32.powf(release_tick as f32))
                                        as u32;
                                    println!("FN {}, {}", tick, volume);
                                }
                            }

                            spu.set_adjusted_channel_volume(channel_id, *volume, 1.0);

                            if *volume <= 1 {
                                *channel = Channel::Open;
                            }
                        }
                    }
                    Channel::Open | Channel::Blocked => {}
                }
            }

            for i in play_next {
                let mut adsr = None;
                match &mut channel_manager.channels[i] {
                    Channel::Used {
                        sound: QueuedSound::Note(note),
                        future_adsr,
                        ..
                    } => {
                        adsr = Some((future_adsr.clone(), note.clone()));
                    }
                    _ => {}
                }
                if let Some((mut future_adsr, note)) = adsr {
                    channel_manager.release_tracks(note.track, timing.tiny_tick);

                    let (_, adsr) = future_adsr.pop_front().unwrap();

                    println!("{:?}", adsr);

                    let tick = ((timing.tiny_tick - note.time as usize * NOTE_RATE) / 171) as u32;

                    add_adsr_to_channel(
                        channel_manager,
                        &mut spu,
                        &note,
                        note.note as u32,
                        24,
                        &instruments,
                        &adsr,
                        future_adsr.clone(),
                        tick,
                        previous_notes,
                    );
                }
            }

            spu.mix(1);
        }

        spu.transfer_output();
        output.append(&mut spu.read_output(2048));
    }

    output
}

#[wasm_bindgen]
pub fn stop_sound() {
    unsafe {
        DEVICE = None;
    }
}

#[wasm_bindgen]
pub fn play_music(ram: &[u8], mio_data: &[u8]) {
    utils::set_panic_hook();
    //alert("Hello, wahdio!");
    let multiplier = 8;
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: SAMPLE_RATE,
        channel_sample_count: 1024 * multiplier,
    };

    //let mut ram = Vec::new();
    //for _ in 0..16384 {
    //    ram.push(0);
    //}
    //let mut dogadpcm: Vec<u8> = fs::read_to_string("dog4.txt")
    //    .expect("Failed to read file")
    //    .lines()
    //    .map(|line| line.parse::<u32>().unwrap())
    //    .flat_map(|val| {
    //        vec![
    //            val as u8,
    //            (val >> 8) as u8,
    //            (val >> 16) as u8,
    //            (val >> 24) as u8,
    //        ]
    //    })
    //    .collect();
    //ram.append(&mut dogadpcm);
    //ram.resize(4 * 1024 * 1024, 0);

    assert_eq!(ram.len(), 4 * 1024 * 1024);

    let nds = Arc::new(Mutex::new(Nds::new(ram.to_vec())));

    let mut spu = Spu::new(nds, AudioBitDepth::_16bit);

    //1, 0, 0 Set Cnt: Volume: 128, VolumeShift: 4, Pan: 0, KeyOn: 0
    spu.write32(67109904, 671088767);
    //1: Set TimerReload: 65024
    //0: Set Capture TimerReload: 65024
    spu.write16(67109912, 65024);
    //1: Set LoopPos: 0
    spu.write16(67109914, 0);
    //1: Set Length: 2048
    spu.write32(67109916, 512);
    //1: Set SrcAddr: 35253536
    spu.write32(67109908, 35253536);

    //0: Set Capture Cnt: 0
    spu.write8(67110152, 0);
    //0: Set Capture DstAddr: 35253536
    spu.write32(67110160, 35253536);
    //0: Set Capture Length: 2048
    spu.write16(67110164, 512);

    //3, 0, 0 Set Cnt: Volume: 128, VolumeShift: 4, Pan: 128, KeyOn: 0
    spu.write32(67109936, 679411839);
    //3: Set TimerReload: 65024
    //1: Set Capture TimerReload: 65024
    spu.write16(67109944, 65024);
    //3: Set LoopPos: 0
    spu.write16(67109946, 0);
    //3: Set Length: 2048
    spu.write32(67109948, 512);
    //3: Set SrcAddr: 35255584
    spu.write32(67109940, 35255584);

    //1: Set Capture Cnt: 0
    spu.write8(67110153, 0);
    //1: Set Capture DstAddr: 35255584
    spu.write32(67110168, 35255584);
    //1: Set Capture Length: 2048
    spu.write16(67110172, 512);
    //_?: Set Cnt: 47487
    spu.write8(67110145, 185);

    //1, 0, 0 Set Cnt: Volume: 128, VolumeShift: 4, Pan: 0, KeyOn: 1
    spu.write8(67109907, 168);

    //3, 0, 0 Set Cnt: Volume: 128, VolumeShift: 4, Pan: 128, KeyOn: 1
    spu.write8(67109939, 168);

    //0: Set Capture Cnt: 128
    //1: Set Capture Cnt: 128
    spu.write16(67110152, 32896);

    //Set MasterVolume: 127
    spu.write8(67110144, 127);

    // =============================

    let mut write_events: Vec<(u32, u32, Write)> = Vec::new();

    {}

    // TODO: Convert lowest values to zeros?
    let recorded_base_tick = write_events.get(0).map(|w| w.0).unwrap_or_default();

    for (tick, _, _) in &mut write_events {
        *tick -= recorded_base_tick;
    }

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Adjust {
        Increase(f32),
        Decrease(f32),
        Stick { at: u32, time: u32 },
    }

    let mut previous_volume = 0;
    let mut previous_tick = 0;
    let mut adjustments = Vec::new();
    let mut adjustments2 = Vec::new();
    let mut raw = Vec::new();
    for (tick, address, write) in write_events.clone().into_iter() {
        match write {
            Write::u16(val) => {
                if let Some(channel_index) = spu.has_updated_channel_control16(address) {
                    let ctrl = (spu.channels[channel_index].control & 0xFFFF0000) | val as u32;

                    let mut volume = (ctrl & 0x7F) as u8;
                    if volume == 127 {
                        volume += 1;
                    }

                    let vol_shift: [u8; 4] = [4, 3, 2, 0];
                    let volume_shift = vol_shift[((ctrl >> 8) & 0x3) as usize];

                    let actual = (volume as u32) << (volume_shift as u32);

                    //println!(
                    //    "{} => {}, {}                     actual={}",
                    //    tick, volume, volume_shift, actual
                    //);

                    //if tick == 0 {
                    //    println!("actual: {}", actual);
                    //}
                    if tick == 1 && previous_tick == 0 {
                        println!("{}", previous_volume);
                    }
                    for _ in previous_tick..tick {
                        println!("{}", previous_volume);
                    }

                    //println!(
                    //    "Rate of change: {}",
                    //    (actual as f32 - previous_volume as f32)
                    //        / (tick as f32 - previous_tick as f32)
                    //);

                    let roc = actual as f32 - previous_volume as f32;

                    if roc > 0.0 && tick - previous_tick == 1 {
                        adjustments.push(Adjust::Increase(roc));
                    } else if roc < 0.0 && tick - previous_tick == 1 {
                        adjustments.push(Adjust::Decrease(roc))
                    } else {
                        adjustments.push(Adjust::Stick {
                            at: actual,
                            time: tick - previous_tick,
                        })
                    }
                    adjustments2
                        .push((tick - previous_tick, actual as i32 - previous_volume as i32));
                    raw.push(roc);

                    previous_volume = actual;
                    previous_tick = tick;
                }
            }
            _ => {}
        }
    }

    let mut zap = Vec::new();

    let mut means = Vec::new();
    for r in raw {
        if zap.is_empty() {
            zap.push(r);
        } else {
            let mean = zap.iter().copied().sum::<f32>() / zap.len() as f32;
            //println!("R {}, {} :: {}, {}", r, mean, mean / 2.0, mean * 2.0);
            if (r == 1.0 || r == 2.0) && r < mean / 2.0 && r > mean * 2.0 {
                zap.push(r);
            } else if r <= mean / 2.0 && r >= mean * 2.0 {
                zap.push(r);
            } else {
                means.push(mean);
                zap.clear();
                zap.push(r);
            }
        }
    }

    adjustments.dedup_by(|a, b| a == b);
    //adjustments2.dedup_by(|a, b| a == b);
    //println!("{:?}", adjustments);
    //println!("{:?}", adjustments2);
    //println!("skipped {:?}", skipped);
    //println!("{:?}", means);
    //
    //println!("LEN before: {}", q);
    //println!("LEN after: {}", adjustments.len());
    //println!("LEN after2: {}", adjustments2.len());
    //
    //println!("aa: {:?}", convert_vec(adjustments2));

    //std::process::exit(0);

    let spu = Arc::new(Mutex::new(spu));

    let mut glundex = 0;

    let mut hop = 770;
    let mut grump = 0;
    //63898
    //63990
    //64077

    //64309

    let instruments = instrument_instructions();

    let rhythm_sections: [RhythmSection; RHYTHM_SECTION_COUNT] = drum_instructions();

    let mut envelope_channel = 4;
    //let mut envelope = None;
    //let mut pitch_adjustments = None;
    //let mut repeat_sample = None;
    //let mut queued_samples = Vec::new();

    const TRACK_COUNT: usize = 4;
    const BASE_SONG_OFFSET: usize = 0xB961;
    const BASE_INSTRUMENT_OFFSET: usize = 0xBA6B;
    const BASE_VOLUME_OFFSET: usize = 0xBA61;
    const BASE_PAN_OFFSET: usize = 0xBA66;
    const BASE_DRUM_OFFSET: usize = 0xB9E1;
    const SIMULTANEOUS_DRUMS: usize = 4;
    // TODO: Add 32 for each offset off centre
    const PAN_MULTIPLIER: u8 = 32;

    //let mio_data = std::fs::read("C:/Users/Ross/Downloads/Battery/mios/Starwing.mio").unwrap();
    //let mio_data = std::fs::read(r"R:\data\mios\MUSIC PIANO2.mio").unwrap();
    //let mio_data = std::fs::read(r"C:\Users\Ross\Downloads\Battery\mios\Cake Defence.mio").unwrap();

    let drum_volume = mio_data[BASE_VOLUME_OFFSET + 4];
    let drum_pan = mio_data[BASE_PAN_OFFSET + 4];

    let mut drum_set = (mio_data[0xBA6F] & 0x7) as usize;

    //assert_eq!(drum_set, 0);

    let mut thing;
    const TRACK_LENGTH: usize = 32;
    let loop_times = 1;

    let mut queued_drums = Vec::new();

    'king_loop: for loop_iter in 0..loop_times {
        for i in 0..TRACK_LENGTH {
            for drum_index in 0..SIMULTANEOUS_DRUMS {
                let drum_used = mio_data[BASE_DRUM_OFFSET + i + drum_index * TRACK_LENGTH];
                if drum_used != 255 {
                    let pan_addition = match drum_pan {
                        0 => -64,
                        1 => -32,
                        2 => 0,
                        3 => 32,
                        4 => 64,
                        _ => unreachable!(),
                    };
                    let volume_multiplier = match drum_volume {
                        0 => 0.0,
                        1 => 0.25,
                        2 => 0.5,
                        3 => 2.0 / 3.0,
                        4 => 1.0,
                        _ => unreachable!(),
                    };
                    thing = 13 - drum_used as usize;
                    //queued_drums.push((i, thing));
                    queued_drums.push(QueuedDrum {
                        time: (i + loop_iter * TRACK_LENGTH) as u32,
                        volume_multiplier,
                        pan_addition,
                        section: drum_set,
                        id: thing,
                        pretend_track: 4 + drum_index as u8,
                    })
                    //break 'king_loop;
                }
            }
        }
    }

    //drum_set = 2;
    // TODO: UNSET!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    //queued_drums = vec![];

    let tmp_instrument = 17;
    let queued_notes = vec![
        QueuedNote {
            time: 0,
            instrument: tmp_instrument,
            note: 24,
            track: 0,
            pan_addition: 0,
            volume_multiplier: 1.0,
        },
        //QueuedNote {
        //    time: 1,
        //    instrument: tmp_instrument,
        //    note: 9,
        //    track: 0,
        //    pan_addition: 0,
        //    volume_multiplier: 1.0,
        //},
        QueuedNote {
            time: 2,
            instrument: tmp_instrument,
            note: 0,
            track: 0,
            pan_addition: 0,
            volume_multiplier: 1.0,
        },
    ];

    let mut queued_notes = vec![];

    for track_index in 0..TRACK_COUNT {
        let song_offset = BASE_SONG_OFFSET + track_index * TRACK_LENGTH;

        let instrument_used = mio_data[BASE_INSTRUMENT_OFFSET + track_index];
        let instrument_volume = mio_data[BASE_VOLUME_OFFSET + track_index];
        let instrument_pan = mio_data[BASE_PAN_OFFSET + track_index];

        'inking_loop: for loop_iter in 0..loop_times {
            for i in 0..TRACK_LENGTH {
                let note = mio_data[song_offset + i];
                if note != 255 {
                    let pan_addition = match instrument_pan {
                        0 => -64,
                        1 => -32,
                        2 => 0,
                        3 => 32,
                        4 => 64,
                        _ => unreachable!(),
                    };
                    let volume_multiplier = match instrument_volume {
                        0 => 0.0,
                        1 => 0.25,
                        2 => 0.5,
                        3 => 2.0 / 3.0,
                        4 => 1.0,
                        _ => unreachable!(),
                    };
                    queued_notes.push(QueuedNote {
                        time: (i + loop_iter * TRACK_LENGTH) as u32,
                        instrument: instrument_used as u32,
                        note: note,
                        track: track_index as u8,
                        volume_multiplier,
                        pan_addition,
                    });
                }
            }
        }
    }

    println!("QUEUED: {:?}", queued_notes);

    let mut channel_manager = ChannelManager {
        channels: [
            Channel::Open,
            Channel::Blocked,
            Channel::Open,
            Channel::Blocked,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
            Channel::Open,
        ],
    };

    let mut previous_notes: [Option<u8>; 4] = [None, None, None, None];

    let mut q_dex = 4;

    let mut timing = Timing {
        tiny_tick: 0,
        loop_times,
    };

    unsafe {
        DEVICE = run_output_device(params, {
            let spu = spu.clone();
            move |data| {
                let mut index = 0;
                let output = play_stuff(
                    spu.clone(),
                    multiplier,
                    &mut timing,
                    &mut channel_manager,
                    &queued_notes,
                    &queued_drums,
                    &instruments,
                    &&rhythm_sections,
                    &mut previous_notes,
                );

                let temp_volume_mul = 1.2;
                for samples_out in data.chunks_mut(params.channels_count) {
                    for sample in samples_out {
                        *sample = temp_volume_mul * output.get(index).copied().unwrap() as f32
                            / i16::MAX as f32;
                        //println!("index: {}", index);
                        //println!("{} sample: {}", index, output.get(index).copied().unwrap());

                        index += 1;
                    }
                }
            }
        })
        .ok();
    }

    //for _ in 0..300 {
    //    //spu.clone().lock().unwrap().transfer_output();
    //    std::thread::sleep(std::time::Duration::from_millis(16));
    //}

    //std::thread::sleep(std::time::Duration::from_secs(100));
}
