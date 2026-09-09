use super::*;
use crate::editor::remodeling::testkit::{app_with_registry, RemodelingApp};
use crate::editor::remodeling::RemodelingPlayApp;

use semio_framework_plugin::testkit::meta;
use semio_framework_plugin::{ArtifactEditor, InvocationResult, PluginApp};

fn test_session(job_id: &str, requested_stage: RequestedStage) -> ReconstructionSession {
    ReconstructionSession {
        job_id: job_id.into(),
        artifact_authority: format!("document=test:app=test:operation={job_id}:generation=test"),
        engine: remodeling_engine::ReconstructionEngine::new(&remodeling_engine::EngineParams::default()),
        gcp_count: 0,
        requested_stage,
        stream_index: 0,
        frame_index: 0,
        ingestion: None,
        tick: 0,
        terminal: None,
    }
}

fn store_session(generation: u64, session: ReconstructionSession) {
    assert!(admit_session(generation, &session.job_id));
    put_session(generation, session);
}

fn forget_all_remodeling_process_state() {
    *sessions().lock().expect("remodeling reconstruction sessions lock") = ReconstructionSessions::default();
    crate::forget_all_remodeling_content_for_test();
}

async fn dispatch_public_action(app: &mut RemodelingApp, action: &str, args: Option<serde_json::Value>) -> InvocationResult {
    let args = args.map(Into::into);
    let command = <RemodelingPlayApp as ArtifactEditor>::command_from_action(action, args.as_ref()).expect("public Remodeling action bridge");
    app.dispatch_typed(command, &meta("local")).await.expect("public ActionBus worker dispatch")
}

async fn dispatch_continuation(app: &mut RemodelingApp, effect: Effect) -> InvocationResult {
    let Effect::DispatchAction { action, args, .. } = effect else { panic!("reconstruction continuation action") };
    assert_eq!(action, ADVANCE_RECONSTRUCTION_ACTION_ID);
    let args = args.map(|value| serde_json::from_str::<serde_json::Value>(&dsl::json::from_dsl_value(&value).to_string()).expect("continuation args"));
    dispatch_public_action(app, &action, args).await
}

async fn drive_public_reconstruction(app: &mut RemodelingApp, start_action: &str) -> (RemodelingSnapshot, Vec<ReconstructionStage>) {
    let start_args = matches!(start_action, "runStage" | "retryStage").then(|| json!({ "stage": "texturing" }));
    let mut result = dispatch_public_action(app, start_action, start_args).await;
    let mut stages = Vec::new();
    for _ in 0..MAX_RECONSTRUCTION_TICKS {
        assert_eq!(result.mutations.len(), 1, "every active reconstruction handler turn emits exactly one durable mutation");
        let snapshot = app.snapshot().expect("worker-applied Remodeling snapshot");
        if !stages.contains(&snapshot.job.stage) {
            stages.push(snapshot.job.stage);
        }
        let next = result.requested_effects.into_iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == ADVANCE_RECONSTRUCTION_ACTION_ID));
        let Some(next) = next else { return (snapshot, stages) };
        result = dispatch_continuation(app, next).await;
    }
    panic!("public reconstruction did not reach a terminal stage")
}

fn typed_rows(ops: &str) -> Vec<String> {
    ops.lines().filter_map(|line| line.strip_prefix("  ").map(str::to_string)).collect()
}

fn assert_durable_chunk_ceiling(mutation: &RemodelingMutation) {
    let RemodelingMutation::CreateAsset(payload) = mutation else { return };
    let chunked = crate::remodeling_asset_stage_parts(&payload.key).is_some() || crate::remodeling_mesh_stage_asset_parts(&payload.key).is_some();
    if chunked {
        let bytes = base64_codec::base64_standard_decode(&payload.asset.data).expect("typed durable chunk base64");
        assert!(bytes.len() <= MESH_CHUNK_BYTES, "every shared durable asset/mesh row is at most 4 KiB raw");
    }
}

fn durable_input_assets(snapshot: &RemodelingSnapshot) -> Vec<(String, ImageAsset)> {
    snapshot
        .streams
        .iter()
        .flat_map(|stream| &stream.frames)
        .map(|frame| {
            let asset = crate::remodeling_asset(snapshot, &frame.asset_id).expect("durable input asset");
            (frame.asset_id.clone(), asset)
        })
        .collect()
}

