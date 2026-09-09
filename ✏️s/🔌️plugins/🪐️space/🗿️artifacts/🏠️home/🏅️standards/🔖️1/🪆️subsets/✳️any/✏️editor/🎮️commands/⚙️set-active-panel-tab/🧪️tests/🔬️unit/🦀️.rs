
use super::*;
use semio_framework_plugin::HistoryView;

#[semio_framework_async_macros::async_test]
async fn home_command_op_text_round_trips_every_variant() {
    use crate::editor::home::HomeCommand;
    store::os_store::test_support::assert_op_line_round_trip(&HomeCommand::SetActivePanelTab(SetActivePanelTab { tab_id: "tab-1".into() }));
}

#[semio_framework_async_macros::async_test]
async fn set_active_panel_tab_emits_config_operation() {
    let projection = SHomeSnapshot { schema: "s.home".into(), catalog_generation: 0 };
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let config = HomeConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let emit = handle(&SetActivePanelTab { tab_id: "tab-1".into() }, &doc, &cfg).expect("handle");
    assert_eq!(emit.config_mutations, vec![HomeConfigMutation::SetActivePanelTab { tab_id: "tab-1".into() }]);
    assert!(emit.artifact_mutations.is_empty());
}
