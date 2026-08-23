//! 🔮️ Mutation oracle for this subset — recorded no-oracle decision `raw-buffer-no-format`
//! (`../🧪️oracle/🔣️component.json`).
//!
//! A raw byte buffer has no format: there is nothing a third-party library could be authoritative
//! about, and no independent reader exists either (there is no grammar to parse). So this module is
//! not a reference-library adapter — it is the specification made executable, an independently
//! written splice/append/truncate implementation that never touches this subset's own
//! `BinaryDiff`/`ByteSplice`/`apply_binary_mutation` (`../🧬️schema/🧬️mutations/🦀️component.rs`,
//! `../🧬️schema/🔺️diff/🦀️component.rs`) — comparing this repository's implementation against
//! itself is the exact failure mode the whole test platform exists to prevent. Bounds validation
//! mirrors the vocabulary's own documented contract (an out-of-range `offset`/`remove_len` is
//! rejected, never silently clamped or corrupted; `TruncateAt` past the current length is the
//! vocabulary's own defined no-op, not an error) but is reimplemented here from scratch.
//!
//! The vocabulary is per SUBSET, not per artifact. This one has exactly 5 kinds: `no-mutation`,
//! `set-snapshot`, `splice`, `append-bytes`, `truncate-at`.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog and the recorded no-oracle decision.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`BinaryMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️SpecReading
/// 🔎️ A byte payload as the wire protocol carries it: a plain JSON array of 0-255 numbers (the
/// protocol's `Json` has no base64 accessor and no other case in this repository uses one — see
/// `../../../../🎥️mp4/🧪️tests/mutate-mp4-isobmff/🦀️component.rs`'s own local `bytes` helper).
#[cfg(feature = "oracles")]
fn bytes_field(value: &Json, key: &str) -> Vec<u8> {
    match value.get(key) {
        Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(number) = item { Some(*number as u8) } else { None }).collect(),
        _ => Vec::new(),
    }
}

#[cfg(feature = "oracles")]
fn usize_field(value: &Json, key: &str) -> Result<usize, String> {
    match value.get(key) {
        Some(Json::Number(number)) => Ok(*number as usize),
        _ => Err(format!("expected a numeric field {key:?}")),
    }
}
//#endregion 🔖️SpecReading

//#region 🔖️Splice
/// ✂️ The specification's own contract for one splice, reimplemented independently of
/// `ByteSplice`/`validate_binary_diff` in `../🧬️schema/🔺️diff/🦀️component.rs`: `offset` must not
/// exceed the buffer's current length, and `remove_len` must not reach past it. Both are rejected
/// with `Err`, never clamped — a clamped offset would silently mutate the wrong range instead of
/// failing, which is exactly the "corrupts silently" failure the spec vectors exist to catch.
#[cfg(feature = "oracles")]
fn splice(buffer: &mut Vec<u8>, offset: usize, remove_len: usize, insert: &[u8]) -> Result<(), String> {
    if offset > buffer.len() {
        return Err(format!("splice offset {offset} is outside the buffer (length {})", buffer.len()));
    }
    if remove_len > buffer.len() - offset {
        return Err(format!("splice remove_len {remove_len} at offset {offset} exceeds the buffer (length {})", buffer.len()));
    }
    buffer.splice(offset..offset + remove_len, insert.iter().copied());
    Ok(())
}
//#endregion 🔖️Splice

