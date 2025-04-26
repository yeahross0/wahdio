/*
    Copyright 2016-2024 melonDS team

    This file is adapted from melonDS.

    melonDS is free software: you can redistribute it and/or modify it under
    the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    melonDS is distributed in the hope that it will be useful, but WITHOUT ANY
    WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
    FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with melonDS. If not, see http://www.gnu.org/licenses/.
*/

use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

// WARNING: SOME OF THESE VALUES ARE INCORRECT FOR CONVENIENCE
// to add an echo originally added in code
const PSG_TABLE: [[i16; 8]; 8] = [
    // Bong, Bing
    [
        -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF,
    ],
    // Ding, Ting
    [
        -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
    ],
    //[
    //    -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
    //],
    // Pong Special
    [-29490, -22937, -26214, -26214, 29490, 22937, 26214, 26214],
    // Pong, Fah, Bling[0]
    [
        -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
    ],
    //[
    //    -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
    //],
    //[
    //    -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF,
    //],
    //[
    //    -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FF,
    //],
    // Bong Special
    [
        -22936, -29490, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 19660,
    ],
    // Ding Special
    //[
    //    -29490, -0x7FFF, -26213, -26213, -0x7FFF, -0x7FFF, 22936, 26213,
    //],
    [-29490, -32767, -26214, -26214, -32767, -32767, 22937, 26214],
    //[-26214, -32767, -19660, -19660, -32767, -32767, 13107, 19660],
    // Pong+Fah Special
    ////[-26213, -29490, -29490, -0x7FFF, 22936, 26213, 26213, 29490],
    //[-22936, -26213, -26213, -0x7FFF, 19660, 22936, 22936, 26213],
    //[-26214, -13107, -19660, -19660, 26214, 13107, 19660, 19660],
    //[-29490, -22937, -26214, -26214, 29490, 22937, 26214, 26214],
    [-27033, -27033, -31948, -31948, 27033, 27033, 31948, 31948],
    // Ting -> Now Bing
    //[
    //    -22936, -0x7FFF, -19660, -19660, -0x7FFF, -0x7FFF, 19660, 22936,
    //],
    [
        -0x7FFF, -22936, -0x7FFF, -29490, -0x7FFF, -0x7FFF, -0x7FFF, 19660,
    ],
];

// THESE ARE THE ORIGINAL CORRECT VALUES

//const s16 SPUChannel::PSGTable[8][8] =
//        {
//            {-0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF},
//            {-0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF},
//            {-0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF},
//            {-0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF},
//            {-0x7FFF, -0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF},
//            {-0x7FFF, -0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF},
//            {-0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF, 0x7FFF},
//            {-0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF, -0x7FFF}};

const PREPROGRAMMED_TABLE: [[i16; 8]; 6] = [
    // Ding-Ding
    [
        -17383, -28490, -32000, -0x7FFF, -0x7FFF, -0x7FFF, 17021, 27851,
    ],
    // Pong-Pong
    [-16383, -28490, -32000, -0x7FFF, 16383, 28490, 32000, 0x7FFF],
    // Fah-Fah
    [-16383, -28490, -32000, -0x7FFF, 16383, 28490, 32000, 0x7FFF],
    // Bong-Bong
    [
        -16383, -28490, -32000, -0x7FFF, -0x7FFF, -0x7FFF, -32000, 0x7FFF,
    ],
    // Bing-Bing
    [
        -16383, -28490, -32000, -0x7FFF, -0x7FFF, -0x7FFF, -32000, 31000,
    ],
    // Ting-Ting
    [
        -30000, -0x7FFF, -0x7FFF, -24383, -30000, -0x7FFF, -32000, 31000,
    ],
];

const ADPCM_TABLE: [u16; 89] = [
    0x0007, 0x0008, 0x0009, 0x000A, 0x000B, 0x000C, 0x000D, 0x000E, 0x0010, 0x0011, 0x0013, 0x0015,
    0x0017, 0x0019, 0x001C, 0x001F, 0x0022, 0x0025, 0x0029, 0x002D, 0x0032, 0x0037, 0x003C, 0x0042,
    0x0049, 0x0050, 0x0058, 0x0061, 0x006B, 0x0076, 0x0082, 0x008F, 0x009D, 0x00AD, 0x00BE, 0x00D1,
    0x00E6, 0x00FD, 0x0117, 0x0133, 0x0151, 0x0173, 0x0198, 0x01C1, 0x01EE, 0x0220, 0x0256, 0x0292,
    0x02D4, 0x031C, 0x036C, 0x03C3, 0x0424, 0x048E, 0x0502, 0x0583, 0x0610, 0x06AB, 0x0756, 0x0812,
    0x08E0, 0x09C3, 0x0ABD, 0x0BD0, 0x0CFF, 0x0E4C, 0x0FBA, 0x114C, 0x1307, 0x14EE, 0x1706, 0x1954,
    0x1BDC, 0x1EA5, 0x21B6, 0x2515, 0x28CA, 0x2CDF, 0x315B, 0x364B, 0x3BB9, 0x41B2, 0x4844, 0x4F7E,
    0x5771, 0x602F, 0x69CE, 0x7462, 0x7FFF,
];

const ADPCM_INDEX_TABLE: [i8; 8] = [-1, -1, -1, -1, 2, 4, 6, 8];

pub enum AudioBitDepth {
    _10bit,
    _16bit,
}

pub struct Nds {
    ram: Vec<u8>,
}

const MAIN_RAM_MAX_SIZE: usize = 0x1000000;

impl Nds {
    pub fn new(ram: Vec<u8>) -> Self {
        Self { ram }
    }

    fn arm7_read32(&self, mut address: usize) -> u32 {
        let mut address = address as u32;
        //address &= !0x3;
        // TODO: Make safe

        // Faking this

        //let address_region = address & 0xFF800000;
        //assert!(address_region == 0x02000000 || address_region == 0x02800000);
        let main_ram_mask = 0x3FFFFF;

        match address & 0xFF800000 {
            0x02000000 | 0x02800000 => {
                unsafe {
                    // MainRAMMask?
                    //println!("read address: {}", address & main_ram_mask);

                    let ptr = std::mem::transmute::<*const u8, *const u32>(
                        self.ram.as_ptr().add((address & main_ram_mask) as usize),
                    );
                    *ptr
                }
            }
            _ => unimplemented!(),
        }
    }

    fn arm7_write32(&mut self, address: usize, val: u32) {
        let mut address = address as u32;
        address &= !0x3;

        let main_ram_mask = 0x3FFFFF;

        match address & 0xFF800000 {
            0x02000000 | 0x02800000 => {
                unsafe {
                    // MainRAMMask?
                    //println!("write address: {}", address & main_ram_mask);

                    let ptr = std::mem::transmute::<*mut u8, *mut u32>(
                        self.ram
                            .as_mut_ptr()
                            .add((address & main_ram_mask) as usize),
                    );
                    *ptr = val;
                }
            }
            _ => unimplemented!(),
        }
    }
}

