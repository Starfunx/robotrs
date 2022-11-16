

// use cortex_m::delay;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::blocking::delay::DelayUs;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct StepperDriverConfig {}

impl Default for StepperDriverConfig {
    fn default() -> Self {
        Self {}
    }
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
            _config: StepperDriverConfig::default(),
            _delay: PhantomData,
        }
    }

    pub fn step(&mut self, delay:&mut Delay) {
        _ = self.step_pin.set_high();  
        delay.delay_us(1);      
        _ = self.step_pin.set_low();
    }  

    // pub fn update(&mut self, timee: )
}
