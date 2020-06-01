use diesel::{
    connection::Connection as _,
    pg::PgConnection,
    r2d2::{Builder, ConnectionManager, Pool, PooledConnection},
};
use rego_domain::Error;

#[derive(Clone)]
pub struct Postgres {
    pool: Pool<Manager>,
}

pub type Manager = ConnectionManager<PgConnection>;
pub type Connection = PooledConnection<Manager>;

impl Postgres {
    pub fn new(url: impl Into<String>) -> Result<Postgres, diesel::r2d2::PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = Pool::new(manager)?;
        Ok(Postgres { pool })
    }

    pub fn with_builder(
        url: impl Into<String>,
        build_func: impl FnOnce(Builder<Manager>) -> Builder<Manager>,
    ) -> Result<Postgres, diesel::r2d2::PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = build_func(Pool::builder()).build(manager)?;
        Ok(Postgres { pool })
    }

    pub async fn with_conn<T, F>(&self, func: F) -> Result<T, Error>
    where
        F: FnOnce(Connection) -> T + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        let pg = pool.get().map_err(Error::internal)?;
        tokio::task::spawn_blocking(move || func(pg))
            .await
            .map_err(Error::internal)

        // TODO
        // smolを使ったバージョンをfeature gateと共に提供する
        // smol::blocking!(Ok(func(pool.get()?)))
    }

    pub async fn try_with_conn<T, F>(&self, func: F) -> Result<T, Error>
    where
        F: FnOnce(Connection) -> Result<T, Error> + Send + 'static,
        T: Send + 'static,
    {
        self.with_conn(func).await?
    }

    pub async fn transaction<T, F>(&self, func: F) -> Result<T, Error>
    where
        for<'a> F: FnOnce(&'a Connection) -> Result<T, Error> + Send + 'static,
        T: Send + 'static,
    {
        struct InnerError(Error);

        impl From<diesel::result::Error> for InnerError {
            fn from(e: diesel::result::Error) -> InnerError {
                InnerError(Error::internal(e))
            }
        }

        self.try_with_conn(|conn| {
            conn.transaction::<T, InnerError, _>(|| func(&conn).map_err(InnerError))
                .map_err(|InnerError(e)| e)
        })
        .await
    }
}
