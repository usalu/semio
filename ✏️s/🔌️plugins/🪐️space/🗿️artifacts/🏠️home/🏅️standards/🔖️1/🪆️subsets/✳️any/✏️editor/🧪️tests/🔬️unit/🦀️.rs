
use super::*;

//#region 🧪️RetainedCommandEnvelope
#[test]
fn retained_command_fixture_matches_exact_routes_and_serde_json_boundaries() {
    let fixture: pack::JsonValue = pack::parse_json(include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json")).expect("language-neutral retained fixture");
    let migrated: Vec<&str> = fixture["routes"].as_array().expect("routes").iter().filter(|row| row["disposition"] == "Migrated").map(|row| row["id"].as_str().expect("route id")).collect();
    assert_eq!(migrated, HOME_RETAINED_TOOL_IDS);
    assert_eq!(HOME_RETAINED_PUBLICATION_CONTRACTS.len(), migrated.len());
    assert_eq!(fixture["controller"].as_str(), Some(S_HOME_CONTROLLER_ID));
    let contract = home_retained_contract();
    for (key, expected) in [
        ("rawBytes", HOME_RETAINED_RAW_BYTES),
        ("checkpointBytes", semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES),
        ("configValueBytes", crate::editor::home::config::HOME_CONFIG_VALUE_BYTES),
        ("configBaseBytes", crate::editor::home::config::HOME_CONFIG_BASE_BYTES),
        ("commandStepBytes", contract.max_output_bytes),
        ("storeStepBytes", crate::editor::home::config::HOME_CONFIG_STEP_BYTES),
        ("workItems", HOME_RETAINED_WORK_ITEMS),
    ] {
        assert_eq!(fixture["limits"][key].as_u64(), Some(expected as u64), "retained limit {key}");
    }
    assert_eq!(contract.max_raw_wire_bytes, HOME_RETAINED_RAW_BYTES);
    for case in fixture["boundaryCases"].as_array().expect("boundary cases") {
        let value = "x".repeat(case["bytes"].as_u64().expect("byte count") as usize);
        let command = HomeCommand::OpenSpace(open_space::OpenSpace { space_id: value });
        let first_party = pack::json_from_dsl_value(&dsl::ToValue::to_value(&command));
        let oracle: serde_json::Value = serde_json::from_str(&pack::json_to_string(&first_party)).expect("third-party JSON decode");
        let oracle_wire = serde_json::to_string(&oracle).expect("third-party JSON encode");
        assert_eq!(pack::parse_json(&oracle_wire).expect("first-party JSON decode"), first_party);
        let decoded: HomeCommand = dsl::from_dsl_value(pack::json_to_dsl_value(&first_party)).expect("command value decode");
        assert_eq!(decoded, command);
        assert_eq!(home_retained_extent(&decoded, &SHomeSnapshot::default(), &protocol::InteractionState::default()).is_some(), case["accepted"].as_bool().expect("admission oracle"));
    }
}

#[test]
fn every_migrated_home_route_has_an_exact_scalar_boundary() {
    let scalar = |bytes: usize| "x".repeat(bytes);
    let pairs = [
        (HomeCommand::OpenSpace(open_space::OpenSpace { space_id: scalar(4096) }), HomeCommand::OpenSpace(open_space::OpenSpace { space_id: scalar(4097) })),
        (HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { node_id: scalar(4096) }), HomeCommand::NavigateVirtualFileSystemNode(navigate_virtual_file_system_node::NavigateVirtualFileSystemNode { node_id: scalar(4097) })),
        (HomeCommand::CreateSpace(create_space::CreateSpace { name: scalar(4094), kind: "k".into(), visibility: "v".into() }), HomeCommand::CreateSpace(create_space::CreateSpace { name: scalar(4095), kind: "k".into(), visibility: "v".into() })),
        (HomeCommand::DeleteSpace(delete_space::DeleteSpace { space_id: scalar(4096), confirmed: true }), HomeCommand::DeleteSpace(delete_space::DeleteSpace { space_id: scalar(4097), confirmed: true })),
        (HomeCommand::ShareSpace(share_space::ShareSpace { space_id: scalar(4094), email: "e".into(), role: "r".into() }), HomeCommand::ShareSpace(share_space::ShareSpace { space_id: scalar(4095), email: "e".into(), role: "r".into() })),
        (HomeCommand::ManageSpace(manage_space::ManageSpace { space_id: scalar(4096) }), HomeCommand::ManageSpace(manage_space::ManageSpace { space_id: scalar(4097) })),
        (HomeCommand::CopyInviteLink(copy_invite_link::CopyInviteLink { space_id: scalar(4095), role: "r".into(), ttl_secs: u64::MAX }), HomeCommand::CopyInviteLink(copy_invite_link::CopyInviteLink { space_id: scalar(4096), role: "r".into(), ttl_secs: u64::MAX })),
        (HomeCommand::CreateStudio(create_studio::CreateStudio { name: scalar(4095), kind: "k".into(), folder_path: None }), HomeCommand::CreateStudio(create_studio::CreateStudio { name: scalar(4096), kind: "k".into(), folder_path: None })),
    ];
    let snapshot = SHomeSnapshot::default();
    let interaction = protocol::InteractionState::default();
    for (at_limit, overflow) in pairs {
        assert_eq!(home_retained_extent(&at_limit, &snapshot, &interaction), Some(1), "{} must admit the exact scalar limit", at_limit.command_id());
        assert_eq!(home_retained_extent(&overflow, &snapshot, &interaction), None, "{} must reject one byte beyond the scalar limit", overflow.command_id());
    }
    for zero in [HomeCommand::GoHome(go_home::GoHome {}), HomeCommand::PresenceHeartbeat(presence_heartbeat::PresenceHeartbeat {})] {
        assert_eq!(home_retained_extent(&zero, &snapshot, &interaction), Some(1), "{} carries no scalar payload", zero.command_id());
    }
    // 📏️ The one route judged against the RETAINED WIRE budget instead of the scalar one: a sealed
    // directory page is a `HostOnly` machine payload the hub already bounds with `hasMore`, and a
    // 4 KiB cap would refuse an ordinary page of a dozen spaces (ticket 26/09/18 S4).
    let page = |bytes: usize| HomeCommand::ApplyDirectoryEventPage(apply_directory_event_page::ApplyDirectoryEventPage { page_json: scalar(bytes) });
    assert_eq!(home_retained_extent(&page(HOME_RETAINED_SCALAR_BYTES + 1), &snapshot, &interaction), Some(1), "a page beyond the scalar cap is still admitted");
    assert_eq!(home_retained_extent(&page(HOME_RETAINED_RAW_BYTES), &snapshot, &interaction), Some(1), "the exact retained wire budget is admitted");
    assert_eq!(home_retained_extent(&page(HOME_RETAINED_RAW_BYTES + 1), &snapshot, &interaction), None, "one byte beyond the retained wire budget is refused");
}

