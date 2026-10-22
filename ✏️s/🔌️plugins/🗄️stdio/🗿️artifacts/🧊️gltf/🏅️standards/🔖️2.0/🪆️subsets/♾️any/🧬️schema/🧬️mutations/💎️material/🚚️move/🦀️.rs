//! 🧬️ Direct move-material mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-material.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveMaterialPayload {
    pub index: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.materials.len() || payload.position >= base.document.materials.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/materials", "indices must address items"));
    }
    if payload.index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/materials", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveMaterialPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Materials, &mut after_move(p.index, p.position));
    let slot = diff.materials.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    slot.added.push(GltfAdded { index: p.position, item: base.document.materials[p.index].clone() });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfMoveMaterialPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![mutation(GltfMoveMaterialPayload { index: p.position, position: p.index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveMaterialMutation {
    Apply(GltfMoveMaterialPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfMoveMaterialPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::move_material::MoveMaterialMutation>,
{
    Op::from(MoveMaterialMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for MoveMaterialMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<MoveMaterialMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "material", kind: "move-material", record: "MovedMaterial" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move Material", "Material verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-material".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for MoveMaterialMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<MoveMaterialMutation>,
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
