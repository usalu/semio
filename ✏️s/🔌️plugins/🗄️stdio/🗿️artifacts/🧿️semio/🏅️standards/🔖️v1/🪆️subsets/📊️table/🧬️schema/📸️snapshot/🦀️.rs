//! 🧬️ SemioTableSnapshot — the neutral tabular interchange shape: named/typed columns plus rows
//! of scalar cells (what csv/tsv/xlsx eventually map onto). LEAF subset (no child slots, no link
//! slots) per the master plan's stdio target vocabulary (ticket UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM,
//! W2b/table).
//!
//! Modeled directly on `🔤️text`'s hand-rolled `ArtifactDsl`/`ArtifactPack` convention (real
//! hex/bracket text codec + real varint-length-prefixed binary codec, both wrapped in the shared
//! `store::semio_format` envelope). Cell VALUES reuse `🔢️value`'s `SemioValue` scalar vocabulary
//! directly (`SemioTableRow.cells: Vec<SemioValue>`) rather than re-deriving a second scalar type —
//! `SemioTableCellKind` only mirrors `SemioValue`'s scalar variant NAMES as a declared column-type
//! tag (`Null`/`Bool`/`Int`/`Float`/`Str`/`Bytes` — no `List`/`Map`/`Ref`, a column's cells are
//! meant to stay scalar).
//!
//! CRITICAL INVARIANT: `rows[i].cells` is POSITIONALLY ALIGNED with `columns` (`cells[j]` belongs
//! to `columns[j]`). Every column insert/remove/reorder mutation applies the IDENTICAL
//! insert/remove/reorder, at the IDENTICAL index, to every row's `cells` — see
//! `🧬️mutations/🏗️create-column`/`🗑️delete-column`/`🔀reorder-columns`.









use crate::standards::v1::subsets::value::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;

//#region 🔖️Ids
/// 🏷️ Document schema / DSL envelope id AND `ArtifactSchema` descriptor id — same literal for
/// both, per the master plan's "Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`"
/// convention, one per subset.
pub const STDIO_SEMIOTABLE_DOCUMENT_SCHEMA: &str = "s.stdio.semio.table";
//#endregion 🔖️Ids

//#region 🔖️CellKind
/// 🏷️ The declared column-type tag — mirrors `SemioValue`'s SCALAR variant names only (`Null`/
/// `Bool`/`Int`/`Float`/`Str`/`Bytes`; no `List`/`Map`/`Ref` — a column's cells are meant to be
/// scalar). Text tags: `n`/`b`/`i`/`f`/`s`/`y`. Binary tags: `0`-`5` in declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub enum SemioTableCellKind {
    #[default]
    Null,
    Bool,
    Int,
    Float,
    Str,
    Bytes,
}
//#endregion 🔖️CellKind

//#region 🔖️Column
/// 🏛️ One declared column: `name` is the NATIVE KEY (name-keyed collection — like cad layers/xlsx
/// sheets, `📓️taxonomy.md`'s addressing rule #2) plus its declared `kind` tag.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct SemioTableColumn {
    pub name: String,
    pub kind: SemioTableCellKind,
}
//#endregion 🔖️Column

//#region 🔖️Row
/// 🧾️ One row: `cells` is POSITIONALLY ALIGNED with `columns` (see this module's own doc comment
/// for the invariant). Rows themselves are index-addressed (no stable id — an intrinsically
/// ordered, anonymous collection, `📓️taxonomy.md` addressing rule #3), the same shape
/// `insert-row`/`remove-row`/`reorder-rows` operate on. `SemioValue` (from `🔢️value`) is reused
/// verbatim for cell data — real reuse, not reinvention.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, Default)]
#[value(rename_all = "camelCase")]
pub struct SemioTableRow {
    #[value(default)]
    pub cells: Vec<SemioValue>,
}
//#endregion 🔖️Row

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.table")]
pub struct SemioTableSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub columns: Vec<SemioTableColumn>,
    #[state(artifact)]
    #[value(default)]
    pub rows: Vec<SemioTableRow>,
}

impl Default for SemioTableSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(), columns: Vec::new(), rows: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TablePrimitives












//#endregion 🔖️TablePrimitives

//#region 🔖️BinaryPrimitives







//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge



//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Wire







//#endregion 🔖️Wire

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.table` document — three columns (`label: Str`, `score: Float`,
/// `active: Bool`) across three rows, exercising every `SemioTableCellKind`/`SemioValue` scalar
/// variant at least once (`Str`/`Float`/`Bool` via the declared column kinds; `Null`/`Int`/`Bytes`
/// via cell values — a cell's actual `SemioValue` kind is independent of its column's declared
/// tag, no runtime enforcement, matching a lenient real-world tabular format). Single source of
/// truth for `📚️examples/📃️sheet/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and for the
/// conformance-law tests in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_table_snapshot() -> SemioTableSnapshot {
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![SemioTableColumn { name: "label".into(), kind: SemioTableCellKind::Str }, SemioTableColumn { name: "score".into(), kind: SemioTableCellKind::Float }, SemioTableColumn { name: "active".into(), kind: SemioTableCellKind::Bool }],
        rows: vec![
            SemioTableRow { cells: vec![SemioValue::Str { value: "widget".into() }, SemioValue::Float { lexeme: "3.500".into() }, SemioValue::Bool { value: true }] },
            SemioTableRow { cells: vec![SemioValue::Null, SemioValue::Int { lexeme: "42".into() }, SemioValue::Bytes { value: vec![0, 1, 2, 255] }] },
            SemioTableRow { cells: vec![SemioValue::Str { value: "gadget".into() }, SemioValue::Float { lexeme: "1.250".into() }, SemioValue::Bool { value: false }] },
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests







