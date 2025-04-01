#[derive(Debug, PartialEq)]
pub struct Id(u8);

impl Id {
    pub fn broadcast() -> Self {
        Self(0xFF)
    }
    pub fn is_broadcast(&self) -> bool {
        *self == Self::broadcast()
    }
}

impl From<u8> for Id {
    fn from(value: u8) -> Self {
        Id(value)
    }
}

impl From<Id> for u8 {
    fn from(value: Id) -> Self {
        value.0
    }
}
