//! ⚖️ Joint weights and distribution: the joint weight keys, the distribution weights, the kind weight groups and the joint vortex measures and children. See [`super`].

use super::*;

/// 🎲️ Nested object/vortex distribution — one group per object kind (header slider = the sum of its
/// vortex joints), vortex children are absolute joint probabilities. The sum of every joint across all
/// objects is 1; `object_kind_weights` mirrors each object's joint sum for persistence and precompute.
pub const PUZZLE3D_JOINT_WEIGHT_SEP: &str = "::";

pub fn puzzle3d_joint_weight_key(object_kind_id: &str, vortex_kind_id: &str) -> String {
    format!("{object_kind_id}{PUZZLE3D_JOINT_WEIGHT_SEP}{vortex_kind_id}")
}

pub fn puzzle3d_joint_weights_use_composite_keys(joint_weights: &HashMap<String, f64>) -> bool {
    joint_weights.keys().any(|key| key.contains(PUZZLE3D_JOINT_WEIGHT_SEP))
}

pub fn puzzle3d_read_joint_weight(joint_weights: &HashMap<String, f64>, object_weights: &HashMap<String, f64>, object_kind_id: &str, vortex_kind_id: &str) -> f64 {
    let composite = puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id);
    if let Some(weight) = joint_weights.get(&composite) {
        return *weight;
    }
    let object_weight = object_weights.get(object_kind_id).copied().unwrap_or(0.0);
    let vortex_weight = joint_weights.get(vortex_kind_id).copied().unwrap_or(0.0);
    object_weight * vortex_weight
}

pub fn puzzle3d_object_joint_total(joint_weights: &HashMap<String, f64>, object_weights: &HashMap<String, f64>, object_kind_id: &str, vortex_kind_ids: &[String]) -> f64 {
    vortex_kind_ids.iter().map(|vortex_kind_id| puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, vortex_kind_id)).sum()
}

pub fn puzzle3d_global_joint_total(joint_weights: &HashMap<String, f64>, object_weights: &HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String]) -> f64 {
    object_kind_ids.iter().map(|object_kind_id| puzzle3d_object_joint_total(joint_weights, object_weights, object_kind_id, vortex_kind_ids)).sum()
}

pub fn puzzle3d_sync_object_kind_weights_from_joints(object_weights: &mut HashMap<String, f64>, joint_weights: &HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String]) {
    for object_kind_id in object_kind_ids {
        let total = puzzle3d_object_joint_total(joint_weights, object_weights, object_kind_id, vortex_kind_ids);
        object_weights.insert(object_kind_id.clone(), total);
    }
}

pub fn puzzle3d_ensure_joint_distribution(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String]) {
    if object_kind_ids.is_empty() || vortex_kind_ids.is_empty() {
        object_weights.clear();
        joint_weights.clear();
        return;
    }
    if puzzle3d_joint_weights_use_composite_keys(joint_weights) {
        puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
        let total = puzzle3d_global_joint_total(joint_weights, object_weights, object_kind_ids, vortex_kind_ids);
        if total > f64::EPSILON && (total - 1.0).abs() > 0.001 {
            for object_kind_id in object_kind_ids {
                for vortex_kind_id in vortex_kind_ids {
                    let key = puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id);
                    if let Some(weight) = joint_weights.get_mut(&key) {
                        *weight /= total;
                    }
                }
            }
            puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
        }
        return;
    }
    puzzle3d_ensure_catalog_kind_weights(object_weights, object_kind_ids);
    puzzle3d_ensure_catalog_kind_weights(joint_weights, vortex_kind_ids);
    let mut composite = HashMap::new();
    for object_kind_id in object_kind_ids {
        let object_weight = object_weights.get(object_kind_id).copied().unwrap_or(0.0);
        for vortex_kind_id in vortex_kind_ids {
            let vortex_weight = joint_weights.get(vortex_kind_id).copied().unwrap_or(0.0);
            composite.insert(puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id), object_weight * vortex_weight);
        }
    }
    *joint_weights = composite;
    puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
}

