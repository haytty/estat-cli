use std::path::{PathBuf};

pub fn create_file_path(output_dir: &str, filename: &str) -> PathBuf {
    let path_buf = PathBuf::from(output_dir);
    path_buf.join(filename)
}
