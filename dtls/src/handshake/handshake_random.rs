use rand::Rng;

use std::io::{Read, Write};
use std::time::{Duration, SystemTime};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use util::Error;

pub const RANDOM_BYTES_LENGTH: usize = 28;
pub const HANDSHAKE_RANDOM_LENGTH: usize = RANDOM_BYTES_LENGTH + 4;

// https://tools.ietf.org/html/rfc4346#section-7.4.1.2
#[derive(Clone, Debug, PartialEq)]
pub struct HandshakeRandom {
    pub gmt_unix_time: SystemTime,
    pub random_bytes: [u8; RANDOM_BYTES_LENGTH],
}

impl HandshakeRandom {
    pub fn marshal<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let secs = match self.gmt_unix_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(d) => d.as_secs() as u32,
            Err(_) => 0,
        };
        writer.write_u32::<BigEndian>(secs)?;
        writer.write_all(&self.random_bytes)?;

        Ok(())
    }

    pub fn unmarshal<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let secs = reader.read_u32::<BigEndian>()?;
        let gmt_unix_time = if let Some(unix_time) =
            SystemTime::UNIX_EPOCH.checked_add(Duration::new(secs as u64, 0))
        {
            unix_time
        } else {
            SystemTime::UNIX_EPOCH
        };

        let mut random_bytes = [0u8; RANDOM_BYTES_LENGTH];
        reader.read_exact(&mut random_bytes)?;

        Ok(HandshakeRandom {
            gmt_unix_time,
            random_bytes,
        })
    }

    // populate fills the HandshakeRandom with random values
    // may be called multiple times
    pub fn populate(&mut self) {
        self.gmt_unix_time = SystemTime::now();
        rand::thread_rng().fill(&mut self.random_bytes);
    }
}
