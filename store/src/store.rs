
use diesel::Connection;
use diesel::ConnectionError;
use diesel::PgConnection;

use crate::config::Config;
pub struct Store {
   pub conn : PgConnection,
}

impl Store {
    pub fn new() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn })
    }
}
