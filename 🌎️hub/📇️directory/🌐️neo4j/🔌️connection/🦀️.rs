//! 🔌️ The backend handle, its constructors and seeding, and the test-only fault hooks.

use super::*;

/// 🕸️ Neo4j-backed implementation of every directory capability trait.
pub struct Neo4jDirectory {
    pub graph: Graph,
    #[cfg(test)]
    pub genesis_test_control: std::sync::Arc<ArtifactGenesisTestControlV1>,
}

#[cfg(test)]
struct ArtifactGenesisTestControlV1 {
    pub pause_before_authority: std::sync::atomic::AtomicBool,
    pub reached_before_authority: tokio::sync::Semaphore,
    pub resume_before_authority: tokio::sync::Semaphore,
    pub observed_now_ms: std::sync::atomic::AtomicU64,
    pub fail_commit_ack: std::sync::atomic::AtomicBool,
}

/// 🚦️ `tokio::sync::Semaphore` has no `Default`, and the two rendezvous gates start closed on
/// purpose: a genesis run only proceeds once the test explicitly adds a permit.
#[cfg(test)]
impl Default for ArtifactGenesisTestControlV1 {
    fn default() -> Self {
        Self {
            pause_before_authority: std::sync::atomic::AtomicBool::new(false),
            reached_before_authority: tokio::sync::Semaphore::new(0),
            resume_before_authority: tokio::sync::Semaphore::new(0),
            observed_now_ms: std::sync::atomic::AtomicU64::new(0),
            fail_commit_ack: std::sync::atomic::AtomicBool::new(false),
        }
    }
}

impl Neo4jDirectory {
    /// 🔌️ Connects to `uri` with `user`/`password` and bootstraps uniqueness constraints.
    pub async fn connect(uri: &str, user: &str, password: &str) -> DirectoryResult<Self> {
        let graph = Graph::new(uri, user, password).await.map_err(backend)?;
        for statement in CONSTRAINTS {
            graph.run(query(statement)).await.map_err(backend)?;
        }
        let mut stamp_result = graph.execute(query("MATCH (f:DirectoryFormat {id: 'singleton'}) RETURN f.schema AS schema, f.version AS version")).await.map_err(backend)?;
        let stamp = match stamp_result.next().await.map_err(backend)? {
            Some(row) => Some((row.get::<String>("schema").map_err(backend)?, row.get::<i64>("version").map_err(backend)?)),
            None => None,
        };
        drop(stamp_result);
        let mut event_result = graph.execute(query("MATCH (e:DirectoryEvent) RETURN count(e) AS count")).await.map_err(backend)?;
        let events: i64 = event_result.next().await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory event count returned no row".into()))?.get("count").map_err(backend)?;
        drop(event_result);
        if admit_directory_format(stamp, events > 0, "Neo4j")? == DirectoryFormatAdmission::Stamp {
            graph
                .run(query("MERGE (f:DirectoryFormat {id: 'singleton'}) ON CREATE SET f.schema = $schema, f.version = $version").param("schema", DIRECTORY_FORMAT_SCHEMA).param("version", DIRECTORY_FORMAT_VERSION))
                .await
                .map_err(backend)?;
        }
        let mut identity = Sha256::new();
        identity.update(b"semio.hub.artifact-cas.barrier-identity.v1\0");
        identity.update(time_ordered_id().as_bytes());
        graph.run(query("MERGE (b:ArtifactCasBarrierIdentity {id: 'singleton'}) ON CREATE SET b.coordinatorId = $coordinator").param("coordinator", identity.finalize().to_vec())).await.map_err(backend)?;
        Ok(Self {
            graph,
            #[cfg(test)]
            genesis_test_control: std::sync::Arc::new(ArtifactGenesisTestControlV1::default()),
        })
    }

    #[cfg(test)]
    pub(super) async fn pause_genesis_before_authority_for_test(&self) -> Option<u64> {
        if self.genesis_test_control.pause_before_authority.swap(false, std::sync::atomic::Ordering::SeqCst) {
            self.genesis_test_control.reached_before_authority.add_permits(1);
            self.genesis_test_control.resume_before_authority.acquire().await.expect("genesis test resume semaphore").forget();
        }
        let observed_now_ms = self.genesis_test_control.observed_now_ms.load(std::sync::atomic::Ordering::SeqCst);
        (observed_now_ms != 0).then_some(observed_now_ms)
    }

    /// 🌱️ Seeds a default `studio`/`private` space authored by a `seed` system user node,
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
}
