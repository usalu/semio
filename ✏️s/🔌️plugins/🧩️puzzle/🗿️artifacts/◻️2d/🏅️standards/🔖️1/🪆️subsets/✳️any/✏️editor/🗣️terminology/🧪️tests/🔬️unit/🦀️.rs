use semio_framework_ui_locale::Locale;
#[allow(unused_imports)]
use semio_framework_ui_locale::LabelText;
#[allow(unused_imports)]
use semio_framework_ui_locale::LocalizedLabel;
use crate::editor::puzzle2d::terminology::*;
use semio_framework_ui_locale::Terminology;

#[test]
fn label_resolution_covers_every_shared_view_axis() {
    for (locale, terminology) in [(Locale::En, Terminology::Native), (Locale::En, Terminology::Reuse), (Locale::De, Terminology::Native), (Locale::De, Terminology::Reuse)] {
        let view_state = semio_framework_plugin::ViewModel { locale, terminology, ..semio_framework_plugin::ViewModel::new(locale, terminology) };
        assert!(!puzzle2d_labels(&view_state).nodes.as_str().is_empty());
    }
}
