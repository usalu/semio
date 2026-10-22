//! 🧬️ Direct move-primitive mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-primitive.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMovePrimitivePayload {
    pub mesh: usize,
    pub primitive: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMovePrimitivePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.position, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    if payload.primitive == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMovePrimitivePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    Ok(GltfDiff { meshes: primitives_rows(p.mesh, GltfPrimitivesDiff { removed: vec![p.primitive], added: vec![GltfAdded { index: p.position, item: base.document.meshes[p.mesh].primitives[p.primitive].clone() }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfMovePrimitivePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![mutation(GltfMovePrimitivePayload { mesh: p.mesh, primitive: p.position, position: p.primitive })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MovePrimitiveMutation {
    Apply(GltfMovePrimitivePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfMovePrimitivePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::move_primitive::MovePrimitiveMutation>,
{
    Op::from(MovePrimitiveMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for MovePrimitiveMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<MovePrimitiveMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "primitive", kind: "move-primitive", record: "MovedPrimitive" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move Primitive", "Primitiv verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-primitive".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for MovePrimitiveMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<MovePrimitiveMutation>,
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
