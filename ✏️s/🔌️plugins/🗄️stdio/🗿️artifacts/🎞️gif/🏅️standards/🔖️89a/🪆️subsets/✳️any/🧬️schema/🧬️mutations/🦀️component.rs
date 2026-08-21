//! 🧬️ GifMutation (89a) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the full ~20-variant
//! vocabulary the plan's worked design calls for (was 6 of ~20, `apply_gif_mutation` returned
//! `()`) — screen/GCT/loop scalars, frame insert/remove/move/geometry/pixels/interlace/delay/
//! disposal/transparency/user-input, and comment/app-extension insert/remove. Every variant's
//! `diff()` is handcrafted directly against the sparse `GifDiff` shape (no apply-and-capture).

use crate::artifacts::gif::standards::v89a::subsets::any::schema::diff::{self, GifAppExtensionAdded, GifAppExtensionsDiff, GifCommentAdded, GifCommentsDiff, GifDiff, GifFrameAdded, GifFrameDiff, GifFrameModified, GifFramesDiff};
use crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifSnapshot};
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.gif.89a`.
///
/// 🧪️ F6-PILOT: `dsl::DslOps` derive — unlike `GifDiff` (blocked by tri-state fields, see the
/// `🔺️diff` module), NO mutation variant here uses `Option<Option<T>>` (a mutation's `Option<T>`
/// argument means "the new value", never a diff tri-state), so every variant's payload binds
/// cleanly. `#[dsl(block)]` on struct-valued payloads (`snapshot`, `frame`, `gct`, `extension`)
/// matches the `SpaceMutation`/`FlowMutationDsl` framework precedent's formatting convention;
/// `#[dsl(base64)]` on the one bare `Vec<u8>` payload (`SetFramePixels::indices`) keeps it compact.
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
    SetLoopCount {
        loop_count: Option<u16>,
    },
    InsertFrame {
        index: usize,
        #[dsl(block)]
        frame: GifFrame,
    },
    RemoveFrame {
        index: usize,
    },
    MoveFrame {
        from: usize,
        to: usize,
    },
    SetFrameGeometry {
        index: usize,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    },
    SetFramePixels {
        index: usize,
        #[dsl(base64)]
        indices: Vec<u8>,
    },
    SetFrameInterlace {
        index: usize,
        interlace: bool,
    },
    SetFrameDelay {
        index: usize,
        delay_cs: u16,
    },
    SetFrameDisposal {
        index: usize,
        disposal: GifDisposal,
    },
    SetFrameTransparency {
        index: usize,
        transparent_index: Option<u8>,
    },
    SetFrameUserInput {
        index: usize,
        user_input: bool,
    },
    InsertComment {
        index: usize,
        text: String,
    },
    RemoveComment {
        index: usize,
    },
    AddAppExtension {
        index: usize,
        #[dsl(block)]
        extension: GifAppExtension,
    },
    RemoveAppExtension {
        index: usize,
    },
}
//#endregion 🔖️Mutations

