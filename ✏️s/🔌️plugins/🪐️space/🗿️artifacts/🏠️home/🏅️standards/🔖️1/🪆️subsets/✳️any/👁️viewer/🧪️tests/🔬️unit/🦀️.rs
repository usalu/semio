use crate::{SHomeSnapshot, HOME_DIALECT};
use crate::{editor::home::commands::apply_directory_event_page::HomeDirectoryPageWork, editor::home::config::HomeConfig, editor::home::config::HOME_DIRECTORY_PAGE_BYTES, editor::home::transient::HomeTransient, editor::home::transient::HomeTransientMutation};
use crate::viewer::home::modes::view::windows::main;
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::ArtifactView;
use semio_plugin::ArtifactViewer;
use semio_plugin::ConfigView;
use semio_plugin::DslValue;
use semio_plugin::Fault;
use semio_plugin::Viewer;
use semio_plugin::ViewerApp;
use crate::viewer::home::{home_view_retained_extent, home_view_page_json};
use semio_framework_test_fixtures::fixture_text;

use crate::viewer::home::*;

fn home_view() -> semio_plugin::ViewModel {
    semio_plugin::ViewModel {
        session_identity: Some(semio_plugin::ViewSessionIdentity { user_id: "u1".into(), display_name: "Ada".into() }),
        ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    }
}

#[semio_framework_async_macros::async_test]
async fn create_home_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_home_viewer().await;
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, HOME_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<HomeViewer as ArtifactViewer>::DIALECT, HOME_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn renders_the_main_body_key_for_the_default_snapshot() {
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let snapshot = SHomeSnapshot::default();
    let history = semio_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = HomeConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let tree = <HomeViewer as ArtifactViewer>::render(main::S_HOME_VIEW_BODY, &doc, &cfg, &home_view()).expect("Home viewer main tree");
    let _ = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("Home viewer main projection");
}


#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<crate::editor::home::HomeApp, HomeViewer>().await;
}

//#region 📬️DirectoryFeed
fn feed_page(fixture: &semio_framework_pack_json::Value, row: &semio_framework_pack_json::Value) -> store::os_directory::DirectoryEventPageV1 {
    let events = row["spaces"]
        .as_array()
        .expect("page spaces")
        .iter()
        .map(|space| {
            let seq = space["seq"].as_u64().expect("event seq");
            let at_ms = i64::try_from(seq).expect("event instant");
            let id = space["id"].as_str().expect("space id").to_string();
            store::os_directory::DirectoryEvent {
                seq,
                id: format!("event-{seq}"),
                hlc: store::os_directory::Hlc { physical_ms: at_ms, logical: 0 },
                actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "user:u1#s1".into() },
                space_id: Some(id.clone()),
                user_id: None,
                body: store::os_directory::DirectoryEventBody::SpaceCreated {
                    space_id: id,
                    name: space["name"].as_str().expect("space name").into(),
                    space_kind: store::os_directory::DirectorySpaceKind::Studio,
                    visibility: store::os_directory::DirectorySpaceVisibility::Private,
                    owner_user_id: "u1".into(),
                },
                recorded_at_ms: at_ms,
            }
        })
        .collect();
    let mut page = store::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: fixture["sessionBindingSha256"].as_str().expect("binding").into(),
        authorization_generation: fixture["authorizationGeneration"].as_u64().expect("generation"),
        after_seq_exclusive: row["afterSeqExclusive"].as_u64().expect("after"),
        through_seq_inclusive: row["throughSeqInclusive"].as_u64().expect("through"),
        has_more: row["hasMore"].as_bool().expect("has more"),
        events,
        receipt_sha256: String::new(),
    };
    page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    page
}

