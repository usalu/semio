//! 🔮️ Mutation oracle for this subset — recorded no-oracle decision `raw-buffer-no-format`
//! (`../🔣️oracle.json`).
//!
//! A raw byte buffer has no format: there is nothing a third-party library could be authoritative
//! about, and no independent reader exists either (there is no grammar to parse). So this module is
//! not a reference-library adapter — it is the specification made executable, an independently
//! written splice/append/truncate implementation that never touches this subset's own
//! `BinaryDiff`/`ByteSplice`/`apply_binary_mutation` (`../🧬️schema/🧬️mutations/🦀️.rs`,
//! `../🧬️schema/🔺️diff/🦀️.rs`) — comparing this repository's implementation against
//! itself is the exact failure mode the whole test platform exists to prevent. Bounds validation
//! mirrors the vocabulary's own documented contract (an out-of-range `offset`/`remove_len` is
//! rejected, never silently clamped or corrupted; `TruncateAt` past the current length is the
//! vocabulary's own defined no-op, not an error) but is reimplemented here from scratch.
//!
//! The vocabulary is per SUBSET, not per artifact. This one has exactly 5 kinds: `no-mutation`,
//! `set-snapshot`, `splice`, `append-bytes`, `truncate-at`.
//!
//! @see ../🔣️oracle.json — the mutation catalog and the recorded no-oracle decision.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`BinaryMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️SpecReading
/// 🔎️ A byte payload as the wire protocol carries it: a plain JSON array of 0-255 numbers (the
/// protocol's `Json` has no base64 accessor and no other case in this repository uses one — see
/// `../../../../🎥️mp4/🧪️tests/🐙️mutate-mp4-isobmff/🦀️.rs`'s own local `bytes` helper).
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
/// `ByteSplice`/`validate_binary_diff` in `../🧬️schema/🔺️diff/🦀️.rs`: `offset` must not
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
