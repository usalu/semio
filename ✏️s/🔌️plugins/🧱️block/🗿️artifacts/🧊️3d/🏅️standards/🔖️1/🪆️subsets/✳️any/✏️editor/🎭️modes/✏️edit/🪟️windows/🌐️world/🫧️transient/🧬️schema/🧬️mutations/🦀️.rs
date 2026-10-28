//! 🫧️ Block3d world-window transient mutation aggregate.

use crate::edit::windows::world::transient::{Block3dBrushPreviewSet, Block3dWorldWindowTransient, Block3dWorldWindowTransientDiff};

#[path = "👁️brush-preview/🦀️.rs"]
pub(crate) mod set_brush_preview;
pub use crate::edit::windows::world::transient::mutations::set_brush_preview::SetBrushPreview;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Block3dWorldWindowTransient, diff = Block3dWorldWindowTransientDiff, schema = "block.3dworldwindowtransient", reference = Block3dWorldWindowTransient, reference_diff = Block3dWorldWindowTransientDiff)]
pub enum Block3dWorldWindowTransientMutation {
    #[dsl(key = "set-brush-preview")]
    SetBrushPreview(SetBrushPreview),
}

