#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m::delay;

use stm32f3xx_hal as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut delay = delay::Delay::new(cp.SYST, 8_000_000);

    loop {
        defmt::println!("Hello, world!");

        delay.delay_ms(1000);
    }
}
