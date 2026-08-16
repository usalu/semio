//! 🖌️ Puzzle 3d play app — the precompute brush rulebook: which object kind may dock onto which vortex
//! (port shape, single-letter family, the manifest's explicit `kindCompatibility` rows, the
//! host-specific tambour/capsule rules), how the surviving candidates are ranked and weighted, how
//! the fill lane's targets and candidates are sampled without replacement, and how one accepted
//! candidate becomes a concrete `BrushPreviewState`/placement spliced into a `Fixture`. Rehomed from
//! the former `⚙️engine/🖌️brush` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this
//! is the interactive brush tool's own decision logic, so it lives with the app, not the artifact.

use crate::editor::puzzle3d::precompute::geometry::{compute_brush_placement_pose, normalize_vec3, quat_rotate_vec, vec3_add};
use crate::artifacts::puzzle3d::schema::{
    puzzle3d_vortex_full_id, AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushKindWeights, BrushPlacePayload, BrushPreviewState, CableKindCatalog, Fixture, FixtureObject, KindCatalogBundle, KindCompatEntry, ObjectKind, ObjectKindRepresentation,
    ObjectKindVortexTemplate, Quat, Vec3, VortexKindCatalog, VortexProps,
};

const DEFAULT_CABLE_KIND_ID: &str = "cable.link";

//#region 🔖️VortexContext
#[derive(Debug, Clone)]
pub(crate) struct AttractionVortexContext {
    pub(crate) object_kind: Option<String>,
    pub(crate) vortex_kind: Option<String>,
}

#[derive(Clone)]
pub(crate) struct BrushFillVortexTarget {
    pub(crate) full_id: String,
    pub(crate) object_id: String,
    pub(crate) object_kind: Option<String>,
    pub(crate) vortex_kind: Option<String>,
    pub(crate) vortex_index: usize,
}
//#endregion 🔖️VortexContext

//#region 🔖️Compatibility
pub(crate) fn puzzle3d_vortex_port_shape(vortex_kind: &str) -> Option<&'static str> {
    if vortex_kind.contains(" circular ") {
        Some("circular")
    } else if vortex_kind.contains(" rectangular ") {
        Some("rectangular")
    } else {
        None
    }
}

pub(crate) fn puzzle3d_vortex_port_shapes_compatible(source: &str, target: &str) -> bool {
    match (puzzle3d_vortex_port_shape(source), puzzle3d_vortex_port_shape(target)) {
        (None, _) | (_, None) => true,
        (Some(a), Some(b)) => a == b,
    }
}

pub(crate) fn puzzle3d_single_letter_port_family(vortex_kind: &str) -> Option<char> {
    let head = vortex_kind.split('-').next()?;
    if head.len() == 1 {
        let ch = head.chars().next()?;
        if ch.is_ascii_lowercase() {
            return Some(ch);
        }
    }
    None
}

pub(crate) fn puzzle3d_single_letter_port_families_compatible(source: &str, target: &str) -> bool {
    match (puzzle3d_single_letter_port_family(source), puzzle3d_single_letter_port_family(target)) {
        (None, _) | (_, None) => true,
        (Some(a), Some(b)) => a == b,
    }
}

fn catalog_vortex_by_id<'a>(catalogs: &'a KindCatalogBundle, vortex_kind: &str) -> Option<&'a VortexKindCatalog> {
    catalogs.vortices.iter().find(|v| v.id == vortex_kind)
}

fn catalog_cable_by_id<'a>(catalogs: &'a KindCatalogBundle, cable_kind: &str) -> Option<&'a CableKindCatalog> {
    catalogs.cables.iter().find(|w| w.id == cable_kind)
}

pub(crate) fn resolve_cable_kind_for_vortex(vortex_kind: &str, catalogs: &KindCatalogBundle) -> String {
    catalog_vortex_by_id(catalogs, vortex_kind).and_then(|v| v.default_cable_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_CABLE_KIND_ID.to_string())
}

