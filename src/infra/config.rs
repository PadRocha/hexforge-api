use std::{
    env::{self, VarError},
    num::ParseIntError,
    str::FromStr,
    time::Duration,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to load .env file")]
    Dotenv {
        #[source]
        source: dotenvy::Error,
    },

    #[error("missing required env var `{name}`")]
    MissingEnv { name: &'static str },

    #[error("env var `{name}` cannot be empty")]
    EmptyEnv { name: &'static str },

    #[error("env var `{name}` must be a valid integer")]
    InvalidInt {
        name: &'static str,
        #[source]
        source: ParseIntError,
    },

    #[error("env var `{name}` contains invalid Unicode")]
    InvalidUnicode { name: &'static str },
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_address: String,
    pub cors_allowed_origin: String,
    pub mongo: MongoConfig,
}

#[derive(Debug, Clone)]
pub struct MongoConfig {
    pub uri: String,
    pub database_name: String,
    pub app_name: String,
    pub max_pool_size: Option<u32>,
    pub min_pool_size: Option<u32>,
    pub max_idle_time: Option<Duration>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let server_address = optional_env_or_default("SERVER_ADDRESS", "127.0.0.1:3001")?;
        let cors_allowed_origin =
            optional_env_or_default("CORS_ALLOWED_ORIGIN", "http://localhost:5173")?;

        let mongo = MongoConfig {
            uri: required_env("MONGODB_URI")?,
            database_name: required_env("MONGODB_DATABASE")?,
            app_name: optional_env_or_default("MONGODB_APP_NAME", "apihules")?,
            max_pool_size: optional_u32_env("MONGODB_MAX_POOL_SIZE")?,
            min_pool_size: optional_u32_env("MONGODB_MIN_POOL_SIZE")?,
            max_idle_time: optional_u64_env("MONGODB_MAX_IDLE_TIME_SECS")?.map(Duration::from_secs),
        };

        Ok(Self {
            server_address,
            cors_allowed_origin,
            mongo,
        })
    }
}

pub fn load_environment() -> Result<(), ConfigError> {
    match dotenvy::dotenv() {
        Ok(_) => Ok(()),
        Err(error) if error.not_found() => Ok(()),
        Err(source) => Err(ConfigError::Dotenv { source }),
    }
}

fn required_env(name: &'static str) -> Result<String, ConfigError> {
    let value = read_env(name)?.ok_or(ConfigError::MissingEnv { name })?;

    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ConfigError::EmptyEnv { name });
    }

    Ok(trimmed.to_owned())
}

fn optional_env_or_default(name: &'static str, default: &str) -> Result<String, ConfigError> {
    Ok(match read_env(name)? {
        Some(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                default.to_owned()
            } else {
                trimmed.to_owned()
            }
        }
        _ => default.to_owned(),
    })
}

fn optional_u32_env(name: &'static str) -> Result<Option<u32>, ConfigError> {
    optional_int_env(name)
}

fn optional_u64_env(name: &'static str) -> Result<Option<u64>, ConfigError> {
    optional_int_env(name)
}

fn optional_int_env<T>(name: &'static str) -> Result<Option<T>, ConfigError>
where
    T: FromStr<Err = ParseIntError>,
{
    read_env(name)?
        .and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_owned())
        })
        .map(|value| parse_int(name, &value))
        .transpose()
}

fn parse_int<T>(name: &'static str, value: &str) -> Result<T, ConfigError>
where
    T: FromStr<Err = ParseIntError>,
{
    value
        .parse()
        .map_err(|source| ConfigError::InvalidInt { name, source })
}

fn read_env(name: &'static str) -> Result<Option<String>, ConfigError> {
    match env::var(name) {
        Ok(value) => Ok(Some(value)),
        Err(VarError::NotPresent) => Ok(None),
        Err(VarError::NotUnicode(_)) => Err(ConfigError::InvalidUnicode { name }),
    }
}
