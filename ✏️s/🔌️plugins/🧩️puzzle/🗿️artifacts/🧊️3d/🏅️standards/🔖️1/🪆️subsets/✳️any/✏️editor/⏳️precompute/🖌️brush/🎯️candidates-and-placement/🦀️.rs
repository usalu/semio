//! 🎯️ Brush candidates and placement: the candidate ranking, the kind weights and the placement of one brush candidate on a snapshot.

use super::*;

//#region 🔖️Candidates
pub fn brush_candidate_rank(candidate: &BrushCompatibleCandidate, template: &ObjectKindVortexTemplate, target: &AttractionVortexContext) -> i64 {
    let mut score: i64 = 0;
    let target_kind = target.vortex_kind.as_deref().unwrap_or("");
    let source_kind = template.vortex_kind.as_deref().unwrap_or("");
    if candidate.object_kind_id == target.object_kind.as_deref().unwrap_or("") {
        score += 10_000;
    }
    if brush_stack_mate_pair(source_kind, target_kind) {
        score += 5_000;
    }
    if source_kind == target_kind && !brush_stack_mate_pair(source_kind, target_kind) {
        score -= 4_000;
    }
    if target_kind.ends_with(" top") && !brush_stack_mate_pair(source_kind, target_kind) {
        score -= 2_000;
    }
    if target_kind.ends_with(" bottom") && !source_kind.ends_with(" top") {
        score -= 2_000;
    }
    if target_kind.contains("tambour circular") || target_kind.contains("tambour rectangular") {
        let host_kind = target.object_kind.as_deref().unwrap_or("");
        let mid_tambour_host = host_kind == "Tambour" || host_kind == "Cylindric Tambour";
        if candidate.object_kind_id.contains("Capital") {
            score -= 50_000;
        } else if candidate.object_kind_id.contains("Cylindric") && candidate.object_kind_id.contains("Tambour") {
            score += 11_000;
        }
        if mid_tambour_host && (candidate.object_kind_id.contains("Last Storey") || candidate.object_kind_id.contains("Single Storey")) {
            score -= 30_000;
        }
        if mid_tambour_host && candidate.object_kind_id == "Cylindric Tambour" {
            score += 5_000;
        }
    }
    score
}

pub fn host_accepts_candidate(rules: &BrushHostRules, target: &AttractionVortexContext, candidate: &BrushCompatibleCandidate, template: &ObjectKindVortexTemplate) -> bool {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    if rules.reject_capital_on_tambour && (target_vk.contains("tambour circular") || target_vk.contains("tambour rectangular")) && candidate.object_kind_id.contains("Capital") {
        return false;
    }
    let host_kind = target.object_kind.as_deref().unwrap_or("");
    if rules.reject_last_single_storey_on_mid_tambour
        && (target_vk.contains("tambour circular") || target_vk.contains("tambour rectangular"))
        && (host_kind == "Tambour" || host_kind == "Cylindric Tambour")
        && (candidate.object_kind_id.contains("Last Storey") || candidate.object_kind_id.contains("Single Storey"))
    {
        return false;
    }
    if !rules.door_tambour_requires_door_capsule || !target_vk.contains("door tambour") {
        return true;
    }
    let source_vk = template.vortex_kind.as_deref().unwrap_or("");
    if !source_vk.contains("door capsule") {
        return false;
    }
    let x = template.point[0].abs();
    let y = template.point[1].abs();
    x >= rules.door_capsule_min_abs_x && y < rules.door_capsule_max_abs_y
}

pub fn brush_placement_uses_host_orientation(target: &AttractionVortexContext, source_vk: &str, candidate_kind: &str) -> bool {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    if brush_stack_mate_pair(source_vk, target_vk) {
        return false;
    }
    if source_vk != target_vk {
        return false;
    }
    candidate_kind == target.object_kind.as_deref().unwrap_or("")
}

