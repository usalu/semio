//! 📃️ The pinned JSON Schema `format` policy: seven asserted string formats.

use super::PatternMatcher;

//#region 🏷️Format

/// 🏷️ The pinned `format` policy of this validator. Exactly these seven values are **assertions**,
/// implemented in-house and verified against the `ajv-formats` "full" oracle by the shared vector
/// corpus. Every other `format` value — including the `double`, `float`, `int64`, `uint32` and
/// `base64` spellings the repo's proto-derived modules carry — is an **annotation** that never
/// rejects an instance.
///
/// `regex` is asserted against the owned [`PatternMatcher`] subset, which is deliberately narrower
/// than ECMA-262: backreferences and lookbehind are rejected instead of accepted.
/// @see <https://datatracker.ietf.org/doc/html/rfc3339>
/// @see <https://ajv.js.org/guide/formats.html>
pub const ASSERTED_STRING_FORMATS: [&str; 7] = ["date", "date-time", "email", "regex", "time", "uri", "uuid"];

/// 🔎 Applies the pinned policy to one string: `Some(verdict)` for an asserted format, `None` when
/// the format is annotation-only and therefore carries no constraint at all.
pub fn string_format_matches(format: &str, value: &str) -> Option<bool> {
    match format {
        "date" => Some(is_rfc3339_date(value)),
        "date-time" => Some(is_rfc3339_date_time(value)),
        "time" => Some(is_rfc3339_time(value)),
        "email" => Some(is_email(value)),
        "uri" => Some(is_uri(value)),
        "uuid" => Some(is_uuid(value)),
        "regex" => Some(PatternMatcher::compile(value).is_ok()),
        _ => None,
    }
}

fn split_digits(value: &str, width: usize) -> Option<(u32, &str)> {
    let head = value.get(..width)?;
    if !head.chars().all(|entry| entry.is_ascii_digit()) {
        return None;
    }
    Some((head.parse::<u32>().ok()?, &value[width..]))
}

fn days_in_month(year: u32, month: u32) -> u32 {
    const DAYS: [u32; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if month == 2 && year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) {
        29
    } else {
        DAYS[month as usize]
    }
}

fn is_rfc3339_date(value: &str) -> bool {
    let Some((year, rest)) = split_digits(value, 4) else { return false };
    let Some(rest) = rest.strip_prefix('-') else { return false };
    let Some((month, rest)) = split_digits(rest, 2) else { return false };
    let Some(rest) = rest.strip_prefix('-') else { return false };
    let Some((day, rest)) = split_digits(rest, 2) else { return false };
    rest.is_empty() && (1..=12).contains(&month) && day >= 1 && day <= days_in_month(year, month)
}

fn is_rfc3339_time(value: &str) -> bool {
    let Some((hour, rest)) = split_digits(value, 2) else { return false };
    let Some(rest) = rest.strip_prefix(':') else { return false };
    let Some((minute, rest)) = split_digits(rest, 2) else { return false };
    let Some(rest) = rest.strip_prefix(':') else { return false };
    let Some((second, rest)) = split_digits(rest, 2) else { return false };
    let rest = match rest.strip_prefix('.') {
        Some(fraction) => {
            let digits = fraction.chars().take_while(|entry| entry.is_ascii_digit()).count();
            if digits == 0 {
                return false;
            }
            &fraction[digits..]
        }
        None => rest,
    };
    let (offset_hour, offset_minute, sign) = if rest.eq_ignore_ascii_case("z") {
        (0, 0, 1i64)
    } else {
        let (sign, rest) = match rest.as_bytes().first() {
            Some(b'+') => (1i64, &rest[1..]),
            Some(b'-') => (-1i64, &rest[1..]),
            _ => return false,
        };
        let Some((offset_hour, rest)) = split_digits(rest, 2) else { return false };
        if rest.is_empty() {
            (offset_hour, 0, sign)
        } else {
            let Some((offset_minute, tail)) = split_digits(rest.strip_prefix(':').unwrap_or(rest), 2) else { return false };
            if !tail.is_empty() {
                return false;
            }
            (offset_hour, offset_minute, sign)
        }
    };
    if offset_hour > 23 || offset_minute > 59 {
        return false;
    }
    if hour <= 23 && minute <= 59 && second < 60 {
        return true;
    }
    let utc_minute = i64::from(minute) - i64::from(offset_minute) * sign;
    let utc_hour = i64::from(hour) - i64::from(offset_hour) * sign - i64::from(utc_minute < 0);
    (utc_hour == 23 || utc_hour == -1) && (utc_minute == 59 || utc_minute == -1) && second < 61
}

