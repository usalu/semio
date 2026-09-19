pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry, settle_registered_typed_operation};
    use semio_framework_plugin::{App, Effect, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `RemodelingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<RemodelingPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<RemodelingPlayApp>` builds it. The newtype is self-closing: every
    /// registry-backed app owns Stores that must retire through the bounded close protocol before drop
    /// (`store drop witness`), so `Drop` runs `close_registered_fixture_app` for every test that returns
    /// early — a test that panics leaves the witness alone so the first failure stays the one reported.
    pub struct RemodelingApp(VcsArtifactApp<EditorApp<RemodelingPlayApp>>);

    impl std::ops::Deref for RemodelingApp {
        type Target = VcsArtifactApp<EditorApp<RemodelingPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for RemodelingApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for RemodelingApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                close_registered_fixture_app(&mut self.0);
            }
        }
    }

    /// ✏️ Adapts `create_remodeling_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `artifact_app_laws::assert_declared_actions_bridge_to_commands`/
    /// `context::new_app_with_registry` still expect — framework test context gap, not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub fn remodeling_app_manifest_for_tests() -> App {
        App { definition: create_remodeling_app(), examples: Vec::new() }
    }

    /// 🧪️ The ONE app shape every test uses: wired to the real manifest registry with a bound instance
    /// id, exactly what the plugin host mounts. A registry-less `new_app` cannot admit the bounded tool
    /// proofs any more (`interactive-job.catalog-authority … migrated={}`), so there is no "bare" variant.
    /// The RUNTIME side stays async (`ArtifactApp`/`VcsArtifactApp` are async traits, unlike the
    /// AUTHORING `ArtifactEditor` this crate implements), so every harness entry point awaits.
    pub async fn app() -> RemodelingApp {
        let mut app = new_app_with_registry::<EditorApp<RemodelingPlayApp>>(remodeling_app_manifest_for_tests).await;
        app.bind_instance_id(meta("local").instance_id).await;
        RemodelingApp(app)
    }

    /// 🧪️ Same as [`app`] — kept as the name the older tests spell.
    pub async fn app_with_registry() -> RemodelingApp {
        app().await
    }

    /// 🧾️ A settled dispatch: the immediate answer plus the store lanes the retained publication
    /// actually wrote (a mounted app publishes AFTER answering, so `result.mutations` is always empty —
    /// read `edited_document()`/`lanes` and the snapshot instead).
    pub struct Dispatched {
        pub result: InvocationResult,
        pub lanes: Vec<TypedOperationResultLane>,
    }

    impl Dispatched {
        /// 📝️ Whether the settled publication wrote the document lane.
        pub fn edited_document(&self) -> bool {
            self.lanes.contains(&TypedOperationResultLane::Artifact)
        }

        /// 👁️ Whether the settled publication wrote exactly one window-config lane and no document lane.
        pub fn edited_only_window_config(&self) -> bool {
            self.lanes.iter().filter(|lane| **lane == TypedOperationResultLane::WindowConfig).count() == 1 && !self.edited_document()
        }
    }

    impl std::ops::Deref for Dispatched {
        type Target = InvocationResult;
        fn deref(&self) -> &Self::Target {
            &self.result
        }
    }

    /// 🔁️ A mounted app answers before its retained typed operation has published: drive it home the
    /// way the plugin host's continuation does, fold the settled receipt's effects into the answer and
    /// apply any `LoadDocument` the way the host would. A faulted publication is a test failure.
    /// 🪟️ The one window instance every view verb dispatches from and every window body renders in —
    /// window configs are keyed per instance, so a `SetReportTable` dispatched here is only visible to a
    /// render addressed at the same instance.
    pub fn test_window(window_kind_id: &str) -> ViewModel {
        ViewModel {
            window_id: Some("remodeling-test-window".into()),
            window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "remodeling-test-window".into(), window_kind_id: window_kind_id.into() }],
            ..Default::default()
        }
    }

    pub async fn dispatch(app: &mut RemodelingApp, command: RemodelingCommand) -> Dispatched {
        let window_kind = match &command {
            RemodelingCommand::SetCamera(_) | RemodelingCommand::SetLayerVisibility(_) => Some(model::windows::model::REMODELING_PLAY_WINDOW_MAIN),
            RemodelingCommand::SetFrameCursor(_) => Some(capture::windows::frames::REMODELING_PLAY_WINDOW_FRAMES),
            RemodelingCommand::SetReportTable(_) => Some(analyze::windows::report::REMODELING_PLAY_WINDOW_REPORT),
            _ => None,
        };
        let view_state = window_kind.map(test_window);
        let command_id = command.command_id();
        let mut result = app.dispatch_typed(command, &semio_framework_plugin::ActionMeta { view_state, ..meta("local") }).await.unwrap_or_else(|fault| panic!("{command_id}: {fault:?}"));
        let settled = settle_registered_typed_operation(&mut app.0, meta("local").instance_id).await.unwrap_or_else(|fault| panic!("{command_id}: settle: {fault:?}"));
        result.requested_effects.extend(settled.effects);
        for effect in &result.requested_effects {
            if let Effect::LoadDocument { pack, spr } = effect {
                let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                app.load_document_pack(&files).await.expect("test host applies load-document effect");
            }
        }
        Dispatched { result, lanes: settled.lanes }
    }

    /// 🖼️ The rendered tree as its JSON projection text (every label, description and surface field) —
    /// `ComponentTree`'s own `Debug` prints a `BuiltChildren` as its LENGTH only, so a body assertion
    /// against it never sees a nested row (every windowed section's rows are nested).
    pub async fn render(app: &mut RemodelingApp, body_key: &str) -> String {
        render_json(app, body_key).await.to_string()
    }

    /// 🪟️ [`render`] addressed at the test window instance of `window_kind_id` — the render that sees the
    /// window config a view verb dispatched through [`dispatch`] wrote.
    pub async fn render_in_window(app: &mut RemodelingApp, body_key: &str, window_kind_id: &str) -> String {
        let tree = app.render(body_key, None, &test_window(window_kind_id)).await.unwrap_or_else(|fault| panic!("render {body_key}: {fault:?}"));
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error| panic!("project {body_key}: {error}"))
    }

    /// 🪧️ The rendered tree as its JSON projection, with the projected tree retired.
    pub async fn render_json(app: &mut RemodelingApp, body_key: &str) -> serde_json::Value {
        let tree = app.render(body_key, None, &ViewModel::default()).await.unwrap_or_else(|fault| panic!("render {body_key}: {fault:?}"));
        serde_json::from_str(&semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap_or_else(|error| panic!("project {body_key}: {error}"))).expect("projection parses")
    }

    /// 👥️ The reconstruction run as the framework ledger summarizes it.
    pub fn run_presence(app: &RemodelingApp) -> Option<protocol::PresenceToolRun> {
        app.tool_run_presence()
    }

    /// ⏯️ Dispatches one framework tool-run action and answers its output.
    pub async fn run_action(app: &mut RemodelingApp, action: &str, arguments: Vec<(String, semio_framework_plugin::DslValue)>) -> semio_framework_plugin::DslValue {
        app.handle_action(action, Some(&semio_framework_plugin::DslValue::Object(arguments)), &meta("local")).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}")).output
    }

    /// ▶️ Starts the reconstruction tool run.
    pub async fn start_reconstruction(app: &mut RemodelingApp) {
        let output = run_action(app, semio_framework_plugin::TOOL_RUN_START_ACTION_ID, vec![("toolId".into(), semio_framework_plugin::DslValue::String(model::tools::reconstruction::TOOL_ID.into()))]).await;
        assert_eq!(output.get("toolRun").and_then(semio_framework_plugin::DslValue::as_str), Some("spawnJob"), "the framework starts the reconstruction run");
    }

    /// 🪪️ The `{runId, generation}` the framework run panel's buttons carry now.
    pub async fn run_arguments(app: &mut RemodelingApp) -> Vec<(String, semio_framework_plugin::DslValue)> {
        fn find(value: &serde_json::Value) -> Option<(String, u64)> {
            if let Some(object) = value.as_object() {
                if let (Some(run), Some(generation)) = (object.get("runId"), object.get("generation")) {
                    let run = run.as_str().map(str::to_string).or_else(|| run.get("text").and_then(serde_json::Value::as_str).map(str::to_string)).or_else(|| run.as_u64().map(|value| value.to_string()))?;
                    let generation = generation.as_u64().or_else(|| generation.as_f64().map(|value| value as u64)).or_else(|| generation.get("number").and_then(serde_json::Value::as_f64).map(|value| value as u64))?;
                    return Some((run, generation));
                }
                return object.values().find_map(find);
            }
            value.as_array()?.iter().find_map(find)
        }
        let panel = render_json(app, semio_framework_plugin::FRAMEWORK_TOOL_RUN_BODY_KEY).await;
        let (run, generation) = find(&panel).unwrap_or_else(|| panic!("the run panel carries run arguments: {panel}"));
        vec![("runId".into(), semio_framework_plugin::DslValue::String(run)), ("generation".into(), semio_framework_plugin::DslValue::String(generation.to_string()))]
    }

    /// 🖥️ One plugin-host turn, exactly as the host drives it: one maintenance grant, one publication
    /// turn, every presented result page acknowledged, every outbox drained. Every tool-run action the
    /// run hands the shell is dispatched back, the way the shell answers its own port.
    pub async fn host_turn(app: &mut RemodelingApp) {
        let meta = meta("local");
        app.maintenance_step(1, semio_framework_os_kernel::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance grant");
        app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("driver turn faulted: {fault:?}"));
        while let Some(page) = app.take_typed_operation_result_page(meta.instance_id) {
            assert!(page.lane != TypedOperationResultLane::Fault, "typed operation fault: {}", String::from_utf8_lossy(page.bytes()));
            assert!(app.acknowledge_typed_operation_result(page.token).expect("result ACK"), "the exact result ACK is admitted");
        }
        let mut effects = app.pending_effects(None).await;
        while let Some(effect) = app.take_typed_operation_effect() {
            effects.push(effect);
        }
        while app.take_typed_operation_event().is_some() {}
        while app.take_typed_operation_ui_scope().is_some() {}
        while app.take_typed_operation_completion().await.expect("completion").is_some() {}
        while app.take_local_interaction_query_reply().is_some() {}
        for effect in effects {
            if let Effect::DispatchAction { action, args, .. } = effect {
                if semio_framework_plugin::is_tool_run_action_id(&action) {
                    Box::pin(app.handle_action(&action, args.as_ref(), &meta)).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));
                }
            }
        }
    }

    /// 📥️ Host turns until no typed operation is pending, bounded so a wedged operation fails loudly.
    pub async fn settle(app: &mut RemodelingApp, what: &str) {
        for _ in 0..100_000 {
            if !app.has_pending_typed_operations() {
                return;
            }
            host_turn(app).await;
        }
        panic!("{what} never retired its typed operations");
    }

    /// 🔁️ Host turns until the run satisfies `done` and rests outside a transitional state.
    pub async fn pump_run(app: &mut RemodelingApp, what: &str, done: impl Fn(&protocol::PresenceToolRun) -> bool) -> protocol::PresenceToolRun {
        for _ in 0..50_000_000u64 {
            if let Some(run) = run_presence(app).filter(|run| done(run)) {
                if !app.has_pending_typed_operations() || !matches!(run.state.wire_name(), "starting" | "finalizing" | "aborting") {
                    return run;
                }
            }
            host_turn(app).await;
        }
        panic!("{what} never settled; run {:?}", run_presence(app));
    }

    /// 🚪️ Closes a registry-backed fixture app through its bounded close protocol — `Drop` does the same,
    /// this only names the moment a test closes on purpose.
    pub fn close(app: RemodelingApp) {
        drop(app);
    }

    /// 🧾️ Everything durable a run may only change by finalizing: document pack and history.
    pub async fn durable(app: &mut RemodelingApp) -> (Vec<u8>, Vec<u8>, String) {
        let pack = app.document_pack().await.expect("document pack");
        (pack.pack, pack.spr, format!("{:?}", app.history_snapshot().await.expect("history")))
    }
}

