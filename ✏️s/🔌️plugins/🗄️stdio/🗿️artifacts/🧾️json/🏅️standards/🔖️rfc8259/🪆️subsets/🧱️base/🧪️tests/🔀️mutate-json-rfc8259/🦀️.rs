//! 🦀️ JSON RFC 8259 exhaustive mutation round-trip case — Rust adapter.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. `oracle` handlers drive the registered `json` (json-rust)
//! reference implementation (via this subset's own `🦀️.rs`), `subject` handlers
//! drive this repository's own decode/mutate/encode round trip, and BOTH results are read back by
//! the SAME independent reader (`project_json_value`, json-rust underneath — never `serde_json`,
//! which is production-reachable in this repository and would compare an implementation with
//! something it already converts from) before the
//! `ordered-json-v1` profile compares them — an independent judge, never the subject's own model.
//! The subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation — see §5.3 of the fleet brief.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::json::standards::v_rfc8259::subsets::base::{oracle_apply_mutation, project_json_value, read_at, round_trip, PathSeg};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};


//#region 🔖️Input
const INPUT: &str = "shared://🔣️.json";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.json"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️SpecHelpers
fn json_object(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn kind_spec(kind: &str, params: Json) -> Json {
    json_object(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}

fn number(value: &Json, key: &str) -> Option<f64> {
    match value.get(key) {
        Some(Json::Number(number)) => Some(*number),
        _ => None,
    }
}

/// 🌳 A projected (plain) JSON value — what [`read_at`] hands back — as the tagged `JsonValue` wire a leaf payload
/// carries (`{"kind":"number","lexeme":…}` …). A whole number prints without a fraction (`99`, not `99.0`) and any
/// other one in its shortest round-trip form, both RFC 8259 §6 number lexemes.
fn wire_value(value: &Json) -> Json {
    let tagged = |kind: &str, pairs: Vec<(&str, Json)>| json_object([vec![("kind", Json::String(kind.to_string()))], pairs].concat());
    match value {
        Json::Null => tagged("null", vec![]),
        Json::Bool(flag) => tagged("bool", vec![("value", Json::Bool(*flag))]),
        Json::Number(n) if n.fract() == 0.0 && n.abs() < 1e15 => tagged("number", vec![("lexeme", Json::String(format!("{}", *n as i64)))]),
        Json::Number(n) => tagged("number", vec![("lexeme", Json::String(format!("{n:?}")))]),
        Json::String(text) => tagged("string", vec![("value", Json::String(text.clone()))]),
        Json::Array(items) => tagged("array", vec![("items", Json::Array(items.iter().map(wire_value).collect()))]),
        Json::Object(entries) => tagged("object", vec![("members", Json::Array(entries.iter().map(|(key, item)| json_object(vec![("key", Json::String(key.clone())), ("value", wire_value(item))])).collect()))]),
    }
}

/// ↩️ The inverse mutation's OWN spec (leaf wire payloads, like every row), computed by reading whatever pre-mutation state it needs
/// straight out of `original` with the SAME independent reader the oracle mutates with — never by
/// calling this repository's own `JsonMutation::inverse`, which would defeat the point of an
/// independently-computed oracle. Simpler than that method's own `RemoveMember` case: RFC 8259 §4
/// declares object member order insignificant, and this case's `ordered-json-v1` comparison profile
/// agrees (array order significant, key order never), so restoring a removed member's exact ORIGINAL
/// POSITION — which `JsonMutation::inverse`'s tail-reordering trick exists purely to do — is not
/// required here; adding the key back with its old value is a complete, independently-derived undo.
fn inverse_spec(original: &[u8], forward: &Json) -> Result<Json, String> {
    let params = forward.get("params").cloned().unwrap_or(Json::Null);
    let path = params.get("path").cloned().unwrap_or(Json::Array(Vec::new()));
    match forward.str("kind").as_str() {
        "set-member" => {
            let key = params.str("key");
            match read_at(original, &path, &[PathSeg::Key(key.clone())])? {
                Some(old) => Ok(kind_spec("set-member", json_object(vec![("path", path), ("key", Json::String(key)), ("value", wire_value(&old))]))),
                None => Ok(kind_spec("remove-member", json_object(vec![("path", path), ("key", Json::String(key))]))),
            }
        }
        "remove-member" => {
            let key = params.str("key");
            let old = read_at(original, &path, &[PathSeg::Key(key.clone())])?.ok_or_else(|| format!("remove-member inverse: the original document has no member {key:?}"))?;
            Ok(kind_spec("set-member", json_object(vec![("path", path), ("key", Json::String(key)), ("value", wire_value(&old))])))
        }
        "insert-array-element" => {
            let index = number(&params, "index").ok_or("insert-array-element inverse: missing `index`")? as usize;
            let len = match read_at(original, &path, &[])? {
                Some(Json::Array(items)) => items.len(),
                _ => return Err("insert-array-element inverse: path does not resolve to an array".to_string()),
            };
            let clamped = index.min(len);
            Ok(kind_spec("remove-array-element", json_object(vec![("path", path), ("index", Json::Number(clamped as f64))])))
        }
        "remove-array-element" => {
            let index = number(&params, "index").ok_or("remove-array-element inverse: missing `index`")? as usize;
            let old = read_at(original, &path, &[PathSeg::Index(index)])?.ok_or_else(|| format!("remove-array-element inverse: index {index} out of bounds"))?;
            Ok(kind_spec("insert-array-element", json_object(vec![("path", path), ("index", Json::Number(index as f64)), ("value", wire_value(&old))])))
        }
        "set-scalar" => {
            let old = read_at(original, &path, &[])?.ok_or("set-scalar inverse: path does not resolve")?;
            Ok(kind_spec("set-scalar", json_object(vec![("path", path), ("value", wire_value(&old))])))
        }
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}
//#endregion 🔖️SpecHelpers

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let output = oracle_apply_mutation(&input, &spec)?;
    let projection = project_json_value(&output)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_json_value(&input)?, &[])?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The inverse law, asserted HERE by the reference against its own pre-mutation reading rather
/// than deferred to the parity phase: `apply(m)` followed by `apply(inverse(m))` has to land back on
/// the ORIGINAL document's semantic projection. Member ORDER is not part of the claim — RFC 8259 §4
/// leaves it to the producer and `ordered-json-v1` agrees — and the divergence walk compares object
/// members by NAME for exactly that reason, so restoring a removed member at the tail is not a
/// failure while losing or changing its value is.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let undo = inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_json_value(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &project_json_value(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `json` (json-rust)
/// reader/writer this subset's mutations use — proves the reference library itself is stable on the
/// real fixture before the subject's own codec is asked to be. Both halves of the identity law are
/// asserted in role: the value tree must survive unchanged, and the output must not be the input
/// bytes back again — the committed fixture is indented and `JsonValue::dump` emits the compact
/// form, so a bit-identical result could only come from a copy that never parsed anything.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = round_trip(&input)?;
    reparsed_not_copied(&output, &input)?;
    let projection = project_json_value(&output)?;
    round_trip_preserves(&projection, &project_json_value(&input)?)?;
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, mutable_input};
    use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, round_trip_preserves};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::mutations::{apply_json_mutation, decode_json_mutation_payload_json, JsonMutation};
    use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::{parse_json_text, write_json_text, JsonSnapshot};
    use semio_s_artifact_stdio_json::STDIO_JSON_DOCUMENT_SCHEMA;
    use semio_s_plugin_stdio_test_oracle::artifacts::json::standards::v_rfc8259::subsets::base::project_json_value;

    /// 🔀️ The scenario's (or its inverse's) `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the
    /// aggregate's own derive-generated payload constructor — the only channel between the feature's parameters and the
    /// subject's codec, never re-declared field by field here.
    fn mutation_from_spec(spec: &Json) -> Result<JsonMutation, String> {
        decode_json_mutation_payload_json(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null).to_string())
    }

    fn decode(bytes: &[u8]) -> Result<JsonSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let value = parse_json_text(&text).map_err(|error| error.to_string())?;
        Ok(JsonSnapshot { schema: STDIO_JSON_DOCUMENT_SCHEMA.into(), value })
    }

    fn encode(snapshot: &JsonSnapshot) -> Vec<u8> {
        write_json_text(&snapshot.value).into_bytes()
    }

    /// 👁️ The forward mutation, with the OBSERVABILITY law asserted IN ROLE through the SAME shared
    /// `⚖️law` helper `super::mutate_oracle` calls. `apply_json_mutation` returns a rejecting
    /// `MutationOutcome` and leaves the snapshot untouched when a path addresses nothing, so without
    /// this a refused mutation reports a green scenario carrying the unmutated document.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode(&input)?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_from_spec(&spec)?;
        apply_json_mutation(&mut snapshot, &mutation);
        let output = encode(&snapshot);
        let projection = project_json_value(&output)?;
        mutation_is_observable(&spec.str("kind"), &projection, &project_json_value(&input)?, &[])?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// ↩️ The inverse law, asserted on the SUBJECT side too rather than deferred to the parity
    /// phase, through the same shared `⚖️law` helper `super::inverse_oracle` calls.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        apply_json_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        apply_json_mutation(&mut snapshot, &mutation_from_spec(&inverse_spec(&input, &spec)?)?);
        let output = encode(&snapshot);
        let projection = project_json_value(&output)?;
        inverse_restores(&spec.str("kind"), &projection, &project_json_value(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating (fleet brief, "the point of this wave") and this tripwire catches it:
    /// the real fixture is committed 2-space pretty-printed (spaces after `:`/`,`, newlines) while
    /// this repository's own writer emits fully compact JSON (`write_json_text`, no whitespace at
    /// all), so a genuine re-encode can never coincidentally reproduce the input bytes.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode(&snapshot);
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_json_value(&output)?;
        round_trip_preserves(&projection, &project_json_value(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
