use async_compression::tokio::write::BrotliEncoder;
use axum::extract::Path;
use bytes::Bytes;
use lib::mime::MimeType;
use rustc_hash::FxHashMap;
use tokio::io::AsyncWriteExt;

/// A shared reference to the static asset cache.
pub type SharedAssetCache = &'static AssetCache;

/// Maps static asset filenames to their compressed bytes and content type. This
/// is used to serve static assets from the build directory without reading from
/// disk, as the cache stays in RAM for the life of the server.
///
/// This type should be accessed via the `cache` property in `AppState`.
pub struct AssetCache {
    inner: FxHashMap<String, StaticAsset>,
}

impl AssetCache {
    pub fn new(inner: FxHashMap<String, StaticAsset>) -> Self {
        Self { inner }
    }

    /// Attempts to return a static asset from the cache from a cache key. If
    /// the asset is not found, `None` is returned.
    pub fn get(&self, key: &str) -> Option<&StaticAsset> {
        self.inner.get(key)
    }

    /// Helper method to get a static asset from an extracted request path.
    pub fn get_from_path(&self, path: &Path<String>) -> Option<&StaticAsset> {
        let key = Self::get_cache_key(path);
        self.get(&key)
    }

    fn get_cache_key(path: &str) -> String {
        let basename = path.split('.').next().unwrap_or_default();
        let ext = path.split('.').last().unwrap_or_default();

        format!("{}.{}", basename, ext)
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
    pub hash: [u8; 32],
}

async fn compress_data(bytes: &[u8]) -> Bytes {
    let mut encoder =
        BrotliEncoder::with_quality(Vec::new(), async_compression::Level::Precise(11));

    if let Err(e) = encoder.write_all(bytes).await {
        tracing::error!("Failed to compress data: {e}");
    };

    if let Err(e) = encoder.shutdown().await {
        tracing::error!("Failed to shutdown compression stream: {e}");
    }

    Bytes::from(encoder.into_inner())
}

pub async fn generate_static_asset_cache() -> AssetCache {
    let mut cache = FxHashMap::default();

    if let Ok(files) = std::fs::read_dir("build") {
        for file in files {
            let Ok(file) = file else {
                continue;
            };

            let path = file.path();

            let Some(filename) = path.file_name() else {
                continue;
            };

            let Some(filename) = filename.to_str() else {
                continue;
            };

            let stored_path = path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap_or_default()
                .replace("build/", "assets/");

            let Ok(bytes) = std::fs::read(&path) else {
                continue;
            };

            let Some(ext) = path.extension() else {
                continue;
            };

            let Some(ext) = ext.to_str() else {
                continue;
            };

            let hash = {
                let mut hasher = blake3::Hasher::new();
                hasher.update(&bytes);
                hasher.finalize()
            };

            let contents = match ext {
                "css" | "js" => compress_data(&bytes).await,
                _ => bytes.into(),
            };

            let key = AssetCache::get_cache_key(filename);

            cache.insert(
                key,
                StaticAsset {
                    path: stored_path,
                    contents,
                    content_type: MimeType::from_extension(ext),
                    hash: *hash.as_bytes(),
                },
            );
        }
    }

    AssetCache::new(cache)
}
