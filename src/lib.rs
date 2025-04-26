mod utils;

use record::Record;
use std::sync::{Arc, Mutex};
use tinyaudio::run_output_device;
use tinyaudio::BaseAudioOutputDevice;
use tinyaudio::OutputDeviceParameters;

use wasm_bindgen::prelude::*;

mod spu;

use spu::{AudioBitDepth, Nds, Spu};

mod drums;
mod ins;
mod record;
mod audio;

use drums::drum_instructions;
use ins::instrument_instructions;
use audio::*;

static mut DEVICE: Option<Box<dyn BaseAudioOutputDevice>> = None;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wahdio!");
}

#[wasm_bindgen]
pub fn stop_music() {
    unsafe {
        DEVICE = None;
    }
}

#[wasm_bindgen]
pub fn play_music(mio_data: &[u8], ram: &[u8], my_volume: f32) {
    utils::set_panic_hook();
    let multiplier = 8;
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: SAMPLE_RATE,
        channel_sample_count: 1024 * multiplier,
    };

    // No longer send in entire ram
    //assert_eq!(ram.len(), 4 * 1024 * 1024);

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

    let spu = Arc::new(Mutex::new(spu));

    let instruments = instrument_instructions();

    let rhythm_sections: [RhythmSection; RHYTHM_SECTION_COUNT] = drum_instructions();

    let record = Record::from_mio(&mio_data);

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

    let mut timing = Timing {
        tiny_tick: 0,
        phrase_tick: 0,
    };

    unsafe {
        DEVICE = run_output_device(params, {
            let spu = spu.clone();
            move |data| {
                play_stuff(
                    spu.clone(),
                    multiplier,
                    &mut timing,
                    &mut channel_manager,
                    &record,
                    &instruments,
                    &&rhythm_sections,
                    &mut previous_notes,
                    data.chunks_mut(params.channels_count),
                    my_volume,
                );
            }
        })
        .ok();
    }
}
