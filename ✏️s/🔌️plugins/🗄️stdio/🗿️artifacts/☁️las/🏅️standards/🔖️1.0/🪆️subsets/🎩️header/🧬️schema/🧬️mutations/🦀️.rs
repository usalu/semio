//! 🧬️ LasMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary beyond
//! the universal `{NoMutation, SetSnapshot}` stub — header fields grouped sensibly
//! (`SetVersion`/`SetSystemIdentifier`/`SetSoftwareInfo`/`SetCreationDate`/`SetScaleAndOffset`/
//! `SetBounds`/`SetPointsByReturn`), `InsertVlr`/`RemoveVlr`/`SetVlrData` and
//! `InsertPoint`/`RemovePoint`/`SetPoint` cover the index-keyed collections. Every variant's
//! `diff()` is handcrafted (constructs `LasDiff` directly via the `schema::diff` builders) —
//! apply-and-capture is never used.
//!
//! Ticket 26/08/29/S-END-TO-END: migrated to `#[derive(dsl::Mutations)]` (mutation-leaf shape,
//! `../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/S-END-TO-END/📓️plan-mutation-leaf-migration.md`),
//! matching `stdio.tiff`'s baseline subset and `stdio.csv`. `NoMutation` was dropped: the derive
//! requires every variant to wrap exactly one leaf payload, a unit variant wraps none, and `no` is
//! not an `APPROVED_VERBS` entry the derive's own const assertion would accept.

use crate::schema::diff::{self, LasDiff};
use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::Mutation;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.las`.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🔢set-version/🦀️.rs"]
pub mod set_version;
#[path = "🏢set-system-identifier/🦀️.rs"]
pub mod set_system_identifier;
#[path = "🛠️set-software-info/🦀️.rs"]
pub mod set_software_info;
#[path = "🕰️set-creation-date/🦀️.rs"]
pub mod set_creation_date;
#[path = "📏set-scale-and-offset/🦀️.rs"]
pub mod set_scale_and_offset;
#[path = "📦set-bounds/🦀️.rs"]
pub mod set_bounds;
#[path = "🔁set-points-by-return/🦀️.rs"]
pub mod set_points_by_return;
#[path = "📥insert-vlr/🦀️.rs"]
pub mod insert_vlr;
#[path = "📤remove-vlr/🦀️.rs"]
pub mod remove_vlr;
#[path = "🗃️set-vlr-data/🦀️.rs"]
pub mod set_vlr_data;
#[path = "➕insert-point/🦀️.rs"]
pub mod insert_point;
#[path = "➖remove-point/🦀️.rs"]
pub mod remove_point;
#[path = "✏️set-point/🦀️.rs"]
pub mod set_point;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = LasSnapshot, diff = LasDiff, schema = "s.stdio.las")]
pub enum LasMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetVersion(set_version::SetVersion),
    SetSystemIdentifier(set_system_identifier::SetSystemIdentifier),
    SetSoftwareInfo(set_software_info::SetSoftwareInfo),
    SetCreationDate(set_creation_date::SetCreationDate),
    SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset),
    SetBounds(set_bounds::SetBounds),
    SetPointsByReturn(set_points_by_return::SetPointsByReturn),
    InsertVlr(insert_vlr::InsertVlr),
    RemoveVlr(remove_vlr::RemoveVlr),
    SetVlrData(set_vlr_data::SetVlrData),
    InsertPoint(insert_point::InsertPoint),
    RemovePoint(remove_point::RemovePoint),
    SetPoint(set_point::SetPoint),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
/// 🧾️ Kebab-case spelling of every `LasMutation` variant, in declaration order — the vocabulary
/// `../../🔣️oracle.json`'s `las-1-0-any` catalog is measured against. Kept honest by
/// `kinds_match_enum_and_catalog` below (the framework never parses Rust to learn this list).
pub const KINDS: &[&str] = &[
    "set-snapshot",
    "set-version",
    "set-system-identifier",
    "set-software-info",
    "set-creation-date",
    "set-scale-and-offset",
    "set-bounds",
    "set-points-by-return",
    "insert-vlr",
    "remove-vlr",
    "set-vlr-data",
    "insert-point",
    "remove-point",
    "set-point",
];
//#endregion 🔖️Kinds

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning a typed error outcome without changing the
/// snapshot when an index target is missing or out of range. `InsertVlr`/`RemoveVlr`/
/// `InsertPoint`/`RemovePoint` keep `header.number_of_vlrs`/
/// `header.number_of_point_records` in sync with the real collection length (`engine::encode_las`
/// also independently recomputes both at encode time — see `LasHeader`'s doc comment — so this
/// sync is a snapshot-level consistency guarantee, not the sole source of correctness).
pub fn apply_las_mutation(snapshot: &mut LasSnapshot, mutation: &LasMutation) -> protocol::MutationOutcome<LasDiff> {
    let outcome = <LasMutation as Mutation<LasSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
/// ▶️ Every variant's `diff()`, dispatched by the leaf that wraps it. Lifted verbatim from the
/// former `impl Mutation<LasSnapshot> for LasMutation` — only each match arm's pattern head
/// changed, from `LasMutation::Variant { a, b }` to `LasMutation::Variant(variant_mod::Variant {
/// a, b })`.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &LasMutation, base: &LasSnapshot) -> protocol::MutationOutcome<LasDiff> {
    protocol::MutationOutcome::new(match this {
        LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        LasMutation::SetVersion(set_version::SetVersion { major, minor }) => diff::diff_set_version(*major, *minor),
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier }) => diff::diff_set_system_identifier(system_identifier),
        LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software }) => diff::diff_set_software_info(generating_software),
        LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year, year }) => diff::diff_set_creation_date(*day_of_year, *year),
        LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale, offset }) => diff::diff_set_scale_and_offset(base, *scale, *offset),
        LasMutation::SetBounds(set_bounds::SetBounds { max, min }) => diff::diff_set_bounds(*max, *min),
        LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts }) => diff::diff_set_points_by_return(*counts),
        LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr }) => diff::diff_insert_vlr(base, *index, vlr.clone()),
        LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }) => diff::diff_remove_vlr(base, *index),
        LasMutation::SetVlrData(set_vlr_data::SetVlrData { index, data }) => diff::diff_set_vlr_data(*index, data.clone()),
        LasMutation::InsertPoint(insert_point::InsertPoint { index, point }) => diff::diff_insert_point(base, *index, point.clone()),
        LasMutation::RemovePoint(remove_point::RemovePoint { index }) => diff::diff_remove_point(base, *index),
        LasMutation::SetPoint(set_point::SetPoint { index, point }) => diff::diff_set_point(base, *index, point),
    })
}

