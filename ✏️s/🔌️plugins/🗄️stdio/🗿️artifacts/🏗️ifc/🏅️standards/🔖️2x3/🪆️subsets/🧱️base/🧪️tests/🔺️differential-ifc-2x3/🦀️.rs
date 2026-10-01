//! 🦀️ IFC2X3/🧱️base differential case — Rust adapter, SUBJECT half only.
//!
//! The oracle half of this case is `🐍️component.py`: IfcOpenShell 0.8.4.post1 applies each mutation
//! through its own API and re-serializes the whole exchange structure with its own C++ Part-21
//! writer. That is the second PRODUCER `ruststep` cannot be, which is why every scenario here is
//! `@mode-differential` while the sibling `../🧱️mutate-ifc-2x3` — same vocabulary, same fixture, all
//! four kinds, `ruststep` as an independent READER — stays `@mode-property`.
//!
//! This file therefore registers NOTHING in the oracle role. The subject does exactly what the
//! sibling case's subject does: `decode_ifc2x3` into this subset's own `Ifc2x3Snapshot`, the row's
//! `params` — the leaf wire payload — decoded by the derive-generated `from_payload_value` and applied,
//! the mutation's own `Mutation::inverse` for the inverse rows, `encode_ifc2x3` from the snapshot alone —
//! no byte pass-through — followed by an independent `ruststep` read-back (`project_ifc_2x3_any`) before
//! `semantic-ifc-v1` compares it with what IfcOpenShell produced from the same input.
//!
//! Three of the four kinds appear here. `remove-instance` does not: `ifcopenshell.file.remove`
//! repairs the references that point at the removed instance, `Ifc2x3Mutation::RemoveInstance` is a
//! bare `retain` that deliberately leaves them dangling, and `#270549` — the instance the sibling
//! case removes precisely because it is referenced — has 8 inverse references in this fixture.
//! Comparing two different verbs is not a differential; it keeps its `ruststep`-backed scenarios
//! next door.
//!
//! @see component.feature — the differential claim and the measurement that bounds it.
//! @see ../🧱️mutate-ifc-2x3/🦀️.rs — the exhaustive four-kind case this one does not replace.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::io::{decode_ifc2x3, encode_ifc2x3};
    use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation;
    use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot;
    use semio_s_artifact_stdio_ifc::{apply_mutation_checked, mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_plugin_stdio_test_oracle::artifacts::ifc::standards::v2x3::subsets::base::project_ifc_2x3_any;

    const INPUT: &str = "shared://🏥️wellness-center-sama-street-level/🏥️wellness-center-sama-street-level.ifc";

    /// 🧫️ Copies the immutable committed fixture into the work directory and returns the mutable
    /// copy's bytes; the committed fixture itself is never written to.
    fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
        let copy = ctx.copy_fixture(INPUT, Some("input.ifc"))?;
        std::fs::read(&copy).map_err(|error| error.to_string())
    }

    /// 🦠️ The row's `params` IS the leaf wire payload, decoded by the derive-generated constructor.
    fn operation_of(spec: &Json) -> Result<Ifc2x3Mutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }

    /// ▶️ Applies `operations` in order through the production diff, refusing the first rejection.
    fn applied(mut snapshot: Ifc2x3Snapshot, operations: &[Ifc2x3Mutation]) -> Result<Ifc2x3Snapshot, String> {
        for operation in operations {
            apply_mutation_checked(&mut snapshot, operation)?;
        }
        Ok(snapshot)
    }

    /// 📐️ Re-serializes from the model alone and refuses a byte pass-through of the committed input —
    /// the one cycle where identical bytes would mean the document was copied instead of decoded. A
    /// second cycle over this repository's own output is exempt: its writer is idempotent, as a correct
    /// writer is.
    fn encoded(input: &[u8], snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
        let bytes = encode_ifc2x3(snapshot)?;
        if bytes == input {
            return Err("byte pass-through: the re-encoded output is bit-identical to the committed input, so nothing here proves the document was parsed".to_string());
        }
        Ok(bytes)
    }

    fn outcome(bytes: Vec<u8>) -> Result<Outcome, String> {
        let projection = project_ifc_2x3_any(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        outcome(encoded(&input, &applied(decode_ifc2x3(&input)?, &[operation])?)?)
    }

    /// ↩️ The mutation's OWN inverse (`Mutation::inverse` against the untouched model), applied to the
    /// re-decoded result of the forward cycle.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let operation = operation_of(&ctx.doc_json()?)?;
        let base = decode_ifc2x3(&input)?;
        let mutated = encoded(&input, &applied(base.clone(), std::slice::from_ref(&operation))?)?;
        outcome(encode_ifc2x3(&applied(decode_ifc2x3(&mutated)?, &mutation_inverse(&operation, &base))?)?)
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        outcome(encoded(&input, &decode_ifc2x3(&input)?)?)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. The oracle role belongs to
/// `🐍️.py`; without the `sut` feature this adapter registers nothing at all, which is
/// exactly right — it has no reference implementation of its own to offer.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("differential", subject::mutate);
        built = built.subject("differential-inverse", subject::inverse);
        built = built.subject("differential-identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
