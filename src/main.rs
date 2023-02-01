#![no_main]
#![no_std]

use defmt::*;
use hello_stm32f401 as _;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[cortex_m_rt::entry]
fn main() -> ! {
    let stm_peripheral = pac::Peripherals::take().unwrap();
    let arm_peripheral = cortex_m::Peripherals::take().unwrap();

    let gpioc = stm_peripheral.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output();

    let rcc = stm_peripheral.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = arm_peripheral.SYST.delay(&clocks);
    let mut state = false;

    loop {
        // On for 1s, off for 1s.
        led.toggle();
        info!("LED State: {}", state as u8);
        state = !state;
        delay.delay_ms(1000_u32);
    }
}
