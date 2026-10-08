//! 🧬️ CsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `CsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::schema::diff::{CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified, CsvRecordsDiff};






use crate::schema::snapshot::{CsvField, CsvRecord};
use crate::CsvSnapshot;

use protocol::{Mutation, MutationDiff};

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
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.csv`. `NoMutation` was dropped: the derive requires every
/// variant to wrap exactly one leaf payload, and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = CsvSnapshot, diff = CsvDiff, schema = "s.stdio.csv")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum CsvMutation {
    SetHasHeader(set_has_header::SetHasHeader),
    InsertRecord(insert_record::InsertRecord),
    RemoveRecord(remove_record::RemoveRecord),
    SetField(set_field::SetField),
}

/// 🧾️ Kebab-case spelling of every `CsvMutation` variant, in declaration order — the exhaustive
/// mutation catalog `csv-rfc4180-any` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-has-header", "insert-record", "remove-record", "set-field"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_csv_mutation(snapshot: &mut CsvSnapshot, mutation: &CsvMutation) -> protocol::MutationOutcome<CsvDiff> {
    let outcome = <CsvMutation as Mutation<CsvSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply

//#endregion 🔖️MutationTrait

//#region 🔖️Net
/// 🧮️ The leaves that carry `base` to exactly `next`: the header flag if it moved, then every record row in place (one `set-field`
/// per differing field; a row whose field count changed is removed and inserted anew) and the diverging tail (surplus rows
/// removed last first, missing rows inserted).
pub fn net_mutations(base: &CsvSnapshot, next: &CsvSnapshot) -> Vec<CsvMutation> {
    let mut leaves = Vec::new();
    if base.has_header != next.has_header {
        leaves.push(CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: next.has_header }));
    }
    let paired = base.records.len().min(next.records.len());
    for (record_index, (before, after)) in base.records.iter().zip(&next.records).enumerate().filter(|(_, (before, after))| before != after) {
        if before.fields.len() == after.fields.len() {
            leaves.extend(before.fields.iter().zip(&after.fields).enumerate().filter(|(_, (old, new))| old != new).map(|(field_index, (_, new))| CsvMutation::SetField(set_field::SetField { record_index, field_index, value: new.value.clone(), quoted: new.quoted })));
        } else {
            leaves.push(CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: record_index }));
            leaves.push(CsvMutation::InsertRecord(insert_record::InsertRecord { index: record_index, record: after.clone() }));
        }
    }
    leaves.extend((paired..base.records.len()).rev().map(|index| CsvMutation::RemoveRecord(remove_record::RemoveRecord { index })));
    leaves.extend(next.records.iter().enumerate().skip(paired).map(|(index, record)| CsvMutation::InsertRecord(insert_record::InsertRecord { index, record: record.clone() })));
    leaves
}
//#endregion 🔖️Net

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



#[cfg(test)]
use protocol::{OpBinary,OpText};
