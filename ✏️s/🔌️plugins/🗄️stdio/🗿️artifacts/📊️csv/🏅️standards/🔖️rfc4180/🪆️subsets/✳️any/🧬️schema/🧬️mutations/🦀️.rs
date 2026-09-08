//! 🧬️ CsvMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `CsvDiff` directly — apply-and-capture is banned); `inverse()` is
//! handcrafted per variant, index-aware, reading the pre-state it needs from `base`.

use crate::schema::diff::{dec_record, dec_str, diff_set_snapshot, enc_record, enc_str, split_top_level, strip_brackets, CsvDiff, CsvFieldDiff, CsvRecordAdded, CsvRecordDiff, CsvRecordModified, CsvRecordsDiff};
use crate::schema::snapshot::{CsvField, CsvRecord};
use crate::CsvSnapshot;
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
fn op_pack_err(e: &dsl::PackError) -> protocol::ProtocolError {
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
        let ordinal = r.read_u8().map_err(|error| op_pack_err(&error))?;
        let mutation = match ordinal {
            1 => CsvMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: read_bin_snapshot(&mut r).map_err(|error| op_pack_err(&error))? }),
            2 => CsvMutation::SetHasHeader(set_has_header::SetHasHeader { has_header: r.read_u8().map_err(|error| op_pack_err(&error))? != 0 }),
            3 => {
                let index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                let record = read_bin_record(&mut r).map_err(|error| op_pack_err(&error))?;
                CsvMutation::InsertRecord(insert_record::InsertRecord { index, record })
            }
            4 => CsvMutation::RemoveRecord(remove_record::RemoveRecord { index: r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize }),
            5 => {
                let record_index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                let field_index = r.read_varint_u64().map_err(|error| op_pack_err(&error))? as usize;
                let quoted = r.read_u8().map_err(|error| op_pack_err(&error))? != 0;
                let value = read_bin_str(&mut r).map_err(|error| op_pack_err(&error))?;
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