pub fn puzzle3d_set_object_kind_distribution_weight(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String], changed_object_kind_id: &str, new_object_total: f64) {
    puzzle3d_ensure_joint_distribution(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
    let new_object_total = new_object_total.clamp(0.0, 1.0);
    let old_object_total = puzzle3d_object_joint_total(joint_weights, object_weights, changed_object_kind_id, vortex_kind_ids);
    let old_global = puzzle3d_global_joint_total(joint_weights, object_weights, object_kind_ids, vortex_kind_ids).max(f64::EPSILON);
    let old_other_total = (old_global - old_object_total).max(0.0);
    let new_other_total = (1.0 - new_object_total).max(0.0);
    if old_object_total > f64::EPSILON {
        let scale = new_object_total / old_object_total;
        for vortex_kind_id in vortex_kind_ids {
            let key = puzzle3d_joint_weight_key(changed_object_kind_id, vortex_kind_id);
            let joint = puzzle3d_read_joint_weight(joint_weights, object_weights, changed_object_kind_id, vortex_kind_id);
            joint_weights.insert(key, joint * scale);
        }
    } else if !vortex_kind_ids.is_empty() {
        let each = new_object_total / vortex_kind_ids.len() as f64;
        for vortex_kind_id in vortex_kind_ids {
            joint_weights.insert(puzzle3d_joint_weight_key(changed_object_kind_id, vortex_kind_id), each);
        }
    }
    if old_other_total > f64::EPSILON && new_other_total > f64::EPSILON {
        let scale = new_other_total / old_other_total;
        for object_kind_id in object_kind_ids {
            if object_kind_id == changed_object_kind_id {
                continue;
            }
            for vortex_kind_id in vortex_kind_ids {
                let key = puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id);
                let joint = puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, vortex_kind_id);
                joint_weights.insert(key, joint * scale);
            }
        }
    } else if new_other_total <= f64::EPSILON {
        for object_kind_id in object_kind_ids {
            if object_kind_id == changed_object_kind_id {
                continue;
            }
            for vortex_kind_id in vortex_kind_ids {
                joint_weights.insert(puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id), 0.0);
            }
        }
    }
    puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
}

pub fn puzzle3d_set_vortex_joint_weight(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String], object_kind_id: &str, vortex_kind_id: &str, new_joint: f64) {
    puzzle3d_ensure_joint_distribution(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
    let object_total = puzzle3d_object_joint_total(joint_weights, object_weights, object_kind_id, vortex_kind_ids);
    if object_total <= f64::EPSILON {
        return;
    }
    let new_joint = new_joint.clamp(0.0, object_total);
    let mut relative = HashMap::new();
    for id in vortex_kind_ids {
        relative.insert(id.clone(), puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, id) / object_total);
    }
    let share = (new_joint / object_total).clamp(0.0, 1.0);
    let normalized = puzzle3d_normalize_kind_weight_group(&relative, vortex_kind_ids, vortex_kind_id, share);
    for id in vortex_kind_ids {
        joint_weights.insert(puzzle3d_joint_weight_key(object_kind_id, id), normalized.get(id).copied().unwrap_or(0.0) * object_total);
    }
    puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
}

pub fn puzzle3d_apply_distribution_weight(object_weights: &mut HashMap<String, f64>, joint_weights: &mut HashMap<String, f64>, object_kind_ids: &[String], vortex_kind_ids: &[String], action: &str, kind_id: &str, object_kind_id: Option<&str>, value: f64) {
    if action == "setObjectKindWeight" {
        puzzle3d_set_object_kind_distribution_weight(object_weights, joint_weights, object_kind_ids, vortex_kind_ids, kind_id, value);
    } else if let Some(object_kind_id) = object_kind_id {
        puzzle3d_set_vortex_joint_weight(object_weights, joint_weights, object_kind_ids, vortex_kind_ids, object_kind_id, kind_id, value);
    } else {
        puzzle3d_ensure_joint_distribution(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
        let legacy = puzzle3d_normalize_kind_weight_group(joint_weights, vortex_kind_ids, kind_id, value);
        for vortex_kind_id in vortex_kind_ids {
            if let Some(weight) = legacy.get(vortex_kind_id) {
                for object_kind_id in object_kind_ids {
                    joint_weights.insert(puzzle3d_joint_weight_key(object_kind_id, vortex_kind_id), object_weights.get(object_kind_id).copied().unwrap_or(0.0) * *weight);
                }
            }
        }
        puzzle3d_sync_object_kind_weights_from_joints(object_weights, joint_weights, object_kind_ids, vortex_kind_ids);
    }
}

pub fn puzzle3d_uniform_kind_weights(ids: &[String]) -> HashMap<String, f64> {
    if ids.is_empty() {
        return HashMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

pub fn puzzle3d_normalize_kind_weight_group(weights: &HashMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> HashMap<String, f64> {
    if kind_ids.is_empty() {
        return HashMap::new();
    }
    if kind_ids.len() == 1 {
        return HashMap::from([(kind_ids[0].clone(), 1.0)]);
    }
    let new_value = new_value.clamp(0.0, 1.0);
    let others: Vec<&String> = kind_ids.iter().filter(|id| id.as_str() != changed_id).collect();
    let remainder = (1.0 - new_value).max(0.0);
    let other_sum: f64 = others.iter().map(|id| weights.get(*id).copied().unwrap_or(0.0)).sum();
    let mut next = HashMap::new();
    next.insert(changed_id.to_string(), new_value);
    if remainder <= f64::EPSILON {
        for id in others {
            next.insert((*id).clone(), 0.0);
        }
        return next;
    }
    if other_sum <= f64::EPSILON {
        let each = remainder / others.len() as f64;
        for id in others {
            next.insert((*id).clone(), each);
        }
    } else {
        for id in others {
            let old = weights.get(id).copied().unwrap_or(0.0);
            next.insert((*id).clone(), old / other_sum * remainder);
        }
    }
    next
}

pub fn puzzle3d_ensure_catalog_kind_weights(weights: &mut HashMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle3d_uniform_kind_weights(kind_ids);
        return;
    }
    let sum: f64 = kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    if (sum - 1.0).abs() > 0.001 {
        for id in kind_ids {
            if let Some(weight) = weights.get_mut(id) {
                *weight /= sum;
            }
        }
    }
}

fn puzzle3d_object_kind_label(scene_snapshot: &Puzzle3dSceneSnapshot, object_kind_id: &str) -> String {
    puzzle3d_catalog_entries(scene_snapshot, "objects")
        .iter()
        .find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(object_kind_id))
        .and_then(|entry| entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())))
        .unwrap_or(object_kind_id)
        .to_string()
}

