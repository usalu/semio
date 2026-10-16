//! 🔌️ The backend handle, its constructors and seeding, and the test-only fault hooks.

use super::*;

/// 🐘️ PostgreSQL-backed implementation of every directory capability trait, pooled via `PgPool`.
pub struct PostgresDirectory {
    pub pool: PgPool,
    #[cfg(test)]
    pub genesis_test_control: std::sync::Arc<ArtifactGenesisTestControlV1>,
}

#[cfg(test)]
struct ArtifactGenesisTestControlV1 {
    pause_before_authority: std::sync::atomic::AtomicBool,
    reached_before_authority: tokio::sync::Semaphore,
    resume_before_authority: tokio::sync::Semaphore,
    observed_now_ms: std::sync::atomic::AtomicU64,
    pub(super) fail_commit_ack: std::sync::atomic::AtomicBool,
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

impl PostgresDirectory {
    /// 🔌️ Connects to `database_url` and bootstraps the schema (idempotent, no migration framework).
    pub async fn connect(database_url: &str) -> DirectoryResult<Self> {
        let pool = PgPoolOptions::new().max_connections(20).connect(database_url).await.map_err(backend)?;
        for statement in SCHEMA.split(';').map(str::trim).filter(|s| !s.is_empty() && !s.starts_with("--")) {
            sqlx_core::query::query(statement).execute(&pool).await.map_err(backend)?;
        }
        let stamp: Option<(String, i64)> = sqlx_core::query_as::query_as("SELECT schema, version FROM hub_directory_format WHERE singleton").fetch_optional(&pool).await.map_err(backend)?;
        let (events,): (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_directory_event").fetch_one(&pool).await.map_err(backend)?;
        if admit_directory_format(stamp, events > 0, "PostgreSQL")? == DirectoryFormatAdmission::Stamp {
            sqlx_core::query::query("INSERT INTO hub_directory_format (singleton, schema, version) VALUES (TRUE, $1, $2) ON CONFLICT(singleton) DO NOTHING")
                .bind(DIRECTORY_FORMAT_SCHEMA)
                .bind(DIRECTORY_FORMAT_VERSION)
                .execute(&pool)
                .await
                .map_err(backend)?;
        }
        let mut identity = Sha256::new();
        identity.update(b"semio.hub.artifact-cas.barrier-identity.v1\0");
        identity.update(time_ordered_id().as_bytes());
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_barrier_identity(singleton, coordinator_id) VALUES (TRUE, $1) ON CONFLICT(singleton) DO NOTHING").bind(identity.finalize().as_slice()).execute(&pool).await.map_err(backend)?;
        Ok(Self {
            pool,
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

    /// 🌱️ Seeds a placeholder `seed` system user and a default `studio`/`private` space it
    /// owns, through the event log (`user.created` + `space.created` + `member.upserted`) like any
    /// other write. The system user satisfies `hub_space.owner_user_id`'s foreign key until a real
    /// bootstrap admin claims ownership through `/admin` (HP-6).
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'").fetch_one(&self.pool).await.map_err(backend)?;
        if user_exists.0 > 0 {
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
