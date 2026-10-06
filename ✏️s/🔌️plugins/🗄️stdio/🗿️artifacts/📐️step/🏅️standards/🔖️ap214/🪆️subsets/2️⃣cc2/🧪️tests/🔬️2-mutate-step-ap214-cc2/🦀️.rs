//! 🦀️ STEP AP214 `2️⃣cc2` mutation case — Rust adapter. ISO 10303-214 CC2 (bounded wireframe/basic surfaces).
//!
//! 🎯️ This case reads the same real committed export as `📐️mutate-step-ap214`, and asks a different
//! question of it. That case exercises the ISO 10303-21 GRAMMAR — ten verbs that would read
//! identically for any Part-21 file on earth. This one exercises a CONFORMANCE CLASS: the 5 kinds
//! it registers are one per axis `check_cc2_conformance` reads, and the projection it compares by
//! reports the declared schema, the `*_SHAPE_REPRESENTATION` ladder census and the product identity
//! chain — nothing else, because a projection carrying all 1,396 entities would drown every
//! class-level difference it exists to see.
//!
//! 🔬️ `ruststep` 0.4 is the independent READER (it has no writer at all), so nothing here is typed
//! `@mode-differential`. The re-serializer is this standard's own from-scratch Part-21 writer, and
//! the §4.3 ladder classification the oracle applies is re-derived from the standard rather than
//! called out of the production `engine::ladder` — both live once, at the standard level
//! (`../../🏅️standards/🔖️ap214/🦀️oracle.rs`), shared by all seven `ap214` subsets.
//!
//! ⚖️ Both law-bearing scenario families assert IN ROLE, without needing a subject: `inverse-<kind>`
//! applies the mutation and then the independently computed inverse and requires the original
//! projection back; `identity-round-trip` requires the projection preserved AND the bytes to differ,
//! since ISO 10303-21 clear text is regenerated from the parsed model rather than copied. Every
//! `mutate-<kind>` row must MOVE the projection, and this class's own claim
//! about what it moved is asserted on top of that.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::cc2::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_step_ap214_cc2};
use semio_repo_test_host::law::{inverse_restores, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Kinds
/// 🏷️ How this class names itself in a failure message.
const CLASS: &str = "ISO 10303-214 CC2 (bounded wireframe/basic surfaces)";


/// 🪜️ This class's own ceiling type — the one `set-shape-representation` writes and the one a
/// demotion lands on. Named here because the per-row claim below asserts it by name.
const CEILING: &str = "GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION";
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🌲️hexagonal-cut-concrete-forest-left-ap214/📐️.stp";

/// 🧫️ Copies the immutable committed export into the work directory and returns the mutable copy's
/// bytes; the committed fixture is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_input(INPUT, Some("input.stp"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Claim
/// 🎯️ This class's own claim, asserted per row on top of observability:
///
/// * `demote-shape-representation` must bring the real rung-6 `#13` INSIDE the class, so
///   `aboveCeiling` reaches 0 and `conformsToClass` becomes true. That is the whole conformance
///   repair this file needs, and a demotion that left the count where it was would be a repair that
///   repaired nothing.
/// * `set-shape-representation` must leave `#836` carrying this class's own ceiling type, `GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION`
///   — the row writes exactly that, and reading it back through the independent parser is what
///   proves the write reached the wire.
fn class_claim(kind: &str, _before: &Json, after: &Json) -> Result<(), String> {
    match kind {
        "demote-shape-representation" => {
            if after.get("aboveCeiling") != Some(&Json::Number(0.0)) || after.get("conformsToClass") != Some(&Json::Bool(true)) {
                return Err(format!("demoting #13 was supposed to bring this document inside {CLASS}; it reads aboveCeiling={:?} conformsToClass={:?}", after.get("aboveCeiling"), after.get("conformsToClass")));
            }
            Ok(())
        }
        "set-shape-representation" => {
            let at_ceiling = after
                .array("representations")
                .iter()
                .any(|entry| entry.get("id") == Some(&Json::Number(836.0)) && entry.get("typeName") == Some(&Json::String(CEILING.to_string())));
            if at_ceiling {
                Ok(())
            } else {
                Err(format!("set-shape-representation was supposed to leave #836 carrying {CEILING}; the census does not show it there"))
            }
        }
        _ => Ok(()),
    }
}
//#endregion 🔖️Claim

//#region 🔖️Oracle
/// 🔮️ Every `mutate-<kind>` scenario id, asserted IN ROLE: the reference performs the kind, the
/// result is read back through the independent parser, and the conformance projection must have
/// MOVED. A row whose parameters make the mutation a no-op is not
/// a test, so it fails here rather than passing silently.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let before = project_step_ap214_cc2(&oracle_round_trip(&input)?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_step_ap214_cc2(&bytes)?;
    if projection == before {
        return Err(format!("{kind:?} left {CLASS}'s conformance projection unchanged -- a mutation that is not observable proves nothing"));
    }
    class_claim(&kind, &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Every `inverse-<kind>` scenario id, and the ORACLE side of the inverse law: the forward
/// mutation is applied, the inverse is computed independently against the UNTOUCHED original, and
/// the restored document must project exactly as the original does. The baseline runs one identity
/// rewrite so both sides carry the same writer normalisation and the comparison isolates the mutation
/// pair itself.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_step_ap214_cc2(&oracle_round_trip(&input)?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&input, &spec)?)?;
    let projection = project_step_ap214_cc2(&restored)?;
    inverse_restores(&kind, &projection, &baseline)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The identity round trip, asserted in role: the reference fully parses the real exchange
/// structure and re-serializes it from its own writer alone, so the conformance projection MUST
/// survive and the bytes MUST NOT be bit-identical. ISO 10303-21 clear text is not a byte-preserving
/// carrier — the whole structure is regenerated — so the tripwire is real evidence of a parse.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_step_ap214_cc2(&input)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_step_ap214_cc2(&bytes)?;
    round_trip_preserves(&projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{class_claim, mutable_input, CLASS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_step::part21::{parse_part21, write_part21};
    use semio_s_artifact_stdio_step::standards::v_ap214::subsets::cc2::schema::mutations::StepCc2Mutation;
    use semio_s_artifact_stdio_step::StepSnapshot;
    use semio_s_artifact_stdio_step::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_step_test_oracle::standards::v_ap214::subsets::cc2::project_step_ap214_cc2;
    use semio_repo_test_host::law::{inverse_restores, reparsed_not_copied, round_trip_preserves};

    /// 🦠️ The row's `params` IS the leaf wire payload, decoded by the derive-generated constructor.
    fn operation_of(spec: &Json) -> Result<StepCc2Mutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    /// 📥️ Genuine ISO 10303-21 text decoded into the typed `StepSnapshot` through the shared Part-21
    /// tokenizer — never a splice of the input's own bytes.
    fn decoded(bytes: &[u8]) -> Result<StepSnapshot, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("input is not UTF-8: {error}"))?;
        Ok(StepSnapshot::from_part21_document(&parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?))
    }

    /// 📤️ Re-serialization from the model alone.
    fn encoded(snapshot: &StepSnapshot) -> Vec<u8> {
        write_part21(&snapshot.to_part21_document()).into_bytes()
    }

    /// ▶️ Applies `operations` in order through the production diff; this class's own refusal surfaces
    /// instead of an untouched model.
    fn applied(mut snapshot: StepSnapshot, operations: &[StepCc2Mutation]) -> Result<StepSnapshot, String> {
        for operation in operations {
            apply_mutation_checked(&mut snapshot, operation)?;
        }
        Ok(snapshot)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let base = decoded(&input)?;
        let before = project_step_ap214_cc2(&encoded(&base))?;
        let bytes = encoded(&applied(base, &[operation_of(&spec)?])?);
        let projection = project_step_ap214_cc2(&bytes)?;
        if projection == before {
            return Err(format!("{kind:?} left {CLASS}'s conformance projection unchanged -- a mutation that is not observable proves nothing"));
        }
        class_claim(&kind, &before, &projection)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The mutation's OWN inverse (`Mutation::inverse` against the untouched model), applied to the
    /// re-decoded result of the forward cycle.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let operation = operation_of(&spec)?;
        let base = decoded(&input)?;
        let baseline = project_step_ap214_cc2(&encoded(&base))?;
        let mutated = encoded(&applied(base.clone(), std::slice::from_ref(&operation))?);
        let restored = encoded(&applied(decoded(&mutated)?, &mutation_inverse(&operation, &base).expect("valid retained mutation inverse fixture"))?);
        let projection = project_step_ap214_cc2(&restored)?;
        inverse_restores(&spec.str("kind"), &projection, &baseline)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let before = project_step_ap214_cc2(&input)?;
        let bytes = encoded(&decoded(&input)?);
        reparsed_not_copied(&bytes, &input)?;
        let projection = project_step_ap214_cc2(&bytes)?;
        round_trip_preserves(&projection, &before)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Handlers are registered under the Scenario Outline
/// base ids, which the host resolves for every Examples row, and plain scenarios under their own ids.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
