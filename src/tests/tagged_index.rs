
use std::collections::HashMap;
use std::io::Cursor;

use loader::stream::{TableId};
use metadata::tables::{Index, TypeDefOrRef};

#[test]
fn type_def_or_ref_16k_rows() {
  let mut bytes = vec![0b0010_0001, 0b0000_0011];
  let mut cursor = Cursor::new(&mut bytes);

  let mut row_counts = HashMap::new();
  row_counts.insert(TableId::TypeDef, 2u32.pow(14) - 1);
  row_counts.insert(TableId::TypeRef, 128);
  row_counts.insert(TableId::TypeSpec, 0);

  let type_def_or_ref = TypeDefOrRef::read_from(&mut cursor, &row_counts).unwrap();

  match type_def_or_ref {
    TypeDefOrRef::TypeRef(Index(index, _)) => assert_eq!(0xC8, index, "TypeRef index must equal 0xC8."),
    otherwise => assert!(false, "TypeDefOrRef must be TypeRef(0xC8) (was {:?})", otherwise)
  }

  assert_eq!(2, cursor.position());
}

#[test]
fn type_def_or_ref_over_16k_rows() {
  let mut bytes = vec![0b0000_0001, 0b0000_0000, 0b0000_0001, 0];
  let mut cursor = Cursor::new(&mut bytes);

  let mut row_counts = HashMap::new();
  row_counts.insert(TableId::TypeDef, 2u32.pow(14));
  row_counts.insert(TableId::TypeRef, 128);
  row_counts.insert(TableId::TypeSpec, 0);

  let type_def_or_ref = TypeDefOrRef::read_from(&mut cursor, &row_counts).unwrap();

  match type_def_or_ref {
    TypeDefOrRef::TypeRef(Index(index, _)) =>
      assert_eq!(0x4000, index, "TypeRef index must equal 0x4000."),
    otherwise =>
      assert!(false, "TypeDefOrRef must be TypeRef(0xC8)")
  }

  assert_eq!(4, cursor.position());
}