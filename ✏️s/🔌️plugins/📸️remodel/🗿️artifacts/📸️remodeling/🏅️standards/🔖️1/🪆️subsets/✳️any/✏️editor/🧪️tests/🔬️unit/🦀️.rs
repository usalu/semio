
use super::*;
use crate::editor::remodeling::testkit::{app, app_with_registry, remodeling_app_manifest_for_testkit, render};
use protocol::{OpBinary, OpText};
use semio_framework_plugin::testkit;
use semio_framework_plugin::{EditorApp, HistoryView};

//#region 🧪️RetainedCatalogOracle
#[derive(Debug, PartialEq, Eq)]
struct RemodelingRetainedCatalogSummary {
    routes: usize,
    bounded: usize,
    resumable: usize,
    unique: bool,
    bounded_ids: std::collections::BTreeSet<String>,
    host_only_ids: std::collections::BTreeSet<String>,
}

trait RemodelingRetainedCatalogOracle {
    fn summarize(&self, fixture: &str) -> RemodelingRetainedCatalogSummary;
}

struct SerdeJsonRemodelingRetainedCatalogOracle;

impl RemodelingRetainedCatalogOracle for SerdeJsonRemodelingRetainedCatalogOracle {
    fn summarize(&self, fixture: &str) -> RemodelingRetainedCatalogSummary {
        let document: serde_json::Value = serde_json::from_str(fixture).expect("language-neutral retained catalog fixture");
        let routes = document.get("routes").and_then(serde_json::Value::as_array).expect("routes array");
        let bounded = routes.iter().filter(|route| route.get("execution").and_then(serde_json::Value::as_str) == Some("bounded")).count();
        let resumable = routes.iter().filter(|route| route.get("execution").and_then(serde_json::Value::as_str) == Some("resumable")).count();
        let ids: std::collections::BTreeSet<&str> = routes.iter().filter_map(|route| route.get("id").and_then(serde_json::Value::as_str)).collect();
        let bounded_ids = routes
            .iter()
            .filter(|route| route.get("execution").and_then(serde_json::Value::as_str) == Some("bounded"))
            .filter_map(|route| route.get("id").and_then(serde_json::Value::as_str).map(str::to_string))
            .collect::<std::collections::BTreeSet<_>>();
        let host_only_ids = document
            .get("publicationContracts")
            .and_then(serde_json::Value::as_array)
            .expect("publication contracts array")
            .iter()
            .filter(|contract| contract.get("lanes").and_then(serde_json::Value::as_array).is_some_and(|lanes| lanes.as_slice() == [serde_json::Value::String("hostOnly".into())]))
            .filter_map(|contract| contract.get("toolId").and_then(serde_json::Value::as_str).map(str::to_string))
            .collect::<std::collections::BTreeSet<_>>();
        RemodelingRetainedCatalogSummary { routes: routes.len(), bounded, resumable, unique: ids.len() == routes.len(), bounded_ids, host_only_ids }
    }
}

