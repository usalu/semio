//! 🧬️ Direct create-primitive mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-primitive.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfCreatePrimitivePayload {
    pub mesh: usize,
    pub position: usize,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub primitive: Option<Box<GltfPrimitive>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreatePrimitivePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_position(payload.position, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    if let Some(record) = &payload.primitive {
        check_primitive(record, &GltfLengths::of(base))?;
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreatePrimitivePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { meshes: primitives_rows(p.mesh, GltfPrimitivesDiff { added: vec![GltfAdded { index: p.position, item: p.primitive.as_deref().cloned().unwrap_or_default() }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfCreatePrimitivePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_primitive::DeletePrimitiveMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::delete_primitive::mutation(crate::standards::v2_0::subsets::any::schema::mutations::delete_primitive::GltfDeletePrimitivePayload { mesh: p.mesh, primitive: p.position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreatePrimitiveMutation {
    Apply(GltfCreatePrimitivePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfCreatePrimitivePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::create_primitive::CreatePrimitiveMutation>,
{
    Op::from(CreatePrimitiveMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for CreatePrimitiveMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<CreatePrimitiveMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "primitive", kind: "create-primitive", record: "CreatedPrimitive" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Primitive", "Primitiv erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-primitive".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for CreatePrimitiveMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<CreatePrimitiveMutation> + From<crate::standards::v2_0::subsets::any::schema::mutations::delete_primitive::DeletePrimitiveMutation>,
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
