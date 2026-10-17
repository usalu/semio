//! 🧬️ Direct bind-primitive-indices mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-primitive-indices.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfBindPrimitiveIndicesPayload {
    pub mesh: usize,
    pub primitive: usize,
    pub accessor: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindPrimitiveIndicesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.accessor, base.document.accessors.len(), "document/accessors")?;
    if base.document.accessors[payload.accessor].kind != semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType::Scalar || base.document.accessors[payload.accessor].component_type == semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType::Float {
        return Err(reject("gltf.mutation.invalid-index-accessor", "document/accessors", "indices require a scalar integer accessor"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindPrimitiveIndicesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let value = Some(p.accessor);
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { indices: (value != base.document.meshes[p.mesh].primitives[p.primitive].indices).then(|| value), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfBindPrimitiveIndicesPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.meshes[p.mesh].primitives[p.primitive].indices {
        Some(current) if current == p.accessor => Vec::new(),
        Some(current) => vec![mutation(GltfBindPrimitiveIndicesPayload { mesh: p.mesh, primitive: p.primitive, accessor: current })],
        None => vec![crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_indices::mutation(crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_indices::GltfUnbindPrimitiveIndicesPayload { mesh: p.mesh, primitive: p.primitive })],
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindPrimitiveIndicesMutation {
    Apply(GltfBindPrimitiveIndicesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfBindPrimitiveIndicesPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_indices::BindPrimitiveIndicesMutation>,
{
    Op::from(BindPrimitiveIndicesMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for BindPrimitiveIndicesMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<BindPrimitiveIndicesMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "primitive-indices", kind: "bind-primitive-indices", record: "BoundPrimitiveIndices" };

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

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Bind Primitive Indices", "Primitivindizes binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-primitive-indices".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests

//#endregion 🧪️Tests
