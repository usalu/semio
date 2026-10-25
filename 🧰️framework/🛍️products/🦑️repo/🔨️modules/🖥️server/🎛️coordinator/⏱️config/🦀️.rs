//! ⏱️ Config of the coordinator: the service configuration with its defaults and its environment reading. See [`super`].


//#region ⏱️Config
/// ⚙️ Every coordinator setting, read from the environment with the same defaults as the Go twin.
#[derive(Debug, Clone)]
pub struct Config {
    pub address: String,
    pub database_path: String,
    pub repo_root: String,
    pub token: String,
    pub github_secret: String,
    pub discord_webhook: String,
    pub request_body_limit: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8787".to_string(),
            database_path: "compose-server.db".to_string(),
            repo_root: String::new(),
            token: String::new(),
            github_secret: String::new(),
            discord_webhook: String::new(),
            request_body_limit: 10 * 1024 * 1024,
        }
    }
}

impl Config {
    /// 🌱️ Reads `COMPOSE_SERVER_*` with the documented fallbacks.
    pub fn from_env() -> Self {
        let cwd = std::env::current_dir().map(|path| path.display().to_string()).unwrap_or_default();
        Self {
            address: env_or("COMPOSE_SERVER_ADDR", "127.0.0.1:8787"),
            database_path: env_or("COMPOSE_SERVER_DB", "compose-server.db"),
            repo_root: env_or("COMPOSE_SERVER_REPO_ROOT", &cwd),
            token: env_or("COMPOSE_SERVER_TOKEN", ""),
            github_secret: env_or("COMPOSE_SERVER_GITHUB_SECRET", ""),
            discord_webhook: env_or("COMPOSE_SERVER_DISCORD_WEBHOOK", ""),
            request_body_limit: env_or("COMPOSE_SERVER_BODY_LIMIT", "")
                .parse()
                .unwrap_or(10 * 1024 * 1024),
        }
    }
}

fn env_or(key: &str, fallback: &str) -> String {
    match std::env::var(key) {
        Ok(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => fallback.to_string(),
    }
}
//#endregion ⏱️Config
