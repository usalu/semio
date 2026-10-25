//! 🧬️ Direct bind-morph-target-attribute mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
#[cfg(test)] use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-morph-target-attribute.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfBindMorphTargetAttributePayload {
    pub mesh: usize,
    pub primitive: usize,
    pub target: usize,
    pub semantic: String,
    pub accessor: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?;
    checked_index(payload.accessor, base.document.accessors.len(), "document/accessors")?;
    if payload.semantic.trim().is_empty() || base.document.meshes[payload.mesh].primitives[payload.primitive].targets[payload.target].0.iter().any(|(semantic, _)| semantic == &payload.semantic) {
        return Err(reject("gltf.mutation.invalid-attribute-semantic", "document/meshes/primitives/targets", "semantic must be non-empty and unique"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindMorphTargetAttributePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let rows = target_replacement(&base.document.meshes[p.mesh].primitives[p.primitive].targets, p.target, target_with_pair(&base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target], (p.semantic.clone(), p.accessor)));
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { targets: Some(rows), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfBindMorphTargetAttributePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::UnbindMorphTargetAttributeMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::GltfUnbindMorphTargetAttributePayload { mesh: p.mesh, primitive: p.primitive, target: p.target, semantic: p.semantic.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindMorphTargetAttributeMutation {
    Apply(GltfBindMorphTargetAttributePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfBindMorphTargetAttributePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::bind_morph_target_attribute::BindMorphTargetAttributeMutation>,
{
    Op::from(BindMorphTargetAttributeMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for BindMorphTargetAttributeMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<BindMorphTargetAttributeMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "morph-target-attribute", kind: "bind-morph-target-attribute", record: "BoundMorphTargetAttribute" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Bind Morph Target Attribute", "Morphzielattribut binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-morph-target-attribute".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for BindMorphTargetAttributeMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<BindMorphTargetAttributeMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::unbind_morph_target_attribute::UnbindMorphTargetAttributeMutation>,
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
