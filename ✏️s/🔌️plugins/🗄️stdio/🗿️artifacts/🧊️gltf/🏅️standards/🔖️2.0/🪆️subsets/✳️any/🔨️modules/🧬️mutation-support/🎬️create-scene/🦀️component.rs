//! 🔒️ Command-local create-scene validation and scene-reference mechanics.

use crate::artifacts::gltf::schema::snapshot::GltfScene;
use crate::artifacts::gltf::GltfSnapshot;

//#region 🔖️Rejection
#[derive(Clone, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateSceneRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn reject(code: impl Into<String>, path: impl Into<String>, detail: impl Into<String>) -> GltfCreateSceneRejection {
    GltfCreateSceneRejection { code: code.into(), path: path.into(), detail: detail.into() }
}
//#endregion 🔖️Rejection

//#region 🔢️U32Domain
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn u32_index(value: usize, path: impl Into<String>) -> Result<u32, GltfCreateSceneRejection> {
    u32::try_from(value).map_err(|_| reject("gltf.mutation.index-out-of-range", path, "index exceeds the u32 command domain"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn usize_index(value: u32, path: impl Into<String>) -> Result<usize, GltfCreateSceneRejection> {
    usize::try_from(value).map_err(|_| reject("gltf.mutation.index-out-of-range", path, "index cannot be represented by this runtime"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn scene_count(scenes: &[GltfScene]) -> Result<u32, GltfCreateSceneRejection> {
    u32::try_from(scenes.len()).map_err(|_| reject("gltf.mutation.collection-overflow", "document/scenes", "scene collection exceeds the u32 command domain"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate_scene_sequence(scenes: &[GltfScene]) -> Result<(), GltfCreateSceneRejection> {
    scene_count(scenes)?;
    for (scene_index, scene) in scenes.iter().enumerate() {
        for node_index in &scene.nodes {
            u32_index(*node_index, format!("document/scenes/{scene_index}/nodes"))?;
        }
    }
    Ok(())
}
//#endregion 🔢️U32Domain

//#region 🎬️SceneState
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn default_scene(snapshot: &GltfSnapshot) -> Result<Option<u32>, GltfCreateSceneRejection> {
    validate_scene_sequence(&snapshot.document.scenes)?;
    snapshot
        .document
        .scene
        .map(|scene| {
            let scene = u32_index(scene, "document/scene")?;
            (usize_index(scene, "document/scene")? < snapshot.document.scenes.len()).then_some(scene).ok_or_else(|| reject("gltf.mutation.reference-out-of-range", "document/scene", "default scene must name an existing scene"))
        })
        .transpose()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insertion_position(position: u32, snapshot: &GltfSnapshot) -> Result<usize, GltfCreateSceneRejection> {
    default_scene(snapshot)?;
    if scene_count(&snapshot.document.scenes)? == u32::MAX {
        return Err(reject("gltf.mutation.collection-overflow", "document/scenes", "creating a scene would exceed the u32 command domain"));
    }
    let position = usize_index(position, "document/scenes")?;
    (position <= snapshot.document.scenes.len()).then_some(position).ok_or_else(|| reject("gltf.mutation.insert-out-of-range", "document/scenes", "position must be within the collection"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn existing_position(position: u32, snapshot: &GltfSnapshot) -> Result<usize, GltfCreateSceneRejection> {
    default_scene(snapshot)?;
    let position = usize_index(position, "document/scenes")?;
    (position < snapshot.document.scenes.len()).then_some(position).ok_or_else(|| reject("gltf.mutation.index-out-of-range", "document/scenes", "position must address an existing scene"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn default_after(default_scene: Option<u32>, position: u32) -> Result<Option<u32>, GltfCreateSceneRejection> {
    default_scene.map(|scene| if scene >= position { scene.checked_add(1).ok_or_else(|| reject("gltf.mutation.reference-overflow", "document/scene", "default scene cannot be remapped beyond u32")) } else { Ok(scene) }).transpose()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_empty_scene(snapshot: &mut GltfSnapshot, position: usize) -> Result<(), GltfCreateSceneRejection> {
    let default_scene_before = default_scene(snapshot)?;
    snapshot.document.scene = default_after(default_scene_before, u32_index(position, "document/scenes")?)?.map(|scene| usize_index(scene, "document/scene")).transpose()?;
    snapshot.document.scenes.insert(position, GltfScene::default());
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_created_scene(snapshot: &mut GltfSnapshot, position: usize, default_scene_before: Option<u32>) -> Result<(), GltfCreateSceneRejection> {
    snapshot.document.scenes.remove(position);
    snapshot.document.scene = default_scene_before.map(|scene| usize_index(scene, "document/scene")).transpose()?;
    Ok(())
}
//#endregion 🎬️SceneState
