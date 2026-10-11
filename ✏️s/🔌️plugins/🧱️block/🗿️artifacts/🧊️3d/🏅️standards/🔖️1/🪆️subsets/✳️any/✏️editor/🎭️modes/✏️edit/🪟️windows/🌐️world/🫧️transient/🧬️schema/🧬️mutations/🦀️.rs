//! 🫧️ Block3d world-window transient mutation aggregate.

use super::{Block3dBrushPreviewSet, Block3dWorldWindowTransient, Block3dWorldWindowTransientDiff};

#[path = "👁️set-brush-preview/🦀️.rs"]
mod set_brush_preview;
pub use set_brush_preview::SetBrushPreview;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Block3dWorldWindowTransient, diff = Block3dWorldWindowTransientDiff, schema = "block.3dworldwindowtransient")]
pub enum Block3dWorldWindowTransientMutation {
    #[dsl(key = "set-brush-preview")]
    SetBrushPreview(SetBrushPreview),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
