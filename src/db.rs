use deadpool_diesel::mysql::{Manager, Pool};
use diesel::Connection;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<Manager<MysqlConnection>>;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database not found");
    MysqlConnection::establish(&database_url).unwrap()
}

fn create_pool() -> DbPool {

}