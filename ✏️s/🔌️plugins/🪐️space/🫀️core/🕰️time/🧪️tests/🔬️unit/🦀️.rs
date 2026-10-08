use super::*;

/// 🕰️ The UTC-minute cases of `🧫️fixtures/🕰️utc-minute/🔣️.json` (shared with the TS `Intl.DateTimeFormat`
/// oracle), and RFC 3339 `saved_at` stamps parsing to the same instants.
#[semio_framework_async_macros::async_test]
async fn utc_minute_text_matches_the_shared_fixture_in_both_languages() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🕰️utc-minute/🔣️.json")).expect("fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let epoch_ms = case["epochMs"].as_u64().expect("epochMs");
        assert_eq!(utc_minute_text(epoch_ms, semio_framework_ui_locale::Locale::En), case["en"].as_str().expect("en"), "{case}");
        assert_eq!(utc_minute_text(epoch_ms, semio_framework_ui_locale::Locale::De), case["de"].as_str().expect("de"), "{case}");
        if let Some(rfc3339) = case["rfc3339"].as_str() {
            assert_eq!(rfc3339_utc_epoch_ms(rfc3339), Some(epoch_ms - epoch_ms % 1_000), "{case}");
        }
    }
    for accepted in fixture["acceptedRfc3339"].as_array().expect("accepted") {
        assert_eq!(rfc3339_utc_epoch_ms(accepted["text"].as_str().expect("accepted text")), accepted["epochMs"].as_u64(), "{accepted}");
    }
    for refused in fixture["refusedRfc3339"].as_array().expect("refused") {
        assert_eq!(rfc3339_utc_epoch_ms(refused.as_str().expect("refused text")), None, "{refused}");
    }
}
