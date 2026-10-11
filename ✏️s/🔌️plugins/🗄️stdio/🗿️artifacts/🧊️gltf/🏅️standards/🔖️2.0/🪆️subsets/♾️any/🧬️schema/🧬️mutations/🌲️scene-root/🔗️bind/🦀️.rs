//! 🧬️ Direct bind-scene-root-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-scene-root-node.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfBindSceneRootNodePayload {
    pub scene: usize,
    pub node: usize,
    pub position: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    checked_position(payload.position, base.document.scenes[payload.scene].nodes.len(), "document/scenes/nodes")?;
    if base.document.scenes[payload.scene].nodes.contains(&payload.node) {
        return Err(reject("gltf.mutation.duplicate-scene-root", format!("document/scenes/{}/nodes", payload.scene), "node is already a scene root"));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { nodes: Some(GltfRefsDelta::insertion(p.position, GltfRef(p.node))), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfBindSceneRootNodePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::unbind_scene_root_node::mutation(super::unbind_scene_root_node::GltfUnbindSceneRootNodePayload { scene: p.scene, node: p.node })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindSceneRootNodeMutation {
    Apply(GltfBindSceneRootNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfBindSceneRootNodePayload) -> super::GltfMutation {
    super::GltfMutation::BindSceneRootNode(BindSceneRootNodeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for BindSceneRootNodeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "scene-root-node", kind: "bind-scene-root-node", record: "BoundSceneRootNode" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Bind Scene Root Node", "Szenenwurzelknoten binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-scene-root-node".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📥️promotes-the-b86f40/🦀️.rs"]
mod case_promotes_the_b86f40;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
