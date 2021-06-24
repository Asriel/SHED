//! SHED is a key-value store which appends it's self to the end of an executable
//! file so if that executable file is moved or shared with another person the data
//! is still retained.
//!
//! ### Basic Examples
//!  ```edition2018
//!// Create a new SHED
//!let mut shed_example = shed::Shed::new();
//!
//!// Test wheather this executable already has a SHED
//!if !shed_example.shed_exists() {
//!     shed_example
//!         .initialize_shed()
//!         .expect("Failed to initialize SHED");
//!}
//!
//!// Create a key value store with the value type String
//!let mut x = shed::Store::new();
//!x.insert(String::from("Test Key"), String::from("Test Value"));
//!
//!// Write our key store to the SHED
//!shed_example
//!     .write_shed(x)
//!     .expect("Failed to write data to SHED");
//!
//!// Read key value store from SHED
//!let read_shed: shed::Store<String> =
//!     shed_example.read_shed().expect("Failed to read from SHED");
//!println!("{:?}", read_shed);
//! ```
pub mod shed;
pub mod structure;

pub use crate::shed::Shed;
pub use crate::structure::Store;

/// The Magic Number used to identify the beginning of the SHED data section
const MAGIC_HEADER: &[u8; 6] = &[0x40, 0x53, 0x48, 0x45, 0x44, 0x40]; // @SHED@
