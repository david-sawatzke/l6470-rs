//! #Documents
//!
//! General description of protocol https://www.st.com/content/ccc/resource/technical/document/application_note/d7/dd/a8/f3/db/3e/49/6f/DM00082126.pdf/files/DM00082126.pdf/jcr:content/translations/en.DM00082126.pdf
#![no_std]

use bitflags::bitflags;
use embedded_hal;
use embedded_hal::blocking::delay;
use embedded_hal::spi::{Mode, Phase, Polarity};

pub mod register;

pub use register::Register;

/// SPI mode that can be used for this crate
pub const MODE: Mode = Mode {
    polarity: Polarity::IdleHigh,
    phase: Phase::CaptureOnSecondTransition,
};

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

#[derive(PartialEq, Debug)]
pub enum Direction {
    CW,
    CCW,
}

pub struct L6470<SPI, CS> {
    spi: SPI,
    cs: CS,
    daisy_chain: u8,
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

impl<SPI, E, CS> L6470<SPI, CS>
where
    SPI: embedded_hal::blocking::spi::Transfer<u8, Error = E>,
    CS: embedded_hal::digital::v2::OutputPin,
    <CS as embedded_hal::digital::v2::OutputPin>::Error: core::fmt::Debug,
{
    pub fn init<DELAY>(&mut self, delay: &mut DELAY)
    where
        DELAY: delay::DelayMs<u16>,
    {
        self.resync_com();
        self.send_reset(delay, Motors::all());

        // self.write_register(Motors::all(), &register::ALARM_EN, 0b1000_0011 as u32);
        // self.get_status(Motors::all());
        // self.set_step_mode(Motors::all(), StepMode::DIV16);
        // self.init_speed();
    }

    pub fn init_speed(&mut self) {
        // Motor with 1.8° / 200 step per revolution

        // 1 tours / second

        // 200 step / s2 = 60tr/min
        self.write_register(Motors::all(), &register::ACC, 0xE);
        self.write_register(Motors::all(), &register::DEC, 0xE);

        self.write_register(Motors::all(), &register::MAX_SPEED, 0xE);

        // 10tr/s
        self.write_register(Motors::all(), &register::MIN_SPEED, 40);
        // 2000 step /
    }

    pub fn set_speed_profile(
        &mut self,
        motors: Motors,
        acc: u32,
        dec: u32,
        max_speed: u32,
        min_speed: u32,
    ) {
        self.write_register(motors, &register::ACC, acc);
        self.write_register(motors, &register::DEC, dec);
        self.write_register(motors, &register::MAX_SPEED, max_speed);

        self.write_register(motors, &register::MIN_SPEED, min_speed);
    }

    pub fn get_status(&mut self, motors: Motors) {
        self.send_byte(motors, 0xD0);

        // Rend two byte
        self.send_byte(motors, 0xFF);
        self.send_byte(motors, 0xFF);
    }

    pub fn send_move(&mut self, motors: Motors, dir: Direction, step: u32) {
        let mut command = 0x40u8;

        if dir == Direction::CW {
            command += 1;
        }
        self.send_byte(motors, command);

        let buf = step.to_be_bytes();
        // Send the three LSB
        self.send_byte(motors, buf[1]);
        self.send_byte(motors, buf[2]);
        self.send_byte(motors, buf[3]);
    }

    pub fn send_goto(&mut self, motors: Motors, pos: u32) {
        let command = 0x60u8;

        self.send_byte(motors, command);

        let buf = pos.to_be_bytes();

        // Send the three LSB
        self.send_byte(motors, buf[1]);
        self.send_byte(motors, buf[2]);
        self.send_byte(motors, buf[3]);
    }

    pub fn send_soft_stop(&mut self, motors: Motors) {
        let command = 0xB0u8;

        self.send_byte(motors, command);
    }

    pub fn send_hard_stop(&mut self, motors: Motors) {
        let command = 0xB8u8;

        self.send_byte(motors, command);
    }

    pub fn send_soft_hiz(&mut self, motors: Motors) {
        let command = 0xA0u8;

        self.send_byte(motors, command);
    }

    pub fn send_hard_hiz(&mut self, motors: Motors) {
        let command = 0xA8u8;

        self.send_byte(motors, command);
    }

    pub fn send_goto_dir(&mut self, motors: Motors, dir: Direction, pos: u32) {
        let mut command = 0x68u8;

        if dir == Direction::CW {
            command += 1;
        }

        self.send_byte(motors, command);

        let buf = pos.to_be_bytes();

        // Send the three LSB
        self.send_byte(motors, buf[1]);
        self.send_byte(motors, buf[2]);
        self.send_byte(motors, buf[3]);
    }

    pub fn send_run(&mut self, motors: Motors, dir: Direction, speed: u32) {
        let mut command = 0x50u8;

        if dir == Direction::CW {
            command += 1;
        }
        self.send_byte(motors, command);

        let buf = speed.to_be_bytes();

        // Send the three LSB
        self.send_byte(motors, buf[1]);
        self.send_byte(motors, buf[2]);
        self.send_byte(motors, buf[3]);
    }

    pub fn send_go_until(&mut self, motors: Motors, dir: Direction, speed: u32) {
        let mut command = 0b1000_1010;

        if dir == Direction::CW {
            command += 1;
        }

        self.send_byte(motors, command);

        let buf = speed.to_be_bytes();

        // Send the three LSB
        self.send_byte(motors, buf[1]);
        self.send_byte(motors, buf[2]);
        self.send_byte(motors, buf[3]);
    }

    pub fn send_reset<DELAY>(&mut self, delay: &mut DELAY, motors: Motors)
    where
        DELAY: delay::DelayMs<u16>,
    {
        self.send_byte(motors, 0xC0);
        delay.delay_ms(1_000 as u16);
    }

    pub fn resync_com(&mut self) {
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
    pub fn set_step_mode(&mut self, motors: Motors, step_mode: StepMode) {
        self.write_register(motors, &register::STEP_MODE, step_mode.bits as u32);

        // From the datasheet:
        // Warning: Every time STEP_SEL is changed, the value in the ABS_POS
        //register loses meaning and should be reset.
        self.write_register(motors, &register::ABS_POS, 0 as u32);
    }

    pub fn send_byte(&mut self, motors: Motors, byte: u8) {
        self.cs.set_low().unwrap();
        let mut buf = [0, 8];

        if self.daisy_chain >= 8 {
            unimplemented!();
        }

        for i in 0..self.daisy_chain {
            if motors.bits & (1 << i) != 0 {
                buf[i as usize] = byte;
            } else {
                buf[i as usize] = 0; // 0 is NOP command
            }
        }

        self.transfer(&mut buf[0..self.daisy_chain as usize])
            .map_err(|_| ())
            .unwrap();
        self.cs.set_high().unwrap();
    }

    pub fn read_byte(&mut self, motors: Motors) -> u8 {
        self.cs.set_low().unwrap();
        let mut buf = [0; 8]; // 0 is NOP command

        if self.daisy_chain >= 8 {
            unimplemented!();
        }

        self.transfer(&mut buf[0..self.daisy_chain as usize])
            .map_err(|_| ())
            .unwrap();

        let mut data = 0;
        for i in 0..self.daisy_chain {
            if motors.bits & (1 << i) != 0 {
                data = buf[i as usize];
                break;
            }
        }
        self.cs.set_high().unwrap();
        data
    }

    pub fn write_register(&mut self, motors: Motors, reg: &Register, value: u32) {
        self.send_byte(motors, 0x00 | reg.address);

        let buf = value.to_be_bytes();

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
            }
            2 => {
                self.send_byte(motors, buf[2]);
                self.send_byte(motors, buf[3]);
            }
            3 => {
                self.send_byte(motors, buf[1]);
                self.send_byte(motors, buf[2]);
                self.send_byte(motors, buf[3]);
            }
            _ => unreachable!(),
        }
    }

