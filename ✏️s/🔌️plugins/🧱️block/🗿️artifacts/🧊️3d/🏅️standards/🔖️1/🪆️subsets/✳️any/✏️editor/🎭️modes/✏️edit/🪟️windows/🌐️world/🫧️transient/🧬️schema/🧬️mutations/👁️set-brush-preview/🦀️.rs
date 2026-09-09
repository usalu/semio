//! 👁️ Sets the brush preview for one concrete Block3d world window.

use super::{Block3dWorldWindowTransient, Block3dWorldWindowTransientMutation};
use crate::editor::block3d::modes::edit::windows::world::transient::Block3dBrushPreview;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-brush-preview")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBrushPreview {
    #[dsl(block)]
    pub preview: Option<Block3dBrushPreview>,
}

impl protocol::MutationKind<Block3dWorldWindowTransient, Block3dWorldWindowTransientMutation> for SetBrushPreview {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "world-window-brush-preview", kind: "set-brush-preview", record: "SetBrushPreview" };

    fn diff(&self, base: &Block3dWorldWindowTransient) -> protocol::MutationOutcome<Block3dWorldWindowTransient> {
        let mut next = base.clone();
        next.brush_preview.clone_from(&self.preview);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Block3dWorldWindowTransient) -> Vec<Block3dWorldWindowTransientMutation> {
        vec![Self { preview: base.brush_preview.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set World Window Brush Preview".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["brushPreview".into()]
    }
}