//#region 🔖️Apply
/// 🦠️ Every declared kind, dispatched by its kebab-case name. `set-snapshot` reads the same
/// `{"snapshot":{"bytes":[...]}}` shape the subject's own `BinarySnapshot` carries; every other kind
/// reads the params `BinaryMutation`'s own variant fields name (camelCase, matching the enum's
/// `#[serde(rename_all = "camelCase")]`).
#[cfg(feature = "oracles")]
fn apply(buffer: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
    let mut out = buffer.to_vec();
    match kind {
        "no-mutation" => Ok(out),
        "set-snapshot" => {
            let snapshot = params.get("snapshot").ok_or("set-snapshot requires a `snapshot` field")?;
            Ok(bytes_field(snapshot, "bytes"))
        }
        "splice" => {
            let offset = usize_field(params, "offset")?;
            let remove_len = usize_field(params, "removeLen")?;
            let insert = bytes_field(params, "insert");
            splice(&mut out, offset, remove_len, &insert)?;
            Ok(out)
        }
        "append-bytes" => {
            let data = bytes_field(params, "data");
            let len = out.len();
            splice(&mut out, len, 0, &data)?;
            Ok(out)
        }
        "truncate-at" => {
            let offset = usize_field(params, "offset")?;
            if offset < out.len() {
                out.truncate(offset);
            }
            // 🌱 `offset >= len` is the vocabulary's own defined no-op (see `BinaryMutation::diff`'s
            // `TruncateAt` arm), not an error — the buffer is returned unchanged.
            Ok(out)
        }
        other => Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
}
//#endregion 🔖️Apply

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let empty_params = Json::Object(Vec::new());
    let params = spec.get("params").unwrap_or(&empty_params);
    apply(input, &kind, params)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    fn spec(kind: &str, params: Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
    }

    fn obj(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn num_array(values: &[u8]) -> Json {
        Json::Array(values.iter().map(|value| Json::Number(*value as f64)).collect())
    }

    #[test]
    fn no_mutation_is_identity() {
        let input = vec![1, 2, 3, 4, 5];
        let out = oracle_apply_mutation(&input, &spec("no-mutation", obj(vec![]))).unwrap();
        assert_eq!(out, input);
    }

    #[test]
    fn set_snapshot_replaces_the_whole_buffer() {
        let input = vec![1, 2, 3];
        let params = obj(vec![("snapshot", obj(vec![("bytes", num_array(&[9, 9]))]))]);
        let out = oracle_apply_mutation(&input, &spec("set-snapshot", params)).unwrap();
        assert_eq!(out, vec![9, 9]);
    }

    #[test]
    fn splice_replaces_the_named_range() {
        let input = vec![1, 2, 3, 4, 5];
        let params = obj(vec![("offset", Json::Number(1.0)), ("removeLen", Json::Number(2.0)), ("insert", num_array(&[0xAA, 0xBB, 0xCC]))]);
        let out = oracle_apply_mutation(&input, &spec("splice", params)).unwrap();
        assert_eq!(out, vec![1, 0xAA, 0xBB, 0xCC, 4, 5]);
    }

    #[test]
    fn splice_out_of_range_offset_is_rejected_without_corrupting() {
        let input = vec![1, 2, 3];
        let params = obj(vec![("offset", Json::Number(4.0)), ("removeLen", Json::Number(0.0)), ("insert", num_array(&[]))]);
        assert!(oracle_apply_mutation(&input, &spec("splice", params)).is_err());
    }

    #[test]
    fn splice_remove_len_past_the_end_is_rejected() {
        let input = vec![1, 2, 3];
        let params = obj(vec![("offset", Json::Number(2.0)), ("removeLen", Json::Number(5.0)), ("insert", num_array(&[]))]);
        assert!(oracle_apply_mutation(&input, &spec("splice", params)).is_err());
    }

    #[test]
    fn append_bytes_extends_the_end() {
        let input = vec![1, 2];
        let params = obj(vec![("data", num_array(&[3, 4]))]);
        let out = oracle_apply_mutation(&input, &spec("append-bytes", params)).unwrap();
        assert_eq!(out, vec![1, 2, 3, 4]);
    }

    #[test]
    fn append_bytes_to_an_empty_buffer() {
        let input: Vec<u8> = vec![];
        let params = obj(vec![("data", num_array(&[7, 8]))]);
        let out = oracle_apply_mutation(&input, &spec("append-bytes", params)).unwrap();
        assert_eq!(out, vec![7, 8]);
    }

    #[test]
    fn truncate_at_drops_the_tail() {
        let input = vec![1, 2, 3, 4, 5];
        let out = oracle_apply_mutation(&input, &spec("truncate-at", obj(vec![("offset", Json::Number(2.0))]))).unwrap();
        assert_eq!(out, vec![1, 2]);
    }

    #[test]
    fn truncate_at_beyond_the_length_is_a_defined_no_op() {
        let input = vec![1, 2, 3];
        let out = oracle_apply_mutation(&input, &spec("truncate-at", obj(vec![("offset", Json::Number(999.0))]))).unwrap();
        assert_eq!(out, input);
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let input = vec![1, 2, 3];
        assert!(oracle_apply_mutation(&input, &spec("not-a-real-kind", obj(vec![]))).is_err());
    }
}
//#endregion 🧪️Tests
