use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};

pub enum DiskError {
    ImageOpenFailure,
    ImageReadFailure,
    ImageWriteFailure
}

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

pub struct BlockDisk {
    file_handle: File,
    num_blocks: usize,
    num_reads: u128,
    num_writes: u128,
    mounted: bool,
    num_mounts: u128
}

impl Disk for BlockDisk {
    fn open(path: String, num_blocks: usize) -> Result<Self, DiskError> {
        let file: File =
            match OpenOptions::new().write(true).create(true).open(path) {
            Ok(f) => f,
            Err(_e) => return Err(DiskError::ImageOpenFailure),
        };
        
        Ok(BlockDisk {
            file_handle: file,
            num_blocks: num_blocks,
            num_reads: 0,
            num_writes: 0,
            mounted: false,
            num_mounts: 0
        })
    }

    fn size(&self) -> usize {
        self.num_blocks
    }

    fn mounted(&self) -> bool {
        self.mounted
    }

    fn mount(&mut self) -> Result<(), DiskError> {
        self.num_mounts += 1;
        self.mounted = true;
        Ok(())
    }

    fn unmount(&mut self) -> Result<(), DiskError> {
        self.mounted = false;
        Ok(())
    }

    fn read(&mut self, block_number: usize) -> Result<Vec<u8>, DiskError> {
        match self.file_handle.seek(SeekFrom::Start(
                (block_number * BLOCK_SIZE) as u64)) {
            Ok(_n) => {},
            Err(_e) => return Err(DiskError::ImageReadFailure)
        };

        let mut data: Vec<u8> = Vec::new();
        
        match self.file_handle.read_to_end(&mut data) {
            Ok(_n) => {},
            Err(_e) => return Err(DiskError::ImageReadFailure)
        }
        
        self.num_reads += 1;
        
        data.truncate(BLOCK_SIZE);
        Ok(data)
    }

    fn write(&mut self, block_number: usize, data: Vec<u8>) ->
        Result<(), DiskError> {
        match self.file_handle.seek(SeekFrom::Start(
                (block_number * BLOCK_SIZE) as u64)) {
            Ok(_n) => {},
            Err(_e) => return Err(DiskError::ImageReadFailure)
        };

        let mut write_data: Vec<u8> = data.clone();
        write_data.truncate(BLOCK_SIZE);

        match self.file_handle.write(&write_data[..]) {
            Ok(_n) => {},
            Err(_e) => return Err(DiskError::ImageWriteFailure)
        };

        self.num_writes += 1;
        Ok(())
    }
}

