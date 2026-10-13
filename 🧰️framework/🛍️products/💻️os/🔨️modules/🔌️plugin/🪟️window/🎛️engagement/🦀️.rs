//! 🎛️ Parses engagement command-line drafts. The shell submits the typed line VERBATIM (trimmed),
//! so an app owns its own tokenization and `"fill 20"` arrives as `"fill 20"`; these helpers stay
//! separator- and case-insensitive on the verb token, so a name-token grammar that normalizes its
//! own line (the cad repl PascalCases in its `onChange`) still matches `"Fill20"`.
/** ✂️ Strips a leading `command` token from `raw`, ignoring case and separators on both
sides, and returns the trimmed remainder (e.g. `strip_engagement_prefix("Fill20", "fill")`
and `strip_engagement_prefix("fill 20", "fill")` both yield `Some("20")`). Decimal points
inside numeric remainders are preserved. Returns `None` when `raw` doesn't start with `command`. */
pub fn strip_engagement_prefix<'a>(raw: &'a str, command: &str) -> Option<&'a str> {
    let raw_bytes = raw.as_bytes();
    let mut raw_index = 0usize;
    let command_chars = command.chars().filter(|ch| ch.is_alphanumeric());
    for expected in command_chars {
        while raw_index < raw_bytes.len() {
            let ch = raw[raw_index..].chars().next().unwrap();
            if ch.is_alphanumeric() {
                break;
            }
            raw_index += ch.len_utf8();
        }
        let actual = raw[raw_index..].chars().next()?;
        if !actual.eq_ignore_ascii_case(&expected) {
            return None;
        }
        raw_index += actual.len_utf8();
    }
    let mut remainder_start = raw_index;
    while remainder_start < raw_bytes.len() {
        let ch = raw[remainder_start..].chars().next().unwrap();
        if ch.is_alphanumeric() || ch == '.' {
            break;
        }
        remainder_start += ch.len_utf8();
    }
    Some(raw[remainder_start..].trim())
}
/** 🔤️ True when `raw` matches `command` in full, ignoring case and separators (e.g.
`engagement_token_matches("LineNumbers", "line numbers")` is `true`). */
pub fn engagement_token_matches(raw: &str, command: &str) -> bool {
    strip_engagement_prefix(raw, command).is_some_and(str::is_empty)
}
#[cfg(test)]
include!("../../🧪️tests/🔬️engagement-unit/🦀️.rs");