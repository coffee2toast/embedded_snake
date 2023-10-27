#![no_main]
#![no_std]
extern crate cortex_m;
extern crate cortex_m_rt as runtime;
extern crate stm32f7;

use core::panic::PanicInfo;
use cortex_m::asm;
use stm32f7::stm32f7x9::Peripherals;

#[no_mangle]
fn main() -> ! {
    let per = Peripherals::take().unwrap();

    // Enable the clock for GPIOB
    per.RCC.ahb1enr.write(|w| w.gpiojen().bit(true));
    asm::nop();
    // Configure pin as output
    per.GPIOJ.moder.write(|w| w.moder13().bits(0b01));

    loop {
        // Toggle the LED output
        per.GPIOJ
            .odr
            .modify(|r, w| w.odr13().bit(r.odr13().bit_is_clear()));

        for _i in 0..100000 {
            asm::nop()
        }
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
