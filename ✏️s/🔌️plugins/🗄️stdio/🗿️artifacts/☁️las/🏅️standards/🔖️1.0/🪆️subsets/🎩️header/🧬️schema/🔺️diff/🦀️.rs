//! 🔺️ LasDiff — handcrafted sparse diff. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: replaces the old
//! `LasDiff{snapshot: Option<LasSnapshot>}` full-replace template with every real header field as
//! a top-level `Option<T>` scalar plus an index-keyed `vlrs` triple and an index-keyed `points`
//! triple, each entity individually patchable.

/// 🧩 Ordered removed keys, modified values, and inserted items.
pub(crate) type IndexedDiffParts<D, T> = (Vec<usize>, Vec<(usize, D)>, Vec<(usize, T)>);

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::schema::snapshot::{LasHeader, LasPoint, LasVlr};
use crate::LasSnapshot;
use protocol::command::DiffAlgebra;
#[cfg(test)] use protocol::{DiffCodec};
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};

#[path = "🧮️indexed-triple-algebra/🦀️.rs"]
mod indexed_triple_algebra;
use indexed_triple_algebra::*;

#[path = "🧱️vlr-diff/🦀️.rs"]
mod vlr_diff;
pub use vlr_diff::*;

#[path = "📍️point-diff/🦀️.rs"]
mod point_diff;
pub use point_diff::*;

#[path = "🔄️diff-algebra-and-builders/🦀️.rs"]
mod diff_algebra_and_builders;
pub use diff_algebra_and_builders::*;

use framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.las`. Every real header field is a top-level scalar; `schema` is an
/// identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.las.diff")]
pub struct LasDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub version_major: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub version_minor: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub system_identifier: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub generating_software: Option<String>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_day_of_year: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub creation_year: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_size: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub offset_to_point_data: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number_of_vlrs: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub point_data_format_id: Option<u8>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub point_data_record_length: Option<u16>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub number_of_point_records: Option<u32>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub points_by_return: Option<[u32; 5]>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x_scale: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y_scale: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z_scale: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x_offset: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y_offset: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub z_offset: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_x: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_x: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_y: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_y: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub max_z: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub min_z: Option<f64>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vlrs: Option<LasVlrsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<LasPointsDiff>,
}

//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6 (las, recon-gap-fill — this artifact was MISSED by the F6 recon sweep's §8
/// classification table entirely): **hand-rolled** `protocol::DiffCodec` for `LasDiff` — the
/// derive path (`#[derive(dsl::DslDiff)]`) is NOT usable here. STEP 1 classification done for
/// real (attribute added, `cargo check -p semio-s-plugin-stdio --lib` run, real errors read, then
/// reverted): two independent, confirmed blockers —
///
/// 1. **3b (tri-state)**, the recon's documented rule: `LasPointDiff::gps_time: Option<Option<f64>>`
///    and `LasPointDiff::rgb: Option<Option<(u16, u16, u16)>>` — real compiler output:
///    `error[E0277]: the trait bound `std::option::Option<f64>: DslField` is not satisfied` at
///    `🔺️diff/component.rs:311` (`pub gps_time: Option<Option<f64>>`) and
///    `error[E0277]: the trait bound `std::option::Option<(u16, u16, u16)>: DslField` is not
///    satisfied` at `🔺️diff/component.rs:313` (`pub rgb: Option<Option<(u16, u16, u16)>>`).
/// 2. **A THIRD blocker not named by the recon's 3a/3b taxonomy**: `LasPoint::rgb`'s inner type is
///    a bare tuple `(u16, u16, u16)`, and — same root cause as 3b's missing
///    `impl<T: DslField> DslField for Option<T>` — there is no blanket `impl<A: DslField, B: DslField,
///    ...> DslField for (A, B, ...)` anywhere in the `dsl` crate either (confirmed by the SAME
///    compiler error above: even a single-layer `Option<(u16, u16, u16)>` would fail to bind, tri-state
///    or not). The Mutation side hits this independently and more directly: `LasMutation::SetScaleAndOffset`/
///    `SetBounds` carry bare `(f64, f64, f64)` fields — confirmed via a SEPARATE real `#[derive(semio_framework_dsl_record_derive::DslEnum)]`
///    probe on `LasMutation`: `error[E0277]: the trait bound `(f64, f64, f64): DslField` is not satisfied`
///    (4 occurrences, `scale`/`offset`/`max`/`min`). Both `LasDiff` and `LasMutation` are hand-rolled.
///
/// **Grammar** (real, not `serde_json`): one space-separated `name=value` token per changed
/// top-level field (absent token = unchanged — every `LasDiff` top-level field is a PLAIN
/// `Option<T>`, never tri-state, since no `LasHeader` field is itself optional in the snapshot);
/// `vlrs`/`points` print as `name{[removed];[modified];[added]}` sections (same collection-triple
/// shape as gif 89a's `frames`/`comments`/`app_extensions`). Strings/byte payloads are lowercase
/// hex (no external base64 dep, matches this artifact's own `ArtifactDsl` hex-dump convention and
/// gif 89a/svg's established local idiom). `Option<T>` (both real optional fields AND the
/// `LasPointDiff` tri-states) use the uniform `[0]`=None / `[1,<T>]`=Some(T) tag. Structs are
/// positional `[f1,f2,...]` tuples. `LasVlrDiff`/`LasPointDiff`'s own sparse fields print as
/// single-letter `tag:value` pairs, matching gif 89a's `GifFrameDiff` convention (`LasVlrDiff`:
/// `U`/`R`/`N`/`X`; `LasPointDiff`: `X`/`Y`/`Z`/`I`/`R`/`N`/`D`/`E`/`C`/`A`/`U`/`P`/`G`/`B` — each
/// namespace is local to its own `[...]` block, no cross-type collision).
//#region 🔖️Primitives

