
use std::io::{Read, Seek, Result};
use std::collections::HashMap;
use enum_primitive::FromPrimitive;
use byteorder::{ReadBytesExt, LittleEndian};

use utils::stream::*;
use loader::tables::{ModuleEntry, TableEntryReader};

#[derive(Debug, Clone)]
pub struct StreamHeader {
  pub offset: u32,
  pub size: u32,
  pub name: String
}

impl ReadableStruct for StreamHeader {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<StreamHeader> {
    let offset = reader.read_u32::<LittleEndian>()?;
    let size = reader.read_u32::<LittleEndian>()?;
    let name = reader.read_c_str_aligned(4)?;

    Ok(StreamHeader { offset, size, name })
  }
}

/// The #~ stream.
#[derive(Debug, Clone)]
pub struct MetaDataTablesStream {

}

/// The #String stream.
#[derive(Debug, Clone)]
pub struct AsciiStringsStream {
  pub strings: Vec<String>
}

#[derive(Debug, Clone)]
pub enum UserString {
  Valid(String),
  Garbage
}

/// The #US stream.
#[derive(Debug, Clone)]
pub struct UserStringsStream {
  pub strings: Vec<UserString>
}

/// The #Blob stream.
#[derive(Debug, Clone)]
pub struct BlobStream {

}

