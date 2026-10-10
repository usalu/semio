//! 🧬️ Direct reorder-morph-target-attributes mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-morph-target-attributes.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderMorphTargetAttributesPayload {
    pub mesh: usize,
    pub primitive: usize,
    pub target: usize,
    pub order: Vec<String>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderMorphTargetAttributesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?;
    let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0;
    if payload.order.len() != attributes.len() || payload.order.iter().any(|semantic| !attributes.iter().any(|(key, _)| key == semantic)) {
        return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives/targets", "order must contain every semantic once"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderMorphTargetAttributesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let rows = target_replacement(&base.document.meshes[p.mesh].primitives[p.primitive].targets, p.target, GltfMorphTarget(p.order.iter().filter_map(|semantic| base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0.iter().find(|(key, _)| key == semantic).cloned()).collect()));
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { targets: Some(rows), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderMorphTargetAttributesPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_morph_target_attributes::mutation(super::reorder_morph_target_attributes::GltfReorderMorphTargetAttributesPayload { mesh: p.mesh, primitive: p.primitive, target: p.target, order: base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0.iter().map(|(semantic, _)| semantic.clone()).collect() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderMorphTargetAttributesMutation {
    Apply(GltfReorderMorphTargetAttributesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderMorphTargetAttributesPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderMorphTargetAttributes(ReorderMorphTargetAttributesMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderMorphTargetAttributesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "morph-target-attributes", kind: "reorder-morph-target-attributes", record: "ReorderedMorphTargetAttributes" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Morph Target Attributes", "Morphzielattribute umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-morph-target-attributes".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t046/🦀️.rs"]
mod case_t046;
//#endregion 🧪️Tests