use super::*;
use crate::editor::remodeling::unit_tests::context::{app, app_with_registry, remodeling_app_manifest_for_tests, render};
use protocol::{OpBinary, OpText};
use semio_framework_plugin::artifact_app_laws;
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
            .filter(|contract| contract.get("lanes").and_then(serde_json::Value::as_array).is_some_and(|lanes| lanes.as_slice() == [serde_json::Value::String("HostOnly".into())]))
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
    assert_eq!(oracle, RemodelingRetainedCatalogSummary { routes: 35, bounded: 35, resumable: 0, unique: true, bounded_ids: bounded_owned, host_only_ids: expected_host_only });
    assert_eq!(subject, oracle);
}

#[semio_framework_async_macros::async_test]
async fn retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures() {
    let fixture = include_str!("../../🧫️fixtures/🚧️retained-command-limits/🔣️.json");
    let expected = ["exportQcReport", "importFrames", "importVideo"].iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
    let wrong_lane = fixture.replacen("\"HostOnly\"", "\"Artifact\"", 1);
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
        RemodelingCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::remodeling::modes::model::windows::model::config::RemodelingModelWindowConfig::default().camera }),
        RemodelingCommand::SetLayerVisibility(set_layer_visibility::SetLayerVisibility { layer: "dense".into(), visible: false }),
        RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 }),
        RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: None, frame_index: 0 }),
        RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: "gcps".into() }),
        RemodelingCommand::ImportFrames(import_frames::ImportFrames {}),
        RemodelingCommand::ImportVideo(import_video::ImportVideo {}),
        RemodelingCommand::ExportQcReport(export_qc_report::ExportQcReport {}),
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
    // 👁️ No route declares the `Config` lane (the four view verbs publish to their exact `WindowConfig`
    // owners, and `Config` is `NoConfig`), so no config one-item preparation factory is owed.
    assert!(!contracts.iter().any(|contract| contract.lanes.contains(&semio_framework_plugin::ArtifactToolPublicationLane::Config)), "a Config-lane route would need a config one-item preparation factory");

    // 🧭️ `try_build_definition` fans every declared action into every window kind, so the built
    // definition's per-window action lists are where a declaration is observable after the fact.
    let definition = create_remodeling_app();
    let declared: std::collections::BTreeMap<&str, InteractiveJobClassification> = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).map(|action| (action.id.as_str(), action.semantics.execution.interactive_job)).collect();
    for id in &retained {
        assert_eq!(declared.get(id), Some(&InteractiveJobClassification::Migrated), "action '{id}' must be declared and classified Migrated");
    }
}

