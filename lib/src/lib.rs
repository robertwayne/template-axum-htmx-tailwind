pub mod asset_cache;
pub mod cache_control;
pub mod config;
pub mod mime;

pub fn leak_alloc<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}
