use super::*;

#[test]
fn label_resolution_has_no_locale_or_terminology_default() {
    for (locale, terminology) in [("en-US", "native"), ("en", "reuse"), ("de-DE", "native"), ("de", "reuse")] {
        let mut config = Puzzle2dConfig::default();
        config.locale = locale.into();
        config.terminology = terminology.into();
        assert!(puzzle2d_labels(&config).is_some());
    }
    for locale in ["", "fr", "de-AT", "en-GB"] {
        let mut unsupported_locale = Puzzle2dConfig::default();
        unsupported_locale.locale = locale.into();
        assert!(puzzle2d_labels(&unsupported_locale).is_none());
        assert!(puzzle2d_config_locale(&unsupported_locale).is_none());
    }
    let mut unsupported_terminology = Puzzle2dConfig::default();
    unsupported_terminology.terminology = "legacy".into();
    assert!(puzzle2d_labels(&unsupported_terminology).is_none());
}
