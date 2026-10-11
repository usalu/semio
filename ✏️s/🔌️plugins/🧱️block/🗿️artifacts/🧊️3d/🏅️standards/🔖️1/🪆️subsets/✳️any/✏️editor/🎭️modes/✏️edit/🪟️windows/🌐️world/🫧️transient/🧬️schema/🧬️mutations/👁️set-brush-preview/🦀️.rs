//! 👁️ Sets the brush preview for one concrete Block3d world window.

use super::{Block3dBrushPreviewSet, Block3dWorldWindowTransient, Block3dWorldWindowTransientDiff, Block3dWorldWindowTransientMutation};
use crate::editor::block3d::modes::edit::windows::world::transient::Block3dBrushPreview;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-brush-preview")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBrushPreview {
    #[dsl(block)]
    pub preview: Option<Block3dBrushPreview>,
}

impl protocol::MutationKind<Block3dWorldWindowTransient, Block3dWorldWindowTransientMutation> for SetBrushPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "world-window-brush-preview", kind: "set-brush-preview", record: "SetBrushPreview" };

    fn diff(&self, base: &Block3dWorldWindowTransient) -> protocol::MutationOutcome<Block3dWorldWindowTransientDiff> {
        if base.brush_preview == self.preview {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The brush preview already holds that value.");
        }
        protocol::MutationOutcome::new(Block3dWorldWindowTransientDiff { brush_preview: Some(Block3dBrushPreviewSet { value: self.preview.clone() }) })
    }

    fn inverse(&self, base: &Block3dWorldWindowTransient) -> Result<Vec<Block3dWorldWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { preview: base.brush_preview.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set World Window Brush Preview", "Pinselvorschau im Weltfenster setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["brushPreview".into()]
    }
}
