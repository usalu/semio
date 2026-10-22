//! 🧬️ Direct unbind-morph-target-attribute mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
#[cfg(test)] use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.unbind-morph-target-attribute.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
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
    let rows = target_replacement(&base.document.meshes[p.mesh].primitives[p.primitive].targets, p.target, target_without_pair(&base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target], index));
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { targets: Some(rows), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfUnbindMorphTargetAttributePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0.iter().position(|(semantic, _)| semantic == &p.semantic) else {
        return Vec::new();
    };
    let accessor = base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0[index].1;
    let mut rows = vec![crate::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::GltfBindMorphTargetAttributePayload { mesh: p.mesh, primitive: p.primitive, target: p.target, semantic: p.semantic.clone(), accessor })];
    if index + 1 != base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].0.len() {
        rows.push(crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::move_morph_target_attribute::GltfMoveMorphTargetAttributePayload { mesh: p.mesh, primitive: p.primitive, target: p.target, semantic: p.semantic.clone(), position: index }));
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum UnbindMorphTargetAttributeMutation {
    Apply(GltfUnbindMorphTargetAttributePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfUnbindMorphTargetAttributePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::UnbindMorphTargetAttributeMutation>,
{
    Op::from(UnbindMorphTargetAttributeMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for UnbindMorphTargetAttributeMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<UnbindMorphTargetAttributeMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unbind", entity: "morph-target-attribute", kind: "unbind-morph-target-attribute", record: "UnboundMorphTargetAttribute" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Unbind Morph Target Attribute", "Bindung des Morphzielattributs aufheben")
    }

    fn target(&self) -> Vec<String> {
        vec!["unbind-morph-target-attribute".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for UnbindMorphTargetAttributeMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<UnbindMorphTargetAttributeMutation>,
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
