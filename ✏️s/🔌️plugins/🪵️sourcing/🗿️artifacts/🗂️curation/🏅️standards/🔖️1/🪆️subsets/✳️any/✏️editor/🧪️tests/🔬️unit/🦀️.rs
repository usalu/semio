pub(crate) mod context {
    
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members as new_app_with_registry_impl};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    
    pub type SourcingApp = VcsArtifactApp<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>;
    
    /// 🧪️ Framework test context gap (contract §2.5, w0-f Gap 3 handoff): `new_app_with_registry` and
    /// `assert_declared_actions_bridge_to_commands` still take the pre-migration `fn() -> App` shape,
    /// not the `AppDefinition`-returning `create_sourcing_curation_app`. Local wrapper until that lands.
    pub(crate) fn sourcing_manifest_for_tests() -> App {
        App { definition: create_sourcing_curation_app(), examples: Vec::new() }
    }
    
    /// 🧪️ The app wired to the real manifest registry — the ONLY constructor this app has.
    /// `semio_framework_plugin::artifact_app_laws::new_app`'s bare, registry-less instance is unreachable here:
    /// `bounded_first_step_tool_proofs!` declares fifteen tool rows, and `tool_job_registration`
    /// admits a row only when its id is `InteractiveJobClassification::Migrated` in the LIVE registry,
    /// so an empty `AppActionRegistry` rejects every row with `interactive-job.catalog-authority`.
    /// Same reason trinity's `🔌️jack`/`♻️rewriting` editors are registry-backed.
    pub async fn new_app() -> SourcingTestApp {
        let mut app = new_app_with_registry_impl::<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>(sourcing_manifest_for_tests).await;
        // 🪪️ `dispatch_typed_command_inner` refuses any command whose `ActionMeta.instance_id` is not the
        // app's bound live runtime instance, and a freshly constructed app has none — so bind the id
        // `artifact_app_laws::meta` stamps. A test that wants another instance rebinds (see
        // `retained_example_load_publishes_authored_stock_and_closes_exact_owners`, which uses 7).
        app.bind_instance_id(1).await;
        SourcingTestApp(app)
    }
    
    /// 🧹️ Owning guard around a live `SourcingApp` that runs the bounded plugin close protocol on the
    /// way out. `ArtifactStore`'s `Drop` asserts a terminal-empty shallow shell, and an app holds four
    /// of them (document, config, draft, interaction), so simply letting a test's app fall out of scope
    /// aborts the whole test process — a non-unwinding `panic in a destructor during cleanup`, which
    /// takes every other test in the binary with it. Draining here rather than at ~14 call sites keeps
    /// the teardown impossible to forget, and it is idempotent: a test that closes explicitly (see
    /// `retained_example_load_publishes_authored_stock_and_closes_exact_owners`) leaves nothing to do.
    pub struct SourcingTestApp(SourcingApp);
    
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
            for _ in 0..1_000_000 {
                if self.0.close_terminal_is_empty() {
                    return;
                }
                self.0.close_step(1, 4_096).expect("test app closes within its exact grant");
            }
            panic!("test app must reach its terminal-empty shell");
        }
    }
    
    /// 🎬️ Dispatches one typed command and drives its retained operation to publication, the way the
    /// host does — a migrated command's document and config edits land only once it settles.
    pub async fn dispatch(app: &mut SourcingApp, command: SourcingCurationCommand) -> InvocationResult {
        let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        settle(app).await;
        result
    }

    /// 🏁️ Drains every pending typed operation for instance 1, returning the document effects it published.
    pub async fn settle(app: &mut SourcingApp) -> Vec<semio_framework::kernel::Effect> {
        let mut effects = Vec::new();
        for _ in 0..100_000 {
            app.maintenance_step(1, 4_096).expect("maintenance step");
            app.advance_typed_operation_publication().await.expect("typed operation publication");
            if let Some(page) = app.take_typed_operation_result_page(1) {
                assert!(app.acknowledge_typed_operation_result(page.token).expect("typed operation acknowledgement"));
            }
            if let Some(effect) = app.take_typed_operation_effect() {
                effects.push(effect);
            }
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            if !app.has_pending_typed_operations() {
                break;
            }
            std::thread::yield_now();
        }
        effects
    }
    
    pub async fn render(app: &mut SourcingApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render").root).expect("render json")
    }

    /// 📊️ The table scene a rendered pool/curated body carries — found anywhere in the tree, since the
    /// pool nests its surface under a filter row while the curated body is the surface itself.
    pub fn table_scene_of(node: &semio_framework_plugin::BuiltNode) -> Option<semio_framework_plugin::TableScene> {
        if let semio_framework_plugin::Component::Surface(props) = &node.component {
            if let Ok(scene) = semio_framework_ui_scene::decode::<semio_framework_plugin::TableScene>(props) {
                return Some(scene);
            }
        }
        node.children.iter().find_map(table_scene_of)
    }

    /// 📊️ Renders `body_key` through the live app and decodes its table scene's column ids and row records.
    pub async fn table_of(app: &mut SourcingApp, body_key: &str) -> (Vec<String>, Vec<serde_json::Value>) {
        let rendered = app.render(body_key, None, &ViewModel::default()).await.expect("render");
        let scene = table_scene_of(&rendered.root).expect("a table scene in the rendered body");
        let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");
        (columns.iter().map(|column| column["id"].as_str().unwrap_or_default().to_string()).collect(), rows)
    }

    /// 🪪️ The `id` of every row a table body currently shows, in display order.
    pub async fn row_ids(app: &mut SourcingApp, body_key: &str) -> Vec<String> {
        table_of(app, body_key).await.1.iter().map(|row| row["id"].as_str().unwrap_or_default().to_string()).collect()
    }
}