pub fn catalog_object_kind_by_id<'a>(catalogs: &'a impl BrushCatalogView, id: &str) -> Option<&'a ObjectKind> {
    catalogs.objects().iter().find(|k| k.id == id)
}

pub fn resolve_object_kind_mesh_url(kind_id: &str, catalogs: &impl BrushCatalogView, scene_snapshot: &impl BrushSceneView) -> Option<String> {
    if let Some(kind) = catalog_object_kind_by_id(catalogs, kind_id) {
        if let Some(url) = kind.representations.iter().map(|r| r.url.trim()).find(|u| !u.is_empty()) {
            return Some(url.to_string());
        }
    }
    scene_snapshot.find_object_kind(kind_id).and_then(|object| object.mesh_url.clone())
}

/// 🥽️ The mesh identity a PLACED object renders and collides with: its own non-empty `meshUrl`, else its
/// kind's. The same law as the renderer's `Puzzle3dKindMeshIndex::resolve`; resolving a placed object by kind
/// alone found the first object of that kind instead, so a body carrying its own mesh was never indexed.
pub fn resolve_placed_object_mesh_url(object: &EngineSceneObject, catalogs: &impl BrushCatalogView, scene_snapshot: &impl BrushSceneView) -> Option<String> {
    object.mesh_url.as_deref().map(str::trim).filter(|url| !url.is_empty()).map(str::to_string).or_else(|| resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), catalogs, scene_snapshot))
}

pub fn brush_compatible_candidates(target: &AttractionVortexContext, catalogs: &KindCatalogBundle, rules: &[KindCompatEntry], host_rules: &BrushHostRules) -> Vec<BrushCompatibleCandidate> {
    let mut scored: Vec<(BrushCompatibleCandidate, i64)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (kind_index, kind) in catalogs.objects.iter().enumerate() {
        for source_vortex_index in 0..kind.vortices.len() {
            let Some((candidate, rank)) = brush_fill_candidate_at(target, catalogs, rules, host_rules, kind_index, source_vortex_index) else { continue };
            let key = format!("{}\u{1}{}", candidate.object_kind_id, candidate.source_vortex_index);
            if !seen.insert(key) {
                continue;
            }
            scored.push((candidate, rank));
        }
    }
    scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.object_kind_id.cmp(&b.0.object_kind_id)).then_with(|| a.0.source_vortex_index.cmp(&b.0.source_vortex_index)));
    scored.into_iter().map(|(c, _)| c).collect()
}

pub fn brush_fill_candidate_at(
    target: &AttractionVortexContext,
    catalogs: &impl BrushCatalogView,
    rules: &[KindCompatEntry],
    host_rules: &BrushHostRules,
    kind_index: usize,
    source_vortex_index: usize,
) -> Option<(BrushCompatibleCandidate, i64)> {
    let kind = catalogs.objects().get(kind_index)?;
    if kind.representations.iter().all(|representation| representation.url.trim().is_empty()) {
        return None;
    }
    let template = kind.vortices.get(source_vortex_index)?;
    let source_vk = template.vortex_kind.as_deref().unwrap_or("");
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    if (target_vk.ends_with(" top") || target_vk.ends_with(" bottom")) && !brush_stack_mate_pair(source_vk, target_vk) {
        return None;
    }
    let attracting = AttractionVortexContext { object_kind: Some(kind.id.clone()), vortex_kind: Some(source_vk.to_string()) };
    if !vortices_attraction_compatible_for_drag(&attracting, target, rules, catalogs) {
        return None;
    }
    let candidate = BrushCompatibleCandidate { object_kind_id: kind.id.clone(), source_vortex_index };
    if !host_accepts_candidate(host_rules, target, &candidate, template) {
        return None;
    }
    let rank = brush_candidate_rank(&candidate, template, target);
    Some((candidate, rank))
}

#[cfg(any(test, feature = "testing"))]
pub fn blocked_vortex_full_ids(attractions: &[AttractionProps]) -> std::collections::HashSet<String> {
    let mut s = std::collections::HashSet::new();
    for a in attractions {
        s.insert(a.attracting.clone());
        s.insert(a.attracted.clone());
    }
    s
}

