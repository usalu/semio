//! 🎨️ Oracle kinds for the asset block, reference remapping and the material, image, sampler and texture collections.

use super::*;

//#region 🔖️Kinds
/// 🧩️ `document/asset`, read as a cloned object — mutated off-tree and written back with
/// [`obj_set`] rather than held as a live `&mut` borrow, so every asset-block function below shares
/// one simple shape instead of juggling nested-borrow lifetimes against `doc`.
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
fn set_optional_string(container: &mut json::JsonValue, key: &str, value: Option<&str>) {
    match value {
        Some(text) => obj_set(container, key, json::JsonValue::String(text.to_string())),
        None => *container = without_key(container, key),
    }
}

/// 🧩️ Sets or clears an optional object member in role — the same law as
/// [`set_optional_string`], for `change-{asset,document}-{extension,extra}-data`'s own
/// `Option<GltfJson>` payload shape.
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
pub(super) fn change_asset_descriptive_metadata(doc: &mut json::JsonValue, generator: Option<&str>, copyright: Option<&str>, min_version: Option<&str>) -> Result<(), String> {
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
pub(super) fn change_asset_version(doc: &mut json::JsonValue, version: &str) -> Result<(), String> {
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
pub(super) fn change_asset_extension_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
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
pub(super) fn change_asset_extra_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
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
pub(super) fn change_document_extension_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
    let next = data.map(from_host_json);
    if obj_get(doc, "extensions").cloned() == next {
        return Err("change-document-extension-data: value already has this value".to_string());
    }
    set_optional_json(doc, "extensions", next);
    Ok(())
}

/// 🧩️ `change-document-extra-data` — `document/extras`, the same shape as
/// [`change_document_extension_data`] for the sibling `extras` member.
pub(super) fn change_document_extra_data(doc: &mut json::JsonValue, data: Option<&Json>) -> Result<(), String> {
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
pub(super) fn create_material(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
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
pub(super) fn delete_material(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "materials") {
        return Err(format!("delete-material: index {index} out of range"));
    }
    apply_primitive_material_ref_change(doc, &IndexChange::Delete(index));
    ensure_array(doc, "materials").remove(index);
    Ok(())
}

/// 🦠️ `move-material` — relocates a real material to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/💎️material/🚚️move/🦀️.rs`'s own `validate` does.
pub(super) fn move_material(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_materials(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
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

/// 🦠️ `create-image` — inserts one canonical empty image (`{}`) at `position` into
/// `document/images`; rejects an out-of-range position, exactly as
/// `../🧬️schema/🧬️mutations/🖼️image/🌱️create/🦀️.rs`'s own `validate` does.
pub(super) fn create_image(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
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
pub(super) fn delete_image(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "images") {
        return Err(format!("delete-image: index {index} out of range"));
    }
    apply_ref_change_in(doc, "textures", "source", &IndexChange::Delete(index));
    ensure_array(doc, "images").remove(index);
    Ok(())
}

/// 🦠️ `move-image` — relocates a real image to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🖼️image/🚚️move/🦀️.rs`'s own `validate` does.
pub(super) fn move_image(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_images(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
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

/// 🦠️ `create-sampler` — inserts one canonical empty sampler (`{}`) at `position` into
/// `document/samplers`; rejects an out-of-range position, exactly as
/// `../🧬️schema/🧬️mutations/🎛️sampler/🌱️create/🦀️.rs`'s own `validate` does.
pub(super) fn create_sampler(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
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
pub(super) fn delete_sampler(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "samplers") {
        return Err(format!("delete-sampler: index {index} out of range"));
    }
    apply_ref_change_in(doc, "textures", "sampler", &IndexChange::Delete(index));
    ensure_array(doc, "samplers").remove(index);
    Ok(())
}

/// 🦠️ `move-sampler` — relocates a real sampler to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🎛️sampler/🚚️move/🦀️.rs`'s own `validate` does.
pub(super) fn move_sampler(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_samplers(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
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

/// 🦠️ `create-texture` — inserts one canonical empty texture (`{}`) at `position` into
/// `document/textures`; rejects an out-of-range position, exactly as
/// `../🧬️schema/🧬️mutations/🎨️texture/🌱️create/🦀️.rs`'s own `validate` does.
pub(super) fn create_texture(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
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
pub(super) fn delete_texture(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "textures") {
        return Err(format!("delete-texture: index {index} out of range"));
    }
    apply_texture_info_ref_change(doc, &IndexChange::Delete(index));
    ensure_array(doc, "textures").remove(index);
    Ok(())
}

/// 🦠️ `move-texture` — relocates a real texture to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🎨️texture/🚚️move/🦀️.rs`'s own `validate` does.
pub(super) fn move_texture(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_textures(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
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
//#endregion 🔖️Kinds