//#endregion 🧪️RetainedCommandEnvelope

use semio_framework_artifact_space_space::{S_SPACE_SCHEMA, SpaceKind, SpaceVisibility, empty_space_snapshot};
use semio_framework_os::{LocalStorageBackbonePort, OsBackbonePorts, OsSpaceDocument, create_backbone_document, load_os_space_document, seed_os_space_catalog_if_empty};
use std::sync::Arc;

fn empty_history() -> semio_framework_plugin::HistoryView {
    semio_framework_plugin::HistoryView::empty()
}

fn home_view(user_id: &str, locale: semio_framework_plugin::Locale) -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        locale,
        session_identity: Some(semio_framework_plugin::ViewSessionIdentity { user_id: user_id.into(), display_name: "Ada".into() }),
        ..Default::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn home_manifest_derives_the_canonical_surface_id() {
    let definition = create_home_app().await;
    assert_eq!(definition.id, semio_framework::surface_app_id(&HomeApp::DIALECT.into(), semio_framework::AppRole::Editor));
    assert_eq!(definition.controller_id, "s.space.home@1/*#editor");
}

#[semio_framework_async_macros::async_test]
async fn home_declares_create_space_action() {
    let definition = create_home_app().await;
    let main = definition.window_kinds.iter().find(|window| window.id == crate::editor::home::modes::explore::windows::main::S_HOME_WINDOW).expect("home main window");
    assert!(main.actions.iter().any(|action| action.id == "createStudio"));
}

#[semio_framework_async_macros::async_test]
async fn space_document_persists_through_backbone_port() {
    // 🕳️ `parse_demo_space_document()` yields a `workflow::WorkflowSnapshot` (the demo fixture's own
    // artifact content), not a `space::SpaceSnapshot`-backed catalog entry
    // `seed_os_space_catalog_if_empty` expects. This test exercises the space-manifest persistence
    // path specifically, so it mints its own manifest instead.
    let port = Arc::new(OsBackbonePorts::Store(store::BackbonePorts::LocalStorage(LocalStorageBackbonePort::default())));
    let projection = empty_space_snapshot("Persist Test", SpaceKind::Atelier, SpaceVisibility::Private);
    let demo: OsSpaceDocument = create_backbone_document(S_SPACE_SCHEMA, "persist-test", "Persist Test", projection);
    let _ = seed_os_space_catalog_if_empty(demo, &port).expect("seed");
    let loaded = load_os_space_document("persist-test", &port).expect("load");
    assert_eq!(loaded.id, "persist-test");
    assert_eq!(loaded.name, "Persist Test");
}

/// 🧪️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the pre-ticket version of
/// these two tests asserted on the VFS scene's ALWAYS-present `emptyMessage` field, which happened
/// to make them incidentally immune to `crate::list_all_space_catalog_entries()`'s process-global
/// catalog singleton being polluted by other tests in this same test binary. The new table render
/// has no such structural field (`TableView` carries no message), so these are rewritten to fold a
/// KNOWN directory event (deterministic, independent of the global catalog) and assert on the
/// locale-correct COLUMN HEADERS instead — the real thing "labels resolve to the right locale" means
/// for a table.
/// 🙋️ `space.created` alone leaves `DirectorySpace.members` EMPTY (`📇️directory/🦀️.rs:98-118`), so a
/// fixture folded from it gives every caller `role: None` and no row can offer a role-scoped
/// affordance. The membership event is folded too, making `u1` the author and `u2` a stranger — which
/// is what the role law below actually needs to be able to distinguish (ticket 26/09/18, S3: that law
/// asserted `manageSpace` against a memberless fixture and had never run, because the assertion above
/// it stopped the test first).
async fn directory_with_one_folded_space() -> HomeTransient {
    let created = pack::json!({
        "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
        "body": {"kind": "space.created", "spaceId": "sp-1", "name": "Fixture", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"},
        "recordedAtMs": 1000
    })
    .to_string();
    let member = pack::json!({
        "seq": 2, "id": "evt-2", "hlc": {"physicalMs": 0, "logical": 1}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-1",
        "body": {"kind": "member.upserted", "spaceId": "sp-1", "userId": "u1", "role": "author"},
        "recordedAtMs": 1001
    })
    .to_string();
    let events = [created, member].iter().map(|event| pack::from_json_str::<store::os_directory::DirectoryEvent>(event).expect("fixture directory event")).collect::<Vec<_>>();
    let mut page = store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 1,
        after_seq_exclusive: 0,
        through_seq_inclusive: 2,
        has_more: false,
        events,
        receipt_sha256: String::new(),
    };
    page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    HomeTransient::with_directory(HomeTransient::default().directory().fold_page(&page))
}

