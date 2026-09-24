use super::*;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, HistoryView};

#[test]
fn promote_relays_create_space_with_source_id() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&PromoteToHubSpace { space_id: "draft-1".into(), name: "Promoted".into() }, &doc, &cfg).expect("handle");
    assert!(emit.effects.iter().any(|effect| matches!(effect, Effect::ReplayShellCommand { action_id, .. } if action_id == "os.directory.create-space")));
}
