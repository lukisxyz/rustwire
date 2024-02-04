use dotenv::dotenv;
use log::warn;
use serde::{Deserialize, Serialize};
use std::env;
use std::error;
use std::path::Path;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 8080;
const DEFAULT_READ_TIMEOUT: u32 = 25;
const DEFAULT_WRITE_TIMEOUT: u32 = 25;
const DEFAULT_IDLE_TIMEOUT: u32 = 300;

const DEFAULT_DB_HOST: &str = "127.0.0.1";
const DEFAULT_DB_PORT: u32 = 5433;
const DEFAULT_DB_NAME: &str = "postgres";
const DEFAULT_DB_SSL_MODE: &str = "disable";
const DEFAULT_DB_PASSWORD: &str = "password";
const DEFAULT_DB_USERNAME: &str = "postgres";

trait AppConfig {
    fn load_default() -> Self;
    fn load_config(yaml: &serde_yaml::Value) -> Self;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListenerConfig {
    pub host: String,
    pub port: u32,
    pub read_timeout: u32,
    pub write_timeout: u32,
    pub idle_timeout: u32,
}

impl AppConfig for ListenerConfig {
    fn load_default() -> Self {
        ListenerConfig {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            read_timeout: DEFAULT_READ_TIMEOUT,
            write_timeout: DEFAULT_WRITE_TIMEOUT,
            idle_timeout: DEFAULT_IDLE_TIMEOUT,
        }
    }

    fn load_config(yaml: &serde_yaml::Value) -> Self {
        ListenerConfig {
            host: load_cfg_str(yaml, "host", "LISTENER_HOST", DEFAULT_HOST),
            port: load_cfg_uint(yaml, "port", "LISTENER_PORT", DEFAULT_PORT),
            read_timeout: load_cfg_uint(
                yaml,
                "read_timeout",
                "LISTENER_READ_TIMEOUT",
                DEFAULT_READ_TIMEOUT,
            ),
            write_timeout: load_cfg_uint(
                yaml,
                "write_timeout",
                "LISTENER_WRITE_TIMEOUT",
                DEFAULT_WRITE_TIMEOUT,
            ),
            idle_timeout: load_cfg_uint(
                yaml,
                "idle_timeout",
                "LISTENER_IDLE_TIMEOUT",
                DEFAULT_IDLE_TIMEOUT,
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PgConfig {
    pub host: String,
    pub port: u32,
    pub db_name: String,
    pub ssl_mode: String,
    pub password: String,
    pub username: String,
}

impl AppConfig for PgConfig {
    fn load_default() -> Self {
        PgConfig {
            host: DEFAULT_DB_HOST.to_string(),
            port: DEFAULT_DB_PORT,
            db_name: DEFAULT_DB_NAME.to_string(),
            ssl_mode: DEFAULT_DB_SSL_MODE.to_string(),
            password: DEFAULT_DB_PASSWORD.to_string(),
            username: DEFAULT_DB_USERNAME.to_string(),
        }
    }

    fn load_config(yaml: &serde_yaml::Value) -> Self {
        PgConfig {
            host: load_cfg_str(yaml, "host", "DB_HOST", DEFAULT_DB_HOST),
            port: load_cfg_uint(yaml, "port", "DB_PORT", DEFAULT_DB_PORT),
            db_name: load_cfg_str(yaml, "db_name", "DB_NAME", DEFAULT_DB_NAME),
            ssl_mode: load_cfg_str(yaml, "ssl_mode", "DB_SSL", DEFAULT_DB_SSL_MODE),
            password: load_cfg_str(yaml, "password", "DB_PASSWORD", DEFAULT_DB_PASSWORD),
            username: load_cfg_str(yaml, "username", "DB_USER", DEFAULT_DB_USERNAME),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub listen: ListenerConfig,
    pub db: PgConfig,
}

impl Config {
    fn default() -> Self {
        let listen = ListenerConfig::load_default();
        let db = PgConfig::load_default();
        Config { listen, db }
    }

    fn load_config(file_path: &str) -> Result<Config, Box<dyn error::Error>> {
        if Path::new(file_path).exists() {
            let content = std::fs::read_to_string(file_path)?;
            let yaml: serde_yaml::Value = serde_yaml::from_str(&content)?;

            let mut config = Config::default();
            config.listen = ListenerConfig::load_config(&yaml["listen"]);
            config.db = PgConfig::load_config(&yaml["db"]);

            Ok(config)
        } else {
            Err("Config file not found".into())
        }
    }
}

fn load_env_str(key: &str) -> Option<String> {
    load_env();
    env::var(key).ok()
}

fn load_env_uint(key: &str) -> Option<u32> {
    load_env();
    env::var(key).ok().and_then(|s| s.parse().ok())
}

fn load_cfg_str(yaml: &serde_yaml::Value, key: &str, env_key: &str, default: &str) -> String {
    let value = load_env_str(env_key).map_or_else(
        || {
            let value_from_yaml = yaml[key].as_str().map_or_else(
                || {
                    warn!("{} not found in YAML config. Using default", key);
                    default.to_string()
                },
                |s| s.to_string(),
            );
            value_from_yaml
        },
        |s| {
            warn!(
                "{} not found in YAML config. Using value from environment",
                key
            );
            s
        },
    );

    value
}

fn load_cfg_uint(yaml: &serde_yaml::Value, key: &str, env_key: &str, default: u32) -> u32 {
    let value = load_env_uint(env_key).map_or_else(
        || {
            let value_from_yaml = yaml[key].as_u64().map_or_else(
                || {
                    warn!("{} not found in YAML config. Using default", key);
                    default
                },
                |u| u as u32,
            );

            value_from_yaml
        },
        |u| {
            warn!(
                "{} not found in YAML config. Using value from environment",
                key
            );
            u
        },
    );

    value
}

fn load_env() {
    if let Err(err) = dotenv() {
        warn!("Error loading .env file: {:?}", err);
    }
}

pub fn load(file_path: &str) -> Config {
    load_env();
    let mut cfg = Config::default();
    match Config::load_config(file_path) {
        Err(err) => warn!(
            "Cannot load config file: {}. Using defaults. Error: {:?}",
            file_path, err
        ),
        Ok(v) => cfg = v,
    }

    cfg
}
