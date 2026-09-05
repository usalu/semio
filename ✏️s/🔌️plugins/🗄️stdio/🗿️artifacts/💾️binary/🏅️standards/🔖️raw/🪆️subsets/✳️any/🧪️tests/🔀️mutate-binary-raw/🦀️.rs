//! 🦀️ Raw-binary exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-
//! REFACTOR wave 7. Recorded no-oracle decision `raw-buffer-no-format` (`../../🏅️standards/🔖️raw/
//! 🪆️subsets/✳️any/🔣️oracle.json`): a raw byte buffer has no format, so `oracle` here
//! drives this subset's own independently written specification-vector implementation
//! (`../../🏅️standards/🔖️raw/🪆️subsets/✳️any/🦀️oracle.rs`'s `oracle_apply_mutation`,
//! which never touches the subject's own `BinaryDiff`/`apply_binary_mutation`); `subject` drives
//! this repository's own `apply_binary_mutation` over the full 5-kind `BinaryMutation` vocabulary,
//! then cross-checks its own result against that SAME independent reference before returning —
//! deliberate, because the test framework's `oracleDecision` never invokes the `oracle` role at all
//! for a `@no-oracle-` feature (see the `subject::apply_and_encode` doc comment below), so `subject`
//! is the only role that ever actually discharges this decision's specification-vector evidence.
//! Both sides project to the exact output byte array and `exact-bytes-v1` compares them literally —
//! there is no independent reader to project through, because there is no format to read. The
//! subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::binary::standards::v_raw::subsets::any::oracle_apply_mutation;
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores};

//#region 🔖️Kinds
/// 🏷️ Mirrors this subset's own `BinaryMutation::KINDS` (`../../🏅️standards/🔖️raw/🪆️subsets/✳️any/
/// 🧬️schema/🧬️mutations/🦀️.rs`). Kept as a plain literal here rather than imported since
/// this adapter's oracle-only build never links the subject crate — the contract gate (mutation
/// coverage against the `binary-raw-any` catalog) is what keeps the two lists honest against
/// each other.
const KINDS: &[&str] = &["set-snapshot", "splice", "append-bytes", "truncate-at"];
//#endregion 🔖️Kinds

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
/// 🔎️ A byte payload as the wire protocol carries it: a plain JSON array of 0-255 numbers (the
/// protocol's `Json` has no base64 accessor — see `../../../🎥️mp4/🧪️tests/🐙️mutate-mp4-isobmff/
/// 🦀️.rs`'s own local `bytes` helper, which this mirrors).
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
/// array, literally. There is no semantic summary to fall back on — for a raw buffer the bytes ARE
/// the whole content, so anything less than the full array would silently under-compare.
fn projection_of(bytes: &[u8]) -> Json {
    bytes_json(bytes)
}
//#endregion 🔖️JsonBuild

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair, computed
/// directly from the REAL pristine `input` bytes at run time rather than a hardcoded literal — the
/// `truncate-at` example alone removes a 283 KB real tail no literal could carry legibly.
/// `set-snapshot` inverts through a REAL `set-snapshot` carrying the pristine buffer as its own
/// payload — mirroring `BinaryMutation::inverse`'s own `SetSnapshot` arm (`../../🏅️standards/
/// 🔖️raw/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — rather than the caller handing
/// the untouched input straight back, which would let the scenario pass without the independent
/// implementation performing the undo at all.
fn inverse_spec(kind: &str, input: &[u8], params: &Json) -> Json {
    match kind {
        "set-snapshot" => json_spec("set-snapshot", json_obj(vec![("snapshot", json_obj(vec![("bytes", bytes_json(input))]))])),
        "splice" => {
            let offset = usize_field(params, "offset").unwrap_or(0).min(input.len());
            let remove_len = usize_field(params, "removeLen").unwrap_or(0);
            let insert = bytes_field(params, "insert");
            let end = (offset + remove_len).min(input.len());
            let removed = input[offset..end].to_vec();
            json_spec("splice", json_obj(vec![("offset", Json::Number(offset as f64)), ("removeLen", Json::Number(insert.len() as f64)), ("insert", bytes_json(&removed))]))
        }
        "append-bytes" => json_spec("truncate-at", json_obj(vec![("offset", Json::Number(input.len() as f64))])),
        "truncate-at" => {
            let offset = usize_field(params, "offset").unwrap_or(input.len()).min(input.len());
            let tail = input[offset..].to_vec();
            json_spec("splice", json_obj(vec![("offset", Json::Number(offset as f64)), ("removeLen", Json::Number(0.0)), ("insert", bytes_json(&tail))]))
        }
        other => json_spec(other, json_obj(vec![])),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
/// 🔮️ Applies the declared mutation with this subset's own independent specification
/// implementation and projects the resulting bytes directly (there is no reader to project
/// through — the projection IS the output).
fn apply_and_project(input: &[u8], spec: &Json) -> Result<Outcome, String> {
    let bytes = oracle_apply_mutation(input, spec)?;
    let projection = projection_of(&bytes);
    Ok(Outcome::with_raw(bytes, projection))
}

fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    apply_and_project(&input, &spec)
}

/// ↩️ The inverse law, asserted HERE by the independent specification implementation against the
/// pristine buffer rather than deferred to a comparison: every kind — INCLUDING `set-snapshot`,
/// which now inverts through a real `set-snapshot` of the original bytes instead of a hand-back —
/// is applied forward and then undone, and the restored buffer must be the original buffer. For a
/// raw carrier the projection IS the byte array, so this is a literal exact-bytes claim. This
/// subset carries a recorded no-oracle decision, so nothing else will ever check it.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let empty = Json::Object(Vec::new());
    let params = spec.get("params").unwrap_or(&empty).clone();
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec(&kind, &input, &params))?;
    let projection = projection_of(&restored);
    inverse_restores(&kind, &projection, &projection_of(&input))?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔮️ For a raw buffer `decode`/`encode` really is the identity (`carrier_native_is_raw`,
/// `../../🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs`), so the trusted reference result
/// is simply the pristine original bytes — byte equality IS correct here, honestly, not a
/// contrived pass. The no-byte-pass-through tripwire every parsed format in this wave asserts is
/// therefore genuinely inapplicable, and inverting it would be a fabricated law; the reference
/// states the carrier law it can honestly satisfy instead, by running the declared `no-mutation`
/// kind through the independent implementation and requiring the result to be the input exactly.
/// That is a real check, not a tautology: `apply` reaching the wrong arm, or clamping, or dropping
/// a byte, all fail it.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_apply_mutation(&input, &json_spec("no-mutation", json_obj(vec![])))?;
    carrier_is_exact(&output, &input)?;
    Ok(Outcome::with_raw(output.clone(), projection_of(&output)))
}

