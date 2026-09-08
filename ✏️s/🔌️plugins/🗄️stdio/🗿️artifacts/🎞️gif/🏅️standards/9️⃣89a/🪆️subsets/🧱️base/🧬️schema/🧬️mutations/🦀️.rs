//! 🧬️ GifMutation (89a) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the full ~20-variant
//! vocabulary the plan's worked design calls for (was 6 of ~20, `apply_gif_mutation` returned
//! `()`) — screen/GCT/loop scalars, frame insert/remove/move/geometry/pixels/interlace/delay/
//! disposal/transparency/user-input, and comment/app-extension insert/remove. Every variant's
//! `diff()` is handcrafted directly against the sparse `GifDiff` shape (no apply-and-capture).

use crate::standards::v89a::subsets::any::schema::diff::{self, GifAppExtensionAdded, GifAppExtensionsDiff, GifCommentAdded, GifCommentsDiff, GifDiff, GifFrameAdded, GifFrameDiff, GifFrameModified, GifFramesDiff};
use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifSnapshot};
use protocol::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.gif.89a`.
///
/// 🧪️ F6-PILOT: `dsl::DslOps` derive — unlike `GifDiff` (blocked by tri-state fields, see the
/// `🔺️diff` module), NO mutation variant here uses `Option<Option<T>>` (a mutation's `Option<T>`
/// argument means "the new value", never a diff tri-state), so every variant's payload binds
/// cleanly. `#[dsl(block)]` on struct-valued payloads (`snapshot`, `frame`, `gct`, `extension`)
/// matches the `SpaceMutation`/`FlowMutationDsl` framework precedent's formatting convention;
/// `#[dsl(base64)]` on the one bare `Vec<u8>` payload (`SetFramePixels::indices`) keeps it compact.
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
#[path = "🔁set-loop-count/🦀️.rs"]
pub mod set_loop_count;
#[path = "🖼️insert-frame/🦀️.rs"]
pub mod insert_frame;
#[path = "🗑️remove-frame/🦀️.rs"]
pub mod remove_frame;
#[path = "🔀move-frame/🦀️.rs"]
pub mod move_frame;
#[path = "📍set-frame-geometry/🦀️.rs"]
pub mod set_frame_geometry;
#[path = "🎞️set-frame-pixels/🦀️.rs"]
pub mod set_frame_pixels;
#[path = "🪜set-frame-interlace/🦀️.rs"]
pub mod set_frame_interlace;
#[path = "⏱️set-frame-delay/🦀️.rs"]
pub mod set_frame_delay;
#[path = "♻️set-frame-disposal/🦀️.rs"]
pub mod set_frame_disposal;
#[path = "👻set-frame-transparency/🦀️.rs"]
pub mod set_frame_transparency;
#[path = "🕹️set-frame-user-input/🦀️.rs"]
pub mod set_frame_user_input;
#[path = "💬insert-comment/🦀️.rs"]
pub mod insert_comment;
#[path = "🚫remove-comment/🦀️.rs"]
pub mod remove_comment;
#[path = "🧩add-app-extension/🦀️.rs"]
pub mod add_app_extension;
#[path = "➖remove-app-extension/🦀️.rs"]
pub mod remove_app_extension;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none —
/// mirrors 87a's own migration precedent.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = GifSnapshot, diff = GifDiff, schema = "GifMutation")]
pub enum GifMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetScreenSize(set_screen_size::SetScreenSize),
    SetGlobalColorTable(set_global_color_table::SetGlobalColorTable),
    SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex),
    SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio),
    SetLoopCount(set_loop_count::SetLoopCount),
    InsertFrame(insert_frame::InsertFrame),
    RemoveFrame(remove_frame::RemoveFrame),
    MoveFrame(move_frame::MoveFrame),
    SetFrameGeometry(set_frame_geometry::SetFrameGeometry),
    SetFramePixels(set_frame_pixels::SetFramePixels),
    SetFrameInterlace(set_frame_interlace::SetFrameInterlace),
    SetFrameDelay(set_frame_delay::SetFrameDelay),
    SetFrameDisposal(set_frame_disposal::SetFrameDisposal),
    SetFrameTransparency(set_frame_transparency::SetFrameTransparency),
    SetFrameUserInput(set_frame_user_input::SetFrameUserInput),
    InsertComment(insert_comment::InsertComment),
    RemoveComment(remove_comment::RemoveComment),
    AddAppExtension(add_app_extension::AddAppExtension),
    RemoveAppExtension(remove_app_extension::RemoveAppExtension),
}
//#endregion 🔖️Mutations

