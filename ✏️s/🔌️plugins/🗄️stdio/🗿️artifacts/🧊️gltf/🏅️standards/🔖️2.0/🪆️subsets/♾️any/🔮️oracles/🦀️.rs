//! 🔮️ Mutation oracle for this subset — the kinds of `GltfMutation` (`../🧬️schema/🧬️mutations/
//! 🦀️.rs`) performed by an independent GLB container codec plus generic JSON manipulation, so the
//! subject's own mutation has a second producer to be compared against instead of being checked
//! against its own reading. It is a `cross-semio-implementation`: the JSON tokenizer is third-party,
//! the glTF semantics below are this repository's own second reading of the specification.
//!
//! **Why `json` (json-rust), not the `gltf` crate (1.4.1, MIT):** `gltf` is a credible, actively
//! maintained reader and IS already production-reachable in this repository — but confirmed
//! genuinely independent of this subset's own codec first. `crate::schema::
//! snapshot::🦀️.rs`'s `GltfSnapshot`/`GltfDocument`/`GltfJson` never names `gltf::`
//! anywhere (no `impl From<gltf::…>`, no import), and `decode_glb`/`encode_glb`/`parse_gltf_document`
//! (`../🚪️io/🦀️.rs`) are hand-rolled over `serde_json` alone. Every real `gltf::` call site
//! in this repository lives in `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs`
//! (`mesh_to_glb`/`mesh_from_glb`/`GlbExporter`/`GlbImporter`, byte-in/byte-out, no `gltf::` type
//! crosses that boundary) — reached from `semio-s-plugin-stdio` only through the unrelated BREP/DWG
//! mesh-IO codecs, never from this artifact's own tree. That is a small, nameable production surface
//! exactly like the `image`/`png` `productionDebt` precedent, so registering `gltf` here WOULD have
//! been legitimate. It was not registered anyway: linking it needs a `Cargo.toml` edit this ticket
//! must not make itself, and `json` 0.12 is already linked (`oracles = […, "dep:json", …]`,
//! `../../../../../../🔮️oracles/📦️packages/🦀️rust/Cargo.toml`) and already proven independent for
//! `stdio.json`'s own oracle (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/
//! 🪆️subsets/♾️any/🦀️oracle.rs`) — it appears nowhere in this repository's production
//! dependency graph. This subset's own production codec also uses `serde_json` (see above), which
//! rules `serde_json` out as an oracle for the same reason it was ruled out for `stdio.json`.
//!
//! `json` is domain-BLIND (no glTF schema awareness at all, unlike `gltf`), so every mutation's
//! actual semantics — index bounds, cycle rejection, duplicate-root rejection, alphaMode enum
//! validity, every top-level family's reference remapping — are reimplemented from scratch below, independently of
//! `../🧬️schema/🧬️mutations/*/🧬️operation/🦀️.rs`, operating on a hand-parsed GLB container
//! and a plain `json::JsonValue` document tree rather than this subset's own `GltfSnapshot`.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — `GLTF_MUTATION_LEAF_DESCRIPTORS`, the real vocabulary.

use semio_repo_test_host::Json;

//#region 🔖️Container
/// 🧊️ glTF binary container magic (`glTF`, little-endian).
#[cfg(feature = "oracles")]
const GLB_MAGIC: u32 = 0x4654_6C67;
/// 🧊️ `JSON` chunk type tag, little-endian.
#[cfg(feature = "oracles")]
const CHUNK_JSON: u32 = 0x4E4F_534A;
/// 🧊️ `BIN\0` chunk type tag, little-endian.
#[cfg(feature = "oracles")]
const CHUNK_BIN: u32 = 0x004E_4942;

/// 🔢️ Little-endian `u32` at `offset`, independent of any shared byte-reading helper.
#[cfg(feature = "oracles")]
fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    bytes.get(offset..offset + 4).map(|slice| u32::from_le_bytes(slice.try_into().expect("4-byte slice"))).ok_or_else(|| "unexpected end of GLB header".to_string())
}

/// 📥️ Independent GLB/`.gltf` read: 12-byte header plus `JSON`/`BIN\0` chunk iteration for a real
/// binary container, or a bare UTF-8 JSON parse for plain-text `.gltf` input — its own loop, its own
/// chunk bounds checks, never a call into `decode_glb`/`parse_gltf_document`.
#[cfg(feature = "oracles")]
fn read_glb(input: &[u8]) -> Result<(json::JsonValue, Option<Vec<u8>>), String> {
    if input.len() < 12 || read_u32(input, 0)? != GLB_MAGIC {
        let text = std::str::from_utf8(input).map_err(|error| format!("independent reader: input is neither a GLB container nor UTF-8 JSON text: {error}"))?;
        let doc = json::parse(text).map_err(|error| format!("independent reader could not parse glTF JSON: {error}"))?;
        return Ok((doc, None));
    }
    let total_length = read_u32(input, 8)? as usize;
    if total_length > input.len() {
        return Err("GLB header length exceeds the actual byte count".to_string());
    }
    let mut offset = 12usize;
    let mut json_chunk: Option<json::JsonValue> = None;
    let mut bin_chunk: Option<Vec<u8>> = None;
    while offset + 8 <= total_length {
        let chunk_length = read_u32(input, offset)? as usize;
        let chunk_type = read_u32(input, offset + 4)?;
        let data_start = offset + 8;
        let data_end = data_start + chunk_length;
        if data_end > total_length {
            return Err("GLB chunk length exceeds the declared container length".to_string());
        }
        let data = &input[data_start..data_end];
        if chunk_type == CHUNK_JSON {
            let text = std::str::from_utf8(data).map_err(|error| format!("GLB JSON chunk is not UTF-8: {error}"))?;
            json_chunk = Some(json::parse(text.trim_end_matches(' ')).map_err(|error| format!("independent reader could not parse the GLB JSON chunk: {error}"))?);
        } else if chunk_type == CHUNK_BIN {
            bin_chunk = Some(data.to_vec());
        }
        offset = data_end;
    }
    let doc = json_chunk.ok_or_else(|| "GLB container carries no JSON chunk".to_string())?;
    Ok((doc, bin_chunk))
}

/// 📤️ Independent GLB write: `JSON` chunk space-padded to a 4-byte boundary (§the format's own
/// padding rule), `BIN\0` chunk copied byte-for-byte from whatever `read_glb` extracted and
/// zero-padded the same way — its own header/length arithmetic, never a call into `encode_glb`.
#[cfg(feature = "oracles")]
fn write_glb(doc: &json::JsonValue, bin: Option<&[u8]>) -> Vec<u8> {
    let mut json_text = doc.dump().into_bytes();
    while json_text.len() % 4 != 0 {
        json_text.push(b' ');
    }
    let bin_padded_len = bin.map(|data| data.len() + (4 - data.len() % 4) % 4);
    let total_length = 12 + 8 + json_text.len() + bin.map_or(0, |_| 8 + bin_padded_len.expect("bin present"));
    let mut out = Vec::with_capacity(total_length);
    out.extend_from_slice(&GLB_MAGIC.to_le_bytes());
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(total_length as u32).to_le_bytes());
    out.extend_from_slice(&(json_text.len() as u32).to_le_bytes());
    out.extend_from_slice(&CHUNK_JSON.to_le_bytes());
    out.extend_from_slice(&json_text);
    if let Some(data) = bin {
        let padded_len = bin_padded_len.expect("bin present");
        out.extend_from_slice(&(padded_len as u32).to_le_bytes());
        out.extend_from_slice(&CHUNK_BIN.to_le_bytes());
        out.extend_from_slice(data);
        out.resize(out.len() + (padded_len - data.len()), 0);
    }
    out
}
//#endregion 🔖️Container

//#region 🔖️Tree
/// 🔎️ Object member lookup, `None` for a non-object or an absent key.
#[cfg(feature = "oracles")]
fn obj_get<'a>(value: &'a json::JsonValue, key: &str) -> Option<&'a json::JsonValue> {
    match value {
        json::JsonValue::Object(object) => object.get(key),
        _ => None,
    }
}

/// 🔧️ Object member upsert; a no-op on a non-object value.
#[cfg(feature = "oracles")]
fn obj_set(value: &mut json::JsonValue, key: &str, item: json::JsonValue) {
    if let json::JsonValue::Object(object) = value {
        object.insert(key, item);
    }
}

/// 🔎️ A cloned array member, or an empty `Vec` for a non-array or an absent key.
#[cfg(feature = "oracles")]
fn arr(value: &json::JsonValue, key: &str) -> Vec<json::JsonValue> {
    match obj_get(value, key) {
        Some(json::JsonValue::Array(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// 🔧️ The array member's own mutable `Vec`, or `None` when `key` is not an array on this object.
#[cfg(feature = "oracles")]
fn arr_mut<'a>(value: &'a mut json::JsonValue, key: &str) -> Option<&'a mut Vec<json::JsonValue>> {
    match value {
        json::JsonValue::Object(object) => match object.get_mut(key) {
            Some(json::JsonValue::Array(items)) => Some(items),
            _ => None,
        },
        _ => None,
    }
}

/// 🔧️ The array member's mutable `Vec`, inserting an empty array first if `key` is absent or holds
/// something else — the same "created on first write" shape `skip_serializing_if = "Vec::is_empty"`
/// gives this subset's own `children`/`nodes` fields.
#[cfg(feature = "oracles")]
fn ensure_array<'a>(value: &'a mut json::JsonValue, key: &str) -> &'a mut Vec<json::JsonValue> {
    if !matches!(obj_get(value, key), Some(json::JsonValue::Array(_))) {
        obj_set(value, key, json::JsonValue::Array(Vec::new()));
    }
    arr_mut(value, key).expect("array member just ensured")
}

/// 🔎️ `document/nodes`, `document/scenes` and `document/materials` lengths, addressed by name so
/// every kind's bounds check reads identically to its production counterpart's `checked_index`.
#[cfg(feature = "oracles")]
fn top_level_len(doc: &json::JsonValue, key: &str) -> usize {
    arr(doc, key).len()
}

/// 🔎️ `document/nodes/{index}/children`, resolved indices only (a non-numeric entry is skipped
/// rather than treated as a fatal error, matching how index resolution reads a real document).
#[cfg(feature = "oracles")]
fn node_children(doc: &json::JsonValue, index: usize) -> Vec<usize> {
    arr(doc, "nodes").get(index).map(|node| arr(node, "children").iter().filter_map(json::JsonValue::as_usize).collect()).unwrap_or_default()
}

/// 🔎️ `document/scenes/{index}/nodes`, resolved indices only.
#[cfg(feature = "oracles")]
fn scene_nodes(doc: &json::JsonValue, index: usize) -> Vec<usize> {
    arr(doc, "scenes").get(index).map(|scene| arr(scene, "nodes").iter().filter_map(json::JsonValue::as_usize).collect()).unwrap_or_default()
}

/// 🔎️ `document/scene`, the default scene index, `None` when the document declares none.
#[cfg(feature = "oracles")]
fn default_scene_index(doc: &json::JsonValue) -> Option<usize> {
    obj_get(doc, "scene").and_then(json::JsonValue::as_usize)
}
//#endregion 🔖️Tree

//#region 🔖️Params
#[cfg(feature = "oracles")]
fn usize_param(params: &Json, key: &str) -> Result<usize, String> {
    match params.get(key) {
        Some(Json::Number(number)) => Ok(*number as usize),
        _ => Err(format!("missing or non-numeric `{key}`")),
    }
}

#[cfg(feature = "oracles")]
fn str_param(params: &Json, key: &str) -> Result<String, String> {
    match params.get(key) {
        Some(Json::String(value)) => Ok(value.clone()),
        _ => Err(format!("missing or non-string `{key}`")),
    }
}

#[cfg(feature = "oracles")]
fn bool_param(params: &Json, key: &str) -> Result<bool, String> {
    match params.get(key) {
        Some(Json::Bool(value)) => Ok(*value),
        _ => Err(format!("missing or non-boolean `{key}`")),
    }
}

/// 🔎️ `usize` elements of an array param — `reorder-cameras`'s own `order` shape.
#[cfg(feature = "oracles")]
fn usize_array_param(params: &Json, key: &str) -> Result<Vec<usize>, String> {
    match params.get(key) {
        Some(Json::Array(items)) => items
            .iter()
            .map(|item| match item {
                Json::Number(number) => Ok(*number as usize),
                _ => Err(format!("`{key}` must hold only numbers")),
            })
            .collect(),
        _ => Err(format!("missing or non-array `{key}`")),
    }
}

/// 🔎️ An object param, handed back as this host's own `Json`, for `create-camera`'s `projection`
/// (a small tagged-union object this reader never needs domain knowledge of — see
/// [`from_host_json`]).
#[cfg(feature = "oracles")]
fn object_param<'a>(params: &'a Json, key: &str) -> Result<&'a Json, String> {
    match params.get(key) {
        Some(value @ Json::Object(_)) => Ok(value),
        _ => Err(format!("missing or non-object `{key}`")),
    }
}

/// 🔎️ An optional string param (shard G2) — `None` for an absent or non-string key, matching an
/// `Option<String>` payload field like `generator`/`copyright`/`minVersion`.
#[cfg(feature = "oracles")]
fn optional_str_param(params: &Json, key: &str) -> Option<String> {
    match params.get(key) {
        Some(Json::String(value)) => Some(value.clone()),
        _ => None,
    }
}

/// 🔎️ An optional object param — `None` for an absent, `null`, or non-object key, matching an
/// `Option<GltfJson>` payload field like `change-{asset,document}-{extension,extra}-data`'s `data`.
#[cfg(feature = "oracles")]
fn optional_object_param<'a>(params: &'a Json, key: &str) -> Option<&'a Json> {
    match params.get(key) {
        Some(value @ Json::Object(_)) => Some(value),
        _ => None,
    }
}

/// 🔎️ String elements of an array param — `reorder-{required,used}-extensions`'s own `order`.
#[cfg(feature = "oracles")]
fn string_array_param(params: &Json, key: &str) -> Result<Vec<String>, String> {
    match params.get(key) {
        Some(Json::Array(items)) => items
            .iter()
            .map(|item| match item {
                Json::String(value) => Ok(value.clone()),
                _ => Err(format!("`{key}` must hold only strings")),
            })
            .collect(),
        _ => Err(format!("missing or non-array `{key}`")),
    }
}
//#endregion 🔖️Params

