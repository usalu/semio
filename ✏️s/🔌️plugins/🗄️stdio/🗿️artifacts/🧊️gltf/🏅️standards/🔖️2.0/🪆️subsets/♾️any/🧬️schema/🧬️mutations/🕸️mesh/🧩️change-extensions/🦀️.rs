//! 🧬️ Direct change-mesh-extension-data mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::GltfTopLevelMutationRejection;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-mesh-extension-data.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "state", rename_all = "camelCase")]
pub enum GltfDataPresence {
    Absent,
    Present { value: GltfJson },
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeMeshExtensionDataPayload {
    pub mesh: usize,
    pub data: GltfDataPresence,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeMeshExtensionDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn requested(payload: &GltfChangeMeshExtensionDataPayload) -> Option<GltfJson> {
    match &payload.data {
        GltfDataPresence::Absent => None,
        GltfDataPresence::Present { value } => Some(value.clone()),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn presence(value: &Option<GltfJson>) -> GltfDataPresence {
    match value {
        None => GltfDataPresence::Absent,
        Some(value) => GltfDataPresence::Present { value: value.clone() },
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeMeshExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.meshes[p.mesh].extensions;
    Ok(GltfDiff { meshes: patch(p.mesh, GltfMeshDiff { extensions: (current != &requested(p)).then(|| requested(p)), ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeMeshExtensionDataPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.meshes[p.mesh].extensions;
    if current == &requested(p) {
        return Vec::new();
    }
    vec![super::change_mesh_extension_data::mutation(super::change_mesh_extension_data::GltfChangeMeshExtensionDataPayload { mesh: p.mesh, data: presence(current) })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeMeshExtensionDataMutation {
    Apply(GltfChangeMeshExtensionDataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeMeshExtensionDataPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeMeshExtensionData(ChangeMeshExtensionDataMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeMeshExtensionDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "mesh-extension-data", kind: "change-mesh-extension-data", record: "ChangedMeshExtensionData" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Mesh Extension Data", "Erweiterungsdaten des Netzes ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-mesh-extension-data".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔌️attaches-an-9e1e51/🦀️.rs"]
mod case_attaches_an_9e1e51;
//#endregion 🧪️Tests
