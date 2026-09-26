//! 🕰️ Native system temporal presentation behind the renderer-neutral host formatter seam.

use crate::scenes::NativeHostTemporalFormatter;
use std::sync::Mutex;
use ui_contract::{HostHourCycleV1, HostTemporalFormatReplyV1, HostTemporalFormatRequestV1, HostTemporalLabelV1, HostTemporalProfileV1, HostTemporalSourceV1};

#[cfg(target_os = "macos")]
#[path = "🍎️macos/🦀️.rs"]
mod platform;
#[cfg(all(unix, not(target_os = "macos")))]
#[path = "🐧️unix/🦀️.rs"]
mod platform;
#[cfg(windows)]
#[path = "🪟️windows/🦀️.rs"]
mod platform;

#[derive(Default)]
struct ProfileState {
    signature: String,
    revision: u64,
}

pub struct PlatformTemporalBatch {
    locale: String,
    time_zone: String,
    hour_cycle: HostHourCycleV1,
    labels: Vec<String>,
}

trait PlatformTemporalAdapter {
    fn format(&self, request: &HostTemporalFormatRequestV1) -> Result<PlatformTemporalBatch, String>;
}

#[derive(Default)]
pub struct SystemTemporalFormatter {
    profile: Mutex<ProfileState>,
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let adjusted_year = year - i64::from(month <= 2);
    let era = adjusted_year.div_euclid(400);
    let year_of_era = adjusted_year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn parse_digits(value: &str, start: usize, length: usize) -> Option<i64> {
    value.get(start..start + length)?.parse().ok()
}

fn parse_iso_epoch_ms(input: &str) -> Option<i64> {
    let date_only;
    let value = if input.len() == 10 {
        date_only = format!("{input}T00:00:00Z");
        date_only.as_str()
    } else {
        input
    };
    if value.len() < 17 || value.as_bytes().get(4) != Some(&b'-') || value.as_bytes().get(7) != Some(&b'-') || value.as_bytes().get(10) != Some(&b'T') || value.as_bytes().get(13) != Some(&b':') {
        return None;
    }
    let year = parse_digits(value, 0, 4)?;
    let month = parse_digits(value, 5, 2)?;
    let day = parse_digits(value, 8, 2)?;
    let hour = parse_digits(value, 11, 2)?;
    let minute = parse_digits(value, 14, 2)?;
    let has_seconds = value.as_bytes().get(16) == Some(&b':');
    let second = if has_seconds { parse_digits(value, 17, 2)? } else { 0 };
    let month_days = [31, if is_leap_year(year) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if !(1..=12).contains(&month) || !(1..=month_days[usize::try_from(month - 1).ok()?]).contains(&day) || hour > 23 || minute > 59 || second > 59 {
        return None;
    }
    let mut cursor = if has_seconds { 19 } else { 16 };
    let mut millis = 0;
    if has_seconds && value.as_bytes().get(cursor) == Some(&b'.') {
        cursor += 1;
        let start = cursor;
        while value.as_bytes().get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor += 1;
        }
        let fraction = value.get(start..cursor)?;
        if fraction.is_empty() {
            return None;
        }
        millis = format!("{fraction:0<3}").get(..3)?.parse().ok()?;
    }
    let offset_seconds = match value.get(cursor..)? {
        "Z" => 0,
        offset if offset.len() == 6 && matches!(offset.as_bytes()[0], b'+' | b'-') && offset.as_bytes()[3] == b':' => {
            let hours: i64 = offset.get(1..3)?.parse().ok()?;
            let minutes: i64 = offset.get(4..6)?.parse().ok()?;
            if hours > 23 || minutes > 59 {
                return None;
            }
            let sign = if offset.as_bytes()[0] == b'-' { -1 } else { 1 };
            sign * (hours * 3_600 + minutes * 60)
        }
        _ => return None,
    };
    let seconds = days_from_civil(year, month, day).checked_mul(86_400)?.checked_add(hour * 3_600 + minute * 60 + second)?.checked_sub(offset_seconds)?;
    seconds.checked_mul(1_000)?.checked_add(millis)
}

fn source_epoch_ms(source: &HostTemporalSourceV1) -> Option<i64> {
    match source {
        HostTemporalSourceV1::EpochMs { timestamp_ms } => Some(*timestamp_ms),
        HostTemporalSourceV1::Iso { iso } => parse_iso_epoch_ms(iso),
    }
}

fn invalid_text(source: &HostTemporalSourceV1) -> String {
    match source {
        HostTemporalSourceV1::Iso { iso } => iso.clone(),
        HostTemporalSourceV1::EpochMs { .. } => "Invalid Date".to_string(),
    }
}

fn relative_amount(timestamp_ms: i64, now_ms: i64) -> (i64, &'static str) {
    let seconds = (timestamp_ms - now_ms) as f64 / 1_000.0;
    let absolute = seconds.abs();
    if absolute < 60.0 {
        (seconds.round() as i64, "second")
    } else if absolute < 3_600.0 {
        ((seconds / 60.0).round() as i64, "minute")
    } else if absolute < 86_400.0 {
        ((seconds / 3_600.0).round() as i64, "hour")
    } else if absolute < 2_592_000.0 {
        ((seconds / 86_400.0).round() as i64, "day")
    } else if absolute < 31_536_000.0 {
        ((seconds / 2_592_000.0).round() as i64, "month")
    } else {
        ((seconds / 31_536_000.0).round() as i64, "year")
    }
}

fn format_with_adapter(adapter: &dyn PlatformTemporalAdapter, profile: &Mutex<ProfileState>, request: &HostTemporalFormatRequestV1) -> Result<HostTemporalFormatReplyV1, String> {
    if !request.validate() {
        return Err("host-temporal-format.request-invalid".to_string());
    }
    let batch = adapter.format(request)?;
    if batch.labels.len() != request.values.len() || batch.locale.is_empty() || batch.time_zone.is_empty() {
        return Err("host-temporal-format.platform-reply-invalid".to_string());
    }
    let signature = format!("{}\0{}\0{:?}", batch.locale, batch.time_zone, batch.hour_cycle);
    let revision = {
        let mut state = profile.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.signature != signature {
            state.signature = signature;
            state.revision = state.revision.wrapping_add(1).max(1);
        }
        state.revision
    };
    let labels = request.values.iter().zip(batch.labels).map(|(value, text)| HostTemporalLabelV1 { id: value.id.clone(), text }).collect();
    let reply = HostTemporalFormatReplyV1 { profile: HostTemporalProfileV1 { locale: batch.locale, time_zone: batch.time_zone, hour_cycle: batch.hour_cycle, profile_revision: revision }, labels };
    reply.matches(request).then_some(reply).ok_or_else(|| "host-temporal-format.reply-invalid".to_string())
}

impl NativeHostTemporalFormatter for SystemTemporalFormatter {
    fn format_temporal_values(&self, request: &HostTemporalFormatRequestV1) -> Result<HostTemporalFormatReplyV1, String> {
        format_with_adapter(&platform::SystemPlatformAdapter, &self.profile, request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_contract::{HostTemporalFormatV1, HostTemporalValueV1};

    struct FixtureAdapter;

    impl PlatformTemporalAdapter for FixtureAdapter {
        fn format(&self, request: &HostTemporalFormatRequestV1) -> Result<PlatformTemporalBatch, String> {
            Ok(PlatformTemporalBatch { locale: "en-GB".into(), time_zone: "Europe/London".into(), hour_cycle: HostHourCycleV1::H23, labels: request.values.iter().map(|value| format!("platform:{}", value.id)).collect() })
        }
    }

    fn request() -> HostTemporalFormatRequestV1 {
        HostTemporalFormatRequestV1 {
            now_ms: 1_772_953_259_000,
            values: vec![
                HostTemporalValueV1 { id: "time".into(), source: HostTemporalSourceV1::EpochMs { timestamp_ms: 0 }, format: HostTemporalFormatV1::Time },
                HostTemporalValueV1 { id: "date-only".into(), source: HostTemporalSourceV1::Iso { iso: "2026-03-08".into() }, format: HostTemporalFormatV1::Date },
                HostTemporalValueV1 { id: "datetime".into(), source: HostTemporalSourceV1::Iso { iso: "2026-03-08T07:00:01+00:00".into() }, format: HostTemporalFormatV1::DateTime },
                HostTemporalValueV1 { id: "relative".into(), source: HostTemporalSourceV1::Iso { iso: "2026-03-08T07:00:00Z".into() }, format: HostTemporalFormatV1::Relative },
                HostTemporalValueV1 { id: "invalid".into(), source: HostTemporalSourceV1::Iso { iso: "not-an-instant".into() }, format: HostTemporalFormatV1::DateTime },
            ],
        }
    }

    #[test]
    fn profile_injected_platform_adapter_owns_every_label_and_revision() {
        let profile = Mutex::new(ProfileState::default());
        let first = format_with_adapter(&FixtureAdapter, &profile, &request()).expect("fixture adapter");
        let second = format_with_adapter(&FixtureAdapter, &profile, &request()).expect("stable fixture adapter");
        assert_eq!(first.profile.locale, "en-GB");
        assert_eq!(first.profile.time_zone, "Europe/London");
        assert_eq!(first.profile.profile_revision, second.profile.profile_revision);
        assert!(first.labels.iter().all(|label| label.text == format!("platform:{}", label.id)));
    }

    #[test]
    fn native_iso_parser_accepts_date_only_in_the_same_utc_domain_as_react_date() {
        assert_eq!(parse_iso_epoch_ms("1970-01-01"), Some(0));
        assert_eq!(parse_iso_epoch_ms("2026-03-08"), Some(1_772_928_000_000));
        assert_eq!(parse_iso_epoch_ms("2026-03-08T07:00Z"), Some(1_772_953_200_000));
        assert_eq!(parse_iso_epoch_ms("2026-03-08T07:00+01:00"), Some(1_772_949_600_000));
        assert_eq!(parse_iso_epoch_ms("2026-03-08T07:00"), None);
        assert_eq!(parse_iso_epoch_ms("2026-03-08T07:00:01"), None);
        assert_eq!(parse_iso_epoch_ms("+002026-03-08T07:00Z"), None);
        assert_eq!(parse_iso_epoch_ms("2026-02-30"), None);
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_foundation_matches_the_shared_intl_fixture_for_injected_profiles() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🖱️ui/🧬️contract/🕰️host-temporal-format/🧫️fixtures/🔣️.json")).expect("shared temporal fixture");
        for case in fixture["cases"].as_array().expect("fixture cases") {
            let request: HostTemporalFormatRequestV1 = serde_json::from_value(serde_json::json!({ "nowMs": case["nowMs"], "values": case["values"] })).expect("fixture request");
            let locale = case["locales"][0].as_str().expect("fixture locale");
            let time_zone = case["timeZone"].as_str().expect("fixture time zone");
            let adapter = platform::InjectedPlatformAdapter { locale, time_zone };
            let reply = format_with_adapter(&adapter, &Mutex::new(ProfileState::default()), &request).expect("Foundation reply");
            let expected = case["expected"].as_array().expect("fixture labels").iter().map(|label| label.as_str().unwrap()).collect::<Vec<_>>();
            assert_eq!(reply.labels.iter().map(|label| label.text.as_str()).collect::<Vec<_>>(), expected, "{}", case["id"]);
            assert_eq!(reply.profile.locale, locale);
            assert_eq!(reply.profile.time_zone, time_zone);
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn native_system_formatter_covers_every_generic_format_and_preserves_invalid_iso() {
        let request = request();
        let reply = SystemTemporalFormatter::default().format_temporal_values(&request).expect("native labels");
        assert!(reply.matches(&request));
        assert_eq!(reply.label("invalid"), Some("not-an-instant"));
        assert!(!reply.profile.locale.is_empty());
        assert!(!reply.profile.time_zone.is_empty());
    }
}
