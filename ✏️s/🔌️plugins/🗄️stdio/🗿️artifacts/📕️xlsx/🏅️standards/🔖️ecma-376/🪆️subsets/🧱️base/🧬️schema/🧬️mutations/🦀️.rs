//! 🧬️ XlsxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::schema::diff::{dec_cell_value, dec_cell_value_bin, dec_sheet, dec_sheet_bin, dec_str, diff_set_snapshot, enc_cell_value, enc_cell_value_bin, enc_sheet, enc_sheet_bin, enc_str, read_str_lp, write_str_lp, XlsxDiff};
#[cfg(test)]
use crate::schema::snapshot::XlsxCell;
#[cfg(test)]
use crate::schema::snapshot::XlsxWorkbook;
use crate::schema::snapshot::{XlsxCellValue, XlsxSheet};
use crate::XlsxSnapshot;
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcRelationship;
#[cfg(test)]
use semio_s_artifact_stdio_zip::opc::OpcTargetMode;

//#region 🔖️Mutations
#[path = "🧭️canonical-edit/🦀️.rs"]
mod canonical_edit;
#[path = "🧭️cell-address/🦀️.rs"]
pub mod cell_address;
#[path = "➕️insert-cell/🦀️.rs"]
pub mod insert_cell;
#[path = "📥️insert-shared-string/🦀️.rs"]
pub mod insert_shared_string;
#[path = "➕insert-sheet/🦀️.rs"]
pub mod insert_sheet;
/// 📐️ Typed content mutation for `stdio.xlsx`. Beyond the baseline `{NoMutation, SetSnapshot}`,
/// this addresses sheets by NAME (identity), cells by `(sheet name, row, col)`, and shared
/// strings by index.
/// 🧪️ F6 CONFIRMED (STEP 1, real `cargo check -p semio-s-plugin-stdio --lib` run, see
/// `f6-xlsx-mutation-check1.txt` in the ticket folder): `#[derive(dsl::DslOps)]` on this enum fails
/// — independent confirmation beyond `XlsxDiff`'s `DiffCodec` blocker:
/// ```text
/// error[E0277]: the trait bound `XlsxCellValue: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️.rs:45:16   (SetCell { .. value: XlsxCellValue })
/// error[E0277]: the trait bound `XlsxSnapshot: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️.rs:23:19   (SetSnapshot { snapshot: XlsxSnapshot })
/// error[E0277]: the trait bound `XlsxSheet: DslField` is not satisfied
///   --> …/🧬️mutations/🦀️.rs:27:16   (InsertSheet { sheet: XlsxSheet })
/// ```
/// `SetCell.value: XlsxCellValue` carries the enum-shaped payload DIRECTLY (same root cause as
/// `XlsxDiff`'s blocker); `SetSnapshot`/`InsertSheet` reach it transitively through
/// `XlsxSnapshot`/`XlsxSheet`. `OpText`/`OpBinary` hand-rolled below, reusing `XlsxDiff`'s
/// `pub(crate)` grammar primitives (`enc_str`/`enc_cell_value`/`enc_sheet`/`split_top_level`/...).
//#region 🔖️Leaves
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "🧽️remove-cell/🦀️.rs"]
pub mod remove_cell;
#[path = "📤️remove-shared-string/🦀️.rs"]
pub mod remove_shared_string;
#[path = "➖remove-sheet/🦀️.rs"]
pub mod remove_sheet;
#[path = "🏷️rename-sheet/🦀️.rs"]
pub mod rename_sheet;
#[path = "✍️set-cell/🦀️.rs"]
pub mod set_cell;
#[path = "🔤️set-shared-string/🦀️.rs"]
pub mod set_shared_string;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = XlsxSnapshot, diff = XlsxDiff, schema = "XlsxMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum XlsxMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    /// ➕️ Inserts a brand-new sheet (possibly pre-populated with cells).
    InsertSheet(insert_sheet::InsertSheet),
    /// ➖️ Removes the sheet named `name`.
    RemoveSheet(remove_sheet::RemoveSheet),
    /// 🏷️ Renames the sheet named `name` to `new_name` (a remove-old+add-new at the diff level —
    /// `name` is the sheet's identity, see the snapshot module's doc comment).
    RenameSheet(rename_sheet::RenameSheet),
    /// ✍️ Replaces one revision-bound canonical SpreadsheetML cell value.
    SetCell(set_cell::SetCell),
    /// ➕️ Inserts one cell into a revision-bound canonical SpreadsheetML vacancy.
    InsertCell(insert_cell::InsertCell),
    /// ➖️ Removes the cell at `(row, col)` in sheet `sheet_name`.
    RemoveCell(remove_cell::RemoveCell),
    /// ➕️ Appends a new shared string.
    InsertSharedString(insert_shared_string::InsertSharedString),
    /// ➖️ Removes the shared string at `index`.
    RemoveSharedString(remove_shared_string::RemoveSharedString),
    /// ✍️ Replaces the shared string at `index`.
    SetSharedString(set_shared_string::SetSharedString),
}

