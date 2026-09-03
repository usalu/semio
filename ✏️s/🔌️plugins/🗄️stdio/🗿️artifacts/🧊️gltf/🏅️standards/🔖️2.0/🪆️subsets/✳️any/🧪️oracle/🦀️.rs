//! 🔮️ Mutation oracle for this subset — every mutation kind `GLTF_MUTATION_LEAF_DESCRIPTORS`
//! (`../🧬️schema/🧬️mutations/🦀️.rs`) currently registers, performed by an independent
//! GLB container codec plus generic JSON manipulation, so the subject's own mutation has a real
//! second producer to be compared against instead of being checked against its own reading.
//!
//! Unlike every other stdio artifact, glTF has no `pub enum GltfMutation`: its vocabulary is a
//! descriptor table (`GltfMutationLeafDescriptor`), and only 7 of the 120 real leaf directories on
//! disk are both mounted as production modules AND listed in the descriptor assembly today — the
//! ones this catalog declares. The other 113 are real, complete (`🧬️operation`/`🔺️diff`/`↩️inverse`
//! files, no stubs) but unmounted, which is `🦀️.rs`-owned wiring out of this ticket's scope.
//!
//! **Why `json` (json-rust), not the `gltf` crate (1.4.1, MIT):** `gltf` is a credible, actively
//! maintained reader and IS already production-reachable in this repository — but confirmed
//! genuinely independent of this subset's own codec first. `crate::artifacts::gltf::schema::
//! snapshot::🦀️.rs`'s `GltfSnapshot`/`GltfDocument`/`GltfJson` never names `gltf::`
//! anywhere (no `impl From<gltf::…>`, no import), and `decode_glb`/`encode_glb`/`parse_gltf_document`
//! (`../🚪️io/🦀️.rs`) are hand-rolled over `serde_json` alone. Every real `gltf::` call site
//! in this repository lives in `🧰️framework/🔨️modules/🔺️mesh-engine/🦀️.rs`
//! (`mesh_to_glb`/`mesh_from_glb`/`GlbExporter`/`GlbImporter`, byte-in/byte-out, no `gltf::` type
//! crosses that boundary) — reached from `semio-s-plugin-stdio` only through the unrelated BREP/DWG
//! mesh-IO codecs, never from this artifact's own tree. That is a small, nameable production surface
//! exactly like the `image`/`png` `productionDebt` precedent, so registering `gltf` here WOULD have
//! been legitimate. It was not registered anyway: linking it needs a `Cargo.toml` edit this ticket
//! must not make itself, and `json` 0.12 is already linked (`oracles = […, "dep:json", …]`,
//! `../../../../../../🧪️oracle/📦️packages/🦀️rust/Cargo.toml`) and already proven independent for
//! `stdio.json`'s own oracle (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/
//! 🪆️subsets/✳️any/🦀️oracle.rs`) — it appears nowhere in this repository's production
//! dependency graph. This subset's own production codec also uses `serde_json` (see above), which
//! rules `serde_json` out as an oracle for the same reason it was ruled out for `stdio.json`.
//!
//! `json` is domain-BLIND (no glTF schema awareness at all, unlike `gltf`), so all seven mutations'
//! actual semantics — index bounds, cycle rejection, duplicate-root rejection, alphaMode enum
//! validity, `document/scene` remapping — are reimplemented from scratch below, independently of
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
/// payload, `../🧬️schema/🧬️mutations/🌱️🧥️create-skin/🦀️.rs`'s `GltfCreateSkinPayload { position }`,
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
/// out-of-range position, exactly as `../🧬️schema/🧬️mutations/🌱️🎥️create-camera/🦀️.rs`'s own
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
/// reference that pointed at it, exactly as `../🧬️schema/🧬️mutations/🗑️🎥️delete-camera/🦀️.rs`'s
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
/// `../🧬️schema/🧬️mutations/🚚️🎥️move-camera/🦀️.rs`'s own `validate` does (both indices in range,
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
/// actually move at least one, exactly as `../🧬️schema/🧬️mutations/🔀️🎥️reorder-cameras/🦀️.rs`'s
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
/// `../🧬️schema/🧬️mutations/🌱️🧥️create-skin/🦀️.rs`'s own `validate` does.
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
/// that pointed at it, exactly as `../🧬️schema/🧬️mutations/🗑️🧥️delete-skin/🦀️.rs`'s own
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
/// `../🧬️schema/🧬️mutations/🚚️🧥️move-skin/🦀️.rs`'s own `validate` does.
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
/// actually move at least one, exactly as `../🧬️schema/🧬️mutations/🔀️🧥️reorder-skins/🦀️.rs`'s own
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
/// position, exactly as `../🧬️schema/🧬️mutations/🌱️🎞️create-animation/🦀️.rs`'s own `validate`
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
/// `../🧬️schema/🧬️mutations/🗑️🎞️delete-animation/🦀️.rs`'s own `validate`/`animations_op` do.
#[cfg(feature = "oracles")]
fn delete_animation(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "animations") {
        return Err(format!("delete-animation: index {index} out of range"));
    }
    ensure_array(doc, "animations").remove(index);
    Ok(())
}

