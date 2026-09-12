
use super::*;

fn glb(doc: &json::JsonValue) -> Vec<u8> {
    write_glb(doc, None)
}

fn base_document() -> json::JsonValue {
    json::object! {
        "asset" => json::object!{ "version" => "2.0" },
        "scene" => 0,
        "scenes" => json::array![ json::object!{ "nodes" => json::array![0, 2] } ],
        "nodes" => json::array![
            json::object!{ "children" => json::array![1] },
            json::object!{},
            json::object!{},
        ],
        "materials" => json::array![ json::object!{} ],
    }
}

fn spec(kind: &str, params: Json) -> Json {
    Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params)])
}
fn obj(pairs: Vec<(&str, Json)>) -> Json {
    Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

#[test]
fn bind_and_unbind_node_child_round_trip() {
    let input = glb(&base_document());
    let bound = oracle_apply_mutation(&input, &spec("bind-node-child", obj(vec![("parent", Json::Number(1.0)), ("child", Json::Number(2.0)), ("position", Json::Number(0.0))]))).unwrap();
    assert_ne!(project_gltf(&bound).unwrap(), project_gltf(&input).unwrap());
    let unbound = oracle_apply_mutation(&bound, &spec("unbind-node-child", obj(vec![("parent", Json::Number(1.0)), ("child", Json::Number(2.0))]))).unwrap();
    assert_eq!(project_gltf(&unbound).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn bind_node_child_rejects_cycles_and_duplicates() {
    let input = glb(&base_document());
    assert!(oracle_apply_mutation(&input, &spec("bind-node-child", obj(vec![("parent", Json::Number(1.0)), ("child", Json::Number(0.0)), ("position", Json::Number(0.0))]))).is_err());
    assert!(oracle_apply_mutation(&input, &spec("bind-node-child", obj(vec![("parent", Json::Number(0.0)), ("child", Json::Number(1.0)), ("position", Json::Number(0.0))]))).is_err());
}

#[test]
fn bind_and_unbind_scene_root_node_round_trip() {
    let input = glb(&base_document());
    let bound = oracle_apply_mutation(&input, &spec("bind-scene-root-node", obj(vec![("scene", Json::Number(0.0)), ("node", Json::Number(1.0)), ("position", Json::Number(0.0))]))).unwrap();
    let unbound = oracle_apply_mutation(&bound, &spec("unbind-scene-root-node", obj(vec![("scene", Json::Number(0.0)), ("node", Json::Number(1.0))]))).unwrap();
    assert_eq!(project_gltf(&unbound).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn change_material_alpha_mode_and_double_sided_reject_no_observable_change() {
    let input = glb(&base_document());
    let mutated = oracle_apply_mutation(&input, &spec("change-material-alpha-mode", obj(vec![("material", Json::Number(0.0)), ("alphaMode", Json::String("MASK".to_string()))]))).unwrap();
    assert!(oracle_apply_mutation(&mutated, &spec("change-material-alpha-mode", obj(vec![("material", Json::Number(0.0)), ("alphaMode", Json::String("MASK".to_string()))]))).is_err());
    let mutated = oracle_apply_mutation(&input, &spec("change-material-double-sided", obj(vec![("material", Json::Number(0.0)), ("doubleSided", Json::Bool(true))]))).unwrap();
    assert!(oracle_apply_mutation(&mutated, &spec("change-material-double-sided", obj(vec![("material", Json::Number(0.0)), ("doubleSided", Json::Bool(true))]))).is_err());
}

#[test]
fn create_scene_bumps_default_scene_and_undo_restores_it() {
    let input = glb(&base_document());
    let created = oracle_apply_mutation(&input, &spec("create-scene", obj(vec![("position", Json::Number(0.0))]))).unwrap();
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(default_scene_index(&doc), Some(1));
    assert_eq!(top_level_len(&doc, "scenes"), 2);
    let restored = undo_create_scene(&created, 0).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn unknown_kind_is_an_error_never_a_silent_no_op() {
    let input = glb(&base_document());
    assert!(oracle_apply_mutation(&input, &spec("not-a-real-kind", Json::Object(vec![]))).is_err());
}

fn camera_document() -> json::JsonValue {
    json::object! {
        "asset" => json::object!{ "version" => "2.0" },
        "scene" => 0,
        "scenes" => json::array![ json::object!{ "nodes" => json::array![0] } ],
        "nodes" => json::array![
            json::object!{ "camera" => 0 },
            json::object!{},
        ],
        "cameras" => json::array![
            json::object!{ "type" => "perspective", "perspective" => json::object!{ "yfov" => 0.8, "znear" => 0.1 } },
            json::object!{ "type" => "perspective", "perspective" => json::object!{ "yfov" => 0.5, "znear" => 0.05 } },
        ],
    }
}

fn projection_param(kind: &str, yfov: f64) -> Json {
    Json::Object(vec![("type".to_string(), Json::String(kind.to_string())), (kind.to_string(), Json::Object(vec![("yfov".to_string(), Json::Number(yfov)), ("znear".to_string(), Json::Number(0.1))]))])
}

#[test]
fn create_and_delete_camera_round_trip() {
    let input = glb(&camera_document());
    let created = oracle_apply_mutation(&input, &spec("create-camera", obj(vec![("position", Json::Number(2.0)), ("projection", projection_param("perspective", 1.0))]))).unwrap();
    assert_ne!(project_gltf(&created).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(top_level_len(&doc, "cameras"), 3);
    let deleted = oracle_apply_mutation(&created, &spec("delete-camera", obj(vec![("index", Json::Number(2.0))]))).unwrap();
    assert_eq!(project_gltf(&deleted).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn delete_camera_clears_the_referencing_node() {
    let input = glb(&camera_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-camera", obj(vec![("index", Json::Number(0.0))]))).unwrap();
    let (doc, _) = read_glb(&deleted).unwrap();
    assert_eq!(obj_get(&arr(&doc, "nodes")[0], "camera"), None);
    assert_eq!(top_level_len(&doc, "cameras"), 1);
}

#[test]
fn move_camera_is_its_own_inverse_with_swapped_arguments() {
    let input = glb(&camera_document());
    let moved = oracle_apply_mutation(&input, &spec("move-camera", obj(vec![("index", Json::Number(0.0)), ("position", Json::Number(1.0))]))).unwrap();
    let (doc, _) = read_glb(&moved).unwrap();
    assert_eq!(obj_get(&arr(&doc, "nodes")[0], "camera").and_then(json::JsonValue::as_usize), Some(1));
    let restored = oracle_apply_mutation(&moved, &spec("move-camera", obj(vec![("index", Json::Number(1.0)), ("position", Json::Number(0.0))]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    assert!(oracle_apply_mutation(&input, &spec("move-camera", obj(vec![("index", Json::Number(0.0)), ("position", Json::Number(0.0))]))).is_err());
}

#[test]
fn reorder_cameras_swap_is_self_inverse() {
    let input = glb(&camera_document());
    let order = Json::Array(vec![Json::Number(1.0), Json::Number(0.0)]);
    let reordered = oracle_apply_mutation(&input, &spec("reorder-cameras", obj(vec![("order", order.clone())]))).unwrap();
    let (doc, _) = read_glb(&reordered).unwrap();
    assert_eq!(obj_get(&arr(&doc, "nodes")[0], "camera").and_then(json::JsonValue::as_usize), Some(1));
    let restored = oracle_apply_mutation(&reordered, &spec("reorder-cameras", obj(vec![("order", order)]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    let identity = Json::Array(vec![Json::Number(0.0), Json::Number(1.0)]);
    assert!(oracle_apply_mutation(&input, &spec("reorder-cameras", obj(vec![("order", identity)]))).is_err());
}

fn skin_document() -> json::JsonValue {
    json::object! {
        "asset" => json::object!{ "version" => "2.0" },
        "scene" => 0,
        "scenes" => json::array![ json::object!{ "nodes" => json::array![0] } ],
        "nodes" => json::array![
            json::object!{ "skin" => 0 },
            json::object!{},
        ],
        "skins" => json::array![
            json::object!{ "joints" => json::array![0] },
            json::object!{ "joints" => json::array![1] },
        ],
    }
}

#[test]
fn create_and_delete_skin_round_trip() {
    let input = glb(&skin_document());
    let created = oracle_apply_mutation(&input, &spec("create-skin", obj(vec![("position", Json::Number(2.0))]))).unwrap();
    assert_ne!(project_gltf(&created).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(top_level_len(&doc, "skins"), 3);
    let deleted = oracle_apply_mutation(&created, &spec("delete-skin", obj(vec![("index", Json::Number(2.0))]))).unwrap();
    assert_eq!(project_gltf(&deleted).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn delete_skin_clears_the_referencing_node() {
    let input = glb(&skin_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-skin", obj(vec![("index", Json::Number(0.0))]))).unwrap();
    let (doc, _) = read_glb(&deleted).unwrap();
    assert_eq!(obj_get(&arr(&doc, "nodes")[0], "skin"), None);
    assert_eq!(top_level_len(&doc, "skins"), 1);
}

#[test]
fn move_skin_is_its_own_inverse_with_swapped_arguments() {
    let input = glb(&skin_document());
    let moved = oracle_apply_mutation(&input, &spec("move-skin", obj(vec![("index", Json::Number(0.0)), ("position", Json::Number(1.0))]))).unwrap();
    let (doc, _) = read_glb(&moved).unwrap();
    assert_eq!(obj_get(&arr(&doc, "nodes")[0], "skin").and_then(json::JsonValue::as_usize), Some(1));
    let restored = oracle_apply_mutation(&moved, &spec("move-skin", obj(vec![("index", Json::Number(1.0)), ("position", Json::Number(0.0))]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    assert!(oracle_apply_mutation(&input, &spec("move-skin", obj(vec![("index", Json::Number(0.0)), ("position", Json::Number(0.0))]))).is_err());
}

#[test]
fn reorder_skins_swap_is_self_inverse() {
    let input = glb(&skin_document());
    let order = Json::Array(vec![Json::Number(1.0), Json::Number(0.0)]);
    let reordered = oracle_apply_mutation(&input, &spec("reorder-skins", obj(vec![("order", order.clone())]))).unwrap();
    let (doc, _) = read_glb(&reordered).unwrap();
    assert_eq!(obj_get(&arr(&doc, "nodes")[0], "skin").and_then(json::JsonValue::as_usize), Some(1));
    let restored = oracle_apply_mutation(&reordered, &spec("reorder-skins", obj(vec![("order", order)]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    let identity = Json::Array(vec![Json::Number(0.0), Json::Number(1.0)]);
    assert!(oracle_apply_mutation(&input, &spec("reorder-skins", obj(vec![("order", identity)]))).is_err());
}

#[test]
fn undo_delete_skin_restores_the_original_content_not_an_empty_substitute() {
    let input = glb(&skin_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-skin", obj(vec![("index", Json::Number(0.0))]))).unwrap();
    let restored = undo_delete_skin(&deleted, &input).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&restored).unwrap();
    assert_eq!(arr(&doc, "skins")[0], json::object! { "joints" => json::array![0] });
}

fn animation_document() -> json::JsonValue {
    json::object! {
        "asset" => json::object!{ "version" => "2.0" },
        "nodes" => json::array![ json::object!{}, json::object!{} ],
        "animations" => json::array![
            json::object!{ "name" => "clip0", "channels" => json::array![], "samplers" => json::array![] },
            json::object!{ "name" => "clip1", "channels" => json::array![], "samplers" => json::array![] },
        ],
    }
}

#[test]
fn create_and_delete_animation_round_trip() {
    let input = glb(&animation_document());
    let created = oracle_apply_mutation(&input, &spec("create-animation", obj(vec![("position", Json::Number(2.0))]))).unwrap();
    assert_ne!(project_gltf(&created).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(top_level_len(&doc, "animations"), 3);
    let deleted = oracle_apply_mutation(&created, &spec("delete-animation", obj(vec![("index", Json::Number(2.0))]))).unwrap();
    assert_eq!(project_gltf(&deleted).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn undo_delete_animation_restores_the_original_content_not_an_empty_substitute() {
    let input = glb(&animation_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-animation", obj(vec![("index", Json::Number(0.0))]))).unwrap();
    let restored = undo_delete_animation(&deleted, &input).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&restored).unwrap();
    assert_eq!(obj_get(&arr(&doc, "animations")[0], "name").and_then(json::JsonValue::as_str), Some("clip0"));
}

#[test]
fn move_and_reorder_animations_round_trip() {
    let input = glb(&animation_document());
    let moved = oracle_apply_mutation(&input, &spec("move-animation", obj(vec![("index", Json::Number(0.0)), ("position", Json::Number(1.0))]))).unwrap();
    let restored = oracle_apply_mutation(&moved, &spec("move-animation", obj(vec![("index", Json::Number(1.0)), ("position", Json::Number(0.0))]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    assert!(oracle_apply_mutation(&input, &spec("move-animation", obj(vec![("index", Json::Number(0.0)), ("position", Json::Number(0.0))]))).is_err());
    let order = Json::Array(vec![Json::Number(1.0), Json::Number(0.0)]);
    let reordered = oracle_apply_mutation(&input, &spec("reorder-animations", obj(vec![("order", order.clone())]))).unwrap();
    let restored = oracle_apply_mutation(&reordered, &spec("reorder-animations", obj(vec![("order", order)]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn glb_round_trip_preserves_the_bin_chunk() {
    let (doc, _) = read_glb(&glb(&base_document())).unwrap();
    let bin_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    let with_bin = write_glb(&doc, Some(&bin_data));
    let (_, read_back) = read_glb(&with_bin).unwrap();
    assert_eq!(read_back, Some(bin_data));
}

fn material_document() -> json::JsonValue {
    json::object! {
        "asset" => json::object!{ "version" => "2.0" },
        "meshes" => json::array![
            json::object!{ "primitives" => json::array![ json::object!{ "material" => 0 } ] },
            json::object!{ "primitives" => json::array![ json::object!{ "material" => 1 }, json::object!{ "material" => 1 } ] },
        ],
        "materials" => json::array![
            json::object!{ "name" => "matA" },
            json::object!{ "name" => "matB" },
        ],
    }
}

#[test]
fn create_and_delete_material_round_trip() {
    let input = glb(&material_document());
    let created = oracle_apply_mutation(&input, &spec("create-material", obj(vec![("position", Json::Number(2.0))]))).unwrap();
    assert_ne!(project_gltf(&created).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(top_level_len(&doc, "materials"), 3);
    let deleted = oracle_apply_mutation(&created, &spec("delete-material", obj(vec![("index", Json::Number(2.0))]))).unwrap();
    assert_eq!(project_gltf(&deleted).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn delete_material_clears_every_referencing_primitive() {
    let input = glb(&material_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-material", obj(vec![("index", Json::Number(1.0))]))).unwrap();
    let (doc, _) = read_glb(&deleted).unwrap();
    let meshes = arr(&doc, "meshes");
    assert_eq!(obj_get(&arr(&meshes[1], "primitives")[0], "material"), None);
    assert_eq!(obj_get(&arr(&meshes[1], "primitives")[1], "material"), None);
    assert_eq!(top_level_len(&doc, "materials"), 1);
}

#[test]
fn undo_delete_material_restores_the_original_content_and_every_reference() {
    let input = glb(&material_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-material", obj(vec![("index", Json::Number(1.0))]))).unwrap();
    let restored = undo_delete_material(&deleted, &input).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&restored).unwrap();
    assert_eq!(arr(&doc, "materials")[1], json::object! { "name" => "matB" });
    let meshes = arr(&doc, "meshes");
    assert_eq!(obj_get(&arr(&meshes[1], "primitives")[0], "material").and_then(json::JsonValue::as_usize), Some(1));
}

#[test]
fn move_and_reorder_materials_track_primitive_references() {
    let input = glb(&material_document());
    let moved = oracle_apply_mutation(&input, &spec("move-material", obj(vec![("index", Json::Number(0.0)), ("position", Json::Number(1.0))]))).unwrap();
    let (doc, _) = read_glb(&moved).unwrap();
    let meshes = arr(&doc, "meshes");
    assert_eq!(obj_get(&arr(&meshes[0], "primitives")[0], "material").and_then(json::JsonValue::as_usize), Some(1));
    assert_eq!(obj_get(&arr(&meshes[1], "primitives")[0], "material").and_then(json::JsonValue::as_usize), Some(0));
    let restored = oracle_apply_mutation(&moved, &spec("move-material", obj(vec![("index", Json::Number(1.0)), ("position", Json::Number(0.0))]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    let order = Json::Array(vec![Json::Number(1.0), Json::Number(0.0)]);
    let reordered = oracle_apply_mutation(&input, &spec("reorder-materials", obj(vec![("order", order.clone())]))).unwrap();
    let (doc, _) = read_glb(&reordered).unwrap();
    let meshes = arr(&doc, "meshes");
    assert_eq!(obj_get(&arr(&meshes[0], "primitives")[0], "material").and_then(json::JsonValue::as_usize), Some(1));
    let restored = oracle_apply_mutation(&reordered, &spec("reorder-materials", obj(vec![("order", order)]))).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
}

fn texture_document() -> json::JsonValue {
    json::object! {
        "asset" => json::object!{ "version" => "2.0" },
        "textures" => json::array![
            json::object!{ "source" => 0, "sampler" => 0 },
            json::object!{ "source" => 1, "sampler" => 1 },
        ],
        "images" => json::array![ json::object!{ "uri" => "a.png" }, json::object!{ "uri" => "b.png" } ],
        "samplers" => json::array![ json::object!{ "magFilter" => 9729 }, json::object!{ "magFilter" => 9728 } ],
        "materials" => json::array![
            json::object!{
                "pbrMetallicRoughness" => json::object!{ "baseColorTexture" => json::object!{ "index" => 0, "texCoord" => 1 } },
                "normalTexture" => json::object!{ "index" => 1, "scale" => 2.0 },
            },
        ],
    }
}

#[test]
fn create_and_delete_texture_round_trip() {
    let input = glb(&texture_document());
    let created = oracle_apply_mutation(&input, &spec("create-texture", obj(vec![("position", Json::Number(2.0))]))).unwrap();
    assert_ne!(project_gltf(&created).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(top_level_len(&doc, "textures"), 3);
    let deleted = oracle_apply_mutation(&created, &spec("delete-texture", obj(vec![("index", Json::Number(2.0))]))).unwrap();
    assert_eq!(project_gltf(&deleted).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn delete_texture_clears_the_whole_texture_info_object_not_just_the_index() {
    let input = glb(&texture_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-texture", obj(vec![("index", Json::Number(0.0))]))).unwrap();
    let (doc, _) = read_glb(&deleted).unwrap();
    let material = &arr(&doc, "materials")[0];
    let pbr = obj_get(material, "pbrMetallicRoughness").unwrap();
    assert_eq!(obj_get(pbr, "baseColorTexture"), None);
    let normal = obj_get(material, "normalTexture").unwrap();
    assert_eq!(obj_get(normal, "index").and_then(json::JsonValue::as_usize), Some(0));
    assert_eq!(obj_get(normal, "scale").and_then(json::JsonValue::as_f64), Some(2.0));
}

#[test]
fn undo_delete_texture_restores_the_whole_texture_info_object() {
    let input = glb(&texture_document());
    let deleted = oracle_apply_mutation(&input, &spec("delete-texture", obj(vec![("index", Json::Number(0.0))]))).unwrap();
    let restored = undo_delete_texture(&deleted, &input).unwrap();
    assert_eq!(project_gltf(&restored).unwrap(), project_gltf(&input).unwrap());
    let (doc, _) = read_glb(&restored).unwrap();
    let material = &arr(&doc, "materials")[0];
    let pbr = obj_get(material, "pbrMetallicRoughness").unwrap();
    let base_color = obj_get(pbr, "baseColorTexture").unwrap();
    assert_eq!(obj_get(base_color, "index").and_then(json::JsonValue::as_usize), Some(0));
    assert_eq!(obj_get(base_color, "texCoord").and_then(json::JsonValue::as_usize), Some(1));
}

#[test]
fn create_and_delete_image_round_trip_updates_texture_source() {
    let input = glb(&texture_document());
    let created = oracle_apply_mutation(&input, &spec("create-image", obj(vec![("position", Json::Number(1.0))]))).unwrap();
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(top_level_len(&doc, "images"), 3);
    assert_eq!(obj_get(&arr(&doc, "textures")[1], "source").and_then(json::JsonValue::as_usize), Some(2));
    let deleted = oracle_apply_mutation(&created, &spec("delete-image", obj(vec![("index", Json::Number(1.0))]))).unwrap();
    assert_eq!(project_gltf(&deleted).unwrap(), project_gltf(&input).unwrap());
}

#[test]
fn create_and_delete_sampler_round_trip_updates_texture_sampler() {
    let input = glb(&texture_document());
    let created = oracle_apply_mutation(&input, &spec("create-sampler", obj(vec![("position", Json::Number(1.0))]))).unwrap();
    let (doc, _) = read_glb(&created).unwrap();
    assert_eq!(top_level_len(&doc, "samplers"), 3);
    assert_eq!(obj_get(&arr(&doc, "textures")[1], "sampler").and_then(json::JsonValue::as_usize), Some(2));
    let deleted = oracle_apply_mutation(&created, &spec("delete-sampler", obj(vec![("index", Json::Number(1.0))]))).unwrap();
    assert_eq!(project_gltf(&deleted).unwrap(), project_gltf(&input).unwrap());
}
