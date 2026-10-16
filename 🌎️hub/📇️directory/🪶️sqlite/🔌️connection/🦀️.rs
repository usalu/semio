//! 🔌️ The backend handle, its constructors and seeding, and the test-only fault hooks.

use super::*;

/// 🗄️ SQLite-backed implementation of every directory capability trait. One `rusqlite::Connection` behind a `Mutex` — see this
/// module's own doc for why this isn't an async SQLite driver.
pub struct SqliteDirectory {
    pub(super) conn: Arc<Mutex<Connection>>,
    #[cfg(test)]
    pub(super) append_commit_ack_failure: Arc<std::sync::atomic::AtomicBool>,
    #[cfg(test)]
    pub(super) genesis_commit_ack_failure: Arc<std::sync::atomic::AtomicU8>,
    #[cfg(test)]
    pub(super) creation_read_failure: Arc<std::sync::atomic::AtomicBool>,
    #[cfg(test)]
    pub(super) event_read_failure: Arc<std::sync::atomic::AtomicBool>,
}

impl SqliteDirectory {
    /// 🔌️ Opens (creating if absent) the SQLite database at `path` and bootstraps the schema.
    /// `path` may be `:memory:` for tests.
    pub async fn connect(path: &str) -> DirectoryResult<Self> {
        let conn = Connection::open(path).map_err(backend)?;
        conn.busy_timeout(std::time::Duration::from_secs(2)).map_err(backend)?;
        conn.execute_batch(SCHEMA).map_err(backend)?;
        let stamp = conn.query_row("SELECT schema, version FROM hub_directory_format WHERE singleton = 1", [], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))).optional().map_err(backend)?;
        let events: i64 = conn.query_row("SELECT COUNT(*) FROM hub_directory_event", [], |row| row.get(0)).map_err(backend)?;
        if admit_directory_format(stamp, events > 0, "SQLite")? == DirectoryFormatAdmission::Stamp {
            conn.execute("INSERT OR IGNORE INTO hub_directory_format (singleton, schema, version) VALUES (1, ?1, ?2)", rusqlite::params![DIRECTORY_FORMAT_SCHEMA, DIRECTORY_FORMAT_VERSION]).map_err(backend)?;
        }
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
            #[cfg(test)]
            append_commit_ack_failure: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            #[cfg(test)]
            genesis_commit_ack_failure: Arc::new(std::sync::atomic::AtomicU8::new(0)),
            #[cfg(test)]
            creation_read_failure: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            #[cfg(test)]
            event_read_failure: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    pub fn lock(&self) -> DirectoryResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|_| DirectoryError::Backend("sqlite connection lock poisoned".into()))
    }

    /// 🧪️ Simulates a lost commit acknowledgement after the real SQLite transaction commits.
    #[cfg(test)]
    pub fn fail_next_append_commit_ack(&self) {
        self.append_commit_ack_failure.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// 🧪️ Drops only the genesis acknowledgement after its entire real transaction commits.
    #[cfg(test)]
    pub fn fail_next_genesis_commit_ack(&self) {
        self.genesis_commit_ack_failure.store(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// 🧪️ Drops the commit acknowledgement and exactly one subsequent receipt or event read.
    #[cfg(test)]
    pub fn fail_next_genesis_reconciliation(&self, events: bool) {
        self.genesis_commit_ack_failure.store(if events { 3 } else { 2 }, std::sync::atomic::Ordering::SeqCst);
    }

    #[cfg(test)]
    pub fn install_invite_projection_failure(&self) -> DirectoryResult<()> {
        self.lock()?
            .execute_batch(
                "CREATE TEMP TRIGGER hub_test_fail_invite_projection BEFORE INSERT ON hub_space_membership
                 WHEN NEW.user_id = 'u-invite-failure'
                 BEGIN SELECT RAISE(ABORT, 'injected invite projection failure'); END;",
            )
            .map_err(backend)
    }

    #[cfg(test)]
    pub fn clear_invite_projection_failure(&self) -> DirectoryResult<()> {
        self.lock()?.execute_batch("DROP TRIGGER hub_test_fail_invite_projection;").map_err(backend)
    }

    #[cfg(test)]
    pub fn install_invite_acceptance_marker_for_test(&self, invite_id: &str, event_id: &str) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_space_invite SET accepted_event_id = ?1 WHERE id = ?2", rusqlite::params![event_id, invite_id]).map(|_| ()).map_err(backend)
    }

    /// 🌱️ Seeds a placeholder `seed` system user and a default `studio`/`private` space it
    /// owns, through the event log (`user.created` + `space.created` + `member.upserted`) like any
    /// other write — the system user satisfies `hub_space.owner_user_id`'s foreign key until a real
    /// bootstrap admin claims ownership through `/admin` (HP-6). Document existence itself is
    /// `db::Database`'s concern (see `bin.rs`), not seeded here.
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: i64 = self.lock()?.query_row("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'", [], |row| row.get(0)).map_err(backend)?;
        if user_exists > 0 {
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
