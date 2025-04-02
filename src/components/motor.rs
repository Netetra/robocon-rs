use core::ops::Not;

use embedded_hal::{digital::OutputPin, pwm::SetDutyCycle};
use num_enum::{FromPrimitive, IntoPrimitive};

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
    fn cw(&mut self, duty: u16);
    fn ccw(&mut self, duty: u16);
    fn run(&mut self, duty: u16, dir: impl Into<Dir>) {
        if dir.into() == Dir::Cw {
            self.cw(duty);
        } else {
            self.ccw(duty);
        }
    }
}

pub struct DcMotor<PWM: SetDutyCycle, DIR: OutputPin> {
    pwm: PWM,
    dir: DIR,
}

impl<PWM: SetDutyCycle, DIR: OutputPin> DcMotor<PWM, DIR> {
    pub fn new(pwm: PWM, dir: DIR) -> Self {
        Self { pwm, dir }
    }
}

impl<PWM: SetDutyCycle, DIR: OutputPin> Motor for DcMotor<PWM, DIR> {
    fn cw(&mut self, duty: u16) {
        self.dir.set_low().unwrap();
        self.pwm.set_duty_cycle(duty).unwrap();
    }
    fn ccw(&mut self, duty: u16) {
        self.dir.set_high().unwrap();
        self.pwm.set_duty_cycle(duty).unwrap();
    }
}
