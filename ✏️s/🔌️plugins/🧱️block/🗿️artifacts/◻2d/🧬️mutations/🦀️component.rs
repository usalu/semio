//! ⚡️ Block 2D artifact — the mutation enum, its `Mutation` law and the store aliases
//! (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block2d::diff::{block2d_index_of, Block2dDiff};
use crate::artifacts::block2d::{Block2dDefinition, Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

// #region 🔖️Operation
/// 🧮️ Block-2d operation: id-keyed table edits plus scalar node_kind/presentation/camera2d/meta, each
/// with a true inverse computed from the pre-operation projection, and a whole-document replace for
/// example loads.
// 🧯️ `large_enum_variant`: `SetDocument`'s whole-document payload makes it far larger than the other
// scalar/id-keyed variants, but boxing it would require the `#[derive(dsl::DslEnum)]` field-shape
// machinery to see through `Box<T>`, which is unverified — same accepted tradeoff as gis's
// `Gis2dConfigMutation`/💡️reasoning's `ReplaceDocument`.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Block2dMutation {
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





fn block2d_mutation_diff(operation: &Block2dMutation) -> Block2dDiff {
    let mut diff = Block2dDiff::default();
    match operation {
        Block2dMutation::SetNodeKind { node_kind } => diff.node_kind = Some(node_kind.clone()),
        Block2dMutation::SetPresentation { presentation } => diff.presentation = Some(presentation.clone()),
        Block2dMutation::SetHandleKind { index, handle_kind } => diff.handle_kinds.set.push((*index, handle_kind.clone())),
        Block2dMutation::RemoveHandleKind { id } => diff.handle_kinds.removed.push(id.clone()),
        Block2dMutation::SetHandle { index, handle } => diff.handles.set.push((*index, handle.clone())),
        Block2dMutation::RemoveHandle { id } => diff.handles.removed.push(id.clone()),
        Block2dMutation::SetCompatibilityRule { index, rule } => diff.compatibility.set.push((*index, rule.clone())),
        Block2dMutation::RemoveCompatibilityRule { id } => diff.compatibility.removed.push(id.clone()),
        Block2dMutation::SetAttribute { index, attribute } => diff.attributes.set.push((*index, attribute.clone())),
        Block2dMutation::RemoveAttribute { key } => diff.attributes.removed.push(key.clone()),
        Block2dMutation::SetAuthors { authors } => diff.authors = Some(authors.clone()),
        Block2dMutation::SetCamera2d { camera2d } => diff.camera2d = Some(camera2d.clone()),
        Block2dMutation::SetMeta { meta } => diff.meta = Some(meta.clone()),
        Block2dMutation::SetDocument { document } => diff.document = Some(document.clone()),
    }
    diff
}

impl Mutation<Block2dDefinition> for Block2dMutation {
    type Diff = Block2dDiff;

    fn diff(&self, _projection: &Block2dDefinition) -> Block2dDiff {
        block2d_mutation_diff(self)
    }

    fn inverse(&self, projection: &Block2dDefinition) -> Vec<Self> {
        match self {
            Block2dMutation::SetNodeKind { .. } => vec![Block2dMutation::SetNodeKind { node_kind: projection.node_kind.clone() }],
            Block2dMutation::SetPresentation { .. } => vec![Block2dMutation::SetPresentation { presentation: projection.presentation.clone() }],
            Block2dMutation::SetHandleKind { handle_kind, .. } => match block2d_index_of(&projection.handle_kinds, &handle_kind.id) {
                Some(index) => vec![Block2dMutation::SetHandleKind { index, handle_kind: projection.handle_kinds[index].clone() }],
                None => vec![Block2dMutation::RemoveHandleKind { id: handle_kind.id.clone() }],
            },
            Block2dMutation::RemoveHandleKind { id } => block2d_index_of(&projection.handle_kinds, id).map(|index| vec![Block2dMutation::SetHandleKind { index, handle_kind: projection.handle_kinds[index].clone() }]).unwrap_or_default(),
            Block2dMutation::SetHandle { handle, .. } => match block2d_index_of(&projection.handles, &handle.id) {
                Some(index) => vec![Block2dMutation::SetHandle { index, handle: projection.handles[index].clone() }],
                None => vec![Block2dMutation::RemoveHandle { id: handle.id.clone() }],
            },
            Block2dMutation::RemoveHandle { id } => block2d_index_of(&projection.handles, id).map(|index| vec![Block2dMutation::SetHandle { index, handle: projection.handles[index].clone() }]).unwrap_or_default(),
            Block2dMutation::SetCompatibilityRule { rule, .. } => match block2d_index_of(&projection.compatibility, &rule.id) {
                Some(index) => vec![Block2dMutation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }],
                None => vec![Block2dMutation::RemoveCompatibilityRule { id: rule.id.clone() }],
            },
            Block2dMutation::RemoveCompatibilityRule { id } => block2d_index_of(&projection.compatibility, id).map(|index| vec![Block2dMutation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }]).unwrap_or_default(),
            Block2dMutation::SetAttribute { attribute, .. } => match block2d_index_of(&projection.attributes, &attribute.key) {
                Some(index) => vec![Block2dMutation::SetAttribute { index, attribute: projection.attributes[index].clone() }],
                None => vec![Block2dMutation::RemoveAttribute { key: attribute.key.clone() }],
            },
            Block2dMutation::RemoveAttribute { key } => block2d_index_of(&projection.attributes, key).map(|index| vec![Block2dMutation::SetAttribute { index, attribute: projection.attributes[index].clone() }]).unwrap_or_default(),
            Block2dMutation::SetAuthors { .. } => vec![Block2dMutation::SetAuthors { authors: projection.authors.clone() }],
            Block2dMutation::SetCamera2d { .. } => vec![Block2dMutation::SetCamera2d { camera2d: projection.camera2d.clone() }],
            Block2dMutation::SetMeta { .. } => vec![Block2dMutation::SetMeta { meta: projection.meta.clone() }],
            Block2dMutation::SetDocument { .. } => vec![Block2dMutation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Block2dEnvelope = store::DocumentEnvelope<Block2dDefinition, Block2dMutation>;
pub type Block2dStore = store::DocumentStore<Block2dDefinition, Block2dMutation>;
// #endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::engine::empty_block2d_definition;
    use protocol::MutationDiff;

    #[test]
    fn set_handle_then_remove_round_trips_through_true_inverse() {
        let mut projection = empty_block2d_definition();
        let set = Block2dMutation::SetHandle { index: 0, handle: Block2dHandleTemplate { id: "h0".into(), handle_kind: "b-l".into(), angle: 0.5, radius: 0.36 } };
        let inverse = set.inverse(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.handles.len(), 1);
        assert_eq!(inverse, vec![Block2dMutation::RemoveHandle { id: "h0".into() }]);
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


pub fn apply_block2d_mutation(projection: &mut Block2dDefinition, mutation: &Block2dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_block2d_mutation(projection: &Block2dDefinition, mutation: &Block2dMutation) -> Vec<Block2dMutation> {
    mutation.inverse(projection)
}
