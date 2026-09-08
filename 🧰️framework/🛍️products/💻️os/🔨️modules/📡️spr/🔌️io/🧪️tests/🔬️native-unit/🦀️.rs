mod tests {
    use super::*;
    use crate::os_spr::history::{HistoryAlternative, HistoryChange, HistoryCheckpoint};
    use std::sync::atomic::{AtomicU64, Ordering};

    /// @emoji 🎲️ Per-test unique scratch directory under `std::env::temp_dir()` — no external
    /// `tempfile` crate dependency, matching `pack_io`'s own test convention.
    static DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

    async fn scratch_dir(name: &str) -> PathBuf {
        let pid = std::process::id();
        let counter = DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("protocol_io_test_{name}_{pid}_{counter}"));
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    async fn sample_edit(id: &str) -> HistoryEdit {
        HistoryEdit {
            id: id.to_string(),
            actor: Some("actor-1".to_string()),
            started_at: "2026-07-27T00:00:00Z".to_string(),
            finished_at: Some("2026-07-27T00:00:01Z".to_string()),
            coalesce_key: None,
            description: Some("a sample edit".to_string()),
            ops: vec![crate::os_spr::history::OpPayload { text: Some("set x 1".to_string()), binary: None }],
            inverse: Vec::new(),
            meta: None,
        }
    }

    //#region 🔖️File
    #[semio_framework_async_macros::async_test]
    async fn create_then_append_then_commit_round_trips_through_open_read_only() {
        let dir = scratch_dir("create_append").await;
        let path = dir.join("doc.spr");

        let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        assert_eq!(file.resume_state().await.last_commit_seq, 0);
        file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-2").await).await.unwrap();
        file.appender().await.commit().await.unwrap();

        let read_only = HistoryFile::open_read_only(&path, &ProtocolLimits::default()).await.unwrap();
        assert_eq!(read_only.resume_state().await.last_commit_seq, 1);
        assert!(read_only.resume_state().await.end_offset > HEADER_SIZE as u64);

        let bytes = std::fs::read(&path).unwrap();
        let log = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(log.doc_id, "doc-1");
        assert_eq!(log.edits.len(), 2);
        assert_eq!(log.edits[0].id, "edit-1");
    }

    #[semio_framework_async_macros::async_test]
    #[should_panic(expected = "open_read_only")]
    async fn appender_panics_on_a_read_only_handle() {
        let dir = scratch_dir("read_only_panic").await;
        let path = dir.join("doc.spr");
        HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        let mut read_only = HistoryFile::open_read_only(&path, &ProtocolLimits::default()).await.unwrap();
        let _ = read_only.appender().await;
    }

    #[semio_framework_async_macros::async_test]
    async fn open_read_only_never_writes_to_the_file() {
        let dir = scratch_dir("read_only_no_write").await;
        let path = dir.join("doc.spr");
        let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
        file.appender().await.commit().await.unwrap();
        let before = std::fs::read(&path).unwrap();

        let _read_only = HistoryFile::open_read_only(&path, &ProtocolLimits::default()).await.unwrap();
        let after = std::fs::read(&path).unwrap();
        assert_eq!(before, after, "open_read_only must never mutate the file on disk");
    }

    #[semio_framework_async_macros::async_test]
    async fn open_append_resumes_and_preserves_prior_edits_across_a_process_restart() {
        let dir = scratch_dir("open_append").await;
        let path = dir.join("doc.spr");

        {
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
            file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
            file.appender().await.commit().await.unwrap();
        }
        {
            let mut file = HistoryFile::open_append(&path, &ProtocolLimits::default()).await.unwrap();
            assert_eq!(file.resume_state().await.last_commit_seq, 1, "the replay-commit during resume is itself commit #1 of a fresh generation");
            file.appender().await.append_edit(&sample_edit("edit-2").await).await.unwrap();
            file.appender().await.commit().await.unwrap();
        }

        let bytes = std::fs::read(&path).unwrap();
        let log = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(log.doc_id, "doc-1");
        assert_eq!(log.edits.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-1", "edit-2"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn open_append_truncates_a_torn_tail_before_resuming() {
        let dir = scratch_dir("open_append_torn").await;
        let path = dir.join("doc.spr");
        {
            let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
            file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
            file.appender().await.commit().await.unwrap();
        }
        // Simulate a crash mid-write: append garbage bytes past the last valid commit.
        {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new().append(true).open(&path).unwrap();
            f.write_all(&[0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01, 0x02]).unwrap();
        }

        let mut file = HistoryFile::open_append(&path, &ProtocolLimits::default()).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-2").await).await.unwrap();
        file.appender().await.commit().await.unwrap();

        let bytes = std::fs::read(&path).unwrap();
        let log = decode_history(&bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(log.edits.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-1", "edit-2"]);
    }
    //#endregion 🔖️File

    //#region 🔖️Sidecar
    #[semio_framework_async_macros::async_test]
    async fn sidecar_write_read_round_trips_and_names_by_hex8_of_hash() {
        let dir = scratch_dir("sidecar").await;
        let protocol_path = dir.join("doc.spr");
        let body_hash = [0xABu8; 32];
        let pack_bytes = b"a complete .spk pack file, opaque to this crate";

        write_sidecar(&protocol_path, &body_hash, pack_bytes).await.unwrap();
        let expected_path = dir.join("doc.abababab.sprc");
        assert!(expected_path.exists());

        let read_back = read_sidecar(&protocol_path, &body_hash).await.unwrap();
        assert_eq!(read_back, pack_bytes);
    }

    #[semio_framework_async_macros::async_test]
    async fn read_sidecar_missing_file_is_an_io_error() {
        let dir = scratch_dir("sidecar_missing").await;
        let protocol_path = dir.join("doc.spr");
        let result = read_sidecar(&protocol_path, &[0u8; 32]);
        assert!(matches!(result.await, Err(ProtocolError::Io(_))));
    }
    //#endregion 🔖️Sidecar

    //#region 🔖️Recover
    #[semio_framework_async_macros::async_test]
    async fn recover_file_reports_the_committed_record_count() {
        let dir = scratch_dir("recover").await;
        let path = dir.join("doc.spr");
        let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-2").await).await.unwrap();
        file.appender().await.commit().await.unwrap();

        let report = recover_file(&path, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.unwrap();
        assert_eq!(report.last_commit_seq, 1);
        assert_eq!(report.torn_tail_bytes, 0);
        // At least REC_DOC + 2x REC_EDIT + REC_COMMIT; may include extra REC_STR_DICT delta
        // frames for interned actor/doc-id/schema strings, so this is a lower bound, not exact.
        assert!(report.records_recovered >= 4);
        assert!(report.bytes_recovered > HEADER_SIZE as u64);
    }
    //#endregion 🔖️Recover

    //#region 🔖️Sync
    #[semio_framework_async_macros::async_test]
    async fn tail_follower_polls_new_edits_across_multiple_commits_and_advances_its_ordinal() {
        let dir = scratch_dir("tail").await;
        let path = dir.join("doc.spr");
        let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
        file.appender().await.commit().await.unwrap();

        let mut follower = TailFollower::open(&path, 0).await.unwrap();
        let first = follower.poll().await.unwrap();
        assert_eq!(first.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-1"]);
        assert_eq!(follower.last_edit_ordinal().await, 1);

        let empty = follower.poll().await.unwrap();
        assert!(empty.is_empty(), "no new edits since the last poll");

        file.appender().await.append_edit(&sample_edit("edit-2").await).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-3").await).await.unwrap();
        file.appender().await.commit().await.unwrap();

        let second = follower.poll().await.unwrap();
        assert_eq!(second.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-2", "edit-3"]);
        assert_eq!(follower.last_edit_ordinal().await, 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn tail_follower_open_from_a_nonzero_ordinal_skips_already_known_edits() {
        let dir = scratch_dir("tail_from_ordinal").await;
        let path = dir.join("doc.spr");
        let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-2").await).await.unwrap();
        file.appender().await.commit().await.unwrap();

        let mut follower = TailFollower::open(&path, 1).await.unwrap();
        let polled = follower.poll().await.unwrap();
        assert_eq!(polled.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["edit-2"]);
    }
    //#endregion 🔖️Sync

    //#region 🔖️Compact
    #[semio_framework_async_macros::async_test]
    async fn compact_preserves_every_edit_and_records_a_compaction_provenance_marker() {
        let dir = scratch_dir("compact").await;
        let path = dir.join("doc.spr");
        let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-1").await).await.unwrap();
        file.appender().await.append_edit(&sample_edit("edit-2").await).await.unwrap();
        file.appender().await.append_change(&HistoryChange { id: "change-1".to_string(), saved_at: "2026-07-27T00:00:02Z".to_string(), edit_ids: vec!["edit-1".to_string(), "edit-2".to_string()], description: None }).await.unwrap();
        file.appender().await.append_checkpoint(&HistoryCheckpoint { id: "ck-1".to_string(), timestamp: "2026-07-27T00:00:03Z".to_string(), change_ids: vec!["change-1".to_string()], parent_id: None, authors: vec![], message: None }).await.unwrap();
        file.appender().await.append_alternative(&HistoryAlternative { id: "alt-1".to_string(), name: "main".to_string(), checkpoint_ids: vec!["ck-1".to_string()] }).await.unwrap();
        file.appender().await.set_active(Some("alt-1")).await.unwrap();
        file.appender().await.commit().await.unwrap();
        drop(file);

        let before = decode_history(&std::fs::read(&path).unwrap(), &DecodeOptions::default()).await.unwrap();

        compact(&path, &CompactOptions { drop_ephemeral: true, keep_snapshots: KeepSnapshots::LatestN(3) }, &ProtocolLimits::default()).await.unwrap();

        let after_bytes = std::fs::read(&path).unwrap();
        let after = decode_history(&after_bytes, &DecodeOptions::default()).await.unwrap();
        assert_eq!(before, after, "compact must be identity-preserving over everything HistoryLog models");

        // The REC_COMPACTION provenance marker is present immediately after REC_DOC (there may
        // be a REC_STR_DICT delta frame before REC_DOC, for the interned doc-id/schema/actor
        // strings, but nothing is ever written between REC_DOC and REC_COMPACTION).
        let mut cursor = FrameCursor::new(&after_bytes, HEADER_SIZE as u64).await;
        let mut kinds = Vec::new();
        while let Some(frame) = cursor.next_frame().await.unwrap() {
            kinds.push((frame.kind, frame.payload().await.to_vec()));
        }
        let doc_index = kinds.iter().position(|(kind, _)| *kind == crate::os_spr::REC_DOC).expect("REC_DOC present");
        let (compaction_kind, compaction_payload) = &kinds[doc_index + 1];
        assert_eq!(*compaction_kind, crate::os_spr::REC_COMPACTION);
        assert_eq!(compaction_payload[1], 1, "drop_ephemeral encoded as 1");
        assert_eq!(compaction_payload[2], 2, "keep_snapshots tag 2 = LatestN");

        // The commit chain genuinely restarted: exactly one commit, seq 1.
        let report = recover_file(&path, &ProtocolLimits::default(), RecoveryMode::LastCommit).await.unwrap();
        assert_eq!(report.last_commit_seq, 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn compact_on_an_already_minimal_file_is_a_harmless_no_op_content_wise() {
        let dir = scratch_dir("compact_minimal").await;
        let path = dir.join("doc.spr");
        let mut file = HistoryFile::create(&path, "doc-1", "schema-1", &WriteOptions::default()).await.unwrap();
        file.appender().await.commit().await.unwrap();
        drop(file);

        compact(&path, &CompactOptions { drop_ephemeral: false, keep_snapshots: KeepSnapshots::All }, &ProtocolLimits::default()).await.unwrap();

        let log = decode_history(&std::fs::read(&path).unwrap(), &DecodeOptions::default()).await.unwrap();
        assert_eq!(log.doc_id, "doc-1");
        assert!(log.edits.is_empty());
    }
    //#endregion 🔖️Compact
}
