/// 🧸️ Fixtures, batch and concurrent ensure: the controlled mount emit and fanout probe fixtures, the declared maximal batch, the create-catalog publication order and the concurrent ensure of one mounted actor.

async fn rejected_document_open_error(mut rejected: DatabaseDocumentOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

#[derive(Default)]
struct ControlledMountEmitState {
    entered: bool,
    pub released: bool,
    events: Vec<EmitEvent>,
}

#[derive(Clone, Default)]
struct ControlledMountEmit {
    pub state: Arc<(Mutex<ControlledMountEmitState>, std::sync::Condvar)>,
}

impl ControlledMountEmit {
    fn wait_until_document_event(&self) {
        let (state, signal) = &*self.state;
        let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        while !state.entered {
            state = signal.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    fn release_document_event(&self) {
        let (state, signal) = &*self.state;
        state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).released = true;
        signal.notify_all();
    }

    fn document_event_count(&self) -> usize {
        self.state.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner).events.iter().filter(|event| event.document.is_some()).count()
    }
}

impl Emit for ControlledMountEmit {
    async fn emit(&self, event: EmitEvent) {
        let (state, signal) = &*self.state;
        let mut state = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if event.document.is_some() {
            state.entered = true;
            signal.notify_all();
            while !state.released {
                state = signal.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        }
        state.events.push(event);
        signal.notify_all();
    }
}

struct MountFanoutLockProbe {
    pub registry: Arc<Mutex<DatabaseDocumentMountRegistry>>,
    called: std::sync::atomic::AtomicBool,
    unlocked: std::sync::atomic::AtomicBool,
}

impl std::task::Wake for MountFanoutLockProbe {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.unlocked.store(self.registry.try_lock().is_ok(), std::sync::atomic::Ordering::Release);
        self.called.store(true, std::sync::atomic::Ordering::Release);
    }
}

async fn decode_query_json(mut stream: QueryStream) -> serde_json::Value {
    let mut bytes = Vec::new();
    for fragment in stream.get(0).and_then(QueryResultEntry::value).expect("retained query value").fragments() {
        bytes.extend_from_slice(fragment);
    }
    let value = db_artifact_contract::decode_pathmap_json(&bytes).await.unwrap();
    while stream.close_step().unwrap() {}
    assert!(stream.terminal_is_empty());
    value
}
//#region 🧸️Fixtures
async fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push(format!("db_engine-test-{name}-{}-{}", std::process::id(), now_ms().await));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

async fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::ArtifactId, entries: &[(&str, serde_json::Value)]) -> protocol::MutationEnvelope {
    let mut payload = serde_json::Map::new();
    for (path, value) in entries {
        payload.insert((*path).to_string(), value.clone());
    }
    protocol::MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: protocol::ActorId(actor.to_string()),
        dependencies: deps.iter().map(|dep| protocol::MutationId((*dep).to_string())).collect(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(payload)).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db_artifact_contract::DB_PATHMAP_SCHEMA.to_string()), payload: db_artifact_contract::encode_pathmap_json(&serde_json::Value::Object(serde_json::Map::new())).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}

