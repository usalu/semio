//! 🔬️ The proctor's SQLite stores held to the server product's storage conformance laws — the same
//! assertion bodies the framework's in-memory reference backends are held to. Every law runs twice:
//! against a durable database in a fresh directory and against a private in-memory database. The
//! failing-sink laws seed a durable store, then reopen the file read-only, which is how a sink that
//! stopped accepting writes looks to a SQLite store.
//!
//! @see ../../../../🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️conformance/🦀️.rs — the laws

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use proctor::storage::{Database, SqliteAuthorityStore, SqliteBlobStore, SqliteProjectionStore, SqliteSessionStore};
use server::conformance;

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn scratch(label: &str) -> Scratch {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("teaching-proctor-conformance-{label}-{}-{nanos}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
    std::fs::create_dir_all(&path).expect("scratch directory");
    Scratch(path)
}

fn databases(label: &str) -> Vec<(Option<Scratch>, Database)> {
    let directory = scratch(label);
    let durable = Database::open(&directory.0).expect("durable database");
    vec![(Some(directory), durable), (None, Database::memory().expect("memory database"))]
}

macro_rules! laws {
    ($($law:ident => $store:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $law() {
                for (_directory, database) in databases(stringify!($law)) {
                    conformance::$law(&mut $store::new(database)).await;
                }
            }
        )*
    };
}

laws! {
    receipt_round_trips_and_is_idempotent => SqliteAuthorityStore,
    append_events_rejects_a_sequence_gap_and_writes_nothing => SqliteAuthorityStore,
    events_since_returns_only_later_events_of_that_actor => SqliteAuthorityStore,
    snapshots_only_move_forward => SqliteAuthorityStore,
    outbox_delivers_each_entry_exactly_once => SqliteAuthorityStore,
    lease_epoch_bump_fences_out_the_previous_holder => SqliteAuthorityStore,
    projection_list_is_prefix_scoped_and_key_ordered => SqliteProjectionStore,
    clearing_a_projection_resets_it_for_rebuild => SqliteProjectionStore,
    blob_put_get_and_has_are_content_addressed => SqliteBlobStore,
    revoke_principal_removes_every_session_of_that_principal => SqliteSessionStore,
}

#[tokio::test]
async fn a_projection_write_reports_a_failing_sink() {
    let directory = scratch("projection-sink");
    conformance::seed_projection_fault_fixture(&mut SqliteProjectionStore::new(Database::open(&directory.0).expect("seed"))).await;
    conformance::a_projection_write_reports_a_failing_sink(&mut SqliteProjectionStore::new(Database::open_read_only(&directory.0).expect("read-only"))).await;
}

#[tokio::test]
async fn a_session_write_reports_a_failing_sink() {
    let directory = scratch("session-sink");
    conformance::seed_session_fault_fixture(&mut SqliteSessionStore::new(Database::open(&directory.0).expect("seed"))).await;
    conformance::a_session_write_reports_a_failing_sink(&mut SqliteSessionStore::new(Database::open_read_only(&directory.0).expect("read-only"))).await;
}

#[tokio::test]
async fn a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart() {
    let directory = scratch("saga-restart");
    let path = directory.0.clone();
    let mut before = SqliteAuthorityStore::new(Database::open(&directory.0).expect("before"));
    conformance::a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart(&mut before, || async move { SqliteAuthorityStore::new(Database::open(&path).expect("reopened")) }).await;
}

#[tokio::test]
async fn an_ephemeral_saga_queue_is_not_resurrected() {
    let mut before = SqliteAuthorityStore::new(Database::memory().expect("memory"));
    conformance::a_saga_reacts_to_a_committed_event_exactly_once_across_a_restart(&mut before, || async { SqliteAuthorityStore::new(Database::memory().expect("fresh memory")) }).await;
}
