use rocket::config::Config;
use rocket::figment::Figment;
use std::collections::HashMap;
use std::env;

pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

// pub const TOKEN_PREFIX: &'static str = "Token ";

/// Create rocket config from environment variables
pub fn from_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", database_url);
    databases.insert("diesel_postgres_pool", database_config);

    Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", port))
        .merge(("databases", databases))
}