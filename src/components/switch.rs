use core::ops::Not;

use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(IntoPrimitive, FromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Switch {
    #[num_enum(default)]
    Off = 0,
    On = 1,
}

impl Not for Switch {
    type Output = Self;
    fn not(self) -> Self::Output {
        if self == Self::Off {
            Self::On
        } else {
            Self::Off
        }
    }
}
