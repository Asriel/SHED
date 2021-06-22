use crate::structure;
use crate::MAGIC_HEADER;
use bincode;
use std::env;
use std::fs;
use std::io::*;
use std::path;

pub struct Shed {
    file_path: path::PathBuf,
}

impl Shed {
    pub fn new() -> Shed {
        Shed {
            file_path: env::current_exe().unwrap(),
        }
    }

    pub fn new_from_path(filepath: &str) -> Shed {
        Shed {
            file_path: path::PathBuf::from(filepath),
        }
    }

    // Check wheather the file already has a SHED database
    pub fn has_shed(&mut self) -> (bool, usize) {
        let file_size = self.get_file_size();
        let buf = self.read_file();

        let mut shed_count = 0;

        for x in 0..file_size {
            if &buf[x] == &MAGIC_HEADER[0] {
                for y in 1..MAGIC_HEADER.len() {
                    if x + y >= file_size {
                        break;
                    } else if &buf[x + y] != &MAGIC_HEADER[y] {
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

    // Create a new SHED key-value store
    pub fn initialize_shed(&mut self) {
        let tmp_exe = self.create_tmp();

        let mut file = fs::OpenOptions::new().write(true).open(&tmp_exe).unwrap();

        // NOTE: Remove unwrap later
        file.seek(SeekFrom::Start(self.get_file_size() as u64))
            .unwrap();
        file.write(MAGIC_HEADER).unwrap();

        self.tmp_to_original(tmp_exe)
    }

    pub fn read_shed<'de, T: serde::de::DeserializeOwned>(&mut self) -> structure::Store<T> {
        let output = self.read_from_shed();
        let de_ser: structure::Store<T> = bincode::deserialize(&output).unwrap();
        de_ser
    }

    pub fn write_shed<T: serde::Serialize>(&mut self, data: structure::Store<T>) {
        let ser_data = bincode::serialize(&data).unwrap();
        self.write_to_shed(&ser_data);
    }

    // Read from SHED section
    fn read_from_shed(&mut self) -> Vec<u8> {
        let shed_banner = self.has_shed().1 + 6;
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(&self.file_path)
            .unwrap();

        let mut output_buf = Vec::new();

        // NOTE: Remove unwrap later
        file.seek(SeekFrom::Start(shed_banner as u64)).unwrap();
        file.read_to_end(&mut output_buf).unwrap();

        output_buf
    }

    fn write_to_shed(&mut self, data: &Vec<u8>) {
        let shed_banner = self.has_shed().1 + 6;

        let tmp_exe = self.create_tmp();

        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(&tmp_exe)
            .unwrap();

        file.set_len(shed_banner as u64).unwrap();
        file.seek(SeekFrom::Start(shed_banner as u64)).unwrap();
        file.write(&data).unwrap();

        self.tmp_to_original(tmp_exe);
    }


    fn get_file_size(&mut self) -> usize {
        let metadata = fs::metadata(&self.file_path).unwrap();
        metadata.len() as usize
    }

    fn read_file(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut f = std::fs::File::open(&self.file_path).unwrap();
        f.read_to_end(&mut buf).unwrap();
        return buf;
    }

    fn create_tmp(&mut self) -> path::PathBuf {
        let mut tmp_exe = env::current_exe().unwrap();
        tmp_exe.pop();
        tmp_exe.push(".tmp");
        fs::copy(&self.file_path, &tmp_exe).unwrap();

        tmp_exe
    }

    fn tmp_to_original(&mut self, tmp_path: path::PathBuf) {
        fs::remove_file(&self.file_path).unwrap();
        fs::rename(&tmp_path, &self.file_path.file_name().unwrap()).unwrap();
    }

    // NOTE: Remove this function later
    pub fn test_func(&mut self, seek_pos: u64) {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(&self.file_path)
            .unwrap();

        let mut test_vec = Vec::new();
        file.seek(SeekFrom::Start(seek_pos)).unwrap();
        file.read_to_end(&mut test_vec).unwrap();
        println!("{:?}", test_vec);
    }

    
}
