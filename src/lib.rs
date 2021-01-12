#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate thiserror;

pub extern crate mnt;

pub mod codec;

mod dd_task;
mod task;
mod unix_backend;
mod win_task;

pub use self::dd_task::DDTask;
pub use self::task::{Progress, Task as Task_}; // XXX
pub use self::unix_backend::UnixDevice;

use anyhow::Context;
use async_std::{
    fs::File,
    path::Path,
};
use async_trait::async_trait;
use futures::prelude::*;
use std::{
    io,
};
use usb_disk_probe::stream::UsbDiskProbe;

#[derive(Debug, Error)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum ImageError {
    #[error("image could not be opened: {}", why)]
    Open { why: io::Error },
    #[error("unable to get image metadata: {}", why)]
    Metadata { why: io::Error },
    #[error("image was not a file")]
    NotAFile,
    #[error("unable to read image: {}", why)]
    ReadError { why: io::Error },
    #[error("reached EOF prematurely")]
    Eof,
}

#[derive(Debug, Error)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum DiskError {
    #[error("failed to fetch devices from USB device stream: {}", _0)]
    DeviceStream(anyhow::Error),
    #[error("unable to open directory at '{}': {}", dir, why)]
    Directory { dir: &'static str, why: io::Error },
    #[error("writing to the device was killed")]
    Killed,
    #[error("unable to read directory entry at '{}': invalid UTF-8", dir.display())]
    UTF8 { dir: Box<Path> },
    #[error("unable to find disk '{}': {}", disk.display(), why)]
    NoDisk { disk: Box<Path>, why: io::Error },
    #[error("failed to unmount {}: {}", path.display(), why)]
    UnmountCommand { path: Box<Path>, why: io::Error },
    #[error("error using disk '{}': {} already mounted at {}", arg.display(), source_.display(), dest.display())]
    AlreadyMounted { arg: Box<Path>, source_: Box<Path>, dest: Box<Path> },
    #[error("'{}' is not a block device", arg.display())]
    NotABlock { arg: Box<Path> },
    #[error("unable to get metadata of disk '{}': {}", arg.display(), why)]
    Metadata { arg: Box<Path>, why: io::Error },
    #[error("unable to open disk '{}': {}", disk.display(), why)]
    Open { disk: Box<Path>, why: io::Error },
    #[error("error writing disk '{}': {}", disk.display(), why)]
    Write { disk: Box<Path>, why: io::Error },
    #[error("error writing disk '{}': reached EOF", disk.display())]
    WriteEOF { disk: Box<Path> },
    #[error("unable to flush disk '{}': {}", disk.display(), why)]
    Flush { disk: Box<Path>, why: io::Error },
    #[error("error seeking disk '{}': seeked to {} instead of 0", disk.display(), invalid)]
    SeekInvalid { disk: Box<Path>, invalid: u64 },
    #[error("error seeking disk '{}': {}", disk.display(), why)]
    Seek { disk: Box<Path>, why: io::Error },
    #[error("error verifying disk '{}': {}", disk.display(), why)]
    Verify { disk: Box<Path>, why: io::Error },
    #[error("error verifying disk '{}': reached EOF", disk.display())]
    VerifyEOF { disk: Box<Path> },
    #[error("error verifying disk '{}': mismatch at {}:{}", disk.display(), x, y)]
    VerifyMismatch { disk: Box<Path>, x: usize, y: usize },
}

pub async fn usb_disk_devices(disks: &mut Vec<Box<Path>>) -> anyhow::Result<()> {
    let mut stream = UsbDiskProbe::new().await.context("failed to create USB disk probe")?;

    while let Some(device_result) = stream.next().await {
        match device_result {
            Ok(disk) => disks.push(disk),
            Err(why) => {
                eprintln!("failed to reach device path: {}", why);
            }
        }
    }

    Ok(())
}

#[async_trait]
pub trait Device: Clone + Send + Sync { // XXX Sync

    async fn devices() -> Vec<Self>;
    // XXX best API to refresh by polling or notification?
    // Can inotify do that?

    async fn from_path<P: AsRef<Path> + Send + Sync>(path: P) -> anyhow::Result<Self>;

    /// Unmounts any mounted partitions
    async fn unmount(&self, force: bool) -> anyhow::Result<()>;

    async fn wait_removed(&self);
    // Is this a good way to deal with events asynchronously in Rust? Make it awaitable?
    // No way to remove handler
    
    async fn open(&self) -> io::Result<File>;

    fn vendor(&self) -> &str;
    fn model(&self) -> &str;
    // /dev/sda or such
    fn display_id(&self) -> &str;
    // These could be combined into just label(), or name() and display_id()
    fn capacity(&self) -> usize;
}

#[async_trait]
pub trait Task<D: Device, P: Progress>: Send {
    // Could it take a Read + Seek? Not useful with extractor... define a new type?
    // Trait with a method to return a reader
    // Not useful if task needs to mount iso
    fn new<T: AsRef<Path>>(image_path: &T, check: bool) -> Self;
    async fn subscribe(&mut self, device: D, progress_device: P::Device, progress: P) -> io::Result<()>;
    async fn process(self, buf: &mut [u8]) -> anyhow::Result<()>;
}

// could it be abstracted to apply this as a filter instead?
struct DDExtractTask {}

struct DevBackend {}
struct UDisksBackend {}
struct WinBackend {}
// https://docs.microsoft.com/en-us/windows/win32/fileio/volume-management-functions