use super::*;
use semio_framework_plugin::plugin_app_close_prelude::TypedOperationResultLane;

//#region 🧪️RetainedConfigOracle
#[semio_framework_async_macros::async_test]
async fn retained_example_load_publishes_authored_stock_and_closes_exact_owners() {
    let oracle: Vec<crate::ObjectKind> = dsl::json::from_json_str(include_str!("../../../🧫️fixtures/📦️expected-stock.json")).unwrap();
    for example_id in [DEMO_STOCK_EXAMPLE_ID, EMPTY_EXAMPLE_ID] {
        let mut app = new_app().await;
        app.bind_instance_id(7).await;
        app.dispatch_typed(SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_framework_plugin::ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None }).await.unwrap();
        let mut document = None;
        let mut terminal = false;
        for _ in 0..100_000 {
            app.maintenance_step(1, 4_096).unwrap();
            app.advance_typed_operation_publication().await.unwrap();
            if let Some(page) = app.take_typed_operation_result_page(7) {
                assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                terminal |= page.lane == TypedOperationResultLane::Terminal;
                assert!(app.acknowledge_typed_operation_result(page.token).unwrap());
            }
            if let Some(effect) = app.take_typed_operation_effect() {
                let semio_framework::kernel::Effect::LoadDocument { pack, spr } = effect else { panic!("example must publish a document load") };
                let history = protocol::os_spr::decode_history(&spr, &protocol::os_spr::DecodeOptions::default()).await.unwrap();
                assert_eq!(history.doc_id, "s.sourcing.curation@1/*#editor", "the load names the live store, not the app-minted id");
                assert!(history.composition.as_ref().and_then(|composition| composition.dialect.as_ref()).is_some(), "the load carries the dialect archive hydration checks");
                assert_eq!(history.schema, SOURCING_CURATION_SCHEMA);
                assert!(history.edits.is_empty());
                document = Some(CurationSnapshot::decode_pack(&pack).unwrap());
            }
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            if !app.has_pending_typed_operations() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(terminal);
        let document = document.expect("retained example document effect");
        let stock = crate::stock_of(&document);
        if example_id == DEMO_STOCK_EXAMPLE_ID {
            assert_eq!(stock, oracle);
        } else {
            assert!(stock.is_empty());
        }
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            app.close_step(1, 4_096).unwrap();
            std::thread::yield_now();
        }
        assert!(app.close_terminal_is_empty());
        eprintln!("[DEBUG] retained example {example_id} published {} authored rows and retired", stock.len());
    }
}