const SPU_FIFO_SIZE: usize = 8;

pub struct SpuChannel {
    num: usize,
    //nds: Nds
    pub control: u32,
    src_address: usize,
    timer_reload: u16,
    loop_pos: usize,
    pub length: usize,
    pub restarts: usize,
    pub tick: usize,
    pub big_offset: usize,

    volume: u8,
    volume_shift: u8,
    pan: u8,

    pub key_on: bool,
    timer: u32,
    pos: i32,
    current_sample: i16,
    noise_val: u16,

    adpcm_val: i32,
    adpcm_index: i32,
    adpcm_val_loop: i32,
    adpcm_index_loop: i32,
    adpcm_current_byte: u8,

    fifo_data: [u32; SPU_FIFO_SIZE],
    fifo_read_pos: usize,
    fifo_write_pos: usize,
    fifo_read_offset: usize,
    fifo_level: usize,

    nds: Arc<Mutex<Nds>>,
}

impl SpuChannel {
    fn new(num: usize, nds: Arc<Mutex<Nds>>) -> SpuChannel {
        SpuChannel {
            num,
            control: 0,
            src_address: 0,
            timer_reload: 0,
            loop_pos: 0,
            length: 0,
            tick: 0,
            restarts: 0,
            big_offset: 0,

            volume: 0,
            volume_shift: 0,
            pan: 0,

            key_on: false,
            timer: 0,
            pos: 0,
            current_sample: 0,
            noise_val: 0,

            adpcm_val: 0,
            adpcm_index: 0,
            adpcm_val_loop: 0,
            adpcm_index_loop: 0,
            adpcm_current_byte: 0,

            fifo_data: [0; SPU_FIFO_SIZE],
            fifo_read_pos: 0,
            fifo_write_pos: 0,
            fifo_read_offset: 0,
            fifo_level: 0,

            nds,
        }
    }

    fn reset(&mut self) {
        self.key_on = false;

        self.set_control(0);
        self.src_address = 0;
        self.timer_reload = 0;
        self.loop_pos = 0;
        self.length = 0;

        self.timer = 0;

        self.pos = 0;
        self.fifo_read_pos = 0;
        self.fifo_write_pos = 0;
        self.fifo_read_offset = 0;
        self.fifo_level = 0;
    }

    // DoSaveState

    fn fifo_buffer_data(&mut self) {
        let total_len = self.loop_pos + self.length;

        if self.fifo_read_offset >= total_len {
            let repeat_mode = self.control >> 27 & 0x3;

            if repeat_mode & 1 != 0 {
                self.fifo_read_offset = self.loop_pos;
            } else if repeat_mode & 2 != 0 {
                // one-shot sound, we're done
                //println!("EARLY buf return");
                return;
            }
        }

        let mut burst_len = 16;
        if (self.fifo_read_offset + 16) > total_len {
            burst_len = total_len - self.fifo_read_offset;
        }

        // sound DMA can't read from the ARM7 BIOS
        if (self.src_address + self.fifo_read_offset) >= 0x00004000 {
            for _ in (0..burst_len).step_by(4) {
                self.fifo_data[self.fifo_write_pos] = self
                    .nds
                    .lock()
                    .unwrap()
                    .arm7_read32(self.src_address + self.fifo_read_offset);
                //println!("nowww {}", self.fifo_data[self.fifo_write_pos]);
                self.fifo_read_offset += 4;
                self.fifo_write_pos += 1;
                self.fifo_write_pos &= 0x7;
            }
        } else {
            for _ in (0..burst_len).step_by(4) {
                println!("   WARNING: IN FAKE ZONE");
                self.fifo_data[self.fifo_write_pos] = 0;
                self.fifo_read_offset += 4;
                self.fifo_write_pos += 1;
                self.fifo_write_pos &= 0x7;
            }
        }

        self.fifo_level += burst_len;
    }

    fn fifo_read_data<T: Copy + Display>(&mut self) -> T {
        // TODO: Replace with safe code
        let ret: T = unsafe {
            let ptr = std::mem::transmute::<*const u32, *const u8>(self.fifo_data.as_ptr())
                .add(self.fifo_read_pos);
            *std::mem::transmute::<*const u8, *const T>(ptr)
        };

        let type_size = size_of::<T>();

        self.fifo_read_pos += type_size;
        self.fifo_read_pos &= 0x1F;
        self.fifo_level -= type_size;

        if self.fifo_level <= 16 {
            self.fifo_buffer_data();
        }

        //println!("Read: {}", ret);

        ret
    }

    pub fn adjust_volume(&mut self, multiplier: f32) {
        self.volume = (self.volume as f32 * multiplier) as u8;
    }

    fn set_control(&mut self, val: u32) {
        let old_control: u32 = self.control;
        self.control = val & 0xFF7F837F;

        self.volume = (self.control & 0x7F) as u8;
        //println!("VOL: {}", self.volume);
        if self.volume == 127 {
            self.volume += 1;
        }

        let vol_shift: [u8; 4] = [4, 3, 2, 0];
        self.volume_shift = vol_shift[((self.control >> 8) & 0x3) as usize];
        //println!("VOLsh: {}", self.volume_shift);

        self.pan = ((self.control >> 16) & 0x7F) as u8;
        //println!("PAN: {}", self.pan);
        if self.pan == 127 {
            self.pan += 1;
        }

        if (val & (1 << 31)) != 0 && (old_control & (1 << 31)) == 0 {
            self.key_on = true;

            println!(
                "{} Set Cnt: Volume: {}, VolumeShift: {}, Pan: {}, KeyOn: {}",
                self.num, self.volume, self.volume_shift, self.pan, self.key_on,
            );
        }
    }

    fn set_src_address(&mut self, val: u32) {
        self.src_address = (val & 0x07FFFFFC) as usize;
    }

    fn set_timer_reload(&mut self, val: u32) {
        self.timer_reload = (val & 0xFFFF) as u16;
    }

    fn set_loop_pos(&mut self, val: u32) {
        self.loop_pos = ((val & 0xFFFF) << 2) as usize;
    }

    fn set_length(&mut self, val: u32) {
        self.length = ((val & 0x001FFFFF) << 2) as usize;
    }

    fn start(&mut self) {
        self.timer = self.timer_reload as u32;

        if ((self.control >> 29) & 0x3) == 3 {
            self.pos = -1;
        } else {
            self.pos = -3;
        }

        self.noise_val = 0x7FFF;
        self.current_sample = 0;

        self.fifo_read_pos = 0;
        self.fifo_write_pos = 0;
        self.fifo_read_offset = 0;
        self.fifo_level = 0;

        self.tick = 0;
        self.restarts += 1;

        //println!("BEFORE BUFFER");

        // when starting a channel, buffer data
        if ((self.control >> 29) & 0x3) != 3 {
            //println!("NOW BUFFER");
            self.fifo_buffer_data();
            self.fifo_buffer_data();
        }
    }

