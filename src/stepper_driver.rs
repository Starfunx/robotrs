

// use cortex_m::delay;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayUs;
use core::marker::PhantomData;

use crate::micros;

#[derive(Debug, Clone, Copy)]
pub struct StepperDriverConfig {}

impl Default for StepperDriverConfig {
    fn default() -> Self {
        Self {}
    }
}


enum Direction {
    Clockwise,
    CounterClockwise
}

pub struct StepperDriver<Step, Dir, Enable, Delay>
where
    Step:   OutputPin,
    Dir:    OutputPin,
    Enable: OutputPin,
{
    step_pin:   Step, 
    dir_pin:    Dir,
    enable_pin: Enable,

    current_pos: i32,
    speed: f32,
    step_interval: u32,
    last_step_time: u32,
    direction: Direction,
    _config: StepperDriverConfig,

    _delay: PhantomData<Delay>,
}



impl<Step, Dir, Enable, Delay> StepperDriver<Step, Dir, Enable, Delay> 
where
    Step:   OutputPin,
    Dir:    OutputPin,
    Enable: OutputPin,
    Delay:  DelayUs<u8>,
{
    pub fn new(step_pin: Step, mut dir_pin: Dir, mut enable_pin: Enable) -> Self {
        _ = dir_pin.set_low();
        _ = enable_pin.set_low(); // enable motor with pin enable\

        Self {
            dir_pin,
            step_pin,
            enable_pin,
            current_pos: 0,
            speed: 0f32,
            step_interval: 0,
            last_step_time: 0,
            direction: Direction::Clockwise,
            _config: StepperDriverConfig::default(),
            _delay: PhantomData,
        }
    }

    pub fn step(&mut self, delay:&mut Delay) {
        _ = self.step_pin.set_high();  
        delay.delay_us(1);      
        _ = self.step_pin.set_low();
    }  

    pub fn setSpeed(&mut self, speed:f32)
    {
        if self.speed == speed {return}

        // speed = constrain(speed, -_maxSpeed, _maxSpeed);
        if speed == 0.0
        {
            self.step_interval = 0;
        }        
        else
        {
            self.step_interval = libm::fabsf(1000000.0 / speed) as u32;
            self.direction = {  
                if speed > 0.0
                    {Direction::Clockwise}
                else
                    {Direction::CounterClockwise}
            };
        }
        self.speed = speed;
    }

    pub fn runSpeed(&mut self, delay:&mut Delay) -> bool
    {
        if self.step_interval == 0 { return false; }
        
            
        let mut time = micros!();   

        // required cause of timer value that can reset while interrupt blocked
        if time < self.last_step_time {
            time += 1<<16;
        }


        if time - self.last_step_time >= self.step_interval
        {
            match self.direction {
                Direction::Clockwise => {
                    self.dir_pin.set_high();
                    self.current_pos += 1;
                },
                Direction::CounterClockwise => {
                    self.dir_pin.set_low();
                    self.current_pos -= 1;
                },

            }

            self.step(delay);
            self.last_step_time = time; // Caution: does not account for costs in step()


        return true;
        }
        return false;
    }
}