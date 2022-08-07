#[derive(Debug, Clone)]
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

    fn load_config(&mut self) {
        if let Ok(host) = std::env::var("HOST") {
            self.host = host;
        }
        if let Ok(port) = std::env::var("PORT") {
            self.port = port.parse::<u16>().unwrap();
        }
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            self.db_url = db_url;
        }
        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            self.jwt_secret = jwt_secret;
        }
    }
}
