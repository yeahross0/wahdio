use crate::split::*;

pub fn drum_instructions() -> [RhythmSection; RHYTHM_SECTION_COUNT] {
    [
        RhythmSection::new(
            "Normal".to_string(),
            [
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 576,
                        timer_reload: 64718,
                        loop_pos: 1196,
                        length: 828,
                        src_address: 34644356,
                        is_repeating: true,
                        base_pan: 55,
                    },
                    decay: DecayInstructions::exponential(192),
                    sustain: SustainInstructions {
                        volume: 22,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                },
                DrumInstructions::Decay {
                    sample: DrumSample {
                        volume: 648,
                        timer_reload: 64718,
                        loop_pos: 1196,
                        length: 828,
                        src_address: 34644356,
                        is_repeating: true,
                        base_pan: 55,
                    },
                    decay: DecayInstructions {
                        duration: 48,
                        kind: DecayKind::Exponential,
                    },
                    final_volume: 0,
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2176,
                        src_address: 34638012,
                        is_repeating: false,
                        base_pan: 70,
                    },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 228,
                        timer_reload: 64854,
                        loop_pos: 4,
                        length: 4136,
                        src_address: 34640204,
                        is_repeating: false,
                        base_pan: 20,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 1,
                            volume: 864,
                        },
                        TimedVolumeAdjustment {
                            time: 2,
                            volume: 1120,
                        },
                        TimedVolumeAdjustment {
                            time: 3,
                            volume: 1184,
                        },
                        TimedVolumeAdjustment {
                            time: 4,
                            volume: 1200,
                        },
                    ],
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1200,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 988,
                        src_address: 34646392,
                        is_repeating: false,
                        base_pan: 80,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1200,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 712,
                        src_address: 34650860,
                        is_repeating: false,
                        base_pan: 64,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1296,
                        timer_reload: 64624,
                        loop_pos: 4,
                        length: 3284,
                        src_address: 34654224,
                        is_repeating: false,
                        base_pan: 40,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1376,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 3284,
                        src_address: 34654224,
                        is_repeating: false,
                        base_pan: 70,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1456,
                        timer_reload: 64387,
                        loop_pos: 4,
                        length: 3284,
                        src_address: 34654224,
                        is_repeating: false,
                        base_pan: 100,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 808,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 3852,
                        src_address: 34629696,
                        is_repeating: false,
                        base_pan: 90,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 968,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2208,
                        src_address: 34635788,
                        is_repeating: false,
                        base_pan: 35,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 880,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1228,
                        src_address: 34628452,
                        is_repeating: false,
                        base_pan: 35,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1456,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2620,
                        src_address: 34651588,
                        is_repeating: false,
                        base_pan: 64,
                    },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 1296,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2104,
                        src_address: 34648740,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 48,
                            volume: 648,
                        },
                        TimedVolumeAdjustment {
                            time: 49,
                            volume: 324,
                        },
                        TimedVolumeAdjustment {
                            time: 50,
                            volume: 164,
                        },
                    ],
                },
            ],
        ),
        RhythmSection::new(
            "Electric".to_string(),
            [
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 744,
                        timer_reload: 64897,
                        loop_pos: 868,
                        length: 400,
                        src_address: 34666300,
                        is_repeating: true,
                        base_pan: 55,
                    },
                    decay: DecayInstructions::exponential(192),
                    sustain: SustainInstructions {
                        volume: 47,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                },
                DrumInstructions::Decay {
                    sample: DrumSample {
                        volume: 904,
                        timer_reload: 64897,
                        loop_pos: 868,
                        length: 400,
                        src_address: 34666300,
                        is_repeating: true,
                        base_pan: 55,
                    },
                    decay: DecayInstructions {
                        duration: 48,
                        kind: DecayKind::Exponential,
                    },
                    final_volume: 0,
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 648,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1188,
                        src_address: 34665096,
                        is_repeating: false,
                        base_pan: 70,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 904,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 956,
                        src_address: 34664124,
                        is_repeating: false,
                        base_pan: 20,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 896,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1208,
                        src_address: 34662900,
                        is_repeating: false,
                        base_pan: 80,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1000,
                        timer_reload: 64717,
                        loop_pos: 4,
                        length: 3408,
                        src_address: 34659476,
                        is_repeating: false,
                        base_pan: 64,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64995,
                        loop_pos: 4,
                        length: 1936,
                        src_address: 34657524,
                        is_repeating: false,
                        base_pan: 40,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64893,
                        loop_pos: 4,
                        length: 1936,
                        src_address: 34657524,
                        is_repeating: false,
                        base_pan: 70,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1056,
                        timer_reload: 64725,
                        loop_pos: 4,
                        length: 1936,
                        src_address: 34657524,
                        is_repeating: false,
                        base_pan: 100,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 728,
                        timer_reload: 64717,
                        loop_pos: 4,
                        length: 4608,
                        src_address: 34674004,
                        is_repeating: false,
                        base_pan: 90,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 880,
                        timer_reload: 64854,
                        loop_pos: 4,
                        length: 3504,
                        src_address: 34670484,
                        is_repeating: false,
                        base_pan: 35,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 808,
                        timer_reload: 64854,
                        loop_pos: 4,
                        length: 1228,
                        src_address: 34669240,
                        is_repeating: false,
                        base_pan: 35,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1456,
                        timer_reload: 64717,
                        loop_pos: 4,
                        length: 1644,
                        src_address: 34667580,
                        is_repeating: false,
                        base_pan: 64,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1680,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 928,
                        src_address: 34678628,
                        is_repeating: false,
                        base_pan: 64,
                    },
                },
            ],
        ),
        RhythmSection::new(
            "Samba".to_string(),
            [
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 728,
                        timer_reload: 64521,
                        loop_pos: 996,
                        length: 368,
                        src_address: 34620292,
                        is_repeating: true,
                        base_pan: 55,
                    },
                    decay: DecayInstructions::linear(12),
                    sustain: SustainInstructions {
                        volume: 448,
                        duration: 54,
                    },
                    release: ReleaseInstructions::Exponential { duration: 19 },
                },
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 880,
                        timer_reload: 64329,
                        loop_pos: 996,
                        length: 368,
                        src_address: 34620292,
                        is_repeating: true,
                        base_pan: 55,
                    },
                    decay: DecayInstructions::linear(7),
                    sustain: SustainInstructions {
                        volume: 688,
                        duration: 13,
                    },
                    release: ReleaseInstructions::Exponential { duration: 20 },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 872,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2124,
                        src_address: 34613700,
                        is_repeating: false,
                        base_pan: 100,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1376,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2400,
                        src_address: 34615840,
                        is_repeating: false,
                        base_pan: 40,
                    },
                },
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 1056,
                        timer_reload: 64515,
                        loop_pos: 1244,
                        length: 1264,
                        src_address: 34624688,
                        is_repeating: true,
                        base_pan: 80,
                    },
                    decay: DecayInstructions::exponential(192),
                    sustain: SustainInstructions {
                        volume: 109,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Exponential { duration: 3 },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 796,
                        src_address: 34619480,
                        is_repeating: false,
                        base_pan: 80,
                    },
                },
                DrumInstructions::Sr {
                    sample: DrumSample {
                        volume: 1296,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1432,
                        src_address: 34609932,
                        is_repeating: false,
                        base_pan: 90,
                    },
                    sustain: SustainInstructions {
                        volume: 1296,
                        duration: 24,
                    },
                    release: ReleaseInstructions::Exponential { duration: 7 },
                },
                DrumInstructions::Sr {
                    sample: DrumSample {
                        volume: 1168,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1300,
                        src_address: 34606352,
                        is_repeating: false,
                        base_pan: 35,
                    },
                    sustain: SustainInstructions {
                        volume: 1168,
                        duration: 20,
                    },
                    release: ReleaseInstructions::Exponential { duration: 7 },
                },
                DrumInstructions::Sr {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2248,
                        src_address: 34607668,
                        is_repeating: false,
                        base_pan: 25,
                    },
                    sustain: SustainInstructions {
                        volume: 1120,
                        duration: 32,
                    },
                    release: ReleaseInstructions::Exponential { duration: 7 },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 936,
                        timer_reload: 64812,
                        loop_pos: 4,
                        length: 2304,
                        src_address: 34611380,
                        is_repeating: false,
                        base_pan: 95,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1000,
                        timer_reload: 64570,
                        loop_pos: 4,
                        length: 2304,
                        src_address: 34611380,
                        is_repeating: false,
                        base_pan: 95,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 728,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1208,
                        src_address: 34618256,
                        is_repeating: false,
                        base_pan: 35,
                    },
                },
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 968,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 3004,
                        src_address: 34621668,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    decay: DecayInstructions::exponential(70),
                    sustain: SustainInstructions {
                        volume: 292,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Exponential { duration: 6 },
                },
                DrumInstructions::Decay {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64170,
                        loop_pos: 4,
                        length: 3004,
                        src_address: 34621668,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    decay: DecayInstructions {
                        duration: 96,
                        kind: DecayKind::Exponential,
                    },
                    final_volume: 216,
                },
            ],
        ),
        RhythmSection::new(
            "Asian".to_string(),
            [
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2052,
                        src_address: 34555692,
                        is_repeating: false,
                        base_pan: 80,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 904,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2868,
                        src_address: 34552808,
                        is_repeating: false,
                        base_pan: 80,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1296,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 528,
                        src_address: 34557760,
                        is_repeating: false,
                        base_pan: 80,
                    },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 1296,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 2152,
                        src_address: 34558304,
                        is_repeating: false,
                        base_pan: 80,
                    },
                    adjustments: vec![TimedVolumeAdjustment {
                        time: 68,
                        volume: 324,
                    }],
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1200,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 988,
                        src_address: 34646392,
                        is_repeating: false,
                        base_pan: 80,
                    },
                },
                DrumInstructions::PitchThenRelease {
                    sample: DrumSample {
                        volume: 1024,
                        timer_reload: 64517,
                        loop_pos: 1332,
                        length: 736,
                        src_address: 34547468,
                        is_repeating: true,
                        base_pan: 55,
                    },
                    sustain: SustainInstructions {
                        volume: 1024,
                        duration: 144,
                    },
                    release: ReleaseInstructions::Exponential { duration: 57 },
                    adjustments: vec![
                        TimedPitchAdjustment {
                            time: 4,
                            timer_reload: 64571,
                        },
                        TimedPitchAdjustment {
                            time: 9,
                            timer_reload: 64592,
                        },
                        TimedPitchAdjustment {
                            time: 16,
                            timer_reload: 64622,
                        },
                        TimedPitchAdjustment {
                            time: 22,
                            timer_reload: 64651,
                        },
                        TimedPitchAdjustment {
                            time: 26,
                            timer_reload: 64670,
                        },
                        TimedPitchAdjustment {
                            time: 32,
                            timer_reload: 64661,
                        },
                        TimedPitchAdjustment {
                            time: 38,
                            timer_reload: 64642,
                        },
                        TimedPitchAdjustment {
                            time: 44,
                            timer_reload: 64622,
                        },
                        TimedPitchAdjustment {
                            time: 50,
                            timer_reload: 64602,
                        },
                        TimedPitchAdjustment {
                            time: 56,
                            timer_reload: 64582,
                        },
                        TimedPitchAdjustment {
                            time: 62,
                            timer_reload: 64582,
                        },
                        TimedPitchAdjustment {
                            time: 68,
                            timer_reload: 64539,
                        },
                        TimedPitchAdjustment {
                            time: 74,
                            timer_reload: 64517,
                        },
                        TimedPitchAdjustment {
                            time: 80,
                            timer_reload: 64561,
                        },
                        TimedPitchAdjustment {
                            time: 86,
                            timer_reload: 64602,
                        },
                        TimedPitchAdjustment {
                            time: 92,
                            timer_reload: 64642,
                        },
                        TimedPitchAdjustment {
                            time: 98,
                            timer_reload: 64680,
                        },
                        TimedPitchAdjustment {
                            time: 104,
                            timer_reload: 64716,
                        },
                        TimedPitchAdjustment {
                            time: 110,
                            timer_reload: 64751,
                        },
                        TimedPitchAdjustment {
                            time: 116,
                            timer_reload: 64792,
                        },
                        TimedPitchAdjustment {
                            time: 122,
                            timer_reload: 64824,
                        },
                        TimedPitchAdjustment {
                            time: 128,
                            timer_reload: 64854,
                        },
                        TimedPitchAdjustment {
                            time: 134,
                            timer_reload: 64883,
                        },
                        TimedPitchAdjustment {
                            time: 140,
                            timer_reload: 64911,
                        },
                        TimedPitchAdjustment {
                            time: 142,
                            timer_reload: 64924,
                        },
                        TimedPitchAdjustment {
                            time: 144,
                            timer_reload: 64904,
                        },
                    ],
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1200,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 1848,
                        src_address: 34560832,
                        is_repeating: false,
                        base_pan: 55,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1616,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 1848,
                        src_address: 34562696,
                        is_repeating: false,
                        base_pan: 20,
                    },
                },
                DrumInstructions::RepeatOnce {
                    sample: DrumSample {
                        volume: 1456,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 344,
                        src_address: 34560472,
                        is_repeating: false,
                        base_pan: 20,
                    },
                    repeat_time: 12,
                },
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 1296,
                        timer_reload: 64516,
                        loop_pos: 2428,
                        length: 1096,
                        src_address: 34566220,
                        is_repeating: true,
                        base_pan: 20,
                    },
                    decay: DecayInstructions::exponential(240),
                    sustain: SustainInstructions {
                        volume: 41,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Exponential { duration: 5 },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 880,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 1500,
                        src_address: 34551292,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1056,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1728,
                        src_address: 34549548,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 968,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1644,
                        src_address: 34564560,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 30,
                            volume: 244,
                        },
                        TimedVolumeAdjustment {
                            time: 31,
                            volume: 61,
                        },
                        TimedVolumeAdjustment {
                            time: 32,
                            volume: 15,
                        },
                        TimedVolumeAdjustment {
                            time: 33,
                            volume: 4,
                        },
                        TimedVolumeAdjustment {
                            time: 34,
                            volume: 1,
                        },
                        TimedVolumeAdjustment {
                            time: 35,
                            volume: 1,
                        },
                    ],
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1376,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 3252,
                        src_address: 34569756,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
            ],
        ),
        RhythmSection::new(
            "Kitchen".to_string(),
            [
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1488,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1716,
                        src_address: 34600884,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1520,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 380,
                        src_address: 34602616,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1376,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1932,
                        src_address: 34603012,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1104,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2184,
                        src_address: 34591900,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1056,
                        timer_reload: 64171,
                        loop_pos: 4,
                        length: 1300,
                        src_address: 34598804,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1424,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1512,
                        src_address: 34590372,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1168,
                        timer_reload: 64624,
                        loop_pos: 4,
                        length: 2460,
                        src_address: 34594100,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1168,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2460,
                        src_address: 34594100,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1168,
                        timer_reload: 64387,
                        loop_pos: 4,
                        length: 2460,
                        src_address: 34594100,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1024,
                        timer_reload: 64717,
                        loop_pos: 4,
                        length: 5124,
                        src_address: 34585232,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 904,
                        timer_reload: 64519,
                        loop_pos: 1004,
                        length: 376,
                        src_address: 34604960,
                        is_repeating: true,
                        base_pan: 64,
                    },
                    decay: DecayInstructions::exponential(96),
                    sustain: SustainInstructions {
                        volume: 144,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Exponential { duration: 11 },
                },
                DrumInstructions::Decay {
                    sample: DrumSample {
                        volume: 808,
                        timer_reload: 64519,
                        loop_pos: 1004,
                        length: 376,
                        src_address: 34604960,
                        is_repeating: true,
                        base_pan: 64,
                    },
                    decay: DecayInstructions::exponential(38),
                    final_volume: 0,
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1536,
                        timer_reload: 64452,
                        loop_pos: 4,
                        length: 748,
                        src_address: 34600120,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1424,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2212,
                        src_address: 34596576,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
            ],
        ),
        RhythmSection::new(
            "Toy".to_string(),
            [
                DrumInstructions::Decay {
                    sample: DrumSample {
                        volume: 1104,
                        timer_reload: 64520,
                        loop_pos: 116,
                        length: 260,
                        src_address: 34682796,
                        base_pan: 55,
                        is_repeating: true,
                    },
                    decay: DecayInstructions {
                        duration: 240,
                        kind: DecayKind::Exponential,
                    },
                    final_volume: 2,
                },
                DrumInstructions::Decay {
                    sample: DrumSample {
                        volume: 1264,
                        timer_reload: 64520,
                        loop_pos: 116,
                        length: 260,
                        src_address: 34682796,
                        base_pan: 55,
                        is_repeating: true,
                    },
                    decay: DecayInstructions {
                        duration: 64,
                        kind: DecayKind::Exponential,
                    },
                    final_volume: 1,
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 832,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2056,
                        src_address: 34694492,
                        base_pan: 70,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 880,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1240,
                        src_address: 34690152,
                        base_pan: 20,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Dsr {
                    sample: DrumSample {
                        volume: 1024,
                        timer_reload: 64516,
                        loop_pos: 692,
                        length: 656,
                        src_address: 34697664,
                        base_pan: 80,
                        is_repeating: true,
                    },
                    decay: DecayInstructions {
                        duration: 240,
                        kind: DecayKind::Exponential,
                    },
                    sustain: SustainInstructions {
                        volume: 18,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Exponential { duration: 4 },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 936,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1700,
                        src_address: 34699024,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1120,
                        timer_reload: 64570,
                        loop_pos: 4,
                        length: 2648,
                        src_address: 34683184,
                        base_pan: 40,
                        is_repeating: false,
                    },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 1264,
                        timer_reload: 64387,
                        loop_pos: 4,
                        length: 1024,
                        src_address: 34685848,
                        is_repeating: false,
                        base_pan: 70,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 36,
                            volume: 316,
                        },
                        TimedVolumeAdjustment {
                            time: 37,
                            volume: 80,
                        },
                        TimedVolumeAdjustment {
                            time: 38,
                            volume: 20,
                        },
                    ],
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 904,
                        timer_reload: 64170,
                        loop_pos: 4,
                        length: 3248,
                        src_address: 34686888,
                        is_repeating: false,
                        base_pan: 100,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 24,
                            volume: 228,
                        },
                        TimedVolumeAdjustment {
                            time: 25,
                            volume: 57,
                        },
                        TimedVolumeAdjustment {
                            time: 26,
                            volume: 14,
                        },
                        TimedVolumeAdjustment {
                            time: 27,
                            volume: 4,
                        },
                        TimedVolumeAdjustment {
                            time: 28,
                            volume: 1,
                        },
                        TimedVolumeAdjustment {
                            time: 29,
                            volume: 1,
                        },
                    ],
                },
                DrumInstructions::Decay {
                    sample: DrumSample {
                        volume: 1424,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 3248,
                        src_address: 34686888,
                        base_pan: 90,
                        is_repeating: false,
                    },
                    decay: DecayInstructions {
                        duration: 77,
                        kind: DecayKind::Exponential,
                    },
                    final_volume: 364,
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1424,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2020,
                        src_address: 34692456,
                        base_pan: 35,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1168,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1032,
                        src_address: 34691408,
                        base_pan: 35,
                        is_repeating: false,
                    },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 880,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1084,
                        src_address: 34696564,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![TimedVolumeAdjustment {
                        time: 26,
                        volume: 220,
                    }],
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1328,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 3208,
                        src_address: 34679572,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
            ],
        ),
        RhythmSection::new(
            "Beat-Box".to_string(),
            [
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1056,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2360,
                        src_address: 34712444,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 968,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 972,
                        src_address: 34704488,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1056,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1224,
                        src_address: 34710220,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1024,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1344,
                        src_address: 34705476,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1680,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2640,
                        src_address: 34707564,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1376,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1308,
                        src_address: 34702128,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1000,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 712,
                        src_address: 34706836,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1296,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1020,
                        src_address: 34703452,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1616,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 968,
                        src_address: 34711460,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Sr {
                    sample: DrumSample {
                        volume: 744,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 5424,
                        src_address: 34716420,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    sustain: SustainInstructions {
                        volume: 744,
                        duration: 100,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 648,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 2436,
                        src_address: 34721860,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Sr {
                    sample: DrumSample {
                        volume: 648,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 416,
                        src_address: 34715988,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    sustain: SustainInstructions {
                        volume: 648,
                        duration: 8,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                },
                DrumInstructions::Sr {
                    sample: DrumSample {
                        volume: 880,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1216,
                        src_address: 34724312,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    sustain: SustainInstructions {
                        volume: 880,
                        duration: 24,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 1168,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1152,
                        src_address: 34714820,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 26,
                            volume: 3,
                        },
                        TimedVolumeAdjustment {
                            time: 27,
                            volume: 3,
                        },
                    ],
                },
            ],
        ),
        RhythmSection::new(
            "8-bit".to_string(),
            [
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 840,
                        timer_reload: 64717,
                        loop_pos: 4,
                        length: 2000,
                        src_address: 34583216,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 648,
                        timer_reload: 64854,
                        loop_pos: 4,
                        length: 824,
                        src_address: 34582376,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Multiple {
                    samples: vec![
                        TimedDrumSample {
                            time: 0,
                            sample: DrumSample {
                                volume: 656,
                                timer_reload: 64717,
                                loop_pos: 4,
                                length: 344,
                                src_address: 34574276,
                                base_pan: 64,
                                is_repeating: false,
                            },
                        },
                        TimedDrumSample {
                            time: 12,
                            sample: DrumSample {
                                volume: 624,
                                timer_reload: 64717,
                                loop_pos: 4,
                                length: 340,
                                src_address: 34574636,
                                base_pan: 64,
                                is_repeating: false,
                            },
                        },
                        TimedDrumSample {
                            time: 24,
                            sample: DrumSample {
                                volume: 600,
                                timer_reload: 64717,
                                loop_pos: 4,
                                length: 348,
                                src_address: 34574992,
                                base_pan: 64,
                                is_repeating: false,
                            },
                        },
                        TimedDrumSample {
                            time: 36,
                            sample: DrumSample {
                                volume: 584,
                                timer_reload: 64717,
                                loop_pos: 4,
                                length: 348,
                                src_address: 34574992,
                                base_pan: 64,
                                is_repeating: false,
                            },
                        },
                    ],
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 568,
                        timer_reload: 64512,
                        loop_pos: 4,
                        length: 1404,
                        src_address: 34575356,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![TimedVolumeAdjustment {
                        time: 34,
                        volume: 284,
                    }],
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 752,
                        timer_reload: 64717,
                        loop_pos: 4,
                        length: 2724,
                        src_address: 34576776,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::Noise {
                    sample: NoiseDrumSample {
                        volume: 484,
                        timer_reload: 61533,
                        base_pan: 64,
                    },
                    attack: AttackInstructions::Linear {
                        volume: 484,
                        duration: 0,
                    },
                    decay: DecayInstructions {
                        duration: 24,
                        kind: DecayKind::Exponential,
                    },
                    sustain: SustainInstructions {
                        volume: 66,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                },
                DrumInstructions::DsrPitchAdjust {
                    sample: DrumSample {
                        volume: 680,
                        timer_reload: 64934,
                        loop_pos: 88,
                        length: 168,
                        src_address: 34812920,
                        base_pan: 64,
                        is_repeating: true,
                    },
                    decay: DecayInstructions {
                        duration: 20,
                        kind: DecayKind::Exponential,
                    },
                    sustain: SustainInstructions {
                        volume: 76,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                    adjustments: vec![
                        TimedPitchAdjustment {
                            time: 2,
                            timer_reload: 64868,
                        },
                        TimedPitchAdjustment {
                            time: 4,
                            timer_reload: 64794,
                        },
                        TimedPitchAdjustment {
                            time: 6,
                            timer_reload: 64713,
                        },
                        TimedPitchAdjustment {
                            time: 8,
                            timer_reload: 64623,
                        },
                        TimedPitchAdjustment {
                            time: 10,
                            timer_reload: 64523,
                        },
                        TimedPitchAdjustment {
                            time: 12,
                            timer_reload: 64412,
                        },
                        TimedPitchAdjustment {
                            time: 14,
                            timer_reload: 64288,
                        },
                        TimedPitchAdjustment {
                            time: 16,
                            timer_reload: 64152,
                        },
                        TimedPitchAdjustment {
                            time: 18,
                            timer_reload: 64001,
                        },
                        TimedPitchAdjustment {
                            time: 20,
                            timer_reload: 63833,
                        },
                    ],
                },
                DrumInstructions::DsrPitchAdjust {
                    sample: DrumSample {
                        volume: 768,
                        timer_reload: 64732,
                        loop_pos: 88,
                        length: 168,
                        src_address: 34812920,
                        base_pan: 64,
                        is_repeating: true,
                    },
                    decay: DecayInstructions {
                        duration: 20,
                        kind: DecayKind::Exponential,
                    },
                    sustain: SustainInstructions {
                        volume: 86,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                    adjustments: vec![
                        TimedPitchAdjustment {
                            time: 2,
                            timer_reload: 64644,
                        },
                        TimedPitchAdjustment {
                            time: 4,
                            timer_reload: 64546,
                        },
                        TimedPitchAdjustment {
                            time: 6,
                            timer_reload: 64438,
                        },
                        TimedPitchAdjustment {
                            time: 8,
                            timer_reload: 64317,
                        },
                        TimedPitchAdjustment {
                            time: 10,
                            timer_reload: 64184,
                        },
                        TimedPitchAdjustment {
                            time: 12,
                            timer_reload: 64035,
                        },
                        TimedPitchAdjustment {
                            time: 14,
                            timer_reload: 63871,
                        },
                        TimedPitchAdjustment {
                            time: 16,
                            timer_reload: 63688,
                        },
                        TimedPitchAdjustment {
                            time: 18,
                            timer_reload: 63486,
                        },
                        TimedPitchAdjustment {
                            time: 20,
                            timer_reload: 63262,
                        },
                    ],
                },
                DrumInstructions::DsrPitchAdjust {
                    sample: DrumSample {
                        volume: 856,
                        timer_reload: 64463,
                        loop_pos: 88,
                        length: 168,
                        src_address: 34812920,
                        base_pan: 64,
                        is_repeating: true,
                    },
                    decay: DecayInstructions {
                        duration: 20,
                        kind: DecayKind::Exponential,
                    },
                    sustain: SustainInstructions {
                        volume: 96,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                    adjustments: vec![
                        TimedPitchAdjustment {
                            time: 2,
                            timer_reload: 64345,
                        },
                        TimedPitchAdjustment {
                            time: 4,
                            timer_reload: 64214,
                        },
                        TimedPitchAdjustment {
                            time: 6,
                            timer_reload: 64070,
                        },
                        TimedPitchAdjustment {
                            time: 8,
                            timer_reload: 63909,
                        },
                        TimedPitchAdjustment {
                            time: 10,
                            timer_reload: 63731,
                        },
                        TimedPitchAdjustment {
                            time: 12,
                            timer_reload: 63532,
                        },
                        TimedPitchAdjustment {
                            time: 14,
                            timer_reload: 63313,
                        },
                        TimedPitchAdjustment {
                            time: 16,
                            timer_reload: 63070,
                        },
                        TimedPitchAdjustment {
                            time: 18,
                            timer_reload: 62800,
                        },
                        TimedPitchAdjustment {
                            time: 20,
                            timer_reload: 62500,
                        },
                    ],
                },
                DrumInstructions::Noise {
                    sample: NoiseDrumSample {
                        volume: 176,
                        timer_reload: 65036,
                        base_pan: 64,
                    },
                    attack: AttackInstructions::Exact {
                        adjustments: vec![TimedVolumeAdjustment {
                            time: 1,
                            volume: 208,
                        }],
                    },
                    decay: DecayInstructions {
                        duration: 140,
                        kind: DecayKind::Exponential,
                    },
                    sustain: SustainInstructions {
                        volume: 10,
                        duration: 0,
                    },
                    release: ReleaseInstructions::Geometric { ratio: 0.5 },
                },
                DrumInstructions::Simple {
                    sample: DrumSample {
                        volume: 1056,
                        timer_reload: 64951,
                        loop_pos: 4,
                        length: 1932,
                        src_address: 34579516,
                        base_pan: 64,
                        is_repeating: false,
                    },
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 1376,
                        timer_reload: 64951,
                        loop_pos: 4,
                        length: 448,
                        src_address: 34573812,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 6,
                            volume: 668,
                        },
                        TimedVolumeAdjustment {
                            time: 7,
                            volume: 344,
                        },
                    ],
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 1616,
                        timer_reload: 64854,
                        loop_pos: 4,
                        length: 896,
                        src_address: 34581464,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 14,
                            volume: 808,
                        },
                        TimedVolumeAdjustment {
                            time: 15,
                            volume: 404,
                        },
                    ],
                },
                DrumInstructions::ExactVolumeAdjust {
                    sample: DrumSample {
                        volume: 1776,
                        timer_reload: 64717,
                        loop_pos: 4,
                        length: 772,
                        src_address: 34573024,
                        is_repeating: false,
                        base_pan: 64,
                    },
                    adjustments: vec![
                        TimedVolumeAdjustment {
                            time: 14,
                            volume: 896,
                        },
                        TimedVolumeAdjustment {
                            time: 15,
                            volume: 448,
                        },
                    ],
                },
            ],
        ),
    ]
}
