//! 🦀️ XLSX ECMA-376 exhaustive mutation round-trip case — Rust adapter.
//!
//! Every scenario copies the immutable real fixture into the case work directory first; the
//! committed file is never written to. `oracle` handlers drive the registered `calamine` +
//! `rust_xlsxwriter` reference pairing (via this subset's own `🦀️.rs`), `subject`
//! handlers drive this repository's own decode/mutate/encode round trip, and both results are read
//! back by the SAME independent reader (`project_xlsx_workbook`) before the `semantic-spreadsheet-v1`
//! profile compares them. The subject half is gated behind the generated host's `sut` feature so the
//! oracle-only run never compiles the local implementation — see §5.3 of the fleet brief.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::base::{oracle_apply_inverse, oracle_apply_mutation, oracle_arrange, oracle_round_trip, project_shared_string_pool, project_xlsx_workbook};


//#region 🔖️Input
const INPUT: &str = "shared://📕️reuse-marketplaces.xlsx";
/// 📑️ The real fixture's own baseline: `xl/sharedStrings.xml` reports `uniqueCount="229"` —
/// confirmed by unzipping the committed file, not assumed. `calamine` cannot re-derive this number
/// (see the oracle module's doc comment), so it is tracked here instead.
const BASELINE_SHARED_STRING_COUNT: usize = 229;

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.xlsx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The real pre-state a scenario's kind runs on: `remove-shared-string` needs a pool entry no cell references, and
/// every entry of the real pool is referenced, so that kind runs on the real workbook after the reference has appended one
/// unreferenced entry (`oracle_arrange`). Every other kind reads the committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️SpecHelpers
/// 🔢️ The `sharedStringCount` a kind moves the pool to, and the value every OTHER declared kind leaves untouched — real
/// arithmetic on the real fixture's baseline, not a placeholder: the grid kinds never touch the pool, and every cell value
/// this case's rows write is a literal (`inlineString`/`number`), never a pool reference. `set-snapshot`'s replacement
/// workbook carries an empty pool, so it moves the count to exactly 0.
fn shared_string_count_after(current: usize, kind: &str) -> usize {
    match kind {
        "set-snapshot" => 0,
        _ => current,
    }
}

/// 📑️ The three kinds that address the raw `xl/sharedStrings.xml` pool by INDEX rather than the
/// sheet grid. They are projected through [`project_shared_string_pool`], which reads the real pool
/// out of the package with `zip` + `quick-xml`, instead of through the `calamine` grid projection
/// and its caller-tracked count — the pool is a storage-layer part, and a part has a real
/// independent reader. Every other declared kind runs through the `calamine` + `rust_xlsxwriter`
/// grid pairing, which legitimately renormalises the pool while preserving every cell value, so
/// holding those to a pool they never claimed to preserve would report a false divergence.
fn is_pool_kind(kind: &str) -> bool {
    matches!(kind, "insert-shared-string" | "remove-shared-string" | "set-shared-string")
}
//#endregion 🔖️SpecHelpers

//#region 🔖️Law
/// 🔬️ First structural divergence between two projections — a dotted field path plus both values,
/// so a law that fails names WHICH cell moved instead of only "not equal". Kept local to this
/// adapter for the same reason `KINDS` is duplicated here: a case adapter is a leaf that links the
/// test host and this subset's own oracle module, nothing else.
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

