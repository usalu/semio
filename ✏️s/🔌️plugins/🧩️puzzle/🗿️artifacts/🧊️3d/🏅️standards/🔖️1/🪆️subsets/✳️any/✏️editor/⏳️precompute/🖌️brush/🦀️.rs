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
use crate::editor::puzzle3d::precompute::geometry::{
    collision_body_from_buffers, compute_brush_placement_pose, normalize_vec3, pose_isometry, quat_rotate_vec, vec3_add, CollisionAabb, CollisionBody, CollisionOverlapState, CollisionStepContext, CollisionStepResult, Pose3d,
};
use crate::standards::v1::subsets::any::schema::{BrushSuggestionsRunCounter, BrushSuggestionsRunReason, BrushSuggestionsRunStage, SceneConfig};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::{ArtifactInstanceOperationOwnerHandle, ToolRunJobPort};
use semio_framework_tool_run::{ToolRunCounter, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

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

/// 🥽️ The mesh identity a PLACED object renders and collides with: its own non-empty `meshUrl`, else its
/// kind's. The same law as the renderer's `Puzzle3dKindMeshIndex::resolve`; resolving a placed object by kind
/// alone found the first object of that kind instead, so a body carrying its own mesh was never indexed.
pub(crate) fn resolve_placed_object_mesh_url(object: &FixtureObject, catalogs: &impl BrushCatalogView, fixture: &impl BrushFixtureView) -> Option<String> {
    object.mesh_url.as_deref().map(str::trim).filter(|url| !url.is_empty()).map(str::to_string).or_else(|| resolve_object_kind_mesh_url(object.object_kind.as_deref().unwrap_or(""), catalogs, fixture))
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

//#region ⏯️BrushSuggestionsRun
/// 🎲️ Overlap samples one candidate × placed-body pair draws, the interactive brush's own resolution.
const BRUSH_SUGGESTIONS_SAMPLES: usize = 1_024;
/// 🎲️ Samples one collision unit tests before it may yield.
const BRUSH_SUGGESTIONS_SAMPLE_BATCH: usize = 8;

/// 🚥️ What the run decided about one compatible candidate of its target vortex.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrushSuggestionVerdict {
    Pending,
    Free,
    Collision,
    Unavailable,
}

/// 🗂️ One target vortex's compatible candidates as the run resolves them: candidate `key` (the trace key)
/// has pose `previews[key]` and verdict `verdicts[key]`. `writer` is the `(run, generation)` that published
/// it, so a closing job never retires what its successor already published.
#[derive(Clone, Debug, PartialEq)]
pub struct BrushSuggestionsFound {
    pub(crate) writer: (u64, u32),
    pub(crate) target: String,
    pub(crate) previews: Vec<Option<BrushPreviewState>>,
    pub(crate) verdicts: Vec<BrushSuggestionVerdict>,
    pub(crate) done: bool,
}

impl BrushSuggestionsFound {
    /// 🟢️ The free candidates in candidate order — what the popup lists, a cycle walks and an accept places.
    pub fn free(&self) -> impl Iterator<Item = &BrushPreviewState> {
        self.previews.iter().zip(&self.verdicts).filter(|(_, verdict)| **verdict == BrushSuggestionVerdict::Free).filter_map(|(preview, _)| preview.as_ref())
    }
}

/// 📨️ A tool run action the link asked the host for and has not seen answered yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrushSuggestionsRequest {
    Start,
    Abort,
}

/// 🔗️ The instance-owned half of the brush suggestions run: the vortex the user points the search at (an
/// open suggestion popup wins over the brush hover), the run's wake port, the one outstanding start or abort
/// request, and the candidates the live run has resolved so far. Commands write the target, the run job
/// follows it and publishes what it found, renders and accepts read it. Ephemeral local-only state.
#[derive(Default)]
pub struct BrushSuggestionsLink {
    menu: Option<String>,
    hover: Option<String>,
    port: Option<ToolRunJobPort>,
    pub(crate) requested: Option<(BrushSuggestionsRequest, Option<u64>)>,
    pub(crate) retry: bool,
    pub(crate) found: Option<BrushSuggestionsFound>,
}

impl BrushSuggestionsLink {
    /// 🎯️ The vortex the search should be looking at right now.
    pub fn target(&self) -> Option<&str> {
        self.menu.as_deref().or(self.hover.as_deref())
    }