/// 📬️ Feeds one sealed page through a REGISTERED surface exactly as the host does: dispatch, then settle the typed operation, so the job context carries the app's own Store issuers.
async fn feed_registered<P: semio_plugin::PluginApp>(app: &mut P, page_json: &str, meta: &semio_plugin::ActionMeta, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope) -> Result<semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt, String> {
    let args = DslValue::Object(vec![("pageJson".into(), DslValue::String(page_json.into()))]);
    semio_plugin::artifact_app_laws::dispatch_fixture_action(app, "applyDirectoryEventPage", Some(args), meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.map_err(|failure| format!("{failure:?}"))?;
    semio_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.map_err(|failure| format!("{failure:?}"))
}

fn receipt_of(settled: &semio_plugin::artifact_app_laws::TypedOperationFixtureReceipt) -> crate::editor::home::transient::DirectoryProjectionReceiptV1 {
    let event = settled.events.iter().find(|event| event.kind == crate::editor::home::transient::DirectoryProjectionReceiptV1::SCHEMA).expect("every accepted page answers its terminal receipt");
    semio_framework_value::FromValue::from_value(event.payload.clone()).expect("typed receipt")
}

#[semio_framework_async_macros::async_test]
async fn the_viewer_folds_every_sealed_page_exactly_as_the_editor_does() {
    use semio_plugin::PluginApp;
    let fixture = semio_framework_pack_json::parse(fixture_text!("../../🧪️tests/🧫️fixtures/📬️directory-feed/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-neutral directory feed fixture");
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let actor = || semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into());
    let mut viewer = semio_plugin::artifact_app_laws::new_registered_app::<ViewerApp<HomeViewer>, _>(async { semio_plugin::App { definition: create_home_viewer().await, examples: Vec::new() } }, actor(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    viewer.bind_instance_id(1).await;
    let mut editor = semio_plugin::artifact_app_laws::new_registered_app::<semio_plugin::EditorApp<crate::editor::home::HomeApp>, _>(async { semio_plugin::App { definition: crate::editor::home::create_home_app().await, examples: Vec::new() } }, actor(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    editor.bind_instance_id(1).await;
    let meta = semio_plugin::ActionMeta { expected_document_revision: None, actor: "local".into(), instance_id: 1, view_state: Some(std::sync::Arc::new(home_view())) };
    for row in fixture["pages"].as_array().expect("pages") {
        let page_json = semio_framework_pack_json::to_json_string(&feed_page(&fixture, row));
        let viewer_settled = feed_registered(&mut viewer, &page_json, &meta, &mut scope).await.expect("viewer admits a sealed page");
        let editor_settled = feed_registered(&mut editor, &page_json, &meta, &mut scope).await.expect("editor admits a sealed page");
        assert_eq!(viewer_settled.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Transient), editor_settled.lanes.contains(&semio_plugin::app::TypedOperationResultLane::Transient), "one page, one transient item on both surfaces");
        assert_eq!((viewer_settled.events.len(), editor_settled.events.len()), (1, 1), "every accepted page answers its terminal receipt");
        assert_eq!(receipt_of(&viewer_settled), receipt_of(&editor_settled), "both surfaces fold the same projection");
    }
    for row in fixture["refused"].as_array().expect("refused") {
        let page_json = semio_framework_pack_json::to_json_string(&feed_page(&fixture, row));
        let code = row["fault"].as_str().expect("fault");
        let refused = feed_registered(&mut viewer, &page_json, &meta, &mut scope).await.err().expect("a frontier race is refused");
        assert!(refused.contains(code), "viewer refusal names {code}: {refused}");
        let refused = feed_registered(&mut editor, &page_json, &meta, &mut scope).await.err().expect("the editor refuses the same page");
        assert!(refused.contains(code), "editor refusal names {code}: {refused}");
    }
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut viewer, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("viewer paid close");
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut editor, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("editor paid close");
}

#[semio_framework_async_macros::async_test]
async fn the_viewer_declares_its_page_feed_as_a_chrome_only_transient_route() {
    let def = create_home_viewer().await;
    let action = def.actions.iter().find(|action| action.id == "applyDirectoryEventPage").expect("the viewer declares the page feed");
    assert_eq!(action.semantics.audience, Some(semio_plugin::CapabilityAudience::Chrome));
    assert_eq!(<HomeViewCommand as protocol::OpBinary>::TOOL_JOB_IDS, HOME_VIEW_TOOL_IDS);
    assert_eq!(<HomeViewer as ArtifactViewer>::bounded_first_step_tool_proofs().len(), HOME_VIEW_TOOL_IDS.len());
    assert!(<HomeViewCommandJobFactory as semio_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().all(|contract| contract.lanes.len() == 1 && matches!(contract.lanes[0], ArtifactToolPublicationLane::Transient)));
    let oversized = HomeViewCommand::ApplyDirectoryEventPage { page_json: "x".repeat(HOME_DIRECTORY_PAGE_BYTES + 1) };
    assert_eq!(home_view_retained_extent(&oversized, &SHomeSnapshot::default(), &protocol::InteractionState::default()), None);
    let encoded = protocol::OpBinary::encode_op(&HomeViewCommand::default()).expect("binary command");
    assert_eq!(<HomeViewCommand as protocol::OpBinary>::decode_op(&encoded).expect("binary round trip"), HomeViewCommand::default());
}
/// 👁️ The Home viewer never mutates the document, draft or config — proven over the REGISTERED viewer surface, dispatched
/// exactly as the host feeds its sealed pages: every page settles as its transient item plus the receipt event and nothing
/// else, and the host-facing render lists every folded space. (The generic `assert_viewer_never_mutates` fixture admits only
/// lane-less viewers; this surface carries a transient lane.)
#[semio_framework_async_macros::async_test]
async fn a_viewer_page_publishes_only_its_transient_item() {
    use semio_plugin::app::TypedOperationResultLane;
    use semio_plugin::PluginApp;
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::artifact_app_laws::new_registered_app::<ViewerApp<HomeViewer>, _>(async { semio_plugin::App { definition: create_home_viewer().await, examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
    app.bind_instance_id(1).await;
    let fixture = semio_framework_pack_json::parse(fixture_text!("../../🧪️tests/🧫️fixtures/📬️directory-feed/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("language-neutral directory feed fixture");
    let pages = fixture["pages"].as_array().expect("pages");
    let meta = semio_plugin::ActionMeta { expected_document_revision:None, actor: "local".into(), instance_id: 1, view_state: Some(std::sync::Arc::new(home_view())) };
    for page in pages {
        let args = DslValue::Object(vec![("pageJson".into(), DslValue::String(semio_framework_pack_json::to_json_string(&feed_page(&fixture, page))))]);
        semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "applyDirectoryEventPage", Some(args.clone()), &meta, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("the viewer admits a sealed page");
        let settled = semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), scope.cx()).await.expect("the page settles");
        assert!(settled.lanes.iter().all(|lane| !matches!(lane, TypedOperationResultLane::Artifact | TypedOperationResultLane::Draft | TypedOperationResultLane::Config)), "a viewer page never writes the document, draft or config: {:?}", settled.lanes);
        assert!(settled.lanes.contains(&TypedOperationResultLane::Transient));
        assert!(settled.events.iter().any(|event| event.kind == crate::editor::home::transient::DirectoryProjectionReceiptV1::SCHEMA));
    }
    let json = semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app, main::S_HOME_VIEW_BODY, &home_view(), scope.cx()).await.expect("viewer projection");
    for space in pages.iter().flat_map(|page| page["spaces"].as_array().expect("page spaces")) {
        let name = space["name"].as_str().expect("space name");
        assert!(json.contains(name), "the viewer lists every folded space ({name}): {json}");
    }
    semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");
}
//#endregion 📬️DirectoryFeed
