use crate::emulator::Emulator;
use std::fs;
use std::fs::File;
use std::io::Read;

pub fn load(e: &mut Emulator, fname: &String) {
    let bytes: Vec<u8> = get_file_as_byte_vec(fname);
    e.store_bytes(&bytes[..]);
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
