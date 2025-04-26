use std::collections::VecDeque;

use crate::audio::*;

const WAVE_12_5_BONG: u8 = 4; // Is correctly 0
const WAVE_12_5_BING: u8 = 7; // Is correctly 0
const WAVE_25_DING: u8 = 5; // Is correctly 1
const WAVE_25_TING: u8 = 5; // Is correctly 1
const WAVE_50_FAH: u8 = 6; // Is correctly 3
const WAVE_50_PONG: u8 = 2; // Is correctly 3
const WAVE_50_BLING: u8 = 6;

pub fn instrument_instructions() -> Vec<Instrument> {
    let cricket_adjust = |n: i32| -> f32 { n as f32 / 16.0 };
    let bird_adjust = |n: i32| -> f32 { n as f32 / 20.7 };
    let bing_adjust = |n: i32| -> f32 { n as f32 / 3.0 };

    let mut bird_pitches = Vec::new();
    for i in 0..70 {
        bird_pitches.push(TimedRelativePitchAdjustment {
            time: 190 + i as u32 * 2,
            pitch_adjust: -bird_adjust(i),
        })
    }

    let mut bird_pitches2 = Vec::new();
    for i in 0..70 {
        bird_pitches2.push(TimedRelativePitchAdjustment {
            time: 146 + i as u32 * 2,
            pitch_adjust: -bird_adjust(i),
        })
    }

    let mut bird_pitches3 = Vec::new();
    for i in 0..70 {
        bird_pitches3.push(TimedRelativePitchAdjustment {
            time: 138 + i as u32 * 2,
            pitch_adjust: -bird_adjust(i),
        })
    }

    vec![
        Instrument {
            name: "Piano".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1488, 1200), // (lowest vs highest note)
                    base_timer_reload: 63898,
                    loop_pos: 1968,
                    length: 548,
                    src_address: 34780940,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 192 }),
                sustain: Some(InstrumentSustain {
                    volume: (94, 4),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Organ".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1168, 944),
                    base_timer_reload: 63496,
                    loop_pos: 688,
                    length: 1136,
                    src_address: 34745660,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (1376, 1104), // TODO: Attack should be shorter for highest pitch
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (1392, 1120),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 286 }),
                sustain: Some(InstrumentSustain {
                    volume: (91, 74),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Harpsichord".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (872, 712),
                    base_timer_reload: 63922,
                    loop_pos: 1180,
                    length: 212,
                    src_address: 34728028,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 190 }),
                sustain: Some(InstrumentSustain {
                    volume: (14, 11),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Harmonica".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (4, 3),
                    base_timer_reload: 63516,
                    loop_pos: 1644,
                    length: 232,
                    src_address: 34741788,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (28, 23),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (96, 78),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (204, 168),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (332, 268),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (444, 360),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 6,
                            volume: (536, 436),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 7,
                            volume: (608, 492),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 8,
                            volume: (648, 528),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 9,
                            volume: (680, 552),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 10,
                            volume: (696, 568),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 11,
                            volume: (712, 576),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 12,
                            volume: (720, 584),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 13,
                            volume: (728, 592),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 22,
                            volume: (736, 600),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Linear { duration: 18 }),
                sustain: Some(InstrumentSustain {
                    volume: (568, 464),
                    duration: 142,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Flute".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (13, 11), // (lowest vs highest note)
                    base_timer_reload: 64174,
                    loop_pos: 1532,
                    length: 220,
                    src_address: 34751464,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (104, 84),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (316, 256),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (568, 464),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (784, 640),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (936, 760),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 6,
                            volume: (1008, 824),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 7,
                            volume: (1072, 872),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 8,
                            volume: (1104, 904),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 9,
                            volume: (1120, 912),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 18,
                            volume: (1136, 928),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Linear { duration: 176 }),
                sustain: Some(InstrumentSustain {
                    volume: (696, 468),
                    duration: 1,
                }),
                release: Some(InstrumentRelease::GeometricStopBlowing { ratio: 0.5 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Trumpet".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1024, 832),
                    base_timer_reload: 64178,
                    loop_pos: 1392,
                    length: 504,
                    src_address: 34773508,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Linear { duration: 7 }),
                sustain: Some(InstrumentSustain {
                    volume: (840, 688),
                    duration: 41,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Saxophone".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1056, 856),
                    base_timer_reload: 63803,
                    loop_pos: 2252,
                    length: 1380,
                    src_address: 34790612,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Linear { duration: 13 }),
                sustain: Some(InstrumentSustain {
                    volume: (936, 688),
                    duration: 82,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Wood Flute".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1456, 1264),
                    // wood flute timer reload is adjusted over time
                    base_timer_reload: 63488,
                    loop_pos: 1032,
                    length: 1384,
                    src_address: 34755776,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 176 }),
                sustain: Some(InstrumentSustain {
                    volume: (168, 144),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: vec![
                TimedRelativePitchAdjustment {
                    time: 6,
                    pitch_adjust: 1.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 8,
                    pitch_adjust: 2.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 10,
                    pitch_adjust: 3.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 12,
                    pitch_adjust: 4.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 14,
                    pitch_adjust: 5.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 16,
                    pitch_adjust: 4.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 18,
                    pitch_adjust: 3.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 20,
                    pitch_adjust: 2.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 22,
                    pitch_adjust: 1.0 / 5.25,
                },
                TimedRelativePitchAdjustment {
                    time: 24,
                    pitch_adjust: 0.0,
                },
            ],
        },
        Instrument {
            name: "Acoustic Guitar".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1360, 1104),
                    base_timer_reload: 63502,
                    loop_pos: 2388,
                    length: 84,
                    src_address: 34725544,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 192 }),
                sustain: Some(InstrumentSustain {
                    volume: (50, 40),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Electric Guitar".to_string(),
            instructions: InstrumentInstructions::Dual([
                Adsr {
                    sample: InstrumentSample::PCM16(Sample {
                        volume: (80, 65),
                        base_timer_reload: 61448,
                        loop_pos: 848,
                        length: 1600,
                        src_address: 34731424,
                        is_repeating: true,
                    }),
                    attack: Some(InstrumentAttack::Exact {
                        adjustments: vec![
                            InstrumentTimedVolumeAdjustment {
                                time: 1,
                                volume: (504, 408),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 2,
                                volume: (912, 744),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 3,
                                volume: (1120, 904),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 4,
                                volume: (1200, 968),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 5,
                                volume: (1216, 992),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 19,
                                volume: (1232, 1000),
                            },
                        ],
                    }),
                    decay: Some(InstrumentDecay::Exponential { duration: 365 }),
                    sustain: Some(InstrumentSustain {
                        volume: (122, 99),
                        duration: 0,
                    }),
                    release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
                },
                Adsr {
                    sample: InstrumentSample::PCM16(Sample {
                        volume: (45, 37),
                        base_timer_reload: 62808,
                        loop_pos: 848,
                        length: 1600,
                        src_address: 34731424,
                        is_repeating: true,
                    }),
                    attack: Some(InstrumentAttack::Exact {
                        adjustments: vec![
                            InstrumentTimedVolumeAdjustment {
                                time: 1,
                                volume: (284, 232),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 2,
                                volume: (520, 424),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 3,
                                volume: (632, 512),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 4,
                                volume: (680, 552),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 5,
                                volume: (696, 568),
                            },
                            InstrumentTimedVolumeAdjustment {
                                time: 9,
                                volume: (704, 568),
                            },
                        ],
                    }),
                    decay: Some(InstrumentDecay::Exponential { duration: 375 }),
                    sustain: Some(InstrumentSustain {
                        volume: (61, 50),
                        duration: 0,
                    }),
                    release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
                },
            ]),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Banjo".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1216, 776),
                    base_timer_reload: 63502,
                    loop_pos: 2116,
                    length: 420,
                    src_address: 34753228,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 192 }),
                sustain: Some(InstrumentSustain {
                    volume: (78, 20),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Bass".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Ranged(vec![
                RangedAdsr {
                    low: 0,
                    high: 18,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1648, 1200),
                            base_timer_reload: 63504,
                            loop_pos: 3536,
                            length: 420,
                            src_address: 34747496,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 92 }),
                        sustain: Some(InstrumentSustain {
                            volume: (78, 20),
                            duration: 0,
                        }),
                        release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
                    },
                },
                RangedAdsr {
                    low: 19,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1168, 1056),
                            base_timer_reload: 64170,
                            loop_pos: 4,
                            length: 2064,
                            src_address: 34758204,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
            ]),
        },
        Instrument {
            name: "Violin".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1200, 1056),
                    base_timer_reload: 63900,
                    loop_pos: 1684,
                    length: 1988,
                    src_address: 34786928,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::ExponentialWithVariableSustain {
                    duration: 186,
                    sustain_duration: (0, 40),
                }),
                sustain: Some(InstrumentSustain {
                    volume: (228, 152),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Miramba".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1360, 1104),
                    base_timer_reload: 63528,
                    loop_pos: 560,
                    length: 128,
                    src_address: 34741088,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 128 }),
                sustain: None,
                release: None,
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Vibraphone".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (760, 616),
                    base_timer_reload: 64519,
                    loop_pos: 808,
                    length: 84,
                    src_address: 34777232,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (1168, 944),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (1200, 984),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::ExponentialWithWobble {
                    duration: 240,
                    wobble_start: 10,
                    max_wobble: 0.375,
                    wave_duration: 30,
                }),
                sustain: Some(InstrumentSustain {
                    volume: (126, 106),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Timpani".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1616, 1312),
                    // timpani timer reload changes over time
                    base_timer_reload: 63494,
                    loop_pos: 1188,
                    length: 1160,
                    src_address: 34771148,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::ExponentialRising {
                    duration: 160,
                    rise_start: 35,
                    max_rise: 2.56,
                }),
                sustain: Some(InstrumentSustain {
                    volume: (51, 5),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Star Drop".to_string(),
            instructions: InstrumentInstructions::TimedMultiple(vec![
                (
                    0,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (992, 632),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (1008, 648),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (1024, 656),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (584, 372),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    24,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (560, 356),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (568, 364),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (576, 368),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (328, 208),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    48,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (960, 608),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (984, 624),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (992, 632),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (568, 360),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    72,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (536, 344),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (536, 352),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (560, 356),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (316, 204),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                // 4-8
                (
                    96,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (784, 504),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (808, 512),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (816, 520),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (464, 296),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    120,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (444, 284),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (452, 288),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (460, 292),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (260, 168),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    144,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (624, 400),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (640, 408),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (648, 412),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (368, 236),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    168,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (352, 224),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (360, 228),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (364, 232),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (208, 132),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                // 8-12
                (
                    192,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (484, 308),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (496, 316),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (504, 320),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (284, 184),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    216,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (272, 176),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (280, 180),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (284, 180),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (160, 103),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    240,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (364, 232),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (372, 236),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (376, 240),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (216, 136),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    264,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (204, 132),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (208, 136),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (212, 136),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (121, 77),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                // 12-16
                (
                    288,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (260, 168),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (268, 172),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (268, 172),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (152, 98),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    312,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (148, 94),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (152, 96),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (152, 97),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (86, 55),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    336,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (172, 110),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (176, 113),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (180, 114),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (101, 65),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    360,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (97, 62),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (99, 63),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (100, 64),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (84, 36),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                // 16-20
                (
                    384,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (104, 66),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (106, 68),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (107, 69),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (61, 39),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    408,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (58, 37),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (60, 38),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (60, 39),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (34, 22),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    432,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (51, 33),
                            base_timer_reload: 64450,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (53, 34),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (53, 34),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 5 }),
                        sustain: Some(InstrumentSustain {
                            volume: (30, 19),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
                (
                    456,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (29, 18),
                            base_timer_reload: 64993,
                            loop_pos: 104,
                            length: 344,
                            src_address: 34794256,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (30, 19),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (30, 19),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 4 }),
                        sustain: Some(InstrumentSustain {
                            volume: (17, 11),
                            duration: 6,
                        }),
                        release: Some(InstrumentRelease::Static((1, 1))),
                    },
                ),
            ]),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "UFO".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (9, 7),
                    // UFO adjusts timer reload when switching between notes, the bigger the note difference the more timer reload changes
                    base_timer_reload: 64450,
                    loop_pos: 104,
                    length: 344,
                    src_address: 34794256,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::ExactWithMagicPitch {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (9, 54),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (66, 176),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (216, 344),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (424, 504),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (624, 640),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (784, 728),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 6,
                            volume: (896, 776),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 7,
                            volume: (960, 816),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 8,
                            volume: (1000, 832),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 9,
                            volume: (1024, 856),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 10,
                            volume: (1056, 864),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 11,
                            volume: (1072, 872),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 21,
                            volume: (1040, 840),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 170 }),
                sustain: Some(InstrumentSustain {
                    volume: (11, 9),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Alien".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1472, 1200),
                    base_timer_reload: 63488,
                    loop_pos: 180,
                    length: 1324,
                    src_address: 34769632,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 190 }),
                sustain: Some(InstrumentSustain {
                    volume: (38, 31),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Robot".to_string(),
            instructions: InstrumentInstructions::Random(vec![
                Adsr {
                    sample: InstrumentSample::PCM16(Sample {
                        volume: (1472, 1200),
                        base_timer_reload: 63828,
                        loop_pos: 1288,
                        length: 200,
                        src_address: 34797716,
                        is_repeating: true,
                    }),
                    attack: None,
                    decay: Some(InstrumentDecay::Exponential { duration: 96 }),
                    sustain: Some(InstrumentSustain {
                        volume: (24, 20),
                        duration: 0,
                    }),
                    release: Some(InstrumentRelease::Basic),
                },
                Adsr {
                    sample: InstrumentSample::PCM16(Sample {
                        volume: (1440, 1200),
                        base_timer_reload: 63924,
                        loop_pos: 1288,
                        length: 200,
                        src_address: 34796216,
                        is_repeating: true,
                    }),
                    attack: None,
                    decay: Some(InstrumentDecay::Exponential { duration: 96 }),
                    sustain: Some(InstrumentSustain {
                        volume: (24, 20),
                        duration: 0,
                    }),
                    release: Some(InstrumentRelease::Basic),
                },
                Adsr {
                    sample: InstrumentSample::PCM16(Sample {
                        volume: (1472, 1200),
                        base_timer_reload: 63828,
                        loop_pos: 1296,
                        length: 200,
                        src_address: 34796220,
                        is_repeating: true,
                    }),
                    attack: None,
                    decay: Some(InstrumentDecay::Exponential { duration: 96 }),
                    sustain: Some(InstrumentSustain {
                        volume: (24, 20),
                        duration: 0,
                    }),
                    release: Some(InstrumentRelease::Basic),
                },
            ]),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Rocket".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1344, 1088),
                    base_timer_reload: 63502,
                    loop_pos: 3124,
                    length: 336,
                    src_address: 34760284,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 190 }),
                sustain: Some(InstrumentSustain {
                    volume: (12, 10),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.3 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Moon".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1200, 984),
                    base_timer_reload: 63488,
                    loop_pos: 4,
                    length: 1800,
                    src_address: 34775416,
                    is_repeating: false,
                }),
                // TODO: Some moon volumes have volume adjustments (at the end)...
                attack: None,
                decay: None,
                sustain: None,
                release: None,
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Green Dude".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (592, 480),
                    base_timer_reload: 63494,
                    loop_pos: 1252,
                    length: 584,
                    src_address: 34739240,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (1264, 1024),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (1376, 1104),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (1392, 1120),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 190 }),
                sustain: Some(InstrumentSustain {
                    volume: (38, 31),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Phone Dial".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1392, 1120),
                    base_timer_reload: 63488,
                    loop_pos: 52,
                    length: 176,
                    src_address: 34866764,
                    is_repeating: true,
                }),
                attack: None,
                decay: None,
                sustain: Some(InstrumentSustain {
                    volume: (1392, 1120),
                    duration: 20,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Cat".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1200, 968),
                    base_timer_reload: 63488,
                    loop_pos: 4,
                    length: 2176,
                    src_address: 34832476,
                    is_repeating: false,
                }),
                attack: None,
                decay: None,
                sustain: Some(InstrumentSustain {
                    volume: (1200, 968),
                    duration: 98,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Dog".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Ranged(vec![
                RangedAdsr {
                    low: 0,
                    high: 18,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1376, 1008),
                            base_timer_reload: 63488,
                            loop_pos: 4,
                            length: 1596,
                            src_address: 34834668,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 19,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (880, 776),
                            base_timer_reload: 64088,
                            loop_pos: 4,
                            length: 1644,
                            src_address: 34836280,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: Some(InstrumentSustain {
                            volume: (880, 776),
                            duration: 40,
                        }),
                        release: Some(InstrumentRelease::Geometric { ratio: 0.25 }),
                    },
                },
            ]),
        },
        Instrument {
            name: "Pig".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1376, 872),
                    base_timer_reload: 63488,
                    loop_pos: 4,
                    length: 1396,
                    src_address: 34853564,
                    is_repeating: false,
                }),
                attack: None,
                decay: None,
                sustain: None,
                release: None,
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Cricket".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Cricket(vec![
                StupidCricket {
                    low: 0,
                    high: 4,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (7, 7),
                            base_timer_reload: 64568,
                            loop_pos: 144,
                            length: 376,
                            src_address: 34858648,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (49, 49),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (168, 168),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 3,
                                    volume: (368, 0),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 4,
                                    volume: (608, 608),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 5,
                                    volume: (840, 840),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 6,
                                    volume: (1024, 1024),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 7,
                                    volume: (1184, 1184),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 8,
                                    volume: (1280, 1280),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 9,
                                    volume: (1360, 1360),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 10,
                                    volume: (1408, 1408),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 11,
                                    volume: (1440, 1440),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 12,
                                    volume: (1456, 1456),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 13,
                                    volume: (1472, 1472),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 14,
                                    volume: (1488, 1488),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 24,
                                    volume: (1504, 1504),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 30 }),
                        sustain: Some(InstrumentSustain {
                            volume: (5, 5),
                            duration: 0,
                        }),
                        release: Some(InstrumentRelease::Basic),
                    },
                    future_adsr: VecDeque::from(vec![(
                        68,
                        Adsr {
                            sample: InstrumentSample::PCM16(Sample {
                                volume: (3, 3),
                                base_timer_reload: 64568,
                                loop_pos: 144,
                                length: 376,
                                src_address: 34858648,
                                is_repeating: true,
                            }),
                            attack: Some(InstrumentAttack::Exact {
                                adjustments: vec![
                                    InstrumentTimedVolumeAdjustment {
                                        time: 1,
                                        volume: (20, 20),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 2,
                                        volume: (66, 66),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 3,
                                        volume: (148, 148),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 4,
                                        volume: (244, 244),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 5,
                                        volume: (336, 336),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 6,
                                        volume: (408, 408),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 7,
                                        volume: (468, 468),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 8,
                                        volume: (536, 536),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 9,
                                        volume: (560, 560),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 10,
                                        volume: (576, 576),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 11,
                                        volume: (584, 584),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 13,
                                        volume: (592, 592),
                                    },
                                    InstrumentTimedVolumeAdjustment {
                                        time: 22,
                                        volume: (600, 600),
                                    },
                                ],
                            }),
                            decay: Some(InstrumentDecay::Exponential { duration: 30 }),
                            sustain: Some(InstrumentSustain {
                                volume: (29, 29),
                                duration: 0,
                            }),
                            release: Some(InstrumentRelease::Basic),
                        },
                    )]),
                    pitch_adjustments: vec![
                        TimedRelativePitchAdjustment {
                            time: 2,
                            pitch_adjust: cricket_adjust(1),
                        },
                        TimedRelativePitchAdjustment {
                            time: 4,
                            pitch_adjust: cricket_adjust(2),
                        },
                        TimedRelativePitchAdjustment {
                            time: 6,
                            pitch_adjust: cricket_adjust(3),
                        },
                        TimedRelativePitchAdjustment {
                            time: 8,
                            pitch_adjust: cricket_adjust(4),
                        },
                        TimedRelativePitchAdjustment {
                            time: 10,
                            pitch_adjust: cricket_adjust(5),
                        },
                        TimedRelativePitchAdjustment {
                            time: 12,
                            pitch_adjust: cricket_adjust(6),
                        },
                        TimedRelativePitchAdjustment {
                            time: 14,
                            pitch_adjust: cricket_adjust(7),
                        },
                        TimedRelativePitchAdjustment {
                            time: 16,
                            pitch_adjust: cricket_adjust(8),
                        },
                        TimedRelativePitchAdjustment {
                            time: 18,
                            pitch_adjust: cricket_adjust(9),
                        },
                        TimedRelativePitchAdjustment {
                            time: 20,
                            pitch_adjust: cricket_adjust(10),
                        },
                        TimedRelativePitchAdjustment {
                            time: 22,
                            pitch_adjust: cricket_adjust(11),
                        },
                        TimedRelativePitchAdjustment {
                            time: 24,
                            pitch_adjust: cricket_adjust(12),
                        },
                        TimedRelativePitchAdjustment {
                            time: 32,
                            pitch_adjust: cricket_adjust(10),
                        },
                        TimedRelativePitchAdjustment {
                            time: 34,
                            pitch_adjust: cricket_adjust(9),
                        },
                        TimedRelativePitchAdjustment {
                            time: 36,
                            pitch_adjust: cricket_adjust(8),
                        },
                        TimedRelativePitchAdjustment {
                            time: 38,
                            pitch_adjust: cricket_adjust(6),
                        },
                        TimedRelativePitchAdjustment {
                            time: 40,
                            pitch_adjust: cricket_adjust(4),
                        },
                        TimedRelativePitchAdjustment {
                            time: 44,
                            pitch_adjust: cricket_adjust(2),
                        },
                        TimedRelativePitchAdjustment {
                            time: 46,
                            pitch_adjust: cricket_adjust(0),
                        },
                        TimedRelativePitchAdjustment {
                            time: 48,
                            pitch_adjust: cricket_adjust(-1),
                        },
                        TimedRelativePitchAdjustment {
                            time: 50,
                            pitch_adjust: cricket_adjust(-2),
                        },
                        TimedRelativePitchAdjustment {
                            time: 52,
                            pitch_adjust: cricket_adjust(-3),
                        },
                        TimedRelativePitchAdjustment {
                            time: 54,
                            pitch_adjust: cricket_adjust(-4),
                        },
                        TimedRelativePitchAdjustment {
                            time: 56,
                            pitch_adjust: cricket_adjust(-3),
                        },
                        TimedRelativePitchAdjustment {
                            time: 58,
                            pitch_adjust: cricket_adjust(-2),
                        },
                        TimedRelativePitchAdjustment {
                            time: 60,
                            pitch_adjust: cricket_adjust(-1),
                        },
                        TimedRelativePitchAdjustment {
                            time: 62,
                            pitch_adjust: cricket_adjust(0),
                        },
                    ],
                },
                StupidCricket {
                    low: 5,
                    high: 10,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (896, 696),
                            base_timer_reload: 63814,
                            loop_pos: 4,
                            length: 1096,
                            src_address: 34841624,
                            is_repeating: false,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (1376, 1056),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 3,
                                    volume: (1408, 1088),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 4,
                                    volume: (1424, 1104),
                                },
                            ],
                        }),
                        decay: None,
                        sustain: Some(InstrumentSustain {
                            volume: (1424, 1104),
                            duration: 48,
                        }),
                        release: None,
                    },
                    future_adsr: VecDeque::from(vec![
                        (
                            52,
                            Adsr {
                                sample: InstrumentSample::PCM16(Sample {
                                    volume: (896, 696),
                                    base_timer_reload: 63814,
                                    loop_pos: 4,
                                    length: 1096,
                                    src_address: 34841624,
                                    is_repeating: false,
                                }),
                                attack: Some(InstrumentAttack::Exact {
                                    adjustments: vec![
                                        InstrumentTimedVolumeAdjustment {
                                            time: 1,
                                            volume: (1376, 1056),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 3,
                                            volume: (1408, 1088),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 4,
                                            volume: (1424, 1104),
                                        },
                                    ],
                                }),
                                decay: None,
                                sustain: Some(InstrumentSustain {
                                    volume: (1424, 1104),
                                    duration: 48,
                                }),
                                release: None,
                            },
                        ),
                        (
                            104,
                            Adsr {
                                sample: InstrumentSample::PCM16(Sample {
                                    volume: (896, 696),
                                    base_timer_reload: 63814,
                                    loop_pos: 4,
                                    length: 1096,
                                    src_address: 34841624,
                                    is_repeating: false,
                                }),
                                attack: Some(InstrumentAttack::Exact {
                                    adjustments: vec![
                                        InstrumentTimedVolumeAdjustment {
                                            time: 1,
                                            volume: (1376, 1056),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 3,
                                            volume: (1408, 1088),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 4,
                                            volume: (1424, 1104),
                                        },
                                    ],
                                }),
                                decay: None,
                                sustain: Some(InstrumentSustain {
                                    volume: (1424, 1104),
                                    duration: 48,
                                }),
                                release: None,
                            },
                        ),
                    ]),
                    pitch_adjustments: vec![],
                },
                StupidCricket {
                    low: 11,
                    high: 16,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (728, 504),
                            base_timer_reload: 64319,
                            loop_pos: 4,
                            length: 1096,
                            src_address: 34841624,
                            is_repeating: false,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (1120, 768),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (1136, 784),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 3,
                                    volume: (1152, 800),
                                },
                            ],
                        }),
                        decay: None,
                        sustain: Some(InstrumentSustain {
                            volume: (1152, 800),
                            duration: 10,
                        }),
                        release: None,
                    },
                    future_adsr: VecDeque::from(vec![
                        (
                            14,
                            Adsr {
                                sample: InstrumentSample::PCM16(Sample {
                                    volume: (728, 504),
                                    base_timer_reload: 64319,
                                    loop_pos: 4,
                                    length: 1096,
                                    src_address: 34841624,
                                    is_repeating: false,
                                }),
                                attack: Some(InstrumentAttack::Exact {
                                    adjustments: vec![
                                        InstrumentTimedVolumeAdjustment {
                                            time: 1,
                                            volume: (1120, 768),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 2,
                                            volume: (1136, 784),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 3,
                                            volume: (1152, 800),
                                        },
                                    ],
                                }),
                                decay: None,
                                sustain: Some(InstrumentSustain {
                                    volume: (1152, 800),
                                    duration: 10,
                                }),
                                release: None,
                            },
                        ),
                        (
                            28,
                            Adsr {
                                sample: InstrumentSample::PCM16(Sample {
                                    volume: (728, 504),
                                    base_timer_reload: 64319,
                                    loop_pos: 4,
                                    length: 1096,
                                    src_address: 34841624,
                                    is_repeating: false,
                                }),
                                attack: Some(InstrumentAttack::Exact {
                                    adjustments: vec![
                                        InstrumentTimedVolumeAdjustment {
                                            time: 1,
                                            volume: (1120, 768),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 2,
                                            volume: (1136, 784),
                                        },
                                        InstrumentTimedVolumeAdjustment {
                                            time: 3,
                                            volume: (1152, 800),
                                        },
                                    ],
                                }),
                                decay: None,
                                sustain: Some(InstrumentSustain {
                                    volume: (1152, 800),
                                    duration: 10,
                                }),
                                release: None,
                            },
                        ),
                    ]),
                    pitch_adjustments: vec![],
                },
                StupidCricket {
                    low: 17,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (7, 6),
                            base_timer_reload: 64321,
                            loop_pos: 332,
                            length: 3328,
                            src_address: 34854976,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![
                                InstrumentTimedVolumeAdjustment {
                                    time: 1,
                                    volume: (45, 40),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 2,
                                    volume: (156, 140),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 3,
                                    volume: (344, 308),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 4,
                                    volume: (800, 504),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 5,
                                    volume: (984, 712),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 6,
                                    volume: (1120, 872),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 7,
                                    volume: (1232, 1000),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 8,
                                    volume: (1360, 1104),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 9,
                                    volume: (1392, 1168),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 10,
                                    volume: (1408, 1200),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 11,
                                    volume: (1424, 1232),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 12,
                                    volume: (1440, 1264),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 13,
                                    volume: (1456, 1280),
                                },
                                InstrumentTimedVolumeAdjustment {
                                    time: 23,
                                    volume: (1440, 1296),
                                },
                            ],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 260 }),
                        sustain: Some(InstrumentSustain {
                            volume: (800, 712),
                            duration: 1,
                        }),
                        release: Some(InstrumentRelease::Geometric { ratio: 0.7 }),
                    },
                    future_adsr: VecDeque::new(),
                    pitch_adjustments: vec![],
                },
            ]),
        },
        Instrument {
            name: "Frog".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1152, 936),
                    base_timer_reload: 63488,
                    loop_pos: 4,
                    length: 1608,
                    src_address: 34837940,
                    is_repeating: false,
                }),
                attack: None,
                decay: None,
                sustain: None,
                release: None,
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Yoshi".to_string(),
            instructions: InstrumentInstructions::TimedMultiple(vec![
                (
                    0,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1152, 936),
                            base_timer_reload: 64170,
                            loop_pos: 4,
                            length: 1684,
                            src_address: 34825516,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: Some(InstrumentRelease::Static((1152, 936))),
                    },
                ),
                (
                    72,
                    Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1152, 936),
                            base_timer_reload: 64170,
                            loop_pos: 4,
                            length: 2192,
                            src_address: 34827216,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                ),
            ]),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Birds".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Cricket(vec![
                StupidCricket {
                    low: 0,
                    high: 11,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1312, 1184),
                            base_timer_reload: 64088,
                            loop_pos: 4,
                            length: 3252,
                            src_address: 34843192,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: Some(InstrumentSustain {
                            volume: (1312, 1184),
                            duration: 96,
                        }),
                        release: Some(InstrumentRelease::Geometric { ratio: 0.25 }),
                    },
                    future_adsr: VecDeque::new(),
                    pitch_adjustments: Vec::new(),
                },
                StupidCricket {
                    low: 12,
                    high: 16,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1056, 1000),
                            base_timer_reload: 64110,
                            loop_pos: 5472,
                            length: 72,
                            src_address: 34848008,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![InstrumentTimedVolumeAdjustment {
                                time: 192,
                                volume: (1024, 992),
                            }],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 138 }),
                        sustain: Some(InstrumentSustain {
                            volume: (216, 196),
                            duration: 0,
                        }),
                        release: Some(InstrumentRelease::Geometric { ratio: 0.25 }),
                    },
                    future_adsr: VecDeque::new(),
                    pitch_adjustments: bird_pitches,
                },
                StupidCricket {
                    low: 17,
                    high: 21,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (984, 912),
                            base_timer_reload: 64468,
                            loop_pos: 5472,
                            length: 72,
                            src_address: 34848008,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![InstrumentTimedVolumeAdjustment {
                                time: 144,
                                volume: (904, 784),
                            }],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 138 }),
                        sustain: Some(InstrumentSustain {
                            volume: (160, 125),
                            duration: 0,
                        }),
                        release: Some(InstrumentRelease::Geometric { ratio: 0.25 }),
                    },
                    future_adsr: VecDeque::new(),
                    pitch_adjustments: bird_pitches2,
                },
                StupidCricket {
                    low: 22,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (904, 872),
                            base_timer_reload: 64736,
                            loop_pos: 5472,
                            length: 72,
                            src_address: 34848008,
                            is_repeating: true,
                        }),
                        attack: Some(InstrumentAttack::Exact {
                            adjustments: vec![InstrumentTimedVolumeAdjustment {
                                time: 144,
                                volume: (768, 752),
                            }],
                        }),
                        decay: Some(InstrumentDecay::Exponential { duration: 138 }),
                        sustain: Some(InstrumentSustain {
                            volume: (123, 119),
                            duration: 0,
                        }),
                        release: Some(InstrumentRelease::Geometric { ratio: 0.25 }),
                    },
                    future_adsr: VecDeque::new(),
                    pitch_adjustments: bird_pitches3,
                },
            ]),
        },
        Instrument {
            name: "Monkeys".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Ranged(vec![
                RangedAdsr {
                    low: 0,
                    high: 11,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1696, 1536),
                            base_timer_reload: 63488,
                            loop_pos: 4,
                            length: 2044,
                            src_address: 34839564,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 12,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1392, 1248),
                            base_timer_reload: 64002,
                            loop_pos: 4,
                            length: 1532,
                            src_address: 34846460,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
            ]),
        },
        Instrument {
            name: "Do-Re-Mi Voice".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Ranged(vec![
                RangedAdsr {
                    low: 0,
                    high: 4,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1408, 1312),
                            base_timer_reload: 64171,
                            loop_pos: 592,
                            length: 656,
                            src_address: 34882052,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 160 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 5,
                    high: 6,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1296, 1264),
                            base_timer_reload: 64171,
                            loop_pos: 464,
                            length: 772,
                            src_address: 34874708,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 157 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 7,
                    high: 8,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1248, 1216),
                            base_timer_reload: 64175,
                            loop_pos: 616,
                            length: 628,
                            src_address: 34877208,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 154 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 9,
                    high: 9,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1200, 1200),
                            base_timer_reload: 64177,
                            loop_pos: 472,
                            length: 692,
                            src_address: 34879712,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 154 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 10,
                    high: 11,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1184, 1152),
                            base_timer_reload: 64178,
                            loop_pos: 516,
                            length: 636,
                            src_address: 34880888,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 154 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 12,
                    high: 13,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1136, 1120),
                            base_timer_reload: 64179,
                            loop_pos: 608,
                            length: 628,
                            src_address: 34883312,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 154 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 14,
                    high: 15,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1104, 1072),
                            base_timer_reload: 64176,
                            loop_pos: 604,
                            length: 644,
                            src_address: 34872188,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 153 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 16,
                    high: 16,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1056, 1056),
                            base_timer_reload: 64179,
                            loop_pos: 608,
                            length: 640,
                            src_address: 34873448,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 152 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 17,
                    high: 18,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1040, 1008),
                            base_timer_reload: 64175,
                            loop_pos: 604,
                            length: 636,
                            src_address: 34875956,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 151 }),
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 19,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (992, 896),
                            base_timer_reload: 64173,
                            loop_pos: 596,
                            length: 640,
                            src_address: 34878464,
                            is_repeating: true,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 150 }),
                        sustain: None,
                        release: None,
                    },
                },
            ]),
        },
        Instrument {
            name: "Wah Dude".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1504, 960),
                    base_timer_reload: 63488,
                    loop_pos: 964,
                    length: 1828,
                    src_address: 34778136,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 190 }),
                sustain: Some(InstrumentSustain {
                    volume: (96, 61),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Opera Man".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1328, 960),
                    base_timer_reload: 63472,
                    loop_pos: 444,
                    length: 1528,
                    src_address: 34743676,
                    is_repeating: true,
                }),
                attack: None,
                decay: Some(InstrumentDecay::Exponential { duration: 192 }),
                sustain: Some(InstrumentSustain {
                    volume: (66, 42),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Soul Girl".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (95, 77),
                    base_timer_reload: 63494,
                    loop_pos: 1592,
                    length: 1316,
                    src_address: 34733884,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (600, 484),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (1088, 880),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (1328, 1072),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (1424, 1152),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (1456, 1184),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 9,
                            volume: (1472, 1200),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 276 }),
                sustain: Some(InstrumentSustain {
                    volume: (21, 17),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.25 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Baby".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1232, 1040),
                    base_timer_reload: 63488,
                    loop_pos: 4,
                    length: 3036,
                    src_address: 34829424,
                    is_repeating: false,
                }),
                attack: None,
                decay: None,
                sustain: Some(InstrumentSustain {
                    volume: (1232, 1040),
                    duration: 142,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.5 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Laughing Men".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Ranged(vec![
                RangedAdsr {
                    low: 0,
                    high: 5,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1456, 1376),
                            base_timer_reload: 64004,
                            loop_pos: 4,
                            length: 2116,
                            src_address: 34816204,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 6,
                    high: 11,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1536, 1456),
                            base_timer_reload: 64171,
                            loop_pos: 4,
                            length: 2508,
                            src_address: 34813680,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 12,
                    high: 17,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1440, 1360),
                            base_timer_reload: 64171,
                            loop_pos: 4,
                            length: 3360,
                            src_address: 34822140,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 18,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1456, 1360),
                            base_timer_reload: 64171,
                            loop_pos: 4,
                            length: 3788,
                            src_address: 34818336,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
            ]),
        },
        Instrument {
            name: "Kung-Fu Men".to_string(),
            pitch_adjustments: Vec::new(),
            instructions: InstrumentInstructions::Ranged(vec![
                RangedAdsr {
                    low: 0,
                    high: 4,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1536, 1440),
                            base_timer_reload: 64194,
                            loop_pos: 4,
                            length: 2592,
                            src_address: 34808020,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 5,
                    high: 9,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1424, 1328),
                            base_timer_reload: 64194,
                            loop_pos: 4,
                            length: 2592,
                            src_address: 34805412,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 10,
                    high: 14,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1536, 1440),
                            base_timer_reload: 64194,
                            loop_pos: 4,
                            length: 2276,
                            src_address: 34810628,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 15,
                    high: 19,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1536, 1440),
                            base_timer_reload: 64194,
                            loop_pos: 4,
                            length: 2552,
                            src_address: 34800728,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
                RangedAdsr {
                    low: 20,
                    high: 24,
                    adsr: Adsr {
                        sample: InstrumentSample::PCM16(Sample {
                            volume: (1536, 1440),
                            base_timer_reload: 64194,
                            loop_pos: 4,
                            length: 2100,
                            src_address: 34803296,
                            is_repeating: false,
                        }),
                        attack: None,
                        decay: None,
                        sustain: None,
                        release: None,
                    },
                },
            ]),
        },
        Instrument {
            name: "Humming".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (7, 6),
                    base_timer_reload: 63492,
                    loop_pos: 1128,
                    length: 1296,
                    src_address: 34736804,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (50, 40),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (168, 136),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (360, 292),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (584, 476),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (800, 648),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 6,
                            volume: (960, 776),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 7,
                            volume: (1088, 880),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 8,
                            volume: (1168, 944),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 9,
                            volume: (1232, 1000),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 10,
                            volume: (1264, 1024),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 11,
                            volume: (1264, 1024),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 12,
                            volume: (1296, 1056),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 13,
                            volume: (1312, 1056),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 15,
                            volume: (1328, 1072),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 24,
                            volume: (1344, 1088),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 264 }),
                sustain: Some(InstrumentSustain {
                    volume: (204, 168),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Geometric { ratio: 0.25 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Ding-Ding".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PSG(ProgrammableSample {
                    volume: (464, 376),
                    base_timer_reload: 54852,
                    // Actual value = table_index: 1,
                    table_index: WAVE_25_DING,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (476, 384),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (480, 392),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 40 }),
                sustain: Some(InstrumentSustain {
                    volume: (36, 30),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::ExponentialUntil {
                    duration: 720,
                    until: 9,
                }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Pong-Pong".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PSG(ProgrammableSample {
                    volume: (340, 340),
                    base_timer_reload: 54852,
                    table_index: WAVE_50_PONG,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (400, 400),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (404, 404),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 26 }),
                sustain: Some(InstrumentSustain {
                    volume: (40, 30),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::ExponentialUntil {
                    duration: 560,
                    until: 5,
                }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Fah-Fah".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PSG(ProgrammableSample {
                    volume: (1, 1),
                    base_timer_reload: 54852,
                    table_index: WAVE_50_FAH,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (3, 3),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (11, 11),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (25, 25),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (49, 49),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (81, 81),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (117, 117),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 6,
                            volume: (152, 152),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 7,
                            volume: (188, 188),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 8,
                            volume: (220, 220),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 9,
                            volume: (248, 248),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 10,
                            volume: (272, 272),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 11,
                            volume: (288, 288),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 12,
                            volume: (308, 308),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 13,
                            volume: (316, 316),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 14,
                            volume: (324, 324),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 15,
                            volume: (332, 332),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 16,
                            volume: (336, 336),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 17,
                            volume: (340, 340),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 18,
                            volume: (344, 344),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 19,
                            volume: (348, 348),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 20,
                            volume: (352, 352),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 35,
                            volume: (356, 356),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 203 }),
                sustain: Some(InstrumentSustain {
                    volume: (43, 43),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::GeometricStopBlowing { ratio: 0.1 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Bong-Bong".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PSG(ProgrammableSample {
                    volume: (600, 484),
                    base_timer_reload: 22792,
                    // Actual value = table_index: 0,
                    table_index: WAVE_12_5_BONG,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (608, 496),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (616, 504),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 30 }),
                sustain: Some(InstrumentSustain {
                    volume: (72, 72),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::ExponentialUntil {
                    duration: 640,
                    until: 12,
                }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Bing-Bing".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PSG(ProgrammableSample {
                    volume: (76, 76),
                    base_timer_reload: 41548,
                    table_index: WAVE_12_5_BING,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (288, 288),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (376, 376),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (396, 396),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (400, 400),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (404, 404),
                        },
                    ],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 46 }),
                sustain: Some(InstrumentSustain {
                    volume: (58, 58),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::ExponentialUntil {
                    duration: 464,
                    until: 3,
                }),
            }),
            pitch_adjustments: vec![
                TimedRelativePitchAdjustment {
                    time: 1,
                    pitch_adjust: bing_adjust(1),
                },
                TimedRelativePitchAdjustment {
                    time: 2,
                    pitch_adjust: bing_adjust(2),
                },
                TimedRelativePitchAdjustment {
                    time: 3,
                    pitch_adjust: bing_adjust(3),
                },
                TimedRelativePitchAdjustment {
                    time: 4,
                    pitch_adjust: bing_adjust(4),
                },
                TimedRelativePitchAdjustment {
                    time: 5,
                    pitch_adjust: bing_adjust(5),
                },
                TimedRelativePitchAdjustment {
                    time: 6,
                    pitch_adjust: bing_adjust(6),
                },
            ],
        },
        Instrument {
            name: "Ting-Ting".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PSG(ProgrammableSample {
                    volume: (344, 344),
                    base_timer_reload: 62868,
                    // Actual value: table_index: 1,
                    table_index: WAVE_25_TING,
                    //table_index: 7,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![
                        InstrumentTimedVolumeAdjustment {
                            time: 1,
                            volume: (352, 352),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 2,
                            volume: (356, 356),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 3,
                            volume: (344, 344),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 4,
                            volume: (336, 336),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 5,
                            volume: (328, 328),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 6,
                            volume: (264, 264),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 7,
                            volume: (256, 256),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 8,
                            volume: (188, 188),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 9,
                            volume: (184, 184),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 10,
                            volume: (128, 128),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 11,
                            volume: (126, 126),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 12,
                            volume: (73, 73),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 13,
                            volume: (71, 71),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 14,
                            volume: (105, 105),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 15,
                            volume: (103, 103),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 16,
                            volume: (156, 156),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 17,
                            volume: (152, 152),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 18,
                            volume: (119, 119),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 19,
                            volume: (117, 117),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 20,
                            volume: (93, 93),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 21,
                            volume: (90, 90),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 22,
                            volume: (72, 72),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 23,
                            volume: (70, 70),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 24,
                            volume: (55, 55),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 25,
                            volume: (54, 54),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 26,
                            volume: (44, 44),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 27,
                            volume: (43, 43),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 28,
                            volume: (35, 35),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 29,
                            volume: (34, 34),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 30,
                            volume: (29, 29),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 31,
                            volume: (28, 28),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 32,
                            volume: (44, 44),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 33,
                            volume: (43, 43),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 34,
                            volume: (64, 64),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 35,
                            volume: (63, 63),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 36,
                            volume: (95, 95),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 37,
                            volume: (93, 93),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 38,
                            volume: (73, 73),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 39,
                            volume: (71, 71),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 40,
                            volume: (56, 56),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 41,
                            volume: (55, 55),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 42,
                            volume: (44, 44),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 43,
                            volume: (43, 43),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 44,
                            volume: (34, 34),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 45,
                            volume: (33, 33),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 46,
                            volume: (27, 27),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 47,
                            volume: (26, 26),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 48,
                            volume: (21, 21),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 50,
                            volume: (18, 18),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 51,
                            volume: (17, 17),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 52,
                            volume: (27, 27),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 53,
                            volume: (26, 26),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 54,
                            volume: (39, 39),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 55,
                            volume: (38, 38),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 56,
                            volume: (58, 58),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 57,
                            volume: (56, 56),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 58,
                            volume: (45, 45),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 59,
                            volume: (43, 43),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 60,
                            volume: (34, 34),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 62,
                            volume: (27, 27),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 63,
                            volume: (26, 26),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 64,
                            volume: (21, 21),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 65,
                            volume: (20, 20),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 66,
                            volume: (16, 16),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 68,
                            volume: (13, 13),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 70,
                            volume: (11, 11),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 72,
                            volume: (17, 17),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 73,
                            volume: (16, 16),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 74,
                            volume: (24, 24),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 75,
                            volume: (23, 23),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 76,
                            volume: (35, 35),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 77,
                            volume: (34, 34),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 78,
                            volume: (27, 27),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 80,
                            volume: (21, 21),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 81,
                            volume: (20, 20),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 82,
                            volume: (16, 16),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 84,
                            volume: (13, 13),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 85,
                            volume: (12, 12),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 86,
                            volume: (10, 10),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 88,
                            volume: (8, 8),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 90,
                            volume: (7, 7),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 91,
                            volume: (6, 6),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 92,
                            volume: (10, 10),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 94,
                            volume: (15, 15),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 95,
                            volume: (14, 14),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 96,
                            volume: (21, 21),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 98,
                            volume: (17, 17),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 99,
                            volume: (16, 16),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 100,
                            volume: (13, 13),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 102,
                            volume: (10, 10),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 104,
                            volume: (8, 8),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 106,
                            volume: (6, 6),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 108,
                            volume: (5, 5),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 110,
                            volume: (4, 4),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 112,
                            volume: (6, 6),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 114,
                            volume: (9, 9),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 116,
                            volume: (13, 13),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 118,
                            volume: (10, 10),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 120,
                            volume: (8, 8),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 122,
                            volume: (6, 6),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 124,
                            volume: (5, 5),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 126,
                            volume: (4, 4),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 128,
                            volume: (3, 3),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 130,
                            volume: (2, 2),
                        },
                        InstrumentTimedVolumeAdjustment {
                            time: 132,
                            volume: (2, 2),
                        },
                    ],
                }),
                decay: None,
                sustain: None,
                release: Some(InstrumentRelease::Geometric { ratio: 0.1 }),
            }),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Bling-Bling".to_string(),
            instructions: InstrumentInstructions::TimedMultiple(vec![
                (
                    0,
                    Adsr {
                        sample: InstrumentSample::PSG(ProgrammableSample {
                            volume: (404, 404),
                            base_timer_reload: 60196,
                            table_index: WAVE_50_BLING,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 10 }),
                        sustain: Some(InstrumentSustain {
                            duration: 1,
                            volume: (85, 85),
                        }),
                        release: None,
                    },
                ),
                (
                    12,
                    Adsr {
                        sample: InstrumentSample::PSG(ProgrammableSample {
                            volume: (160, 160),
                            base_timer_reload: 62868,
                            table_index: WAVE_50_BLING,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 26 }),
                        sustain: Some(InstrumentSustain {
                            duration: 1,
                            volume: (22, 22),
                        }),
                        release: None,
                    },
                ),
                (
                    44,
                    Adsr {
                        sample: InstrumentSample::PSG(ProgrammableSample {
                            volume: (63, 63),
                            base_timer_reload: 62868,
                            table_index: WAVE_50_BLING,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 28 }),
                        sustain: Some(InstrumentSustain {
                            duration: 1,
                            volume: (8, 8),
                        }),
                        release: None,
                    },
                ),
                (
                    76,
                    Adsr {
                        sample: InstrumentSample::PSG(ProgrammableSample {
                            volume: (22, 22),
                            base_timer_reload: 62868,
                            table_index: WAVE_50_BLING,
                        }),
                        attack: None,
                        decay: Some(InstrumentDecay::Exponential { duration: 28 }),
                        sustain: Some(InstrumentSustain {
                            duration: 1,
                            volume: (8, 8),
                        }),
                        release: None,
                    },
                ),
            ]),
            pitch_adjustments: Vec::new(),
        },
        Instrument {
            name: "Boon-Boon".to_string(),
            instructions: InstrumentInstructions::Adsr(Adsr {
                sample: InstrumentSample::PCM16(Sample {
                    volume: (1616, 1616),
                    base_timer_reload: 64454,
                    loop_pos: 164,
                    length: 316,
                    src_address: 34813188,
                    is_repeating: true,
                }),
                attack: Some(InstrumentAttack::Exact {
                    adjustments: vec![InstrumentTimedVolumeAdjustment {
                        time: 36,
                        volume: (1360, 1360),
                    }],
                }),
                decay: Some(InstrumentDecay::Exponential { duration: 36 }),
                sustain: Some(InstrumentSustain {
                    volume: (1, 1),
                    duration: 0,
                }),
                release: Some(InstrumentRelease::Basic),
            }),
            pitch_adjustments: Vec::new(),
        },
    ]
}
