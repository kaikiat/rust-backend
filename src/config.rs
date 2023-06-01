use rocket::config::Config;
use rocket::fairing::AdHoc;
use rocket::figment::Figment;
use std::collections::HashMap;
use std::env;

/// Debug only secret for JWT encoding & decoding.
// const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const TOKEN_PREFIX: &'static str = "Token ";

/// Create rocket config from environment variables
pub fn from_env() -> Figment {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", database_url);
    databases.insert("diesel_postgres_pool", database_config);

    Config::figment()
        .merge(("port", port))
        .merge(("databases", databases))
}