/// 🚪️ The browser host answers an example load by handing the published `LoadDocument` back through
/// the document archive door with an empty member roster; the load must settle `Ready` and publish the
/// authored stock rather than trap the instance.
#[semio_framework_async_macros::async_test]
async fn example_load_settles_through_the_host_document_archive_door() {
    let oracle: Vec<crate::ObjectKind> = dsl::json::from_json_str(include_str!("../../../🧫️fixtures/📦️expected-stock.json")).unwrap();
    let mut app = new_app().await;
    app.bind_instance_id(7).await;
    app.dispatch_typed(SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }), &semio_framework_plugin::ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None }).await.unwrap();
    let mut loaded = None;
    for _ in 0..100_000 {
        app.maintenance_step(1, 4_096).unwrap();
        app.advance_typed_operation_publication().await.unwrap();
        if let Some(page) = app.take_typed_operation_result_page(7) {
            assert!(app.acknowledge_typed_operation_result(page.token).unwrap());
        }
        if let Some(semio_framework::kernel::Effect::LoadDocument { pack, spr }) = app.take_typed_operation_effect() {
            loaded = Some((pack, spr));
        }
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
        if !app.has_pending_typed_operations() {
            break;
        }
        std::thread::yield_now();
    }
    let (parent_pack, parent_spr) = loaded.expect("example publishes a document load");
    PluginApp::begin_document_archive_load(&mut *app, 91, protocol::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).expect("archive admission");
    let mut status = None;
    for _ in 0..1_000_000 {
        let polled = PluginApp::poll_document_archive_load(&mut *app, 91).await.expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        let _ = PluginApp::maintenance_step(&mut *app, 1, 4_096).expect("archive maintenance step");
        std::thread::yield_now();
    }
    let status = status.expect("archive load reaches a terminal state");
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    PluginApp::acknowledge_document_archive_load(&mut *app, 91).expect("archive acknowledgement");
    assert_eq!(crate::stock_of(&app.snapshot().expect("loaded snapshot")), oracle);
}

/// 💾️ What the shell persists is `document_archive()`; a curation's archive must carry its derived
/// catalog member from the first frame (boot genesis) so a reload reopens it through the roster rather
/// than re-deriving it — and the reloaded document keeps the edits made on top of the example.
#[semio_framework_async_macros::async_test]
async fn a_saved_curation_archive_carries_its_catalog_member_and_reloads_with_its_edits() {
    let mut app = new_app().await;
    let snapshot = app.snapshot().expect("boot snapshot");
    let catalog_id = snapshot.catalog.child_id.clone();
    assert_eq!(catalog_id, crate::catalog_child_handle(&crate::stock_of(&snapshot)).child_id, "the boot document declares the handle its own stock derives");
    let object_id = crate::stock_of(&snapshot)[0].id.clone();
    dispatch(&mut app, SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id.clone() })).await;
    let archive = PluginApp::document_archive(&*app).await.expect("document archive");
    assert_eq!(archive.members.len(), 1, "boot genesis opened exactly the catalog member");
    assert_eq!((archive.members[0].owner.slot.as_str(), archive.members[0].reference.artifact_id.as_str(), archive.members[0].reference.subset.as_str()), (crate::CATALOG_CHILD_SLOT, catalog_id.as_str(), "kit"));
    PluginApp::begin_document_archive_load(&mut *app, 92, archive).expect("archive admission");
    let mut status = None;
    for _ in 0..1_000_000 {
        let polled = PluginApp::poll_document_archive_load(&mut *app, 92).await.expect("archive status");
        if matches!(polled.state, protocol::DocumentArchiveLoadState::Ready | protocol::DocumentArchiveLoadState::Cancelled | protocol::DocumentArchiveLoadState::Fault) {
            status = Some(polled);
            break;
        }
        std::thread::yield_now();
    }
    let status = status.expect("archive reload reaches a terminal state");
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "{}", String::from_utf8_lossy(&status.fault));
    PluginApp::acknowledge_document_archive_load(&mut *app, 92).expect("archive acknowledgement");
    let reloaded = app.snapshot().expect("reloaded snapshot");
    assert_eq!(reloaded.catalog.child_id, catalog_id);
    assert_eq!(reloaded.curated.iter().map(|item| (item.object_id.as_str(), item.count)).collect::<Vec<_>>(), vec![(object_id.as_str(), 1)]);
    assert!(app.child_store(crate::CATALOG_CHILD_SLOT, &catalog_id).await.is_some(), "the reopened catalog member is live after the reload");
    eprintln!("[DEBUG] saved curation archive reloaded with {} member(s)", status.total.saturating_sub(1));
}

