//! 🦠️ Creates one empty top-level glTF scene at an explicit position.
use crate::artifacts::gltf::schema::mutations::top_level_collections_private::{reject, repair, Change, GltfTopLevelFamily, GltfTopLevelMutationRejection};
use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};
pub const ID: &str = "s.stdio.gltf.mutation.create-scene.v1";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfCreateScenePayload {
    pub position: usize,
}
pub fn validate(payload: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.scenes.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection"));
    }
    Ok(())
}
pub fn apply(payload: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfTopLevelMutationRejection> {
    validate(payload, base)?;
    let mut next = base.clone();
    repair(&mut next.document, GltfTopLevelFamily::Scenes, &Change::Insert(payload.position))?;
    next.document.scenes.insert(payload.position, GltfScene::default());
    Ok(next)
}
