use super::interface::DisplayInterface;
use super::st7789v::*;
use display_interface::DisplayError;
use embedded_hal::spi::SpiBus;
use embedded_hal::{delay::DelayNs, digital::OutputPin};

use super::graphics::*;

pub struct Lcd<SPI, RST, WR, PWR> {
    interface: DisplayInterface<SPI, RST, WR, PWR>,
}

impl<SPI, RST, WR, PWR> Lcd<SPI, RST, WR, PWR>
where
    SPI: SpiBus,
    RST: OutputPin,
    WR: OutputPin,
    PWR: OutputPin,
{
    pub fn init(spi: SPI, rst: RST, wr: WR, pwr: PWR, delay: &mut impl DelayNs) -> Self {
        let interface = DisplayInterface::new(spi, rst, wr, pwr);
        let mut lcd = Self { interface };
        lcd.interface.reset(delay);
        lcd.sleep_out(delay);
        lcd.set_pixel_format(0x65);
        lcd.display_inversion_on(delay);
        lcd.display_on(delay);

        lcd.mem_data_ac(0x00);
        lcd.interface.lcd_on(delay);
        
        lcd
    }

    /// Turn off sleep mode
    #[inline]
    pub fn sleep_out(&mut self, delay: &mut impl DelayNs) {
        self.interface.cmd(Cmd::SLPOUT.bits()).unwrap();
        delay.delay_ms(WAIT_MS);
    }
    /// Recover from display inversion mode
    #[inline]
    pub fn display_inversion_on(&mut self, delay: &mut impl DelayNs) {
        self.interface.cmd(Cmd::INVON.bits()).unwrap();
        delay.delay_ms(WAIT_MS);
    }
    /// Recover from DISPLAY OFF mode
    #[inline]
    pub fn display_on(&mut self, delay: &mut impl DelayNs) {
        self.interface.cmd(Cmd::DISPON.bits()).unwrap();
        delay.delay_ms(WAIT_MS);
    }
    /// Sets the Memory Data Access Control (MADCTL) register.
    ///
    /// This function configures the frame memory scanning direction and color order
    /// by sending the MADCTL command (`0x36`) followed by a single parameter byte.
    ///
    /// # Parameters
    /// - `param`: A 8-bit value that determines the memory access control settings.
    ///   - **D7 (MY)**: Page Address Order  
    ///     - `0`: Top to Bottom  
    ///     - `1`: Bottom to Top  
    ///   - **D6 (MX)**: Column Address Order  
    ///     - `0`: Left to Right  
    ///     - `1`: Right to Left  
    ///   - **D5 (MV)**: Page/Column Order  
    ///     - `0`: Normal Mode  
    ///     - `1`: Reverse Mode  
    ///   - **D4 (ML)**: Line Address Order  
    ///     - `0`: LCD Refresh Top to Bottom  
    ///     - `1`: LCD Refresh Bottom to Top  
    ///   - **D3 (RGB/BGR)**: Color Order  
    ///     - `0`: RGB  
    ///     - `1`: BGR  
    ///   - **D2 (MH)**: Display Data Latch Data Order  
    ///     - `0`: LCD Refresh Left to Right  
    ///     - `1`: LCD Refresh Right to Left  
    ///
    /// # Panics
    /// This function will panic if sending the command or data fails.
    ///
    /// # Example
    /// ```rust
    /// // Set memory access control to normal mode, top-to-bottom, left-to-right, RGB order
    /// display.mem_data_ac(0b0000_0000);
    /// ```
    #[inline]
    pub fn mem_data_ac(&mut self, param: u8) {
        self.interface.cmd(Cmd::MADCTL.bits()).unwrap();
        self.interface.data(&[param]).unwrap();
    }
    /// Sets the interface pixel format (COLMOD, 0x3A).
    ///
    /// This function defines the format of RGB picture data to be transferred via the MCU interface.
    /// The pixel format determines the number of bits per pixel (bpp) and affects both the RGB
    /// interface and the control interface.
    ///
    /// # Parameters
    /// - `param`: An 8-bit value that specifies the pixel format.
    ///   - **D7**: Always set to `0`.
    ///   - **D6-D4**: RGB interface color format  
    ///     - `101` (0b0101_0000): 65K colors  
    ///     - `110` (0b0110_0000): 262K colors  
    ///   - **D3**: Always set to `0`.
    ///   - **D2-D0**: Control interface color format  
    ///     - `011` (0b0000_0110): 12-bit per pixel  
    ///     - `101` (0b0000_0101): 16-bit per pixel  
    ///     - `110` (0b0000_0110): 18-bit per pixel  
    ///     - `111` (0b0000_0111): 16M truncated  
    ///
    /// # Panics
    /// This function will panic if sending the command or data fails.
    ///
    /// # Example
    /// ```rust
    /// // Set the display to 262K colors/16-bit per pixel mode
    /// display.set_pixel_format(0b01100101);
    /// ```
    pub fn set_pixel_format(&mut self, param: u8) {
        self.interface.cmd(Cmd::COLMOD.bits()).unwrap();
        self.interface.data(&[param]).unwrap();
    }
}

