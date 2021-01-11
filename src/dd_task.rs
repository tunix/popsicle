use async_trait::async_trait;
use std::io;
use std::marker::PhantomData;
use async_std::path::Path;

use crate::{Backend, Device, Task};

// Equivalent to just dding a file
struct DDTask<B: Backend> {
    _phantom_backend: PhantomData<B>
} 

#[async_trait]
impl<B: Backend + 'static> Task<B> for DDTask<B> {
    fn new(image_path: &Path) -> Self {
        Self {
            _phantom_backend: PhantomData,
        }
    }

    async fn subscribe(device: B::Device) -> io::Result<()> {
        let file = device.open().await?;
        Ok(())
    }

    async fn process() {
    }
}
