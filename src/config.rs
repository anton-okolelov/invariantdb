use std::env;

const DEFAULT_PORT: u16= 6444;

pub struct PgConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
}

pub struct Config {
    pub port: u16,
    pub pg: PgConfig,
}

impl Config {
    pub fn from_env() -> Result<Config, Box<dyn std::error::Error>> {
        let port = match env::var("PORT") {
            Ok(val) => val.parse::<u16>()?,
            Err(_) => DEFAULT_PORT
        } ;

        Ok(Config {
            port: port,
            pg: PgConfig {
                host: env::var("DB_HOST")?,
                port: env::var("DB_PORT")?.parse::<u16>()?,
                user: env::var("DB_USER")?,
                password: env::var("DB_PASS")?,
                db_name: env::var("DB_NAME")?,
            },
        })
    }
}
