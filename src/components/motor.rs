use core::ops::Not;

use num_enum::{FromPrimitive, IntoPrimitive};

pub struct Duty(u16);

impl From<u16> for Duty {
    fn from(value: u16) -> Self {
        Duty(value)
    }
}

impl From<Duty> for u16 {
    fn from(value: Duty) -> Self {
        value.0
    }
}

#[derive(IntoPrimitive, FromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Dir {
    Cw = 0,
    #[num_enum(default)]
    Ccw = 1,
}

impl Not for Dir {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self == Dir::Cw {
            Dir::Ccw
        } else {
            Dir::Cw
        }
    }
}

pub trait Motor {
    fn cw(&mut self, duty: impl Into<Duty>);
    fn ccw(&mut self, duty: impl Into<Duty>);
    fn run(&mut self, duty: impl Into<Duty>, dir: impl Into<Dir>) {
        if dir.into() == Dir::Cw {
            self.cw(duty);
        } else {
            self.ccw(duty);
        }
    }
}