//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs

//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs

//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel

//#region 🔖️BinaryDiffCodec
/// 🧪️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: REAL
/// LEB128-varint-framed binary twins of the hex-text codecs above, backing the upgraded
/// `DiffBinary::encode_diff`/`decode_diff` below (`#region 🔖️TopLevel`) — replaces the old F6
/// `print_diff().into_bytes()` text-as-binary shortcut (per the P2-W0 census, 100% of stdio's
/// `DiffCodec` impls were still on it before this pilot ladder; the shortcut this file used to
/// carry is documented, and confirmed removed, in this same region). `LasDiff` is FLAT — every
/// header field a top-level `Option<T>` scalar, matching zip's own `ZipDiff` shape (not json's
/// self-recursive `JsonValue`) — so the header is real, individually-walkable binary: a `u32`
/// bitmask (bit i = field i present, declaration order) followed by only the present fields'
/// values, then two runtime-counted index-keyed triples (`vlrs`/`points`). The triples hit the
/// SAME category of gap zip's own `entries` hits (`protocol-array-of-records`, filed in this
/// wave's `mechanism_gaps`): `Block::Repeat`'s arms are tag-dispatched per iteration and
/// `Prim::Array` only repeats one fixed-width scalar, neither can express "repeat N times, N from
/// a runtime count, each iteration a multi-field record" — the Rust encode/decode below IS
/// genuinely, fully structured binary all the way down (round-trip tested by
/// `diff_codec_text_binary_roundtrip_law`), only the sibling `../💾️binary/📡️component.protocol.
/// semio` file's DESCRIPTION of it bottoms out in one opaque trailing chain past the two real
/// leading fields (`format`, `header_mask`).
//#region 🔖️BinaryPrimitives

//#endregion 🔖️BinaryPrimitives

//#region 🔖️RecordBinaryCodec

//#endregion 🔖️RecordBinaryCodec

//#region 🔖️SparseDiffBinaryCodec

//#endregion 🔖️SparseDiffBinaryCodec

//#region 🔖️HeaderMaskBits

//#endregion 🔖️HeaderMaskBits
//#endregion 🔖️BinaryDiffCodec

//#endregion 🔖️TopLevel

/// 🧪️ Shared demo-case fixtures (moved out of `mod tests` so both `demo_diff_cases` below AND
/// `mod tests` itself can use them without a `tests::` qualification cycle).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn base_point(seed: u8) -> LasPoint {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn base_vlr(record_id: u16) -> LasVlr {
    LasVlr { user_id: "LASF_Spec".into(), record_id, description: format!("vlr {record_id}"), data: vec![record_id as u8; 3] }
}

/// 🧪️ Representative `LasDiff` cases built declaratively (empty, header scalars with an array and float lanes, and the keyed `vlrs`/`points`
/// row triples with the tri-state `gps_time`/`rgb` point fields) — shared by `diff_codec_text_binary_roundtrip_law` and the engine's
/// `diff_grammar_conformance_law`/`protocol_walk_law`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<LasDiff> {
    vec![
        LasDiff::default(),
        LasDiff { version_major: Some(2), version_minor: Some(4), system_identifier: Some("after-system".into()), points_by_return: Some([0, 0, 2, 1, 0]), x_scale: Some(0.001), max_z: Some(50.0), ..LasDiff::default() },
        LasDiff {
            vlrs: Some(LasVlrsDiff { removed: vec![0], modified: vec![LasVlrModified { index: 1, diff: LasVlrDiff { record_id: Some(7), data: Some(vec![1, 2, 3]), ..LasVlrDiff::default() } }], added: vec![LasVlrAdded { index: 2, vlr: base_vlr(9) }] }),
            points: Some(LasPointsDiff {
                removed: vec![1],
                modified: vec![LasPointModified { index: 0, diff: LasPointDiff { gps_time: Some(None), rgb: Some(Some((10, 20, 30))), ..LasPointDiff::default() } }],
                added: vec![LasPointAdded { index: 1, point: base_point(3) }],
            }),
            ..LasDiff::default()
        },
    ]
}
//#endregion 🔖️HandcraftedDiffCodec

//#region Tests

//#endregion Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
