//! 🧬️ Direct unbind-scene-root-node mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.unbind-scene-root-node.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfUnbindSceneRootNodePayload {
    pub scene: usize,
    pub node: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    if !base.document.scenes[payload.scene].nodes.contains(&payload.node) {
        return Err(reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", payload.scene), "node is not a root of this scene"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.scenes[p.scene].nodes.iter().position(|node| *node == p.node) else {
        return Err(reject("gltf.mutation.relation-absent", format!("document/scenes/{}/nodes", p.scene), "node is not a root of this scene"));
    };
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { nodes: Some(GltfRefsDelta::removal(&refs(&base.document.scenes[p.scene].nodes), index)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfUnbindSceneRootNodePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.scenes[p.scene].nodes.iter().position(|node| *node == p.node) else {
        return Vec::new();
    };
    vec![super::bind_scene_root_node::mutation(super::bind_scene_root_node::GltfBindSceneRootNodePayload { scene: p.scene, node: p.node, position: index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum UnbindSceneRootNodeMutation {
    Apply(GltfUnbindSceneRootNodePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfUnbindSceneRootNodePayload) -> super::GltfMutation {
    super::GltfMutation::UnbindSceneRootNode(UnbindSceneRootNodeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for UnbindSceneRootNodeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "scene-root-node", kind: "unbind-scene-root-node", record: "UnboundSceneRootNode" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Unbind Scene Root Node", "Bindung des Szenenwurzelknotens aufheben")
    }

    fn target(&self) -> Vec<String> {
        vec!["unbind-scene-root-node".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/📤️demotes-the-root-84e1cf/🦀️.rs"]
mod case_demotes_the_root_84e1cf;
#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod case_middle_row;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "📜️contract/🧪️tests/🔬️unit/🦀️.rs"]
mod contract;
