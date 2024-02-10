use diesel::{
    r2d2::{ConnectionManager, Pool},
    Connection, MysqlConnection,
};
use dotenvy::dotenv;
use std::{env, time::Duration};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn connect_to_db() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .connection_timeout(Duration::from_secs(3))
        .build(manager)
        .expect("Could not build db connection pool")
}
