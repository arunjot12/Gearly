use deadpool_diesel::mysql::{Manager, Pool};
use diesel::Connection;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    MysqlConnection::establish(&database_url).unwrap()
}

pub type DbPool = Pool;


fn create_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);

    Pool::builder(manager)
        .max_size(10) // tune this based on your DB's max_connections and load
        .build()
        .expect("Failed to create DB pool")
}