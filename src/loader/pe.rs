
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom, Result};
use utils::stream::{ReadExt, ReadSeekExt, ReadableStruct};
use byteorder::{ReadBytesExt, LittleEndian};

#[derive(Debug)]
pub struct DataDirectory {
  pub virtual_address: u32,
  pub size: u32
}

#[derive(Debug)]
pub struct SectionHeader {
  pub name: String,
  pub virtual_address: u32,
  pub virtual_size: u32,
  pub real_size: u32,
  pub offset: u32
}

impl ReadableStruct for DataDirectory {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<DataDirectory> {
    let virtual_address = reader.read_u32::<LittleEndian>()?;
    let size = reader.read_u32::<LittleEndian>()?;
    Ok (DataDirectory { virtual_address: virtual_address, size: size })
  }
}

impl ReadableStruct for SectionHeader {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<SectionHeader> {
    let name = reader.read_c_str_padded(8)?;
    let virtual_size = reader.read_u32::<LittleEndian>()?;
    // Skip virtual address
    let virtual_address = reader.read_u32::<LittleEndian>()?;
    let real_size = reader.read_u32::<LittleEndian>()?;
    let offset = reader.read_u32::<LittleEndian>()?;
    // Skip the rest
    reader.seek(SeekFrom::Current(16))?;
    Ok (SectionHeader {name, virtual_address, virtual_size, real_size, offset})
  }
}

#[derive(Debug)]
pub struct Section {
  pub header: SectionHeader,
  pub data: Vec<u8>
}

impl Section {
  pub fn resolve_rva(&self, rva: u32) -> u32 {
    rva - self.header.virtual_address
  }
}

#[derive(Debug)]
pub struct PEFile {
  pub sections: HashMap<String, Section>
}

impl PEFile {
  pub fn rva_to_section_offset(&self, rva: u32) -> Option<(&Section, u32)> {
    let section = self.sections.values().find(|s| s.header.virtual_address <= rva &&
                                                  s.header.virtual_address + s.header.real_size >= rva);
    section.map(|s| (s, s.resolve_rva(rva)))
  }

  pub fn read_from<R: Read + Seek>(reader: &mut R) -> Result<PEFile> {
    println!("Reading PE file...");

    let mut dos_magic = [0 as u8; 2];
    reader.read_exact(&mut dos_magic)?;

    assert_eq!(0x4D as u8, dos_magic[0]);
    assert_eq!(0x5A as u8, dos_magic[1]);
    println!("DOS magic ✓");

    reader.seek(SeekFrom::Start(0x3C))?;
    let pe_header_start = reader.read_u32::<LittleEndian>()?;

    println!("PE header pointer: {}", pe_header_start);
    reader.seek(SeekFrom::Start(pe_header_start as u64))?;

    let mut pe_magic = [0 as u8; 4];
    reader.read_exact(&mut pe_magic)?;

    assert_eq!('P' as u8, pe_magic[0]);
    assert_eq!('E' as u8, pe_magic[1]);
    assert_eq!(0 as u8, pe_magic[2]);
    assert_eq!(0 as u8, pe_magic[3]);
    println!("PE magic ✓");

    let coff_header_start = pe_header_start + 4;
    let section_count_offset = coff_header_start + 2;

    let coff_opt_header_start = coff_header_start + 20;
    let alignment_offset = coff_opt_header_start + 36;
    let rvas_and_sizes_offset = coff_opt_header_start + 92;

    let section_count = reader.read_u16_at(section_count_offset as u64)?;
    println!("Section count: {}", section_count);
    let alignment = reader.read_u32_at(alignment_offset as u64)?;
    println!("Section alignment: {}", alignment);
    let rva_count = reader.read_u32_at(rvas_and_sizes_offset as u64)?;
    println!("RVA count: {}", rva_count);

    let mut rvas: Vec<DataDirectory> = vec![];

    for i in 0 .. rva_count {
      let rva = DataDirectory::read_from(reader)?;
      println!("RVA {}: {:?}", i, rva);
      rvas.push(rva);
    }

    let mut section_headers: Vec<SectionHeader> = vec![];

    for i in 0 .. (section_count as u64) {
      let section_header = SectionHeader::read_from(reader)?;
      println!("Section {}: {:?}", i, section_header);
      section_headers.push(section_header);
    }

    let mut section_datas: Vec<(String, Section)> = vec![];

    for header in section_headers {
      reader.seek(SeekFrom::Start(header.offset as u64))?;
      let mut buffer = vec![0 as u8; header.virtual_size as usize];
      reader.read_exact(&mut buffer)?;
      let header_name = header.name.clone();
      let section = Section { header, data: buffer };
      section_datas.push((header_name, section));
    }

    let sections = section_datas.into_iter().collect::<HashMap<_, _>>();

    Ok (PEFile { sections })
  }
}
