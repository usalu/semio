//! 🧬️ Direct reorder-primitive-attributes mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-primitive-attributes.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderPrimitiveAttributesPayload {
    pub mesh: usize,
    pub primitive: usize,
    pub order: Vec<String>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderPrimitiveAttributesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    let attributes = &base.document.meshes[payload.mesh].primitives[payload.primitive].attributes;
    if payload.order.len() != attributes.len() || payload.order.iter().any(|semantic| !attributes.iter().any(|(key, _)| key == semantic)) || {
        let mut order = payload.order.clone();
        order.sort();
        order.dedup();
        order.len() != attributes.len()
    } {
        return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives/attributes", "order must contain every semantic once"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderPrimitiveAttributesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let rows = GltfAttributesDelta::rows(Vec::new(), Vec::new(), moves_to_keys(&attribute_rows(&base.document.meshes[p.mesh].primitives[p.primitive].attributes), &p.order));
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { attributes: Some(rows), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderPrimitiveAttributesPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_primitive_attributes::mutation(super::reorder_primitive_attributes::GltfReorderPrimitiveAttributesPayload { mesh: p.mesh, primitive: p.primitive, order: base.document.meshes[p.mesh].primitives[p.primitive].attributes.iter().map(|(semantic, _)| semantic.clone()).collect() })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderPrimitiveAttributesMutation {
    Apply(GltfReorderPrimitiveAttributesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderPrimitiveAttributesPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderPrimitiveAttributes(ReorderPrimitiveAttributesMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderPrimitiveAttributesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "primitive-attributes", kind: "reorder-primitive-attributes", record: "ReorderedPrimitiveAttributes" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Primitive Attributes", "Primitivattribute umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-primitive-attributes".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t059/🦀️.rs"]
mod case_t059;
//#endregion 🧪️Tests
