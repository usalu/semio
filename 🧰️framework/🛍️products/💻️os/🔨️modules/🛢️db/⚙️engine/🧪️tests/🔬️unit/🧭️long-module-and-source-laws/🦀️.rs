/// 🧭️ Long module and source laws: the long module and the retained sync hello and retry owner source laws.

/// 🐢️ Laws whose honest size takes minutes in a debug build; the `long` level runs them.
mod long {
    use super::*;

    /// ⏱️ Two dozen grown documents greeted at once right after the database reopens — a hub restart's reconnect storm —
    /// are each welcomed within the reopen bound of the hub's growth e2e (30 s; growth run g17: after 24 documents grew
    /// together and the hub restarted, a document was not welcomed in 30 s). Prints the welcome distribution.
    #[semio_framework_async_macros::async_test]
    async fn two_dozen_grown_documents_greeted_at_once_after_a_reopen_are_welcomed_within_the_reopen_bound() {
        if !db_io_driver::process_isolated_law("tests::long::two_dozen_grown_documents_greeted_at_once_after_a_reopen_are_welcomed_within_the_reopen_bound") {
            return;
        }
        const DOCUMENTS: usize = 24;
        const BATCHES: usize = 8;
        const BATCH_EDITS: usize = 16;
        const REOPEN_BOUND: std::time::Duration = std::time::Duration::from_secs(30);
        let root = tempdir("greeting-storm").await;
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 10)));
        let grown_at = std::time::Instant::now();
        {
            let mut database = Database::open_at(pool.clone(), &root, Profile::Dev).await.unwrap();
            for document_index in 0..DOCUMENTS {
                let document = protocol::ArtifactId(format!("storm-{document_index}"));
                let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
                let mut previous: Option<protocol::MutationId> = None;
                for batch_index in 0..BATCHES {
                    let mut envelopes = Vec::with_capacity(BATCH_EDITS);
                    for edit_index in 0..BATCH_EDITS {
                        let mutation_id = protocol::MutationId(format!("storm-{document_index}-{batch_index}-{edit_index}"));
                        envelopes.push(protocol::MutationEnvelope {
                            mutation_id: mutation_id.clone(),
                            document_id: document.clone(),
                            actor: protocol::ActorId("storm-author".to_string()),
                            dependencies: previous.iter().cloned().collect(),
                            observed: None,
                            target: Vec::new(),
                            diff: protocol::ArtifactDiff { schema: protocol::SchemaId("storm.v1".to_string()), payload: vec![0x5a; 256] },
                            inverse: protocol::InverseMutation { schema: protocol::SchemaId("storm.v1".to_string()), payload: Vec::new() },
                            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
                            transaction: None, verb: None, line: None,
                        });
                        previous = Some(mutation_id);
                    }
                    handle.submit(db_artifact_contract::CommandBatch::new(envelopes).await.unwrap(), db_artifact_contract::SubmitOptions::default()).await.unwrap().unwrap();
                }
                drop(handle);
            }
            database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
        }
        let grown = grown_at.elapsed();
        let database = Arc::new(Database::open_at(pool.clone(), &root, Profile::Dev).await.unwrap());
        eprintln!("greeting storm of {DOCUMENTS} documents begins after growing them in {grown:?}");
        let storm_at = std::time::Instant::now();
        let mut welcomes: Vec<std::time::Duration> = std::thread::scope(|scope| {
            let greetings: Vec<_> = (0..DOCUMENTS)
                .map(|document_index| {
                    let database = database.clone();
                    scope.spawn(move || {
                        db_actor::block_on(async move {
                            let asked_at = std::time::Instant::now();
                            let mut session = database.hello(protocol::ArtifactId(format!("storm-{document_index}")), None, format!("storm-session-{document_index}"), protocol::ActorId("semio_hub".to_string()), 64 * 1024).await.unwrap();
                            session.take_welcome().unwrap().acknowledge().unwrap();
                            let welcomed = asked_at.elapsed();
                            while let Some(frame) = session.next_frame().await.unwrap() {
                                frame.acknowledge().unwrap();
                            }
                            welcomed
                        })
                    })
                })
                .collect();
            greetings.into_iter().map(|greeting| greeting.join().unwrap()).collect()
        });
        let storm = storm_at.elapsed();
        welcomes.sort();
        let ms = |duration: std::time::Duration| duration.as_secs_f64() * 1_000.0;
        eprintln!(
            "greeting storm of {DOCUMENTS} in {:.0} ms: welcome p50 {:.1} ms p90 {:.1} ms max {:.1} ms; {DOCUMENTS} documents x {} edits grown in {:.0} ms",
            ms(storm),
            ms(welcomes[DOCUMENTS / 2]),
            ms(welcomes[DOCUMENTS * 9 / 10]),
            ms(welcomes[DOCUMENTS - 1]),
            BATCHES * BATCH_EDITS,
            ms(grown)
        );
        assert!(welcomes[DOCUMENTS - 1] <= REOPEN_BOUND, "the slowest of {DOCUMENTS} simultaneous greetings took {:?}", welcomes[DOCUMENTS - 1]);
        let mut database = Arc::try_unwrap(database).ok().expect("the storm released every database handle");
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
    }

    /// 📈️ The same growth over SQLite storage, the hub's storage backend of choice: past every former
    /// wall (the eighth index entry, the 128th replaced value) with a small payload.
    #[semio_framework_async_macros::async_test]
    async fn a_sqlite_document_keeps_accepting_edits_past_every_former_wall() {
        const EDITS: usize = 150;
        let root = tempdir("growing-sqlite-document").await;
        let pool = test_worker_pool();
        let storage = db_storage_sqlite::SqliteStorage::open(pool.clone(), &root.join("db.sqlite3")).await.unwrap();
        let mut database = Database::open(pool, DbConfig::for_profile(Profile::Prod), Arc::new(db_storage::DbBackend::Sqlite(storage))).await.unwrap();
        let document = protocol::ArtifactId("growing-sqlite-doc".to_string());
        let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
        let mut previous: Option<String> = None;
        for index in 0..EDITS {
            let id = format!("grow-{index}");
            let dependencies: Vec<&str> = previous.iter().map(String::as_str).collect();
            let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&id, &dependencies, "alice", &document, &[(&format!("feature-{}", index % 8), serde_json::json!(format!("{index}:{}", "s".repeat(512))))]).await]).await.unwrap();
            handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }).await.unwrap_or_else(|error| panic!("edit {index} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("edit {index} was refused: {error:?}"));
            previous = Some(id);
        }
        assert_eq!(handle.frontier().await.unwrap().head_seq, EDITS as u64);
        drop(handle);
        database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    }

    /// 📈️ A long-lived document keeps accepting edits however large it grows: hundreds of edits with a
    /// large payload each, then a full shutdown and reopen at the same root, then more edits — every one
    /// admitted, the whole history replayed. A per-operation I/O bound that the document's size outgrew
    /// made every hub document read-only after about twenty map edits, even across a restart.
    #[semio_framework_async_macros::async_test]
    async fn a_document_keeps_accepting_edits_as_it_grows_across_restart() {
        const EDITS_BEFORE_RESTART: usize = 400;
        const EDITS_AFTER_RESTART: usize = 60;
        const PAYLOAD_BYTES: usize = 24 * 1024;
        let root = tempdir("growing-document").await;
        let document = protocol::ArtifactId("growing-doc".to_string());
        let payload = |index: usize| serde_json::json!(format!("{index:08}{}", "g".repeat(PAYLOAD_BYTES)));
        let submit = |handle: ArtifactHandle, index: usize| {
            let document = document.clone();
            async move {
                let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&format!("grow-{index}"), &[], "alice", &document, &[(&format!("feature-{}", index % 32), payload(index))]).await]).await.unwrap();
                handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }).await.unwrap_or_else(|error| panic!("edit {index} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("edit {index} was refused: {error:?}"))
            }
        };
        {
            let mut database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
            let handle = database.create_document(ArtifactSpec::new(document.clone()).await).await.unwrap();
            for index in 0..EDITS_BEFORE_RESTART {
                submit(handle.clone(), index).await;
            }
            assert_eq!(handle.frontier().await.unwrap().head_seq, EDITS_BEFORE_RESTART as u64);
            drop(handle);
            database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
        }
        let mut reopened = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
        let handle = reopened.document(&document).await.unwrap();
        assert_eq!(handle.frontier().await.unwrap().head_seq, EDITS_BEFORE_RESTART as u64, "the whole grown history replays");
        for index in EDITS_BEFORE_RESTART..EDITS_BEFORE_RESTART + EDITS_AFTER_RESTART {
            submit(handle.clone(), index).await;
        }
        assert_eq!(handle.frontier().await.unwrap().head_seq, (EDITS_BEFORE_RESTART + EDITS_AFTER_RESTART) as u64);
        let last = EDITS_BEFORE_RESTART + EDITS_AFTER_RESTART - 1;
        let queried = handle.query(Query::Get { path: format!("feature-{}", last % 32) }, Consistency::Canonical).await.unwrap();
        assert_eq!(decode_query_json(queried).await, payload(last));
        drop(handle);
        reopened.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
    }

    /// 🗃️ A database keeps opening documents however many it has served: hundreds of documents, a
    /// bounded window of them open at once, each edited, closed and reopened, then a full shutdown and
    /// reopen at the same root and every one reopened and edited again. What a document holds while it
    /// is mounted — its WAL writer and tail, its retained state — must bound how many documents are
    /// open at once, never how many a process has ever opened: keeping every served document mounted
    /// stopped a hub from opening its sixteenth document, and after every restart again.
    #[semio_framework_async_macros::async_test]
    async fn a_database_keeps_opening_documents_long_after_the_first_twenty() {
        const DOCUMENTS: usize = 240;
        const OPEN_AT_ONCE: usize = 16;
        let root = tempdir("many-documents").await;
        let document = |index: usize| protocol::ArtifactId(format!("doc-{index:04}"));
        let edit = |handle: ArtifactHandle, index: usize, round: usize| async move {
            let batch = db_artifact_contract::CommandBatch::new(vec![envelope(&format!("doc-{index}-edit-{round}"), &[], "alice", &document(index), &[(&format!("round-{round}"), serde_json::json!(index))]).await]).await.unwrap();
            handle.submit(batch, db_artifact_contract::SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() }).await.unwrap_or_else(|error| panic!("document {index} round {round} was not delivered: {error:?}")).unwrap_or_else(|error| panic!("document {index} round {round} was refused: {error:?}"))
        };
        let window = |open: &mut std::collections::VecDeque<ArtifactHandle>, handle: ArtifactHandle| {
            open.push_back(handle);
            if open.len() > OPEN_AT_ONCE {
                open.pop_front();
            }
        };
        {
            let mut database = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
            let mut open = std::collections::VecDeque::with_capacity(OPEN_AT_ONCE + 1);
            for index in 0..DOCUMENTS {
                let handle = database.create_document(ArtifactSpec::new(document(index)).await).await.unwrap_or_else(|error| panic!("document {index} was not created: {error:?}"));
                edit(handle.clone(), index, 0).await;
                window(&mut open, handle);
            }
            assert!(database.health().await.open_artifacts <= OPEN_AT_ONCE, "only the open window stays mounted");
            for index in 0..DOCUMENTS {
                let handle = database.document(&document(index)).await.unwrap_or_else(|error| panic!("document {index} was not remounted: {error:?}"));
                edit(handle.clone(), index, 1).await;
                window(&mut open, handle);
            }
            open.clear();
            database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
        }
        let mut reopened = Database::open_at(test_worker_pool(), &root, Profile::Prod).await.unwrap();
        let mut open = std::collections::VecDeque::with_capacity(OPEN_AT_ONCE + 1);
        for index in 0..DOCUMENTS {
            let handle = reopened.document(&document(index)).await.unwrap_or_else(|error| panic!("document {index} was not reopened: {error:?}"));
            assert_eq!(handle.frontier().await.unwrap().head_seq, 2, "document {index} replays both edits");
            edit(handle.clone(), index, 2).await;
            let queried = handle.query(Query::Get { path: "round-2".to_string() }, Consistency::Canonical).await.unwrap();
            assert_eq!(decode_query_json(queried).await, serde_json::json!(index));
            window(&mut open, handle);
        }
        assert!(reopened.health().await.open_artifacts <= OPEN_AT_ONCE, "only the open window stays mounted after the restart");
        open.clear();
        reopened.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(60))).await.unwrap();
    }
}

