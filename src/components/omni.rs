#[allow(unused_imports)]
use micromath::F32Ext;

use super::motor::Motor;

pub struct OmniWheel<M: Motor> {
    motor: M,
    vx: f32,
    vy: f32,
    radius: f32,
}

impl<M: Motor> OmniWheel<M> {
    pub fn new(motor: M, angle: f32, radius: f32) -> Self {
        let (vy, vx) = angle.sin_cos();
        Self {
            motor,
            vx,
            vy,
            radius,
        }
    }
    pub fn run(&mut self, x: f32, y: f32, rotation: f32) {
        let output = self.vx * x + self.vy * y + self.radius * rotation;
        if output >= 0. {
            let duty = output.clamp(u16::MIN.into(), u16::MAX.into()).round() as u16;
            self.motor.cw(duty);
        } else {
            let duty = output.abs().clamp(u16::MIN.into(), u16::MAX.into()).round() as u16;
            self.motor.ccw(duty);
        }
    }
}

pub struct OmniWheels<M: Motor, const N: usize>([OmniWheel<M>; N]);

impl<M: Motor, const N: usize> OmniWheels<M, N> {
    pub fn run(&mut self, x: f32, y: f32, rotation: f32) {
        for wheel in self.0.iter_mut() {
            wheel.run(x, y, rotation);
        }
    }
}

impl<M: Motor, const N: usize> From<[OmniWheel<M>; N]> for OmniWheels<M, N> {
    fn from(value: [OmniWheel<M>; N]) -> Self {
        Self(value)
    }
}