/// ↩️ Handcrafted, index-aware mutation-level inverses. Index-targeted variants look the prior
/// value up in `base`; a stale/out-of-range index inverts to an empty inverse list (nothing to
/// undo) — lifted verbatim from the former `impl Mutation`, `NoMutation`'s single-element
/// `vec![LasMutation::NoMutation]` sentinel replaced by `Vec::new()` now that no unit "do
/// nothing" variant exists (same convention `stdio.csv`'s `agg_inverse` uses for its own
/// out-of-range cases).
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &LasMutation, base: &LasSnapshot) -> Vec<LasMutation> {
    match this {
        LasMutation::SetSnapshot(_) => vec![LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        LasMutation::SetVersion(_) => vec![LasMutation::SetVersion(set_version::SetVersion { major: base.header.version_major, minor: base.header.version_minor })],
        LasMutation::SetSystemIdentifier(_) => vec![LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: base.header.system_identifier.clone() })],
        LasMutation::SetSoftwareInfo(_) => vec![LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: base.header.generating_software.clone() })],
        LasMutation::SetCreationDate(_) => vec![LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: base.header.creation_day_of_year, year: base.header.creation_year })],
        LasMutation::SetScaleAndOffset(_) => vec![LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: (base.header.x_scale, base.header.y_scale, base.header.z_scale), offset: (base.header.x_offset, base.header.y_offset, base.header.z_offset) })],
        LasMutation::SetBounds(_) => vec![LasMutation::SetBounds(set_bounds::SetBounds { max: (base.header.max_x, base.header.max_y, base.header.max_z), min: (base.header.min_x, base.header.min_y, base.header.min_z) })],
        LasMutation::SetPointsByReturn(_) => vec![LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: base.header.points_by_return })],
        LasMutation::InsertVlr(insert_vlr::InsertVlr { index, .. }) => vec![LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: (*index).min(base.vlrs.len()) })],
        LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }) => match base.vlrs.get(*index) {
            Some(v) => vec![LasMutation::InsertVlr(insert_vlr::InsertVlr { index: *index, vlr: v.clone() })],
            None => Vec::new(),
        },
        LasMutation::SetVlrData(set_vlr_data::SetVlrData { index, .. }) => match base.vlrs.get(*index) {
            Some(v) => vec![LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: *index, data: v.data.clone() })],
            None => Vec::new(),
        },
        LasMutation::InsertPoint(insert_point::InsertPoint { index, .. }) => vec![LasMutation::RemovePoint(remove_point::RemovePoint { index: (*index).min(base.points.len()) })],
        LasMutation::RemovePoint(remove_point::RemovePoint { index }) => match base.points.get(*index) {
            Some(p) => vec![LasMutation::InsertPoint(insert_point::InsertPoint { index: *index, point: p.clone() })],
            None => Vec::new(),
        },
        LasMutation::SetPoint(set_point::SetPoint { index, .. }) => match base.points.get(*index) {
            Some(p) => vec![LasMutation::SetPoint(set_point::SetPoint { index: *index, point: p.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6 (las, recon-gap-fill): **hand-rolled** `OpText`/`OpBinary` for `LasMutation` — the
/// derive path (`#[derive(dsl::DslOps)]`) is NOT usable here. STEP 1 classification done for real
/// (attribute added, `cargo check -p semio-s-plugin-stdio --lib` run, real errors read, then
/// reverted): `LasMutation::SetScaleAndOffset`/`SetBounds` carry bare tuple fields
/// (`scale`/`offset`/`max`/`min`: `(f64, f64, f64)`) — real compiler output:
/// `error[E0277]: the trait bound `(f64, f64, f64): DslField` is not satisfied` (4 occurrences).
/// Root cause: no blanket `impl<..> DslField for (A, B, C)` exists in the `dsl` crate (same gap
/// class as `LasDiff`'s tri-state blocker — see that module's doc comment). `SetSnapshot`
/// independently fails too, transitively: `LasSnapshot` embeds `LasPoint::rgb: Option<(u16, u16,
/// u16)>`, the exact same bare-tuple gap. This gap is orthogonal to the `#[derive(dsl::MutationLeaf)]`
/// used by each leaf's payload struct below (a different derive, unaffected by it), so leaves keep
/// that derive while the AGGREGATE keeps this hand-rolled `OpText`/`OpBinary` pair.
///
/// **Grammar**: `keyword arg=value ...` (space-separated, one token per argument — no argument is
/// ever omitted since a mutation's `Option<T>` argument means "the new value", never a diff
/// tri-state, so no field is ever elided the way `LasDiff`'s sparse tokens are). Reuses every
/// primitive/value-codec the `🔺️diff` module already made `pub(crate)` for exactly this purpose
/// (`diff::hex_encode`/`diff::enc_vlr`/`diff::enc_point`/`diff::enc_u32x5`/etc. — same
/// intra-artifact reuse pattern svg's mutations module uses against its own diff module's
/// primitives). `encode_op`/`decode_op` = the text bytes verbatim, same simplification the
/// hand-rolled `DiffCodec` above uses.
//#region 🔖️SnapshotCodec
/// 📋 Whole-`LasHeader` positional codec — only needed by `SetSnapshot`'s `snapshot` argument (no
/// other variant carries a full header).
fn enc_header(h: &LasHeader) -> String {
    let fields: Vec<String> = vec![
        h.version_major.to_string(),
        h.version_minor.to_string(),
        diff::hex_encode(h.system_identifier.as_bytes()),
        diff::hex_encode(h.generating_software.as_bytes()),
        h.creation_day_of_year.to_string(),
        h.creation_year.to_string(),
        h.header_size.to_string(),
        h.offset_to_point_data.to_string(),
        h.number_of_vlrs.to_string(),
        h.point_data_format_id.to_string(),
        h.point_data_record_length.to_string(),
        h.number_of_point_records.to_string(),
        diff::enc_u32x5(&h.points_by_return),
        h.x_scale.to_string(),
        h.y_scale.to_string(),
        h.z_scale.to_string(),
        h.x_offset.to_string(),
        h.y_offset.to_string(),
        h.z_offset.to_string(),
        h.max_x.to_string(),
        h.min_x.to_string(),
        h.max_y.to_string(),
        h.min_y.to_string(),
        h.max_z.to_string(),
        h.min_z.to_string(),
    ];
    let las_field_separator = ",";
    format!("[{}]", fields.join(las_field_separator))
}
fn dec_header(s: &str) -> Result<LasHeader, String> {
    let parts = diff::split_top_level(diff::strip_brackets(s)?, ',');
    let [version_major, version_minor, system_identifier, generating_software, creation_day_of_year, creation_year, header_size, offset_to_point_data, number_of_vlrs, point_data_format_id, point_data_record_length, number_of_point_records, points_by_return, x_scale, y_scale, z_scale, x_offset, y_offset, z_offset, max_x, min_x, max_y, min_y, max_z, min_z] =
        parts.as_slice()
    else {
        return Err(format!("header: expected 25 fields, got {}", parts.len()));
    };
    Ok(LasHeader {
        version_major: diff::parse_u8(version_major)?,
        version_minor: diff::parse_u8(version_minor)?,
        system_identifier: String::from_utf8(diff::hex_decode(system_identifier)?).map_err(|e| e.to_string())?,
        generating_software: String::from_utf8(diff::hex_decode(generating_software)?).map_err(|e| e.to_string())?,
        creation_day_of_year: diff::parse_u16(creation_day_of_year)?,
        creation_year: diff::parse_u16(creation_year)?,
        header_size: diff::parse_u16(header_size)?,
        offset_to_point_data: diff::parse_u32(offset_to_point_data)?,
        number_of_vlrs: diff::parse_u32(number_of_vlrs)?,
        point_data_format_id: diff::parse_u8(point_data_format_id)?,
        point_data_record_length: diff::parse_u16(point_data_record_length)?,
        number_of_point_records: diff::parse_u32(number_of_point_records)?,
        points_by_return: diff::dec_u32x5(points_by_return)?,
        x_scale: diff::parse_f64(x_scale)?,
        y_scale: diff::parse_f64(y_scale)?,
        z_scale: diff::parse_f64(z_scale)?,
        x_offset: diff::parse_f64(x_offset)?,
        y_offset: diff::parse_f64(y_offset)?,
        z_offset: diff::parse_f64(z_offset)?,
        max_x: diff::parse_f64(max_x)?,
        min_x: diff::parse_f64(min_x)?,
        max_y: diff::parse_f64(max_y)?,
        min_y: diff::parse_f64(min_y)?,
        max_z: diff::parse_f64(max_z)?,
        min_z: diff::parse_f64(min_z)?,
    })
}
fn enc_snapshot(s: &LasSnapshot) -> String {
    let las_list_separator = ",";
    let vlrs = s.vlrs.iter().map(diff::enc_vlr).collect::<Vec<_>>().join(las_list_separator);
    let points = s.points.iter().map(diff::enc_point).collect::<Vec<_>>().join(las_list_separator);
    format!("[{},[{}],[{}]]", enc_header(&s.header), vlrs, points)
}
fn dec_snapshot(s: &str) -> Result<LasSnapshot, String> {
    let inner = diff::strip_brackets(s)?;
    let parts = diff::split_top_level(inner, ',');
    let [header_s, vlrs_s, points_s] = parts.as_slice() else {
        return Err(format!("snapshot: expected 3 top-level fields, got {}", parts.len()));
    };
    let header = dec_header(header_s)?;
    let vlrs = diff::split_top_level(diff::strip_brackets(vlrs_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(diff::dec_vlr).collect::<Result<Vec<_>, String>>()?;
    let points = diff::split_top_level(diff::strip_brackets(points_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(diff::dec_point).collect::<Result<Vec<_>, String>>()?;
    Ok(LasSnapshot { schema: crate::STDIO_LAS_DOCUMENT_SCHEMA.into(), header, vlrs, points })
}
//#endregion 🔖️SnapshotCodec

//#region 🔖️TupleCodec
fn enc_f64x3(t: &(f64, f64, f64)) -> String {
    format!("[{},{},{}]", t.0, t.1, t.2)
}
fn dec_f64x3(s: &str) -> Result<(f64, f64, f64), String> {
    let parts = diff::split_top_level(diff::strip_brackets(s)?, ',');
    let [a, b, c] = parts.as_slice() else { return Err(format!("f64x3: expected 3 fields, got {}", parts.len())) };
    Ok((diff::parse_f64(a)?, diff::parse_f64(b)?, diff::parse_f64(c)?))
}
//#endregion 🔖️TupleCodec

//#region 🔖️TopLevel
fn print_las_mutation(m: &LasMutation) -> String {
    match m {
        LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        LasMutation::SetVersion(set_version::SetVersion { major, minor }) => format!("set-version major={major} minor={minor}"),
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier }) => format!("set-system-identifier system-identifier={}", diff::hex_encode(system_identifier.as_bytes())),
        LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software }) => format!("set-software-info generating-software={}", diff::hex_encode(generating_software.as_bytes())),
        LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year, year }) => format!("set-creation-date day-of-year={day_of_year} year={year}"),
        LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale, offset }) => format!("set-scale-and-offset scale={} offset={}", enc_f64x3(scale), enc_f64x3(offset)),
        LasMutation::SetBounds(set_bounds::SetBounds { max, min }) => format!("set-bounds max={} min={}", enc_f64x3(max), enc_f64x3(min)),
        LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts }) => format!("set-points-by-return counts={}", diff::enc_u32x5(counts)),
        LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr }) => format!("insert-vlr index={index} vlr={}", diff::enc_vlr(vlr)),
        LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }) => format!("remove-vlr index={index}"),
        LasMutation::SetVlrData(set_vlr_data::SetVlrData { index, data }) => format!("set-vlr-data index={index} data={}", diff::hex_encode(data)),
        LasMutation::InsertPoint(insert_point::InsertPoint { index, point }) => format!("insert-point index={index} point={}", diff::enc_point(point)),
        LasMutation::RemovePoint(remove_point::RemovePoint { index }) => format!("remove-point index={index}"),
        LasMutation::SetPoint(set_point::SetPoint { index, point }) => format!("set-point index={index} point={}", diff::enc_point(point)),
    }
}
fn parse_las_mutation(line: &str) -> Result<LasMutation, String> {
    let mut tokens = line.split(' ');
    let keyword = tokens.next().filter(|k| !k.is_empty()).ok_or_else(|| "empty mutation line".to_string())?;
    let rest: Vec<&str> = tokens.collect();
    let arg = |key: &str| -> Result<&str, String> { rest.iter().find_map(|t| t.strip_prefix(key)).ok_or_else(|| format!("{keyword}: missing arg {key:?}")) };
    match keyword {
        "set-snapshot" => Ok(LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_snapshot(arg("snapshot=")?)? })),
        "set-version" => Ok(LasMutation::SetVersion(set_version::SetVersion { major: diff::parse_u8(arg("major=")?)?, minor: diff::parse_u8(arg("minor=")?)? })),
        "set-system-identifier" => Ok(LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: String::from_utf8(diff::hex_decode(arg("system-identifier=")?)?).map_err(|e| e.to_string())? })),
        "set-software-info" => Ok(LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: String::from_utf8(diff::hex_decode(arg("generating-software=")?)?).map_err(|e| e.to_string())? })),
        "set-creation-date" => Ok(LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: diff::parse_u16(arg("day-of-year=")?)?, year: diff::parse_u16(arg("year=")?)? })),
        "set-scale-and-offset" => Ok(LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: dec_f64x3(arg("scale=")?)?, offset: dec_f64x3(arg("offset=")?)? })),
        "set-bounds" => Ok(LasMutation::SetBounds(set_bounds::SetBounds { max: dec_f64x3(arg("max=")?)?, min: dec_f64x3(arg("min=")?)? })),
        "set-points-by-return" => Ok(LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: diff::dec_u32x5(arg("counts=")?)? })),
        "insert-vlr" => Ok(LasMutation::InsertVlr(insert_vlr::InsertVlr { index: diff::parse_usize(arg("index=")?)?, vlr: diff::dec_vlr(arg("vlr=")?)? })),
        "remove-vlr" => Ok(LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: diff::parse_usize(arg("index=")?)? })),
        "set-vlr-data" => Ok(LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: diff::parse_usize(arg("index=")?)?, data: diff::hex_decode(arg("data=")?)? })),
        "insert-point" => Ok(LasMutation::InsertPoint(insert_point::InsertPoint { index: diff::parse_usize(arg("index=")?)?, point: diff::dec_point(arg("point=")?)? })),
        "remove-point" => Ok(LasMutation::RemovePoint(remove_point::RemovePoint { index: diff::parse_usize(arg("index=")?)? })),
        "set-point" => Ok(LasMutation::SetPoint(set_point::SetPoint { index: diff::parse_usize(arg("index=")?)?, point: diff::dec_point(arg("point=")?)? })),
        other => Err(format!("las mutation: unknown keyword {other:?}")),
    }
}
//#endregion 🔖️TopLevel