pub(crate) fn resolve_attraction_kind_for_cable(cable_kind: &str, catalogs: &KindCatalogBundle) -> String {
    catalog_cable_by_id(catalogs, cable_kind).and_then(|c| c.default_attraction_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
}

pub(crate) fn compat_pair_matches(rule: &KindCompatEntry, a: &str, b: &str) -> bool {
    (rule.source == a && rule.target == b) || (rule.bidirectional && rule.source == b && rule.target == a)
}

pub(crate) fn specificity_rank(spec: Option<&str>) -> i32 {
    match spec {
        Some("general") => 0,
        Some("object") => 1,
        Some("attraction") => 2,
        Some("cable") => 3,
        Some("vortex") => 4,
        _ => 4,
    }
}

pub(crate) fn attraction_gesture_rule_applies(rule: &KindCompatEntry, attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, catalogs: &KindCatalogBundle) -> bool {
    let cable_src = resolve_cable_kind_for_vortex(attracting.vortex_kind.as_deref().unwrap_or(""), catalogs);
    let cable_tgt = resolve_cable_kind_for_vortex(attracted.vortex_kind.as_deref().unwrap_or(""), catalogs);
    let attraction_src = resolve_attraction_kind_for_cable(&cable_src, catalogs);
    let attraction_tgt = resolve_attraction_kind_for_cable(&cable_tgt, catalogs);
    let sn = attracting.object_kind.as_deref().unwrap_or("");
    let tn = attracted.object_kind.as_deref().unwrap_or("");
    let sv = attracting.vortex_kind.as_deref().unwrap_or("");
    let tv = attracted.vortex_kind.as_deref().unwrap_or("");
    match rule.specificity.as_deref().unwrap_or("vortex") {
        "general" => compat_pair_matches(rule, sv, tv),
        "object" => compat_pair_matches(rule, sn, tn),
        "attraction" => compat_pair_matches(rule, &attraction_src, &attraction_tgt),
        "vortex" => compat_pair_matches(rule, sv, tv),
        "cable" => compat_pair_matches(rule, &cable_src, &cable_tgt),
        _ => compat_pair_matches(rule, sv, tv),
    }
}

pub(crate) fn vortices_attraction_compatible_for_drag(attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, rules: &[KindCompatEntry], catalogs: &KindCatalogBundle) -> bool {
    let sv = attracting.vortex_kind.as_deref().unwrap_or("");
    let tv = attracted.vortex_kind.as_deref().unwrap_or("");
    if !puzzle3d_vortex_port_shapes_compatible(sv, tv) {
        return false;
    }
    if !puzzle3d_single_letter_port_families_compatible(sv, tv) {
        return false;
    }
    if rules.is_empty() {
        return true;
    }
    let mut matched: Vec<&KindCompatEntry> = rules.iter().filter(|r| attraction_gesture_rule_applies(r, attracting, attracted, catalogs)).collect();
    if matched.is_empty() {
        return false;
    }
    if matched.iter().any(|r| r.important) {
        matched.retain(|r| r.important);
    } else {
        let max_rank = matched.iter().map(|r| specificity_rank(r.specificity.as_deref())).max().unwrap_or(4);
        matched.retain(|r| specificity_rank(r.specificity.as_deref()) == max_rank);
    }
    !matched.is_empty()
}
//#endregion 🔖️Compatibility

//#region 🔖️StackPairs
pub(crate) fn brush_stack_vortex_base(vortex_kind: &str) -> Option<&str> {
    if let Some(base) = vortex_kind.strip_suffix(" bottom") {
        Some(base)
    } else if let Some(base) = vortex_kind.strip_suffix(" top") {
        Some(base)
    } else {
        None
    }
}

pub(crate) fn brush_stack_bottom_top_pair(source: &str, target: &str) -> bool {
    let (Some(sb), Some(tb)) = (brush_stack_vortex_base(source), brush_stack_vortex_base(target)) else {
        return false;
    };
    source.ends_with(" bottom") && target.ends_with(" top") && sb == tb
}

pub(crate) fn brush_stack_top_bottom_pair(source: &str, target: &str) -> bool {
    let (Some(sb), Some(tb)) = (brush_stack_vortex_base(source), brush_stack_vortex_base(target)) else {
        return false;
    };
    source.ends_with(" top") && target.ends_with(" bottom") && sb == tb
}

pub(crate) fn brush_stack_mate_pair(source: &str, target: &str) -> bool {
    if !puzzle3d_vortex_port_shapes_compatible(source, target) {
        return false;
    }
    brush_stack_bottom_top_pair(source, target) || brush_stack_top_bottom_pair(source, target)
}
//#endregion 🔖️StackPairs

//#region 🔖️Candidates
pub(crate) fn brush_candidate_rank(candidate: &BrushCompatibleCandidate, template: &ObjectKindVortexTemplate, target: &AttractionVortexContext) -> i64 {
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

pub(crate) fn host_accepts_candidate(rules: &BrushHostRules, target: &AttractionVortexContext, candidate: &BrushCompatibleCandidate, template: &ObjectKindVortexTemplate) -> bool {
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

pub(crate) fn brush_placement_uses_host_orientation(target: &AttractionVortexContext, source_vk: &str, candidate_kind: &str) -> bool {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    if brush_stack_mate_pair(source_vk, target_vk) {
        return false;
    }
    if source_vk != target_vk {
        return false;
    }
    candidate_kind == target.object_kind.as_deref().unwrap_or("")
}

pub(crate) fn catalog_object_kind_by_id<'a>(catalogs: &'a KindCatalogBundle, id: &str) -> Option<&'a ObjectKind> {
    catalogs.objects.iter().find(|k| k.id == id)
}

pub(crate) fn resolve_object_kind_mesh_url(kind_id: &str, catalogs: &KindCatalogBundle, fixture: &Fixture) -> Option<String> {
    if let Some(kind) = catalog_object_kind_by_id(catalogs, kind_id) {
        if let Some(url) = kind.representations.iter().map(|r| r.url.trim()).find(|u| !u.is_empty()) {
            return Some(url.to_string());
        }
    }
    fixture.objects.iter().find(|o| o.object_kind.as_deref() == Some(kind_id)).and_then(|o| o.mesh_url.clone())
}

pub(crate) fn brush_compatible_candidates(target: &AttractionVortexContext, catalogs: &KindCatalogBundle, rules: &[KindCompatEntry], host_rules: &BrushHostRules) -> Vec<BrushCompatibleCandidate> {
    let target_vk = target.vortex_kind.as_deref().unwrap_or("");
    let stack_top_target = target_vk.ends_with(" top");
    let stack_bottom_target = target_vk.ends_with(" bottom");
    let mut scored: Vec<(BrushCompatibleCandidate, i64)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for kind in &catalogs.objects {
        if kind.representations.iter().all(|r| r.url.trim().is_empty()) || kind.vortices.is_empty() {
            continue;
        }
        for (source_vortex_index, template) in kind.vortices.iter().enumerate() {
            let source_vk = template.vortex_kind.as_deref().unwrap_or("");
            if stack_top_target && !brush_stack_mate_pair(source_vk, target_vk) {
                continue;
            }
            if stack_bottom_target && !brush_stack_mate_pair(source_vk, target_vk) {
                continue;
            }
            let attracting = AttractionVortexContext { object_kind: Some(kind.id.clone()), vortex_kind: Some(source_vk.to_string()) };
            if !vortices_attraction_compatible_for_drag(&attracting, target, rules, catalogs) {
                continue;
            }
            let candidate = BrushCompatibleCandidate { object_kind_id: kind.id.clone(), source_vortex_index };
            if !host_accepts_candidate(host_rules, target, &candidate, template) {
                continue;
            }
            let key = format!("{}\u{1}{}", candidate.object_kind_id, candidate.source_vortex_index);
            if !seen.insert(key) {
                continue;
            }
            let rank = brush_candidate_rank(&candidate, template, target);
            scored.push((candidate, rank));
        }
    }
    scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.object_kind_id.cmp(&b.0.object_kind_id)).then_with(|| a.0.source_vortex_index.cmp(&b.0.source_vortex_index)));
    scored.into_iter().map(|(c, _)| c).collect()
}

