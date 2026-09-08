
use super::*;

#[test]
fn labels_resolve_every_host_locale_and_terminology_axis() {
    for (locale, terminology) in [(Locale::En, Terminology::Native), (Locale::En, Terminology::Reuse), (Locale::De, Terminology::Native), (Locale::De, Terminology::Reuse)] {
        let view_state = semio_framework_plugin::ViewModel { locale, terminology, ..Default::default() };
        assert!(!puzzle5d_labels(&view_state).parts.as_str().is_empty());
        assert_eq!(puzzle5d_is_de_locale(&view_state), locale == Locale::De);
    }
}
