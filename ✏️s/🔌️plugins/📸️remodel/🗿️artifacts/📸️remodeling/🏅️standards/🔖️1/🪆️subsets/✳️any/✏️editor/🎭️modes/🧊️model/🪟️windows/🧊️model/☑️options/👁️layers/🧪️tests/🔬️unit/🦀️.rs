
use super::*;
use crate::editor::remodeling::config::RemodelingConfig;

#[semio_framework_async_macros::async_test]
async fn every_layer_gets_its_own_toggle_and_a_default_open_group() {
    let config = RemodelingConfig::default();
    let labels = semio_framework_plugin::resolve_labels_for_locale::<RemodelingLabels>("en-US");
    let WindowMeasure::Group { children, default_open, .. } = measure(&config.layers, labels) else { panic!("expected a Group measure") };
    assert_eq!(children.len(), 5);
    assert_eq!(default_open, Some(true));
}
