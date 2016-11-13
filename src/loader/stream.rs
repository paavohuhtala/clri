
use std::io::{Read, Seek, Result};
use utils::stream::*;

#[derive(Debug)]
pub struct StreamHeader {
  pub offset: u32,
  pub size: u32,
  pub name: String
}

/// The #~ stream.
pub struct MetaDataTablesStream {

}

/// The #String stream.
pub struct AsciiStringsStream {

}

/// The #US stream.
pub struct UnicodeStringsStream {

}

/// The #Blob stream.
pub struct BlobStream {

}

/// The #GUID stream.
pub struct GuidStream {

}
 
impl ReadableStruct for MetaDataTablesStream {
  fn read_from<R: Read + Seek>(reader: &mut R) -> Result<MetaDataTablesStream> {
    Ok(MetaDataTablesStream { })
  }
}
