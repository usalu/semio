//! 🧬️ Direct unbind-morph-target-attribute mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.unbind-morph-target-attribute.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfUnbindMorphTargetAttributePayload {
    pub mesh: usize,
    pub primitive: usize,
    pub target: usize,
    pub semantic: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfUnbindMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?;
    if !base.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0.iter().any(|(semantic, _)| semantic == &payload.semantic) {
        return Err(reject("gltf.mutation.relation-absent", "document/meshes/primitives/targets", "semantic is not bound"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfUnbindMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0.iter().position(|(semantic, _)| semantic == &p.semantic) else {
        return Err(reject("gltf.mutation.relation-absent", "document/meshes/primitives/targets", "semantic is not bound"));
    };
    let value = with_replaced(&base.document.meshes[p.mesh].primitives[p.primitive].targets, p.target, GltfMorphTarget(without(&base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0, index)));
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { targets: (value != base.document.meshes[p.mesh].primitives[p.primitive].targets).then(|| value), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfUnbindMorphTargetAttributePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0.iter().position(|(semantic, _)| semantic == &p.semantic) else {
        return Vec::new();
    };
    let accessor = base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0[index].1;
    let mut rows = vec![super::bind_morph_target_attribute::mutation(super::bind_morph_target_attribute::GltfBindMorphTargetAttributePayload { mesh: p.mesh, primitive: p.primitive, target: p.target, semantic: p.semantic.clone(), accessor })];
    if index + 1 != base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0.len() {
        rows.push(super::move_morph_target_attribute::mutation(super::move_morph_target_attribute::GltfMoveMorphTargetAttributePayload { mesh: p.mesh, primitive: p.primitive, target: p.target, semantic: p.semantic.clone(), position: index }));
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum UnbindMorphTargetAttributeMutation {
    Apply(GltfUnbindMorphTargetAttributePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfUnbindMorphTargetAttributePayload) -> super::GltfMutation {
    super::GltfMutation::UnbindMorphTargetAttribute(UnbindMorphTargetAttributeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for UnbindMorphTargetAttributeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "morph-target-attribute", kind: "unbind-morph-target-attribute", record: "UnboundMorphTargetAttribute" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Unbind Morph Target Attribute", "Bindung des Morphzielattributs aufheben")
    }

    fn target(&self) -> Vec<String> {
        vec!["unbind-morph-target-attribute".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔗️unbinds-normal-9e80f9/🦀️.rs"]
mod case_unbinds_normal_9e80f9;
#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod case_middle_row;
//#endregion 🧪️Tests
