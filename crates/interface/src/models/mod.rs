use std::{
    fs::File,
    io::{BufReader, Read, Seek as _},
    path::{Path, PathBuf},
};

use base_util::project::root_path;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use sha2::{Digest, Sha256};
use tar::Archive;

pub struct ModelDb {}

impl ModelDb {
    pub fn get(
        &self,
        kind: &str,
        name: &str,
        file: &str,
        url: &str,
        hash: &str,
    ) -> Option<PathBuf> {
        let file_path = root_path().join("models").join(kind).join(name).join(file);
        std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
        if !file_path.exists() {
            download_and_extract(url, &file_path).unwrap();
            if failure(&file_path, hash) {
                let _ = std::fs::remove_file(&file_path);
                download_and_extract(url, &file_path).unwrap();
            }
            if failure(&file_path, hash) {
                panic!()
            }
        } else {
            if failure(&file_path, hash) {
                let _ = std::fs::remove_file(&file_path);
                download_and_extract(url, &file_path).unwrap();
            }
            if failure(&file_path, hash) {
                panic!()
            }
        }
        Some(file_path)
    }
}

fn failure<P: AsRef<Path>>(file_path: P, expected_hash: &str) -> bool {
    let mut file = match std::fs::File::open(&file_path) {
        Ok(f) => f,
        Err(_) => return true,
    };

    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    if file.read_to_end(&mut buffer).is_err() {
        return true;
    }

    hasher.update(&buffer);
    let result = hasher.finalize();
    let file_hash = format!("{:x}", result);
    debug!("File hash: {}", file_hash);
    file_hash != expected_hash.to_lowercase()
}

fn download_and_extract(url: &str, file_path: &Path) -> anyhow::Result<()> {
    info!("Downloading from: {}", url);

    let mut response = ureq::get(url).call()?;
    let total_size = response
        .headers()
        .get("Content-Length")
        .and_then(|val| val.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0);

    let pb = if total_size > 0 {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message("Downloading");
        Some(pb)
    } else {
        None
    };

    struct ProgressReader<R> {
        inner: R,
        progress_bar: Option<ProgressBar>,
        bytes_read: u64,
    }

    impl<R: Read> Read for ProgressReader<R> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let n = self.inner.read(buf)?;
            self.bytes_read += n as u64;
            if let Some(pb) = &self.progress_bar {
                pb.set_position(self.bytes_read);
            }
            Ok(n)
        }
    }

    let mut temp_file = tempfile::tempfile()?;

    let b = response.body_mut();
    let b = b.as_reader();
    let mut progress_reader = ProgressReader {
        inner: b,
        progress_bar: pb.clone(),
        bytes_read: 0,
    };

    std::io::copy(&mut progress_reader, &mut temp_file)?;

    if let Some(pb) = pb {
        pb.finish_with_message("Download complete");
    }

    if url.ends_with(".tar.gz") {
        debug!("Extracting archive...");

        temp_file.rewind()?;

        let buf_reader = BufReader::new(temp_file);
        let decoder = GzDecoder::new(buf_reader);
        let mut archive = Archive::new(decoder);

        let extract_dir = file_path.parent().ok_or(anyhow::anyhow!(
            "Failed to determine parent directory of the provided path"
        ))?;

        archive.unpack(extract_dir)?;
        debug!("Extraction complete.");
    } else {
        debug!("Downloaded file is not a .tar.gz archive, saving as normal file.");

        temp_file.rewind()?;
        let mut output = File::create(file_path)?;
        std::io::copy(&mut temp_file, &mut output)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    //TODO: test successfull download

    #[test]
    fn hashing() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        assert_eq!(
            failure(root_path().join("models/detector/default/model.onnx"), ""),
            true
        );
    }

    #[test]
    fn test_failure_returns_true_for_nonexistent_file() {
        let path = PathBuf::from("nonexistent.file");
        assert!(failure(path, "abc"));
    }

    #[test]
    fn test_failure_returns_false_for_correct_hash() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");
        fs::write(&path, "correct content").unwrap();

        let correct_hash = format!("{:x}", Sha256::digest(b"correct content"));
        assert!(!failure(&path, &correct_hash));
    }

    #[test]
    #[should_panic]
    fn test_get_panics_on_double_hash_failure() {
        let db = ModelDb {};
        let _ = db.get(
            "invalid",
            "invalid",
            "bad.txt",
            "https://example.com/404.tar.gz",
            "invalidhash",
        );
    }

    #[test]
    fn hashing2() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
        assert_eq!(
            failure(root_path().join("models/detector/default/model.onnx"), ""),
            true
        );
    }

    // Optional: test download_and_extract with a fake URL
    // (would need a test server or mock ureq)
}