/// 🔮️ The specification-vector scenarios share the SAME forward-apply shape as `mutate_oracle`,
/// just against whichever `kind` the vector names (not necessarily `<id>`).
fn vector_oracle(ctx: &Context) -> Result<Outcome, String> {
    mutate_oracle(ctx)
}

fn append_to_empty_buffer_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    apply_and_project(&[], &spec)
}

/// 🔮️ An invalid splice must be REJECTED, never silently applied. Rejection is itself the passing
/// outcome — a handler that returns `Err` here would mean the framework SILENTLY skipped a scenario
/// registration, not "the mutation was invalid"; that failure mode is caught by returning `Err`
/// only when the reference did NOT reject the invalid input.
fn invalid_splice_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    match oracle_apply_mutation(&input, &spec) {
        Err(_) => Ok(Outcome::with_raw(input, json_obj(vec![("rejected", Json::Bool(true))]))),
        Ok(bytes) => Err(format!("expected the invalid splice to be rejected, but it produced {} byte(s) without erroring", bytes.len())),
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{bytes_field, inverse_spec, json_obj, mutable_input, projection_of, usize_field};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::binary::standards::v_raw::subsets::any::schema::mutations::{append_bytes, apply_binary_mutation, set_snapshot, splice, truncate_at, BinaryMutation};
    use semio_s_plugin_stdio::artifacts::binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::binary::standards::v_raw::subsets::any::oracle_apply_mutation;
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores};

    //#region 🔖️MutationFromSpec
    /// 🦠️ The same `(kind, params)` wire shape the oracle dispatcher reads, translated into a real
    /// `BinaryMutation` value for this subset's own `apply_binary_mutation`.
    fn mutation_from_spec(spec: &Json) -> Result<BinaryMutation, String> {
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty);
        Ok(match kind.as_str() {
            "set-snapshot" => {
                let snapshot_json = params.get("snapshot").ok_or("set-snapshot requires a snapshot field")?;
                BinaryMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: BinarySnapshot { bytes: bytes_field(snapshot_json, "bytes"), ..Default::default() } })
            }
            "splice" => BinaryMutation::ReplaceByteRange(replace_byte_range::ReplaceByteRange { offset: usize_field(params, "offset")?, remove_len: usize_field(params, "removeLen")?, insert: bytes_field(params, "insert") }),
            "append-bytes" => BinaryMutation::AppendBytes(append_bytes::AppendBytes { data: bytes_field(params, "data") }),
            "truncate-at" => BinaryMutation::TruncateAt(truncate_at::TruncateAt { offset: usize_field(params, "offset")? }),
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }
    //#endregion 🔖️MutationFromSpec

    //#region 🔖️Codec
    /// 📐️ "Decode" is `BinarySnapshot { bytes: input.to_vec(), .. }` directly — not routed through
    /// `store::ArtifactPack::decode_pack` (that trait alias is PRIVATE to this subset's own crate,
    /// established by its own `extern crate semio_framework_os_kernel as store;`, and not reachable
    /// from an external test-host crate). `BinarySnapshot::decode_pack`/`encode_pack` are proven
    /// to be exactly this identity by `carrier_native_is_raw` (`../../🏅️standards/🔖️raw/🪆️subsets/
    /// ✳️any/🚪️io/🦀️.rs`), so constructing/reading the public `bytes` field directly here
    /// is the same operation, not a shortcut around it.
    fn decode(input: &[u8]) -> BinarySnapshot {
        BinarySnapshot { bytes: input.to_vec(), ..Default::default() }
    }

    /// 📐️ Full parse → typed mutation → re-serialize from the model alone. A successful mutation is
    /// free to be byte-identical to the input here (e.g. `no-mutation`) — unlike every sibling
    /// subset in this wave, this one's decode/encode really is the carrier-law identity, so the
    /// no-byte-pass-through tripwire cannot apply; only an outright REJECTED mutation (offset/
    /// remove_len outside the buffer) is an error.
    ///
    /// Cross-checked here, INSIDE `subject`, against `oracle_apply_mutation` — this subset's own
    /// independently written specification reference (`../../🏅️standards/🔖️raw/🪆️subsets/✳️any/
    /// 🦀️oracle.rs`). That is deliberate, not redundant with the top-level `oracle`
    /// registrations above: the test framework's `oracleDecision` (`🧰️framework/🛍️products/🦑️repo/
    /// 🔨️modules/🧪️tests/📜️cript.ts`) never invokes the `oracle` role at all for a feature carrying
    /// `@no-oracle-` instead of `@oracle-` — its whole `oracle`/parity machinery exists for a
    /// registered THIRD-PARTY reference, which this subset by definition has none of. `subject` is
    /// therefore the only role the runner ever actually executes for this case, so the
    /// specification-vector substitute this no-oracle decision rests on has to be discharged here,
    /// self-contained, exactly like the repository's other no-oracle precedents (`🧰️framework/
    /// 🔨️modules/🎠️kernel/🧪️tests/🚫️reject-malformed-version-input`, `🧰️framework/🔨️modules/🖱️ui/
    /// 🔨️modules/🏷️class-name-composition/🧪️tests/🔀️merge-conflicting-utilities`).
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let mut snapshot = decode(input);
        let mutation = mutation_from_spec(spec)?;
        let outcome = apply_binary_mutation(&mut snapshot, &mutation);
        if !outcome.messages().is_empty() {
            return Err(format!("mutation rejected: {:?}", outcome.messages()));
        }
        let bytes = snapshot.bytes;
        match oracle_apply_mutation(input, spec) {
            Ok(reference) if reference == bytes => Ok(bytes),
            Ok(reference) => Err(format!("subject/reference mismatch: subject produced {} byte(s), independent reference produced {} byte(s)", bytes.len(), reference.len())),
            Err(error) => Err(format!("independent reference rejected a mutation the subject accepted: {error}")),
        }
    }
    //#endregion 🔖️Codec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let bytes = apply_and_encode(&input, &spec)?;
        Ok(Outcome::with_raw(bytes.clone(), projection_of(&bytes)))
    }

    /// ↩️ Every kind, INCLUDING `set-snapshot`, is genuinely applied forward and then undone through
    /// this repository's own `BinaryMutation` pipeline — `set-snapshot` inverts through a real
    /// `set-snapshot` carrying the pristine buffer, never through a hand-back of the input bytes.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let params = spec.get("params").unwrap_or(&empty).clone();
        let mutated = apply_and_encode(&input, &spec)?;
        let restored = apply_and_encode(&mutated, &inverse_spec(&kind, &input, &params))?;
        Ok(Outcome::with_raw(restored.clone(), projection_of(&restored)))
    }

    /// 📐️ Honest identity: for this subset `decode`/`encode` really is the identity on `bytes`, so
    /// the no-byte-pass-through tripwire every sibling subset enforces cannot apply.
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
        let spec = ctx.doc_json()?;
        let bytes = apply_and_encode(&[], &spec)?;
        Ok(Outcome::with_raw(bytes.clone(), projection_of(&bytes)))
    }

    pub fn invalid_splice(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        match apply_and_encode(&input, &spec) {
            Err(_) => Ok(Outcome::with_raw(input, json_obj(vec![("rejected", Json::Bool(true))]))),
            Ok(bytes) => Err(format!("expected the invalid splice to be rejected, but it produced {} byte(s) without erroring", bytes.len())),
        }
    }
    //#endregion 🔖️Handlers
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

    for id in ["zero-length-splice", "splice-at-offset-zero", "splice-at-exact-end", "splice-spans-whole-buffer", "truncate-to-zero", "truncate-beyond-length"] {
        built = built.oracle(&format!("vector-{id}"), vector_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("vector-{id}"), subject::vector);
        }
    }

    built = built.oracle("append-to-empty-buffer", append_to_empty_buffer_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("append-to-empty-buffer", subject::append_to_empty_buffer);
    }

    for id in ["offset-beyond-buffer", "remove-len-exceeds-buffer"] {
        built = built.oracle(&format!("invalid-splice-{id}"), invalid_splice_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("invalid-splice-{id}"), subject::invalid_splice);
        }
    }

    built
}
//#endregion 🔖️Registration