#[test]
fn retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one() {
    let base = SourcingCurationConfig::default();
    let mut expected = serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&base)).expect("JSON oracle base");
    expected["filters"]["query"] = serde_json::json!("timber");
    let (post, inverse, _) = prepare_sourcing_curation_config(&base, SourcingCurationConfigMutation::SetFilterQuery { value: "timber".into() }).expect("bounded config candidate");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&dsl::json::to_json_string(&post)).expect("JSON oracle post"), expected);
    assert!(matches!(&inverse[0], SourcingCurationConfigMutation::SetFilterQuery { value } if value == &base.filters.query));
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetFilterQuery { value: "x".repeat(SOURCING_CURATION_CONFIG_TEXT_BYTES) }).is_ok());
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetFilterQuery { value: "x".repeat(SOURCING_CURATION_CONFIG_TEXT_BYTES + 1) }).is_err());
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetFilterModules { module_ids: Vec::new() }).is_ok(), "clearing the module filter is a retained edit");
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetFilterModules { module_ids: vec!["m".into(); SOURCING_CURATION_CONFIG_STORE_MAXIMUM_ITEMS + 1] }).is_err());
    assert_eq!(SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES, 768 + SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, "the retained config store is the filter envelope plus its own contributions lane");
    assert_eq!(SOURCING_CURATION_CONFIG_GRANT_BYTES, 4_096, "one config turn must still fit the host's fixed 4 KiB typed-operation page grant");
}

/// ⚖️ LAW: the contributions lane is priced on its OWN envelope, not the 96-byte filter-text one —
/// a `sourcing.module` pack is a host-pushed capability blob, never typed-in filter text. Its
/// ceiling is exactly the wire envelope `setContributions` is already granted, so nothing can be
/// retained that could not have crossed the tool boundary.
#[test]
fn the_contributions_lane_is_priced_apart_from_the_filter_text_envelope() {
    assert!(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES > SOURCING_CURATION_CONFIG_TEXT_BYTES);
    let inside = SourcingCurationConfigMutation::SetContributions { json: "x".repeat(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES) };
    assert!(sourcing_curation_config_mutation_footprint(&inside).is_ok(), "a pack at the lane ceiling is admitted");
    let over = SourcingCurationConfigMutation::SetContributions { json: "x".repeat(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES + 1) };
    assert!(sourcing_curation_config_mutation_footprint(&over).is_err(), "one byte past the lane is refused, never truncated");
    let base = SourcingCurationConfig { contributions_json: "x".repeat(SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES), ..Default::default() };
    let (post, inverse, _) = prepare_sourcing_curation_config(&base, SourcingCurationConfigMutation::SetFilterQuery { value: "timber".into() }).expect("a filter edit over a full contributions lane stays bounded");
    assert_eq!(post.contributions_json.len(), SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, "a filter edit never disturbs the retained pack");
    assert!(matches!(&inverse[0], SourcingCurationConfigMutation::SetFilterQuery { .. }));
}
/// 🧩️ The pack the DEMONSTRATOR's aussuchen pane actually receives: the demonstrator consumes
/// `sourcing.module`, `cad.computer` and `process.machines`, so the host hands this app all three
/// topics in ONE crossing. Sizes are the REAL per-entry bulk strings of the shipped extensions,
/// measured off the built dev manifests 2026-09-16 (ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS).
fn demonstrator_contributions_pack() -> String {
    let entry = |plugin_id: &str, topic: &str, app_id: &str, bulk: usize| semio_framework::ProgramContributionEntry {
        plugin_id: plugin_id.to_string(),
        topic_contribution: Some(semio_framework::TopicContribution::new(
            topic,
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String(app_id.to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String(plugin_id.to_string())),
                ("bulkJson".to_string(), semio_framework::DslValue::String("m".repeat(bulk))),
            ]),
        )),
    };
    let entries = vec![
        entry("cad-extension-aec-building-structure", "cad.computer", "cad-play", 3_337),
        entry("cad-extension-aec-building", "cad.computer", "cad-play", 940),
        entry("cad-extension-aec-building-energy", "cad.computer", "cad-play", 635),
        entry("cad-extension-spatial-shape", "cad.computer", "cad-play", 176),
        entry("process-extension-wood", "process.machines", "process3d-play", 4_705),
        entry("process-extension-metal", "process.machines", "process3d-play", 4_011),
        entry("process-extension-robotic", "process.machines", "process3d-play", 4_010),
        entry("process-extension-concrete", "process.machines", "process3d-play", 3_311),
        entry("sourcing-module-beams", "sourcing.module", "sourcing-curation", 780),
        entry("sourcing-module-windows", "sourcing.module", "sourcing-curation", 669),
        entry("sourcing-module-slabs", "sourcing.module", "sourcing-curation", 589),
    ];
    semio_framework_os_kernel::json::to_json_string(&entries)
}

