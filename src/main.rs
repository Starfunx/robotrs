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
        
        // hprintln!("CNT {:?}", time);

        if (time / 1_000_000)%2 == 0 {
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
        let time = cortex_m::interrupt::free( move |cs| {
            let tim = G_TIM.borrow(cs).borrow();
            let time: u32 = tim.as_ref().unwrap().now().ticks();
            time
        });


        time as u32
    }    
}


#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        let mut tim = G_TIM.borrow(cs).borrow_mut();
        // gpioa.as_ref().unwrap().idr.read().idr0().bit_is_set()
        tim.as_mut().unwrap().clear_interrupt(Event::Update);
        let timer = tim.as_mut().unwrap(); 
        // hprintln!("CNT_int {:?}", timer.now().ticks());
    });

    // inc overflow count

}



