//! 🧬️ CsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `CsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::artifacts::csv::schema::diff::{dec_record, dec_str, diff_set_snapshot, enc_record, enc_str, split_top_level, strip_brackets, CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified, CsvRecordsDiff};
use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
use crate::artifacts::csv::CsvSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};

//#region 🔖️Mutations
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
#[path = "🧾set-has-header/🦀️.rs"]
pub mod set_has_header;
#[path = "📥insert-record/🦀️.rs"]
pub mod insert_record;
#[path = "📤remove-record/🦀️.rs"]
pub mod remove_record;
#[path = "✏️set-field/🦀️.rs"]
pub mod set_field;
//#endregion 🔖️Leaves

/// 📐️ Typed content mutation for `stdio.csv`. `NoMutation` was dropped: the derive requires every
/// variant to wrap exactly one leaf payload, and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = CsvSnapshot, diff = CsvDiff, schema = "s.stdio.csv")]
pub enum CsvMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetHasHeader(set_has_header::SetHasHeader),
    InsertRecord(insert_record::InsertRecord),
    RemoveRecord(remove_record::RemoveRecord),
    SetField(set_field::SetField),
}

/// 🧾️ Kebab-case spelling of every `CsvMutation` variant, in declaration order — the exhaustive
/// mutation catalog `csv-rfc4180-any` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "set-has-header", "insert-record", "remove-record", "set-field"];
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
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &CsvMutation, base: &CsvSnapshot) -> protocol::MutationOutcome<CsvDiff> {
        protocol::MutationOutcome::new(match this {
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header }) => CsvDiff { has_header: Some(*has_header), records: None },
            CsvMutation::InsertRecord(insert_record::InsertRecord { index, record }) => CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: Vec::new(), modified: Vec::new(), added: vec![CsvRecordAdded { index: *index, record: record.clone() }] }) },
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index }) => CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) },
            CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted }) => {
                let mut fields = vec![None; field_index + 1];
                fields[*field_index] = Some(CsvFieldDiff { value: Some(value.clone()), quoted: Some(*quoted) });
                CsvDiff { has_header: None, records: Some(CsvRecordsDiff { removed: Vec::new(), modified: vec![CsvRecordModified { index: *record_index, diff: CsvRecordDiff { fields: Some(fields) } }], added: Vec::new() }) }
            }
        })
    }

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &CsvMutation, base: &CsvSnapshot) -> Vec<CsvMutation> {
        match this {
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
    }
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `CsvMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above — a macro hygiene bug, not §3a/§3b) — reuses `CsvDiff`'s
/// `pub(crate)` grammar primitives (`hex`/`split_top_level`/`encode_option`/`enc_record`/...)
/// rather than duplicating them a second time in this file. Grammar: `keyword arg=value ...`
/// (space-separated), same convention gif89a's/svg's own hand-rolled `OpText` impls use, one
/// match arm per variant (no `DslVariants` scaffolding available since nothing here derives it).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_csv_snapshot(s: &CsvSnapshot) -> String {
    format!("[{},{},[{}]]", enc_str(&s.schema), if s.has_header { 1 } else { 0 }, s.records.iter().map(enc_record).collect::<Vec<_>>().join(","),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_csv_snapshot(s: &str) -> Result<CsvSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, has_header, records] = parts.as_slice() else {
        return Err(format!("csv snapshot: expected 3 fields, got {}", parts.len()));
    };
    let records = split_top_level(strip_brackets(records)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_record).collect::<Result<Vec<_>, String>>()?;
    Ok(CsvSnapshot { schema: dec_str(schema)?, has_header: *has_header == "1", records })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_csv_mutation(m: &CsvMutation) -> String {
    match m {
        CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_csv_snapshot(snapshot)),
        CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header }) => format!("set-has-header has-header={}", if *has_header { 1 } else { 0 }),
        CsvMutation::InsertRecord(insert_record::InsertRecord { index, record }) => format!("insert-record index={index} record={}", enc_record(record)),
        CsvMutation::RemoveRecord(remove_record::RemoveRecord { index }) => format!("remove-record index={index}"),
        CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted }) => format!("set-field record-index={record_index} field-index={field_index} value={} quoted={}", enc_str(value), if *quoted { 1 } else { 0 },),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_csv_mutation(line: &str) -> Result<CsvMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("csv mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("csv mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_csv_snapshot(arg("snapshot")?)? })),
        "set-has-header" => Ok(CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: arg("has-header")? == "1" })),
        "insert-record" => Ok(CsvMutation::InsertRecord(insert_record::InsertRecord { index: usize_arg("index")?, record: dec_record(arg("record")?)? })),
        "remove-record" => Ok(CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: usize_arg("index")? })),
        "set-field" => Ok(CsvMutation::SetField(set_field::SetField { record_index: usize_arg("record-index")?, field_index: usize_arg("field-index")?, value: dec_str(arg("value")?)?, quoted: arg("quoted")? == "1" })),
        other => Err(format!("csv mutation: unknown keyword {other:?}")),
    }
}

impl OpText for CsvMutation {
    fn print_op(&self) -> String {
        print_csv_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_csv_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️RealBinaryOpFrame
/// 🧪️ P2-P1: **real binary op-frame** for `CsvMutation` — upgraded from the F6-era
/// `print_op().into_bytes()` text-as-binary shortcut. `tag u8` ordinal (hand-assigned, this
/// enum cannot use `#[derive(dsl::DslOps)]`, see the doc comment above) + per-variant fields,
/// via `dsl::ByteWriter`/`dsl::ByteReader` (the real framework LEB128-varint/length-prefixed
/// primitives, `🧰️framework/…/🎒️pack/🧾️codec/🦀️.rs`, reachable from stdio because
/// `extern crate self as pack;` is re-exported at the kernel crate root and `dsl`/`store`/
/// `protocol` all alias that SAME crate root). Matches
/// `../💾️binary/📡️.protocol.semio`'s real `repeat`/`arm` shape exactly — see that
/// file's own doc comment for why the deeply nested `CsvSnapshot`/`CsvRecord` payload inside
/// arms 1/3 is one honest opaque tail blob rather than individually walked.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_str(w: &mut dsl::ByteWriter, s: &str) {
    let bytes = s.as_bytes();
    w.write_varint_u64(bytes.len() as u64);
    w.write_bytes(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_str(r: &mut dsl::ByteReader<'_>) -> Result<String, dsl::PackError> {
    let len = r.read_varint_u64()? as usize;
    let bytes = r.read_bytes(len)?;
    String::from_utf8(bytes.to_vec()).map_err(|e| dsl::PackError::Malformed { what: "csv binary utf8 string", offset: 0, detail: e.to_string() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_field(w: &mut dsl::ByteWriter, f: &CsvField) {
    write_bin_str(w, &f.value);
    w.write_u8(if f.quoted { 1 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_field(r: &mut dsl::ByteReader<'_>) -> Result<CsvField, dsl::PackError> {
    let value = read_bin_str(r)?;
    let quoted = r.read_u8()? != 0;
    Ok(CsvField { value, quoted })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bin_record(w: &mut dsl::ByteWriter, rec: &CsvRecord) {
    w.write_varint_u64(rec.fields.len() as u64);
    for f in &rec.fields {
        write_bin_field(w, f);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bin_record(r: &mut dsl::ByteReader<'_>) -> Result<CsvRecord, dsl::PackError> {
    let n = r.read_varint_u64()? as usize;
    let mut fields = Vec::with_capacity(n);
    for _ in 0..n {
        fields.push(read_bin_field(r)?);
    }
    Ok(CsvRecord { fields })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bin_snapshot(w: &mut dsl::ByteWriter, s: &CsvSnapshot) {
    write_bin_str(w, &s.schema);
    w.write_u8(if s.has_header { 1 } else { 0 });
    w.write_varint_u64(s.records.len() as u64);
    for r in &s.records {
        write_bin_record(w, r);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bin_snapshot(r: &mut dsl::ByteReader<'_>) -> Result<CsvSnapshot, dsl::PackError> {
    let schema = read_bin_str(r)?;
    let has_header = r.read_u8()? != 0;
    let n = r.read_varint_u64()? as usize;
    let mut records = Vec::with_capacity(n);
    for _ in 0..n {
        records.push(read_bin_record(r)?);
    }
    Ok(CsvSnapshot { schema, has_header, records })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn op_pack_err(e: dsl::PackError) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what: "csv op binary", offset: 0, detail: e.to_string() }
}

impl OpBinary for CsvMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut w = dsl::ByteWriter::new();
        match self {
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
                w.write_u8(1);
                write_bin_snapshot(&mut w, snapshot);
            }
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header }) => {
                w.write_u8(2);
                w.write_u8(if *has_header { 1 } else { 0 });
            }
            CsvMutation::InsertRecord(insert_record::InsertRecord { index, record }) => {
                w.write_u8(3);
                w.write_varint_u64(*index as u64);
                write_bin_record(&mut w, record);
            }
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index }) => {
                w.write_u8(4);
                w.write_varint_u64(*index as u64);
            }
            CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted }) => {
                w.write_u8(5);
                w.write_varint_u64(*record_index as u64);
                w.write_varint_u64(*field_index as u64);
                w.write_u8(if *quoted { 1 } else { 0 });
                write_bin_str(&mut w, value);
            }
        }
        Ok(w.into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut r = dsl::ByteReader::new(bytes);
        let ordinal = r.read_u8().map_err(op_pack_err)?;
        let mutation = match ordinal {
            1 => CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: read_bin_snapshot(&mut r).map_err(op_pack_err)? }),
            2 => CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: r.read_u8().map_err(op_pack_err)? != 0 }),
            3 => {
                let index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let record = read_bin_record(&mut r).map_err(op_pack_err)?;
                CsvMutation::InsertRecord(insert_record::InsertRecord { index, record })
            }
            4 => CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: r.read_varint_u64().map_err(op_pack_err)? as usize }),
            5 => {
                let record_index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let field_index = r.read_varint_u64().map_err(op_pack_err)? as usize;
                let quoted = r.read_u8().map_err(op_pack_err)? != 0;
                let value = read_bin_str(&mut r).map_err(op_pack_err)?;
                CsvMutation::SetField(set_field::SetField { record_index, field_index, value, quoted })
            }
            other => {
                return Err(protocol::ProtocolError::Malformed { what: "csv op ordinal", offset: 0, detail: format!("unknown ordinal {other}") });
            }
        };
        Ok(mutation)
    }
}
//#endregion 🔖️RealBinaryOpFrame
//#endregion OpCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::csv::schema::snapshot::CsvField;
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn field(value: &str, quoted: bool) -> CsvField {
        CsvField { value: value.into(), quoted }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn record(fields: &[(&str, bool)]) -> CsvRecord {
        CsvRecord { fields: fields.iter().map(|(v, q)| field(v, *q)).collect() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> CsvSnapshot {
        CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("name", false), ("note", true)]), record(&[("a", false), ("b", false)]), record(&[("x", false), ("y", false)])] }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ Canonical "differs in every mutable field" snapshot A: 3 records — one that will
    /// be removed, one that will be modified in every field, one untouched (so `sweep_b`'s
    /// added record has something stable to anchor its own index against).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> CsvSnapshot {
        CsvSnapshot { schema: "stdio.csv".into(), has_header: true, records: vec![record(&[("gone", false), ("also-gone", true)]), record(&[("old-a", false), ("old-b", true)]), record(&[("stable", false)])] }
    }
    /// 🧬️ Sweep B: `has_header` flips, record 0 is removed, record 1 (now index 0) is
    /// modified in every field (value AND quoted), record 2 (now index 1) is untouched, and
    /// a brand-new record is added at the end.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> CsvSnapshot {
        CsvSnapshot { schema: "stdio.csv".into(), has_header: false, records: vec![record(&[("new-a", true), ("new-b", false)]), record(&[("stable", false)]), record(&[("brand-new", true)])] }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️MutationDiffLaw
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        let variants = vec![
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
            CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new", true)]) }),
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
            CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
        ];
        for m in variants {
            let diff = m.diff(&base);
            let expected = diff.diff().apply(&base).unwrap();

            let mut via_apply = base.clone();
            let returned_diff = apply_csv_mutation(&mut via_apply, &m);

            assert_eq!(via_apply, expected, "apply_csv_mutation mismatch for {m:?}");
            assert_eq!(returned_diff, diff, "returned diff mismatch for {m:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base = base_snapshot();
        let variants = vec![
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
            CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new", true)]) }),
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
            CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
        ];
        for m in variants {
            // 🔁️ mutation-level round trip
            let mut forward = base.clone();
            apply_csv_mutation(&mut forward, &m);
            for inv in m.inverse(&base) {
                apply_csv_mutation(&mut forward, &inv);
            }
            assert_eq!(forward, base, "mutation-level inverse round trip failed for {m:?}");

            // 🔁️ diff-level round trip
            let d = m.diff(&base);
            let mid = d.diff().apply(&base).unwrap();
            let back = d.diff().inverse(&base).apply(&mid).unwrap();
            assert_eq!(back, base, "diff-level inverse round trip failed for {m:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        let base = base_snapshot();

        // 🧩 Insert(2) + Remove(0): the two-op sequence base → mid → after.
        let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: record(&[("ins", false)]) }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Remove-before absorb mismatch");

        // 🧩 Insert(2,f) + Insert(2,g): both must survive (fixes the old op-slot LWW bug).
        let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: record(&[("f", false)]) }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 2, record: record(&[("g", false)]) }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Insert+Insert-same-index absorb mismatch");
        assert_eq!(after.records.len(), base.records.len() + 2, "both inserts must survive");

        // 🧩 Add + SetField: patch into the added payload.
        let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("orig", false)]) }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "patched".into(), quoted: true }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Add+SetField absorb mismatch");
        assert_eq!(after.records[1].fields[0].value, "patched");

        // 🧩 Modify + Remove: modifying then removing the same record collapses to a removal.
        let d1 = CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "will-vanish".into(), quoted: false }).diff(&base);
        let mid = d1.diff().apply(&base).unwrap();
        let d2 = CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 1 }).diff(&mid);
        let after = d2.diff().apply(&mid).unwrap();
        let mut composed = d1.diff().clone();
        composed.absorb(d2.diff().clone());
        assert_eq!(composed.apply(&base).unwrap(), after, "Modify+Remove absorb mismatch");

        // 🧩 Associativity over a triple.
        let base = base_snapshot();
        let d1 = CsvMutation::InsertRecord(insert_record::InsertRecord { index: 0, record: record(&[("a", false)]) }).diff(&base);
        let s1 = d1.diff().apply(&base).unwrap();
        let d2 = CsvMutation::SetField(set_field::SetField { record_index: 0, field_index: 0, value: "a2".into(), quoted: true }).diff(&s1);
        let s2 = d2.diff().apply(&s1).unwrap();
        let d3 = CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 2 }).diff(&s2);
        let s3 = d3.diff().apply(&s2).unwrap();

        let mut left = d1.diff().clone();
        left.absorb(d2.diff().clone());
        left.absorb(d3.diff().clone());

        let mut d23 = d2.diff().clone();
        d23.absorb(d3.diff().clone());
        let mut right = d1.diff().clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base).unwrap(), s3);
        assert_eq!(right.apply(&base).unwrap(), s3);
        assert_eq!(left.apply(&base).unwrap(), right.apply(&base).unwrap(), "absorb must be associative");
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = base_snapshot();
        let b = sweep_b();
        assert_eq!(CsvDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(CsvDiff::between(&b, &a).apply(&b).unwrap(), a);

        // synthetic: differing field counts within an overlapping index (record replace path).
        let mut c = a.clone();
        c.records[0] = record(&[("only-one-field", false)]);
        assert_eq!(CsvDiff::between(&a, &c).apply(&a).unwrap(), c);
        assert_eq!(CsvDiff::between(&c, &a).apply(&c).unwrap(), a);

        assert!(CsvDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️FieldSweep
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_every_mutable_field_changes() {
        let a = sweep_a();
        let b = sweep_b();

        let d_ab = CsvDiff::between(&a, &b);
        assert_eq!(d_ab.apply(&a).unwrap(), b, "between(a,b).apply(a) == b");

        let d_ba = CsvDiff::between(&b, &a);
        assert_eq!(d_ba.apply(&b).unwrap(), a, "between(b,a).apply(b) == a");

        // 🔍 Hand-written per-field assertion: every field of `CsvDiff` is populated.
        assert!(d_ab.has_header.is_some(), "has_header must be populated");
        let records = d_ab.records.as_ref().expect("records diff must be populated");
        assert!(!records.removed.is_empty(), "removed must be non-empty (record 0 dropped)");
        assert!(!records.modified.is_empty(), "modified must be non-empty (record 1 changed in every field)");
        assert!(!records.added.is_empty(), "added must be non-empty (brand-new record)");
        let modified = &records.modified[0];
        let field_patches = modified.diff.fields.as_ref().expect("record 1's field patch list must be populated");
        assert!(field_patches.iter().all(|f| f.is_some()), "every field of the modified record must be patched");
        for patch in field_patches.iter().flatten() {
            assert!(patch.value.is_some() && patch.quoted.is_some(), "every field patch must set BOTH value and quoted");
        }

        assert!(CsvDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `CsvMutation` grammar —
    /// exercises every variant, incl. a `SetSnapshot` payload whose record fields contain the
    /// grammar's own reserved separator characters (`,`/`[`/`]`/space) to prove hex-encoding
    /// sidesteps escaping entirely.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let mutations = vec![
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: CsvSnapshot { schema: "stdio.csv".into(), has_header: false, records: vec![record(&[("a, tricky [value]", true), ("plain", false)])] } }),
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: true }),
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
            CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new, [tricky]", true)]) }),
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
            CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
            CsvMutation::SetField(set_field::SetField { record_index: 0, field_index: 2, value: "with, comma [and] brackets".into(), quoted: false }),
        ];
        for m in mutations {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = CsvMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = CsvMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#region 🔖️OpsGrammarConformanceLaw
    /// 🧪️ P2-P1 item 6: `dsl::parse_grammar` + `dsl::Recognizer` recognize REAL `print_op`
    /// output for several real mutations (not just one trivial case), incl. `SetSnapshot`'s own
    /// nested positional-tuple `snapshot-value` production.
    #[semio_framework_async_macros::async_test]
    async fn ops_grammar_conformance_law() {
        let grammar_text = crate::artifacts::csv::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO;
        let grammar = dsl::parse_grammar(grammar_text).expect("parse mutations grammar");
        let recognizer = dsl::Recognizer::compile(&grammar);

        let mutations = vec![
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
            CsvMutation::InsertRecord(insert_record::InsertRecord { index: 1, record: record(&[("new", true)]) }),
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
            CsvMutation::SetField(set_field::SetField { record_index: 1, field_index: 0, value: "changed".into(), quoted: true }),
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        ];
        for m in mutations {
            let printed = m.print_op();
            let ok = recognizer.recognize(&printed).unwrap_or_else(|e| panic!("recognize({printed:?}) errored: {e:?}"));
            assert!(ok, "mutations grammar must recognize real print_op output {printed:?} for {m:?}");
        }
    }
    //#endregion 🔖️OpsGrammarConformanceLaw

    //#region 🔖️KindsConformanceLaw
    /// 🧭️ `kind_of` is an EXHAUSTIVE match (no wildcard arm) — the compiler refuses this file if a
    /// variant is added to `CsvMutation` without a matching kebab-case spelling here, which is what
    /// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
    /// `kinds` array as text (the framework never parses Rust, so this is the only side that can
    /// prove the manifest matches) and asserts the same list, in the same order.
    #[semio_framework_async_macros::async_test]
    async fn kinds_match_enum_and_catalog() {
        fn kind_of(mutation: &CsvMutation) -> &'static str {
            match mutation {
                CsvMutation::SetSnapshot(_) => "set-snapshot",
                CsvMutation::SetHasHeader(_) => "set-has-header",
                CsvMutation::InsertRecord(_) => "insert-record",
                CsvMutation::RemoveRecord(_) => "remove-record",
                CsvMutation::SetField(_) => "set-field",
            }
        }
        let samples = [
            CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: CsvSnapshot::default() }),
            CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: false }),
            CsvMutation::InsertRecord(insert_record::InsertRecord { index: 0, record: CsvRecord::default() }),
            CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: 0 }),
            CsvMutation::SetField(set_field::SetField { record_index: 0, field_index: 0, value: String::new(), quoted: false }),
        ];
        let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
        assert_eq!(from_enum, KINDS, "KINDS must list every CsvMutation variant, in declaration order");

        let manifest = include_str!("../../🔮️oracle/🔣️.json");
        let needle = "\"kinds\": [";
        let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
        let end = start + manifest[start..].find(']').expect("kinds array is closed");
        let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
        assert_eq!(declared, KINDS, "the oracle manifest's kinds must match CsvMutation exactly");
    }
    //#endregion 🔖️KindsConformanceLaw
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📸️set-snapshot/🧪️tests/✏️corrects-the-area-cell-and-quotes-it/🦀️.rs"]
    mod tests_set_snapshot_corrects_the_area_cell_and_quotes_it;
}
//#endregion 🧪️FixtureTests
