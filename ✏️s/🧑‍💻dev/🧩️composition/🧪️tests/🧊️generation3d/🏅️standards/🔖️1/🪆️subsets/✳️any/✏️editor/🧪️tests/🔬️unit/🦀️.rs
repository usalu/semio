use semio_framework_test_fixtures::repo_text;

pub(crate) fn keyboard_reachability_fixture_json() -> &'static str { repo_text!("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/⌨️keyboard-reachability.json") }
use semio_s_artifact_procedural_3d_mesh_bridge::generation3d_document_from_mesh;
use semio_s_artifact_procedural_3d_mesh_bridge::generation3d_mesh_from_document;
use semio_s_artifact_procedural_3d_mesh_bridge::merge_meshes;

pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry, settle_registered_typed_operation, TypedOperationFixtureReceipt};
    use semio_framework_plugin::{ActionMeta, EditorApp, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    /// ✏️ `Generation3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Generation3dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Generation3dPlayApp>` builds it.
    pub type Generation3dApp = VcsArtifactApp<EditorApp<Generation3dPlayApp>>;
    
    /// ✏️ Adapts `create_generation3d_app`'s `AppDefinition` (contract §2.4) into the
    /// `App { definition, examples }` shape `context::assert_declared_actions_bridge_to_commands` /
    /// `context::new_app_with_registry` still expect — framework test context gap, not modifiable here
    /// (`🧰️framework/**` is outside this packet's lease).
    pub fn generation3d_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_generation3d_app(), examples: Vec::new() }
    }
    
    /// 🧹️ A live app fixture that CLOSES itself. `VcsArtifactApp`'s `ArtifactStore` owns an
    /// `ArtifactStoreCursorDisposer` whose `Drop` asserts terminal-empty ownership, and the document
    /// snapshot behind it owns `OrderedMap` roots that reject a bare drop — so a plainly-dropped fixture
    /// double-panics and aborts the whole test binary. Dereferences to the app, and drains the exact
    /// retained close ladder (`PluginApp::close_step`) on the way out, which is the same law the runtime
    /// uses (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub struct Generation3dAppFixture(Generation3dApp);
    
    impl std::ops::Deref for Generation3dAppFixture {
        type Target = Generation3dApp;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl std::ops::DerefMut for Generation3dAppFixture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    
    /// 🔎️ Names the authority a stalled close ladder was last waiting on, the way the framework's own
    /// `artifact_app_laws::close_registered_fixture_app` does. A bare `is_err()`/`break` reported only
    /// that the fixture never reached terminal-empty, which is the one thing that is never the cause —
    /// every real diagnosis of this family started by recovering this string
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub(crate) fn close_ladder_witness<A: PluginApp>(app: &mut A) -> String {
        let mut witness = String::new();
        for _ in 0..1_000_000 {
            if app.close_terminal_is_empty() {
                return String::new();
            }
            match app.close_step(1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
                Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { reason } | semio_framework_plugin::PluginCloseStep::Blocked { reason }) => witness = format!(", last pending close authority: {reason}"),
                Ok(_) => {}
                Err(fault) => return format!(", close ladder faulted: {fault:?}"),
            }
        }
        witness
    }

    impl Drop for Generation3dAppFixture {
        fn drop(&mut self) {
            let witness = close_ladder_witness(&mut self.0);
            assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "Generation3d app fixture did not reach its terminal-empty close witness{witness}");
        }
    }
    
    /// 🧪️ The ONE app host_snapshot. This app publishes `bounded_first_step_tool_proofs!` factories, so the
    /// registryless `context::new_app` cannot satisfy the framework's tool-proof catalog and faults with
    /// `interactive-job.catalog-authority` — which then aborts the whole process, because the unwind
    /// runs `ArtifactStoreCursorDisposer`'s Drop before its terminal-empty close. Every fixture
    /// therefore goes through the registry variant (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn app() -> Generation3dAppFixture {
        app_with_registry().await
    }
    
    pub async fn app_with_registry() -> Generation3dAppFixture {
        let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
        app.bind_instance_id(1).await;
        Generation3dAppFixture(app)
    }
    
    /// 📸️ Reads the live projection into a self-retiring [`SnapshotRead`] — never
    /// `app.snapshot()` directly, whose owned `Generation3dSnapshot` aborts the binary on a bare drop.
    pub fn snapshot(app: &Generation3dApp) -> crate::SnapshotRead {
        crate::SnapshotRead::new(app.snapshot().expect("snapshot"))
    }
    
    /// 🎯️ Dispatches a typed command AND completes its bounded host publication protocol. Every one of
    /// this app's 28 tools is a RETAINED job (`GENERATION3D_RETAINED_TOOL_IDS`), so `dispatch_typed`
    /// only returns an admission receipt — `{"operationId","generation"}` — and the mutation does not
    /// reach the store until the host advances maintenance/publication turns and acknowledges the
    /// result page. Reading `snapshot(&app)` straight after a bare `dispatch_typed` therefore observes
    /// the PRE-command document, which is why a whole class of editor-command tests passed vacuously
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn dispatch(app: &mut Generation3dApp, command: Generation3dCommand) -> TypedOperationFixtureReceipt {
        dispatch_with_view_meta(app, command, meta("local")).await.expect("dispatch")
    }
    
    /// 🎯️ Selects `ids` in the framework-owned `graph` interaction domain AND settles the reserved tool
    /// job the dispatch only ADMITS. `interactionSelect` is a `FrameworkInteractionSelectJob`
    /// (`🔌️plugin/🦀️.rs`), so a bare `handle_action` returns an admission receipt and nothing has
    /// touched `protocol::InteractionState` yet — a caller that reads `interaction_state()` or dispatches
    /// a selection-fed command straight afterwards observes the PRE-selection state and its law passes
    /// vacuously (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, the same admission-vs-publication trap
    /// [`dispatch`] documents for typed commands).
    pub async fn select_graph(app: &mut Generation3dApp, granularity: &str, ids: &[&str]) -> semio_framework_plugin::InvocationResult {
        select_domain(app, semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_INTERACTION_DOMAIN, granularity, ids).await
    }

    pub async fn select_domain(app: &mut Generation3dApp, domain: &str, granularity: &str, ids: &[&str]) -> semio_framework_plugin::InvocationResult {
        let targets: Vec<semio_framework_plugin::InteractionTarget> = ids.iter().map(|id| semio_framework_plugin::InteractionTarget { granularity: granularity.into(), id: (*id).into() }).collect();
        let targets_json = serde_json::to_string(&targets).expect("selection targets");
        let args: semio_framework_value::DslValue = serde_json::json!({ "domainId": domain, "targets": targets_json, "merge": "replace", "method": "pick" }).into();
        let admitted = app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("interaction selection admitted");
        semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.expect("interaction selection settles its reserved tool job")
    }
    
    pub fn preview_views(left: &str, right: &str) -> (ViewModel, ViewModel) {
        let roster = vec![
            ViewWindowInstance { id: left.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
            ViewWindowInstance { id: right.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
        ];
        let view = ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        (view.for_window_instance(left).expect("left Generation3d preview window"), view.for_window_instance(right).expect("right Generation3d preview window"))
    }
    
    pub async fn dispatch_with_view(app: &mut Generation3dApp, command: Generation3dCommand, view_state: ViewModel) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin::Fault> {
        dispatch_with_view_meta(app, command, ActionMeta { view_state: Some(view_state), ..meta("local") }).await
    }

    /// 🎚️ One action exactly as a host sends it — named args, the window's own view — settled through the registered
    /// ladder: the door a continuous control's press (`gesture`/`commit`/`abort`) takes into the framework scrub machine.
    pub async fn act_with_view(app: &mut Generation3dApp, action: &str, args: serde_json::Value, view_state: ViewModel) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin::Fault> {
        let action_meta = ActionMeta { view_state: Some(view_state), ..meta("local") };
        let args: semio_framework_value::DslValue = args.into();
        app.handle_action(action, Some(&args), &action_meta).await?;
        settle_registered_typed_operation(app, action_meta.instance_id).await
    }
    
    /// 🔁️ Completes whatever retained publication is already in flight — the settle half of
    /// [`dispatch`], for a caller that entered through `PluginApp::handle_action` instead.
    pub async fn settle(app: &mut Generation3dApp) -> TypedOperationFixtureReceipt {
        settle_registered_typed_operation(app, meta("local").instance_id).await.expect("retained publication")
    }
    
    async fn dispatch_with_view_meta(app: &mut Generation3dApp, command: Generation3dCommand, action_meta: ActionMeta) -> Result<TypedOperationFixtureReceipt, semio_framework_plugin::Fault> {
        app.dispatch_typed(command, &action_meta).await?;
        settle_registered_typed_operation(app, action_meta.instance_id).await
    }
    
    pub async fn render(app: &mut Generation3dApp, body_key: &str) -> String {
        let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
        render_with_view(app, body_key, &view).await
    }
    
    pub async fn render_with_view(app: &mut Generation3dApp, body_key: &str, view_state: &ViewModel) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")
    }
    
    /// 🧵️ The `previewEval` run hands its hops to the host through `requestedEffects` and the typed
    /// effect outbox, which only the JS renderer drains in production — a test has to do that draining
    /// itself. Its hops declare their geometry work through `Emit::extension_invocations`, which only
    /// the SHELL answers in production. Both halves are driven by [`drive_preview_run`], so the run
    /// converges on a REAL preview instead of on an empty one (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn drain_flow_eval_ticks(app: &mut Generation3dApp) {
        let (view, _) = preview_views("procedural-preview-test", "procedural-preview-test-other");
        drain_flow_eval_ticks_with_view(app, &view).await;
    }

    pub async fn drain_flow_eval_ticks_with_view(app: &mut Generation3dApp, view: &ViewModel) {
        drive_preview_run(app, view, &[]).await;
    }
    
    /// 🏛️ The SHELL's own roster: the flow window is current (that is the window a served boot focuses,
    /// and the window an unaddressed `Effect::DispatchAction` would be redispatched under), with the
    /// preview window attached alongside it. Every window-scoped route has to find its own window in
    /// here rather than assume it is the current one.
    pub fn shell_views(flow: &str, preview: &str) -> (ViewModel, ViewModel) {
        let roster = vec![
            ViewWindowInstance { id: flow.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::flow::GENERATION_3D_PLAY_WINDOW_MAIN.into() },
            ViewWindowInstance { id: preview.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() },
        ];
        let view = ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        (view.for_window_instance(flow).expect("flow window instance"), view.for_window_instance(preview).expect("preview window instance"))
    }
    
    /// 🔁️ The REAL served chain, end to end: nothing is hand-addressed here. `pending_effects` arms the
    /// first tick off the host's attached-window roster, and every following tick is the `action`+`args`
    /// of an `Effect::DispatchAction` the app itself emitted, replayed through `PluginApp::handle_action`
    /// under the SHELL's current window — the flow window — exactly the way `ShellHost` feeds
    /// `requestedEffects` back. So the tick's window address, its wire decode and the retained route's
    /// own preflight are all exercised, which a hand-built preview-addressed command skips entirely
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// 🏛️ Redispatches one armed `Effect::DispatchAction` exactly the way `makeEffectDispatchOne`
    /// (`🛠️ShellHelpers/🟦️.tsx`) does: an action id the app declares as a COMMAND re-enters the typed
    /// command channel with the shell's own live view attached, never the scoped action channel — which
    /// is why `handle_action("flowEvalTick", …)` is answered with `interactive-job.unknown-key`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub async fn dispatch_effect_command(app: &mut Generation3dApp, command_id: &str, args: Option<&semio_framework_value::DslValue>, action_meta: &ActionMeta) -> Result<(), semio_framework_plugin::Fault> {
        crate::brep_extension::dispatch_effect_command(app, command_id, args, action_meta).await
    }
    
    pub async fn drain_armed_flow_eval_ticks(app: &mut Generation3dApp, shell_view: &ViewModel) -> usize {
        drive_preview_run(app, shell_view, &[]).await.hops
    }

    /// 🔁️ The same drive, started from effects some OTHER route already handed back — the shape a law
    /// needs when the claim under test is what a route owes the run.
    pub async fn drain_armed_flow_eval_ticks_from(app: &mut Generation3dApp, shell_view: &ViewModel, initial: &[Effect]) -> usize {
        drive_preview_run(app, shell_view, initial).await.hops
    }

    pub use crate::brep_extension::{run_actions, PreviewRunReceipt};

    /// 🚦️ The framework run actions a refresh over `shell_view` owes the `previewEval` run right now.
    pub async fn owed_run_actions(app: &mut Generation3dApp, shell_view: &ViewModel) -> Vec<String> {
        crate::brep_extension::owed_run_actions(app, shell_view).await
    }

    /// 🔁️ The served `previewEval` loop on this surface, answered by the in-process kernel
    /// ([`crate::brep_extension::drive_preview_run`]).
    pub async fn drive_preview_run(app: &mut Generation3dApp, shell_view: &ViewModel, initial: &[Effect]) -> PreviewRunReceipt {
        crate::brep_extension::drive_preview_run(app, shell_view, initial, &mut crate::brep_extension::serve).await
    }

    /// 🔁️ [`drive_preview_run`] with the host's extension answers served by `serve`.
    pub async fn drive_preview_run_serving(app: &mut Generation3dApp, shell_view: &ViewModel, initial: &[Effect], serve: &mut dyn FnMut(&semio_framework_plugin::artifact_app_laws::PendingExtensionInvocation) -> Result<Vec<u8>, semio_framework_plugin::Fault>) -> PreviewRunReceipt {
        crate::brep_extension::drive_preview_run(app, shell_view, initial, serve).await
    }

    /// 🧳️ A standalone retained-operation owner handle of THIS app's concrete type, for a law that
    /// drives one piece of retained work directly instead of through a live instance. Its session is a
    /// real one, so anything the work arms lands on a real latch.
    pub fn instance_operation_owner() -> semio_framework_plugin::ArtifactInstanceOperationOwnerHandle {
        semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(<Generation3dPlayApp as semio_framework_plugin::ArtifactEditor>::build_instance_operation_owner())
    }
    
    /// 🧹️ Walks a test context-built owner across the same close boundary `VcsArtifactApp` runs — its
    /// `FlowEvalSession` rejects a live drop, so a law that builds one owns its retirement.
    pub fn retire_instance_operation_owner(handle: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) {
        use semio_framework_plugin::ArtifactInstanceOperationOwner;
        for _ in 0..1_000_000 {
            let complete = handle
                .with_mut::<semio_s_artifact_procedural_3d_editor::Generation3dInstanceOperationOwner, _>(|owner| Ok(matches!(ArtifactInstanceOperationOwner::close_step(owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete))))
                .expect("the test context owner lends itself to its own close ladder");
            if complete {
                return;
            }
        }
        panic!("the test context instance operation owner did not reach terminal-empty under a positive close grant");
    }
    
    /// 🧹️ `FlowEvalSession` rejects a live drop (`🌊️flow/🖥️host/🦀️.rs`'s `Drop` +
    /// `live_session_drop_is_rejected_without_recursive_payload_destruction`), so a test that owns one
    /// must walk it across the close boundary itself — the same `begin_close` + granted `close_step`
    /// loop `FlowInstanceOperationOwner::maintenance_step` runs in production.
    pub use crate::flow_operators::retire_flow_eval_session;
    
    /// 📜️ The empty `HistoryView` a command-handler unit test hands `ArtifactView::new` — built here once
    /// because `HistoryView` (`🧰️framework/…/🔌️plugin/🦀️.rs`) derives no `Default`.
    pub fn empty_history_view() -> semio_framework_plugin::HistoryView {
        semio_framework_plugin::HistoryView {
            columns: Vec::new(),
            can_undo: false,
            can_redo: false,
            active_alternative_id: None, alternatives: Vec::new(),
            current_checkpoint_id: None,
            commands: Vec::new(),
            command_filter: semio_framework_plugin::app::HistoryCommandFilter::default(),
        }
    }
    
    /// 🧩️ The host's `contributionsJson` for the generation3d closure — surface-neutral, so it lives
    /// in `🧪️tests/🔬️flow-operators` beside the two manifests it is built from and both surfaces' laws
    /// push the identical payload. Re-exported here because every editor law already names it.
    pub use crate::flow_operators::staged_flow_extension_contributions_json;
    
    /// 🪪️ A contributed `flow.extension` manifest that carries no operators at all — the witness a
    /// delivery law adds to the payload so that "the registry now holds what the host pushed" is a claim
    /// about THIS run, never about a manifest some earlier `install_flow_extension_manifest` left behind.
    pub const CONTRIBUTIONS_WITNESS_EXTENSION_ID: &str = "contributions-delivery-witness";
    pub const CONTRIBUTIONS_WITNESS_PLUGIN_ID: &str = "flow-extension-contributions-delivery-witness";
    
    pub fn contributions_witness_manifest_json() -> String {
        semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object([
            ("schema".to_string(), semio_framework_value::DslValue::String("flow.extension".into())),
            ("id".to_string(), semio_framework_value::DslValue::String(CONTRIBUTIONS_WITNESS_EXTENSION_ID.into())),
            ("name".to_string(), semio_framework_value::DslValue::String("Contributions Delivery Witness".into())),
            ("version".to_string(), semio_framework_value::DslValue::String("1.0.0".into())),
            ("activationEvents".to_string(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String("onStartup".into())])),
            (
                "contributes".to_string(),
                semio_framework_value::DslValue::object([
                    ("schemas".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                    ("operators".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                    ("widgets".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                    ("commands".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                    ("settings".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                ]),
            ),
        ]))
    }
    
    //#region 🧮️HeapWitness
    /// 🧮️ This test binary's allocator. The guest this artifact ships into runs in ONE fixed
    /// `GUEST_LINEAR_MEMORY_MAXIMUM_BYTES` linear memory with ONE allocator and no threads, so an owner
    /// the install path never frees is a hard `rust_oom` trap in a browser tab and nothing at all in a
    /// native suite — the only way a law can state the bound is to weigh the heap itself
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    #[global_allocator]
    static GENERATION3D_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
    
    /// 📸️ One labelled reading of the process heap, printed as `[MEMORY]` so a lane can read the whole
    /// boot → page → install → eval → tessellation sequence off one run's output.
    pub fn heap_probe(label: &str) -> (isize, isize) {
        let retained = semio_framework_trace::retained_heap_bytes();
        let peak = semio_framework_trace::peak_heap_bytes();
        println!("[MEMORY] {label}: retained={retained} peak={peak}");
        (retained, peak)
    }
    //#endregion 🧮️HeapWitness
    
    /// 🏛 Generate-mode roster: generations + form + generate preview. The generations window is current.
    pub fn generate_shell_views(generations: &str, form: &str, preview: &str) -> (ViewModel, ViewModel) {
        let roster = vec![
            ViewWindowInstance { id: generations.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_generate::windows::generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS.into() },
            ViewWindowInstance { id: form.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_generate::windows::form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM.into() },
            ViewWindowInstance { id: preview.into(), window_kind_id: semio_s_artifact_procedural_3d_mode_generate::windows::preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
        ];
        let view = ViewModel { window_instances: roster, ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
        (view.for_window_instance(generations).expect("generations window instance"), view.for_window_instance(preview).expect("generate preview window instance"))
    }
}

pub(crate) mod serial_execution {
    //! 🔒️ The editor door onto this binary's ONE serial lock ([`semio_s_artifact_procedural_3d_testkit::test_serial`]), which also
    //! installs the packaged `brep`/`math` operator sets behind it — without that installation
    //! `FlowHost::evaluate` answers `unknown kind: …` for every bundled fixture's nodes
    //! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    //!
    //! 🐛️ This used to own a mutex of its own, disjoint from the viewer's and the publication
    //! table's. All three guard the SAME process-global state — the flow extension registry, the
    //! neural kernel cache and the publication lease table — so three locks serialised nothing
    //! across surfaces and left a lock-order inversion available.

    /// 🔒️ Takes the crate's one test serial lock.
    pub(crate) fn lock() -> crate::test_serial::TestSerialGuard {
        crate::test_serial::lock()
    }
}

use super::*;
use self::context::{app, app_with_registry, drain_flow_eval_ticks, drain_flow_eval_ticks_with_view, preview_views};
use semio_framework_plugin::PluginApp;
use serde_json::json;
include!("🧰️shared-helpers/🦀️.rs");