#[semio_framework_async_macros::async_test]
async fn retained_command_catalog_matches_the_serde_json_oracle() {
    let oracle = SerdeJsonRemodelingRetainedCatalogOracle.summarize(include_str!("../../🧫️fixtures/🚧️retained-command-limits/🔣️.json"));
    let command_ids: std::collections::BTreeSet<&str> = every_command().iter().map(RemodelingCommand::command_id).collect();
    let bounded_ids: std::collections::BTreeSet<&str> = REMODELING_RETAINED_TOOL_IDS.iter().copied().collect();
    let bounded_owned = bounded_ids.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let host_only_ids = <RemodelingRetainedCommandJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .filter(|contract| contract.lanes == [semio_framework_plugin::ArtifactToolPublicationLane::HostOnly])
        .map(|contract| contract.tool_id.to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let subject = RemodelingRetainedCatalogSummary {
        routes: command_ids.len(),
        bounded: bounded_ids.len(),
        resumable: command_ids.difference(&bounded_ids).count(),
        unique: bounded_ids.len() == REMODELING_RETAINED_TOOL_IDS.len() && bounded_ids.is_subset(&command_ids),
        bounded_ids: bounded_owned.clone(),
        host_only_ids,
    };
    let expected_host_only = ["exportQcReport", "importFrames", "importVideo"].iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(oracle, RemodelingRetainedCatalogSummary { routes: 42, bounded: 42, resumable: 0, unique: true, bounded_ids: bounded_owned, host_only_ids: expected_host_only });
    assert_eq!(subject, oracle);
}

#[semio_framework_async_macros::async_test]
async fn retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures() {
    let fixture = include_str!("../../🧫️fixtures/🚧️retained-command-limits/🔣️.json");
    let expected = ["exportQcReport", "importFrames", "importVideo"].iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let wrong_lane = fixture.replacen("\"hostOnly\"", "\"artifact\"", 1);
    let wrong_tool = fixture.replacen("\"importFrames\"", "\"forgedImport\"", 1);
    assert_ne!(wrong_lane, fixture, "the hostile lane mutation must actually change the fixture");
    assert_ne!(wrong_tool, fixture, "the hostile tool mutation must actually change the fixture");
    assert_ne!(SerdeJsonRemodelingRetainedCatalogOracle.summarize(&wrong_lane).host_only_ids, expected);
    assert_ne!(SerdeJsonRemodelingRetainedCatalogOracle.summarize(&wrong_tool).host_only_ids, expected);
}
//#endregion 🧪️RetainedCatalogOracle

//#region 🔖️CommandSurface
/// ⚡️ One representative value per `RemodelingCommand` row, in declaration (= binary ordinal) order —
/// the permanent wire guard's fixture, carried over verbatim from the pre-migration
/// `remodeling_protocol` baseline (see this ticket's `🧪️wire-baseline-before.txt`).
fn every_command() -> Vec<RemodelingCommand> {
    vec![
        RemodelingCommand::RunReconstruction(run_reconstruction::RunReconstruction {}),
        RemodelingCommand::RetryStage(retry_stage::RetryStage { stage: "extracting-features".into() }),
        RemodelingCommand::RunStage(run_stage::RunStage { stage: "dense-stereo".into() }),
        RemodelingCommand::ImportFramePayload(import_frame_payload::ImportFramePayload { payload: "data:image/png;base64,abc".into(), name: "frame.png".into(), index: 0 }),
        RemodelingCommand::ImportVideoFramePayload(import_video_frame_payload::ImportVideoFramePayload { payload: "data:image/jpeg;base64,abc".into(), name: "clip.mp4".into(), index: 1, frame_index: 1, timestamp_ms: 33.3 }),
        RemodelingCommand::ImportVideoDone(import_video_done::ImportVideoDone { name: "clip.mp4".into(), duration_ms: 400.0, frame_count: 4, width: 24, height: 24, codec: "mjpeg".into() }),
        RemodelingCommand::ImportVideoBytesPayload(import_video_bytes_payload::ImportVideoBytesPayload { payload: "data:video/mp4;base64,abc".into(), name: "clip.mp4".into() }),
        RemodelingCommand::AddStream(add_stream::AddStream { name: "Stream".into(), kind: "video".into(), camera_id: "cam-0".into() }),
        RemodelingCommand::RemoveStream(remove_stream::RemoveStream { stream_id: "stream-1".into() }),
        RemodelingCommand::SetStreamSync(set_stream_sync::SetStreamSync { stream_id: "stream-1".into(), sync_offset_ms: 12.5 }),
        RemodelingCommand::EditCalibration(edit_calibration::EditCalibration {
            camera_id: "cam-1".into(),
            label: "Front".into(),
            model: "pinhole".into(),
            fx: 1000.0,
            fy: 1000.0,
            cx: 0.0,
            cy: 0.0,
            skew: 0.0,
            k1: 0.0,
            k2: 0.0,
            k3: 0.0,
            p1: 0.0,
            p2: 0.0,
            locked: false,
        }),
        RemodelingCommand::CalibrateCameras(calibrate_cameras::CalibrateCameras {}),
        RemodelingCommand::AddGcp(add_gcp::AddGcp { name: "GCP".into(), world_x: 0.0, world_y: 0.0, world_z: 0.0 }),
        RemodelingCommand::RemoveGcp(remove_gcp::RemoveGcp { gcp_id: "gcp-1".into() }),
        RemodelingCommand::PlaceGcpObservation(place_gcp_observation::PlaceGcpObservation { gcp_id: "gcp-1".into(), stream_id: "stream-1".into(), frame_index: 0, pixel_x: 10.0, pixel_y: 20.0 }),
        RemodelingCommand::SetIngestParams(set_ingest_params::SetIngestParams { frame_sample_stride: 5, max_frames: 200, downscale_long_edge_px: 1600, min_sharpness: 0.3 }),
        RemodelingCommand::SetFeatureParams(set_feature_params::SetFeatureParams { detector: "orb".into(), target_count: 4000, octaves: 4, edge_threshold: 10.0 }),
        RemodelingCommand::SetMatchParams(set_match_params::SetMatchParams { matcher: "brute-force".into(), ratio_test: 0.8, cross_check: true, sequential_window: 8, max_pairs_per_frame: 16, loop_closure: true }),
        RemodelingCommand::SetSfmParams(set_sfm_params::SetSfmParams { ransac_iterations: 1000, ransac_threshold_px: 2.0, min_track_length: 3, ba_max_iterations: 50, robust_loss: "huber".into(), huber_delta_px: 1.5 }),
        RemodelingCommand::SetDenseParams(set_dense_params::SetDenseParams { resolution: "medium".into(), window_radius_px: 3, min_view_consistency: 3, confidence_threshold: 0.5, max_points: 500_000 }),
        RemodelingCommand::SetMeshParams(set_mesh_params::SetMeshParams {
            tsdf_voxel_size_mm: 5.0,
            tsdf_truncation_mm: 20.0,
            decimate_target_triangles: 200_000,
            smoothing_iterations: 2,
            texture_enabled: true,
            texture_size: 2048,
            guarantee_watertight: true,
            hole_fill_max_boundary_verts: 512,
            self_intersection_check: false,
        }),
        RemodelingCommand::SetMotionParams(set_motion_params::SetMotionParams { enabled: false, max_tracks: 64, track_window_px: 21, min_track_quality: 0.3, min_track_length_frames: 5 }),
        RemodelingCommand::SetGeoParams(set_geo_params::SetGeoParams { enabled: false, origin_lon: None, origin_lat: Some(1.0), origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 }),
        RemodelingCommand::ResetPlaceholderMesh(reset_placeholder_mesh::ResetPlaceholderMesh {}),
        RemodelingCommand::ClearSparse(clear_sparse::ClearSparse {}),
        RemodelingCommand::ClearDense(clear_dense::ClearDense {}),
        RemodelingCommand::ClearMeshResult(clear_mesh_result::ClearMeshResult {}),
        RemodelingCommand::ClearTracks(clear_tracks::ClearTracks {}),
        RemodelingCommand::ClearGeoProducts(clear_geo_products::ClearGeoProducts {}),
        RemodelingCommand::ClearResult(clear_result::ClearResult {}),
        RemodelingCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::remodeling::config::RemodelingWorldCamera::default() }),
        RemodelingCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: "dense".into(), visible: false }),
        RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 }),
        RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: None, frame_index: 0 }),
        RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: "gcps".into() }),
        RemodelingCommand::ImportFrames(import_frames::ImportFrames {}),
        RemodelingCommand::ImportVideo(import_video::ImportVideo {}),
        RemodelingCommand::ExportQcReport(export_qc_report::ExportQcReport {}),
        RemodelingCommand::AdvanceReconstruction(advance_reconstruction::AdvanceReconstruction {
            generation: 1,
            job_id: "job-1".into(),
            requested_stage: "full".into(),
            phase: "pipeline".into(),
            stream_index: 0,
            frame_index: 1,
            terminal_cursor: 0,
            tick: 2,
        }),
        RemodelingCommand::CancelReconstruction(cancel_reconstruction::CancelReconstruction {}),
        RemodelingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }),
    ]
}

