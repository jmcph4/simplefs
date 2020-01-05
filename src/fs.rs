#![allow(dead_code)]
#![allow(unused_variables)]
extern crate math;
extern crate serde;
extern crate bincode;
use serde::{Serialize, Deserialize};
use math::round;
use crate::disk::{Disk, BlockDisk, BLOCK_SIZE};
use std::convert::TryInto;

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

    fn mount(mut disk: BlockDisk) -> Result<Self, FileSystemError> {
        /* read first block from disk as this contains the superblock */
        let first_block: Vec<u8> = match disk.read(0) {
            Ok(bytes) => bytes,
            Err(_e) => return Err(FileSystemError::DiskReadFailure)
        };

        /* extract superblock from the first disk block */
        let superblock: Superblock =
            match bincode::deserialize(&first_block[..15]) {
            Ok(block) => block,
            Err(_e) => return Err(FileSystemError::MiscellaneousFailure)
        };

        /* validate superblock by checking for magic number */
        if superblock.magic != MAGIC_NUMBER {
            return Err(FileSystemError::InvalidSuperblock);
        }

        let inode_block: Vec<u8> = match disk.read(1) {
            Ok(bytes) => bytes,
            Err(_e) => return Err(FileSystemError::DiskReadFailure)
        };

        /* allocate space for free block bitmap */
        let bitmap: Vec<bool> = vec![true; disk.size()];

        /* construct filesystem type */
        let mut filesystem: SimpleFileSystem<BlockDisk> = SimpleFileSystem {
            disk: disk,
            inode_block: inode_block,
            bitmap: bitmap
        };

        /* scan filesystem looking for free blocks */
        for i in 0..(BLOCK_SIZE/INODE_SIZE) {
            let inode: Inode = filesystem.get_inode(i)?;

            for j in 0..POINTERS_PER_INODE {
                filesystem.bitmap[inode.direct[j as usize] as usize] = false;
            }
        }

        Ok(filesystem)
    }

    fn create(&mut self) -> Result<usize, FileSystemError> {
        let new_inode: Inode = Inode {
            valid: true as u32,
            size: 0,
            direct: [0u32; POINTERS_PER_INODE as usize],
            indirect: 0
        };

        let mut curr_inumber: usize = 0;

        let mut inode_block: Vec<u8> = self.inode_block.clone();

        for i in (0..BLOCK_SIZE).step_by(INODE_SIZE) {
            let inode_bytes: Vec<u8> = inode_block[i..i+INODE_SIZE].to_vec();
            let valid_field_bytes: &[u8; 4] =
                &inode_bytes[i..i+4].try_into().unwrap();

            let valid: bool =
                SimpleFileSystem::slice_to_u32(valid_field_bytes) != 0;

            /* when first free inode encountered, mark it as valid and
             * return inumber */
            if !valid {
                inode_block[i+3] = 1;

                /* write updated inode block to disk */
                match self.disk.write(1, inode_block) {
                    Ok(()) => {},
                    Err(_e) => return Err(FileSystemError::DiskWriteFailure)
                };

                /* update cached inode block by reading back from disk */
                self.inode_block = match self.disk.read(1) {
                    Ok(bytes) => bytes,
                    Err(_e) => return Err(FileSystemError::DiskReadFailure)
                };

                return Ok(curr_inumber);
            }

            curr_inumber += 1;
        }

        Err(FileSystemError::NoFreeInodes)
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

    fn get_inode(&self, inumber: usize) -> Result<Inode, FileSystemError> {
        let inode_block: &Vec<u8> = &self.inode_block;

        for i in (0..BLOCK_SIZE).step_by(INODE_SIZE) {
            if i == inumber {
                let inode: Inode = Inode {
                    valid: SimpleFileSystem::slice_to_u32(
                               &inode_block[i..i+4].try_into().unwrap()),
                    size: SimpleFileSystem::slice_to_u32(
                        &inode_block[i+4..i+8].try_into().unwrap()),
                    direct: [
                        SimpleFileSystem::slice_to_u32(
                            &inode_block[i+8..i+12].try_into().unwrap()),
                        SimpleFileSystem::slice_to_u32(
                            &inode_block[i+12..i+16].try_into().unwrap()),
                        SimpleFileSystem::slice_to_u32(
                            &inode_block[i+16..i+20].try_into().unwrap()),
                        SimpleFileSystem::slice_to_u32(
                            &inode_block[i+20..i+24].try_into().unwrap()),
                        SimpleFileSystem::slice_to_u32(
                            &inode_block[i+24..i+28].try_into().unwrap())
                    ],
                    indirect: SimpleFileSystem::slice_to_u32(
                        &inode_block[i+28..i+i+32].try_into().unwrap())
                };

                return Ok(inode);
            }
        }

        Err(FileSystemError::MiscellaneousFailure)
    }
    
    fn slice_to_u32(bytes: &[u8;4]) -> u32 {
        ((bytes[0] as u32) << 24) |
            ((bytes[1] as u32) << 16) |
            ((bytes[2] as u32) << 8) |
            bytes[3] as u32
    }
}