fn is_rfc3339_date_time(value: &str) -> bool {
    let separators: Vec<(usize, char)> = value.char_indices().filter(|(_, entry)| *entry == 't' || *entry == 'T' || entry.is_whitespace()).collect();
    let [(index, separator)] = separators[..] else { return false };
    is_rfc3339_date(&value[..index]) && is_rfc3339_time(&value[index + separator.len_utf8()..])
}

const EMAIL_ATOM_EXTRA: &str = "!#$%&'*+/=?^_`{|}~-";

fn is_email_atom(atom: &str) -> bool {
    !atom.is_empty() && atom.chars().all(|entry| entry.is_ascii_alphanumeric() || EMAIL_ATOM_EXTRA.contains(entry))
}

fn is_domain_label(label: &str) -> bool {
    let boundaries_are_alphanumeric = label.chars().next().is_some_and(|entry| entry.is_ascii_alphanumeric()) && label.chars().next_back().is_some_and(|entry| entry.is_ascii_alphanumeric());
    boundaries_are_alphanumeric && label.chars().all(|entry| entry.is_ascii_alphanumeric() || entry == '-')
}

fn is_email(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else { return false };
    let labels: Vec<&str> = domain.split('.').collect();
    local.split('.').all(is_email_atom) && labels.len() >= 2 && labels.iter().all(|label| is_domain_label(label))
}

fn is_uuid(value: &str) -> bool {
    let value = match value.get(..9) {
        Some(prefix) if prefix.eq_ignore_ascii_case("urn:uuid:") => &value[9..],
        _ => value,
    };
    let groups: Vec<&str> = value.split('-').collect();
    groups.len() == 5 && [8usize, 4, 4, 4, 12].iter().zip(&groups).all(|(width, group)| group.len() == *width && group.chars().all(|entry| entry.is_ascii_hexdigit()))
}

fn is_pct_encoded_set(text: &str, extra: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '%' {
            if index + 2 >= chars.len() || !chars[index + 1].is_ascii_hexdigit() || !chars[index + 2].is_ascii_hexdigit() {
                return false;
            }
            index += 3;
            continue;
        }
        if !chars[index].is_ascii_alphanumeric() && !extra.contains(chars[index]) {
            return false;
        }
        index += 1;
    }
    true
}

fn is_uri_scheme(scheme: &str) -> bool {
    let mut chars = scheme.chars();
    chars.next().is_some_and(|first| first.is_ascii_alphabetic()) && chars.all(|entry| entry.is_ascii_alphanumeric() || matches!(entry, '+' | '-' | '.'))
}

fn is_uri_authority(authority: &str) -> bool {
    let (userinfo, host) = match authority.rsplit_once('@') {
        Some((userinfo, host)) => (Some(userinfo), host),
        None => (None, authority),
    };
    if userinfo.is_some_and(|userinfo| !is_pct_encoded_set(userinfo, "-._~!$&'()*+,;=:")) {
        return false;
    }
    let host = match host.rsplit_once(':') {
        Some((head, port)) if port.chars().all(|entry| entry.is_ascii_digit()) => head,
        _ => host,
    };
    match host.strip_prefix('[') {
        Some(literal) => literal.ends_with(']') && literal[..literal.len() - 1].chars().all(|entry| entry.is_ascii_hexdigit() || matches!(entry, ':' | '.')),
        None => is_pct_encoded_set(host, "-._~!$&'()*+,;="),
    }
}

/// 🔗 RFC 3986 absolute URI with an optional fragment — a scheme is mandatory, which is exactly the
/// distinction draft-07 draws between `uri` and `uri-reference`.
fn is_uri(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once(':') else { return false };
    if !is_uri_scheme(scheme) {
        return false;
    }
    let (rest, fragment) = match rest.split_once('#') {
        Some((rest, fragment)) => (rest, Some(fragment)),
        None => (rest, None),
    };
    let (rest, query) = match rest.split_once('?') {
        Some((rest, query)) => (rest, Some(query)),
        None => (rest, None),
    };
    if fragment.is_some_and(|fragment| !is_pct_encoded_set(fragment, "-._~!$&'()*+,;=:@/?")) || query.is_some_and(|query| !is_pct_encoded_set(query, "-._~!$&'()*+,;=:@/?")) {
        return false;
    }
    let path_is_valid = |path: &str| path.split('/').all(|segment| is_pct_encoded_set(segment, "-._~!$&'()*+,;=:@"));
    match rest.strip_prefix("//") {
        Some(rest) => {
            let (authority, path) = match rest.find('/') {
                Some(index) => rest.split_at(index),
                None => (rest, ""),
            };
            is_uri_authority(authority) && path_is_valid(path)
        }
        None => !rest.is_empty() && path_is_valid(rest),
    }
}

//#endregion 🏷️Format
