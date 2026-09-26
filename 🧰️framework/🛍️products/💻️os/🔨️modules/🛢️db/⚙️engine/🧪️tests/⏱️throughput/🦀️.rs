//! ⏱️ Write and reopen throughput laws (`🧫️fixtures/⏱️throughput`, schema `ThroughputV1` in
//! `⚙️engine/🧬️schema`): what one durable commit and one reopened greeting cost in typed DB I/O (the
//! census: exact, load-free), commit acknowledgements relative to the backend's own durable append
//! measured in the same moment by the same number of writers, and a reopen storm relative to the same
//! greetings made one after another on a freshly opened database (both under the same machine load),
//! on every backend. Absolute targets are printed next to each verdict.
use super::*;
use db_storage::WalStorage as _;

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/⏱️throughput/🔣️.json")).unwrap()
}

fn fixture_u64(value: &serde_json::Value, path: &[&str]) -> u64 {
    path.iter().fold(value, |node, key| &node[*key]).as_u64().unwrap_or_else(|| panic!("throughput fixture {path:?}"))
}

fn fixture_f64(value: &serde_json::Value, path: &[&str]) -> f64 {
    path.iter().fold(value, |node, key| &node[*key]).as_f64().unwrap_or_else(|| panic!("throughput fixture {path:?}"))
}

/// 🎯️ Where a throughput law runs; the same target reopens the same storage.
#[derive(Clone)]
enum ThroughputTarget {
    Fs { root: std::path::PathBuf },
    #[cfg(feature = "sqlite")]
    Sqlite { path: std::path::PathBuf },
    #[cfg(feature = "postgres")]
    Postgres { url: String },
    #[cfg(feature = "neo4j")]
    Neo4j { uri: String, user: String, password: String },
}

impl ThroughputTarget {
    fn label(&self) -> &'static str {
        match self {
            Self::Fs { .. } => "fs",
            #[cfg(feature = "sqlite")]
            Self::Sqlite { .. } => "sqlite",
            #[cfg(feature = "postgres")]
            Self::Postgres { .. } => "postgres",
            #[cfg(feature = "neo4j")]
            Self::Neo4j { .. } => "neo4j",
        }
    }

    async fn open(&self, pool: Arc<WorkerPool>) -> Database {
        match self {
            Self::Fs { root } => Database::open_at(pool, root, Profile::Dev).await.unwrap(),
            #[cfg(feature = "sqlite")]
            Self::Sqlite { path } => {
                let storage = crate::db_storage_sqlite::SqliteStorage::open(pool.clone(), path).await.unwrap();
                Database::open(pool, DbConfig::for_profile(Profile::Dev), Arc::new(db_storage::DbBackend::Sqlite(storage))).await.unwrap()
            }
            #[cfg(feature = "postgres")]
            Self::Postgres { url } => {
                let storage = crate::db_storage_postgres::PostgresStorage::connect(pool.clone(), url).await.unwrap();
                Database::open(pool, DbConfig::for_profile(Profile::Dev), Arc::new(db_storage::DbBackend::Postgres(storage))).await.unwrap()
            }
            #[cfg(feature = "neo4j")]
            Self::Neo4j { uri, user, password } => {
                let storage = crate::db_storage_neo4j::Neo4jStorage::connect(pool.clone(), uri, user, password).await.unwrap();
                Database::open(pool, DbConfig::for_profile(Profile::Dev), Arc::new(db_storage::DbBackend::Neo4j(storage))).await.unwrap()
            }
        }
    }
}

fn opaque_edit(document: &protocol::ArtifactId, mutation_id: String, previous: Option<&protocol::MutationId>, payload_bytes: usize) -> protocol::MutationEnvelope {
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(mutation_id),
        document_id: document.clone(),
        actor: protocol::ActorId("throughput-author".to_string()),
        dependencies: previous.into_iter().cloned().collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("throughput.v1".to_string()), payload: vec![0x5a; payload_bytes] },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("throughput.v1".to_string()), payload: Vec::new() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