pub fn vortex_world_from_object(obj: &EngineSceneObject, vortex_index: usize) -> Option<(Vec3, Vec3)> {
    let vortex = obj.vortices.get(vortex_index)?;
    let orientation = obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let position = vec3_add(obj.origin, quat_rotate_vec(orientation, vortex.position));
    let direction = normalize_vec3(quat_rotate_vec(orientation, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
    Some((position, direction))
}

#[cfg(any(test, feature = "testing"))]
pub fn enumerate_brush_fill_vortex_targets(scene_snapshot: &EngineSceneSnapshot) -> Vec<BrushFillVortexTarget> {
    let blocked = blocked_vortex_full_ids(&scene_snapshot.attractions);
    let mut out = Vec::new();
    for obj in &scene_snapshot.objects {
        for (i, vortex) in obj.vortices.iter().enumerate() {
            let full_id = puzzle3d_vortex_full_id(&obj.id, &vortex.id);
            if !blocked.contains(&full_id) {
                out.push(BrushFillVortexTarget { full_id, object_id: obj.id.clone(), object_kind: obj.object_kind.clone(), vortex_kind: vortex.vortex_kind.clone(), vortex_index: i });
            }
        }
    }
    out
}
//#endregion 🔖️Candidates

//#region 🔖️Weights
pub fn brush_kind_weight_value(weights: &std::collections::BTreeMap<String, f64>, id: &str) -> f64 {
    weights.get(id).copied().unwrap_or(1.0)
}

pub fn brush_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &BrushKindWeights, catalogs: &KindCatalogBundle) -> f64 {
    let vortex_kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
    brush_joint_weight_value(&weights.object_weights, &weights.vortex_weights, &candidate.object_kind_id, vortex_kind)
}

pub fn brush_target_vortex_allows_suggestion(vortex_kind: Option<&str>, weights: &BrushKindWeights) -> bool {
    brush_kind_weight_value(&weights.vortex_weights, vortex_kind.unwrap_or("")) > 0.0
}

fn brush_joint_weight_value(object_weights: &std::collections::BTreeMap<String, f64>, joint_weights: &std::collections::BTreeMap<String, f64>, object_kind_id: &str, vortex_kind_id: &str) -> f64 {
    let sep = semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_JOINT_WEIGHT_SEP;
    let composite = format!("{object_kind_id}{sep}{vortex_kind_id}");
    if let Some(weight) = joint_weights.get(&composite) {
        return *weight;
    }
    brush_kind_weight_value(object_weights, object_kind_id) * brush_kind_weight_value(joint_weights, vortex_kind_id)
}

#[cfg(any(test, feature = "testing"))]
pub fn fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &BrushKindWeights) -> f64 {
    let object_kind = target.object_kind.as_deref().unwrap_or("");
    let vortex_kind = target.vortex_kind.as_deref().unwrap_or("");
    brush_joint_weight_value(&weights.object_weights, &weights.vortex_weights, object_kind, vortex_kind)
}

#[cfg(any(test, feature = "testing"))]
pub fn weighted_sample_without_replacement<T, F>(items: &[T], weight_of: F, rng_state: &mut u32) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> f64,
{
    let eligible: Vec<T> = items.iter().filter(|item| weight_of(item) > 0.0).cloned().collect();
    if eligible.len() < 2 {
        return eligible;
    }
    let mut remaining = eligible;
    let mut out = Vec::new();
    while !remaining.is_empty() {
        let w_list: Vec<f64> = remaining.iter().map(&weight_of).collect();
        let total: f64 = w_list.iter().sum();
        if total <= 0.0 {
            break;
        }
        let mut r = fill_rng(rng_state) * total;
        let mut pick = remaining.len() - 1;
        for (i, weight) in w_list.iter().enumerate() {
            r -= weight;
            if r <= 0.0 {
                pick = i;
                break;
            }
        }
        out.push(remaining[pick].clone());
        remaining.remove(pick);
    }
    out
}