    /// 💡️ A suggestion popup opened on `vortex_full_id`.
    pub fn open_menu(&mut self, vortex_full_id: &str) {
        self.menu = Some(vortex_full_id.to_string()).filter(|id| !id.is_empty());
        self.gesture();
    }

    /// 🔒️ The suggestion popup closed.
    pub fn close_menu(&mut self) {
        if self.menu.take().is_some() {
            self.gesture();
        }
    }

    /// 🖌️ The armed brush now points at `vortex_full_id`, or at nothing.
    pub fn hover(&mut self, vortex_full_id: Option<String>) {
        let hover = vortex_full_id.filter(|id| !id.is_empty());
        if hover != self.hover {
            self.hover = hover;
            self.gesture();
        }
    }

    /// 👆️ A gesture releases the outstanding request and wakes the run to re-read its target.
    fn gesture(&mut self) {
        self.requested = None;
        self.retry = true;
        self.wake();
    }

    pub fn wake(&self) {
        if let Some(port) = &self.port {
            port.wake();
        }
    }

    /// 🗂️ What the run resolved for `target`, if it is the target the run resolved.
    pub fn found(&self, target: &str) -> Option<&BrushSuggestionsFound> {
        self.found.as_ref().filter(|found| found.target == target)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.menu.is_none() && self.hover.is_none() && self.port.is_none() && self.requested.is_none() && self.found.is_none()
    }

    pub(crate) fn clear(&mut self) {
        *self = Self::default();
    }
}

/// 🧠️ An instance operation owner that carries a [`BrushSuggestionsLink`].
pub(crate) trait BrushSuggestionsOwner: std::any::Any {
    fn brush_suggestions(&mut self) -> &mut BrushSuggestionsLink;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BrushSuggestionsPhase {
    Meshes(usize),
    Placed(usize),
    Ready,
}

/// 🧱️ One placed object's collision footprint.
struct BrushSuggestionsPlaced {
    object_id: String,
    mesh_url: String,
    world: Pose3d,
    bounds: CollisionAabb,
}

/// 🔎️ The search over one target vortex's compatible candidates, resumable between collision units.
struct BrushSuggestionsSearch {
    found: BrushSuggestionsFound,
    host: String,
    cursor: usize,
    pair: usize,
    collision: Option<CollisionOverlapState>,
    tested: u64,
    free: u64,
    collisions: u64,
    settled: bool,
}

/// ⏱️ Collision units spend the step's wall clock but never its fuel: one fuel unit is one candidate verdict.
struct BrushSuggestionsCollisionContext<'a, 'b> {
    outer: &'a StepContext<'b>,
}

impl CollisionStepContext for BrushSuggestionsCollisionContext<'_, '_> {
    fn is_cancelled(&self) -> bool {
        self.outer.is_cancelled()
    }

    fn should_yield(&self) -> bool {
        self.outer.deadline_exceeded()
    }