//#region 🔖️JsonBridge
/// 🌉️ This host's own `Json` → the independent `json`-crate tree — domain-blind, structural only,
/// used solely to carry `create-camera`'s `projection` param into the parsed document without this
/// reader ever having to know the camera schema's own field names.
#[cfg(feature = "oracles")]
fn from_host_json(value: &Json) -> json::JsonValue {
    match value {
        Json::Null => json::JsonValue::Null,
        Json::Bool(flag) => json::JsonValue::Boolean(*flag),
        Json::Number(number) => json::JsonValue::from(*number),
        Json::String(text) => json::JsonValue::String(text.clone()),
        Json::Array(items) => json::JsonValue::Array(items.iter().map(from_host_json).collect()),
        Json::Object(entries) => {
            let mut object = json::object::Object::new();
            for (key, item) in entries {
                object.insert(key, from_host_json(item));
            }
            json::JsonValue::Object(object)
        }
    }
}

/// 🌉️ The independent `json`-crate tree → this host's own `Json` — the reverse of
/// [`from_host_json`], used only to project `document/cameras` (a small self-contained object with
/// no cross-references of its own, unlike `nodes`/`scenes`) onto the comparison shape without
/// hand-listing every one of `GltfPerspective`/`GltfOrthographic`'s own field names here.
#[cfg(feature = "oracles")]
fn to_host_json(value: &json::JsonValue) -> Json {
    match value {
        json::JsonValue::Null => Json::Null,
        json::JsonValue::Boolean(flag) => Json::Bool(*flag),
        json::JsonValue::Number(_) => Json::Number(value.as_f64().unwrap_or(0.0)),
        json::JsonValue::Short(_) | json::JsonValue::String(_) => Json::String(value.as_str().unwrap_or("").to_string()),
        json::JsonValue::Array(items) => Json::Array(items.iter().map(to_host_json).collect()),
        json::JsonValue::Object(object) => Json::Object(object.iter().map(|(key, item)| (key.to_string(), to_host_json(item))).collect()),
    }
}
//#endregion 🔖️JsonBridge

//#region 🔖️Kinds
/// 🦠️ `bind-node-child` — validated exactly as `../🧬️schema/🧬️mutations/bind-node-child/
/// 🧬️operation/🦀️.rs` documents (index bounds, no self-parenting, no duplicate link, no
/// cycle through the independently-walked `children` graph), reimplemented against the parsed tree.
#[cfg(feature = "oracles")]
fn bind_node_child(doc: &mut json::JsonValue, parent: usize, child: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "nodes");
    if parent >= total {
        return Err(format!("bind-node-child: parent {parent} out of range"));
    }
    if child >= total {
        return Err(format!("bind-node-child: child {child} out of range"));
    }
    if parent == child {
        return Err("bind-node-child: a node cannot parent itself".to_string());
    }
    if node_children(doc, parent).contains(&child) {
        return Err("bind-node-child: duplicate child link".to_string());
    }
    let mut pending = vec![child];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(current) = pending.pop() {
        if current == parent {
            return Err("bind-node-child: relationship closes a cycle".to_string());
        }
        if seen.insert(current) {
            pending.extend(node_children(doc, current));
        }
    }
    if position > node_children(doc, parent).len() {
        return Err("bind-node-child: position out of range".to_string());
    }
    let node = arr_mut(doc, "nodes").and_then(|nodes| nodes.get_mut(parent)).ok_or("bind-node-child: parent node missing")?;
    ensure_array(node, "children").insert(position, json::JsonValue::from(child));
    Ok(())
}

/// 🦠️ `unbind-node-child` — removes a real, currently-present link.
#[cfg(feature = "oracles")]
fn unbind_node_child(doc: &mut json::JsonValue, parent: usize, child: usize) -> Result<(), String> {
    let total = top_level_len(doc, "nodes");
    if parent >= total {
        return Err(format!("unbind-node-child: parent {parent} out of range"));
    }
    if child >= total {
        return Err(format!("unbind-node-child: child {child} out of range"));
    }
    let node = arr_mut(doc, "nodes").and_then(|nodes| nodes.get_mut(parent)).ok_or("unbind-node-child: parent node missing")?;
    let children = ensure_array(node, "children");
    let position = children.iter().position(|value| value.as_usize() == Some(child)).ok_or("unbind-node-child: child is not linked to parent")?;
    children.remove(position);
    Ok(())
}

/// 🦠️ `bind-scene-root-node` — an existing node becomes an additional root of a scene it is not
/// already a root of. Mirrors production in NOT checking whether the node is already someone's
/// child elsewhere: `../🧬️schema/🧬️mutations/bind-scene-root-node/🧬️operation/🦀️.rs`'s own
/// `validate` only rejects an out-of-range index or an already-present root.
#[cfg(feature = "oracles")]
fn bind_scene_root_node(doc: &mut json::JsonValue, scene: usize, node: usize, position: usize) -> Result<(), String> {
    if scene >= top_level_len(doc, "scenes") {
        return Err(format!("bind-scene-root-node: scene {scene} out of range"));
    }
    if node >= top_level_len(doc, "nodes") {
        return Err(format!("bind-scene-root-node: node {node} out of range"));
    }
    let roots = scene_nodes(doc, scene);
    if roots.contains(&node) {
        return Err("bind-scene-root-node: node is already a scene root".to_string());
    }
    if position > roots.len() {
        return Err("bind-scene-root-node: position out of range".to_string());
    }
    let scene_value = arr_mut(doc, "scenes").and_then(|scenes| scenes.get_mut(scene)).ok_or("bind-scene-root-node: scene missing")?;
    ensure_array(scene_value, "nodes").insert(position, json::JsonValue::from(node));
    Ok(())
}

/// 🦠️ `unbind-scene-root-node` — removes a real, currently-present scene root.
#[cfg(feature = "oracles")]
fn unbind_scene_root_node(doc: &mut json::JsonValue, scene: usize, node: usize) -> Result<(), String> {
    if scene >= top_level_len(doc, "scenes") {
        return Err(format!("unbind-scene-root-node: scene {scene} out of range"));
    }
    if node >= top_level_len(doc, "nodes") {
        return Err(format!("unbind-scene-root-node: node {node} out of range"));
    }
    let scene_value = arr_mut(doc, "scenes").and_then(|scenes| scenes.get_mut(scene)).ok_or("unbind-scene-root-node: scene missing")?;
    let nodes = ensure_array(scene_value, "nodes");
    let position = nodes.iter().position(|value| value.as_usize() == Some(node)).ok_or("unbind-scene-root-node: node is not a root of this scene")?;
    nodes.remove(position);
    Ok(())
}

/// 🔎️ `document/materials/{index}/alphaMode`, defaulted to `OPAQUE` exactly as
/// `GltfAlphaMode::default()` and its `skip_serializing_if = "is_opaque"` field attribute do.
#[cfg(feature = "oracles")]
fn material_alpha_mode(doc: &json::JsonValue, index: usize) -> String {
    arr(doc, "materials").get(index).and_then(|material| obj_get(material, "alphaMode")).and_then(json::JsonValue::as_str).unwrap_or("OPAQUE").to_string()
}

/// 🔎️ `document/materials/{index}/doubleSided`, defaulted to `false` exactly as
/// `GltfMaterial::default()` and its `skip_serializing_if = "is_false"` field attribute do.
#[cfg(feature = "oracles")]
fn material_double_sided(doc: &json::JsonValue, index: usize) -> bool {
    arr(doc, "materials").get(index).and_then(|material| obj_get(material, "doubleSided")).and_then(json::JsonValue::as_bool).unwrap_or(false)
}

/// 🦠️ `change-material-alpha-mode` — rejects an out-of-range material, an invalid enum spelling and
/// a no-observable-change identity, exactly as `../🧬️schema/🧬️mutations/change-material-alpha-mode/
/// 🧬️operation/🦀️.rs` does.
#[cfg(feature = "oracles")]
fn change_material_alpha_mode(doc: &mut json::JsonValue, material: usize, alpha_mode: &str) -> Result<(), String> {
    if material >= top_level_len(doc, "materials") {
        return Err(format!("change-material-alpha-mode: material {material} out of range"));
    }
    if !matches!(alpha_mode, "OPAQUE" | "MASK" | "BLEND") {
        return Err(format!("change-material-alpha-mode: {alpha_mode:?} is not a valid alphaMode"));
    }
    if material_alpha_mode(doc, material) == alpha_mode {
        return Err("change-material-alpha-mode: alphaMode already has that value".to_string());
    }
    let entry = arr_mut(doc, "materials").and_then(|materials| materials.get_mut(material)).ok_or("change-material-alpha-mode: material missing")?;
    obj_set(entry, "alphaMode", json::JsonValue::from(alpha_mode));
    Ok(())
}

/// 🦠️ `change-material-double-sided` — rejects an out-of-range material and a no-observable-change
/// identity, exactly as `../🧬️schema/🧬️mutations/change-material-double-sided/🧬️operation/
/// 🦀️.rs` does.
#[cfg(feature = "oracles")]
fn change_material_double_sided(doc: &mut json::JsonValue, material: usize, double_sided: bool) -> Result<(), String> {
    if material >= top_level_len(doc, "materials") {
        return Err(format!("change-material-double-sided: material {material} out of range"));
    }
    if material_double_sided(doc, material) == double_sided {
        return Err("change-material-double-sided: doubleSided already has that value".to_string());
    }
    let entry = arr_mut(doc, "materials").and_then(|materials| materials.get_mut(material)).ok_or("change-material-double-sided: material missing")?;
    obj_set(entry, "doubleSided", json::JsonValue::from(double_sided));
    Ok(())
}

/// 🦠️ `create-scene` — inserts one canonical empty scene (`{}`, matching `GltfScene::default()`)
/// and, if `document/scene` names a scene at or after `position`, bumps it by one so it still names
/// the same scene — the same `default_after` remap `../🧬️schema/🧬️mutations/create-scene/🔒️private/
/// 🦀️.rs` performs, reimplemented independently against the parsed tree.
#[cfg(feature = "oracles")]
fn create_scene(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "scenes") {
        return Err("create-scene: position out of range".to_string());
    }
    let after = default_scene_index(doc).map(|scene| if scene >= position { scene + 1 } else { scene });
    let scenes = arr_mut(doc, "scenes").ok_or("document has no scenes array")?;
    scenes.insert(position, json::JsonValue::Object(json::object::Object::new()));
    if let Some(scene) = after {
        obj_set(doc, "scene", json::JsonValue::from(scene));
    }
    Ok(())
}

