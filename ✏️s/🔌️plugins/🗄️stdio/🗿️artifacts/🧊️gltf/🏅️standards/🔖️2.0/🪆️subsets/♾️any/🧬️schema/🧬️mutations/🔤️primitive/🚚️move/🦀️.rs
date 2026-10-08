//! 🧬️ Direct move-primitive-attribute mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-primitive-attribute.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfMovePrimitiveAttributePayload {
    pub mesh: usize,
    pub primitive: usize,
    pub semantic: String,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMovePrimitiveAttributePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].attributes;
    let index = attributes.iter().position(|(semantic, _)| semantic == &payload.semantic).ok_or_else(|| reject("gltf.mutation.relation-absent", "document/meshes/primitives/attributes", "semantic is not bound"))?;
    checked_index(payload.position, attributes.len(), "document/meshes/primitives/attributes")?;
    if index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives/attributes", "destination equals source"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMovePrimitiveAttributePayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let Some(index) = base.document.meshes[p.mesh].primitives[p.primitive].attributes.iter().position(|(semantic, _)| semantic == &p.semantic) else {
        return Err(reject("gltf.mutation.relation-absent", "document/meshes/primitives/attributes", "semantic is not bound"));
    };
    let value = with_moved(&base.document.meshes[p.mesh].primitives[p.primitive].attributes, index, p.position);
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { attributes: (value != base.document.meshes[p.mesh].primitives[p.primitive].attributes).then(|| GltfMorphTarget(value)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfMovePrimitiveAttributePayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let Some(index) = base.document.meshes[p.mesh].primitives[p.primitive].attributes.iter().position(|(semantic, _)| semantic == &p.semantic) else {
        return Vec::new();
    };
    vec![super::move_primitive_attribute::mutation(super::move_primitive_attribute::GltfMovePrimitiveAttributePayload { mesh: p.mesh, primitive: p.primitive, semantic: p.semantic.clone(), position: index })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MovePrimitiveAttributeMutation {
    Apply(GltfMovePrimitiveAttributePayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfMovePrimitiveAttributePayload) -> super::GltfMutation {
    super::GltfMutation::MovePrimitiveAttribute(MovePrimitiveAttributeMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MovePrimitiveAttributeMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "primitive-attribute", kind: "move-primitive-attribute", record: "MovedPrimitiveAttribute" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Move Primitive Attribute", "Primitivattribut verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-primitive-attribute".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t060/🦀️.rs"]
mod case_t060;
//#endregion 🧪️Tests