/// ⚖️ The exhaustiveness law `validate_tool_job_rows` enforces at runtime, asserted here so a new
/// `RemodelingCommand` row that forgets its classification, its proof or its publication contract
/// fails in `cargo test` rather than at app-construction time in the browser.
#[semio_framework_async_macros::async_test]
async fn retained_route_dispositions_are_exact_and_exhaustive() {
    let commands: std::collections::BTreeSet<&str> = every_command().iter().map(RemodelingCommand::command_id).collect();
    let retained: std::collections::BTreeSet<&str> = REMODELING_RETAINED_TOOL_IDS.iter().copied().collect();
    assert_eq!(retained.len(), REMODELING_RETAINED_TOOL_IDS.len(), "retained tool ids must be unique");
    assert_eq!(retained, commands, "every command row is a retained route and vice versa");

    assert_eq!(<RemodelingPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), REMODELING_RETAINED_TOOL_IDS.len());

    let contracts = <RemodelingRetainedCommandJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(contracts.len(), REMODELING_RETAINED_TOOL_IDS.len());
    assert_eq!(contracts.iter().map(|contract| contract.tool_id).collect::<std::collections::BTreeSet<_>>(), retained);
    for contract in contracts {
        assert_eq!(contract.lanes.len(), 1, "route '{}' declares exactly one publication lane", contract.tool_id);
    }

    assert!(<RemodelingPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the Artifact lane is rejected outright without a document one-item preparation factory");
    assert!(<RemodelingPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some(), "the Config lane is rejected outright without a config one-item preparation factory");

    // 🧭️ `try_build_definition` fans every declared action into every window kind, so the built
    // definition's per-window action lists are where a declaration is observable after the fact.
    let definition = create_remodeling_app();
    let declared: std::collections::BTreeMap<&str, InteractiveJobClassification> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|action| (action.id.as_str(), action.semantics.execution.interactive_job)).collect();
    for id in &retained {
        assert_eq!(declared.get(id), Some(&InteractiveJobClassification::Migrated), "action '{id}' must be declared and classified Migrated");
    }
}

