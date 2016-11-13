
use std::io::{Read, Seek, Result};
use std::collections::HashMap;
use enum_primitive::FromPrimitive;
use byteorder::{ReadBytesExt, LittleEndian};
use utils::stream::*;

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

}

/// The #US stream.
#[derive(Debug, Clone)]
pub struct UnicodeStringsStream {

}

/// The #Blob stream.
#[derive(Debug, Clone)]
pub struct BlobStream {

}

/// The #GUID stream.
#[derive(Debug, Clone)]
pub struct GuidStream {

}

#[derive(Debug)]
enum FieldSize {
  Word,
  Dword
}

impl From<bool> for FieldSize {
  fn from(x: bool) -> FieldSize {
    if x {FieldSize::Dword} else {FieldSize::Word} 
  }
}

#[derive(Debug)]
struct HeapOffsetSizes {
  string_index: FieldSize,
  guid_index: FieldSize,
  blob_index: FieldSize
}

enum_from_primitive! {
  #[derive(Hash, PartialEq, Eq, Debug)]
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

#[derive(Debug)]
pub struct RowCounts(HashMap<TableId, u32>);

impl From<u8> for HeapOffsetSizes {
  fn from(x: u8) -> HeapOffsetSizes {
    let str_index_bit = (x & 0b1) > 0;
    let guid_index_bit = (x & 0b10) > 0;
    let blob_index_bit = (x & 0b100) > 0;

    HeapOffsetSizes {
      string_index: FieldSize::from(str_index_bit),
      guid_index: FieldSize::from(guid_index_bit),
      blob_index: FieldSize::from(blob_index_bit)
    }
  }
}

impl ReadableStruct for MetaDataTablesStream {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<MetaDataTablesStream> {
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

    Ok(MetaDataTablesStream { })
  }
}
