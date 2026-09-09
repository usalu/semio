//! 🧩️ The reserved refresh-section identities, pinned against the language-neutral fixture the
//! TypeScript mirror (`UI_REFRESH_SECTIONS` in `🛂️manifest/🟦️.ts`) reads from the same file.

use super::{UiRefreshSection, UI_REFRESH_SECTION_BODY_KEYS, UI_REFRESH_SECTION_KEYS};

#[semio_framework_async_macros::async_test]
async fn reserved_section_identities_match_the_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔬️ui-refresh-section/🔣️.json")).unwrap();
    let sections = fixture["sections"].as_array().unwrap();
    assert_eq!(sections.len(), UiRefreshSection::ALL.len());
    for (section, declared) in UiRefreshSection::ALL.into_iter().zip(sections) {
        assert_eq!(section.key(), declared["key"].as_str().unwrap());
        assert_eq!(section.body_key(), declared["bodyKey"].as_str().unwrap());
        assert_eq!(UiRefreshSection::from_body_key(section.body_key()), Some(section));
    }
    assert_eq!(UI_REFRESH_SECTION_KEYS.len(), UI_REFRESH_SECTION_BODY_KEYS.len());
    for authored in fixture["authoredBodyKeys"].as_array().unwrap() {
        assert_eq!(UiRefreshSection::from_body_key(authored.as_str().unwrap()), None);
    }
}
