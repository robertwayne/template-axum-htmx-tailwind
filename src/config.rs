use std::{env, error, net::SocketAddr, path::PathBuf};

use axum::http::HeaderValue;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub cors_origin: HeaderValue,
    pub postgres_url: String,
    pub encryption_key: String,
}

impl Config {
    pub fn new(path: &str) -> Self {
        Config::set_vars(path).expect("failed to set env vars");

        #[rustfmt::skip]
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        #[rustfmt::skip]
        let cors_origin = std::env::var("CORS_ORIGIN")
            .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());

        #[rustfmt::skip]
        let postgres_url = std::env::var("POSTGRES_URL")
            .unwrap_or_else(|_|
                "postgres://postgres:postgres@localhost:5432/postgres"
                .to_string()
            );

        let set_insecure_encryption_key = || {
            tracing::warn!("ENCRYPTION_KEY not set. Reverting to a default key -- this is NOT SAFE FOR PRODUCTION. This must be set to at least 64-bytes generated from a CSPRNG (eg. openssl rand -base64 64).");

            "dLvewSBvt0VNAJX4p7HLvBAfIeltnMCeOBHgzh7FBrDeysTm4FTkAVvEH4ydFdNezrGY65dy99lWSCTrb27IIA==".to_string()
        };

        #[rustfmt::skip]
        let encryption_key = match std::env::var("ENCRYPTION_KEY") {
            Ok(key) => {
                if key.len() < 64 {
                    set_insecure_encryption_key()
                } else {
                    key
                }
            }
            Err(_) => set_insecure_encryption_key(),
        };

        Config::unset_vars();

        Config {
            host,
            port,
            cors_origin: cors_origin.parse().expect("failed to parse CORS origin"),
            postgres_url,
            encryption_key,
        }
    }

    pub fn addr(&self) -> SocketAddr {
        let ip = self.host.parse().expect("failed to parse IP address");

        SocketAddr::new(ip, self.port)
    }

    /// Set environment variables from a given file.
    fn set_vars(path: impl Into<PathBuf>) -> Result<(), Box<dyn error::Error>> {
        let file = std::fs::read_to_string(path.into())?;

        for line in file.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') || line.contains('\0') {
                continue;
            }

            let parts = line.splitn(2, '=').collect::<Vec<_>>();

            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();

            if key.is_empty() || key.contains(['=']) || value.is_empty() {
                continue;
            }

            env::set_var(key, value)
        }

        Ok(())
    }

    /// Unset the environment variables set by `Config::set_vars`.
    fn unset_vars() {
        env::remove_var("HOST");
        env::remove_var("PORT");
        env::remove_var("CORS_ORIGIN");
        env::remove_var("POSTGRES_URL");
        env::remove_var("ENCRYPTION_KEY");
    }
}
