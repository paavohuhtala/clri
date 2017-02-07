
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum UserString {
  Valid(String),
  Garbage
}

#[derive(Debug)]
pub struct StringHeap {
  pub strings: HashMap<u32, String>
}

#[derive(Debug)]
pub struct UserStringHeap {
  pub strings: HashMap<u32, UserString>
}

#[derive(Debug)]
pub struct BlobHeap {
  pub blobs: HashMap<u32, Vec<u8>>
}

pub trait Heap<T> {
  fn get_at_index(&self, index: u32) -> Option<&T>;
}

impl Heap<String> for StringHeap {
  fn get_at_index(&self, index: u32) -> Option<&String> {
    self.strings.get(&index)
  }
}

#[derive(Debug)]
pub struct Heaps {
  pub strings: StringHeap,
  pub user_strings: UserStringHeap,
  pub blobs: BlobHeap
}
