//! 🧬️ SemioImageMutation — named-variant enum covering every mutable field of
//! `SemioImageSnapshot`. Every `diff()`/`inverse()` arm is HAND-WRITTEN (never apply-and-capture
//! — `🧬️schema-design.md`'s svg infinite-recursion warning): each variant builds its own sparse
//! `SemioImageDiff` directly and computes its own base-aware inverse mutation.
//!
//! `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
//! leaf payload, and its sentinel verb `no` is not in `APPROVED_VERBS` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs`. Every variant is now a
//! tuple variant wrapping its own mutation leaf (`./*/🦀️.rs`), and this file's `agg_diff`/
//! `agg_inverse` carry the handcrafted semantics every leaf's `MutationKind` impl delegates back to.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, NamedModified};


use crate::standards::v1::subsets::image::schema::diff::{diff_set_snapshot, SemioImageDiff, SemioImageFrameDiff, SemioImageFramesDiff, SemioImageMetadataDiff};








use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioImageMutation` below calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in scope in
/// production code (was missing entirely, even test-gated) (W2b closer fix).


//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "➕️insert-frame/🦀️.rs"]
pub mod insert_frame;
#[path = "🔀️move-frame/🦀️.rs"]
pub mod move_frame;
#[path = "🚫️remove-frame/🦀️.rs"]
pub mod remove_frame;
#[path = "🗑️remove-metadata-entry/🦀️.rs"]
pub mod remove_metadata_entry;
#[path = "🔢️set-bit-depth/🦀️.rs"]
pub mod set_bit_depth;
#[path = "🌈️set-colorspace/🦀️.rs"]
pub mod set_colorspace;
#[path = "📐️set-dimensions/🦀️.rs"]
pub mod set_dimensions;
#[path = "⏱️set-frame-delay/🦀️.rs"]
pub mod set_frame_delay;
#[path = "🖌️set-frame-pixels/🦀️.rs"]
pub mod set_frame_pixels;
#[path = "🎨️set-icc/🦀️.rs"]
pub mod set_icc;
#[path = "🏷️set-metadata-entry/🦀️.rs"]
pub mod set_metadata_entry;
#[path = "🩹️patch-snapshot/🦀️.rs"]
pub mod patch_snapshot;
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioImageSnapshot, diff = SemioImageDiff, schema = "SemioImageMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioImageMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    SetDimensions(set_dimensions::SetDimensions),
    SetColorspace(set_colorspace::SetColorspace),
    SetBitDepth(set_bit_depth::SetBitDepth),
    /// 🎨️ `icc: None` clears the profile — the mutation payload is the FINAL value (unlike the
    /// diff's own tri-state, a mutation never needs to distinguish "no-op" from "clear").
    SetIcc(set_icc::SetIcc),
    InsertFrame(insert_frame::InsertFrame),
    RemoveFrame(remove_frame::RemoveFrame),
    MoveFrame(move_frame::MoveFrame),
    SetFrameDelay(set_frame_delay::SetFrameDelay),
    SetFramePixels(set_frame_pixels::SetFramePixels),
    SetMetadataEntry(set_metadata_entry::SetMetadataEntry),
    RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry),
}

/// 🏷️ Kebab-case spelling of every `SemioImageMutation` variant, in declaration order — the
/// vocabulary the `semio-v1-image` mutation catalog (`../../🔣️oracle.json`) declares and
/// `🖼️mutate-semio-image`'s exhaustive test case measures itself against.
pub const KINDS: &[&str] = &["set-snapshot", "patch-snapshot", "set-dimensions", "set-colorspace", "set-bit-depth", "set-icc", "insert-frame", "remove-frame", "move-frame", "set-frame-delay", "set-frame-pixels", "set-metadata-entry", "remove-metadata-entry"];

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and every triad leaf).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_image_mutation(snapshot: &mut SemioImageSnapshot, mutation: &SemioImageMutation) -> protocol::MutationOutcome<SemioImageDiff> {
    let outcome = <SemioImageMutation as Mutation<SemioImageSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`Mutation::inverse`], named only in this subset's own reachable types.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol;` alias that nothing
/// re-exports, so an owner-root test adapter compiled as an external crate cannot bring the
/// `Mutation` trait into scope to call the method form — the structural gap wave 7 recorded for
/// `kit`/`object`/`text`/`table`, and the same thin-wrapper remedy `kit` adopted. Used by
/// `🖼️mutate-semio-image`'s `inverse-*` scenarios.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_image_mutation(mutation: &SemioImageMutation, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    Ok({
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(mutation, base)?

    })
}
//#endregion 🔖️Mutation

