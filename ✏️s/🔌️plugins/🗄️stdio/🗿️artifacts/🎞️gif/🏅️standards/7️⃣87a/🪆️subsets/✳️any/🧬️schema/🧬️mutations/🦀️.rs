//! 🧬️ GifMutation (87a) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: beyond the universal
//! `{NoMutation, SetSnapshot}` stub, 87a's real vocabulary covers everything GIF87a actually has —
//! screen descriptor, GCT, and the image sequence (insert/remove/move/geometry/pixels/interlace).
//! No GCE-shaped mutations here (delay/disposal/transparency/loop) — 87a genuinely has none of
//! those concepts; that scope lives entirely on 89a's mutation enum.
//!
//! # Mutation-leaf migration (ticket 26/08/12/SEMANTIC-MUTATIONS-OVERHAUL)
//!
//! `protocol::Mutation<GifSnapshot>` now requires `DESCRIPTORS`/`descriptor()`, synthesized by
//! `#[derive(dsl::Mutations)]` from one mutation LEAF per variant (`../🧬️mutations/<emoji><kind>/`,
//! mirroring `stdio.tiff`'s `🔖️6.0/🧱️baseline` reference migration). `NoMutation` is dropped: the
//! derive requires every variant to wrap exactly one leaf payload and asserts
//! `is_approved_verb(SEMANTICS.verb)`, and `"no"` is not an approved verb. The old
//! `impl Mutation<GifSnapshot> for GifMutation` block is gone; its `diff`/`inverse` bodies live on
//! verbatim as the free functions `agg_diff`/`agg_inverse` below, which every leaf's own
//! `MutationKind::diff`/`inverse` delegates back into.
//!
//! `#[derive(dsl::DslOps)]` is KEPT alongside `#[derive(dsl::Mutations)]`: the hand-rolled
//! `OpText`/`OpBinary` impls below (P6: `DslOps` emits `DslVariants` only) still need
//! `dsl::DslVariants`, and `dsl_variants_codegen`'s single-tuple-field branch delegates a newtype
//! variant's `RecordSpec` to its inner type's own `DslField` impl — which is why every leaf payload
//! struct below derives `dsl::DslRecord` in addition to `dsl::MutationLeaf`.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::diff::{self, GifDiff, GifImageAdded, GifImageDiff, GifImageModified, GifImagesDiff};
#[cfg(test)]
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifRgb;
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifSnapshot};
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.gif` (87a).
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "📐set-screen-size/🦀️.rs"]
pub mod set_screen_size;
#[path = "🎨set-global-color-table/🦀️.rs"]
pub mod set_global_color_table;
#[path = "🖌️set-background-color-index/🦀️.rs"]
pub mod set_background_color_index;
#[path = "📏set-pixel-aspect-ratio/🦀️.rs"]
pub mod set_pixel_aspect_ratio;
#[path = "🖼️insert-image/🦀️.rs"]
pub mod insert_image;
#[path = "🗑️remove-image/🦀️.rs"]
pub mod remove_image;
#[path = "🔀move-image/🦀️.rs"]
pub mod move_image;
#[path = "📍set-image-geometry/🦀️.rs"]
pub mod set_image_geometry;
#[path = "🎞️set-image-pixels/🦀️.rs"]
pub mod set_image_pixels;
#[path = "🪜set-image-interlace/🦀️.rs"]
pub mod set_image_interlace;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = GifSnapshot, diff = GifDiff, schema = "GifMutation")]
pub enum GifMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetScreenSize(set_screen_size::SetScreenSize),
    SetGlobalColorTable(set_global_color_table::SetGlobalColorTable),
    SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex),
    SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio),
    InsertImage(insert_image::InsertImage),
    RemoveImage(remove_image::RemoveImage),
    MoveImage(move_image::MoveImage),
    SetImageGeometry(set_image_geometry::SetImageGeometry),
    SetImagePixels(set_image_pixels::SetImagePixels),
    SetImageInterlace(set_image_interlace::SetImageInterlace),
}

/// 🏷️ Wave 7 mutation-oracle catalog: the kebab-case spelling of every `GifMutation` variant, in
/// declaration order — what `../../🔣️oracle.json`'s `mutationCatalogs[].kinds` and
/// `../../../../../../🧪️tests/🖼️mutate-gif-87a/🥒️.feature`'s `@id-mutate`/`@id-inverse` row
/// ids are measured against. `kinds_match_enum_variants_and_manifest_catalog` below is what keeps
/// this list honest against the enum — the framework never parses Rust, so nothing else notices if
/// this list and the enum drift apart.
pub const KINDS: &[&str] = &["set-snapshot", "set-screen-size", "set-global-color-table", "set-background-color-index", "set-pixel-aspect-ratio", "insert-image", "remove-image", "move-image", "set-image-geometry", "set-image-pixels", "set-image-interlace"];
//#endregion 🔖️Mutations

/// 🧪️ P2-FG2: representative `GifMutation` cases for `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️.rs`'s `conformance_laws` module) —
/// every one of the 11 real variants, incl. both `Some`/`None` shapes of the one
/// `Option<T>`-of-struct-block field (`SetGlobalColorTable::gct`) — mirrors png's own
/// `demo_mutation_cases()`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<GifMutation> {
    let base = crate::artifacts::gif::standards::v87a::subsets::any::schema::demo_gif_snapshot();
    let sample_image = GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 9, g: 9, b: 9 }; 2] }), indices: vec![0, 1, 1, 0] };
    vec![
        GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: base.gct.clone() }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: None }),
        GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
        GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
        GifMutation::InsertImage(insert_image::InsertImage { index: 1, image: sample_image }),
        GifMutation::RemoveImage(remove_image::RemoveImage { index: 1 }),
        GifMutation::MoveImage(move_image::MoveImage { from: 0, to: 1 }),
        GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
        GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] }),
        GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 0, interlace: true }),
    ]
}

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range image indices are no-ops rather than panics.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_gif_mutation(snapshot: &mut GifSnapshot, mutation: &GifMutation) -> protocol::MutationOutcome<GifDiff> {
    let outcome = <GifMutation as Mutation<GifSnapshot>>::diff(mutation, snapshot);
    match MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// ↩️ This subset's own inverse algebra as a free function, so a caller that legitimately drives the
/// vocabulary from outside the crate reaches it without naming the `protocol::Mutation` trait.
pub fn inverse_gif_mutation(mutation: &GifMutation, base: &GifSnapshot) -> Vec<GifMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &GifMutation, base: &GifSnapshot) -> protocol::MutationOutcome<GifDiff> {
    protocol::MutationOutcome::new(match this {
        GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff::diff_set_snapshot(base, snapshot),
        GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width, height }) => GifDiff { width: (*width != base.width).then_some(*width), height: (*height != base.height).then_some(*height), ..Default::default() },
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct }) => GifDiff { gct: (*gct != base.gct).then_some(gct.clone()), ..Default::default() },
        GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index }) => GifDiff { background_color_index: (*index != base.background_color_index).then_some(*index), ..Default::default() },
        GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio }) => GifDiff { pixel_aspect_ratio: (*ratio != base.pixel_aspect_ratio).then_some(*ratio), ..Default::default() },
        GifMutation::InsertImage(insert_image::InsertImage { index, image }) => GifDiff { images: Some(GifImagesDiff { added: vec![GifImageAdded { index: (*index).min(base.images.len()), image: image.clone() }], ..Default::default() }), ..Default::default() },
        GifMutation::RemoveImage(remove_image::RemoveImage { index }) => GifDiff { images: Some(GifImagesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        GifMutation::MoveImage(move_image::MoveImage { from, to }) => {
            let mut images = base.images.clone();
            if *from < images.len() {
                let item = images.remove(*from);
                let at = (*to).min(images.len());
                images.insert(at, item);
            }
            GifDiff { images: Some(GifImagesDiff::between(&base.images, &images)), ..Default::default() }
        }
        GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index, left, top, width, height }) => {
            let d = GifImageDiff { left: Some(*left), top: Some(*top), width: Some(*width), height: Some(*height), ..Default::default() };
            GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index, indices }) => {
            let d = GifImageDiff { indices: Some(indices.clone()), ..Default::default() };
            GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index, interlace }) => {
            let d = GifImageDiff { interlace: Some(*interlace), ..Default::default() };
            GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
    })
}

/// ↩️ Real, round-trippable inverses. `apply(inverse(m), apply(m, base)) == base` for every variant,
/// incl. image-index ops. A target that no longer exists (an out-of-range index) inverts to the
/// EMPTY step list — there is no `NoMutation` stand-in any more, and an empty list is already what
/// "nothing to undo" means to every caller of `Mutation::inverse`.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &GifMutation, base: &GifSnapshot) -> Vec<GifMutation> {
    match this {
        GifMutation::SetSnapshot(_) => vec![GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        GifMutation::SetScreenSize(_) => vec![GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: base.width, height: base.height })],
        GifMutation::SetGlobalColorTable(_) => vec![GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: base.gct.clone() })],
        GifMutation::SetBackgroundColorIndex(_) => vec![GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: base.background_color_index })],
        GifMutation::SetPixelAspectRatio(_) => vec![GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: base.pixel_aspect_ratio })],
        GifMutation::InsertImage(insert_image::InsertImage { index, .. }) => vec![GifMutation::RemoveImage(remove_image::RemoveImage { index: (*index).min(base.images.len()) })],
        GifMutation::RemoveImage(remove_image::RemoveImage { index }) => match base.images.get(*index) {
            Some(image) => vec![GifMutation::InsertImage(insert_image::InsertImage { index: *index, image: image.clone() })],
            None => Vec::new(),
        },
        GifMutation::MoveImage(move_image::MoveImage { from, to }) => {
            let mut images = base.images.clone();
            let landed_at = if *from < images.len() {
                let item = images.remove(*from);
                let at = (*to).min(images.len());
                images.insert(at, item);
                at
            } else {
                *from
            };
            vec![GifMutation::MoveImage(move_image::MoveImage { from: landed_at, to: *from })]
        }
        GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index, .. }) => match base.images.get(*index) {
            Some(img) => vec![GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: *index, left: img.left, top: img.top, width: img.width, height: img.height })],
            None => Vec::new(),
        },
        GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index, .. }) => match base.images.get(*index) {
            Some(img) => vec![GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: *index, indices: img.indices.clone() })],
            None => Vec::new(),
        },
        GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index, .. }) => match base.images.get(*index) {
            Some(img) => vec![GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: *index, interlace: img.interlace })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — the same ~15-line body
/// every `DslOps`-derived enum's `OpText` impl uses (identical to gif89a's `GifMutation` impl).
impl OpText for GifMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for GifMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifRgb;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_image(seed: u8) -> GifImage {
        GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }), indices: vec![0, 1, 1, 0] }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> GifSnapshot {
        GifSnapshot { schema: "stdio.gif".into(), width: 2, height: 2, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, images: vec![sample_image(1), sample_image(2), sample_image(3)] }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn round_trips(base: &GifSnapshot, mutation: GifMutation) {
        let diff = mutation.diff(base);
        let mutated = diff.diff().apply(base).expect("diff must apply to base");
        let inverses = mutation.inverse(base);
        let mut restored = mutated.clone();
        for inv in &inverses {
            let inv_diff = inv.diff(&restored);
            restored = inv_diff.diff().apply(&restored).expect("inverse diff must apply to restored");
        }
        assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
    }

    /// 🧪️ `mutation_diff_law`: every variant's `diff()` matches what `apply_gif_mutation` returns
    /// and applying it reproduces the same mutated state.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for mutation in [
            GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { background_color_index: 9, ..base.clone() } }),
            GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
            GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb::default(); 2] }) }),
            GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
            GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
            GifMutation::InsertImage(insert_image::InsertImage { index: 1, image: sample_image(9) }),
            GifMutation::RemoveImage(remove_image::RemoveImage { index: 1 }),
            GifMutation::MoveImage(move_image::MoveImage { from: 0, to: 2 }),
            GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
            GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] }),
            GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 0, interlace: true }),
        ] {
            let mut snap = base.clone();
            let returned_diff = apply_gif_mutation(&mut snap, &mutation);
            let expected_diff = mutation.diff(&base);
            assert_eq!(returned_diff, expected_diff, "returned diff must equal mutation.diff(base) for {mutation:?}");
            assert_eq!(snap, expected_diff.diff().apply(&base).expect("diff must apply to base"), "apply_gif_mutation must match diff.diff().apply(base) for {mutation:?}");
        }
    }

    /// 🧪️ `inverse_law` (mutation-level): every variant round-trips.
    #[semio_framework_async_macros::async_test]
    async fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base_snapshot();
        round_trips(&base, GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { background_color_index: 5, ..base.clone() } }));
        round_trips(&base, GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 8, height: 6 }));
        round_trips(&base, GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb::default(); 4] }) }));
        round_trips(&base, GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 2 }));
        round_trips(&base, GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 9 }));
        round_trips(&base, GifMutation::InsertImage(insert_image::InsertImage { index: 1, image: sample_image(9) }));
        round_trips(&base, GifMutation::RemoveImage(remove_image::RemoveImage { index: 1 }));
        round_trips(&base, GifMutation::MoveImage(move_image::MoveImage { from: 0, to: 2 }));
        round_trips(&base, GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }));
        round_trips(&base, GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] }));
        round_trips(&base, GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 2, interlace: true }));
    }

    /// 🧪️ Wave 7: `KINDS` must name exactly the enum's variants (kebab-cased, declaration order),
    /// and exactly the manifest's `mutationCatalogs[].kinds` — the framework never parses Rust, so
    /// this is what keeps the oracle catalog declaration honest against a drifted enum.
    #[semio_framework_async_macros::async_test]
    async fn kinds_match_enum_variants_and_manifest_catalog() {
        assert_eq!(KINDS, ["set-snapshot", "set-screen-size", "set-global-color-table", "set-background-color-index", "set-pixel-aspect-ratio", "insert-image", "remove-image", "move-image", "set-image-geometry", "set-image-pixels", "set-image-interlace"]);
        assert_eq!(KINDS.len(), 11, "one kebab-case entry per GifMutation variant");
        let manifest = include_str!("../../🔮️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "manifest mutationCatalogs[].kinds must list {kind:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_image_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_gif_mutation(&mut snap, &GifMutation::RemoveImage(remove_image::RemoveImage { index: 99 }));
        assert_eq!(snap, base);
    }

    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws over the full 11-variant vocabulary (handcrafted
    /// impls over the `dsl::DslOps`-derived `DslVariants`, mirroring gif89a's `GifMutation`).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for mutation in [
            GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: GifSnapshot { background_color_index: 9, ..base.clone() } }),
            GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
            GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb::default(); 2] }) }),
            GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: None }),
            GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
            GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
            GifMutation::InsertImage(insert_image::InsertImage { index: 1, image: sample_image(9) }),
            GifMutation::RemoveImage(remove_image::RemoveImage { index: 1 }),
            GifMutation::MoveImage(move_image::MoveImage { from: 0, to: 2 }),
            GifMutation::SetImageGeometry(set_image_geometry::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
            GifMutation::SetImagePixels(set_image_pixels::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] }),
            GifMutation::SetImageInterlace(set_image_interlace::SetImageInterlace { index: 0, interlace: true }),
        ] {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = GifMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = GifMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
}
//#endregion Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `🦀️.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📸️set-snapshot/🧪️tests/🖼️repaints-the-right-pixel-of-the-single-image/🦀️.rs"]
    mod tests_set_snapshot_repaints_the_right_pixel_of_the_single_image;
}
//#endregion 🧪️FixtureTests