#[semio_framework_async_macros::async_test]
async fn home_labels_resolve_native_english_by_default() {
    let transient = directory_with_one_folded_space().await;
    let view_state = home_view("u1", semio_framework_plugin::Locale::En);
    let home_node = render_body(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &HomeConfig::default(), transient.directory(), &view_state).expect("English Home assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(home_node).expect("English Home tree projection");
    assert!(json.contains("Updated"), "English column header must resolve: {json}");
    assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
}

#[semio_framework_async_macros::async_test]
async fn home_labels_resolve_native_german_locale() {
    let transient = directory_with_one_folded_space().await;
    let view_state = home_view("u1", semio_framework_plugin::Locale::De);
    let home_node = render_body(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &HomeConfig::default(), transient.directory(), &view_state).expect("German Home assembly");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(home_node).expect("German Home tree projection");
    assert!(json.contains("Aktualisiert"), "German column header must resolve: {json}");
    assert!(json.contains("Fixture"), "the folded space's name must render: {json}");
}

/// 🪪️ Signed out is a STATE: the landing window must publish for a visitor with no identity — that is
/// the ordinary first paint of a hub-configured shell — and the projection is the assertion, because a
/// tree that merely builds and is then refused by the UI document publishes nothing at all (ticket
/// 26/09/18: S2 §3.4 removed the refusal, S3 removed the `DuplicateSiblingKey` underneath it).
/// Identity still decides the ROWS and the role-scoped affordances, which is the other half here.
#[semio_framework_async_macros::async_test]
async fn home_render_publishes_signed_out_and_changes_roles_with_the_identity() {
    let transient = directory_with_one_folded_space().await;
    let config = HomeConfig::default();
    let render = |view: &semio_framework_plugin::ViewModel| render_body(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, &config, transient.directory(), view);
    let anonymous = render(&semio_framework_plugin::ViewModel::default()).expect("signed-out render");
    let anonymous_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(anonymous).expect("signed-out projection");
    assert!(anonymous_json.contains("s-home-create-space"), "the signed-out landing window still publishes its own body: {anonymous_json}");
    assert!(!anonymous_json.contains("manageSpace"), "a signed-out visitor owns no space and gets no administration affordance: {anonymous_json}");
    let author = render(&home_view("u1", semio_framework_plugin::Locale::En)).expect("author render");
    let author_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(author).expect("author projection");
    let foreign = render(&home_view("u2", semio_framework_plugin::Locale::En)).expect("foreign render");
    let foreign_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(foreign).expect("foreign projection");
    assert!(author_json.contains("manageSpace"));
    assert!(!foreign_json.contains("manageSpace"));
}

//#region 🫧️DirectoryTransient
/// 📬️ End to end over a registered Home editor, dispatched exactly as the host feeds a page: the sealed page is admitted as
/// a retained job that publishes ONE transient item plus the receipt event — never a config lane, so no history row — and
/// the host-facing render lists the folded space. A second page with the frontier the projection already holds answers the
/// same receipt and publishes no transient item at all.
#[semio_framework_async_macros::async_test]
async fn a_dispatched_page_publishes_one_transient_item_and_no_history_row() {
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::PluginApp;
    let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<HomeApp>>::with_registry(Default::default(), semio_framework_plugin::AppActionRegistry::from_definition(&create_home_app().await)).await;
    app.bind_instance_id(1).await;
    let view = home_view("u1", semio_framework_plugin::Locale::En);
    let meta = semio_framework_plugin::ActionMeta { actor: "local".into(), instance_id: 1, view_state: Some(view.clone()) };
    let page_json = pack::to_json_string(&{
        let events = [
            pack::json!({ "seq": 1, "id": "evt-1", "hlc": {"physicalMs": 0, "logical": 0}, "actor": {"kind": "user", "id": "u"}, "spaceId": "sp-e2e", "body": {"kind": "space.created", "spaceId": "sp-e2e", "name": "Transient Fixture", "spaceKind": "atelier", "visibility": "private", "ownerUserId": "u1"}, "recordedAtMs": 1000 }),
        ]
        .iter()
        .map(|event| pack::from_json_str::<store::os_directory::DirectoryEvent>(&event.to_string()).expect("fixture directory event"))
        .collect::<Vec<_>>();
        let mut page = store::os_directory::DirectoryEventPageV1 { schema: "semio.directory.event-page.v1".into(), session_binding_sha256: "a".repeat(64), authorization_generation: 1, after_seq_exclusive: 0, through_seq_inclusive: 1, has_more: false, events, receipt_sha256: String::new() };
        page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
        page
    });
    let args = DslValue::Object(vec![("pageJson".into(), DslValue::String(page_json))]);
    for (turn, expects_item) in [(1, true), (2, false)] {
        app.handle_action("applyDirectoryEventPage", Some(&args), &meta).await.unwrap_or_else(|fault| panic!("page {turn} admission: {fault:?}"));
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1).await.unwrap_or_else(|fault| panic!("page {turn} settles: {fault:?}"));
        assert!(!settled.lanes.contains(&TypedOperationResultLane::Config) && !settled.lanes.contains(&TypedOperationResultLane::Artifact), "page {turn}: a directory page never writes a history lane: {:?}", settled.lanes);
        assert_eq!(settled.lanes.contains(&TypedOperationResultLane::Transient), expects_item, "page {turn}: {:?}", settled.lanes);
        let receipt = settled.events.iter().find(|event| event.kind == crate::editor::home::transient::DirectoryProjectionReceiptV1::SCHEMA).expect("the terminal receipt event");
        let receipt: crate::editor::home::transient::DirectoryProjectionReceiptV1 = protocol::FromValue::from_value(receipt.payload.clone()).expect("typed receipt");
        assert_eq!(receipt.through_seq_inclusive, 1);
    }
    let rendered = app.render(crate::editor::home::modes::explore::windows::main::S_HOME_BODY, None, &view).await.expect("host-facing render");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(rendered).expect("render projection");
    assert!(json.contains("Transient Fixture"), "the host-facing render lists the folded space: {json}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
//#endregion 🫧️DirectoryTransient

//#region 💾️CatalogCommands
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep};

fn catalog_home() -> SHomeSnapshot {
    SHomeSnapshot { schema: "s.home".into(), catalog_generation: 4 }
}

fn studio_dsl(name: &str) -> String {
    <semio_framework_artifact_space_space::SpaceSnapshot as store::ArtifactDsl>::print_dsl(&empty_space_snapshot(name, SpaceKind::Atelier, SpaceVisibility::Private))
}

fn catalog_entries_named(name: &str) -> usize {
    semio_framework_plugin::resolve_ready(crate::list_all_space_catalog_entries()).iter().filter(|entry| entry.name == name).count()
}

fn refused_as(result: Result<ArtifactCommandWorkStep<EditorApp<HomeApp>>, Fault>, code: &str) -> bool {
    matches!(result, Err(fault) if format!("{fault:?}").contains(code))
}

/// 📏️ The three catalog routes are judged against the same ceilings as their siblings: a bind or a removal carries
/// public-invocation scalars, an import carries one authored studio text up to the retained wire budget.
#[test]
fn the_catalog_routes_admit_their_exact_ceilings() {
    let snapshot = SHomeSnapshot::default();
    let interaction = protocol::InteractionState::default();
    let text = |bytes: usize| "x".repeat(bytes);
    let bind = |space: usize, path: usize| HomeCommand::BindSpaceFile(bind_space_file::BindSpaceFile { space_id: text(space), file_path: text(path) });
    let remove = |bytes: usize| HomeCommand::DeleteVirtualFileSystemNode(delete_virtual_file_system_node::DeleteVirtualFileSystemNode { node_id: text(bytes) });
    let import = |bytes: Option<usize>| HomeCommand::ImportSpace(import_space::ImportSpace { dsl: bytes.map(text) });
    assert_eq!(home_retained_extent(&bind(4095, 1), &snapshot, &interaction), Some(1));
    assert_eq!(home_retained_extent(&bind(4095, 2), &snapshot, &interaction), None);
    assert_eq!(home_retained_extent(&remove(HOME_RETAINED_SCALAR_BYTES), &snapshot, &interaction), Some(1));
    assert_eq!(home_retained_extent(&remove(HOME_RETAINED_SCALAR_BYTES + 1), &snapshot, &interaction), None);
    assert_eq!(home_retained_extent(&import(None), &snapshot, &interaction), Some(1));
    assert_eq!(home_retained_extent(&import(Some(HOME_RETAINED_RAW_BYTES)), &snapshot, &interaction), Some(1));
    assert_eq!(home_retained_extent(&import(Some(HOME_RETAINED_RAW_BYTES + 1)), &snapshot, &interaction), None);
    for tool in ["bindSpaceFile", "importSpace", "deleteVirtualFileSystemNode"] {
        assert!(HOME_RETAINED_TOOL_IDS.contains(&tool), "{tool} is a retained Home route");
    }
}

/// 📥️ The import job writes the catalog only in its commit stage: after `validate` no studio exists (a cancellation
/// there writes nothing), a job restored from the post-validate checkpoint commits exactly one studio and answers the
/// catalog generation bump, and a committed job never writes twice.
#[semio_framework_async_macros::async_test]
async fn the_import_job_writes_the_catalog_only_in_its_commit_stage() {
    let name = "SH1 staged import law";
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let command = HomeCommand::ImportSpace(import_space::ImportSpace { dsl: Some(studio_dsl(name)) });
    let mut work = HomeCatalogWork::new("importSpace");
    assert!(matches!(work.advance(&command, &doc).expect("validate stage"), ArtifactCommandWorkStep::Progress { stage: "space-home.catalog.validated", .. }));
    assert_eq!(catalog_entries_named(name), 0, "validate must not write the catalog");
    let mut checkpoint = [0_u8; 8];
    let written = ArtifactCommandWork::checkpoint(&work, &mut checkpoint).expect("post-validate checkpoint");
    let mut restored = HomeCatalogWork::new("importSpace");
    ArtifactCommandWork::restore(&mut restored, &checkpoint[..written]).expect("restore");
    let Ok(ArtifactCommandWorkStep::Complete(emit)) = restored.advance(&command, &doc) else { panic!("the restored job commits") };
    assert_eq!(catalog_entries_named(name), 1, "commit admits exactly one studio");
    assert_eq!(emit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation(5)]);
    assert!(refused_as(restored.advance(&command, &doc), "space-home-catalog-work-repeated"), "a committed job never writes twice");
}

