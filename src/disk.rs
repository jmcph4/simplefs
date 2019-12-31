#![allow(dead_code)]
pub enum DiskError {}

pub const BLOCK_SIZE: usize = 4096;

pub trait Disk {
    fn open(path: String, num_blocks: usize) -> Result<(), DiskError>;
    
    fn size() -> usize;

    fn mounted() -> bool;
    fn mount() -> Result<(), DiskError>;
    fn unmount() -> Result<(), DiskError>;

    fn read(block_number: usize) -> Result<Vec<u8>, DiskError>;
    fn write(block_number: usize, data: Vec<u8>) -> Result<(), DiskError>;
}

