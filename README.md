# ATK-MD0240 LCD Driver (ST7789V)

[![Crates.io](https://img.shields.io/crates/v/atk_md0240.svg)](https://crates.io/crates/atk_md0240)
[![docs.rs](https://docs.rs/atk_md0240/badge.svg)](https://docs.rs/atk_md0240)

A `no_std` driver for the ATK-MD0240 2.4-inch LCD module, which uses the ST7789V controller.

This crate is built upon the `embedded-hal` traits for hardware abstraction and uses `embedded-graphics` for all drawing operations.

The driver includes an internal framebuffer to allow for composing graphics before sending a complete frame to the display.

## Features

- `no_std` compatible: Suitable for bare-metal and RTOS-based systems.
- SPI interface for communication.
- `embedded-graphics` `DrawTarget` implementation for easy drawing of shapes, text, and images.
- Internal framebuffer (`Display2in14`) for composing graphics before sending to the display.
- Configurable framebuffer allocation via Cargo features:
  - `stack_alloc` (default): Allocates the framebuffer on the stack. Simple and no allocator needed.
  - `heap_alloc`: Allocates the framebuffer on the heap. Requires a global allocator.
- Screen rotation support.

## Hardware Connections

The driver requires an SPI interface and three GPIO pins for operation.

| LCD Pin | MCU Pin       | Description in Code | Description                               |
|---------|---------------|---------------------|-------------------------------------------|
| VCC     | 3.3V / 5V     | -                   | Power Supply                              |
| GND     | GND           | -                   | Ground                                    |
| CS      | SPI CS        | -                   | Chip Select (often handled by `SpiDevice`) |
| SCLK    | SPI SCLK      | `spi`               | Serial Clock                              |
| SDA/SDI | SPI MOSI      | `spi`               | Serial Data In (Master Out)               |
| RESET   | GPIO Output   | `rst`               | Reset Pin (active low)                    |
| RS/DC   | GPIO Output   | `wr`                | Data/Command Select Pin                   |
| BL      | GPIO Output   | `pwr`               | Backlight Control Pin (active high)       |

## Usage

### 1. Add to `Cargo.toml`

Add the following to your `Cargo.toml` file.

```toml
[dependencies]
atk_md0240 = { git = "https://github.com/KaidRommel/ATK-MD0240-rs.git" }
```

If you need to use heap allocation for the framebuffer, enable the `heap_alloc` feature.

```toml
[dependencies]
atk_md0240 = { git = "https://github.com/KaidRommel/ATK-MD0240-rs.git", features = ["heap_alloc"] }
```

### 2. Example


## API Overview

- **`Lcd`**: The main driver struct. It handles communication with the LCD.
  - `init()`: Initializes the display controller.
  - `clear_frame()`: Sends the entire content of a `Display2in14` buffer to the screen.
  - `set_pixel()`: Sets a single pixel on the display (less efficient for multiple pixels).
- **`Display2in14`**: An in-memory framebuffer that implements `embedded_graphics::DrawTarget`.
  - `new()`: Creates a new framebuffer, filling it with a specified color.
  - `clear_buffer()`: Clears the buffer to a single color.
  - All `embedded-graphics` drawing functions can be used on a `Display2in14` instance.
