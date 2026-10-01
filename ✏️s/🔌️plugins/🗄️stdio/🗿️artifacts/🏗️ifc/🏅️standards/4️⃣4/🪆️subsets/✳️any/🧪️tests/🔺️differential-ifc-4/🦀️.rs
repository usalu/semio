//! 🦀️ IFC4/✳️any differential case — Rust adapter, SUBJECT half only.
//!
//! The oracle half of this case is `🐍️.py`: IfcOpenShell 0.8.4.post1 applies each mutation through its
//! own API and re-serializes the whole exchange structure with its own C++ Part-21 writer. That is the
//! second PRODUCER `ruststep` cannot be, which is why every scenario here is `@mode-differential` while
//! the sibling `../🏗️mutate-ifc-4` — same vocabulary, same fixture, all ten kinds, `ruststep` as an
//! independent READER — stays `@mode-property`.
//!
//! This file therefore registers NOTHING in the oracle role. The subject does exactly what the sibling
//! case's subject does: a full parse into this subset's own `IfcSnapshot`, the row's `params` — the leaf
//! wire payload — decoded by the derive-generated `from_payload_value` and applied, the mutation's own
//! `Mutation::inverse` for the inverse rows, and a re-serialization from the snapshot alone — no byte
//! pass-through — followed by an independent `ruststep` read-back (`project_ifc_4_any`) before
//! `semantic-ifc-v1` compares it with what IfcOpenShell produced from the same input.
//!
//! Six of the ten kinds appear here. The four that do not (`set-entity-name`, `insert-entity-arg`,
//! `remove-entity-arg`, `remove-entity`) are the ones IfcOpenShell cannot produce or cannot read back
//! faithfully, each measured against this exact fixture and recorded in the feature file; they keep
//! their `ruststep`-backed scenarios next door.
//!
//! @see 🥒️.feature — the differential claim and the four measurements that bound it.
//! @see ../🏗️mutate-ifc-4/🦀️.rs — the exhaustive ten-kind case this one does not replace.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_ifc::part21::{parse_part21, write_part21};
    use semio_s_artifact_stdio_ifc::standards::v4::subsets::any::schema::mutations::IfcMutation;
    use semio_s_artifact_stdio_ifc::standards::v4::subsets::any::schema::snapshot::{from_part21_document, to_part21_document, IfcSnapshot};
    use semio_s_artifact_stdio_ifc::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json, STDIO_IFC_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v4::subsets::any::project_ifc_4_any;

    const INPUT: &str = "shared://🏢️nakagin-capsule-tower/🏢️nakagin-capsule-tower.ifc";

    /// 🧫️ Copies the immutable committed fixture into the work directory and returns the mutable
    /// copy's bytes; the committed fixture itself is never written to.
    fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
        let copy = ctx.copy_fixture(INPUT, Some("input.ifc"))?;
        std::fs::read(&copy).map_err(|error| error.to_string())
    }

    /// 🦠️ The row's `params` IS the leaf wire payload, decoded by the derive-generated constructor.
    fn operation_of(spec: &Json) -> Result<IfcMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    /// 📥️ Genuine ISO 10303-21 text decoded into the subset's own snapshot through the shared Part-21
    /// tokenizer — no semio pack/DSL envelope, the shape `ruststep` and the committed fixture expect.
    fn decoded(bytes: &[u8]) -> Result<IfcSnapshot, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("input is not UTF-8: {error}"))?;
        Ok(from_part21_document(STDIO_IFC_DOCUMENT_SCHEMA, &parse_part21(text).map_err(|error| format!("parse_part21 failed: {error}"))?))
    }

    /// ▶️ Applies `operations` in order through the production diff, refusing the first rejection.
    fn applied(mut snapshot: IfcSnapshot, operations: &[IfcMutation]) -> Result<IfcSnapshot, String> {
        for operation in operations {
            apply_mutation_checked(&mut snapshot, operation)?;
        }
        Ok(snapshot)
    }

    /// 📐️ Re-serializes from the model alone and refuses a byte pass-through of the committed input.
    fn encoded(input: &[u8], snapshot: &IfcSnapshot) -> Result<Vec<u8>, String> {
        let bytes = write_part21(&to_part21_document(snapshot)).into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        Ok(bytes)
    }

    fn outcome(bytes: Vec<u8>) -> Result<Outcome, String> {
        let projection = project_ifc_4_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        outcome(encoded(&input, &applied(decoded(&input)?, &[operation])?)?)
    }

    /// ↩️ The mutation's OWN inverse (`Mutation::inverse` against the untouched model), applied to the
    /// re-decoded result of the forward cycle.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        let base = decoded(&input)?;
        let mutated = encoded(&input, &applied(base.clone(), std::slice::from_ref(&operation))?)?;
        outcome(write_part21(&to_part21_document(&applied(decoded(&mutated)?, &mutation_inverse(&operation, &base))?)).into_bytes())
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        outcome(encoded(&input, &decoded(&input)?)?)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. The oracle role belongs to `🐍️.py`; without
/// the `sut` feature this adapter registers nothing at all, which is exactly right — it has no reference
/// implementation of its own to offer.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("differential", subject::mutate).subject("differential-inverse", subject::inverse);
        built = built.subject("differential-identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
