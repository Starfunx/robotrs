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
static G_OVF: Mutex<RefCell<Option<u32>>> = Mutex::new(RefCell::new(Some(0 as u32)));


#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .freeze(&mut flash.acr);


    // timer 2
    let mut timer = dp.TIM2.counter_us(&clocks);
    timer.start(65535.micros()).unwrap();
    timer.listen(Event::Update);

    // Move the timer into our global storage
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    // enable tim2 interupt in nvic
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }


    // Setup gpios
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



