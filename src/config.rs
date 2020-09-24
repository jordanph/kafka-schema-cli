pub struct AppConfig {
    pub schema_registry_url: String,
    pub bootstrap_servers: String,
}

impl AppConfig {
    pub fn from_env() -> AppConfig {
        let schema_registry_url = std::env::var("SCHEMA_REGISTRY_URL")
            .unwrap_or_else(|_| "http://localhost:8081".to_string());

        let bootstrap_servers =
            std::env::var("BOOTSTRAP_SERVERS").unwrap_or_else(|_| "localhost:39092".to_string());

        println!("⚙️  Config");
        println!("----------------------------------------------");
        println!("🔧 Schema registry url: {}", schema_registry_url);
        println!("🥾 Kafka Bootstrap servers: {}\n", bootstrap_servers);

        AppConfig {
            schema_registry_url,
            bootstrap_servers,
        }
    }
}
