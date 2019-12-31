#![allow(dead_code)]
use crate::disk::Disk;

pub enum FileSystemError {}

pub trait FileSystem<T: Disk> where Self: Sized {
    fn format(disk: T) -> Result<(), FileSystemError>;

    fn mount(disk: T) -> Result<Self, FileSystemError>;

    fn create(&mut self) -> Result<usize, FileSystemError>;
    fn remove(&mut self, inumber: usize) -> Result<bool, FileSystemError>;
    fn stat(&self, inumber: usize) -> Result<usize, FileSystemError>;

    fn read(&mut self, inumber: usize, data: &mut Vec<u8>, offset: usize) ->
        Result<usize, FileSystemError>;
    fn write(&mut self, inumber: usize, data: Vec<u8>, offset: usize) ->
        Result<usize, FileSystemError>;
}

