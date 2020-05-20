use diesel::{
    pg::PgConnection,
    r2d2::{Builder, ConnectionManager, Pool, PooledConnection},
};

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

    pub async fn with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> T + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || Ok(func(pool.get()?))).await?

        // TODO
        // smolを使ったバージョンをfeature gateと共に提供する
        // smol::blocking!(Ok(func(pool.get()?)))
    }

    pub async fn try_with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.with_conn(func).await?
    }
}
