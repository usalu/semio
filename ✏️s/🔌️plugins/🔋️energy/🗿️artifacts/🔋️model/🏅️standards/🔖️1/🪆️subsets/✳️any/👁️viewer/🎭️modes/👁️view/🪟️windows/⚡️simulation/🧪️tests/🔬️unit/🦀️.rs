use super::*;

fn text(node: BuiltNode) -> String {
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("window projects")
}

#[test]
fn viewer_is_structurally_read_only_and_localized() {
    let definition = definition();
    assert!(definition.actions.is_empty());
    assert_eq!(definition.label, LocalizedLabel::native("Energy results", "Energieergebnisse"));
}

#[test]
fn the_viewer_renders_the_model_run_period_in_both_languages() {
    let model = crate::model::Model::default();
    let text = text(render(&model));
    assert!(text.contains("energy-viewer-run-period"));
    assert!(text.contains("Simulationszeitraum"));
    assert!(text.contains("never change the document"));
}
