#![allow(non_upper_case_globals)]

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Result};

bitflags! {
  // Actually 12 bits
  pub struct MethodHeaderFlags: u16 {
    const TinyFormat   = 0x02;
    const FatFormat    = 0x03;
    const MoreSections = 0x8;
    const InitLocals   = 0x10;
  }
}

#[derive(Debug)]
pub struct MethodHeader {
    flags: MethodHeaderFlags,
    // Present, but useless
    //header_size: u8,
    max_stack: u16,
    code_size: u32,
    local_var_signature_token: u32,
}

impl MethodHeader {
    pub fn read<R: Read>(reader: &mut R) -> Result<MethodHeader> {
        let flags_and_tiny_size = reader.read_u16::<LittleEndian>()?;

        let is_tiny = (flags_and_tiny_size & 0b11) as u16 == MethodHeaderFlags::TinyFormat.bits();

        let flags: MethodHeaderFlags;
        let max_stack: u16;
        let code_size: u32;
        let local_var_signature_token: u32;

        if is_tiny {
            flags = MethodHeaderFlags::TinyFormat;
            code_size = ((flags_and_tiny_size & 0xFC) >> 2) as u32;
            max_stack = 8;
            local_var_signature_token = 0;
        } else {
            flags = MethodHeaderFlags::from_bits(flags_and_tiny_size & 0xFFF).unwrap();
            // header size should be (flags_and_tiny_size >> 12)
            max_stack = reader.read_u16::<LittleEndian>()?;
            code_size = reader.read_u32::<LittleEndian>()?;
            local_var_signature_token = reader.read_u32::<LittleEndian>()?;
        }

        Ok(MethodHeader {
            flags,
            max_stack,
            code_size,
            local_var_signature_token,
        })
    }
}

#[derive(Debug)]
pub struct MethodBody {
    header: MethodHeader,
    code: Vec<u8>,
}

impl MethodBody {
    pub fn read<R: Read>(reader: &mut R) -> Result<MethodBody> {
        let header = MethodHeader::read(reader)?;
        let mut code = vec![0u8; header.code_size as usize];
        reader.read_exact(&mut code)?;

        Ok(MethodBody { header, code })
    }
}
