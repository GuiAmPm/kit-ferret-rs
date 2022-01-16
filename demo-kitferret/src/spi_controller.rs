use embedded_hal::blocking::spi;

pub trait SpiController {
    fn write(&mut self, buffer: &[u8]);
}

pub struct BlockingSpiController<SPI>
where SPI: spi::Write<u8> {
    spi: SPI
}

impl<SPI> BlockingSpiController<SPI>
where SPI: spi::Write<u8>
{
    pub fn new(spi: SPI) -> Self {
        Self {
            spi
        }
    }
}

impl<SPI> SpiController for BlockingSpiController<SPI>
where SPI: spi::Write<u8> {

    fn write(&mut self, buffer: &[u8]) {
        self.spi.write(buffer);
    }
}

pub struct DmaTransferSpiController<F>
where F: FnMut(&[u8])
{
    do_write: F
}

impl<F> DmaTransferSpiController<F>
where F: FnMut(&[u8])
{
    pub fn new(do_write: F) -> Self
    {
        Self {
            do_write
        }
    }
}

impl<F> SpiController for DmaTransferSpiController<F>
where F: FnMut(&[u8])
{
    fn write(&mut self, buffer: &[u8]) {
        (self.do_write)(buffer);
    }
}