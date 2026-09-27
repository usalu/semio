//! 🗝️ The claimed shared development server of the db crate's live storage laws (the WAL writer fence lanes, the
//! PostgreSQL round-trip laws) and — through the `claimed-backend-laws` feature — of other crates' live laws (the hub's
//! directory lanes): `os-hub-ts backend run <postgres|neo4j> -- …` claims the ONE shared server (a run database of its own
//! on postgres, the exclusive lease over a reset graph on neo4j), hands the command the hub environment that selects it
//! (`OS_HUB_DATABASE_URL`, `OS_HUB_NEO4J_URI`/`_USER`/`_PASSWORD`) and the server's own client scoped to the claim as a
//! JSON argv (`SEMIO_BACKEND_CLIENT`: `psql` on the run database, `cypher-shell`), and releases the claim afterwards. A law
//! never starts a server of its own (preamble rule 22: one shared pg/neo4j on the Docker VM).

/// 🔑️ One variable of the claim's hub environment.
pub fn env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} is unset: run this law under `os-hub-ts backend run <postgres|neo4j> -- …`"))
}

/// 🔎️ Runs one SQL statement or Cypher query through the claimed server's own client and answers its trimmed output.
pub fn client(query: &str) -> String {
    try_client(query).unwrap_or_else(|failure| panic!("{query}: {failure}"))
}

/// 🧯️ [`client`] that answers the client's failure instead of panicking (for cleanup on a failing law's drop path).
pub fn try_client(query: &str) -> Result<String, String> {
    let argv: Vec<String> = serde_json::from_str(&env("SEMIO_BACKEND_CLIENT")).map_err(|error| format!("SEMIO_BACKEND_CLIENT is not a JSON argv: {error}"))?;
    let (program, arguments) = argv.split_first().ok_or("SEMIO_BACKEND_CLIENT names no program")?;
    let output = std::process::Command::new(program).args(arguments).arg(query).output().map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// 🆕️ A PostgreSQL database of one law's own on the claimed server, created through the server's own client and dropped
/// with the guard, so laws running side by side in one claim never share tables or table statistics.
pub struct FreshPostgresDatabase {
    pub name: String,
    pub url: String,
}

impl FreshPostgresDatabase {
    /// 🆕️ Creates `<claimed run database>_<suffix>` and answers its connection URL.
    pub fn create(suffix: &str) -> Self {
        let name = format!("{}_{suffix}", client("SELECT current_database()"));
        client(&format!("DROP DATABASE IF EXISTS {name} WITH (FORCE)"));
        client(&format!("CREATE DATABASE {name}"));
        let run = env("OS_HUB_DATABASE_URL");
        let url = format!("{}/{name}", run.rsplit_once('/').expect("the claimed URL names its database").0);
        Self { name, url }
    }
}

impl Drop for FreshPostgresDatabase {
    fn drop(&mut self) {
        let _ = try_client(&format!("DROP DATABASE IF EXISTS {} WITH (FORCE)", self.name));
    }
}