    fn next_sample_pcm8(&mut self) {
        self.pos += 1;
        if self.pos < 0 {
            return;
        }
        if self.pos >= (self.loop_pos + self.length) as i32 {
            let repeat = (self.control >> 27) & 0x3;
            if repeat & 1 != 0 {
                self.pos = self.loop_pos as i32;
            } else if repeat & 2 != 0 {
                self.current_sample = 0;
                self.control &= !(1 << 31);
                return;
            }
        }

        let val = self.fifo_read_data::<i8>();
        self.current_sample = (val as i16) << 8;
    }

    fn next_sample_pcm16(&mut self) {
        self.pos += 1;
        if self.pos < 0 {
            return;
        }
        if (self.pos << 1) >= (self.loop_pos + self.length) as i32 {
            let repeat = (self.control >> 27) & 0x3;
            if repeat & 1 != 0 {
                self.pos = (self.loop_pos >> 1) as i32;
            } else if repeat & 2 != 0 {
                self.current_sample = 0;
                self.control &= !(1 << 31);
                return;
            }
        }

        let val = self.fifo_read_data::<i16>();
        self.current_sample = val;

        //println!(
        //    "{} CURSAMP: {}, {}",
        //    self.num, self.current_sample, self.src_address
        //);

        //if self.current_sample != 0 {
        //    assert!([-3712,].contains(&self.current_sample));
        //}
    }

    fn next_sample_adpcm(&mut self) {
        //println!("UP HERE");
        self.pos += 1;
        //println!("self.pos: {}", self.pos);
        if self.pos < 8 {
            if self.pos == 0 {
                // setup ADPCM
                let header = self.fifo_read_data::<u32>();
                //println!("{}: header: {}, {}", self.num, header, self.src_address);
                self.adpcm_val = (header & 0xFFFF) as i16 as i32;
                self.adpcm_index = ((header >> 16) & 0x7F) as i32;
                if self.adpcm_index > 88 {
                    self.adpcm_index = 88;
                }
                self.adpcm_val_loop = self.adpcm_val;
                self.adpcm_index_loop = self.adpcm_index;
            }

            return;
        }

        if (self.pos >> 1) >= (self.loop_pos + self.length) as i32 {
            let repeat = (self.control >> 27) & 0x3;
            if (repeat & 1) != 0 {
                self.pos = (self.loop_pos << 1) as i32;
                self.adpcm_val = self.adpcm_val_loop;
                self.adpcm_index = self.adpcm_index_loop;
                self.adpcm_current_byte = self.fifo_read_data::<u8>();
            } else if (repeat & 2) != 0 {
                self.current_sample = 0;
                self.control &= !(1 << 31);
                return;
            }
        } else {
            if (self.pos & 0x1) == 0 {
                self.adpcm_current_byte = self.fifo_read_data::<u8>();
            } else {
                self.adpcm_current_byte >>= 4;
            }

            let val = ADPCM_TABLE[self.adpcm_index as usize];
            let mut diff = val >> 3;
            if (self.adpcm_current_byte & 0x1) != 0 {
                diff += val >> 2;
            }
            if (self.adpcm_current_byte & 0x2) != 0 {
                diff += val >> 1;
            }
            if (self.adpcm_current_byte & 0x4) != 0 {
                diff += val;
            }

            if (self.adpcm_current_byte & 0x8) != 0 {
                self.adpcm_val -= diff as i32;
                if self.adpcm_val < -0x7FFF {
                    self.adpcm_val = -0x7FFF;
                }
            } else {
                self.adpcm_val += diff as i32;
                if self.adpcm_val > 0x7FFF {
                    self.adpcm_val = 0x7FFF;
                }
            }

            self.adpcm_index += ADPCM_INDEX_TABLE[(self.adpcm_current_byte & 0x7) as usize] as i32;
            if self.adpcm_index < 0 {
                self.adpcm_index = 0;
            } else if self.adpcm_index > 88 {
                self.adpcm_index = 88;
            }

            if self.pos == (self.loop_pos << 1) as i32 {
                self.adpcm_val_loop = self.adpcm_val;
                self.adpcm_index_loop = self.adpcm_index;
            }
        }

        //println!("MADE IT HERE");

        //println!("samp: {}", self.adpcm_val);

        self.current_sample = self.adpcm_val as i16;
    }

    fn next_sample_psg(&mut self) {
        self.pos += 1;
        let wave = ((self.control >> 24) & 0x7) as usize;
        let index = (self.pos & 0x7) as usize;
        //println!(
        //    "SAMP: {} <-> {}, {}",
        //    PSG_TABLE[wave][index],
        //    self.control >> 24,
        //    self.pos
        //);
        self.current_sample = PSG_TABLE[wave][index];
    }

    fn next_sample_noise(&mut self) {
        if (self.noise_val & 0x1) != 0 {
            self.noise_val = (self.noise_val >> 1) ^ 0x6000;
            self.current_sample = -0x7FFF;
        } else {
            self.noise_val >>= 1;
            self.current_sample = 0x7FFF;
        }
    }

    fn run(&mut self, run_type: u32) -> i32 {
        //println!("{} CHANNEL CNT {}", self.num, self.control);
        if (self.control & (1 << 31)) == 0 {
            //println!("EARLY RETURN 1");
            return 0;
        }

        //println!(
        //    "RUN TYPE: {}, ll: {}",
        //    run_type,
        //    self.length + self.loop_pos
        //);

        if run_type < 3 && (self.length + self.loop_pos) < 16 {
            //println!("EARLY RETURN 1");
            return 0;
        }

        if self.key_on {
            self.start();
            self.key_on = false;

            //println!("KEYED ON:");
        }

        //println!("TIMERB: {}", self.timer);
        self.timer += 512; // 1 sample = 512 cycles at 16MHz

        //println!("TIMER BUSINESS: {}", self.timer >> 16);

        while (self.timer >> 16) != 0 {
            self.timer = self.timer_reload as u32 + (self.timer - 0x10000);

            match run_type {
                0 => self.next_sample_pcm8(),
                1 => self.next_sample_pcm16(),
                2 => self.next_sample_adpcm(),
                3 => self.next_sample_psg(),
                4 => self.next_sample_noise(),
                _ => unreachable!(),
            }
        }

        //println!("VAL: vol {:?}, {}", self.volume << self.volume_shift, self.current_sample);

        self.current_adjusted_sample()
    }

    fn current_adjusted_sample(&self) -> i32 {
        self.adjust_sample(self.current_sample)
    }

    fn adjust_sample(&self, sample: i16) -> i32 {
        let mut val = sample as i32;
        val <<= self.volume_shift;
        val = val.wrapping_mul(self.volume as i32);

        val
    }