/// 🧪️ P2-FG2: representative `GifMutation` (89a) cases for `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️component.rs`'s `conformance_laws` module) —
/// every one of the 21 real variants, incl. `Some`/`None` shapes of every `Option<T>` field
/// (`gct`, `loop_count`, `transparent_index`) — mirrors 87a's own `demo_mutation_cases()`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<GifMutation> {
    // 🧭️ Deliberately a small, hand-built snapshot for `SetSnapshot`'s own payload — NOT
    // `engine::demo_gif_snapshot()` (the real, 800×800/54-frame `dancing.gif` fixture used by
    // the snapshot-facet conformance laws): embedding that full fixture inside a `SetSnapshot`
    // op-text payload is unnecessarily large for exercising the mutations grammar's own
    // shape, which this compact snapshot already covers field-for-field.
    let base = GifSnapshot {
        schema: crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width: 2,
        height: 2,
        gct: Some(GifColorTable { sorted: false, colors: vec![crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifRgb { r: 4, g: 5, b: 6 }; 2] }),
        background_color_index: 0,
        pixel_aspect_ratio: 0,
        loop_count: Some(0),
        frames: vec![],
        comments: vec!["c0".into()],
        app_extensions: vec![],
    };
    let sample_frame = GifFrame {
        left: 0,
        top: 0,
        width: 2,
        height: 2,
        interlace: false,
        lct: Some(GifColorTable { sorted: false, colors: vec![crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifRgb { r: 9, g: 9, b: 9 }; 2] }),
        indices: vec![0, 1, 1, 0],
        delay_cs: 10,
        disposal: GifDisposal::DoNotDispose,
        transparent_index: None,
        user_input: false,
        plain_text: None,
    };
    let gct_value = Some(GifColorTable { sorted: true, colors: vec![Default::default(); 2] });
    vec![
        GifMutation::NoMutation,
        GifMutation::SetSnapshot { snapshot: base.clone() },
        GifMutation::SetScreenSize { width: 10, height: 10 },
        GifMutation::SetGlobalColorTable { gct: gct_value },
        GifMutation::SetGlobalColorTable { gct: None },
        GifMutation::SetBackgroundColorIndex { index: 5 },
        GifMutation::SetPixelAspectRatio { ratio: 3 },
        GifMutation::SetLoopCount { loop_count: Some(7) },
        GifMutation::SetLoopCount { loop_count: None },
        GifMutation::InsertFrame { index: 1, frame: sample_frame },
        GifMutation::RemoveFrame { index: 1 },
        GifMutation::MoveFrame { from: 0, to: 1 },
        GifMutation::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 },
        GifMutation::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] },
        GifMutation::SetFrameInterlace { index: 0, interlace: true },
        GifMutation::SetFrameDelay { index: 0, delay_cs: 77 },
        GifMutation::SetFrameDisposal { index: 0, disposal: GifDisposal::RestoreToBackground },
        GifMutation::SetFrameTransparency { index: 0, transparent_index: Some(1) },
        GifMutation::SetFrameTransparency { index: 0, transparent_index: None },
        GifMutation::SetFrameUserInput { index: 0, user_input: true },
        GifMutation::InsertComment { index: 0, text: "newc".into() },
        GifMutation::RemoveComment { index: 0 },
        GifMutation::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMPDATA1", auth_code: *b"XMP", data: vec![1] } },
        GifMutation::RemoveAppExtension { index: 0 },
    ]
}

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range frame/comment/extension indices are no-ops
/// rather than panics -- a stale index (e.g. from a concurrent edit) should degrade gracefully.
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
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<GifSnapshot> for GifMutation {
    type Diff = GifDiff;

    async fn diff(&self, base: &GifSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            GifMutation::NoMutation => GifDiff::default(),
            GifMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            GifMutation::SetScreenSize { width, height } => GifDiff { width: (*width != base.width).then_some(*width), height: (*height != base.height).then_some(*height), ..Default::default() },
            GifMutation::SetGlobalColorTable { gct } => GifDiff { gct: (*gct != base.gct).then_some(gct.clone()), ..Default::default() },
            GifMutation::SetBackgroundColorIndex { index } => GifDiff { background_color_index: (*index != base.background_color_index).then_some(*index), ..Default::default() },
            GifMutation::SetPixelAspectRatio { ratio } => GifDiff { pixel_aspect_ratio: (*ratio != base.pixel_aspect_ratio).then_some(*ratio), ..Default::default() },
            GifMutation::SetLoopCount { loop_count } => GifDiff { loop_count: (*loop_count != base.loop_count).then_some(*loop_count), ..Default::default() },
            GifMutation::InsertFrame { index, frame } => GifDiff { frames: Some(GifFramesDiff { added: vec![GifFrameAdded { index: (*index).min(base.frames.len()), frame: frame.clone() }], ..Default::default() }), ..Default::default() },
            GifMutation::RemoveFrame { index } => GifDiff { frames: Some(GifFramesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
            GifMutation::MoveFrame { from, to } => {
                let mut frames = base.frames.clone();
                if *from < frames.len() {
                    let item = frames.remove(*from);
                    let at = (*to).min(frames.len());
                    frames.insert(at, item);
                }
                GifDiff { frames: Some(GifFramesDiff::between(&base.frames, &frames)), ..Default::default() }
            }
            GifMutation::SetFrameGeometry { index, left, top, width, height } => {
                let d = GifFrameDiff { left: Some(*left), top: Some(*top), width: Some(*width), height: Some(*height), ..Default::default() };
                GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetFramePixels { index, indices } => {
                let d = GifFrameDiff { indices: Some(indices.clone()), ..Default::default() };
                GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetFrameInterlace { index, interlace } => {
                let d = GifFrameDiff { interlace: Some(*interlace), ..Default::default() };
                GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetFrameDelay { index, delay_cs } => {
                let d = GifFrameDiff { delay_cs: Some(*delay_cs), ..Default::default() };
                GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetFrameDisposal { index, disposal } => {
                let d = GifFrameDiff { disposal: Some(*disposal), ..Default::default() };
                GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetFrameTransparency { index, transparent_index } => {
                let d = GifFrameDiff { transparent_index: Some(*transparent_index), ..Default::default() };
                GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::SetFrameUserInput { index, user_input } => {
                let d = GifFrameDiff { user_input: Some(*user_input), ..Default::default() };
                GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::InsertComment { index, text } => GifDiff { comments: Some(GifCommentsDiff { added: vec![GifCommentAdded { index: (*index).min(base.comments.len()), text: text.clone() }], ..Default::default() }), ..Default::default() },
            GifMutation::RemoveComment { index } => GifDiff { comments: Some(GifCommentsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
            GifMutation::AddAppExtension { index, extension } => {
                GifDiff { app_extensions: Some(GifAppExtensionsDiff { added: vec![GifAppExtensionAdded { index: (*index).min(base.app_extensions.len()), extension: extension.clone() }], ..Default::default() }), ..Default::default() }
            }
            GifMutation::RemoveAppExtension { index } => GifDiff { app_extensions: Some(GifAppExtensionsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        }).await
    }

    /// ↩️ Real, round-trippable inverses: `apply(inverse(m, base), apply(m, base)) == base` for
    /// every variant, including the frame/comment/extension-index ops.
    async fn inverse(&self, base: &GifSnapshot) -> Vec<Self> {
        match self {
            GifMutation::NoMutation => vec![GifMutation::NoMutation],
            GifMutation::SetSnapshot { .. } => vec![GifMutation::SetSnapshot { snapshot: base.clone() }],
            GifMutation::SetScreenSize { .. } => vec![GifMutation::SetScreenSize { width: base.width, height: base.height }],
            GifMutation::SetGlobalColorTable { .. } => vec![GifMutation::SetGlobalColorTable { gct: base.gct.clone() }],
            GifMutation::SetBackgroundColorIndex { .. } => vec![GifMutation::SetBackgroundColorIndex { index: base.background_color_index }],
            GifMutation::SetPixelAspectRatio { .. } => vec![GifMutation::SetPixelAspectRatio { ratio: base.pixel_aspect_ratio }],
            GifMutation::SetLoopCount { .. } => vec![GifMutation::SetLoopCount { loop_count: base.loop_count }],
            GifMutation::InsertFrame { index, .. } => vec![GifMutation::RemoveFrame { index: (*index).min(base.frames.len()) }],
            GifMutation::RemoveFrame { index } => match base.frames.get(*index) {
                Some(frame) => vec![GifMutation::InsertFrame { index: *index, frame: frame.clone() }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::MoveFrame { from, to } => {
                let mut frames = base.frames.clone();
                let landed_at = if *from < frames.len() {
                    let item = frames.remove(*from);
                    let at = (*to).min(frames.len());
                    frames.insert(at, item);
                    at
                } else {
                    *from
                };
                vec![GifMutation::MoveFrame { from: landed_at, to: *from }]
            }
            GifMutation::SetFrameGeometry { index, .. } => match base.frames.get(*index) {
                Some(f) => vec![GifMutation::SetFrameGeometry { index: *index, left: f.left, top: f.top, width: f.width, height: f.height }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetFramePixels { index, .. } => match base.frames.get(*index) {
                Some(f) => vec![GifMutation::SetFramePixels { index: *index, indices: f.indices.clone() }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetFrameInterlace { index, .. } => match base.frames.get(*index) {
                Some(f) => vec![GifMutation::SetFrameInterlace { index: *index, interlace: f.interlace }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetFrameDelay { index, .. } => match base.frames.get(*index) {
                Some(f) => vec![GifMutation::SetFrameDelay { index: *index, delay_cs: f.delay_cs }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetFrameDisposal { index, .. } => match base.frames.get(*index) {
                Some(f) => vec![GifMutation::SetFrameDisposal { index: *index, disposal: f.disposal }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetFrameTransparency { index, .. } => match base.frames.get(*index) {
                Some(f) => vec![GifMutation::SetFrameTransparency { index: *index, transparent_index: f.transparent_index }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::SetFrameUserInput { index, .. } => match base.frames.get(*index) {
                Some(f) => vec![GifMutation::SetFrameUserInput { index: *index, user_input: f.user_input }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::InsertComment { index, .. } => vec![GifMutation::RemoveComment { index: (*index).min(base.comments.len()) }],
            GifMutation::RemoveComment { index } => match base.comments.get(*index) {
                Some(text) => vec![GifMutation::InsertComment { index: *index, text: text.clone() }],
                None => vec![GifMutation::NoMutation],
            },
            GifMutation::AddAppExtension { index, .. } => vec![GifMutation::RemoveAppExtension { index: (*index).min(base.app_extensions.len()) }],
            GifMutation::RemoveAppExtension { index } => match base.app_extensions.get(*index) {
                Some(ext) => vec![GifMutation::AddAppExtension { index: *index, extension: ext.clone() }],
                None => vec![GifMutation::NoMutation],
            },
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — the same ~15-line body
/// every `DslOps`-derived enum's `OpText` impl uses.
impl OpText for GifMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline }).await?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record).await;
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self).await;
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline).await
    }
}

/// ⚡️ Handcrafted `OpBinary` (P6) — pure forward to `dsl::variants_binary`.
impl OpBinary for GifMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self).await
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes).await
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_frame(seed: u8) -> GifFrame {
        GifFrame {
            left: 0,
            top: 0,
            width: 2,
            height: 2,
            interlace: false,
            lct: Some(GifColorTable { sorted: false, colors: vec![crate::artifacts::gif::standards::v89a::subsets::any::schema::snapshot::GifRgb { r: seed, g: seed, b: seed }; 2] }),
            indices: vec![0, 1, 1, 0],
            delay_cs: 10,
            disposal: GifDisposal::DoNotDispose,
            transparent_index: None,
            user_input: false,
            plain_text: None,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> GifSnapshot {
        GifSnapshot {
            schema: "stdio.gif.89a".into(),
            width: 2,
            height: 2,
            gct: None,
            background_color_index: 0,
            pixel_aspect_ratio: 0,
            loop_count: Some(0),
            frames: vec![sample_frame(1), sample_frame(2), sample_frame(3)],
            comments: vec!["c0".into()],
            app_extensions: vec![GifAppExtension { identifier: *b"EXISTING", auth_code: *b"ext", data: vec![0] }],
        }
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

    /// 🧪️ `mutation_diff_law`: every variant's `diff()` matches what `apply_gif_mutation` returns.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        let base = base_snapshot();
        for mutation in [
            GifMutation::NoMutation,
            GifMutation::SetSnapshot { snapshot: GifSnapshot { loop_count: Some(9), ..base.clone() } },
            GifMutation::SetScreenSize { width: 10, height: 10 },
            GifMutation::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![Default::default(); 2] }) },
            GifMutation::SetBackgroundColorIndex { index: 5 },
            GifMutation::SetPixelAspectRatio { ratio: 3 },
            GifMutation::SetLoopCount { loop_count: None },
            GifMutation::InsertFrame { index: 1, frame: sample_frame(9) },
            GifMutation::RemoveFrame { index: 1 },
            GifMutation::MoveFrame { from: 0, to: 2 },
            GifMutation::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 },
            GifMutation::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] },
            GifMutation::SetFrameInterlace { index: 0, interlace: true },
            GifMutation::SetFrameDelay { index: 0, delay_cs: 77 },
            GifMutation::SetFrameDisposal { index: 0, disposal: GifDisposal::RestoreToBackground },
            GifMutation::SetFrameTransparency { index: 0, transparent_index: Some(1) },
            GifMutation::SetFrameUserInput { index: 0, user_input: true },
            GifMutation::InsertComment { index: 0, text: "new".into() },
            GifMutation::RemoveComment { index: 0 },
            GifMutation::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1] } },
            GifMutation::RemoveAppExtension { index: 0 },
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
        round_trips(&base, GifMutation::NoMutation);
        round_trips(&base, GifMutation::SetSnapshot { snapshot: GifSnapshot { loop_count: Some(5), ..base.clone() } });
        round_trips(&base, GifMutation::SetScreenSize { width: 8, height: 6 });
        round_trips(&base, GifMutation::SetGlobalColorTable { gct: Some(GifColorTable { sorted: false, colors: vec![Default::default(); 4] }) });
        round_trips(&base, GifMutation::SetBackgroundColorIndex { index: 2 });
        round_trips(&base, GifMutation::SetPixelAspectRatio { ratio: 9 });
        round_trips(&base, GifMutation::SetLoopCount { loop_count: Some(3) });
        round_trips(&base, GifMutation::SetLoopCount { loop_count: None });
        round_trips(&base, GifMutation::InsertFrame { index: 1, frame: sample_frame(9) });
        round_trips(&base, GifMutation::RemoveFrame { index: 1 });
        round_trips(&base, GifMutation::MoveFrame { from: 0, to: 2 });
        round_trips(&base, GifMutation::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 });
        round_trips(&base, GifMutation::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] });
        round_trips(&base, GifMutation::SetFrameInterlace { index: 2, interlace: true });
        round_trips(&base, GifMutation::SetFrameDelay { index: 0, delay_cs: 42 });
        round_trips(&base, GifMutation::SetFrameDisposal { index: 2, disposal: GifDisposal::RestoreToBackground });
        round_trips(&base, GifMutation::SetFrameTransparency { index: 0, transparent_index: Some(1) });
        round_trips(&base, GifMutation::SetFrameUserInput { index: 0, user_input: true });
        round_trips(&base, GifMutation::InsertComment { index: 0, text: "new".into() });
        round_trips(&base, GifMutation::RemoveComment { index: 0 });
        round_trips(&base, GifMutation::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1] } });
        round_trips(&base, GifMutation::RemoveAppExtension { index: 0 });
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_frame_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_gif_mutation(&mut snap, &GifMutation::RemoveFrame { index: 99 });
        assert_eq!(snap, base);
    }

    /// 🧪️ F6-PILOT: `OpText`/`OpBinary` round-trip laws over the full ~20-variant vocabulary
    /// (handcrafted impls over the `dsl::DslOps`-derived `DslVariants`).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        for mutation in [
            GifMutation::NoMutation,
            GifMutation::SetSnapshot { snapshot: GifSnapshot { loop_count: Some(9), ..base.clone() } },
            GifMutation::SetScreenSize { width: 10, height: 10 },
            GifMutation::SetGlobalColorTable { gct: Some(GifColorTable { sorted: true, colors: vec![Default::default(); 2] }) },
            GifMutation::SetGlobalColorTable { gct: None },
            GifMutation::SetBackgroundColorIndex { index: 5 },
            GifMutation::SetPixelAspectRatio { ratio: 3 },
            GifMutation::SetLoopCount { loop_count: None },
            GifMutation::InsertFrame { index: 1, frame: sample_frame(9) },
            GifMutation::RemoveFrame { index: 1 },
            GifMutation::MoveFrame { from: 0, to: 2 },
            GifMutation::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 },
            GifMutation::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] },
            GifMutation::SetFrameInterlace { index: 0, interlace: true },
            GifMutation::SetFrameDelay { index: 0, delay_cs: 77 },
            GifMutation::SetFrameDisposal { index: 0, disposal: GifDisposal::RestoreToBackground },
            GifMutation::SetFrameTransparency { index: 0, transparent_index: Some(1) },
            GifMutation::SetFrameTransparency { index: 0, transparent_index: None },
            GifMutation::SetFrameUserInput { index: 0, user_input: true },
            GifMutation::InsertComment { index: 0, text: "new".into() },
            GifMutation::RemoveComment { index: 0 },
            GifMutation::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMP Data", auth_code: *b"XMP", data: vec![1] } },
            GifMutation::RemoveAppExtension { index: 0 },
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
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/slows-the-second-frame-and-marks-it-do-not-dispose/🦀️component.rs"]
    mod tests_set_snapshot_slows_the_second_frame_and_marks_it_do_not_dispose;
}
//#endregion 🧪️FixtureTests
