use failure::{Fail, SyncFailure};
use tera::{Context, Error, Tera as TeraInner};
use warp::{self, Rejection, Reply};

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Tera {
    inner: Arc<TeraInner>,
}

impl Tera {
    #[inline]
    #[must_use]
    pub fn new() -> Result<Self, SyncFailure<Error>> {
        const TEMPLATES_PATH: &str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/static/templates/**/*");

        TeraInner::new(TEMPLATES_PATH)
            .map(Self::from)
            .map_err(SyncFailure::new)
    }

    #[inline]
    #[must_use]
    pub fn render(
        &self,
        name: &str,
        context: &Context,
    ) -> Result<impl Reply, Rejection> {
        self.inner.render(name, context).map_err(|error| {
            eprintln!("Error on render `{}` template: {}", name, error);

            warp::reject::custom(SyncFailure::new(error).compat())
        })
    }
}

impl From<TeraInner> for Tera {
    #[inline]
    fn from(inner: TeraInner) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}
