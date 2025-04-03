use core::ops::Not;

use embedded_hal::digital::InputPin;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(IntoPrimitive, FromPrimitive, PartialEq)]
#[repr(u8)]
pub enum SwitchState {
    #[num_enum(default)]
    Open = 0,
    Close = 1,
}

impl Not for SwitchState {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self == Self::Open {
            Self::Close
        } else {
            Self::Open
        }
    }
}

pub struct Switch<P: InputPin> {
    pin: P,
    pull_up: bool,
}

impl<P: InputPin> Switch<P> {
    pub fn new(pin: P, pull_up: bool) -> Self {
        Self { pin, pull_up }
    }
    pub fn is_close(&mut self) -> bool {
        if self.pull_up {
            self.pin.is_low().unwrap()
        } else {
            self.pin.is_high().unwrap()
        }
    }
    pub fn is_open(&mut self) -> bool {
        !self.is_close()
    }
}
