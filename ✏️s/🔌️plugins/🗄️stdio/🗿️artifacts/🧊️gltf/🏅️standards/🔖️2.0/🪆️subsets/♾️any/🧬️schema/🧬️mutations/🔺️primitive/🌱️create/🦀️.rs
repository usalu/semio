//! 🧬️ Direct create-primitive mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::{checked_index, checked_position};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
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
pub fn inverse(p: &GltfCreatePrimitivePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::delete_primitive::mutation(super::delete_primitive::GltfDeletePrimitivePayload { mesh: p.mesh, primitive: p.position })]
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
pub fn mutation(payload: GltfCreatePrimitivePayload) -> super::GltfMutation {
    super::GltfMutation::CreatePrimitive(CreatePrimitiveMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreatePrimitiveMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "primitive", kind: "create-primitive", record: "CreatedPrimitive" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Create Primitive", "Primitiv erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-primitive".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t061/🦀️.rs"]
mod case_t061;
//#endregion 🧪️Tests
