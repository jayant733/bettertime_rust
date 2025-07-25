use std::env;


pub struct Config {
    pub db_url : String,


}

impl Default for Config {
    fn default() -> Self{
          dotenv::dotenv().ok();
           let db_url: String = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("please set the DATABASE_URL environment variable"));
           Self { db_url }
    }
 }