impl<SPI, RST, WR, PWR> Lcd<SPI, RST, WR, PWR>
where
    SPI: SpiBus,
    RST: OutputPin,
    WR: OutputPin,
    PWR: OutputPin,
{
    /// Sets the frame memory area (column and row address range).
    ///
    /// This function defines the rectangular area of the display where pixel data will be written.
    /// It sends the `CASET` (Column Address Set) and `RASET` (Row Address Set) commands
    /// followed by the corresponding start and end coordinates.
    ///
    /// # Parameters
    /// - `start_x`: The starting column address (0–maximum width of the display).
    /// - `end_x`: The ending column address (must be ≥ `start_x`).
    /// - `start_y`: The starting row address (0–maximum height of the display).
    /// - `end_y`: The ending row address (must be ≥ `start_y`).
    ///
    /// # Returns
    /// - `Ok(())` if the command and data transmissions succeed.
    /// - `Err(DisplayError)` if sending a command or data fails.
    pub fn set_frame_area(
        &mut self,
        start_x: u16,
        start_y: u16,
        end_x: u16,
        end_y: u16,
    ) -> Result<(), DisplayError> {
        self.interface.cmd(Cmd::CASET.bits())?;
        self.interface.data(&[
            (start_x >> 8) as u8,
            start_x as u8,
            (end_x >> 8) as u8,
            end_x as u8,
        ])?;
        self.interface.cmd(Cmd::RASET.bits())?;
        self.interface.data(&[
            (start_y >> 8) as u8,
            start_y as u8,
            (end_y >> 8) as u8,
            end_y as u8,
        ])
    }
    // pub fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), DisplayError> {
    //     self.set_frame_area(x, y, x, y)
    // }
    pub fn set_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), DisplayError> {
        self.set_frame_area(x, y, x, y)?;
        self.interface.cmd(Cmd::RAMWR.bits())?;
        self.interface.data(&[(color >> 8) as u8, color as u8])
    }
    // pub fn clear_frame(&mut self, color: u16) -> Result<(), DisplayError> {
    //     self.set_frame_area(0, 0, COLS - 1, ROWS - 1)?;
    //     let msb = (color >> 8) as u8;
    //     let lsb = color as u8;
    //     let mut buffer = [0u8; FRAME_SIZE];
    //     buffer.chunks_exact_mut(2).for_each(|pixel| {
    //         pixel[0] = msb;
    //         pixel[1] = lsb;
    //     });
    //     self.interface.cmd(Cmd::RAMWR.bits())?;
    //     self.interface.data(&buffer)?;
    //     Ok(())
    // }
    pub fn clear_frame(&mut self, display: &Display2in14) -> Result<(), DisplayError> {
        self.set_frame_area(0, 0, COLS - 1, ROWS - 1)?;
        self.interface.cmd(Cmd::RAMWR.bits())?;
        self.interface.data(&display.buffer)
    }
}

// impl<SPI, RST, WR, PWR> Dimensions for Lcd<SPI, RST, WR, PWR>
// where
//     SPI: SpiBus,
//     RST: OutputPin,
//     WR: OutputPin,
//     PWR: OutputPin,
// {
//     fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
//         embedded_graphics::primitives::Rectangle::new(
//             Point { x: 0, y: 0 },
//             Size::new(COLS as u32, ROWS as u32)
//         )
//     }
// }

// impl<SPI, RST, WR, PWR> DrawTarget for Lcd<SPI, RST, WR, PWR>
// where
//     SPI: SpiBus,
//     RST: OutputPin,
//     WR: OutputPin,
//     PWR: OutputPin,
// {
//     type Color = embedded_graphics::pixelcolor::Rgb565;

//     type Error = display_interface::DisplayError;

//     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
//     where
//         I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
//     {
//         pixels.into_iter().try_for_each(|pixel| {
//             let ((x, y), color) = ((pixel.0.x as u16, pixel.0.y as u16), pixel.1);
//             self.set_pixel(x, y, color.into_storage())
//         })
//     }
// }