//#region 🔖️Kinds
impl GifMutation {
    /// 🏷️ Kebab-case kind spelling — the exact vocabulary `.../🔣️oracle.json`'s
    /// `mutationCatalogs[].kinds` declares and the exhaustive mutation test case's Scenario Outline
    /// row ids equal. Hand-matched (never derived) so `kinds_matches_every_variant_and_manifest`
    /// below actually catches drift instead of restating the enum.
    pub fn kind(&self) -> &'static str {
        match self {
            GifMutation::SetSnapshot(_) => "set-snapshot",
            GifMutation::SetScreenSize(_) => "set-screen-size",
            GifMutation::SetGlobalColorTable(_) => "set-global-color-table",
            GifMutation::SetBackgroundColorIndex(_) => "set-background-color-index",
            GifMutation::SetPixelAspectRatio(_) => "set-pixel-aspect-ratio",
            GifMutation::SetLoopCount(_) => "set-loop-count",
            GifMutation::InsertFrame(_) => "insert-frame",
            GifMutation::RemoveFrame(_) => "remove-frame",
            GifMutation::MoveFrame(_) => "move-frame",
            GifMutation::SetFrameGeometry(_) => "set-frame-geometry",
            GifMutation::SetFramePixels(_) => "set-frame-pixels",
            GifMutation::SetFrameInterlace(_) => "set-frame-interlace",
            GifMutation::SetFrameDelay(_) => "set-frame-delay",
            GifMutation::SetFrameDisposal(_) => "set-frame-disposal",
            GifMutation::SetFrameTransparency(_) => "set-frame-transparency",
            GifMutation::SetFrameUserInput(_) => "set-frame-user-input",
            GifMutation::InsertComment(_) => "insert-comment",
            GifMutation::RemoveComment(_) => "remove-comment",
            GifMutation::AddAppExtension(_) => "add-app-extension",
            GifMutation::RemoveAppExtension(_) => "remove-app-extension",
        }
    }
}

/// 🏷️ Every declared kind, kebab-case — mirrors the catalog's `mutationCatalogs[].kinds` exactly.
pub const KINDS: &[&str] = &[
    "set-snapshot",
    "set-screen-size",
    "set-global-color-table",
    "set-background-color-index",
    "set-pixel-aspect-ratio",
    "set-loop-count",
    "insert-frame",
    "remove-frame",
    "move-frame",
    "set-frame-geometry",
    "set-frame-pixels",
    "set-frame-interlace",
    "set-frame-delay",
    "set-frame-disposal",
    "set-frame-transparency",
    "set-frame-user-input",
    "insert-comment",
    "remove-comment",
    "add-app-extension",
    "remove-app-extension",
];

#[cfg(test)]
#[path = "🧪️tests/🔬️kinds/🦀️.rs"]
mod kinds_tests;
//#endregion 🔖️Kinds

