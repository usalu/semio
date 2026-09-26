
use super::*;

fn home_view() -> semio_framework_plugin::ViewModel {
    semio_framework_plugin::ViewModel {
        session_identity: Some(semio_framework_plugin::ViewSessionIdentity { user_id: "u1".into(), display_name: "Ada".into() }),
        ..Default::default()
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
    let snapshot = SHomeSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = HomeConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let tree = <HomeViewer as ArtifactViewer>::render(main::S_HOME_VIEW_BODY, &doc, &cfg, &home_view()).expect("Home viewer main tree");
    let _ = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("Home viewer main projection");
}

#[semio_framework_async_macros::async_test]
async fn viewer_never_touches_the_document_store() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<HomeViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<crate::editor::home::HomeApp, HomeViewer>().await;
}

//#region 📬️DirectoryFeed
fn feed_page(fixture: &pack::JsonValue, row: &pack::JsonValue) -> store::os_directory::DirectoryEventPageV1 {
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

fn viewer_feed(config: &HomeConfig, page_json: &str) -> Result<Emit<SHomeMutation, HomeConfigMutation, NoDraftMutation>, Fault> {
    let snapshot = SHomeSnapshot::default();
    let history = HistoryView::empty();
    let state = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::new();
    let command = <HomeViewer as ArtifactViewer>::command_from_action("applyDirectoryEventPage", Some(&DslValue::Object(vec![("pageJson".into(), DslValue::String(page_json.into()))])))?;
    assert!(home_view_retained_extent(&command, &snapshot, &state).is_some(), "an ordinary page fits the retained wire budget");
    home_view_retained_reduce(&command, &snapshot, config, &history, &state, &hover, None, &AppOperationContext { app_instance_id: 1, parent_document_id: "s.home".into(), operation_id: 1, generation: 1, canonical_base_revision: [0; 32] })
}

fn editor_feed(config: &HomeConfig, page_json: &str) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let snapshot = SHomeSnapshot::default();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&snapshot, &history);
    crate::editor::home::commands::apply_directory_event_page::handle(&crate::editor::home::commands::apply_directory_event_page::ApplyDirectoryEventPage { page_json: page_json.into() }, &view, &ConfigView { snapshot: config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn the_viewer_folds_every_sealed_page_exactly_as_the_editor_does() {
    use protocol::Mutation as _;
    let fixture = pack::parse_json(include_str!("../../🧫️fixtures/📬️directory-feed/🔣️.json")).expect("language-neutral directory feed fixture");
    let mut viewer = HomeConfig::default();
    let mut editor = HomeConfig::default();
    for row in fixture["pages"].as_array().expect("pages") {
        let page_json = pack::to_json_string(&feed_page(&fixture, row));
        let from_viewer = viewer_feed(&viewer, &page_json).expect("viewer admits a sealed page");
        let from_editor = editor_feed(&editor, &page_json).expect("editor admits a sealed page");
        assert!(from_viewer.artifact_mutations.is_empty(), "a viewer page never mutates the document");
        assert_eq!(from_viewer.config_mutations, from_editor.config_mutations, "one page, one config answer on both surfaces");
        assert_eq!(from_viewer.events.len(), 1, "every accepted page answers its terminal receipt");
        viewer = from_viewer.config_mutations[0].diff(&viewer).diff().clone();
        editor = from_editor.config_mutations[0].diff(&editor).diff().clone();
        let oracle: serde_json::Value = serde_json::from_str(&viewer.directory_json).expect("third-party JSON reading of the projection");
        assert_eq!(oracle["cursor"].as_u64(), row["expected"]["cursor"].as_u64());
        let ids: Vec<&str> = oracle["spaces"].as_object().expect("spaces map").keys().map(String::as_str).collect();
        let expected: Vec<&str> = row["expected"]["spaceIds"].as_array().expect("expected ids").iter().map(|id| id.as_str().expect("id")).collect();
        assert_eq!(ids, expected);
        let receipt: crate::editor::home::config::DirectoryProjectionReceiptV1 = protocol::FromValue::from_value(from_viewer.events[0].payload.clone()).expect("typed viewer receipt");
        assert_eq!(receipt, viewer.directory_projection_receipt().expect("viewer receipt"));
    }
    assert_eq!(viewer, editor);
    for row in fixture["refused"].as_array().expect("refused") {
        let page_json = pack::to_json_string(&feed_page(&fixture, row));
        let Err(refused) = viewer_feed(&viewer, &page_json) else { panic!("a frontier race is refused") };
        assert_eq!(refused.message, row["fault"].as_str().expect("fault"));
        assert!(editor_feed(&editor, &page_json).is_err(), "the editor refuses the same page");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_viewer_declares_its_page_feed_as_a_chrome_only_config_route() {
    let def = create_home_viewer().await;
    let action = def.actions.iter().find(|action| action.id == "applyDirectoryEventPage").expect("the viewer declares the page feed");
    assert_eq!(action.semantics.audience, Some(semio_framework_plugin::CapabilityAudience::Chrome));
    assert_eq!(<HomeViewCommand as protocol::OpBinary>::TOOL_JOB_IDS, HOME_VIEW_TOOL_IDS);
    assert_eq!(<HomeViewer as ArtifactViewer>::bounded_first_step_tool_proofs().len(), HOME_VIEW_TOOL_IDS.len());
    assert!(<HomeViewCommandJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.iter().all(|contract| contract.lanes.len() == 1 && matches!(contract.lanes[0], ArtifactToolPublicationLane::Config)));
    let oversized = HomeViewCommand::ApplyDirectoryEventPage { page_json: "x".repeat(HOME_DIRECTORY_PAGE_BYTES + 1) };
    assert_eq!(home_view_retained_extent(&oversized, &SHomeSnapshot::default(), &protocol::InteractionState::default()), None);
    let encoded = protocol::OpBinary::encode_op(&HomeViewCommand::default()).expect("binary command");
    assert_eq!(<HomeViewCommand as protocol::OpBinary>::decode_op(&encoded).expect("binary round trip"), HomeViewCommand::default());
}
//#endregion 📬️DirectoryFeed
