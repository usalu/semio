//! 🧰️ The mounted-app harness of the sourcing editor tests: it drives the whole app, so it sits above every layer; it is its own compile unit so the app machinery it instantiates is paid once, and each editor test target mounts it by `crate::editor_unit_tests::context`.

use semio_plugin::EditorApp;
use crate::editor::sourcing::{SourcingCurationApp, SourcingCurationCommand, create_sourcing_curation_app};
use semio_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry_and_members as new_app_with_registry_impl, observe_registered_fixture_app, render_registered_fixture_app, OriginalFixtureScope};
use semio_plugin::{App, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

/// 🏟️ One test's UI value arena; leaving it first drains the built-node pages the test's dropped trees queued, since those pages still own arena values the arena cannot close over.
pub struct ArenaScope(Option<semio_ui_contract_values::testkit::UiArenaFixture>);

impl Drop for ArenaScope {
    fn drop(&mut self) {
        for _ in 0..1_000_000 {
            if semio_plugin::plugin_app_close_prelude::close_built_node_page_one() {
                break;
            }
        }
        drop(self.0.take());
    }
}

pub fn arena() -> ArenaScope {
    ArenaScope(Some(semio_ui_contract_values::testkit::UiArenaFixture::new()))
}

pub type SourcingApp = VcsArtifactApp<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>;

/// 🧪️ Framework test context gap (contract §2.5, w0-f Gap 3 handoff): `new_app_with_registry` and
/// `assert_declared_actions_bridge_to_commands` still take the pre-migration `fn() -> App` shape,
/// not the `AppDefinition`-returning `create_sourcing_curation_app`. Local wrapper until that lands.
pub fn sourcing_manifest_for_tests() -> App {
    App { definition: create_sourcing_curation_app(), examples: Vec::new() }
}


/// 📱️ The app wired to the real manifest registry — the ONLY constructor this app has.
/// `semio_plugin::artifact_app_laws::new_app`'s bare, registry-less instance is unreachable here:
/// `bounded_first_step_tool_proofs!` declares fifteen tool rows, and `tool_job_registration`
/// admits a row only when its id is `InteractiveJobClassification::Migrated` in the LIVE registry,
/// so an empty `AppActionRegistry` rejects every row with `interactive-job.catalog-authority`.
/// Same reason trinity's `🔌️jack`/`♻️rewriting` editors are registry-backed.
pub async fn new_app() -> SourcingTestApp {
    let scope = OriginalFixtureScope::admit();
    let mut app = new_app_with_registry_impl::<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>(sourcing_manifest_for_tests, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(1).await;
    SourcingTestApp(app, scope)
}

/// 🧹️ Owning guard around a live `SourcingApp` that runs the bounded plugin close protocol on the
/// way out. `ArtifactStore`'s `Drop` asserts a terminal-empty shallow shell, and an app holds four
/// of them (document, config, draft, interaction), so simply letting a test's app fall out of scope
/// aborts the whole test process — a non-unwinding `panic in a destructor during cleanup`, which
/// takes every other test in the binary with it. Draining here rather than at ~14 call sites keeps
/// the teardown impossible to forget, and it is idempotent: a test that closes explicitly (see
/// `retained_example_load_publishes_authored_stock_and_closes_exact_owners`) leaves nothing to do.
/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads
/// `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
pub struct SourcingTestApp(pub SourcingApp, pub OriginalFixtureScope);

impl SourcingTestApp {
    /// 🔄️ Closes the registered app through its exact retained close state machine under the scope's original caller; closing a closed app is a no-op.
    pub fn close_fixture(&mut self) {
        close_registered_fixture_app(&mut self.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
    }
}

impl std::ops::Deref for SourcingTestApp {
    type Target = SourcingApp;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SourcingTestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for SourcingTestApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close_fixture();
        }
    }
}

/// 🎬️ Dispatches one typed command and drives its retained operation to publication, the way the
/// host does — a migrated command's document and config edits land only once it settles.
pub async fn dispatch(app: &mut SourcingTestApp, command: SourcingCurationCommand) -> InvocationResult {
    let result = match &command {
        SourcingCurationCommand::SetContributions(payload) => {
            let address = semio_framework::manifest::CommandAddress { owner: semio_framework::manifest::CommandOwnerAddress::App { plugin_id: "sourcing".into(), app_id: "s.sourcing.curation@1/*#editor".into() }, command_id: "setContributions".into() };
            let arguments = Some(semio_framework_value::DslValue::object([("json".to_owned(), semio_framework_value::DslValue::String(payload.json.clone()))]));
            semio_plugin::artifact_app_laws::dispatch_fixture_command(&mut app.0, &address, arguments, None, &meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).await.expect("dispatch")
        }
        _ => semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &meta("local"), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch"),
    };
    settle(app).await;
    result
}

/// 🌐️ Dispatches one UI action through the action endpoint with the JSON arguments the browser host delivers, then settles it.
pub async fn dispatch_wire(app: &mut SourcingTestApp, action: &str, arguments: &str) -> Result<InvocationResult, semio_framework_job::OriginalTurnFailure> {
    let result = dispatch_wire_unsettled(app, action, arguments).await;
    settle(app).await;
    result
}

/// 🌐️ Dispatches one UI action like [`dispatch_wire`] but leaves its typed operation unsettled, as the browser does: the dispatch answers its operation id at once and the next wire message can arrive while the operation still publishes.
pub async fn dispatch_wire_unsettled(app: &mut SourcingTestApp, action: &str, arguments: &str) -> Result<InvocationResult, semio_framework_job::OriginalTurnFailure> {
    let args = Some(semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(arguments, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("wire json")));
    semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, args, &meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).await
}

