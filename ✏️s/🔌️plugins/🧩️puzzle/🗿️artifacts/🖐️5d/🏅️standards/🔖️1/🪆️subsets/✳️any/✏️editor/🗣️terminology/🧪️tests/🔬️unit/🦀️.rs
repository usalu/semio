use super::*;

#[test]
fn labels_resolve_every_host_locale_and_terminology_axis() {
    for (locale, terminology) in [(Locale::En, Terminology::Native), (Locale::En, Terminology::Reuse), (Locale::De, Terminology::Native), (Locale::De, Terminology::Reuse)] {
        let view_state = semio_framework_plugin::ViewModel { locale, terminology, ..Default::default() };
        assert!(!puzzle5d_labels(&view_state).expect("admitted host axis").parts.as_str().is_empty());
        assert_eq!(puzzle5d_is_de_locale(&view_state), Some(locale == Locale::De));
    }
}

#[test]
fn every_authored_locale_tag_including_its_region_form_is_admitted() {
    for (tag, expected) in [("en", Locale::En), ("en-US", Locale::En), ("de", Locale::De), ("de-DE", Locale::De)] {
        for (terminology_tag, expected_terminology) in [("native", Terminology::Native), ("reuse", Terminology::Reuse)] {
            assert_eq!(puzzle5d_label_axes(tag, terminology_tag), Some((expected, expected_terminology)));
        }
    }
}

#[test]
fn an_unauthored_locale_or_terminology_fails_closed_instead_of_defaulting() {
    for tag in ["fr", "en_US", "EN", "de-AT", "", "e", "den"] {
        assert!(puzzle5d_label_axes(tag, "native").is_none(), "{tag} must not resolve a label set");
    }
    for terminology_tag in ["", "Native", "reuse-de", "brand"] {
        assert!(puzzle5d_label_axes("en", terminology_tag).is_none(), "{terminology_tag} must not resolve a label set");
    }
}

#[test]
fn the_fill_progress_status_label_is_authored_in_both_languages_and_both_terminologies() {
    assert_eq!(puzzle5d_label_axes("en", "native").map(|(l, t)| Puzzle5dLabels::labels(l, t).fill_progress.as_str()), Some("Fill progress"));
    assert_eq!(puzzle5d_label_axes("de-DE", "native").map(|(l, t)| Puzzle5dLabels::labels(l, t).fill_progress.as_str()), Some("Füllfortschritt"));
    assert_eq!(puzzle5d_label_axes("en-US", "reuse").map(|(l, t)| Puzzle5dLabels::labels(l, t).fill_progress.as_str()), Some("Fill progress"));
    assert_eq!(puzzle5d_label_axes("de", "reuse").map(|(l, t)| Puzzle5dLabels::labels(l, t).fill_progress.as_str()), Some("Füllfortschritt"));
}
