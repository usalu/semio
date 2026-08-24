//! ⚖️ The metamorphic laws this plugin's oracle adapters assert IN ROLE, before any oracle/subject
//! comparison happens.
//!
//! Two of the three scenario modes every `mutate-*` case declares state a property that is
//! checkable by the reference ALONE — no subject required:
//!
//! * `inverse-<kind>`: applying a mutation and then its own inverse must restore the original
//!   document's semantic projection.
//! * `identity-round-trip`: decoding and re-encoding must preserve that projection, and — unless
//!   the carrier is deliberately byte-exact — must not hand back the input bytes themselves.
//!
//! A handler that merely projects its result and returns asserts neither: it passes whenever the
//! reference library did not error. These helpers make the law an assertion with an error message
//! that names the first divergence, so a violated law fails the scenario that claims it.
//!
//! Dependency-free and format-neutral on purpose — this is the shape of the argument, not knowledge
//! of any format. The per-format tolerances a comparison profile declares (`ignoreKeys`,
//! `tolerance`, `arrays: "set"`) are passed in by the caller, so an in-handler check is exactly as
//! strict as the profile the case is measured by and never stricter.
//!
//! @see ../🔣️component.json — the comparison profiles whose tolerances these helpers mirror.

use semio_repo_test_host::Json;

//#region 🔖️Render
/// ✂️ A value rendered for an error message, truncated so a divergence inside an 8,448-point cloud
/// still produces a readable line.
fn render(value: &Json) -> String {
    let text = value.to_string();
    match text.char_indices().nth(120) {
        Some((cut, _)) => format!("{}… ({} chars)", &text[..cut], text.chars().count()),
        None => text,
    }
}
//#endregion 🔖️Render

//#region 🔖️Divergence
/// 🔎️ The first place `actual` differs from `expected`, as a JSON-path-qualified sentence, or
/// `None` when the two agree. `ignore_keys` drops object members either side declares (a profile's
/// own writer-freedom list) and `tolerance` is the absolute slack two numbers may differ by.
pub fn divergence_within(actual: &Json, expected: &Json, ignore_keys: &[&str], tolerance: f64) -> Option<String> {
    walk("$", actual, expected, ignore_keys, tolerance)
}

/// 🔎️ [`divergence_within`] with no writer freedom at all — exact equality.
pub fn divergence(actual: &Json, expected: &Json) -> Option<String> {
    divergence_within(actual, expected, &[], 0.0)
}

fn walk(path: &str, actual: &Json, expected: &Json, ignore_keys: &[&str], tolerance: f64) -> Option<String> {
    match (actual, expected) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in right {
                if ignore_keys.contains(&key.as_str()) {
                    continue;
                }
                match left.iter().find(|(name, _)| name == key) {
                    Some((_, found)) => {
                        if let Some(hit) = walk(&format!("{path}.{key}"), found, value, ignore_keys, tolerance) {
                            return Some(hit);
                        }
                    }
                    None => return Some(format!("{path}.{key} is absent, expected {}", render(value))),
                }
            }
            left.iter().find(|(key, _)| !ignore_keys.contains(&key.as_str()) && right.iter().all(|(name, _)| name != key)).map(|(key, value)| format!("{path}.{key} appeared out of nowhere, carrying {}", render(value)))
        }
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{path} holds {} item(s), expected {}", left.len(), right.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (one, other))| walk(&format!("{path}[{index}]"), one, other, ignore_keys, tolerance))
        }
        (Json::Number(left), Json::Number(right)) => {
            if (left - right).abs() <= tolerance || left == right {
                None
            } else {
                Some(format!("{path} is {left}, expected {right}"))
            }
        }
        (left, right) if left == right => None,
        (left, right) => Some(format!("{path} is {}, expected {}", render(left), render(right))),
    }
}
//#endregion 🔖️Divergence

