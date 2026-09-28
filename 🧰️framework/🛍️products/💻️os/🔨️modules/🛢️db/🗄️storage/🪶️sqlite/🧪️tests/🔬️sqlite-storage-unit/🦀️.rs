mod tests {
    use super::*;

    #[derive(serde::Deserialize)]
    struct PageLifecycleFixture {
        pattern_modulo: usize,
        pattern_addend: usize,
        lengths: Vec<usize>,
    }

    async fn pages(bytes: &[u8]) -> DbIoPages {
        crate::db_storage::db_io_copy_pages(bytes).unwrap().await.unwrap()
    }

    async fn assert_payload_roundtrip(storage: &SqliteStorage, bytes: Vec<u8>) {
        let expected_hash = ContentHash(*semio_framework_hash::hash(&bytes).as_bytes());
        assert!(matches!(storage.get(&expected_hash).await, Err(DbError::NotFound(_))));
        let hash = storage.put(pages(&bytes).await).await.unwrap();
        assert_eq!(hash, expected_hash);
        assert!(storage.contains(&hash).await.unwrap());
        assert_eq!(storage.len(&hash).await.unwrap(), bytes.len() as u64);
        let mut fetched = storage.get(&hash).await.unwrap();
        assert_eq!(fetched, bytes);
        while fetched.close_step().unwrap().is_some() {}
        storage.delete(&hash).await.unwrap();
        assert!(!storage.contains(&hash).await.unwrap());
        assert!(matches!(storage.get(&hash).await, Err(DbError::NotFound(_))));
    }

    #[test]
    fn sqlite_blob_append_preserves_storage_class_length_and_hex() {
        let connection = Connection::open_in_memory().unwrap();
        init_connection(&connection).unwrap();
        connection.execute("INSERT INTO db_io_stage (operation, bytes, number) VALUES (1, x'', NULL)", []).unwrap();
        connection.execute(STAGE_APPEND_SQL, params![1, &[0x00_u8, 0xff, 0x80][..]]).unwrap();
        connection.execute(STAGE_APPEND_SQL, params![1, &[0xc3_u8, 0x28][..]]).unwrap();
        let stage: (String, i64, String) = connection.query_row("SELECT typeof(bytes), length(bytes), hex(bytes) FROM db_io_stage WHERE operation = 1", [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).unwrap();
        assert_eq!(stage, ("blob".to_string(), 5, "00FF80C328".to_string()));

        connection.execute("INSERT INTO wal_segment (document, segment_index, bytes, sealed) VALUES ('oracle', 0, x'7F', 0)", []).unwrap();
        connection.execute(WAL_APPEND_STAGE_SQL, params!["oracle", 0, 1]).unwrap();
        let wal: (String, i64, String) = connection.query_row("SELECT typeof(bytes), length(bytes), hex(bytes) FROM wal_segment WHERE document = 'oracle' AND segment_index = 0", [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).unwrap();
        assert_eq!(wal, ("blob".to_string(), 6, "7F00FF80C328".to_string()));

        connection.execute("INSERT INTO payload (hash, bytes, len) SELECT 'oracle', bytes, length(bytes) FROM db_io_stage WHERE operation = 1", []).unwrap();
        let payload: (String, i64, String) = connection.query_row("SELECT typeof(bytes), len, hex(bytes) FROM payload WHERE hash = 'oracle'", [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).unwrap();
        assert_eq!(payload, ("blob".to_string(), 5, "00FF80C328".to_string()));
    }

    #[semio_framework_async_macros::async_test]
    async fn payload_roundtrip_obeys_neutral_page_boundaries_and_arbitrary_bytes() {
        let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
        let storage = SqliteStorage::open_in_memory(crate::db_storage::db_io_test_pool()).await.unwrap();

        assert_payload_roundtrip(&storage, Vec::new()).await;
        for length in fixture.lengths {
            let bytes = (0..length).map(|index| (index % fixture.pattern_modulo + fixture.pattern_addend) as u8).collect();
            assert_payload_roundtrip(&storage, bytes).await;
        }
        let mut arbitrary = (0..DB_IO_PAGE_BYTES + 1).map(|index| (index % 251 + 1) as u8).collect::<Vec<_>>();
        arbitrary[..5].copy_from_slice(&[0x00, 0xff, 0x80, 0xc3, 0x28]);
        arbitrary[DB_IO_PAGE_BYTES - 1] = 0;
        assert_payload_roundtrip(&storage, arbitrary).await;

        drop(storage);
    }

    #[semio_framework_async_macros::async_test]
    async fn typed_lane_is_lossless_at_page_boundary_and_zero() {
        let storage = SqliteStorage::open_in_memory(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "typed-sqlite".into();
        let writer = storage.acquire_writer(&document).await.unwrap();
        storage.create_segment(&writer, 0).await.unwrap();
        let bytes = vec![0x5a; DB_IO_PAGE_BYTES + 1];
        assert_eq!(storage.append(&writer, 0, pages(&bytes).await).await.unwrap(), bytes.len() as u64);
        assert_eq!(storage.read(&document, 0, ByteRange { offset: 0, len: bytes.len() as u64 }).await.unwrap(), bytes);
        let hash = storage.put(pages(&[]).await).await.unwrap();
        assert_eq!(storage.get(&hash).await.unwrap(), b"");
        writer.release().await.unwrap();
        storage.close().await.unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn wal_segment_state_observes_active_sealed_and_missing_rows() {
        let storage = SqliteStorage::open_in_memory(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "typed-sqlite-state".into();
        let writer = storage.acquire_writer(&document).await.unwrap();
        storage.create_segment(&writer, 0).await.unwrap();
        assert_eq!(storage.segment_state(&document, 0).await.unwrap(), WalSegmentState::Active);
        storage.seal(&writer, 0).await.unwrap();
        assert_eq!(storage.segment_state(&document, 0).await.unwrap(), WalSegmentState::Sealed);
        assert!(matches!(storage.segment_state(&document, 99).await, Err(DbError::NotFound(_))));
        writer.release().await.unwrap();
        storage.close().await.unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_wal_writer_real_database_alias_and_crash_are_exclusive() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🔐️writer/🧫️fixtures/🌐️remote-guard/🔣️.json")).unwrap();
        let child_mode = std::env::var("SEMIO_SQLITE_WRITER_CHILD_MODE").ok();
        let child_path = std::env::var_os("SEMIO_SQLITE_WRITER_CHILD_PATH").map(std::path::PathBuf::from);
        if let (Some(mode), Some(path)) = (child_mode, child_path) {
            let storage = SqliteStorage::open(crate::db_storage::db_io_test_pool(), &path).await.unwrap();
            let document: ArtifactId = "sqlite-writer-alias".into();
            let sentinel = std::env::var_os("SEMIO_SQLITE_WRITER_CHILD_SENTINEL").unwrap();
            match mode.as_str() {
                "conflict" => {
                    assert!(matches!(storage.acquire_writer(&document).await, Err(DbError::Conflict(_))));
                    std::fs::write(sentinel, format!("{}:conflict", std::process::id())).unwrap();
                    storage.close().await.unwrap();
                }
                "crash" => {
                    let _writer = storage.acquire_writer(&document).await.unwrap();
                    std::fs::write(sentinel, format!("{}:acquired", std::process::id())).unwrap();
                    std::process::exit(0);
                }
                _ => panic!("unknown SQLite writer child mode"),
            }
            return;
        }

        assert_eq!(fixture["mutations"], serde_json::json!(["create", "append", "sync", "seal", "truncateTail", "delete"]));
        assert!(fixture["backends"].as_array().unwrap().iter().any(|row| row["backend"] == "sqlite" && row["mechanism"] == "sidecar-file-lock" && row["crashRelease"] == "immediate"));

        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = base.join(format!("sqlite-writer-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let database = root.join("writer.sqlite3");
        let alias = root.join(".").join("writer.sqlite3");
        let storage = SqliteStorage::open(crate::db_storage::db_io_test_pool(), &database).await.unwrap();
        let contender = SqliteStorage::open(crate::db_storage::db_io_test_pool(), &alias).await.unwrap();
        let document: ArtifactId = "sqlite-writer-alias".into();
        let writer = storage.acquire_writer(&document).await.unwrap();
        assert!(matches!(contender.acquire_writer(&document).await, Err(DbError::Conflict(_))));

        let executable = std::env::current_exe().unwrap();
        let conflict_sentinel = root.join("conflict.txt");
        let conflict = std::process::Command::new(&executable)
            .args(["db_storage_sqlite::sqlite_storage::tests::sqlite_wal_writer_real_database_alias_and_crash_are_exclusive", "--exact", "--test-threads=1"])
            .env("SEMIO_SQLITE_WRITER_CHILD_MODE", "conflict")
            .env("SEMIO_SQLITE_WRITER_CHILD_PATH", &alias)
            .env("SEMIO_SQLITE_WRITER_CHILD_SENTINEL", &conflict_sentinel)
            .output()
            .unwrap();
        assert!(conflict.status.success(), "SQLite conflict child failed: {}\n{}", String::from_utf8_lossy(&conflict.stdout), String::from_utf8_lossy(&conflict.stderr));
        assert!(std::fs::read_to_string(&conflict_sentinel).unwrap().ends_with(":conflict"));
        writer.release().await.unwrap();

        let next = contender.acquire_writer(&document).await.unwrap();
        next.release().await.unwrap();
        let crash_sentinel = root.join("crash.txt");
        let crash = std::process::Command::new(&executable)
            .args(["db_storage_sqlite::sqlite_storage::tests::sqlite_wal_writer_real_database_alias_and_crash_are_exclusive", "--exact", "--test-threads=1"])
            .env("SEMIO_SQLITE_WRITER_CHILD_MODE", "crash")
            .env("SEMIO_SQLITE_WRITER_CHILD_PATH", &database)
            .env("SEMIO_SQLITE_WRITER_CHILD_SENTINEL", &crash_sentinel)
            .output()
            .unwrap();
        assert!(crash.status.success(), "SQLite crash child failed: {}\n{}", String::from_utf8_lossy(&crash.stdout), String::from_utf8_lossy(&crash.stderr));
        assert!(std::fs::read_to_string(&crash_sentinel).unwrap().ends_with(":acquired"));
        let after_crash = storage.acquire_writer(&document).await.unwrap();
        after_crash.release().await.unwrap();
        contender.close().await.unwrap();
        storage.close().await.unwrap();
        eprintln!("[DEBUG] physical SQLite aliases and a separate process shared one stable writer sidecar; terminal close and process exit each permitted exact reacquisition");
    }

    /// 📖️ Reads of a file database run on WAL readers: while another connection holds the database's write lock, concurrent
    /// reads of every blob family (WAL range, snapshot generation, index run, payload; the neutral page-lifecycle lengths)
    /// complete with exactly the committed bytes an independent SQLite connection reads (third-party oracle), and they write
    /// nothing — the oracle's `data_version` and the `-wal` file length are unchanged across them. The staging reads they
    /// replace wrote `db_io_stage` rows and blocked behind that lock.
    #[semio_framework_async_macros::async_test]
    async fn file_reads_run_on_wal_readers_beside_a_held_write_lock_and_never_write() {
        let fixture: PageLifecycleFixture = serde_json::from_str(include_str!("../../../🧫️fixtures/🧬️page-lifecycle/🔣️.json")).unwrap();
        let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let root = base.join(format!("sqlite-readers-{}-{nonce}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let database = root.join("readers.sqlite3");
        let storage = SqliteStorage::open(crate::db_storage::db_io_test_pool(), &database).await.unwrap();
        let document: ArtifactId = "sqlite-readers".into();
        let writer = storage.acquire_writer(&document).await.unwrap();
        let blobs: Vec<Vec<u8>> = fixture.lengths.iter().map(|length| (0..*length).map(|index| (index % fixture.pattern_modulo + fixture.pattern_addend) as u8).collect()).collect();
        let mut hashes = Vec::with_capacity(blobs.len());
        for (index, bytes) in blobs.iter().enumerate() {
            let key = index as u64 + 1;
            storage.create_segment(&writer, key).await.unwrap();
            storage.append(&writer, key, pages(bytes).await).await.unwrap();
            storage.write_generation(&document, key, pages(bytes).await).await.unwrap();
            storage.write_run(&document, key, pages(bytes).await).await.unwrap();
            hashes.push(storage.put(pages(bytes).await).await.unwrap());
        }

        let oracle = Connection::open(&database).unwrap();
        let data_version = |connection: &Connection| connection.query_row("PRAGMA data_version", [], |row| row.get::<_, i64>(0)).unwrap();
        let wal_len = || std::fs::metadata(root.join("readers.sqlite3-wal")).map(|metadata| metadata.len()).unwrap_or(0);
        let version_before = data_version(&oracle);
        let wal_before = wal_len();
        oracle.execute_batch("BEGIN IMMEDIATE").unwrap();
        let oracle_bytes = |sql: &str, key: i64| -> Vec<u8> { oracle.query_row(sql, params!["sqlite-readers", key], |row| row.get(0)).unwrap() };
        for (index, bytes) in blobs.iter().enumerate() {
            let key = index as i64 + 1;
            assert_eq!(&oracle_bytes("SELECT bytes FROM wal_segment WHERE document = ?1 AND segment_index = ?2", key), bytes);
            assert_eq!(&oracle_bytes("SELECT bytes FROM snapshot_generation WHERE document = ?1 AND generation = ?2", key), bytes);
            assert_eq!(&oracle_bytes("SELECT bytes FROM index_run WHERE document = ?1 AND run_id = ?2", key), bytes);
            let payload: Vec<u8> = oracle.query_row("SELECT bytes FROM payload WHERE hash = ?1", params![hashes[index].to_string()], |row| row.get(0)).unwrap();
            assert_eq!(&payload, bytes);
        }

        let storage_ref = &storage;
        let document_ref = &document;
        let reads: Vec<Result<(), String>> = std::thread::scope(|scope| {
            let threads: Vec<_> = blobs
                .iter()
                .enumerate()
                .flat_map(|(index, bytes)| ["wal", "snapshot", "index", "payload"].map(|family| (index, bytes, family)))
                .map(|(index, bytes, family)| {
                    let hash = &hashes[index];
                    scope.spawn(move || {
                        crate::db_actor::block_on(async move {
                            let key = index as u64 + 1;
                            let read = match family {
                                "wal" => storage_ref.read(document_ref, key, ByteRange { offset: 0, len: bytes.len() as u64 }).await,
                                "snapshot" => storage_ref.read_generation(document_ref, key).await,
                                "index" => storage_ref.read_run(document_ref, key).await,
                                _ => storage_ref.get(hash).await,
                            };
                            let mut pages = read.map_err(|error| format!("{family} read of blob {index} beside the held write lock failed: {error:?}"))?;
                            let equal = pages == *bytes;
                            while pages.close_step().map_err(|error| format!("{family} pages of blob {index} did not close: {error:?}"))?.is_some() {}
                            if equal {
                                Ok(())
                            } else {
                                Err(format!("{family} read of blob {index} differs from the committed bytes"))
                            }
                        })
                    })
                })
                .collect();
            threads.into_iter().map(|thread| thread.join().unwrap()).collect()
        });
        oracle.execute_batch("ROLLBACK").unwrap();
        for read in reads {
            read.unwrap();
        }
        assert_eq!(data_version(&oracle), version_before, "a read committed a write");
        assert_eq!(wal_len(), wal_before, "a read grew the SQLite WAL");

        writer.release().await.unwrap();
        storage.close().await.unwrap();
        drop(oracle);
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn wal_segment_state_decoder_rejects_non_boolean_storage_values() {
        assert_eq!(decode_wal_segment_state(0).unwrap(), WalSegmentState::Active);
        assert_eq!(decode_wal_segment_state(1).unwrap(), WalSegmentState::Sealed);
        assert!(matches!(decode_wal_segment_state(2), Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn typed_list_and_catalog_cas_are_stable() {
        let storage = SqliteStorage::open_in_memory(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document: ArtifactId = "typed-list".into();
        storage.write_generation(&document, 2, pages(b"two").await).await.unwrap();
        storage.write_generation(&document, 1, pages(b"one").await).await.unwrap();
        assert_eq!(storage.list_generations(&document).await.unwrap(), [1, 2]);
        assert_eq!(storage.latest_generation(&document).await.unwrap(), Some(2));
        let fence = storage.cas_root(EpochFence::INITIAL, pages(b"root").await).await.unwrap();
        assert_eq!(storage.read_root().await.unwrap().unwrap().1, fence);
        assert!(matches!(storage.cas_root(EpochFence::INITIAL, pages(b"stale").await).await, Err(DbError::Fenced { .. })));
    }
}
