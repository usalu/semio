//! 🧬️ Direct bind-primitive-attribute mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
#[cfg(test)] use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-primitive-attribute.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfBindPrimitiveAttributePayload {
    pub mesh: usize,
    pub primitive: usize,
    pub semantic: String,
    pub accessor: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindPrimitiveAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.accessor, base.document.accessors.len(), "document/accessors")?;
    if payload.semantic.trim().is_empty() || base.document.meshes[payload.mesh].primitives[payload.primitive].attributes.iter().any(|(semantic, _)| semantic == &payload.semantic) {
        return Err(reject("gltf.mutation.invalid-attribute-semantic", "document/meshes/primitives/attributes", "semantic must be non-empty and unique"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindPrimitiveAttributePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let rows = GltfAttributesDelta::insertion(base.document.meshes[p.mesh].primitives[p.primitive].attributes.len(), GltfAttribute { semantic: p.semantic.clone(), accessor: p.accessor });
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { attributes: Some(rows), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfBindPrimitiveAttributePayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_attribute::mutation(crate::standards::v2_0::subsets::any::schema::mutations::unbind_primitive_attribute::GltfUnbindPrimitiveAttributePayload { mesh: p.mesh, primitive: p.primitive, semantic: p.semantic.clone() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindPrimitiveAttributeMutation {
    Apply(GltfBindPrimitiveAttributePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfBindPrimitiveAttributePayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::bind_primitive_attribute::BindPrimitiveAttributeMutation>,
{
    Op::from(BindPrimitiveAttributeMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for BindPrimitiveAttributeMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<BindPrimitiveAttributeMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "primitive-attribute", kind: "bind-primitive-attribute", record: "BoundPrimitiveAttribute" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Bind Primitive Attribute", "Primitivattribut binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-primitive-attribute".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests

//#endregion 🧪️Tests
