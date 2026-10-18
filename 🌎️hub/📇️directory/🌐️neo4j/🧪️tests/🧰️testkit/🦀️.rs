//! 🧰️ Live-lane fixture of the neo4j directory backend: the claimed shared server and the directory over an emptied store.

use crate::Neo4jDirectory;

/// 🕸️ The ONE claimed shared Neo4j (`os-hub-ts backend run neo4j -- …`, an exclusive lease): Community edition serves one
/// graph, so each lane starts from an emptied graph and the lanes run one at a time (`directory-live-lanes neo4j` passes
/// `--test-threads=1`) — the lanes never start a server of their own (preamble rule 22).
pub struct Neo4jFixture {
    pub uri: String,
    pub user: String,
    pub password: String,
}

impl Neo4jFixture {
    pub async fn connect(&self) -> Neo4jDirectory {
        Neo4jDirectory::connect(&self.uri, &self.user, &self.password).await.expect("connect second neo4j directory")
    }
}

/// 🕸️ The claimed shared Neo4j with an emptied graph, and the directory over it.
pub async fn test_directory() -> (Neo4jDirectory, Neo4jFixture) {
    db::storage::claimed_backend::client("MATCH (n) CALL { WITH n DETACH DELETE n } IN TRANSACTIONS OF 10000 ROWS");
    let fixture = Neo4jFixture { uri: db::storage::claimed_backend::env("OS_HUB_NEO4J_URI"), user: db::storage::claimed_backend::env("OS_HUB_NEO4J_USER"), password: db::storage::claimed_backend::env("OS_HUB_NEO4J_PASSWORD") };
    let directory = fixture.connect().await;
    (directory, fixture)
}
