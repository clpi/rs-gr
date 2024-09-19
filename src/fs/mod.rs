use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tokio::{
    fs::{File, OpenOptions},
    io::{copy, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, Error as IoError},
    time::Instant,
};

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FileStore {
    id: uuid::Uuid,
    path: PathBuf,
    #[serde(default)]
    updated: Instant,
    revision: uuid::Uuid,
}

impl Default for FileStore {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            revision: uuid::Uuid::new_v4(),
            updated: Instant::now(),
            path: dirs::data_dir().unwrap_or_default().as_path().to_path_buf(),
        }
    }
}

impl FileStore {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub async fn get(&self, key: &str) -> Result<File, IoError> {
        let path = self.path.join(key);
        File::open(path).await
    }

    pub async fn put(&self, key: &str, mut file: File) -> Result<(), IoError> {
        let path = self.path.join(key);
        let mut out = File::create(path).await?;
        copy(&mut file, &mut out).await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), IoError> {
        let path = self.path.join(key);
        tokio::fs::remove_file(path).await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<String>, IoError> {
        let mut files = Vec::new();
        while let Some(e) = tokio::fs::read_dir(self.path.clone())
            .await?
            .next_entry()
            .await?
        {
            let p = e.path();
            if p.is_file() {
                files.push(p.file_name().unwrap().to_string_lossy().to_string());
            }
        }
        Ok(files)
    }

    pub fn exists(&self, key: &str) -> Result<bool, IoError> {
        let path = self.path.join(key);
        Ok(path.exists())
    }

    pub fn size(&self, key: &str) -> Result<u64, IoError> {
        let path = self.path.join(key);
        let metadata = path.metadata()?;
        Ok(metadata.len())
    }

    pub fn url(&self, key: &str) -> Result<String, IoError> {
        let path = self.path.join(key);
        let url = format!("file://{}", path.to_string_lossy());
        Ok(url)
    }

    pub fn path(&self, key: &str) -> Result<PathBuf, IoError> {
        let path = self.path.join(key);
        Ok(path)
    }

    pub async fn update(&self, key: &str, mut file: File) -> Result<(), IoError> {
        let path = self.path.join(key);
        let mut out = File::create(path).await?;
        copy(&mut file, &mut out).await?;
        Ok(())
    }
}

