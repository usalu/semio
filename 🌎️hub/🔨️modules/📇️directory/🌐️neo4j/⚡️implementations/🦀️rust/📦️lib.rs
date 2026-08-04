mod header {
    // 🧲️Header
    // HubDirectory over Neo4j (neo4rs). Users/Spaces/Memberships are real nodes+relationships —
    // where graph traversal earns its keep (role lookups, VFS tree walks). Document persistence
    // and blobs are no longer this crate's concern — `db::Database` and `db_storage_neo4j` own
    // that half now (see `os-semio_hub`'s `bin.rs`).
}

use async_trait::async_trait;
use neo4rs::{query, Graph};
use os_hub_directory::error::{DirectoryError, DirectoryResult};
use os_hub_directory::model::*;
use os_hub_directory::HubDirectory;
use uuid::Uuid;

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

const CONSTRAINTS: &[&str] = &[
    "CREATE CONSTRAINT IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:Space) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (t:ShareToken) REQUIRE t.token IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthSession) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:SyncSession) REQUIRE s.id IS UNIQUE",
];

/// @emoji 🕸️ Neo4j-backed `HubDirectory`.
pub struct Neo4jDirectory {
    graph: Graph,
}

impl Neo4jDirectory {
    /// @emoji 🔌️ Connects to `uri` with `user`/`password` and bootstraps uniqueness constraints.
    pub async fn connect(uri: &str, user: &str, password: &str) -> DirectoryResult<Self> {
        let graph = Graph::new(uri, user, password).await.map_err(backend)?;
        for statement in CONSTRAINTS {
            graph.run(query(statement)).await.map_err(backend)?;
        }
        Ok(Self { graph })
    }

