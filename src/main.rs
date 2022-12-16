
// #![deny(unsafe_code)]
#![no_std]
#![no_main]


// use panic_halt as _;
use panic_semihosting as _;

// use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};
use stm32f1xx_hal as hal;
use hal::timer::TimerExt;

mod stepper_driver;
use stepper_driver::StepperDriver;

// use cortex_m_semihosting::hprintln;

use pac::{interrupt, Interrupt, TIM2};

use hal::timer::{Event, CounterUs};


mod time;
use time::{GlobalClock};

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::{interrupt::Mutex};

// Make timer interrupt registers globally available
static G_TIM: Mutex<RefCell<Option<CounterUs<TIM2>>>> = Mutex::new(RefCell::new(None));
static G_OVF: Mutex<RefCell<Option<u16>>> = Mutex::new(RefCell::new(Some(0 as u16)));


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


    let mut timer = dp.TIM2.counter_us(&clocks);
    timer.start(65535.micros()).unwrap();
    timer.listen(Event::Update);

    // Move the timer into our global storage
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }


    let mut delay = dp.TIM3.delay_us(&clocks);


    // Setup gpios
    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut stepper_driver_left = StepperDriver::new(
        gpiob.pb13.into_push_pull_output(&mut gpiob.crh),
        gpiob.pb12.into_push_pull_output(&mut gpiob.crh),
        gpiob.pb14.into_push_pull_output(&mut gpiob.crh)
    );

    let mut stepper_driver_right = StepperDriver::new(
        gpiob.pb6.into_push_pull_output(&mut gpiob.crl),
        gpiob.pb5.into_push_pull_output(&mut gpiob.crl),
        gpiob.pb7.into_push_pull_output(&mut gpiob.crl)
    );
    
    stepper_driver_left.setSpeed(800f32);
    stepper_driver_right.setSpeed(800f32);

    loop {

        let time_us = micros!();
        

        if (time_us / 1_000_000)%2 == 0 {
            led.set_high();
        }
        else {
            led.set_low();
        }

        stepper_driver_left.runSpeed(&mut delay);
        stepper_driver_right.runSpeed(&mut delay);
    }
}


struct Clock;

global_clock!(Clock{});

impl GlobalClock for Clock {
    fn micros(&self) -> u32 {
        let (time, ovf_count) = cortex_m::interrupt::free( move |cs| {
            let time: u32 = G_TIM.borrow(cs).borrow().as_ref().unwrap().now().ticks();
            let ovf_count = G_OVF.borrow(cs).borrow_mut().unwrap();
            (time as u32, ovf_count as u32)
        });

        let mut last__time:u32 = ((ovf_count as u32) << 16) + time as u32;

        ((ovf_count as u32) << 16) + time as u32
    }    
}


#[interrupt]
fn TIM2() {
    cortex_m::interrupt::free(|cs| {
        if let Some(ref mut tim) =  G_TIM.borrow(cs).borrow_mut().as_deref_mut() {
            tim.clear_interrupt(Event::Update);
        }
        else {
            panic!()
        }

        if let Some(ref mut ovf_counter) =  G_OVF.borrow(cs).borrow_mut().deref_mut() {
            *ovf_counter += 1;
        }
        else {
            panic!()
        }
    });
}


