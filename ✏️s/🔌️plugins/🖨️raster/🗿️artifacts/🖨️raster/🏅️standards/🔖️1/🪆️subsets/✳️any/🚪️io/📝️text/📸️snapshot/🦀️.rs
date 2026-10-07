//! 📜️ Raster artifact — textual document grammar surface + laws (constitutional: dsl).

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::RasterSnapshot;

/// 📄️ The `semio` example document, handcrafted in the `.raster` DSL.
pub const SEMIO_RASTER_EXAMPLE_TEXT: &str = include_str!("../../../🖼️assets/🎬️demo/🗣️.dsl.semio");

/// 📖️ Parses `.raster` DSL text into a `RasterSnapshot`.
pub fn parse_dsl(text: &str) -> Result<RasterSnapshot, semio_framework_diagnostic::TextError> {
    <RasterSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `RasterSnapshot` back to `.raster` DSL text.
pub fn print_dsl(document: &RasterSnapshot) -> String {
    store::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type RasterSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::snapshot::*;
use crate::{RasterAssetChild, RasterLayerMask, RasterLayerNode, RasterOwnedMap, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use super::record::RasterNativeDocument;

/// 🖨️ Prints the owner's literal typed forest and intrinsic records.
pub(crate) fn print_pack_record_text(snapshot:&RasterSnapshot)->String{
 semio_framework_dsl_record::print(&RasterNativeDocument::ordinary(snapshot).__dsl_to_record(),&RasterNativeDocument::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document)
}

/// 📖️ Reconstructs the literal native forest without an embedded layer or child container.
pub(crate) fn parse_pack_record_text(body:&str)->Result<RasterSnapshot,semio_framework_diagnostic::TextError>{
 let record=semio_framework_dsl_record::parse(body,&RasterNativeDocument::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;
 RasterNativeDocument::__dsl_from_record(&record)?.ordinary_snapshot().map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error,semio_framework_diagnostic::TextSpan::at(1,1)))
}

/// ✉️ Native text and pack encode the same literal typed Raster graph.
impl store::ArtifactDsl for RasterSnapshot {
    const EXTENSION: &'static str = "raster";
    fn envelope_id() -> &'static str {
        "raster.raster"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::RasterDiff;
use crate::RasterSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::add_layer_asset;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_adjustment_kind;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_blend_mode;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_opacity;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_visible;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_locked;
use crate::standards::v1::subsets::any::schema::mutations::create_layer;
use crate::standards::v1::subsets::any::schema::mutations::delete_layer;
use crate::standards::v1::subsets::any::schema::mutations::move_layer;
use crate::standards::v1::subsets::any::schema::mutations::remove_layer_asset;
use crate::standards::v1::subsets::any::schema::mutations::rename_layer;
use crate::standards::v1::subsets::any::schema::mutations::reorder_layers;
use crate::standards::v1::subsets::any::schema::mutations::resize_layer;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_pixels;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_mask;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_transform;
use crate::standards::v1::subsets::any::schema::mutations::change_layer_adjustment_parameter;
use crate::standards::v1::subsets::any::schema::mutations::paint_stroke;
use crate::standards::v1::subsets::any::schema::mutations::fill_region;
use crate::standards::v1::subsets::any::schema::mutations::apply_filter;
use crate::standards::v1::subsets::any::schema::mutations::transform_image;
use crate::standards::v1::subsets::any::schema::mutations::fill_selection;

/// 🔁️ Parses the committed `.dsl.semio` example, prints it back and parses that, answering
/// `{"printed": …, "snapshot": …, "reparsed": …}` so a caller can weigh the identity law's two
/// halves — the bytes against the committed artifact, and the projection against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn round_trip_raster_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <RasterSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed raster example does not parse: {error:?}"))?;
    let printed = <RasterSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <RasterSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted raster document does not parse: {error:?}"))?;
    let value = semio_framework_pack_json::object([
        ("printed".to_string(), semio_framework_pack_json::Value::from(printed)),
        ("snapshot".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&parsed))),
        ("reparsed".to_string(), semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&reparsed))),
    ]);
    Ok(semio_framework_pack_json::to_string(&value))
}
}
pub use mutations_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use crate::standards::v1::subsets::any::schema::*;
use crate::{RasterAssetChild, RasterOwnedMap, RASTER_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
/// 🌱️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 3: pure helpers over document types live in `🧬️schema/`). Every external call site now reads
/// `crate::standards::v1::subsets::any::schema::…` (the artifact root's own pre-existing `pub mod schema { pub
/// use crate::standards::v1::subsets::any::schema::standards::v1::subsets::any::schema::*; }` shim keeps that path resolving).
use crate::{RasterSnapshot, RasterTransform};
#[cfg(test)]
use crate::standards::v1::subsets::any::schema::testing::raster_image_test_snapshot;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use crate::RasterImageAsset;
use crate::RasterLayerNode;
use crate::RasterViewportSize;



pub fn empty_raster_snapshot() -> RasterSnapshot {
    RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: "raster".into(), title: Some("Untitled".into()), layers: Vec::new(), assets: RasterOwnedMap::new() }
}

/// 🖼️ `pub` (not `fn` as it was inside `⚙️engine`, where crate-locality made privacy moot): now called
/// cross-module from `🚪️io/🦀️.rs`'s `MediaImport` region (`raster_document_from_dwg_drawing`,
/// `raster_image_layer_and_asset`), which need a specific name/width/height rather than
/// `create_layer_of_kind`'s generic defaults.


pub fn empty_raster_document() -> RasterSnapshot {
    let mut document = empty_raster_snapshot();
    document.id = "empty".into();
    document.layers = vec![create_pixel_layer("Background", 512, 512)];
    document
}

/// 📄️ The demo document every raster surface lands on right after boot (the shell replays
/// `setActiveExample demo`; the store itself boots on [`empty_raster_document`], see
/// `RasterPlayApp::initial_snapshot`): the committed `📚️examples/🎬️demo` Semio-logo carrier, read through the artifact's own text codec so the `.dsl.semio` asset stays the single
/// source of truth instead of being restated in Rust. Falls back to [`empty_raster_document`] when
/// the carrier does not parse — the same shape `block2d`'s `default_block2d_snapshot` uses.
pub fn default_raster_document() -> RasterSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::examples::art_raster_demo::PRIMARY_TEXT).unwrap_or_else(|_| empty_raster_document())
}
}
pub use snapshot_wire_codec::*;

pub fn raster_example_document(example_id: &str) -> Option<RasterSnapshot> {
    (example_id == crate::examples::art_raster_demo::ID).then(default_raster_document)
}

#[path="📦️record/🦀️.rs"]
pub(crate) mod record;
