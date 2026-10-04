//! 🏛️ The framework notice law, driven by the language-agnostic fixture `🧫️fixtures/🧫️framework-notices/🔣️.json` that the
//! TypeScript twin (`🧪️tests/🧪️framework-notices/🟦️.ts`) validates with Ajv: the kernel carries exactly its rows, in order, each
//! in both locales, no row shadows a history-lane notice, and a guest fault under one of its codes resolves to its notice.

use super::*;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🧫️framework-notices/🔣️.json");

#[test]
fn the_framework_notices_mirror_the_fixture_in_both_locales() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("the framework notices fixture is JSON");
    let rows: Vec<(&str, &str, &str)> = fixture["notices"].as_array().expect("notices").iter().map(|row| (row["code"].as_str().expect("code"), row["en"].as_str().expect("en"), row["de"].as_str().expect("de"))).collect();
    assert_eq!(rows, FRAMEWORK_FAULT_NOTICE_LABELS.to_vec());
    for (code, en, de) in FRAMEWORK_FAULT_NOTICE_LABELS {
        assert_eq!(framework_fault_notice(code), Some((en, de)), "{code}");
        assert_eq!(history_notice(code), None, "{code} is no history-lane notice");
        assert!(!en.is_empty() && !de.is_empty() && en != de, "{code} names both locales");
        assert_eq!(crate::manifest::fault_notice_placeholders(en), crate::manifest::fault_notice_placeholders(de), "{code} names the same placeholders in both locales");
    }
    assert_eq!(framework_fault_notice("history.full"), None, "a history-lane refusal is the history table's");
}

#[test]
fn a_guest_fault_under_a_framework_code_resolves_to_its_notice_in_both_locales() {
    let fault = Fault::new(FaultOrigin::App, "mutation.too-large".to_string(), "batched preparation footprint exceeds its fixed item capacity");
    let (en, de) = framework_fault_notice("mutation.too-large").expect("the framework labels mutation.too-large");
    for (locale, expected) in [(semio_framework_ui_locale::Locale::En, en), (semio_framework_ui_locale::Locale::De, de)] {
        let notice = fault_notice(&fault, &[], semio_framework_ui_locale::Terminology::Native, locale).expect("a framework code always earns its notice");
        assert_eq!(notice, FaultNotice { code: "mutation.too-large".to_string(), text: expected.to_string() });
    }
}

/// 🎞️ LAW (S4-LOAD): a whole document of a foreign schema on a media edge is refused under `plugin.media.schema-mismatch` and its
/// notice names both schemas in both locales; without them it never shows an unfilled placeholder.
#[test]
fn a_foreign_document_schema_on_a_media_edge_names_both_schemas_in_both_locales() {
    let fault = Fault::new(FaultOrigin::Framework, "plugin.media.schema-mismatch".to_string(), "document schema mismatch").with_param("found", "s.draw.drawing").with_param("expected", "s.note.note");
    let (en, de) = framework_fault_notice("plugin.media.schema-mismatch").expect("the framework labels plugin.media.schema-mismatch");
    for (locale, template) in [(semio_framework_ui_locale::Locale::En, en), (semio_framework_ui_locale::Locale::De, de)] {
        let notice = fault_notice(&fault, &[], semio_framework_ui_locale::Terminology::Native, locale).expect("a schema mismatch always earns its notice");
        assert_eq!(notice, FaultNotice { code: "plugin.media.schema-mismatch".to_string(), text: template.replace("{found}", "s.draw.drawing").replace("{expected}", "s.note.note") });
    }
    let unnamed = Fault::new(FaultOrigin::Framework, "plugin.media.schema-mismatch".to_string(), "no schemas named");
    assert_eq!(fault_notice(&unnamed, &[], semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), None);
}

#[test]
fn a_channel_mismatch_names_both_versions_in_both_locales() {
    let fault = Fault::new(FaultOrigin::Framework, "plugin.channel-mismatch".to_string(), "the guest speaks app channel 20, the host app channel 21").with_param("guest", "20").with_param("host", "21");
    let (en, de) = framework_fault_notice("plugin.channel-mismatch").expect("the framework labels plugin.channel-mismatch");
    for (locale, template) in [(semio_framework_ui_locale::Locale::En, en), (semio_framework_ui_locale::Locale::De, de)] {
        let notice = fault_notice(&fault, &[], semio_framework_ui_locale::Terminology::Native, locale).expect("a channel mismatch always earns its notice");
        assert_eq!(notice, FaultNotice { code: "plugin.channel-mismatch".to_string(), text: template.replace("{guest}", "20").replace("{host}", "21") });
    }
    let unnamed = Fault::new(FaultOrigin::Framework, "plugin.channel-mismatch".to_string(), "no versions named");
    assert_eq!(fault_notice(&unnamed, &[], semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), None, "a notice never shows an unfilled placeholder");
}
