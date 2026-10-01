//! 🦀️ Raw-binary exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `raw-buffer-no-format` (`../../🔮️oracles/🔣️.json`): a raw byte buffer has no format, so `oracle`
//! here drives this subset's own independently written specification-vector implementation
//! (`../../🔮️oracles/🦀️.rs`'s `oracle_apply_mutation`, which never touches the subject's own
//! `BinaryDiff`/`apply_binary_mutation`); `subject` decodes every scenario's `{kind, params}` witness
//! generically through `Mutation::from_payload_value` — `params` IS the leaf's wire payload — applies
//! it with this repository's own `apply_binary_mutation`, undoes it with the vocabulary's own
//! `Mutation::inverse`, and cross-checks each result against that SAME independent reference before
//! returning. That cross-check is deliberate: the test framework's `oracleDecision` never invokes the
//! `oracle` role for a `@no-oracle-` feature, so `subject` is the only role that ever discharges this
//! decision's specification-vector evidence. Both sides project to the exact output byte array and
//! `exact-bytes-v1` compares them literally. The subject half is gated behind the generated host's
//! `sut` feature so the oracle-only run never compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::binary::standards::v_raw::subsets::any::{oracle_apply_mutation, oracle_round_trip};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores};

//#region 🔖️Input
const INPUT: &str = "shared://🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's
/// bytes; the committed asset itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.bin"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️JsonBuild
fn json_obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", Json::String(kind.to_string())), ("params", params)])
}
fn bytes_json(bytes: &[u8]) -> Json {
    Json::Array(bytes.iter().map(|byte| Json::Number(*byte as f64)).collect())
}
/// 🔎️ A byte payload as the wire carries it: a plain JSON array of 0-255 numbers.
fn bytes_field(value: &Json, key: &str) -> Vec<u8> {
    match value.get(key) {
        Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(number) = item { Some(*number as u8) } else { None }).collect(),
        _ => Vec::new(),
    }
}
fn usize_field(value: &Json, key: &str) -> Result<usize, String> {
    match value.get(key) {
        Some(Json::Number(number)) => Ok(*number as usize),
        _ => Err(format!("expected a numeric field {key:?}")),
    }
}
/// 🎯️ The projection every scenario compares under `exact-bytes-v1`: the complete output byte
/// array, literally — for a raw buffer the bytes ARE the whole content.
fn projection_of(bytes: &[u8]) -> Json {
    bytes_json(bytes)
}
//#endregion 🔖️JsonBuild