/// ⚖️ Every row round trips through BOTH projections, and its printed line starts with the row's own
/// wire keyword — the guard that catches a missing `#[dsl(keyword = ..)]` on a payload struct, which
/// no round-trip law alone would notice.
#[semio_framework_async_macros::async_test]
async fn every_command_variant_roundtrips_and_prints_its_wire_keyword() {
    let keywords: Vec<&str> = vec![
        "run-reconstruction",
        "retry-stage",
        "run-stage",
        "import-frame-payload",
        "import-video-frame-payload",
        "import-video-done",
        "import-video-bytes-payload",
        "add-stream",
        "remove-stream",
        "stream-sync",
        "edit-calibration",
        "calibrate-cameras",
        "add-gcp",
        "remove-gcp",
        "place-gcp-observation",
        "ingest-params",
        "feature-params",
        "match-params",
        "sfm-params",
        "dense-params",
        "mesh-params",
        "motion-params",
        "geo-params",
        "reset-placeholder-mesh",
        "clear-sparse",
        "clear-dense",
        "clear-mesh-result",
        "clear-tracks",
        "clear-geo-products",
        "clear-result",
        "camera",
        "layer-visibility",
        "frame-cursor",
        "frame-cursor",
        "report-table",
        "active-utility",
        "locale",
        "import-frames",
        "import-video",
        "export-qc-report",
        "advance-reconstruction",
        "cancel-reconstruction",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), keywords.len(), "the keyword list must cover every row");
    for (command, keyword) in commands.iter().zip(keywords) {
        store::os_store::test_support::assert_op_text_binary_equivalence(command);
        assert!(command.print_op().starts_with(keyword), "row must print its wire keyword {keyword}, got {:?}", command.print_op());
    }
}

