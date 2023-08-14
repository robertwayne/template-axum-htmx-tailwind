use axum::{extract::FromRef, response::Html};
use axum_extra::extract::cookie::Key;
use minijinja::{context, value::Value};
use sqlx::PgPool;

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
    pub fn render(&self, boosted: bool, template: &str) -> Result<Html<String>, ApiError> {
        let template = self
            .env
            .get_template(template)
            .map_err(|_| ApiError::TemplateNotFound(template.into()))?;

        if boosted {
            let Ok(r) = template.render(context! {}) else {
                return Err(ApiError::TemplateRender(template.name().into()));
            };

            return Ok(Html(r));
        }

        let r = template
            .render(context! { base => Some(self.base_template_data) })
            .map_err(|_| ApiError::TemplateRender(template.name().into()))?;

        Ok(Html(r))
    }

    pub fn render_with_context(
        &self,
        boosted: bool,
        template: &str,
        ctx: Value,
    ) -> Result<Html<String>, ApiError> {
        let template = self
            .env
            .get_template(template)
            .map_err(|_| ApiError::TemplateNotFound(template.into()))?;

        if boosted {
            let r = template
                .render(ctx)
                .map_err(|_| ApiError::TemplateRender(template.name().into()))?;

            return Ok(Html(r));
        }

        let ctx = context! {
            base => Some(self.base_template_data),
            ..ctx
        };

        let r = template
            .render(ctx)
            .map_err(|_| ApiError::TemplateRender(template.name().into()))?;

        Ok(Html(r))
    }
}
