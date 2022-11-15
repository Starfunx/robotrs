

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


pub struct StepperDriver<Step, Dir, Delay>
where
    Step:   OutputPin,
    Dir:    OutputPin,
{
    step_pin: Step, 
    dir_pin:  Dir,

    _config: StepperDriverConfig,

    _delay: PhantomData<Delay>,

}



impl<Step, Dir, Delay> StepperDriver<Step, Dir, Delay> 
where
    Step:   OutputPin,
    Dir:    OutputPin,
    Delay:  DelayUs<u8>,
{
    pub fn new(step_pin: Step, dir_pin: Dir) -> Self {

        Self {
            dir_pin,
            step_pin,
            _config: StepperDriverConfig::default(),
            _delay: PhantomData,
        }
    }

    pub fn step(&mut self, delay:&mut Delay) {
        _ = self.dir_pin.set_high();
        _ = self.step_pin.set_high();  
        delay.delay_us(1);      
        _ = self.step_pin.set_low();
        // delay.release();
     }  
}