    fn consume_fuel(&mut self, _units: u64) {}
}

/// 📏️ The uniform trace scale of a pose: a number, else the first vector component, else 1.
fn brush_suggestions_scale(scale: &Option<dsl::DslValue>) -> f32 {
    scale.as_ref().and_then(|value| value.as_f64().or_else(|| value.as_array().and_then(|values| values.first()).and_then(dsl::DslValue::as_f64))).map_or(1.0, |value| value as f32)
}

/// 🥽️ Where a run reads one mesh identity's geometry: the process-wide derived mesh store in production.
pub(crate) type BrushSuggestionsMeshSource = fn(&str) -> Option<(Vec<f32>, Vec<u32>)>;

/// ⏯️ The read-only brush suggestions run job (`📋️tool-run-contract.md` §3.7): bounded preparation of the
/// collision meshes (process-wide derived geometry, else the scaled box fallback) and placed bodies, then a
/// search over the target vortex's compatible candidates. Every candidate is upserted `testing` with its pose,
/// then `success/free` or `danger/collision` — one fuel unit per verdict — and every verdict is published to the
/// instance's [`BrushSuggestionsLink`]. The job follows the link's target: a new target clears the trace and
/// searches again, and a settled search waits on its port until a gesture wakes it.
pub(crate) struct BrushSuggestionsRunJob<O: BrushSuggestionsOwner> {
    owner: ArtifactInstanceOperationOwnerHandle,
    port: ToolRunJobPort,
    writer: ToolRunTickWriter,
    scene: Arc<SceneConfig>,
    catalogs: KindCatalogBundle,
    lane: Vec<String>,
    mesh_source: BrushSuggestionsMeshSource,
    phase: BrushSuggestionsPhase,
    meshes: HashMap<String, CollisionBody>,
    fallback: CollisionBody,
    placed: Vec<BrushSuggestionsPlaced>,
    search: Option<BrushSuggestionsSearch>,
    stage: BrushSuggestionsRunStage,
    published: bool,
    progress_sequence: u64,
    closed: bool,
    owner_kind: PhantomData<fn() -> O>,
}

impl<O: BrushSuggestionsOwner> BrushSuggestionsRunJob<O> {
    /// 🌱️ A run over `scene` whose trace subjects index the published mesh `lane`; a mesh identity `mesh_source`
    /// cannot serve collides as the `fallback` box.
    pub(crate) fn new(owner: ArtifactInstanceOperationOwnerHandle, port: ToolRunJobPort, identity: ToolRunIdentity, scene: Arc<SceneConfig>, lane: Vec<String>, mesh_source: BrushSuggestionsMeshSource, fallback: (Vec<f32>, Vec<u32>)) -> Option<Self> {
        let fallback = collision_body_from_buffers(&fallback.0, &fallback.1)?;
        let catalogs = scene.kind_catalogs.clone().unwrap_or_default();
        Some(Self {
            owner,
            port,
            writer: ToolRunTickWriter::new(identity),
            scene,
            catalogs,
            lane,
            mesh_source,
            phase: BrushSuggestionsPhase::Meshes(0),
            meshes: HashMap::new(),
            fallback,
            placed: Vec::new(),
            search: None,
            stage: BrushSuggestionsRunStage::Prepare,
            published: true,
            progress_sequence: 0,
            closed: false,
            owner_kind: PhantomData,
        })
    }

    fn writer_run(&self) -> (u64, u32) {
        let identity = self.writer.identity();
        (identity.id.run, identity.generation)
    }

    /// 🥽️ One preparation unit; `true` once every mesh and placed body is ready.
    fn prepare_one(&mut self) -> bool {
        match self.phase {
            BrushSuggestionsPhase::Meshes(cursor) => {
                self.phase = match self.lane.get(cursor) {
                    Some(url) => {
                        if let Some(body) = (self.mesh_source)(url).and_then(|(positions, indices)| collision_body_from_buffers(&positions, &indices)) {
                            self.meshes.insert(url.clone(), body);
                        }
                        BrushSuggestionsPhase::Meshes(cursor + 1)
                    }
                    None => BrushSuggestionsPhase::Placed(0),
                };
                false
            }
            BrushSuggestionsPhase::Placed(cursor) => {
                self.phase = match self.scene.fixture.objects.get(cursor) {
                    Some(object) => {
                        if let Some(mesh_url) = resolve_placed_object_mesh_url(object, &self.catalogs, &self.scene.fixture) {
                            let world = pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale);
                            let bounds = CollisionAabb::from_body(self.meshes.get(&mesh_url).unwrap_or(&self.fallback), &world);
                            self.placed.push(BrushSuggestionsPlaced { object_id: object.id.clone(), mesh_url, world, bounds });
                        }
                        BrushSuggestionsPhase::Placed(cursor + 1)
                    }
                    None => BrushSuggestionsPhase::Ready,
                };
                false
            }
            BrushSuggestionsPhase::Ready => true,
        }
    }