/// 🧾️ Kebab-case spelling of every `XlsxMutation` variant, in declaration order — the exhaustive
/// mutation catalog `xlsx-ecma-376-base` (`../../🔣️oracle.json`) is measured against
/// this exact list. `kinds_match_enum_and_catalog` proves it never drifts from either side.
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "insert-sheet", "remove-sheet", "rename-sheet", "set-cell", "insert-cell", "remove-cell", "insert-shared-string", "remove-shared-string", "set-shared-string"];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_xlsx_mutation(snapshot: &mut XlsxSnapshot, mutation: &XlsxMutation) -> protocol::MutationOutcome<XlsxDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
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
pub(crate) fn agg_diff(this: &XlsxMutation, base: &XlsxSnapshot) -> protocol::MutationOutcome<XlsxDiff> {
    if let XlsxMutation::PatchSnapshot(patch) = this {
        return <patch_snapshot::PatchSnapshot as protocol::MutationKind<XlsxSnapshot, XlsxMutation>>::diff(patch, base);
    }
    match canonical_edit::mutate(base, this) {
        Ok(next) => protocol::MutationOutcome::new(diff_set_snapshot(base, &next)),
        Err(message) => protocol::MutationOutcome::error("mutation.target-mismatch", message, ["xmlParts"]),
    }
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &XlsxMutation, base: &XlsxSnapshot) -> Result<Vec<XlsxMutation>, semio_framework_value::ValueError> {
    Ok({
    if let XlsxMutation::PatchSnapshot(patch) = this {
        return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<XlsxSnapshot, XlsxMutation>>::inverse(patch, base)?);
    }
    canonical_edit::mutate(base, this).ok().filter(|next| next != base).map_or_else(Vec::new, |_| vec![XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })])

    })
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `XlsxMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `XlsxDiff`'s `pub(crate)` grammar primitives rather than
/// duplicating them a second time in this file. Grammar: `keyword arg=value ...` (space-separated,
/// same shape the derive's own handcrafted-wrapper convention uses), one match arm per variant.
//#region 🔖️SnapshotCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_xlsx_snapshot(s: &XlsxSnapshot) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(s))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_xlsx_snapshot(s: &str) -> Result<XlsxSnapshot, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

fn enc_cell_address(address: &cell_address::XlsxCellAddress) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(address))
}

fn dec_cell_address(value: &str) -> Result<cell_address::XlsxCellAddress, String> {
    semio_framework_pack_json::from_json_str(&dec_str(value)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

fn enc_cell_vacancy_address(address: &cell_address::XlsxCellVacancyAddress) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(address))
}

fn dec_cell_vacancy_address(value: &str) -> Result<cell_address::XlsxCellVacancyAddress, String> {
    semio_framework_pack_json::from_json_str(&dec_str(value)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️MutationCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_xlsx_mutation(m: &XlsxMutation) -> String {
    match m {
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_xlsx_snapshot(snapshot)),
        XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => semio_s_artifact_stdio_contract::editing::snapshot_patch_text(patch),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => format!("insert-sheet sheet={}", enc_sheet(sheet)),
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => format!("remove-sheet name={}", enc_str(name)),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => format!("rename-sheet name={} new-name={}", enc_str(name), enc_str(new_name)),
        XlsxMutation::SetCell(set_cell::SetCell { address, value }) => format!("set-cell address={} value={}", enc_cell_address(address), enc_cell_value(value)),
        XlsxMutation::InsertCell(insert_cell::InsertCell { address, value }) => format!("insert-cell address={} value={}", enc_cell_vacancy_address(address), enc_cell_value(value)),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }) => format!("remove-cell address={}", enc_cell_address(address)),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => format!("insert-shared-string value={}", enc_str(value)),
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => format!("remove-shared-string index={index}"),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => format!("set-shared-string index={index} value={}", enc_str(value)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_xlsx_mutation(line: &str) -> Result<XlsxMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').map(|tok| tok.split_once('=').ok_or_else(|| format!("xlsx mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("xlsx mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "patch-snapshot" => semio_s_artifact_stdio_contract::editing::snapshot_patch_from_text(line).map(|patch| XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch })),
        "set-snapshot" => Ok(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_xlsx_snapshot(arg("snapshot")?)? })),
        "insert-sheet" => Ok(XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: dec_sheet(arg("sheet")?)? })),
        "remove-sheet" => Ok(XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: dec_str(arg("name")?)? })),
        "rename-sheet" => Ok(XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: dec_str(arg("name")?)?, new_name: dec_str(arg("new-name")?)? })),
        "set-cell" => Ok(XlsxMutation::SetCell(set_cell::SetCell { address: dec_cell_address(arg("address")?)?, value: dec_cell_value(arg("value")?)? })),
        "insert-cell" => Ok(XlsxMutation::InsertCell(insert_cell::InsertCell { address: dec_cell_vacancy_address(arg("address")?)?, value: dec_cell_value(arg("value")?)? })),
        "remove-cell" => Ok(XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: dec_cell_address(arg("address")?)? })),
        "insert-shared-string" => Ok(XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: dec_str(arg("value")?)? })),
        "remove-shared-string" => Ok(XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: usize_arg("index")? })),
        "set-shared-string" => Ok(XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: usize_arg("index")?, value: dec_str(arg("value")?)? })),
        other => Err(format!("xlsx mutation: unknown keyword {other:?}")),
    }
}

impl OpText for XlsxMutation {
    fn print_op(&self) -> String {
        print_xlsx_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_xlsx_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_xlsx_snapshot_bin(s: &XlsxSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &semio_framework_pack_json::to_json_string(s));
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_xlsx_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<XlsxSnapshot, String> {
    semio_framework_pack_json::from_json_str(&read_str_lp(reader)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
//#endregion 🔖️OpBinaryCodec

//#region 🏷️WireTags
/// 🏷️ Op tags of `XlsxMutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.
const WIRE_PROTOCOL: &str = include_str!("💾️binary/📡️.protocol.semio");
const TAG_SET_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-snapshot");
const TAG_PATCH_SNAPSHOT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "patch-snapshot");
const TAG_INSERT_SHEET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-sheet");
const TAG_REMOVE_SHEET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-sheet");
const TAG_RENAME_SHEET: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "rename-sheet");
const TAG_SET_CELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-cell");
const TAG_INSERT_CELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-cell");
const TAG_REMOVE_CELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-cell");
const TAG_INSERT_SHARED_STRING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-shared-string");
const TAG_REMOVE_SHARED_STRING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-shared-string");
const TAG_SET_SHARED_STRING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "set-shared-string");
//#endregion 🏷️WireTags

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut (confirmed still on that
/// shortcut live by direct read of this file before this wave, not assumed). `tag` is the
/// `XlsxMutation` variant ordinal, in the SAME 0-9 order `print_xlsx_mutation`'s own keyword
/// match uses.
impl OpBinary for XlsxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            XlsxMutation::SetSnapshot(_) => TAG_SET_SNAPSHOT,
            XlsxMutation::PatchSnapshot(_) => TAG_PATCH_SNAPSHOT,
            XlsxMutation::InsertSheet(_) => TAG_INSERT_SHEET,
            XlsxMutation::RemoveSheet(_) => TAG_REMOVE_SHEET,
            XlsxMutation::RenameSheet(_) => TAG_RENAME_SHEET,
            XlsxMutation::SetCell(_) => TAG_SET_CELL,
            XlsxMutation::InsertCell(_) => TAG_INSERT_CELL,
            XlsxMutation::RemoveCell(_) => TAG_REMOVE_CELL,
            XlsxMutation::InsertSharedString(_) => TAG_INSERT_SHARED_STRING,
            XlsxMutation::RemoveSharedString(_) => TAG_REMOVE_SHARED_STRING,
            XlsxMutation::SetSharedString(_) => TAG_SET_SHARED_STRING,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => enc_xlsx_snapshot_bin(snapshot, &mut out),
            XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch }) => out.extend(protocol::OpBinary::encode_op(patch)?),
            XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }) => enc_sheet_bin(sheet, &mut out),
            XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }) => write_str_lp(&mut out, name),
            XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }) => {
                write_str_lp(&mut out, name);
                write_str_lp(&mut out, new_name);
            }
            XlsxMutation::SetCell(set_cell::SetCell { address, value }) => {
                write_str_lp(&mut out, &semio_framework_pack_json::to_json_string(address));
                enc_cell_value_bin(value, &mut out);
            }
            XlsxMutation::InsertCell(insert_cell::InsertCell { address, value }) => {
                write_str_lp(&mut out, &semio_framework_pack_json::to_json_string(address));
                enc_cell_value_bin(value, &mut out);
            }
            XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }) => write_str_lp(&mut out, &semio_framework_pack_json::to_json_string(address)),
            XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }) => write_str_lp(&mut out, value),
            XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }) => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }) => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                write_str_lp(&mut out, value);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_PATCH_SNAPSHOT => Ok(XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot {
                patch: <semio_s_artifact_stdio_contract::editing::SnapshotPatch as protocol::OpBinary>::decode_op(reader.read_bytes(reader.remaining()).map_err(|e| protocol::ProtocolError::Malformed {
                    what: "patch-snapshot payload",
                    offset: reader.position() as u64,
                    detail: e.to_string(),
                })?)?,
            })),
            TAG_SET_SNAPSHOT => {
                let snapshot = dec_xlsx_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            TAG_INSERT_SHEET => {
                let sheet = dec_sheet_bin(&mut reader).map_err(|e| malformed("op sheet", reader.position(), e))?;
                Ok(XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet }))
            }
            TAG_REMOVE_SHEET => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                Ok(XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name }))
            }
            TAG_RENAME_SHEET => {
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", reader.position(), e))?;
                let new_name = read_str_lp(&mut reader).map_err(|e| malformed("op new_name", reader.position(), e))?;
                Ok(XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name, new_name }))
            }
            TAG_SET_CELL => {
                let address = semio_framework_pack_json::from_json_str(&read_str_lp(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| malformed("op address", reader.position(), error.to_string()))?;
                let value = dec_cell_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::SetCell(set_cell::SetCell { address, value }))
            }
            TAG_INSERT_CELL => {
                let address = semio_framework_pack_json::from_json_str(&read_str_lp(&mut reader).map_err(|e| malformed("op vacancy address", reader.position(), e))?, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| malformed("op vacancy address", reader.position(), error.to_string()))?;
                let value = dec_cell_value_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::InsertCell(insert_cell::InsertCell { address, value }))
            }
            TAG_REMOVE_CELL => {
                let address = semio_framework_pack_json::from_json_str(&read_str_lp(&mut reader).map_err(|e| malformed("op address", reader.position(), e))?, semio_framework_pack_json::JsonMemberPolicy::Reject)
                    .map_err(|error| malformed("op address", reader.position(), error.to_string()))?;
                Ok(XlsxMutation::RemoveCell(remove_cell::RemoveCell { address }))
            }
            TAG_INSERT_SHARED_STRING => {
                let value = read_str_lp(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value }))
            }
            TAG_REMOVE_SHARED_STRING => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index }))
            }
            TAG_SET_SHARED_STRING => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let value = read_str_lp(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index, value }))
            }
            other => Err(malformed("op tag", 1, format!("unknown XlsxMutation tag {other}"))),
        }
    }
}
//#endregion 🔖️MutationCodec
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `XlsxSnapshot`/`XlsxMutation` fixtures -- the single source of
/// truth reused by this file's own `mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`
/// tests below AND by `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape docx's own `demo_mutation_cases()` establishes (this wave's OPC
/// pattern-setter). Promoted from the former test-only `fixture`/`sweep_a`/`sweep_b`/
/// `sample_mutations` (the last renamed for the same convention).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn fixture() -> XlsxSnapshot {
    crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![XlsxSheet { name: "Sheet1".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }] }, XlsxSheet { name: "Sheet2".into(), cells: vec![] }],
        shared_strings: vec!["hello".into()],
    })
}