    /// @emoji 🌱️ Seeds a default `studio`/`private` space authored by a `seed` system user node.
    pub async fn seed(&self) -> DirectoryResult<()> {
        let mut existing = self.graph.execute(query("MATCH (s:Space {id: 'default'}) RETURN s.id AS id")).await.map_err(backend)?;
        if existing.next().await.map_err(backend)?.is_none() {
            self.graph
                .run(query("CREATE (s:Space {id: 'default', name: 'Space', ownerUserId: 'seed', kind: 'studio', visibility: 'private', createdAt: $created_at})").param("created_at", now_ms()))
                .await
                .map_err(backend)?;
            let mut seed_user = self.graph.execute(query("MATCH (u:User {id: 'seed'}) RETURN u.id AS id")).await.map_err(backend)?;
            if seed_user.next().await.map_err(backend)?.is_some() {
                self.upsert_membership("default", "seed", SpaceRole::Author).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl HubDirectory for Neo4jDirectory {
    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String> {
        let token = Uuid::now_v7().to_string();
        self.graph.run(query("CREATE (t:ShareToken {token: $token, documentId: $document_id, createdAt: $created_at})").param("document_id", document_id).param("token", token.clone()).param("created_at", now_ms())).await.map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool> {
        let mut count_result = self.graph.execute(query("MATCH (t:ShareToken {documentId: $document_id}) RETURN count(t) AS c").param("document_id", document_id)).await.map_err(backend)?;
        let has_tokens: i64 = count_result.next().await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
        if has_tokens == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let mut valid_result = self.graph.execute(query("MATCH (t:ShareToken {token: $token, documentId: $document_id}) RETURN count(t) AS c").param("token", token).param("document_id", document_id)).await.map_err(backend)?;
                let valid: i64 = valid_result.next().await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
                Ok(valid > 0)
            }
        }
    }
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        self.graph
            .run(
                query(
                    "CREATE (u:User {id: $id, email: $email, displayName: $display_name, passwordHash: $password_hash,
                                      ssoSubject: $sso_subject, ssoProvider: $sso_provider, createdAt: $created_at})",
                )
                .param("id", id.clone())
                .param("email", email)
                .param("display_name", display_name)
                .param("password_hash", password_hash.unwrap_or_default())
                .param("sso_subject", sso_subject.unwrap_or_default())
                .param("sso_provider", sso_provider.unwrap_or_default())
                .param("created_at", created_at),
            )
            .await
            .map_err(backend)?;
        Ok(UserRecord {
            id,
            email: email.to_string(),
            display_name: display_name.to_string(),
            password_hash: password_hash.map(str::to_string),
            sso_subject: sso_subject.map(str::to_string),
            sso_provider: sso_provider.map(str::to_string),
            created_at,
        })
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {email: $email}) RETURN u AS u").param("email", email)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {ssoProvider: $provider, ssoSubject: $subject}) RETURN u AS u").param("provider", provider).param("subject", subject)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User) RETURN u AS u ORDER BY u.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut users = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            users.push(user_from_node(&row)?);
        }
        Ok(users)
    }
    //#endregion

    //#region Spaces
    async fn create_space(&self, name: &str, owner_user_id: &str, kind: &str, visibility: &str) -> DirectoryResult<SpaceRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        // 🎯️ An archive is frozen — even its owner is a spectator, never an author (matches
        // `upsert_membership`'s own archive-rejects-author law below).
        let owner_role = if kind == "archive" { "spectator" } else { "author" };
        self.graph
            .run(
                query(
                    "MATCH (u:User {id: $owner_user_id})
                     CREATE (s:Space {id: $id, name: $name, ownerUserId: $owner_user_id, kind: $kind, visibility: $visibility, createdAt: $created_at})
                     CREATE (u)-[:MEMBER_OF {role: $owner_role, createdAt: $created_at}]->(s)",
                )
                .param("id", id.clone())
                .param("name", name)
                .param("owner_user_id", owner_user_id)
                .param("kind", kind)
                .param("visibility", visibility)
                .param("owner_role", owner_role)
                .param("created_at", created_at),
            )
            .await
            .map_err(backend)?;
        Ok(SpaceRecord { id, name: name.to_string(), owner_user_id: owner_user_id.to_string(), created_at, kind: kind.to_string(), visibility: visibility.to_string() })
    }

    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        let mut result = self.graph.execute(query("MATCH (s:Space {id: $space_id}) RETURN s AS s").param("space_id", space_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(space_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(s:Space) RETURN s AS s, m.role AS role ORDER BY s.createdAt").param("user_id", user_id)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let studio = space_from_node(&row)?;
            let role: String = row.get("role").map_err(backend)?;
            if let Some(role) = SpaceRole::parse(&role) {
                studios.push((studio, role));
            }
        }
        Ok(studios)
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let mut result = self.graph.execute(query("MATCH (s:Space) RETURN s AS s ORDER BY s.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            studios.push(space_from_node(&row)?);
        }
        Ok(studios)
    }

    async fn upsert_membership(&self, space_id: &str, user_id: &str, role: SpaceRole) -> DirectoryResult<()> {
        if role == SpaceRole::Author {
            if let Some(space) = self.get_space(space_id).await? {
                if space.kind == "archive" {
                    return Err(DirectoryError::Conflict(format!("space '{space_id}' is an archive; no author memberships are allowed")));
                }
                if space.kind == "atelier" {
                    let mut count_result = self
                        .graph
                        .execute(
                            query("MATCH (u:User)-[m:MEMBER_OF {role: 'author'}]->(:Space {id: $space_id}) WHERE u.id <> $user_id RETURN count(m) AS c")
                                .param("space_id", space_id)
                                .param("user_id", user_id),
                        )
                        .await
                        .map_err(backend)?;
                    let other_authors: i64 = count_result.next().await.map_err(backend)?.and_then(|row| row.get("c").ok()).unwrap_or(0);
                    if other_authors > 0 {
                        return Err(DirectoryError::Conflict(format!("space '{space_id}' is an atelier; it already has a distinct author")));
                    }
                }
            }
        }
        self.graph
            .run(
                query(
                    "MATCH (u:User {id: $user_id}), (s:Space {id: $space_id})
                     MERGE (u)-[m:MEMBER_OF]->(s)
                     ON CREATE SET m.role = $role, m.createdAt = $created_at
                     ON MATCH SET m.role = $role",
                )
                .param("user_id", user_id)
                .param("space_id", space_id)
                .param("role", role.as_str())
                .param("created_at", now_ms()),
            )
            .await
            .map_err(backend)?;
        Ok(())
    }

    async fn remove_membership(&self, space_id: &str, user_id: &str) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Space {id: $space_id}) DELETE m").param("user_id", user_id).param("space_id", space_id)).await.map_err(backend)?;
        Ok(())
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN m.role AS role").param("user_id", user_id).param("space_id", space_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => {
                let role: String = row.get("role").map_err(backend)?;
                Ok(SpaceRole::parse(&role))
            }
            None => Ok(None),
        }
    }
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        self.graph
            .run(
                query(
                    "MATCH (u:User {id: $user_id})
                     CREATE (a:AuthSession {id: $id, createdAt: $created_at, expiresAt: $expires_at, ssoProvider: $sso_provider})-[:BELONGS_TO]->(u)",
                )
                .param("user_id", user_id)
                .param("id", id.clone())
                .param("created_at", created_at)
                .param("expires_at", expires_at)
                .param("sso_provider", sso_provider.unwrap_or_default()),
            )
            .await
            .map_err(backend)?;
        Ok(AuthSessionRecord { id, user_id: user_id.to_string(), created_at, expires_at, sso_provider: sso_provider.map(str::to_string) })
    }

    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>> {
        let mut result = self
            .graph
            .execute(query("MATCH (a:AuthSession {id: $id})-[:BELONGS_TO]->(u:User) RETURN a.id AS id, u.id AS userId, a.createdAt AS createdAt, a.expiresAt AS expiresAt, a.ssoProvider AS ssoProvider").param("id", id))
            .await
            .map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(AuthSessionRecord {
                id: row.get("id").map_err(backend)?,
                user_id: row.get("userId").map_err(backend)?,
                created_at: row.get("createdAt").map_err(backend)?,
                expires_at: row.get("expiresAt").map_err(backend)?,
                sso_provider: row.get::<String>("ssoProvider").ok().filter(|s| !s.is_empty()),
            })),
            None => Ok(None),
        }
    }

    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (a:AuthSession {id: $id}) DETACH DELETE a").param("id", id)).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(&self, document_id: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str().to_string()).unwrap_or_default();
        match user_id {
            Some(user_id) => {
                self.graph
                    .run(
                        query(
                            "MATCH (u:User {id: $user_id})
                             CREATE (s:SyncSession {id: $id, documentId: $document_id, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})
                             CREATE (s)-[:AS_USER]->(u)",
                        )
                        .param("document_id", document_id)
                        .param("user_id", user_id)
                        .param("id", id.clone())
                        .param("client_label", client_label)
                        .param("role", role_str.clone())
                        .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
            None => {
                self.graph
                    .run(
                        query("CREATE (s:SyncSession {id: $id, documentId: $document_id, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})")
                            .param("document_id", document_id)
                            .param("id", id.clone())
                            .param("client_label", client_label)
                            .param("role", role_str.clone())
                            .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
        }
        Ok(SyncSessionRecord { id, document_id: document_id.to_string(), user_id: user_id.map(str::to_string), space_role, client_label: client_label.to_string(), connected_at, disconnected_at: None })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (s:SyncSession {id: $id}) SET s.disconnectedAt = $disconnected_at").param("id", sync_session_id).param("disconnected_at", now_ms())).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (s:SyncSession {documentId: $document_id})
                     OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                     RETURN s.id AS id, u.id AS userId, s.spaceRole AS role, s.clientLabel AS clientLabel,
                            s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                     ORDER BY s.connectedAt DESC",
                )
                .param("document_id", document_id),
            )
            .await
            .map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").unwrap_or_default();
            sessions.push(SyncSessionRecord {
                id: row.get("id").map_err(backend)?,
                document_id: document_id.to_string(),
                user_id: row.get::<String>("userId").ok(),
                space_role: SpaceRole::parse(&role),
                client_label: row.get("clientLabel").map_err(backend)?,
                connected_at: row.get("connectedAt").map_err(backend)?,
                disconnected_at: row.get::<i64>("disconnectedAt").ok(),
            });
        }
        Ok(sessions)
    }
    //#endregion
}

