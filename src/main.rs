#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde_repr;

use serde_repr::*;
use std::fs::File;
use bincode::{serialize_into, deserialize};
use std::io::{BufWriter, Read, Write};

mod params;

// Require Deserialize_repr for enums
// https://serde.rs/enum-number.html
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum INodeFileType {
    // This is not a file type; it indicates that the inode is not initialized
    Unitialized,
    // Correspond to T_DIR in xv6
    Directory,
    // Correspond to T_FILE in xv6
    File,
    // Correspond to
    Device,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[repr(packed)]
pub struct INodeData {
    // File type
    pub file_type: INodeFileType,
    // Major device number (T_DEVICE only)
    pub major: i16,
    // Minor device number (T_DEVICE only)
    pub minor: i16,
    // Number of links to inode in file system
    pub nlink: i16,
    // Size of file (bytes)
    pub size: usize,
    // Data block addresses
    pub addresses: [u32; params::NDIRECT as usize + 1],
}

impl INodeData {
    fn new() -> INodeData {
        INodeData {
            file_type: INodeFileType::File,
            major: 3,
            minor: 4,
            nlink: 12,
            size: 14,
            addresses: [0xdeadbeef; params::NDIRECT as usize + 1],
        }
    }
}

fn deser_into_inodedata(buffer: &[u8]) -> INodeData {
    let m1: INodeData = deserialize(buffer).unwrap();
    println!("In disk {:?}", m1);
    m1
}

const INODE_SZ: usize = std::mem::size_of::<INodeData>();

fn main() {
    let m = INodeData::new();
    let mut buffer = BufWriter::new(File::create("foo.bin").unwrap());

    println!("In memory inode {:?}", m);
    // Serialize into a file
    serialize_into(&mut buffer, &m).unwrap();
    // flush it
    buffer.flush().unwrap();

    // Read the file from disk to see if we can deser into INodeData
    let mut buf: [u8; INODE_SZ] = [0u8; INODE_SZ];
    File::open("foo.bin").unwrap().read(&mut buf).unwrap();
    assert_eq!(m, deser_into_inodedata(&buf));
}