    pub fn read_register(&mut self, motors: Motors, reg: &Register) -> u32 {
        self.send_byte(motors, 0x20 | reg.address);

        let mut buf = [0; 4];

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
                buf[3] = self.read_byte(motors);
            }
            2 => {
                buf[2] = self.read_byte(motors);
                buf[3] = self.read_byte(motors);
            }
            3 => {
                buf[1] = self.read_byte(motors);
                buf[3] = self.read_byte(motors);
                buf[3] = self.read_byte(motors);
            }
            _ => unreachable!(),
        };
        u32::from_be_bytes(buf)
    }

    /*
    pub fn get_register() {
       //         self.send_byte(motors, 0b0010_0000 | reg.address );
    }*/

    pub fn transfer(&mut self, input: &mut [u8]) -> Result<(), E> {
        self.spi.transfer(input)?;
        // TODO 1 ms delay
        // println!("tx: {:02X?}, rx: {:02X?}", input, rx_buf);
        Ok(())
    }
}

/// A connector to on or more L6470
pub struct L6470Connector<SPI, CS> {
    spi_bus: SPI,
    cs: CS,
    daisy_chain: u8,
}

impl<SPI, CS> L6470Connector<SPI, CS>
where
    SPI: embedded_hal::blocking::spi::Transfer<u8>,
    CS: embedded_hal::digital::v2::OutputPin,
    <CS as embedded_hal::digital::v2::OutputPin>::Error: core::fmt::Debug,
{
    /// Returns a new connectors
    ///
    /// 5 MHz max
    pub fn new(spi: SPI, cs: CS, daisy_chain: u8) -> Self {
        L6470Connector {
            spi_bus: spi,
            cs,
            daisy_chain,
        }
    }

    pub fn build(mut self) -> Result<L6470<SPI, CS>, ()> {
        self.cs.set_high().unwrap();
        Ok(L6470 {
            spi: self.spi_bus,
            cs: self.cs,
            daisy_chain: self.daisy_chain,
        })
    }
}
