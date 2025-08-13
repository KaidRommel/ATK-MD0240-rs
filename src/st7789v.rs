//! Single-chip controller/driver for 262K-color

use bitflags::bitflags;

pub const COLS: u16 = 240;
pub const ROWS: u16 = 320;

pub const WAIT_MS: u32 = 120;

pub const FRAME_SIZE: usize = (COLS as usize) * (ROWS as usize) * 2;

bitflags! {
    pub struct Cmd: u8 {
        const NOP = 0x00;
        /// Sleep Out
        const SLPOUT = 0x11;
        /// Display Inversion On
        const INVON = 0x21;
        /// Display On
        const DISPON = 0x29;
        /// Column Address Set
        const CASET = 0x2A;
        /// Row Address Set
        const RASET = 0x2B;
        /// Transfer data from MCU to frame memory
        const RAMWR = 0x2C;
        /// Memory Data Access Control
        const MADCTL = 0x36;
        /// Interface Pixel Format
        const COLMOD = 0x3A;
    }
}


