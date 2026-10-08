//! 🧬️ Direct move-node-parent mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-node-parent.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReparentNodePayload {
    pub parent: usize,
    pub child: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.parent, base.document.nodes.len(), "document/nodes")?;
    checked_index(payload.child, base.document.nodes.len(), "document/nodes")?;
    if payload.parent == payload.child {
        return Err(reject("gltf.mutation.node-cycle", "document/nodes", "a node cannot parent itself"));
    }
    let mut pending = vec![payload.child];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(node) = pending.pop() {
        if node == payload.parent {
            return Err(reject("gltf.mutation.node-cycle", "document/nodes", "relationship closes a cycle"));
        }
        if seen.insert(node) {
            pending.extend(base.document.nodes[node].children.iter().copied());
        }
    }
    let length = base.document.nodes[payload.parent].children.iter().filter(|child| **child != payload.child).count();
    checked_position(payload.position, length, "document/nodes/children")?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReparentNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let child = p.child;
    let mut nodes = Vec::new();
    for (index, node) in base.document.nodes.iter().enumerate() {
        let kept: Vec<usize> = node.children.iter().copied().filter(|candidate| *candidate != child).collect();
        let next = if index == p.parent { with_inserted(&kept, p.position, child) } else { kept };
        if next != node.children {
            nodes.push(GltfModified { index, diff: GltfNodeDiff { children: Some(next), ..Default::default() } });
        }
    }
    let mut scenes = Vec::new();
    for (index, scene) in base.document.scenes.iter().enumerate() {
        if scene.nodes.contains(&child) {
            scenes.push(GltfModified { index, diff: GltfSceneDiff { nodes: Some(scene.nodes.iter().copied().filter(|candidate| *candidate != child).collect()), ..Default::default() } });
        }
    }
    Ok(GltfDiff {
        nodes: (!nodes.is_empty()).then(|| GltfNodesDiff { modified: nodes, ..Default::default() }),
        scenes: (!scenes.is_empty()).then(|| GltfScenesDiff { modified: scenes, ..Default::default() }),
        ..Default::default()
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReparentNodePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let child = p.child;
    let parent_children = &base.document.nodes[p.parent].children;
    let kept: Vec<usize> = parent_children.iter().copied().filter(|candidate| *candidate != child).collect();
    let parent_changed = with_inserted(&kept, p.position, child) != *parent_children;
    let mut rows = Vec::new();
    if parent_changed {
        rows.push(super::unbind_node_child::mutation(super::unbind_node_child::GltfUnbindNodeChildPayload { parent: p.parent, child }));
    }
    for (parent, node) in base.document.nodes.iter().enumerate() {
        if let Some(position) = node.children.iter().position(|candidate| *candidate == child) {
            if parent != p.parent || parent_changed {
                rows.push(super::bind_node_child::mutation(super::bind_node_child::GltfBindNodeChildPayload { parent, child, position }));
            }
        }
    }
    for (scene, entry) in base.document.scenes.iter().enumerate() {
        if let Some(position) = entry.nodes.iter().position(|candidate| *candidate == child) {
            rows.push(super::bind_scene_root_node::mutation(super::bind_scene_root_node::GltfBindSceneRootNodePayload { scene, node: child, position }));
        }
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveNodeParentMutation {
    Apply(GltfReparentNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReparentNodePayload) -> super::GltfMutation {
    super::GltfMutation::MoveNodeParent(MoveNodeParentMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveNodeParentMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node-parent", kind: "move-node-parent", record: "MovedNodeParent" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<super::GltfMutation>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move Node Parent", "Elternknoten verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-node-parent".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🌿️adopts-the-3c6965/🦀️.rs"]
mod case_adopts_the_3c6965;
//#endregion 🧪️Tests