/// The #GUID stream.
#[derive(Debug, Clone)]
pub struct GuidStream {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IndexSize {
  Word,
  Dword
}

impl IndexSize {
  pub fn to_u32(self) -> u32 {
    match self {
      IndexSize::Word => u16::max_value() as u32,
      IndexSize::Dword => u32::max_value()
    }
  }
}

impl From<bool> for IndexSize {
  fn from(x: bool) -> IndexSize {
    if x {IndexSize::Dword} else {IndexSize::Word} 
  }
}

#[derive(Debug)]
pub struct HeapOffsetSizes {
  pub string_index: IndexSize,
  pub guid_index: IndexSize,
  pub blob_index: IndexSize
}

enum_from_primitive! {
  #[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
  pub enum TableId {
    Module                  = 00,
    Field                   = 04,
    InterfaceImpl           = 09,
    CustomAttribute         = 12,
    ClassLayout             = 15,
    EventMap                = 18,
    Property                = 23,
    ModuleRef               = 26,
    FieldRVA                = 29,
    AssemblyOS              = 34,
    AssemblyRefOS           = 37,
    ManifestResource        = 40,
    GenericParamConstraint  = 44,
    TypeRef                 = 01,
    TypeDef                 = 02,
    MethodDef               = 06,
    Param                   = 08,
    MemberRef               = 10,
    Constant                = 11,
    FieldMarshal            = 13,
    DeclSecurity            = 14,
    FieldLayout             = 16,
    StandAloneSig           = 17,
    Event                   = 20,
    PropertyMap             = 21,
    MethodSemantics         = 24,
    MethodImpl              = 25,
    TypeSpec                = 27,
    ImplMap                 = 28,
    Assembly                = 32,
    AssemblyProcessor       = 33,
    AssemblyRef             = 35,
    AssemblyRefProcessor    = 36,
    File                    = 38,
    ExportedType            = 39,
    NestedClass             = 41,
    GenericParam            = 42
  }
}

#[derive(Debug)]
pub struct TableIds(Vec<TableId>);

impl From<u64> for TableIds {
  fn from(x: u64) -> TableIds {
    let mut table_ids = vec![];

    for i in 0 .. 64  {
      if ((1 << i) & x) > 0 {
        let table_id_opt = TableId::from_u8(i);
        if let Some(table_id) = table_id_opt {
          table_ids.push(table_id);
        } else {
          panic!("Unknown metadata table: {:02x}", i);
        }
      }
    }

    TableIds(table_ids)
  }
}

pub type RowCounts = HashMap<TableId, u32>;
pub type IndexSizes = HashMap<TableId, IndexSize>;

impl From<u8> for HeapOffsetSizes {
  fn from(x: u8) -> HeapOffsetSizes {
    let str_index_bit = (x & 0b1) > 0;
    let guid_index_bit = (x & 0b10) > 0;
    let blob_index_bit = (x & 0b100) > 0;

    HeapOffsetSizes {
      string_index: IndexSize::from(str_index_bit),
      guid_index: IndexSize::from(guid_index_bit),
      blob_index: IndexSize::from(blob_index_bit)
    }
  }
}

impl MetaDataTablesStream {
  fn get_index_sizes(row_counts: &HashMap<TableId, u32>) -> IndexSizes {
    row_counts.iter().map(|(&k, &v)|
      (k.clone(), if v > (u16::max_value() as u32) { IndexSize::Dword } else { IndexSize::Word } )).collect()
  }
}

impl StreamReader for MetaDataTablesStream {
  fn read_from<R: Read + Seek>(reader: &mut R, header: &StreamHeader) -> Result<MetaDataTablesStream> {
    // Reserved, not used
    assert_eq!(0, reader.read_u32::<LittleEndian>()?);

    let major_version = reader.read_u8()?;
    let minor_version = reader.read_u8()?;
    println!("#~ version: {}.{}", major_version, minor_version);

    let heap_offset_sizes = HeapOffsetSizes::from(reader.read_u8()?);
    println!("{:?}", heap_offset_sizes);

    // Reserved, not used. Specified to always be 1, but mcs seems to emit binaries with it set to 0x10.
    reader.read_u8()?;

    let table_ids = TableIds::from(reader.read_u64::<LittleEndian>()?);
    println!("Metadata tables: {:?}", table_ids);

    let sorted_table_ids = TableIds::from(reader.read_u64::<LittleEndian>()?);
    println!("Metadata tables (sorted): {:?}", sorted_table_ids);

    let mut table_row_counts_vec = vec![];

    for table_id in table_ids.0 {
      let table_count = reader.read_u32::<LittleEndian>()?;
      table_row_counts_vec.push((table_id, table_count));
    }

    let table_rows_counts = table_row_counts_vec.into_iter().collect::<HashMap<TableId, u32>>();
    println!("Metadata table row counts: {:?}", table_rows_counts);

    let index_sizes = MetaDataTablesStream::get_index_sizes(&table_rows_counts);
    println!("Metadata table index sizes: {:?}", index_sizes);

    let module = ModuleEntry::read_entry(reader, heap_offset_sizes, index_sizes)?;
    println!("Module: {:?}", module);

    Ok(MetaDataTablesStream { })
  }
}

pub trait StreamReader {
  fn read_from<R: Read + Seek>(reader: &mut R, header: &StreamHeader) -> Result<Self> where Self : Sized;
}

impl StreamReader for AsciiStringsStream {
  fn read_from<R: Read + Seek>(reader: &mut R, header: &StreamHeader) -> Result<AsciiStringsStream> {
    let mut strings: Vec<String> = vec![];
    let mut bytes_read: usize = 0;

    while bytes_read < (header.size as usize) {
      let string_buffer = reader.read_c_str()?;
      bytes_read += string_buffer.len() + 1;
      strings.push(string_buffer);
    }

    Ok(AsciiStringsStream { strings })
  }
}

struct StreamUtils { }

pub struct CompressedUint {
  pub value: u32,
  pub compressed_size: u8
}

impl StreamUtils {
  // ECMA 335, page 272
  // Inspired by
  // https://github.com/jbevain/cecil/blob/505b07d6974d8405a63124139733c6fdc0e67bc7/Mono.Cecil.PE/ByteBuffer.cs#L101
  pub fn decode_compressed_int<R: Read + Seek>(reader: &mut R) -> Result<CompressedUint> {
    let first_byte = reader.read_u8()?;

    // Starts with a zero bit -> bits 1-7 are the length
    let (value, compressed_size) = if (first_byte & 0x80) == 0 {
      (first_byte as u32, 1)
    // Starts with 0b10 -> bits 2-7 + the next byte is the length
    } else if (first_byte & 0x40) == 0 {
      ((((first_byte & !0x80) as u32) << 8) | (reader.read_u8()? as u32), 2)
    // We assume the blob starts with 0b110 -> bits 3-7 + the next 3 bytes is the length 
    } else {
      let mut rest = [0u8; 3];
      reader.read_exact(&mut rest)?;
      ((((first_byte & !0xc0) as u32) << 24)
        | ((rest[0] as u32) << 16)
        | ((rest[1] as u32) << 8)
        | (rest[2] as u32), 4)
    };

    Ok(CompressedUint { value, compressed_size })
  }
}

impl UserString {
  pub fn from_utf16(buffer: &[u16]) -> UserString {
    match String::from_utf16(buffer) {
      Ok(string) => UserString::Valid(string),
      _ => UserString::Garbage
    }
  }
}

impl StreamReader for UserStringsStream {
  fn read_from<R: Read + Seek>(reader: &mut R, header: &StreamHeader) -> Result<UserStringsStream> {
    let mut strings: Vec<UserString> = vec![];
    let mut bytes_read: usize = 0;

    // Always starts with a null byte, which is handled like any other table entry

    while bytes_read < header.size as usize {
      let decoded = StreamUtils::decode_compressed_int(reader)?;
      bytes_read += decoded.compressed_size as usize;

      if decoded.value == 0 {
        strings.push(UserString::Valid("".to_string()));
        continue;
      }

      let mut string_buffer = vec![0u16; (decoded.value / 2) as usize];
      reader.read_exact_16(&mut string_buffer)?;
      strings.push(UserString::from_utf16(&string_buffer));

      let is_ascii = reader.read_u8()?;
      bytes_read += decoded.value as usize;
    }

    Ok( { UserStringsStream { strings } } )
  }
}
