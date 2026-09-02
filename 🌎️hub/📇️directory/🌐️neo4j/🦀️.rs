//! 🌐️ `HubDirectory` over Neo4j (neo4rs). Users/Spaces/Memberships are real nodes+relationships —
//! where graph traversal earns its keep (role lookups, VFS tree walks). Document persistence and
//! blobs are no longer this crate's concern — `db::Database` and `db_storage_neo4j` own that now
//! (see `bin.rs`). `#[cfg(feature = "neo4j")]`-gated as a whole by the parent `directory` module
//! (see `📇️directory/🦀️.rs`'s `//#region 🔖️Backends`).

use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::*;
use crate::directory::{kind_to_str, role_from_wire, visibility_to_str, HubClock, HubDirectory, NewDirectoryEvent};
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, Hlc};
use directory::os_identity::time_ordered_id;
use neo4rs::{query, Graph, Txn};

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

fn actor_kind_to_str(kind: DirectoryActorKind) -> &'static str {
    match kind {
        DirectoryActorKind::User => "user",
        DirectoryActorKind::Admin => "admin",
        DirectoryActorKind::System => "system",
    }
}

fn actor_kind_from_str(value: &str) -> DirectoryActorKind {
    match value {
        "admin" => DirectoryActorKind::Admin,
        "system" => DirectoryActorKind::System,
        _ => DirectoryActorKind::User,
    }
}

const CONSTRAINTS: &[&str] = &[
    "CREATE CONSTRAINT IF NOT EXISTS FOR (u:User) REQUIRE u.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:Space) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (t:ShareToken) REQUIRE t.token IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (a:AuthSession) REQUIRE a.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (s:SyncSession) REQUIRE s.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:SpaceInvite) REQUIRE i.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (i:SpaceInvite) REQUIRE i.token IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.id IS UNIQUE",
    "CREATE CONSTRAINT IF NOT EXISTS FOR (e:DirectoryEvent) REQUIRE e.seq IS UNIQUE",
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

    /// @emoji 🌱️ Seeds a default `studio`/`private` space authored by a `seed` system user node,
    /// through the event log (`user.created` + `space.created` + `member.upserted`) like any other
    /// write.
    pub async fn seed(&self) -> DirectoryResult<()> {
        let mut existing = self.graph.execute(query("MATCH (u:User {id: 'seed'}) RETURN u.id AS id")).await.map_err(backend)?;
        if existing.next().await.map_err(backend)?.is_some() {
            return Ok(());
        }
        let actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:seed".into() };
        let mut clock = HubClock::new();
        let events = vec![
            NewDirectoryEvent { hlc: clock.tick(), actor: actor.clone(), space_id: None, user_id: Some("seed".into()), body: DirectoryEventBody::UserCreated { user_id: "seed".into(), email: "seed@localhost".into(), display_name: "System".into() } },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor.clone(),
                space_id: Some("default".into()),
                user_id: Some("seed".into()),
                body: DirectoryEventBody::SpaceCreated { space_id: "default".into(), name: "Space".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private, owner_user_id: "seed".into() },
            },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor,
                space_id: Some("default".into()),
                user_id: Some("seed".into()),
                body: DirectoryEventBody::MemberUpserted { space_id: "default".into(), user_id: "seed".into(), role: DirectorySpaceRole::Author },
            },
        ];
        self.append_events(&events).await?;
        Ok(())
    }

    //#region 🔖️Projections
    /// @emoji 🧮️ The only place `:User`/`:Space`/`:MEMBER_OF` graph state is written — see the
    /// sqlite backend's twin for the full rationale (unconditional: `decide` already enforced every
    /// law before this event existed).
    async fn project(&self, txn: &mut Txn, event: &DirectoryEvent) -> DirectoryResult<()> {
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                txn.run(
                    query("MERGE (u:User {id: $id}) ON CREATE SET u.email = $email, u.displayName = $display_name, u.createdAt = $created_at")
                        .param("id", user_id.clone())
                        .param("email", email.clone())
                        .param("display_name", display_name.clone())
                        .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                txn.run(
                    query("MERGE (s:Space {id: $id}) ON CREATE SET s.name = $name, s.ownerUserId = $owner_user_id, s.kind = $kind, s.visibility = $visibility, s.createdAt = $created_at")
                        .param("id", space_id.clone())
                        .param("name", name.clone())
                        .param("owner_user_id", owner_user_id.clone())
                        .param("kind", kind_to_str(*space_kind))
                        .param("visibility", visibility_to_str(*visibility))
                        .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.name = $name").param("id", space_id.clone()).param("name", name.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.visibility = $visibility").param("id", space_id.clone()).param("visibility", visibility_to_str(*visibility))).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                txn.run(query("MATCH (s:Space {id: $id}) SET s.kind = 'archive'").param("id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                txn.run(query("MATCH (i:SpaceInvite {spaceId: $id}) DETACH DELETE i").param("id", space_id.clone())).await.map_err(backend)?;
                txn.run(query("MATCH (s:Space {id: $id}) DETACH DELETE s").param("id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                txn.run(
                    query(
                        "MATCH (u:User {id: $user_id}), (s:Space {id: $space_id})
                         MERGE (u)-[m:MEMBER_OF]->(s)
                         ON CREATE SET m.role = $role, m.createdAt = $created_at
                         ON MATCH SET m.role = $role",
                    )
                    .param("user_id", user_id.clone())
                    .param("space_id", space_id.clone())
                    .param("role", role_from_wire(*role).as_str())
                    .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                txn.run(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Space {id: $space_id}) DELETE m").param("user_id", user_id.clone()).param("space_id", space_id.clone())).await.map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                txn.run(
                    query(
                        "MATCH (u:User {id: $user_id}), (s:Space {id: $space_id})
                         MERGE (u)-[m:MEMBER_OF]->(s)
                         ON CREATE SET m.role = $role, m.createdAt = $created_at
                         ON MATCH SET m.role = $role",
                    )
                    .param("user_id", user_id.clone())
                    .param("space_id", space_id.clone())
                    .param("role", role_from_wire(*role).as_str())
                    .param("created_at", event.recorded_at_ms),
                )
                .await
                .map_err(backend)?;
            }
        }
        Ok(())
    }
    //#endregion 🔖️Projections
}