pub(crate) fn blocked_vortex_full_ids(attractions: &[AttractionProps]) -> std::collections::HashSet<String> {
    let mut s = std::collections::HashSet::new();
    for a in attractions {
        s.insert(a.attracting.clone());
        s.insert(a.attracted.clone());
    }
    s
}

pub(crate) fn vortex_world_from_object(obj: &FixtureObject, vortex_index: usize) -> Option<(Vec3, Vec3)> {
    let vortex = obj.vortices.get(vortex_index)?;
    let orientation = obj.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let position = vec3_add(obj.origin, quat_rotate_vec(orientation, vortex.position));
    let direction = normalize_vec3(quat_rotate_vec(orientation, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
    Some((position, direction))
}

pub(crate) fn enumerate_brush_fill_vortex_targets(fixture: &Fixture) -> Vec<BrushFillVortexTarget> {
    let blocked = blocked_vortex_full_ids(&fixture.attractions);
    let mut out = Vec::new();
    for obj in &fixture.objects {
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
pub(crate) fn brush_kind_weight_value(weights: &std::collections::BTreeMap<String, f64>, id: &str) -> f64 {
    weights.get(id).copied().unwrap_or(1.0)
}

pub(crate) fn brush_candidate_suggestion_weight(candidate: &BrushCompatibleCandidate, weights: &BrushKindWeights, catalogs: &KindCatalogBundle) -> f64 {
    let vortex_kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id).and_then(|kind| kind.vortices.get(candidate.source_vortex_index)).and_then(|template| template.vortex_kind.as_deref()).unwrap_or("");
    brush_kind_weight_value(&weights.object_weights, &candidate.object_kind_id) * brush_kind_weight_value(&weights.vortex_weights, vortex_kind)
}

pub(crate) fn brush_target_vortex_allows_suggestion(vortex_kind: Option<&str>, weights: &BrushKindWeights) -> bool {
    brush_kind_weight_value(&weights.vortex_weights, vortex_kind.unwrap_or("")) > 0.0
}

pub(crate) fn fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &BrushKindWeights) -> f64 {
    brush_kind_weight_value(&weights.vortex_weights, target.vortex_kind.as_deref().unwrap_or(""))
}

pub(crate) fn weighted_sample_without_replacement<T, F>(items: &[T], weight_of: F, rng_state: &mut u32) -> Vec<T>
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

pub(crate) fn fill_rng(rng_state: &mut u32) -> f64 {
    *rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
    *rng_state as f64 / 4_294_967_296.0
}

pub(crate) fn weighted_order_fill_vortex_targets(targets: &[BrushFillVortexTarget], weights: &BrushKindWeights, rng_state: &mut u32) -> Vec<BrushFillVortexTarget> {
    weighted_sample_without_replacement(targets, |target| fill_vortex_target_weight(target, weights), rng_state)
}

pub(crate) fn weighted_order_brush_compatible_candidates(candidates: &[BrushCompatibleCandidate], weights: &BrushKindWeights, catalogs: &KindCatalogBundle, rng_state: &mut u32) -> Vec<BrushCompatibleCandidate> {
    weighted_sample_without_replacement(candidates, |candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs), rng_state)
}

pub(crate) fn fill_candidate_diversity_score(candidate: &BrushCompatibleCandidate, target_vortex_index: usize, target_object_kind: Option<&str>) -> i64 {
    if target_object_kind != Some(candidate.object_kind_id.as_str()) {
        return 0;
    }
    1000 + (candidate.source_vortex_index as i64 - target_vortex_index as i64).unsigned_abs() as i64 * 100
}

pub(crate) fn order_brush_fill_compatible_candidates(
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
pub(crate) struct TargetVortexWorld {
    pub(crate) position: Vec3,
    pub(crate) direction: Vec3,
    pub(crate) reference_orientation: Option<Quat>,
}

pub(crate) fn brush_preview_from_candidate(target_full_id: &str, candidate: &BrushCompatibleCandidate, target: &AttractionVortexContext, world: TargetVortexWorld, catalogs: &KindCatalogBundle, fixture: &Fixture) -> Option<BrushPreviewState> {
    let kind = catalog_object_kind_by_id(catalogs, &candidate.object_kind_id)?;
    let template = kind.vortices.get(candidate.source_vortex_index)?;
    let mesh_url = resolve_object_kind_mesh_url(&candidate.object_kind_id, catalogs, fixture)?;
    let source_vk = template.vortex_kind.as_deref().unwrap_or("");
    let use_host = brush_placement_uses_host_orientation(target, source_vk, &candidate.object_kind_id);
    let (origin, orientation) = compute_brush_placement_pose(template.point, template.direction.unwrap_or([0.0, 0.0, -1.0]), &kind.scale, world.position, world.direction, world.reference_orientation, use_host);
    Some(BrushPreviewState { target_vortex_full_id: target_full_id.to_string(), object_kind_id: kind.id.clone(), source_vortex_index: candidate.source_vortex_index, mesh_url, origin, orientation, scale: kind.scale.clone() })
}

/// 🧱️ Splices one accepted brush placement (the new object plus the attraction docking it onto the
/// pre-existing target vortex) into `fixture`; returns `fixture` unchanged when the kind/template/
/// mesh cannot be resolved or the target vortex is already attracting something.
pub fn apply_brush_placement_to_fixture(fixture: &Fixture, payload: &BrushPlacePayload, catalogs: &KindCatalogBundle) -> Fixture {
    let Some(kind) = catalog_object_kind_by_id(catalogs, &payload.object_kind_id) else {
        return fixture.clone();
    };
    let Some(template) = kind.vortices.get(payload.source_vortex_index) else {
        return fixture.clone();
    };
    let Some(mesh_url) = resolve_object_kind_mesh_url(&payload.object_kind_id, catalogs, fixture) else {
        return fixture.clone();
    };
    let object_id = brush_object_id(fixture, payload);
    let vortices: Vec<VortexProps> = kind.vortices.iter().enumerate().map(|(index, entry)| VortexProps { id: format!("{object_id}:v{index}"), vortex_kind: entry.vortex_kind.clone(), position: entry.point, direction: entry.direction }).collect();
    // 🌲️ The new object attaches as `attracted`: the pre-existing target vortex it's docking onto stays the
    // resolution root. Params start at zero (a bare port-to-port docking); the app's
    // `puzzle3d_rederive_all_attractions` rederives them from this placement's actual pose right after
    // merge, so the object never visibly jumps when the directed-attraction resolver runs.
    let attracted = puzzle3d_vortex_full_id(&object_id, &vortices[payload.source_vortex_index].id);
    let attraction_id = format!("attraction-{}-{attracted}", payload.target_vortex_full_id);
    let mut next = fixture.clone();
    if next.attractions.iter().any(|a| a.attracting == payload.target_vortex_full_id || a.attracted == attracted) {
        return fixture.clone();
    }
    next.attractions.push(AttractionProps { id: attraction_id, attracting: payload.target_vortex_full_id.clone(), attracted, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    next.objects.push(FixtureObject {
        id: object_id,
        object_kind: Some(kind.id.clone()),
        anchor: Default::default(),
        mesh_url: Some(mesh_url),
        origin: payload.origin,
        orientation: Some(payload.orientation),
        scale: payload.scale.clone().or(kind.scale.clone()),
        vortices,
        reveal_index: None,
    });
    let _ = template;
    next
}

/// 🪪️ Content-addressed brush object id — keyed by fixture size and placement payload (no global counter).
fn brush_object_id(fixture: &Fixture, payload: &BrushPlacePayload) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    fixture.objects.len().hash(&mut hasher);
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

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_distribution_excludes_zero_weight_vortices() {
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Placed".to_string(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/placed.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![
                    ObjectKindVortexTemplate { vortex_kind: Some("c-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]) , ..Default::default() },
                    ObjectKindVortexTemplate { vortex_kind: Some("b-s".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]) , ..Default::default() },
                ],
            }],
            vortices: vec![VortexKindCatalog { id: "c-b".to_string(), default_cable_kind: None , ..Default::default() }, VortexKindCatalog { id: "c-t".to_string(), default_cable_kind: None , ..Default::default() }, VortexKindCatalog { id: "b-s".to_string(), default_cable_kind: None , ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None , ..Default::default() }],
        };
        let candidates = vec![BrushCompatibleCandidate { object_kind_id: "Placed".to_string(), source_vortex_index: 0 }, BrushCompatibleCandidate { object_kind_id: "Placed".to_string(), source_vortex_index: 1 }];
        let mut weights = BrushKindWeights::default();
        weights.vortex_weights.insert("c-b".to_string(), 0.0);
        weights.vortex_weights.insert("c-t".to_string(), 0.0);
        weights.vortex_weights.insert("b-s".to_string(), 1.0);
        weights.object_weights.insert("Placed".to_string(), 1.0);
        let mut rng = 7u32;
        let ordered = order_brush_fill_compatible_candidates(&candidates, Some("b-s"), 1, Some("Host"), &catalogs, &weights, &mut rng);
        assert_eq!(ordered.len(), 1);
        assert_eq!(ordered[0].source_vortex_index, 1);
        let targets = vec![
            BrushFillVortexTarget { full_id: "host:v0".to_string(), object_id: "host".to_string(), object_kind: Some("Host".to_string()), vortex_kind: Some("c-t".to_string()), vortex_index: 0 },
            BrushFillVortexTarget { full_id: "host:v1".to_string(), object_id: "host".to_string(), object_kind: Some("Host".to_string()), vortex_kind: Some("b-s".to_string()), vortex_index: 1 },
        ];
        let target_ordered = weighted_order_fill_vortex_targets(&targets, &weights, &mut rng);
        assert_eq!(target_ordered.len(), 1);
        assert_eq!(target_ordered[0].vortex_kind.as_deref(), Some("b-s"));
    }

    #[test]
    fn brush_placement_emits_attraction_with_id_and_directed_root() {
        let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Placed".to_string(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/placed.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) , ..Default::default() }],
            }],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None , ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None , ..Default::default() }],
            cables: vec![],
        };
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".to_string(), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let next = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
        assert_eq!(next.attractions.len(), 1, "brush placement should append exactly one attraction");
        let attraction = &next.attractions[0];
        assert!(!attraction.id.is_empty(), "brush-placed attraction must carry a non-empty id (regression: engine attractions with no id were silently dropped by fixture_from_engine_json)");
        assert_eq!(attraction.attracting, "host:v0", "the pre-existing target vortex must stay the resolution root");
        assert!(attraction.attracted.starts_with(&format!("{}:", next.objects[0].id)), "the newly placed object's vortex must be the attracted (non-root) side");
        assert_eq!(attraction.gap, 0.0);
        assert_eq!(attraction.rotation, 0.0);
    }

    /// 🪪️ Regression: successive brush placements must mint distinct object ids when the fixture grows.
    #[test]
    fn successive_brush_placements_never_collide_on_object_id() {
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind {
                id: "Placed".to_string(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/placed.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) , ..Default::default() }],
            }],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None , ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None , ..Default::default() }],
            cables: vec![],
        };
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".to_string(), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let mut fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let mut ids = std::collections::HashSet::new();
        for i in 0..8 {
            fixture = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
            let placed = fixture.objects.last().expect("placement should append an object");
            assert!(ids.insert(placed.id.clone()), "brush placement #{i} minted a duplicate object id {:?}", placed.id);
            // Successive placements target the same fixed `host:v0`, so only the first actually attaches;
            // reset attractions so every iteration re-exercises `apply_brush_placement_to_fixture` fresh.
            fixture.attractions.clear();
        }
    }

    #[test]
    fn vortex_port_shape_and_compatibility() {
        assert_eq!(puzzle3d_vortex_port_shape("foo circular bar"), Some("circular"));
        assert_eq!(puzzle3d_vortex_port_shape("foo rectangular bar"), Some("rectangular"));
        assert_eq!(puzzle3d_vortex_port_shape("plain"), None);
        assert!(puzzle3d_vortex_port_shapes_compatible("plain", "foo circular bar"));
        assert!(puzzle3d_vortex_port_shapes_compatible("foo circular bar", "baz circular qux"));
        assert!(!puzzle3d_vortex_port_shapes_compatible("foo circular bar", "baz rectangular qux"));
    }

    #[test]
    fn single_letter_port_family_and_compatibility() {
        assert_eq!(puzzle3d_single_letter_port_family("a-socket"), Some('a'));
        assert_eq!(puzzle3d_single_letter_port_family("ab-socket"), None);
        assert_eq!(puzzle3d_single_letter_port_family("A-socket"), None);
        assert_eq!(puzzle3d_single_letter_port_family("plain"), None);
        assert!(puzzle3d_single_letter_port_families_compatible("plain", "a-socket"));
        assert!(puzzle3d_single_letter_port_families_compatible("a-socket", "a-plug"));
        assert!(!puzzle3d_single_letter_port_families_compatible("a-socket", "b-plug"));
    }

    #[test]
    fn resolve_cable_and_attraction_kind_defaults_and_lookup() {
        let catalogs = KindCatalogBundle {
            objects: vec![],
            vortices: vec![VortexKindCatalog { id: "vk".into(), default_cable_kind: Some("  cable.custom  ".into()) , ..Default::default() }, VortexKindCatalog { id: "vk-empty".into(), default_cable_kind: Some("   ".into()) , ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.custom".into(), default_attraction_kind: Some("attraction.custom".into()) , ..Default::default() }],
        };
        assert_eq!(resolve_cable_kind_for_vortex("vk", &catalogs), "cable.custom");
        assert_eq!(resolve_cable_kind_for_vortex("vk-empty", &catalogs), DEFAULT_CABLE_KIND_ID);
        assert_eq!(resolve_cable_kind_for_vortex("missing", &catalogs), DEFAULT_CABLE_KIND_ID);
        assert_eq!(resolve_attraction_kind_for_cable("cable.custom", &catalogs), "attraction.custom");
        assert_eq!(resolve_attraction_kind_for_cable("missing", &catalogs), "");
    }

    #[test]
    fn compat_pair_matches_and_specificity_rank() {
        let rule = KindCompatEntry { source: "a".into(), target: "b".into(), bidirectional: false, important: false, specificity: None };
        assert!(compat_pair_matches(&rule, "a", "b"));
        assert!(!compat_pair_matches(&rule, "b", "a"));
        let bidi = KindCompatEntry { bidirectional: true, ..rule };
        assert!(compat_pair_matches(&bidi, "b", "a"));
        assert_eq!(specificity_rank(Some("general")), 0);
        assert_eq!(specificity_rank(Some("object")), 1);
        assert_eq!(specificity_rank(Some("attraction")), 2);
        assert_eq!(specificity_rank(Some("cable")), 3);
        assert_eq!(specificity_rank(Some("vortex")), 4);
        assert_eq!(specificity_rank(Some("unknown")), 4);
        assert_eq!(specificity_rank(None), 4);
    }

    #[test]
    fn attraction_gesture_rule_applies_specificity_branches() {
        let catalogs = KindCatalogBundle {
            objects: vec![],
            vortices: vec![VortexKindCatalog { id: "sv".into(), default_cable_kind: Some("cable.a".into()) , ..Default::default() }, VortexKindCatalog { id: "tv".into(), default_cable_kind: Some("cable.b".into()) , ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.a".into(), default_attraction_kind: Some("attr.a".into()) , ..Default::default() }, CableKindCatalog { id: "cable.b".into(), default_attraction_kind: Some("attr.b".into()) , ..Default::default() }],
        };
        let attracting = AttractionVortexContext { object_kind: Some("ObjA".into()), vortex_kind: Some("sv".into()) };
        let attracted = AttractionVortexContext { object_kind: Some("ObjB".into()), vortex_kind: Some("tv".into()) };
        let rule_for = |source: &str, target: &str, specificity: Option<&str>| KindCompatEntry { source: source.into(), target: target.into(), bidirectional: false, important: false, specificity: specificity.map(String::from) };
        assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", Some("general")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("ObjA", "ObjB", Some("object")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("attr.a", "attr.b", Some("attraction")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("cable.a", "cable.b", Some("cable")), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", None), &attracting, &attracted, &catalogs));
        assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", Some("weird")), &attracting, &attracted, &catalogs));
        assert!(!attraction_gesture_rule_applies(&rule_for("sv", "other", Some("general")), &attracting, &attracted, &catalogs));
    }

    #[test]
    fn vortices_attraction_compatible_for_drag_branches() {
        let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
        let a_circ = AttractionVortexContext { object_kind: None, vortex_kind: Some("x circular y".into()) };
        let a_rect = AttractionVortexContext { object_kind: None, vortex_kind: Some("x rectangular y".into()) };
        assert!(!vortices_attraction_compatible_for_drag(&a_circ, &a_rect, &[], &catalogs), "incompatible port shapes must reject regardless of rules");

        let a_letter = AttractionVortexContext { object_kind: None, vortex_kind: Some("a-socket".into()) };
        let b_letter = AttractionVortexContext { object_kind: None, vortex_kind: Some("b-plug".into()) };
        assert!(!vortices_attraction_compatible_for_drag(&a_letter, &b_letter, &[], &catalogs), "mismatched single-letter families must reject");

        let sv = AttractionVortexContext { object_kind: None, vortex_kind: Some("sv".into()) };
        let tv = AttractionVortexContext { object_kind: None, vortex_kind: Some("tv".into()) };
        assert!(vortices_attraction_compatible_for_drag(&sv, &tv, &[], &catalogs), "no rules means compatible");

        let unrelated = KindCompatEntry { source: "sv".into(), target: "other".into(), bidirectional: false, important: false, specificity: Some("general".into()) };
        assert!(!vortices_attraction_compatible_for_drag(&sv, &tv, &[unrelated], &catalogs), "no matching rule must reject");

        let low = KindCompatEntry { source: "sv".into(), target: "tv".into(), bidirectional: false, important: false, specificity: Some("general".into()) };
        let important = KindCompatEntry { important: true, ..low.clone() };
        assert!(vortices_attraction_compatible_for_drag(&sv, &tv, &[low, important], &catalogs), "an important match among matched rules must keep it compatible");
    }

    #[test]
    fn brush_stack_pair_helpers() {
        assert_eq!(brush_stack_vortex_base("column bottom"), Some("column"));
        assert_eq!(brush_stack_vortex_base("column top"), Some("column"));
        assert_eq!(brush_stack_vortex_base("column"), None);
        assert!(brush_stack_bottom_top_pair("column bottom", "column top"));
        assert!(!brush_stack_bottom_top_pair("column top", "column bottom"));
        assert!(brush_stack_top_bottom_pair("column top", "column bottom"));
        assert!(!brush_stack_top_bottom_pair("column bottom", "column top"));
        assert!(brush_stack_mate_pair("column bottom", "column top"));
        assert!(brush_stack_mate_pair("column top", "column bottom"));
        assert!(!brush_stack_mate_pair("column bottom", "beam top"));
        assert!(!brush_stack_mate_pair("x circular column bottom", "x rectangular column top"), "incompatible port shapes must reject even a stack mate pair");
    }

    #[test]
    fn brush_candidate_rank_scores_kind_match_and_stack_and_tambour_rules() {
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
        let template = ObjectKindVortexTemplate { vortex_kind: Some("column bottom".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() };
        let same_kind = BrushCompatibleCandidate { object_kind_id: "Host".into(), source_vortex_index: 0 };
        let score = brush_candidate_rank(&same_kind, &template, &target);
        assert_eq!(score, 15_000, "matching object kind (+10000) plus a stack mate pair (+5000)");

        let target_tambour = AttractionVortexContext { object_kind: Some("Tambour".into()), vortex_kind: Some("door tambour circular".into()) };
        let capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("door tambour circular".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() };
        let capital = BrushCompatibleCandidate { object_kind_id: "Capital".into(), source_vortex_index: 0 };
        assert!(brush_candidate_rank(&capital, &capsule_template, &target_tambour) < 0, "capital on tambour must be penalized");

        let cylindric = BrushCompatibleCandidate { object_kind_id: "Cylindric Tambour".into(), source_vortex_index: 0 };
        assert!(brush_candidate_rank(&cylindric, &capsule_template, &target_tambour) > 0, "cylindric tambour stacking onto a mid-tambour host should score positively");
    }

    #[test]
    fn host_accepts_candidate_rule_branches() {
        let rules = BrushHostRules::default();
        let target = AttractionVortexContext { object_kind: Some("Tambour".into()), vortex_kind: Some("door tambour circular".into()) };
        let door_capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("door capsule".into()), point: [1.0, 0.0, 0.0], direction: None , ..Default::default() };

        let capital = BrushCompatibleCandidate { object_kind_id: "Capital".into(), source_vortex_index: 0 };
        assert!(!host_accepts_candidate(&rules, &target, &capital, &door_capsule_template), "reject_capital_on_tambour must reject Capital");

        let storey = BrushCompatibleCandidate { object_kind_id: "Last Storey".into(), source_vortex_index: 0 };
        assert!(!host_accepts_candidate(&rules, &target, &storey, &door_capsule_template), "reject_last_single_storey_on_mid_tambour must reject Last Storey on a Tambour host");

        let door_ok = BrushCompatibleCandidate { object_kind_id: "Door".into(), source_vortex_index: 0 };
        assert!(host_accepts_candidate(&rules, &target, &door_ok, &door_capsule_template), "a door capsule far enough on x and close enough on y must be accepted");

        let non_capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("not a capsule".into()), point: [1.0, 0.0, 0.0], direction: None , ..Default::default() };
        assert!(!host_accepts_candidate(&rules, &target, &door_ok, &non_capsule_template), "a door tambour target requires a door-capsule source vortex");

        let close_template = ObjectKindVortexTemplate { vortex_kind: Some("door capsule".into()), point: [0.1, 0.0, 0.0], direction: None , ..Default::default() };
        assert!(!host_accepts_candidate(&rules, &target, &door_ok, &close_template), "the door capsule position must satisfy the minimum absolute x");

        let door_rule_off = BrushHostRules { door_tambour_requires_door_capsule: false, ..BrushHostRules::default() };
        assert!(host_accepts_candidate(&door_rule_off, &target, &door_ok, &non_capsule_template), "disabling door_tambour_requires_door_capsule accepts regardless of the source vortex kind");
    }

    #[test]
    fn brush_placement_uses_host_orientation_branches() {
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
        assert!(!brush_placement_uses_host_orientation(&target, "column bottom", "Host"), "stack mate pairs never use host orientation");
        assert!(!brush_placement_uses_host_orientation(&target, "other", "Host"), "different vortex kinds never use host orientation");
        assert!(brush_placement_uses_host_orientation(&target, "column top", "Host"), "matching vortex kind and object kind uses host orientation");
        assert!(!brush_placement_uses_host_orientation(&target, "column top", "OtherKind"), "matching vortex kind but a different candidate kind rejects host orientation");
    }

    #[test]
    fn resolve_object_kind_mesh_url_prefers_catalog_then_falls_back_to_fixture() {
        let catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/catalog.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![] }], vortices: vec![], cables: vec![] };
        let fixture = Fixture { attractions: vec![], target_volumes: vec![], objects: vec![] };
        assert_eq!(resolve_object_kind_mesh_url("Kind", &catalogs, &fixture), Some("/catalog.glb".to_string()));

        let empty_catalogs = KindCatalogBundle { objects: vec![ObjectKind { id: "Kind".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![] }], vortices: vec![], cables: vec![] };
        let fixture_with_object = Fixture {
            attractions: vec![],
            target_volumes: vec![],
            objects: vec![FixtureObject { id: "o1".into(), object_kind: Some("Kind".into()), anchor: Default::default(), mesh_url: Some("/fixture.glb".into()), origin: [0.0, 0.0, 0.0], orientation: None, scale: None, vortices: vec![], reveal_index: None }],
        };
        assert_eq!(resolve_object_kind_mesh_url("Kind", &empty_catalogs, &fixture_with_object), Some("/fixture.glb".to_string()));
        assert_eq!(resolve_object_kind_mesh_url("Missing", &empty_catalogs, &fixture_with_object), None);
    }

    #[test]
    fn brush_compatible_candidates_filters_and_sorts() {
        let catalogs = KindCatalogBundle {
            objects: vec![
                ObjectKind { id: "NoMesh".into(), representations: vec![], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] },
                ObjectKind { id: "NoVortices".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/a.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![] },
                ObjectKind { id: "Match".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/b.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] },
            ],
            vortices: vec![],
            cables: vec![],
        };
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("sv".into()) };
        let candidates = brush_compatible_candidates(&target, &catalogs, &[], &BrushHostRules::default());
        assert_eq!(candidates.len(), 1, "kinds with no mesh url or no vortices must be excluded: {candidates:?}");
        assert_eq!(candidates[0].object_kind_id, "Match");
    }

    #[test]
    fn brush_compatible_candidates_stack_target_only_matches_mates() {
        let catalogs = KindCatalogBundle {
            objects: vec![
                ObjectKind { id: "Mate".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/a.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("column bottom".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] },
                ObjectKind { id: "NotMate".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/b.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("beam".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] },
            ],
            vortices: vec![],
            cables: vec![],
        };
        let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
        let candidates = brush_compatible_candidates(&target, &catalogs, &[], &BrushHostRules::default());
        assert_eq!(candidates.len(), 1, "a stack-top target must only match stack mates: {candidates:?}");
        assert_eq!(candidates[0].object_kind_id, "Mate");
    }

    #[test]
    fn blocked_vortex_full_ids_and_enumeration_excludes_them() {
        let attractions = vec![AttractionProps { id: "a1".into(), attracting: "host:v0".into(), attracted: "guest:v0".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 , x: 0.0, y: 0.0}];
        let blocked = blocked_vortex_full_ids(&attractions);
        assert!(blocked.contains("host:v0") && blocked.contains("guest:v0"));

        let fixture = Fixture {
            attractions,
            target_volumes: vec![],
            objects: vec![
                FixtureObject {
                    id: "host".into(),
                    object_kind: Some("Host".into()),
                    anchor: Default::default(),
                    mesh_url: None,
                    origin: [0.0, 0.0, 0.0],
                    orientation: None,
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None }],
                    reveal_index: None,
                },
                FixtureObject {
                    id: "free".into(),
                    object_kind: Some("Free".into()),
                    anchor: Default::default(),
                    mesh_url: None,
                    origin: [0.0, 0.0, 0.0],
                    orientation: None,
                    scale: None,
                    vortices: vec![VortexProps { id: "v0".into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None }],
                    reveal_index: None,
                },
            ],
        };
        let targets = enumerate_brush_fill_vortex_targets(&fixture);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].full_id, "free:v0");
    }

    #[test]
    fn vortex_world_from_object_none_for_missing_index() {
        let object = FixtureObject { id: "o".into(), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [1.0, 2.0, 3.0], orientation: None, scale: None, vortices: vec![], reveal_index: None };
        assert!(vortex_world_from_object(&object, 0).is_none());
    }

    #[test]
    fn weight_lookup_helpers_default_to_one_or_gate_on_zero() {
        let mut weights = BrushKindWeights::default();
        weights.object_weights.insert("A".into(), 2.0);
        weights.vortex_weights.insert("v".into(), 0.0);
        assert_eq!(brush_kind_weight_value(&weights.object_weights, "A"), 2.0);
        assert_eq!(brush_kind_weight_value(&weights.object_weights, "missing"), 1.0);
        assert!(!brush_target_vortex_allows_suggestion(Some("v"), &weights));
        assert!(brush_target_vortex_allows_suggestion(Some("other"), &weights));
        assert!(brush_target_vortex_allows_suggestion(None, &weights));

        let target = BrushFillVortexTarget { full_id: "f".into(), object_id: "o".into(), object_kind: None, vortex_kind: Some("v".into()), vortex_index: 0 };
        assert_eq!(fill_vortex_target_weight(&target, &weights), 0.0);
    }

    #[test]
    fn weighted_sample_without_replacement_edge_cases() {
        let items = vec![1, 2, 3];
        let mut rng = 42u32;
        let single: Vec<i32> = weighted_sample_without_replacement(&[1], |_| 1.0, &mut rng);
        assert_eq!(single, vec![1]);
        let all_zero: Vec<i32> = weighted_sample_without_replacement(&items, |_| 0.0, &mut rng);
        assert!(all_zero.is_empty(), "all-zero weights leave nothing eligible");
        let sampled = weighted_sample_without_replacement(&items, |_| 1.0, &mut rng);
        let mut sorted = sampled;
        sorted.sort_unstable();
        assert_eq!(sorted, items, "every eligible item appears exactly once");
    }

    #[test]
    fn fill_rng_is_deterministic_for_a_given_seed() {
        let mut a = 123u32;
        let mut b = 123u32;
        for _ in 0..5 {
            assert_eq!(fill_rng(&mut a), fill_rng(&mut b));
        }
        assert_ne!(a, 123);
    }

    #[test]
    fn fill_candidate_diversity_score_rewards_distance_within_same_kind() {
        let candidate = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 3 };
        assert_eq!(fill_candidate_diversity_score(&candidate, 0, Some("Other")), 0, "a different target object kind never scores");
        assert_eq!(fill_candidate_diversity_score(&candidate, 0, Some("Kind")), 1000 + 300);
        assert_eq!(fill_candidate_diversity_score(&candidate, 3, Some("Kind")), 1000);
    }

    #[test]
    fn brush_preview_from_candidate_none_branches() {
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind { id: "Kind".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/mesh.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] }],
            vortices: vec![],
            cables: vec![],
        };
        let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let target_ctx = AttractionVortexContext { object_kind: None, vortex_kind: None };
        let world = TargetVortexWorld { position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, -1.0], reference_orientation: None };

        let missing_kind = BrushCompatibleCandidate { object_kind_id: "Missing".into(), source_vortex_index: 0 };
        assert!(brush_preview_from_candidate("t", &missing_kind, &target_ctx, world, &catalogs, &fixture).is_none());

        let bad_index = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 5 };
        assert!(brush_preview_from_candidate("t", &bad_index, &target_ctx, world, &catalogs, &fixture).is_none());

        let empty_mesh_catalogs = KindCatalogBundle {
            objects: vec![ObjectKind { id: "Kind".into(), representations: vec![], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] }],
            vortices: vec![],
            cables: vec![],
        };
        let ok_candidate = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 0 };
        assert!(brush_preview_from_candidate("t", &ok_candidate, &target_ctx, world, &empty_mesh_catalogs, &fixture).is_none(), "a missing mesh url must yield no preview");

        let preview = brush_preview_from_candidate("t", &ok_candidate, &target_ctx, world, &catalogs, &fixture).expect("a valid candidate should produce a preview");
        assert_eq!(preview.mesh_url, "/mesh.glb");
        assert_eq!(preview.object_kind_id, "Kind");
    }

    #[test]
    fn apply_brush_placement_to_fixture_rejects_missing_kind_template_or_mesh() {
        let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind { id: "Kind".into(), representations: vec![], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] }],
            vortices: vec![],
            cables: vec![],
        };

        let missing_kind = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Missing".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_kind, &catalogs).objects.len(), 0);

        let missing_template = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 9, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_template, &catalogs).objects.len(), 0);

        let missing_mesh = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_mesh, &catalogs).objects.len(), 0, "no resolvable mesh url means the placement must be rejected");
    }

    #[test]
    fn apply_brush_placement_to_fixture_rejects_duplicate_attraction_target() {
        let catalogs = KindCatalogBundle {
            objects: vec![ObjectKind { id: "Kind".into(), representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/mesh.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None , ..Default::default() }] }],
            vortices: vec![],
            cables: vec![],
        };
        let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
        let fixture =
            Fixture { attractions: vec![AttractionProps { id: "a".into(), attracting: "host:v0".into(), attracted: "other:v0".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 , x: 0.0, y: 0.0}], objects: vec![], target_volumes: vec![] };
        let next = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
        assert_eq!(next.objects.len(), 0, "a target vortex that is already attracting must reject the placement");
    }
}
//#endregion 🧪️Tests