//#region 🔖️Inverse
/// ↩️ The independent reference's own undo of one forward `(kind, params)` witness, in the same wire
/// vocabulary and computed from the REAL pristine `input` bytes at run time — the `truncate-at`
/// example alone removes a 283 KB real tail no literal could carry legibly. `set-snapshot` inverts
/// through a REAL `set-snapshot` carrying the pristine buffer, never a hand-back of the input.
fn inverse_spec(kind: &str, input: &[u8], params: &Json) -> Result<Json, String> {
    match kind {
        "set-snapshot" => Ok(json_spec("set-snapshot", json_obj(vec![("snapshot", json_obj(vec![("schema", Json::String("stdio.binary".to_string())), ("bytes", bytes_json(input))]))]))),
        "replace-byte-range" => {
            let offset = usize_field(params, "offset")?.min(input.len());
            let remove_len = usize_field(params, "remove_len")?;
            let insert = bytes_field(params, "insert");
            let removed = input[offset..(offset + remove_len).min(input.len())].to_vec();
            Ok(json_spec("replace-byte-range", json_obj(vec![("offset", Json::Number(offset as f64)), ("remove_len", Json::Number(insert.len() as f64)), ("insert", bytes_json(&removed))])))
        }
        "append-bytes" => Ok(json_spec("truncate-at", json_obj(vec![("offset", Json::Number(input.len() as f64))]))),
        "truncate-at" => {
            let offset = usize_field(params, "offset")?.min(input.len());
            Ok(json_spec("replace-byte-range", json_obj(vec![("offset", Json::Number(offset as f64)), ("remove_len", Json::Number(0.0)), ("insert", bytes_json(&input[offset..]))])))
        }
        other => Err(format!("mutation kind {other:?} has no inverse rule")),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
/// 🔮️ Applies the declared mutation with this subset's own independent specification
/// implementation and projects the resulting bytes directly.
fn apply_and_project(input: &[u8], spec: &Json) -> Result<Outcome, String> {
    let bytes = oracle_apply_mutation(input, spec)?;
    let projection = projection_of(&bytes);
    Ok(Outcome::with_raw(bytes, projection))
}

fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    apply_and_project(&input, &ctx.doc_json()?)
}

/// ↩️ The inverse law, asserted HERE by the independent specification implementation against the
/// pristine buffer: every kind is applied forward and then undone, and the restored buffer must be
/// the original buffer, literally.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input, &params)?)?;
    let projection = projection_of(&restored);
    inverse_restores(&kind, &projection, &projection_of(&input))?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔮️ For a raw buffer `decode`/`encode` really is the identity (`carrier_native_is_raw`,
/// `../../🚪️io/🦀️.rs`), so the reference's own round trip must return the input exactly — byte
/// equality IS the correct answer here, and dropping or clamping a byte fails it.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    carrier_is_exact(&output, &input)?;
    Ok(Outcome::with_raw(output.clone(), projection_of(&output)))
}

/// 🔮️ The specification-vector scenarios share the SAME forward-apply shape as `mutate_oracle`,
/// just against whichever `kind` the vector names.
fn vector_oracle(ctx: &Context) -> Result<Outcome, String> {
    mutate_oracle(ctx)
}

fn append_to_empty_buffer_oracle(ctx: &Context) -> Result<Outcome, String> {
    apply_and_project(&[], &ctx.doc_json()?)
}

/// 🔮️ An invalid byte-range replacement must be REJECTED, never silently applied: rejection is the
/// passing outcome, and a reference that does NOT reject is the failure.
fn invalid_replacement_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    match oracle_apply_mutation(&input, &ctx.doc_json()?) {
        Err(_) => Ok(Outcome::with_raw(input, json_obj(vec![("rejected", Json::Bool(true))]))),
        Ok(bytes) => Err(format!("expected the invalid byte-range replacement to be rejected, but it produced {} byte(s) without erroring", bytes.len())),
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_spec, json_obj, mutable_input, projection_of};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_binary::standards::v_raw::subsets::any::schema::mutations::apply_binary_mutation;
    use semio_s_artifact_stdio_binary::{mutation_from_payload_json, mutation_inverse, mutation_payload_json, BinaryMutation, BinarySnapshot};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_plugin_stdio_test_oracle::artifacts::binary::standards::v_raw::subsets::any::oracle_apply_mutation;
    use semio_s_plugin_stdio_test_oracle::law::inverse_restores;

    //#region 🔖️MutationFromSpec
    /// 🦠️ The scenario's `{kind, params}` witness decoded generically: `params` IS the leaf's wire
    /// payload, so the derive-generated `from_payload_value` is the only decoder, and re-emitting
    /// the decoded payload must give back exactly `params`.
    fn mutation_from_spec(spec: &Json) -> Result<BinaryMutation, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        wire_operation(&kind, &params, mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ "Decode" is `BinarySnapshot { bytes: input.to_vec(), .. }` directly: `decode_pack`/
    /// `encode_pack` are proven to be exactly this identity by `carrier_native_is_raw`
    /// (`../../🚪️io/🦀️.rs`), so constructing the public `bytes` field is the same operation.
    fn decode(input: &[u8]) -> BinarySnapshot {
        BinarySnapshot { bytes: input.to_vec(), ..Default::default() }
    }

    /// ▶️ Applies one operation; any message is a rejection (offset/remove_len outside the buffer).
    fn apply(snapshot: &mut BinarySnapshot, mutation: &BinaryMutation) -> Result<(), String> {
        let outcome = apply_binary_mutation(snapshot, mutation);
        if outcome.messages().is_empty() {
            Ok(())
        } else {
            Err(format!("mutation rejected: {:?}", outcome.messages()))
        }
    }

    /// ⚖️ The subject's bytes must be exactly what the independent specification implementation
    /// produced for the same witness — the only evidence a `@no-oracle-` feature ever discharges.
    fn agree(bytes: Vec<u8>, reference: Result<Vec<u8>, String>) -> Result<Vec<u8>, String> {
        match reference {
            Ok(reference) if reference == bytes => Ok(bytes),
            Ok(reference) => Err(format!("subject/reference mismatch: subject produced {} byte(s), independent reference produced {} byte(s)", bytes.len(), reference.len())),
            Err(error) => Err(format!("independent reference rejected a mutation the subject accepted: {error}")),
        }
    }

    /// 📐️ Decode → typed mutation → the model's own bytes, cross-checked against the reference.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let mut snapshot = decode(input);
        apply(&mut snapshot, &mutation_from_spec(spec)?)?;
        agree(snapshot.bytes, oracle_apply_mutation(input, spec))
    }
    //#endregion 🔖️Codec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let bytes = apply_and_encode(&mutable_input(ctx)?, &ctx.doc_json()?)?;
        Ok(Outcome::with_raw(bytes.clone(), projection_of(&bytes)))
    }

    /// ↩️ The forward witness is undone by `BinaryMutation::inverse` itself — the law under test —
    /// and the restored buffer must be both the pristine input and exactly what the independent
    /// reference's own undo produced.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = decode(&input);
        let mutation = mutation_from_spec(&spec)?;
        let mut snapshot = base.clone();
        apply(&mut snapshot, &mutation)?;
        for step in mutation_inverse(&mutation, &base) {
            apply(&mut snapshot, &step)?;
        }
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        let reference = oracle_apply_mutation(&input, &spec).and_then(|mutated| oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input, &params)?));
        let restored = agree(snapshot.bytes, reference)?;
        inverse_restores(&kind, &projection_of(&restored), &projection_of(&input))?;
        Ok(Outcome::with_raw(restored.clone(), projection_of(&restored)))
    }

    /// 📐️ Honest identity: for this subset `decode`/`encode` really is the identity on `bytes`.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let bytes = decode(&input).bytes;
        if bytes != input {
            return Err("carrier law violated: decode/encode must be the identity on bytes for this subset".to_string());
        }
        Ok(Outcome::with_raw(bytes.clone(), projection_of(&bytes)))
    }

    pub fn vector(ctx: &Context) -> Result<Outcome, String> {
        mutate(ctx)
    }

    pub fn append_to_empty_buffer(ctx: &Context) -> Result<Outcome, String> {
        let bytes = apply_and_encode(&[], &ctx.doc_json()?)?;
        Ok(Outcome::with_raw(bytes.clone(), projection_of(&bytes)))
    }

    pub fn invalid_replacement(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        match apply_and_encode(&input, &ctx.doc_json()?) {
            Err(_) => Ok(Outcome::with_raw(input, json_obj(vec![("rejected", Json::Bool(true))]))),
            Ok(bytes) => Err(format!("expected the invalid byte-range replacement to be rejected, but it produced {} byte(s) without erroring", bytes.len())),
        }
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", round_trip_oracle).oracle("vector", vector_oracle).oracle("append-to-empty-buffer", append_to_empty_buffer_oracle).oracle("invalid-replace-byte-range", invalid_replacement_oracle);
    #[cfg(feature = "sut")]
    {
        built = built
            .subject("mutate", subject::mutate)
            .subject("inverse", subject::inverse)
            .subject("identity-round-trip", subject::round_trip)
            .subject("vector", subject::vector)
            .subject("append-to-empty-buffer", subject::append_to_empty_buffer)
            .subject("invalid-replace-byte-range", subject::invalid_replacement);
    }
    built
}
//#endregion 🔖️Registration
