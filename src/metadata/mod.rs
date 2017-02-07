
use std::fmt::Debug;
use typemap::{TypeMap, DebugMap, Key};

pub mod heap;
pub mod tables;
pub mod debug;

use loader::stream::TableId;
use metadata::heap::Heaps;

#[derive(Debug)]
pub struct Metadata {
  pub tables: DebugMap,
  pub heaps: Heaps
}

impl Metadata {
  pub fn get_string(&self, index: &Index<StringHeap>) -> Option<&String> {
    self.heaps.strings.strings.get(&index.0)
  }
}

struct KeyType;

pub trait MetadataTable {
  const TABLE_ID: TableId;
}

macro_rules! metadata_tables {
  [$($type_name: ident = $table_id: ident),*] => {
    $(
      impl Key for $type_name {
        type Value = Vec<$type_name>;
      }

      impl MetadataTable for $type_name {
        const TABLE_ID: TableId = TableId::$table_id;
      }
    )*
  }
}

use metadata::tables::*;

metadata_tables! [
  ModuleEntry = Module,
  TypeRefEntry = TypeRef,
  TypeDefEntry = TypeDef,
  MethodDefEntry = MethodDef,
  MemberRefEntry = MemberRef,
  CustomAttributeEntry = CustomAttribute,
  AssemblyEntry = Assembly,
  AssemblyRefEntry = AssemblyRef
];

impl Metadata {
  pub fn add_table<T: Key>(&mut self, table: T::Value) where T::Value : Debug {
    self.tables.insert::<T>(table);
  }

  pub fn get_table<T: Key + Debug>(&self) -> Option<&T::Value> where T::Value : Debug {
    self.tables.get::<T>()
  }
}
