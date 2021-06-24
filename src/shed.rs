use crate::structure;
use crate::MAGIC_HEADER;
use std::env;
use std::fs;
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path;

pub struct Shed {
    file_path: path::PathBuf,
}

impl Shed {
    pub fn new() -> Self {
        Shed {
            file_path: env::current_exe().expect("Failed to get current executable file path"),
        }
    }

    pub fn new_from_path(filepath: &str) -> Shed {
        Shed {
            file_path: path::PathBuf::from(filepath),
        }
    }

    /// Internal version of shed_exists used to get the offset of the SHED
    fn has_shed(&mut self) -> (bool, usize) {
        let file_size = self.get_file_size();
        let buf = self.read_file();

        let mut shed_count = 0;

        for x in 0..file_size {
            if buf[x] == MAGIC_HEADER[0] {
                for y in 1..MAGIC_HEADER.len() {
                    if buf[x + y] != MAGIC_HEADER[y] {
                        break;
                    } else if y == MAGIC_HEADER.len() - 1 {
                        shed_count += 1;
                    }
                }
            }
            if shed_count > 1 {
                return (true, x);
            }
        }

        (false, 0)
    }

    /// Checks wheather the file already has a SHED database
    pub fn shed_exists(&mut self) -> bool {
        let file_size = self.get_file_size();
        let buf = self.read_file();

        let mut shed_count = 0;

        for x in 0..file_size {
            if buf[x] == MAGIC_HEADER[0] {
                for y in 1..MAGIC_HEADER.len() {
                    if buf[x + y] != MAGIC_HEADER[y] {
                        break;
                    } else if y == MAGIC_HEADER.len() - 1 {
                        shed_count += 1;
                    }
                }
            }
            if shed_count > 1 {
                return true;
            }
        }

        false
    }

    /// Creates a new SHED key-value store
    pub fn initialize_shed(&mut self) -> io::Result<()> {
        let tmp_exe = self.create_tmp();

        let mut file = fs::OpenOptions::new().write(true).open(&tmp_exe)?;

        file.seek(SeekFrom::Start(self.get_file_size() as u64))?;
        if file.write(MAGIC_HEADER)? != MAGIC_HEADER.len() {
            panic!("Failed to write magic header");
        }

        if let Err(err) = self.tmp_to_original(tmp_exe) {
            panic!("IO Error: {}", err);
        }
        Ok(())
    }

    /// Handles the deserialization back to a Store object
    pub fn read_shed<T: serde::de::DeserializeOwned>(&mut self) -> io::Result<structure::Store<T>> {
        let output = match self.read_from_shed() {
            Ok(output) => output,
            Err(err) => panic!("Error: {}", err),
        };
        let de_ser: structure::Store<T> =
            rmp_serde::from_read_ref(&output).expect("Deserialization error");
        Ok(de_ser)
    }

    /// Handles the serialization of Store objects
    pub fn write_shed<T: serde::Serialize>(&mut self, data: structure::Store<T>) -> io::Result<()> {
        let ser_data = rmp_serde::to_vec(&data).expect("Serialization error");
        if let Err(err) = self.write_to_shed(&ser_data) {
            panic!("Error: {}", err);
        }
        Ok(())
    }

    /// Read the raw bytes from the SHED
    fn read_from_shed(&mut self) -> io::Result<Vec<u8>> {
        let shed_banner = self.has_shed().1 + 6;
        let mut file = fs::OpenOptions::new().read(true).open(&self.file_path)?;

        let mut output_buf = Vec::new();

        file.seek(SeekFrom::Start(shed_banner as u64))?;
        file.read_to_end(&mut output_buf)?;

        Ok(output_buf)
    }

    /// Handles the higher level operations of writing to the SHED such as creating and moving files
    fn write_to_shed(&mut self, data: &[u8]) -> io::Result<()> {
        let shed_banner = self.has_shed().1 + 6;

        let tmp_exe = self.create_tmp();

        let mut file = fs::OpenOptions::new().write(true).open(&tmp_exe)?;

        file.set_len(shed_banner as u64)?;
        file.seek(SeekFrom::Start(shed_banner as u64))?;
        file.write_all(data)?;

        if let Err(err) = self.tmp_to_original(tmp_exe) {
            panic!("IO Error: {}", err);
        }
        Ok(())
    }

    /// A helper function to get the size of a file
    fn get_file_size(&mut self) -> usize {
        let metadata = fs::metadata(&self.file_path).expect("Error opening file metadata");
        metadata.len() as usize
    }

    /// Read a file into a vector of u8's
    fn read_file(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut f = std::fs::File::open(&self.file_path).expect("Failed to open file");
        f.read_to_end(&mut buf)
            .expect("Failed to read till end of file");
        buf
    }

    /// Create a temporary executable we can edit
    fn create_tmp(&mut self) -> path::PathBuf {
        let mut tmp_exe = env::current_exe().expect("Failed to get current executable file path");
        tmp_exe.pop();
        tmp_exe.push(".tmp");
        fs::copy(&self.file_path, &tmp_exe).expect("Failed to copy file");

        tmp_exe
    }

    /// Replace the original executable file with the new edited one
    fn tmp_to_original(&mut self, tmp_path: path::PathBuf) -> io::Result<()> {
        fs::remove_file(&self.file_path).unwrap();
        fs::rename(&tmp_path, &self.file_path.file_name().unwrap()).unwrap();
        Ok(())
    }
}

impl Default for Shed {
    fn default() -> Shed {
        Shed {
            file_path: env::current_exe().expect("Failed to get current executable file path"),
        }
    }
}
