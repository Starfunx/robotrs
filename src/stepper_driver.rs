/// Example rust-embedded driver
/// 
/// This includes more options than you'll usually need, and is intended
/// to be adapted (read: have bits removed) according to your use case.

extern crate embedded_hal;
use embedded_hal::blocking::delay;
use embedded_hal::digital::v2::OutputPin;


#[derive(Debug, Clone, PartialEq)]
pub enum Error<PinError> {
    /// Underlying GPIO pin error
    Pin(PinError),
    
    /// Device failed to resume from reset
    ResetTimeout
}

/// Driver object is generic over peripheral traits 
/// - You should include a unique type for each pin object as some HALs will export different types per-pin or per-bus
/// 
pub struct StepperDriver<DirPin, StepPin, Delay> {
    /// Device configuration
    // config: Config,

    dir : DirPin,
    step: StepPin,

    /// Delay implementation
    delay: Delay,

}

/// Driver configuration data
pub struct Config {
    /// Device polling time
    pub poll_ms: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_ms: 100,
        }
    }
}

/// Device reset timeout
pub const RESET_TIMEOUT_MS: u32 = 100;

impl<DirPin, StepPin, Delay> StepperDriver <DirPin, StepPin, Delay>
where
    DirPin: OutputPin<>,
    StepPin: OutputPin<>,
    Delay: delay::DelayMs<u32>,
{
    /// Create and initialise a new driver
    pub fn new(dir: DirPin, step: StepPin, delay: Delay) -> Result<Self, Error<()>> {
        // Create the driver object
        let mut s = Self { 
            // config,
            dir, step, delay,
        };


        s.dir.set_low();
        s.step.set_low();
        s.delay.delay_ms(10);

        Ok(s)
    }


}