/// ✍️ Grows `document` by `batches` durable commits of `batch_edits` chained edits; answers each
/// commit's acknowledgement latency.
async fn grow_document(handle: &ArtifactHandle, document: &protocol::ArtifactId, batches: usize, batch_edits: usize, payload_bytes: usize) -> Vec<std::time::Duration> {
    let mut previous: Option<protocol::MutationId> = None;
    let mut acks = Vec::with_capacity(batches);
    for batch in 0..batches {
        let envelopes: Vec<_> = (0..batch_edits)
            .map(|edit| {
                let envelope = opaque_edit(document, format!("{}-{batch}-{edit}", document.0), previous.as_ref(), payload_bytes);
                previous = Some(envelope.mutation_id.clone());
                envelope
            })
            .collect();
        let asked = std::time::Instant::now();
        handle.submit(db_artifact::CommandBatch::new(envelopes).await.unwrap(), db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.unwrap().unwrap();
        acks.push(asked.elapsed());
    }
    acks
}

/// 🧱️ The storage floor of the same workload: `batches` raw WAL appends of one batch's bytes, each
/// synced `Fsync`, on a scratch document of the same backend — no engine, no index, no actor.
async fn durable_appends(database: &Database, document: &str, batches: usize, batch_bytes: usize) -> Vec<std::time::Duration> {
    let storage = database.storage().await;
    let wal = storage.wal().await;
    let document = ArtifactId(document.to_string());
    let writer = wal.acquire_writer(&document).await.unwrap();
    wal.create_segment(&writer, 0).await.unwrap();
    let bytes = vec![0x5a; batch_bytes];
    let mut appends = Vec::with_capacity(batches);
    for _ in 0..batches {
        let asked = std::time::Instant::now();
        wal.append(&writer, 0, db_storage::db_io_copy_pages(&bytes).unwrap().await.unwrap()).await.unwrap();
        wal.sync(&writer, 0, DurabilityClass::Fsync).await.unwrap();
        appends.push(asked.elapsed());
    }
    writer.release().await.unwrap();
    appends
}

/// 🤝️ One greeting of a storm: time to the mounted handle, time to the `Welcome` (both from the
/// ask) and the drained bootstrap frame count.
struct Greeting {
    mounted: std::time::Duration,
    welcomed: std::time::Duration,
    frames: usize,
}

/// 🤝️ Mounts `document` as the hub's document socket does and greets it as a fresh replica.
async fn greet(database: &Database, document: &protocol::ArtifactId, session: String) -> Greeting {
    let asked = std::time::Instant::now();
    let handle = database.ensure_document(document).await.unwrap();
    let mounted = asked.elapsed();
    let mut greeting = database.hello(document.clone(), None, session, protocol::ActorId("semio_hub".to_string()), 64 * 1024).await.unwrap();
    greeting.take_welcome().unwrap().acknowledge().unwrap();
    let welcomed = asked.elapsed();
    let mut frames = 0;
    while let Some(frame) = greeting.next_frame().await.unwrap() {
        frame.acknowledge().unwrap();
        frames += 1;
    }
    drop(greeting);
    drop(handle);
    Greeting { mounted, welcomed, frames }
}

fn percentile(samples: &[std::time::Duration], fraction: f64) -> std::time::Duration {
    let mut sorted = samples.to_vec();
    sorted.sort();
    sorted[((sorted.len() as f64 - 1.0) * fraction).round() as usize]
}

fn ms(duration: std::time::Duration) -> f64 {
    duration.as_secs_f64() * 1_000.0
}

fn on_threads<T: Send>(count: usize, work: impl Fn(usize) -> T + Sync) -> Vec<T> {
    std::thread::scope(|scope| {
        let work = &work;
        let threads: Vec<_> = (0..count).map(|index| scope.spawn(move || work(index))).collect();
        threads.into_iter().map(|thread| thread.join().unwrap()).collect()
    })
}

/// ⚖️ Runs the fixture's cost model and storm on `target` and holds every bound.
async fn throughput_law(target: ThroughputTarget) {
    let fixture = fixture();
    let workers = fixture_u64(&fixture, &["workers"]) as usize;
    let payload_bytes = fixture_u64(&fixture, &["payloadBytes"]) as usize;
    let cost_batches = fixture_u64(&fixture, &["costModel", "batches"]) as usize;
    let cost_edits = fixture_u64(&fixture, &["costModel", "batchEdits"]) as usize;
    let documents = fixture_u64(&fixture, &["storm", "documents"]) as usize;
    let batches = fixture_u64(&fixture, &["storm", "batches"]) as usize;
    let batch_edits = fixture_u64(&fixture, &["storm", "batchEdits"]) as usize;
    let label = target.label();
    let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, workers)));

    let mut database = target.open(pool.clone()).await;
    let cost_document = protocol::ArtifactId("throughput-cost".to_string());
    let cost_handle = database.create_document(ArtifactSpec::new(cost_document.clone()).await).await.unwrap();
    let mut commits = Vec::with_capacity(cost_batches);
    let mut previous: Option<protocol::MutationId> = None;
    for batch in 0..cost_batches {
        let envelopes: Vec<_> = (0..cost_edits)
            .map(|edit| {
                let envelope = opaque_edit(&cost_document, format!("cost-{batch}-{edit}"), previous.as_ref(), payload_bytes);
                previous = Some(envelope.mutation_id.clone());
                envelope
            })
            .collect();
        let before = db_storage::db_io_census();
        cost_handle.submit(db_artifact::CommandBatch::new(envelopes).await.unwrap(), db_artifact::SubmitOptions { durability: DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.unwrap().unwrap();
        commits.push(db_storage::db_io_census().since(&before));
    }
    drop(cost_handle);
    for (index, commit) in commits.iter().enumerate() {
        eprintln!("throughput[{label}] commit {index}: tasks {} steps {} turns {} :: {}", commit.total_tasks(), commit.total_steps(), commit.total_turns(), commit.describe());
    }

    let storm_documents: Vec<protocol::ArtifactId> = (0..documents).map(|index| protocol::ArtifactId(format!("throughput-storm-{index}"))).collect();
    let batch_bytes = batch_edits * (payload_bytes + 96) * 2;
    let floor: Vec<std::time::Duration> = on_threads(documents, |index| db_actor::block_on(durable_appends(&database, &format!("throughput-floor-{index}"), batches, batch_bytes))).into_iter().flatten().collect();
    let mut storm_handles = Vec::with_capacity(documents);
    for document in &storm_documents {
        storm_handles.push(database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap());
    }
    let grown_at = std::time::Instant::now();
    let acks: Vec<Vec<std::time::Duration>> = on_threads(documents, |index| db_actor::block_on(grow_document(&storm_handles[index], &storm_documents[index], batches, batch_edits, payload_bytes)));
    let grown = grown_at.elapsed();
    drop(storm_handles);
    let all_acks: Vec<std::time::Duration> = acks.iter().flatten().copied().collect();
    let early: Vec<std::time::Duration> = acks.iter().flat_map(|document| document[..batches / 2].iter().copied()).collect();
    let late: Vec<std::time::Duration> = acks.iter().flat_map(|document| document[batches / 2..].iter().copied()).collect();
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(120))).await.unwrap();
    drop(database);

    let database = target.open(pool.clone()).await;
    let before = db_storage::db_io_census();
    let mount_started = std::time::Instant::now();
    let cost_handle = database.ensure_document(&cost_document).await.unwrap();
    let mount = mount_started.elapsed();
    let mounted = db_storage::db_io_census().since(&before);
    let before = db_storage::db_io_census();
    let hello_started = std::time::Instant::now();
    let mut greeting = database.hello(cost_document.clone(), None, "throughput-solo".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024).await.unwrap();
    greeting.take_welcome().unwrap().acknowledge().unwrap();
    let hello = hello_started.elapsed();
    while let Some(frame) = greeting.next_frame().await.unwrap() {
        frame.acknowledge().unwrap();
    }
    let greeted = db_storage::db_io_census().since(&before);
    drop(greeting);
    drop(cost_handle);
    let solo = mount + hello;
    let serial_census = db_storage::db_io_census();
    let serial_at = std::time::Instant::now();
    for (index, document) in storm_documents.iter().enumerate() {
        greet(&database, document, format!("throughput-serial-session-{index}")).await;
    }
    let serial = serial_at.elapsed();
    let serialized = db_storage::db_io_census().since(&serial_census);
    let mut database = database;
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(120))).await.unwrap();
    drop(database);

    let database = Arc::new(target.open(pool.clone()).await);
    eprintln!("throughput[{label}] reopen storm of {documents} documents begins");
    let storm_at = std::time::Instant::now();
    let storm_census = db_storage::db_io_census();
    let greetings: Vec<Greeting> = on_threads(documents, |index| db_actor::block_on(greet(&database, &storm_documents[index], format!("throughput-storm-session-{index}"))));
    let storm = storm_at.elapsed();
    let stormed = db_storage::db_io_census().since(&storm_census);
    let welcome_times: Vec<std::time::Duration> = greetings.iter().map(|greeting| greeting.welcomed).collect();
    let mount_times: Vec<std::time::Duration> = greetings.iter().map(|greeting| greeting.mounted).collect();
    let bootstrap_frames: usize = greetings.iter().map(|greeting| greeting.frames).sum();
    let mut database = Arc::try_unwrap(database).ok().expect("the storm released every database handle");
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(120))).await.unwrap();

    let ack_p50 = percentile(&all_acks, 0.5);
    let ack_p95 = percentile(&all_acks, 0.95);
    let floor_p50 = percentile(&floor, 0.5);
    let early_p50 = percentile(&early, 0.5);
    let late_p50 = percentile(&late, 0.5);
    let welcome_p50 = percentile(&welcome_times, 0.5);
    let welcome_max = percentile(&welcome_times, 1.0);
    let targets = &fixture["targets"];
    eprintln!(
        "throughput[{label}] grow {documents}x{batches}x{batch_edits} in {:.0} ms: ack p50 {:.1} ms (target {}) p95 {:.1} ms (target {}) max {:.1} ms; durable append p50 {:.1} ms; early p50 {:.1} ms late p50 {:.1} ms",
        ms(grown),
        ms(ack_p50),
        targets["ackP50Ms"],
        ms(ack_p95),
        targets["ackP95Ms"],
        ms(percentile(&all_acks, 1.0)),
        ms(floor_p50),
        ms(early_p50),
        ms(late_p50)
    );
    eprintln!(
        "throughput[{label}] solo reopen: mount {:.1} ms ({} tasks, {} turns :: {}) hello {:.1} ms ({} tasks, {} turns :: {})",
        ms(mount),
        mounted.total_tasks(),
        mounted.total_turns(),
        mounted.describe(),
        ms(hello),
        greeted.total_tasks(),
        greeted.total_turns(),
        greeted.describe()
    );
    eprintln!(
        "throughput[{label}] storm of {documents} in {:.0} ms: welcome p50 {:.1} ms (target {}) p90 {:.1} ms max {:.1} ms (target {}); mounted p50 {:.1} ms max {:.1} ms; {bootstrap_frames} frames; {} tasks, {} turns :: {}; solo {:.1} ms",
        ms(storm),
        ms(welcome_p50),
        targets["welcomeP50Ms"],
        ms(percentile(&welcome_times, 0.9)),
        ms(welcome_max),
        targets["welcomeMaxMs"],
        ms(percentile(&mount_times, 0.5)),
        ms(percentile(&mount_times, 1.0)),
        stormed.total_tasks(),
        stormed.total_turns(),
        stormed.describe(),
        ms(solo)
    );
    eprintln!("throughput[{label}] the same {documents} greeted one after another in {:.0} ms ({} tasks, {} turns): storm/serial {:.2}", ms(serial), serialized.total_tasks(), serialized.total_turns(), ms(storm) / ms(serial));

    let bounds = &fixture["bounds"];
    let steady = &commits[1..];
    let (first_half, second_half) = steady.split_at(steady.len() / 2);
    let half_tasks = |half: &[db_storage::DbIoCensusV1]| half.iter().map(db_storage::DbIoCensusV1::total_tasks).sum::<u64>();
    assert!(half_tasks(second_half) <= half_tasks(first_half), "[{label}] later commits cost {} tasks, earlier ones {}: per-commit I/O grows with history", half_tasks(second_half), half_tasks(first_half));
    let mean_tasks = half_tasks(steady) as f64 / steady.len() as f64;
    assert!(mean_tasks <= fixture_f64(bounds, &["commitTasksMeanMax"]), "[{label}] a commit costs {mean_tasks:.1} DB I/O tasks on average");
    for (index, commit) in steady.iter().enumerate() {
        assert!(commit.total_tasks() <= fixture_u64(bounds, &["commitTasksMax"]), "[{label}] commit {} costs {} tasks: {}", index + 1, commit.total_tasks(), commit.describe());
        assert!(commit.tasks(db_storage::DbIoTaskKind::IndexRead) <= fixture_u64(bounds, &["commitIndexReadsMax"]), "[{label}] commit {} reads {} index runs", index + 1, commit.tasks(db_storage::DbIoTaskKind::IndexRead));
        assert!(commit.total_turns() <= commit.total_tasks() * fixture_u64(bounds, &["commitTurnsPerTaskMax"]), "[{label}] commit {} spends {} worker turns on {} tasks", index + 1, commit.total_turns(), commit.total_tasks());
    }
    assert!(greeted.total_tasks() <= fixture_u64(bounds, &["helloTasksMax"]), "[{label}] a greeting of a mounted document costs {} DB I/O tasks: {}", greeted.total_tasks(), greeted.describe());
    assert!(ms(late_p50) <= ms(early_p50) * fixture_f64(bounds, &["ackLateToEarlyRatioMax"]), "[{label}] late commits ack p50 {:.1} ms vs early {:.1} ms: commit cost grows with history", ms(late_p50), ms(early_p50));
    assert!(ms(ack_p50) <= ms(floor_p50) * fixture_f64(bounds, &["ackToDurableAppendRatioMax"]), "[{label}] ack p50 {:.1} ms vs the backend's own durable append p50 {:.1} ms", ms(ack_p50), ms(floor_p50));
    assert!(stormed.total_tasks() as f64 <= serialized.total_tasks() as f64 * fixture_f64(bounds, &["stormToSerialTasksRatioMax"]), "[{label}] the storm cost {} DB I/O tasks, the same greetings one after another {}: {}", stormed.total_tasks(), serialized.total_tasks(), stormed.describe());
    assert!(ms(storm) <= ms(serial) * fixture_f64(bounds, &["stormToSerialRatioMax"]), "[{label}] {documents} greetings at once took {:.0} ms, one after another {:.0} ms: the storm serializes", ms(storm), ms(serial));
    assert!(ms(welcome_max) <= fixture_f64(targets, &["welcomeMaxMs"]), "[{label}] slowest storm welcome {:.1} ms", ms(welcome_max));
}

