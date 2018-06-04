extern crate spidev;

use std::io;
use std::io::prelude::*;
use std::path::Path;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

/// A connector to on or more L6470
pub struct L6470Connector{
    spi_bus: io::Result<Spidev>,
    spi_options: SpidevOptions,
    daisy_chain: u8,

}

impl L6470Connector {

    /// Returns a new connectors
    pub fn new<P: AsRef<Path>>(path: P) -> L6470Connector {
        L6470Connector {
            spi_bus: Spidev::open(path),
            spi_options: SpidevOptions::new()
                .bits_per_word(8)
                .max_speed_hz(20_000)
                .mode(SPI_MODE_0)
                .build(),
            daisy_chain: 1,
        }
    }

    /// Set the maximum SPI transfer speed in Hz
    ///
    /// The controller can't necessarily assign that specific clock speed
    pub fn set_spi_clock_hz(&mut self, clock_hz: u32){
        self.spi_options.max_speed_hz(clock_hz);
    }


    pub fn build(self) -> io::Result<Spidev> {
        let mut spi = self.spi_bus?;
        spi.configure(&self.spi_options)?;
        Ok(spi)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
