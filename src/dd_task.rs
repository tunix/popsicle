use async_std::{
    fs::File,
    path::Path,
};
use async_trait::async_trait;
use srmw::{CopyEvent, MultiWriter, ValidationEvent};
use std::{
    collections::HashMap,
    io::{self, SeekFrom},
    marker::PhantomData,
    time::Instant,
};

use crate::{Device, Progress, Task};

// Equivalent to just dding a file
pub struct DDTask<D: Device, P: Progress> {
    image: File,
    pub writer: MultiWriter<File>,
    pub state: HashMap<usize, (P::Device, P)>,
    millis_between: u64,
    check: bool,
    _phantom_device: PhantomData<D>,
    _phantom_progress: PhantomData<P>
} 

#[async_trait]
impl<D: Device + 'static, P: Progress> Task<D, P> for DDTask<D, P> {
    async fn new<T: AsRef<Path> + Send + Sync>(image_path: T, check: bool) -> Self {
        Self {
            image: File::open(image_path).await.unwrap(), // XXX
            state: HashMap::new(),
            writer: MultiWriter::default(),
            millis_between: 125,
            check,
            _phantom_device: PhantomData,
            _phantom_progress: PhantomData,
        }
    }

    async fn subscribe(&mut self, device: D, progress_device: P::Device, progress: P) -> io::Result<()> {
        let file = device.open().await?;
        let entity = self.writer.insert(file);
        self.state.insert(entity, (progress_device, progress));
        Ok(())
    }

    async fn process(self, buf: &mut [u8]) -> anyhow::Result<()> {
        Ok(())
    }
}
