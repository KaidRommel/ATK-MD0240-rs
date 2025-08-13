use crate::st7789v::{COLS, ROWS};
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};

use super::st7789v::FRAME_SIZE;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DisplayRotation {
    /// No rotation
    Rotate0,
    /// Rotate by 90 degrees clockwise
    Rotate90,
    /// Rotate by 180 degrees clockwise
    Rotate180,
    /// Rotate 270 degrees clockwise
    Rotate270,
}

impl Default for DisplayRotation {
    fn default() -> Self {
        DisplayRotation::Rotate0
    }
}

#[cfg(feature = "heap_alloc")]
extern crate alloc;
#[cfg(feature = "heap_alloc")]
use alloc::vec::Vec;
pub struct Display2in14 {
    #[cfg(feature = "stack_alloc")]
    pub(crate) buffer: [u8; FRAME_SIZE],
    #[cfg(feature = "heap_alloc")]
    pub(crate) buffer: Vec<u8>,
    rotation: DisplayRotation,
}

impl Display2in14 {
    /// Create a buffer with a background color
    #[cfg(feature = "stack_alloc")]
    pub fn new(color: Rgb565) -> Self {
        let color = color.into_storage();
        let msb = (color >> 8) as u8;
        let lsb = color as u8;
        let mut buffer = [0u8; FRAME_SIZE];
        buffer.chunks_exact_mut(2).for_each(|pixel| {
            pixel[0] = msb;
            pixel[1] = lsb;
        });
        Self {
            buffer,
            rotation: DisplayRotation::default(),
        }
    }
    #[cfg(feature = "heap_alloc")]
    pub fn new(mut buffer: Vec<u8>, color: Rgb565) -> Self {
        if buffer.len() != FRAME_SIZE {
            panic!("Incorrect buffer size")
        }
        let color = color.into_storage();
        let msb = (color >> 8) as u8;
        let lsb = color as u8;
        buffer.chunks_exact_mut(2).for_each(|pixel| {
            pixel[0] = msb;
            pixel[1] = lsb;
        });
        Self {
            buffer,
            rotation: DisplayRotation::default(),
        }
    }
    /// Clear the buffer with a background color
    pub fn clear_buffer(&mut self, color: Rgb565) {
        let color = color.into_storage();
        let msb = (color >> 8) as u8;
        let lsb = color as u8;
        self.buffer.chunks_exact_mut(2).for_each(|pixel| {
            pixel[0] = msb;
            pixel[1] = lsb;
        });
    }

    pub fn get_rotation(&self) -> DisplayRotation {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: DisplayRotation) {
        self.rotation = rotation
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str, style: MonoTextStyle<Rgb565>) {
        Text::new(
            text,
            Point {
                x: x as i32,
                y: y as i32,
            },
            style,
        )
        .draw(self)
        .unwrap();
    }

    pub fn draw_text_default_style(&mut self, x: u16, y: u16, text: &str) {
        let style = MonoTextStyle::new(&FONT_10X20, Rgb565::BLACK);
        self.draw_text(x, y, text, style);
    }

    fn get_location(&self, x: u16, y: u16) -> usize {
        let x = x as usize;
        let y = y as usize;
        match self.rotation {
            DisplayRotation::Rotate0 => (y * COLS as usize + x) * 2,
            _ => {
                todo!();
            }
        }
    }
    fn set_pixel(&mut self, x: u16, y: u16, color: Rgb565) {
        let idx = self.get_location(x, y);
        let color = color.into_storage();
        self.buffer[idx] = (color >> 8) as u8;
        self.buffer[idx + 1] = color as u8;
    }
}

impl OriginDimensions for Display2in14 {
    fn size(&self) -> Size {
        match self.get_rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                Size::new(COLS as u32, ROWS as u32)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                Size::new(ROWS as u32, COLS as u32)
            }
        }
    }
}

impl DrawTarget for Display2in14 {
    type Color = Rgb565;

    type Error = display_interface::DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        pixels.into_iter().try_for_each(|pixel| {
            let ((x, y), color) = ((pixel.0.x as u16, pixel.0.y as u16), pixel.1);
            self.set_pixel(x, y, color);
            Ok(())
        })
    }
}
