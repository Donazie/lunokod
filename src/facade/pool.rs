use diesel::{r2d2, sqlite};
use failure::{Error, Fail, Fallible};
use warp::{self, Rejection};

type ConnectionManager = r2d2::ConnectionManager<sqlite::SqliteConnection>;
type PoolInner = r2d2::Pool<ConnectionManager>;
type PooledConnection = r2d2::PooledConnection<ConnectionManager>;

#[derive(Clone)]
pub struct Pool {
    inner: PoolInner,
}

impl Pool {
    #[inline]
    #[must_use]
    pub fn new() -> Fallible<Self> {
        const DATABASE_PATH: &str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/",
            env!("CARGO_PKG_NAME"),
            ".db",
        );

        PoolInner::new(ConnectionManager::new(DATABASE_PATH))
            .map(Self::from)
            .map_err(Error::from)
    }

    #[inline]
    #[must_use]
    pub fn get(&self) -> Result<PooledConnection, Rejection> {
        self.inner.get().map_err(|error| {
            eprintln!("Failed to get connection: {}", error);

            warp::reject::custom(error.compat())
        })
    }
}

impl From<PoolInner> for Pool {
    #[inline]
    fn from(inner: PoolInner) -> Self {
        Self { inner }
    }
}