pub fn fill_rng(rng_state: &mut u32) -> f64 {
    *rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
    *rng_state as f64 / 4_294_967_296.0
}

#[cfg(any(test, feature = "testing"))]
pub fn weighted_order_fill_vortex_targets(targets: &[BrushFillVortexTarget], weights: &BrushKindWeights, rng_state: &mut u32) -> Vec<BrushFillVortexTarget> {
    weighted_sample_without_replacement(targets, |target| fill_vortex_target_weight(target, weights), rng_state)
}

#[cfg(any(test, feature = "testing"))]
pub fn weighted_order_brush_compatible_candidates(candidates: &[BrushCompatibleCandidate], weights: &BrushKindWeights, catalogs: &KindCatalogBundle, rng_state: &mut u32) -> Vec<BrushCompatibleCandidate> {
    weighted_sample_without_replacement(candidates, |candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs), rng_state)
}

pub fn fill_candidate_diversity_score(candidate: &BrushCompatibleCandidate, target_vortex_index: usize, target_object_kind: Option<&str>) -> i64 {
    if target_object_kind != Some(candidate.object_kind_id.as_str()) {
        return 0;
    }
    1000 + (candidate.source_vortex_index as i64 - target_vortex_index as i64).unsigned_abs() as i64 * 100
}

#[cfg(any(test, feature = "testing"))]
pub fn order_brush_fill_compatible_candidates(
    candidates: &[BrushCompatibleCandidate],
    target_vortex_kind: Option<&str>,
    target_vortex_index: usize,
    target_object_kind: Option<&str>,
    catalogs: &KindCatalogBundle,
    weights: &BrushKindWeights,
    rng_state: &mut u32,
) -> Vec<BrushCompatibleCandidate> {
    let allowed: Vec<BrushCompatibleCandidate> = candidates.iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs) > 0.0).cloned().collect();
    let target = target_vortex_kind.unwrap_or("");
    let mut cross = Vec::new();
    let mut same = Vec::new();
    for candidate in allowed {
        let source_vk = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
        if source_vk != target || brush_stack_mate_pair(source_vk, target) {
            cross.push(candidate);
        } else {
            same.push(candidate);
        }
    }
    cross.sort_by(|left, right| {
        fill_candidate_diversity_score(right, target_vortex_index, target_object_kind)
            .cmp(&fill_candidate_diversity_score(left, target_vortex_index, target_object_kind))
            .then_with(|| left.object_kind_id.cmp(&right.object_kind_id))
            .then_with(|| left.source_vortex_index.cmp(&right.source_vortex_index))
    });
    let mut same_sorted = same;
    same_sorted.sort_by(|left, right| left.object_kind_id.cmp(&right.object_kind_id).then_with(|| left.source_vortex_index.cmp(&right.source_vortex_index)));
    cross.extend(weighted_order_brush_compatible_candidates(&same_sorted, weights, catalogs, rng_state));
    cross
}
//#endregion 🔖️Weights

//#region 🔖️Placement
/// 🎯️ A target vortex's world-space pose, bundled so `brush_preview_from_candidate` stays under clippy's arg-count limit.
#[derive(Clone, Copy)]
pub struct TargetVortexWorld {
    pub position: Vec3,
    pub direction: Vec3,
    pub reference_orientation: Option<Quat>,
}