/// 📌️ Pinned hex for the rows whose `Option` fields make `None`/`Some` distinct wire cases, plus the
/// two fieldless-variant shapes. `SetFrameCursor`'s ordinal shifted 33→32 (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM deleted the `setSelection` row ahead of it) —
/// a legitimate wire break on this greenfield repo, not a bug. A reordered row or a changed field
/// order breaks these immediately.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &RemodelingCommand| command.encode_op().expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex(&RemodelingCommand::RunReconstruction(run_reconstruction::RunReconstruction {})), "01000000", "fieldless row 0");
    assert_eq!(hex(&RemodelingCommand::ClearResult(clear_result::ClearResult {})), "011d0000", "fieldless row 29");
    assert_eq!(hex(&RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: None, frame_index: 0 })), "01200001010400", "Option field absent");
    assert_eq!(hex(&RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 })), "0120010873747265616d2d3102000600010402", "Option field present");
    assert_eq!(
        hex(&RemodelingCommand::SetGeoParams(set_geo_params::SetGeoParams { enabled: false, origin_lon: None, origin_lat: Some(1.0), origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 })),
        "0116000600010205000000000000f03f0405000000a09999a93f0505000000a09999b93f0605000000000000004007048020",
        "three interleaved Option fields, only the middle one present"
    );
}

/// 🏷️ Every manifest action id and every wire keyword is distinct — the cross-cutting invariant
/// `app_commands!` exists to keep true (the fixture lists `setFrameCursor` twice on purpose, so the
/// row count, not the fixture length, is what must dedupe cleanly).
#[semio_framework_async_macros::async_test]
async fn command_ids_and_wire_keywords_are_unique_per_row() {
    let mut ids: Vec<&str> = every_command().iter().map(RemodelingCommand::command_id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 41, "41 distinct manifest action ids");

    let mut keywords: Vec<String> = every_command().iter().map(|command| command.print_op().split_whitespace().next().unwrap_or_default().to_string()).collect();
    keywords.sort();
    keywords.dedup();
    assert_eq!(keywords.len(), 41, "41 distinct wire keywords");
}
/// 🌉️ The action bridge covers every action the manifest declares (framework-injected ones aside)
/// and rejects anything else — the gap this migration closed (see `command_from_action`'s doc).
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    testkit::assert_declared_actions_bridge_to_commands::<EditorApp<RemodelingPlayApp>>(remodeling_app_manifest_for_testkit).await;
    assert!(RemodelingPlayApp::command_from_action("nonsense", None).is_err());
}

/// 🌉️ Select-typed args arrive as strings; numeric-option selects (`textureSize`) must still land in
/// a `u32` field, and a `setCamera` payload is accepted both flat and `{camera:{…}}`-nested.
#[semio_framework_async_macros::async_test]
async fn the_action_bridge_coerces_select_strings_and_both_camera_arg_shapes() {
    let mesh = RemodelingPlayApp::command_from_action("setMeshParams", Some(&dsl::DslValue::from(&serde_json::json!({ "textureSize": "4096" })))).expect("bridge");
    let RemodelingCommand::SetMeshParams(payload) = mesh else { panic!("expected SetMeshParams") };
    assert_eq!(payload.texture_size, 4096);

    let flat = RemodelingPlayApp::command_from_action("setCamera", Some(&dsl::DslValue::from(&serde_json::json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "fov": 60.0 })))).expect("bridge");
    let nested = RemodelingPlayApp::command_from_action("setCamera", Some(&dsl::DslValue::from(&serde_json::json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "fov": 60.0 } })))).expect("bridge");
    assert_eq!(flat, nested);

    let example = RemodelingPlayApp::command_from_action("setActiveExample", Some(&dsl::DslValue::from(&serde_json::json!({ "exampleId": "demo-session" })))).expect("bridge");
    assert_eq!(example, RemodelingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo-session".into() }));
}

