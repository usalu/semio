//! 🦀️ UTF-8 text-line exhaustive mutation round-trip case — Rust adapter.
//!
//! The oracle role reads every document with the registered third-party reader
//! (`bstr-txt-utf-8-mutate-reader`, `bstr_split` in `../../🔮️oracles/🦀️.rs`) and applies each kind to
//! the lines bstr read; it never calls this repository's production `TxtSnapshot`/`TxtMutation`
//! code. The `@id-spec-vector` rows pin the format's own split rule (a bare LF inside a CRLF document
//! is line content, which bstr reads as a boundary), so they stay on the hand-written re-derivation.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. The subject half is gated behind the generated host's `sut`
//! feature so the oracle-only run never compiles the local implementation. Both roles assert their
//! laws in role rather than deferring every check to the comparison.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::txt::standards::v_utf_8::subsets::any::{bstr_split, independent_render, independent_split, oracle_apply_mutation, oracle_inverse_spec, project_txt};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, round_trip_preserves};

//#region 🔖️Kinds

/// 🧾️ The `@id-spec-vector` Examples table's row ids, in declaration order — registration only,
/// same reasoning as `KINDS` (these ids are not catalog kinds, so the completeness gate does not
/// check them; a missing registration is still a hard runtime error if the scenario ever runs).
const VECTOR_IDS: &[&str] =
    &["pure-lf", "pure-crlf", "lf-no-trailing-terminator", "mixed-crlf-and-bare-lf", "bom-as-first-line-content", "astral-emoji-and-variation-selectors", "combining-mark-distinct-from-precomposed", "nel-ls-ps-as-ordinary-content", "empty-document"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🎙️interview-transkript.tex";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("📥️input.txt"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️SpecHelpers
/// 🔤️ Reads the `@id-spec-vector` docstring, which is a bare JSON STRING (not an object like the
/// mutate/inverse specs) — the exact literal byte vector the scenario asserts round-trips.
fn spec_vector_text(spec: &Json) -> Result<String, String> {
    match spec {
        Json::String(text) => Ok(text.clone()),
        other => Err(format!("spec vector docstring must be a JSON string, got {other:?}")),
    }
}
//#endregion 🔖️SpecHelpers

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let output = oracle_apply_mutation(&input, &spec)?;
    let projection = project_txt(&output)?;
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ The inverse law, asserted HERE by the independent implementation against its own pre-mutation
/// reading rather than deferred to a comparison: `apply(m)` followed by `apply(inverse(m))` has to
/// land back on the ORIGINAL document's semantic projection — every line, the trailing-terminator
/// flag and the whole-document line ending.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let undo = oracle_inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_txt(&restored)?;
    inverse_restores(&spec.str("kind"), &projection, &project_txt(&input)?)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent split/render this subset's
/// mutations use. Unlike every other format in this wave, byte-identical output here is the
/// CORRECT and EXPECTED result — splitting a string on a fixed separator and rejoining with that
/// same separator is a mathematical identity regardless of content (this subset's carrier law; see
/// the feature file's own note and `mixed_crlf_lf_is_still_a_lossless_round_trip` in the oracle
/// module). A must-differ tripwire would therefore be a fabricated law; the carrier law is asserted
/// in its place, which is the same claim stated the way this format can honestly satisfy it: the
/// output must equal the input EXACTLY, and the projection must be preserved. Both still fail
/// loudly if the split or the render ever drifts.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let (lines, trailing, crlf) = bstr_split(&input)?;
    let output = independent_render(&lines, trailing, crlf).into_bytes();
    carrier_is_exact(&output, &input)?;
    let projection = project_txt(&output)?;
    round_trip_preserves(&projection, &project_txt(&input)?)?;
    Ok(Outcome::with_raw(output, projection))
}