/// 🧪️ P2-FG2: representative `GifMutation` (89a) cases for `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️.rs`'s `conformance_laws` module) —
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
        schema: crate::standards::v89a::subsets::any::schema::snapshot::STDIO_GIF89A_DOCUMENT_SCHEMA.into(),
        width: 2,
        height: 2,
        gct: Some(GifColorTable { sorted: false, colors: vec![crate::standards::v89a::subsets::any::schema::snapshot::GifRgb { r: 4, g: 5, b: 6 }; 2] }),
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
        lct: Some(GifColorTable { sorted: false, colors: vec![crate::standards::v89a::subsets::any::schema::snapshot::GifRgb { r: 9, g: 9, b: 9 }; 2] }),
        indices: vec![0, 1, 1, 0],
        delay_cs: 10,
        disposal: GifDisposal::DoNotDispose,
        transparent_index: None,
        user_input: false,
        plain_text: None,
    };
    let gct_value = Some(GifColorTable { sorted: true, colors: vec![Default::default(); 2] });
    vec![
        GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: 10, height: 10 }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: gct_value }),
        GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: None }),
        GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: 5 }),
        GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: 3 }),
        GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: Some(7) }),
        GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: None }),
        GifMutation::InsertFrame(insert_frame::InsertFrame { index: 1, frame: sample_frame }),
        GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: 1 }),
        GifMutation::MoveFrame(move_frame::MoveFrame { from: 0, to: 1 }),
        GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: 0, left: 1, top: 1, width: 2, height: 2 }),
        GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 0, indices: vec![1, 1, 1, 1] }),
        GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index: 0, interlace: true }),
        GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_cs: 77 }),
        GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index: 0, disposal: GifDisposal::RestoreToBackground }),
        GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: 0, transparent_index: Some(1) }),
        GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: 0, transparent_index: None }),
        GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index: 0, user_input: true }),
        GifMutation::InsertComment(insert_comment::InsertComment { index: 0, text: "newc".into() }),
        GifMutation::RemoveComment(remove_comment::RemoveComment { index: 0 }),
        GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: 0, extension: GifAppExtension { identifier: *b"XMPDATA1", auth_code: *b"XMP", data: vec![1] } }),
        GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: 0 }),
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