/// 📏️ C12 P1 live (hub 8010 on p24: a 600-envelope outbox answered `hub.unavailable` forever): the largest batch the document
/// backbone declares legal (the wire's own exact decoder admits it) commits through a production-profile `Database` →
/// `ArtifactHandle::submit` as one transaction (the hub's socket judges the declared bytes), and a batch of one envelope more
/// than the declared count is refused PERMANENTLY (`LimitExceeded`), never as the transient `Unavailable` a client would resend
/// without end.
#[semio_framework_async_macros::async_test]
async fn a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently() {
    if !db_io_driver::process_isolated_law("tests::a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently") {
        return;
    }
    let _history_capacity = db_artifact_history::history_capacity_test_lock();
    let root = tempdir("declared-maximal-batch").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
    let document = protocol::ArtifactId("declared-maximal".to_string());
    let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
    let mut envelopes = Vec::new();
    let mut bytes = 0usize;
    let next = loop {
        let next = envelope(&format!("{:x}", envelopes.len()), &[], "a", &document, &[]).await;
        let mut encoded = Vec::new();
        protocol::encode_envelope(&next, &mut encoded);
        let count_bytes = if envelopes.len() + 1 < 128 { 1 } else if envelopes.len() + 1 < 16_384 { 2 } else { 3 };
        if envelopes.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
            break next;
        }
        bytes += encoded.len();
        envelopes.push(next);
    };
    assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&envelopes)).is_ok(), "the batch is declared legal by the wire's own decoder");
    let mut beyond = envelopes.clone();
    beyond.push(next);
    assert!(protocol::decode_document_backbone_envelopes_exact(&protocol::encode_envelopes(&beyond)).is_err(), "one more envelope leaves the declaration");
    let mut over = Vec::with_capacity(protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES + 1);
    for index in 0..=protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES {
        over.push(envelope(&format!("over-{index:x}"), &[], "a", &document, &[]).await);
    }
    assert!(envelopes.len() > 256, "the declared-maximal batch ({} envelopes) exceeds the former 256-envelope submit credit", envelopes.len());
    let count = envelopes.len() as u64;
    let receipt = db_actor::block_on(handle.submit(db_artifact_contract::CommandBatch::new(envelopes).await.unwrap(), db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() })).unwrap().unwrap();
    assert_eq!(receipt.frontier.head_seq, count, "the whole declared batch commits");
    let refused = db_actor::block_on(handle.submit(db_artifact_contract::CommandBatch { envelopes: over }, db_artifact_contract::SubmitOptions::default())).and_then(std::convert::identity);
    assert!(matches!(refused, Err(DbError::LimitExceeded(_))), "a batch beyond the declaration is a permanent refusal: {refused:?}");
    assert_eq!(handle.frontier().await.unwrap().head_seq, count, "the refused batch applied nothing");
}
//#endregion 🧸️Fixtures
//#region 🔖️Database open/catalog
#[semio_framework_async_macros::async_test]
async fn database_create_catalog_durable_publication_precedes_authority_spawn_emit_and_registration() {
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let database = Database::open(test_worker_pool(), DbConfig::for_profile(Profile::Test), storage).await.unwrap();
    let transaction = database.create_document_catalog_retained(protocol::ArtifactId(String::from("durable-first"))).unwrap();
    let (_, document, _, actual) = transaction.await.unwrap().into_parts().unwrap();
    assert!(actual.is_ok());
    assert!(database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
    assert_eq!(database.catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entries.last().map(|entry| &entry.document), Some(&document));
}

#[semio_framework_async_macros::async_test]
async fn database_concurrent_ensure_mounts_one_actor_and_one_writer() {
    let pool = test_worker_pool();
    let storage = Arc::new(db_storage::DbBackend::Memory(db_storage_memory::MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let mut database = Database::open(pool.clone(), DbConfig::for_profile(Profile::Test), storage.clone()).await.unwrap();
    let document = protocol::ArtifactId(String::from("single-flight-empty"));
    let mut first_future = Box::pin(database.ensure_document(&document));
    let mut second_future = Box::pin(database.ensure_document(&document));
    let mut first_result = None;
    let mut second_result = None;
    let (first_result, second_result) = std::future::poll_fn(|context| {
        if first_result.is_none() {
            if let std::task::Poll::Ready(result) = first_future.as_mut().poll(context) {
                first_result = Some(result);
            }
        }
        if second_result.is_none() {
            if let std::task::Poll::Ready(result) = second_future.as_mut().poll(context) {
                second_result = Some(result);
            }
        }
        match (first_result.take(), second_result.take()) {
            (Some(first), Some(second)) => std::task::Poll::Ready((first, second)),
            (first, second) => {
                first_result = first;
                second_result = second;
                std::task::Poll::Pending
            }
        }
    })
    .await;
    drop(first_future);
    drop(second_future);
    let first = first_result.unwrap();
    let second = second_result.unwrap();
    assert!(Arc::ptr_eq(&first.authority, &second.authority));
    assert_eq!(database.catalog().await.artifacts.iter().filter(|entry| entry.document == document).count(), 1);
    let wal = storage.wal().await;
    let core = to_core_document_id(&document).await;
    assert!(matches!(wal.acquire_writer(&core).await, Err(DbError::Conflict(_))));
    drop(wal);
    drop(first);
    drop(second);
    if let Err(error) = database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await {
        let closing = database.closing_authority.as_ref().map(|(document, authority)| format!("document={document} {}", authority.shutdown_debug_witness())).unwrap_or_else(|| String::from("no-closing-authority"));
        eprintln!(
            "concurrent mount shutdown retained state: closing={closing} registry={} emit_started={} pool_use={}; error={error}",
            database.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
            database.shutdown_emit_started,
            database.pool_use.as_ref().map_or(0, Arc::strong_count),
        );
        panic!("database shutdown after concurrent mount failed: {error}");
    }
    pool.shutdown().unwrap();
}
//#endregion 🔖️Database open/catalog