/// 🚫️ Every import refusal is named and writes nothing; without text the job asks the host for the file.
#[semio_framework_async_macros::async_test]
async fn the_import_job_refuses_by_name_and_asks_the_host_for_a_missing_file() {
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let import = |dsl: Option<&str>| HomeCommand::ImportSpace(import_space::ImportSpace { dsl: dsl.map(str::to_owned) });
    assert!(refused_as(HomeCatalogWork::new("importSpace").advance(&import(Some("  ")), &doc), "s.home.import-space.empty"));
    assert!(refused_as(HomeCatalogWork::new("importSpace").advance(&import(Some("schema nothing-like-a-studio {{{")), &doc), "s.home.import-space.not-a-studio"));
    let Ok(ArtifactCommandWorkStep::Complete(emit)) = HomeCatalogWork::new("importSpace").advance(&import(None), &doc) else { panic!("a text-less import asks the host") };
    assert!(emit.artifact_mutations.is_empty());
    assert!(matches!(emit.effects.as_slice(), [semio_framework_plugin::Effect::RequestFileOpen { import_action, .. }] if import_action == "importSpace"));
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    assert!(matches!(import_space::handle(&import_space::ImportSpace { dsl: Some(studio_dsl("SH1 direct")) }, &doc, &cfg), Err(fault) if format!("{fault:?}").contains("s.home.import-space.requires-retained-job")), "the direct lane never writes the catalog");
    assert_eq!(catalog_entries_named("SH1 direct"), 0);
    assert!(refused_as(HomeCatalogWork::new("importSpace").advance(&import(Some(&studio_dsl("  "))), &doc), "s.home.import-space.unnamed"));
}