/// ↩️ This subset's own inverse algebra as a free function, so a caller driving the vocabulary from
/// outside the crate reaches it without naming `protocol::Mutation` — `protocol` is an
/// `extern crate` ALIAS private to this plugin's glue (`🦀️.rs`), so that trait's name simply
/// does not exist for a dependent. Same shape and same reason as `inverse_png_mutation`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
        GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count }) => GifDiff { loop_count: (*loop_count != base.loop_count).then_some(*loop_count), ..Default::default() },
        GifMutation::InsertFrame(insert_frame::InsertFrame { index, frame }) => GifDiff { frames: Some(GifFramesDiff { added: vec![GifFrameAdded { index: (*index).min(base.frames.len()), frame: frame.clone() }], ..Default::default() }), ..Default::default() },
        GifMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => GifDiff { frames: Some(GifFramesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        GifMutation::MoveFrame(move_frame::MoveFrame { from, to }) => {
            let mut frames = base.frames.clone();
            if *from < frames.len() {
                let item = frames.remove(*from);
                let at = (*to).min(frames.len());
                frames.insert(at, item);
            }
            GifDiff { frames: Some(GifFramesDiff::between(&base.frames, &frames)), ..Default::default() }
        }
        GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index, left, top, width, height }) => {
            let d = GifFrameDiff { left: Some(*left), top: Some(*top), width: Some(*width), height: Some(*height), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, indices }) => {
            let d = GifFrameDiff { indices: Some(indices.clone()), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index, interlace }) => {
            let d = GifFrameDiff { interlace: Some(*interlace), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, delay_cs }) => {
            let d = GifFrameDiff { delay_cs: Some(*delay_cs), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index, disposal }) => {
            let d = GifFrameDiff { disposal: Some(*disposal), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index, transparent_index }) => {
            let d = GifFrameDiff { transparent_index: Some(*transparent_index), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index, user_input }) => {
            let d = GifFrameDiff { user_input: Some(*user_input), ..Default::default() };
            GifDiff { frames: Some(GifFramesDiff { modified: vec![GifFrameModified { index: *index, diff: d }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::InsertComment(insert_comment::InsertComment { index, text }) => GifDiff { comments: Some(GifCommentsDiff { added: vec![GifCommentAdded { index: (*index).min(base.comments.len()), text: text.clone() }], ..Default::default() }), ..Default::default() },
        GifMutation::RemoveComment(remove_comment::RemoveComment { index }) => GifDiff { comments: Some(GifCommentsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index, extension }) => {
            GifDiff { app_extensions: Some(GifAppExtensionsDiff { added: vec![GifAppExtensionAdded { index: (*index).min(base.app_extensions.len()), extension: extension.clone() }], ..Default::default() }), ..Default::default() }
        }
        GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index }) => GifDiff { app_extensions: Some(GifAppExtensionsDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
    })
}

/// ↩️ Real, round-trippable inverses: `apply(inverse(m, base), apply(m, base)) == base` for
/// every variant, including the frame/comment/extension-index ops. A target that no longer
/// exists (an out-of-range index) inverts to the EMPTY step list — mirrors 87a's own precedent
/// now that `NoMutation` no longer exists as a stand-in.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &GifMutation, base: &GifSnapshot) -> Vec<GifMutation> {
    match this {
        GifMutation::SetSnapshot(_) => vec![GifMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        GifMutation::SetScreenSize(_) => vec![GifMutation::SetScreenSize(set_screen_size::SetScreenSize { width: base.width, height: base.height })],
        GifMutation::SetGlobalColorTable(_) => vec![GifMutation::SetGlobalColorTable(set_global_color_table::SetGlobalColorTable { gct: base.gct.clone() })],
        GifMutation::SetBackgroundColorIndex(_) => vec![GifMutation::SetBackgroundColorIndex(set_background_color_index::SetBackgroundColorIndex { index: base.background_color_index })],
        GifMutation::SetPixelAspectRatio(_) => vec![GifMutation::SetPixelAspectRatio(set_pixel_aspect_ratio::SetPixelAspectRatio { ratio: base.pixel_aspect_ratio })],
        GifMutation::SetLoopCount(_) => vec![GifMutation::SetLoopCount(set_loop_count::SetLoopCount { loop_count: base.loop_count })],
        GifMutation::InsertFrame(insert_frame::InsertFrame { index, .. }) => vec![GifMutation::RemoveFrame(remove_frame::RemoveFrame { index: (*index).min(base.frames.len()) })],
        GifMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => match base.frames.get(*index) {
            Some(frame) => vec![GifMutation::InsertFrame(insert_frame::InsertFrame { index: *index, frame: frame.clone() })],
            None => Vec::new(),
        },
        GifMutation::MoveFrame(move_frame::MoveFrame { from, to }) => {
            let mut frames = base.frames.clone();
            let landed_at = if *from < frames.len() {
                let item = frames.remove(*from);
                let at = (*to).min(frames.len());
                frames.insert(at, item);
                at
            } else {
                *from
            };
            vec![GifMutation::MoveFrame(move_frame::MoveFrame { from: landed_at, to: *from })]
        }
        GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index, .. }) => match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameGeometry(set_frame_geometry::SetFrameGeometry { index: *index, left: f.left, top: f.top, width: f.width, height: f.height })],
            None => Vec::new(),
        },
        GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, .. }) => match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: *index, indices: f.indices.clone() })],
            None => Vec::new(),
        },
        GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index, .. }) => match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameInterlace(set_frame_interlace::SetFrameInterlace { index: *index, interlace: f.interlace })],
            None => Vec::new(),
        },
        GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, .. }) => match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: *index, delay_cs: f.delay_cs })],
            None => Vec::new(),
        },
        GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index, .. }) => match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameDisposal(set_frame_disposal::SetFrameDisposal { index: *index, disposal: f.disposal })],
            None => Vec::new(),
        },
        GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index, .. }) => match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameTransparency(set_frame_transparency::SetFrameTransparency { index: *index, transparent_index: f.transparent_index })],
            None => Vec::new(),
        },
        GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index, .. }) => match base.frames.get(*index) {
            Some(f) => vec![GifMutation::SetFrameUserInput(set_frame_user_input::SetFrameUserInput { index: *index, user_input: f.user_input })],
            None => Vec::new(),
        },
        GifMutation::InsertComment(insert_comment::InsertComment { index, .. }) => vec![GifMutation::RemoveComment(remove_comment::RemoveComment { index: (*index).min(base.comments.len()) })],
        GifMutation::RemoveComment(remove_comment::RemoveComment { index }) => match base.comments.get(*index) {
            Some(text) => vec![GifMutation::InsertComment(insert_comment::InsertComment { index: *index, text: text.clone() })],
            None => Vec::new(),
        },
        GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index, .. }) => vec![GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index: (*index).min(base.app_extensions.len()) })],
        GifMutation::RemoveAppExtension(remove_app_extension::RemoveAppExtension { index }) => match base.app_extensions.get(*index) {
            Some(ext) => vec![GifMutation::AddAppExtension(add_app_extension::AddAppExtension { index: *index, extension: ext.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Handcrafted `OpText` (P6: `dsl::DslOps` emits `DslVariants` only) — the same ~15-line body
/// every `DslOps`-derived enum's `OpText` impl uses.
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