#[semio_framework_async_macros::async_test]
async fn remodel_window_ownership_one_item_preparation_transfers_its_candidate_once() {
    let envelope = store::create_document_envelope(REMODELING_DOCUMENT_SCHEMA, "remodel-window-preparation-law", RemodelingSnapshot::default(), None);
    let mut document = store::ArtifactStore::new(envelope).await.expect("Remodel window preparation-law Store opens");
    document.install_document_store_owners_exact(
        <RemodelingPlayApp as ArtifactEditor>::build_document_store_owners().expect("Remodel document Store owners"),
    );
    let factory = <RemodelingPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("Remodel document one-item preparation factory");
    let mutation = RemodelingMutation::ReplaceQc(crate::mutations::replace_qc::ReplaceQc { qc: None });
    let mut publication = document
        .begin_apply_batch(
            semio_framework_job::OperationId(992),
            document.generation_now(),
            document.content_revision_now(),
            "remodel-window-preparation-law".into(),
            vec![mutation],
            Some("candidate transfer".into()),
            store::HistoryLane::Document,
            Some(&factory),
        )
        .expect("Remodel one-item publication admits its exact factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES };
    let mut published = false;
    for _ in 0..64 {
        match document.advance_apply_batch(&mut publication, grant).expect("Remodel candidate transfer turn") {
            store::ArtifactStoreOneItemAdvance::Published(_) => {
                published = true;
                break;
            }
            store::ArtifactStoreOneItemAdvance::Blocked => panic!("Remodel one-item preparation blocked after admitting its exact candidate"),
            store::ArtifactStoreOneItemAdvance::Progress(_) => {}
            store::ArtifactStoreOneItemAdvance::AwaitingAck(_) => panic!("Remodel one-item preparation reached ACK without exposing its exact publication receipt"),
            store::ArtifactStoreOneItemAdvance::Complete => panic!("Remodel one-item preparation completed before publishing its exact candidate"),
        }
    }
    assert!(published, "Remodel one-item preparation must transfer its candidate exactly once");
    assert!(publication.acknowledge());
    for _ in 0..64 {
        if matches!(publication.close_step(grant).expect("Remodel publication close"), store::SnapshotRetirementStep::Complete) {
            break;
        }
    }
    assert!(publication.terminal_is_empty());
    drop(publication);
    let mut disposer = semio_framework_plugin::ArtifactDocumentStoreDisposer::<RemodelingSnapshot, RemodelingMutation>::new();
    for _ in 0..100_000 {
        if matches!(
            semio_framework_plugin::ArtifactOwnedDisposer::close_step(&mut disposer, &mut document, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)
                .expect("Remodel preparation-law Store close"),
            semio_framework_plugin::PluginCloseStep::Complete
        ) {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactOwnedDisposer::terminal_is_empty(&disposer, &document));
}

/// ⚖️ Every row round trips through BOTH projections, and its printed line starts with the row's own
/// wire keyword — the guard that catches a missing `#[dsl(keyword = ..)]` on a payload struct, which
/// no round-trip law alone would notice.
#[semio_framework_async_macros::async_test]
async fn every_command_variant_roundtrips_and_prints_its_wire_keyword() {
    let keywords: Vec<&str> = vec![
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
        "import-frames",
        "import-video",
        "export-qc-report",
        "active-example",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), keywords.len(), "the keyword list must cover every row");
    for (command, keyword) in commands.iter().zip(keywords) {
        store::os_store::test_support::assert_op_text_binary_equivalence(command);
        assert!(command.print_op().starts_with(keyword), "row must print its wire keyword {keyword}, got {:?}", command.print_op());
    }
}

/// 📌️ Pinned hex for the rows whose `Option` fields make `None`/`Some` distinct wire cases, plus the
/// two fieldless-variant shapes. The reconstruction run verbs left the command channel for the framework
/// tool run (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS), shifting every later ordinal by three —
/// a legitimate wire break on this greenfield repo, not a bug. A reordered row or a changed field
/// order breaks these immediately.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let hex = |command: &RemodelingCommand| command.encode_op().expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex(&RemodelingCommand::CalibrateCameras(calibrate_cameras::CalibrateCameras {})), "01080000", "fieldless row 8");
    assert_eq!(hex(&RemodelingCommand::ClearResult(clear_result::ClearResult {})), "011a0000", "fieldless row 26");
    assert_eq!(hex(&RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: None, frame_index: 0 })), "011d0001010400", "Option field absent");
    assert_eq!(hex(&RemodelingCommand::SetFrameCursor(set_frame_cursor::SetFrameCursor { stream_id: Some("stream-1".into()), frame_index: 2 })), "011d010873747265616d2d3102000600010402", "Option field present");
    assert_eq!(
        hex(&RemodelingCommand::SetGeoParams(set_geo_params::SetGeoParams { enabled: false, origin_lon: None, origin_lat: Some(1.0), origin_alt: None, gsd_m: 0.05, dsm_cell_m: 0.1, dtm_filter_radius_m: 2.0, ortho_max_px: 4096 })),
        "0113000600010205000000000000f03f0405000000a09999a93f0505000000a09999b93f0605000000000000004007048020",
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
    assert_eq!(ids.len(), 35, "35 distinct manifest action ids");

    let mut keywords: Vec<String> = every_command().iter().map(|command| command.print_op().split_whitespace().next().unwrap_or_default().to_string()).collect();
    keywords.sort();
    keywords.dedup();
    assert_eq!(keywords.len(), 35, "35 distinct wire keywords");
}
/// 🌉️ The action bridge covers every action the manifest declares (framework-injected ones aside)
/// and rejects anything else — the gap this migration closed (see `command_from_action`'s doc).
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<RemodelingPlayApp>>(remodeling_app_manifest_for_tests).await;
    assert!(RemodelingPlayApp::command_from_action("nonsense", None).is_err());
}

