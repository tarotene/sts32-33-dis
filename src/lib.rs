#![no_std]

use embedded_hal::i2c;
use bitfield_struct::bitfield;

#[bitfield(u16, order = Msb)]
pub struct StatusRegister {
    #[bits(1, access = RO)]
    pub alert_pending: u8,
    #[bits(1)]
    __: u8,
    #[bits(1, access = RO)]
    heater: u8,
    #[bits(2)]
    __: u8,
    #[bits(1, access = RO)]
    tracking_alert: u8,
    #[bits(5)]
    __: u8,
    #[bits(1, access = RO)]
    system_reset_detected: u8,
    #[bits(2)]
    __: u8,
    #[bits(1, access = RO)]
    command: u8,
    #[bits(1, access = RO)]
    write_data_check_sum: u8,
}

pub struct Sts32_33Dis<I2C>
where I2C: i2c::I2c {
    i2c: I2C,
    address: u8,
}

impl<I2C> Sts32_33Dis<I2C>
where I2C: i2c::I2c {
    pub fn new(i2c: I2C, address: u8) -> Self {
        Sts32_33Dis {
            i2c,
            address,
        }
    }

    pub fn status_register(&mut self) -> Result<StatusRegister, I2C::Error> {
        self.read_status_register()
    }

    fn read_status_register(&mut self) -> Result<StatusRegister, I2C::Error> {
        let mut data = [0; 3];

        self.i2c.write(self.address, &[0xF3, 0x2D])?;
        self.i2c.read(self.address, &mut data)?;

        Ok(StatusRegister::from(u16::from_be_bytes([data[0], data[1]])))
    }
}
