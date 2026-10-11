//! 🧬️ GifMutation (89a) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: the full ~20-variant
//! vocabulary the plan's worked design calls for (was 6 of ~20, `apply_gif_mutation` returned
//! `()`) — screen/GCT/loop scalars, frame insert/remove/move/geometry/pixels/interlace/delay/
//! disposal/transparency/user-input, and comment/app-extension insert/remove. Every variant's
//! `diff()` is handcrafted directly against the sparse `GifDiff` shape (no apply-and-capture).

use crate::standards::v89a::subsets::any::schema::diff::{GifAppExtensionAdded, GifAppExtensionsDiff, GifCommentAdded, GifCommentsDiff, GifDiff, GifFrameAdded, GifFrameDiff, GifFrameModified, GifFramesDiff};
use crate::standards::v89a::subsets::any::schema::snapshot::{GifAppExtension, GifColorTable, GifDisposal, GifFrame, GifSnapshot};
use protocol::Mutation;


//#region 🔖️Mutations
#[path = "🧩add-app-extension/🦀️.rs"]
pub mod add_app_extension;
#[path = "💬insert-comment/🦀️.rs"]
pub mod insert_comment;
#[path = "🖼️insert-frame/🦀️.rs"]
pub mod insert_frame;
#[path = "🔀move-frame/🦀️.rs"]
pub mod move_frame;
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "➖remove-app-extension/🦀️.rs"]
pub mod remove_app_extension;
#[path = "🚫remove-comment/🦀️.rs"]
pub mod remove_comment;
#[path = "🗑️remove-frame/🦀️.rs"]
pub mod remove_frame;
#[path = "🖌️set-background-color-index/🦀️.rs"]
pub mod set_background_color_index;
#[path = "⏱️set-frame-delay/🦀️.rs"]
pub mod set_frame_delay;
#[path = "♻️set-frame-disposal/🦀️.rs"]
pub mod set_frame_disposal;
#[path = "📍set-frame-geometry/🦀️.rs"]
pub mod set_frame_geometry;
#[path = "🪜set-frame-interlace/🦀️.rs"]
pub mod set_frame_interlace;
#[path = "🎞️set-frame-pixels/🦀️.rs"]
pub mod set_frame_pixels;
#[path = "👻set-frame-transparency/🦀️.rs"]
pub mod set_frame_transparency;
#[path = "🕹️set-frame-user-input/🦀️.rs"]
pub mod set_frame_user_input;
#[path = "🎨set-global-color-table/🦀️.rs"]
pub mod set_global_color_table;
#[path = "🔁set-loop-count/🦀️.rs"]
pub mod set_loop_count;
#[path = "📏set-pixel-aspect-ratio/🦀️.rs"]
pub mod set_pixel_aspect_ratio;
#[path = "📐set-screen-size/🦀️.rs"]
pub mod set_screen_size;
/// 📐️ Typed content mutation for `stdio.gif.89a`.
///
/// 🧪️ F6-PILOT: `dsl::DslOps` derive — unlike `GifDiff` (blocked by tri-state fields, see the
/// `🔺️diff` module), NO mutation variant here uses `Option<Option<T>>` (a mutation's `Option<T>`
/// argument means "the new value", never a diff tri-state), so every variant's payload binds
/// cleanly. `#[dsl(block)]` on struct-valued payloads (`snapshot`, `frame`, `gct`, `extension`)
/// matches the `SpaceMutation`/`FlowMutationDsl` framework precedent's formatting convention;
/// `#[dsl(base64)]` on the one bare `Vec<u8>` payload (`SetFramePixels::indices`) keeps it compact.
//#region 🔖️Leaves
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none —
/// mirrors 87a's own migration precedent.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = GifSnapshot, diff = GifDiff, schema = "GifMutation")]
pub enum GifMutation {
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
/// every one of the 19 real variants, incl. `Some`/`None` shapes of every `Option<T>` field
/// (`gct`, `loop_count`, `transparent_index`) — mirrors 87a's own `demo_mutation_cases()`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<GifMutation> {
    // 🧭️ Deliberately a small, hand-built snapshot — NOT
    // `engine::demo_gif_snapshot()` (the real, 800×800/54-frame `dancing.gif` fixture used by
    // the snapshot-facet conformance laws): embedding that full fixture in every case
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
#[cfg(test)]
pub fn apply_gif_mutation(snapshot: &mut GifSnapshot, mutation: &GifMutation) -> protocol::MutationOutcome<GifDiff> {
    let outcome = <GifMutation as Mutation<GifSnapshot>>::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply

//#region 🔖️MutationTrait

//#region 🔖️RasterGuard
/// 🧱️ Whether `frame` carries an image at all — a plain-text-only frame has no rectangle and no indices.
fn carries_image(frame: &GifFrame) -> bool {
    !(frame.plain_text.is_some() && frame.width == 0 && frame.height == 0 && frame.indices.is_empty())
}

/// 📐️ §20: the frame's rectangle lies inside the Logical Screen.
fn frame_fits(index: usize, frame: &GifFrame, (width, height): (u32, u32)) -> Option<(String, Vec<String>)> {
    (carries_image(frame) && (u64::from(frame.left) + u64::from(frame.width) > u64::from(width) || u64::from(frame.top) + u64::from(frame.height) > u64::from(height)))
        .then(|| (format!("frame {index} at ({}, {}) sized {}x{} would not fit the {width}x{height} Logical Screen (GIF89a §20)", frame.left, frame.top, frame.width, frame.height), vec!["frames".to_string(), index.to_string()]))
}

/// 🔢️ §22: one index per pixel of the frame's rectangle.
fn frame_covers(index: usize, frame: &GifFrame) -> Option<(String, Vec<String>)> {
    (carries_image(frame) && frame.indices.len() as u64 != u64::from(frame.width) * u64::from(frame.height))
        .then(|| (format!("frame {index} would declare {}x{} pixels over {} indices (GIF89a §22: one index per pixel)", frame.width, frame.height, frame.indices.len()), vec!["frames".to_string(), index.to_string(), "indices".to_string()]))
}

/// 🎨️ §19/§21/§22: every index addresses an entry of the frame's active colour table.
fn frame_colored(index: usize, frame: &GifFrame, gct: Option<&GifColorTable>) -> Option<(String, Vec<String>)> {
    let colors = frame.lct.as_ref().or(gct).map_or(0, |table| table.colors.len());
    frame.indices.iter().max().filter(|max| usize::from(**max) >= colors).map(|max| (format!("frame {index} uses colour index {max}, past its {colors}-entry active colour table (GIF89a §22)"), vec!["frames".to_string(), index.to_string(), "indices".to_string()]))
}
//#endregion 🔖️RasterGuard

//#endregion 🔖️MutationTrait

//#region OpCodecs



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

#[cfg(test)]
use protocol::{OpBinary,OpText};