impl HubDirectory for Neo4jDirectory {
    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String> {
        let token = time_ordered_id();
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
        let id = time_ordered_id();
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

    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>> {
        let mut result = self.graph.execute(query("MATCH (u:User {id: $id}) RETURN u AS u").param("id", user_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(user_from_node(&row)?)),
            None => Ok(None),
        }
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

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        let mut result = self.graph.execute(query("MATCH (u:User)-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN u AS u, m.role AS role ORDER BY m.createdAt").param("space_id", space_id)).await.map_err(backend)?;
        let mut members = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let user = user_from_node(&row)?;
            let role: String = row.get("role").map_err(backend)?;
            if let Some(role) = SpaceRole::parse(&role) {
                members.push((user, role));
            }
        }
        Ok(members)
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
        let id = time_ordered_id();
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

    //#region Invites
    async fn create_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64) -> DirectoryResult<InviteRecord> {
        let id = time_ordered_id();
        let token = time_ordered_id();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        self.graph
            .run(
                query("CREATE (i:SpaceInvite {id: $id, token: $token, spaceId: $space_id, role: $role, createdAt: $created_at, expiresAt: $expires_at})")
                    .param("id", id.clone())
                    .param("token", token.clone())
                    .param("space_id", space_id)
                    .param("role", role.as_str())
                    .param("created_at", created_at)
                    .param("expires_at", expires_at),
            )
            .await
            .map_err(backend)?;
        Ok(InviteRecord { id, token, space_id: space_id.to_string(), role, created_at, expires_at, revoked_at: None })
    }