//#region 🔖️MutationTrait
/// ↩️ An index/key that no longer exists in `base` has nothing to restore, so those arms return
/// the empty inverse rather than a sentinel no-op mutation — the convention this migration adopted
/// once `NoMutation` stopped being an available payload.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioImageMutation, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    protocol::MutationOutcome::new(match this {
        SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioImageMutation::PatchSnapshot(patch) => return <patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioImageSnapshot, SemioImageMutation>>::diff(patch, base),
        SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width, height }) => SemioImageDiff { width: (base.width != *width).then_some(*width), height: (base.height != *height).then_some(*height), ..Default::default() },
        SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace }) => SemioImageDiff { colorspace: (base.colorspace != *colorspace).then_some(*colorspace), ..Default::default() },
        SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth }) => SemioImageDiff { bit_depth: (base.bit_depth != *bit_depth).then_some(*bit_depth), ..Default::default() },
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc }) => SemioImageDiff { icc: (base.icc != *icc).then_some(icc.clone()), ..Default::default() },
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, frame }) => {
            SemioImageDiff { frames: Some(SemioImageFramesDiff { added: vec![IndexAdded { index: *index, item: frame.clone() }], ..Default::default() }), ..Default::default() }
        }
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => SemioImageDiff { frames: Some(SemioImageFramesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }) => {
            let frames = base.frames.get(*from).map(|item| SemioImageFramesDiff { removed: vec![*from], added: vec![IndexAdded { index: *to, item: item.clone() }], ..Default::default() });
            SemioImageDiff { frames, ..Default::default() }
        }
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, delay_ms }) => {
            SemioImageDiff { frames: Some(SemioImageFramesDiff { modified: vec![IndexModified { index: *index, diff: SemioImageFrameDiff { delay_ms: Some(*delay_ms), rgba8: None } }], ..Default::default() }), ..Default::default() }
        }
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8 }) => {
            SemioImageDiff { frames: Some(SemioImageFramesDiff { modified: vec![IndexModified { index: *index, diff: SemioImageFrameDiff { delay_ms: None, rgba8: Some(rgba8.clone()) } }], ..Default::default() }), ..Default::default() }
        }
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key, value }) => {
            let metadata = if base.metadata.iter().any(|e| &e.key == key) {
                SemioImageMetadataDiff { modified: vec![NamedModified { key: key.clone(), diff: value.clone() }], ..Default::default() }
            } else {
                SemioImageMetadataDiff { added: vec![SemioImageMetadataEntry { key: key.clone(), value: value.clone() }], ..Default::default() }
            };
            SemioImageDiff { metadata: Some(metadata), ..Default::default() }
        }
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }) => SemioImageDiff { metadata: Some(SemioImageMetadataDiff { removed: vec![key.clone()], ..Default::default() }), ..Default::default() },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioImageMutation, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    Ok({
    vec![match this {
        SemioImageMutation::SetSnapshot(_) => SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        SemioImageMutation::PatchSnapshot(patch) => return Ok(<patch_snapshot::PatchSnapshot as protocol::MutationKind<SemioImageSnapshot, SemioImageMutation>>::inverse(patch, base)?),
        SemioImageMutation::SetDimensions(_) => SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: base.width, height: base.height }),
        SemioImageMutation::SetColorspace(_) => SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: base.colorspace }),
        SemioImageMutation::SetBitDepth(_) => SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: base.bit_depth }),
        SemioImageMutation::SetIcc(_) => SemioImageMutation::SetIcc(set_icc::SetIcc { icc: base.icc.clone() }),
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, .. }) => SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: *index }),
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => match base.frames.get(*index) {
            Some(frame) => SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: *index, frame: frame.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }) => SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: *to, to: *from }),
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, .. }) => match base.frames.get(*index) {
            Some(frame) => SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: *index, delay_ms: frame.delay_ms }),
            None => return Ok(Vec::new()),
        },
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, .. }) => match base.frames.get(*index) {
            Some(frame) => SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: *index, rgba8: frame.rgba8.clone() }),
            None => return Ok(Vec::new()),
        },
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key, .. }) => match base.metadata.iter().find(|e| &e.key == key) {
            Some(entry) => SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: entry.value.clone() }),
            None => SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: key.clone() }),
        },
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }) => match base.metadata.iter().find(|e| &e.key == key) {
            Some(entry) => SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: entry.value.clone() }),
            None => return Ok(Vec::new()),
        },
    }]

    })
}
//#endregion 🔖️MutationTrait

//#region 🔖️OpCodecs




















//#endregion 🔖️OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioImageMutation` cases, one per variant — single source of truth for
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs` and this
/// file's own `op_text_binary_roundtrip_law`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioImageMutation> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn frame(seed: u8, len: usize) -> SemioImageFrame {
        SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture() -> SemioImageSnapshot {
        SemioImageSnapshot {
            width: 4,
            height: 4,
            colorspace: SemioColorspace::Rgba,
            bit_depth: 8,
            frames: vec![frame(1, 16), frame(2, 16)],
            icc: Some(vec![9, 9]),
            metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "old".into() }],
            ..SemioImageSnapshot::default()
        }
    }
    vec![
        SemioImageMutation::PatchSnapshot(patch_snapshot::PatchSnapshot { patch: semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set { path: "/schema".into(), value: semio_framework_value::DslValue::String("stdio.patch-snapshot.witness".into()) } }),
        SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: fixture() }),
        SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: 8, height: 8 }),
        SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: SemioColorspace::Grayscale }),
        SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: 16 }),
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc: None }),
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc: Some(vec![1, 2, 3]) }),
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: 1, frame: frame(5, 16) }),
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: 0 }),
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: 0, to: 1 }),
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_ms: 250 }),
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 1, rgba8: vec![7; 16] }),
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Title".into(), value: "new".into() }),
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Author".into(), value: "someone".into() }),
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: "Title".into() }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

//#region 🧪️FixtureTests
/// 🧪️ Handcrafted mutation fixtures (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`)
/// — one case per triad leaf, self-wired here rather than in `🦀️.rs` so this subset owns its
/// own test surface. `#[path = "."]` re-roots the nested `#[path]`s at THIS file's directory (the
/// `🧬️mutations` root) instead of the implicit `🦀️component/` child directory. Each case file
/// additionally mounts its OWN leaf `🔺️diff` module, because the enum arms above carry no guard
/// branches — the leaves own every diagnostic.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests

#[cfg(test)]
use protocol::{OpText};
