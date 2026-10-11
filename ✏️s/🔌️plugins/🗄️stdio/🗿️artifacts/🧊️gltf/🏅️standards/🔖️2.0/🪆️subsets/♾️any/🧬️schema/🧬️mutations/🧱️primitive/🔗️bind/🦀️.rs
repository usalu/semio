//! 🧬️ Direct bind-primitive-material mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.bind-primitive-material.v1";
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfBindPrimitiveMaterialPayload {
    pub mesh: usize,
    pub primitive: usize,
    pub material: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfBindPrimitiveMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    checked_index(payload.primitive, base.document.meshes[payload.mesh].primitives.len(), "document/meshes/primitives")?;
    checked_index(payload.material, base.document.materials.len(), "document/materials")?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfBindPrimitiveMaterialPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let value = Some(p.material);
    Ok(GltfDiff { meshes: primitive_patch(p.mesh, p.primitive, GltfPrimitiveDiff { material: (value != base.document.meshes[p.mesh].primitives[p.primitive].material).then(|| value), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfBindPrimitiveMaterialPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    match base.document.meshes[p.mesh].primitives[p.primitive].material {
        Some(current) if current == p.material => Vec::new(),
        Some(current) => vec![super::bind_primitive_material::mutation(super::bind_primitive_material::GltfBindPrimitiveMaterialPayload { mesh: p.mesh, primitive: p.primitive, material: current })],
        None => vec![super::unbind_primitive_material::mutation(super::unbind_primitive_material::GltfUnbindPrimitiveMaterialPayload { mesh: p.mesh, primitive: p.primitive })],
    }
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum BindPrimitiveMaterialMutation {
    Apply(GltfBindPrimitiveMaterialPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfBindPrimitiveMaterialPayload) -> super::GltfMutation {
    super::GltfMutation::BindPrimitiveMaterial(BindPrimitiveMaterialMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for BindPrimitiveMaterialMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "primitive-material", kind: "bind-primitive-material", record: "BoundPrimitiveMaterial" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Bind Primitive Material", "Primitivmaterial binden")
    }

    fn target(&self) -> Vec<String> {
        vec!["bind-primitive-material".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔗️binds-the-steel-1cd6cc/🦀️.rs"]
mod case_binds_the_steel_1cd6cc;
//#endregion 🧪️Tests
