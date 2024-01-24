use core::cmp::min;
use core2::io::{Error, Read};

pub struct SliceReader<'a> {
    data: &'a [u8],
}

impl<'a> SliceReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        SliceReader { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

impl<'a> Read for SliceReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let count = min(buf.len(), self.data.len());
        for i in 0..count {
            buf[i] = self.data[i];
        }
        self.data = &self.data[count..];
        Ok(count)
    }
}

pub fn read_u8<R>(reader: &mut R) -> Result<u8, Error>
where
    R: Read,
{
    let mut bytes = [0u8];
    reader.read_exact(&mut bytes)?;
    Ok(bytes[0])
}

pub fn read_u16<R>(reader: &mut R) -> Result<u16, Error>
where
    R: Read,
{
    let mut bytes = [0u8; 2];
    reader.read_exact(&mut bytes)?;
    Ok(u16::from_le_bytes(bytes))
}

pub fn read_u32<R>(reader: &mut R) -> Result<u32, Error>
where
    R: Read,
{
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}