//#region 🔖️Order
/// 🔢️ The projection with the named member arrays put in a canonical order, for a profile that
/// declares `arrays: "set"`. Member ORDER is writer freedom under such a profile, so an in-handler
/// check has to normalize it rather than demand from a positional comparison a strictness the
/// profile itself never applies.
pub fn unordered(projection: &Json, keys: &[&str]) -> Json {
    match projection {
        Json::Object(members) => Json::Object(
            members
                .iter()
                .map(|(key, value)| match (keys.contains(&key.as_str()), value) {
                    (true, Json::Array(items)) => {
                        let mut sorted: Vec<(String, Json)> = items.iter().map(|item| (item.to_string(), item.clone())).collect();
                        sorted.sort_by(|one, other| one.0.cmp(&other.0));
                        (key.clone(), Json::Array(sorted.into_iter().map(|(_, item)| item).collect()))
                    }
                    _ => (key.clone(), value.clone()),
                })
                .collect(),
        ),
        other => other.clone(),
    }
}
//#endregion 🔖️Order

//#region 🔖️Laws
/// ↩️ The inverse law: `apply(inverse(m))` after `apply(m)` must land back on the original's own
/// semantic projection. Asserted by the reference against its OWN pre-mutation reading, so an
/// `inverse-<kind>` scenario carries evidence on its own instead of only as an oracle/subject
/// agreement.
pub fn inverse_restores_within(kind: &str, restored: &Json, original: &Json, ignore_keys: &[&str], tolerance: f64) -> Result<(), String> {
    match divergence_within(restored, original, ignore_keys, tolerance) {
        None => Ok(()),
        Some(first) => Err(format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original — {first}")),
    }
}

/// ↩️ [`inverse_restores_within`] with no writer freedom.
pub fn inverse_restores(kind: &str, restored: &Json, original: &Json) -> Result<(), String> {
    inverse_restores_within(kind, restored, original, &[], 0.0)
}

/// 🔁️ The identity law's semantic half: decoding and re-encoding must not move the projection.
pub fn round_trip_preserves_within(reencoded: &Json, original: &Json, ignore_keys: &[&str], tolerance: f64) -> Result<(), String> {
    match divergence_within(reencoded, original, ignore_keys, tolerance) {
        None => Ok(()),
        Some(first) => Err(format!("identity law violated: decoding and re-encoding moved the semantic projection — {first}")),
    }
}

/// 🔁️ [`round_trip_preserves_within`] with no writer freedom.
pub fn round_trip_preserves(reencoded: &Json, original: &Json) -> Result<(), String> {
    round_trip_preserves_within(reencoded, original, &[], 0.0)
}

/// 🔒️ The identity law's no-byte-pass-through half: output that is bit-identical to the input is
/// indistinguishable from a `read`/`write` shortcut that never parsed anything. Applies only where
/// the encoder genuinely re-derives its bytes; a deliberately byte-exact carrier asserts
/// [`carrier_is_exact`] instead.
pub fn reparsed_not_copied(output: &[u8], input: &[u8]) -> Result<(), String> {
    if output == input {
        return Err(format!("byte pass-through: the re-encoded output is bit-identical to the {}-byte input, which a decode/encode that never parsed anything would also produce", input.len()));
    }
    Ok(())
}

/// 🔒️ The mirror law of [`reparsed_not_copied`], for the cases where reproducing the input exactly
/// is the CORRECT answer and anything else is the defect — a carrier whose decode/encode is an
/// identity by construction, a format that leaves a writer no freedom at all, or a fixture the
/// reference's OWN writer authored with the same options it will re-encode under. Each call site
/// states which of those it is; stating the law this way keeps the scenario's claim checkable
/// instead of merely excused, and it still fails loudly the moment the reader or writer drifts.
pub fn carrier_is_exact(output: &[u8], input: &[u8]) -> Result<(), String> {
    if output != input {
        let at = output.iter().zip(input.iter()).position(|(one, other)| one != other);
        return Err(format!(
            "exact-bytes law violated: the re-encoded output was required to reproduce the input, yet {} byte(s) out differ from {} byte(s) in{}",
            output.len(),
            input.len(),
            match at {
                Some(offset) => format!(" (first at byte {offset})"),
                None => String::new(),
            }
        ));
    }
    Ok(())
}
//#endregion 🔖️Laws

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn object(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    #[test]
    fn equal_projections_do_not_diverge() {
        let one = object(vec![("a", Json::Number(1.0)), ("b", Json::Array(vec![Json::String("x".to_string())]))]);
        assert_eq!(divergence(&one, &one.clone()), None);
    }

    #[test]
    fn a_changed_scalar_is_named_by_path() {
        let one = object(vec![("header", object(vec![("year", Json::Number(2026.0))]))]);
        let other = object(vec![("header", object(vec![("year", Json::Number(1999.0))]))]);
        assert_eq!(divergence(&one, &other), Some("$.header.year is 2026, expected 1999".to_string()));
    }

    #[test]
    fn a_missing_member_is_named_by_path() {
        let one = object(vec![("a", Json::Number(1.0))]);
        let other = object(vec![("a", Json::Number(1.0)), ("b", Json::Bool(true))]);
        assert_eq!(divergence(&one, &other), Some("$.b is absent, expected true".to_string()));
    }

    #[test]
    fn an_extra_member_is_named_by_path() {
        let one = object(vec![("a", Json::Number(1.0)), ("b", Json::Bool(true))]);
        let other = object(vec![("a", Json::Number(1.0))]);
        assert_eq!(divergence(&one, &other), Some("$.b appeared out of nowhere, carrying true".to_string()));
    }

    #[test]
    fn array_length_and_element_divergence_are_distinguished() {
        let one = Json::Array(vec![Json::Number(1.0), Json::Number(2.0)]);
        assert_eq!(divergence(&one, &Json::Array(vec![Json::Number(1.0)])), Some("$ holds 2 item(s), expected 1".to_string()));
        assert_eq!(divergence(&one, &Json::Array(vec![Json::Number(1.0), Json::Number(3.0)])), Some("$[1] is 2, expected 3".to_string()));
    }

    #[test]
    fn ignored_keys_and_tolerance_mirror_a_profile() {
        let one = object(vec![("fileSize", Json::Number(10.0)), ("x", Json::Number(1.000_001))]);
        let other = object(vec![("fileSize", Json::Number(99.0)), ("x", Json::Number(1.0))]);
        assert_eq!(divergence_within(&one, &other, &["fileSize"], 1e-5), None);
        assert!(divergence_within(&one, &other, &[], 1e-5).is_some());
        assert!(divergence_within(&one, &other, &["fileSize"], 0.0).is_some());
    }

    #[test]
    fn unordered_normalizes_only_the_named_arrays() {
        let value = object(vec![("entries", Json::Array(vec![Json::String("b".to_string()), Json::String("a".to_string())])), ("order", Json::Array(vec![Json::Number(2.0), Json::Number(1.0)]))]);
        let normalized = unordered(&value, &["entries"]);
        assert_eq!(normalized.array("entries"), vec![Json::String("a".to_string()), Json::String("b".to_string())]);
        assert_eq!(normalized.array("order"), vec![Json::Number(2.0), Json::Number(1.0)]);
    }

    #[test]
    fn the_inverse_law_names_the_kind_and_the_divergence() {
        let restored = object(vec![("count", Json::Number(4.0))]);
        let original = object(vec![("count", Json::Number(5.0))]);
        assert!(inverse_restores("remove-point", &restored, &original).unwrap_err().contains("remove-point"));
        assert!(inverse_restores("remove-point", &restored, &original).unwrap_err().contains("$.count is 4, expected 5"));
        assert!(inverse_restores("remove-point", &original, &original.clone()).is_ok());
    }

    #[test]
    fn the_two_byte_laws_are_mirrors_of_each_other() {
        assert!(reparsed_not_copied(b"abc", b"abc").is_err());
        assert!(reparsed_not_copied(b"abcd", b"abc").is_ok());
        assert!(carrier_is_exact(b"abc", b"abc").is_ok());
        assert!(carrier_is_exact(b"abd", b"abc").unwrap_err().contains("byte 2"));
    }
}
//#endregion 🧪️Tests
