#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate typemap;

#[macro_use]
mod utils;
mod loader;
mod metadata;
mod runtime;

use loader::pe;
use std::marker::PhantomData;

#[allow(non_snake_case)]
pub mod Corlib {
    pub mod Console {
        pub fn WriteLine(str: String) {
            println!("{}", str);
        }
    }
}

struct ManagedObject {}

enum StackValue {
    Int64(i64),
    Int32(i32),
    Float(f64),
    Object(Box<ManagedObject>),
}

struct MethodInfo {}

struct MethodState<'a> {
    ip: u32,
    stack: Vec<u8>,
    local_variables: Vec<u8>,
    local_heap: Vec<u8>,
    arguments: Vec<u8>,
    method_info: Box<MethodInfo>,
    return_state: (),
    phantom: PhantomData<&'a u8>,
}

impl<'a> MethodState<'a> {
    pub fn new() -> MethodState<'a> {
        MethodState {
            ip: 0,
            stack: vec![],
            local_variables: vec![],
            local_heap: vec![],
            arguments: vec![],
            method_info: Box::new(MethodInfo {}),
            return_state: (),
            phantom: PhantomData,
        }
    }
}

fn main() {
    println!("CLRi 0.1");
    let file = std::fs::File::open("sample/helloworld/HelloWorld.exe").unwrap();
    let mut file_reader = std::io::BufReader::new(file);
    let pe_file = pe::PEFile::read_from(&mut file_reader).unwrap();
    let image = loader::clr::CLRImage::from_pe(&pe_file).unwrap();

    use metadata::debug::*;
    use metadata::tables::*;

    println!(
        "{}",
        (*image.metadata.tables.get::<ModuleEntry>().unwrap())[0].as_debug(&image.metadata)
    );
}
