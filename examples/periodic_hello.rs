#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use stm32f3xx_hal::{pac, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let _clocks = {
        let mut flash = dp.FLASH.constrain();
        let rcc = dp.RCC.constrain();

        rcc
            .cfgr
            .hclk(48.MHz()) // AHB bus (HCLK) frequency
            .sysclk(48.MHz()) // System clock (SYSCLK) frequency
            .pclk1(12.MHz()) // APB1 peripheral clock
            .pclk2(12.MHz()) // APB2 peripheral clock
            .freeze(&mut flash.acr)
    };

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 48_000_000);

    loop {
        defmt::println!("Hello, world!");

        delay.delay_ms(1000);
    }
}
