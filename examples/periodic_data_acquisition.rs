#![no_std]
#![no_main]

use core::convert::TryInto;

use defmt_rtt as _;
use panic_probe as _;

use stm32f3xx_hal::{self as hal, pac, prelude::*};

use sts32_33_dis::{Sts32_33DisDriver, Repeatability, Mps};

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();

    let _clocks = rcc
        .cfgr
        .hclk(48.MHz()) // AHB bus (HCLK) frequency
        .sysclk(48.MHz()) // System clock (SYSCLK) frequency
        .pclk1(12.MHz()) // APB1 peripheral clock
        .pclk2(12.MHz()) // APB2 peripheral clock
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let i2c = {
        let mut scl = gpiob.pb8.into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
        scl.internal_pull_up(&mut gpiob.pupdr, true);

        let mut sda = gpiob.pb9.into_af_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);
        sda.internal_pull_up(&mut gpiob.pupdr, true);

        hal::i2c::I2c::new(dp.I2C1, (scl, sda), 100.kHz().try_into().unwrap(), _clocks, &mut rcc.apb1)
    };

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 48_000_000);

    let mut sts32_33_dis = Sts32_33DisDriver::new(i2c, 0x4A);

    sts32_33_dis.start_periodic_measurement(Repeatability::High, Mps::_1).unwrap();

    // Wait for the first measurement to be ready
    delay.delay_ms(1000);

    loop {
        let data = sts32_33_dis.fetch_data().unwrap();
        let data = u16::from_be_bytes(data);

        let temperature = -45.0 + 175.0 * f32::from(data) / 65535.0;
        defmt::println!("Temperature: {}", temperature);

        delay.delay_ms(1000);
    }
}
