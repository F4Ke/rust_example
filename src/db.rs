use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::pg::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &String) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

// #[cfg(test)]
// pub fn establish_test_connection() -> DbPool {
//   let database_url = std::env::var("APP_DATABASE_TEST_URL")
//       .expect("APP_DATABASE_TEST_URL must be set for tests");
//     init_pool(&database_url)
// }