/// 🧪️ A literal specification vector, decoded and re-encoded through the independent
/// split/render alone, asserted bit-identical to the vector itself.
fn spec_vector_oracle(ctx: &Context) -> Result<Outcome, String> {
    let vector = spec_vector_text(&ctx.doc_json()?)?;
    let (lines, trailing, crlf) = independent_split(&vector);
    let output = independent_render(&lines, trailing, crlf);
    if output != vector {
        return Err(format!("independent split/render is not bit-identical for vector {vector:?}: got {output:?}"));
    }
    Ok(Outcome::with_raw(output.clone().into_bytes(), Json::String(output)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, spec_vector_text};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_txt::standards::v_utf_8::subsets::any::schema::mutations::{InsertLineMutation, RemoveLineMutation, SetLineEndingMutation, SetLineMutation, SetTrailingNewlineMutation, apply_txt_mutation};
    use semio_s_artifact_stdio_txt::standards::v_utf_8::subsets::any::schema::snapshot::LineEnding;
    use semio_s_artifact_stdio_txt::{STDIO_TXT_DOCUMENT_SCHEMA, TxtMutation, TxtSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::txt::standards::v_utf_8::subsets::any::project_txt;
    use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, round_trip_preserves};

    fn json_u32(params: &Json, key: &str) -> Result<u32, String> {
        match params.get(key) {
            Some(Json::Number(value)) if value.is_finite() && value.fract() == 0.0 && *value >= 0.0 && *value <= u32::MAX as f64 => Ok(*value as u32),
            _ => Err(format!("mutation spec is missing numeric `{key}`")),
        }
    }

    fn json_bool(params: &Json, key: &str) -> bool {
        matches!(params.get(key), Some(Json::Bool(true)))
    }

    fn json_strings(params: &Json, key: &str) -> Vec<String> {
        params
            .array(key)
            .iter()
            .map(|entry| match entry {
                Json::String(text) => text.clone(),
                _ => String::new(),
            })
            .collect()
    }

    fn line_ending_of(params: &Json, key: &str) -> LineEnding {
        if params.str(key) == "crLf" { LineEnding::CrLf } else { LineEnding::Lf }
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `TxtMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(spec: &Json) -> Result<TxtMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "set-trailing-newline" => TxtMutation::SetTrailingNewline(SetTrailingNewlineMutation { value: json_bool(&params, "value") }),
            "set-line-ending" => TxtMutation::SetLineEnding(SetLineEndingMutation { value: line_ending_of(&params, "value") }),
            "insert-line" => TxtMutation::InsertLine(InsertLineMutation { index: json_u32(&params, "index")?, text: params.str("text") }),
            "remove-line" => TxtMutation::RemoveLine(RemoveLineMutation { index: json_u32(&params, "index")? }),
            "set-line" => TxtMutation::SetLine(SetLineMutation { index: json_u32(&params, "index")?, text: params.str("text") }),
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    fn decode(bytes: &[u8]) -> Result<TxtSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        Ok(TxtSnapshot::from_body(&text))
    }

    /// 👁️ The forward mutation, with the OBSERVABILITY law asserted IN ROLE — an un-asserting handler
    /// here would report green while proving nothing. Two outcomes are admissible and they are told apart, never merged: the
    /// subset ACCEPTS the mutation, in which case the named semantic operation must move the
    /// projection; or it REFUSES it with `stdio.txt.mutation-not-representable`, in which case the
    /// bytes must be exactly the input's (see the feature's 🔒️ note — `set-trailing-newline false`
    /// on a fixture whose last line is empty is the one documented refusal, and it is required to
    /// be that kind and no other, so a codec that started refusing everything would fail here).
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode(&input)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let mutation = mutation_from_spec(&spec)?;
        let outcome = apply_txt_mutation(&mut snapshot, &mutation);
        let output = snapshot.to_body().into_bytes();
        let projection = project_txt(&output)?;
        if outcome.messages().is_empty() {
            if projection == project_txt(&input)? {
                return Err(format!("{kind:?} was accepted and still left the semantic projection exactly as it found it -- the parameters address nothing in the real document"));
            }
        } else {
            if kind != "set-trailing-newline" {
                return Err(format!("{kind:?} was refused on the real document, and only `set-trailing-newline` is documented as unrepresentable here: {:?}", outcome.messages()));
            }
            if output != input {
                return Err(format!("{kind:?} was refused and still changed the document -- a refusal must leave the bytes untouched: {:?}", outcome.messages()));
            }
        }
        Ok(Outcome::with_raw(output, projection))
    }

    /// ↩️ The inverse law, asserted IN ROLE through the same shared `⚖️law` helper the oracle
    /// handler uses — apply-then-undo must land back on the REAL original document's projection.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        let mutation = mutation_from_spec(&spec)?;
        let inverse = <TxtMutation as protocol::Mutation<TxtSnapshot>>::inverse(&mutation, &snapshot);
        apply_txt_mutation(&mut snapshot, &mutation);
        for step in inverse {
            apply_txt_mutation(&mut snapshot, &step);
        }
        let output = snapshot.to_body().into_bytes();
        let projection = project_txt(&output)?;
        inverse_restores(&spec.str("kind"), &projection, &project_txt(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone. See the module doc comment and
    /// the feature file: for this carrier-law subset, byte-identical output is the CORRECT result,
    /// so — deliberately, unlike every other case in this wave — no pass-through tripwire fires
    /// here.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = snapshot.to_body().into_bytes();
        carrier_is_exact(&output, &input)?;
        let projection = project_txt(&output)?;
        round_trip_preserves(&projection, &project_txt(&input)?)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🧪️ A literal specification vector, decoded and re-encoded through this repository's own
    /// `TxtSnapshot::from_body`/`to_body`, asserted bit-identical to the vector itself.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let vector = spec_vector_text(&ctx.doc_json()?)?;
        let output = TxtSnapshot::from_body(&vector).to_body();
        if output != vector {
            return Err(format!("decode/encode is not bit-identical for vector {vector:?}: got {output:?}"));
        }
        Ok(Outcome::with_raw(output.clone().into_bytes(), Json::String(output)))
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
    for id in VECTOR_IDS {
        built = built.oracle(&format!("spec-vector-{id}"), spec_vector_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("spec-vector-{id}"), subject::spec_vector);
        }
    }
    built
}
//#endregion 🔖️Registration
