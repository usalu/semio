//! 🧬️ Direct bind-primitive-indices mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-primitive-indices.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    if base.document.accessors[payload.accessor].kind != crate::standards::v2_0::subsets::any::schema::snapshot::GltfAccessorType::Scalar || base.document.accessors[payload.accessor].component_type == crate::standards::v2_0::subsets::any::schema::snapshot::GltfComponentType::Float {
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
pub fn inverse(p: &GltfBindPrimitiveIndicesPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.meshes[p.mesh].primitives[p.primitive].indices {
        Some(current) if current == p.accessor => Vec::new(),
        Some(current) => vec![super::bind_primitive_indices::mutation(super::bind_primitive_indices::GltfBindPrimitiveIndicesPayload { mesh: p.mesh, primitive: p.primitive, accessor: current })],
        None => vec![super::unbind_primitive_indices::mutation(super::unbind_primitive_indices::GltfUnbindPrimitiveIndicesPayload { mesh: p.mesh, primitive: p.primitive })],
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindPrimitiveIndicesMutation {
    Apply(GltfBindPrimitiveIndicesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfBindPrimitiveIndicesPayload) -> super::GltfMutation {
    super::GltfMutation::BindPrimitiveIndices(BindPrimitiveIndicesMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for BindPrimitiveIndicesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "primitive-indices", kind: "bind-primitive-indices", record: "BoundPrimitiveIndices" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Bind Primitive Indices", "Primitivindizes binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-primitive-indices".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔢️binds-the-scalar-2b0259/🦀️.rs"]
mod case_binds_the_scalar_2b0259;
//#endregion 🧪️Tests
