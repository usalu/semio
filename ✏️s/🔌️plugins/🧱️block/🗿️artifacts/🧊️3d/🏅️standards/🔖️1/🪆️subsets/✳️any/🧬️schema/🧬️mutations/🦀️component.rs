//! ⚡️ Block 3D artifact — the mutation enum, its `Mutation` law and the store aliases
//! (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::block3d::diff::*;
use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind, Block3dVortexTemplate};
use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

// #region 🔖️Operation
/// 🧮️ Block-3d operation: id-keyed table edits plus scalar object_kind/camera3d/meta, each with a
/// true inverse computed from the pre-operation projection, and a whole-document replace for example
/// loads.
// 🧯️ `large_enum_variant`: `SetSnapshot`'s whole-document payload makes it far larger than the other
// scalar/id-keyed variants, but boxing it would require the `#[derive(dsl::DslEnum)]` field-shape
// machinery to see through `Box<T>`, which is unverified — same accepted tradeoff as gis's
// `Gis2dConfigMutation`/💡️reasoning's `ReplaceDocument`.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum Block3dMutation {
    #[dsl(key = "setObjectKind")]
    SetObjectKind {
        #[dsl(block)]
        object_kind: BlockKindIdentity,
    },
    #[dsl(key = "setRepresentation")]
    SetRepresentation {
        index: usize,
        #[dsl(block)]
        representation: BlockRepresentation,
    },
    #[dsl(key = "removeRepresentation")]
    RemoveRepresentation { id: String },
    #[dsl(key = "setVortexKind")]
    SetVortexKind {
        index: usize,
        #[dsl(block)]
        vortex_kind: Block3dVortexKind,
    },
    #[dsl(key = "removeVortexKind")]
    RemoveVortexKind { id: String },
    #[dsl(key = "setVortex")]
    SetVortex {
        index: usize,
        #[dsl(block)]
        vortex: Block3dVortexTemplate,
    },
    #[dsl(key = "removeVortex")]
    RemoveVortex { id: String },
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
    #[dsl(key = "setCamera3d")]
    SetCamera3d {
        #[dsl(block)]
        camera3d: BlockCamera3d,
    },
    #[dsl(key = "setMeta")]
    SetMeta {
        #[dsl(block)]
        meta: BlockMeta,
    },
    #[dsl(key = "setSnapshot")]
    SetSnapshot {
        #[dsl(block)]
        snapshot: Block3dSnapshot,
    },
}





fn block3d_mutation_diff(operation: &Block3dMutation, base: &Block3dSnapshot) -> Block3dDiff {
    match operation {
        Block3dMutation::SetObjectKind { object_kind } => Block3dDiff { object_kind: Some(object_kind.clone()), ..Default::default() },
        Block3dMutation::SetRepresentation { index, representation } => diff_set_representation(*index, representation.clone(), base),
        Block3dMutation::RemoveRepresentation { id } => diff_remove_representation(id.clone()),
        Block3dMutation::SetVortexKind { index, vortex_kind } => diff_set_vortex_kind(*index, vortex_kind.clone(), base),
        Block3dMutation::RemoveVortexKind { id } => diff_remove_vortex_kind(id.clone()),
        Block3dMutation::SetVortex { index, vortex } => diff_set_vortex(*index, vortex.clone(), base),
        Block3dMutation::RemoveVortex { id } => diff_remove_vortex(id.clone()),
        Block3dMutation::SetCompatibilityRule { index, rule } => diff_set_compatibility_rule(*index, rule.clone(), base),
        Block3dMutation::RemoveCompatibilityRule { id } => diff_remove_compatibility_rule(id.clone()),
        Block3dMutation::SetAttribute { index, attribute } => diff_set_attribute(*index, attribute.clone(), base),
        Block3dMutation::RemoveAttribute { key } => diff_remove_attribute(key.clone()),
        Block3dMutation::SetAuthors { authors } => Block3dDiff { authors: Some(Block3dAuthorList { values: authors.clone() }), ..Default::default() },
        Block3dMutation::SetCamera3d { camera3d } => Block3dDiff { camera3d: Some(camera3d.clone()), ..Default::default() },
        Block3dMutation::SetMeta { meta } => Block3dDiff { meta: Some(meta.clone()), ..Default::default() },
        Block3dMutation::SetSnapshot { snapshot } => diff_set_snapshot(snapshot.clone()),
    }
}

impl Mutation<Block3dSnapshot> for Block3dMutation {
    type Diff = Block3dDiff;

    fn diff(&self, projection: &Block3dSnapshot) -> Block3dDiff {
        block3d_mutation_diff(self, projection)
    }

