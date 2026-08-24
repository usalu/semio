//! 🦀️ UTF-8 text-line exhaustive mutation round-trip case — Rust adapter.
//!
//! No differential oracle is registered for this subset (see `../../🏅️standards/🔖️utf-8/
//! 🪆️subsets/✳️any/🧪️oracle/🔣️component.json`'s `noOracleDecisions`), so the "oracle" role handlers
//! below are never dispatched by the repository test platform for a `@no-oracle-` feature (it has
//! no `@oracle-<id>` tag to resolve an implementation for). They are still registered, matching
//! every other stdio case's shape, and they still compute a REAL, independently-derived answer
//! (`oracle_apply_mutation`/`independent_split`/`independent_render` from this subset's own
//! `🧪️oracle/🦀️component.rs`, which never calls this repository's production `TxtSnapshot`/
//! `TxtMutation` code) — genuinely useful once the subject phase compiles again, and exercised
//! directly by that module's own `cargo test --features oracles --lib` unit tests today.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. The subject half is gated behind the generated host's `sut`
//! feature so the oracle-only run never compiles the local implementation — see §5.3 of the fleet
//! brief. The Rust SUBJECT phase cannot compile this wave (a concurrent os-kernel refactor), so it
//! is written and gated but not run.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::txt::standards::v_utf_8::subsets::any::{independent_render, independent_split, oracle_apply_mutation, project_txt};
use semio_s_plugin_stdio_test_oracle::law::{carrier_is_exact, inverse_restores, round_trip_preserves};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `txt-utf-8-any` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs::KINDS` — that
/// module lives in the SUBJECT crate, and the oracle role must not link the subject crate at all
/// (fleet brief §5.3). That other `KINDS` carries its own test proving it matches the enum AND the
/// catalog manifest; a mismatch HERE against either one is caught structurally instead — the
/// contract phase fails with `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list
/// omits or invents a kind, and the runner fails every unregistered scenario id outright.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-trailing-newline", "set-line-ending", "insert-line", "remove-line", "set-line"];

/// 🧾️ The `@id-spec-vector` Examples table's row ids, in declaration order — registration only,
/// same reasoning as `KINDS` (these ids are not catalog kinds, so the completeness gate does not
/// check them; a missing registration is still a hard runtime error if the scenario ever runs).
const VECTOR_IDS: &[&str] = &["pure-lf", "pure-crlf", "lf-no-trailing-terminator", "mixed-crlf-and-bare-lf", "bom-as-first-line-content", "astral-emoji-and-variation-selectors", "combining-mark-distinct-from-precomposed", "nel-ls-ps-as-ordinary-content", "empty-document"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://📄️interview-transkript.tex";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.txt"))?;
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

fn line_ending_str(is_crlf: bool) -> &'static str {
    if is_crlf {
        "crLf"
    } else {
        "lf"
    }
}

