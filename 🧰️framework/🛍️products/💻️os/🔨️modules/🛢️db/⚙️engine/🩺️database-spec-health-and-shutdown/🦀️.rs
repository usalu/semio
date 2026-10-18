//! 🩺️ Database spec, health and shutdown: the artifact spec, the health report, the shutdown control with its interruption witness, the live guards and census and the shutdown phases and progress. See [`super`].

use super::*;

//#region 🔖️ArtifactSpec
/// 📄️ What `Database::create_document` needs to mint a brand-new document — this crate's own
/// choice (the contract fixes `create_document`'s signature, not `ArtifactSpec`'s shape).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactSpec {
    pub document: protocol::ArtifactId,
}

impl ArtifactSpec {
    pub async fn new(document: protocol::ArtifactId) -> ArtifactSpec {
        ArtifactSpec { document }
    }
}
//#endregion 🔖️ArtifactSpec

//#region 🔖️Health
/// 🩺️ The frozen `Database::health()` return shape, wrapping a real
/// `db_observe::HealthRegistry` snapshot plus this crate's own catalog-level fact (open document
/// count) that no lower crate could know.
#[derive(Clone, Debug)]
pub struct DbHealth {
    pub report: db_observe::HealthReport,
    pub open_artifacts: usize,
}
//#endregion 🔖️Health

/// 🚪️ Shared interruption authority for a retained database shutdown.
pub struct DatabaseShutdownControl {
    pub deadline: std::time::Instant,
    pub cancelled: Arc<std::sync::atomic::AtomicBool>,
}

impl DatabaseShutdownControl {
    pub fn new(deadline: std::time::Instant, cancelled: Arc<std::sync::atomic::AtomicBool>) -> DatabaseShutdownControl {
        DatabaseShutdownControl { deadline, cancelled }
    }

    pub fn for_timeout(timeout: std::time::Duration) -> DatabaseShutdownControl {
        DatabaseShutdownControl::new(std::time::Instant::now() + timeout, Arc::new(std::sync::atomic::AtomicBool::new(false)))
    }

    pub fn interrupted(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire) || std::time::Instant::now() >= self.deadline
    }

    /// 🔦️ The interruption an expired or cancelled shutdown answers with. A deadline names the
    /// phase it expired in and that phase's retained owner counts, because the caller's only other
    /// evidence — the step sequence — is already gone by the time the error reaches it.
    pub(super) fn interruption_error(&self, witness: DatabaseShutdownInterruptionWitness) -> DbError {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            DbError::Closed
        } else {
            DbError::Unavailable(format!(
                "database shutdown deadline elapsed in phase {:?}: open artifacts {}, emit started {}, retained pool-use owners {}, retained slots {:?}",
                witness.phase, witness.open_artifacts, witness.emit_started, witness.pool_use_owners, witness.retained_slots
            ))
        }
    }
}

/// 🔎️ What one interrupted shutdown had reached when its deadline expired.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DatabaseShutdownInterruptionWitness {
    pub phase: Option<DatabaseShutdownPhase>,
    pub(super) open_artifacts: usize,
    pub(super) emit_started: bool,
    pub(super) pool_use_owners: usize,
    pub(super) retained_slots: DatabaseRetainedPoolUseCensus,
}

/// 🔬️ How many LIVE owners of each retained-activity family still hold a `WorkerPoolUse` clone.
/// Every slot registry in this crate is a TERMINAL table — a state registers its slot only when it
/// is orphaned — so a census built from those tables reads 0 for a family whose owner is simply
/// still alive. These four counters are incremented where the state that stores the clone is built
/// and decremented in its `Drop`, so they see the live owner the tables cannot.
static DATABASE_MOUNT_FUTURE_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ One live document-mount future. A mount future OWNS a `WorkerPoolUse` clone from the moment
/// it is built, and the clone lives exactly as long as the future — whoever holds it. Every other
/// census family counts a struct field; this one counts the futures themselves, which is the only
/// carrier left once every field reads 0.
pub(super) struct DatabaseMountFutureLiveGuardV1;

impl DatabaseMountFutureLiveGuardV1 {
    pub fn new() -> Self {
        DATABASE_MOUNT_FUTURE_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Self
    }
}

