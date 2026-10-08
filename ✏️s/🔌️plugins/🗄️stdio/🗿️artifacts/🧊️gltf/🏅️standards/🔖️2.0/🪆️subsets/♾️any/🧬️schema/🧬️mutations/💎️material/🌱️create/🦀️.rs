//! 🧬️ Direct create-material mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-material.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/materials"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateMaterialPayload {
    pub position: usize,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub material: Option<Box<GltfMaterial>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateMaterialPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.materials.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/materials", "position must be within the collection"));
    }
    let lengths = GltfLengths::after_insert(base, GltfTopLevelFamily::Materials);
    if let Some(record) = &payload.material {
        check_material(record, &lengths)?;
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateMaterialPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Materials, &mut after_insert(p.position));
    diff.materials.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.material.as_deref().cloned().unwrap_or_else(|| GltfMaterial::default()) });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfCreateMaterialPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::delete_material::mutation(super::delete_material::GltfDeleteMaterialPayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateMaterialMutation {
    Apply(GltfCreateMaterialPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfCreateMaterialPayload) -> super::GltfMutation {
    super::GltfMutation::CreateMaterial(CreateMaterialMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreateMaterialMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Create Material", "Material erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-material".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/💎️inserts-a-5ae67c/🦀️.rs"]
mod case_inserts_a_5ae67c;
//#endregion 🧪️Tests