async fn public_app_with_inputs(document: &str) -> RemodelingApp {
    let mut app = app_with_registry().await;
    for index in 0..4 {
        let payload = crate::editor::remodeling::commands::import_frame_payload::checker_data_url(24, 24, 3).await;
        dispatch_public_action(&mut app, "importFramePayload", Some(json!({ "payload": payload, "name": format!("{document}-frame-{index}.png"), "index": index }))).await;
    }
    app
}

fn continuation_identity(effect: &Effect) -> (u64, String) {
    let Effect::DispatchAction { args: Some(args), .. } = effect else { panic!("continuation identity") };
    let value = serde_json::from_str::<serde_json::Value>(&dsl::json::from_dsl_value(args).to_string()).expect("continuation identity args");
    (value["generation"].as_u64().expect("generation"), value["jobId"].as_str().expect("job id").to_string())
}

#[semio_framework_async_macros::async_test]
async fn one_continuation_advances_at_most_one_engine_unit() {
    assert_eq!(RECONSTRUCTION_STEP_BUDGET, 1);
    let payload = AdvanceReconstruction { generation: 7, job_id: "job-7".into(), requested_stage: "matching-features".into(), phase: "pipeline".into(), stream_index: 2, frame_index: 3, terminal_cursor: 0, tick: 11 };
    let Effect::DispatchAction { args, .. } = queue(&payload) else { panic!("continuation effect") };
    let value = serde_json::from_str::<serde_json::Value>(&dsl::json::from_dsl_value(&args.expect("args")).to_string()).expect("decode args");
    assert_eq!(value["generation"], 7);
    assert_eq!(value["requestedStage"], "matching-features");
    assert_eq!(value["tick"], 11);
}

#[semio_framework_async_macros::async_test]
async fn snapshot_frame_ingestion_decodes_owned_leaves_without_whole_asset_reassembly() {
    let app = public_app_with_inputs("snapshot-leaves").await;
    let snapshot = app.snapshot().expect("snapshot with durable input");
    let frame = snapshot.streams.first().and_then(|stream| stream.frames.first()).expect("durable frame");
    let expected_identity = snapshot.assets.get(&frame.asset_id).expect("durable asset handle").child_id.clone();
    let mut ingestion = frame_ingestion(&snapshot, frame).expect("active snapshot-to-ingestion path");
    assert_eq!(ingestion._asset_identity, expected_identity);
    assert!(ingestion.compressed.leaf_lengths().iter().all(|length| *length <= MESH_CHUNK_BYTES));
    let input_len = ingestion.compressed.len();
    let observed_reads = ingestion.compressed.clone();
    ingestion.decoder = Some(BoundedStillDecoder::new(&ingestion.mime, std::mem::take(&mut ingestion.compressed)));
    loop {
        match ingestion.decoder.as_mut().expect("active decoder").advance() {
            BoundedDecodeProgress::Working => {}
            BoundedDecodeProgress::Complete(image) => {
                assert_eq!((image.width, image.height), (24, 24));
                break;
            }
            BoundedDecodeProgress::Failed(error) => panic!("snapshot-owned leaves failed to decode: {error}"),
        }
    }
    let metrics = observed_reads.read_metrics();
    assert!(metrics.sequential_bytes > 0 && metrics.sequential_bytes <= input_len, "active PNG ingestion permits no duplicate full read");
    assert!(metrics.sequential_reads <= input_len.saturating_add(2));
    assert!(metrics.largest_sequential_read <= MESH_CHUNK_BYTES);
    assert!(metrics.random_byte_reads <= input_len.min(32));
    assert!(metrics.largest_random_read <= 1);
}

#[test]
fn distinct_stage_requests_stop_after_distinct_dependency_prefixes() {
    let matching = RequestedStage::parse("matching-features").expect("matching stage");
    let dense = RequestedStage::parse("dense-stereo").expect("dense stage");
    assert!(requested_stage_complete(matching, remodeling_engine::EngineStage::EstimatingPoses));
    assert!(!requested_stage_complete(dense, remodeling_engine::EngineStage::EstimatingPoses));
    assert!(requested_stage_complete(dense, remodeling_engine::EngineStage::FusingVolume));
    assert_eq!(matching.wire(), "matching-features");
    assert_eq!(dense.wire(), "dense-stereo");
}