/// 🦠️ `move-animation` — relocates a real animation to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🚚️🎞️move-animation/🦀️.rs`'s own `validate` does.
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
/// actually move at least one, exactly as `../🧬️schema/🧬️mutations/🔀️🎞️reorder-animations/🦀️.rs`'s
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
/// `../🧬️schema/🧬️mutations/✅️🧩️add-required-extension/🦀️.rs`'s(/`📣️🧩️add-used-extension`'s) own
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
/// extension name, exactly as the sibling `🚫️🧩️remove-required-extension`/`🔙️🧩️remove-used-extension`
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
/// `🚚️🧩️move-required-extension`/`🚚️🧩️move-used-extension` leaves' own `validate` does.
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
/// sibling `🔀️🧩️reorder-required-extensions`/`🔀️🧩️reorder-used-extensions` leaves' own `validate`
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
/// `../🧬️schema/🧬️mutations/✏️📦️change-asset-descriptive-metadata/🦀️.rs`'s own `apply` writes
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
/// block, exactly as `../🧬️schema/🧬️mutations/✏️📦️change-asset-version/🦀️.rs`'s own `validate`
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
/// `../🧬️schema/🧬️mutations/✏️📦️change-asset-extension-data/🦀️.rs`'s own `validate` does
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
/// root), exactly as `../🧬️schema/🧬️mutations/✏️📄️change-document-extension-data/🦀️.rs`'s own
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

/// 🎨️ `✳️material` (shard G4, this ticket) — the 4 families `document/materials`,
/// `document/textures`, `document/images`, `document/samplers` each own (§5.20/§5.31/§5.24/§5.29),
/// `create`/`delete`/`move`/`reorder` per family. Read `top_level_collections.rs`'s own `repair`
/// match before writing anything: `materials`/`images`/`samplers` are each a SINGLE simple
/// `Option<usize>` reference site (`meshes[].primitives[].material`, `textures[].source`,
/// `textures[].sampler`), structurally identical in difficulty to `✳️camera`/`✳️skin`; `textures` is
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
/// position, exactly as `../🧬️schema/🧬️mutations/🌱️💎️create-material/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🗑️💎️delete-material/🦀️.rs`'s own `validate`/`materials_op` do.
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
/// `../🧬️schema/🧬️mutations/🚚️💎️move-material/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🔀️💎️reorder-materials/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🌱️🖼️create-image/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🗑️🖼️delete-image/🦀️.rs`'s own `validate`/`images_op` do.
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
/// `../🧬️schema/🧬️mutations/🚚️🖼️move-image/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🔀️🖼️reorder-images/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🌱️🎛️create-sampler/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🗑️🎛️delete-sampler/🦀️.rs`'s own `validate`/`samplers_op` do.
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
/// `../🧬️schema/🧬️mutations/🚚️🎛️move-sampler/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🔀️🎛️reorder-samplers/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🌱️🎨️create-texture/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🗑️🎨️delete-texture/🦀️.rs`'s own `validate`/`textures_op` do.
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
/// `../🧬️schema/🧬️mutations/🚚️🎨️move-texture/🦀️.rs`'s own `validate` does.
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
/// `../🧬️schema/🧬️mutations/🔀️🎨️reorder-textures/🦀️.rs`'s own `validate` does.
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