/// ⚖️ LAW: the REAL demonstrator pack crosses this app's registered `setContributions` admission and
/// distils to a roster the retained contributions lane holds.
///
/// 🏁️ Before the per-app admission, sourcing priced `setContributions` on the 8 KiB gesture envelope
/// every retained tool shares, and the live push died with `typed command raw JSON exceeds its
/// registered retained-page admission` the moment the host stopped cutting capability packs to `[]`
/// (2026-09-16, ticket 26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS).
#[test]
fn the_real_demonstrator_pack_is_admitted_by_the_registered_contributions_wire() {
    let pack = demonstrator_contributions_pack();
    let wire = semio_framework_os_kernel::json::to_json_string(&("setContributions", semio_framework::DslValue::object([("json".to_string(), semio_framework::DslValue::String(pack.clone()))])));
    println!("[STATS] sourcing demonstrator pack packChars={} wireChars={}", pack.len(), wire.len());
    assert!(pack.len() > SOURCING_CURATION_RETAINED_RAW_BYTES, "the real pack is past the gesture envelope");
    assert!(wire.len() <= semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, "the real pack's command wire ({} B) must fit the registered admission", wire.len());
    let distilled = crate::schema::installable_contributions(&pack, SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES);
    assert!(distilled.len() <= SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES, "what is RETAINED is the distilled roster, never the pack");
    assert!(sourcing_curation_config_mutation_footprint(&SourcingCurationConfigMutation::SetContributions { json: distilled }).is_ok());
}
//#endregion 🧪️RetainedConfigOracle
use crate::editor::sourcing::unit_tests::context::{dispatch, new_app, sourcing_manifest_for_tests};
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::{EditorApp, PluginApp};

#[semio_framework_async_macros::async_test]
async fn view_kind_config_only_commands_pass_kind_discipline() {
    // 🧬️ A registry-backed wrapper so the View-kind declarations actually get enforced.
    let mut app = new_app().await;
    // 🏁️ Through the settling helper: a bare `dispatch_typed` leaves the migrated command's retained
    // operation pending, and a store that still owes a publication never reaches the terminal-empty
    // shallow shell the guard drains for on `Drop`.
    let result = dispatch(&mut app, SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() })).await;
    assert!(result.mutations.is_empty(), "setFilterQuery is config-only, no document operations");
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 14, "every SourcingCurationCommand row must be covered by every_command()");
}

/// ⚖️ LAW: the PRODUCTION action bridge covers every declared row. `ArtifactEditor`'s default
/// `command_from_action` rejects app-owned actions outright, so an app that declares actions but
/// never overrides it ships a UI whose every control faults — which is exactly how this pane's
/// `setActiveExample` (and with it the whole `demo-stock` example) was dead in the demonstrator.
/// Asserted through `<SourcingCurationApp as ArtifactEditor>` — NOT the free function — because the
/// trait method is the seam the host actually calls.
#[semio_framework_async_macros::async_test]
async fn the_production_action_bridge_admits_every_declared_command() {
    for command in every_command() {
        let action = command.command_id();
        let built = <SourcingCurationApp as ArtifactEditor>::command_from_action(action, None).unwrap_or_else(|fault| panic!("action '{action}' is declared but the production bridge rejects it: {fault:?}"));
        assert_eq!(built.command_id(), action, "the bridge routed '{action}' to the wrong command");
    }
}