#[semio_framework_async_macros::async_test]
async fn fs_commits_and_reopen_storms_stay_within_their_throughput_bounds() {
    if !db_storage::process_isolated_law("db_engine::throughput_tests::fs_commits_and_reopen_storms_stay_within_their_throughput_bounds") {
        return;
    }
    let root = std::env::temp_dir().join(format!("db-throughput-fs-{}-{}", std::process::id(), now_ms().await));
    throughput_law(ThroughputTarget::Fs { root: root.clone() }).await;
    let _ = std::fs::remove_dir_all(root);
}

#[cfg(feature = "sqlite")]
#[semio_framework_async_macros::async_test]
async fn sqlite_commits_and_reopen_storms_stay_within_their_throughput_bounds() {
    if !db_storage::process_isolated_law("db_engine::throughput_tests::sqlite_commits_and_reopen_storms_stay_within_their_throughput_bounds") {
        return;
    }
    let root = std::env::temp_dir().join(format!("db-throughput-sqlite-{}-{}", std::process::id(), now_ms().await));
    std::fs::create_dir_all(&root).unwrap();
    throughput_law(ThroughputTarget::Sqlite { path: root.join("throughput.sqlite3") }).await;
    let _ = std::fs::remove_dir_all(root);
}

/// 🗝️ The claimed shared server a live throughput law runs against, from the environment the hub itself reads
/// (`OS_HUB_DATABASE_URL`, `OS_HUB_NEO4J_URI`/`_USER`/`_PASSWORD`): set by `os-hub-ts backend run <postgres|neo4j> -- …`,
/// which claims the ONE shared development server (a run database of its own on postgres, the exclusive lease over a
/// reset graph on neo4j) and releases it afterwards.
#[cfg(any(feature = "postgres", feature = "neo4j"))]
fn claimed_backend_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} is unset: run this law under `os-hub-ts backend run <postgres|neo4j> -- …`"))
}

