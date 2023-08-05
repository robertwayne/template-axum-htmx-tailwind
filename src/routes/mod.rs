use lib::asset_cache::SharedAssetCache;
use serde::Serialize;

pub mod index;
pub mod not_found;

pub type SharedBaseTemplateData = &'static BaseTemplateData;

#[derive(Clone, Serialize)]
pub struct BaseTemplateData {
    styles: String,
    htmx: String,
}

impl BaseTemplateData {
    pub fn new(assets: SharedAssetCache) -> Self {
        let styles = assets
            .get("index.css")
            .expect("failed to build base template data: index.css")
            .path
            .clone();

        let htmx = assets
            .get("index.js")
            .expect("failed to build base template data: index.js")
            .path
            .clone();

        Self { styles, htmx }
    }
}