impl protocol::OpText for LasMutation {
    fn print_op(&self) -> String {
        print_las_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_las_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️BinaryOpCodec
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: REAL binary
/// twins backing the upgraded `OpBinary::encode_op`/`decode_op` below — replaces the old F6
/// `print_las_mutation(self).into_bytes()` text-as-binary shortcut. Reuses the diff facet's own
/// `write_bytes_lp`/`write_str_lp`/`enc_header_bin`/`enc_vlr_bin`/`enc_point_bin` primitives
/// (`../🔺️diff/🦀️.rs`'s `#region 🔖️BinaryDiffCodec`, `pub(crate)`) — `LasHeader`/
/// `LasVlr`/`LasPoint` are the SAME real records whether embedded in a sparse diff-patch or (here)
/// a whole `SetSnapshot`/`InsertVlr`/`InsertPoint`/`SetPoint` payload, so one binary encoder per
/// record type, shared across both facets, is the correct de-duplication (not a second,
/// independently-drifting copy).
fn enc_f64x3_bin(t: (f64, f64, f64), out: &mut Vec<u8>) {
    out.extend_from_slice(&t.0.to_le_bytes());
    out.extend_from_slice(&t.1.to_le_bytes());
    out.extend_from_slice(&t.2.to_le_bytes());
}
fn dec_f64x3_bin(reader: &mut store::ByteReader<'_>) -> Result<(f64, f64, f64), String> {
    Ok((reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?))
}

/// 🧭️ A whole `LasSnapshot` — `schema` (real, genuinely round-tripped identity field) + the full
/// `LasHeader` record + runtime-counted `vlrs`/`points` lists, each item a full record.
fn enc_snapshot_bin(s: &LasSnapshot, out: &mut Vec<u8>) {
    diff::write_str_lp(out, &s.schema);
    diff::enc_header_bin(&s.header, out);
    store::pack_rt::write_varint_u64(out, s.vlrs.len() as u64);
    for v in &s.vlrs {
        diff::enc_vlr_bin(v, out);
    }
    store::pack_rt::write_varint_u64(out, s.points.len() as u64);
    for p in &s.points {
        diff::enc_point_bin(p, out);
    }
}
fn dec_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<LasSnapshot, String> {
    let schema = diff::read_str_lp(reader)?;
    let header = diff::dec_header_bin(reader)?;
    let vlr_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let vlrs = (0..vlr_count).map(|_| diff::dec_vlr_bin(reader)).collect::<Result<Vec<_>, String>>()?;
    let point_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let points = (0..point_count).map(|_| diff::dec_point_bin(reader)).collect::<Result<Vec<_>, String>>()?;
    Ok(LasSnapshot { schema, header, vlrs, points })
}

/// 🔢️ Variant tag byte — declaration order, matching `LasMutation`'s own enum order exactly.
const TAG_SET_SNAPSHOT: u8 = 0;
const TAG_SET_VERSION: u8 = 1;
const TAG_SET_SYSTEM_IDENTIFIER: u8 = 2;
const TAG_SET_SOFTWARE_INFO: u8 = 3;
const TAG_SET_CREATION_DATE: u8 = 4;
const TAG_SET_SCALE_AND_OFFSET: u8 = 5;
const TAG_SET_BOUNDS: u8 = 6;
const TAG_SET_POINTS_BY_RETURN: u8 = 7;
const TAG_INSERT_VLR: u8 = 8;
const TAG_REMOVE_VLR: u8 = 9;
const TAG_SET_VLR_DATA: u8 = 10;
const TAG_INSERT_POINT: u8 = 11;
const TAG_REMOVE_POINT: u8 = 12;
const TAG_SET_POINT: u8 = 13;
//#endregion 🔖️BinaryOpCodec

impl protocol::OpBinary for LasMutation {
    /// ⚡️ REAL binary frame (`format u8 | tag u8 | <variant-specific fields>`), matching
    /// `../💾️binary/📡️.protocol.semio`'s `format`/`tag` leading fields exactly —
    /// upgraded from F6's `print_las_mutation(self).into_bytes()` text-as-binary shortcut. Every
    /// variant's payload is genuinely, individually field-by-field encoded below (see
    /// `#region 🔖️BinaryOpCodec` for the shared record encoders).
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        match self {
            LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => {
                out.push(TAG_SET_SNAPSHOT);
                enc_snapshot_bin(snapshot, &mut out);
            }
            LasMutation::SetVersion(set_version::SetVersion { major, minor }) => {
                out.push(TAG_SET_VERSION);
                out.push(*major);
                out.push(*minor);
            }
            LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier }) => {
                out.push(TAG_SET_SYSTEM_IDENTIFIER);
                diff::write_str_lp(&mut out, system_identifier);
            }
            LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software }) => {
                out.push(TAG_SET_SOFTWARE_INFO);
                diff::write_str_lp(&mut out, generating_software);
            }
            LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year, year }) => {
                out.push(TAG_SET_CREATION_DATE);
                store::pack_rt::write_varint_u64(&mut out, *day_of_year as u64);
                store::pack_rt::write_varint_u64(&mut out, *year as u64);
            }
            LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale, offset }) => {
                out.push(TAG_SET_SCALE_AND_OFFSET);
                enc_f64x3_bin(*scale, &mut out);
                enc_f64x3_bin(*offset, &mut out);
            }
            LasMutation::SetBounds(set_bounds::SetBounds { max, min }) => {
                out.push(TAG_SET_BOUNDS);
                enc_f64x3_bin(*max, &mut out);
                enc_f64x3_bin(*min, &mut out);
            }
            LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts }) => {
                out.push(TAG_SET_POINTS_BY_RETURN);
                for c in counts {
                    store::pack_rt::write_varint_u64(&mut out, *c as u64);
                }
            }
            LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr }) => {
                out.push(TAG_INSERT_VLR);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_vlr_bin(vlr, &mut out);
            }
            LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }) => {
                out.push(TAG_REMOVE_VLR);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            LasMutation::SetVlrData(set_vlr_data::SetVlrData { index, data }) => {
                out.push(TAG_SET_VLR_DATA);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::write_bytes_lp(&mut out, data);
            }
            LasMutation::InsertPoint(insert_point::InsertPoint { index, point }) => {
                out.push(TAG_INSERT_POINT);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_point_bin(point, &mut out);
            }
            LasMutation::RemovePoint(remove_point::RemovePoint { index }) => {
                out.push(TAG_REMOVE_POINT);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            LasMutation::SetPoint(set_point::SetPoint { index, point }) => {
                out.push(TAG_SET_POINT);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_point_bin(point, &mut out);
            }
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        fn go(bytes: &[u8]) -> Result<LasMutation, String> {
            let mut reader = store::ByteReader::new(bytes);
            let format = reader.read_u8().map_err(|e| e.to_string())?;
            if format != store::pack_rt::OP_BINARY_FORMAT {
                return Err(format!("bad op format byte {format}"));
            }
            let tag = reader.read_u8().map_err(|e| e.to_string())?;
            Ok(match tag {
                TAG_SET_SNAPSHOT => LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_snapshot_bin(&mut reader)? }),
                TAG_SET_VERSION => LasMutation::SetVersion(set_version::SetVersion { major: reader.read_u8().map_err(|e| e.to_string())?, minor: reader.read_u8().map_err(|e| e.to_string())? }),
                TAG_SET_SYSTEM_IDENTIFIER => LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: diff::read_str_lp(&mut reader)? }),
                TAG_SET_SOFTWARE_INFO => LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: diff::read_str_lp(&mut reader)? }),
                TAG_SET_CREATION_DATE => LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: reader.read_varint_u64().map_err(|e| e.to_string())? as u16, year: reader.read_varint_u64().map_err(|e| e.to_string())? as u16 }),
                TAG_SET_SCALE_AND_OFFSET => LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: dec_f64x3_bin(&mut reader)?, offset: dec_f64x3_bin(&mut reader)? }),
                TAG_SET_BOUNDS => LasMutation::SetBounds(set_bounds::SetBounds { max: dec_f64x3_bin(&mut reader)?, min: dec_f64x3_bin(&mut reader)? }),
                TAG_SET_POINTS_BY_RETURN => {
                    let mut counts = [0u32; 5];
                    for slot in counts.iter_mut() {
                        *slot = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
                    }
                    LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts })
                }
                TAG_INSERT_VLR => LasMutation::InsertVlr(insert_vlr::InsertVlr { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, vlr: diff::dec_vlr_bin(&mut reader)? }),
                TAG_REMOVE_VLR => LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
                TAG_SET_VLR_DATA => LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, data: diff::read_bytes_lp(&mut reader)? }),
                TAG_INSERT_POINT => LasMutation::InsertPoint(insert_point::InsertPoint { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, point: diff::dec_point_bin(&mut reader)? }),
                TAG_REMOVE_POINT => LasMutation::RemovePoint(remove_point::RemovePoint { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize }),
                TAG_SET_POINT => LasMutation::SetPoint(set_point::SetPoint { index: reader.read_varint_u64().map_err(|e| e.to_string())? as usize, point: diff::dec_point_bin(&mut reader)? }),
                other => return Err(format!("las mutation: unknown binary tag {other}")),
            })
        }
        go(bytes).map_err(|e| protocol::ProtocolError::Malformed { what: "las op binary", offset: 0, detail: e })
    }
}
//#endregion OpCodecs