impl Drop for DatabaseMountFutureLiveGuardV1 {
    fn drop(&mut self) {
        DATABASE_MOUNT_FUTURE_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

pub(super) static DATABASE_POOL_USE_MOUNT_WAIT_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🔬️ One live BARE `require_open_use` clone that never reaches a registered owner. `mount_document`
/// clones the Database's use into a plain local and holds it across its awaits while the mount waiter
/// parks on another caller's `Opening` slot. That clone is in no slot table, in no `live_*` struct family
/// and in no mount future, so it is invisible to every other census family while it pins the use.
/// `require_open_use` has exactly three call sites; the other two hand their clone to a counted future, so
/// this guard closes the census over every carrier of `Database::pool_use`.
pub(super) struct DatabasePoolUseSiteGuardV1(pub(super) &'static std::sync::atomic::AtomicUsize);

impl DatabasePoolUseSiteGuardV1 {
    pub fn new(site: &'static std::sync::atomic::AtomicUsize) -> Self {
        site.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Self(site)
    }
}

impl Drop for DatabasePoolUseSiteGuardV1 {
    fn drop(&mut self) {
        self.0.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}

/// 🔬️ How many retained owners of each family still hold a `WorkerPoolUse` clone of the
/// `Database`'s use, because `Arc::strong_count` names a number and never an owner. The first six
/// are process-global slot tables; `runner_handoff` is the family that has no table at all — a
/// runner handoff parks its clone inside the runner until the terminal transition takes it, so an
/// authority already absent from `open_artifacts` can still pin the use through one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DatabaseRetainedPoolUseCensus {
    capability_open: usize,
    catalog_read: usize,
    catalog_bootstrap: usize,
    create_catalog: usize,
    sync_hello: usize,
    artifact_retirement: usize,
    runner_handoff: usize,
    mount_work: usize,
    live_authority: usize,
    live_capability_open: usize,
    live_catalog_read: usize,
    live_catalog_bootstrap: usize,
    live_create_catalog: usize,
    live_mount_future: usize,
    live_sync_hello: usize,
    mount_wait: usize,
}

impl DatabaseRetainedPoolUseCensus {
    pub fn observe() -> Self {
        Self {
            capability_open: database_capability_open_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            catalog_read: database_catalog_read_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            catalog_bootstrap: database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            create_catalog: database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().filter(|slot| slot.is_some()).count(),
            sync_hello: db_sync::database_sync_hello_live_slots(),
            artifact_retirement: db_artifact::artifact_runner_retirement_live_slots(),
            runner_handoff: db_artifact::artifact_runner_handoff_pool_use_live_slots(),
            mount_work: database_document_mount_work_live_slots(),
            live_authority: db_artifact::artifact_authority_live_pool_uses(),
            live_capability_open: DATABASE_CAPABILITY_OPEN_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_catalog_read: DATABASE_CATALOG_READ_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_catalog_bootstrap: DATABASE_CATALOG_BOOTSTRAP_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_create_catalog: DATABASE_CREATE_CATALOG_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_mount_future: DATABASE_MOUNT_FUTURE_LIVE.load(std::sync::atomic::Ordering::Acquire),
            live_sync_hello: db_sync::database_sync_hello_live_states(),
            mount_wait: DATABASE_POOL_USE_MOUNT_WAIT_LIVE.load(std::sync::atomic::Ordering::Acquire),
        }
    }
}

/// 🧭️ The retained owner advanced by one database shutdown step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabaseShutdownPhase {
    Authority,
    Emit,
    PoolUse,
}

/// 🧱️ A live shared owner that prevents destructive shutdown progress.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabaseShutdownBlock {
    Authorities(usize),
    Executor(semio_framework_async::WorkerSubmitErrorKind),
    ArtifactClose(db_artifact::ArtifactCloseRetryProgress),
}

/// 🚦️ One bounded retained database shutdown result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatabaseShutdownProgress {
    Progress { phase: DatabaseShutdownPhase, remaining_authorities: usize },
    Blocked(DatabaseShutdownBlock),
    Interrupted,
    Complete,
}
