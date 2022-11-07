/// Example rust-embedded driver
/// 
/// This includes more options than you'll usually need, and is intended
/// to be adapted (read: have bits removed) according to your use case.

use std::marker::PhantomData;

extern crate embedded_hal;
use embedded_hal::blocking::{delay, spi, i2c};
use embedded_hal::digital::v2::{InputPin, OutputPin};



#[derive(Debug, Clone, PartialEq)]
pub enum Error<I2cError, SpiError, PinError> {
    /// Underlying GPIO pin error
    Pin(PinError),
    
    /// Device failed to resume from reset
    ResetTimeout
}

/// Driver object is generic over peripheral traits 
/// - You should include a unique type for each pin object as some HALs will export different types per-pin or per-bus
/// 
pub struct StepperDriver<DirPin, StepPin, PinError, Delay> {
    /// Device configuration
    config: Config,

    dir : DirPin,
    step: StepPin,

    /// Delay implementation
    delay: Delay,

    // Error types must be bound to the object
    _pin_err: PhantomData<PinError>,
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

impl<DirPin, StepPin, PinError, Delay> StepperDriver <DirPin, StepPin, PinError, Delay>
where
    DirPin: OutputPin<Error = PinError>,
    StepPin: OutputPin<Error = PinError>,
    Delay: delay::DelayMs<u32>,
{
    /// Create and initialise a new driver
    pub fn new(config: Config, dir: DirPin, step: StepPin, delay: Delay) -> Result<Self, Error<PinError>> {
        // Create the driver object
        let mut s = Self { 
            config, dir, step, delay,
            _pin_err: PhantomData,
        };


        s.dir.set_low().map_err(Error::Pin)?;
        s.step.set_low().map_err(Error::Pin)?;
        s.delay.delay_ms(10);

        Ok(s)
    }


}