#[test]
fn terminal_phase_and_preview_work_are_finite() {
    assert_eq!(TERMINAL_CAMERA_WORK, 64);
    assert_eq!(TERMINAL_POINT_WORK, 256);
    assert_eq!(TERMINAL_QUALITY_WORK, 256);
    assert_eq!(TERMINAL_GEO_WORK, 256);
    assert_eq!(PREVIEW_CAMERA_LIMIT, 64);
    assert_eq!(PREVIEW_POINT_LIMIT, 256);
    assert_eq!(terminal_phase_wire(TerminalPhase::Sparse), "terminal-sparse");
    assert_eq!(terminal_phase_wire(TerminalPhase::Commit), "terminal-commit");
}

#[semio_framework_async_macros::async_test]
async fn public_action_bus_workers_replay_every_start_action_from_genesis_after_total_process_loss() {
    for start_action in ["runReconstruction", "runStage", "retryStage"] {
        forget_all_remodeling_process_state();
        let mut app = public_app_with_inputs(start_action).await;

        let (terminal, stages) = drive_public_reconstruction(&mut app, start_action).await;
        assert_eq!(terminal.job.stage, ReconstructionStage::Done, "{start_action} reaches the real terminal commit");
        for required in [
            ReconstructionStage::Ingesting,
            ReconstructionStage::ExtractingFeatures,
            ReconstructionStage::MatchingFeatures,
            ReconstructionStage::EstimatingPoses,
            ReconstructionStage::BundleAdjusting,
            ReconstructionStage::DenseStereo,
            ReconstructionStage::FusingVolume,
            ReconstructionStage::ExtractingSurface,
            ReconstructionStage::CleaningMesh,
            ReconstructionStage::Texturing,
            ReconstructionStage::ReportingQc,
            ReconstructionStage::Done,
        ] {
            assert!(stages.contains(&required), "{start_action} missed {required:?}");
        }

        let terminal_handle = terminal.results.mesh.mesh.clone();
        let terminal_mesh = crate::resolve_bounded_remodeling_mesh(&terminal.durable_artifacts, &terminal_handle).expect("terminal bounded mesh");
        let terminal_sparse = terminal.results.sparse.as_ref().map(|sparse| sparse.points.to_f32_vec_from(&terminal.durable_artifacts)).unwrap_or_default();
        assert!(!terminal_sparse.is_empty(), "terminal sparse content is committed through the compact event");
        assert!(!terminal.durable_artifacts.is_empty());
        for artifact in terminal.durable_artifacts.values() {
            assert!(!artifact.chunks.is_empty());
            for chunk in &artifact.chunks {
                let leaf = base64_codec::base64_standard_decode(chunk).expect("durable leaf encoding");
                assert!(leaf.len() <= 4 * 1024, "durable state never hides a whole unbounded payload");
            }
        }
        let terminal_inputs = durable_input_assets(&terminal);
        assert_eq!(terminal_inputs.len(), 4);
        let (_, _, terminal_chunk_count) = crate::replayable_remodeling_mesh_handle_parts(&terminal_handle).expect("terminal durable mesh handle");
        assert!(terminal_chunk_count > 0);

        let files = app.document_text().await.expect("public typed Remodeling op log");
        let rows = typed_rows(&files.ops);
        assert!(!rows.is_empty());
        for row in &rows {
            let mutation: RemodelingMutation = protocol::OpText::parse_op(row).expect("typed Remodeling OpText row");
            assert_durable_chunk_ceiling(&mutation);
        }

        forget_all_remodeling_process_state();
        assert_eq!(crate::resolve_bounded_remodeling_mesh(&terminal.durable_artifacts, &terminal_handle), Some(terminal_mesh.clone()));
        assert_eq!(terminal.results.sparse.as_ref().expect("terminal sparse handle").points.to_f32_vec_from(&terminal.durable_artifacts), terminal_sparse);
        let mut replayed = app_with_registry().await;
        for row in &rows {
            replayed.ingest_operations_text(row).await.expect("one typed row replayed from genesis");
        }
        let replayed_snapshot = replayed.snapshot().expect("replayed snapshot");
        assert_eq!(replayed_snapshot.results.mesh.mesh, terminal_handle);
        assert_eq!(replayed_snapshot.results.sparse.as_ref().expect("replayed sparse").points.to_f32_vec_from(&replayed_snapshot.durable_artifacts), terminal_sparse);
        assert_eq!(durable_input_assets(&replayed_snapshot), terminal_inputs);
        assert_eq!(crate::resolve_bounded_remodeling_mesh(&replayed_snapshot.durable_artifacts, &replayed_snapshot.results.mesh.mesh), Some(terminal_mesh.clone()));

        replayed.handle_action("commitCheckpoint", None, &meta("local")).await.expect("checkpoint replayed terminal document");
        let checkpoint = replayed.document_pack().await.expect("checkpointed terminal pack");
        forget_all_remodeling_process_state();
        let mut restored = app_with_registry().await;
        restored.load_document_pack(&checkpoint).await.expect("restore checkpointed terminal document");
        let restored_snapshot = restored.snapshot().expect("restored terminal snapshot");
        assert_eq!(restored_snapshot.results.mesh.mesh, terminal_handle);
        assert_eq!(restored_snapshot.results.sparse.as_ref().expect("restored sparse").points.to_f32_vec_from(&restored_snapshot.durable_artifacts), terminal_sparse);
        assert_eq!(durable_input_assets(&restored_snapshot), terminal_inputs);
        assert_eq!(crate::resolve_bounded_remodeling_mesh(&restored_snapshot.durable_artifacts, &restored_snapshot.results.mesh.mesh), Some(terminal_mesh));
    }
}

