#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;

use rt::entry;
use stm32f4::stm32f401;

#[entry]
fn main() -> ! {
    let peripherals = stm32f401::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let rcc = &peripherals.RCC;

    rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
    rcc.apb1enr.write(|w| w.usart2en().set_bit());
    gpioa.otyper.write(|w| w.ot5().clear_bit());
    gpioa.moder.write(|w| w.moder5().output());
    gpioa.pupdr.write(|w| w.pupdr5().pull_up());
 
    loop {
   gpioa.odr.write(|w| w.odr5().bit(true));
  cortex_m::asm::delay(8000000);
   gpioa.odr.write(|w| w.odr5().bit(false));
  cortex_m::asm::delay(8000000);
  

    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