/// ↩️ `create-scene`'s own inverse — never a catalog kind of its own, exactly as production dispatches
/// it through the SAME `create-scene` descriptor's `phase: Inverse` rather than through a separate
/// `delete-scene` leaf (see `../🧬️schema/🧬️mutations/create-scene/↩️inverse/🦀️.rs`). Removes
/// the scene `create-scene` inserted at `position` and inverts the exact `default_after` remap: the
/// current `document/scene` (if any) is `> position` only when it was bumped, so subtracting one
/// recovers the pre-mutation value; `<= position` (impossible to equal `position` itself, since that
/// slot now holds the freshly created scene) means it was never touched.
#[cfg(feature = "oracles")]
pub fn undo_create_scene(input: &[u8], position: usize) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(input)?;
    if position >= top_level_len(&doc, "scenes") {
        return Err("undo-create-scene: position out of range".to_string());
    }
    let restored = default_scene_index(&doc).map(|scene| if scene > position { scene - 1 } else { scene });
    let scenes = arr_mut(&mut doc, "scenes").ok_or("document has no scenes array")?;
    scenes.remove(position);
    match restored {
        Some(scene) => obj_set(&mut doc, "scene", json::JsonValue::from(scene)),
        None => {
            if let json::JsonValue::Object(object) = &mut doc {
                object.remove("scene");
            }
        }
    }
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn undo_create_scene(_input: &[u8], _position: usize) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ `delete-skin`'s own inverse — deliberately NOT dispatched through `create-skin` (whose own
/// payload, `../🧬️schema/🧬️mutations/🦴️skin/🌱️create/🦀️.rs`'s `GltfCreateSkinPayload { position }`,
/// carries no field content at all, so it can only ever recreate an EMPTY skin — real production
/// dispatches this inverse through `DeleteSkinMutation`'s own diff-based `Restore` variant, which
/// this domain-blind reader has no typed access to). Reads `document/skins` and every
/// `nodes/{i}/skin` reference straight off the ORIGINAL (pre-mutation) document and splices them
/// back — the same "restore the exact removed content, not a same-shaped substitute" rule
/// `undo_create_scene` above follows for its own kind, applied to the one other kind in this
/// catalog whose forward mutation is not exactly invertible through a sibling kind's own payload.
#[cfg(feature = "oracles")]
pub fn undo_delete_skin(mutated: &[u8], original: &[u8]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, _) = read_glb(original)?;
    obj_set(&mut doc, "skins", json::JsonValue::Array(arr(&source, "skins")));
    let source_nodes = arr(&source, "nodes");
    if let Some(nodes) = arr_mut(&mut doc, "nodes") {
        for (index, node) in nodes.iter_mut().enumerate() {
            match source_nodes.get(index).and_then(|node| obj_get(node, "skin")).and_then(json::JsonValue::as_usize) {
                Some(value) => obj_set(node, "skin", json::JsonValue::from(value)),
                None => {
                    if let json::JsonValue::Object(object) = node {
                        object.remove("skin");
                    }
                }
            }
        }
    }
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn undo_delete_skin(_mutated: &[u8], _original: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ `delete-animation`'s own inverse — the identical reasoning as [`undo_delete_skin`] above:
/// `create-animation`'s own payload (`GltfCreateAnimationPayload { position }`) carries no field
/// content, so it can only ever recreate an EMPTY animation. Restores `document/animations`
/// straight off the ORIGINAL (pre-mutation) document; no node reference to splice back (see
/// `create_animation`'s own doc comment — `Animations` is the one family `repair` never touches).
#[cfg(feature = "oracles")]
pub fn undo_delete_animation(mutated: &[u8], original: &[u8]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, _) = read_glb(original)?;
    obj_set(&mut doc, "animations", json::JsonValue::Array(arr(&source, "animations")));
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn undo_delete_animation(_mutated: &[u8], _original: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔀️ The four remap shapes a single-parent top-level collection's own `repair`/`family_ops!`
/// machinery (`../../🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs`) applies to every
/// scalar `nodes/{i}/<field>` reference into it (`camera` for `document/cameras`, `skin` for
/// `document/skins` — the only two top-level families a bare node scalar field points at),
/// reimplemented independently here — this reader never calls into that production module.
#[cfg(feature = "oracles")]
enum IndexChange<'a> {
    Insert(usize),
    Delete(usize),
    Move(usize, usize),
    Reorder(&'a [usize]),
}

/// 🔀️ One reference's new value under `change`, `None` meaning the reference is dropped — the
/// exact four-branch arithmetic `top_level_collections::remap` documents for a `Change`, re-derived
/// from the format's own rule (an index list shrinks/grows/moves/permutes, every reference to it
/// must track the same motion) rather than copied from that private helper.
#[cfg(feature = "oracles")]
fn remap_index(change: &IndexChange, value: usize) -> Option<usize> {
    match change {
        IndexChange::Insert(at) => Some(if value >= *at { value + 1 } else { value }),
        IndexChange::Delete(at) => (value != *at).then_some(if value > *at { value - 1 } else { value }),
        IndexChange::Move(from, to) => Some(if value == *from {
            *to
        } else if from < to && value > *from && value <= *to {
            value - 1
        } else if to < from && value >= *to && value < *from {
            value + 1
        } else {
            value
        }),
        IndexChange::Reorder(order) => order.iter().position(|candidate| *candidate == value),
    }
}

/// 🔧️ Applies one [`IndexChange`] to every `nodes/{i}/<field>` reference in place — `field` is
/// `"camera"` (§5.7.2, `document/cameras`) or `"skin"` (§5.7.3, `document/skins`), the only two
/// top-level families a bare node scalar field points at.
#[cfg(feature = "oracles")]
fn apply_node_ref_change(doc: &mut json::JsonValue, field: &str, change: &IndexChange) {
    if let Some(nodes) = arr_mut(doc, "nodes") {
        for node in nodes.iter_mut() {
            let current = obj_get(node, field).and_then(json::JsonValue::as_usize);
            if let Some(value) = current {
                match remap_index(change, value) {
                    Some(mapped) => obj_set(node, field, json::JsonValue::from(mapped)),
                    None => {
                        if let json::JsonValue::Object(object) = node {
                            object.remove(field);
                        }
                    }
                }
            }
        }
    }
}

/// 🦠️ `create-camera` — inserts one camera object at `position` into `document/cameras`; rejects an
/// out-of-range position, exactly as `../🧬️schema/🧬️mutations/🎥️camera/🌱️create/🦀️.rs`'s own
/// `validate` does. `projection`'s own field names (`type`/`perspective`/`orthographic`/…) are never
/// inspected here — [`from_host_json`] carries the param through structurally.
#[cfg(feature = "oracles")]
fn create_camera(doc: &mut json::JsonValue, position: usize, projection: &Json) -> Result<(), String> {
    if position > top_level_len(doc, "cameras") {
        return Err(format!("create-camera: position {position} out of range"));
    }
    apply_node_ref_change(doc, "camera", &IndexChange::Insert(position));
    ensure_array(doc, "cameras").insert(position, from_host_json(projection));
    Ok(())
}

/// 🦠️ `delete-camera` — removes a real, currently-present camera and clears every `node.camera`
/// reference that pointed at it, exactly as `../🧬️schema/🧬️mutations/🎥️camera/🗑️delete/🦀️.rs`'s
/// own `validate`/`cameras_op` do.
#[cfg(feature = "oracles")]
fn delete_camera(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "cameras") {
        return Err(format!("delete-camera: index {index} out of range"));
    }
    apply_node_ref_change(doc, "camera", &IndexChange::Delete(index));
    ensure_array(doc, "cameras").remove(index);
    Ok(())
}

/// 🦠️ `move-camera` — relocates a real camera to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🎥️camera/🚚️move/🦀️.rs`'s own `validate` does (both indices in range,
/// source and destination distinct — a no-observable-change rejection otherwise).
#[cfg(feature = "oracles")]
fn move_camera(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "cameras");
    if index >= total || position >= total {
        return Err(format!("move-camera: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err("move-camera: position already equals index".to_string());
    }
    apply_node_ref_change(doc, "camera", &IndexChange::Move(index, position));
    let cameras = ensure_array(doc, "cameras");
    let value = cameras.remove(index);
    cameras.insert(position, value);
    Ok(())
}

/// 🦠️ `reorder-cameras` — `order` must be a genuine permutation of every existing index and must
/// actually move at least one, exactly as `../🧬️schema/🧬️mutations/🎥️camera/🔀️reorder/🦀️.rs`'s
/// own `validate` does.
#[cfg(feature = "oracles")]
fn reorder_cameras(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
    let total = top_level_len(doc, "cameras");
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err("reorder-cameras: order must contain every index exactly once".to_string());
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err("reorder-cameras: order already matches".to_string());
    }
    apply_node_ref_change(doc, "camera", &IndexChange::Reorder(order));
    let cameras = ensure_array(doc, "cameras");
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| cameras[*index].clone()).collect();
    *cameras = reordered;
    Ok(())
}

/// 🦠️ `create-skin` — inserts one canonical empty skin (`{}`, matching `GltfSkin::default()`) at
/// `position` into `document/skins`; rejects an out-of-range position, exactly as
/// `../🧬️schema/🧬️mutations/🦴️skin/🌱️create/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn create_skin(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "skins") {
        return Err(format!("create-skin: position {position} out of range"));
    }
    apply_node_ref_change(doc, "skin", &IndexChange::Insert(position));
    ensure_array(doc, "skins").insert(position, json::JsonValue::Object(json::object::Object::new()));
    Ok(())
}

/// 🦠️ `delete-skin` — removes a real, currently-present skin and clears every `node.skin` reference
/// that pointed at it, exactly as `../🧬️schema/🧬️mutations/🦴️skin/🗑️delete/🦀️.rs`'s own
/// `validate`/`skins_op` do.
#[cfg(feature = "oracles")]
fn delete_skin(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "skins") {
        return Err(format!("delete-skin: index {index} out of range"));
    }
    apply_node_ref_change(doc, "skin", &IndexChange::Delete(index));
    ensure_array(doc, "skins").remove(index);
    Ok(())
}

/// 🦠️ `move-skin` — relocates a real skin to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🦴️skin/🚚️move/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn move_skin(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "skins");
    if index >= total || position >= total {
        return Err(format!("move-skin: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err("move-skin: position already equals index".to_string());
    }
    apply_node_ref_change(doc, "skin", &IndexChange::Move(index, position));
    let skins = ensure_array(doc, "skins");
    let value = skins.remove(index);
    skins.insert(position, value);
    Ok(())
}

/// 🦠️ `reorder-skins` — `order` must be a genuine permutation of every existing index and must
/// actually move at least one, exactly as `../🧬️schema/🧬️mutations/🦴️skin/🔀️reorder/🦀️.rs`'s own
/// `validate` does.
#[cfg(feature = "oracles")]
fn reorder_skins(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
    let total = top_level_len(doc, "skins");
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err("reorder-skins: order must contain every index exactly once".to_string());
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err("reorder-skins: order already matches".to_string());
    }
    apply_node_ref_change(doc, "skin", &IndexChange::Reorder(order));
    let skins = ensure_array(doc, "skins");
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| skins[*index].clone()).collect();
    *skins = reordered;
    Ok(())
}

/// 🦠️ `create-animation` — inserts one canonical empty animation (`{}`, matching
/// `GltfAnimation::default()`) at `position` into `document/animations`; rejects an out-of-range
/// position, exactly as `../🧬️schema/🧬️mutations/🎞️animation/🌱️create/🦀️.rs`'s own `validate`
/// does. `document/animations` is the ONE top-level family `repair`'s own match has an EMPTY arm
/// for (`GltfTopLevelFamily::Animations => {}`) — no node scalar field, nor any other family, ever
/// points at an animation by index (only the reverse: `animations[i].channels[j].target.node`
/// points AT a node) — so no `apply_node_ref_change` call belongs here at all.
#[cfg(feature = "oracles")]
fn create_animation(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "animations") {
        return Err(format!("create-animation: position {position} out of range"));
    }
    ensure_array(doc, "animations").insert(position, json::JsonValue::Object(json::object::Object::new()));
    Ok(())
}

/// 🦠️ `delete-animation` — removes a real, currently-present animation, exactly as
/// `../🧬️schema/🧬️mutations/🎞️animation/🗑️delete/🦀️.rs`'s own `validate`/`animations_op` do.
#[cfg(feature = "oracles")]
fn delete_animation(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "animations") {
        return Err(format!("delete-animation: index {index} out of range"));
    }
    ensure_array(doc, "animations").remove(index);
    Ok(())
}

/// 🦠️ `move-animation` — relocates a real animation to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🎞️animation/🚚️move/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn move_animation(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "animations");
    if index >= total || position >= total {
        return Err(format!("move-animation: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err("move-animation: position already equals index".to_string());
    }
    let animations = ensure_array(doc, "animations");
    let value = animations.remove(index);
    animations.insert(position, value);
    Ok(())
}

/// 🦠️ `reorder-animations` — `order` must be a genuine permutation of every existing index and must
/// actually move at least one, exactly as `../🧬️schema/🧬️mutations/🎞️animation/🔀️reorder/🦀️.rs`'s
/// own `validate` does.
#[cfg(feature = "oracles")]
fn reorder_animations(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
    let total = top_level_len(doc, "animations");
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err("reorder-animations: order must contain every index exactly once".to_string());
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err("reorder-animations: order already matches".to_string());
    }
    let animations = ensure_array(doc, "animations");
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| animations[*index].clone()).collect();
    *animations = reordered;
    Ok(())
}

/// 🧩️ `add-required-extension`/`add-used-extension` (shard G2, this ticket) — inserts a real
/// extension NAME at `position` into `document/extensionsRequired`(/`Used`), a plain string array
/// with no cross-reference from anywhere else in the document (unlike `cameras`/`skins`, no
/// `apply_node_ref_change` call belongs here), exactly as
/// `../🧬️schema/🧬️mutations/✅️required-extension/➕️add/🦀️.rs`'s(/`📣️used-extension/➕️add`'s) own
/// `validate` checks the position bound. `add-required-extension`'s own production `validate` ALSO
/// requires the same name to already be present in `extensionsUsed` — this independent reader does
/// not re-derive that second-array cross-check, since every committed fixture this reader is run
/// against already satisfies it (the same scope camera/skin's own oracle functions keep: index
/// bounds and permutation validity, not every domain invariant the production leaf enforces).
#[cfg(feature = "oracles")]
fn add_extension(doc: &mut json::JsonValue, key: &str, extension: &str, position: usize) -> Result<(), String> {
    let list = ensure_array(doc, key);
    if position > list.len() {
        return Err(format!("{key}: position {position} out of range"));
    }
    list.insert(position, json::JsonValue::String(extension.to_string()));
    Ok(())
}

/// 🧩️ `remove-required-extension`/`remove-used-extension` — removes a real, currently-declared
/// extension name, exactly as the sibling `✅️required-extension/➖️remove`/`📣️used-extension/➖️remove`
/// leaves' own `validate` does (the name must be present).
#[cfg(feature = "oracles")]
fn remove_extension(doc: &mut json::JsonValue, key: &str, extension: &str) -> Result<(), String> {
    let list = ensure_array(doc, key);
    let index = list.iter().position(|value| value.as_str() == Some(extension)).ok_or_else(|| format!("{key}: extension {extension:?} is not declared"))?;
    list.remove(index);
    Ok(())
}

/// 🧩️ `move-required-extension`/`move-used-extension` — relocates a real, currently-declared
/// extension name to a real, different position, exactly as the sibling
/// `✅️required-extension/🚚️move`/`📣️used-extension/🚚️move` leaves' own `validate` does.
#[cfg(feature = "oracles")]
fn move_extension(doc: &mut json::JsonValue, key: &str, extension: &str, position: usize) -> Result<(), String> {
    let list = ensure_array(doc, key);
    let index = list.iter().position(|value| value.as_str() == Some(extension)).ok_or_else(|| format!("{key}: extension {extension:?} is not declared"))?;
    if position >= list.len() {
        return Err(format!("{key}: position {position} out of range"));
    }
    if index == position {
        return Err(format!("{key}: position already equals index"));
    }
    let value = list.remove(index);
    list.insert(position, value);
    Ok(())
}

/// 🧩️ `reorder-required-extensions`/`reorder-used-extensions` — `order` must be a genuine
/// permutation of every currently-declared name and must actually move at least one, exactly as the
/// sibling `✅️required-extension/🔀️reorder`/`📣️used-extension/🔀️reorder` leaves' own `validate`
/// does.
#[cfg(feature = "oracles")]
fn reorder_extensions(doc: &mut json::JsonValue, key: &str, order: &[String]) -> Result<(), String> {
    let current: Vec<String> = arr(doc, key).iter().filter_map(json::JsonValue::as_str).map(str::to_string).collect();
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != current.len() || order.iter().any(|value| !current.contains(value) || !seen.insert(value.clone())) {
        return Err(format!("{key}: order must contain every declaration exactly once"));
    }
    if order == current.as_slice() {
        return Err(format!("{key}: order already matches"));
    }
    let list = ensure_array(doc, key);
    *list = order.iter().map(|value| json::JsonValue::String(value.clone())).collect();
    Ok(())
}

/// 🧩️ `document/asset`, read as a cloned object — mutated off-tree and written back with
/// [`obj_set`] rather than held as a live `&mut` borrow, so every asset-block function below shares
/// one simple shape instead of juggling nested-borrow lifetimes against `doc`.
#[cfg(feature = "oracles")]
fn asset_object(doc: &json::JsonValue) -> json::JsonValue {
    match obj_get(doc, "asset") {
        Some(value @ json::JsonValue::Object(_)) => value.clone(),
        _ => json::JsonValue::Object(json::object::Object::new()),
    }
}