//#endregion 🔖️CommandSurface

/// 🖼️ Render smoke test: every window/panel body key this app declares must render without panicking.
#[semio_framework_async_macros::async_test]
async fn render_does_not_panic_for_known_body_keys() {
    let mut app = app().await;
    for body_key in [
        model::windows::model::REMODELING_PLAY_BODY_MAIN,
        capture::windows::frames::REMODELING_PLAY_BODY_FRAMES,
        analyze::windows::report::REMODELING_PLAY_BODY_REPORT,
        media::REMODELING_PLAY_BODY_MEDIA,
        document::REMODELING_PLAY_BODY_PIPELINE,
        results::REMODELING_PLAY_BODY_RESULTS,
        parameters::REMODELING_PLAY_BODY_PARAMETERS,
        calibration_panel::REMODELING_PLAY_BODY_CALIBRATION,
        tracks::REMODELING_PLAY_BODY_TRACKS,
        quality::REMODELING_PLAY_BODY_QC,
    ] {
        let _ = render(&mut app, body_key).await;
    }
}

#[semio_framework_async_macros::async_test]
async fn render_unknown_body_key_reports_it_by_name() {
    let mut app = app().await;
    assert!(render(&mut app, "remodeling.play.nope").await.contains("Unknown body: remodeling.play.nope"));
}

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_declares_three_modes_three_windows_and_this_apps_panel_tabs() {
    let definition = create_remodeling_app();
    assert_eq!(definition.modes.len(), 3);
    assert_eq!(definition.window_kinds.len(), 3);
    for panel_id in
        [media::REMODELING_PANEL_MEDIA_ID, results::REMODELING_PANEL_RESULTS_ID, parameters::REMODELING_PANEL_PARAMETERS_ID, calibration_panel::REMODELING_PANEL_CALIBRATION_ID, tracks::REMODELING_PANEL_TRACKS_ID, quality::REMODELING_PANEL_QC_ID]
    {
        assert!(definition.panel_tabs.iter().any(|tab| tab.id() == panel_id), "panel tab {panel_id} must be present");
    }
}

#[semio_framework_async_macros::async_test]
async fn remodeling_io_declares_photos_in_and_mesh_out_on_the_manifest() {
    let definition = create_remodeling_app();
    assert!(definition.media_inputs.iter().any(|port| port.id == "photos:in"));
    assert!(definition.media_outputs.iter().any(|port| port.id == "mesh:out"));
}

/// 🧰️ The registry-backed app enforces View/Shell kind discipline — a view row must not slip
/// through as an operation.
#[semio_framework_async_macros::async_test]
async fn view_rows_dispatch_cleanly_against_the_real_registry() {
    let mut app = app_with_registry().await;
    let result = testkit::meta("local");
    app.dispatch_typed(RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: "tracks".into() }), &result).await.expect("view dispatch");
}
//#endregion 🔖️ManifestSanity

/// 🧪️ The definitional proof: two independent instances start from the same document, apply DISJOINT
/// field edits (A tunes feature params, B adds a ground control point), and exchanging operations
/// over a `MemoryBackbone` converges both sides to contain BOTH edits — impossible under a
/// whole-document `setDocument` snapshot, where one side's write would clobber the other's.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    testkit::assert_two_instances_converge::<EditorApp<RemodelingPlayApp>, _>(
        "mem://remodeling-convergence",
        RemodelingCommand::SetFeatureParams(set_feature_params::SetFeatureParams { detector: "akaze".into(), target_count: 1000, octaves: 4, edge_threshold: 10.0 }),
        RemodelingCommand::AddGcp(add_gcp::AddGcp { name: "corner".into(), world_x: 1.0, world_y: 2.0, world_z: 3.0 }),
        |app| {
            let projection = app.snapshot().expect("materialize projection");
            (projection.params.feature.detector, projection.gcps.first().map(|gcp| gcp.name.clone()))
        },
    )
    .await;
}

