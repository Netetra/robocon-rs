use core::time::Duration;

use super::motor::Dir;

pub trait RotaryEncoder {
    fn resolution(&self) -> u32;
}

pub trait Incremental: RotaryEncoder {
    fn get_count(&self) -> u32;
    fn get_dir(&self) -> Dir;
    fn reset_count(&mut self);

    fn rotations(&self) -> f32 {
        if self.get_dir() == Dir::Cw {
            self.get_count() as f32 / self.resolution() as f32
        } else {
            -1. * self.get_count() as f32 / self.resolution() as f32
        }
    }
    fn rpm(&self, dt: Duration) -> f32 {
        self.rotations() / dt.as_secs_f32() * 60.
    }
}

pub trait Absolute: RotaryEncoder {
    fn get_position(&self) -> u32;
}
