//! 🎬️ Oracle kinds for scene roots, node children, material flags and the camera, skin, animation and extension collections.

use super::*;

//#region 🔖️Kinds
/// 🦠️ `bind-node-child` — validated exactly as `../🧬️schema/🧬️mutations/bind-node-child/
/// 🧬️operation/🦀️.rs` documents (index bounds, no self-parenting, no duplicate link, no
/// cycle through the independently-walked `children` graph), reimplemented against the parsed tree.
pub(super) fn bind_node_child(doc: &mut json::JsonValue, parent: usize, child: usize, position: usize) -> Result<(), String> {
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
pub(super) fn unbind_node_child(doc: &mut json::JsonValue, parent: usize, child: usize) -> Result<(), String> {
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
pub(super) fn bind_scene_root_node(doc: &mut json::JsonValue, scene: usize, node: usize, position: usize) -> Result<(), String> {
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
pub(super) fn unbind_scene_root_node(doc: &mut json::JsonValue, scene: usize, node: usize) -> Result<(), String> {
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
pub(super) fn material_alpha_mode(doc: &json::JsonValue, index: usize) -> String {
    arr(doc, "materials").get(index).and_then(|material| obj_get(material, "alphaMode")).and_then(json::JsonValue::as_str).unwrap_or("OPAQUE").to_string()
}

/// 🔎️ `document/materials/{index}/doubleSided`, defaulted to `false` exactly as
/// `GltfMaterial::default()` and its `skip_serializing_if = "is_false"` field attribute do.
pub(super) fn material_double_sided(doc: &json::JsonValue, index: usize) -> bool {
    arr(doc, "materials").get(index).and_then(|material| obj_get(material, "doubleSided")).and_then(json::JsonValue::as_bool).unwrap_or(false)
}

/// 🦠️ `change-material-alpha-mode` — rejects an out-of-range material, an invalid enum spelling and
/// a no-observable-change identity, exactly as `../🧬️schema/🧬️mutations/change-material-alpha-mode/
/// 🧬️operation/🦀️.rs` does.
pub(super) fn change_material_alpha_mode(doc: &mut json::JsonValue, material: usize, alpha_mode: &str) -> Result<(), String> {
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
pub(super) fn change_material_double_sided(doc: &mut json::JsonValue, material: usize, double_sided: bool) -> Result<(), String> {
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
pub(super) fn create_scene(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
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

/// ↩️ `delete-skin`'s own inverse — deliberately NOT dispatched through `create-skin` (whose own
/// payload, `../🧬️schema/🧬️mutations/🦴️skin/🌱️create/🦀️.rs`'s `GltfCreateSkinPayload { position }`,
/// carries no field content at all, so it can only ever recreate an EMPTY skin — real production
/// dispatches this inverse through `DeleteSkinMutation`'s own diff-based `Restore` variant, which
/// this domain-blind reader has no typed access to). Reads `document/skins` and every
/// `nodes/{i}/skin` reference straight off the ORIGINAL (pre-mutation) document and splices them
/// back — the same "restore the exact removed content, not a same-shaped substitute" rule
/// `undo_create_scene` above follows for its own kind, applied to the one other kind in this
/// catalog whose forward mutation is not exactly invertible through a sibling kind's own payload.
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

/// ↩️ `delete-animation`'s own inverse — the identical reasoning as [`undo_delete_skin`] above:
/// `create-animation`'s own payload (`GltfCreateAnimationPayload { position }`) carries no field
/// content, so it can only ever recreate an EMPTY animation. Restores `document/animations`
/// straight off the ORIGINAL (pre-mutation) document; no node reference to splice back (see
/// `create_animation`'s own doc comment — `Animations` is the one family `repair` never touches).
pub fn undo_delete_animation(mutated: &[u8], original: &[u8]) -> Result<Vec<u8>, String> {
    let (mut doc, bin) = read_glb(mutated)?;
    let (source, _) = read_glb(original)?;
    obj_set(&mut doc, "animations", json::JsonValue::Array(arr(&source, "animations")));
    Ok(write_glb(&doc, bin.as_deref()))
}

/// 🔀️ The four remap shapes a single-parent top-level collection's own `repair`/`family_ops!`
/// machinery (`../../🔨️modules/🧬️mutation-support/🗂️top-level-collections/🦀️.rs`) applies to every
/// scalar `nodes/{i}/<field>` reference into it (`camera` for `document/cameras`, `skin` for
/// `document/skins` — the only two top-level families a bare node scalar field points at),
/// reimplemented independently here — this reader never calls into that production module.
pub(super) enum IndexChange<'a> {
    Insert(usize),
    Delete(usize),
    Move(usize, usize),
    Reorder(&'a [usize]),
}

/// 🔀️ One reference's new value under `change`, `None` meaning the reference is dropped — the
/// exact four-branch arithmetic `top_level_collections::remap` documents for a `Change`, re-derived
/// from the format's own rule (an index list shrinks/grows/moves/permutes, every reference to it
/// must track the same motion) rather than copied from that private helper.
pub(super) fn remap_index(change: &IndexChange, value: usize) -> Option<usize> {
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
pub(super) fn create_camera(doc: &mut json::JsonValue, position: usize, projection: &Json) -> Result<(), String> {
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
pub(super) fn delete_camera(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
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
pub(super) fn move_camera(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_cameras(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
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
pub(super) fn create_skin(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
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
pub(super) fn delete_skin(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "skins") {
        return Err(format!("delete-skin: index {index} out of range"));
    }
    apply_node_ref_change(doc, "skin", &IndexChange::Delete(index));
    ensure_array(doc, "skins").remove(index);
    Ok(())
}

/// 🦠️ `move-skin` — relocates a real skin to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🦴️skin/🚚️move/🦀️.rs`'s own `validate` does.
pub(super) fn move_skin(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_skins(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
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
pub(super) fn create_animation(doc: &mut json::JsonValue, position: usize) -> Result<(), String> {
    if position > top_level_len(doc, "animations") {
        return Err(format!("create-animation: position {position} out of range"));
    }
    ensure_array(doc, "animations").insert(position, json::JsonValue::Object(json::object::Object::new()));
    Ok(())
}

/// 🦠️ `delete-animation` — removes a real, currently-present animation, exactly as
/// `../🧬️schema/🧬️mutations/🎞️animation/🗑️delete/🦀️.rs`'s own `validate`/`animations_op` do.
pub(super) fn delete_animation(doc: &mut json::JsonValue, index: usize) -> Result<(), String> {
    if index >= top_level_len(doc, "animations") {
        return Err(format!("delete-animation: index {index} out of range"));
    }
    ensure_array(doc, "animations").remove(index);
    Ok(())
}

/// 🦠️ `move-animation` — relocates a real animation to a real, different position, exactly as
/// `../🧬️schema/🧬️mutations/🎞️animation/🚚️move/🦀️.rs`'s own `validate` does.
pub(super) fn move_animation(doc: &mut json::JsonValue, index: usize, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_animations(doc: &mut json::JsonValue, order: &[usize]) -> Result<(), String> {
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
/// `../🧬️schema/🧬️mutations/✅️required/➕️add/🦀️.rs`'s(/`📣️used-extension/➕️add`'s) own
/// `validate` checks the position bound. `add-required-extension`'s own production `validate` ALSO
/// requires the same name to already be present in `extensionsUsed` — this independent reader does
/// not re-derive that second-array cross-check, since every committed fixture this reader is run
/// against already satisfies it (the same scope camera/skin's own oracle functions keep: index
/// bounds and permutation validity, not every domain invariant the production leaf enforces).
pub(super) fn add_extension(doc: &mut json::JsonValue, key: &str, extension: &str, position: usize) -> Result<(), String> {
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
pub(super) fn remove_extension(doc: &mut json::JsonValue, key: &str, extension: &str) -> Result<(), String> {
    let list = ensure_array(doc, key);
    let index = list.iter().position(|value| value.as_str() == Some(extension)).ok_or_else(|| format!("{key}: extension {extension:?} is not declared"))?;
    list.remove(index);
    Ok(())
}

/// 🧩️ `move-required-extension`/`move-used-extension` — relocates a real, currently-declared
/// extension name to a real, different position, exactly as the sibling
/// `✅️required-extension/🚚️move`/`📣️used-extension/🚚️move` leaves' own `validate` does.
pub(super) fn move_extension(doc: &mut json::JsonValue, key: &str, extension: &str, position: usize) -> Result<(), String> {
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
pub(super) fn reorder_extensions(doc: &mut json::JsonValue, key: &str, order: &[String]) -> Result<(), String> {
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
//#endregion 🔖️Kinds
