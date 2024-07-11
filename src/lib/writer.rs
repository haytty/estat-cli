use std::path::Path;
use tokio::fs::File;
use tokio::io;
use tokio::io::AsyncWrite;
use anyhow::Result;

pub trait Writer: AsyncWrite + Unpin {}

impl<T: AsyncWrite + Unpin> Writer for T {}

pub async fn initialize_writer<P: AsRef<Path>>(path_opt: Option<P>) -> Result<Box<dyn Writer>> {
    let writer: Box<dyn Writer> = match path_opt {
        Some(o) => {
            Box::new(File::create(o).await?)
        }
        None => Box::new(io::stdout()),
    };

    Ok(writer)
}