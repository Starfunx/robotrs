

use cortex_m::delay;
use embedded_hal::digital::v2::OutputPin;

#[derive(Debug, Clone, Copy)]
pub struct StepperDriverConfig {}

impl Default for StepperDriverConfig {
    fn default() -> Self {
        Self {}
    }
}


pub struct StepperDriver<Step, Dir>
where
    Step:   OutputPin,
    Dir:    OutputPin,
{
    step_pin: Step, 
    dir_pin:  Dir,

    _config: StepperDriverConfig,

}



impl<Step, Dir> StepperDriver<Step, Dir> 
where
    Step:   OutputPin,
    Dir:    OutputPin,
{
    pub fn new(step_pin: Step, dir_pin: Dir) -> Self {

        Self {
            dir_pin,
            step_pin,
            _config: StepperDriverConfig::default(),
        }
    }

    pub fn step(&mut self) {
        _ = self.step_pin.set_high();        
        _ = self.step_pin.set_low();      
     }  
}
