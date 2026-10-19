//! 🧭️ The oracle kind dispatch, the round trip and the normalized `project_gltf` projection.

use super::*;

//#region 🔖️Dispatch
fn apply(doc: &mut json::JsonValue, kind: &str, params: &Json) -> Result<(), String> {
    if let Some(result) = apply_structure(doc, kind, params) {
        return result;
    }
    match kind {
        "change-material-alpha-mode" => change_material_alpha_mode(doc, usize_param(params, "material")?, &str_param(params, "alphaMode")?),
        "change-material-double-sided" => change_material_double_sided(doc, usize_param(params, "material")?, bool_param(params, "doubleSided")?),
        "create-camera" => create_camera(doc, usize_param(params, "position")?, object_param(params, "projection")?),
        "delete-camera" => delete_camera(doc, usize_param(params, "index")?),
        "move-camera" => move_camera(doc, usize_param(params, "index")?, usize_param(params, "position")?),
        "reorder-cameras" => reorder_cameras(doc, &usize_array_param(params, "order")?),
        "create-skin" => create_skin(doc, usize_param(params, "position")?),
        "delete-skin" => delete_skin(doc, usize_param(params, "index")?),
        "move-skin" => move_skin(doc, usize_param(params, "index")?, usize_param(params, "position")?),
        "reorder-skins" => reorder_skins(doc, &usize_array_param(params, "order")?),
        "create-animation" => create_animation(doc, usize_param(params, "position")?),
        "delete-animation" => delete_animation(doc, usize_param(params, "index")?),
        "move-animation" => move_animation(doc, usize_param(params, "index")?, usize_param(params, "position")?),
        "reorder-animations" => reorder_animations(doc, &usize_array_param(params, "order")?),
        "add-required-extension" => add_extension(doc, "extensionsRequired", &str_param(params, "extension")?, usize_param(params, "position")?),
        "add-used-extension" => add_extension(doc, "extensionsUsed", &str_param(params, "extension")?, usize_param(params, "position")?),
        "remove-required-extension" => remove_extension(doc, "extensionsRequired", &str_param(params, "extension")?),
        "remove-used-extension" => remove_extension(doc, "extensionsUsed", &str_param(params, "extension")?),
        "move-required-extension" => move_extension(doc, "extensionsRequired", &str_param(params, "extension")?, usize_param(params, "position")?),
        "move-used-extension" => move_extension(doc, "extensionsUsed", &str_param(params, "extension")?, usize_param(params, "position")?),
        "reorder-required-extensions" => reorder_extensions(doc, "extensionsRequired", &string_array_param(params, "order")?),
        "reorder-used-extensions" => reorder_extensions(doc, "extensionsUsed", &string_array_param(params, "order")?),
        "change-asset-descriptive-metadata" => change_asset_descriptive_metadata(doc, optional_str_param(params, "generator").as_deref(), optional_str_param(params, "copyright").as_deref(), optional_str_param(params, "minVersion").as_deref()),
        "change-asset-version" => change_asset_version(doc, &str_param(params, "version")?),
        "change-asset-extension-data" => change_asset_extension_data(doc, optional_object_param(params, "data")),
        "change-asset-extra-data" => change_asset_extra_data(doc, optional_object_param(params, "data")),
        "change-document-extension-data" => change_document_extension_data(doc, optional_object_param(params, "data")),
        "change-document-extra-data" => change_document_extra_data(doc, optional_object_param(params, "data")),
        "create-material" => create_material(doc, usize_param(params, "position")?),
        "delete-material" => delete_material(doc, usize_param(params, "index")?),
        "move-material" => move_material(doc, usize_param(params, "index")?, usize_param(params, "position")?),
        "reorder-materials" => reorder_materials(doc, &usize_array_param(params, "order")?),
        "create-image" => create_image(doc, usize_param(params, "position")?),
        "delete-image" => delete_image(doc, usize_param(params, "index")?),
        "move-image" => move_image(doc, usize_param(params, "index")?, usize_param(params, "position")?),
        "reorder-images" => reorder_images(doc, &usize_array_param(params, "order")?),
        "create-sampler" => create_sampler(doc, usize_param(params, "position")?),
        "delete-sampler" => delete_sampler(doc, usize_param(params, "index")?),
        "move-sampler" => move_sampler(doc, usize_param(params, "index")?, usize_param(params, "position")?),
        "reorder-samplers" => reorder_samplers(doc, &usize_array_param(params, "order")?),
        "create-texture" => create_texture(doc, usize_param(params, "position")?),
        "delete-texture" => delete_texture(doc, usize_param(params, "index")?),
        "move-texture" => move_texture(doc, usize_param(params, "index")?, usize_param(params, "position")?),
        "reorder-textures" => reorder_textures(doc, &usize_array_param(params, "order")?),
        other => Err(format!("mutation kind {other:?} has no oracle implementation")),
    }
}

