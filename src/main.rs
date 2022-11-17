
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
// use panic_semihosting as _;

use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};
use stm32f1xx_hal as hal;
use hal::timer::Timer;
use hal::timer::TimerExt;

mod stepper_driver;
use stepper_driver::StepperDriver;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(70.MHz())
        .freeze(&mut flash.acr);
    
    // let mut delay = cp.SYST.delay(&clocks);
    // let mut delay = hal::timer::Timer::syst(cp.SYST, &clocks).delay();
    let mut delay = dp.TIM2.delay_us(&clocks);

    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut stepper_driver = StepperDriver::new(
        gpiob.pb13.into_push_pull_output(&mut gpiob.crh),
        gpiob.pb12.into_push_pull_output(&mut gpiob.crh),
        gpiob.pb14.into_push_pull_output(&mut gpiob.crh)
    );

    let mut stepper_driver2 = StepperDriver::new(
        gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
        gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
        gpiob.pb7.into_push_pull_output(&mut gpiob.crl)
    );
    
    let mut time_counter = Timer::syst(cp.SYST, &clocks).counter_us();
    _ = time_counter.start(1.secs());

    loop {
        let time = time_counter.now();

        if (time.ticks() / 1_000_000)%2 == 0 {
            led.set_high();
        }
        else {
            led.set_low();
        }
        hprintln!("time {}", time.ticks());
        stepper_driver.step(&mut delay);
        stepper_driver2.step(&mut delay);
        delay.delay_us(10000_u16);
    }
}
