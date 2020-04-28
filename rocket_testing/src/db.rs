use diesel::prelude::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use std::env;

pub type ConnectionPool = Pool<ConnectionManager<SqliteConnection>>;

fn init_pool(database_url: &str) -> Result<ConnectionPool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let msg = format!("Error connecting to {}", database_url);
    init_pool(&database_url).expect(&msg)
}