    fn inverse(&self, projection: &Block3dSnapshot) -> Vec<Self> {
        match self {
            Block3dMutation::SetObjectKind { .. } => vec![Block3dMutation::SetObjectKind { object_kind: projection.object_kind.clone() }],
            Block3dMutation::SetRepresentation { representation, .. } => match block3d_index_of(&projection.representations, &representation.id) {
                Some(index) => vec![Block3dMutation::SetRepresentation { index, representation: projection.representations[index].clone() }],
                None => vec![Block3dMutation::RemoveRepresentation { id: representation.id.clone() }],
            },
            Block3dMutation::RemoveRepresentation { id } => {
                block3d_index_of(&projection.representations, id).map(|index| vec![Block3dMutation::SetRepresentation { index, representation: projection.representations[index].clone() }]).unwrap_or_default()
            }
            Block3dMutation::SetVortexKind { vortex_kind, .. } => match block3d_index_of(&projection.vortex_kinds, &vortex_kind.id) {
                Some(index) => vec![Block3dMutation::SetVortexKind { index, vortex_kind: projection.vortex_kinds[index].clone() }],
                None => vec![Block3dMutation::RemoveVortexKind { id: vortex_kind.id.clone() }],
            },
            Block3dMutation::RemoveVortexKind { id } => block3d_index_of(&projection.vortex_kinds, id).map(|index| vec![Block3dMutation::SetVortexKind { index, vortex_kind: projection.vortex_kinds[index].clone() }]).unwrap_or_default(),
            Block3dMutation::SetVortex { vortex, .. } => match block3d_index_of(&projection.vortices, &vortex.id) {
                Some(index) => vec![Block3dMutation::SetVortex { index, vortex: projection.vortices[index].clone() }],
                None => vec![Block3dMutation::RemoveVortex { id: vortex.id.clone() }],
            },
            Block3dMutation::RemoveVortex { id } => block3d_index_of(&projection.vortices, id).map(|index| vec![Block3dMutation::SetVortex { index, vortex: projection.vortices[index].clone() }]).unwrap_or_default(),
            Block3dMutation::SetCompatibilityRule { rule, .. } => match block3d_index_of(&projection.compatibility, &rule.id) {
                Some(index) => vec![Block3dMutation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }],
                None => vec![Block3dMutation::RemoveCompatibilityRule { id: rule.id.clone() }],
            },
            Block3dMutation::RemoveCompatibilityRule { id } => block3d_index_of(&projection.compatibility, id).map(|index| vec![Block3dMutation::SetCompatibilityRule { index, rule: projection.compatibility[index].clone() }]).unwrap_or_default(),
            Block3dMutation::SetAttribute { attribute, .. } => match block3d_index_of(&projection.attributes, &attribute.key) {
                Some(index) => vec![Block3dMutation::SetAttribute { index, attribute: projection.attributes[index].clone() }],
                None => vec![Block3dMutation::RemoveAttribute { key: attribute.key.clone() }],
            },
            Block3dMutation::RemoveAttribute { key } => block3d_index_of(&projection.attributes, key).map(|index| vec![Block3dMutation::SetAttribute { index, attribute: projection.attributes[index].clone() }]).unwrap_or_default(),
            Block3dMutation::SetAuthors { .. } => vec![Block3dMutation::SetAuthors { authors: projection.authors.clone() }],
            Block3dMutation::SetCamera3d { .. } => vec![Block3dMutation::SetCamera3d { camera3d: projection.camera3d.clone() }],
            Block3dMutation::SetMeta { .. } => vec![Block3dMutation::SetMeta { meta: projection.meta.clone() }],
            Block3dMutation::SetSnapshot { .. } => vec![Block3dMutation::SetSnapshot { snapshot: projection.clone() }],
        }
    }
}

pub type Block3dEnvelope = store::ArtifactEnvelope<Block3dSnapshot, Block3dMutation>;
pub type Block3dStore = store::ArtifactStore<Block3dSnapshot, Block3dMutation>;
// #endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::block3d::engine::empty_block3d_snapshot;
    use protocol::MutationDiff;

    #[test]
    fn set_vortex_then_remove_round_trips_through_true_inverse() {
        let mut projection = empty_block3d_snapshot();
        let set = Block3dMutation::SetVortex { index: 0, vortex: Block3dVortexTemplate { id: "v0".into(), vortex_kind: "door".into(), position: [1.0, 0.0, 0.0], direction: [0.0, 1.0, 0.0], radius: 0.3, label: None } };
        let inverse = set.inverse(&projection);
        projection = set.diff(&projection).apply(&projection);
        assert_eq!(projection.vortices.len(), 1);
        assert_eq!(inverse, vec![Block3dMutation::RemoveVortex { id: "v0".into() }]);
        for operation in &inverse {
            projection = operation.diff(&projection).apply(&projection);
        }
        assert_eq!(projection, empty_block3d_snapshot());
    }
}
//#endregion 🧪️Tests


pub fn apply_block3d_mutation(projection: &mut Block3dSnapshot, mutation: &Block3dMutation) {
    *projection = vcs::apply_mutation(projection, mutation);
}

pub fn inverse_block3d_mutation(projection: &Block3dSnapshot, mutation: &Block3dMutation) -> Vec<Block3dMutation> {
    mutation.inverse(projection)
}