#[semio_framework_async_macros::async_test]
async fn public_workers_isolate_two_documents_and_reject_cancelled_stale_aba_continuations() {
    forget_all_remodeling_process_state();
    let mut document_a = public_app_with_inputs("document-a").await;
    let mut document_b = public_app_with_inputs("document-b").await;

    let start_a = dispatch_public_action(&mut document_a, "runReconstruction", None).await;
    let old_a = start_a.requested_effects.into_iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == ADVANCE_RECONSTRUCTION_ACTION_ID)).expect("document A continuation");
    let (old_generation, old_job) = continuation_identity(&old_a);

    let (terminal_b, _) = drive_public_reconstruction(&mut document_b, "runReconstruction").await;
    assert_eq!(terminal_b.job.stage, ReconstructionStage::Done, "document B completes while document A remains admitted");

    dispatch_public_action(&mut document_a, "cancelReconstruction", None).await;
    let cancelled_a = document_a.snapshot().expect("cancelled document A");
    assert!(cancelled_a.job.cancel_requested);
    let restart_a = dispatch_public_action(&mut document_a, "runReconstruction", None).await;
    let new_a = restart_a.requested_effects.into_iter().find(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == ADVANCE_RECONSTRUCTION_ACTION_ID)).expect("document A replacement continuation");
    let (new_generation, new_job) = continuation_identity(&new_a);
    assert_ne!(new_generation, old_generation, "generation identity is never reused after cancellation");
    assert_ne!(new_job, old_job, "job identity is never reused after cancellation");

    let stale = dispatch_continuation(&mut document_a, old_a).await;
    assert!(stale.mutations.is_empty());
    assert!(stale.requested_effects.is_empty());
    let live_a = document_a.snapshot().expect("live document A replacement");
    assert_eq!(live_a.job.id, new_job, "stale ABA delivery cannot overwrite the replacement job");
    assert_eq!(terminal_b, document_b.snapshot().expect("document B remains terminal"));
    dispatch_public_action(&mut document_a, "cancelReconstruction", None).await;
}

#[test]
fn shared_durable_chunk_admission_accepts_4k_and_rejects_overflow_and_malformed_rows() {
    forget_all_remodeling_process_state();
    let asset_max = vec![0x5a; MESH_CHUNK_BYTES];
    let asset_over = vec![0x5a; MESH_CHUNK_BYTES + 1];
    assert!(crate::stage_remodeling_asset_chunk("asset-max", crate::RemodelingAssetContentKind::Raster, 0, &base64_codec::base64_standard_encode(&asset_max)).is_ok());
    assert!(crate::stage_remodeling_asset_chunk("asset-over", crate::RemodelingAssetContentKind::Raster, 0, &base64_codec::base64_standard_encode(&asset_over)).is_err());
    assert!(crate::stage_remodeling_asset_chunk("asset-malformed", crate::RemodelingAssetContentKind::Sparse, 0, "%%%").is_err());
    assert!(crate::stage_remodeling_asset_chunk("asset-index-overflow", crate::RemodelingAssetContentKind::Sparse, u64::MAX, &base64_codec::base64_standard_encode([1])).is_err());

    let mut mesh_max = vec![11];
    mesh_max.resize(MESH_CHUNK_BYTES, 1);
    let mut mesh_over = mesh_max.clone();
    mesh_over.push(1);
    assert!(crate::stage_remodeling_mesh_chunk("mesh-max", 0, &base64_codec::base64_standard_encode(&mesh_max)).is_ok());
    assert!(crate::stage_remodeling_mesh_chunk("mesh-over", 0, &base64_codec::base64_standard_encode(&mesh_over)).is_err());
    assert!(crate::stage_remodeling_mesh_chunk("mesh-malformed", 0, "%%%").is_err());
    assert!(crate::stage_remodeling_mesh_chunk("mesh-index-overflow", u64::MAX, &base64_codec::base64_standard_encode([10, 1])).is_err());
    forget_all_remodeling_process_state();
}

