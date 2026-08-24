//! 🔮️ Mutation oracle for this subset — every mutation kind `GLTF_MUTATION_LEAF_DESCRIPTORS`
//! (`../🧬️schema/🧬️mutations/🦀️component.rs`) currently registers, performed by an independent
//! GLB container codec plus generic JSON manipulation, so the subject's own mutation has a real
//! second producer to be compared against instead of being checked against its own reading.
//!
//! Unlike every other stdio artifact, glTF has no `pub enum GltfMutation`: its vocabulary is a
//! descriptor table (`GltfMutationLeafDescriptor`), and only 7 of the 120 real leaf directories on
//! disk are both mounted as production modules AND listed in the descriptor assembly today — the
//! ones this catalog declares. The other 113 are real, complete (`🦠️mutation`/`🔺️diff`/`↩️inverse`
//! files, no stubs) but unmounted, which is `📦️glue.rs`-owned wiring out of this ticket's scope.
//!
//! **Why `json` (json-rust), not the `gltf` crate (1.4.1, MIT):** `gltf` is a credible, actively
//! maintained reader and IS already production-reachable in this repository — but confirmed
//! genuinely independent of this subset's own codec first. `crate::artifacts::gltf::schema::
//! snapshot::🦀️component.rs`'s `GltfSnapshot`/`GltfDocument`/`GltfJson` never names `gltf::`
//! anywhere (no `impl From<gltf::…>`, no import), and `decode_glb`/`encode_glb`/`parse_gltf_document`
//! (`../🚪️io/🦀️component.rs`) are hand-rolled over `serde_json` alone. Every real `gltf::` call site
//! in this repository lives in `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs`
//! (`mesh_to_glb`/`mesh_from_glb`/`GlbExporter`/`GlbImporter`, byte-in/byte-out, no `gltf::` type
//! crosses that boundary) — reached from `semio-s-plugin-stdio` only through the unrelated BREP/DWG
//! mesh-IO codecs, never from this artifact's own tree. That is a small, nameable production surface
//! exactly like the `image`/`png` `productionDebt` precedent, so registering `gltf` here WOULD have
//! been legitimate. It was not registered anyway: linking it needs a `Cargo.toml` edit this ticket
//! must not make itself, and `json` 0.12 is already linked (`oracles = […, "dep:json", …]`,
//! `../../../../../../🧪️oracle/📦️packages/🦀️rust/Cargo.toml`) and already proven independent for
//! `stdio.json`'s own oracle (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/
//! 🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`) — it appears nowhere in this repository's production
//! dependency graph. This subset's own production codec also uses `serde_json` (see above), which
//! rules `serde_json` out as an oracle for the same reason it was ruled out for `stdio.json`.
//!
//! `json` is domain-BLIND (no glTF schema awareness at all, unlike `gltf`), so all seven mutations'
//! actual semantics — index bounds, cycle rejection, duplicate-root rejection, alphaMode enum
//! validity, `document/scene` remapping — are reimplemented from scratch below, independently of
//! `../🧬️schema/🧬️mutations/*/🦠️mutation/🦀️component.rs`, operating on a hand-parsed GLB container
//! and a plain `json::JsonValue` document tree rather than this subset's own `GltfSnapshot`.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — `GLTF_MUTATION_LEAF_DESCRIPTORS`, the real vocabulary.

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
//#endregion 🔖️Params

//#region 🔖️Kinds
/// 🦠️ `bind-node-child` — validated exactly as `../🧬️schema/🧬️mutations/bind-node-child/
/// 🦠️mutation/🦀️component.rs` documents (index bounds, no self-parenting, no duplicate link, no
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
/// child elsewhere: `../🧬️schema/🧬️mutations/bind-scene-root-node/🦠️mutation/🦀️component.rs`'s own
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
/// 🦠️mutation/🦀️component.rs` does.
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
/// identity, exactly as `../🧬️schema/🧬️mutations/change-material-double-sided/🦠️mutation/
/// 🦀️component.rs` does.
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
/// 🦀️component.rs` performs, reimplemented independently against the parsed tree.
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
/// `delete-scene` leaf (see `../🧬️schema/🧬️mutations/create-scene/↩️inverse/🦀️component.rs`). Removes
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
        .map(|node| Json::Object(vec![("children".to_string(), Json::Array(arr(node, "children").iter().filter_map(json::JsonValue::as_usize).map(|index| Json::Number(index as f64)).collect()))]))
        .collect();
    let materials: Vec<Json> = (0..top_level_len(&doc, "materials"))
        .map(|index| Json::Object(vec![("alphaMode".to_string(), Json::String(material_alpha_mode(&doc, index))), ("doubleSided".to_string(), Json::Bool(material_double_sided(&doc, index)))]))
        .collect();
    Ok(Json::Object(vec![
        ("format".to_string(), Json::String("gltf".to_string())),
        ("defaultScene".to_string(), default_scene),
        ("sceneCount".to_string(), Json::Number(scenes.len() as f64)),
        ("scenes".to_string(), Json::Array(scenes)),
        ("nodeCount".to_string(), Json::Number(nodes.len() as f64)),
        ("nodes".to_string(), Json::Array(nodes)),
        ("materialCount".to_string(), Json::Number(materials.len() as f64)),
        ("materials".to_string(), Json::Array(materials)),
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

    #[test]
    fn glb_round_trip_preserves_the_bin_chunk() {
        let (doc, _) = read_glb(&glb(&base_document())).unwrap();
        let bin_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let with_bin = write_glb(&doc, Some(&bin_data));
        let (_, read_back) = read_glb(&with_bin).unwrap();
        assert_eq!(read_back, Some(bin_data));
    }
}
//#endregion 🧪️Tests
