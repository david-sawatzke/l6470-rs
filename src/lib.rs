#[macro_use]
extern crate bitflags;

extern crate byteorder;
extern crate spidev;

use std::io;
use std::path::Path;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_3};
use std::{thread, time};


use byteorder::{ByteOrder, BigEndian};

pub mod register;

pub use register::Register;
//pub mod command;

bitflags! {
    pub struct Motors: u8 {
        const M1 = 0b00000001;
        const M2 = 0b00000010;
        const M3 = 0b00000100;
        const M4 = 0b00001000;
        const M5 = 0b00010000;
        const M6 = 0b00100000;
        const M7 = 0b01000000;
        const M8 = 0b10000000;
    }
}

#[derive(PartialEq)]
pub enum Direction {
    CW,
    CCW,
}

pub struct L6470 {
    spi: Spidev,
    daisy_chain: u8
}

bitflags! {
    pub struct StepMode: u8 {
        const DIV1 = 0;
        const DIV2  = 0b00000001;
        const DIV4  = 0b00000010;
        const DIV16 = 0b00000100;
        const DIV8 = Self::DIV2.bits | Self::DIV4.bits;
        const DIV32 = Self::DIV2.bits | Self::DIV16.bits;
        const DIV64 = Self::DIV4.bits | Self::DIV16.bits;
        const DIV128 = Self::DIV2.bits | Self::DIV4.bits | Self::DIV16.bits;
    }
}

impl L6470{

    pub fn init(&self) {
        self.resync_com();
        self.send_reset(Motors::all());
        self.write_register(Motors::all(), &register::ALARM_EN, 0b1000_0011 as u32);
        self.get_status(Motors::all());
        self.set_step_mode(Motors::all(), StepMode::DIV16);
        self.init_speed();
    }

    pub fn init_speed(&self){
        // Motor with 1.8° / 200 step per revolution

        let tick_ns = 250;

        // 1 tours / second

        // 200 step / s2 = 60tr/min
        self.write_register(Motors::all(), &register::ACC, 0xE);
        self.write_register( Motors::all(), &register::DEC, 0xE);

        self.write_register(Motors::all(), &register::MAX_SPEED, 0xE);

        // 10tr/s
        self.write_register( Motors::all(), &register::MIN_SPEED, 40);
        // 2000 step /

    }

    pub fn set_speed_profile(&self, motors: Motors, acc: u32, dec: u32, max_speed: u32, min_speed: u32){
        self.write_register(motors, &register::ACC, acc);
        self.write_register( motors, &register::DEC, dec);
        self.write_register(motors, &register::MAX_SPEED, max_speed);

        self.write_register( motors, &register::MIN_SPEED, min_speed);
    }

    pub fn get_status(&self, motors: Motors){
        self.send_byte(motors, 0xD0);

        // Rend two byte
        self.send_byte(motors, 0xFF);
        self.send_byte(motors, 0xFF);

    }

    pub fn send_move(&self, motors: Motors, dir: Direction, step: u32){
        let mut command = 0x40u8;

        if dir == Direction::CW {
            command += 1;
        }
        self.send_byte(motors, command );

        let mut buf= [0;4];
        BigEndian::write_u32(&mut buf, step);

        // Send the three LSB
        self.send_byte(motors, buf[1] );
        self.send_byte(motors, buf[2] );
        self.send_byte(motors, buf[3] );
    }

    pub fn send_goto(&self, motors: Motors,  pos: u32){
        let mut command = 0x60u8;

        self.send_byte(motors, command );

        let mut buf= [0;4];
        BigEndian::write_u32(&mut buf, pos);

        // Send the three LSB
        self.send_byte(motors, buf[1] );
        self.send_byte(motors, buf[2] );
        self.send_byte(motors, buf[3] );
    }

    pub fn send_soft_stop(&self, motors: Motors){
        let mut command = 0xB0u8;

        self.send_byte(motors, command );
    }

    pub fn send_hard_stop(&self, motors: Motors){
        let mut command = 0xB8u8;

        self.send_byte(motors, command );
    }


    pub fn send_soft_hiz(&self, motors: Motors){
        let mut command = 0xA0u8;

        self.send_byte(motors, command );
    }

    pub fn send_hard_hiz(&self, motors: Motors){
        let mut command = 0xA8u8;

        self.send_byte(motors, command );
    }


    pub fn send_goto_dir(&self, motors: Motors, dir: Direction, pos: u32){
        let mut command = 0x68u8;

        if dir == Direction::CW {
            command += 1;
        }

        self.send_byte(motors, command );

        let mut buf= [0;4];
        BigEndian::write_u32(&mut buf, pos);

        // Send the three LSB
        self.send_byte(motors, buf[1] );
        self.send_byte(motors, buf[2] );
        self.send_byte(motors, buf[3] );
    }