#[test]
fn aggregate_staging_rejects_overflow_malformed_field_order_and_513th_elements_with_cleanup() {
    forget_all_remodeling_process_state();
    let full = base64_codec::base64_standard_encode(vec![1; MESH_CHUNK_BYTES]);
    let tail = base64_codec::base64_standard_encode(vec![1; 2_048]);
    assert!(crate::stage_remodeling_asset_chunk("sparse-overflow", crate::RemodelingAssetContentKind::Sparse, 0, &full).is_ok());
    assert!(crate::stage_remodeling_asset_chunk("sparse-overflow", crate::RemodelingAssetContentKind::Sparse, 1, &tail).is_ok());
    assert!(crate::stage_remodeling_asset_chunk("sparse-overflow", crate::RemodelingAssetContentKind::Sparse, 2, &base64_codec::base64_standard_encode([1])).is_err());
    assert_eq!(crate::staged_remodeling_asset_chunk_count("sparse-overflow"), 0);

    assert!(crate::stage_remodeling_asset_chunk("kind-mismatch", crate::RemodelingAssetContentKind::Sparse, 0, &tail).is_ok());
    assert_eq!(crate::stage_remodeling_asset_chunk("kind-mismatch", crate::RemodelingAssetContentKind::Raster, 1, &tail), Err(crate::RemodelingStagingFault::Invalid));
    assert_eq!(crate::staged_remodeling_asset_chunk_count("kind-mismatch"), 0);

    let indices = base64_codec::base64_standard_encode([3, 0, 0, 0, 0]);
    let positions = base64_codec::base64_standard_encode([0, 0, 0, 0, 0]);
    assert!(crate::stage_remodeling_mesh_chunk("field-order", 0, &indices).is_ok());
    assert!(crate::stage_remodeling_mesh_chunk("field-order", 1, &positions).is_err());
    assert_eq!(crate::staged_remodeling_mesh_chunk_count("field-order"), 0);
    assert_eq!(crate::stage_remodeling_mesh_chunk("component-count", 0, &base64_codec::base64_standard_encode([0, 1])), Err(crate::RemodelingStagingFault::Invalid));

    for (staging_id, field) in [("vertex-513", 0u8), ("triangle-513", 3u8)] {
        let mut values = Vec::with_capacity(513 * 3 * 4);
        for value in 0..513 * 3 {
            values.extend_from_slice(&(value as u32).to_le_bytes());
        }
        for (index, chunk) in values.chunks(MESH_CHUNK_BYTES - 4).enumerate() {
            let mut framed = vec![field];
            framed.extend_from_slice(chunk);
            let result = crate::stage_remodeling_mesh_chunk(staging_id, index as u64, &base64_codec::base64_standard_encode(framed));
            if index == 1 {
                assert!(result.is_err(), "513th semantic element is rejected before retention");
            } else {
                assert!(result.is_ok());
            }
        }
        assert_eq!(crate::staged_remodeling_mesh_chunk_count(staging_id), 0);
    }
    forget_all_remodeling_process_state();
}

