use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(IntoPrimitive, FromPrimitive, Debug)]
#[repr(u8)]
pub enum Command {
    Stop = 0x00,
    Ping = 0x01,
    Pong = 0x02,
    SetDuty = 0x0A,
    SetRpm = 0x0B,
    NotifySwitchState = 0x5C,
    NotifyRpm = 0x5D,
    NotifyGamepadState = 0x5E,
    SetControlFreq = 0xAE,
    SetPGain = 0xAF,
    SetIGain = 0xB0,
    SetDGain = 0xB1,
    #[num_enum(default)]
    Unknown,
}
