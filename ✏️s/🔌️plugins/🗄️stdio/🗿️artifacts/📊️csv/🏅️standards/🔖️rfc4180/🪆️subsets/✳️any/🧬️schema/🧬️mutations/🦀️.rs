//! 🧬️ CsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `CsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::schema::diff::{diff_set_snapshot, CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified, CsvRecordsDiff};






use crate::schema::snapshot::{CsvField, CsvRecord};
use crate::CsvSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

//#region 🔖️Mutations
#[path = "📥insert-record/🦀️.rs"]
pub mod insert_record;
#[path = "📤remove-record/🦀️.rs"]
pub mod remove_record;
#[path = "✏️set-field/🦀️.rs"]
pub mod set_field;
#[path = "🧾set-has-header/🦀️.rs"]
pub mod set_has_header;
/// 📐️ Typed content mutation for `stdio.csv`.
/// 🧪️ F6: `#[derive(dsl::DslOps)]` on this enum CANNOT be used — confirmed via a real `cargo
/// check` error, and NOT one of the recon report's documented §3a/§3b failure modes: it is a
/// genuine derive-macro hygiene bug. `InsertRecord`'s field is literally named `record`, and
/// `dsl_derive::dsl_variants_codegen`'s generated `to_named_arms` match-arm body shadows any
/// field bound by that same name with its own internal accumulator —
/// `let mut record = ::dsl::RecordValue::default();` — declared AFTER the match pattern destructures
/// the variant's fields. The subsequent `record.fields.insert(#id, ::dsl::DslField::to_value(record))`
/// statement for the `record` field then resolves `record` to the SHADOWING `RecordValue`, not the
/// `&CsvRecord` binding, giving: `error[E0308]: mismatched types … expected reference `&_`, found
/// struct `RecordValue`` at this variant's `record: CsvRecord` field (verified: renaming the field
/// to `csvrec` alone made the same derive attempt compile clean). Renaming the field back would fix
/// the derive but changes the Mutation enum's wire shape, which is out of scope here — `OpText`/
/// `OpBinary` hand-rolled below instead, reusing `CsvDiff`'s `pub(crate)` grammar primitives.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.csv`. `NoMutation` was dropped: the derive requires every
/// variant to wrap exactly one leaf payload, and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = CsvSnapshot, diff = CsvDiff, schema = "s.stdio.csv")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum CsvMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    SetHasHeader(set_has_header::SetHasHeader),
    InsertRecord(insert_record::InsertRecord),
    RemoveRecord(remove_record::RemoveRecord),
    SetField(set_field::SetField),
}

/// 🧾️ Kebab-case spelling of every `CsvMutation` variant, in declaration order — the exhaustive
/// mutation catalog `csv-rfc4180-any` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "set-has-header", "insert-record", "remove-record", "set-field"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_csv_mutation(snapshot: &mut CsvSnapshot, mutation: &CsvMutation) -> protocol::MutationOutcome<CsvDiff> {
    let outcome = <CsvMutation as Mutation<CsvSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &CsvMutation, base: &CsvSnapshot) -> protocol::MutationOutcome<CsvDiff> {
    protocol::MutationOutcome::new(match this {
        CsvMutation::PatchSnapshot(payload) => return protocol::MutationKind::diff(payload, base),
        CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header }) => CsvDiff { has_header: Some(*has_header), records: None },
        CsvMutation::InsertRecord(insert_record::InsertRecord { index, record }) => {
            CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![CsvRecordAdded { index: *index, record: record.clone() }] }) }
        }
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index }) => CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) },
        CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted }) => {
            let mut fields = vec![None; field_index + 1];
            fields[*field_index] = Some(CsvFieldDiff { value: Some(value.clone()), quoted: Some(*quoted) });
            CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: Vec::new(), modified: vec![CsvRecordModified { index: *record_index, diff: CsvRecordDiff { fields: Some(fields) } }], added: Vec::new() }) }
        }
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &CsvMutation, base: &CsvSnapshot) -> Result<Vec<CsvMutation>, semio_framework_value::ValueError> {
    Ok({
    match this {
        CsvMutation::PatchSnapshot(payload) => protocol::MutationKind::inverse(payload, base)?,
        CsvMutation::SetSnapshot(_) => {
            vec![CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })]
        }
        CsvMutation::SetHasHeader(_) => {
            vec![CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: base.has_header })]
        }
        CsvMutation::InsertRecord(insert_record::InsertRecord { index, .. }) => {
            vec![CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: *index })]
        }
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index }) => match base.records.get(*index) {
            Some(record) => vec![CsvMutation::InsertRecord(insert_record::InsertRecord { index: *index, record: record.clone() })],
            None => Vec::new(),
        },
        CsvMutation::SetField(set_field::SetField { record_index, field_index, .. }) => match base.records.get(*record_index).and_then(|r| r.fields.get(*field_index)) {
            Some(field) => vec![CsvMutation::SetField(set_field::SetField { record_index: *record_index, field_index: *field_index, value: field.value.clone(), quoted: field.quoted })],
            None => Vec::new(),
        },
    }

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs








//#region 🔖️RealBinaryOpFrame








// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9




//#endregion 🔖️RealBinaryOpFrame
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
