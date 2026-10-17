//! 🧰️ The independent GLB container codec, the JSON tree and parameter helpers and the host JSON bridge.

use super::*;

//#region 🔖️Container
/// 🧊️ glTF binary container magic (`glTF`, little-endian).
const GLB_MAGIC: u32 = 0x4654_6C67;
/// 🧊️ `JSON` chunk type tag, little-endian.
const CHUNK_JSON: u32 = 0x4E4F_534A;
/// 🧊️ `BIN\0` chunk type tag, little-endian.
const CHUNK_BIN: u32 = 0x004E_4942;

/// 🔢️ Little-endian `u32` at `offset`, independent of any shared byte-reading helper.
fn read_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    bytes.get(offset..offset + 4).map(|slice| u32::from_le_bytes(slice.try_into().expect("4-byte slice"))).ok_or_else(|| "unexpected end of GLB header".to_string())
}

/// 📥️ Independent GLB/`.gltf` read: 12-byte header plus `JSON`/`BIN\0` chunk iteration for a real
/// binary container, or a bare UTF-8 JSON parse for plain-text `.gltf` input — its own loop, its own
/// chunk bounds checks, never a call into `decode_glb`/`parse_gltf_document`.
pub(super) fn read_glb(input: &[u8]) -> Result<(json::JsonValue, Option<Vec<u8>>), String> {
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
pub(super) fn write_glb(doc: &json::JsonValue, bin: Option<&[u8]>) -> Vec<u8> {
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
pub(super) fn obj_get<'a>(value: &'a json::JsonValue, key: &str) -> Option<&'a json::JsonValue> {
    match value {
        json::JsonValue::Object(object) => object.get(key),
        _ => None,
    }
}

/// 🔧️ Object member upsert; a no-op on a non-object value.
pub(super) fn obj_set(value: &mut json::JsonValue, key: &str, item: json::JsonValue) {
    if let json::JsonValue::Object(object) = value {
        object.insert(key, item);
    }
}

/// 🔎️ A cloned array member, or an empty `Vec` for a non-array or an absent key.
pub(super) fn arr(value: &json::JsonValue, key: &str) -> Vec<json::JsonValue> {
    match obj_get(value, key) {
        Some(json::JsonValue::Array(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// 🔧️ The array member's own mutable `Vec`, or `None` when `key` is not an array on this object.
pub(super) fn arr_mut<'a>(value: &'a mut json::JsonValue, key: &str) -> Option<&'a mut Vec<json::JsonValue>> {
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
pub(super) fn ensure_array<'a>(value: &'a mut json::JsonValue, key: &str) -> &'a mut Vec<json::JsonValue> {
    if !matches!(obj_get(value, key), Some(json::JsonValue::Array(_))) {
        obj_set(value, key, json::JsonValue::Array(Vec::new()));
    }
    arr_mut(value, key).expect("array member just ensured")
}

/// 🔎️ `document/nodes`, `document/scenes` and `document/materials` lengths, addressed by name so
/// every kind's bounds check reads identically to its production counterpart's `checked_index`.
pub(super) fn top_level_len(doc: &json::JsonValue, key: &str) -> usize {
    arr(doc, key).len()
}

/// 🔎️ `document/nodes/{index}/children`, resolved indices only (a non-numeric entry is skipped
/// rather than treated as a fatal error, matching how index resolution reads a real document).
pub(super) fn node_children(doc: &json::JsonValue, index: usize) -> Vec<usize> {
    arr(doc, "nodes").get(index).map(|node| arr(node, "children").iter().filter_map(json::JsonValue::as_usize).collect()).unwrap_or_default()
}

/// 🔎️ `document/scenes/{index}/nodes`, resolved indices only.
pub(super) fn scene_nodes(doc: &json::JsonValue, index: usize) -> Vec<usize> {
    arr(doc, "scenes").get(index).map(|scene| arr(scene, "nodes").iter().filter_map(json::JsonValue::as_usize).collect()).unwrap_or_default()
}

/// 🔎️ `document/scene`, the default scene index, `None` when the document declares none.
pub(super) fn default_scene_index(doc: &json::JsonValue) -> Option<usize> {
    obj_get(doc, "scene").and_then(json::JsonValue::as_usize)
}
//#endregion 🔖️Tree

//#region 🔖️Params
pub(super) fn usize_param(params: &Json, key: &str) -> Result<usize, String> {
    match params.get(key) {
        Some(Json::Number(number)) => Ok(*number as usize),
        _ => Err(format!("missing or non-numeric `{key}`")),
    }
}

pub(super) fn str_param(params: &Json, key: &str) -> Result<String, String> {
    match params.get(key) {
        Some(Json::String(value)) => Ok(value.clone()),
        _ => Err(format!("missing or non-string `{key}`")),
    }
}

pub(super) fn bool_param(params: &Json, key: &str) -> Result<bool, String> {
    match params.get(key) {
        Some(Json::Bool(value)) => Ok(*value),
        _ => Err(format!("missing or non-boolean `{key}`")),
    }
}

/// 🔎️ `usize` elements of an array param — `reorder-cameras`'s own `order` shape.
pub(super) fn usize_array_param(params: &Json, key: &str) -> Result<Vec<usize>, String> {
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
pub(super) fn object_param<'a>(params: &'a Json, key: &str) -> Result<&'a Json, String> {
    match params.get(key) {
        Some(value @ Json::Object(_)) => Ok(value),
        _ => Err(format!("missing or non-object `{key}`")),
    }
}

/// 🔎️ An optional string param (shard G2) — `None` for an absent or non-string key, matching an
/// `Option<String>` payload field like `generator`/`copyright`/`minVersion`.
pub(super) fn optional_str_param(params: &Json, key: &str) -> Option<String> {
    match params.get(key) {
        Some(Json::String(value)) => Some(value.clone()),
        _ => None,
    }
}

/// 🔎️ An optional object param — `None` for an absent, `null`, or non-object key, matching an
/// `Option<GltfJson>` payload field like `change-{asset,document}-{extension,extra}-data`'s `data`.
pub(super) fn optional_object_param<'a>(params: &'a Json, key: &str) -> Option<&'a Json> {
    match params.get(key) {
        Some(value @ Json::Object(_)) => Some(value),
        _ => None,
    }
}

/// 🔎️ String elements of an array param — `reorder-{required,used}-extensions`'s own `order`.
pub(super) fn string_array_param(params: &Json, key: &str) -> Result<Vec<String>, String> {
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
pub(super) fn from_host_json(value: &Json) -> json::JsonValue {
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
pub(super) fn to_host_json(value: &json::JsonValue) -> Json {
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
