use super::*;
use crate::editor::remodeling::modes::model::windows::model::config::RemodelingModelWindowConfig;

#[semio_framework_async_macros::async_test]
async fn every_layer_gets_its_own_toggle_and_a_default_open_group() {
    let config = RemodelingModelWindowConfig::default();
    let labels = <RemodelingLabels as semio_framework_ui_locale::AppLabels>::labels(semio_framework_ui_locale::Locale::from_language_tag("en-US").expect("declared fixture locale"), semio_framework_ui_locale::Terminology::Native);
    let WindowMeasure::Group { children, default_open, .. } = measure(&config.layers, labels) else { panic!("expected a Group measure") };
    assert_eq!(children.len(), 5);
    assert_eq!(default_open, Some(true));
}
