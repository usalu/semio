//! 🦀️ STEP AP214 `✳️cc4` mutation case — Rust adapter. ISO 10303-214 CC4 (manifold surfaces with topology).
//!
//! 🎯️ This case reads the same real committed export as `mutate-step-ap214`, and asks a different
//! question of it. That case exercises the ISO 10303-21 GRAMMAR — eleven verbs that would read
//! identically for any Part-21 file on earth. This one exercises a CONFORMANCE CLASS: the 6 kinds
//! it registers are one per axis `check_cc4_conformance` reads, and the projection it compares by
//! reports the declared schema, the `*_SHAPE_REPRESENTATION` ladder census and the product identity
//! chain — nothing else, because a projection carrying all 1,396 entities would drown every
//! class-level difference it exists to see.
//!
//! 🔬️ `ruststep` 0.4 is the independent READER (it has no writer at all), so nothing here is typed
//! `@mode-differential`. The re-serializer is this standard's own from-scratch Part-21 writer, and
//! the §4.3 ladder classification the oracle applies is re-derived from the standard rather than
//! called out of the production `engine::ladder` — both live once, at the standard level
//! (`../../🏅️standards/🔖️ap214/🧪️oracle/🦀️component.rs`), shared by all seven `ap214` subsets.
//!
//! ⚖️ Both law-bearing scenario families assert IN ROLE, without needing a subject: `inverse-<kind>`
//! applies the mutation and then the independently computed inverse and requires the original
//! projection back; `identity-round-trip` requires the projection preserved AND the bytes to differ,
//! since ISO 10303-21 clear text is regenerated from the parsed model rather than copied. Every
//! `mutate-<kind>` row other than `no-mutation` must MOVE the projection, and this class's own claim
//! about what it moved is asserted on top of that.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc4::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip, project_step_ap214_cc4};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Kinds
/// 🏷️ How this class names itself in a failure message.
const CLASS: &str = "ISO 10303-214 CC4 (manifold surfaces with topology)";

/// 🧾️ Case-local mirror of the `step-ap214-cc4` catalog. Duplicated rather than imported: `KINDS`
/// lives in the SUBJECT crate, which the oracle role must never link. The contract phase fails with
/// `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list drifts from the catalog.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-file-schema", "set-product-identity", "set-shape-representation", "demote-shape-representation"];

/// 🪜️ This class's own ceiling type — the one `set-shape-representation` writes and the one a
/// demotion lands on. Named here because the per-row claim below asserts it by name.
const CEILING: &str = "MANIFOLD_SURFACE_SHAPE_REPRESENTATION";
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://📐️hexagonal-cut-concrete-forest-left-ap214.stp";

/// 🧫️ Copies the immutable committed export into the work directory and returns the mutable copy's
/// bytes; the committed fixture is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.stp"))?;
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
/// * `set-shape-representation` must leave `#836` carrying this class's own ceiling type, `MANIFOLD_SURFACE_SHAPE_REPRESENTATION`
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
fn no_mutation() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(Vec::new()))])
}

/// 🔮️ Every `mutate-<kind>` scenario id, asserted IN ROLE: the reference performs the kind, the
/// result is read back through the independent parser, and — for anything but `no-mutation` — the
/// conformance projection must have MOVED. A row whose parameters make the mutation a no-op is not
/// a test, so it fails here rather than passing silently.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let before = project_step_ap214_cc4(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_step_ap214_cc4(&bytes)?;
    if kind != "no-mutation" && projection == before {
        return Err(format!("{kind:?} left {CLASS}'s conformance projection unchanged -- a mutation that is not observable proves nothing"));
    }
    class_claim(&kind, &before, &projection)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Every `inverse-<kind>` scenario id, and the ORACLE side of the inverse law: the forward