//#region 🔖️SharedFixtures
/// 🧪️ Moved out of `mod tests` (was originally local to it) so `demo_mutation_cases()` below can
/// share the exact same fixtures `mod tests` itself uses — single source of truth, per CLAUDE.md.
#[cfg(test)]
pub(crate) fn vlr(user_id: &str, record_id: u16, data: &[u8]) -> LasVlr {
    LasVlr { user_id: user_id.into(), record_id, description: format!("vlr {record_id}"), data: data.to_vec() }
}

#[cfg(test)]
pub(crate) fn point(seed: u8) -> LasPoint {
    LasPoint {
        x: 100.0 + seed as f64,
        y: -50.0 + seed as f64 * 0.5,
        z: 10.0 + seed as f64 * 0.1,
        intensity: 100 + seed as u16,
        return_number: (seed % 5) + 1,
        number_of_returns: ((seed + 1) % 5) + 1,
        scan_direction_flag: seed % 2 == 0,
        edge_of_flight_line: seed % 3 == 0,
        classification: seed,
        scan_angle_rank: seed as i8 - 10,
        user_data: seed,
        point_source_id: 1000 + seed as u16,
        gps_time: None,
        rgb: None,
    }
}

#[cfg(test)]
pub(crate) fn base_snapshot() -> LasSnapshot {
    let vlrs = vec![vlr("LASF_Spec", 100, b"vlr-a"), vlr("LASF_Spec", 101, b"vlr-b")];
    let points = vec![point(0), point(1), point(2)];
    LasSnapshot { schema: "stdio.las".into(), header: LasHeader { number_of_vlrs: vlrs.len() as u32, number_of_point_records: points.len() as u32, ..LasHeader::default() }, vlrs, points }
}

