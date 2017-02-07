
use std::io::{Read, Seek, Cursor, SeekFrom, Result};
use std::collections::HashMap;
use std::iter::FromIterator;

use byteorder::{ReadBytesExt, LittleEndian};

use utils::stream::*;
use loader::pe::{DataDirectory, Section};
use metadata::heap::{StringHeap, UserStringHeap, BlobHeap, Heaps};
use loader::stream::{StreamHeader, MetaDataTablesStream, StreamReader};

use metadata::Metadata;

#[derive(Debug)]
pub struct CLIHeader {
  pub entry_point_token: u32,
  pub strong_name_rva: DataDirectory,
  pub metadata_rva: DataDirectory
}

#[derive(Debug, Clone)]
pub struct MetadataHeader {
  clr_version: String,
  stream_headers: HashMap<String, StreamHeader>
}

#[derive(Debug)]
pub struct MethodBody {

}

#[derive(Debug)]
pub struct CLRImage {
  pub cli_header: CLIHeader,
  pub strong_name_signature: Option<Vec<u8>>,
  pub metadata: Metadata
}

impl ReadableStruct for CLIHeader {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<CLIHeader> {
    let header_size = reader.read_u32::<LittleEndian>()?;
    // The header is always 0x48 bytes
    assert_eq!(0x48, header_size);
    let clr_major = reader.read_u16::<LittleEndian>()?;
    let clr_minor = reader.read_u16::<LittleEndian>()?;
    // These are always 2 and 5
    assert_eq!(2, clr_major);
    assert_eq!(5, clr_minor);

    let metadata_rva = DataDirectory::read_from(reader)?;

    let flags = reader.read_u32::<LittleEndian>()?;
    // 1 = IL only
    assert_eq!(1, flags & 0b1);

    let entry_point_token = reader.read_u32::<LittleEndian>()?; 
    let resources_rva = DataDirectory::read_from(reader)?;
    let strong_name_rva = DataDirectory::read_from(reader)?;
    let code_manager_rva = DataDirectory::read_from(reader)?;
    let vtable_fixups_rva = DataDirectory::read_from(reader)?;
    let export_address_table_jumps_rva = DataDirectory::read_from(reader)?;
    let managed_native_header_rva = DataDirectory::read_from(reader)?;

    println!("Managed entry point: {}", entry_point_token);

    Ok(CLIHeader {entry_point_token, strong_name_rva, metadata_rva})
  }
}

impl ReadableStruct for MetadataHeader {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<MetadataHeader> {
    let signature = reader.read_u32::<LittleEndian>()?;
    assert_eq!(0x424A5342, signature);
    println!("CLR metadata header magic ✓");

    let major_version = reader.read_u16::<LittleEndian>()?;
    let minor_version = reader.read_u16::<LittleEndian>()?;

    println!("CLR metadata version: {}.{}", major_version, minor_version);

    // Skip reserved dword
    reader.read_u32::<LittleEndian>()?;

    let version_length = reader.read_u32::<LittleEndian>()?;
    let version = reader.read_c_str_padded(version_length as usize)?;

    println!("CLR version: {}", version);

    // Skip reserved word
    reader.read_u16::<LittleEndian>()?;

    let stream_count = reader.read_u16::<LittleEndian>()?;
    println!("Stream count: {}", stream_count);

    let mut stream_headers = vec![];

    for _ in 0 .. stream_count {
      let stream_header = StreamHeader::read_from(reader)?;
      println!("{:?}", stream_header);
      stream_headers.push((stream_header.name.clone(), stream_header));
    }

    Ok(MetadataHeader {
      clr_version: version,
      stream_headers: HashMap::from_iter(stream_headers.into_iter())
    })
  }
}

impl CLRImage {
  pub fn from_section(section: &Section) -> Result<CLRImage> {
    let mut reader = Cursor::new(section.data.clone());
    // Skip the CLR loader stub
    reader.skip(8)?;

    let cli_header = CLIHeader::read_from(&mut reader)?;

    let strong_name_signature = if cli_header.strong_name_rva.size > 0 {
      println!("Assembly has a strong name signature (size {})", cli_header.strong_name_rva.size);
      let mut signature_buffer = vec![0 as u8; cli_header.strong_name_rva.size as usize];
      reader.read_exact(&mut signature_buffer)?;
      Some(signature_buffer)
    } else {
      println!("Assembly has no strong name signature, skipping.");
      None
    };

    let metadata_header_offset = cli_header.metadata_rva.virtual_address - section.header.virtual_address;
    reader.seek(SeekFrom::Start(metadata_header_offset as u64))?;
    let metadata_header = MetadataHeader::read_from(&mut reader)?;

    fn read_stream<T: StreamReader, R: Read + Seek>(reader: &mut R, offset: u32, metadata_header: &MetadataHeader, name: &str) -> Result<T> {
      let header = metadata_header.stream_headers.get(name).unwrap();
      reader.seek(SeekFrom::Start((offset + header.offset) as u64))?;
      T::read_from(reader, header)
    }

    let strings: StringHeap = read_stream(&mut reader, metadata_header_offset, &metadata_header, "#Strings")?;
    let user_strings: UserStringHeap = read_stream(&mut reader, metadata_header_offset, &metadata_header, "#US")?;
    let metadata_stream: MetaDataTablesStream = read_stream(&mut reader, metadata_header_offset, &metadata_header, "#~")?;

    let heaps = Heaps {
      strings, user_strings, blobs: BlobHeap { blobs: HashMap::new() }
    };

    let metadata = Metadata { heaps, tables: metadata_stream.tables };

    Ok(CLRImage { cli_header, strong_name_signature, metadata })
  }
}