/// mutation is applied, the inverse is computed independently against the UNTOUCHED original, and
/// the restored document must project exactly as the original does. The baseline runs one
/// `no-mutation` cycle so both sides carry the same writer normalisation and the comparison isolates
/// the mutation pair itself. `no-mutation` is not short-circuited: the trivial case is evidence.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let baseline = project_step_ap214_cc4(&oracle_apply_mutation(&input, &no_mutation())?)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &oracle_inverse_spec(&input, &spec)?)?;
    let projection = project_step_ap214_cc4(&restored)?;
    inverse_restores(&kind, &projection, &baseline)?;
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The identity round trip, asserted in role: the reference fully parses the real exchange
/// structure and re-serializes it from its own writer alone, so the conformance projection MUST
/// survive and the bytes MUST NOT be bit-identical. ISO 10303-21 clear text is not a byte-preserving
/// carrier — the whole structure is regenerated — so the tripwire is real evidence of a parse.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let before = project_step_ap214_cc4(&input)?;
    let bytes = oracle_round_trip(&input)?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_step_ap214_cc4(&bytes)?;
    round_trip_preserves(&projection, &before)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{class_claim, mutable_input, no_mutation, CLASS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::step::engine::ladder::{ProductIdentity, ShapeRepresentationRow};
    use semio_s_plugin_stdio::artifacts::step::engine::part21::{parse_part21, write_part21};
    use semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::{apply_step_cc4_mutation_checked, inverse_step_cc4_mutation, StepCc4Mutation};
    use semio_s_plugin_stdio::artifacts::step::StepSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::step::standards::v_ap214::subsets::cc4::project_step_ap214_cc4;
    use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, reparsed_not_copied, round_trip_preserves};

    fn params_of(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()))
    }

    fn u64_field(value: &Json, key: &str) -> Result<u64, String> {
        match value.get(key) {
            Some(Json::Number(number)) => Ok(*number as u64),
            _ => Err(format!("expected numeric field {key:?}")),
        }
    }

    fn text_field(value: &Json, key: &str) -> String {
        match value.get(key) {
            Some(Json::String(text)) => text.clone(),
            _ => String::new(),
        }
    }

    fn str_array(value: &Json, key: &str) -> Vec<String> {
        value
            .array(key)
            .iter()
            .filter_map(|entry| match entry {
                Json::String(text) => Some(text.clone()),
                _ => None,
            })
            .collect()
    }

    fn identity_from(value: &Json) -> Result<ProductIdentity, String> {
        Ok(ProductIdentity {
            product: u64_field(value, "product")?,
            product_name: text_field(value, "productName"),
            formation: u64_field(value, "formation")?,
            formation_id: text_field(value, "formationId"),
            definition: u64_field(value, "definition")?,
            definition_id: text_field(value, "definitionId"),
        })
    }

    fn representation_from(value: &Json) -> Result<ShapeRepresentationRow, String> {
        Ok(ShapeRepresentationRow {
            type_name: match value.get("typeName") {
                Some(Json::String(text)) => text.clone(),
                _ => return Err("a representation requires a typeName".to_string()),
            },
            name: text_field(value, "name"),
            items: value
                .array("items")
                .iter()
                .filter_map(|entry| match entry {
                    Json::Number(number) => Some(*number as u64),
                    _ => None,
                })
                .collect(),
            context: match value.get("context") {
                Some(Json::Number(number)) => Some(*number as u64),
                _ => None,
            },
        })
    }

    /// 🦠️ The `(kind, params)` wire shape read into a real `StepCc4Mutation`. `set-snapshot` builds
    /// the same minimal conformant document the reference builds — schema plus, optionally, a product
    /// identity chain and nothing on the ladder — so both sides mean the same verb by it.
    fn mutation_from_spec(spec: &Json, base: &StepSnapshot) -> Result<StepCc4Mutation, String> {
        let params = params_of(spec);
        Ok(match spec.str("kind").as_str() {
            "set-snapshot" => {
                let mut snapshot = StepSnapshot::default();
                let mut document = snapshot.to_part21_document();
                let schemas = str_array(&params, "fileSchema");
                if schemas.is_empty() {
                    return Err("set-snapshot requires a non-empty fileSchema field".to_string());
                }
                semio_s_plugin_stdio::artifacts::step::engine::ladder::set_file_schema_names(&mut document, &schemas);
                if let Some(identity) = params.get("productIdentity").filter(|value| !matches!(value, Json::Null)) {
                    semio_s_plugin_stdio::artifacts::step::engine::ladder::set_product_identity(&mut document, Some(&identity_from(identity)?));
                }
                snapshot = StepSnapshot::from_part21_document(document);
                let _ = base;
                StepCc4Mutation::SetSnapshot(semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::set_snapshot::SetSnapshot { snapshot })
            }
            "set-file-schema" => {
                let schemas = str_array(&params, "schemas");
                if schemas.is_empty() {
                    return Err(format!("{CLASS} requires FILE_SCHEMA to declare a schema"));
                }
                StepCc4Mutation::SetFileSchema(semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::set_file_schema::SetFileSchema { schemas })
            }
            "set-product-identity" => StepCc4Mutation::SetProductIdentity(semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::set_product_identity::SetProductIdentity {
                identity: match params.get("identity").filter(|value| !matches!(value, Json::Null)) {
                    Some(value) => Some(identity_from(value)?),
                    None => None,
                },
            }),
            "set-shape-representation" => StepCc4Mutation::SetShapeRepresentation(semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::set_shape_representation::SetShapeRepresentation {
                id: u64_field(&params, "id")?,
                representation: match params.get("representation").filter(|value| !matches!(value, Json::Null)) {
                    Some(value) => Some(representation_from(value)?),
                    None => None,
                },
            }),
            "demote-shape-representation" => StepCc4Mutation::DemoteShapeRepresentation(semio_s_plugin_stdio::artifacts::step::standards::v_ap214::subsets::cc4::schema::mutations::demote_shape_representation::DemoteShapeRepresentation { id: u64_field(&params, "id")? }),
            other => return Err(format!("unrecognised mutation kind {other:?}")),
        })
    }

    /// 📐️ Full parse into the typed `StepSnapshot` and re-serialization from the model alone — never
    /// a splice of the input's own bytes. `parse_part21`/`write_part21` are the shared low-level
    /// Part-21 codec this subset's snapshot already converts through, used directly so the output is
    /// genuine ISO 10303-21 text rather than a semio pack/DSL envelope.
    fn apply_and_encode(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
        let text = std::str::from_utf8(input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let document = parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?;
        let mut snapshot = StepSnapshot::from_part21_document(document);
        let mutation = mutation_from_spec(spec, &snapshot.clone())?;
        apply_step_cc4_mutation_checked(&mut snapshot, &mutation)?;
        Ok(write_part21(&snapshot.to_part21_document()).into_bytes())
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let before = project_step_ap214_cc4(&apply_and_encode(&input, &no_mutation())?)?;
        let bytes = apply_and_encode(&input, &spec)?;
        let projection = project_step_ap214_cc4(&bytes)?;
        if kind != "no-mutation" && projection == before {
            return Err(format!("{kind:?} left {CLASS}'s conformance projection unchanged -- a mutation that is not observable proves nothing"));
        }
        class_claim(&kind, &before, &projection)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let baseline = project_step_ap214_cc4(&apply_and_encode(&input, &no_mutation())?)?;
        let text = std::str::from_utf8(&input).map_err(|error| format!("input is not UTF-8: {error}"))?;
        let base = StepSnapshot::from_part21_document(parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?);
        let mutated = apply_and_encode(&input, &spec)?;
        let mutated_text = std::str::from_utf8(&mutated).map_err(|error| format!("output is not UTF-8: {error}"))?;
        let mut snapshot = StepSnapshot::from_part21_document(parse_part21(mutated_text).map_err(|error| format!("parse_part21 failed: {error}"))?);
        for step in inverse_step_cc4_mutation(&base, &mutation_from_spec(&spec, &base)?) {
            apply_step_cc4_mutation_checked(&mut snapshot, &step)?;
        }
        let restored = write_part21(&snapshot.to_part21_document()).into_bytes();
        let projection = project_step_ap214_cc4(&restored)?;
        inverse_restores(&kind, &projection, &baseline)?;
        Ok(Outcome::with_raw(restored, projection))
    }

    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let before = project_step_ap214_cc4(&input)?;
        let bytes = apply_and_encode(&input, &no_mutation())?;
        reparsed_not_copied(&bytes, &input)?;
        let projection = project_step_ap214_cc4(&bytes)?;
        round_trip_preserves(&projection, &before)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the outline's rows are enumerated here rather than its base id.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