#[test]
fn staging_busy_and_preparation_accounting_overflow_are_typed_and_early() {
    forget_all_remodeling_process_state();
    let one = base64_codec::base64_standard_encode([1]);
    for index in 0..32 {
        assert!(crate::stage_remodeling_asset_chunk(&format!("busy-{index}"), crate::RemodelingAssetContentKind::Sparse, 0, &one).is_ok());
    }
    assert_eq!(crate::stage_remodeling_asset_chunk("busy-overflow", crate::RemodelingAssetContentKind::Sparse, 0, &one), Err(crate::RemodelingStagingFault::Busy));

    let mut content_digest = ContentPreparation::new("digest-overflow".into());
    content_digest.digest_len = u64::MAX;
    assert!(content_digest.record(&[1]).is_err());
    let mut content_chunks = ContentPreparation::new("count-overflow".into());
    content_chunks.chunk_count = u64::MAX;
    assert!(content_chunks.record(&[]).is_err());

    let mesh = semio_framework::MeshData { positions: vec![0.0], ..Default::default() };
    let mut mesh_digest = MeshPreparation::new(mesh.clone(), "mesh-digest-overflow".into());
    mesh_digest.digest_len = u64::MAX;
    assert!(mesh_digest.next_chunk().is_err());
    let mut mesh_chunks = MeshPreparation::new(mesh, "mesh-count-overflow".into());
    mesh_chunks.chunk_count = u64::MAX;
    assert!(mesh_chunks.next_chunk().is_err());
    forget_all_remodeling_process_state();
}

#[semio_framework_async_macros::async_test]
async fn stale_generation_is_cancelled_before_any_mutation_or_continuation() {
    let mut old = test_session("old", RequestedStage::MatchingFeatures);
    old.terminal = Some(terminal_preparation(41, &old.artifact_authority));
    store_session(41, old);
    store_session(42, test_session("live", RequestedStage::DenseStereo));
    let mut scene = crate::default_remodeling_scene();
    scene.job.id = "live".into();
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&scene, &history);
    let stale =
        advance_reconstruction(&AdvanceReconstruction { generation: 41, job_id: "old".into(), requested_stage: "matching-features".into(), phase: "terminal-sparse".into(), stream_index: 0, frame_index: 0, terminal_cursor: 0, tick: 0 }, &view)
            .expect("stale step");
    assert!(stale.artifact_mutations.is_empty());
    assert!(stale.effects.is_empty());
    assert!(take_session(41).is_none());
    assert!(take_session(42).is_some());
    complete_session(42);
}

#[test]
fn maximum_envelope_mesh_chunks_are_bounded_replayable_and_resolve_across_threads() {
    let mesh = semio_framework::MeshData { positions: (0..512 * 3).map(|index| index as f32 * 0.001).collect(), indices: (0..512 * 3).map(|index| (index % 512) as u32).collect(), ..Default::default() };
    let expected = mesh.clone();
    let mut preparation = MeshPreparation::new(mesh, "cross-thread-stage".into());
    while let Some(chunk) = preparation.next_chunk().expect("checked mesh chunk accounting") {
        assert!(chunk.len() <= MESH_CHUNK_BYTES);
        let index = preparation.chunk_count.checked_sub(1).expect("emitted mesh chunk has a checked index");
        let encoded = base64_codec::base64_standard_encode(chunk);
        let started = std::time::Instant::now();
        assert!(std::thread::spawn(move || crate::stage_remodeling_mesh_chunk("cross-thread-stage", index, &encoded)).join().expect("worker stage").is_ok());
        assert!(started.elapsed() < std::time::Duration::from_millis(8), "full worker-hop mesh stage exceeded 8 ms");
    }
    let content_id = preparation.content_id();
    let chunk_count = preparation.chunk_count;
    let started = std::time::Instant::now();
    let durable = crate::durable_staged_remodeling_mesh("cross-thread-stage").expect("bounded staged mesh materializes");
    let mut durable_store = crate::RemodelingDurableArtifactStore::default();
    durable_store.insert(content_id.clone(), durable);
    let handle = crate::replayable_remodeling_mesh_handle(&content_id, "cross-thread-stage", chunk_count);
    crate::discard_staged_remodeling_mesh("cross-thread-stage");
    assert!(started.elapsed() < std::time::Duration::from_millis(8), "compact snapshot-owned mesh publication exceeded 8 ms");
    let resolved = std::thread::spawn(move || crate::resolve_bounded_remodeling_mesh(&durable_store, &handle)).join().expect("worker resolve").expect("durable mesh");
    assert_eq!(resolved, expected);
}

