use std::collections::VecDeque;

use crate::spu::Spu;
use nanoserde::{DeBin, SerBin};

#[derive(DeBin, SerBin, Clone, Copy, Debug)]
pub enum Write {
    u8(u8),
    u16(u16),
    u32(u32),
}

pub const SAMPLE_RATE: usize = 32824;
pub const NOTE_RATE: usize = SAMPLE_RATE / 8;

pub fn interp_val(note: u32, (lowest, highest): (u32, u32)) -> u32 {
    let note = note as f32 / 24.0;
    let (lowest, highest) = (lowest as i32, highest as i32);
    let diff = (highest - lowest) as f32;
    (lowest as f32 + note * diff) as u32
}

pub fn interp_val_until(note: u32, (lowest, highest): (u32, u32), until: u32) -> u32 {
    if until == 0 {
        return (lowest + highest) / 2;
    }
    let note = note as f32 / until as f32;
    let (lowest, highest) = (lowest as i32, highest as i32);
    let diff = (highest - lowest) as f32;
    (lowest as f32 + note * diff) as u32
}

#[derive(Debug, Clone)]
pub enum AttackEnvelope {
    Linear {
        volume: u32,
        duration: u32,
    },
    Exact {
        adjustments: Vec<TimedVolumeAdjustment>,
    },
}

impl AttackEnvelope {
    pub fn last_volume(&self) -> u32 {
        match self {
            AttackEnvelope::Linear { volume, .. } => *volume,
            AttackEnvelope::Exact { adjustments } => {
                adjustments.iter().map(|adj| adj.volume).last().unwrap()
            }
        }
    }

