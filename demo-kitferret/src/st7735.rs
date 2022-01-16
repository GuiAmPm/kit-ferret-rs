pub mod instruction;

use crate::spi_controller::SpiController;
use ferret_rs::system::ScreenTrait;
use crate::st7735::instruction::Instruction;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

/// ST7735 driver to connect to TFT displays.
pub struct ST7735<'a, SPI, DC, RST>
where
    SPI: SpiController,
    DC: OutputPin,
    RST: OutputPin,
{
    /// SPI
    spi: SPI,

    /// Data/command pin.
    dc: DC,

    /// Reset pin.
    rst: RST,

    /// Whether the display is RGB (true) or BGR (false)
    rgb: bool,

    /// Whether the colours are inverted (true) or not (false)
    inverted: bool,
    pub width: u16,
    pub height: u16,
    buffer: Option<&'a mut [u8]>,
    interlace: bool,
    interlace_even: bool
}

impl<'a, SPI, DC, RST> ST7735<'a, SPI, DC, RST>
where
    SPI: SpiController,
    DC: OutputPin,
    RST: OutputPin,
{
    /// Creates a new driver instance that uses hardware SPI.
    pub fn new(
        spi: SPI,
        dc: DC,
        rst: RST,
        rgb: bool,
        inverted: bool,
        width: u16,
        height: u16,
    ) -> Self {
        let display = ST7735 {
            spi,
            dc,
            rst,
            rgb,
            inverted,
            width,
            height,
            buffer: None,
            interlace: false,
            interlace_even: false
        };

        display
    }

    pub fn set_buffer(&mut self, buffer: Option<&'a mut [u8]>) {
        self.buffer = buffer;
    }

    pub fn set_interlace(&mut self, value: bool) {
        if !value {
            self.set_address_window(0, 0, self.width - 1, self.height - 1)
        }

        self.interlace = value;
    }

    fn update_entire_screen(&mut self) -> Result<(), ()> {
        if self.buffer != None {
            self.write_command(Instruction::RAMWR, &[])?;
            self.start_data()?;
            let buffer = self.buffer.as_ref().unwrap();
            self.spi.write(&buffer);

            Ok(())
        } else {
            todo!()
        }
    }

    fn update_screen_interlace(&mut self) -> Result<(), ()> {
        if self.buffer != None {
            let width = self.width;
            let height = self.height;
            let even = self.interlace_even;

            let start = if even { 0 } else { 1 };

            for y in (start..height).step_by(2) {
                self.set_address_window(0, y, 160, y + 1);

                self.write_command(Instruction::RAMWR, &[])?;
                self.start_data()?;
                let buffer = self.buffer.as_ref().unwrap();
                let start_y = y as usize * 160 * 2;
                let end_y = (y as usize + 1) * 160 * 2;
                self.spi.write(&buffer[(start_y..end_y)]);
            }

            // Toggle active row
            self.interlace_even = !self.interlace_even;

            Ok(())
        } else {
            todo!()
        }
    }

    /// Runs commands to initialize the display.
    pub fn init<DELAY>(&mut self, delay: &mut DELAY)
    where
        DELAY: DelayMs<u8>,
    {
        log::info!("Initialising screen");
        self.hard_reset(delay);
        log::info!("SWRESET");
        self.write_command(Instruction::SWRESET, &[]);
        delay.delay_ms(200);
        log::info!("SLPOUT");
        self.write_command(Instruction::SLPOUT, &[]);
        delay.delay_ms(200);
        log::info!("FRMCTR1");
        self.write_command(Instruction::FRMCTR1, &[0x01, 0x2C, 0x2D]);
        log::info!("FRMCTR2");
        self.write_command(Instruction::FRMCTR2, &[0x01, 0x2C, 0x2D]);
        log::info!("FRMCTR3");
        self.write_command(Instruction::FRMCTR3, &[0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D]);
        log::info!("INVCTR");
        self.write_command(Instruction::INVCTR, &[0x07]);
        log::info!("PWCTR1");
        self.write_command(Instruction::PWCTR1, &[0xA2, 0x02, 0x84]);
        log::info!("PWCTR2");
        self.write_command(Instruction::PWCTR2, &[0xC5]);
        log::info!("PWCTR3");
        self.write_command(Instruction::PWCTR3, &[0x0A, 0x00]);
        log::info!("PWCTR4");
        self.write_command(Instruction::PWCTR4, &[0x8A, 0x2A]);
        log::info!("PWCTR5");
        self.write_command(Instruction::PWCTR5, &[0x8A, 0xEE]);
        log::info!("VMCTR1");
        self.write_command(Instruction::VMCTR1, &[0x0E]);
        if self.inverted {
            log::info!("INVON");
            self.write_command(Instruction::INVON, &[]);
        } else {
            log::info!("INVOFF");
            self.write_command(Instruction::INVOFF, &[]);
        }
        if self.rgb {
            log::info!("MADCTL");
            self.write_command(Instruction::MADCTL, &[0x00]);
        } else {
            log::info!("MADCTL");
            self.write_command(Instruction::MADCTL, &[0x08]);
        }

        log::info!("COLMOD");
        self.write_command(Instruction::COLMOD, &[0x05]);
        log::info!("COLMOD");
        self.write_command(Instruction::DISPON, &[]);

        // Set landscape
        log::info!("MADCTL");
        self.write_command(Instruction::MADCTL, &[0x60]);
        self.set_address_window(0, 0, self.width - 1, self.height - 1);

        delay.delay_ms(200);
    }

    pub fn hard_reset<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>
    where
        DELAY: DelayMs<u8>,
    {
        self.rst.set_high().map_err(|_| ())?;
        delay.delay_ms(10);
        self.rst.set_low().map_err(|_| ())?;
        delay.delay_ms(10);
        self.rst.set_high().map_err(|_| ())
    }

    /// Sets the address window for the display.
    pub fn set_address_window(&mut self, sx: u16, sy: u16, ex: u16, ey: u16) {
        self.write_command(Instruction::CASET, &[]);
        self.start_data();
        self.write_word(sx);
        self.write_word(ex);
        self.write_command(Instruction::RASET, &[]);
        self.start_data();
        self.write_word(sy);
        self.write_word(ey);
    }

    /// Writes a data word to the display.
    fn write_word(&mut self, value: u16) {
        self.write_data(&value.to_be_bytes());
    }

    fn write_command(&mut self, command: Instruction, params: &[u8]) -> Result<(), ()> {
        self.dc.set_low().map_err(|_| ())?;
        self.spi.write(&[command as u8]);
        if !params.is_empty() {
            self.start_data()?;
            self.write_data(params);
        }
        Ok(())
    }

    fn start_data(&mut self) -> Result<(), ()> {
        self.dc.set_high().map_err(|_| ())
    }

    fn write_data(&mut self, data: &[u8]) {
        self.spi.write(data);
    }

    fn set_pixel_internal(&mut self, x: u16, y: u16, r: u8, g: u8, b: u8) {
        let width = self.width;

        // Skip if out-of-bounds
        if x > self.width as u16
            || y > self.height as u16
        {
            return;
        }

        if let Some(buffer) = &mut self.buffer {
            let index = (y * width as u16 + x) as usize;

            let r = ((((r as u16) * 31 / 255) & 0b0011_1111) as u16) << 11;
            let g = ((((g as u16) * 63 / 255) & 0b0111_1111) as u16) << 5;
            let b = ((((b as u16) * 31 / 255) & 0b0011_1111) as u16) << 0;

            let color = r+g+b;
            let bytes = color.to_be_bytes();

            buffer[index * 2 + 0] = bytes[0];
            buffer[index * 2 + 1] = bytes[1];
        } else {

        }
    }


    fn clear_internal(&mut self, red: u8, green: u8, blue: u8) {
        if let Some(buffer) = &mut self.buffer {
            for x in (0..buffer.len()).step_by(2) {
                let r = ((((red as u16) * 31 / 255) & 0b0011_1111) as u16) << 11;
                let g = ((((green as u16) * 63 / 255) & 0b0111_1111) as u16) << 5;
                let b = ((((blue as u16) * 31 / 255) & 0b0011_1111) as u16) << 0;

                let color = r+g+b;
                let bytes = color.to_be_bytes();

                buffer[x + 0] = bytes[0];
                buffer[x + 1] = bytes[1];
            }
        }
    }
}

impl<'a, SPI, DC, RST> ScreenTrait for ST7735<'a, SPI, DC, RST>
where
    SPI: SpiController,
    DC: OutputPin,
    RST: OutputPin,
{
    fn get_width(&self) -> u16 { self.width }
    fn get_height(&self) -> u16 { self.height }
    fn set_pixel(&mut self, x: u16, y: u16, r: u8, g: u8, b: u8) {
        self.set_pixel_internal(x, y, r, g, b);
    }

    fn clear(&mut self, r: u8, g: u8, b: u8) {
        self.clear_internal(r, g, b);
    }

    fn update_screen(&mut self) -> core::result::Result<(), ()> {
        if self.interlace {
            self.update_screen_interlace()
        } else {
            self.update_entire_screen()
        }
    }
}
