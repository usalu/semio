//! 🦠️ Deletes one top-level glTF scene with typed default-scene reference repair.
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, scenes_op, GltfTopLevelFamily, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.delete-scene.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDeleteScenePayload {
    pub index: usize,
}
pub fn validate(payload: &GltfDeleteScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.scenes.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/scenes", "index must address a scene"));
    }
    if base.document.scene.is_some_and(|scene| scene >= base.document.scenes.len()) {
        return Err(reject("gltf.reference.invalid-default-scene", "document/scene", "default scene must address an existing scene"));
    }
    Ok(())
}
pub fn apply(payload: &GltfDeleteScenePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let mut next = base.clone();
    scenes_op(&mut next, GltfTopLevelFamily::Scenes, payload.index, None, None)?;
    Ok(next)
}
