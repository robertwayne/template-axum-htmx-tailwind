use serde::Serialize;

use crate::asset_cache::SharedAssetCache;

pub mod index;
pub mod not_found;
pub mod robots;

pub type SharedBaseTemplateData = &'static BaseTemplateData;

#[derive(Clone, Serialize)]
pub struct BaseTemplateData {
    styles: String,
    scripts: String,
}

impl BaseTemplateData {
    pub fn new(assets: SharedAssetCache) -> Self {
        let styles = assets
            .get("index.css")
            .expect("failed to build base template data: index.css")
            .path
            .clone();

        let scripts = assets
            .get("index.js")
            .expect("failed to build base template data: index.js")
            .path
            .clone();

        Self { styles, scripts }
    }
}
