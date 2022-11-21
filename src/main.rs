//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

// #![deny(unsafe_code)]
#![no_std]
#![no_main]


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

    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);


    let mut timer = dp.TIM2.counter_us(&clocks);
    timer.start(65535.micros()).unwrap();

    timer.listen(Event::Update);

    let tim = TIM2::ptr();


    // Move the timer into our global storage
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));


    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }


    // Setup gpios
    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        
    loop {
        let time = micros!();
        
        if (time / 1_000_000)%2 == 0 {
            led.set_high();
        }
        else {
            led.set_low();
        }
    }
}


// struct GlobalTime {
//     overflow_count: u32,
// }

global_clock!(Clock);
struct Clock;

impl GlobalClock for Clock {
    fn micros(&self) -> u32 {
        static mut TIM: Option<CounterUs<TIM2>> = None;

        let time: u32;
        unsafe {

            let tim = TIM.get_or_insert_with(|| {
                cortex_m::interrupt::free(|cs| {
                    G_TIM.borrow(cs).replace(None).unwrap()
                })
            });
            time = tim.now().ticks();
        }


        time
    }


    
}


#[interrupt]
fn TIM2() {
    static mut TIM: Option<CounterUs<TIM2>> = None;

    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            G_TIM.borrow(cs).replace(None).unwrap()
        })
    });


    hprintln!("CNT {:?}", tim.now().ticks());

    // let _ = tim.wait();
    tim.clear_interrupt(Event::Update);
}