/// 🧩️ Rebuilds `container` without `key` — this reader's own removal primitive, built from
/// `.iter()`/`Object::insert` alone (both already used by [`from_host_json`]/[`to_host_json`])
/// rather than assuming the `json` crate's `Object` exposes a `remove` method this file has not
/// otherwise needed. Used wherever a payload's `Option<…>` field goes from `Some` to `None`, which
/// this subset's own `skip_serializing_if = "Option::is_none"` encodes as the KEY BEING ABSENT, not
/// present with a `null` value (confirmed by grepping a committed fixture for the literal key
/// substring before writing this).
#[cfg(feature = "oracles")]
fn without_key(container: &json::JsonValue, key: &str) -> json::JsonValue {
    match container {
        json::JsonValue::Object(object) => {
            let mut rebuilt = json::object::Object::new();
            for (entry_key, entry_value) in object.iter() {
                if entry_key != key {
                    rebuilt.insert(entry_key, entry_value.clone());
                }
            }
            json::JsonValue::Object(rebuilt)
        }
        other => other.clone(),
    }
}

/// 🧩️ Sets or clears an optional string member in role: `Some` upserts, `None` removes the key
/// entirely via [`without_key`] — never writes a literal `null`.
#[cfg(feature = "oracles")]
fn set_optional_string(container: &mut json::JsonValue, key: &str, value: Option<&str>) {
    match value {
        Some(text) => obj_set(container, key, json::JsonValue::String(text.to_string())),
        None => *container = without_key(container, key),
    }
}

/// 🧩️ Sets or clears an optional object member in role — the same law as
/// [`set_optional_string`], for `change-{asset,document}-{extension,extra}-data`'s own
/// `Option<GltfJson>` payload shape.
#[cfg(feature = "oracles")]
fn set_optional_json(container: &mut json::JsonValue, key: &str, value: Option<json::JsonValue>) {
    match value {
        Some(item) => obj_set(container, key, item),
        None => *container = without_key(container, key),
    }
}

/// 🧩️ `change-asset-descriptive-metadata` — the three plain scalar setters
/// `../🧬️schema/🧬️mutations/🪪️asset/📝️change-description/🦀️.rs`'s own `apply` writes
/// together (`generator`/`copyright`/`minVersion`), rejecting a call that changes none of them
/// exactly as that leaf's own `validate` does.
#[cfg(feature = "oracles")]
fn change_asset_descriptive_metadata(doc: &mut json::JsonValue, generator: Option<&str>, copyright: Option<&str>, min_version: Option<&str>) -> Result<(), String> {
    let mut asset = asset_object(doc);
    let unchanged = obj_get(&asset, "generator").and_then(json::JsonValue::as_str) == generator && obj_get(&asset, "copyright").and_then(json::JsonValue::as_str) == copyright && obj_get(&asset, "minVersion").and_then(json::JsonValue::as_str) == min_version;
    if unchanged {
        return Err("change-asset-descriptive-metadata: descriptive metadata already has these values".to_string());
    }
    set_optional_string(&mut asset, "generator", generator);
    set_optional_string(&mut asset, "copyright", copyright);
    set_optional_string(&mut asset, "minVersion", min_version);
    obj_set(doc, "asset", asset);
    Ok(())
}

/// 🧩️ `change-asset-version` — `document/asset/version`, the one REQUIRED scalar of the asset
/// block, exactly as `../🧬️schema/🧬️mutations/🪪️asset/🔖️version/🦀️.rs`'s own `validate`
/// does (non-empty, observably different).
#[cfg(feature = "oracles")]
fn change_asset_version(doc: &mut json::JsonValue, version: &str) -> Result<(), String> {
    let mut asset = asset_object(doc);
    if version.trim().is_empty() {
        return Err("change-asset-version: version must be non-empty".to_string());
    }
    if obj_get(&asset, "version").and_then(json::JsonValue::as_str) == Some(version) {
        return Err("change-asset-version: version already has this value".to_string());
    }
    obj_set(&mut asset, "version", json::JsonValue::String(version.to_string()));
    obj_set(doc, "asset", asset);
    Ok(())
}

/// 🧩️ `change-asset-extension-data` — `document/asset/extensions`, an opaque tagged-object bag
/// carried structurally via [`from_host_json`]/[`to_host_json`] exactly as `create-camera`'s own
/// `projection` param already is, exactly as
/// `../🧬️schema/🧬️mutations/🪪️asset/🧩️change-extensions/🦀️.rs`'s own `validate` does
/// (observably different).
#[cfg(feature = "oracles")]
fn change_asset_extension_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
    let mut asset = asset_object(doc);
    let next = data.map(from_host_json);
    if obj_get(&asset, "extensions").cloned() == next {
        return Err("change-asset-extension-data: value already has this value".to_string());
    }
    set_optional_json(&mut asset, "extensions", next);
    obj_set(doc, "asset", asset);
    Ok(())
}

/// 🧩️ `change-asset-extra-data` — `document/asset/extras`, the same shape as
/// [`change_asset_extension_data`] for the sibling `extras` member.
#[cfg(feature = "oracles")]
fn change_asset_extra_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
    let mut asset = asset_object(doc);
    let next = data.map(from_host_json);
    if obj_get(&asset, "extras").cloned() == next {
        return Err("change-asset-extra-data: value already has this value".to_string());
    }
    set_optional_json(&mut asset, "extras", next);
    obj_set(doc, "asset", asset);
    Ok(())
}

/// 🧩️ `change-document-extension-data` — `document/extensions`, the DOCUMENT-level sibling of
/// `change-asset-extension-data` (no `asset` wrapper — this one sits directly on the document
/// root), exactly as `../🧬️schema/🧬️mutations/📃️document/🧩️change-extensions/🦀️.rs`'s own
/// `validate` does.
#[cfg(feature = "oracles")]
fn change_document_extension_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
    let next = data.map(from_host_json);
    if obj_get(doc, "extensions").cloned() == next {
        return Err("change-document-extension-data: value already has this value".to_string());
    }
    set_optional_json(doc, "extensions", next);
    Ok(())
}

/// 🧩️ `change-document-extra-data` — `document/extras`, the same shape as
/// [`change_document_extension_data`] for the sibling `extras` member.
#[cfg(feature = "oracles")]
fn change_document_extra_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
    let next = data.map(from_host_json);
    if obj_get(doc, "extras").cloned() == next {
        return Err("change-document-extra-data: value already has this value".to_string());
    }
    set_optional_json(doc, "extras", next);
    Ok(())
}

/// 🎨️ `💎️material` (shard G4, this ticket) — the 4 families `document/materials`,
/// `document/textures`, `document/images`, `document/samplers` each own (§5.20/§5.31/§5.24/§5.29),
/// `create`/`delete`/`move`/`reorder` per family. Read `top_level_collections.rs`'s own `repair`
/// match before writing anything: `materials`/`images`/`samplers` are each a SINGLE simple
/// `Option<usize>` reference site (`meshes[].primitives[].material`, `textures[].source`,
/// `textures[].sampler`), structurally identical in difficulty to `🎥️camera`/`🦴️skin`; `textures` is
/// harder — FIVE reference sites per material (`pbrMetallicRoughness.{baseColorTexture,
/// metallicRoughnessTexture}.index`, `normalTexture.index`, `occlusionTexture.index`,
/// `emissiveTexture.index`), each wrapped in its own `Option<TextureInfo>` CLEARED ENTIRELY (not
/// just the index field) when the referenced texture is deleted, per `repair`'s own `Textures` arm.
/// All four `create-*` payloads (`GltfCreate{Material,Texture,Image,Sampler}Payload { position }`)
/// carry no field content — the same shape `create-skin`/`create-animation` already established —
/// so every `delete-*`'s inverse gets the same bespoke `undo_delete_*` treatment `undo_delete_skin`/
/// `undo_delete_animation` document above, never a second `create-*` call.

/// 🔀️ [`apply_node_ref_change`] generalized to an arbitrary top-level container array —
/// `("textures", "source")` for `document/images`, `("textures", "sampler")` for
/// `document/samplers`, the two other top-level families a bare scalar field on a SIBLING top-level
/// array (not `nodes`) points at.
#[cfg(feature = "oracles")]
fn apply_ref_change_in(doc: &mut json::JsonValue, container_key: &str, field: &str, change: &IndexChange) {
    if let Some(items) = arr_mut(doc, container_key) {
        for item in items.iter_mut() {
            let current = obj_get(item, field).and_then(json::JsonValue::as_usize);
            if let Some(value) = current {
                match remap_index(change, value) {
                    Some(mapped) => obj_set(item, field, json::JsonValue::from(mapped)),
                    None => {
                        if let json::JsonValue::Object(object) = item {
                            object.remove(field);
                        }
                    }
                }
            }
        }
    }
}

