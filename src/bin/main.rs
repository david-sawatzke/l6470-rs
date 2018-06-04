extern crate linux_l6470;

extern crate spidev;
use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};


fn main() -> Result<(), std::io::Error> {

    let spi = linux_l6470::L6470Connector::new("/dev/spidev1.0")
        .build()?;


    let tx_buf = [0x01, 0x02, 0x03];
    let mut rx_buf = [0; 3];
    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        try!(spi.transfer(&mut transfer));
    }
    println!("{:?}", rx_buf);
    Ok(())
}