    pub fn send_run(&self, motors: Motors, dir: Direction, speed: u32){
        let mut command = 0x50u8;

        if dir == Direction::CW {
            command += 1;
        }
        self.send_byte(motors, command );

        let mut buf= [0;4];
        BigEndian::write_u32(&mut buf, speed);

        // Send the three LSB
        self.send_byte(motors, buf[1] );
        self.send_byte(motors, buf[2] );
        self.send_byte(motors, buf[3] );
    }

    pub fn send_go_until(&self, motors: Motors, dir: Direction, speed: u32){
        let mut command = 0b10001010;

        if dir == Direction::CW {
            command += 1;
        }

        self.send_byte(motors, command );

        let mut buf= [0;4];
        BigEndian::write_u32(&mut buf, speed);

        // Send the three LSB
        self.send_byte(motors, buf[1] );
        self.send_byte(motors, buf[2] );
        self.send_byte(motors, buf[3] );
    }

    pub fn send_reset(&self, motors: Motors){
        self.send_byte(motors, 0xC0);
        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);
    }

    pub fn resync_com(&self) {
        // Some commande can take 1, 2, 3 or even 4 byte
        // In case of communication failure, some L6470 can be out of sync

        // By sending command NOP 0x00 4 times to every node, we can recover from such error
        self.send_byte(Motors::all(), 0x00);
        self.send_byte(Motors::all(), 0x00);
        self.send_byte(Motors::all(), 0x00);
        self.send_byte(Motors::all(), 0x00);
    }

    /// Change step mode
    ///
    /// # Side effect:
    ///
    /// Reset ABS_POS to 0
    fn set_step_mode(&self, motors: Motors, step_mode: StepMode){

        self.write_register(motors, &register::STEP_MODE, step_mode.bits as u32 );

        // From the datasheet:
        // Warning: Every time STEP_SEL is changed, the value in the ABS_POS
        //register loses meaning and should be reset.
        //self.write_register(motors, &register::ABS_POS, 0 as u32);
    }

    pub fn send_byte(&self, motors: Motors, byte: u8){
        let mut buf = [0,8];

        if self.daisy_chain >= 8 {
            unimplemented!();
        }

        for i in 0..self.daisy_chain {
            if motors.bits & (1<<i) != 0 {
                buf[i as usize] = byte;
            } else {
                buf[i as usize] = 0; // 0 is NOP command
            }
        }

        self.transfer(&buf[0..self.daisy_chain as usize]);
    }

    pub fn write_register(&self, motors: Motors, reg: &Register, value: u32 ) {

        self.send_byte(motors, 0x00 | reg.address );

        let mut buf= [0;4];

        BigEndian::write_u32(&mut buf, value);

        // We check len_bit to choose between 1, 2 and 3 bytes register
        let buf_size = match reg.len_bit {
            1..=8 => 1,
            9..=16 => 2,
            17..=24 => 3,
            _ => unreachable!(),
        };


        // Parameters need to be sended in MSB order, with 1, 2 or 3 bytes
        // We need to reorder a bit when we only have to save 1 or 2 bytes.
        match buf_size {
            1 => {
                self.send_byte(motors, buf[3]);
            },
            2 => {
                self.send_byte(motors, buf[2]);
                self.send_byte(motors, buf[3]);
            },
            3 => {
                self.send_byte(motors, buf[1]);
                self.send_byte(motors, buf[2]);
                self.send_byte(motors, buf[3]);
            },
            _ => unreachable!(),
        }
    }

    /*
    pub fn get_register() {
       //         self.send_byte(motors, 0b0010_0000 | reg.address );
    }*/

    pub fn transfer(&self, input: &[u8]) -> io::Result<Vec<u8>> {

        let mut rx_buf = input.to_vec();
        {
            let mut transfer = SpidevTransfer::read_write(&input, &mut rx_buf);
            try!(self.spi.transfer(&mut transfer));
        }
        let ten_millis = time::Duration::from_millis(1);
        thread::sleep(ten_millis);
        println!("tx: {:02X?}, rx: {:02X?}", input, rx_buf);
        Ok(rx_buf)
    }

}

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
                .max_speed_hz(20_000) // 5_000_000 max
                .mode(SPI_MODE_3)
                .build(),
            daisy_chain: 2,
        }
    }

    /// Set the maximum SPI transfer speed in Hz
    ///
    /// The controller can't necessarily assign that specific clock speed
    pub fn set_spi_clock_hz(&mut self, clock_hz: u32){
        self.spi_options.max_speed_hz(clock_hz);
    }


    pub fn build(self) -> io::Result<L6470> {
        let mut spi = self.spi_bus?;
        spi.configure(&self.spi_options)?;
        Ok(L6470{
            spi: spi,
            daisy_chain: self.daisy_chain,
        })
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
