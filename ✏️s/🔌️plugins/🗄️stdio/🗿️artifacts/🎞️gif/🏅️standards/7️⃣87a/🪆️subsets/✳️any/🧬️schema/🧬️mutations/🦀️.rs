//! 🧬️ GifMutation (87a) — document mutation dispatch. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: 87a's real vocabulary covers everything GIF87a actually has —
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
//! `impl Mutation<GifSnapshot> for GifMutation` block is gone; every leaf's own `MutationKind::diff`/`inverse`
//! builds its sparse diff and concrete inverse from its payload and reads of `base`.
//!
//! `#[derive(dsl::DslOps)]` is KEPT alongside `#[derive(dsl::Mutations)]`: the hand-rolled
//! `OpText`/`OpBinary` impls below (P6: `DslOps` emits `DslVariants` only) still need
//! `dsl::DslVariants`, and `dsl_variants_codegen`'s single-tuple-field branch delegates a newtype
//! variant's `RecordSpec` to its inner type's own `DslField` impl — which is why every leaf payload
//! struct below derives `dsl::DslRecord` in addition to `dsl::MutationLeaf`.

use crate::standards::v87a::subsets::any::schema::diff::{GifDiff, GifImageAdded, GifImageDiff, GifImageModified, GifImagesDiff};
#[cfg(test)]
use crate::standards::v87a::subsets::any::schema::snapshot::GifRgb;
use crate::standards::v87a::subsets::any::schema::snapshot::{GifColorTable, GifImage, GifSnapshot};
use protocol::Mutation;


//#region 🔖️Mutations
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "🖼️insert-image/🦀️.rs"]
pub mod insert_image;
#[path = "🔀move-image/🦀️.rs"]
pub mod move_image;
#[path = "🗑️remove-image/🦀️.rs"]
pub mod remove_image;
#[path = "🖌️set-background-color-index/🦀️.rs"]
pub mod set_background_color_index;
#[path = "🎨set-global-color-table/🦀️.rs"]
pub mod set_global_color_table;
#[path = "📍set-image-geometry/🦀️.rs"]
pub mod set_image_geometry;
#[path = "🪜set-image-interlace/🦀️.rs"]
pub mod set_image_interlace;
#[path = "🎞️set-image-pixels/🦀️.rs"]
pub mod set_image_pixels;
#[path = "📏set-pixel-aspect-ratio/🦀️.rs"]
pub mod set_pixel_aspect_ratio;
#[path = "📐set-screen-size/🦀️.rs"]
pub mod set_screen_size;

/// 📐️ Typed mutation for this artifact. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "mutation", rename_all = "camelCase")]
#[mutations(snapshot = GifSnapshot, diff = GifDiff, schema = "GifMutation")]
pub enum GifMutation {
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
pub const KINDS: &[&str] =
    &["set-screen-size", "set-global-color-table", "set-background-color-index", "set-pixel-aspect-ratio", "insert-image", "remove-image", "move-image", "set-image-geometry", "set-image-pixels", "set-image-interlace"];
//#endregion 🔖️Mutations

/// 🧪️ P2-FG2: representative `GifMutation` cases for `ops_grammar_conformance_law`/
/// `protocol_walk_law` (`../../../../⚙️engine/🦀️.rs`'s `conformance_laws` module) —
/// every one of the 11 real variants, incl. both `Some`/`None` shapes of the one
/// `Option<T>`-of-struct-block field (`SetGlobalColorTable::gct`) — mirrors png's own
/// `demo_mutation_cases()`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<GifMutation> {
    let base = crate::standards::v87a::subsets::any::schema::demo_gif_snapshot();
    let sample_image = GifImage { left: 0, top: 0, width: 2, height: 2, interlace: false, lct: Some(GifColorTable { sorted: false, colors: vec![GifRgb { r: 9, g: 9, b: 9 }; 2] }), indices: vec![0, 1, 1, 0] };
    vec![
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


//#region 🔖️RasterGuard
/// 📐️ The image's rectangle lies inside the screen the Screen Descriptor defines.
fn image_fits(index: usize, image: &GifImage, (width, height): (u32, u32)) -> Option<(String, Vec<String>)> {
    (u64::from(image.left) + u64::from(image.width) > u64::from(width) || u64::from(image.top) + u64::from(image.height) > u64::from(height))
        .then(|| (format!("image {index} at ({}, {}) sized {}x{} would not be confined to the {width}x{height} screen (GIF87a Image Descriptor)", image.left, image.top, image.width, image.height), vec!["images".to_string(), index.to_string()]))
}

/// 🔢️ The raster carries image-width*image-height pixel indices.
fn image_covers(index: usize, image: &GifImage) -> Option<(String, Vec<String>)> {
    (image.indices.len() as u64 != u64::from(image.width) * u64::from(image.height))
        .then(|| (format!("image {index} would declare {}x{} pixels over {} indices (GIF87a Raster Data)", image.width, image.height, image.indices.len()), vec!["images".to_string(), index.to_string(), "indices".to_string()]))
}

/// 🎨️ Every pixel index addresses an entry of the image's active colour map.
fn image_colored(index: usize, image: &GifImage, gct: Option<&GifColorTable>) -> Option<(String, Vec<String>)> {
    let colors = image.lct.as_ref().or(gct).map_or(0, |table| table.colors.len());
    image.indices.iter().max().filter(|max| usize::from(**max) >= colors).map(|max| (format!("image {index} uses colour index {max}, past its {colors}-entry active colour map (GIF87a Color Map)"), vec!["images".to_string(), index.to_string(), "indices".to_string()]))
}
//#endregion 🔖️RasterGuard

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
