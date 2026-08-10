//! ⚡️ Block 2D artifact — the mutation enum, its `Mutation` law and the store aliases
//! (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block2d::diff::{
    block2d_index_of, diff_remove_attribute, diff_remove_compatibility_rule, diff_remove_handle,
    diff_remove_handle_kind, diff_set_attribute, diff_set_compatibility_rule, diff_set_handle,
    diff_set_handle_kind, diff_set_snapshot, Block2dAuthorList, Block2dDiff,
};
use crate::artifacts::block2d::{Block2dSnapshot, Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation};
use crate::{BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

// #region 🔖️Operation
/// 🧮️ Block-2d operation: id-keyed table edits plus scalar node_kind/presentation/camera2d/meta, each
/// with a true inverse computed from the pre-operation projection, and a whole-document replace for
/// example loads.
// 🧯️ `large_enum_variant`: `SetSnapshot`'s whole-document payload makes it far larger than the other
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
    #[dsl(key = "setSnapshot")]
    SetSnapshot {
        #[dsl(block)]
        snapshot: Block2dSnapshot,
    },
}





fn block2d_mutation_diff(operation: &Block2dMutation, base: &Block2dSnapshot) -> Block2dDiff {
    match operation {
        Block2dMutation::SetNodeKind { node_kind } => Block2dDiff { node_kind: Some(node_kind.clone()), ..Default::default() },
        Block2dMutation::SetPresentation { presentation } => Block2dDiff { presentation: Some(presentation.clone()), ..Default::default() },
        Block2dMutation::SetHandleKind { index, handle_kind } => diff_set_handle_kind(*index, handle_kind.clone(), base),
        Block2dMutation::RemoveHandleKind { id } => diff_remove_handle_kind(id.clone()),
        Block2dMutation::SetHandle { index, handle } => diff_set_handle(*index, handle.clone(), base),
        Block2dMutation::RemoveHandle { id } => diff_remove_handle(id.clone()),
        Block2dMutation::SetCompatibilityRule { index, rule } => diff_set_compatibility_rule(*index, rule.clone(), base),
        Block2dMutation::RemoveCompatibilityRule { id } => diff_remove_compatibility_rule(id.clone()),
        Block2dMutation::SetAttribute { index, attribute } => diff_set_attribute(*index, attribute.clone(), base),
        Block2dMutation::RemoveAttribute { key } => diff_remove_attribute(key.clone()),
        Block2dMutation::SetAuthors { authors } => Block2dDiff { authors: Some(Block2dAuthorList { values: authors.clone() }), ..Default::default() },
        Block2dMutation::SetCamera2d { camera2d } => Block2dDiff { camera2d: Some(camera2d.clone()), ..Default::default() },
        Block2dMutation::SetMeta { meta } => Block2dDiff { meta: Some(meta.clone()), ..Default::default() },
        Block2dMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot.clone()),
    }
}

impl Mutation<Block2dSnapshot> for Block2dMutation {
    type Diff = Block2dDiff;

    fn diff(&self, projection: &Block2dSnapshot) -> Block2dDiff {
        block2d_mutation_diff(self, projection)
    }

    fn inverse(&self, projection: &Block2dSnapshot) -> Vec<Self> {
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
            Block2dMutation::SetSnapshot { .. } => vec![Block2dMutation::SetSnapshot { snapshot: projection.clone() }],
        }
    }
}

pub type Block2dEnvelope = store::DocumentEnvelope<Block2dSnapshot, Block2dMutation>;
pub type Block2dStore = store::DocumentStore<Block2dSnapshot, Block2dMutation>;
// #endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block2d::engine::empty_block2d_snapshot;
    use protocol::MutationDiff;

    #[test]
    fn set_handle_then_remove_round_trips_through_true_inverse() {
        let mut projection = empty_block2d_snapshot();
        let set = Block2dMutation::SetHandle { index: 0, handle: Block2dHandleTemplate { id: "h0".into(), handle_kind: "b-l".into(), angle: 0.5, radius: 0.36 } };
        let inverse = set.inverse(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.handles.len(), 1);
        assert_eq!(inverse, vec![Block2dMutation::RemoveHandle { id: "h0".into() }]);
        for operation in &inverse {
            projection = operation.diff(&projection).apply(&projection);
        }
        assert_eq!(projection, empty_block2d_snapshot());
    }

    #[test]
    fn diff_absorb_collapses_to_latest_set_snapshot() {
        let mut diff = Block2dDiff::default();
        diff.absorb(Block2dDiff { node_kind: Some(BlockKindIdentity::default()), ..Default::default() });
        diff.absorb(Block2dDiff { artifact: Some(Box::new(crate::artifacts::block2d::schema::Block2dArtifact::from_snapshot(empty_block2d_snapshot()))), ..Default::default() });
        assert!(diff.artifact.is_some());
        assert!(diff.node_kind.is_none());
    }
}
//#endregion 🧪️Tests


pub fn apply_block2d_mutation(projection: &mut Block2dSnapshot, mutation: &Block2dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_block2d_mutation(projection: &Block2dSnapshot, mutation: &Block2dMutation) -> Vec<Block2dMutation> {
    mutation.inverse(projection)
}
