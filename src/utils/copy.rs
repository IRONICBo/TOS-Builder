use core::fmt;
use std::{path::Path, error::Error, fs};

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn Error>> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    if !src.is_dir() {
        return Err("src is not a directory".into());
    }
    if !dst.is_dir() {
        return Err("dst is not a directory".into());
    }

    let name = src.file_name().unwrap().to_string_lossy();

    for entry in src.read_dir()? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if ty.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_copy_dir_recursive() {
        let src = PathBuf::from("/Users/asklv/Projects/TSOC/test/test1");
        let dst = PathBuf::from("/Users/asklv/Projects/TSOC/test/test2");
        let _ = copy_dir_recursive(&src, &dst);
    }
}