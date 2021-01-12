use async_trait::async_trait;
use std::io;
use std::marker::PhantomData;
use async_std::path::Path;

use crate::{Backend, Device, Progress, Task};

// Equivalent to just dding a file
pub struct DDTask<D: Device, P: Progress> {
    _phantom_device: PhantomData<D>,
    _phantom_progress: PhantomData<P>
} 

#[async_trait]
impl<D: Device + 'static, P: Progress> Task<D, P> for DDTask<D, P> {
    fn new<T: AsRef<Path>>(image_path: &T, check: bool) -> Self {
        Self {
            _phantom_device: PhantomData,
            _phantom_progress: PhantomData,
        }
    }

    async fn subscribe(&mut self, device: D, progress_device: P::Device, progress: P) -> io::Result<()> {
        let file = device.open().await?;
        Ok(())
    }

    async fn process(self, buf: &mut [u8]) -> anyhow::Result<()> {
        Ok(())
    }
}