/// 📥️ The host's generic file-open import (`RequestFileOpen` → `{payload, name, chunk, chunkCount}`) decodes into the
/// retained import: one chunk carries the studio text, a manifest spanning several chunks is refused by name.
#[test]
fn the_host_file_open_import_decodes_one_chunk_and_refuses_more() {
    let args = |chunk_count: u32| pack::json_to_dsl_value(&pack::json!({ "payload": "schema os.space", "name": "studio.os", "chunk": 0, "chunkCount": chunk_count }));
    let Ok(HomeCommand::ImportSpace(import)) = HomeApp::command_from_action("importSpace", Some(&args(1))) else { panic!("one chunk decodes into the import") };
    assert_eq!(import.dsl.as_deref(), Some("schema os.space"));
    assert!(matches!(HomeApp::command_from_action("importSpace", Some(&args(2))), Err(fault) if format!("{fault:?}").contains("s.home.import-space.oversized")));
    assert!(matches!(HomeApp::command_from_action("importSpace", None), Ok(HomeCommand::ImportSpace(import_space::ImportSpace { dsl: None }))), "a bare import asks the host for the file");
}

/// 🔁️ A studio's own `.os` export (`export-studio-dsl`) imports back as a new catalog studio under the same name.
#[semio_framework_async_macros::async_test]
async fn a_studio_dsl_export_imports_back_under_its_name() {
    let name = "SH2 round trip law";
    let space_id = crate::create_and_register_ephemeral_studio(name, "u1", "Ada").await;
    let document = crate::resolve_studio_document(&space_id).await.expect("the fresh studio resolves");
    let exported = semio_framework_os::host::export_os_space_dsl(&document).expect("studio dsl export");
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let command = HomeCommand::ImportSpace(import_space::ImportSpace { dsl: Some(exported.dsl) });
    let mut work = HomeCatalogWork::new("importSpace");
    assert!(matches!(work.advance(&command, &doc), Ok(ArtifactCommandWorkStep::Progress { .. })));
    assert!(matches!(work.advance(&command, &doc), Ok(ArtifactCommandWorkStep::Complete(_))));
    assert_eq!(catalog_entries_named(name), 2, "the draft and its imported copy are both listed by name");
}

