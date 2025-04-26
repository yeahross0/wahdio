use crate::audio::{QueuedDrum, QueuedNote, NOTE_RATE, SAMPLE_RATE, TRACK_LENGTH};

const TRACK_COUNT: usize = 4;
const SIMULTANEOUS_DRUMS: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum Repeats {
    None,
    Once,
    Endless,
}

impl Repeats {
    pub fn exact(self) -> Option<usize> {
        match self {
            Repeats::None => Some(0),
            Repeats::Once => Some(1),
            Repeats::Endless => None,
        }
    }

    pub fn loop_times(self) -> Option<usize> {
        match self {
            Repeats::None => Some(1),
            Repeats::Once => Some(2),
            Repeats::Endless => None,
        }
    }
}

pub struct Record {
    pub notes: Vec<QueuedNote>,
    pub drums: Vec<QueuedDrum>,
    pub repeats: Repeats,
    pub phrase_count: usize,
    pub note_rate: usize,
    pub swing_offset: Option<usize>,
}

impl Record {
    pub fn from_mio(mio_data: &[u8]) -> Record {
        match mio_data.len() {
            8192 => Self::from_record(mio_data),
            65536 => Self::from_game(mio_data),
            _ => panic!("Unknown mio size"),
        }
    }

    fn from_game(mio_data: &[u8]) -> Self {
        const BASE_SONG_OFFSET: usize = 0xB961;
        const BASE_INSTRUMENT_OFFSET: usize = 0xBA6B;
        const BASE_VOLUME_OFFSET: usize = 0xBA61;
        const BASE_PAN_OFFSET: usize = 0xBA66;
        const BASE_DRUM_OFFSET: usize = 0xB9E1;

        let drum_volume = mio_data[BASE_VOLUME_OFFSET + 4];
        let drum_pan = mio_data[BASE_PAN_OFFSET + 4];

        let drum_set = (mio_data[0xBA6F] & 0x7) as usize;

        let repeats = match mio_data[0xE605] {
            0 => Repeats::None,
            1 => Repeats::Once,
            _ => Repeats::Endless,
        };

        let mut queued_drums = Vec::new();

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
                    let drum_id = 13 - drum_used as usize;
                    queued_drums.push(QueuedDrum {
                        time: i as u32,
                        volume_multiplier,
                        pan_addition,
                        section: drum_set,
                        id: drum_id,
                        pretend_track: 4 + drum_index as u8,
                    })
                }
            }
        }

        let mut queued_notes = vec![];

        for track_index in 0..TRACK_COUNT {
            let song_offset = BASE_SONG_OFFSET + track_index * TRACK_LENGTH;

            let instrument_used = mio_data[BASE_INSTRUMENT_OFFSET + track_index];
            let instrument_volume = mio_data[BASE_VOLUME_OFFSET + track_index];
            let instrument_pan = mio_data[BASE_PAN_OFFSET + track_index];

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
                        time: i as u32,
                        instrument: instrument_used as u32,
                        note: note,
                        track: track_index as u8,
                        volume_multiplier,
                        pan_addition,
                    });
                }
            }
        }

        Record {
            notes: queued_notes,
            drums: queued_drums,
            repeats,
            phrase_count: 1,
            note_rate: NOTE_RATE,
            swing_offset: None,
        }
    }

    fn from_record(mio_data: &[u8]) -> Self {
        const END_INDEX: usize = 0x102;
        const BASE_INSTRUMENT_OFFSET: usize = 0x211;
        const SEGMENT_LENGTH: usize = 0x114;
        let segment_offset = |n: usize| n * SEGMENT_LENGTH;
        const BASE_SONG_OFFSET: usize = 0x107;
        const BASE_VOLUME_OFFSET: usize = 0x207;
        const BASE_PAN_OFFSET: usize = 0x20C;
        const BASE_DRUM_OFFSET: usize = 0x187;
        const BASE_DRUMSET_OFFSET: usize = 0x211;
        const TEMPO_OFFSET: usize = 0x101;
        const SWING_OFFSET: usize = 0x100;

        let tempo: u32 = mio_data[TEMPO_OFFSET] as u32 * 10 + 60;
        let is_swing = mio_data[SWING_OFFSET] != 0;
        let tempo_rate = 120.0 / tempo as f32;
        let adjusted_rate = tempo_rate / 8.0;
        println!("TEMPO: {:?}, {}, {}", tempo, tempo_rate, adjusted_rate);

        let note_rate = ((SAMPLE_RATE as f32) * adjusted_rate) as usize;
        let swing_jump: usize = note_rate / 3;

        let swing_offset = if is_swing { Some(swing_jump) } else { None };

        let mut queued_drums = Vec::new();

        for segment_index in 0..mio_data[END_INDEX] as usize {
            let drum_volume = mio_data[BASE_VOLUME_OFFSET + segment_offset(segment_index) + 4];
            let drum_pan = mio_data[BASE_PAN_OFFSET + segment_offset(segment_index) + 4];

            let set_offset: usize = BASE_DRUMSET_OFFSET + segment_offset(segment_index) + 4;
            let drum_set = (mio_data[set_offset] & 0x7) as usize;
            for i in 0..TRACK_LENGTH {
                for drum_index in 0..SIMULTANEOUS_DRUMS {
                    let drum_used = mio_data[BASE_DRUM_OFFSET
                        + segment_offset(segment_index)
                        + i
                        + drum_index * TRACK_LENGTH];
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

                        let drum_id = 13 - drum_used as usize;
                        queued_drums.push(QueuedDrum {
                            time: (32 * segment_index) as u32 + i as u32,
                            volume_multiplier,
                            pan_addition,
                            section: drum_set,
                            id: drum_id,
                            pretend_track: 4 + drum_index as u8,
                        })
                    }
                }
            }
        }

        let mut queued_notes = vec![];

        for segment_index in 0..mio_data[END_INDEX] as usize {
            for track_index in 0..TRACK_COUNT {
                let song_offset =
                    BASE_SONG_OFFSET + segment_offset(segment_index) + track_index * TRACK_LENGTH;

                let instrument_used =
                    mio_data[BASE_INSTRUMENT_OFFSET + segment_offset(segment_index) + track_index];
                let instrument_volume =
                    mio_data[BASE_VOLUME_OFFSET + segment_offset(segment_index) + track_index];
                let instrument_pan =
                    mio_data[BASE_PAN_OFFSET + segment_offset(segment_index) + track_index];

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
                            time: (32 * segment_index) as u32 + i as u32,
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

        println!("NOTE RATE: {}", note_rate);

        Record {
            notes: queued_notes,
            drums: queued_drums,
            repeats: Repeats::None,
            phrase_count: mio_data[END_INDEX] as usize,
            note_rate,
            swing_offset,
        }
    }
}