/// ↩️ The inverse mutation's OWN spec, computed by reading whatever pre-mutation state it needs
/// straight out of `original` with the SAME independent reader the oracle mutates with — never by
/// calling this repository's own `TxtMutation::inverse`, which would defeat the point of an
/// independently-computed oracle. Mirrors that method's documented rule exactly (index-aware,
/// reading the pre-state it needs from the ORIGINAL document; `InsertLine`'s inverse lands at
/// `min(index, len)`, matching the clamped position it actually inserted at).
fn inverse_spec(original: &[u8], forward: &Json) -> Result<Json, String> {
    let body = std::str::from_utf8(original).map_err(|error| format!("input is not UTF-8: {error}"))?;
    let (lines, trailing_newline, is_crlf) = independent_split(body);
    let params = forward.get("params").cloned().unwrap_or(Json::Null);
    let index = |key: &str| match params.get(key) {
        Some(Json::Number(value)) => Some(*value as usize),
        _ => None,
    };
    match forward.str("kind").as_str() {
        "no-mutation" => Ok(kind_spec("no-mutation", json_object(vec![]))),
        "set-snapshot" => Ok(kind_spec(
            "set-snapshot",
            json_object(vec![("lines", Json::Array(lines.into_iter().map(Json::String).collect())), ("trailingNewline", Json::Bool(trailing_newline)), ("lineEnding", Json::String(line_ending_str(is_crlf).to_string()))]),
        )),
        "set-trailing-newline" => Ok(kind_spec("set-trailing-newline", json_object(vec![("value", Json::Bool(trailing_newline))]))),
        "set-line-ending" => Ok(kind_spec("set-line-ending", json_object(vec![("value", Json::String(line_ending_str(is_crlf).to_string()))]))),
        "insert-line" => {
            let requested = index("index").ok_or("insert-line inverse: missing `index`")?;
            let landed_at = requested.min(lines.len());
            Ok(kind_spec("remove-line", json_object(vec![("index", Json::Number(landed_at as f64))])))
        }
        "remove-line" => {
            let requested = index("index").ok_or("remove-line inverse: missing `index`")?;
            match lines.get(requested) {
                Some(text) => Ok(kind_spec("insert-line", json_object(vec![("index", Json::Number(requested as f64)), ("text", Json::String(text.clone()))]))),
                None => Ok(kind_spec("no-mutation", json_object(vec![]))),
            }
        }
        "set-line" => {
            let requested = index("index").ok_or("set-line inverse: missing `index`")?;
            match lines.get(requested) {
                Some(text) => Ok(kind_spec("set-line", json_object(vec![("index", Json::Number(requested as f64)), ("text", Json::String(text.clone()))]))),
                None => Ok(kind_spec("no-mutation", json_object(vec![]))),
            }
        }
        other => Err(format!("no inverse rule for kind {other:?}")),
    }
}

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
/// flag and the whole-document line ending. This subset carries a recorded no-oracle decision, so
/// nothing else will ever check it.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let undo = inverse_spec(&input, &spec)?;
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
    let body = std::str::from_utf8(&input).map_err(|error| format!("input is not UTF-8: {error}"))?;
    let (lines, trailing, crlf) = independent_split(body);
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
    use super::{inverse_spec, mutable_input, spec_vector_text};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::txt::standards::v_utf_8::subsets::any::schema::mutations::apply_txt_mutation;
    use semio_s_plugin_stdio::artifacts::txt::standards::v_utf_8::subsets::any::schema::snapshot::LineEnding;
    use semio_s_plugin_stdio::artifacts::txt::{TxtMutation, TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::artifacts::txt::standards::v_utf_8::subsets::any::project_txt;

    fn json_usize(params: &Json, key: &str) -> Result<usize, String> {
        match params.get(key) {
            Some(Json::Number(value)) => Ok(*value as usize),
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
        if params.str(key) == "crLf" {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        }
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `TxtMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(spec: &Json) -> Result<TxtMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => TxtMutation::NoMutation,
            "set-snapshot" => TxtMutation::SetSnapshot { snapshot: TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), lines: json_strings(&params, "lines"), trailing_newline: json_bool(&params, "trailingNewline"), line_ending: line_ending_of(&params, "lineEnding") } },
            "set-trailing-newline" => TxtMutation::SetTrailingNewline { value: json_bool(&params, "value") },
            "set-line-ending" => TxtMutation::SetLineEnding { value: line_ending_of(&params, "value") },
            "insert-line" => TxtMutation::InsertLine { index: json_usize(&params, "index")?, text: params.str("text") },
            "remove-line" => TxtMutation::RemoveLine { index: json_usize(&params, "index")? },
            "set-line" => TxtMutation::SetLine { index: json_usize(&params, "index")?, text: params.str("text") },
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    fn decode(bytes: &[u8]) -> Result<TxtSnapshot, String> {
        let text = String::from_utf8(bytes.to_vec()).map_err(|error| format!("input is not UTF-8: {error}"))?;
        Ok(TxtSnapshot::from_body(&text))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode(&mutable_input(ctx)?)?;
        let mutation = mutation_from_spec(&ctx.doc_json()?)?;
        apply_txt_mutation(&mut snapshot, &mutation);
        let output = snapshot.to_body().into_bytes();
        let projection = project_txt(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&input)?;
        apply_txt_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        apply_txt_mutation(&mut snapshot, &mutation_from_spec(&inverse_spec(&input, &spec)?)?);
        let output = snapshot.to_body().into_bytes();
        let projection = project_txt(&output)?;
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
        let projection = project_txt(&output)?;
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
