#![no_std]

use bitfield_struct::bitfield;
use embedded_hal::blocking::i2c;

#[derive(Debug, PartialEq)]
pub enum Sts32_33DisError<E> {
    I2C(E),
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
}
