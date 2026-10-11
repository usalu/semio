//! 🧬️ Direct change-mesh-morph-weights mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-mesh-morph-weights.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMeshMorphWeightsPayload {
    pub mesh: usize,
    pub weights: Vec<f64>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeMeshMorphWeightsPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    if !payload.weights.iter().all(|value| value.is_finite()) || (!payload.weights.is_empty() && base.document.meshes[payload.mesh].primitives.iter().any(|primitive| primitive.targets.len() != payload.weights.len())) {
        return Err(reject("gltf.mutation.invalid-morph-weights", "document/meshes/weights", "weights must be finite and match every primitive target list"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeMeshMorphWeightsPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.meshes[p.mesh].weights;
    Ok(GltfDiff { meshes: patch(p.mesh, GltfMeshDiff { weights: (current != &p.weights).then(|| p.weights.clone()), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeMeshMorphWeightsPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.meshes[p.mesh].weights;
    if current == &p.weights {
        return Vec::new();
    }
    vec![super::change_mesh_morph_weights::mutation(super::change_mesh_morph_weights::GltfChangeMeshMorphWeightsPayload { mesh: p.mesh, weights: current.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMeshMorphWeightsMutation {
    Apply(GltfChangeMeshMorphWeightsPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeMeshMorphWeightsPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeMeshMorphWeights(ChangeMeshMorphWeightsMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeMeshMorphWeightsMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mesh-morph-weights", kind: "change-mesh-morph-weights", record: "ChangedMeshMorphWeights" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Mesh Morph Weights", "Morph-Gewichte des Netzes ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-mesh-morph-weights".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t064/🦀️.rs"]
mod case_t064;
//#endregion 🧪️Tests