    async fn get_invite_by_token(&self, token: &str) -> DirectoryResult<Option<InviteRecord>> {
        let mut result = self.graph.execute(query("MATCH (i:SpaceInvite {token: $token}) RETURN i AS i").param("token", token)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(invite_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn revoke_invite(&self, invite_id: &str) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (i:SpaceInvite {id: $id}) SET i.revokedAt = $revoked_at").param("id", invite_id).param("revoked_at", now_ms())).await.map_err(backend)?;
        Ok(())
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let mut result = self.graph.execute(query("MATCH (i:SpaceInvite {spaceId: $space_id}) RETURN i AS i ORDER BY i.createdAt DESC").param("space_id", space_id)).await.map_err(backend)?;
        let mut invites = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            invites.push(invite_from_node(&row)?);
        }
        Ok(invites)
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(&self, space_id: &str, document_id: &str, surface: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord> {
        let id = time_ordered_id();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str().to_string()).unwrap_or_default();
        match user_id {
            Some(user_id) => {
                self.graph
                    .run(
                        query(
                            "MATCH (u:User {id: $user_id})
                             CREATE (s:SyncSession {id: $id, spaceId: $space_id, documentId: $document_id, surface: $surface, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})
                             CREATE (s)-[:AS_USER]->(u)",
                        )
                        .param("space_id", space_id)
                        .param("document_id", document_id)
                        .param("surface", surface)
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
                        query("CREATE (s:SyncSession {id: $id, spaceId: $space_id, documentId: $document_id, surface: $surface, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})")
                            .param("space_id", space_id)
                            .param("document_id", document_id)
                            .param("surface", surface)
                            .param("id", id.clone())
                            .param("client_label", client_label)
                            .param("role", role_str.clone())
                            .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
        }
        Ok(SyncSessionRecord {
            id,
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            surface: surface.to_string(),
            user_id: user_id.map(str::to_string),
            space_role,
            client_label: client_label.to_string(),
            connected_at,
            disconnected_at: None,
        })
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
                     RETURN s.id AS id, s.spaceId AS spaceId, s.surface AS surface, u.id AS userId, s.spaceRole AS role, s.clientLabel AS clientLabel,
                            s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                     ORDER BY s.connectedAt DESC",
                )
                .param("document_id", document_id),
            )
            .await
            .map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            sessions.push(sync_session_from_row(&row, document_id)?);
        }
        Ok(sessions)
    }

    async fn list_active_sync_sessions(&self, space_id: Option<&str>) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let cypher = "MATCH (s:SyncSession)
                       WHERE s.disconnectedAt IS NULL AND ($space_id IS NULL OR s.spaceId = $space_id)
                       OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                       RETURN s.id AS id, s.spaceId AS spaceId, s.documentId AS documentId, s.surface AS surface, u.id AS userId,
                              s.spaceRole AS role, s.clientLabel AS clientLabel, s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                       ORDER BY s.connectedAt DESC";
        let mut result = self.graph.execute(query(cypher).param("space_id", space_id.map(str::to_string))).await.map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let document_id: String = row.get("documentId").map_err(backend)?;
            sessions.push(sync_session_from_row(&row, &document_id)?);
        }
        Ok(sessions)
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (s:SyncSession) WHERE s.disconnectedAt IS NULL SET s.disconnectedAt = $now").param("now", now_ms())).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region EventLog
    /// @emoji ➕️ Assigns a dense `seq` via a `(:DirectoryCounter {id:'singleton'})` node
    /// incremented in the same transaction as the `(:DirectoryEvent)` node and the projection —
    /// the write's atomicity comes from `Txn`, not from any Neo4j auto-increment primitive (Neo4j
    /// has none).
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::to_value(&event.body).map_err(backend)?;
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
            let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
            txn.run(
                query(
                    "CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id,
                                                spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})",
                )
                .param("seq", seq)
                .param("id", id.clone())
                .param("hlc_physical", event.hlc.physical_ms)
                .param("hlc_logical", event.hlc.logical as i64)
                .param("actor_kind", actor_kind_to_str(event.actor.kind))
                .param("actor_id", event.actor.id.clone())
                .param("space_id", event.space_id.clone())
                .param("user_id", event.user_id.clone())
                .param("kind", kind)
                .param("payload", payload_value.to_string())
                .param("recorded_at", recorded_at_ms),
            )
            .await
            .map_err(backend)?;
            let full = DirectoryEvent { seq: seq as u64, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            self.project(&mut txn, &full).await?;
            persisted.push(full);
        }
        txn.commit().await.map_err(backend)?;
        Ok(persisted)
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut result = self.graph.execute(query("MATCH (e:DirectoryEvent) WHERE e.seq > $since_seq RETURN e AS e ORDER BY e.seq LIMIT $limit").param("since_seq", since_seq as i64).param("limit", limit as i64)).await.map_err(backend)?;
        let mut events = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            events.push(event_from_node(&row)?);
        }
        Ok(events)
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let mut result = self.graph.execute(query("MATCH (c:DirectoryCounter {id: 'singleton'}) RETURN c.seq AS seq")).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(row.get::<i64>("seq").map_err(backend)? as u64),
            None => Ok(0),
        }
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        txn.run(query("MATCH (s:Space) DETACH DELETE s")).await.map_err(backend)?;
        txn.run(query("MATCH (u:User) DETACH DELETE u")).await.map_err(backend)?;
        let mut result = txn.execute(query("MATCH (e:DirectoryEvent) RETURN e AS e ORDER BY e.seq")).await.map_err(backend)?;
        let mut events = Vec::new();
        while let Some(row) = result.next(txn.handle()).await.map_err(backend)? {
            events.push(event_from_node(&row)?);
        }
        let mut replayed = 0u64;
        for event in &events {
            self.project(&mut txn, event).await?;
            replayed += 1;
        }
        txn.commit().await.map_err(backend)?;
        Ok(replayed)
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

