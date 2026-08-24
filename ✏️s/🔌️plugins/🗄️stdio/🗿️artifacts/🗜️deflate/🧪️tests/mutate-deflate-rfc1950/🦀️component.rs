//! 🦀️ RFC1950 mutation case — Rust adapter.
//!
//! Every scenario copies one of the two real, committed zlib fixtures into the case work directory
//! first; the committed fixtures are never written to. `oracle` drives the registered `flate2`
//! reference implementation through this subset's own oracle module; `subject` drives this
//! repository's own `DeflateMutation`/`apply_deflate_mutation`/`decode_deflate_snapshot`/
//! `encode_deflate_snapshot` — the real, typed, event-sourced mutation pipeline, not an ad hoc
//! byte edit. Both results are read back by the INDEPENDENT `flate2` projection before the
//! `ordered-json-v1` profile compares them. The subject half is gated behind the generated host's
//! `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::deflate::standards::v_rfc1950::subsets::any::{independent_payload, inverse_mutation_spec, oracle_apply_mutation, project_deflate};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Kinds
/// 🗂️ Mirrors `DeflateMutation`'s kebab-case `KINDS` (schema/mutations/component.rs). Duplicated
/// here, rather than imported, because the oracle-only host build never links the SUT crate at all
/// (it is an optional dependency gated behind the `sut` feature this crate's own registration loop
/// runs unconditionally), so this list has to be reachable without it.
const KINDS: [&str; 5] = ["no-mutation", "set-snapshot", "set-compression-params", "set-preset-dictionary", "set-payload"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const MUTATE_INPUT: &str = "shared://📄️readme-level9.zz";
const IDENTITY_INPUT: &str = "shared://📄️readme-level1.zz";

/// 🧫️ Copies the immutable fixture into the work directory and returns its bytes.
fn mutable_input(ctx: &Context, uri: &str, name: &str) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(uri, Some(name))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_deflate(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies the row's forward mutation, then this subset's own algebraic inverse of it (computed
/// from the ORIGINAL header/payload, the same restore-the-prior-value law
/// `DeflateMutation::inverse` implements), and asserts the restoration against the ORIGINAL
/// document's own projection before ever reaching the framework's oracle-vs-subject comparison.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
    let original_projection = project_deflate(&input)?;
    let (original_header, original_payload) = original_fields(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let inverse_spec = inverse_mutation_spec(&spec.str("kind"), original_header.0, original_header.1, original_header.2, original_header.3, &original_payload)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec)?;
    let projection = project_deflate(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &original_projection)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔎️ The typed `(method, windowBits, levelHintBits, dictId)` fields plus the decoded payload,
/// read from `project_deflate`'s own JSON shape and `independent_payload` so this file never
/// parses RFC1950 header bytes itself.
fn original_fields(input: &[u8]) -> Result<((u8, u8, u8, Option<u32>), Vec<u8>), String> {
    let projection = project_deflate(input)?;
    let method = match projection.get("compressionMethod") {
        Some(Json::Number(value)) => *value as u8,
        _ => return Err("projection carries no compressionMethod".to_string()),
    };
    let window_bits = match projection.get("windowBits") {
        Some(Json::Number(value)) => *value as u8,
        _ => return Err("projection carries no windowBits".to_string()),
    };
    let level_hint_bits = match projection.str("compressionLevelHint").as_str() {
        "fastest" => 0u8,
        "fast" => 1u8,
        "default" => 2u8,
        "maximum" => 3u8,
        other => return Err(format!("unknown compressionLevelHint {other:?}")),
    };
    let dict_id = match projection.get("presetDictionaryId") {
        Some(Json::Number(value)) => Some(*value as u32),
        _ => None,
    };
    let payload = independent_payload(input)?;
    Ok(((method, window_bits, level_hint_bits, dict_id), payload))
}

/// 🔁️ The identity law, both halves asserted in role. The semantic half: inflating and
/// re-deflating must leave the typed header fields and the payload digest exactly where they were.
/// The no-byte-pass-through half: this scenario deliberately reads the LEVEL-1 fixture while the
/// reference re-compresses at `flate2`'s own default level, so an output equal to the input would
/// mean the DEFLATE stream was copied rather than genuinely inflated and re-coded.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx, IDENTITY_INPUT, "input.zz")?;
    let spec = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(Vec::new()))]);
    let bytes = oracle_apply_mutation(&input, &spec)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_deflate(&bytes)?;
    round_trip_preserves(&projection, &project_deflate(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, IDENTITY_INPUT, MUTATE_INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::deflate::standards::v_rfc1950::subsets::any::io::{decode_deflate_snapshot, encode_deflate_snapshot};
    use semio_s_plugin_stdio::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::mutations::apply_deflate_mutation;
    use semio_s_plugin_stdio::artifacts::deflate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateLevelHint;
    use semio_s_plugin_stdio::artifacts::deflate::{DeflateMutation, DeflateSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::deflate::standards::v_rfc1950::subsets::any::project_deflate;

    /// 🎚️ Independent of `DeflateLevelHint::from_bits`/`to_bits` — mirrors the oracle module's own
    /// `levelHint` string spelling, since both read/write the same JSON param shape.
    fn level_hint_of(name: &str) -> Result<DeflateLevelHint, String> {
        match name {
            "fastest" => Ok(DeflateLevelHint::Fastest),
            "fast" => Ok(DeflateLevelHint::Fast),
            "default" => Ok(DeflateLevelHint::Default),
            "maximum" => Ok(DeflateLevelHint::Maximum),
            other => Err(format!("unknown levelHint {other:?}")),
        }
    }

    fn level_hint_name(hint: DeflateLevelHint) -> &'static str {
        match hint {
            DeflateLevelHint::Fastest => "fastest",
            DeflateLevelHint::Fast => "fast",
            DeflateLevelHint::Default => "default",
            DeflateLevelHint::Maximum => "maximum",
        }
    }

    fn json_number(value: &Json, key: &str, default: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => default,
        }
    }

    fn json_optional_u32(value: &Json, key: &str) -> Option<u32> {
        match value.get(key) {
            Some(Json::Number(found)) => Some(*found as u32),
            _ => None,
        }
    }

    /// 🦠️ Builds the real `DeflateMutation` the spec describes, defaulting untouched fields from
    /// `base` — the same shape `oracle_apply_mutation` reads, so both producers see one spec.
    fn spec_to_mutation(spec: &Json, base: &DeflateSnapshot) -> Result<DeflateMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(DeflateMutation::NoMutation),
            "set-snapshot" => Ok(DeflateMutation::SetSnapshot {
                snapshot: DeflateSnapshot {
                    schema: base.schema.clone(),
                    compression_method: json_number(&params, "method", base.compression_method as f64) as u8,
                    window_bits: json_number(&params, "windowBits", base.window_bits as f64) as u8,
                    compression_level_hint: level_hint_of(&params.str("levelHint"))?,
                    dict_id: json_optional_u32(&params, "dictId"),
                    payload: params.str("payload").into_bytes(),
                },
            }),
            "set-compression-params" => Ok(DeflateMutation::SetCompressionParams { method: json_number(&params, "method", base.compression_method as f64) as u8, window_bits: json_number(&params, "windowBits", base.window_bits as f64) as u8, level_hint: level_hint_of(&params.str("levelHint"))? }),
            "set-preset-dictionary" => Ok(DeflateMutation::SetPresetDictionary { dict_id: json_optional_u32(&params, "dictId") }),
            "set-payload" => Ok(DeflateMutation::SetPayload { payload: params.str("payload").into_bytes() }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }

    /// ↩️ The inverse spec for `kind`, restoring `base`'s own fields — the same algebra
    /// `DeflateMutation::inverse` implements, computed here from data rather than through that
    /// trait (unreachable from this generated host crate without a forbidden Cargo.toml edit).
    fn inverse_spec(kind: &str, base: &DeflateSnapshot) -> Result<Json, String> {
        let dict_id_json = base.dict_id.map(|id| Json::Number(id as f64)).unwrap_or(Json::Null);
        let payload_text = String::from_utf8(base.payload.clone()).map_err(|error| format!("original payload is not UTF-8 text: {error}"))?;
        let params = match kind {
            "no-mutation" => Json::Object(Vec::new()),
            "set-snapshot" => Json::Object(vec![
                ("method".to_string(), Json::Number(base.compression_method as f64)),
                ("windowBits".to_string(), Json::Number(base.window_bits as f64)),
                ("levelHint".to_string(), Json::String(level_hint_name(base.compression_level_hint).to_string())),
                ("dictId".to_string(), dict_id_json),
                ("payload".to_string(), Json::String(payload_text)),
            ]),
            "set-compression-params" => Json::Object(vec![("method".to_string(), Json::Number(base.compression_method as f64)), ("windowBits".to_string(), Json::Number(base.window_bits as f64)), ("levelHint".to_string(), Json::String(level_hint_name(base.compression_level_hint).to_string()))]),
            "set-preset-dictionary" => Json::Object(vec![("dictId".to_string(), dict_id_json)]),
            "set-payload" => Json::Object(vec![("payload".to_string(), Json::String(payload_text))]),
            other => return Err(format!("mutation kind {other:?} has no inverse spec")),
        };
        Ok(Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)]))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
        let mut snapshot = decode_deflate_snapshot(&input).map_err(|error| format!("decode_deflate_snapshot failed: {error}"))?;
        let mutation = spec_to_mutation(&spec, &snapshot)?;
        apply_deflate_mutation(&mut snapshot, &mutation);
        let bytes = encode_deflate_snapshot(&snapshot);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_deflate(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, MUTATE_INPUT, "input.zz")?;
        let original = decode_deflate_snapshot(&input).map_err(|error| format!("decode_deflate_snapshot failed: {error}"))?;
        let original_projection = project_deflate(&input)?;
        let mut mutated = original.clone();
        let mutation = spec_to_mutation(&spec, &original)?;
        apply_deflate_mutation(&mut mutated, &mutation);
        let mutated_bytes = encode_deflate_snapshot(&mutated);
        if mutated_bytes == input {
            return Err("byte pass-through: mutated output is bit-identical to the input".to_string());
        }
        let kind = spec.str("kind");
        let inverse = spec_to_mutation(&inverse_spec(&kind, &original)?, &mutated)?;
        let mut restored = mutated.clone();
        apply_deflate_mutation(&mut restored, &inverse);
        let restored_bytes = encode_deflate_snapshot(&restored);
        let projection = project_deflate(&restored_bytes)?;
        if projection != original_projection {
            return Err(format!("inverse of {kind} did not restore the subject's original semantic projection"));
        }
        Ok(Outcome::with_raw(restored_bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx, IDENTITY_INPUT, "input.zz")?;
        let snapshot = decode_deflate_snapshot(&input).map_err(|error| format!("decode_deflate_snapshot failed: {error}"))?;
        let bytes = encode_deflate_snapshot(&snapshot);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_deflate(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