//#region 🔖️MediaPortTests
/// 🔌️ `photos:in` inserts an incoming photo as one new frame on the well-known workflow-photos
/// stream, creating it on the first import and appending on subsequent ones.
#[semio_framework_async_macros::async_test]
async fn import_media_photos_in_creates_and_appends_to_the_workflow_stream() {
    let app = app().await;
    let projection = app.snapshot().expect("projection");
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let media = Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        payload: MediaPayload::Structured {
            schema: "2d.image".into(),
            json: base64_codec::base64_standard_encode(crate::editor::remodeling::engine::images::encode_png(&crate::editor::remodeling::engine::images::ImageRgba8::new(4, 4)).expect("encode png")),
        },
    };
    let emit = RemodelingPlayApp::import_media("photos:in", &media, &doc).expect("photos:in import");
    assert_eq!(emit.artifact_mutations.len(), 2, "one create-asset + one create-stream");
    let next = emit.artifact_mutations.iter().fold(projection.clone(), |scene, operation| crate::op::apply_remodeling_mutation(&scene, operation).expect("valid mutation diff"));
    assert_eq!(next.streams.len(), 1);
    assert_eq!(next.streams[0].id, REMODELING_WORKFLOW_PHOTOS_STREAM_ID);
    assert_eq!(next.streams[0].frames.len(), 1);

    let history2 = HistoryView::empty();
    let doc2 = ArtifactView::new(&next, &history2);
    let emit2 = RemodelingPlayApp::import_media("photos:in", &media, &doc2).expect("second photos:in import");
    let next2 = emit2.artifact_mutations.iter().fold(next.clone(), |scene, operation| crate::op::apply_remodeling_mutation(&scene, operation).expect("valid mutation diff"));
    assert_eq!(next2.streams.len(), 1, "still one workflow-photos stream");
    assert_eq!(next2.streams[0].frames.len(), 2, "second import appends a second frame");
}

/// 🔌️ `mesh:out` exports the current reconstructed mesh as a GLB-encoded `3d.mesh` `Media`.
#[semio_framework_async_macros::async_test]
async fn export_media_mesh_out_exports_a_structured_3d_mesh() {
    let app = app().await;
    let projection = app.snapshot().expect("projection");
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let media = RemodelingPlayApp::export_media("mesh:out", &doc).expect("mesh:out export");
    assert_eq!(media.media_type.class, MediaClass::ThreeD);
    assert_eq!(media.media_type.form, MediaForm::Mesh);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "3d.mesh");
            assert!(!json.is_empty());
        }
        MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
    }
}
//#endregion 🔖️MediaPortTests

//#region 🔖️IoTests
/// 🧪️ Relocated from the artifact's `⚙️engine/🦀️.rs` (#2553): `remodeling_io()` returns
/// `AppIo` and lives app-side now, so its own declaration test travels with it.
#[semio_framework_async_macros::async_test]
async fn remodeling_io_declares_photos_in_and_mesh_out() {
    let io = remodeling_io();
    assert_eq!(io.document_schema, "remodeling.scene");
    assert_eq!(io.artifact.id, "3d.remodeling");
    let photos_in = io.ports.iter().find(|port| port.id == "photos:in").expect("photos:in declared");
    assert_eq!(photos_in.direction, MediaPortDirection::In);
    assert_eq!(photos_in.kind_id.as_deref(), Some("2d.image"));
    assert!(!photos_in.required);
    assert_eq!(photos_in.multiplicity, semio_framework::PortMultiplicity::Many);
    let mesh_out = io.ports.iter().find(|port| port.id == "mesh:out").expect("mesh:out declared");
    assert_eq!(mesh_out.direction, MediaPortDirection::Out);
    assert_eq!(mesh_out.kind_id.as_deref(), Some("3d.mesh"));
    assert!(!mesh_out.required);
    assert_eq!(mesh_out.multiplicity, semio_framework::PortMultiplicity::Many);
}
//#endregion 🔖️IoTests
