#![allow(dead_code)]
pub enum DiskError {}

pub const BLOCK_SIZE: usize = 4096;

pub trait Disk where Self: Sized{
    fn open(path: String, num_blocks: usize) -> Result<Self, DiskError>;
    
    fn size(&self) -> usize;

    fn mounted(&self) -> bool;
    fn mount(&mut self) -> Result<(), DiskError>;
    fn unmount(&mut self) -> Result<(), DiskError>;

    fn read(&mut self, block_number: usize) -> Result<Vec<u8>, DiskError>;
    fn write(&mut self, block_number: usize, data: Vec<u8>) ->
        Result<(), DiskError>;
}

