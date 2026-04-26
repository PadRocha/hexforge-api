use std::{
    env::{self, VarError},
    num::ParseIntError,
    str::FromStr,
    time::Duration,
};

use thiserror::Error;

/// Errores producidos al cargar o interpretar configuración.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// El archivo `.env` existe, pero no pudo leerse o parsearse.
    #[error("failed to load .env file")]
    Dotenv {
        /// Error original reportado por `dotenvy`.
        #[source]
        source: dotenvy::Error,
    },

    /// Una variable obligatoria no está presente en el entorno.
    #[error("missing required env var `{name}`")]
    MissingEnv {
        /// Nombre de la variable requerida.
        name: &'static str,
    },

    /// Una variable obligatoria existe, pero solo contiene espacios o está vacía.
    #[error("env var `{name}` cannot be empty")]
    EmptyEnv {
        /// Nombre de la variable vacía.
        name: &'static str,
    },

    /// Una variable numérica no pudo convertirse al tipo esperado.
    #[error("env var `{name}` must be a valid integer")]
    InvalidInt {
        /// Nombre de la variable inválida.
        name: &'static str,
        /// Error original de parseo numérico.
        #[source]
        source: ParseIntError,
    },

    /// Una variable de entorno contiene bytes que no son Unicode válido.
    #[error("env var `{name}` contains invalid Unicode")]
    InvalidUnicode {
        /// Nombre de la variable inválida.
        name: &'static str,
    },
}

/// Configuración efectiva de la API.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Dirección donde el servidor HTTP intentará escuchar.
    pub server_address: String,
    /// Origen autorizado para CORS con credenciales.
    pub cors_allowed_origin: String,
    /// Configuración de MongoDB.
    pub mongo: MongoConfig,
}

/// Configuración de conexión y pool para MongoDB.
#[derive(Debug, Clone)]
pub struct MongoConfig {
    /// URI de conexión aceptada por el driver oficial de MongoDB.
    pub uri: String,
    /// Nombre de la base de datos que usará la API.
    pub database_name: String,
    /// Nombre de aplicación enviado al servidor MongoDB.
    pub app_name: String,
    /// Tamaño máximo opcional del pool de conexiones.
    pub max_pool_size: Option<u32>,
    /// Tamaño mínimo opcional del pool de conexiones.
    pub min_pool_size: Option<u32>,
    /// Tiempo máximo opcional que una conexión puede permanecer ociosa.
    pub max_idle_time: Option<Duration>,
}

impl AppConfig {
    /// Construye la configuración leyendo variables de entorno.
    ///
    /// Las variables obligatorias son `MONGODB_URI` y `MONGODB_DATABASE`.
    /// `SERVER_ADDRESS`, `CORS_ALLOWED_ORIGIN`, `MONGODB_APP_NAME` y los ajustes
    /// de pool tienen valores por defecto o son opcionales.
    ///
    /// # Errors
    ///
    /// Devuelve [`ConfigError`] cuando falta una variable obligatoria, una
    /// variable obligatoria está vacía, una variable numérica no es válida o el
    /// entorno contiene valores que no son Unicode.
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

/// Carga variables desde `.env` si el archivo existe.
///
/// La ausencia de `.env` no es un error para permitir configuración por entorno
/// en contenedores, CI o despliegues.
///
/// # Errors
///
/// Devuelve [`ConfigError::Dotenv`] si el archivo `.env` existe pero no puede
/// leerse o parsearse.
///
/// # Side effects
///
/// Modifica el entorno del proceso con las variables definidas en `.env`.
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