    pub fn duration(&self) -> u32 {
        match self {
            AttackEnvelope::Linear { duration, .. } => *duration,
            AttackEnvelope::Exact { adjustments } => {
                adjustments.iter().map(|adj| adj.time).max().unwrap()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SustainEnvelope {
    pub volume: u32,
    pub duration: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum DecayEnvelope {
    Linear { duration: u32 },
    Exponential { constant: f32, duration: u32 },
}

impl DecayEnvelope {
    pub fn duration(&self) -> u32 {
        match self {
            DecayEnvelope::Linear { duration } => *duration,
            DecayEnvelope::Exponential { duration, .. } => *duration,
        }
    }
}

#[derive(Debug)]
pub struct Envelope {
    pub attack: Option<AttackEnvelope>,
    pub initial_volume: u32,
    pub decay: Option<DecayEnvelope>,
    pub sustain: Option<SustainEnvelope>,
    pub release: Option<ReleaseInstructions>,
}

#[derive(Debug, Clone)]
pub struct Sample {
    pub volume: (u32, u32),
    pub base_timer_reload: u16,
    pub loop_pos: usize,
    pub length: usize,
    pub src_address: usize,
    pub is_repeating: bool,
}

#[derive(Debug, Clone)]
pub struct ProgrammableSample {
    pub volume: (u32, u32),
    pub base_timer_reload: u16,
    pub table_index: u8,
}

// 1 asian drum and 3 8-bit drum timer reload adjusts over time, maybe more
// 1 8-bit drum repeats with reduced volume, maybe more
// 2 noise drum
#[derive(Debug, Clone)]
pub struct DrumSample {
    pub volume: u32,
    pub timer_reload: u16,
    pub loop_pos: usize,
    pub length: usize,
    pub src_address: usize,
    pub is_repeating: bool,
    pub base_pan: u8,
}

pub struct NoiseDrumSample {
    pub volume: u32,
    pub timer_reload: u16,
    pub base_pan: u8,
}

pub struct TimedDrumSample {
    pub time: u32,
    pub sample: DrumSample,
}

#[derive(Debug, Clone, Copy)]
pub struct TimedPitchAdjustment {
    pub time: u32,
    pub timer_reload: u16,
}

#[derive(Debug, Clone)]
pub struct TimedVolumeAdjustment {
    pub time: u32,
    pub volume: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct TimedRelativePitchAdjustment {
    pub time: u32,
    pub pitch_adjust: f32,
}

pub const DRUM_COUNT: usize = 14; //14;
pub const RHYTHM_SECTION_COUNT: usize = 8; //8;

pub enum DrumInstructions {
    Dsr {
        sample: DrumSample,
        decay: DecayInstructions,
        sustain: SustainInstructions,
        release: ReleaseInstructions,
    },
    Sr {
        sample: DrumSample,
        sustain: SustainInstructions,
        release: ReleaseInstructions,
    },
    Decay {
        sample: DrumSample,
        decay: DecayInstructions,
        final_volume: u32,
    },
    Simple {
        sample: DrumSample,
    },
    ExactVolumeAdjust {
        sample: DrumSample,
        adjustments: Vec<TimedVolumeAdjustment>,
    },
    DsrPitchAdjust {
        sample: DrumSample,
        decay: DecayInstructions,
        sustain: SustainInstructions,
        release: ReleaseInstructions,
        adjustments: Vec<TimedPitchAdjustment>,
    },
    PitchThenRelease {
        sample: DrumSample,
        adjustments: Vec<TimedPitchAdjustment>,
        sustain: SustainInstructions,
        release: ReleaseInstructions,
    },
    RepeatOnce {
        sample: DrumSample,
        repeat_time: u32,
    },
    Multiple {
        samples: Vec<TimedDrumSample>,
    },
    Noise {
        sample: NoiseDrumSample,
        attack: AttackInstructions,
        decay: DecayInstructions,
        sustain: SustainInstructions,
        release: ReleaseInstructions,
    },
    //Noise { sample: NoiseDrumSample },
    //Multiple { samples: Vec<TimedDrumSample>},
    //SimplePitchAdjust {sample: DrumSample, multiplier: f32, count: usize, rate: u16 },
    //ExactPitchAdjust {sample: DrumSample, adjustments: Vec<TimedPitchAdjustments> },
}

pub struct RhythmSection {
    pub name: String,
    pub instructions: [DrumInstructions; DRUM_COUNT],
}

impl RhythmSection {
    pub fn new(name: String, instructions: [DrumInstructions; DRUM_COUNT]) -> RhythmSection {
        RhythmSection { name, instructions }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ReleaseInstructions {
    Geometric { ratio: f32 },
    GeometricStopBlowing { ratio: f32 },
    Exponential { duration: u32 },
    Basic,
    Static(u32),
    //GeometricFastExit { ratio: f32 },
}

pub enum DecayKind {
    Exponential,
    Linear,
}

#[derive(Debug, Clone)]
pub enum AttackInstructions {
    Linear {
        volume: u32,
        duration: u32,
    },
    Exact {
        adjustments: Vec<TimedVolumeAdjustment>,
    },
    //ExactWithMagicPitch {
    //    adjustments: Vec<TimedVolumeAdjustment>,
    //},
}

pub struct DecayInstructions {
    pub duration: u32,
    pub kind: DecayKind,
}

impl DecayInstructions {
    pub fn linear(duration: u32) -> DecayInstructions {
        DecayInstructions {
            duration: duration,
            kind: DecayKind::Linear,
        }
    }

    pub fn exponential(duration: u32) -> DecayInstructions {
        DecayInstructions {
            duration: duration,
            kind: DecayKind::Exponential,
        }
    }
}

pub struct SustainInstructions {
    pub volume: u32,
    pub duration: u32,
}

#[derive(Debug, Clone)]
pub enum InstrumentSample {
    PCM16(Sample),
    PSG(ProgrammableSample),
}

impl InstrumentSample {
    pub fn volume(&self) -> (u32, u32) {
        match self {
            Self::PCM16(sample) => sample.volume,
            Self::PSG(sample) => sample.volume,
        }
    }

    pub fn base_timer_reload(&self) -> u16 {
        match self {
            Self::PCM16(sample) => sample.base_timer_reload,
            Self::PSG(sample) => sample.base_timer_reload,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstrumentTimedVolumeAdjustment {
    pub time: u32,
    pub volume: (u32, u32),
}

pub struct Instrument {
    pub name: String,
    pub instructions: InstrumentInstructions,
    pub pitch_adjustments: Vec<TimedRelativePitchAdjustment>,
}

#[derive(Debug, Clone)]
pub enum InstrumentAttack {
    Exact {
        adjustments: Vec<InstrumentTimedVolumeAdjustment>,
    },
    ExactWithMagicPitch {
        adjustments: Vec<InstrumentTimedVolumeAdjustment>,
    },
}

#[derive(Debug, Clone)]
pub enum InstrumentDecay {
    Exponential {
        duration: u32,
    },
    ExponentialWithVariableSustain {
        duration: u32,
        sustain_duration: (u32, u32),
    },
    ExponentialWithWobble {
        duration: u32,
        wobble_start: u32,
        max_wobble: f32,
        wave_duration: u32,
    },
    ExponentialRising {
        duration: u32,
        rise_start: u32,
        max_rise: f32,
    },
    Linear {
        duration: u32,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct InstrumentSustain {
    pub volume: (u32, u32),
    pub duration: u32,
}

#[derive(Debug, Clone)]
pub enum InstrumentRelease {
    Basic,
    Geometric { ratio: f32 },
    GeometricStopBlowing { ratio: f32 },
    //GeometricFastExit { ratio: f32 },
    Static((u32, u32)),
}

#[derive(Debug, Clone)]
pub struct Adsr {
    pub sample: InstrumentSample,
    pub attack: Option<InstrumentAttack>,
    pub decay: Option<InstrumentDecay>,
    pub sustain: Option<InstrumentSustain>,
    pub release: Option<InstrumentRelease>,
}

pub struct RangedAdsr {
    pub low: u8,
    pub high: u8,
    pub adsr: Adsr,
}

pub enum InstrumentInstructions {
    Adsr(Adsr),
    Dual([Adsr; 2]),
    Ranged(Vec<RangedAdsr>),
    TimedMultiple(Vec<(u32, Adsr)>),
}

#[derive(Debug, Clone)]
pub struct QueuedNote {
    pub time: u32,
    pub instrument: u32,
    pub note: u8,
    pub track: u8,
    pub pan_addition: i32,
    pub volume_multiplier: f32,
}

#[derive(Debug, Clone)]
pub struct QueuedDrum {
    pub time: u32,
    pub section: usize,
    pub id: usize,
    pub pretend_track: u8,
    pub pan_addition: i32,
    pub volume_multiplier: f32,
}

#[derive(Debug)]
pub enum Channel {
    Open,
    Used {
        sound: QueuedSound,
        envelope: Option<Envelope>,
        volume: u32,
        pitch_adjustments: Vec<TimedPitchAdjustment>,
        future_adsr: VecDeque<(u32, Adsr)>,
        queued_samples: Vec<(DrumSample, u32)>,
        start_tick: u32,
    },
    Freeing {
        sound: QueuedSound,
        initial_release_volume: u32,
        volume: u32,
        kill_tick: u32,
        release: Option<ReleaseInstructions>,
    },
    Blocked,
}

#[derive(Debug, Clone)]
pub enum QueuedSound {
    Note(QueuedNote),
    Drum(QueuedDrum),
}

impl QueuedSound {
    pub fn time(&self) -> u32 {
        match self {
            QueuedSound::Note(note) => note.time,
            QueuedSound::Drum(drum) => drum.time,
        }
    }

    pub fn track(&self) -> u8 {
        match self {
            QueuedSound::Note(note) => note.track,
            QueuedSound::Drum(drum) => drum.pretend_track,
        }
    }

    pub fn volume_multiplier(&self) -> f32 {
        match self {
            QueuedSound::Note(note) => note.volume_multiplier,
            QueuedSound::Drum(drum) => drum.volume_multiplier,
        }
    }

    pub fn pan_addition(&self) -> i32 {
        match self {
            QueuedSound::Note(note) => note.pan_addition,
            QueuedSound::Drum(drum) => drum.pan_addition,
        }
    }
}

#[derive(Debug)]
pub struct ChannelManager {
    pub channels: [Channel; 16],
}

impl ChannelManager {
    pub fn allocate_pcm(
        &mut self,
        note: QueuedNote,
        envelope: Option<Envelope>,
        volume: u32,
        pitch_adjustments: Vec<TimedPitchAdjustment>,
        future_adsr: VecDeque<(u32, Adsr)>,
        start_tick: u32,
    ) {
        let channel_id = self.request_channel_pcm(note.track).unwrap();

        self.channels[channel_id] = Channel::Used {
            sound: QueuedSound::Note(note),
            envelope: envelope,
            volume,
            pitch_adjustments,
            future_adsr,
            queued_samples: Vec::new(),
            start_tick,
        }
    }

    pub fn allocate_drum(
        &mut self,
        drum: QueuedDrum,
        envelope: Option<Envelope>,
        volume: u32,
        pitch_adjustments: Vec<TimedPitchAdjustment>,
        queued_samples: Vec<(DrumSample, u32)>,
    ) {
        let channel_id = self.request_channel_pcm(drum.pretend_track).unwrap();

        self.channels[channel_id] = Channel::Used {
            sound: QueuedSound::Drum(drum),
            envelope: envelope,
            volume,
            pitch_adjustments,
            future_adsr: VecDeque::new(),
            queued_samples: Vec::new(),
            start_tick: 0,
        }
    }

    pub fn request_channel_pcm(&mut self, track: u8) -> Option<usize> {
        let preferred_order = [4, 5, 6, 7, 0, 2, 8, 9, 10, 11, 12, 13, 14, 15];
        for i in preferred_order {
            match self.channels[i] {
                Channel::Open => {
                    return Some(i);
                }
                _ => {}
            }
        }
        for i in preferred_order {
            match &mut self.channels[i] {
                Channel::Freeing { volume, .. } => {
                    if *volume <= 1 {
                        self.channels[i] = Channel::Open;
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        for i in preferred_order {
            match &mut self.channels[i] {
                Channel::Used {
                    sound: QueuedSound::Drum(drum),
                    ..
                } => {
                    if drum.pretend_track == track {
                        self.channels[i] = Channel::Open;
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        for i in preferred_order {
            match &mut self.channels[i] {
                Channel::Used {
                    sound: QueuedSound::Note(note),
                    ..
                } => {
                    if note.track == track {
                        self.channels[i] = Channel::Open;
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn request_channel_noise(&mut self, track: u8) -> Option<usize> {
        let preferred_order = [8, 9, 10, 11, 12, 13, 14, 15];
        for i in preferred_order {
            match self.channels[i] {
                Channel::Open => {
                    return Some(i);
                }
                _ => {}
            }
        }
        for i in preferred_order {
            match &mut self.channels[i] {
                Channel::Freeing { volume, .. } => {
                    if *volume <= 1 {
                        self.channels[i] = Channel::Open;
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        for i in preferred_order {
            match &mut self.channels[i] {
                Channel::Used {
                    sound: QueuedSound::Drum(drum),
                    ..
                } => {
                    if drum.pretend_track == track {
                        self.channels[i] = Channel::Open;
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        for i in preferred_order {
            match &mut self.channels[i] {
                Channel::Used {
                    sound: QueuedSound::Note(note),
                    ..
                } => {
                    if note.track == track {
                        self.channels[i] = Channel::Open;
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        None
    }

    pub fn release_tracks(&mut self, track: u8, mio_tick: usize) {
        for channel in &mut self.channels {
            match channel {
                Channel::Used {
                    sound,
                    envelope,
                    volume,
                    ..
                } => {
                    let tick = ((mio_tick - sound.time() as usize * NOTE_RATE) / 171) as u32;
                    if sound.track() == track {
                        *channel = Channel::Freeing {
                            sound: sound.clone(),
                            volume: *volume,
                            initial_release_volume: *volume,
                            kill_tick: tick,
                            release: envelope.as_ref().map(|env| env.release.clone()).flatten(),
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn add_adsr_to_channel(
    channel_manager: &mut ChannelManager,
    spu: &mut Spu,
    note: &QueuedNote,
    note_offset: u32,
    until_note: u32,
    instruments: &[Instrument],
    adsr: &Adsr,
    future_adsr: VecDeque<(u32, Adsr)>,
    tick: u32,
    previous_notes: &mut [Option<u8>; 4],
) -> usize {
    let Adsr {
        sample,
        attack,
        decay,
        sustain,
        release,
    } = adsr;

    let channel_id = channel_manager.request_channel_pcm(note.track).unwrap();
    let initial_volume = interp_val_until(note_offset, sample.volume(), until_note);

    let attack_volume = match attack {
        Some(InstrumentAttack::Exact { adjustments })
        | Some(InstrumentAttack::ExactWithMagicPitch { adjustments }) => {
            interp_val_until(note_offset, adjustments.last().unwrap().volume, until_note)
        }
        None => initial_volume,
    };

    match sample {
        InstrumentSample::PCM16(sample) => {
            let timer_reload = nth_timer_reload(sample.base_timer_reload, note_offset as i32);

            spu.set_adjusted_channel_pan(channel_id, 64, note.pan_addition);
            spu.set_adjusted_channel_volume(channel_id, initial_volume, note.volume_multiplier);
            spu.set_channel_timer_reload(channel_id, timer_reload as u32);
            spu.set_channel_loop_pos(channel_id, sample.loop_pos);
            spu.set_channel_length(channel_id, sample.length);
            spu.set_channel_src_address(channel_id, sample.src_address);
            spu.channel_play_note(channel_id, sample.is_repeating);
        }
        InstrumentSample::PSG(sample) => unimplemented!(),
    }

    let mut pitch_adjustments: Vec<TimedPitchAdjustment> = instruments[note.instrument as usize]
        .pitch_adjustments
        .iter()
        .map(|adj| TimedPitchAdjustment {
            time: adj.time,
            timer_reload: nth_micro_timer_reload(
                sample.base_timer_reload(),
                note_offset as f32 + adj.pitch_adjust,
            ),
        })
        .collect();

    let sustain_range = sustain.map(|s| s.volume).unwrap_or((1, 1));
    let this_sustain = interp_val_until(note_offset, sustain_range, until_note);
    let mut sustain_addition = 0;

    let decay_envelope = if let Some(decay_env) = decay {
        match decay_env {
            InstrumentDecay::Exponential { duration } => {
                let decay_constant =
                    (1.0 / *duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: *duration,
                })
            }
            InstrumentDecay::ExponentialWithVariableSustain {
                duration,
                sustain_duration,
            } => {
                sustain_addition = interp_val_until(note_offset, *sustain_duration, until_note);
                let duration = *duration - sustain_addition;
                let decay_constant =
                    (1.0 / duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: duration,
                })
            }
            InstrumentDecay::ExponentialWithWobble {
                duration,
                wobble_start,
                max_wobble,
                wave_duration,
            } => {
                let rate = 2;
                let mut ascending = true;
                let mut wobble = 0.0;
                for i in (*wobble_start..*duration).step_by(rate as usize) {
                    if ascending {
                        wobble += max_wobble / *wave_duration as f32 * 4.0;
                        if wobble >= *max_wobble {
                            ascending = false;
                        }
                    }
                    if !ascending {
                        wobble -= max_wobble / *wave_duration as f32 * 4.0;
                        if wobble <= -*max_wobble {
                            ascending = true;
                        }
                    }
                    //println!("DIG: {}, {}", i, wobble);
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time: i,
                        timer_reload: nth_micro_timer_reload(
                            sample.base_timer_reload(),
                            note_offset as f32 + wobble,
                        ),
                    });
                }
                pitch_adjustments.push(TimedPitchAdjustment {
                    time: *duration + 1,
                    timer_reload: nth_micro_timer_reload(
                        sample.base_timer_reload(),
                        note_offset as f32,
                    ),
                });
                println!("PITHC: {:?}", pitch_adjustments);
                let decay_constant =
                    (1.0 / *duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: *duration,
                })
            }
            InstrumentDecay::ExponentialRising {
                duration,
                rise_start,
                max_rise,
            } => {
                let rate = 2;
                let mut rise = 0.0;
                for i in (*rise_start..*duration).step_by(rate as usize) {
                    rise += max_rise / (*duration - *rise_start) as f32 * 2.0;
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time: i,
                        timer_reload: nth_micro_timer_reload(
                            sample.base_timer_reload(),
                            note_offset as f32 + rise,
                        ),
                    });
                }
                let decay_constant =
                    (1.0 / *duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: *duration,
                })
            }
            InstrumentDecay::Linear { duration } => Some(DecayEnvelope::Linear {
                duration: *duration,
            }),
        }
    } else {
        None
    };

    let sustain_envelope = match sustain {
        Some(sustain) => Some(SustainEnvelope {
            volume: interp_val_until(note_offset, sustain.volume, until_note),
            duration: sustain.duration + sustain_addition,
        }),
        None => None,
    };

    let release_envelope = match release {
        Some(InstrumentRelease::Basic) => Some(ReleaseInstructions::Basic),
        Some(InstrumentRelease::Geometric { ratio }) => {
            Some(ReleaseInstructions::Geometric { ratio: *ratio })
        }
        Some(InstrumentRelease::GeometricStopBlowing { ratio }) => {
            Some(ReleaseInstructions::GeometricStopBlowing { ratio: *ratio })
        }
        Some(InstrumentRelease::Static(value)) => Some(ReleaseInstructions::Static(
            interp_val_until(note_offset, *value, until_note),
        )),
        //Some(InstrumentRelease::GeometricFastExit { ratio }) => {
        //    Some(ReleaseInstructions::GeometricFastExit { ratio: *ratio })
        //}
        None => None,
    };

    let attack_envelope = match attack {
        Some(InstrumentAttack::Exact { adjustments }) => Some(AttackEnvelope::Exact {
            adjustments: adjustments
                .iter()
                .cloned()
                .map(|adj| TimedVolumeAdjustment {
                    time: adj.time,
                    volume: interp_val_until(note_offset, adj.volume, until_note),
                })
                .collect(),
        }),
        Some(InstrumentAttack::ExactWithMagicPitch { adjustments }) => {
            let x = 0;
            if let Some(previous_note) = previous_notes[note.track as usize] {
                let mut n = previous_note;
                let mut time = 0;
                while n != note.note {
                    let step = if n < note.note { 1.0 } else { -1.0 };
                    time += 1;
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time,
                        timer_reload: nth_micro_timer_reload(
                            sample.base_timer_reload(),
                            n as f32 + step / 2.0,
                        ),
                    });

                    if n < note.note {
                        n += 1;
                    } else {
                        n -= 1;
                    }

                    time += 1;
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time,
                        timer_reload: nth_micro_timer_reload(sample.base_timer_reload(), n as f32),
                    });
                }
            }
            previous_notes[note.track as usize] = Some(note.note);
            Some(AttackEnvelope::Exact {
                adjustments: adjustments
                    .iter()
                    .cloned()
                    .map(|adj| TimedVolumeAdjustment {
                        time: adj.time,
                        volume: interp_val_until(note_offset, adj.volume, until_note),
                    })
                    .collect(),
            })
        }
        None => None,
    };

    let envelope = if attack_envelope.is_none()
        && decay_envelope.is_none()
        && sustain_envelope.is_none()
        && release_envelope.is_none()
    {
        None
    } else {
        Some(Envelope {
            attack: attack_envelope,
            initial_volume,
            decay: decay_envelope,
            sustain: sustain_envelope,
            release: release_envelope,
        })
    };

    println!("ADJ {:?}", pitch_adjustments);

    channel_manager.allocate_pcm(
        note.clone(),
        envelope,
        initial_volume,
        pitch_adjustments,
        future_adsr,
        tick,
    );

    channel_id
}

/*pub fn add_drum_to_channel(
    channel_manager: &mut ChannelManager,
    spu: &mut Spu,
    drum: &QueuedDrum,
    rhythm_section: &[RhythmSection],
    adsr: &Adsr,
    future_adsr: VecDeque<(u32, Adsr)>,
    tick: u32,
    previous_notes: &mut [Option<u8>; 4],
) -> usize {
    let Adsr {
        sample,
        attack,
        decay,
        sustain,
        release,
    } = adsr;

    let channel_id = channel_manager
        .request_channel_pcm(drum.pretend_track)
        .unwrap();
    let initial_volume = sample.volume();

    let attack_volume = match attack {
        Some(InstrumentAttack::Exact { adjustments })
        | Some(InstrumentAttack::ExactWithMagicPitch { adjustments }) => {
            interp_val_until(note_offset, adjustments.last().unwrap().volume, until_note)
        }
        None => initial_volume,
    };

    match sample {
        InstrumentSample::PCM16(sample) => {
            let timer_reload = nth_timer_reload(sample.base_timer_reload, note_offset as i32);

            spu.set_adjusted_channel_pan(channel_id, 64, note.pan_addition);
            spu.set_adjusted_channel_volume(channel_id, initial_volume, note.volume_multiplier);
            spu.set_channel_timer_reload(channel_id, timer_reload as u32);
            spu.set_channel_loop_pos(channel_id, sample.loop_pos);
            spu.set_channel_length(channel_id, sample.length);
            spu.set_channel_src_address(channel_id, sample.src_address);
            spu.channel_play_note(channel_id, sample.is_repeating);
        }
        InstrumentSample::PSG(sample) => unimplemented!(),
    }

    let mut pitch_adjustments: Vec<TimedPitchAdjustment> = instruments[note.instrument as usize]
        .pitch_adjustments
        .iter()
        .map(|adj| TimedPitchAdjustment {
            time: adj.time,
            timer_reload: nth_micro_timer_reload(
                sample.base_timer_reload(),
                note_offset as f32 + adj.pitch_adjust,
            ),
        })
        .collect();

    let sustain_range = sustain.map(|s| s.volume).unwrap_or((1, 1));
    let this_sustain = interp_val_until(note_offset, sustain_range, until_note);
    let mut sustain_addition = 0;

    let decay_envelope = if let Some(decay_env) = decay {
        match decay_env {
            InstrumentDecay::Exponential { duration } => {
                let decay_constant =
                    (1.0 / *duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: *duration,
                })
            }
            InstrumentDecay::ExponentialWithVariableSustain {
                duration,
                sustain_duration,
            } => {
                sustain_addition = interp_val_until(note_offset, *sustain_duration, until_note);
                let duration = *duration - sustain_addition;
                let decay_constant =
                    (1.0 / duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: duration,
                })
            }
            InstrumentDecay::ExponentialWithWobble {
                duration,
                wobble_start,
                max_wobble,
                wave_duration,
            } => {
                let rate = 2;
                let mut ascending = true;
                let mut wobble = 0.0;
                for i in (*wobble_start..*duration).step_by(rate as usize) {
                    if ascending {
                        wobble += max_wobble / *wave_duration as f32 * 4.0;
                        if wobble >= *max_wobble {
                            ascending = false;
                        }
                    }
                    if !ascending {
                        wobble -= max_wobble / *wave_duration as f32 * 4.0;
                        if wobble <= -*max_wobble {
                            ascending = true;
                        }
                    }
                    //println!("DIG: {}, {}", i, wobble);
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time: i,
                        timer_reload: nth_micro_timer_reload(
                            sample.base_timer_reload(),
                            note_offset as f32 + wobble,
                        ),
                    });
                }
                pitch_adjustments.push(TimedPitchAdjustment {
                    time: *duration + 1,
                    timer_reload: nth_micro_timer_reload(
                        sample.base_timer_reload(),
                        note_offset as f32,
                    ),
                });
                println!("PITHC: {:?}", pitch_adjustments);
                let decay_constant =
                    (1.0 / *duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: *duration,
                })
            }
            InstrumentDecay::ExponentialRising {
                duration,
                rise_start,
                max_rise,
            } => {
                let rate = 2;
                let mut rise = 0.0;
                for i in (*rise_start..*duration).step_by(rate as usize) {
                    rise += max_rise / (*duration - *rise_start) as f32 * 2.0;
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time: i,
                        timer_reload: nth_micro_timer_reload(
                            sample.base_timer_reload(),
                            note_offset as f32 + rise,
                        ),
                    });
                }
                let decay_constant =
                    (1.0 / *duration as f32) * (this_sustain as f32 / attack_volume as f32).ln();
                Some(DecayEnvelope::Exponential {
                    constant: decay_constant,
                    duration: *duration,
                })
            }
            InstrumentDecay::Linear { duration } => Some(DecayEnvelope::Linear {
                duration: *duration,
            }),
        }
    } else {
        None
    };

    let sustain_envelope = match sustain {
        Some(sustain) => Some(SustainEnvelope {
            volume: interp_val_until(note_offset, sustain.volume, until_note),
            duration: sustain.duration + sustain_addition,
        }),
        None => None,
    };

    let release_envelope = match release {
        Some(InstrumentRelease::Basic) => Some(ReleaseInstructions::Basic),
        Some(InstrumentRelease::Geometric { ratio }) => {
            Some(ReleaseInstructions::Geometric { ratio: *ratio })
        }
        Some(InstrumentRelease::GeometricStopBlowing { ratio }) => {
            Some(ReleaseInstructions::GeometricStopBlowing { ratio: *ratio })
        }
        Some(InstrumentRelease::Static(value)) => Some(ReleaseInstructions::Static(
            interp_val_until(note_offset, *value, until_note),
        )),
        //Some(InstrumentRelease::GeometricFastExit { ratio }) => {
        //    Some(ReleaseInstructions::GeometricFastExit { ratio: *ratio })
        //}
        None => None,
    };

    let attack_envelope = match attack {
        Some(InstrumentAttack::Exact { adjustments }) => Some(AttackEnvelope::Exact {
            adjustments: adjustments
                .iter()
                .cloned()
                .map(|adj| TimedVolumeAdjustment {
                    time: adj.time,
                    volume: interp_val_until(note_offset, adj.volume, until_note),
                })
                .collect(),
        }),
        Some(InstrumentAttack::ExactWithMagicPitch { adjustments }) => {
            let x = 0;
            if let Some(previous_note) = previous_notes[note.track as usize] {
                let mut n = previous_note;
                let mut time = 0;
                while n != note.note {
                    let step = if n < note.note { 1.0 } else { -1.0 };
                    time += 1;
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time,
                        timer_reload: nth_micro_timer_reload(
                            sample.base_timer_reload(),
                            n as f32 + step / 2.0,
                        ),
                    });

                    if n < note.note {
                        n += 1;
                    } else {
                        n -= 1;
                    }

                    time += 1;
                    pitch_adjustments.push(TimedPitchAdjustment {
                        time,
                        timer_reload: nth_micro_timer_reload(sample.base_timer_reload(), n as f32),
                    });
                }
            }
            previous_notes[note.track as usize] = Some(note.note);
            Some(AttackEnvelope::Exact {
                adjustments: adjustments
                    .iter()
                    .cloned()
                    .map(|adj| TimedVolumeAdjustment {
                        time: adj.time,
                        volume: interp_val_until(note_offset, adj.volume, until_note),
                    })
                    .collect(),
            })
        }
        None => None,
    };

    let envelope = if attack_envelope.is_none()
        && decay_envelope.is_none()
        && sustain_envelope.is_none()
        && release_envelope.is_none()
    {
        None
    } else {
        Some(Envelope {
            attack: attack_envelope,
            initial_volume,
            decay: decay_envelope,
            sustain: sustain_envelope,
            release: release_envelope,
        })
    };

    println!("ADJ {:?}", pitch_adjustments);

    channel_manager.allocate_pcm(
        note.clone(),
        envelope,
        initial_volume,
        pitch_adjustments,
        future_adsr,
        tick,
    );

    channel_id
}*/

fn nth_timer_reload(original: u16, offset: i32) -> u16 {
    let timer = 512.0;
    let max_reload = 65536.0;
    let m = timer / (max_reload - original as f32);
    println!("offset: {}, M: {}", offset, m);
    (max_reload - timer / (2_f32.powf(offset as f32 / 12.0) * m)).round() as u16
}

fn nth_micro_timer_reload(original: u16, offset: f32) -> u16 {
    let timer = 512.0;
    let max_reload = 65536.0;
    let m = timer / (max_reload - original as f32);
    (max_reload - timer / (2_f32.powf(offset / 12.0) * m)).round() as u16
}

mod tests {
    use super::*;

    #[test]
    fn test_timer_reload() {
        let timer_reloads = vec![
            vec![
                63898, 63990, 64077, 64159, 64236, 64309, 64378, 64443, 64504, 64562, 64617, 64669,
                64717, 64763, 64807, 64848, 64886, 64923, 64957, 64990, 65020, 65049, 65077, 65103,
                65127,
            ],
            vec![
                63496, 63611, 63719, 63821, 63917, 64008, 64094, 64175, 64251, 64324, 64392, 64456,
                64516, 64574, 64628, 64679, 64727, 64772, 64815, 64856, 64894, 64930, 64964, 64996,
                65026,
            ],
            vec![
                63922, 64013, 64099, 64179, 64255, 64327, 64395, 64459, 64520, 64577, 64631, 64682,
                64729, 64775, 64818, 64858, 64896, 64932, 64966, 64998, 65028, 65057, 65084, 65109,
                65133,
            ],
            vec![
                63516, 63630, 63737, 63838, 63933, 64023, 64108, 64188, 64264, 64335, 64403, 64466,
                64526, 64583, 64637, 64687, 64735, 64780, 64822, 64862, 64900, 64936, 64970, 65001,
                65031,
            ],
            vec![
                64174, 64251, 64323, 64391, 64455, 64516, 64573, 64627, 64678, 64727, 64772, 64815,
                64855, 64894, 64930, 64964, 64996, 65026, 65055, 65082, 65107, 65132, 65154, 65176,
                65196,
            ],
            vec![
                64178, 64255, 64327, 64395, 64459, 64519, 64576, 64630, 64681, 64729, 64774, 64817,
                64857, 64896, 64932, 64966, 64998, 65028, 65056, 65083, 65109, 65133, 65155, 65177,
                65197,
            ],
            vec![
                63803, 63900, 63992, 64079, 64161, 64238, 64311, 64380, 64445, 64506, 64564, 64618,
                64670, 64718, 64764, 64808, 64849, 64887, 64924, 64958, 64991, 65021, 65050, 65077,
                65103,
            ],
            vec![
                61448, 61678, 61895, 62099, 62292, 62474, 62646, 62808, 62961, 63106, 63242, 63371,
                63492, 63607, 63716, 63818, 63914, 64005, 64091, 64172, 64249, 64321, 64389, 64454,
                64514,
            ],
            vec![
                63504, 63619, 63726, 63828, 63924, 64014, 64100, 64180, 64256, 64328, 64396, 64460,
                64520, 64578, 64631, 64682, 64730, 64775, 64818,
            ],
            vec![64170, 64246, 64319, 64387, 64452, 64512],
            vec![
                63900, 63992, 64079, 64161, 64238, 64311, 64380, 64445, 64506, 64564, 64618, 64670,
                64718, 64764, 64808, 64849, 64887, 64924, 64958, 64991, 65021, 65050, 65077, 65103,
                65127,
            ],
            vec![
                63528, 63641, 63748, 63848, 63943, 64032, 64117, 64196, 64272, 64343, 64410, 64473,
                64532, 64589, 64642, 64692, 64740, 64784, 64827, 64866, 64904, 64940, 64973, 65005,
                65034,
            ],
            vec![
                64519, 64577, 64630, 64681, 64729, 64775, 64817, 64858, 64896, 64932, 64966, 64998,
                65028, 65057, 65083, 65109, 65133, 65156, 65177, 65197, 65216, 65234, 65251, 65267,
                65282,
            ],
            vec![
                64450, 64511, 64568, 64623, 64674, 64722, 64768, 64811, 64852, 64890, 64927, 64961,
                64993, 65024, 65052, 65080, 65105, 65129, 65152, 65174, 65194, 65213, 65232,
                65249, // ...?
            ],
            vec![
                63828, 63924, 64014, 64100, 64180, 64256, 64328, 64396, 64460, 64520, 64578, 64631,
                64682, 64730, 64775, 64818, 64858, 64896, 64932, 64966, 64998, 65028, 65057, 65084,
                65109,
            ],
            vec![
                63502, 63617, 63724, 63826, 63922, 64013, 64098, 64179, 64255, 64327, 64395, 64459,
                64519, 64577, 64630, 64681, 64729, 64775, 64817, 64858, 64896, 64932, 64966, 64998,
                65028,
            ],
            vec![64088, 64170, 64246, 64319, 64387, 64452, 64512],
            vec![
                64170, 64246, 64319, 64387, 64452, 64512, 64570, 64624, 64675, 64724, 64769, 64812,
                64853, 64891, 64928, 64962, 64994, 65024, 65053, 65080, 65106, 65130, 65153, 65174,
                65195,
            ],
            vec![
                63488, 63603, 63712, 63814, 63911, 64002, 64088, 64170, 64246, 64319, 64387, 64452,
            ],
            vec![
                64002, 64088, 64170, 64246, 64319, 64387, 64452, 64512, 64570, 64624, 64675, 64724,
                64769,
            ],
            vec![64171, 64248, 64320, 64389, 64453],
            vec![64171, 64248],
            vec![64175, 64252],
            vec![64178, 64255],
            vec![64179, 64256],
            vec![64176, 64253],
            vec![64173, 64250, 64322, 64390, 64455, 64515],
            vec![
                63472, 63588, 63697, 63800, 63898, 63990, 64077, 64159, 64236, 64309, 64378, 64443,
                64504, 64562, 64617, 64668, 64717, 64763, 64807, 64848, 64886, 64923, 64957, 64990,
                65020,
            ],
            vec![
                63494, 63609, 63717, 63819, 63916, 64007, 64093, 64174, 64250, 64322, 64390, 64455,
                64515, 64573, 64627, 64678, 64726, 64772, 64815, 64855, 64893, 64929, 64963, 64996,
                65026,
            ],
            vec![
                63488, 63603, 63712, 63814, 63911, 64002, 64088, 64170, 64246, 64319, 64387, 64452,
                64512, 64570, 64624, 64675, 64724, 64769, 64812, 64853, 64891, 64928, 64962, 64994,
                65024,
            ],
            vec![64004, 64090, 64171, 64248, 64320, 64389],
            vec![64171, 64248, 64320, 64389, 64453, 64514],
        ];

        for timer_reload_list in timer_reloads {
            println!("{:?}", timer_reload_list);
            for (note, &timer_reload) in timer_reload_list.iter().enumerate() {
                assert!(
                    nth_timer_reload(timer_reload_list[0], note as i32).abs_diff(timer_reload) <= 1
                )
            }
        }
    }
}