pub fn puzzle3d_joint_vortex_weight(object_weight: f64, vortex_weight: f64) -> f64 {
    object_weight * vortex_weight
}

/// 🎲️ Vortex-kind sliders under an object row — displayed value is the joint probability for that
/// object/vortex pair. Siblings under the same object redistribute when one moves; the object header
/// is the sum of its joints. Disabled when that sum is 0.
pub fn puzzle3d_joint_vortex_measures(object_kind_id: &str, object_weight: f64, vortex_kind_ids: &[String], object_weights: &HashMap<String, f64>, joint_weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    let object_kind_zero = object_weight <= f64::EPSILON;
    vortex_kind_ids
        .iter()
        .map(|vortex_kind_id| {
            let joint = puzzle3d_read_joint_weight(joint_weights, object_weights, object_kind_id, vortex_kind_id);
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-joint-vortex-{object_kind_id}-{vortex_kind_id}"),
                label: Some(vortex_kind_id.clone()),
                value: joint,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                disabled: if object_kind_zero { Some(true) } else { None },
                on_change: puzzle3d_action("setVortexKindWeight", Some(json!({ "kindId": vortex_kind_id.as_str(), "objectKindId": object_kind_id }))),
            }
        })
        .collect()
}

pub fn puzzle3d_distribution_children(envelope: &Puzzle3dScene, default_open: Option<bool>) -> Vec<WindowMeasure> {
    let object_ids = puzzle3d_kind_ids(&envelope.scene_snapshot, "objects");
    let vortex_kind_ids = puzzle3d_kind_ids(&envelope.scene_snapshot, "vortices");
    let mut object_weights = envelope.runtime.object_kind_weights.clone();
    let mut joint_weights = envelope.runtime.vortex_kind_weights.clone();
    puzzle3d_ensure_joint_distribution(&mut object_weights, &mut joint_weights, &object_ids, &vortex_kind_ids);
    object_ids
        .iter()
        .map(|object_kind_id| {
            let object_weight = puzzle3d_object_joint_total(&joint_weights, &object_weights, object_kind_id, &vortex_kind_ids);
            let label = puzzle3d_object_kind_label(&envelope.scene_snapshot, object_kind_id);
            WindowMeasure::Group {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{object_kind_id}"),
                label,
                default_open,
                active_utility_id: None,
                value: Some(object_weight),
                min: Some(0.0),
                max: Some(1.0),
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                on_change: Some(puzzle3d_action("setObjectKindWeight", Some(json!({ "kindId": object_kind_id.as_str() })))),
                children: puzzle3d_joint_vortex_measures(object_kind_id, object_weight, &vortex_kind_ids, &object_weights, &joint_weights),
            }
        })
        .collect()
}

pub fn puzzle3d_distribution_group(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, default_open: Option<bool>) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution"),
        label: labels.distribution.into(),
        default_open,
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: puzzle3d_distribution_children(envelope, Some(false)),
    }
}
