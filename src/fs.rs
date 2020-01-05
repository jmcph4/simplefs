#![allow(dead_code)]
#![allow(unused_variables)]
extern crate math;
extern crate serde;
extern crate bincode;
use serde::{Serialize, Deserialize};
use math::round;
use crate::disk::{Disk, BlockDisk, BLOCK_SIZE};

const MAGIC_NUMBER: u32 = 0xf0f03410;
const INODES_PER_BLOCK: u32 = 128;
const POINTERS_PER_INODE: u32 = 5;
const POINTERS_PER_BLOCK: u32 = 1024;
const INODE_BLOCKS_FRACTION: f64 = 0.10;
const INODE_SIZE: usize = 32;

#[derive(Debug)]
pub enum FileSystemError {
    DiskWriteFailure,
    DiskReadFailure,
    MiscellaneousFailure,
    InvalidSuperblock,
    NoFreeInodes
}

pub trait FileSystem<BlockDisk: Disk> where Self: Sized {
    fn format(disk: BlockDisk) -> Result<(), FileSystemError>;

    fn mount(disk: BlockDisk) -> Result<Self, FileSystemError>;

    fn create(&mut self) -> Result<usize, FileSystemError>;
    fn remove(&mut self, inumber: usize) -> Result<bool, FileSystemError>;
    fn stat(&self, inumber: usize) -> Result<usize, FileSystemError>;

    fn read(&mut self, inumber: usize, data: &mut Vec<u8>, offset: usize) ->
        Result<usize, FileSystemError>;
    fn write(&mut self, inumber: usize, data: Vec<u8>, offset: usize) ->
        Result<usize, FileSystemError>;
}

#[derive(Serialize, Deserialize)]
struct Superblock {
    magic: u32,
    num_blocks: u32,
    num_inode_blocks: u32,
    num_inodes: u32
}

#[derive(Serialize, Deserialize)]
struct Inode {
    valid: u32,
    size: u32,
    direct: [u32;POINTERS_PER_INODE as usize],
    indirect: u32
}

pub struct SimpleFileSystem<BlockDisk: Disk> {
    disk: BlockDisk,
    inode_block: Vec<u8>,
    bitmap: Vec<bool>
}

impl FileSystem<BlockDisk> for SimpleFileSystem<BlockDisk> {
    fn format(mut disk: BlockDisk) -> Result<(), FileSystemError> {
        SimpleFileSystem::clear_disk(&mut disk);
        
        /* superblock metadata */
        let num_blocks: u32 = disk.size() as u32;
        let num_inode_blocks: u32 = round::ceil(num_blocks as f64 *
                                     INODE_BLOCKS_FRACTION, 0) as u32;
        let num_inodes: u32 = (INODES_PER_BLOCK * num_inode_blocks) as u32;

        let superblock: Superblock = Superblock {
            magic: MAGIC_NUMBER,
            num_blocks: num_blocks,
            num_inode_blocks: num_inode_blocks,
            num_inodes: INODES_PER_BLOCK * num_inode_blocks
        };

        let superblock_bytes: Vec<u8> = match bincode::serialize(&superblock) {
            Ok(bytes) => bytes,
            Err(_e) => return Err(FileSystemError::MiscellaneousFailure)
        };

        match disk.write(0, superblock_bytes) {
            Ok(()) => {},
            Err(_e) => return Err(FileSystemError::DiskWriteFailure)
        };

        Ok(())
    }

    fn mount(disk: BlockDisk) -> Result<Self, FileSystemError> {
        unimplemented!();
    }

    fn create(&mut self) -> Result<usize, FileSystemError> {
        unimplemented!();
    }

    fn remove(&mut self, inumber: usize) -> Result<bool, FileSystemError> {
        unimplemented!();
    }

    fn stat(&self, inumber: usize) -> Result<usize, FileSystemError> {
        unimplemented!();
    }

    fn read(&mut self, inumber: usize, data: &mut Vec<u8>, offset: usize) ->
        Result<usize, FileSystemError> {
        unimplemented!();
    }

    fn write(&mut self, inumber: usize, data: Vec<u8>, offset: usize) -> 
        Result<usize, FileSystemError> {
        unimplemented!();
    }
}

impl SimpleFileSystem<BlockDisk> {
    fn clear_disk(disk: &mut BlockDisk) {
        for i in 0..disk.size() {
            disk.write(i, vec![0;BLOCK_SIZE]).unwrap();
        }
    }
}


