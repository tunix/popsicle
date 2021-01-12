use async_std::fs::File;
use async_trait::async_trait;
use std::io;
use async_std::path::Path;

use crate::Device;

#[derive(Clone)]
pub struct UnixDevice;

#[async_trait]
impl Device for UnixDevice {
    async fn devices() -> Vec<Self> {
        // XXX
        Vec::new()
    }

    async fn from_path<P: AsRef<Path> + Send + Sync>(path: P) -> anyhow::Result<Self> {
        Err(anyhow::Error::msg("foo"))
    }

    async fn unmount(&self, force: bool) {
        // TODO
    }

    async fn wait_removed(&self) {
        // TODO
    }

    async fn open(&self) -> io::Result<File> {
        // TODO
        Err(io::Error::new(io::ErrorKind::Other, ""))
    }

    fn vendor(&self) -> &str {
        // TODO
        ""
    }

    fn model(&self) -> &str {
        // TODO
        ""
    }

    fn display_id(&self) -> &str {
        // TODO
        ""
    }

    fn capacity(&self) -> usize {
        // TODO
        0
    }
}