fn invite_from_node(row: &neo4rs::Row) -> DirectoryResult<InviteRecord> {
    let node: neo4rs::Node = row.get("i").map_err(backend)?;
    let role: String = node.get("role").map_err(backend)?;
    Ok(InviteRecord {
        id: node.get("id").map_err(backend)?,
        token: node.get("token").map_err(backend)?,
        space_id: node.get("spaceId").map_err(backend)?,
        role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator),
        created_at: node.get("createdAt").map_err(backend)?,
        expires_at: node.get("expiresAt").map_err(backend)?,
        revoked_at: node.get::<i64>("revokedAt").ok(),
    })
}

/// 🧭️ Shared by `list_sync_sessions_for_document` (caller already knows `document_id`) and
/// `list_active_sync_sessions` (reads `documentId` off the row itself, since it spans documents).
fn sync_session_from_row(row: &neo4rs::Row, document_id: &str) -> DirectoryResult<SyncSessionRecord> {
    let role: String = row.get("role").unwrap_or_default();
    Ok(SyncSessionRecord {
        id: row.get("id").map_err(backend)?,
        space_id: row.get("spaceId").map_err(backend)?,
        document_id: document_id.to_string(),
        surface: row.get("surface").unwrap_or_default(),
        user_id: row.get::<String>("userId").ok(),
        space_role: SpaceRole::parse(&role),
        client_label: row.get("clientLabel").map_err(backend)?,
        connected_at: row.get("connectedAt").map_err(backend)?,
        disconnected_at: row.get::<i64>("disconnectedAt").ok(),
    })
}

fn event_from_node(row: &neo4rs::Row) -> DirectoryResult<DirectoryEvent> {
    let node: neo4rs::Node = row.get("e").map_err(backend)?;
    let payload: String = node.get("payload").map_err(backend)?;
    let body: DirectoryEventBody = serde_json::from_str(&payload).map_err(backend)?;
    let actor_kind: String = node.get("actorKind").map_err(backend)?;
    Ok(DirectoryEvent {
        seq: node.get::<i64>("seq").map_err(backend)? as u64,
        id: node.get("id").map_err(backend)?,
        hlc: Hlc { physical_ms: node.get("hlcPhysical").map_err(backend)?, logical: node.get::<i64>("hlcLogical").map_err(backend)? as u32 },
        actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: node.get("actorId").map_err(backend)? },
        space_id: node.get::<String>("spaceId").ok(),
        user_id: node.get::<String>("userId").ok(),
        body,
        recorded_at_ms: node.get("recordedAt").map_err(backend)?,
    })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    // 🔬️ Neo4j has no in-memory test mode; integration tests run against a live/testcontainers
    // instance per the verification plan (HP-3) — not exercised in unit-test CI without a running
    // Neo4j, unlike the sqlite backend's `:memory:` tests.
}
//#endregion 🧪️Tests