/// ⚖️ LAW: the framework's own conformance harness — it walks the actions this app's window kinds
/// actually RENDER, stages each one's declared args exactly as the host does, and knows which ids
/// are framework-injected (`undo`/`copy`/`recordTutorial`/…) and must be skipped. Strictly stronger
/// than enumerating the command rows: it catches an action that chrome declares but no command row
/// backs.
#[semio_framework_async_macros::async_test]
async fn every_rendered_action_bridges_through_the_framework_harness() {
    artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<SourcingCurationApp>>(sourcing_manifest_for_tests).await;
}

/// ⚖️ LAW: the bridge reads the manifest's OWN arg names — `setActiveExample` declares a select
/// arg keyed `exampleId` (`🔖️Manifest`), and the payload field is `example_id`; the two
/// vocabularies are joined here and nowhere else.
#[semio_framework_async_macros::async_test]
async fn the_action_bridge_reads_the_declared_arg_names() {
    let built = <SourcingCurationApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&protocol::DslValue::from(&serde_json::json!({ "exampleId": DEMO_STOCK_EXAMPLE_ID })))).expect("setActiveExample must convert");
    assert_eq!(built, SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }));
    let filter = <SourcingCurationApp as ArtifactEditor>::command_from_action("setFilterModule", Some(&protocol::DslValue::from(&serde_json::json!({ "moduleId": "walls", "enabled": true })))).expect("setFilterModule must convert");
    assert_eq!(filter, SourcingCurationCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "walls".into(), enabled: true }));
    assert!(<SourcingCurationApp as ArtifactEditor>::command_from_action("noSuchAction", None).is_err(), "an undeclared action must fault, not silently no-op");
}

/// ⚖️ LAW (S15 matrix, 2026-09-25): the rail stages `curationSetCount`'s `delta`/`value` through the argument schema
/// the manifest declares, and the bridge reads them as numbers. Declared as text, the staged `"1"` read as no delta,
/// the command resolved to a no-op and the rail journaled a row that changed nothing. The count args are numeric, and
/// the numeric staging reaches the command as the delta the user entered.
#[semio_framework_async_macros::async_test]
async fn the_rail_stages_numeric_curation_counts_into_one_curated_edit() {
    let definition = create_sourcing_curation_app();
    let action = definition.actions.iter().find(|action| action.id == "curationSetCount").expect("curationSetCount is declared on the rail");
    for id in ["delta", "value"] {
        let arg = action.args.iter().find(|arg| arg.id == id).unwrap_or_else(|| panic!("curationSetCount declares {id}"));
        assert!(matches!(arg.schema, semio_framework_plugin::ArgSchema::Number { .. }), "curationSetCount.{id} must be a number argument, found {:?}", arg.schema);
    }
    let built = <SourcingCurationApp as ArtifactEditor>::command_from_action("curationSetCount", Some(&protocol::DslValue::from(&serde_json::json!({ "objectId": "beam-glulam-gl24h", "delta": 1 })))).expect("curationSetCount must convert");
    assert_eq!(built, SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }));
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row — the
/// permanent successor of the old `📡️protocol` crate's
/// `sourcing_curation_command_op_text_round_trips_every_variant`.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — copied
/// verbatim from each `app_commands!` row's `as "…"` literal (NOT a mechanical kebab-case of the
/// manifest action id: `setDocument`/`document-json`, `setActiveExample`/`active-example`, the whole
/// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    fn expected_wire_key(id: &str) -> &'static str {
        match id {
            "setDocument" => "document-json",
            "setActiveExample" => "active-example",
            "stockFromCatalogue" => "stock-from-catalogue",
            "curationAdd" => "curation-add",
            "curationSetCount" => "curation-set-count",
            "curationRemove" => "curation-remove",
            "dropOnPool" => "drop-on-pool",
            "dropOnCurated" => "drop-on-curated",
            "setFilterQuery" => "filter-query",
            "setFilterModule" => "filter-module",
            "setFilterTypology" => "filter-typology",
            "setFilterMinAvailability" => "filter-min-availability",
            "sortTable" => "sort-table",
            "setContributions" => "contributions",
            other => panic!("expected_wire_key: unhandled command id {other}"),
        }
    }
    for command in every_command() {
        let id = command.command_id();
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_wire_key(id), "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact bytes
/// captured from the pre-merge `semio-s-app-sourcing-curation-protocol` crate
/// (`wire-baseline-before.txt` in this ticket's folder). A regression here is a real format break, not
/// a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(SourcingCurationCommand, &str, &str); 2] = [
        (
            SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
            "curation-set-count curation-set-count object-id=beam-glulam-gl24h delta=1",
            "010401116265616d2d676c756c616d2d676c323468020006000105000000000000f03f",
        ),
        (
            SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: None, value: Some(4.0) }),
            "curation-set-count curation-set-count object-id=beam-glulam-gl24h value=4",
            "010401116265616d2d676c756c616d2d676c3234680200060002050000000000001040",
        ),
    ];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order — mirrors the
