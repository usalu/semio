use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

fn doc_view<'a>(history: &'a HistoryView, doc_snapshot: &'a SHomeSnapshot) -> ArtifactView<'a, SHomeSnapshot> {
    ArtifactView::new(doc_snapshot, history)
}

fn hub_config(space_id: &str) -> HomeConfig {
    let event = store::os_directory::DirectoryEvent {
        seq: 1,
        id: "evt-1".into(),
        hlc: store::os_directory::Hlc { physical_ms: 0, logical: 0 },
        actor: store::os_directory::DirectoryActor { kind: store::os_directory::DirectoryActorKind::User, id: "u".into() },
        space_id: Some(space_id.into()),
        user_id: None,
        body: store::os_directory::DirectoryEventBody::SpaceCreated {
            space_id: space_id.into(),
            name: "Fabrication".into(),
            space_kind: store::os_directory::DirectorySpaceKind::Studio,
            visibility: store::os_directory::DirectorySpaceVisibility::Public,
            owner_user_id: "u1".into(),
        },
        recorded_at_ms: 1000,
    };
    let model = store::os_directory::fold(store::os_directory::DirectoryReadModel::default(), &event);
    HomeConfig {
        directory_json: crate::editor::home::config::directory_to_json(&model),
        ..HomeConfig::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn empty_email_opens_the_share_dialog() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = hub_config("sp-1");
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: String::new(), role: String::new() }, &doc, &cfg).expect("handle");
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "shareSpace"));
}

#[semio_framework_async_macros::async_test]
async fn ephemeral_local_only_blocks_share_with_accessible_notice() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "draft-1".into(), email: String::new(), role: String::new() }, &doc, &cfg).expect("handle");
    assert!(matches!(emit.effects.as_slice(), [Effect::OpenDialog { dialog_id, .. }] if dialog_id == "ephemeralShareBlocked"));
}

#[semio_framework_async_macros::async_test]
async fn email_and_role_relay_upsert_member() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = hub_config("sp-1");
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: "author".into() }, &doc, &cfg).expect("handle");
    let (action_id, args) = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { action_id, args } => Some((action_id.clone(), args.clone())),
            _ => None,
        })
        .expect("a ReplayShellCommand effect");
    assert_eq!(action_id, "os.directory.upsert-member");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args.expect("args"));
    assert_eq!(args_value["email"], "ada@semio.dev");
    assert_eq!(args_value["role"], "author");
}

#[semio_framework_async_macros::async_test]
async fn blank_role_defaults_to_spectator() {
    let history = HistoryView::empty();
    let doc_snapshot = SHomeSnapshot::default();
    let doc = doc_view(&history, &doc_snapshot);
    let config = hub_config("sp-1");
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&ShareSpace { space_id: "sp-1".into(), email: "ada@semio.dev".into(), role: String::new() }, &doc, &cfg).expect("handle");
    let args = emit
        .effects
        .iter()
        .find_map(|effect| match effect {
            Effect::ReplayShellCommand { args, .. } => args.clone(),
            _ => None,
        })
        .expect("args");
    let args_value: pack::JsonValue = pack::json_from_dsl_value(&args);
    assert_eq!(args_value["role"], "spectator");
}
