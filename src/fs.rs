#![allow(dead_code)]
use crate::disk::Disk;

pub enum FileSystemError {}

pub trait FileSystem<T: Disk> {
    fn format(disk: T) -> Result<bool, FileSystemError>;

    fn mount(disk: T) -> Result<bool, FileSystemError>;

    fn create() -> Result<usize, FileSystemError>;
    fn remove(inumber: usize) -> Result<bool, FileSystemError>;
    fn stat(inumber: usize) -> Result<usize, FileSystemError>;

    fn read(inumber: usize, data: &mut Vec<u8>, offset: usize) ->
        Result<usize, FileSystemError>;
    fn write(inumber: usize, data: Vec<u8>, offset: usize) ->
        Result<usize, FileSystemError>;
}

