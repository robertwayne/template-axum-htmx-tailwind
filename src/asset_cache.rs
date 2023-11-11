use std::collections::HashMap;

use async_compression::tokio::write::BrotliEncoder;
use axum::extract::Path;
use axum_cc::MimeType;
use bytes::Bytes;
use tokio::io::AsyncWriteExt;

/// A shared reference to the static asset cache.
pub type SharedAssetCache = &'static AssetCache;

const HASH_SPLIT_CHAR: char = '.';

/// Maps static asset filenames to their compressed bytes and content type. This
/// is used to serve static assets from the build directory without reading from
/// disk, as the cache stays in RAM for the life of the server.
///
/// This type should be accessed via the `cache` property in `AppState`.
pub struct AssetCache(HashMap<String, StaticAsset>);

impl AssetCache {
    /// Attempts to return a static asset from the cache from a cache key. If
    /// the asset is not found, `None` is returned.
    pub fn get(&self, key: &str) -> Option<&StaticAsset> {
        self.0.get(key)
    }

    /// Helper method to get a static asset from an extracted request path.
    pub fn get_from_path(&self, path: &Path<String>) -> Option<&StaticAsset> {
        let key = Self::get_cache_key(path);
        self.get(&key)
    }

    fn get_cache_key(path: &str) -> String {
        let mut parts = path.split(|c| c == '.' || c == HASH_SPLIT_CHAR);

        let basename = parts.next().unwrap_or_default();
        let ext = parts.last().unwrap_or_default();

        format!("{}.{}", basename, ext)
    }

    pub async fn load_files() -> Self {
        let mut cache = HashMap::default();

        let assets: Vec<_> = std::fs::read_dir("build")
            .unwrap_or_else(|e| panic!("failed to read build directory: {}", e))
            .filter_map(Result::ok)
            .filter_map(|file| {
                let path = file.path();
                let filename = path.file_name()?.to_str()?;
                let ext = path.extension()?.to_str()?;

                let stored_path = path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .ok()?
                    .replace("build/", "assets/");

                std::fs::read(&path)
                    .ok()
                    .map(|bytes| (stored_path, bytes, ext.to_string(), filename.to_string()))
            })
            .collect();

        for (stored_path, bytes, ext, filename) in assets {
            let contents = match ext.as_str() {
                "css" | "js" => compress_data(&bytes).await.unwrap_or_default(),
                _ => bytes.into(),
            };

            cache.insert(
                Self::get_cache_key(&filename),
                StaticAsset {
                    path: stored_path,
                    contents,
                    content_type: MimeType::from_extension(&ext),
                },
            );
        }

        tracing::debug!("loaded {} assets", cache.len());
        for (key, asset) in &cache {
            tracing::debug!("{} -> {}", key, asset.path);
        }

        Self(cache)
    }

    /// Returns an iterator over the static assets in the cache.
    pub fn values(&self) -> impl Iterator<Item = &StaticAsset> {
        self.0.values()
    }

    /// Returns an iterator over the static asset cache keys.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }
}

/// Represents a single static asset from the build directory. Assets are
/// represented as pre-compressed bytes via Brotli and their original content
/// type so the set_content_type middleware service can set the correct
/// Content-Type header.
pub struct StaticAsset {
    pub path: String,
    pub contents: Bytes,
    pub content_type: MimeType,
}

async fn compress_data(bytes: &[u8]) -> Result<Bytes, std::io::Error> {
    let mut encoder =
        BrotliEncoder::with_quality(Vec::new(), async_compression::Level::Precise(11));

    encoder.write_all(bytes).await?;
    encoder.shutdown().await?;

    Ok(Bytes::from(encoder.into_inner()))
}
