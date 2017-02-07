
use std::fmt::Write;

use metadata::tables::*;
use metadata::Metadata;

impl ModuleEntry {
  pub fn as_debug(&self, meta: &Metadata) -> String {
    let mut res = String::new();

    let ref self_name = self.name;
    let name = meta.get_string(self_name).unwrap();
    write!(&mut res, "Module: \"{}\"", name).unwrap();

    res
  }
}

impl MemberRefEntry {
  pub fn as_debug(&self, meta: &Metadata) -> String {
    let mut res = String::new();

    let ref self_name = self.name;
    let ref self_class = self.class;
    let name = meta.get_string(self_name).unwrap();
    write!(&mut res, "MethodRef {{name: {}, class: {:?}}}", name, self_class).unwrap();

    res
  }
}