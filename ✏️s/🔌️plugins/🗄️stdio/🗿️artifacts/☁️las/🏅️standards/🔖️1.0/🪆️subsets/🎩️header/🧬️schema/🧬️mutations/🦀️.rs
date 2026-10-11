//! 🧬️ LasMutation — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: real vocabulary: header
//! fields grouped sensibly
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
#[path = "➕insert-point/🦀️.rs"]
pub mod insert_point;
#[path = "📥insert-vlr/🦀️.rs"]
pub mod insert_vlr;
#[path = "➖remove-point/🦀️.rs"]
pub mod remove_point;
#[path = "📤remove-vlr/🦀️.rs"]
pub mod remove_vlr;
#[path = "📦set-bounds/🦀️.rs"]
pub mod set_bounds;
#[path = "🕰️set-creation-date/🦀️.rs"]
pub mod set_creation_date;
#[path = "✏️set-point/🦀️.rs"]
pub mod set_point;
#[path = "🔁set-points-by-return/🦀️.rs"]
pub mod set_points_by_return;
#[path = "📏set-scale-and-offset/🦀️.rs"]
pub mod set_scale_and_offset;
/// 📐️ Typed content mutation for `stdio.las`.
//#region 🔖️Leaves
#[path = "🛠️set-software-info/🦀️.rs"]
pub mod set_software_info;
#[path = "🏢set-system-identifier/🦀️.rs"]
pub mod set_system_identifier;
#[path = "🔢set-version/🦀️.rs"]
pub mod set_version;
#[path = "🗃️set-vlr-data/🦀️.rs"]
pub mod set_vlr_data;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = LasSnapshot, diff = LasDiff, schema = "s.stdio.las")]
pub enum LasMutation {
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


//#endregion 🔖️Apply


//#endregion 🔖️MutationTrait


//#region OpCodecs
/// 🧪️ F6 (las, recon-gap-fill): **hand-rolled** `OpText`/`OpBinary` for `LasMutation` — the
/// derive path (`#[derive(semio_framework_dsl_record_derive::DslEnum)]`) is NOT usable here. STEP 1 classification done for real
/// (attribute added, `cargo check -p semio-s-plugin-stdio --lib` run, real errors read, then
/// reverted): `LasMutation::SetScaleAndOffset`/`SetBounds` carry bare tuple fields
/// (`scale`/`offset`/`max`/`min`: `(f64, f64, f64)`) — real compiler output:
/// `error[E0277]: the trait bound `(f64, f64, f64): DslField` is not satisfied` (4 occurrences).
/// Root cause: no blanket `impl<..> DslField for (A, B, C)` exists in the `dsl` crate (same gap
/// class as `LasDiff`'s tri-state blocker — see that module's doc comment). `InsertPoint`/`SetPoint`
/// fail too, transitively: `LasPoint::rgb` is `Option<(u16, u16, u16)>`, the exact same bare-tuple gap. This gap is orthogonal to the `#[derive(dsl::MutationLeaf)]`
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
/// hand-rolled `DiffBinary,DiffCodec,DiffText` above uses.
//#region 🔖️SnapshotCodec




//#endregion 🔖️SnapshotCodec

//#region 🔖️TupleCodec


//#endregion 🔖️TupleCodec

//#region 🔖️TopLevel


//#endregion 🔖️TopLevel



//#region 🔖️BinaryOpCodec







//#endregion 🔖️BinaryOpCodec


//#endregion OpCodecs

//#region 🔖️SharedFixtures




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
/// duplicated literal case lists). Exercises both bare-tuple variants (`SetScaleAndOffset`/`SetBounds`), the `[u32; 5]`
/// array (`SetPointsByReturn`), and a point/VLR carrying both tri-state-capable fields set
/// (`gps_time`/`rgb`).
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<LasMutation> {
    let mut rich_point = point(9);
    rich_point.gps_time = Some(1234.5);
    rich_point.rgb = Some((11, 22, 33));
    vec![
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

/// 📍️ A point as the decoder yields it under the default header (`scale 0.01`, `offset 0`):
/// every coordinate is `record * scale + offset` of an integer record, so a scale/offset edit
/// re-reads the same records and its inverse lands on the identical `f64`s.
#[cfg(test)]
pub(crate) fn point(seed: u8) -> LasPoint {
    let step = i32::from(seed);
    LasPoint {
        x: (10_000 + step * 100) as f64 * 0.01 + 0.0,
        y: (-5_000 + step * 50) as f64 * 0.01 + 0.0,
        z: (1_000 + step * 10) as f64 * 0.01 + 0.0,
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
/// 🧪️ Moved out of `mod tests` (was originally local to it) so `demo_mutation_cases()` below can
/// share the exact same fixtures `mod tests` itself uses — single source of truth, per CLAUDE.md.
#[cfg(test)]
pub(crate) fn vlr(user_id: &str, record_id: u16, data: &[u8]) -> LasVlr {
    LasVlr { user_id: user_id.into(), record_id, description: format!("vlr {record_id}"), data: data.to_vec() }
}
