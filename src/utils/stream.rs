
use std;
use std::io::{Read, Seek, SeekFrom, Result};
use byteorder::{ReadBytesExt, LittleEndian};

pub trait ReadableStruct {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<Self> where Self : std::marker::Sized;
}

pub trait SeekExt 
{
  fn get_seek_pos(&mut self) -> Result<u64>;
  fn skip(&mut self, len: i64) -> Result<u64>;
}

impl<T: Seek> SeekExt for T {
  fn get_seek_pos(&mut self) -> Result<u64> {
    self.seek(SeekFrom::Current(0))
  }

  fn skip(&mut self, len: i64) -> Result<u64> {
    self.seek(SeekFrom::Current(len))
  }
}

pub trait ReadExt {
  /// Reads a (potentially) null-padded string of a known maximum size. Consumes `size` characters.
  fn read_c_str_padded(&mut self, size: usize) -> Result<String>;
  /// Reads a string of an unknown length, which is null-padded to a multiple of `alignment`.
  /// Consumes a number of characters equal to the length of the string plus the padding.
  fn read_c_str_aligned(&mut self, alignment: usize) -> Result<String>;
  // Reads a null-terminated string of unspecified maximum size. 
  fn read_c_str(&mut self) -> Result<String>;
  // Read the exact number of little-endian 16-bit integers to fill the buffer.
  fn read_exact_16(&mut self, buffer: &mut[u16]) -> Result<()>;
}

impl<T: Read> ReadExt for T {
  fn read_c_str_padded(&mut self, size: usize) -> Result<String> {
    let mut buffer = vec![0 as u8; size];
    self.read_exact(&mut buffer)?;
    let real_size = buffer.iter().position(|&x| x == 0).unwrap_or(size);
    buffer.truncate(real_size);
    // We unwrap here, because result types are incompatible
    Ok(String::from_utf8(buffer).unwrap())
  }

  fn read_c_str_aligned(&mut self, alignment: usize) -> Result<String> {
    let mut str_buffer = Vec::with_capacity(alignment * 2);
    let mut buffer = vec![0 as u8; alignment];

    // Read in chunks of alignment
    loop {
      self.read_exact(&mut buffer)?;
      str_buffer.extend_from_slice(buffer.as_slice());
      if buffer[alignment - 1] == 0 {
        break;
      }
    }

    // Remove zeroes
    let real_size = str_buffer.iter().position(|&x| x == 0).unwrap();
    str_buffer.truncate(real_size);

    Ok(String::from_utf8(str_buffer).unwrap())
  }

  fn read_c_str(&mut self) -> Result<String> {
    let mut str_buffer = Vec::with_capacity(16);

    loop {
      let next_char = self.read_u8()?;

      if next_char == 0 {
        break;
      }

      str_buffer.push(next_char);
    }

    Ok(String::from_utf8(str_buffer).unwrap())
  }

  fn read_exact_16(&mut self, buffer: &mut[u16]) -> Result<()> {
    for i in 0 .. buffer.len() {
      buffer[i] = self.read_u16::<LittleEndian>()?
    }

    Ok(())
  }
}

pub trait ReadSeekExt {
  fn read_u16_at(&mut self, offset: u64) -> Result<u16>;
  fn read_u32_at(&mut self, offset: u64) -> Result<u32>;
}

impl<T: Read + Seek> ReadSeekExt for T {
  fn read_u16_at(&mut self, offset: u64) -> Result<u16> {
    self.seek(SeekFrom::Start(offset))?;
    self.read_u16::<LittleEndian>()
  }

  fn read_u32_at(&mut self, offset: u64) -> Result<u32> {
    self.seek(SeekFrom::Start(offset))?;
    self.read_u32::<LittleEndian>()
  }
}