//#region 🔖️Dispatch
#[cfg(feature = "oracles")]
fn apply(doc: &mut json::JsonValue, kind: &str, params: &Json) -> Result<(), String> {
    match kind {
        "bind-node-child" => bind_node_child(doc, usize_param(params, "parent")?, usize_param(params, "child")?, usize_param(params, "position")?),
        "unbind-node-child" => unbind_node_child(doc, usize_param(params, "parent")?, usize_param(params, "child")?),
        "bind-scene-root-node" => bind_scene_root_node(doc, usize_param(params, "scene")?, usize_param(params, "node")?, usize_param(params, "position")?),
        "unbind-scene-root-node" => unbind_scene_root_node(doc, usize_param(params, "scene")?, usize_param(params, "node")?),
        "change-material-alpha-mode" => change_material_alpha_mode(doc, usize_param(params, "material")?, &str_param(params, "alphaMode")?),
        "change-material-double-sided" => change_material_double_sided(doc, usize_param(params, "material")?, bool_param(params, "doubleSided")?),
        "create-scene" => create_scene(doc, usize_param(params, "position")?),
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
/// 👁️ Projects GLB/`.gltf` bytes with the INDEPENDENT container/JSON reader onto the
/// `semantic-gltf-v1` shape this case's oracle and subject are both compared through: the default
/// scene, every scene's root node list, every node's child list and every material's `alphaMode`/
/// `doubleSided` — the entire normative surface the seven registered kinds touch. Geometry, buffers,
/// accessors and every other document member are out of scope for the same reason `semantic-mesh-v1`
/// leaves precision and naming to writer freedom: no registered kind here observes them.
#[cfg(feature = "oracles")]
pub fn project_gltf(bytes: &[u8]) -> Result<Json, String> {
    let (doc, _bin) = read_glb(bytes)?;
    let default_scene = match default_scene_index(&doc) {
        Some(index) => Json::Number(index as f64),
        None => Json::Null,
    };
    let scenes: Vec<Json> = arr(&doc, "scenes")
        .iter()
        .map(|scene| Json::Object(vec![("nodes".to_string(), Json::Array(arr(scene, "nodes").iter().filter_map(json::JsonValue::as_usize).map(|index| Json::Number(index as f64)).collect()))]))
        .collect();
    let nodes: Vec<Json> = arr(&doc, "nodes")
        .iter()
        .map(|node| {
            let camera = obj_get(node, "camera").and_then(json::JsonValue::as_usize).map(|index| Json::Number(index as f64)).unwrap_or(Json::Null);
            let skin = obj_get(node, "skin").and_then(json::JsonValue::as_usize).map(|index| Json::Number(index as f64)).unwrap_or(Json::Null);
            Json::Object(vec![
                ("children".to_string(), Json::Array(arr(node, "children").iter().filter_map(json::JsonValue::as_usize).map(|index| Json::Number(index as f64)).collect())),
                ("camera".to_string(), camera),
                ("skin".to_string(), skin),
            ])
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
    let animations: Vec<Json> = arr(&doc, "animations").iter().map(to_host_json).collect();
    // 🧩️ `document/asset`, `document/extensionsUsed`/`extensionsRequired` and
    // `document/extensions`/`extras` (shard G2, this ticket) — projected the same structural way as
    // `cameras`/`skins`/`animations` above, the entire normative surface the 14 `✳️asset` kinds
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
    ]))
}
#[cfg(not(feature = "oracles"))]
pub fn project_gltf(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Projection

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
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
}
//#endregion 🧪️Tests
