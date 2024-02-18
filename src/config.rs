use config::{Config, ConfigError, Environment}; // config from config dependency 
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
  pub host: String,
  pub port: i32
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
  pub url: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub database: DatabaseConfig
}

impl AppConfig {
  pub fn from_env() -> Result<Self, ConfigError> {
    let builder = Config::builder()
    .add_source(Environment::with_prefix("APP").separator("_")); // for eg APP_SERVER_HOST, APP_DATABASE_HOST etc.
    let cfg = builder.build()?;

    let server_config = cfg.get::<ServerConfig>("server")?;
    let database_config = cfg.get::<DatabaseConfig>("database")?; // TODO : make theses 3 lines into One ?
    Ok(AppConfig { server: server_config, database: database_config })
  }
}