
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_space_index_editor_builds_a_definition_for_this_dialect() {
    let definition = create_space_index_editor();
    assert_eq!(definition.dialect, SPACE_INDEX_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<SpaceIndexEditor as ArtifactEditor>::DIALECT, SPACE_INDEX_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_mutation_action_is_registered() {
    let definition = create_space_index_editor();
    for command in [
        "createArtifact",
        "deleteArtifact",
        "renameArtifact",
        "touchArtifact",
        "requestDeleteArtifact",
        "openArtifact",
        "openArtifactWith",
        "inviteMember",
        "requestInviteMember",
        "removeMember",
        "setVisibility",
        "copyInviteLink",
        "foldDirectoryEvents",
        "presenceHeartbeat",
    ] {
        assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == command), "registry declares {command}");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_three_dialogs_are_registered_with_the_right_submit_actions() {
    let definition = create_space_index_editor();
    assert_eq!(definition.dialogs.len(), 3);
    let by_id = |id: &str| definition.dialogs.iter().find(|dialog| dialog.id == id).unwrap_or_else(|| panic!("dialog {id} must be registered"));
    assert_eq!(by_id("createArtifact").submit_action, ActionRef::new("createArtifact"));
    assert_eq!(by_id("createArtifact").args.iter().map(|arg| arg.id.as_str()).collect::<Vec<_>>(), ["name", "kindChoice"]);
    assert_eq!(by_id("deleteArtifact").submit_action, ActionRef::new("deleteArtifact"));
    assert_eq!(by_id("inviteMember").submit_action, ActionRef::new("inviteMember"));
    assert_eq!(by_id("inviteMember").args.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView};
    let snapshot = SSpaceSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot };
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(<SpaceIndexEditor as ArtifactEditor>::render("nope", &doc, &cfg).expect("unknown body diagnostic tree"), &semio_framework_plugin::ViewModel::default()).expect("unknown body projection");
    assert!(json.contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn the_members_panel_body_renders_through_the_editor_dispatch() {
    use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView};
    let snapshot = SSpaceSnapshot::default();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = SpaceIndexConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot };
    let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(<SpaceIndexEditor as ArtifactEditor>::render(members_panel::SPACE_INDEX_BODY_MEMBERS, &doc, &cfg).expect("members panel tree"), &semio_framework_plugin::ViewModel::default()).expect("members panel projection");
    assert!(json.contains("s-space-invite"));
}

/// 🆔️ Lane 4-F: `command_from_action` was entirely missing before this — every one of this app's
/// own actions dispatched via a plain button click (`onAction`/`handleAction`) hit the default
/// trait impl's unconditional error. Covers every declared action id, mirroring the sibling
/// `command_from_action_covers_every_declared_action_and_rejects_unknown_ones` convention other
/// artifact editors already use (e.g. `🌍️gis/🗿️artifacts/🗺️gismap`).
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    let cases: Vec<(&str, pack::JsonValue)> = vec![
        ("createArtifact", pack::json!({ "name": "First", "kindChoice": "{\"kindId\":\"s.gis.gismap\"}" })),
        ("deleteArtifact", pack::json!({ "id": "artifact-1" })),
        ("renameArtifact", pack::json!({ "id": "artifact-1", "newName": "Renamed" })),
        ("touchArtifact", pack::json!({ "id": "artifact-1", "nowMs": 2, "actor": "user:1" })),
        ("requestDeleteArtifact", pack::json!({ "id": "artifact-1" })),
        ("openArtifact", pack::json!({ "id": "artifact-1" })),
        ("openArtifactWith", pack::json!({ "id": "artifact-1", "role": "editor", "pluginId": "writer", "appId": "writer.editor" })),
        ("foldDirectoryEvents", pack::json!({ "eventsJson": "[]" })),
        ("presenceHeartbeat", pack::json!({ "artifactId": "artifact-1", "actorsCsv": "user:1" })),
        ("inviteMember", pack::json!({ "email": "a@example.com", "role": "author" })),
        ("removeMember", pack::json!({ "userId": "user:1" })),
        ("setVisibility", pack::json!({ "visibility": "public" })),
        ("copyInviteLink", pack::json!({ "role": "spectator", "ttlSecs": 604800u64 })),
        ("requestInviteMember", pack::json!({})),
    ];
    for (action, args) in cases {
        let args = pack::json_to_dsl_value(&args);
        let command = SpaceIndexEditor::command_from_action(action, Some(&args)).unwrap_or_else(|error| panic!("{action} must bridge: {error:?}"));
        assert_eq!(command.command_id(), action, "the bridged command's own id must round-trip");
    }
    assert!(SpaceIndexEditor::command_from_action("bogus", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn create_artifact_dialog_submission_preserves_the_exact_catalog_choice_in_the_host_relay() {
    let kind_choice = "{\"kindId\":\"s.gis.gismap\",\"schema\":\"gis.map\"}";
    let args = pack::json_to_dsl_value(&pack::json!({ "name": " First map ", "kindChoice": kind_choice }));
    let SpaceIndexCommand::CreateArtifact(command) = SpaceIndexEditor::command_from_action("createArtifact", Some(&args)).expect("dialog submission must bridge") else {
        panic!("expected CreateArtifact");
    };
    assert_eq!(command.kind_choice, kind_choice);
    let mut app = testkit::new_app().await;
    let result = app.dispatch_typed(SpaceIndexCommand::CreateArtifact(command), &semio_framework_plugin::testkit::meta("dialog")).await.expect("dialog submission");
    assert!(app.snapshot().expect("projection").artifacts.is_empty());
    let [Effect::ReplayShellCommand { action_id, args }] = result.requested_effects.as_slice() else {
        panic!("dialog submission must emit one ReplayShellCommand");
    };
    assert_eq!(action_id, "os.create-space-artifact");
    let args = pack::json_from_dsl_value(args.as_ref().expect("relay args"));
    assert_eq!(args, pack::json!({ "kindChoice": kind_choice, "name": "First map" }));
}

/// 🆔️ Lane 4-F: `#s-space-create-artifact`'s no-args click must bridge to an EMPTY `CreateArtifact`
/// payload (not error on missing fields) — its own handler treats empty `name`/`kindId` as "open
/// the dialog", mirroring Home's `createSpace`.
#[semio_framework_async_macros::async_test]
async fn command_from_action_bridges_an_empty_create_artifact_click() {
    let SpaceIndexCommand::CreateArtifact(payload) = SpaceIndexEditor::command_from_action("createArtifact", None).expect("no-args click must bridge") else { panic!("expected CreateArtifact") };
    assert_eq!(payload.name, "");
    assert_eq!(payload.kind_choice, "");
}