/// ✂️ A projection value, truncated: a divergence message must stay readable, and this projection
/// carries a real 50-row survey table.
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
/// is asserted in-role: a mutation that leaves the projection it claims to move untouched proves
/// nothing, whatever the reference implementation returned.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = arranged_input(ctx, &spec)?;
    let kind = spec.str("kind");
    let output = oracle_apply_mutation(&input, &spec)?;
    let project = |bytes: &[u8]| -> Result<Json, String> {
        if is_pool_kind(&kind) {
            project_shared_string_pool(bytes)
        } else {
            project_xlsx_workbook(bytes, shared_string_count_after(BASELINE_SHARED_STRING_COUNT, &kind))
        }
    };
    let projection = project(&output)?;
    if projection == project(&input)? {
        return Err(format!("mutate-{kind}: the mutation left the projection unchanged — a mutation that is not observable proves nothing"));
    }
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ Applies the mutation, undoes it with the reference's own independently computed inverse
/// (`oracle_apply_inverse`, sourcing what the forward kind discarded from the real input), and ASSERTS
/// the law in role: the restored workbook must project onto exactly what the real input projects
/// onto — every sheet, every cell, every value, and the restored pool's count, for every kind.
/// `set-snapshot` is undone by rebuilding the original's own grid through the reference pairing.
/// The three POOL kinds are projected through `project_shared_string_pool`, so their restored
/// result is observed rather than tracked, entry by entry.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = arranged_input(ctx, &spec)?;
    let kind = spec.str("kind");
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_inverse(&input, &mutated, &spec)?;
    let (projection, original) = if is_pool_kind(&kind) {
        (project_shared_string_pool(&restored)?, project_shared_string_pool(&input)?)
    } else {
        (project_xlsx_workbook(&restored, BASELINE_SHARED_STRING_COUNT)?, project_xlsx_workbook(&input, BASELINE_SHARED_STRING_COUNT)?)
    };
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("inverse-{kind}: the mutation followed by its own computed inverse did not restore the original workbook — {divergence}"));
    }
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔁️ The oracle's own decode/re-encode, through the SAME independent `calamine` + `rust_xlsxwriter`
/// pairing this subset's mutations use — and ASSERTED, not merely produced: the rebuilt package's
/// bytes must differ from the input (nothing was copied — `rust_xlsxwriter` assembles a brand-new
/// package and cannot reproduce another writer's object layout) and its projection must be
/// identical to the input's (nothing was lost). `sharedStringCount` is the same adapter-tracked
/// baseline on both sides, so it carries no evidence here; the sheet grid does.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    if output == input {
        return Err("byte pass-through: the rebuilt package is bit-identical to the input".to_string());
    }
    let projection = project_xlsx_workbook(&output, BASELINE_SHARED_STRING_COUNT)?;
    let original = project_xlsx_workbook(&input, BASELINE_SHARED_STRING_COUNT)?;
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("identity round trip: reading and rebuilding the real workbook did not preserve its semantic projection — {divergence}"));
    }
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, is_pool_kind, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::export::serializers::encode_xlsx;
    use semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_xlsx;
    use semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::schema::mutations::apply_xlsx_mutation;
    use semio_s_artifact_stdio_xlsx::{mutation_from_payload_json, mutation_inverse, mutation_payload_json, XlsxMutation, XlsxSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::xlsx::standards::v_ecma_376::subsets::base::{project_shared_string_pool, project_xlsx_workbook};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;

    /// 📑️ The SAME projector choice the oracle half makes for the same scenario id: the three pool
    /// kinds read `xl/sharedStrings.xml` back with `zip` + `quick-xml`, every other kind reads the grid
    /// back with `calamine`, carrying the subject's own real pool size.
    fn outcome_of(kind: &str, snapshot: &XlsxSnapshot) -> Result<Outcome, String> {
        let output = encode_xlsx(snapshot).map_err(|error| error.to_string())?;
        let projection = if is_pool_kind(kind) {
            project_shared_string_pool(&output)?
        } else {
            project_xlsx_workbook(&output, snapshot.project_workbook().map_err(|error| error.to_string())?.shared_strings.len())?
        };
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🦠️ The scenario's `{kind, params}` witness decoded generically: `params` IS the leaf's wire
    /// payload, so the derive-generated `from_payload_value` is the only decoder, and re-emitting the
    /// decoded payload must give back exactly `params`.
    fn mutation_from_spec(spec: &Json) -> Result<XlsxMutation, String> {
        let kind = spec.str("kind");
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        wire_operation(&kind, &params, mutation_from_payload_json, mutation_payload_json)
    }

    fn decode(bytes: &[u8]) -> Result<XlsxSnapshot, String> {
        decode_xlsx(bytes).map_err(|error| error.to_string())
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        apply_xlsx_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        outcome_of(&spec.str("kind"), &snapshot)
    }

    /// ↩️ The forward witness is undone by `XlsxMutation::inverse` itself — the vocabulary's own
    /// algebra is the law under test, never a transcription of it.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = decode(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(&spec)?;
        let undo = mutation_inverse(&mutation, &base);
        let mut snapshot = base;
        apply_xlsx_mutation(&mut snapshot, &mutation);
        for step in &undo {
            apply_xlsx_mutation(&mut snapshot, step);
        }
        outcome_of(&spec.str("kind"), &snapshot)
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating, and this tripwire catches it.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode_xlsx(&snapshot).map_err(|error| error.to_string())?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_xlsx_workbook(&output, snapshot.project_workbook().map_err(|error| error.to_string())?.shared_strings.len())?;
        Ok(Outcome::with_raw(output, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