/// pre-migration wire baseline captured into this ticket's `wire-baseline-before.txt`.
fn every_command() -> Vec<SourcingCurationCommand> {
    vec![
        SourcingCurationCommand::SetArtifactJson(set_artifact_json::SetArtifactJson { json: "{}".into() }),
        SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }),
        SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
        SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: "beam-glulam-gl24h".into(), delta: Some(1.0), value: None }),
        SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: "beam-glulam-gl24h".into() }),
        SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: "glulam".into() }),
        SourcingCurationCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: "beams".into(), enabled: true }),
        SourcingCurationCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: "beams/steel".into() }),
        SourcingCurationCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: Some(1.0), value: None }),
        SourcingCurationCommand::SortTable(sort_table::SortTable { column_id: "availability".into(), direction: "desc".into() }),
        SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
    ]
}
//#endregion 🔖️CommandSurface

#[semio_framework_async_macros::async_test]
async fn app_definition_labels_resolve_german() {
    let def = &create_sourcing_curation_app();
    let (terminology, locale) = (semio_framework_plugin::Terminology::Native, semio_framework_plugin::Locale::De);
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == pool::SOURCING_CURATION_WINDOW_POOL).expect("pool window").label.resolve(terminology, locale), "Pool");
    assert_eq!(def.window_kinds.iter().find(|entry| entry.id == curated::SOURCING_CURATION_WINDOW_CURATED).expect("curated window").label.resolve(terminology, locale), "Kuratiert");
    assert_eq!(def.modes.iter().find(|entry| entry.id == edit::SOURCING_CURATION_MODE_CURATION).expect("curation mode").label.resolve(terminology, locale), "Kuratierung");
}

/// ⚖️ LAW: the host bridge lands on the event-sourced config lane and retains the INSTALLABLE share
/// of the pushed pack, never the pack itself — a payload carrying no `sourcing.module` entry this app
/// can act on installs nothing, and a real module survives verbatim.
#[test]
fn host_contributions_resolve_to_the_event_sourced_config_lane() {
    let foreign = <SourcingCurationApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&protocol::DslValue::from(&serde_json::json!({ "json": "[{\"id\":\"sourcing\"}]" }))))
        .expect("host configuration")
        .expect("sourcing contribution mutation");
    assert_eq!(foreign, SourcingCurationConfigMutation::SetContributions { json: "[]".into() }, "a pack with nothing this app installs is retained as an empty roster");
    let pack = crate::schema::installable_contributions(&sourcing_reuse_contribution(), SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES);
    let installed = <SourcingCurationApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&protocol::DslValue::from(&serde_json::json!({ "json": sourcing_reuse_contribution() }))))
        .expect("host configuration")
        .expect("sourcing contribution mutation");
    assert_eq!(installed, SourcingCurationConfigMutation::SetContributions { json: pack });
    assert_eq!(<SourcingCurationApp as ArtifactEditor>::host_configuration_mutation("setFilterQuery", None).expect("non-host action"), None);
}

