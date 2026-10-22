//! 🧬️ Direct delete-morph-target mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.delete-morph-target.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfDeleteMorphTargetPayload {
    pub mesh: usize,
    pub primitive: usize,
    pub target: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeleteMorphTargetPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.target, base.document.meshes[payload.mesh].primitives[payload.primitive].targets.len(), "document/meshes/primitives/targets")?;
    if base.document.meshes[payload.mesh].primitives.len() != 1 || !base.document.meshes[payload.mesh].weights.is_empty() {
        return Err(reject("gltf.mutation.morph-target-arity", "document/meshes", "target deletion would violate mesh target-count coherence"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeleteMorphTargetPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let rows = GltfTargetsDelta::removal(&base.document.meshes[p.mesh].primitives[p.primitive].targets, p.target);
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { targets: Some(rows), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfDeleteMorphTargetPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::create_morph_target::mutation(crate::standards::v2_0::subsets::any::schema::mutations::create_morph_target::GltfCreateMorphTargetPayload { mesh: p.mesh, primitive: p.primitive, position: p.target, target: Some(Box::new(base.document.meshes[p.mesh].primitives[p.primitive].targets[p.target].clone())) })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum DeleteMorphTargetMutation {
    Apply(GltfDeleteMorphTargetPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfDeleteMorphTargetPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_morph_target::DeleteMorphTargetMutation>,
{
    Op::from(DeleteMorphTargetMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for DeleteMorphTargetMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<DeleteMorphTargetMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "morph-target", kind: "delete-morph-target", record: "DeletedMorphTarget" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete Morph Target", "Morphziel löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-morph-target".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for DeleteMorphTargetMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<DeleteMorphTargetMutation>,
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
