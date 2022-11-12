//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};

mod stepper_driver;
use stepper_driver::StepperDriver;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let mut delay = cp.SYST.delay(&clocks);
    let mut delay = dp.TIM2.delay_us(&clocks);
    
    // Store the GPIOA in the mutex, moving it.
    // interrupt::free(|cs| MY_GPIO.borrow(cs).replace(Some(delay)));
    // We can no longer use `gpioa` or `dp.GPIOA`, and instead have to
    // access it via the mutex.

    // Acquire the GPIOC peripheral
    let mut gpiob = dp.GPIOB.split();
    // let mut gpioc = dp.GPIOC.split();

    // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut dir = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let mut step = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);

    let mut stepper_driver = StepperDriver::new(dir, step);

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        // led.set_high();
        // delay.delay_ms(1_000_u16);
        // led.set_low();
        stepper_driver.step();
        // delay.delay(1.secs());
    }
}