/// 📎️ The bind job validates the studio and the path before it writes; its commit writes the studio's document into the
/// file, re-lists it as a file-backed catalog studio and bumps the catalog generation.
#[semio_framework_async_macros::async_test]
async fn the_bind_job_validates_then_writes_the_studio_file_once() {
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let bind = |space_id: &str, file_path: &str| HomeCommand::BindSpaceFile(bind_space_file::BindSpaceFile { space_id: space_id.into(), file_path: file_path.into() });
    assert!(refused_as(HomeCatalogWork::new("bindSpaceFile").advance(&bind("sh1-no-such-studio", "/tmp/sh1.os"), &doc), "s.home.bind-space-file.unknown-studio"));
    assert!(refused_as(HomeCatalogWork::new("bindSpaceFile").advance(&bind("", "/tmp/sh1.os"), &doc), "s.home.bind-space-file.studio-invalid"));
    let space_id = crate::create_and_register_ephemeral_studio("SH1 bind law", "u1", "Ada").await;
    assert!(refused_as(HomeCatalogWork::new("bindSpaceFile").advance(&bind(&space_id, " "), &doc), "s.home.bind-space-file.path-invalid"));
    let stem = format!("sh1-bind-law-{space_id}");
    let file_path = std::env::temp_dir().join(format!("{stem}.os")).to_string_lossy().into_owned();
    let read_back = || {
        use semio_framework_os::OsBackbonePort as _;
        semio_framework_os::open_file_space_backbone(&file_path).and_then(|port| port.read(&format!("file://{file_path}"))).and_then(|payload| semio_framework_os::decode_backbone_payload::<semio_framework_artifact_space_space::SpaceSnapshot, semio_framework_artifact_space_space::SpaceMutation>(&payload, S_SPACE_SCHEMA))
    };
    let mut work = HomeCatalogWork::new("bindSpaceFile");
    assert!(matches!(work.advance(&bind(&space_id, &file_path), &doc).expect("validate stage"), ArtifactCommandWorkStep::Progress { .. }));
    assert!(read_back().is_err(), "validate must not touch the filesystem");
    let Ok(ArtifactCommandWorkStep::Complete(emit)) = work.advance(&bind(&space_id, &file_path), &doc) else { panic!("the bind job commits") };
    assert_eq!(emit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation(5)]);
    let bound = read_back().expect("commit writes the studio's document to its file backbone");
    assert_eq!(semio_framework_os::materialize_backbone_snapshot(&bound, &[]).expect("the bound studio materializes").name, "SH1 bind law", "a fresh file backbone reads the studio back");
    let listed = semio_framework_plugin::resolve_ready(crate::list_all_space_catalog_entries()).into_iter().find(|entry| entry.id == space_id).expect("the bound studio stays listed");
    assert!(!listed.backbone_uri.is_empty(), "the bound studio is a persisted catalog studio, no longer an ephemeral draft");
    assert_eq!(listed.name, "SH1 bind law", "the bound studio keeps its name");
    for entry in std::fs::read_dir(std::env::temp_dir()).into_iter().flatten().flatten() {
        if entry.file_name().to_string_lossy().starts_with(&stem) {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

/// 💾️ The persist job validates before it writes; its commit keeps the ephemeral studio in the folder, lists it as a
/// persisted catalog studio under its own id and name, retires the draft and bumps the catalog generation once. Without a
/// folder the job opens the folder dialog and writes nothing.
#[semio_framework_async_macros::async_test]
async fn the_persist_job_validates_then_keeps_the_studio_in_its_folder_once() {
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let persist = |space_id: &str, folder_path: Option<&str>| HomeCommand::PersistLocally(persist_locally::PersistLocally { space_id: space_id.into(), folder_path: folder_path.map(str::to_owned) });
    let space_id = crate::create_and_register_ephemeral_studio("SH2 persist law", "u1", "Ada").await;
    let Ok(ArtifactCommandWorkStep::Complete(dialog)) = HomeCatalogWork::new("persistLocally").advance(&persist(&space_id, None), &doc) else { panic!("a folder-less persist opens the dialog") };
    assert!(dialog.artifact_mutations.is_empty() && matches!(dialog.effects.as_slice(), [semio_framework_plugin::Effect::OpenDialog { dialog_id, .. }] if dialog_id == "persistLocally"));
    let folder = std::env::temp_dir().join(format!("sh2-persist-law-{space_id}"));
    let folder_path = folder.to_string_lossy().into_owned();
    let mut work = HomeCatalogWork::new("persistLocally");
    assert!(matches!(work.advance(&persist(&space_id, Some(&folder_path)), &doc), Ok(ArtifactCommandWorkStep::Progress { .. })));
    assert!(!folder.exists(), "validate must not touch the filesystem");
    let Ok(ArtifactCommandWorkStep::Complete(emit)) = work.advance(&persist(&space_id, Some(&folder_path)), &doc) else { panic!("the persist job commits") };
    assert_eq!(emit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation(5)]);
    assert!(folder.exists(), "commit writes the studio into its folder");
    let listed: Vec<_> = semio_framework_plugin::resolve_ready(crate::list_all_space_catalog_entries()).into_iter().filter(|entry| entry.id == space_id).collect();
    assert_eq!(listed.len(), 1, "the persisted studio is listed once");
    assert!(!listed[0].backbone_uri.is_empty() && listed[0].name == "SH2 persist law", "listed as a persisted catalog studio under its own name");
    assert!(refused_as(work.advance(&persist(&space_id, Some(&folder_path)), &doc), "space-home-catalog-work-repeated"), "a committed job never writes twice");
    let _ = std::fs::remove_dir_all(&folder);
}

/// 🚫️ A folder studio without a folder is refused by name, never answered with an empty success.
#[test]
fn a_folder_studio_without_a_folder_is_refused_by_name() {
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let identity = semio_framework_plugin::ViewSessionIdentity { user_id: "u1".into(), display_name: "Ada".into() };
    for folder_path in [None, Some(" ".to_owned())] {
        let created = create_studio::handle_with_identity(&create_studio::CreateStudio { name: "SH2 folderless".into(), kind: "folder".into(), folder_path }, &doc, &cfg, &identity);
        assert!(matches!(created, Err(fault) if fault.code.0.as_str() == "s.home.create-studio.folder-path-required"));
    }
}

/// 🪦️ Removing a local studio is ONE config event — a tombstone — never a catalog delete: the studio's document stays
/// listed by the catalog, Home stops listing it, the event's exact inverse lists it again, and every refusal is named.
#[semio_framework_async_macros::async_test]
async fn removing_a_local_studio_is_a_config_tombstone_that_keeps_the_studio() {
    use protocol::Mutation as _;
    let space_id = crate::create_and_register_ephemeral_studio("SH1 tombstone law", "u1", "Ada").await;
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let transient = directory_with_one_folded_space().await;
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let remove = |node_id: String| delete_virtual_file_system_node::DeleteVirtualFileSystemNode { node_id };
    let row = |node_id: &str| transient.directory().space(delete_virtual_file_system_node::local_studio_id(node_id));
    let emit = delete_virtual_file_system_node::handle_with_row(&remove(format!("studio:{space_id}")), &doc, &cfg, None).expect("retire the local studio");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "the removal performs no catalog or host IO");
    assert_eq!(emit.config_mutations, vec![HomeConfigMutation::RetireLocalStudio { space_id: space_id.clone() }]);
    let retired = emit.config_mutations[0].diff(&config).diff().clone();
    assert!(retired.is_local_studio_retired(&space_id));
    assert!(semio_framework_plugin::resolve_ready(crate::list_all_space_catalog_entries()).iter().any(|entry| entry.id == space_id), "the tombstone never erases the studio");
    let listed = |tombstones: &[String]| semio_framework_plugin::resolve_ready(crate::home_space_rows(transient.directory().spaces(), "u1", tombstones)).iter().any(|row| row.id == space_id);
    assert!(listed(&config.retired_local_studio_ids));
    assert!(matches!(delete_virtual_file_system_node::handle(&remove(format!("studio:{space_id}")), &doc, &cfg), Err(fault) if fault.code.0.as_str() == "s.home.delete-vfs-node.requires-retained-job"), "the direct lane cannot tell a hub space from a local studio");
    assert!(!listed(&retired.retired_local_studio_ids), "Home stops listing a retired studio");
    let inverse = emit.config_mutations[0].inverse(&config);
    assert_eq!(inverse, vec![HomeConfigMutation::RestoreLocalStudio { space_id: space_id.clone() }]);
    assert_eq!(inverse[0].diff(&retired).diff(), &config, "the exact inverse lists the studio again");
    let retired_cfg = ConfigView { snapshot: &retired, window: None };
    for (node_id, code, view) in [
        (format!("studio:{space_id}"), "s.home.delete-vfs-node.already-retired", &retired_cfg),
        ("studio:sh1-no-such-studio".to_owned(), "s.home.delete-vfs-node.unknown-local-studio", &cfg),
        ("sp-1".to_owned(), "s.home.delete-vfs-node.hub-space", &cfg),
        ("studio:".to_owned(), "s.home.delete-vfs-node.node-invalid", &cfg),
    ] {
        assert!(matches!(delete_virtual_file_system_node::handle_with_row(&remove(node_id.clone()), &doc, view, row(&node_id)), Err(fault) if format!("{fault:?}").contains(code)), "{code}");
    }
}
//#endregion 💾️CatalogCommands

