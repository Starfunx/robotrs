
// #![deny(unsafe_code)]
#![no_std]
#![no_main]


use panic_halt as _;
// use panic_semihosting as _;

// use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};
use stm32f1xx_hal as hal;
use hal::timer::TimerExt;

mod stepper_driver;
use stepper_driver::StepperDriver;

// use panic_halt as _;
use panic_semihosting as _;

use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;

use stm32f1xx_hal as hal;
use hal::{
    prelude::*,
    pac
};
use pac::{interrupt, Interrupt, TIM2};

use hal::timer::TimerExt;
use hal::timer::{Event, CounterUs};


mod time;
use time::{GlobalClock};

use core::cell::RefCell;
use cortex_m::{interrupt::Mutex};

// Make timer interrupt registers globally available
static G_TIM: Mutex<RefCell<Option<CounterUs<TIM2>>>> = Mutex::new(RefCell::new(None));
static G_OVF: Mutex<RefCell<Option<u32>>> = Mutex::new(RefCell::new(Some(0 as u32)));


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
        let rcc = dp.RCC.constrain();
    


    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);


    // Create a delay abstraction based on general-pupose 32-bit timer TIM2

    //let mut delay = hal::timer::FTimerUs::new(dp.TIM2, &clocks).delay();
    // or
    let mut delay = dp.TIM2.delay_us(&clocks);


    // Setup gpios
    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // let mut stepper_driver = StepperDriver::new(
    //     gpiob.pb13.into_push_pull_output(&mut gpiob.crh),
    //     gpiob.pb12.into_push_pull_output(&mut gpiob.crh),
    //     gpiob.pb14.into_push_pull_output(&mut gpiob.crh)
    // );

    // let mut stepper_driver2 = StepperDriver::new(
    //     gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
    //     gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
    //     gpiob.pb7.into_push_pull_output(&mut gpiob.crl)
    // );
    

    let mut syst = cp.SYST;
    // hprintln!("syst clock source {:?}", syst.get_clock_source());
    // hprintln!("ticks per 10ms {:?}", cortex_m::peripheral::SYST::get_ticks_per_10ms());

    let mut dwt = cp.DWT;
    dwt.enable_cycle_counter();

    loop {

        let time_us = micros!();
        

        if (time_us / 1_000_000)%2 == 0 {
            led.set_high();
        }
        else {
            led.set_low();
        }
    }
}


struct Clock{
    overflow_count:u16,
}

global_clock!(Clock{overflow_count: 0 });

impl GlobalClock for Clock {
    fn micros(&self) -> u32 {
        let (time, ovf) = cortex_m::interrupt::free( move |cs| {
            let tim = G_TIM.borrow(cs).borrow();
            let time: u32 = tim.as_ref().unwrap().now().ticks();
            let mut ovf_count = G_OVF.borrow(cs).borrow_mut().unwrap();
            (time, ovf_count)
        });

        ((ovf as u32) << 16) + time as u32
    }    
}


#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        let mut tim = G_TIM.borrow(cs).borrow_mut();
        // gpioa.as_ref().unwrap().idr.read().idr0().bit_is_set()
        tim.as_mut().unwrap().clear_interrupt(Event::Update);
    });


    let value = cortex_m::interrupt::free(|cs| {
        // inc overflow count
        G_OVF.borrow(cs).borrow_mut().unwrap()
    });
    cortex_m::interrupt::free(|cs| {
        // inc overflow count
        *G_OVF.borrow(cs).borrow_mut() = Some(value+1);
    });


}