    /// 🎯️ Retargets the search: clears the trace and lists `target`'s compatible candidates with their poses.
    fn begin_search(&mut self, target: Option<String>) {
        self.writer.clear_trace();
        self.published = false;
        let Some(target) = target else {
            self.search = None;
            self.stage = BrushSuggestionsRunStage::Idle;
            return;
        };
        let mut found = BrushSuggestionsFound { writer: self.writer_run(), target: target.clone(), previews: Vec::new(), verdicts: Vec::new(), done: true };
        let scene = Arc::clone(&self.scene);
        let host = scene.fixture.objects.iter().find_map(|object| object.vortices.iter().position(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == target).map(|index| (object, index)));
        let refusal = match host {
            None => Some(BrushSuggestionsRunReason::TargetMissing),
            Some((host, index)) if !brush_target_vortex_allows_suggestion(host.vortices[index].vortex_kind.as_deref(), &scene.weights) => Some(BrushSuggestionsRunReason::SuggestionsBlocked),
            Some((host, index)) => match vortex_world_from_object(host, index) {
                None => Some(BrushSuggestionsRunReason::TargetMissing),
                Some((position, direction)) => {
                    let context = AttractionVortexContext { object_kind: host.object_kind.clone(), vortex_kind: host.vortices[index].vortex_kind.clone() };
                    let world = TargetVortexWorld { position, direction, reference_orientation: host.orientation };
                    let candidates = brush_compatible_candidates(&context, &self.catalogs, &scene.kind_compatibility, &scene.host_rules);
                    found.previews = candidates.iter().filter(|candidate| brush_candidate_suggestion_weight(candidate, &scene.weights, &self.catalogs) > 0.0).map(|candidate| brush_preview_from_candidate(&target, candidate, &context, world, &self.catalogs, &scene.fixture)).collect();
                    found.verdicts = vec![BrushSuggestionVerdict::Pending; found.previews.len()];
                    found.done = false;
                    None
                }
            },
        };
        if let Some(reason) = refusal {
            let _ = self.writer.step(ToolRunStepKind::Warning, BrushSuggestionsRunStage::Target.index(), reason.code(), None, &[]);
        }
        self.stage = if refusal.is_some() { BrushSuggestionsRunStage::Idle } else { BrushSuggestionsRunStage::Test };
        self.search = Some(BrushSuggestionsSearch { found, host: host.map(|(host, _)| host.id.clone()).unwrap_or_default(), cursor: 0, pair: 0, collision: None, tested: 0, free: 0, collisions: 0, settled: refusal.is_some() });
    }

    /// 🧪️ One collision unit of the current candidate, or its verdict, or the search's completion step.
    fn test_unit(&mut self, context: &mut StepContext<'_>) {
        let Self { search, writer, meshes, fallback, placed, lane, scene, stage, published, .. } = self;
        let (meshes, fallback, placed, lane) = (&*meshes, &*fallback, &*placed, &*lane);
        let Some(search) = search.as_mut().filter(|search| !search.settled) else { return };
        let key = search.cursor;
        let Some(slot) = search.found.previews.get(key) else {
            search.settled = true;
            search.found.done = true;
            *stage = BrushSuggestionsRunStage::Idle;
            *published = false;
            let _ = writer.step(ToolRunStepKind::Success, BrushSuggestionsRunStage::Test.index(), BrushSuggestionsRunReason::SearchComplete.code(), None, &[ToolRunStepArg::Unsigned(search.free), ToolRunStepArg::Unsigned(search.tested)]);
            return;
        };
        let Some(preview) = slot else {
            writer.upsert(key as u64, ToolRunVerdict::Warning, BrushSuggestionsRunReason::PoseUnavailable.code(), ToolRunTraceSubject::Entity { entity: key as u64 });
            search.found.verdicts[key] = BrushSuggestionVerdict::Unavailable;
            search.tested += 1;
            search.cursor += 1;
            *published = false;
            context.consume_fuel(1);
            return;
        };
        let subject = ToolRunTraceSubject::Instance3d {
            mesh: lane.iter().position(|url| *url == preview.mesh_url).unwrap_or(0) as u32,
            position: preview.origin.map(|value| value as f32),
            rotation: preview.orientation.map(|value| value as f32),
            scale: brush_suggestions_scale(&preview.scale),
        };
        if search.pair == 0 && search.collision.is_none() {
            writer.upsert(key as u64, ToolRunVerdict::Testing, BrushSuggestionsRunReason::Free.code(), subject);
        }
        let body = meshes.get(&preview.mesh_url).unwrap_or(fallback);
        let world = pose_isometry(preview.origin, preview.orientation, &preview.scale);
        let bounds = CollisionAabb::from_body(body, &world);
        let budget = scene.overlap_budget;
        let collides = loop {
            let Some(entry) = placed.get(search.pair) else { break Some(false) };
            if search.collision.is_none() && (entry.object_id == search.host || !entry.bounds.intersects(&bounds)) {
                search.pair += 1;
                continue;
            }
            let collision = search.collision.get_or_insert_with(|| CollisionOverlapState::new(BRUSH_SUGGESTIONS_SAMPLES, BRUSH_SUGGESTIONS_SAMPLE_BATCH, budget));
            match collision.step(&mut BrushSuggestionsCollisionContext { outer: context }, body, &world, meshes.get(&entry.mesh_url).unwrap_or(fallback), &entry.world) {
                CollisionStepResult::Pending | CollisionStepResult::Cancelled => break None,
                CollisionStepResult::Complete { overlap, .. } => {
                    search.collision = None;
                    search.pair += 1;
                    if overlap > budget {
                        break Some(true);
                    }
                }
            }
        };
        let Some(collides) = collides else { return };
        let (verdict, reason, decided) = if collides { (ToolRunVerdict::Danger, BrushSuggestionsRunReason::Collision, BrushSuggestionVerdict::Collision) } else { (ToolRunVerdict::Success, BrushSuggestionsRunReason::Free, BrushSuggestionVerdict::Free) };
        writer.upsert(key as u64, verdict, reason.code(), subject);
        search.found.verdicts[key] = decided;
        search.tested += 1;
        search.free += u64::from(!collides);
        search.collisions += u64::from(collides);
        search.cursor += 1;
        search.pair = 0;
        search.collision = None;
        *published = false;
        context.consume_fuel(1);
    }