//#region 🗃️LocalCatalogLane
fn kept_pair(space_id: &str) -> (String, String) {
    let document = semio_framework_plugin::resolve_ready(crate::resolve_studio_document(space_id)).expect("the studio resolves");
    let files = semio_framework_os::export_backbone_pack(&document).expect("the studio exports its pair");
    (protocol::base64_standard_encode(&files.pack), protocol::base64_standard_encode(&files.spr))
}

fn rehydrate(document_id: &str, pack: &str, spr: &str) -> HomeCommand {
    HomeCommand::ApplyLocalCatalogDocument(apply_local_catalog_document::ApplyLocalCatalogDocument { document_id: document_id.into(), pack: pack.into(), spr: spr.into() })
}

/// 🗃️ The host's re-hydration validates before it writes, then lists the kept studio once under its own id and name as a
/// persisted studio (the ephemeral draft of the same id is retired) and bumps the catalog generation; handing the same pair
/// back again re-lists nothing twice.
#[semio_framework_async_macros::async_test]
async fn the_host_rehydration_lists_the_kept_studio_once_under_its_name() {
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let space_id = crate::create_and_register_ephemeral_studio("SH2 kept law", "u1", "Ada").await;
    let (pack, spr) = kept_pair(&space_id);
    let mut work = HomeCatalogWork::new("applyLocalCatalogDocument");
    assert!(matches!(work.advance(&rehydrate(&space_id, &pack, &spr), &doc), Ok(ArtifactCommandWorkStep::Progress { .. })));
    let Ok(ArtifactCommandWorkStep::Complete(emit)) = work.advance(&rehydrate(&space_id, &pack, &spr), &doc) else { panic!("the re-hydration commits") };
    assert_eq!(emit.artifact_mutations, vec![crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation(5)]);
    assert!(emit.effects.is_empty(), "a re-hydration never asks the host to keep the studio again");
    let listed = |id: &str| semio_framework_plugin::resolve_ready(crate::list_all_space_catalog_entries()).into_iter().filter(|entry| entry.id == id).collect::<Vec<_>>();
    let rows = listed(&space_id);
    assert_eq!(rows.len(), 1, "the kept studio is listed once");
    assert!(!rows[0].backbone_uri.is_empty() && rows[0].name == "SH2 kept law", "listed as a persisted studio under its own name");
    let mut again = HomeCatalogWork::new("applyLocalCatalogDocument");
    assert!(matches!(again.advance(&rehydrate(&space_id, &pack, &spr), &doc), Ok(ArtifactCommandWorkStep::Progress { .. })));
    assert!(matches!(again.advance(&rehydrate(&space_id, &pack, &spr), &doc), Ok(ArtifactCommandWorkStep::Complete(_))));
    assert_eq!(listed(&space_id).len(), 1, "a repeated page re-lists nothing twice");
    let config = HomeConfig::default();
    let removal = delete_virtual_file_system_node::handle_with_row(&delete_virtual_file_system_node::DeleteVirtualFileSystemNode { node_id: format!("studio:{space_id}") }, &doc, &ConfigView { snapshot: &config, window: None }, None).expect("remove the kept studio from Home");
    let [semio_framework_plugin::Effect::ReplayShellCommand { action_id, args: Some(args) }] = removal.effects.as_slice() else { panic!("removing a kept studio unkeeps it: {:?}", removal.effects) };
    assert_eq!(action_id, "os.local-catalog.retire");
    assert_eq!(args.get("documentId").and_then(DslValue::as_str), Some(space_id.as_str()));
}

