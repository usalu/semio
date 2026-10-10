//! 🧬️ Direct change-node-morph-weights mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-node-morph-weights.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeNodeMorphWeightsPayload {
    pub node: usize,
    pub weights: Vec<f64>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeNodeMorphWeightsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.node, base.document.nodes.len(), "document/nodes")?;
    if !payload.weights.iter().all(|value| value.is_finite()) {
        return Err(reject("gltf.mutation.invalid-morph-weights", "document/nodes/weights", "weights must be finite"));
    }
    let mesh = base.document.nodes[payload.node].mesh;
    if !payload.weights.is_empty() && mesh.is_none() {
        return Err(reject("gltf.mutation.missing-mesh", "document/nodes/mesh", "morph weights require a mesh"));
    }
    if let Some(mesh) = mesh.filter(|_| !payload.weights.is_empty()) {
        if base.document.meshes[mesh].primitives.iter().any(|primitive| primitive.targets.len() != payload.weights.len()) {
            return Err(reject("gltf.mutation.morph-weight-arity", "document/nodes/weights", "weights must match primitive target count"));
        }
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeNodeMorphWeightsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.nodes[p.node].weights;
    Ok(GltfDiff { nodes: patch(p.node, GltfNodeDiff { weights: (current != &p.weights).then(|| p.weights.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeNodeMorphWeightsPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.nodes[p.node].weights;
    if current == &p.weights {
        return Vec::new();
    }
    vec![super::change_node_morph_weights::mutation(super::change_node_morph_weights::GltfChangeNodeMorphWeightsPayload { node: p.node, weights: current.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeNodeMorphWeightsMutation {
    Apply(GltfChangeNodeMorphWeightsPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeNodeMorphWeightsPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeNodeMorphWeights(ChangeNodeMorphWeightsMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeNodeMorphWeightsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "node-morph-weights", kind: "change-node-morph-weights", record: "ChangedNodeMorphWeights" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Node Morph Weights", "Morph-Gewichte des Knotens ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-node-morph-weights".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/⚖️sets-the-node-1b9600/🦀️.rs"]
mod case_sets_the_node_1b9600;
//#endregion 🧪️Tests
