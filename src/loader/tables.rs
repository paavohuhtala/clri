
#![allow(non_upper_case_globals)]

use std::io::{Read, Seek, Result};
use std::marker::PhantomData;
use byteorder::{ReadBytesExt, LittleEndian};

use loader::stream::{TableId, IndexSize, IndexSizes, RowCounts, HeapOffsetSizes, FieldSizes};

pub trait ReadIndexSizeExt {
  fn read_index<T>(&mut self, size: IndexSize) -> Result<TableIndex<T>>;
}

impl<T: Read> ReadIndexSizeExt for T {
  fn read_index<I>(&mut self, size: IndexSize) -> Result<TableIndex<I>> {
    match size {
      IndexSize::Word => self.read_u16::<LittleEndian>().map(|x| x as u32),
      IndexSize::Dword => self.read_u32::<LittleEndian>()
    }.map(|x| TableIndex::new(x))
  }
}

#[derive(Debug, Copy, Clone)]
pub struct TableIndex<T>(pub u32, PhantomData<T>);

pub trait TableEntryReader {
  fn read_entry<R: Read>(reader: &mut R, sizes: &FieldSizes) -> Result<Self> where Self : Sized;
}

impl<T> TableIndex<T> {
  fn new(index: u32) -> TableIndex<T> {
    TableIndex(index, PhantomData::<T>)
  }

  fn to_u32(self) -> u32 {
    self.0
  }
}

#[derive(Debug)]
pub struct ModuleEntry {
  generation: u16,
  name: TableIndex<StringHeap>,
  mv_id: TableIndex<GuidHeap>,
  enc_id: TableIndex<GuidHeap>,
  enc_base_id: TableIndex<GuidHeap>
}

#[derive(Debug)]
pub struct ModuleRefEntry { }

#[derive(Debug)]
pub struct AssemblyRefEntry { }

#[derive(Debug)]
pub struct TypeRefEntry {
  resolution_scope: ResolutionScope,
  name: TableIndex<StringHeap>,
  namespace: TableIndex<StringHeap>
}

#[derive(Debug)]
pub struct TypeDefEntry {
  flags: TypeAttributes,
  name: TableIndex<StringHeap>,
  namespace: TableIndex<StringHeap>,
  extends: TypeDefOrRef
}

#[derive(Debug)]
pub struct TypeSpecEntry {
  signature: TableIndex<BlobHeap>
}

impl TableEntryReader for ModuleEntry {
  fn read_entry<R: Read>(reader: &mut R, sizes: &FieldSizes) -> Result<ModuleEntry> {
    let generation = reader.read_u16::<LittleEndian>()?;
    let name = reader.read_index(sizes.heap_sizes.string_index)?;
    let mv_id = reader.read_index(sizes.heap_sizes.guid_index)?;
    let enc_id = reader.read_index(sizes.heap_sizes.guid_index)?;
    let enc_base_id = reader.read_index(sizes.heap_sizes.guid_index)?;

    Ok(ModuleEntry {generation, name, mv_id, enc_id, enc_base_id} )
  }
}

impl TableEntryReader for TypeRefEntry {
  fn read_entry<R: Read>(reader: &mut R, sizes: &FieldSizes) -> Result<TypeRefEntry> {
    let resolution_scope = ResolutionScope::read_from(reader, &sizes.row_counts)?;
    let name = reader.read_index(sizes.heap_sizes.string_index)?;
    let namespace = reader.read_index(sizes.heap_sizes.string_index)?;

    Ok(TypeRefEntry { resolution_scope, name, namespace })
  }
}

pub struct TaggedIndex {
  pub tag: u8,
  pub index: u32
}

impl TaggedIndex {
  pub fn tag_mask_from_len(length: u8) -> u8 {
    match length {
      0 => 0b0000_0000,
      1 => 0b0000_0001,
      2 => 0b0000_0011,
      3 => 0b0000_0111,
      4 => 0b0000_1111,
      5 => 0b0001_1111,
      6 => 0b0011_1111,
      7 => 0b0111_1111,
      8 => 0b1111_1111,
      _ => panic!("Invalid tag length: {}", length)
    }
  }

  pub fn read_from<R: Read>(reader: &mut R, tag_bits_length: u8, row_count: u32) -> Result<TaggedIndex> {
    // The maximum number of rows that can be encoded with a 16-bit tagged index
    let max_length_for_word_index = 2u32.pow(16 - (tag_bits_length as u32)) - 1;
    let tag_mask = TaggedIndex::tag_mask_from_len(tag_bits_length);

    let tagged_index = if row_count > max_length_for_word_index {
      reader.read_u32::<LittleEndian>()
    } else {
      reader.read_u16::<LittleEndian>().map(|x| x as u32)
    }?;

    let tag = (tagged_index & (tag_mask as u32)) as u8;
    let index = (tagged_index & !(tag_mask as u32)) >> tag_bits_length;

    Ok (TaggedIndex { tag, index } )
  }
}

macro_rules! max_table_entries {
  ($row_counts: expr, [$x: ident]) => {
    $row_counts.get(&TableId::$x).map(|x|*x).unwrap_or(0u32)
  };
  ($row_counts: expr, [$x: ident, $($tail: ident),+]) => {
    {
      use std::cmp::max;
      max(max_table_entries!($row_counts, [$x]), max_table_entries!($row_counts, [$($tail),+]))
    }
  };
}