/// 🚫️ A malformed or mismatched pair is refused by name and writes nothing.
#[semio_framework_async_macros::async_test]
async fn the_host_rehydration_refuses_a_malformed_or_foreign_pair_by_name() {
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let space_id = crate::create_and_register_ephemeral_studio("SH2 kept refusal", "u1", "Ada").await;
    let (pack, spr) = kept_pair(&space_id);
    assert!(refused_as(HomeCatalogWork::new("applyLocalCatalogDocument").advance(&rehydrate(&space_id, "not base64!", &spr), &doc), "s.home.apply-local-catalog-document.encoding-invalid"));
    assert!(refused_as(HomeCatalogWork::new("applyLocalCatalogDocument").advance(&rehydrate("sh2-some-other-studio", &pack, &spr), &doc), "s.home.apply-local-catalog-document.mismatch"));
    assert!(refused_as(HomeCatalogWork::new("applyLocalCatalogDocument").advance(&rehydrate("", &pack, &spr), &doc), "s.home.apply-local-catalog-document.id-invalid"));
    assert!(matches!(apply_local_catalog_document::handle(&apply_local_catalog_document::ApplyLocalCatalogDocument { document_id: space_id.clone(), pack, spr }, &doc, &ConfigView { snapshot: &HomeConfig::default(), window: None }), Err(fault) if format!("{fault:?}").contains("requires-retained-job")), "the direct lane never writes the catalog");
}

/// 📮️ An import commit asks the host to keep exactly the imported studio: one `os.local-catalog.admit` carrying the new
/// studio's id, the device's own data folder and the studio's own pair (the pair decodes back to the imported studio).
#[semio_framework_async_macros::async_test]
async fn the_import_commit_asks_the_host_to_keep_the_imported_studio() {
    let name = "SH2 keep import law";
    let snapshot = catalog_home();
    let history = empty_history();
    let doc = ArtifactView::new(&snapshot, &history);
    let command = HomeCommand::ImportSpace(import_space::ImportSpace { dsl: Some(studio_dsl(name)) });
    let mut work = HomeCatalogWork::new("importSpace");
    assert!(matches!(work.advance(&command, &doc), Ok(ArtifactCommandWorkStep::Progress { .. })));
    let Ok(ArtifactCommandWorkStep::Complete(emit)) = work.advance(&command, &doc) else { panic!("the import commits") };
    let [semio_framework_plugin::Effect::ReplayShellCommand { action_id, args: Some(args) }] = emit.effects.as_slice() else { panic!("exactly one keep request: {:?}", emit.effects) };
    assert_eq!(action_id, "os.local-catalog.admit");
    let field = |key: &str| args.get(key).and_then(DslValue::as_str).map(str::to_owned).unwrap_or_default();
    let imported = semio_framework_plugin::resolve_ready(crate::list_all_space_catalog_entries()).into_iter().find(|entry| entry.name == name).expect("the import is listed");
    assert_eq!((field("documentId"), field("schema"), field("name"), field("storage"), field("target")), (imported.id.clone(), S_SPACE_SCHEMA.to_owned(), name.to_owned(), "folder".to_owned(), String::new()));
    let pack = protocol::base64_standard_decode(field("pack")).expect("the pack is base64");
    let spr = protocol::base64_standard_decode(field("spr")).expect("the spr is base64");
    let parsed: store::ParsedDocumentText<semio_framework_artifact_space_space::SpaceSnapshot, semio_framework_artifact_space_space::SpaceMutation> = semio_framework_plugin::resolve_ready(store::parse_document_pack(&pack, &spr)).expect("the kept pair parses");
    let envelope = parsed.into_envelope();
    let same = envelope.id == imported.id && envelope.schema == S_SPACE_SCHEMA;
    envelope.retire_unadopted();
    assert!(same, "the kept pair is the imported studio's own document");
}
//#endregion 🗃️LocalCatalogLane

