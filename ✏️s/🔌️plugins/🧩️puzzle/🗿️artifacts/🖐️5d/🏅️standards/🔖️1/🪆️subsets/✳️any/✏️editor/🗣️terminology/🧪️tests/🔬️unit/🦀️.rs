
use super::*;

#[test]
fn label_resolution_has_no_locale_or_terminology_default() {
    for (locale, terminology) in [("en-US", "native"), ("en", "reuse"), ("de-DE", "native"), ("de", "reuse")] {
        let mut config = Puzzle5dConfig::default();
        config.locale = locale.into();
        config.terminology = terminology.into();
        assert!(puzzle5d_labels(&config).is_some());
    }
    for locale in ["fr", "de-AT"] {
        let mut unsupported_locale = Puzzle5dConfig::default();
        unsupported_locale.locale = locale.into();
        assert!(puzzle5d_labels(&unsupported_locale).is_none());
        assert_eq!(puzzle5d_is_de_locale(&unsupported_locale), None);
    }
    let mut unsupported_terminology = Puzzle5dConfig::default();
    unsupported_terminology.terminology = "legacy".into();
    assert!(puzzle5d_labels(&unsupported_terminology).is_none());
}
