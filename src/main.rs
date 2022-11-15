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
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    
    let mut delay = cp.SYST.delay(&clocks);

    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // let mut dir = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    // let mut step = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);

    let mut stepper_driver = StepperDriver::new(
        gpiob.pb13.into_push_pull_output(&mut gpiob.crh),
        gpiob.pb12.into_push_pull_output(&mut gpiob.crh)
    );

    let mut stepper_driver2 = StepperDriver::new(
        gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
        gpiob.pb5.into_push_pull_output(&mut gpiob.crl)
    );
    
    let mut enable1 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let mut enable2 = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);


    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        // led.set_high();
        // delay.delay_ms(1_000_u16);
        led.set_low();
        // delay.delay(1.secs());
        enable1.set_low();
        enable2.set_low();

        stepper_driver.step(&mut delay);
        stepper_driver2.step(&mut delay);
        delay.delay_ms(50_u16);
    }
}
