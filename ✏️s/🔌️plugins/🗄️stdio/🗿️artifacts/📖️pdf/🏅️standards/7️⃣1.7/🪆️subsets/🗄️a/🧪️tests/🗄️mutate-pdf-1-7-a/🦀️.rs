//! 🦀️ PDF 1.7 🗄️a exhaustive conformance-class mutation case — Rust adapter.
//!
//! Every scenario copies this subset's own committed lopdf-generated seed document (built by
//! this subset's own 🏭️generator, the same one that produced this catalog's per-mutation fixture
//! pairs) into the case work directory first; the committed asset is never written to. `oracle` handlers drive the registered `lopdf`
//! 0.44 reference implementation through this subset's own
//! `../../🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🔮️oracles/🦀️.rs`, `subject` handlers drive this
//! repository's own decode/mutate/encode round trip, and both results are read back by the SAME
//! independent `project_conformance` before the `semantic-pdf-conformance-a-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never compiles the local implementation.
//!
//! @see ../../🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🧬️schema/🦀️.rs — `check_pdf_a_conformance`, the one
//!      axis list this whole case derives from.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::a::{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, project_conformance};


//#region 🔖️Input
const INPUT: &str = "asset://🧬️conformance-seed/🧬️conformance-seed.pdf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(INPUT, Some("conformance-seed.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The real pre-state a scenario's mutation runs on. For the kinds whose target the committed
/// document does not carry, this is the real document after the reference implementation has
/// independently put it there — the feature file names every one of them rather than papering over
/// it. Every other kind reads the committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Law
/// 🔬️ First structural divergence between two projections — a dotted field path plus both values,
/// so a law that fails names WHICH axis moved instead of only "not equal". Kept local to this
/// adapter: a case adapter is a leaf that links the framework host and its own subset's oracle, and
/// nothing else.
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    let here = if path.is_empty() { "the projection".to_string() } else { path.to_string() };
    let child = |key: &str| if path.is_empty() { key.to_string() } else { format!("{path}.{key}") };
    match (expected, actual) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in left {
                match right.iter().find(|(name, _)| name == key) {
                    Some((_, other)) => {
                        if let Some(found) = first_divergence(&child(key), value, other) {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{} is gone (the original carried {})", child(key), brief(value))),
                }
            }
            right.iter().find(|(name, _)| !left.iter().any(|(other, _)| other == name)).map(|(name, value)| format!("{} appeared (absent in the original, now {})", child(name), brief(value)))
        }
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{here} has {} entries, the original had {}", right.len(), left.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&child(&index.to_string()), value, other))
        }
        (left, right) if left == right => None,
        (left, right) => Some(format!("{here} is {} — the original had {}", brief(right), brief(left))),
    }
}

fn brief(value: &Json) -> String {
    let text = value.to_string();
    match text.char_indices().nth(160) {
        Some((cut, _)) => format!("{}…", &text[..cut]),
        None => text,
    }
}
//#endregion 🔖️Law

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id — and the place the OBSERVABILITY law
/// is asserted in-role: a mutation that leaves this subset's conformance-class projection untouched
/// proves nothing, whatever the reference implementation returned.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let output = oracle_apply_mutation(&base, &spec)?;
    let projection = project_conformance(&output)?;
    if projection == project_conformance(&base)? {
        return Err(format!("mutate-{}: the mutation left the conformance-class projection unchanged — a mutation that is not observable proves nothing", spec.str("kind")));
    }
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id — and the place the INVERSE law is
/// asserted in-role, without needing the subject: `apply(inverse(m), apply(m, base))` must land back
/// on the pre-state's own projection, read through the same independent reader.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let original = project_conformance(&base)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let undo = oracle_inverse_spec(&base, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_conformance(&restored)?;
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("inverse-{}: the mutation followed by its own computed inverse ({}) did not restore the document — {}", spec.str("kind"), undo.str("kind"), divergence));
    }
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, asserted rather than merely claimed in
/// prose: `lopdf` parses the whole seed object graph and re-serializes a fresh file from that graph
/// alone, and BOTH halves are checked — the bytes must differ from the input (nothing was copied)
/// and their projection must be identical to the input's (nothing was lost).
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    if output == input {
        return Err("byte pass-through: the re-serialized document is bit-identical to the input".to_string());
    }
    let projection = project_conformance(&output)?;
    let original = project_conformance(&input)?;
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("identity round trip: parsing and re-serializing the real document did not preserve its conformance-class projection — {divergence}"));
    }
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::{decode_pdf, encode_pdf};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_pdf::{mutation_from_payload_json, mutation_payload_json};
    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::a::io::mutation_bridge::apply_a_conformance_mutation;
    use semio_s_artifact_stdio_pdf::standards::v1_7::subsets::a::schema::mutations::PdfAMutation;
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::a::{oracle_inverse_spec, project_conformance};

    fn decode(bytes: &[u8]) -> Result<PdfSnapshot, String> {
        decode_pdf(bytes).map_err(|error| error.to_string())
    }

    fn encode(snapshot: &PdfSnapshot) -> Result<Vec<u8>, String> {
        encode_pdf(snapshot).map_err(|error| error.to_string())
    }

    /// 📨️ The scenario's `{kind, params}` row — or the oracle's computed undo spec — decoded generically: `params` is the
    /// leaf wire payload, the only channel between the feature and the subject's typed `PdfAMutation`.
    fn mutation_from_spec(spec: &Json) -> Result<PdfAMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(&spec)?;
        apply_a_conformance_mutation(&mut snapshot, &mutation);
        let output = encode(&snapshot)?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = arranged_input(ctx, &spec)?;
        let mut snapshot = decode(&base)?;
        let forward = mutation_from_spec(&spec)?;
        apply_a_conformance_mutation(&mut snapshot, &forward);
        let undo = oracle_inverse_spec(&base, &spec)?;
        let backward = mutation_from_spec(&undo)?;
        apply_a_conformance_mutation(&mut snapshot, &backward);
        let output = encode(&snapshot)?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating, and this tripwire catches it.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode(&snapshot)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_conformance(&output)?;
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