macro_rules! tagged_index_parser {
    {
      type: $type_: ident,
      tag_length: $tag_length: expr,
      patterns: [$($pattern: expr => $case: ident),*] 
    } => {
      impl $type_ {
        pub fn read_from<R: Read>(reader: &mut R, row_counts: &RowCounts) -> Result<$type_> {
          let max_size = max_table_entries!(row_counts, [$($case),*]);
          let tagged_index = TaggedIndex::read_from(reader, 2, max_size)?;
          match tagged_index.tag {
            $(
              $pattern => Ok($type_::$case(TableIndex::new(tagged_index.index)))
            ),+,
            otherwise => panic!("Invalid tag: {}", otherwise)
          }
      }
    }
  }
}

tagged_index_parser! {
  type: TypeDefOrRef,
  tag_length: 2,
  patterns: [
    0b00 => TypeDef,
    0b01 => TypeRef,
    0b10 => TypeSpec
  ]
}

tagged_index_parser! {
  type: ResolutionScope,
  tag_length: 2,
  patterns: [
      0b00 => Module,
      0b01 => ModuleRef,
      0b10 => AssemblyRef,
      0b11 => TypeRef
  ]
}

#[derive(Debug)]
pub struct StringHeap;
#[derive(Debug)]
pub struct GuidHeap;
#[derive(Debug)]
pub struct BlobHeap;

#[derive(Debug)]
pub enum ResolutionScope {
  Module(TableIndex<ModuleEntry>),
  ModuleRef(TableIndex<TypeDefEntry>),
  AssemblyRef(TableIndex<TypeDefEntry>),
  TypeRef(TableIndex<TypeDefEntry>)
}

#[derive(Debug)]
pub enum TypeDefOrRef {
  TypeDef(TableIndex<TypeDefEntry>),
  TypeRef(TableIndex<TypeRefEntry>),
  TypeSpec(TableIndex<TypeSpecEntry>)
}

bitflags! {
  // https://github.com/dotnet/coreclr/blob/master/src/inc/corhdr.h#L276
  pub flags TypeAttributes: u32 {
    // Use this mask to retrieve the type visibility information.
    const tdVisibilityMask        =   0x00000007,
    const tdNotPublic             =   0x00000000,     // Class is not public scope.
    const tdPublic                =   0x00000001,     // Class is public scope.
    const tdNestedPublic          =   0x00000002,     // Class is nested with public visibility.
    const tdNestedPrivate         =   0x00000003,     // Class is nested with private visibility.
    const tdNestedFamily          =   0x00000004,     // Class is nested with family visibility.
    const tdNestedAssembly        =   0x00000005,     // Class is nested with assembly visibility.
    const tdNestedFamANDAssem     =   0x00000006,     // Class is nested with family and assembly visibility.
    const tdNestedFamORAssem      =   0x00000007,     // Class is nested with family or assembly visibility.

    // Use this mask to retrieve class layout information
    const tdLayoutMask            =   0x00000018,
    const tdAutoLayout            =   0x00000000,     // Class fields are auto-laid out
    const tdSequentialLayout      =   0x00000008,     // Class fields are laid out sequentially
    const tdExplicitLayout        =   0x00000010,     // Layout is supplied explicitly
    // end layout mask

    // Use this mask to retrieve class semantics information.
    const tdClassSemanticsMask    =   0x00000060,
    const tdClass                 =   0x00000000,     // Type is a class.
    const tdInterface             =   0x00000020,     // Type is an interface.
    // end semantics mask

    // Special semantics in addition to class semantics.
    const tdAbstract              =   0x00000080,     // Class is abstract
    const tdSealed                =   0x00000100,     // Class is concrete and may not be extended
    const tdSpecialName           =   0x00000400,     // Class name is special. Name describes how.

    // Implementation attributes.
    const tdImport                =   0x00001000,     // Class / interface is imported
    const tdSerializable          =   0x00002000,     // The class is Serializable.

    // Use tdStringFormatMask to retrieve string information for native interop
    const tdStringFormatMask      =   0x00030000,
    const tdAnsiClass             =   0x00000000,     // LPTSTR is interpreted as ANSI in this class
    const tdUnicodeClass          =   0x00010000,     // LPTSTR is interpreted as UNICODE
    const tdAutoClass             =   0x00020000,     // LPTSTR is interpreted automatically
    const tdCustomFormatClass     =   0x00030000,     // A non-standard encoding specified by CustomFormatMask
    const tdCustomFormatMask      =   0x00C00000,     // Use this mask to retrieve non-standard encoding information for native interop. The meaning of the values of these 2 bits is unspecified.

    // end string format mask

    const tdBeforeFieldInit       =   0x00100000,     // Initialize the class any time before first static field access.
    const tdForwarder             =   0x00200000,     // This ExportedType is a type forwarder.

    // Flags reserved for runtime use.
    const tdReservedMask          =   0x00040800,
    const tdRTSpecialName         =   0x00000800,     // Runtime should check name encoding.
    const tdHasSecurity           =   0x00040000,     // Class has security associate with it.
  }
}
