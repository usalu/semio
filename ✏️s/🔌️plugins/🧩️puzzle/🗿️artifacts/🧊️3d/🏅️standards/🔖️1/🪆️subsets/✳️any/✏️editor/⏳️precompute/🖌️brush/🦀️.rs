//! 🖌️ Puzzle 3d play app — the precompute brush rulebook: which object kind may dock onto which vortex
//! (port shape, single-letter family, the manifest's explicit `kindCompatibility` rows, the
//! host-specific tambour/capsule rules), how the surviving candidates are ranked and weighted, how
//! the fill lane's targets and candidates are sampled without replacement, and how one accepted
//! candidate becomes a concrete `BrushPreviewState`/placement spliced into a `Fixture`. Rehomed from
//! the former `⚙️engine/🖌️brush` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this
//! is the interactive brush tool's own decision logic, so it lives with the app, not the artifact.

use crate::standards::v1::subsets::any::schema::{
    puzzle3d_vortex_full_id, AttractionProps, BrushCompatibleCandidate, BrushHostRules, BrushKindWeights, BrushPlacePayload, BrushPreviewState, CableKindCatalog, Fixture, FixtureObject, KindCatalogBundle, KindCompatEntry, ObjectKind,
    ObjectKindVortexTemplate, Quat, Vec3, VortexKindCatalog, VortexProps,
};
use crate::editor::puzzle3d::precompute::geometry::{compute_brush_placement_pose, normalize_vec3, quat_rotate_vec, vec3_add};

const DEFAULT_CABLE_KIND_ID: &str = "cable.link";

//#region 🔖️VortexContext
#[derive(Debug, Clone)]
pub(crate) struct AttractionVortexContext {
    pub(crate) object_kind: Option<String>,
    pub(crate) vortex_kind: Option<String>,
}

#[derive(Clone, value_derive::ToValue, value_derive::FromValue)]
pub(crate) struct BrushFillVortexTarget {
    pub(crate) full_id: String,
    pub(crate) object_id: String,
    pub(crate) object_kind: Option<String>,
    pub(crate) vortex_kind: Option<String>,
    pub(crate) vortex_index: usize,
}

pub(crate) trait BrushCatalogView {
    fn objects(&self) -> &[ObjectKind];
    fn vortices(&self) -> &[VortexKindCatalog];
    fn cables(&self) -> &[CableKindCatalog];
}

impl BrushCatalogView for KindCatalogBundle {
    fn objects(&self) -> &[ObjectKind] {
        &self.objects
    }

    fn vortices(&self) -> &[VortexKindCatalog] {
        &self.vortices
    }

    fn cables(&self) -> &[CableKindCatalog] {
        &self.cables
    }
}

pub(crate) trait BrushFixtureView {
    fn object_count(&self) -> usize;
    fn find_object_kind(&self, kind_id: &str) -> Option<&FixtureObject>;
}

impl BrushFixtureView for Fixture {
    fn object_count(&self) -> usize {
        self.objects.len()
    }

    fn find_object_kind(&self, kind_id: &str) -> Option<&FixtureObject> {
        self.objects.iter().find(|object| object.object_kind.as_deref() == Some(kind_id))
    }
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

fn catalog_vortex_by_id<'a>(catalogs: &'a impl BrushCatalogView, vortex_kind: &str) -> Option<&'a VortexKindCatalog> {
    catalogs.vortices().iter().find(|v| v.id == vortex_kind)
}

fn catalog_cable_by_id<'a>(catalogs: &'a impl BrushCatalogView, cable_kind: &str) -> Option<&'a CableKindCatalog> {
    catalogs.cables().iter().find(|w| w.id == cable_kind)
}

pub(crate) fn resolve_cable_kind_for_vortex(vortex_kind: &str, catalogs: &impl BrushCatalogView) -> String {
    catalog_vortex_by_id(catalogs, vortex_kind).and_then(|v| v.default_cable_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_CABLE_KIND_ID.to_string())
}

pub(crate) fn resolve_attraction_kind_for_cable(cable_kind: &str, catalogs: &impl BrushCatalogView) -> String {
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

pub(crate) fn attraction_gesture_rule_applies(rule: &KindCompatEntry, attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, catalogs: &impl BrushCatalogView) -> bool {
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

pub(crate) fn vortices_attraction_compatible_for_drag(attracting: &AttractionVortexContext, attracted: &AttractionVortexContext, rules: &[KindCompatEntry], catalogs: &impl BrushCatalogView) -> bool {
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

pub(crate) fn catalog_object_kind_by_id<'a>(catalogs: &'a impl BrushCatalogView, id: &str) -> Option<&'a ObjectKind> {
    catalogs.objects().iter().find(|k| k.id == id)
}

pub(crate) fn resolve_object_kind_mesh_url(kind_id: &str, catalogs: &impl BrushCatalogView, fixture: &impl BrushFixtureView) -> Option<String> {
    if let Some(kind) = catalog_object_kind_by_id(catalogs, kind_id) {
        if let Some(url) = kind.representations.iter().map(|r| r.url.trim()).find(|u| !u.is_empty()) {
            return Some(url.to_string());
        }
    }
    fixture.find_object_kind(kind_id).and_then(|object| object.mesh_url.clone())
}

pub(crate) fn brush_compatible_candidates(target: &AttractionVortexContext, catalogs: &KindCatalogBundle, rules: &[KindCompatEntry], host_rules: &BrushHostRules) -> Vec<BrushCompatibleCandidate> {
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

pub(crate) fn brush_fill_candidate_at(
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

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
pub(crate) fn fill_vortex_target_weight(target: &BrushFillVortexTarget, weights: &BrushKindWeights) -> f64 {
    brush_kind_weight_value(&weights.vortex_weights, target.vortex_kind.as_deref().unwrap_or(""))
}

#[cfg(test)]
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

#[cfg(test)]
pub(crate) fn weighted_order_fill_vortex_targets(targets: &[BrushFillVortexTarget], weights: &BrushKindWeights, rng_state: &mut u32) -> Vec<BrushFillVortexTarget> {
    weighted_sample_without_replacement(targets, |target| fill_vortex_target_weight(target, weights), rng_state)
}

#[cfg(test)]
pub(crate) fn weighted_order_brush_compatible_candidates(candidates: &[BrushCompatibleCandidate], weights: &BrushKindWeights, catalogs: &KindCatalogBundle, rng_state: &mut u32) -> Vec<BrushCompatibleCandidate> {
    weighted_sample_without_replacement(candidates, |candidate| brush_candidate_suggestion_weight(candidate, weights, catalogs), rng_state)
}

pub(crate) fn fill_candidate_diversity_score(candidate: &BrushCompatibleCandidate, target_vortex_index: usize, target_object_kind: Option<&str>) -> i64 {
    if target_object_kind != Some(candidate.object_kind_id.as_str()) {
        return 0;
    }
    1000 + (candidate.source_vortex_index as i64 - target_vortex_index as i64).unsigned_abs() as i64 * 100
}

#[cfg(test)]
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

pub(crate) fn brush_preview_from_candidate(
    target_full_id: &str,
    candidate: &BrushCompatibleCandidate,
    target: &AttractionVortexContext,
    world: TargetVortexWorld,
    catalogs: &impl BrushCatalogView,
    fixture: &impl BrushFixtureView,
) -> Option<BrushPreviewState> {
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
pub(crate) fn brush_object_id(fixture: &impl BrushFixtureView, payload: &BrushPlacePayload) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    fixture.object_count().hash(&mut hasher);
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
