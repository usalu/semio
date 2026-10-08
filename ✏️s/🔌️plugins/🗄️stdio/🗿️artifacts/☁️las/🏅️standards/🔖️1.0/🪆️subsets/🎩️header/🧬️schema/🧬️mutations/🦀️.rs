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

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
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

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`, returning a typed error outcome without changing the
/// snapshot when an index target is missing or out of range. `InsertVlr`/`RemoveVlr`/
/// `InsertPoint`/`RemovePoint` keep `header.number_of_vlrs`/
/// `header.number_of_point_records` in sync with the real collection length (`engine::encode_las`
/// also independently recomputes both at encode time — see `LasHeader`'s doc comment — so this
/// sync is a snapshot-level consistency guarantee, not the sole source of correctness).
pub fn apply_las_mutation(snapshot: &mut LasSnapshot, mutation: &LasMutation) -> protocol::MutationOutcome<LasDiff> {
    let outcome = <LasMutation as Mutation<LasSnapshot>>::diff(mutation, snapshot);
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
/// 🧮️ The leaves that carry `base` to exactly `next`: the header groups that moved, then each VLR and point row in place (its
/// data alone via `set-vlr-data`, a whole point via `set-point`, any other VLR change as remove-then-insert), then the diverging
/// tails. The snapshot `schema` is a constant of the artifact; header fields the collections keep in sync are never leaves.
pub fn net_mutations(base: &LasSnapshot, next: &LasSnapshot) -> Vec<LasMutation> {
    let (old, new) = (&base.header, &next.header);
    let mut leaves = Vec::new();
    if (old.version_major, old.version_minor) != (new.version_major, new.version_minor) {
        leaves.push(LasMutation::SetVersion(set_version::SetVersion { major: new.version_major, minor: new.version_minor }));
    }
    if old.system_identifier != new.system_identifier {
        leaves.push(LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: new.system_identifier.clone() }));
    }
    if old.generating_software != new.generating_software {
        leaves.push(LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: new.generating_software.clone() }));
    }
    if (old.creation_day_of_year, old.creation_year) != (new.creation_day_of_year, new.creation_year) {
        leaves.push(LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: new.creation_day_of_year, year: new.creation_year }));
    }
    let scale_and_offset = |header: &LasHeader| ((header.x_scale, header.y_scale, header.z_scale), (header.x_offset, header.y_offset, header.z_offset));
    if scale_and_offset(old) != scale_and_offset(new) {
        let (scale, offset) = scale_and_offset(new);
        leaves.push(LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale, offset }));
    }
    let bounds = |header: &LasHeader| ((header.max_x, header.max_y, header.max_z), (header.min_x, header.min_y, header.min_z));
    if bounds(old) != bounds(new) {
        let (max, min) = bounds(new);
        leaves.push(LasMutation::SetBounds(set_bounds::SetBounds { max, min }));
    }
    if old.points_by_return != new.points_by_return {
        leaves.push(LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: new.points_by_return }));
    }
    let paired = base.vlrs.len().min(next.vlrs.len());
    for (index, (before, after)) in base.vlrs.iter().zip(&next.vlrs).enumerate().filter(|(_, (before, after))| before != after) {
        if (&before.user_id, before.record_id, &before.description) == (&after.user_id, after.record_id, &after.description) {
            leaves.push(LasMutation::SetVlrData(set_vlr_data::SetVlrData { index, data: after.data.clone() }));
        } else {
            leaves.push(LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index }));
            leaves.push(LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr: after.clone() }));
        }
    }
    leaves.extend((paired..base.vlrs.len()).rev().map(|index| LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index })));
    leaves.extend(next.vlrs.iter().enumerate().skip(paired).map(|(index, vlr)| LasMutation::InsertVlr(insert_vlr::InsertVlr { index, vlr: vlr.clone() })));
    let paired = base.points.len().min(next.points.len());
    leaves.extend(base.points.iter().zip(&next.points).enumerate().filter(|(_, (before, after))| before != after).map(|(index, (_, after))| LasMutation::SetPoint(set_point::SetPoint { index, point: after.clone() })));
    leaves.extend((paired..base.points.len()).rev().map(|index| LasMutation::RemovePoint(remove_point::RemovePoint { index })));
    leaves.extend(next.points.iter().enumerate().skip(paired).map(|(index, point)| LasMutation::InsertPoint(insert_point::InsertPoint { index, point: point.clone() })));
    leaves
}
//#endregion 🔖️Net

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
