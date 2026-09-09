use super::*;
use semio_framework_plugin::Terminology;

#[test]
fn label_resolution_covers_every_shared_view_axis() {
    for (locale, terminology) in [(Locale::En, Terminology::Native), (Locale::En, Terminology::Reuse), (Locale::De, Terminology::Native), (Locale::De, Terminology::Reuse)] {
        let view_state = semio_framework_plugin::ViewModel { locale, terminology, ..Default::default() };
        assert!(!puzzle2d_labels(&view_state).nodes.as_str().is_empty());
    }
}
