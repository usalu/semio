
use super::*;

#[test]
fn label_resolution_has_no_locale_or_terminology_default() {
    for (locale, terminology) in [("en-US", "native"), ("en", "reuse"), ("de-DE", "native"), ("de", "reuse")] {
        let mut config = Puzzle3dConfig::default();
        config.locale = locale.into();
        config.terminology = terminology.into();
        assert!(puzzle3d_labels(&config).is_some());
    }
    for locale in ["fr", "de-AT"] {
        let mut unsupported_locale = Puzzle3dConfig::default();
        unsupported_locale.locale = locale.into();
        assert!(puzzle3d_labels(&unsupported_locale).is_none());
    }
    let mut unsupported_terminology = Puzzle3dConfig::default();
    unsupported_terminology.terminology = "legacy".into();
    assert!(puzzle3d_labels(&unsupported_terminology).is_none());
}
