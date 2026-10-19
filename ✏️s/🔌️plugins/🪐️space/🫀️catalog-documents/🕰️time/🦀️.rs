//! 🕰️ Shared UTC table formatting and catalog stamp parsing.
/// 🕰️ One UTC instant to the minute, in the order and punctuation of `locale`: `2026-09-25 21:05 UTC` (en) and
/// `25.09.2026, 21:05 UTC` (de) — the Home and space tables' "Updated" cell. The guest has no time zone, so it
/// says UTC instead of pretending to be local. Cases: `🧫️fixtures/🕰️utc-minute/🔣️.json` beside this module,
/// cross-checked against `Intl.DateTimeFormat` by its TS unit test.
pub fn utc_minute_text(epoch_ms: u64, locale: semio_framework_ui_locale::Locale) -> String {
    let minutes = epoch_ms / 60_000;
    let (hour, minute) = ((minutes / 60) % 24, minutes % 60);
    let days = (minutes / 1_440) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era = (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 { month_index + 3 } else { month_index - 9 };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    match locale {
        semio_framework_ui_locale::Locale::En => format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02} UTC"),
        semio_framework_ui_locale::Locale::De => format!("{day:02}.{month:02}.{year:04}, {hour:02}:{minute:02} UTC"),
    }
}

/// 🕰️ Epoch milliseconds (whole seconds) of a local catalog entry's RFC 3339 UTC `saved_at`
/// (`2026-09-25T21:05:00Z`, optional `.fraction`); `None` for the never-saved `"0"` and for anything else.
pub fn rfc3339_utc_epoch_ms(value: &str) -> Option<u64> {
    let bytes = value.as_bytes();
    let fraction = bytes.get(19..bytes.len().checked_sub(1)?)?;
    let digits = |range: std::ops::Range<usize>| bytes.get(range.clone()).filter(|part| part.iter().all(u8::is_ascii_digit)).and_then(|_| value.get(range)?.parse::<i64>().ok());
    if bytes.len() < 20 || bytes[4] != b'-' || bytes[7] != b'-' || bytes[10] != b'T' || bytes[13] != b':' || bytes[16] != b':' || bytes.last() != Some(&b'Z') {
        return None;
    }
    if !(fraction.is_empty() || (fraction.len() > 1 && fraction[0] == b'.' && fraction[1..].iter().all(u8::is_ascii_digit))) {
        return None;
    }
    let (year, month, day, hour, minute, second) = (digits(0..4)?, digits(5..7)?, digits(8..10)?, digits(11..13)?, digits(14..16)?, digits(17..19)?);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) || hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = shifted.div_euclid(400);
    let year_of_era = shifted.rem_euclid(400);
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    u64::try_from(((days * 24 + hour) * 60 + minute) * 60 + second).ok().map(|seconds| seconds * 1_000)
}
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
