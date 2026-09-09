use super::*;

#[test]
fn viewer_is_structurally_read_only_and_localized() {
    let definition = definition();
    assert!(definition.actions.is_empty());
    assert_eq!(definition.label, LocalizedLabel::native("Energy results", "Energieergebnisse"));
}

#[test]
fn an_unadopted_run_still_renders_the_model_run_period() {
    let model = crate::model::Model::default();
    let text = format!("{:?}", render(None, &model));
    assert!(text.contains("energy-viewer-run-period"));
    assert!(text.contains("Simulationszeitraum"));
}
