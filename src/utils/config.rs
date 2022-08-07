#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            host: "127.0.0.1".to_string(),
            port: 8080,
            db_url: "postgres://postgres:postgres@localhost:5432/postgres".to_string(),
            jwt_secret: "secret".to_string(),
        };

        config.load_config();
        return config;
    }

    fn load_config() {}
}
