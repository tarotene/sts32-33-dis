#![no_std]

use bitfield_struct::bitfield;
use embedded_hal::blocking::i2c;

#[derive(Debug, PartialEq)]
pub enum Sts32_33DisError<E> {
    I2C(E),
}

#[derive(Debug, PartialEq)]
pub enum Repeatability {
    Low,
    Medium,
    High,
}

pub enum Mps {
    _0_5,
    _1,
    _2,
    _4,
    _10
}

#[bitfield(u16, order = Msb)]
pub struct StatusRegister {
    #[bits(1, access = RO)]
    pub alert_pending: u8,
    #[bits(1)]
    __: u8,
    #[bits(1, access = RO)]
    pub heater: u8,
    #[bits(2)]
    __: u8,
    #[bits(1, access = RO)]
    pub tracking_alert: u8,
    #[bits(5)]
    __: u8,
    #[bits(1, access = RO)]
    pub system_reset_detected: u8,
    #[bits(2)]
    __: u8,
    #[bits(1, access = RO)]
    pub command: u8,
    #[bits(1, access = RO)]
    pub write_data_check_sum: u8,
}

pub struct Sts32_33DisDriver<I2C>
where
    I2C: i2c::Read + i2c::Write + i2c::WriteRead,
{
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Sts32_33DisDriver<I2C>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Sts32_33DisDriver { i2c, address }
    }

    pub fn status_register(&mut self) -> Result<StatusRegister, Sts32_33DisError<E>> {
        self.read_status_register()
    }

    fn read_status_register(&mut self) -> Result<StatusRegister, Sts32_33DisError<E>> {
        let mut data = [0; 3];

        self.i2c.write(self.address, &[0xF3, 0x2D]).map_err(|e| Sts32_33DisError::I2C(e))?;
        self.i2c.read(self.address, &mut data).map_err(|e| Sts32_33DisError::I2C(e))?;

        Ok(StatusRegister::from(u16::from_be_bytes([data[0], data[1]])))
    }

    // FIXME: pub と non-pub をイイ感じにする
    pub fn start_periodic_measurement(&mut self, repeatability: Repeatability, mps: Mps) -> Result<(), Sts32_33DisError<E>> {
        let msb = match mps {
            Mps::_0_5 => 0x20,
            Mps::_1 => 0x21,
            Mps::_2 => 0x22,
            Mps::_4 => 0x23,
            Mps::_10 => 0x27,
        };

        let lsb = match (repeatability, mps) {
            (Repeatability::High, Mps::_0_5) => 0x32,
            (Repeatability::Medium, Mps::_0_5) => 0x24,
            (Repeatability::Low, Mps::_0_5) => 0x2F,
            (Repeatability::High, Mps::_1) => 0x30,
            (Repeatability::Medium, Mps::_1) => 0x26,
            (Repeatability::Low, Mps::_1) => 0x2D,
            (Repeatability::High, Mps::_2) => 0x36,
            (Repeatability::Medium, Mps::_2) => 0x20,
            (Repeatability::Low, Mps::_2) => 0x2B,
            (Repeatability::High, Mps::_4) => 0x34,
            (Repeatability::Medium, Mps::_4) => 0x22,
            (Repeatability::Low, Mps::_4) => 0x29,
            (Repeatability::High, Mps::_10) => 0x37,
            (Repeatability::Medium, Mps::_10) => 0x21,
            (Repeatability::Low, Mps::_10) => 0x2A,
        };

        self.i2c.write(self.address, &[msb, lsb]).map_err(|e| Sts32_33DisError::I2C(e))?;

        Ok(())
    }

    // FIXME: pub と non-pub をイイ感じにする
    pub fn stop_periodic_measurement(&mut self) -> Result<(), Sts32_33DisError<E>> {
        self.i2c.write(self.address, &[0x30, 0x93]).map_err(|e| Sts32_33DisError::I2C(e))?;

        Ok(())
    }

    // FIXME: pub と non-pub をイイ感じにする
    pub fn fetch_data(&mut self) -> Result<[u8; 2], Sts32_33DisError<E>> {
        let mut data = [0; 3];

        self.i2c.write(self.address, &[0xE0, 0x00]).map_err(|e| Sts32_33DisError::I2C(e))?;
        self.i2c.read(self.address, &mut data).map_err(|e| Sts32_33DisError::I2C(e))?;

        // TODO: Check CRC

        Ok(data[0..2].try_into().unwrap())
    }
}