//#region 🔖️Fixtures
/// 📦️ Content type of the sweep fixtures' binary OPC parts.
#[cfg(test)]
const SWEEP_BINARY_CONTENT_TYPE: &str = "application/octet-stream";

/// 🌱 `sweep_a`/`sweep_b`: differ in EVERY lane the snapshot carries. The authoritative XML parts
/// (built by `build_minimal_xlsx` from each workbook): the workbook part (sheet `toDrop` replaced by
/// `added`), the `toModify` worksheet (cells removed + modified + added), the shared-string table
/// (index-keyed, a different length on each side, so `a -> b` removes + modifies and `b -> a` adds +
/// modifies). The lossless OPC lane: binary parts (one removed, one modified in bytes AND content type,
/// one added), a content-type default and overrides, and part-owned relationships (one owner removed,
/// one modified, one added — the added one External).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_a() -> XlsxSnapshot {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet { name: "toModify".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(1.0) }, XlsxCell { row: 2, col: 0, value: XlsxCellValue::Boolean(false) }] },
            XlsxSheet { name: "stay".into(), cells: vec![] },
            XlsxSheet { name: "toDrop".into(), cells: vec![XlsxCell { row: 1, col: 0, value: XlsxCellValue::SharedString(0) }] },
        ],
        // 🎯️ Length 3 vs `sweep_b`'s 2: per this ticket's "known structural trap" note, a
        // single same-direction `between()` over an index-keyed (pairwise-position-matched)
        // collection can never show BOTH `removed` AND `added` from one call -- so `a -> b`
        // exercises `shared_strings.removed` (index 2, since `b` is shorter) +
        // `shared_strings.modified` (index 1); `b -> a` (asserted separately in
        // `field_sweep`) exercises `shared_strings.added` (the same index 2, recurring).
        shared_strings: vec!["keep".into(), "toModify".into(), "toRemove".into()],
    });
    snapshot.opc.content_types.set_default("bin", SWEEP_BINARY_CONTENT_TYPE);
    snapshot.opc.set_part("xl/media/toModify.bin", SWEEP_BINARY_CONTENT_TYPE, b"old".to_vec());
    snapshot.opc.set_part("xl/media/toRemove.bin", SWEEP_BINARY_CONTENT_TYPE, b"gone".to_vec());
    snapshot.opc.relationships.replace_owner("xl/media/toModify.bin".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "old.bin".into(), target_mode: OpcTargetMode::Internal }]);
    snapshot.opc.relationships.replace_owner("xl/media/toRemove.bin".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "gone.bin".into(), target_mode: OpcTargetMode::Internal }]);
    snapshot.opc.parts.sort_by(|left, right| left.path.cmp(&right.path));
    snapshot
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn sweep_b() -> XlsxSnapshot {
    let mut snapshot = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook {
        sheets: vec![
            XlsxSheet {
                name: "toModify".into(),
                cells: vec![
                    // row (1,0) survives with a changed value; row (2,0) is dropped; row
                    // (3,0) is a NET-NEW cell -- exercises `cells.removed` +
                    // `cells.modified` + `cells.added` all in the SAME sheet-modify.
                    XlsxCell { row: 1, col: 0, value: XlsxCellValue::Number(2.0) },
                    XlsxCell { row: 3, col: 0, value: XlsxCellValue::Formula { expr: "SUM(A1:A2)".into(), cached: Some(Box::new(XlsxCellValue::Number(3.0))) } },
                ],
            },
            XlsxSheet { name: "stay".into(), cells: vec![] },
            XlsxSheet { name: "added".into(), cells: vec![XlsxCell { row: 1, col: 1, value: XlsxCellValue::InlineString("brand new".into()) }] },
        ],
        // 🎯️ Length 2: index 2 ("toRemove") no longer exists — exercises
        // `shared_strings.removed` on `a -> b` (see `sweep_a`'s doc comment); the same
        // index recurs as `shared_strings.added` on `b -> a`.
        shared_strings: vec!["keep".into(), "toModify-changed".into()],
    });
    snapshot.opc.content_types.set_default("bin", SWEEP_BINARY_CONTENT_TYPE);
    snapshot.opc.content_types.set_default("dat", SWEEP_BINARY_CONTENT_TYPE);
    snapshot.opc.set_part("xl/media/toModify.bin", "application/x-semio-sweep", b"new".to_vec());
    snapshot.opc.set_part("xl/media/added.dat", SWEEP_BINARY_CONTENT_TYPE, b"fresh".to_vec());
    snapshot.opc.relationships.replace_owner("xl/media/toModify.bin".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "new.bin".into(), target_mode: OpcTargetMode::Internal }]);
    snapshot.opc.relationships.replace_owner("xl/media/added.dat".into(), vec![OpcRelationship { id: "rId1".into(), rel_type: "http://example/sweep".into(), target: "added.bin".into(), target_mode: OpcTargetMode::External }]);
    snapshot.opc.parts.sort_by(|left, right| left.path.cmp(&right.path));
    snapshot
}
//#endregion 🔖️Fixtures