fn user_from_node(row: &neo4rs::Row) -> DirectoryResult<UserRecord> {
    let node: neo4rs::Node = row.get("u").map_err(backend)?;
    Ok(UserRecord {
        id: node.get("id").map_err(backend)?,
        email: node.get("email").map_err(backend)?,
        display_name: node.get("displayName").map_err(backend)?,
        password_hash: node.get::<String>("passwordHash").ok().filter(|s| !s.is_empty()),
        sso_subject: node.get::<String>("ssoSubject").ok().filter(|s| !s.is_empty()),
        sso_provider: node.get::<String>("ssoProvider").ok().filter(|s| !s.is_empty()),
        created_at: node.get("createdAt").map_err(backend)?,
    })
}

fn space_from_node(row: &neo4rs::Row) -> DirectoryResult<SpaceRecord> {
    let node: neo4rs::Node = row.get("s").map_err(backend)?;
    Ok(SpaceRecord {
        id: node.get("id").map_err(backend)?,
        name: node.get("name").map_err(backend)?,
        owner_user_id: node.get("ownerUserId").map_err(backend)?,
        created_at: node.get("createdAt").map_err(backend)?,
        kind: node.get("kind").map_err(backend)?,
        visibility: node.get("visibility").map_err(backend)?,
    })
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    // 🔬️ Neo4j has no in-memory test mode; integration tests run against a live/testcontainers
    // instance per the verification plan (HP-3) — not exercised in unit-test CI without a running
    // Neo4j, unlike the sqlite backend's `:memory:` tests.
}
//#endregion 🔖️Tests