/// 🌉️ Select-typed args arrive as strings; numeric-option selects (`textureSize`) must still land in
/// a `u32` field, and a `setCamera` payload is accepted both flat and `{camera:{…}}`-nested.
#[semio_framework_async_macros::async_test]
async fn the_action_bridge_coerces_select_strings_and_both_camera_arg_shapes() {
    let mesh = RemodelingPlayApp::command_from_action("setMeshParams", Some(&dsl::DslValue::from(&serde_json::json!({ "textureSize": "4096" })))).expect("bridge");
    let RemodelingCommand::SetMeshParams(payload) = mesh else { panic!("expected SetMeshParams") };
    assert_eq!(payload.texture_size, 4096);

    let flat = RemodelingPlayApp::command_from_action("setCamera", Some(&dsl::DslValue::from(&serde_json::json!({ "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 1.25 })))).expect("bridge");
    let nested = RemodelingPlayApp::command_from_action("setCamera", Some(&dsl::DslValue::from(&serde_json::json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 1.25 } })))).expect("bridge");
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
    let view = semio_framework_plugin::ViewModel {
        window_id: Some("report-test".into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "report-test".into(), window_kind_id: analyze::windows::report::REMODELING_PLAY_WINDOW_REPORT.into() }],
        ..Default::default()
    };
    let result = semio_framework_plugin::ActionMeta { view_state: Some(view), ..artifact_app_laws::meta("local") };
    app.dispatch_typed(RemodelingCommand::SetReportTable(set_report_table::SetReportTable { table: "tracks".into() }), &result).await.expect("view dispatch");
    let receipt = artifact_app_laws::settle_registered_typed_operation(&mut *app, artifact_app_laws::meta("local").instance_id).await.expect("view publication settles");
    assert!(receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::WindowConfig) && !receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Artifact), "a view row publishes its window-config lane and never the document lane: {:?}", receipt.lanes);
}
//#endregion 🔖️ManifestSanity

/// 🧪️ The definitional proof: two independent instances start from the same document, apply DISJOINT
/// field edits (A tunes feature params, B adds a ground control point), and exchanging operations
/// over a `MemoryBackbone` converges both sides to contain BOTH edits — impossible under a
/// whole-document `setDocument` snapshot, where one side's write would clobber the other's.
#[semio_framework_async_macros::async_test]
#[ignore = "framework gap: paired_registered_apps refuses attach_backbone (remote snapshot merge is fail-closed)"]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    // 🧹️ The REGISTERED pair: remodel publishes bounded tool proofs, so a registry-less `paired_apps`
    // instance faults in the `interactive-job.catalog-authority` proof join before any edit lands.
    artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<RemodelingPlayApp>, _, _, _>(
        "mem://remodeling-convergence",
        || async { remodeling_app_manifest_for_tests() },
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
    assert_eq!(io.artifact_schema, "remodeling.scene");
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
