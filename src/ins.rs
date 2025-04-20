use crate::split::*;

pub fn instrument_instructions() -> Vec<Instrument> {
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
    ]
}