    /// 📣️ Publishes an unpublished search to the link, waits on the port once the search it follows is
    /// settled, and flushes the step's tick.
    fn settle_step(&mut self, context: &mut StepContext<'_>, desired: Option<String>) -> StepOutcome {
        let following = self.search.as_ref().map(|search| search.found.target.as_str()) == desired.as_deref();
        let idle = self.phase == BrushSuggestionsPhase::Ready && following && self.search.as_ref().is_none_or(|search| search.settled);
        let found = (!self.published).then(|| self.search.as_ref().map(|search| search.found.clone()));
        let port = self.port.clone();
        let exchanged = self.owner.with_mut::<O, _>(|owner| {
            let link = owner.brush_suggestions();
            link.port = Some(port.clone());
            if let Some(found) = found {
                link.found = found;
            }
            if idle && link.target() == desired.as_deref() {
                port.wait();
            }
            Ok(())
        });
        if exchanged.is_ok() {
            self.published = true;
        }
        self.flush(context)
    }

    fn flush(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if self.writer.is_empty() {
            return StepOutcome::Yield;
        }
        self.progress_sequence += 1;
        let (tested, free, collisions, total) = self.search.as_ref().map_or((0, 0, 0, None), |search| (search.tested, search.free, search.collisions, Some(search.found.previews.len() as u64)));
        let counters = BrushSuggestionsRunCounter::ALL.iter().zip([tested, free, collisions]).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect();
        self.writer.progress(ToolRunProgress { identity: self.writer.identity(), sequence: self.progress_sequence, state: ToolRunState::Running, stage: self.stage.index(), completed: tested, total, counters, units_per_second: 0.0, conflicts: 0, steps: ToolRunStepRing::default() });
        let Some(Ok(bytes)) = self.writer.finish().map(|tick| tick.encode()) else {
            return StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) });
        };
        match context.payload_from_bytes(JobPayloadStream::Preview, &bytes) {
            Ok(payload) => StepOutcome::PreviewReady(payload),
            Err(rejected) => {
                drop(rejected.into_source());
                StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) })
            }
        }
    }
}

impl<O: BrushSuggestionsOwner> InteractiveJob for BrushSuggestionsRunJob<O> {
    fn step(&mut self, context: &mut StepContext<'_>) -> StepOutcome {
        if context.is_cancelled() || self.closed {
            return StepOutcome::Cancelled;
        }
        let Ok(desired) = self.owner.with_mut::<O, _>(|owner| Ok(owner.brush_suggestions().target().map(str::to_string))) else {
            return self.flush(context);
        };
        while !context.deadline_exceeded() && !context.fuel_exhausted() {
            if !self.prepare_one() {
                continue;
            }
            if self.search.as_ref().map(|search| search.found.target.as_str()) != desired.as_deref() {
                self.begin_search(desired.clone());
                continue;
            }
            if self.search.as_ref().is_none_or(|search| search.settled) {
                break;
            }
            self.test_unit(context);
        }
        self.settle_step(context, desired)
    }

    fn begin_close(&mut self) {
        self.closed = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.closed = true;
        let writer = self.writer_run();
        let retired = self.owner.with_mut::<O, _>(|owner| {
            let link = owner.brush_suggestions();
            if link.found.as_ref().is_some_and(|found| found.writer == writer) {
                link.found = None;
            }
            Ok(())
        });
        if retired.is_err() {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.search = None;
        self.meshes = HashMap::new();
        self.placed = Vec::new();
        let _ = self.writer.finish();
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closed && self.search.is_none() && self.meshes.is_empty() && self.placed.is_empty()
    }
}
//#endregion ⏯️BrushSuggestionsRun

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