/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op — a mutation that is quietly skipped
/// reports as a passing test.
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let empty = Json::Object(Vec::new());
    let params = spec.get("params").unwrap_or(&empty);
    let (mut doc, bin) = read_glb(input)?;
    apply(&mut doc, &kind, params)?;
    Ok(write_glb(&doc, bin.as_deref()))
}


/// 🔁️ The oracle's own decode/re-encode, entirely through the independent GLB codec above — the
/// identity-round-trip scenario's oracle side.
pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    let (doc, bin) = read_glb(input)?;
    Ok(write_glb(&doc, bin.as_deref()))
}
//#endregion 🔖️Dispatch

//#region 🔖️Projection
/// 🧾️ One tree object as a fixed member list: every listed member present, a missing one at its
/// declared default, so a writer that spells a default out and one that omits it project the same.
fn normalized(item: &json::JsonValue, members: &[(&str, Json)]) -> Json {
    Json::Object(members.iter().map(|(key, default)| ((*key).to_string(), obj_get(item, key).map(to_host_json).unwrap_or_else(|| default.clone()))).collect())
}

/// 🔤️ A semantic → accessor map as ordered `[semantic, accessor]` pairs, so key order is observed.
fn semantic_pairs(map: Option<&json::JsonValue>) -> Json {
    match map {
        Some(json::JsonValue::Object(object)) => Json::Array(object.iter().map(|(key, value)| Json::Array(vec![Json::String(key.to_string()), to_host_json(value)])).collect()),
        _ => Json::Array(Vec::new()),
    }
}

