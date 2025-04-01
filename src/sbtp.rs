use embedded_io_async::{Read, Write};
use heapless::Vec;

const GENERATE_POLYNOMIAL: u8 = 0xD5;
const INITIAL_VALUE: u8 = 0xFF;

const SBTP_START_BYTE: u8 = 0x55;
const SBTP_ESCAPE_BYTE: u8 = 0x5A;
const SBTP_END_BYTE: u8 = 0xAA;
const SBTP_XOR_BYTE: u8 = 0x42;
const SBTP_FRAME_MAX_SIZE: usize = u8::MAX as usize + 4;
const SBTP_PAYLOAD_MAX_SIZE: usize = u8::MAX as usize;

fn crc8(data: &[u8]) -> u8 {
    let mut crc = INITIAL_VALUE;
    for d in data {
        crc ^= d;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ GENERATE_POLYNOMIAL;
            } else {
                crc <<= 1;
            }
        }
    }
    crc ^ 0xFF
}

pub enum Error<IO: Read + Write> {
    PayloadOverflow,
    InvalidFormat,
    Crc,
    TransportError(IO::Error),
    Unknown,
}

pub struct Sbtp<IO: Read + Write> {
    transport: IO,
}

impl<IO: Read + Write> Sbtp<IO> {
    pub fn new(transport: IO) -> Self {
        Self { transport }
    }
    pub async fn send(&mut self, data: &[u8]) -> Result<(), Error<IO>> {
        let mut buf = Vec::<u8, SBTP_FRAME_MAX_SIZE>::new();

        if data.len() > SBTP_PAYLOAD_MAX_SIZE {
            return Err(Error::PayloadOverflow);
        }

        let mut payload_len = 0;
        for d in data {
            if *d == SBTP_START_BYTE || *d == SBTP_END_BYTE || *d == SBTP_ESCAPE_BYTE {
                if payload_len > (SBTP_PAYLOAD_MAX_SIZE - 2) {
                    return Err(Error::PayloadOverflow);
                }
                buf.push(SBTP_ESCAPE_BYTE).unwrap();
                buf.push(*d ^ SBTP_XOR_BYTE).unwrap();
                payload_len += 2;
            } else {
                if payload_len > (SBTP_PAYLOAD_MAX_SIZE - 1) {
                    return Err(Error::PayloadOverflow);
                }
                buf.push(*d).unwrap();
                payload_len += 1;
            }
        }
        buf.insert(0, SBTP_START_BYTE).unwrap();
        buf.insert(1, buf.len() as u8).unwrap();
        buf.push(crc8(data)).unwrap();
        buf.push(SBTP_END_BYTE).unwrap();

        if let Err(e) = self.transport.write_all(buf.as_slice()).await {
            return Err(Error::TransportError(e));
        }

        Ok(())
    }
    pub async fn receive(&mut self) -> Result<Vec<u8, SBTP_PAYLOAD_MAX_SIZE>, Error<IO>> {
        let mut payload = Vec::new();

        loop {
            if self.read_byte().await? == SBTP_START_BYTE {
                break;
            }
        }

        let len = self.read_byte().await?;
        for _ in 0..len {
            let byte = self.read_byte().await?;
            if byte == SBTP_ESCAPE_BYTE {
                payload
                    .push(self.read_byte().await? ^ SBTP_XOR_BYTE)
                    .unwrap();
            } else {
                payload.push(byte).unwrap();
            }
        }
        let crc = self.read_byte().await?;

        if self.read_byte().await? != SBTP_END_BYTE {
            return Err(Error::InvalidFormat);
        }

        if crc != crc8(&payload) {
            return Err(Error::Crc);
        }

        Ok(payload)
    }
    async fn read_byte(&mut self) -> Result<u8, Error<IO>> {
        let mut buf = [0u8];
        match self.transport.read(&mut buf).await {
            Ok(len) => {
                if len != 1 {
                    return Err(Error::Unknown);
                }
                Ok(buf[0])
            }
            Err(e) => Err(Error::TransportError(e)),
        }
    }
}