/// 🧪️ The demo cases proper -- one representative `XlsxMutation` per variant.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<XlsxMutation> {
    let base = fixture();
    let address = cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).expect("fixture cell address");
    let vacancy = cell_address::xlsx_cell_vacancy_address(&base, "Sheet1", 2, 1).expect("fixture cell vacancy address");
    vec![
        XlsxMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        XlsxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: sweep_b() }),
        XlsxMutation::InsertSheet(insert_sheet::InsertSheet { sheet: XlsxSheet { name: "x".into(), cells: vec![] } }),
        // 🎯️ `RemoveSheet`/`RenameSheet` target `"Sheet2"`, the LAST sheet in `fixture()` --
        // same last-position caveat as the dedicated `remove_sheet_inverse_restores_removed_sheet`/
        // `rename_sheet_apply_and_inverse` tests document (`sheets` is name-keyed; a
        // mutation-level `InsertSheet`-based inverse always APPENDS, so exact Vec-position
        // restoration is only guaranteed when the target was already last).
        XlsxMutation::RemoveSheet(remove_sheet::RemoveSheet { name: "Sheet2".into() }),
        XlsxMutation::RenameSheet(rename_sheet::RenameSheet { name: "Sheet2".into(), new_name: "Renamed".into() }),
        XlsxMutation::SetCell(set_cell::SetCell { address, value: XlsxCellValue::Boolean(true) }),
        XlsxMutation::InsertCell(insert_cell::InsertCell { address: vacancy, value: XlsxCellValue::InlineString("created".into()) }),
        XlsxMutation::RemoveCell(remove_cell::RemoveCell { address: cell_address::xlsx_cell_address(&base, "Sheet1", 1, 0).expect("fixture cell address") }),
        XlsxMutation::InsertSharedString(insert_shared_string::InsertSharedString { value: "z".into() }),
        // 🎯️ `RemoveSharedString` targets index 0, the LAST-in-position entry `fixture()`
        // (which has only one shared string) has -- like docx's own `RemovePart` precedent
        // (see that artifact's `sample_mutations` doc comment), a name/key-keyed (here
        // index-keyed) collection's mutation-level inverse only restores exact ORIGINAL
        // Vec position when the removed item was already last; the same caveat applies here.
        XlsxMutation::RemoveSharedString(remove_shared_string::RemoveSharedString { index: 0 }),
        XlsxMutation::SetSharedString(set_shared_string::SetSharedString { index: 0, value: "y".into() }),
    ]
}
//#endregion 🔖️DemoCases

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