/// 🔀️ `meshes[].primitives[].material` — the ONE reference site `document/materials` owns, nested
/// two levels deep (unlike `nodes[].camera`/`nodes[].skin`, a bare top-level scalar).
#[cfg(feature = "oracles")]
fn apply_primitive_material_ref_change(doc: &mut json::JsonValue, change: &IndexChange) {
    if let Some(meshes) = arr_mut(doc, "meshes") {
        for mesh in meshes.iter_mut() {
            if let Some(primitives) = arr_mut(mesh, "primitives") {
                for primitive in primitives.iter_mut() {
                    let current = obj_get(primitive, "material").and_then(json::JsonValue::as_usize);
                    if let Some(value) = current {
                        match remap_index(change, value) {
                            Some(mapped) => obj_set(primitive, "material", json::JsonValue::from(mapped)),
                            None => {
                                if let json::JsonValue::Object(object) = primitive {
                                    object.remove("material");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 🔀️ One `TextureInfo`-shaped reference site (`key` on `container`) — remaps `.index` under
/// `Insert`/`Move`/`Reorder`, but under a `Delete` that drops the reference entirely CLEARS THE
/// WHOLE OBJECT, not just `.index` — the cascading-clear `repair`'s own `Textures` arm documents
/// (`pbr.base_color_texture = None`, never a partial edit), reimplemented independently here.
#[cfg(feature = "oracles")]
fn remap_texture_info_site(container: &mut json::JsonValue, key: &str, change: &IndexChange) {
    let Some(info) = obj_get(container, key).cloned() else { return };
    let Some(index) = obj_get(&info, "index").and_then(json::JsonValue::as_usize) else { return };
    match remap_index(change, index) {
        Some(mapped) => {
            let mut updated = info;
            obj_set(&mut updated, "index", json::JsonValue::from(mapped));
            obj_set(container, key, updated);
        }
        None => {
            if let json::JsonValue::Object(object) = container {
                object.remove(key);
            }
        }
    }
}

/// 🔀️ All five `TextureInfo` reference sites across every material, for `document/textures`'
/// `create`/`delete`/`move`/`reorder`.
#[cfg(feature = "oracles")]
fn apply_texture_info_ref_change(doc: &mut json::JsonValue, change: &IndexChange) {
    if let Some(materials) = arr_mut(doc, "materials") {
        for material in materials.iter_mut() {
            if let Some(pbr) = obj_get(material, "pbrMetallicRoughness").cloned() {
                let mut pbr = pbr;
                remap_texture_info_site(&mut pbr, "baseColorTexture", change);
                remap_texture_info_site(&mut pbr, "metallicRoughnessTexture", change);
                obj_set(material, "pbrMetallicRoughness", pbr);
            }
            remap_texture_info_site(material, "normalTexture", change);
            remap_texture_info_site(material, "occlusionTexture", change);
            remap_texture_info_site(material, "emissiveTexture", change);
        }
    }
}

/// 🦠️ `create-material` — inserts one canonical empty material (`{}`, matching
/// `GltfMaterial::default()`) at `position` into `document/materials`; rejects an out-of-range
/// position, exactly as `../🧬️schema/🧬️mutations/💎️material/🌱️create/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn create_material(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "materials") {
        return Err(format!("create-material: position {position} out of range"));
    }
    apply_primitive_material_ref_change(doc, &IndexChange::Insert(position));
    ensure_array(doc, "materials").insert(position, json::JsonValue::Object(json::object::Object::new()));
    Ok(())
}

/// 🦠️ `delete-material` — removes a real, currently-present material and clears every
/// `meshes[].primitives[].material` reference that pointed at it, exactly as
/// `../🧬️schema/🧬️mutations/💎️material/🗑️delete/🦀️.rs`'s own `validate`/`materials_op` do.
#[cfg(feature = "oracles")]
fn delete_material(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "materials") {
        return Err(format!("delete-material: index {index} out of range"));
    }
    apply_primitive_material_ref_change(doc, &IndexChange::Delete(index));
    ensure_array(doc, "materials").remove(index);
    Ok(())
}

/// 🦠️ `move-material` — relocates a real material to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/💎️material/🚚️move/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn move_material(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "materials");
    if index >= total || position >= total {
        return Err(format!("move-material: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err("move-material: position already equals index".to_string());
    }
    apply_primitive_material_ref_change(doc, &IndexChange::Move(index, position));
    let materials = ensure_array(doc, "materials");
    let value = materials.remove(index);
    materials.insert(position, value);
    Ok(())
}

/// 🦠️ `reorder-materials` — `order` must be a genuine permutation of every existing index and must
/// actually move at least one, exactly as
/// `../🧬️schema/🧬️mutations/💎️material/🔀️reorder/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn reorder_materials(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
    let total = top_level_len(doc, "materials");
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err("reorder-materials: order must contain every index exactly once".to_string());
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err("reorder-materials: order already matches".to_string());
    }
    apply_primitive_material_ref_change(doc, &IndexChange::Reorder(order));
    let materials = ensure_array(doc, "materials");
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| materials[*index].clone()).collect();
    *materials = reordered;
    Ok(())
}

/// ↩️ `delete-material`'s own inverse — the identical reasoning [`undo_delete_skin`] documents:
/// `create-material`'s own payload carries no field content, so it can only ever recreate an EMPTY
/// material. Restores `document/materials` and every `meshes[].primitives[].material` reference
/// straight off the ORIGINAL (pre-mutation) document.
#[cfg(feature = "oracles")]
pub fn undo_delete_material(mutated: &[u8], original: &[u8]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, _) = read_glb(original)?;
    obj_set(&mut doc, "materials", json::JsonValue::Array(arr(&source, "materials")));
    let source_meshes = arr(&source, "meshes");
    if let Some(meshes) = arr_mut(&mut doc, "meshes") {
        for (mesh_index, mesh) in meshes.iter_mut().enumerate() {
            let source_primitives = source_meshes.get(mesh_index).map(|mesh| arr(mesh, "primitives")).unwrap_or_default();
            if let Some(primitives) = arr_mut(mesh, "primitives") {
                for (primitive_index, primitive) in primitives.iter_mut().enumerate() {
                    match source_primitives.get(primitive_index).and_then(|primitive| obj_get(primitive, "material")).and_then(json::JsonValue::as_usize) {
                        Some(value) => obj_set(primitive, "material", json::JsonValue::from(value)),
                        None => {
                            if let json::JsonValue::Object(object) = primitive {
                                object.remove("material");
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn undo_delete_material(_mutated: &[u8], _original: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🦠️ `create-image` — inserts one canonical empty image (`{}`) at `position` into
/// `document/images`; rejects an out-of-range position, exactly as
/// `../🧬️schema/🧬️mutations/🖼️image/🌱️create/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn create_image(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "images") {
        return Err(format!("create-image: position {position} out of range"));
    }
    apply_ref_change_in(doc, "textures", "source", &IndexChange::Insert(position));
    ensure_array(doc, "images").insert(position, json::JsonValue::Object(json::object::Object::new()));
    Ok(())
}

/// 🦠️ `delete-image` — removes a real, currently-present image and clears every `textures[].source`
/// reference that pointed at it, exactly as
/// `../🧬️schema/🧬️mutations/🖼️image/🗑️delete/🦀️.rs`'s own `validate`/`images_op` do.
#[cfg(feature = "oracles")]
fn delete_image(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "images") {
        return Err(format!("delete-image: index {index} out of range"));
    }
    apply_ref_change_in(doc, "textures", "source", &IndexChange::Delete(index));
    ensure_array(doc, "images").remove(index);
    Ok(())
}

/// 🦠️ `move-image` — relocates a real image to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🖼️image/🚚️move/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn move_image(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "images");
    if index >= total || position >= total {
        return Err(format!("move-image: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err("move-image: position already equals index".to_string());
    }
    apply_ref_change_in(doc, "textures", "source", &IndexChange::Move(index, position));
    let images = ensure_array(doc, "images");
    let value = images.remove(index);
    images.insert(position, value);
    Ok(())
}

/// 🦠️ `reorder-images` — `order` must be a genuine permutation of every existing index and must
/// actually move at least one, exactly as
/// `../🧬️schema/🧬️mutations/🖼️image/🔀️reorder/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn reorder_images(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
    let total = top_level_len(doc, "images");
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err("reorder-images: order must contain every index exactly once".to_string());
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err("reorder-images: order already matches".to_string());
    }
    apply_ref_change_in(doc, "textures", "source", &IndexChange::Reorder(order));
    let images = ensure_array(doc, "images");
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| images[*index].clone()).collect();
    *images = reordered;
    Ok(())
}

/// ↩️ `delete-image`'s own inverse — the identical reasoning [`undo_delete_material`] documents.
/// Restores `document/images` and every `textures[].source` reference straight off the ORIGINAL
/// (pre-mutation) document.
#[cfg(feature = "oracles")]
pub fn undo_delete_image(mutated: &[u8], original: &[u8]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, _) = read_glb(original)?;
    obj_set(&mut doc, "images", json::JsonValue::Array(arr(&source, "images")));
    let source_textures = arr(&source, "textures");
    if let Some(textures) = arr_mut(&mut doc, "textures") {
        for (index, texture) in textures.iter_mut().enumerate() {
            match source_textures.get(index).and_then(|texture| obj_get(texture, "source")).and_then(json::JsonValue::as_usize) {
                Some(value) => obj_set(texture, "source", json::JsonValue::from(value)),
                None => {
                    if let json::JsonValue::Object(object) = texture {
                        object.remove("source");
                    }
                }
            }
        }
    }
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn undo_delete_image(_mutated: &[u8], _original: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🦠️ `create-sampler` — inserts one canonical empty sampler (`{}`) at `position` into
/// `document/samplers`; rejects an out-of-range position, exactly as
/// `../🧬️schema/🧬️mutations/🎛️sampler/🌱️create/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn create_sampler(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "samplers") {
        return Err(format!("create-sampler: position {position} out of range"));
    }
    apply_ref_change_in(doc, "textures", "sampler", &IndexChange::Insert(position));
    ensure_array(doc, "samplers").insert(position, json::JsonValue::Object(json::object::Object::new()));
    Ok(())
}

/// 🦠️ `delete-sampler` — removes a real, currently-present sampler and clears every
/// `textures[].sampler` reference that pointed at it, exactly as
/// `../🧬️schema/🧬️mutations/🎛️sampler/🗑️delete/🦀️.rs`'s own `validate`/`samplers_op` do.
#[cfg(feature = "oracles")]
fn delete_sampler(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "samplers") {
        return Err(format!("delete-sampler: index {index} out of range"));
    }
    apply_ref_change_in(doc, "textures", "sampler", &IndexChange::Delete(index));
    ensure_array(doc, "samplers").remove(index);
    Ok(())
}

/// 🦠️ `move-sampler` — relocates a real sampler to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🎛️sampler/🚚️move/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn move_sampler(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "samplers");
    if index >= total || position >= total {
        return Err(format!("move-sampler: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err("move-sampler: position already equals index".to_string());
    }
    apply_ref_change_in(doc, "textures", "sampler", &IndexChange::Move(index, position));
    let samplers = ensure_array(doc, "samplers");
    let value = samplers.remove(index);
    samplers.insert(position, value);
    Ok(())
}

/// 🦠️ `reorder-samplers` — `order` must be a genuine permutation of every existing index and must
/// actually move at least one, exactly as
/// `../🧬️schema/🧬️mutations/🎛️sampler/🔀️reorder/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn reorder_samplers(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
    let total = top_level_len(doc, "samplers");
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err("reorder-samplers: order must contain every index exactly once".to_string());
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err("reorder-samplers: order already matches".to_string());
    }
    apply_ref_change_in(doc, "textures", "sampler", &IndexChange::Reorder(order));
    let samplers = ensure_array(doc, "samplers");
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| samplers[*index].clone()).collect();
    *samplers = reordered;
    Ok(())
}

/// ↩️ `delete-sampler`'s own inverse — the identical reasoning [`undo_delete_image`] documents.
/// Restores `document/samplers` and every `textures[].sampler` reference straight off the ORIGINAL
/// (pre-mutation) document.
#[cfg(feature = "oracles")]
pub fn undo_delete_sampler(mutated: &[u8], original: &[u8]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, _) = read_glb(original)?;
    obj_set(&mut doc, "samplers", json::JsonValue::Array(arr(&source, "samplers")));
    let source_textures = arr(&source, "textures");
    if let Some(textures) = arr_mut(&mut doc, "textures") {
        for (index, texture) in textures.iter_mut().enumerate() {
            match source_textures.get(index).and_then(|texture| obj_get(texture, "sampler")).and_then(json::JsonValue::as_usize) {
                Some(value) => obj_set(texture, "sampler", json::JsonValue::from(value)),
                None => {
                    if let json::JsonValue::Object(object) = texture {
                        object.remove("sampler");
                    }
                }
            }
        }
    }
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn undo_delete_sampler(_mutated: &[u8], _original: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🦠️ `create-texture` — inserts one canonical empty texture (`{}`) at `position` into
/// `document/textures`; rejects an out-of-range position, exactly as
/// `../🧬️schema/🧬️mutations/🎨️texture/🌱️create/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn create_texture(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "textures") {
        return Err(format!("create-texture: position {position} out of range"));
    }
    apply_texture_info_ref_change(doc, &IndexChange::Insert(position));
    ensure_array(doc, "textures").insert(position, json::JsonValue::Object(json::object::Object::new()));
    Ok(())
}

/// 🦠️ `delete-texture` — removes a real, currently-present texture and clears every one of the five
/// `TextureInfo` reference sites (whole object, not just `.index`) that pointed at it, exactly as
/// `../🧬️schema/🧬️mutations/🎨️texture/🗑️delete/🦀️.rs`'s own `validate`/`textures_op` do.
#[cfg(feature = "oracles")]
fn delete_texture(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "textures") {
        return Err(format!("delete-texture: index {index} out of range"));
    }
    apply_texture_info_ref_change(doc, &IndexChange::Delete(index));
    ensure_array(doc, "textures").remove(index);
    Ok(())
}

/// 🦠️ `move-texture` — relocates a real texture to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🎨️texture/🚚️move/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn move_texture(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, "textures");
    if index >= total || position >= total {
        return Err(format!("move-texture: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err("move-texture: position already equals index".to_string());
    }
    apply_texture_info_ref_change(doc, &IndexChange::Move(index, position));
    let textures = ensure_array(doc, "textures");
    let value = textures.remove(index);
    textures.insert(position, value);
    Ok(())
}

/// 🦠️ `reorder-textures` — `order` must be a genuine permutation of every existing index and must
/// actually move at least one, exactly as
/// `../🧬️schema/🧬️mutations/🎨️texture/🔀️reorder/🦀️.rs`'s own `validate` does.
#[cfg(feature = "oracles")]
fn reorder_textures(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
    let total = top_level_len(doc, "textures");
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err("reorder-textures: order must contain every index exactly once".to_string());
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err("reorder-textures: order already matches".to_string());
    }
    apply_texture_info_ref_change(doc, &IndexChange::Reorder(order));
    let textures = ensure_array(doc, "textures");
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| textures[*index].clone()).collect();
    *textures = reordered;
    Ok(())
}

/// ↩️ `delete-texture`'s own inverse — the identical reasoning [`undo_delete_material`] documents,
/// restoring the FULL `TextureInfo` object at each of the five sites (never just `.index`) straight
/// off the ORIGINAL (pre-mutation) document, mirroring [`apply_texture_info_ref_change`]'s own
/// cascading-clear shape in reverse.
#[cfg(feature = "oracles")]
pub fn undo_delete_texture(mutated: &[u8], original: &[u8]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, _) = read_glb(original)?;
    obj_set(&mut doc, "textures", json::JsonValue::Array(arr(&source, "textures")));
    fn restore_site(container: &mut json::JsonValue, source_container: &json::JsonValue, key: &str) {
        match obj_get(source_container, key) {
            Some(info) => obj_set(container, key, info.clone()),
            None => {
                if let json::JsonValue::Object(object) = container {
                    object.remove(key);
                }
            }
        }
    }
    let source_materials = arr(&source, "materials");
    if let Some(materials) = arr_mut(&mut doc, "materials") {
        for (index, material) in materials.iter_mut().enumerate() {
            let Some(source_material) = source_materials.get(index) else { continue };
            match obj_get(source_material, "pbrMetallicRoughness") {
                Some(source_pbr) => {
                    let mut pbr = obj_get(material, "pbrMetallicRoughness").cloned().unwrap_or_else(|| json::JsonValue::Object(json::object::Object::new()));
                    restore_site(&mut pbr, source_pbr, "baseColorTexture");
                    restore_site(&mut pbr, source_pbr, "metallicRoughnessTexture");
                    obj_set(material, "pbrMetallicRoughness", pbr);
                }
                None => {
                    if let json::JsonValue::Object(object) = material {
                        object.remove("pbrMetallicRoughness");
                    }
                }
            }
            restore_site(material, source_material, "normalTexture");
            restore_site(material, source_material, "occlusionTexture");
            restore_site(material, source_material, "emissiveTexture");
        }
    }
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn undo_delete_texture(_mutated: &[u8], _original: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Kinds

//#region 🔖️StructureKinds
/// 🧭️ The 76 kinds of the `🎬️scene`, `💿️buffer` and `🕸️mesh` catalogs, each performed on the
/// independently parsed JSON tree. Every top-level family change re-derives, from the format's own
/// reference rules, which sites point into that family and how they must track the change:
/// scene roots, node children, skin skeleton/joints and animation channel targets point at nodes;
/// node meshes point at meshes; primitive attributes/indices/morph targets, skin inverse-bind
/// matrices and animation sampler input/output point at accessors; accessor and image buffer views
/// point at buffer views; buffer views point at buffers. A dropped optional reference is removed, a
/// dropped required one (sampler input/output, sparse views, a view's buffer) refuses the change.
#[cfg(feature = "oracles")]
fn remap_optional_field(item: &mut json::JsonValue, key: &str, change: &IndexChange) {
    if let Some(value) = obj_get(item, key).and_then(json::JsonValue::as_usize) {
        match remap_index(change, value) {
            Some(mapped) => obj_set(item, key, json::JsonValue::from(mapped)),
            None => remove_key(item, key),
        }
    }
}

/// 🔀️ A required index field: a change that drops its target refuses instead of detaching it.
#[cfg(feature = "oracles")]
fn remap_required_field(item: &mut json::JsonValue, key: &str, change: &IndexChange, path: &str) -> Result<(), String> {
    if let Some(value) = obj_get(item, key).and_then(json::JsonValue::as_usize) {
        let mapped = remap_index(change, value).ok_or_else(|| format!("{path}: the change would break a required reference"))?;
        obj_set(item, key, json::JsonValue::from(mapped));
    }
    Ok(())
}

/// 🔀️ An index list field (`children`, scene `nodes`, skin `joints`): dropped entries leave the list.
#[cfg(feature = "oracles")]
fn remap_list_field(item: &mut json::JsonValue, key: &str, change: &IndexChange) {
    if let Some(items) = arr_mut(item, key) {
        *items = items.iter().filter_map(json::JsonValue::as_usize).filter_map(|value| remap_index(change, value)).map(json::JsonValue::from).collect();
    }
}

/// 🔀️ A semantic → accessor map (`attributes`, one morph target): dropped entries leave the map,
/// every other entry keeps its position.
#[cfg(feature = "oracles")]
fn remap_semantic_map(map: &mut json::JsonValue, change: &IndexChange) {
    if let json::JsonValue::Object(object) = map {
        let mut next = json::object::Object::new();
        for (semantic, value) in object.iter() {
            if let Some(mapped) = value.as_usize().and_then(|index| remap_index(change, index)) {
                next.insert(semantic, json::JsonValue::from(mapped));
            }
        }
        *object = next;
    }
}

/// ✂️ Removes one object member; a no-op when absent.
#[cfg(feature = "oracles")]
fn remove_key(item: &mut json::JsonValue, key: &str) {
    if let json::JsonValue::Object(object) = item {
        object.remove(key);
    }
}

/// 🔀️ Every reference site of one top-level family, moved with `change`.
#[cfg(feature = "oracles")]
fn remap_family_references(doc: &mut json::JsonValue, family: &str, change: &IndexChange) -> Result<(), String> {
    match family {
        "scenes" => remap_optional_field(doc, "scene", change),
        "nodes" => {
            for scene in arr_mut(doc, "scenes").into_iter().flatten() {
                remap_list_field(scene, "nodes", change);
            }
            for node in arr_mut(doc, "nodes").into_iter().flatten() {
                remap_list_field(node, "children", change);
            }
            for skin in arr_mut(doc, "skins").into_iter().flatten() {
                remap_optional_field(skin, "skeleton", change);
                remap_list_field(skin, "joints", change);
            }
            for animation in arr_mut(doc, "animations").into_iter().flatten() {
                for channel in arr_mut(animation, "channels").into_iter().flatten() {
                    if let json::JsonValue::Object(object) = channel {
                        if let Some(target) = object.get_mut("target") {
                            remap_optional_field(target, "node", change);
                        }
                    }
                }
            }
        }
        "meshes" => {
            for node in arr_mut(doc, "nodes").into_iter().flatten() {
                remap_optional_field(node, "mesh", change);
            }
        }
        "accessors" => {
            for mesh in arr_mut(doc, "meshes").into_iter().flatten() {
                for primitive in arr_mut(mesh, "primitives").into_iter().flatten() {
                    if let json::JsonValue::Object(object) = primitive {
                        if let Some(attributes) = object.get_mut("attributes") {
                            remap_semantic_map(attributes, change);
                        }
                    }
                    remap_optional_field(primitive, "indices", change);
                    for target in arr_mut(primitive, "targets").into_iter().flatten() {
                        remap_semantic_map(target, change);
                    }
                }
            }
            for skin in arr_mut(doc, "skins").into_iter().flatten() {
                remap_optional_field(skin, "inverseBindMatrices", change);
            }
            for animation in arr_mut(doc, "animations").into_iter().flatten() {
                for sampler in arr_mut(animation, "samplers").into_iter().flatten() {
                    remap_required_field(sampler, "input", change, "document/animations")?;
                    remap_required_field(sampler, "output", change, "document/animations")?;
                }
            }
        }
        "bufferViews" => {
            for accessor in arr_mut(doc, "accessors").into_iter().flatten() {
                remap_optional_field(accessor, "bufferView", change);
                if let json::JsonValue::Object(object) = accessor {
                    if let Some(sparse) = object.get_mut("sparse") {
                        for part in ["indices", "values"] {
                            if let json::JsonValue::Object(sparse_object) = sparse {
                                if let Some(section) = sparse_object.get_mut(part) {
                                    remap_required_field(section, "bufferView", change, "document/accessors")?;
                                }
                            }
                        }
                    }
                }
            }
            for image in arr_mut(doc, "images").into_iter().flatten() {
                remap_optional_field(image, "bufferView", change);
            }
        }
        "buffers" => {
            for view in arr_mut(doc, "bufferViews").into_iter().flatten() {
                remap_required_field(view, "buffer", change, "document/bufferViews")?;
            }
        }
        other => return Err(format!("no reference rule for top-level family {other}")),
    }
    Ok(())
}

/// 🔢️ `order` is a genuine permutation of `0..total` that moves at least one entry.
#[cfg(feature = "oracles")]
fn checked_permutation(kind: &str, order: &[usize], total: usize) -> Result<(), String> {
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err(format!("{kind}: order must contain every index exactly once"));
    }
    if order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err(format!("{kind}: order already matches"));
    }
    Ok(())
}

