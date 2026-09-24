use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

#[test]
fn empty_folder_opens_persist_dialog() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PersistLocally { space_id: "draft-1".into(), folder_path: None }, &doc, &cfg).expect("handle");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::OpenDialog { dialog_id, .. } if dialog_id == "persistLocally")));
}

#[test]
fn folder_path_bumps_catalog_without_shell_relay() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 3 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(
        &PersistLocally { space_id: "draft-1".into(), folder_path: Some("/tmp/does-not-need-to-exist-for-unit".into()) },
        &doc,
        &cfg,
    )
    .expect("handle");
    assert!(emit.effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })));
    assert!(emit.effects.iter().all(|effect| !matches!(effect, Effect::OpenDialog { .. })));
    assert_eq!(emit.artifact_mutations.len(), 1);
}
