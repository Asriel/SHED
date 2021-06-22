pub mod shed;
pub mod structure;

pub use crate::shed::Shed;
pub use crate::structure::Store;

const MAGIC_HEADER: &[u8; 6] = &[0x40, 0x53, 0x48, 0x45, 0x44, 0x40]; // @SHED@