/// 🌱️ Inserts `value` at `position` of top-level `family`, moving every reference first.
#[cfg(feature = "oracles")]
fn family_insert(doc: &mut json::JsonValue, kind: &str, family: &str, position: usize, value: json::JsonValue) -> Result<(), String> {
    if position > top_level_len(doc, family) {
        return Err(format!("{kind}: position {position} out of range"));
    }
    remap_family_references(doc, family, &IndexChange::Insert(position))?;
    ensure_array(doc, family).insert(position, value);
    Ok(())
}

/// 🗑️ Removes entry `index` of top-level `family`, dropping or refusing every reference to it.
#[cfg(feature = "oracles")]
fn family_delete(doc: &mut json::JsonValue, kind: &str, family: &str, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, family) {
        return Err(format!("{kind}: index {index} out of range"));
    }
    remap_family_references(doc, family, &IndexChange::Delete(index))?;
    ensure_array(doc, family).remove(index);
    Ok(())
}

/// 🚚️ Moves entry `index` of top-level `family` to `position`, moving every reference with it.
#[cfg(feature = "oracles")]
fn family_move(doc: &mut json::JsonValue, kind: &str, family: &str, index: usize, position: usize) -> Result<(), String> {
    let total = top_level_len(doc, family);
    if index >= total || position >= total {
        return Err(format!("{kind}: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    remap_family_references(doc, family, &IndexChange::Move(index, position))?;
    let items = ensure_array(doc, family);
    let value = items.remove(index);
    items.insert(position, value);
    Ok(())
}

/// 🔀️ Permutes top-level `family` by `order`, moving every reference with it.
#[cfg(feature = "oracles")]
fn family_reorder(doc: &mut json::JsonValue, kind: &str, family: &str, order: &[usize]) -> Result<(), String> {
    checked_permutation(kind, order, top_level_len(doc, family))?;
    remap_family_references(doc, family, &IndexChange::Reorder(order))?;
    let items = ensure_array(doc, family);
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| items[*index].clone()).collect();
    *items = reordered;
    Ok(())
}

/// 🔎️ `document/<family>/<index>`, mutable, or a range error.
#[cfg(feature = "oracles")]
fn entry_mut<'a>(doc: &'a mut json::JsonValue, kind: &str, family: &str, index: usize) -> Result<&'a mut json::JsonValue, String> {
    arr_mut(doc, family).and_then(|items| items.get_mut(index)).ok_or_else(|| format!("{kind}: {family} {index} out of range"))
}

/// 🔎️ `document/meshes/<mesh>/primitives/<primitive>`, mutable, or a range error.
#[cfg(feature = "oracles")]
fn primitive_mut<'a>(doc: &'a mut json::JsonValue, kind: &str, mesh: usize, primitive: usize) -> Result<&'a mut json::JsonValue, String> {
    let mesh_value = entry_mut(doc, kind, "meshes", mesh)?;
    arr_mut(mesh_value, "primitives").and_then(|items| items.get_mut(primitive)).ok_or_else(|| format!("{kind}: primitive {primitive} of mesh {mesh} out of range"))
}

/// 🔎️ One morph target of one primitive, mutable, or a range error.
#[cfg(feature = "oracles")]
fn target_mut<'a>(doc: &'a mut json::JsonValue, kind: &str, mesh: usize, primitive: usize, target: usize) -> Result<&'a mut json::JsonValue, String> {
    let primitive_value = primitive_mut(doc, kind, mesh, primitive)?;
    arr_mut(primitive_value, "targets").and_then(|items| items.get_mut(target)).ok_or_else(|| format!("{kind}: target {target} out of range"))
}

/// 🔧️ Sets `key` to `value`, or removes it when `value` is `None`.
#[cfg(feature = "oracles")]
fn set_or_remove(item: &mut json::JsonValue, key: &str, value: Option<json::JsonValue>) {
    match value {
        Some(value) => obj_set(item, key, value),
        None => remove_key(item, key),
    }
}

/// 🧩️ A `{state: present, value}` / `{state: absent}` payload member, as the tree value to store.
#[cfg(feature = "oracles")]
fn presence_param(params: &Json) -> Result<Option<json::JsonValue>, String> {
    let data = object_param(params, "data")?;
    match data.get("state") {
        Some(Json::String(state)) if state == "absent" => Ok(None),
        Some(Json::String(state)) if state == "present" => data.get("value").map(|value| Some(from_host_json(value))).ok_or_else(|| "present data carries no `value`".to_string()),
        _ => Err("`data.state` must be `present` or `absent`".to_string()),
    }
}

/// 📝️ A nullable string payload member.
#[cfg(feature = "oracles")]
fn nullable_str_param(params: &Json, key: &str) -> Result<Option<String>, String> {
    match params.get(key) {
        Some(Json::String(value)) => Ok(Some(value.clone())),
        Some(Json::Null) | None => Ok(None),
        _ => Err(format!("`{key}` must be a string or null")),
    }
}

/// 🔢️ A number array payload member.
#[cfg(feature = "oracles")]
fn f64_array_param(params: &Json, key: &str) -> Result<Vec<f64>, String> {
    match params.get(key) {
        Some(Json::Array(items)) => items
            .iter()
            .map(|item| match item {
                Json::Number(number) => Ok(*number),
                _ => Err(format!("`{key}` must hold only numbers")),
            })
            .collect(),
        _ => Err(format!("missing or non-array `{key}`")),
    }
}

/// 🔢️ `weights` as the tree stores it: an empty list is no member at all.
#[cfg(feature = "oracles")]
fn weights_value(weights: &[f64]) -> Option<json::JsonValue> {
    (!weights.is_empty()).then(|| json::JsonValue::Array(weights.iter().map(|value| json::JsonValue::from(*value)).collect()))
}

/// 🔢️ Every primitive of `mesh` carries exactly `count` morph targets.
#[cfg(feature = "oracles")]
fn mesh_target_arity_is(doc: &json::JsonValue, mesh: usize, count: usize) -> bool {
    arr(doc, "meshes").get(mesh).is_some_and(|mesh_value| arr(mesh_value, "primitives").iter().all(|primitive| arr(primitive, "targets").len() == count))
}

