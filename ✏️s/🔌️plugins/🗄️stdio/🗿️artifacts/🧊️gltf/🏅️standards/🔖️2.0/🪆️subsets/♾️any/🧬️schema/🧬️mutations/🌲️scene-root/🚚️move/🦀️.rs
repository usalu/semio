//! 🧬️ Direct move-scene-root-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-scene-root-node.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveSceneRootNodePayload {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveSceneRootNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    let roots = &base.document.scenes[payload.scene].nodes;
    let index = roots.iter().position(|node| *node == payload.node).ok_or_else(|| reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", payload.scene), "node is not a root of this scene"))?;
    checked_index(payload.position, roots.len(), "document/scenes/nodes")?;
    if index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/scenes/nodes", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.scenes[p.scene].nodes.iter().position(|node| *node == p.node) else {
        return Err(reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", p.scene), "node is not a root of this scene"));
    };
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { nodes: Some(GltfRefsDelta::relocation(&refs(&base.document.scenes[p.scene].nodes), index, p.position)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfMoveSceneRootNodePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.scenes[p.scene].nodes.iter().position(|node| *node == p.node) else {
        return Vec::new();
    };
    vec![super::move_scene_root_node::mutation(super::move_scene_root_node::GltfMoveSceneRootNodePayload { scene: p.scene, node: p.node, position: index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveSceneRootNodeMutation {
    Apply(GltfMoveSceneRootNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfMoveSceneRootNodePayload) -> super::GltfMutation {
    super::GltfMutation::MoveSceneRootNode(MoveSceneRootNodeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveSceneRootNodeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "scene-root-node", kind: "move-scene-root-node", record: "MovedSceneRootNode" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Move Scene Root Node", "Szenenwurzelknoten verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-scene-root-node".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🛑️refuses-to-move-ba235e/🦀️.rs"]
mod case_refuses_to_move_ba235e;
//#endregion 🧪️Tests