pub fn brush_preview_from_candidate(
    target_full_id: &str,
    candidate: &BrushCompatibleCandidate,
    target: &AttractionVortexContext,
    world: TargetVortexWorld,
    catalogs: &impl BrushCatalogView,
    scene_snapshot: &impl BrushSceneView,
) -> Option<BrushPreviewState> {
    let kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id)?;
    let template = kind.vortices.get(candidate.source_vortex_index)?;
    let mesh_url = resolve_object_kind_mesh_url(&candidate.object_kind_id, catalogs, scene_snapshot)?;
    let source_vk = template.vortex_kind.as_deref().unwrap_or("");
    let use_host = brush_placement_uses_host_orientation(target, source_vk, &candidate.object_kind_id);
    let (origin, orientation) = compute_brush_placement_pose(template.point, template.direction.unwrap_or([0.0, 0.0, -1.0]), &kind.scale, world.position, world.direction, world.reference_orientation, use_host);
    Some(BrushPreviewState { target_vortex_full_id: target_full_id.to_string(), object_kind_id: kind.id.clone(), source_vortex_index: candidate.source_vortex_index, mesh_url, origin, orientation, scale: kind.scale.clone() })
}

/// 🧱️ Splices one accepted brush placement (the new object plus the attraction docking it onto the
/// pre-existing target vortex) into `scene_snapshot`; returns `scene_snapshot` unchanged when the kind/template/
/// mesh cannot be resolved or the target vortex is already attracting something.
pub fn apply_brush_placement_to_snapshot(scene_snapshot: &EngineSceneSnapshot, payload: &BrushPlacePayload, catalogs: &KindCatalogBundle) -> EngineSceneSnapshot {
    let Some(kind) = catalog_object_kind_by_id(catalogs, &payload.object_kind_id) else {
        return scene_snapshot.clone();
    };
    let Some(template) = kind.vortices.get(payload.source_vortex_index) else {
        return scene_snapshot.clone();
    };
    let Some(mesh_url) = resolve_object_kind_mesh_url(&payload.object_kind_id, catalogs, scene_snapshot) else {
        return scene_snapshot.clone();
    };
    let object_id = brush_object_id(scene_snapshot, payload);
    let vortices: Vec<VortexProps> = kind.vortices.iter().enumerate().map(|(index, entry)| VortexProps { id: format!("{object_id}:v{index}"), vortex_kind: entry.vortex_kind.clone(), position: entry.point, direction: entry.direction }).collect();
    // 🌲️ The new object attaches as `attracted`: the pre-existing target vortex it's docking onto stays the
    // resolution root. Params start at zero (a bare port-to-port docking).
    let attracted = puzzle3d_vortex_full_id(&object_id, &vortices[payload.source_vortex_index].id);
    let attraction_id = format!("attraction-{}-{attracted}", payload.target_vortex_full_id);
    let mut next = scene_snapshot.clone();
    if next.attractions.iter().any(|a| a.attracting == payload.target_vortex_full_id || a.attracted == attracted) {
        return scene_snapshot.clone();
    }
    next.attractions.push(AttractionProps { id: attraction_id, attracting: payload.target_vortex_full_id.clone(), attracted, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    next.objects.push(EngineSceneObject {
        id: object_id,
        object_kind: Some(kind.id.clone()),
        anchor: Default::default(),
        mesh_url: Some(mesh_url),
        origin: payload.origin,
        orientation: Some(payload.orientation),
        scale: payload.scale.clone().or(kind.scale.clone()),
        vortices,
    });
    let _ = template;
    next
}

/// 🪪️ Content-addressed brush object id — keyed by scene_snapshot size and placement payload (no global counter).
pub fn brush_object_id(scene_snapshot: &impl BrushSceneView, payload: &BrushPlacePayload) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    scene_snapshot.object_count().hash(&mut hasher);
    payload.target_vortex_full_id.hash(&mut hasher);
    payload.object_kind_id.hash(&mut hasher);
    payload.source_vortex_index.hash(&mut hasher);
    for axis in &payload.origin {
        axis.to_bits().hash(&mut hasher);
    }
    for axis in &payload.orientation {
        axis.to_bits().hash(&mut hasher);
    }
    if let Some(scale) = &payload.scale {
        format!("{scale:?}").hash(&mut hasher);
    }
    format!("puzzle3d.brush.{:016x}", hasher.finish())
}
//#endregion 🔖️Placement