#[cfg(feature = "postgres")]
#[semio_framework_async_macros::async_test]
#[ignore = "needs the claimed shared postgres: `os-hub-ts backend run postgres -- …`"]
async fn postgres_commits_and_reopen_storms_stay_within_their_throughput_bounds() {
    if !db_storage::process_isolated_law("db_engine::throughput_tests::postgres_commits_and_reopen_storms_stay_within_their_throughput_bounds") {
        return;
    }
    throughput_law(ThroughputTarget::Postgres { url: claimed_backend_env("OS_HUB_DATABASE_URL") }).await;
}

#[cfg(feature = "neo4j")]
#[semio_framework_async_macros::async_test]
#[ignore = "needs the claimed shared neo4j: `os-hub-ts backend run neo4j -- …`"]
async fn neo4j_commits_and_reopen_storms_stay_within_their_throughput_bounds() {
    if !db_storage::process_isolated_law("db_engine::throughput_tests::neo4j_commits_and_reopen_storms_stay_within_their_throughput_bounds") {
        return;
    }
    throughput_law(ThroughputTarget::Neo4j { uri: claimed_backend_env("OS_HUB_NEO4J_URI"), user: claimed_backend_env("OS_HUB_NEO4J_USER"), password: claimed_backend_env("OS_HUB_NEO4J_PASSWORD") }).await;
}
