//! 🧬️ Direct move-node-parent mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
#[cfg(test)] use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-node-parent.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    let occurrences = |children: &[usize]| -> Vec<(usize, usize)> { children.iter().enumerate().filter(|(_, candidate)| **candidate == child).map(|(at, _)| (child, at)).collect() };
    let mut nodes = Vec::new();
    for (index, node) in base.document.nodes.iter().enumerate() {
        let removed = occurrences(&node.children);
        if index == p.parent && removed == [(child, p.position)] {
            continue;
        }
        let inserted = if index == p.parent { vec![(p.position, GltfRef(child))] } else { Vec::new() };
        if !removed.is_empty() || !inserted.is_empty() {
            nodes.push(GltfModified { index, diff: GltfNodeDiff { children: Some(GltfRefsDelta::rows(removed, inserted, Vec::new())), ..Default::default() } });
        }
    }
    let mut scenes = Vec::new();
    for (index, scene) in base.document.scenes.iter().enumerate() {
        let removed = occurrences(&scene.nodes);
        if !removed.is_empty() {
            scenes.push(GltfModified { index, diff: GltfSceneDiff { nodes: Some(GltfRefsDelta::rows(removed, Vec::new(), Vec::new())), ..Default::default() } });
        }
    }
    Ok(GltfDiff {
        nodes: (!nodes.is_empty()).then(|| GltfNodesDiff { modified: nodes, ..Default::default() }),
        scenes: (!scenes.is_empty()).then(|| GltfScenesDiff { modified: scenes, ..Default::default() }),
        ..Default::default()
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfReparentNodePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::UnbindNodeChildMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::BindNodeChildMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::BindSceneRootNodeMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let child = p.child;
    let parent_children = &base.document.nodes[p.parent].children;
    let parent_changed = parent_children.iter().position(|candidate| *candidate == child) != Some(p.position) || parent_children.iter().filter(|candidate| **candidate == child).count() != 1;
    let mut rows = Vec::new();
    if parent_changed {
        rows.push(crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::mutation(crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::GltfUnbindNodeChildPayload { parent: p.parent, child }));
    }
    for (parent, node) in base.document.nodes.iter().enumerate() {
        if let Some(position) = node.children.iter().position(|candidate| *candidate == child) {
            if parent != p.parent || parent_changed {
                rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::GltfBindNodeChildPayload { parent, child, position }));
            }
        }
    }
    for (scene, entry) in base.document.scenes.iter().enumerate() {
        if let Some(position) = entry.nodes.iter().position(|candidate| *candidate == child) {
            rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::GltfBindSceneRootNodePayload { scene, node: child, position }));
        }
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveNodeParentMutation {
    Apply(GltfReparentNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfReparentNodePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::move_node_parent::MoveNodeParentMutation>,
{
    Op::from(MoveNodeParentMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for MoveNodeParentMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<MoveNodeParentMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node-parent", kind: "move-node-parent", record: "MovedNodeParent" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move Node Parent", "Elternknoten verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-node-parent".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for MoveNodeParentMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<MoveNodeParentMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::UnbindNodeChildMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_node_child::BindNodeChildMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::BindSceneRootNodeMutation>,
{
    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }
    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests

//#endregion 🧪️Tests
