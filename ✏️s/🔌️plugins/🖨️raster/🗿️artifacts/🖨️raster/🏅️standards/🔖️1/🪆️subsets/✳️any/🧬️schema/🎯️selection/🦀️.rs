//! 🎯️ Shared immutable raster coverage belongs to document edits and session selection alike.
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::ToValue,semio_framework_value::FromValue,semio_framework_dsl_record_derive::DslRecord,schema::ArtifactSchema,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree,semio_framework_value::RetainedClone,semio_framework_value::NativeReceive,semio_framework_dsl_record_derive::DslOwnedReceiving)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all="camelCase")]
#[dsl(keyword="selection-span")]
#[artifact_schema(id="s.raster.raster.selectionspan")]
pub struct RasterSelectionSpan{
    #[state(config)]
    pub start:u32,
    #[state(config)]
    pub length:u32,
    #[state(config)]
    pub coverage:u32,
}

#[path="📦️coverage/🦀️.rs"]
pub mod coverage;
pub use coverage::{RasterSelectionSpans,RasterSelectionSpansBuilder,RasterSpanPage,RASTER_SELECTION_PAGE_ROWS,RASTER_SELECTION_PAGE_BYTES};
