use walkdir::WalkDir;

use crate::zbus_udisks2::*;
use crate::Progress;
use async_std::fs::{self, File};
use futures::{stream::FuturesUnordered, StreamExt};
use std::{
    collections::HashMap,
    ffi::OsStr,
    os::unix::{ffi::OsStrExt, io::AsRawFd},
    path::{Path, PathBuf},
};
use zbus::{azync::Connection, Error};

// TODO: set progress some way

async fn format_and_copy<P: Progress>(
    connection: &Connection,
    iso_mount_path: &Path,
    drive: &str,
    progress: &mut P,
) -> anyhow::Result<()> {
    // Create partition table
    let block = AsyncBlockProxy::builder(&connection).path(drive.clone())?.build();
    block.format("gpt", HashMap::new()).await?;
    let partition_table = AsyncPartitionTableProxy::builder(&connection).path(drive)?.build();

    let mut offset = 1024u64.pow(2);

    // UEFI system partition
    // XXX timeout?
    let partition = partition_table
        .create_partition_and_format(
            offset,
            1024u64.pow(3),
            "0xef",
            "",
            HashMap::new(),
            "vfat",
            HashMap::new(),
        )
        .await?;
    let fat_filesystem = AsyncFilesystemProxy::builder(&connection).path(partition)?.build();
    let fat_path = fat_filesystem.mount(HashMap::new()).await?;
    offset += 1024u64.pow(3);

    // Ntfs partition
    let partition = partition_table
        .create_partition_and_format(offset, 0, "0xb", "", HashMap::new(), "ntfs", HashMap::new())
        .await?;
    let ntfs_filesystem = AsyncFilesystemProxy::builder(&connection).path(partition)?.build();
    let ntfs_path = ntfs_filesystem.mount(HashMap::new()).await?;

    // Copy the files
    for entry in WalkDir::new(&iso_mount_path) {
        let entry = entry?;
        let relpath = entry.path().strip_prefix(&iso_mount_path)?;

        let mut destpath = if relpath.starts_with("sources") {
            PathBuf::from(&ntfs_path)
        } else {
            PathBuf::from(&fat_path)
        };
        destpath.push(relpath);

        if entry.file_type().is_dir() {
            if !destpath.exists() {
                fs::create_dir(destpath).await?
            }
        } else {
            fs::copy(entry.path(), &destpath).await?;
        }
    }

    fat_filesystem.unmount(HashMap::new()).await?;
    ntfs_filesystem.unmount(HashMap::new()).await?;

    Ok(())
}

pub struct WinTask<P: Progress> {
    pub image: File,
    pub devices: Vec<(String, P::Device, P)>,
}

impl<P: Progress> WinTask<P> {
    pub fn new(image: File) -> Self {
        Self { image, devices: Vec::new() }
    }

    pub async fn process(mut self) -> anyhow::Result<()> {
        let connection = Connection::new_system().await?;

        // Setup loopback for .iso
        let manager = AsyncManagerProxy::new(&connection);
        let loop_path = manager.loop_setup(self.image.as_raw_fd().into(), HashMap::new()).await?;
        let loop_ = AsyncLoopProxy::builder(&connection).path(&loop_path)?.build();

        // Mount the loopback
        let loop_filesystem = AsyncFilesystemProxy::builder(&connection).path(&loop_path)?.build();
        let iso_mount_path = match loop_filesystem.mount(HashMap::new()).await {
            Ok(mount_path) => PathBuf::from(mount_path),
            Err(Error::MethodError(name, _, _))
                if name == "org.freedesktop.UDisks2.Error.AlreadyMounted" =>
            {
                PathBuf::from(OsStr::from_bytes(&loop_filesystem.mount_points().await?[0]))
            }
            Err(err) => {
                return Err(err.into());
            }
        };
        loop_.set_autoclear(true, HashMap::new()).await?;

        // TODO: support multiple devices here
        let futures = FuturesUnordered::new();
        for (path, device, mut progress) in self.devices.drain(..) {
            let connection = &connection;
            let iso_mount_path = &iso_mount_path;
            futures.push(async move {
                if let Err(err) =
                    format_and_copy(connection, iso_mount_path, &path, &mut progress).await
                {
                    progress.message(&device, "", &format!("{}", err));
                }
                progress.finish();
            });
        }
        futures.collect::<()>().await;

        loop_filesystem.unmount(HashMap::new()).await?;

        Ok(())
    }

    pub fn subscribe(&mut self, file: String, device: P::Device, progress: P) -> &mut Self {
        self.devices.push((file, device, progress));
        self
    }
}
