//! 🧬️ Direct delete-material mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.delete-material.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfDeleteMaterialPayload {
    pub index: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.materials.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/materials", "index must address an item"));
    }
    require_unreferenced(base, GltfTopLevelFamily::Materials, payload.index)?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Materials, &mut after_delete(p.index));
    let slot = diff.materials.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfDeleteMaterialPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::create_material::CreateMaterialMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material::BindPrimitiveMaterialMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let mut rows = vec![crate::standards::v2_0::subsets::any::schema::mutations::create_material::mutation(crate::standards::v2_0::subsets::any::schema::mutations::create_material::GltfCreateMaterialPayload { position: p.index, material: Some(Box::new(base.document.materials[p.index].clone())) })];
    for (mesh, entry) in base.document.meshes.iter().enumerate() {
        for (primitive, item) in entry.primitives.iter().enumerate() {
            if item.material == Some(p.index) {
                rows.push(crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material::mutation(crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material::GltfBindPrimitiveMaterialPayload { mesh, primitive, material: p.index }));
            }
        }
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum DeleteMaterialMutation {
    Apply(GltfDeleteMaterialPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfDeleteMaterialPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_material::DeleteMaterialMutation>,
{
    Op::from(DeleteMaterialMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for DeleteMaterialMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<DeleteMaterialMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "material", kind: "delete-material", record: "DeletedMaterial" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Delete Material", "Material löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-material".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for DeleteMaterialMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<DeleteMaterialMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::create_material::CreateMaterialMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_material::BindPrimitiveMaterialMutation>,
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
