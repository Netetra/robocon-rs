use bit_field::BitField;
use embedded_can::{ExtendedId, Frame, Id as CanId};
use heapless::Vec;

use super::{command::Command, id::Id};

pub trait ExtendedIdExt {
    type Output;
    fn parse(&self) -> (Id, Id, Command);
    fn build(from: impl Into<Id>, to: impl Into<Id>, command: impl Into<Command>) -> Self::Output;
}

impl ExtendedIdExt for ExtendedId {
    type Output = Self;
    fn parse(&self) -> (Id, Id, Command) {
        let raw_id = self.as_raw();
        let from = (raw_id.get_bits(0..7) as u8).into();
        let to = (raw_id.get_bits(8..15) as u8).into();
        let command = (raw_id.get_bits(16..23) as u8).into();
        (from, to, command)
    }
    fn build(from: impl Into<Id>, to: impl Into<Id>, command: impl Into<Command>) -> Self::Output {
        let from: Id = from.into();
        let to: Id = to.into();
        let command: Command = command.into();

        let raw_from: u8 = from.into();
        let raw_to: u8 = to.into();
        let raw_command: u8 = command.into();

        let mut raw_id = 0u32;
        raw_id.set_bits(0..7, raw_from.into());
        raw_id.set_bits(8..15, raw_to.into());
        raw_id.set_bits(16..23, raw_command.into());

        unsafe { ExtendedId::new_unchecked(raw_id) }
    }
}

pub type Payload<const N: usize> = Vec<u8, N>;

#[derive(Debug)]
pub struct Message<const N: usize> {
    from: Id,
    to: Id,
    command: Command,
    payload: Payload<N>,
}

pub type CanMessage = Message<8>;
pub type EspNowMessage = Message<247>;

impl<const N: usize> Message<N> {
    pub fn new(
        from: impl Into<Id>,
        to: impl Into<Id>,
        command: impl Into<Command>,
        payload: impl Into<Payload<N>>,
    ) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            command: command.into(),
            payload: payload.into(),
        }
    }

    pub fn split(self) -> (Id, Id, Command, Payload<N>) {
        (self.from, self.to, self.command, self.payload)
    }

    pub fn into_vec<const N1: usize>(self) -> Vec<u8, N1> {
        let mut vec = Vec::new();
        vec.push(self.from.into()).unwrap();
        vec.push(self.to.into()).unwrap();
        vec.push(self.command.into()).unwrap();

        for p in self.payload.into_iter() {
            vec.push(p).unwrap()
        }
        vec
    }
    pub fn from_slice(slice: &[u8]) -> Option<Self> {
        let from = (*slice.first()?).into();
        let to = (*slice.get(1)?).into();
        let command = (*slice.get(2)?).into();
        let payload = Vec::from_slice(slice.get(3..)?).ok()?;
        Some(Self {
            from,
            to,
            command,
            payload,
        })
    }
}

impl CanMessage {
    pub fn from_frame(frame: impl Frame) -> Option<Self> {
        if let CanId::Extended(can_id) = frame.id() {
            let (from, to, command) = can_id.parse();
            let payload = Vec::from_slice(frame.data()).unwrap();
            Some(Self {
                from,
                to,
                command,
                payload,
            })
        } else {
            None
        }
    }
    pub fn into_frame<F: Frame>(self) -> F {
        let id = ExtendedId::build(self.from, self.to, self.command);
        let data = self.payload;
        Frame::new(id, &data).unwrap()
    }
}

impl EspNowMessage {
    pub fn into_esp_now_data(self) -> Vec<u8, 250> {
        let mut vec = Vec::new();
        vec.push(self.from.into()).unwrap();
        vec.push(self.to.into()).unwrap();
        vec.push(self.command.into()).unwrap();

        for p in self.payload.into_iter() {
            vec.push(p).unwrap()
        }
        vec
    }
}