/// 🚚️ Moves one key of a semantic map to `position`, keeping every other key's order.
#[cfg(feature = "oracles")]
fn move_semantic(map: &mut json::JsonValue, kind: &str, semantic: &str, position: usize) -> Result<(), String> {
    let json::JsonValue::Object(object) = map else { return Err(format!("{kind}: not a semantic map")) };
    let mut entries: Vec<(String, json::JsonValue)> = object.iter().map(|(key, value)| (key.to_string(), value.clone())).collect();
    let index = entries.iter().position(|(key, _)| key == semantic).ok_or_else(|| format!("{kind}: semantic {semantic} is not bound"))?;
    if position >= entries.len() {
        return Err(format!("{kind}: position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    let entry = entries.remove(index);
    entries.insert(position, entry);
    let mut next = json::object::Object::new();
    for (key, value) in entries {
        next.insert(&key, value);
    }
    *object = next;
    Ok(())
}

/// 🔀️ Rebuilds a semantic map in `order`, which must name every bound semantic once.
#[cfg(feature = "oracles")]
fn reorder_semantics(map: &mut json::JsonValue, kind: &str, order: &[String]) -> Result<(), String> {
    let json::JsonValue::Object(object) = map else { return Err(format!("{kind}: not a semantic map")) };
    let unique: std::collections::BTreeSet<&String> = order.iter().collect();
    if order.len() != object.len() || unique.len() != order.len() || order.iter().any(|semantic| object.get(semantic).is_none()) {
        return Err(format!("{kind}: order must contain every semantic once"));
    }
    let mut next = json::object::Object::new();
    for semantic in order {
        next.insert(semantic, object.get(semantic).cloned().expect("validated semantic"));
    }
    *object = next;
    Ok(())
}

/// 🔗️ Appends `semantic → accessor` to a semantic map; the semantic must be new and non-blank.
#[cfg(feature = "oracles")]
fn bind_semantic(map: &mut json::JsonValue, kind: &str, semantic: &str, accessor: usize) -> Result<(), String> {
    if semantic.trim().is_empty() || obj_get(map, semantic).is_some() {
        return Err(format!("{kind}: semantic must be non-empty and unique"));
    }
    if !matches!(map, json::JsonValue::Object(_)) {
        *map = json::JsonValue::Object(json::object::Object::new());
    }
    obj_set(map, semantic, json::JsonValue::from(accessor));
    Ok(())
}

/// ✂️ Removes a bound semantic from a semantic map.
#[cfg(feature = "oracles")]
fn unbind_semantic(map: &mut json::JsonValue, kind: &str, semantic: &str) -> Result<(), String> {
    if obj_get(map, semantic).is_none() {
        return Err(format!("{kind}: semantic {semantic} is not bound"));
    }
    remove_key(map, semantic);
    Ok(())
}

/// 🧮️ The node transform a `{kind: trs | matrix, …}` payload writes: a matrix clears TRS and
/// TRS clears the matrix, each absent TRS component removed.
#[cfg(feature = "oracles")]
fn change_node_transform(doc: &mut json::JsonValue, node: usize, transform: &Json) -> Result<(), String> {
    let kind = "change-node-transform";
    let numbers = |key: &str, length: usize| -> Result<Option<json::JsonValue>, String> {
        match transform.get(key) {
            None | Some(Json::Null) => Ok(None),
            Some(Json::Array(items)) if items.len() == length => items
                .iter()
                .map(|item| match item {
                    Json::Number(number) if number.is_finite() => Ok(json::JsonValue::from(*number)),
                    _ => Err(format!("{kind}: {key} must hold finite numbers")),
                })
                .collect::<Result<Vec<_>, String>>()
                .map(|values| Some(json::JsonValue::Array(values))),
            _ => Err(format!("{kind}: {key} must hold {length} numbers")),
        }
    };
    let (matrix, translation, rotation, scale) = match transform.get("kind") {
        Some(Json::String(mode)) if mode == "matrix" => (numbers("matrix", 16)?.ok_or(format!("{kind}: matrix missing"))?.into(), None, None, None),
        Some(Json::String(mode)) if mode == "trs" => (None, numbers("translation", 3)?, numbers("rotation", 4)?, numbers("scale", 3)?),
        _ => return Err(format!("{kind}: transform kind must be `trs` or `matrix`")),
    };
    let node_value = entry_mut(doc, kind, "nodes", node)?;
    set_or_remove(node_value, "matrix", matrix);
    set_or_remove(node_value, "translation", translation);
    set_or_remove(node_value, "rotation", rotation);
    set_or_remove(node_value, "scale", scale);
    Ok(())
}

/// 🌿️ `move-node-parent` — detaches the child from every parent and every scene root, then
/// inserts it under the new parent at `position`, refusing a cycle.
#[cfg(feature = "oracles")]
fn move_node_parent(doc: &mut json::JsonValue, parent: usize, child: usize, position: usize) -> Result<(), String> {
    let kind = "move-node-parent";
    let total = top_level_len(doc, "nodes");
    if parent >= total || child >= total {
        return Err(format!("{kind}: node out of range"));
    }
    if parent == child {
        return Err(format!("{kind}: a node cannot parent itself"));
    }
    let mut pending = vec![child];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(current) = pending.pop() {
        if current == parent {
            return Err(format!("{kind}: relationship closes a cycle"));
        }
        if seen.insert(current) {
            pending.extend(node_children(doc, current));
        }
    }
    if position > node_children(doc, parent).iter().filter(|value| **value != child).count() {
        return Err(format!("{kind}: position out of range"));
    }
    for node in arr_mut(doc, "nodes").into_iter().flatten() {
        if let Some(children) = arr_mut(node, "children") {
            children.retain(|value| value.as_usize() != Some(child));
        }
    }
    for scene in arr_mut(doc, "scenes").into_iter().flatten() {
        if let Some(roots) = arr_mut(scene, "nodes") {
            roots.retain(|value| value.as_usize() != Some(child));
        }
    }
    ensure_array(entry_mut(doc, kind, "nodes", parent)?, "children").insert(position, json::JsonValue::from(child));
    Ok(())
}

/// 🚚️ Moves one element of an index list (`children`, scene `nodes`) to `position`.
#[cfg(feature = "oracles")]
fn move_in_index_list(list: &mut Vec<json::JsonValue>, kind: &str, value: usize, position: usize) -> Result<(), String> {
    let index = list.iter().position(|item| item.as_usize() == Some(value)).ok_or_else(|| format!("{kind}: {value} is not linked"))?;
    if position >= list.len() {
        return Err(format!("{kind}: position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    let item = list.remove(index);
    list.insert(position, item);
    Ok(())
}

/// 🔀️ Replaces an index list with `order`, which must name every current member once and differ.
#[cfg(feature = "oracles")]
fn reorder_index_list(list: &mut Vec<json::JsonValue>, kind: &str, order: &[usize]) -> Result<(), String> {
    let current: Vec<usize> = list.iter().filter_map(json::JsonValue::as_usize).collect();
    let mut sorted_order = order.to_vec();
    sorted_order.sort_unstable();
    sorted_order.dedup();
    let mut sorted_current = current.clone();
    sorted_current.sort_unstable();
    if order.len() != current.len() || sorted_order != sorted_current {
        return Err(format!("{kind}: order must contain every member once"));
    }
    if order == current.as_slice() {
        return Err(format!("{kind}: reorder must change order"));
    }
    *list = order.iter().map(|value| json::JsonValue::from(*value)).collect();
    Ok(())
}

/// 🔀️ Permutes a nested array (`primitives`, `targets`) by `order`.
#[cfg(feature = "oracles")]
fn reorder_nested(list: &mut Vec<json::JsonValue>, kind: &str, order: &[usize], require_change: bool) -> Result<(), String> {
    let total = list.len();
    let mut seen = std::collections::BTreeSet::new();
    if order.len() != total || order.iter().any(|index| *index >= total || !seen.insert(*index)) {
        return Err(format!("{kind}: order must contain every index exactly once"));
    }
    if require_change && order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err(format!("{kind}: reorder must change order"));
    }
    let reordered: Vec<json::JsonValue> = order.iter().map(|index| list[*index].clone()).collect();
    *list = reordered;
    Ok(())
}

/// 🚚️ Moves one element of a nested array to `position`.
#[cfg(feature = "oracles")]
fn move_nested(list: &mut Vec<json::JsonValue>, kind: &str, index: usize, position: usize) -> Result<(), String> {
    if index >= list.len() || position >= list.len() {
        return Err(format!("{kind}: index {index} or position {position} out of range"));
    }
    if index == position {
        return Err(format!("{kind}: position already equals index"));
    }
    let item = list.remove(index);
    list.insert(position, item);
    Ok(())
}

/// 🔤️ RFC 4648 base64 with padding, the encoding a `data:` buffer URI carries.
#[cfg(feature = "oracles")]
fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let triple = (u32::from(chunk[0]) << 16) | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8) | u32::from(*chunk.get(2).unwrap_or(&0));
        for (position, shift) in [18u32, 12, 6, 0].into_iter().enumerate() {
            if position <= chunk.len() {
                out.push(ALPHABET[((triple >> shift) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// 🔤️ RFC 4648 base64 decode, padding optional; `None` on any foreign character.
#[cfg(feature = "oracles")]
fn base64_decode(text: &str) -> Option<Vec<u8>> {
    let value = |byte: u8| -> Option<u32> {
        match byte {
            b'A'..=b'Z' => Some(u32::from(byte - b'A')),
            b'a'..=b'z' => Some(u32::from(byte - b'a') + 26),
            b'0'..=b'9' => Some(u32::from(byte - b'0') + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    };
    let digits: Vec<u8> = text.bytes().filter(|byte| *byte != b'=').collect();
    let mut out = Vec::with_capacity(digits.len() * 3 / 4);
    for chunk in digits.chunks(4) {
        let mut accumulator = 0u32;
        for (index, byte) in chunk.iter().enumerate() {
            accumulator |= value(*byte)? << (18 - 6 * index as u32);
        }
        out.extend_from_slice(&accumulator.to_be_bytes()[1..chunk.len()]);
    }
    Some(out)
}

/// 🦠️ Dispatches one of the 76 structure kinds; `None` when `kind` is not one of them.
#[cfg(feature = "oracles")]
fn apply_structure(doc: &mut json::JsonValue, kind: &str, params: &Json) -> Option<Result<(), String>> {
    let run = || -> Result<(), String> {
        match kind {
            "bind-default-scene" => {
                let scene = usize_param(params, "scene")?;
                if scene >= top_level_len(doc, "scenes") {
                    return Err(format!("{kind}: scene {scene} out of range"));
                }
                if default_scene_index(doc) == Some(scene) {
                    return Err(format!("{kind}: scene is already default"));
                }
                obj_set(doc, "scene", json::JsonValue::from(scene));
            }
            "unbind-default-scene" => {
                if default_scene_index(doc).is_none() {
                    return Err(format!("{kind}: no default scene is bound"));
                }
                remove_key(doc, "scene");
            }
            "bind-node-camera" | "bind-node-mesh" | "bind-node-skin" => {
                let (field, family) = match kind {
                    "bind-node-camera" => ("camera", "cameras"),
                    "bind-node-mesh" => ("mesh", "meshes"),
                    _ => ("skin", "skins"),
                };
                let target = usize_param(params, field)?;
                if target >= top_level_len(doc, family) {
                    return Err(format!("{kind}: {field} {target} out of range"));
                }
                obj_set(entry_mut(doc, kind, "nodes", usize_param(params, "node")?)?, field, json::JsonValue::from(target));
            }
            "unbind-node-camera" | "unbind-node-mesh" | "unbind-node-skin" => {
                let field = kind.trim_start_matches("unbind-node-");
                let node = entry_mut(doc, kind, "nodes", usize_param(params, "node")?)?;
                if obj_get(node, field).is_none() {
                    return Err(format!("{kind}: node has no binding"));
                }
                remove_key(node, field);
            }
            "bind-node-child" => bind_node_child(doc, usize_param(params, "parent")?, usize_param(params, "child")?, usize_param(params, "position")?)?,
            "unbind-node-child" => unbind_node_child(doc, usize_param(params, "parent")?, usize_param(params, "child")?)?,
            "bind-scene-root-node" => bind_scene_root_node(doc, usize_param(params, "scene")?, usize_param(params, "node")?, usize_param(params, "position")?)?,
            "unbind-scene-root-node" => unbind_scene_root_node(doc, usize_param(params, "scene")?, usize_param(params, "node")?)?,
            "move-node-child" => {
                let child = usize_param(params, "child")?;
                let position = usize_param(params, "position")?;
                let parent = entry_mut(doc, kind, "nodes", usize_param(params, "parent")?)?;
                move_in_index_list(ensure_array(parent, "children"), kind, child, position)?;
            }
            "reorder-node-children" => {
                let order = usize_array_param(params, "order")?;
                let parent = entry_mut(doc, kind, "nodes", usize_param(params, "parent")?)?;
                reorder_index_list(ensure_array(parent, "children"), kind, &order)?;
            }
            "move-scene-root-node" => {
                let node = usize_param(params, "node")?;
                let position = usize_param(params, "position")?;
                let scene = entry_mut(doc, kind, "scenes", usize_param(params, "scene")?)?;
                move_in_index_list(ensure_array(scene, "nodes"), kind, node, position)?;
            }
            "reorder-scene-root-nodes" => {
                let order = usize_array_param(params, "order")?;
                let scene = entry_mut(doc, kind, "scenes", usize_param(params, "scene")?)?;
                reorder_index_list(ensure_array(scene, "nodes"), kind, &order)?;
            }
            "move-node-parent" => move_node_parent(doc, usize_param(params, "parent")?, usize_param(params, "child")?, usize_param(params, "position")?)?,
            "change-node-name" | "change-scene-name" | "change-mesh-name" => {
                let (family, field) = match kind {
                    "change-node-name" => ("nodes", "node"),
                    "change-scene-name" => ("scenes", "scene"),
                    _ => ("meshes", "mesh"),
                };
                let value = nullable_str_param(params, "value")?;
                let item = entry_mut(doc, kind, family, usize_param(params, field)?)?;
                if kind == "change-node-name" && obj_get(item, "name").and_then(json::JsonValue::as_str).map(str::to_string) == value {
                    return Err(format!("{kind}: name already has the requested presence and value"));
                }
                set_or_remove(item, "name", value.map(json::JsonValue::from));
            }
            "change-node-extra-data" | "change-node-extension-data" | "change-scene-extra-data" | "change-scene-extension-data" | "change-mesh-extra-data" | "change-mesh-extension-data" => {
                let (family, field) = match kind.split('-').nth(1) {
                    Some("node") => ("nodes", "node"),
                    Some("scene") => ("scenes", "scene"),
                    _ => ("meshes", "mesh"),
                };
                let key = if kind.ends_with("extra-data") { "extras" } else { "extensions" };
                let value = presence_param(params)?;
                let item = entry_mut(doc, kind, family, usize_param(params, field)?)?;
                if kind == "change-node-extra-data" && obj_get(item, key).cloned() == value {
                    return Err(format!("{kind}: extras already has the requested presence and value"));
                }
                set_or_remove(item, key, value);
            }
            "change-primitive-extra-data" | "change-primitive-extension-data" => {
                let key = if kind.ends_with("extra-data") { "extras" } else { "extensions" };
                let value = presence_param(params)?;
                set_or_remove(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, key, value);
            }
            "change-node-transform" => change_node_transform(doc, usize_param(params, "node")?, object_param(params, "transform")?)?,
            "change-node-morph-weights" => {
                let node = usize_param(params, "node")?;
                let weights = f64_array_param(params, "weights")?;
                if weights.iter().any(|value| !value.is_finite()) {
                    return Err(format!("{kind}: weights must be finite"));
                }
                let mesh = arr(doc, "nodes").get(node).ok_or(format!("{kind}: node {node} out of range"))?.clone();
                match obj_get(&mesh, "mesh").and_then(json::JsonValue::as_usize) {
                    None if !weights.is_empty() => return Err(format!("{kind}: morph weights require a mesh")),
                    Some(mesh_index) if !mesh_target_arity_is(doc, mesh_index, weights.len()) => return Err(format!("{kind}: weights must match primitive target count")),
                    _ => {}
                }
                set_or_remove(entry_mut(doc, kind, "nodes", node)?, "weights", weights_value(&weights));
            }
            "change-mesh-morph-weights" => {
                let mesh = usize_param(params, "mesh")?;
                let weights = f64_array_param(params, "weights")?;
                if weights.iter().any(|value| !value.is_finite()) || !mesh_target_arity_is(doc, mesh, weights.len()) {
                    return Err(format!("{kind}: weights must be finite and match every primitive target list"));
                }
                set_or_remove(entry_mut(doc, kind, "meshes", mesh)?, "weights", weights_value(&weights));
            }
            "create-node" => family_insert(doc, kind, "nodes", usize_param(params, "position")?, json::JsonValue::Object(json::object::Object::new()))?,
            "delete-node" => family_delete(doc, kind, "nodes", usize_param(params, "index")?)?,
            "move-node" => family_move(doc, kind, "nodes", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-nodes" => family_reorder(doc, kind, "nodes", &usize_array_param(params, "order")?)?,
            "create-scene" => create_scene(doc, usize_param(params, "position")?)?,
            "delete-scene" => family_delete(doc, kind, "scenes", usize_param(params, "index")?)?,
            "move-scene" => family_move(doc, kind, "scenes", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-scenes" => family_reorder(doc, kind, "scenes", &usize_array_param(params, "order")?)?,
            "create-mesh" => {
                let mut mesh = json::object::Object::new();
                mesh.insert("primitives", json::JsonValue::Array(Vec::new()));
                family_insert(doc, kind, "meshes", usize_param(params, "position")?, json::JsonValue::Object(mesh))?;
            }
            "delete-mesh" => family_delete(doc, kind, "meshes", usize_param(params, "index")?)?,
            "move-mesh" => family_move(doc, kind, "meshes", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-meshs" => family_reorder(doc, kind, "meshes", &usize_array_param(params, "order")?)?,
            "create-accessor" => {
                let mut accessor = json::object::Object::new();
                accessor.insert("componentType", json::JsonValue::from(usize_param(params, "componentType")?));
                accessor.insert("count", json::JsonValue::from(usize_param(params, "count")?));
                accessor.insert("type", json::JsonValue::from(str_param(params, "kind")?));
                family_insert(doc, kind, "accessors", usize_param(params, "position")?, json::JsonValue::Object(accessor))?;
            }
            "delete-accessor" => family_delete(doc, kind, "accessors", usize_param(params, "index")?)?,
            "move-accessor" => family_move(doc, kind, "accessors", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-accessors" => family_reorder(doc, kind, "accessors", &usize_array_param(params, "order")?)?,
            "create-buffer-view" => {
                let buffer = usize_param(params, "buffer")?;
                if buffer >= top_level_len(doc, "buffers") {
                    return Err(format!("{kind}: backing buffer {buffer} missing"));
                }
                let mut view = json::object::Object::new();
                view.insert("buffer", json::JsonValue::from(buffer));
                view.insert("byteOffset", json::JsonValue::from(usize_param(params, "byteOffset")?));
                view.insert("byteLength", json::JsonValue::from(usize_param(params, "byteLength")?));
                family_insert(doc, kind, "bufferViews", usize_param(params, "position")?, json::JsonValue::Object(view))?;
            }
            "delete-buffer-view" => family_delete(doc, kind, "bufferViews", usize_param(params, "index")?)?,
            "move-buffer-view" => family_move(doc, kind, "bufferViews", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-buffer-views" => family_reorder(doc, kind, "bufferViews", &usize_array_param(params, "order")?)?,
            "create-buffer" => {
                let bytes: Vec<u8> = usize_array_param(params, "bytes")?.into_iter().map(|byte| u8::try_from(byte).map_err(|_| format!("{kind}: byte {byte} out of range"))).collect::<Result<_, _>>()?;
                let mut buffer = json::object::Object::new();
                buffer.insert("byteLength", json::JsonValue::from(bytes.len()));
                buffer.insert("uri", json::JsonValue::from(format!("data:application/octet-stream;base64,{}", base64_encode(&bytes))));
                family_insert(doc, kind, "buffers", usize_param(params, "position")?, json::JsonValue::Object(buffer))?;
            }
            "delete-buffer" => family_delete(doc, kind, "buffers", usize_param(params, "index")?)?,
            "move-buffer" => family_move(doc, kind, "buffers", usize_param(params, "index")?, usize_param(params, "position")?)?,
            "reorder-buffers" => family_reorder(doc, kind, "buffers", &usize_array_param(params, "order")?)?,
            "create-primitive" => {
                let position = usize_param(params, "position")?;
                let mesh = entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?;
                let primitives = ensure_array(mesh, "primitives");
                if position > primitives.len() {
                    return Err(format!("{kind}: position {position} out of range"));
                }
                let mut primitive = json::object::Object::new();
                primitive.insert("attributes", json::JsonValue::Object(json::object::Object::new()));
                primitives.insert(position, json::JsonValue::Object(primitive));
            }
            "delete-primitive" => {
                let primitive = usize_param(params, "primitive")?;
                let mesh = entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?;
                let primitives = ensure_array(mesh, "primitives");
                if primitive >= primitives.len() {
                    return Err(format!("{kind}: primitive {primitive} out of range"));
                }
                primitives.remove(primitive);
            }
            "move-primitive" => {
                let (primitive, position) = (usize_param(params, "primitive")?, usize_param(params, "position")?);
                move_nested(ensure_array(entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?, "primitives"), kind, primitive, position)?;
            }
            "reorder-primitives" => {
                let order = usize_array_param(params, "order")?;
                reorder_nested(ensure_array(entry_mut(doc, kind, "meshes", usize_param(params, "mesh")?)?, "primitives"), kind, &order, true)?;
            }
            "change-primitive-topology-mode" => {
                let mode = usize_param(params, "mode")?;
                if mode > 6 {
                    return Err(format!("{kind}: mode must be in the glTF topology domain"));
                }
                obj_set(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "mode", json::JsonValue::from(mode));
            }
            "bind-primitive-material" => {
                let material = usize_param(params, "material")?;
                if material >= top_level_len(doc, "materials") {
                    return Err(format!("{kind}: material {material} out of range"));
                }
                obj_set(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "material", json::JsonValue::from(material));
            }
            "unbind-primitive-material" | "unbind-primitive-indices" => {
                let field = if kind == "unbind-primitive-material" { "material" } else { "indices" };
                let primitive = primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?;
                if obj_get(primitive, field).is_none() {
                    return Err(format!("{kind}: primitive has no {field}"));
                }
                remove_key(primitive, field);
            }
            "bind-primitive-indices" => {
                let accessor = usize_param(params, "accessor")?;
                let accessor_value = arr(doc, "accessors").get(accessor).cloned().ok_or(format!("{kind}: accessor {accessor} out of range"))?;
                if obj_get(&accessor_value, "type").and_then(json::JsonValue::as_str) != Some("SCALAR") || obj_get(&accessor_value, "componentType").and_then(json::JsonValue::as_usize) == Some(5126) {
                    return Err(format!("{kind}: indices require a scalar integer accessor"));
                }
                obj_set(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "indices", json::JsonValue::from(accessor));
            }
            "bind-primitive-attribute" => {
                let accessor = usize_param(params, "accessor")?;
                if accessor >= top_level_len(doc, "accessors") {
                    return Err(format!("{kind}: accessor {accessor} out of range"));
                }
                let semantic = str_param(params, "semantic")?;
                let primitive = primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?;
                if obj_get(primitive, "attributes").is_none() {
                    obj_set(primitive, "attributes", json::JsonValue::Object(json::object::Object::new()));
                }
                let json::JsonValue::Object(object) = primitive else { return Err(format!("{kind}: primitive is not an object")) };
                bind_semantic(object.get_mut("attributes").expect("attributes ensured"), kind, &semantic, accessor)?;
            }
            "unbind-primitive-attribute" | "move-primitive-attribute" | "reorder-primitive-attributes" => {
                let primitive = primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?;
                let json::JsonValue::Object(object) = primitive else { return Err(format!("{kind}: primitive is not an object")) };
                let attributes = object.get_mut("attributes").ok_or(format!("{kind}: primitive has no attributes"))?;
                match kind {
                    "unbind-primitive-attribute" => unbind_semantic(attributes, kind, &str_param(params, "semantic")?)?,
                    "move-primitive-attribute" => move_semantic(attributes, kind, &str_param(params, "semantic")?, usize_param(params, "position")?)?,
                    _ => reorder_semantics(attributes, kind, &string_array_param(params, "order")?)?,
                }
            }
            "create-morph-target" => {
                let position = usize_param(params, "position")?;
                let mesh = usize_param(params, "mesh")?;
                let primitive_index = usize_param(params, "primitive")?;
                if arr(doc, "meshes").get(mesh).map(|value| arr(value, "primitives").len()) != Some(1) {
                    return Err(format!("{kind}: all primitive target counts must remain coherent"));
                }
                let targets = ensure_array(primitive_mut(doc, kind, mesh, primitive_index)?, "targets");
                if position > targets.len() {
                    return Err(format!("{kind}: position {position} out of range"));
                }
                targets.insert(position, json::JsonValue::Object(json::object::Object::new()));
            }
            "delete-morph-target" => {
                let (mesh, primitive_index, target) = (usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?);
                let mesh_value = arr(doc, "meshes").get(mesh).cloned().ok_or(format!("{kind}: mesh {mesh} out of range"))?;
                if arr(&mesh_value, "primitives").len() != 1 || !arr(&mesh_value, "weights").is_empty() {
                    return Err(format!("{kind}: target deletion would violate mesh target-count coherence"));
                }
                let targets = ensure_array(primitive_mut(doc, kind, mesh, primitive_index)?, "targets");
                if target >= targets.len() {
                    return Err(format!("{kind}: target {target} out of range"));
                }
                targets.remove(target);
            }
            "move-morph-target" => {
                let (target, position) = (usize_param(params, "target")?, usize_param(params, "position")?);
                move_nested(ensure_array(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "targets"), kind, target, position)?;
            }
            "reorder-morph-targets" => {
                let order = usize_array_param(params, "order")?;
                reorder_nested(ensure_array(primitive_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?)?, "targets"), kind, &order, false)?;
            }
            "bind-morph-target-attribute" => {
                let accessor = usize_param(params, "accessor")?;
                if accessor >= top_level_len(doc, "accessors") {
                    return Err(format!("{kind}: accessor {accessor} out of range"));
                }
                let semantic = str_param(params, "semantic")?;
                bind_semantic(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &semantic, accessor)?;
            }
            "unbind-morph-target-attribute" => {
                let semantic = str_param(params, "semantic")?;
                unbind_semantic(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &semantic)?;
            }
            "move-morph-target-attribute" => {
                let (semantic, position) = (str_param(params, "semantic")?, usize_param(params, "position")?);
                move_semantic(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &semantic, position)?;
            }
            "reorder-morph-target-attributes" => {
                let order = string_array_param(params, "order")?;
                reorder_semantics(target_mut(doc, kind, usize_param(params, "mesh")?, usize_param(params, "primitive")?, usize_param(params, "target")?)?, kind, &order)?;
            }
            _ => unreachable!("dispatched only for structure kinds"),
        }
        Ok(())
    };
    STRUCTURE_KINDS.contains(&kind).then(run)
}

/// 🧾️ The 76 kinds [`apply_structure`] performs.
#[cfg(feature = "oracles")]
const STRUCTURE_KINDS: &[&str] = &[
    "bind-default-scene", "bind-node-camera", "bind-node-child", "bind-node-mesh", "bind-node-skin", "bind-scene-root-node", "change-node-extension-data", "change-node-extra-data", "change-node-morph-weights", "change-node-name", "change-node-transform", "change-scene-extension-data", "change-scene-extra-data", "change-scene-name", "create-node", "create-scene", "delete-node", "delete-scene", "move-node", "move-node-child", "move-node-parent", "move-scene", "move-scene-root-node", "reorder-node-children", "reorder-nodes", "reorder-scene-root-nodes", "reorder-scenes", "unbind-default-scene", "unbind-node-camera", "unbind-node-child", "unbind-node-mesh", "unbind-node-skin", "unbind-scene-root-node",
    "create-buffer", "create-buffer-view", "delete-buffer", "delete-buffer-view", "move-buffer", "move-buffer-view", "reorder-buffer-views", "reorder-buffers",
    "bind-morph-target-attribute", "bind-primitive-attribute", "bind-primitive-indices", "bind-primitive-material", "change-mesh-extension-data", "change-mesh-extra-data", "change-mesh-morph-weights", "change-mesh-name", "change-primitive-extension-data", "change-primitive-extra-data", "change-primitive-topology-mode", "create-accessor", "create-mesh", "create-morph-target", "create-primitive", "delete-accessor", "delete-mesh", "delete-morph-target", "delete-primitive", "move-accessor", "move-mesh", "move-morph-target", "move-morph-target-attribute", "move-primitive", "move-primitive-attribute", "reorder-accessors", "reorder-meshs", "reorder-morph-target-attributes", "reorder-morph-targets", "reorder-primitive-attributes", "reorder-primitives", "unbind-morph-target-attribute", "unbind-primitive-attribute", "unbind-primitive-indices", "unbind-primitive-material",
];

/// ↩️ The inverse of a kind whose own payload cannot carry what it removed (every delete, a node
/// weight override, a new morph target): restores the named top-level members straight off the
/// ORIGINAL document, removing any member the original lacked — `buffers` also restores the
/// binary chunk, which a buffer change can shift.
#[cfg(feature = "oracles")]
pub fn restore_members(mutated: &[u8], original: &[u8], members: &[String]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, source_bin) = read_glb(original)?;
    for member in members {
        set_or_remove(&mut doc, member, obj_get(&source, member).cloned());
    }
    let bin = if members.iter().any(|member| member == "buffers") { source_bin } else { bin };
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn restore_members(_mutated: &[u8], _original: &[u8], _members: &[String]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️StructureKinds

//#region 🔖️Dispatch
#[cfg(feature = "oracles")]
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
#[cfg(feature = "oracles")]
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
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The oracle's own decode/re-encode, entirely through the independent GLB codec above — the
/// identity-round-trip scenario's oracle side.
#[cfg(feature = "oracles")]
pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    let (doc, bin) = read_glb(input)?;
    Ok(write_glb(&doc, bin.as_deref()))
}
#[cfg(not(feature = "oracles"))]
pub fn round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Projection
/// 🧾️ One tree object as a fixed member list: every listed member present, a missing one at its
/// declared default, so a writer that spells a default out and one that omits it project the same.
#[cfg(feature = "oracles")]
fn normalized(item: &json::JsonValue, members: &[(&str, Json)]) -> Json {
    Json::Object(members.iter().map(|(key, default)| ((*key).to_string(), obj_get(item, key).map(to_host_json).unwrap_or_else(|| default.clone()))).collect())
}

/// 🔤️ A semantic → accessor map as ordered `[semantic, accessor]` pairs, so key order is observed.
#[cfg(feature = "oracles")]
fn semantic_pairs(map: Option<&json::JsonValue>) -> Json {
    match map {
        Some(json::JsonValue::Object(object)) => Json::Array(object.iter().map(|(key, value)| Json::Array(vec![Json::String(key.to_string()), to_host_json(value)])).collect()),
        _ => Json::Array(Vec::new()),
    }
}

/// 💾️ One buffer's bytes as lowercase hex: a `data:` URI decoded, the binary chunk for a
/// URI-less first buffer, `null` for an external URI this reader does not follow.
#[cfg(feature = "oracles")]
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
#[cfg(feature = "oracles")]
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
    // 🎥️ `document/cameras` and `document/skins` — projected structurally via [`to_host_json`]
    // rather than a hand-picked field list, since `create-camera`'s own `projection` param is
    // carried the same way (see `create_camera`/[`from_host_json`]) and this keeps the two
    // directions symmetric.
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
    // 🧩️ `document/asset`, `document/extensionsUsed`/`extensionsRequired` and
    // `document/extensions`/`extras` (shard G2, this ticket) — projected the same structural way as
    // `cameras`/`skins`/`animations` above, the entire normative surface the 14 `🪪️asset` kinds
    // touch.
    let asset = obj_get(&doc, "asset").map(to_host_json).unwrap_or(Json::Null);
    let extensions_used: Vec<Json> = arr(&doc, "extensionsUsed").iter().filter_map(json::JsonValue::as_str).map(|value| Json::String(value.to_string())).collect();
    let extensions_required: Vec<Json> = arr(&doc, "extensionsRequired").iter().filter_map(json::JsonValue::as_str).map(|value| Json::String(value.to_string())).collect();
    let document_extensions = obj_get(&doc, "extensions").map(to_host_json).unwrap_or(Json::Null);
    let document_extras = obj_get(&doc, "extras").map(to_host_json).unwrap_or(Json::Null);
    // 🎨️ `document/{materials,textures,images,samplers}` (shard G4, this ticket) — `materialsFull`
    // is the FULL structural material dump (`pbrMetallicRoughness`/`{normal,occlusion,emissive}
    // Texture` included), projected the same structural way as `cameras`/`skins`/`animations`
    // above, since `create`/`delete`/`move`/`reorder-materials` and every `*-texture` kind's own
    // cascading clear are only observable through the WHOLE material object, not the `alphaMode`/
    // `doubleSided` pair `materials` above already carries for the artifact-root case's own 2 kinds.
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
#[cfg(not(feature = "oracles"))]
pub fn project_gltf(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