#[semio_framework_async_macros::async_test]
async fn cancellation_and_stale_delivery_are_isolated_between_documents() {
    let generation_a = 8_100_001;
    let generation_b = 8_100_002;
    let mut scene_a = crate::default_remodeling_scene();
    scene_a.job.id = "document-a-job".into();
    let mut scene_b = crate::default_remodeling_scene();
    scene_b.job.id = "document-b-job".into();
    let session_a = test_session(&scene_a.job.id, RequestedStage::Full);
    let session_b = test_session(&scene_b.job.id, RequestedStage::DenseStereo);
    let stale_payload_a = checkpoint(generation_a, &session_a);
    store_session(generation_a, session_a);
    store_session(generation_b, session_b);

    let cancel = cancel_current_reconstruction(&scene_a);
    assert!(!cancel.artifact_mutations.is_empty());
    assert!(take_session(generation_a).is_none());
    let retained_b = take_session(generation_b).expect("cancelling document A preserves document B");
    put_session(generation_b, retained_b);

    let history = semio_framework_plugin::HistoryView::empty();
    let view_a = ArtifactView::new(&scene_a, &history);
    let config = RemodelingConfig::default();
    let stale = handle_advance(&stale_payload_a, &view_a, &ConfigView { snapshot: &config, window: None }).expect("stale handler delivery");
    assert!(stale.artifact_mutations.is_empty());
    assert!(stale.effects.is_empty());
    assert!(take_session(generation_b).is_some(), "stale document A delivery cannot cancel document B");
    complete_session(generation_b);
}

#[semio_framework_async_macros::async_test]
async fn user_cancel_drops_generation_and_private_mesh_staging() {
    let mut scene = crate::default_remodeling_scene();
    scene.job.id = "cancel-job".into();
    let mut session = test_session("cancel-job", RequestedStage::Full);
    let mut terminal = terminal_preparation(92, "document=test:app=test:operation=cancel:92");
    terminal.mesh = Some(MeshPreparation::new(semio_framework::MeshData::default(), "cancel-stage".into()));
    session.terminal = Some(terminal);
    crate::stage_remodeling_mesh_chunk("cancel-stage", 0, &base64_codec::base64_standard_encode([10, 1])).expect("cancel fixture staged");
    store_session(92, session);

    let emit = cancel_current_reconstruction(&scene);
    assert!(take_session(92).is_none());
    assert_eq!(crate::staged_remodeling_mesh_chunk_count("cancel-stage"), 0);
    assert!(emit.effects.is_empty());
    assert_eq!(emit.artifact_mutations.len(), 1);
}

#[test]
fn cancellation_during_compressed_streaming_drops_the_rope_without_decode_or_publication() {
    let mut scene = crate::default_remodeling_scene();
    scene.job.id = "stream-cancel-job".into();
    let leaf = std::sync::Arc::<[u8]>::from([0x89, b'P', b'N', b'G']);
    let mut compressed = CompressedChunkRope::default();
    compressed.push(leaf.clone(), MAX_STILL_INPUT_BYTES).expect("streaming leaf admitted");
    let mut session = test_session(&scene.job.id, RequestedStage::Full);
    session.ingestion = Some(FrameIngestion { _asset_identity: "cancel-input".into(), mime: "image/png".into(), frame_index: 0, timestamp_ms: 0.0, compressed, decoder: None, decoded_image: None, sharpness_cursor: 0, sharpness_sum: 0.0 });
    assert_eq!(std::sync::Arc::strong_count(&leaf), 2);
    store_session(93, session);
    let emit = cancel_current_reconstruction(&scene);
    assert_eq!(emit.artifact_mutations.len(), 1);
    assert!(emit.effects.is_empty());
    assert!(take_session(93).is_none());
    assert_eq!(std::sync::Arc::strong_count(&leaf), 1, "cancellation releases the persistent compressed rope before decode");
}

#[semio_framework_async_macros::async_test]
async fn admission_never_evicts_an_active_worker_owned_generation() {
    let mut registry = ReconstructionSessions::default();
    for generation in 1..=MAX_LIVE_SESSIONS as u64 {
        assert!(registry.admit(generation, &format!("job-{generation}")).is_ok());
        registry.live.insert(generation, test_session(&format!("job-{generation}"), RequestedStage::Full));
    }
    let worker_owned = registry.live.remove(&1).expect("worker owns admitted session");
    assert!(registry.admit(MAX_LIVE_SESSIONS as u64 + 1, "overflow").is_err());
    assert!(registry.admitted.contains(&1));
    assert!(registry.live.contains_key(&2));
    registry.live.insert(1, worker_owned);
    assert_eq!(registry.live.len(), MAX_LIVE_SESSIONS);
}