/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: representative
/// `LasMutation` cases, one per variant (14 total) — single source of truth shared by
/// `op_text_binary_roundtrip_law` below AND `⚙️engine/🦀️.rs`'s
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests, per CLAUDE.md (no
/// duplicated literal case lists). Exercises `SetSnapshot` (whole-header + vlrs + points
/// positional codec), both bare-tuple variants (`SetScaleAndOffset`/`SetBounds`), the `[u32; 5]`
/// array (`SetPointsByReturn`), and a point/VLR carrying both tri-state-capable fields set
/// (`gps_time`/`rgb`).
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<LasMutation> {
    let base = base_snapshot();
    let mut rich_point = point(9);
    rich_point.gps_time = Some(1234.5);
    rich_point.rgb = Some((11, 22, 33));
    vec![
        LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: LasSnapshot { header: LasHeader { creation_year: 2031, ..base.header.clone() }, ..base.clone() } }),
        LasMutation::SetVersion(set_version::SetVersion { major: 1, minor: 4 }),
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: "semio".into() }),
        LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: "semio-las-writer".into() }),
        LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: 42, year: 2026 }),
        LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: (0.001, 0.001, 0.001), offset: (1000.0, 2000.0, 0.0) }),
        LasMutation::SetBounds(set_bounds::SetBounds { max: (999.0, 888.0, 777.0), min: (-1.0, -2.0, -3.0) }),
        LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: [1, 2, 3, 4, 5] }),
        LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 1, vlr: vlr("EXTRA", 200, b"new-vlr") }),
        LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: 0 }),
        LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: 0, data: b"patched".to_vec() }),
        LasMutation::InsertPoint(insert_point::InsertPoint { index: 1, point: rich_point.clone() }),
        LasMutation::RemovePoint(remove_point::RemovePoint { index: 0 }),
        LasMutation::SetPoint(set_point::SetPoint { index: 0, point: rich_point }),
    ]
}
//#endregion 🔖️SharedFixtures

//#region Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion Tests

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