/// 🧩️ One host `sourcing.module` contribution for a module this crate does NOT author.
fn sourcing_reuse_contribution() -> String {
    let kind = crate::ObjectKind { id: "reuse-salvaged-oak".into(), name: "Salvaged Oak Beam".into(), module_id: "reuse".into(), typology_path: vec!["reuse".into()], availability: 4, geometry: Box::new(crate::GeometryRecipe::Box { width: 0.2, height: 0.2, depth: 3.0 }) };
    let entry = semio_framework::ProgramContributionEntry {
        plugin_id: "sourcing-module-reuse".into(),
        topic_contribution: Some(semio_framework::TopicContribution::new(
            crate::schema::SOURCING_MODULE_TOPIC,
            semio_framework::DslValue::object([
                ("appId".to_string(), semio_framework::DslValue::String("sourcing-curation".to_string())),
                ("moduleId".to_string(), semio_framework::DslValue::String("reuse".to_string())),
                ("label".to_string(), semio_framework::DslValue::String("Reuse".to_string())),
                ("iconId".to_string(), semio_framework::DslValue::String("recycle".to_string())),
                ("typologyJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&crate::schema::TypologyNode::new("reuse", "Reuse", vec![])))),
                ("kindsJson".to_string(), semio_framework::DslValue::String(semio_framework_os_kernel::json::to_json_string(&vec![kind]))),
            ]),
        )),
    };
    dsl::json::to_json_string(&vec![entry])
}

#[test]
fn retained_factories_declare_every_publication_lane() {
    let bounded = <SourcingCurationBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS;
    assert_eq!(bounded.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), SOURCING_CURATION_BOUNDED_TOOL_IDS);
    let lane_of = |tool_id: &str| bounded.iter().find(|contract| contract.tool_id == tool_id).expect("declared tool").lanes;
    for tool_id in ["setActiveExample", "setDocument", "stockFromCatalogue"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::HostOnly], "{tool_id} replaces the whole document through an effect");
    }
    for tool_id in ["curationAdd", "curationSetCount", "curationRemove", "dropOnPool", "dropOnCurated"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::Artifact], "{tool_id} publishes a curated-selection mutation");
    }
    for tool_id in ["setFilterQuery", "setFilterModule", "setFilterTypology", "setFilterMinAvailability", "sortTable", "setContributions"] {
        assert_eq!(lane_of(tool_id), [ArtifactToolPublicationLane::Config], "{tool_id} publishes view state only");
    }
    assert_eq!(lane_of("setGridInstanceDisplay"), [ArtifactToolPublicationLane::WindowConfig], "grid display mode is per-window config");
}

#[semio_framework_async_macros::async_test]
async fn sourcing_curation_io_declares_the_catalog_out_port_alongside_the_implicit_document_ports() {
    let io = sourcing_curation_io();
    assert_eq!(io.artifact_schema, SOURCING_CURATION_SCHEMA);
    let ports = io.all_ports().await;
    assert_eq!(ports.len(), 3, "document:in, document:out, catalog:out");
    let catalog_out = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
    assert_eq!(catalog_out.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(catalog_out.media_type.class, MediaClass::Kit);
    assert_eq!(catalog_out.media_type.form, MediaForm::Type);
}

#[semio_framework_async_macros::async_test]
async fn sourcing_curation_io_and_catalog_export_round_trip() {
    let mut app = new_app().await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("catalog:out")).expect("catalog export");
    assert_eq!(media.media_type.class, MediaClass::Kit);
    assert_eq!(media.media_type.form, MediaForm::Type);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "kit.catalog");
            let fragment: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert_eq!(fragment["objectKinds"].as_array().unwrap().len(), app.snapshot().expect("snapshot").stock_extra.len());
        }
        MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
    }
}

/// deliberately absent — it is `ForbiddenFromUi`, dispatched only through the host configuration
/// route, and it is the ONLY command of this app that is not a retained tool.
#[test]
fn retained_route_catalog_covers_every_ui_reachable_command() {
    let mut routes = SOURCING_CURATION_BOUNDED_TOOL_IDS.to_vec();
    routes.sort_unstable();
    routes.dedup();
    assert_eq!(routes.len(), SOURCING_CURATION_BOUNDED_TOOL_IDS.len(), "no route is declared twice");
    assert_eq!(routes.len(), 15);
}

#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_sourcing_curation_app()).expect("app definition json");
    for id in [pool::SOURCING_CURATION_WINDOW_POOL, curated::SOURCING_CURATION_WINDOW_CURATED, preview::SOURCING_CURATION_WINDOW_PREVIEW, grid::SOURCING_CURATION_WINDOW_GRID] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::SOURCING_CURATION_MODE_CURATION), "mode missing from the manifest");
    assert!(json.contains("catalogue.sourcing"), "artifact kind missing from the manifest");
}