//#region 🔖️Source laws
#[test]
fn retained_sync_hello_engine_production_has_zero_blocking_waits_and_no_eager_follow_up() {
    let engine = include_str!("../../../🦀️.rs");
    let production = engine.rsplit_once("\n//#region 🧪️Tests").unwrap().0;
    assert!(!production.contains("db_actor::block_on(db_sync::handle_hello"));
    assert!(production.contains("DatabaseSyncHelloFuture::try_submit"));
}

#[test]
fn retained_retry_owners_release_their_guard_before_resubmission() {
    for (name, source) in [("db_engine", include_str!("../../../🦀️.rs"))] {
        let lines: Vec<&str> = source.lines().collect();
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start().trim_start_matches("} else ");
            let Some((_, scrutinee)) = trimmed.strip_prefix("if let ").or_else(|| trimmed.strip_prefix("while let ")).and_then(|rest| rest.split_once(" = ")) else { continue };
            if !(scrutinee.contains("retry_job.lock()") && !scrutinee.starts_with("{ ") && scrutinee.trim_end().ends_with('{')) {
                continue;
            }
            let mut depth = 1i64;
            for body in &lines[index + 1..] {
                assert!(!body.contains("retry_job.lock()") && !body.contains("submit_exact("), "{name}:{} holds the retry-job guard across a body that relocks or resubmits it: {line}", index + 1);
                depth += body.matches('{').count() as i64 - body.matches('}').count() as i64;
                if depth <= 0 {
                    break;
                }
            }
        }
    }
}
//#endregion 🔖️Source laws
