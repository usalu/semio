//! 🧬️ GifMutation (87a) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: beyond the universal
//! `{NoMutation, SetSnapshot}` stub, 87a's real vocabulary covers everything GIF87a actually has —
//! screen descriptor, GCT, and the image sequence (insert/remove/move/geometry/pixels/interlace).
//! No GCE-shaped mutations here (delay/disposal/transparency/loop) — 87a genuinely has none of
//! those concepts; that scope lives entirely on 89a's mutation enum.

use crate::artifacts::gif::standards::v87a::subsets::any::schema::diff::{self, GifDiff, GifImageAdded, GifImageDiff, GifImageModified, GifImagesDiff};
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifSnapshot};
#[cfg(test)]
use crate::artifacts::gif::standards::v87a::subsets::any::schema::snapshot::GifRgb;
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.gif` (87a).
///
/// 🧪️ F6: `dsl::DslOps` derive — like 89a's `GifMutation` (and unlike `GifDiff`, blocked by
/// tri-state fields, see the `🔺️diff` module), NO mutation variant here uses `Option<Option<T>>`
/// (a mutation's `Option<T>` argument means "the new value", never a diff tri-state) and
/// `GifSnapshot`'s tree (87a's, no GCE/frame concept) has no data-carrying enum anywhere, so every
/// variant's payload binds cleanly once `GifRgb`/`GifColorTable`/`GifImage`/`GifSnapshot` get
/// `#[derive(dsl::DslRecord)]` (see the `📸️snapshot` module — cascading requirement, confirmed via
/// real `cargo check`: `DslField is not implemented for GifSnapshot`/`GifColorTable`/`GifImage`
/// until those derives were added, zero errors after). `#[dsl(block)]` on struct-valued payloads
/// (`snapshot`, `image`) matches the `SpaceMutation`/`FlowMutationDsl`/gif89a framework precedent's
/// formatting convention; `#[dsl(base64)]` on the one bare `Vec<u8>` payload (`SetImagePixels::indices`)
/// keeps it compact.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum GifMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        #[dsl(block)]
        snapshot: GifSnapshot,
    },
    SetScreenSize {
        width: u32,
        height: u32,
    },
    SetGlobalColorTable {
        #[dsl(block)]
        gct: Option<GifColorTable>,
    },
    SetBackgroundColorIndex {
        index: u8,
    },
    SetPixelAspectRatio {
        ratio: u8,
    },
    InsertImage {
        index: usize,
        #[dsl(block)]
        image: GifImage,
    },
    RemoveImage {
        index: usize,
    },
    MoveImage {
        from: usize,
        to: usize,
    },
    SetImageGeometry {
        index: usize,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    },
    SetImagePixels {
        index: usize,
        #[dsl(base64)]
        indices: Vec<u8>,
    },
    SetImageInterlace {
        index: usize,
        interlace: bool,
    },
}
//#endregion 🔖️Mutations

