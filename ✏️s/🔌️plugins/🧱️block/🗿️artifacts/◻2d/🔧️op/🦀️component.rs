//! ⚡️ Block 2D artifact — the operation enum, its `Operation` law and the store aliases
//! (constitutional: op).

use crate::artifacts::block2d::diff::{block2d_index_of, Block2dDiff};
use crate::artifacts::block2d::{Block2dDefinition, Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation};
use crate::core::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use protocol::Operation;
use serde::{Deserialize, Serialize};

// #region 🔖️Operation
/// 🧮️ Block-2d operation: id-keyed table edits plus scalar node_kind/presentation/camera2d/meta, each
/// with a true inverse computed from the pre-operation projection, and a whole-document replace for
/// example loads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Block2dOperation {
    #[dsl(key = "setNodeKind")]
    SetNodeKind {
        #[dsl(block)]
        node_kind: BlockKindIdentity,
    },
    #[dsl(key = "setPresentation")]
    SetPresentation {
        #[dsl(block)]
        presentation: Block2dPresentation,
    },
    #[dsl(key = "setHandleKind")]
    SetHandleKind {
        index: usize,
        #[dsl(block)]
        handle_kind: Block2dHandleKind,
    },
    #[dsl(key = "removeHandleKind")]
    RemoveHandleKind { id: String },
    #[dsl(key = "setHandle")]
    SetHandle {
        index: usize,
        #[dsl(block)]
        handle: Block2dHandleTemplate,
    },
    #[dsl(key = "removeHandle")]
    RemoveHandle { id: String },
    #[dsl(key = "setCompatibilityRule")]
    SetCompatibilityRule {
        index: usize,
        #[dsl(block)]
        rule: BlockCompatibilityRule,
    },
    #[dsl(key = "removeCompatibilityRule")]
    RemoveCompatibilityRule { id: String },
    #[dsl(key = "setAttribute")]
    SetAttribute {
        index: usize,
        #[dsl(block)]
        attribute: BlockAttribute,
    },
    #[dsl(key = "removeAttribute")]
    RemoveAttribute { key: String },
    #[dsl(key = "setAuthors")]
    SetAuthors { authors: Vec<BlockAuthor> },
    #[dsl(key = "setCamera2d")]
    SetCamera2d {
        #[dsl(block)]
        camera2d: BlockCamera2d,
    },
    #[dsl(key = "setMeta")]
    SetMeta {
        #[dsl(block)]
        meta: BlockMeta,
    },
    /// 🌍️ Replaces the whole document (example import / reset).
    #[dsl(key = "setDocument")]
    SetDocument {
        #[dsl(block)]
        document: Block2dDefinition,
    },
}

