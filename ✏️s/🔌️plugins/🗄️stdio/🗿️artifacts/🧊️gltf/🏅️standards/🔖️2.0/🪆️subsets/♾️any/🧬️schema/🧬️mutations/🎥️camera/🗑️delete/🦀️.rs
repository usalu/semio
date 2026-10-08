//! 🧬️ Direct delete-camera mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.delete-camera.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/cameras"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfDeleteCameraPayload {
    pub index: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeleteCameraPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.cameras.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/cameras", "index must address an item"));
    }
    require_unreferenced(base, GltfTopLevelFamily::Cameras, payload.index)?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeleteCameraPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Cameras, &mut after_delete(p.index));
    let slot = diff.cameras.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfDeleteCameraPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let mut rows = vec![super::create_camera::mutation(super::create_camera::GltfCreateCameraPayload { position: p.index, projection: base.document.cameras[p.index].projection.clone(), camera: Some(Box::new(base.document.cameras[p.index].clone())) })];
    for (node, entry) in base.document.nodes.iter().enumerate() {
        if entry.camera == Some(p.index) {
            rows.push(super::bind_node_camera::mutation(super::bind_node_camera::GltfBindNodeCameraPayload { node, camera: p.index }));
        }
    }
    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum DeleteCameraMutation {
    Apply(GltfDeleteCameraPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfDeleteCameraPayload) -> super::GltfMutation {
    super::GltfMutation::DeleteCamera(DeleteCameraMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for DeleteCameraMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "camera", kind: "delete-camera", record: "DeletedCamera" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Delete Camera", "Kamera löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-camera".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🚫️removes-the-2257c3/🦀️.rs"]
mod case_removes_the_2257c3;
#[cfg(test)]
#[path = "🧪️tests/🔬️middle-row/🦀️.rs"]
mod case_middle_row;
//#endregion 🧪️Tests
