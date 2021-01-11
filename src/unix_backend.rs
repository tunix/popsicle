use async_std::fs::File;
use async_trait::async_trait;
use std::io;
use std::marker::PhantomData;
use std::path::Path;

use crate::{Backend, Device};

struct UnixBackend;

#[async_trait]
impl Backend for UnixBackend {
    type Device = UnixDevice;

    async fn devices() -> Vec<Self::Device> {
        // XXX
        Vec::new()
    }

}

#[derive(Clone)]
struct UnixDevice;

#[async_trait]
impl Device for UnixDevice {
    type Backend = UnixBackend;

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
