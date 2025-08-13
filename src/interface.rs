//! Display interface using SPI
use super::st7789v::*;
use display_interface::DisplayError;
use embedded_hal::{delay::DelayNs, digital::OutputPin, spi::SpiBus};

const RESET_DELAY_US: u32 = 12;
const LCD_ON_DELAY_US: u32 = 1;

pub struct DisplayInterface<SPI, RST, WR, PWR> {
    /// SPI device
    spi: SPI,
    /// Pin for Reseting
    rst: RST,
    /// Data/Command Control Pin (High for data, Low for command)
    wr: WR,
    /// LCD backlight control pin (Low: Off, High: On)
    pwr: PWR,
}

impl<SPI, RST, WR, PWR> DisplayInterface<SPI, RST, WR, PWR> {
    /// Create and initialize display
    pub fn new(spi: SPI, rst: RST, wr: WR, pwr: PWR) -> Self {
        Self { spi, rst, wr, pwr }
    }
}

impl<SPI, RST, WR, PWR> DisplayInterface<SPI, RST, WR, PWR>
where
    SPI: SpiBus,
    RST: OutputPin,
    WR: OutputPin,
    PWR: OutputPin,
{
    /// Sends a command byte synchronously over SPI.
    ///
    /// This function performs a synchronous SPI operation. It sets the data/command (DC) line low
    /// to indicate a command, writes the command byte to the SPI bus, and waits for the operation
    /// to complete by flushing the SPI buffer. The function blocks until the command is fully
    /// transmitted.
    #[inline]
    pub fn cmd(&mut self, command: u8) -> Result<(), DisplayError> {
        self.wr.set_low().map_err(|_| DisplayError::DCError)?;
        self.spi
            .write(&[command])
            .map_err(|_| DisplayError::BusWriteError)?;
        self.spi.flush().map_err(|_| DisplayError::BusWriteError)
    }
    /// Sends an array of data bytes synchronously over SPI.
    ///
    /// This function performs a synchronous SPI operation. It sets the data/command (DC) line high
    /// to indicate data, writes the provided data bytes to the SPI bus, and waits for the operation
    /// to complete by flushing the SPI buffer. The function blocks until the data is fully transmitted.
    #[inline]
    pub fn data(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.wr.set_high().map_err(|_| DisplayError::DCError)?;
        self.spi
            .write(data)
            .map_err(|_| DisplayError::BusWriteError)?;
        self.spi.flush().map_err(|_| DisplayError::BusWriteError)
    }
    /// Sends a command byte asynchronously over SPI.
    ///
    /// This function performs an asynchronous SPI operation. It sets the data/command (DC) line low
    /// to indicate a command, writes the command byte to the SPI bus, then return.
    ///
    /// **Note:** Ensure that all commands are fully sent before calling this function again or changing
    /// the state of the `wr` pin to prevent peripheral misinterpretation.
    #[inline]
    pub fn cmd_async(&mut self, command: u8) -> Result<(), DisplayError> {
        self.wr.set_low().map_err(|_| DisplayError::DCError)?;
        self.spi
            .write(&[command])
            .map_err(|_| DisplayError::BusWriteError)
    }
    /// Sends an array of data bytes asynchronously over SPI.
    ///
    /// This function performs an asynchronous SPI operation. It sets the data/command (DC) line high
    /// to indicate data, writes the provided data bytes to the SPI bus, then return.
    ///
    /// **Note:** Ensure that all data is fully sent before calling this function again or changing
    /// the state of the `wr` pin to prevent peripheral misinterpretation.
    #[inline]
    pub fn data_async(&mut self, data: &[u8]) -> Result<(), DisplayError> {
        self.wr.set_high().map_err(|_| DisplayError::DCError)?;
        self.spi
            .write(data)
            .map_err(|_| DisplayError::BusWriteError)
    }
    /// Waits until all commands or data have been sent over SPI.
    ///
    /// This function ensures that any pending SPI transmissions are fully completed before proceeding.
    /// It is useful in scenarios where the state of the `wr` pin or other peripheral control signals
    /// may change after data transmission. Calling this function helps avoid data corruption or
    /// unexpected peripheral behavior.
    #[inline]
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.spi.flush().map_err(|_| DisplayError::BusWriteError)
    }
    /// Reset the device
    #[inline]
    pub fn reset(&mut self, delay: &mut impl DelayNs) {
        self.rst.set_low().unwrap();
        delay.delay_us(RESET_DELAY_US);
        self.rst.set_high().unwrap();
        delay.delay_ms(WAIT_MS);
    }
    /// LCD on
    #[inline]
    pub fn lcd_on(&mut self, delay: &mut impl DelayNs) {
        self.pwr.set_high().unwrap();
        delay.delay_us(LCD_ON_DELAY_US);
    }
    /// LCD off
    #[inline]
    pub fn lcd_off(&mut self, delay: &mut impl DelayNs) {
        self.pwr.set_low().unwrap();
        delay.delay_us(LCD_ON_DELAY_US);
    }
}
