//! 🦀️ glTF 2.0 `🎬️scene` subset mutation case — Rust adapter for the 33 kinds of the
//! `gltf-2-0-scene` catalog (`../../🔮️oracles/🔣️.json`), which own document/scenes, document/nodes and every node binding. Every leaf stays
//! physically owned by `♾️any`'s aggregate mutation root and is reached by import.
//!
//! The oracle performs each kind by independent GLB/JSON-tree manipulation
//! (`../../../♾️any/🔮️oracles/🦀️.rs`); the subject fully parses the committed `⬅️before.gltf` into
//! `GltfSnapshot`, dispatches through the leaf's own typed `apply()`, and re-serializes from the
//! model alone. The feature's spec names the input fixture and the inverse: either the kinds that
//! undo the mutation, applied the same way on both sides, or — for a kind whose payload cannot
//! carry what it removed — the top-level members restored from the original document.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf, restore_members};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within};


//#region 🔖️Spec
/// 📏️ The `semantic-gltf-v1` writer freedom every glTF mutation case is measured under.
const GLTF_WRITER_FREEDOM: &[&str] = &["byteLength", "fileSize", "generator", "copyright"];

/// 🧫️ A work-directory copy of the spec's own `fixture`; the committed fixture is never written to.
fn mutable_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(&spec.str("fixture"), Some("input.gltf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// ↩️ The spec's inverse kinds, in application order.
fn inverse_specs(spec: &Json) -> Vec<Json> {
    match spec.get("inverse") {
        Some(Json::Array(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// ↩️ The top-level members the inverse restores from the original document, when it does.
fn restored_members(spec: &Json) -> Option<Vec<String>> {
    match spec.get("restore") {
        Some(Json::Array(items)) => Some(
            items
                .iter()
                .filter_map(|item| match item {
                    Json::String(member) => Some(member.clone()),
                    _ => None,
                })
                .collect(),
        ),
        _ => None,
    }
}
//#endregion 🔖️Spec

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, &spec)?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_gltf(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_gltf(&input)?, &[], GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, &spec)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = match restored_members(&spec) {
        Some(members) => restore_members(&mutated, &input, &members)?,
        None => inverse_specs(&spec).iter().try_fold(mutated, |current, inverse| oracle_apply_mutation(&current, inverse))?,
    };
    let projection = project_gltf(&restored)?;
    inverse_restores_within(&spec.str("kind"), &projection, &project_gltf(&input)?, GLTF_WRITER_FREEDOM, 0.0)?;
    Ok(Outcome::with_raw(restored, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{inverse_specs, mutable_input, restored_members};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io::{parse_gltf_document, serialize_gltf_document};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::mutations::{bind_default_scene, bind_node_camera, bind_node_child, bind_node_mesh, bind_node_skin, bind_scene_root_node, change_node_extension_data, change_node_extra_data, change_node_morph_weights, change_node_name, change_node_transform, change_scene_extension_data, change_scene_extra_data, change_scene_name, create_node, create_scene, delete_node, delete_scene, move_node, move_node_child, move_node_parent, move_scene, move_scene_root_node, reorder_node_children, reorder_nodes, reorder_scene_root_nodes, reorder_scenes, unbind_default_scene, unbind_node_camera, unbind_node_child, unbind_node_mesh, unbind_node_skin, unbind_scene_root_node};
    use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::snapshot::{GltfJson, GltfSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

    //#region 🔖️Params
    /// 🔢️ A non-negative integer payload member.
    fn num(params: &Json, key: &str) -> Result<usize, String> {
        match params.get(key) {
            Some(Json::Number(value)) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
            _ => Err(format!("missing or non-integer `{key}`")),
        }
    }
    /// 🔢️ A `u32` payload member.
    fn num_u32(params: &Json, key: &str) -> Result<u32, String> {
        u32::try_from(num(params, key)?).map_err(|_| format!("`{key}` exceeds u32"))
    }
    /// 🔢️ An index list payload member.
    fn order(params: &Json, key: &str) -> Result<Vec<usize>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(value) if *value >= 0.0 && value.fract() == 0.0 => Ok(*value as usize),
                    _ => Err(format!("`{key}` must hold only non-negative integers")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    /// 🔢️ A number list payload member.
    fn floats(params: &Json, key: &str) -> Result<Vec<f64>, String> {
        match params.get(key) {
            Some(Json::Array(items)) => items
                .iter()
                .map(|item| match item {
                    Json::Number(value) => Ok(*value),
                    _ => Err(format!("`{key}` must hold only numbers")),
                })
                .collect(),
            _ => Err(format!("missing or non-array `{key}`")),
        }
    }
    /// 📝️ A string payload member.
    fn text(params: &Json, key: &str) -> Result<String, String> {
        match params.get(key) {
            Some(Json::String(value)) => Ok(value.clone()),
            _ => Err(format!("missing or non-string `{key}`")),
        }
    }
    /// 📝️ A nullable string payload member.
    fn optional_text(params: &Json, key: &str) -> Result<Option<String>, String> {
        match params.get(key) {
            Some(Json::String(value)) => Ok(Some(value.clone())),
            Some(Json::Null) | None => Ok(None),
            _ => Err(format!("`{key}` must be a string or null")),
        }
    }
    //#endregion 🔖️Params

    //#region 🔖️Presence
    /// 🌱️ The snapshot's own JSON value for one spec value.
    fn gltf_json(value: &Json) -> GltfJson {
        match value {
            Json::Null => GltfJson::Null,
            Json::Bool(flag) => GltfJson::Bool(*flag),
            Json::Number(number) => GltfJson::Number(*number),
            Json::String(text) => GltfJson::String(text.clone()),
            Json::Array(items) => GltfJson::Array(items.iter().map(gltf_json).collect()),
            Json::Object(entries) => GltfJson::Object(entries.iter().map(|(key, item)| (key.clone(), gltf_json(item))).collect()),
        }
    }
    /// 🧩️ `data: {state: present, value}` as `Some(value)`, `data: {state: absent}` as `None`.
    fn presence(params: &Json) -> Result<Option<GltfJson>, String> {
        let data = params.get("data").ok_or("missing `data`")?;
        match data.get("state") {
            Some(Json::String(state)) if state == "absent" => Ok(None),
            Some(Json::String(state)) if state == "present" => data.get("value").map(|value| Some(gltf_json(value))).ok_or_else(|| "present data carries no `value`".to_string()),
            _ => Err("`data.state` must be `present` or `absent`".to_string()),
        }
    }
    //#endregion 🔖️Presence

    //#region 🔖️Transform
    /// 🧮️ `transform: {kind: matrix, matrix} | {kind: trs, translation?, rotation?, scale?}`.
    fn transform<T>(params: &Json, matrix: impl Fn([f64; 16]) -> T, trs: impl Fn(Option<[f64; 3]>, Option<[f64; 4]>, Option<[f64; 3]>) -> T) -> Result<T, String> {
        fn fixed<const N: usize>(value: &Json, key: &str) -> Result<Option<[f64; N]>, String> {
            match value.get(key) {
                None | Some(Json::Null) => Ok(None),
                Some(Json::Array(items)) if items.len() == N => {
                    let mut out = [0.0; N];
                    for (slot, item) in out.iter_mut().zip(items) {
                        *slot = match item {
                            Json::Number(number) => *number,
                            _ => return Err(format!("`{key}` must hold only numbers")),
                        };
                    }
                    Ok(Some(out))
                }
                _ => Err(format!("`{key}` must hold {N} numbers")),
            }
        }
        let value = params.get("transform").ok_or("missing `transform`")?;
        match value.get("kind") {
            Some(Json::String(kind)) if kind == "matrix" => Ok(matrix(fixed::<16>(value, "matrix")?.ok_or("missing `matrix`")?)),
            Some(Json::String(kind)) if kind == "trs" => Ok(trs(fixed::<3>(value, "translation")?, fixed::<4>(value, "rotation")?, fixed::<3>(value, "scale")?)),
            _ => Err("`transform.kind` must be `matrix` or `trs`".to_string()),
        }
    }
    //#endregion 🔖️Transform

    //#region 🔖️Dispatch
    /// 📐️ One kind through its leaf's own typed `apply()`.
    fn apply_kind(before: &GltfSnapshot, kind: &str, params: &Json) -> Result<GltfSnapshot, String> {
        match kind {
            "bind-default-scene" => bind_default_scene::apply(&bind_default_scene::GltfBindDefaultScenePayload { scene: num(params, "scene")? }, before).map_err(|error| error.detail),
            "bind-node-camera" => bind_node_camera::apply(&bind_node_camera::GltfBindNodeCameraPayload { node: num(params, "node")?, camera: num(params, "camera")? }, before).map_err(|error| error.detail),
            "bind-node-child" => bind_node_child::apply(&bind_node_child::GltfBindNodeChildPayload { parent: num(params, "parent")?, child: num(params, "child")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "bind-node-mesh" => bind_node_mesh::apply(&bind_node_mesh::GltfBindNodeMeshPayload { node: num(params, "node")?, mesh: num(params, "mesh")? }, before).map_err(|error| error.detail),
            "bind-node-skin" => bind_node_skin::apply(&bind_node_skin::GltfBindNodeSkinPayload { node: num(params, "node")?, skin: num(params, "skin")? }, before).map_err(|error| error.detail),
            "bind-scene-root-node" => bind_scene_root_node::apply(&bind_scene_root_node::GltfBindSceneRootNodePayload { scene: num(params, "scene")?, node: num(params, "node")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "change-node-extension-data" => change_node_extension_data::apply(&change_node_extension_data::GltfChangeNodeExtensionDataPayload { node: num(params, "node")?, data: match presence(params)? { Some(value) => change_node_extension_data::GltfDataPresence::Present { value }, None => change_node_extension_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-node-extra-data" => change_node_extra_data::apply(&change_node_extra_data::GltfChangeNodeExtraDataPayload { node: num(params, "node")?, data: match presence(params)? { Some(value) => change_node_extra_data::GltfDataPresence::Present { value }, None => change_node_extra_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-node-morph-weights" => change_node_morph_weights::apply(&change_node_morph_weights::GltfChangeNodeMorphWeightsPayload { node: num(params, "node")?, weights: floats(params, "weights")? }, before).map_err(|error| error.detail),
            "change-node-name" => change_node_name::apply(&change_node_name::GltfChangeNodeNamePayload { node: num_u32(params, "node")?, value: optional_text(params, "value")? }, before).map_err(|error| error.detail),
            "change-node-transform" => change_node_transform::apply(&change_node_transform::GltfTransformNodePayload { node: num(params, "node")?, transform: transform::<change_node_transform::GltfNodeTransform>(params, |matrix| change_node_transform::GltfNodeTransform::Matrix { matrix }, |translation, rotation, scale| change_node_transform::GltfNodeTransform::Trs { translation, rotation, scale })? }, before).map_err(|error| error.detail),
            "change-scene-extension-data" => change_scene_extension_data::apply(&change_scene_extension_data::GltfChangeSceneExtensionDataPayload { scene: num(params, "scene")?, data: match presence(params)? { Some(value) => change_scene_extension_data::GltfDataPresence::Present { value }, None => change_scene_extension_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-scene-extra-data" => change_scene_extra_data::apply(&change_scene_extra_data::GltfChangeSceneExtraDataPayload { scene: num(params, "scene")?, data: match presence(params)? { Some(value) => change_scene_extra_data::GltfDataPresence::Present { value }, None => change_scene_extra_data::GltfDataPresence::Absent } }, before).map_err(|error| error.detail),
            "change-scene-name" => change_scene_name::apply(&change_scene_name::GltfChangeSceneNamePayload { scene: num(params, "scene")?, value: optional_text(params, "value")? }, before).map_err(|error| error.detail),
            "create-node" => create_node::apply(&create_node::GltfCreateNodePayload { position: num(params, "position")? }, before).map_err(|error| error.detail),
            "create-scene" => create_scene::apply(&create_scene::GltfCreateScenePayload { position: num_u32(params, "position")? }, before).map_err(|error| error.detail),
            "delete-node" => delete_node::apply(&delete_node::GltfDeleteNodePayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "delete-scene" => delete_scene::apply(&delete_scene::GltfDeleteScenePayload { index: num(params, "index")? }, before).map_err(|error| error.detail),
            "move-node" => move_node::apply(&move_node::GltfMoveNodePayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-node-child" => move_node_child::apply(&move_node_child::GltfMoveNodeChildPayload { parent: num(params, "parent")?, child: num(params, "child")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-node-parent" => move_node_parent::apply(&move_node_parent::GltfReparentNodePayload { parent: num(params, "parent")?, child: num(params, "child")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-scene" => move_scene::apply(&move_scene::GltfMoveScenePayload { index: num(params, "index")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "move-scene-root-node" => move_scene_root_node::apply(&move_scene_root_node::GltfMoveSceneRootNodePayload { scene: num(params, "scene")?, node: num(params, "node")?, position: num(params, "position")? }, before).map_err(|error| error.detail),
            "reorder-node-children" => reorder_node_children::apply(&reorder_node_children::GltfReorderNodeChildrenPayload { parent: num(params, "parent")?, order: order(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-nodes" => reorder_nodes::apply(&reorder_nodes::GltfReorderNodesPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-scene-root-nodes" => reorder_scene_root_nodes::apply(&reorder_scene_root_nodes::GltfReorderSceneRootNodesPayload { scene: num(params, "scene")?, order: order(params, "order")? }, before).map_err(|error| error.detail),
            "reorder-scenes" => reorder_scenes::apply(&reorder_scenes::GltfReorderScenesPayload { order: order(params, "order")? }, before).map_err(|error| error.detail),
            "unbind-default-scene" => unbind_default_scene::apply(&unbind_default_scene::GltfUnbindDefaultScenePayload {}, before).map_err(|error| error.detail),
            "unbind-node-camera" => unbind_node_camera::apply(&unbind_node_camera::GltfUnbindNodeCameraPayload { node: num(params, "node")? }, before).map_err(|error| error.detail),
            "unbind-node-child" => unbind_node_child::apply(&unbind_node_child::GltfUnbindNodeChildPayload { parent: num(params, "parent")?, child: num(params, "child")? }, before).map_err(|error| error.detail),
            "unbind-node-mesh" => unbind_node_mesh::apply(&unbind_node_mesh::GltfUnbindNodeMeshPayload { node: num(params, "node")? }, before).map_err(|error| error.detail),
            "unbind-node-skin" => unbind_node_skin::apply(&unbind_node_skin::GltfUnbindNodeSkinPayload { node: num(params, "node")? }, before).map_err(|error| error.detail),
            "unbind-scene-root-node" => unbind_scene_root_node::apply(&unbind_scene_root_node::GltfUnbindSceneRootNodePayload { scene: num(params, "scene")?, node: num(params, "node")? }, before).map_err(|error| error.detail),
            other => Err(format!("unrecognised mutation kind {other:?}")),
        }
    }

    /// ↩️ The named top-level members copied back from `before`.
    fn restore(before: &GltfSnapshot, mutated: &GltfSnapshot, members: &[String]) -> Result<GltfSnapshot, String> {
        let mut restored = mutated.clone();
        for member in members {
            match member.as_str() {
                "scene" => restored.document.scene = before.document.scene,
                "scenes" => restored.document.scenes = before.document.scenes.clone(),
                "nodes" => restored.document.nodes = before.document.nodes.clone(),
                "skins" => restored.document.skins = before.document.skins.clone(),
                "animations" => restored.document.animations = before.document.animations.clone(),
                "meshes" => restored.document.meshes = before.document.meshes.clone(),
                "accessors" => restored.document.accessors = before.document.accessors.clone(),
                "images" => restored.document.images = before.document.images.clone(),
                "bufferViews" => restored.document.buffer_views = before.document.buffer_views.clone(),
                "buffers" => {
                    restored.document.buffers = before.document.buffers.clone();
                    restored.buffers = before.buffers.clone();
                }
                other => return Err(format!("no restore rule for member {other:?}")),
            }
        }
        Ok(restored)
    }
    //#endregion 🔖️Dispatch

    //#region 🔖️Handlers
    fn params(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, &spec)?;
        let before = parse_gltf_document(&input)?;
        let after = apply_kind(&before, &spec.str("kind"), &params(&spec))?;
        let bytes = serialize_gltf_document(&after);
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, &spec)?;
        let before = parse_gltf_document(&input)?;
        let mutated = apply_kind(&before, &spec.str("kind"), &params(&spec))?;
        let restored = match restored_members(&spec) {
            Some(members) => restore(&before, &mutated, &members)?,
            None => inverse_specs(&spec).iter().try_fold(mutated, |current, inverse| apply_kind(&current, &inverse.str("kind"), &params(inverse)))?,
        };
        let bytes = serialize_gltf_document(&restored);
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration
