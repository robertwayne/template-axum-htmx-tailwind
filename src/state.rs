use axum::{extract::FromRef, response::Html};
use axum_extra::extract::cookie::Key;
use axum_htmx::HxBoosted;
use deadpool_postgres::Pool as PgPool;
use minijinja::{context, value::Value};

use crate::{api_error::ApiError, asset_cache::SharedAssetCache, routes::SharedBaseTemplateData};

pub type SharedState = &'static AppState;

#[derive(Clone)]
pub struct AppState {
    pub pg: PgPool,
    pub assets: SharedAssetCache,
    pub base_template_data: SharedBaseTemplateData,
    pub env: minijinja::Environment<'static>,
    pub encryption_key: Key,
}

impl FromRef<&'static AppState> for Key {
    fn from_ref(state: &&'static AppState) -> Self {
        state.encryption_key.clone()
    }
}

impl AppState {
    pub fn render(
        &self,
        HxBoosted(boosted): HxBoosted,
        template: &str,
    ) -> Result<Html<String>, ApiError> {
        let template = self
            .env
            .get_template(template)
            .map_err(|_| ApiError::TemplateNotFound(template.into()))?;

        if boosted {
            match template.render(context! {}) {
                Ok(rendered) => return Ok(Html(rendered)),
                Err(_) => return Err(ApiError::TemplateRender(template.name().into())),
            }
        }

        match template.render(context! {
            base => Some(self.base_template_data )
        }) {
            Ok(rendered) => Ok(Html(rendered)),
            Err(_) => Err(ApiError::TemplateRender(template.name().into())),
        }
    }

    pub fn render_with_context(
        &self,
        HxBoosted(boosted): HxBoosted,
        template: &str,
        ctx: Value,
    ) -> Result<Html<String>, ApiError> {
        let template = self
            .env
            .get_template(template)
            .map_err(|_| ApiError::TemplateNotFound(template.into()))?;

        if boosted {
            let rendered = template
                .render(ctx)
                .map_err(|_| ApiError::TemplateRender(template.name().into()))?;

            return Ok(Html(rendered));
        }

        match template.render(context! {
            base => Some(self.base_template_data), ..ctx
        }) {
            Ok(rendered) => Ok(Html(rendered)),
            Err(_) => Err(ApiError::TemplateRender(template.name().into())),
        }
    }
}
