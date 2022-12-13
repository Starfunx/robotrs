#![no_std]

pub trait GlobalClock: Sync {
    fn micros(&self) -> u32;


}

pub trait Clock {
    fn micros(&mut self) -> u32;
}


#[macro_export]
macro_rules! micros {

    // NEW!
    () => {
        unsafe {
            extern "Rust" {
                static CLOCK: &'static dyn $crate::GlobalClock;
            }

            $crate::GlobalClock::micros(CLOCK)
        }
    };

    ($clock:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log"]

        $crate::Clock::micros(&mut $clock)
    }};

}

// NEW!
#[macro_export]
macro_rules! global_clock {
    ($clock:expr) => {
        #[no_mangle]
        pub static CLOCK: &dyn $crate::GlobalClock = &$clock;
    };
}

#[macro_export]
macro_rules! get_clock {
    // NEW!
    () => {
        unsafe {
            extern "Rust" {
                static CLOCK: &'static dyn $crate::GlobalClock;
            }

            CLOCK
        }
    };

}

// #[macro_export]
// macro_rules! inc_overflow {
//     // NEW!
//     () => {
//         unsafe {
//             extern "Rust" {
//                 static CLOCK: &'static dyn $crate::GlobalClock;
//             }

//             CLOCK.overflow_count +=1;
//         }
//     };

// }