fn block2d_operation_diff(operation: &Block2dOperation) -> Block2dDiff {
    let mut diff = Block2dDiff::default();
    match operation {
        Block2dOperation::SetNodeKind { node_kind } => diff.node_kind = Some(node_kind.clone()),
        Block2dOperation::SetPresentation { presentation } => diff.presentation = Some(presentation.clone()),
        Block2dOperation::SetHandleKind { index, handle_kind } => diff.handle_kinds.set.push((*index, handle_kind.clone())),
        Block2dOperation::RemoveHandleKind { id } => diff.handle_kinds.removed.push(id.clone()),
        Block2dOperation::SetHandle { index, handle } => diff.handles.set.push((*index, handle.clone())),
        Block2dOperation::RemoveHandle { id } => diff.handles.removed.push(id.clone()),
        Block2dOperation::SetCompatibilityRule { index, rule } => diff.compatibility.set.push((*index, rule.clone())),
        Block2dOperation::RemoveCompatibilityRule { id } => diff.compatibility.removed.push(id.clone()),
        Block2dOperation::SetAttribute { index, attribute } => diff.attributes.set.push((*index, attribute.clone())),
        Block2dOperation::RemoveAttribute { key } => diff.attributes.removed.push(key.clone()),
        Block2dOperation::SetAuthors { authors } => diff.authors = Some(authors.clone()),
        Block2dOperation::SetCamera2d { camera2d } => diff.camera2d = Some(camera2d.clone()),
        Block2dOperation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Block2dOperation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Operation<Block2dDefinition> for Block2dOperation {
    type Diff = Block2dDiff;

    fn diff(&self, _projection: &Block2dDefinition) -> Block2dDiff {
        block2d_operation_diff(self)
    }

    fn backwards(&self, projection: &Block2dDefinition) -> Vec<Self> {
        match self {
            Block2dOperation::SetNodeKind { .. } => vec![Block2dOperation::SetNodeKind { node_kind: projection.node_kind.clone() }],
            Block2dOperation::SetPresentation { .. } => vec![Block2dOperation::SetPresentation { presentation: projection.presentation.clone() }],
            Block2dOperation::SetHandleKind { handle_kind, .. } => match block2d_index_of(&projection.handle_kinds, &handle_kind.id) {
                Some(index) => vec![Block2dOperation::SetHandleKind { index, handle_kind: projection.handle_kinds[index].clone() }],
                None => vec![Block2dOperation::RemoveHandleKind { id: handle_kind.id.clone() }],
            },
            Block2dOperation::RemoveHandleKind { id } => block2d_index_of(&projection.handle_kinds, id).map(|index| vec![Block2dOperation::SetHandleKind { index, handle_kind: projection.handle_kinds[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetHandle { handle, .. } => match block2d_index_of(&projection.handles, &handle.id) {
                Some(index) => vec![Block2dOperation::SetHandle { index, handle: projection.handles[index].clone() }],
                None => vec![Block2dOperation::RemoveHandle { id: handle.id.clone() }],
            },
            Block2dOperation::RemoveHandle { id } => block2d_index_of(&projection.handles, id).map(|index| vec![Block2dOperation::SetHandle { index, handle: projection.handles[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetCompatibilityRule { rule, .. } => match block2d_index_of(&projection.compatibility, &rule.id) {
                Some(index) => vec![Block2dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }],
                None => vec![Block2dOperation::RemoveCompatibilityRule { id: rule.id.clone() }],
            },
            Block2dOperation::RemoveCompatibilityRule { id } => block2d_index_of(&projection.compatibility, id).map(|index| vec![Block2dOperation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetAttribute { attribute, .. } => match block2d_index_of(&projection.attributes, &attribute.key) {
                Some(index) => vec![Block2dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }],
                None => vec![Block2dOperation::RemoveAttribute { key: attribute.key.clone() }],
            },
            Block2dOperation::RemoveAttribute { key } => block2d_index_of(&projection.attributes, key).map(|index| vec![Block2dOperation::SetAttribute { index, attribute: projection.attributes[index].clone() }]).unwrap_or_default(),
            Block2dOperation::SetAuthors { .. } => vec![Block2dOperation::SetAuthors { authors: projection.authors.clone() }],
            Block2dOperation::SetCamera2d { .. } => vec![Block2dOperation::SetCamera2d { camera2d: projection.camera2d.clone() }],
            Block2dOperation::SetMeta { .. } => vec![Block2dOperation::SetMeta { meta: projection.meta.clone() }],
            Block2dOperation::SetDocument { .. } => vec![Block2dOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Block2dEnvelope = store::DocumentEnvelope<Block2dDefinition, Block2dOperation>;
pub type Block2dStore = store::DocumentStore<Block2dDefinition, Block2dOperation>;
// #endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::engine::empty_block2d_definition;
    use protocol::OperationDiff;

    #[test]
    fn set_handle_then_remove_round_trips_through_true_inverse() {
        let mut projection = empty_block2d_definition();
        let set = Block2dOperation::SetHandle { index: 0, handle: Block2dHandleTemplate { id: "h0".into(), handle_kind: "b-l".into(), angle: 0.5, radius: 0.36 } };
        let inverse = set.backwards(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.handles.len(), 1);
        assert_eq!(inverse, vec![Block2dOperation::RemoveHandle { id: "h0".into() }]);
        for operation in &inverse {
            projection = operation.diff(&projection).apply(&projection);
        }
        assert_eq!(projection, empty_block2d_definition());
    }

    #[test]
    fn diff_absorb_collapses_to_latest_set_document() {
        let mut diff = Block2dDiff::default();
        diff.absorb(Block2dDiff { node_kind: Some(BlockKindIdentity::default()), ..Default::default() });
        diff.absorb(Block2dDiff { document: Some(empty_block2d_definition()), ..Default::default() });
        assert!(diff.document.is_some());
        assert!(diff.node_kind.is_none());
    }
}
//#endregion 🧪️Tests
