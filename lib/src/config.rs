pub struct Config {
    pub host: String,
    pub port: u16,
    pub cors_origin: String,
    pub postgres_url: String,
    pub encryption_key: String,
}

impl Config {
    pub fn new() -> Self {
        #[rustfmt::skip]
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("PORT must be a number");

        #[rustfmt::skip]
        let cors_origin = std::env::var("CORS_ORIGIN")
            .unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());

        #[rustfmt::skip]
        let postgres_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_|
                "postgres://postgres:postgres@localhost:5432/postgres"
                .to_string()
            );

        let set_insecure_encryption_key = || {
            tracing::warn!("ENCRYPTION_KEY not set. Reverting to a default key -- this is NOT SAFE FOR PRODUCTION. This must be set to at least 64-bytes generated from a CSPRNG (eg. openssl rand -base64 64).");

            "dLvewSBvt0VNAJX4p7HLvBAfIeltnMCeOBHgzh7FBrDeysTm4FTkAVvEH4ydFdNezrGY65dy99lWSCTrb27IIA==".to_string()
        };

        #[rustfmt::skip]
        let encryption_key = std::env::var("ENCRYPTION_KEY");

        #[allow(clippy::unnecessary_unwrap)]
        let encryption_key =
            if encryption_key.is_err() || encryption_key.as_ref().unwrap().len() < 64 {
                set_insecure_encryption_key()
            } else {
                encryption_key.unwrap()
            };

        Config {
            host,
            port,
            cors_origin,
            postgres_url,
            encryption_key,
        }
    }
}
