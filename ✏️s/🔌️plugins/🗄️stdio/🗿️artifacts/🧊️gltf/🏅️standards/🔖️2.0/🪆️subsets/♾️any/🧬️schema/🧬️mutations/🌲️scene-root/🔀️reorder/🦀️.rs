//! 🧬️ Direct reorder-scene-root-nodes mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-scene-root-nodes.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderSceneRootNodesPayload {
    pub scene: usize,
    pub order: Vec<usize>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderSceneRootNodesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.scene, base.document.scenes.len(), "document/scenes")?;
    let roots = &base.document.scenes[payload.scene].nodes;
    if payload.order.len() != roots.len() || payload.order.iter().any(|node| !roots.contains(node)) || {
        let mut order = payload.order.clone();
        order.sort_unstable();
        order.dedup();
        order.len() != roots.len()
    } {
        return Err(reject("gltf.mutation.invalid-permutation", "document/scenes/nodes", "order must contain every root identity once"));
    }
    if payload.order == *roots {
        return Err(reject("gltf.mutation.no-observable-change", "document/scenes/nodes", "reorder must change order"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderSceneRootNodesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { scenes: patch(p.scene, GltfSceneDiff { nodes: Some(GltfRefsDelta::rows(Vec::new(), Vec::new(), moves_to_keys(&refs(&base.document.scenes[p.scene].nodes), &p.order))), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderSceneRootNodesPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_scene_root_nodes::mutation(super::reorder_scene_root_nodes::GltfReorderSceneRootNodesPayload { scene: p.scene, order: base.document.scenes[p.scene].nodes.clone() })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderSceneRootNodesMutation {
    Apply(GltfReorderSceneRootNodesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderSceneRootNodesPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderSceneRootNodes(ReorderSceneRootNodesMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderSceneRootNodesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "scene-root-nodes", kind: "reorder-scene-root-nodes", record: "ReorderedSceneRootNodes" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Scene Root Nodes", "Szenenwurzelknoten umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-scene-root-nodes".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🚫️refuses-a-scene-e8c50c/🦀️.rs"]
mod case_refuses_a_scene_e8c50c;
//#endregion 🧪️Tests
