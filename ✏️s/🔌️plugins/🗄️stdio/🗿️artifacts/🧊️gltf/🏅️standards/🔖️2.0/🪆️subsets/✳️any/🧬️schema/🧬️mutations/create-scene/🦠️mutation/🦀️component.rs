//! 🦠️ Creates one empty top-level glTF scene at an explicit u32 position.

use crate::artifacts::gltf::schema::mutations::create_scene::private::{insert_empty_scene, insertion_position, GltfCreateSceneRejection};
use crate::artifacts::gltf::GltfSnapshot;
use serde::{Deserialize, Serialize};

pub const ID: &str = "s.stdio.gltf.mutation.create-scene.v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GltfCreateScenePayload {
    pub position: u32,
}

pub async fn validate(payload: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<(), GltfCreateSceneRejection> {
    insertion_position(payload.position, base).map(|_| ())
}

pub async fn apply(payload: &GltfCreateScenePayload, base: &GltfSnapshot) -> Result<GltfSnapshot, GltfCreateSceneRejection> {
    let position = insertion_position(payload.position, base)?;
    let mut next = base.clone();
    insert_empty_scene(&mut next, position)?;
    Ok(next)
}