    fn do_run(&mut self) -> i32 {
        self.tick += 1;
        self.big_offset += 1;
        //println!("cont {}", (self.control >> 29) & 0x3);
        match (self.control >> 29) & 0x3 {
            c @ 0..=2 => self.run(c),
            3 => {
                if self.num >= 14 {
                    self.run(4)
                } else if self.num >= 8 {
                    self.run(3)
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    fn pan_output(&self, input: i32, left: &mut i32, right: &mut i32) {
        *left += ((input as i64 * (128 - self.pan as i64)) >> 10) as i32;
        *right += ((input as i64 * self.pan as i64) >> 10) as i32;
    }
}

struct SpuCaptureUnit {
    num: usize,
    nds: Arc<Mutex<Nds>>,

    control: u8,
    dest_address: usize,
    timer_reload: u16,
    length: usize,

    timer: u32,
    pos: i32,

    fifo_data: [u32; 4],
    fifo_read_pos: usize,
    fifo_write_pos: usize,
    fifo_write_offset: usize,
    fifo_level: usize,
}

impl SpuCaptureUnit {
    fn new(num: usize, nds: Arc<Mutex<Nds>>) -> Self {
        Self {
            num,
            nds: nds.clone(),

            control: 0,
            dest_address: 0,
            timer_reload: 0,
            length: 0,

            timer: 0,
            pos: 0,

            fifo_data: [0, 0, 0, 0],
            fifo_read_pos: 0,
            fifo_write_pos: 0,
            fifo_write_offset: 0,
            fifo_level: 0,
        }
    }

    fn reset(&mut self) {
        self.set_control(0);
        self.dest_address = 0;
        self.timer_reload = 0;
        self.length = 0;

        self.timer = 0;

        self.pos = 0;
        self.fifo_read_pos = 0;
        self.fifo_write_pos = 0;
        self.fifo_write_offset = 0;
        self.fifo_level = 0;
    }

    // DoSavestate

    fn fifo_flush_data(&mut self) {
        for i in 0..4 {
            self.nds.lock().unwrap().arm7_write32(
                self.dest_address + self.fifo_write_offset,
                self.fifo_data[self.fifo_read_pos],
            );

            //println!(
            //    "{} FIFOWRITE: {} !! {} = {}",
            //    self.num,
            //    self.fifo_read_pos,
            //    (self.dest_address + self.fifo_write_offset) & 0x3FFFFF,
            //    self.fifo_data[self.fifo_read_pos]
            //);

            //assert!(
            //    [
            //        0, 4051697664, 4051759488, 243207807, 249499359, 4045410015, 4045467936,
            //        249557280, 249499359
            //    ]
            //    .contains(&self.fifo_data[self.fifo_read_pos])
            //);

            self.fifo_read_pos += 1;
            self.fifo_read_pos &= 0x3;
            self.fifo_level -= 4;

            self.fifo_write_offset += 4;
            if self.fifo_write_offset >= self.length {
                self.fifo_write_offset = 0;
                break;
            }
        }
    }

    fn fifo_write_data<T: Copy + Display>(&mut self, val: T) {
        // TODO: Rewrite as safe
        unsafe {
            let ptr = std::mem::transmute::<*mut u32, *mut u8>(self.fifo_data.as_mut_ptr())
                .add(self.fifo_write_pos);
            *std::mem::transmute::<*mut u8, *mut T>(ptr) = val;
            //println!("{} write: {} = {}", self.num, self.fifo_write_pos, val);
        }

        self.fifo_write_pos += size_of::<T>();
        self.fifo_write_pos &= 0xF;
        self.fifo_level += size_of::<T>();

        if self.fifo_level >= 16 {
            self.fifo_flush_data();
        }
    }

    fn set_control(&mut self, mut val: u8) {
        if (val & 0x80) != 0 && (self.control & 0x80) == 0 {
            self.start();
        }

        val &= 0x8F;
        if (val & 0x80) == 0 {
            val &= !0x01;
        }
        self.control = val;
    }

    fn set_dest_address(&mut self, val: u32) {
        self.dest_address = (val & 0x07FFFFFC) as usize;
    }

    fn set_timer_reload(&mut self, val: u32) {
        self.timer_reload = (val & 0xFFFF) as u16;
    }

    fn set_length(&mut self, val: u32) {
        self.length = (val << 2) as usize;
        if self.length == 0 {
            self.length = 4;
        }
    }

    fn start(&mut self) {
        self.timer = self.timer_reload as u32;
        self.pos = 0;
        self.fifo_read_pos = 0;
        self.fifo_write_pos = 0;
        self.fifo_write_offset = 0;
        self.fifo_level = 0;
    }

    fn run(&mut self, sample: i32) {
        self.timer += 512;

        if (self.control & 0x08) != 0 {
            while (self.timer >> 16) != 0 {
                self.timer = self.timer_reload as u32 + (self.timer - 0x10000);

                self.fifo_write_data::<i8>((sample >> 8) as i8);
                self.pos += 1;
                if self.pos >= self.length as i32 {
                    if self.fifo_level >= 4 {
                        self.fifo_flush_data();
                    }

                    if (self.control & 0x04) != 0 {
                        self.control &= 0x7F;
                        return;
                    } else {
                        self.pos = 0;
                    }
                }
            }
        } else {
            while (self.timer >> 16) != 0 {
                self.timer = self.timer_reload as u32 + (self.timer - 0x10000);

                self.fifo_write_data::<i16>(sample as i16);
                //println!("SAMPLE @@ {}", sample);
                //assert!([0, -3712, 3711, 3807, -3808, 3839, -3840].contains(&sample));
                self.pos += 2;
                if self.pos >= self.length as i32 {
                    if self.fifo_level >= 4 {
                        self.fifo_flush_data();
                    }

                    if (self.control & 0x04) != 0 {
                        self.control &= 0x7F;
                    } else {
                        self.pos = 0;
                    }
                }
            }
        }
    }
}

const SPU_OUTPUT_BUFFER_SIZE: usize = 2 * 2048;
pub struct Spu {
    nds: Arc<Mutex<Nds>>,
    bit_depth: AudioBitDepth,
    output_back_buffer: [i16; 2 * SPU_OUTPUT_BUFFER_SIZE],
    pub output_back_buffer_write_position: usize,

    output_front_buffer: [i16; 2 * SPU_OUTPUT_BUFFER_SIZE],
    output_front_buffer_write_position: usize,
    output_front_buffer_read_position: usize,

    // Platform::Mutex *AudioLock;
    control: u16,
    master_volume: u8,
    bias: u16,
    apply_bias: bool,

    pub channels: [SpuChannel; 16],
    capture: [SpuCaptureUnit; 2],
}

impl Spu {
    pub fn new(nds: Arc<Mutex<Nds>>, depth: AudioBitDepth) -> Self {
        Self {
            nds: nds.clone(),
            bit_depth: depth,

            output_back_buffer: [0; 2 * SPU_OUTPUT_BUFFER_SIZE],
            output_back_buffer_write_position: 0,

            output_front_buffer: [0; 2 * SPU_OUTPUT_BUFFER_SIZE],
            output_front_buffer_write_position: 0,
            output_front_buffer_read_position: 0,

            control: 0,
            master_volume: 0,
            bias: 0,
            apply_bias: false,

            channels: [
                SpuChannel::new(0, nds.clone()),
                SpuChannel::new(1, nds.clone()),
                SpuChannel::new(2, nds.clone()),
                SpuChannel::new(3, nds.clone()),
                SpuChannel::new(4, nds.clone()),
                SpuChannel::new(5, nds.clone()),
                SpuChannel::new(6, nds.clone()),
                SpuChannel::new(7, nds.clone()),
                SpuChannel::new(8, nds.clone()),
                SpuChannel::new(9, nds.clone()),
                SpuChannel::new(10, nds.clone()),
                SpuChannel::new(11, nds.clone()),
                SpuChannel::new(12, nds.clone()),
                SpuChannel::new(13, nds.clone()),
                SpuChannel::new(14, nds.clone()),
                SpuChannel::new(15, nds.clone()),
            ],

            capture: [
                SpuCaptureUnit::new(0, nds.clone()),
                SpuCaptureUnit::new(1, nds),
                //SpuCaptureUnit::new(0, nds),
                //SpuCaptureUnit::new(1, nds),
            ],
        }
    }

    fn reset(&mut self) {
        self.init_output();

        self.control = 0;
        self.master_volume = 0;
        self.bias = 0;

        for channel in &mut self.channels {
            channel.reset();
        }

        self.capture[0].reset();
        self.capture[1].reset();
    }

    fn stop(&mut self) {
        self.output_back_buffer_write_position = 0;
        self.output_front_buffer_read_position = 0;
        self.output_front_buffer_write_position = 0;
    }

    // DoSaveState

    pub fn mix(&mut self, dummy: u32) -> (i16, i16) {
        let mut left = 0;
        let mut right = 0;
        let mut left_output = 0;
        let mut right_output = 0;

        // TODO: Temp
        //self.control = 47487;
        //self.channels[0].control = 1346371942;
        //self.channels[1].control = 2818572415;
        //self.channels[2].control = 1344602743;
        //self.channels[3].control = 2826895487;
        //self.channels[4].control = 3493855311;
        //self.channels[5].control = 1346371650;
        //self.channels[6].control = 1346371941;
        //self.channels[7].control = 1212154625;
        //self.channels[8].control = 1631585042;
        //self.channels[9].control = 1631585033;
        //self.channels[10].control = 1665139522;
        //self.channels[11].control = 1665139522;
        //self.channels[12].control = 1665139457;
        //self.channels[13].control = 1212154625;
        //self.channels[14].control = 1614807809;
        //self.channels[15].control = 1614807809;
        //println!("{}", self.control);
        if (self.control & (1 << 15)) != 0 && dummy != 0 {
            let ch0 = self.channels[0].do_run();
            let ch1 = self.channels[1].do_run();
            let ch2 = self.channels[2].do_run();
            let ch3 = self.channels[3].do_run();

            // println!("chs {}, {}, {}, {}", ch0, ch1, ch2, ch3);
            if ch1 != 0 {
                //assert_eq!(ch1, -7602176);
            }

            self.channels[0].pan_output(ch0, &mut left, &mut right);
            self.channels[2].pan_output(ch2, &mut left, &mut right);

            if (self.control & (1 << 12)) == 0 {
                self.channels[1].pan_output(ch1, &mut left, &mut right);
            }
            if (self.control & (1 << 13)) == 0 {
                self.channels[3].pan_output(ch3, &mut left, &mut right);
            }

            for i in 4..16 {
                let chan = &mut self.channels[i];

                let channel = chan.do_run();

                if i == 4 {
                    //println!("4 Dorun: {}", channel);
                }
                if i == 8 {
                    //println!("8 Dorun: {}", channel);
                    if channel != 0 {
                        //assert!(
                        //    [
                        //        -15203888, 0, 15203888, 15597092, -15597092, 15728160, -15728160,
                        //        14810684, -14810684, 15334956, -15334956, 15072820, -15072820
                        //    ]
                        //    .contains(&channel)
                        //);
                    }
                }
                chan.pan_output(channel, &mut left, &mut right);
            }

            // sound capture

            if self.capture[0].control & (1 << 7) != 0 {
                let mut val = left;

                val >>= 8;
                if val < -0x8000 {
                    val = -0x8000;
                } else if val > 0x7FFF {
                    val = 0x7FFF;
                }

                self.capture[0].run(val);
            }
            if self.capture[1].control & (1 << 7) != 0 {
                let mut val = right;

                val >>= 8;
                if val < -0x8000 {
                    val = -0x8000;
                } else if val > 0x7FFF {
                    val = 0x7FFF;
                }

                self.capture[1].run(val);
            }

            // final output

            match self.control & 0x0300 {
                0x0000 => {
                    // left mixer
                    left_output = left;
                }
                0x0100 => {
                    // channel 1
                    let pan: i32 = 128 - self.channels[1].pan as i32;
                    left_output = ((ch1 as i64 * pan as i64) >> 10) as i32;
                }
                0x0200 => {
                    // channel 3
                    let pan: i32 = 128 - self.channels[3].pan as i32;
                    left_output = ((ch3 as i64 * pan as i64) >> 10) as i32;
                }
                0x0300 => {
                    let pan1 = 128 - self.channels[1].pan as i32;
                    let pan3 = 128 - self.channels[3].pan as i32;
                    left_output = ((ch1 as i64 * pan1 as i64) >> 10) as i32
                        + ((ch3 as i64 * pan3 as i64) >> 10) as i32;
                }
                _ => {}
            }

            //println!("{}", self.control);
            match self.control & 0x0C00 {
                0x0000 => {
                    // right mixer
                    right_output = right;
                }
                0x0400 => {
                    // channel 1
                    let pan: i32 = self.channels[1].pan as i32;
                    right_output = ((ch1 as i64 * pan as i64) >> 10) as i32;
                }
                0x0800 => {
                    // channel 3
                    let pan: i32 = self.channels[3].pan as i32;
                    right_output = ((ch3 as i64 * pan as i64) >> 10) as i32;
                }
                0x0C00 => {
                    let pan1 = self.channels[1].pan as i32;
                    let pan3 = self.channels[3].pan as i32;
                    right_output = ((ch1 as i64 * pan1 as i64) >> 10) as i32
                        + ((ch3 as i64 * pan3 as i64) >> 10) as i32;
                }
                _ => {}
            }
        }

        //println!("lrlrlr {}, {}", left_output >> 1, right_output >> 1);

        left_output = (left_output * self.master_volume as i32) >> 7;
        right_output = (right_output * self.master_volume as i32) >> 7;

        left_output >>= 8;
        right_output >>= 8;

        //println!("lrlrlr2 {}, {}", left_output >> 1, right_output >> 1);

        if self.apply_bias {
            left_output += ((self.bias as i32) << 6) - 0x8000;
            right_output += ((self.bias as i32) << 6) - 0x8000;
        }

        //println!("lrlrlr3 {}, {}", left_output >> 1, right_output >> 1);

        if left_output < -0x8000 {
            left_output = -0x8000;
        } else if left_output > 0x7FFF {
            left_output = 0x7FFF;
        }
        if right_output < -0x8000 {
            right_output = -0x8000;
        } else if right_output > 0x7FFF {
            right_output = 0x7FFF;
        }

        // The original DS and DS lite degrade the output from 16 to 10 bit before output
        if let AudioBitDepth::_10bit = self.bit_depth {
            left_output &= 0xFFFFFFC0u32 as i32;
            right_output &= 0xFFFFFFC0u32 as i32;
        }

        //println!(
        //    "HERE: {} < {}",
        //    self.output_back_buffer_write_position * 2,
        //    SPU_OUTPUT_BUFFER_SIZE - 1
        //);
        //if self.output_back_buffer_write_position * 2 < SPU_OUTPUT_BUFFER_SIZE - 1 {
        //    let left_write = (left_output >> 1) as i16;
        //    let right_write = (right_output >> 1) as i16;
        //
        //    self.output_back_buffer[self.output_back_buffer_write_position] = left_write;
        //    self.output_back_buffer[self.output_back_buffer_write_position + 1] = right_write;
        //
        //    //println!("qlr {}, {}", left_output >> 1, right_output >> 1);
        //    if left_output != 0 {
        //        //assert!([-1856, 1855].contains(&(left_output >> 1)));
        //    }
        //
        //    self.output_back_buffer_write_position += 2;
        //
        //    return Some((left_write, right_write));
        //}

        let left_write = (left_output >> 1) as i16;
        let right_write = (right_output >> 1) as i16;

        (left_write, right_write)
    }

    pub fn transfer_output(&mut self) {
        for i in (0..self.output_back_buffer_write_position).step_by(2) {
            self.output_front_buffer[self.output_front_buffer_write_position] =
                self.output_back_buffer[i];
            self.output_front_buffer[self.output_front_buffer_write_position + 1] =
                self.output_back_buffer[i + 1];

            self.output_front_buffer_write_position += 2;
            self.output_front_buffer_write_position &= SPU_OUTPUT_BUFFER_SIZE * 2 - 1;
            if self.output_front_buffer_write_position == self.output_front_buffer_read_position {
                self.output_front_buffer_read_position += 2;
                self.output_front_buffer_read_position &= SPU_OUTPUT_BUFFER_SIZE * 2 - 1;
            }
        }

        self.output_back_buffer_write_position = 0;
    }

    fn trim_output(&mut self) {
        let mut read_pos = self.output_front_buffer_write_position - SPU_OUTPUT_BUFFER_SIZE;
        if read_pos < 0 {
            read_pos += SPU_OUTPUT_BUFFER_SIZE * 2;
        }

        self.output_front_buffer_read_position = read_pos;
    }

    fn drain_output(&mut self) {
        self.output_front_buffer_write_position = 0;
        self.output_front_buffer_read_position = 0;
    }

    fn init_output(&mut self) {
        self.output_back_buffer.fill(0);
        self.output_front_buffer.fill(0);
        self.output_front_buffer_read_position = 0;
        self.output_front_buffer_write_position = 0;
    }

    fn get_output_size(&self) -> usize {
        let mut ret =
            if self.output_front_buffer_write_position >= self.output_front_buffer_read_position {
                self.output_front_buffer_write_position - self.output_front_buffer_read_position
            } else {
                SPU_OUTPUT_BUFFER_SIZE * 2 - self.output_front_buffer_read_position
                    + self.output_front_buffer_write_position
            };

        ret >>= 1;

        ret
    }

    pub fn read_output(&mut self, samples: usize) -> Vec<i16> {
        let mut data = Vec::new();

        if self.output_front_buffer_read_position == self.output_front_buffer_write_position {
            return data;
        }

        for _ in 0..samples {
            data.push(self.output_front_buffer[self.output_front_buffer_read_position]);
            data.push(self.output_front_buffer[self.output_front_buffer_read_position + 1]);

            self.output_front_buffer_read_position += 2;
            self.output_front_buffer_read_position &= (2 * SPU_OUTPUT_BUFFER_SIZE) - 1;

            if self.output_front_buffer_write_position == self.output_front_buffer_read_position {
                return data;
            }
        }

        data
    }

    pub fn write8(&mut self, address: u32, val: u8) {
        if address < 0x04000500 {
            let chan = &mut self.channels[((address >> 4) & 0xF) as usize];

            match address & 0xF {
                0x0 => chan.set_control((chan.control & 0xFFFFFF00) | val as u32),
                0x1 => chan.set_control((chan.control & 0xFFFF00FF) | (val as u32) << 8),
                0x2 => chan.set_control((chan.control & 0xFF00FFFF) | (val as u32) << 16),
                0x3 => chan.set_control((chan.control & 0x00FFFFFF) | (val as u32) << 24),
                _ => {}
            }
        } else {
            match address {
                0x04000500 => {
                    self.control = (self.control & 0xBF00) | (val & 0x7F) as u16;
                    println!("Controlset: {}", self.control);
                    self.master_volume = (self.control & 0x7F) as u8;
                    if self.master_volume == 127 {
                        self.master_volume += 1;
                    }
                }
                0x04000501 => {
                    self.control = (self.control & 0x007F) | (((val & 0xBF) as u16) << 8) as u16;
                }
                0x04000508 => {
                    self.capture[0].set_control(val);
                    if (val & 0x03) != 0 {
                        println!("Warning !! UNSUPPORTED SPU CAPTURE MODE {}\n", val);
                    }
                }
                0x04000509 => {
                    self.capture[1].set_control(val);
                    if (val & 0x03) != 0 {
                        println!("Warning !! UNSUPPORTED SPU CAPTURE MODE {}\n", val);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn has_updated_channel_control8(&mut self, address: u32) -> bool {
        if address < 0x04000500 {
            let chan = &mut self.channels[((address >> 4) & 0xF) as usize];

            match address & 0xF {
                0x0..=0x3 => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn write16(&mut self, address: u32, val: u16) {
        if address < 0x04000500 {
            let chan = &mut self.channels[((address >> 4) & 0xF) as usize];

            match address & 0xF {
                0x0 => chan.set_control((chan.control & 0xFFFF0000) | val as u32),
                0x2 => chan.set_control((chan.control & 0x0000FFFF) | (val as u32) << 16),
                0x8 => {
                    chan.set_timer_reload(val as u32);
                    if (address & 0xF0) == 0x10 {
                        self.capture[0].set_timer_reload(val as u32);
                    } else if (address & 0xF0) == 0x30 {
                        self.capture[1].set_timer_reload(val as u32);
                    }
                }
                0xA => {
                    chan.set_loop_pos(val as u32);
                }
                0xC => {
                    chan.set_length(((chan.length as u32 >> 2) & 0xFFFF0000) | val as u32);
                }
                0xE => {
                    chan.set_length(((chan.length as u32 >> 2) & 0x0000FFFF) | (val as u32) << 16);
                }
                _ => {}
            }
        } else {
            match address {
                0x04000500 => {
                    self.control = val & 0xBF7F;
                    println!("Controlset: {}", self.control);
                    self.master_volume = (self.control & 0x7F) as u8;
                    if self.master_volume == 127 {
                        self.master_volume += 1;
                    }
                }
                0x04000504 => {
                    self.bias = val & 0x3FF;
                }
                0x04000508 => {
                    self.capture[0].set_control((val & 0xFF) as u8);
                    self.capture[1].set_control((val >> 8) as u8);
                    if (val & 0x0303) != 0 {
                        println!("Warning !! UNSUPPORTED SPU CAPTURE MODE {}\n", val);
                    }
                }
                0x04000514 => {
                    self.capture[0].set_length(val as u32);
                }
                0x0400051C => {
                    self.capture[1].set_length(val as u32);
                }
                _ => {}
            }
        }
    }

    pub fn has_updated_channel_control16(&mut self, address: u32) -> Option<usize> {
        if address < 0x04000500 {
            let chan = &mut self.channels[((address >> 4) & 0xF) as usize];

            match address & 0xF {
                0x0 | 0x2 => Some(((address >> 4) & 0xF) as usize),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn write32(&mut self, address: u32, mut val: u32) {
        if address < 0x04000500 {
            let chan = &mut self.channels[((address >> 4) & 0xF) as usize];

            match address & 0xF {
                0x0 => chan.set_control(val),
                0x4 => {
                    chan.set_src_address(val);
                }
                0x8 => {
                    chan.set_loop_pos(val >> 16);

                    val &= 0xFFFF;
                    chan.set_timer_reload(val);
                    if (address & 0xF0) == 0x10 {
                        self.capture[0].set_timer_reload(val as u32);
                    } else if (address & 0xF0) == 0x30 {
                        self.capture[1].set_timer_reload(val as u32);
                    }
                }
                0xC => {
                    chan.set_length(val);
                }
                _ => {}
            }
        } else {
            match address {
                0x04000500 => {
                    self.control = (val & 0xBF7F) as u16;
                    println!("Controlset: {}", self.control);
                    self.master_volume = (self.control & 0x7F) as u8;
                    if self.master_volume == 127 {
                        self.master_volume += 1;
                    }
                }
                0x04000504 => {
                    self.bias = (val & 0x3FF) as u16;
                }
                0x04000508 => {
                    self.capture[0].set_control((val & 0xFF) as u8);
                    self.capture[1].set_control((val >> 8) as u8);
                    if (val & 0x0303) != 0 {
                        println!("Warning !! UNSUPPORTED SPU CAPTURE MODE {}\n", val);
                    }
                }
                0x04000510 => self.capture[0].set_dest_address(val),
                0x04000514 => {
                    self.capture[0].set_length(val & 0xFFFF);
                }
                0x04000518 => self.capture[1].set_dest_address(val),
                0x0400051C => {
                    self.capture[1].set_length(val & 0xFFFF);
                }
                _ => {}
            }
        }
    }

    pub fn has_updated_channel_control32(&mut self, address: u32) -> bool {
        if address < 0x04000500 {
            let chan = &mut self.channels[((address >> 4) & 0xF) as usize];

            match address & 0xF {
                0x0 => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub fn set_adjusted_channel_volume(
        &mut self,
        channel: usize,
        volume: u32,
        volume_multiplier: f32,
    ) {
        fn adjust_volume(volume: u32, volume_multiplier: f32) -> u32 {
            (volume as f32 * volume_multiplier) as u32
        }
        let adjust_vol = |volume: u32| -> u32 { adjust_volume(volume, volume_multiplier) };
        self.set_channel_volume(channel, adjust_vol(volume));

        //println!("{} VOL: {}", channel, adjust_vol(volume));
    }

    fn set_channel_volume(&mut self, channel: usize, mut volume: u32) {
        let mut volume_shift_index = 3;
        if volume > 127 {
            volume >>= 2;
            volume_shift_index -= 1;
        }
        if volume > 127 {
            volume >>= 1;
            volume_shift_index -= 1;
        }
        if volume > 127 {
            volume >>= 1;
            volume_shift_index -= 1;
        }
        assert!(volume < 128);
        //if volume == 0 {
        //    volume = 1;
        //}
        //let address = channel << 4;
        let control = self.channels[channel].control;
        let control = (control & 0xFFFFFF00) | volume;
        let control = (control & 0xFFFFF0FF) | (volume_shift_index << 8);
        //self.write32(address as u32, control);
        self.channels[channel].set_control(control);
        //self.channels[channel].volume = volume as u8;
        //self.channels[channel].volume_shift = volume_shift;
    }

    pub fn set_channel_timer_reload(&mut self, channel: usize, timer_reload: u32) {
        self.channels[channel].set_timer_reload(timer_reload);
        //println!("TM: {}", (timer_reload & 0xFFFF) as u16);
    }

    pub fn set_channel_loop_pos(&mut self, channel: usize, loop_pos: usize) {
        self.channels[channel].loop_pos = loop_pos;
        //println!("LP: {}", loop_pos);
    }

    pub fn set_channel_length(&mut self, channel: usize, length: usize) {
        self.channels[channel].length = length;
        // println!("LEN: {}", length);
    }

    pub fn set_channel_src_address(&mut self, channel: usize, src_address: usize) {
        self.channels[channel].src_address = src_address;
        //println!("SRC: {}", src_address);
    }

    pub fn set_adjusted_channel_pan(&mut self, channel: usize, pan: u8, pan_addition: i32) {
        let adjust_pan =
            |pan: u8| -> u8 { (pan as i32 + pan_addition).max(0).min(u8::MAX as i32) as u8 };
        let control = self.channels[channel].control;
        let control = (control & 0xFF00FFFF) | ((adjust_pan(pan) as u32) << 16);

        println!("PANO: {}", adjust_pan(pan));

        self.channels[channel].set_control(control)
    }

    pub fn set_channel_pan(&mut self, channel: usize, pan: u8) {
        let control = self.channels[channel].control;
        let control = (control & 0xFF00FFFF) | ((pan as u32) << 16);

        //println!("PAN2");
        self.channels[channel].set_control(control)
    }

    pub fn channel_play_note(&mut self, channel: usize, is_repeating: bool) {
        //self.channels[channel].key_on = true;

        self.channels[channel].set_control(self.channels[channel].control & 0x00FFFFFF);

        if is_repeating {
            self.channels[channel]
                .set_control((self.channels[channel].control & 0x00FFFFFF) | 200 << 24)
        } else {
            self.channels[channel]
                .set_control((self.channels[channel].control & 0x00FFFFFF) | 208 << 24)
        }
        println!("{} PLAY {}", channel, self.channels[channel].control)
    }

    pub fn channel_play_psg(&mut self, channel: usize, table_index: u8) {
        //self.channels[channel].key_on = true;

        self.channels[channel].set_control(self.channels[channel].control & 0x00FFFFFF);
        self.channels[channel].set_control(
            (self.channels[channel].control & 0x00FFFFFF) | (224 + table_index as u32) << 24,
        );
        println!(
            "{} PLAYPSG {} (ti: {})",
            channel, self.channels[channel].control, table_index
        )
    }

    pub fn channel_play_noise(&mut self, channel: usize) {
        self.channels[channel].set_control(self.channels[channel].control & 0x00FFFFFF);
        self.channels[channel]
            .set_control((self.channels[channel].control & 0x00FFFFFF) | 224 << 24);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut ram = Vec::new();
        //for _ in 0..16384 {
        //    ram.push(0);
        //}
        let mut dogadpcm: Vec<u8> = std::fs::read_to_string("dog4.txt")
            .expect("Failed to read file")
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .flat_map(|val| {
                vec![
                    val as u8,
                    (val >> 8) as u8,
                    (val >> 16) as u8,
                    (val >> 24) as u8,
                ]
            })
            .collect();
        ram.append(&mut dogadpcm);
        ram.resize(4 * 1024 * 1024, 0);

        let nds = Arc::new(Mutex::new(Nds::new(ram)));

        let mut spu = Spu::new(nds, AudioBitDepth::_16bit);

        spu.write32(67109952, 1346371937);
        spu.write32(67109952, 1346371663);
        spu.write16(67109960, 64002);
        spu.write16(67109962, 1);
        spu.write32(67109964, 399);
        spu.write32(67109956, 0 + 0x02000000);
        spu.write8(67109955, 208);

        spu.channels[4].control = 3493855311;

        //assert!()
        spu.channels[4].start();

        assert_eq!(
            spu.channels[4].fifo_data,
            [
                1441792, 2567473280, 3131804585, 3420244909, 573994266, 2377879824, 834441539,
                2829908754
            ]
        );
    }

    #[test]
    fn test_2() {
        let mut ram = Vec::new();
        //for _ in 0..16384 {
        //    ram.push(0);
        //}
        let mut dogadpcm: Vec<u8> = std::fs::read_to_string("dog4.txt")
            .expect("Failed to read file")
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .flat_map(|val| {
                vec![
                    val as u8,
                    (val >> 8) as u8,
                    (val >> 16) as u8,
                    (val >> 24) as u8,
                ]
            })
            .collect();
        ram.append(&mut dogadpcm);
        ram.resize(4 * 1024 * 1024, 0);

        let nds = Arc::new(Mutex::new(Nds::new(ram)));

        let mut spu = Spu::new(nds, AudioBitDepth::_16bit);

        spu.write32(67109952, 1346371937);
        spu.write32(67109952, 1346371663);
        spu.write16(67109960, 64002);
        spu.write16(67109962, 1);
        spu.write32(67109964, 399);
        spu.write32(67109956, 0 + 0x02000000);
        spu.write8(67109955, 208);

        spu.channels[4].control = 3493855311;

        //assert!()
        //spu.channels[4].do_run();

        let mut q = [0; 38];

        for k in &mut q {
            *k = spu.channels[4].do_run();
        }

        let z = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 8848, 8848, 8848, 1264, 1264, 1264,
        ];

        assert_eq!(q, z);
    }

    #[test]
    fn test_3() {
        let mut ram = Vec::new();
        //for _ in 0..16384 {
        //    ram.push(0);
        //}
        let mut dogadpcm: Vec<u8> = std::fs::read_to_string("dog4.txt")
            .expect("Failed to read file")
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .flat_map(|val| {
                vec![
                    val as u8,
                    (val >> 8) as u8,
                    (val >> 16) as u8,
                    (val >> 24) as u8,
                ]
            })
            .collect();
        ram.append(&mut dogadpcm);
        ram.resize(4 * 1024 * 1024, 0);

        let nds = Arc::new(Mutex::new(Nds::new(ram)));

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

        spu.mix(1);

        spu.write32(67109952, 1346371937);
        spu.write32(67109952, 1346371663);
        spu.write16(67109960, 64002);
        spu.write16(67109962, 1);
        spu.write32(67109964, 399);
        spu.write32(67109956, 0 + 0x02000000);
        spu.write8(67109955, 208);

        // TODO: Temp
        spu.control = 47487;
        spu.channels[0].control = 1346371942;
        spu.channels[1].control = 2818572415;
        spu.channels[2].control = 1344602743;
        spu.channels[3].control = 2826895487;
        spu.channels[4].control = 3493855311;

        //assert!()
        //spu.channels[4].do_run();

        while spu.channels[1].current_sample == 0 {
            //for _ in 0..100 {
            spu.mix(1);
        }

        assert_eq!(spu.channels[1].current_adjusted_sample(), 4096);
        assert_eq!(spu.channels[3].current_adjusted_sample(), 4096);
    }

    #[test]
    fn test_4() {
        let mut ram = Vec::new();
        //for _ in 0..16384 {
        //    ram.push(0);
        //}
        let mut dogadpcm: Vec<u8> = std::fs::read_to_string("dog4.txt")
            .expect("Failed to read file")
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .flat_map(|val| {
                vec![
                    val as u8,
                    (val >> 8) as u8,
                    (val >> 16) as u8,
                    (val >> 24) as u8,
                ]
            })
            .collect();
        ram.append(&mut dogadpcm);
        ram.resize(4 * 1024 * 1024, 0);

        let nds = Arc::new(Mutex::new(Nds::new(ram)));

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

        spu.mix(1);

        spu.write32(67109952, 1346371937);
        spu.write32(67109952, 1346371663);
        spu.write16(67109960, 64002);
        spu.write16(67109962, 1);
        spu.write32(67109964, 399);
        spu.write32(67109956, 0 + 0x02000000);
        spu.write8(67109955, 208);

        // TODO: Temp
        spu.control = 47487;
        spu.channels[0].control = 1346371942;
        spu.channels[1].control = 2818572415;
        spu.channels[2].control = 1344602743;
        spu.channels[3].control = 2826895487;
        spu.channels[4].control = 3493855311;

        //assert!()
        //spu.channels[4].do_run();

        let mut left = 0;

        while spu.channels[1].current_sample == 0 {
            //for _ in 0..100 {
            let out = spu.mix(1);
            left = out.0;
            //if spu.output_back_buffer_write_position * 2 >= SPU_OUTPUT_BUFFER_SIZE - 1 {
            //    spu.transfer_output();
            //}
        }

        assert_eq!(left, 1);
    }
}