/// 🧪️ P2-FG2: representative `GifMutation` cases for `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️component.rs`'s `conformance_laws` module) —
/// every one of the 12 real variants, incl. both `Some`/`None` shapes of the one
/// `Option<T>`-of-struct-block field (`SetGlobalColorTable::gct`) — mirrors png's own
/// `demo_mutation_cases()`.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<GifMutation> {
    let base = crate::artifacts::gif::standards::v87a::subsets::any::schema::demo_gif_snapshot();
    let sample_image = GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 9, g: 9, b: 9 }; 2] }), indices: vec![0, 1, 1, 0] };
    vec![
        GifMutation::NoMutation,
        GifMutation::SetSnapshot { snapshot: base.clone() },
        GifMutation::SetScreenSize { width: 10, height: 10 },
        GifMutation::SetGlobalColorTable { gct: base.gct.clone() },
        GifMutation::SetGlobalColorTable { gct: None },
        GifMutation::SetBackgroundColorIndex { index: 5 },
        GifMutation::SetPixelAspectRatio { ratio: 3 },
        GifMutation::InsertImage { index: 1, image: sample_image },
        GifMutation::RemoveImage { index: 1 },
        GifMutation::MoveImage { from: 0, to: 1 },
        GifMutation::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 },
        GifMutation::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] },
        GifMutation::SetImageInterlace { index: 0, interlace: true },
    ]
}

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range image indices are no-ops rather than panics.
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
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<GifSnapshot> for GifMutation {
    type Diff = GifDiff;

    fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            GifMutation::NoMutation => GifDiff::default(),
            GifMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            GifMutation::SetScreenSize { width, height } => GifDiff { width: (*width != base.width).then_some(*width), height: (*height != base.height).then_some(*height), ..Default::default() },
            GifMutation::SetGlobalColorTable { gct } => GifDiff { gct: (*gct != base.gct).then_some(gct.clone()), ..Default::default() },
            GifMutation::SetBackgroundColorIndex { index } => GifDiff { background_color_index: (*index != base.background_color_index).then_some(*index), ..Default::default() },
            GifMutation::SetPixelAspectRatio { ratio } => GifDiff { pixel_aspect_ratio: (*ratio != base.pixel_aspect_ratio).then_some(*ratio), ..Default::default() },
            GifMutation::InsertImage { index, image } => GifDiff { images: Some(GifImagesDiff { added: vec![GifImageAdded { index: (*index).min(base.images.len()), image: image.clone() }], ..Default::default() }), ..Default::default() },
            GifMutation::RemoveImage { index } => GifDiff { images: Some(GifImagesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
            GifMutation::MoveImage { from, to } => {
                let mut images = base.images.clone();
                if *from < images.len() {
                    let item = images.remove(*from);
                    let at = (*to).min(images.len());
                    images.insert(at, item);
                }
                GifDiff { images: Some(GifImagesDiff::between(&base.images, &images)), ..Default::default() }
            }
            GifMutation::SetImageGeometry { index, left, top, width, height } => {
                let d = GifImageDiff { left: Some(*left), top: Some(*top), width: Some(*width), height: Some(*height), ..Default::default() };
                GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetImagePixels { index, indices } => {
                let d = GifImageDiff { indices: Some(indices.clone()), ..Default::default() };
                GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetImageInterlace { index, interlace } => {
                let d = GifImageDiff { interlace: Some(*interlace), ..Default::default() };
                GifDiff { images: Some(GifImagesDiff { modified: vec![GifImageModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
        })
    }

    /// ↩️ Real, round-trippable inverses. `apply(inverse(m, base), apply(m, base)) == base` for
    /// every variant, incl. image-index ops.
    fn inverse(&self, base: &GifSnapshot) -> Vec<Self> {
        match self {
            GifMutation::NoMutation => vec![GifMutation::NoMutation],
            GifMutation::SetSnapshot { .. } => vec![GifMutation::SetSnapshot { snapshot: base.clone() }],
            GifMutation::SetScreenSize { .. } => vec![GifMutation::SetScreenSize { width: base.width, height: base.height }],
            GifMutation::SetGlobalColorTable { .. } => vec![GifMutation::SetGlobalColorTable { gct: base.gct.clone() }],
            GifMutation::SetBackgroundColorIndex { .. } => vec![GifMutation::SetBackgroundColorIndex { index: base.background_color_index }],
            GifMutation::SetPixelAspectRatio { .. } => vec![GifMutation::SetPixelAspectRatio { ratio: base.pixel_aspect_ratio }],
            GifMutation::InsertImage { index, .. } => vec![GifMutation::RemoveImage { index: (*index).min(base.images.len()) }],
            GifMutation::RemoveImage { index } => match base.images.get(*index) {
                Some(image) => vec![GifMutation::InsertImage { index: *index, image: image.clone() }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::MoveImage { from, to } => {
                let mut images = base.images.clone();
                let landed_at = if *from < images.len() {
                    let item = images.remove(*from);
                    let at = (*to).min(images.len());
                    images.insert(at, item);
                    at
                } else {
                    *from
                };
                vec![GifMutation::MoveImage { from: landed_at, to: *from }]
            }
            GifMutation::SetImageGeometry { index, .. } => match base.images.get(*index) {
                Some(img) => vec![GifMutation::SetImageGeometry { index: *index, left: img.left, top: img.top, width: img.width, height: img.height }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetImagePixels { index, .. } => match base.images.get(*index) {
                Some(img) => vec![GifMutation::SetImagePixels { index: *index, indices: img.indices.clone() }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetImageInterlace { index, .. } => match base.images.get(*index) {
                Some(img) => vec![GifMutation::SetImageInterlace { index: *index, interlace: img.interlace }],
                None => vec![GifMutation::NoMutation],
            },
        }
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

    fn sample_image(seed: u8) -> GifImage {
        GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: seed, g: seed, b: seed }; 2] }), indices: vec![0, 1, 1, 0] }
    }

    fn base_snapshot() -> GifSnapshot {
        GifSnapshot { schema: "stdio.gif".into(), width: 2, height: 2, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, images: vec![sample_image(1), sample_image(2), sample_image(3)] }
    }

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
    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        for mutation in [
            GifMutation::NoMutation,
            GifMutation::SetSnapshot { snapshot: GifSnapshot { background_color_index: 9, ..base.clone() } },
            GifMutation::SetScreenSize { width: 10, height: 10 },
            GifMutation::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb::default(); 2] }) },
            GifMutation::SetBackgroundColorIndex { index: 5 },
            GifMutation::SetPixelAspectRatio { ratio: 3 },
            GifMutation::InsertImage { index: 1, image: sample_image(9) },
            GifMutation::RemoveImage { index: 1 },
            GifMutation::MoveImage { from: 0, to: 2 },
            GifMutation::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 },
            GifMutation::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] },
            GifMutation::SetImageInterlace { index: 0, interlace: true },
        ] {
            let mut snap = base.clone();
            let returned_diff = apply_gif_mutation(&mut snap, &mutation);
            let expected_diff = mutation.diff(&base);
            assert_eq!(returned_diff, expected_diff, "returned diff must equal mutation.diff(base) for {mutation:?}");
            assert_eq!(snap, expected_diff.diff().apply(&base).expect("diff must apply to base"), "apply_gif_mutation must match diff.diff().apply(base) for {mutation:?}");
        }
    }

    /// 🧪️ `inverse_law` (mutation-level): every variant round-trips.
    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base_snapshot();
        round_trips(&base, GifMutation::NoMutation);
        round_trips(&base, GifMutation::SetSnapshot { snapshot: GifSnapshot { background_color_index: 5, ..base.clone() } });
        round_trips(&base, GifMutation::SetScreenSize { width: 8, height: 6 });
        round_trips(&base, GifMutation::SetGlobalColorTable { gct: Some(GifColorTable { sorted: false, colors: vec![GifRgb::default(); 4] }) });
        round_trips(&base, GifMutation::SetBackgroundColorIndex { index: 2 });
        round_trips(&base, GifMutation::SetPixelAspectRatio { ratio: 9 });
        round_trips(&base, GifMutation::InsertImage { index: 1, image: sample_image(9) });
        round_trips(&base, GifMutation::RemoveImage { index: 1 });
        round_trips(&base, GifMutation::MoveImage { from: 0, to: 2 });
        round_trips(&base, GifMutation::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 });
        round_trips(&base, GifMutation::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] });
        round_trips(&base, GifMutation::SetImageInterlace { index: 2, interlace: true });
    }

    #[test]
    fn remove_image_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_gif_mutation(&mut snap, &GifMutation::RemoveImage { index: 99 });
        assert_eq!(snap, base);
    }

    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws over the full 12-variant vocabulary (handcrafted
    /// impls over the `dsl::DslOps`-derived `DslVariants`, mirroring gif89a's `GifMutation`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for mutation in [
            GifMutation::NoMutation,
            GifMutation::SetSnapshot { snapshot: GifSnapshot { background_color_index: 9, ..base.clone() } },
            GifMutation::SetScreenSize { width: 10, height: 10 },
            GifMutation::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![GifRgb::default(); 2] }) },
            GifMutation::SetGlobalColorTable { gct: None },
            GifMutation::SetBackgroundColorIndex { index: 5 },
            GifMutation::SetPixelAspectRatio { ratio: 3 },
            GifMutation::InsertImage { index: 1, image: sample_image(9) },
            GifMutation::RemoveImage { index: 1 },
            GifMutation::MoveImage { from: 0, to: 2 },
            GifMutation::SetImageGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 },
            GifMutation::SetImagePixels { index: 0, indices: vec![1, 1, 1, 1] },
            GifMutation::SetImageInterlace { index: 0, interlace: true },
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