/// 💾️ One buffer's bytes as lowercase hex: a `data:` URI decoded, the binary chunk for a
/// URI-less first buffer, `null` for an external URI this reader does not follow.
fn buffer_bytes(buffer: &json::JsonValue, index: usize, bin: Option<&[u8]>) -> Json {
    let bytes = match obj_get(buffer, "uri").and_then(json::JsonValue::as_str) {
        Some(uri) => uri.strip_prefix("data:").and_then(|rest| rest.split_once(";base64,")).and_then(|(_, payload)| base64_decode(payload)),
        None if index == 0 => bin.map(|data| data[..obj_get(buffer, "byteLength").and_then(json::JsonValue::as_usize).unwrap_or(data.len()).min(data.len())].to_vec()),
        None => None,
    };
    bytes.map(|data| Json::String(data.iter().map(|byte| format!("{byte:02x}")).collect())).unwrap_or(Json::Null)
}
/// 👁️ Projects GLB/`.gltf` bytes with the INDEPENDENT container/JSON reader onto the
/// `semantic-gltf-v1` shape every glTF mutation case's oracle and subject are compared through:
/// the default scene and every scene, node, mesh (primitive attributes and morph targets as ordered
/// semantic pairs), accessor, buffer view and buffer (its bytes decoded, never its URI spelling),
/// plus the camera, skin, animation, asset, extension and material-family members. Each object is
/// normalized to a fixed member list with the format's defaults, so a writer that spells a default
/// out and one that omits it project identically.
pub fn project_gltf(bytes: &[u8]) -> Result<Json, String> {
    let (doc, bin) = read_glb(bytes)?;
    let default_scene = match default_scene_index(&doc) {
        Some(index) => Json::Number(index as f64),
        None => Json::Null,
    };
    let empty = || Json::Array(Vec::new());
    let scenes: Vec<Json> = arr(&doc, "scenes").iter().map(|scene| normalized(scene, &[("name", Json::Null), ("nodes", empty()), ("extras", Json::Null), ("extensions", Json::Null)])).collect();
    let nodes: Vec<Json> = arr(&doc, "nodes")
        .iter()
        .map(|node| {
            normalized(
                node,
                &[
                    ("name", Json::Null),
                    ("children", empty()),
                    ("mesh", Json::Null),
                    ("camera", Json::Null),
                    ("skin", Json::Null),
                    ("matrix", Json::Null),
                    ("translation", Json::Null),
                    ("rotation", Json::Null),
                    ("scale", Json::Null),
                    ("weights", empty()),
                    ("extras", Json::Null),
                    ("extensions", Json::Null),
                ],
            )
        })
        .collect();
    let meshes: Vec<Json> = arr(&doc, "meshes")
        .iter()
        .map(|mesh| {
            let primitives: Vec<Json> = arr(mesh, "primitives")
                .iter()
                .map(|primitive| {
                    let Json::Object(mut members) = normalized(primitive, &[("indices", Json::Null), ("material", Json::Null), ("mode", Json::Null), ("extras", Json::Null), ("extensions", Json::Null)]) else { unreachable!("normalized is an object") };
                    members.push(("attributes".to_string(), semantic_pairs(obj_get(primitive, "attributes"))));
                    members.push(("targets".to_string(), Json::Array(arr(primitive, "targets").iter().map(|target| semantic_pairs(Some(target))).collect())));
                    Json::Object(members)
                })
                .collect();
            let Json::Object(mut members) = normalized(mesh, &[("name", Json::Null), ("weights", empty()), ("extras", Json::Null), ("extensions", Json::Null)]) else { unreachable!("normalized is an object") };
            members.push(("primitives".to_string(), Json::Array(primitives)));
            Json::Object(members)
        })
        .collect();
    let accessors: Vec<Json> = arr(&doc, "accessors")
        .iter()
        .map(|accessor| {
            normalized(
                accessor,
                &[
                    ("bufferView", Json::Null),
                    ("byteOffset", Json::Number(0.0)),
                    ("componentType", Json::Null),
                    ("normalized", Json::Bool(false)),
                    ("count", Json::Null),
                    ("type", Json::Null),
                    ("max", Json::Null),
                    ("min", Json::Null),
                    ("sparse", Json::Null),
                    ("name", Json::Null),
                    ("extras", Json::Null),
                    ("extensions", Json::Null),
                ],
            )
        })
        .collect();
    let buffer_views: Vec<Json> = arr(&doc, "bufferViews")
        .iter()
        .map(|view| normalized(view, &[("buffer", Json::Null), ("byteOffset", Json::Number(0.0)), ("byteLength", Json::Null), ("byteStride", Json::Null), ("target", Json::Null), ("name", Json::Null), ("extras", Json::Null), ("extensions", Json::Null)]))
        .collect();
    let buffers: Vec<Json> = arr(&doc, "buffers")
        .iter()
        .enumerate()
        .map(|(index, buffer)| {
            let Json::Object(mut members) = normalized(buffer, &[("name", Json::Null), ("extras", Json::Null), ("extensions", Json::Null)]) else { unreachable!("normalized is an object") };
            members.push(("bytes".to_string(), buffer_bytes(buffer, index, bin.as_deref())));
            Json::Object(members)
        })
        .collect();
    let materials: Vec<Json> = (0..top_level_len(&doc, "materials"))
        .map(|index| Json::Object(vec![("alphaMode".to_string(), Json::String(material_alpha_mode(&doc, index))), ("doubleSided".to_string(), Json::Bool(material_double_sided(&doc, index)))]))
        .collect();
    let cameras: Vec<Json> = arr(&doc, "cameras").iter().map(to_host_json).collect();
    let skins: Vec<Json> = arr(&doc, "skins").iter().map(to_host_json).collect();
    let animations: Vec<Json> = arr(&doc, "animations")
        .iter()
        .map(|animation| {
            let samplers = arr(animation, "samplers").iter().map(|sampler| normalized(sampler, &[("input", Json::Null), ("output", Json::Null), ("interpolation", Json::String("LINEAR".to_string())), ("extras", Json::Null), ("extensions", Json::Null)])).collect();
            let Json::Object(mut members) = normalized(animation, &[("name", Json::Null), ("channels", Json::Array(Vec::new())), ("extras", Json::Null), ("extensions", Json::Null)]) else { unreachable!("normalized is an object") };
            members.push(("samplers".to_string(), Json::Array(samplers)));
            Json::Object(members)
        })
        .collect();
    let asset = obj_get(&doc, "asset").map(to_host_json).unwrap_or(Json::Null);
    let extensions_used: Vec<Json> = arr(&doc, "extensionsUsed").iter().filter_map(json::JsonValue::as_str).map(|value| Json::String(value.to_string())).collect();
    let extensions_required: Vec<Json> = arr(&doc, "extensionsRequired").iter().filter_map(json::JsonValue::as_str).map(|value| Json::String(value.to_string())).collect();
    let document_extensions = obj_get(&doc, "extensions").map(to_host_json).unwrap_or(Json::Null);
    let document_extras = obj_get(&doc, "extras").map(to_host_json).unwrap_or(Json::Null);
    let materials_full: Vec<Json> = arr(&doc, "materials").iter().map(to_host_json).collect();
    let textures: Vec<Json> = arr(&doc, "textures").iter().map(to_host_json).collect();
    let images: Vec<Json> = arr(&doc, "images").iter().map(to_host_json).collect();
    let samplers: Vec<Json> = arr(&doc, "samplers").iter().map(to_host_json).collect();
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("gltf".to_string())),
        ("defaultScene".to_string(), default_scene),
        ("sceneCount".to_string(), Json::Number(scenes.len() as f64)),
        ("scenes".to_string(), Json::Array(scenes)),
        ("nodeCount".to_string(), Json::Number(nodes.len() as f64)),
        ("nodes".to_string(), Json::Array(nodes)),
        ("materialCount".to_string(), Json::Number(materials.len() as f64)),
        ("materials".to_string(), Json::Array(materials)),
        ("cameraCount".to_string(), Json::Number(cameras.len() as f64)),
        ("cameras".to_string(), Json::Array(cameras)),
        ("skinCount".to_string(), Json::Number(skins.len() as f64)),
        ("skins".to_string(), Json::Array(skins)),
        ("animationCount".to_string(), Json::Number(animations.len() as f64)),
        ("animations".to_string(), Json::Array(animations)),
        ("asset".to_string(), asset),
        ("extensionsUsed".to_string(), Json::Array(extensions_used)),
        ("extensionsRequired".to_string(), Json::Array(extensions_required)),
        ("documentExtensions".to_string(), document_extensions),
        ("documentExtras".to_string(), document_extras),
        ("materialsFull".to_string(), Json::Array(materials_full)),
        ("textureCount".to_string(), Json::Number(textures.len() as f64)),
        ("textures".to_string(), Json::Array(textures)),
        ("imageCount".to_string(), Json::Number(images.len() as f64)),
        ("images".to_string(), Json::Array(images)),
        ("samplerCount".to_string(), Json::Number(samplers.len() as f64)),
        ("samplers".to_string(), Json::Array(samplers)),
        ("meshCount".to_string(), Json::Number(meshes.len() as f64)),
        ("meshes".to_string(), Json::Array(meshes)),
        ("accessorCount".to_string(), Json::Number(accessors.len() as f64)),
        ("accessors".to_string(), Json::Array(accessors)),
        ("bufferViewCount".to_string(), Json::Number(buffer_views.len() as f64)),
        ("bufferViews".to_string(), Json::Array(buffer_views)),
        ("bufferCount".to_string(), Json::Number(buffers.len() as f64)),
        ("buffers".to_string(), Json::Array(buffers)),
    ]))
}
//#endregion 🔖️Projection