/// 🏁️ Drains every pending typed operation for instance 1, returning the document effects it published.
pub async fn settle(app: &mut SourcingTestApp) -> Vec<semio_framework::kernel::Effect> {
    let receipt = semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, 1, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("complete original typed operation settlement");
    receipt.effects
}

pub async fn drain_auxiliary_outboxes(app: &mut SourcingApp) {
    while app.take_typed_operation_ui_progress().is_some() {}
    while app.take_typed_operation_composed_result().is_some() {}
    while app.take_typed_operation_completion().await.expect("typed operation completion").is_some() {
    }
    while let Some(reply) = app.take_local_interaction_query_reply() {
        if let protocol::LocalInteractionQueryReply::Page { page } = reply {
            let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
            assert!(app.acknowledge_local_interaction_query(&token), "exact local interaction query acknowledgement");
        }
    }
}

/// 📤️ One media port exported through the app's original caller, its pending turns driven until the port answers.
pub async fn export_media(app: &mut SourcingTestApp, port: &str) -> semio_plugin::Media {
    for _ in 0..semio_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
        if let Some(media) = app.0.export_media(port, app.1.cx()).await.expect("media export turn") {
            return media;
        }
    }
    panic!("media export of {port} stayed pending under its original caller");
}

pub async fn render(app: &mut SourcingTestApp, body_key: &str) -> String {
    render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), app.1.cx()).await.expect("bounded retained fixture projection")
}

/// 📊️ The table scene a rendered pool/curated body carries — found anywhere in the tree, since the
/// pool nests its surface under a filter row while the curated body is the surface itself.
pub fn table_scene_of(node: &semio_plugin::BuiltNode) -> Option<semio_plugin::TableScene> {
    if let semio_plugin::Component::Surface(props) = &node.component {
        if props.kind == semio_plugin::plugin_app_close_prelude::SurfaceKind::Table {
            let spine: semio_plugin::TableScene = semio_ui_scene::decode(props).expect("typed table spine");
            assert!(node.rejected_children.is_empty(), "exact retained table has no rejected children");
            assert_eq!(spine.lanes.len(), 2, "complete table lane declarations");
            assert_eq!(node.children.len(), spine.lanes.len(), "complete retained table carriers");
            for (name, key) in [("columns", semio_ui_scene::TABLE_COLUMNS_LANE_KEY), ("rows", semio_ui_scene::TABLE_ROWS_LANE_KEY)] {
                let declared: Vec<_> = spine.lanes.iter().filter(|lane| lane.lane == name).collect();
                assert_eq!(declared.len(), 1, "one exact declared table lane {name}");
                let carriers: Vec<_> = node.children.iter().filter(|child| child.key.as_str() == key).collect();
                assert_eq!(carriers.len(), 1, "one exact retained table carrier {key}");
                let payload = semio_plugin::artifact_app_laws::built_carrier_text(carriers[0]);
                assert_eq!(payload.len(), declared[0].bytes as usize, "complete lane UTF-8 bytes");
                assert_eq!(semio_ui_scene::scene_lane_hash(&payload), declared[0].hash, "complete lane digest");
            }
            return Some(semio_plugin::artifact_app_laws::built_surface_scene(node).expect("complete first-party retained table scene"));
        }
    }
    node.children.iter().find_map(table_scene_of)
}

/// 🖼️ Renders `body_key` through the live app and decodes its table scene's column ids and row records.
pub async fn table_of(app: &mut SourcingTestApp, body_key: &str) -> (Vec<String>, Vec<serde_json::Value>) {
    observe_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), app.1.cx(), |root| {
        let scene = table_scene_of(root).expect("a table scene in the rendered body");
        let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");
        (columns.iter().map(|column| column["id"].as_str().unwrap_or_default().to_string()).collect(), rows)
    }).await.expect("render")
}

/// 🪪️ The `id` of every row a table body currently shows, in display order.
pub async fn row_ids(app: &mut SourcingTestApp, body_key: &str) -> Vec<String> {
    table_of(app, body_key).await.1.iter().map(|row| row["id"].as_str().unwrap_or_default().to_string()).collect()
}
