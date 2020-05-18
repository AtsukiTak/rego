use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool as PgPool, PooledConnection},
};

#[derive(Clone)]
pub struct Postgres {
    pool: PgPool<ConnectionManager<PgConnection>>,
}

pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

impl Postgres {
    pub fn new(url: impl Into<String>) -> Postgres {
        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool = PgPool::new(manager).unwrap();
        Postgres { pool }
    }

    pub async fn with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> T + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        smol::blocking!(Ok(func(pool.get()?)))
    }

    pub async fn try_with_conn<T, F>(&self, func: F) -> anyhow::Result<T>
    where
        F: FnOnce(Connection) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        self.with_conn(func).await?
    }
}
