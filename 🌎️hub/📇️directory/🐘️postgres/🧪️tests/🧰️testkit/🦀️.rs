//! 🧰️ Live-lane fixture of the postgres directory backend: the claimed shared server and the directory over an emptied store.

use std::sync::atomic::{AtomicU64, Ordering};
use crate::PostgresDirectory;

//#region 🔖️PostgresFixture
static NEXT_DATABASE: AtomicU64 = AtomicU64::new(1);

/// 🐘️ One lane's own database on the ONE claimed shared PostgreSQL (`os-hub-ts backend run postgres -- …`), dropped with
/// the guard — the lanes never start a server of their own (preamble rule 22).
pub struct PostgresFixture {
    database: db::storage::claimed_backend::FreshPostgresDatabase,
}

impl PostgresFixture {
    pub fn url(&self) -> &str {
        &self.database.url
    }

    pub async fn connect(&self) -> PostgresDirectory {
        PostgresDirectory::connect(self.url()).await.expect("connect second postgres directory")
    }
}

/// 🐘️ A fresh database of this lane's own on the claimed shared PostgreSQL and the directory over it.
pub async fn test_directory() -> (PostgresDirectory, PostgresFixture) {
    let sequence = NEXT_DATABASE.fetch_add(1, Ordering::Relaxed);
    let database = db::storage::claimed_backend::FreshPostgresDatabase::create(&format!("hub_{}_{sequence}", std::process::id()));
    let directory = PostgresDirectory::connect(&database.url).await.expect("connect postgres directory on the claimed server");
    (directory, PostgresFixture { database })
}
//#endregion 🔖️PostgresFixture
