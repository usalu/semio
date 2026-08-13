//! ✂️ Puzzle 5d schema — the typed copy/paste/translate transfer rules over a `Puzzle5dSnapshot`:
//! the closure selection a copy fragment expands to, its centroid, the fresh-id materialization a
//! paste performs, the direct position write a translate is, and the replace-kind candidate walk.
//! Mirrors semio-compose's `copyDesign`/`pasteDesign`/`dragPieces`/`findReplaceableTypesForSelection`.
//!
//! 🚚️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
//! pure helpers over the document schema (no app/AppIo/wasm_bindgen dependency) belong beside the
//! rest of the artifact's schema, not behind an engine facade.

use crate::artifacts::puzzle5d::standards::v1::subsets::any::schema::next_id;
use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dPart, Puzzle5dSnapshot, Puzzle5dPartAnchor, Puzzle5dCompatSpecificity};
use std::collections::{HashMap, HashSet};

//#region 🔖️GripRefs
/// 🧩️ The part id a `"part_id:grip_id"` full grip reference belongs to.
fn owning_part_id(grip_ref: &str) -> &str {
    grip_ref.split(':').next().unwrap_or(grip_ref)
}

fn rewrite_grip_ref(grip_ref: &str, id_map: &HashMap<String, String>) -> String {
    match grip_ref.split_once(':') {
        Some((part_id, grip_id)) => match id_map.get(part_id) {
            Some(fresh_part_id) => format!("{fresh_part_id}:{grip_id}"),
            None => grip_ref.to_string(),
        },
        None => grip_ref.to_string(),
    }
}
//#endregion 🔖️GripRefs

//#region 🔖️CopyPasteTranslate
/// 🧮️ Closure-selects a copy fragment from `projection`: expands the part set to include every
/// selected fastener's endpoint parts, then expands the fastener set to include every fastener whose
/// BOTH endpoints are now in the part set — mirrors semio_compose_rs's `copyDesign` closure rule
/// (`semio_compose_rs/dev/algorithm/js/index.ts:483`).
pub fn copy_selection(projection: &Puzzle5dSnapshot, part_ids: &[String], fastener_ids: &[String]) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut part_set: HashSet<String> = part_ids.iter().cloned().collect();
    for fastener in &projection.fasteners {
        if fastener_ids.contains(&fastener.id) {
            part_set.insert(owning_part_id(&fastener.source).to_string());
            part_set.insert(owning_part_id(&fastener.target).to_string());
        }
    }
    let mut fastener_set: HashSet<String> = fastener_ids.iter().cloned().collect();
    if !part_set.is_empty() {
        for fastener in &projection.fasteners {
            let source_part = owning_part_id(&fastener.source);
            let target_part = owning_part_id(&fastener.target);
            if part_set.contains(source_part) && part_set.contains(target_part) {
                fastener_set.insert(fastener.id.clone());
            }
        }
    }
    let parts = projection.parts.iter().filter(|part| part_set.contains(&part.id)).cloned().collect();
    let fasteners = projection.fasteners.iter().filter(|fastener| fastener_set.contains(&fastener.id)).cloned().collect();
    (parts, fasteners)
}

/// 🧮️ The average 2D board position of `parts` — `None` for an empty slice.
pub fn centroid_2d(parts: &[Puzzle5dPart]) -> Option<(f64, f64)> {
    if parts.is_empty() {
        return None;
    }
    let (mut sum_x, mut sum_y) = (0.0, 0.0);
    for part in parts {
        sum_x += part.part_2d.x;
        sum_y += part.part_2d.y;
    }
    let count = parts.len() as f64;
    Some((sum_x / count, sum_y / count))
}

/// 🧮️ Materializes a copied fragment against `projection` at 2D delta `delta_2d` (applied verbatim to
/// the 3D origin's x/y too; z unchanged) — fresh ids are minted for every part to dodge collisions
/// with the target document, and fastener endpoints are remapped to the fresh part ids. Mirrors
/// semio_compose_rs's `pasteDesign` (`semio_compose_rs/dev/algorithm/js/index.ts:515`). Returns the ready-to-insert
/// parts/fasteners; the caller turns each into one `SetPart`/`SetFastener` operation appended past the
/// document's current `parts`/`fasteners` length.
pub fn paste_selection(projection: &Puzzle5dSnapshot, fragment_parts: &[Puzzle5dPart], fragment_fasteners: &[Puzzle5dFastener], delta_2d: (f64, f64)) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut existing_ids: HashSet<String> = projection.parts.iter().map(|part| part.id.clone()).collect();
    let mut fresh_parts = Vec::with_capacity(fragment_parts.len());
    for part in fragment_parts {
        let fresh_id = next_id(existing_ids.iter().map(String::as_str), "part-");
        existing_ids.insert(fresh_id.clone());
        id_map.insert(part.id.clone(), fresh_id.clone());
        let mut next_part = part.clone();
        next_part.id = fresh_id;
        next_part.part_2d.x += delta_2d.0;
        next_part.part_2d.y += delta_2d.1;
        next_part.part_3d.origin[0] += delta_2d.0;
        next_part.part_3d.origin[1] += delta_2d.1;
        fresh_parts.push(next_part);
    }
    let mut existing_fastener_ids: HashSet<String> = projection.fasteners.iter().map(|fastener| fastener.id.clone()).collect();
    let mut fresh_fasteners = Vec::with_capacity(fragment_fasteners.len());
    for fastener in fragment_fasteners {
        let fresh_id = next_id(existing_fastener_ids.iter().map(String::as_str), "fastener-");
        existing_fastener_ids.insert(fresh_id.clone());
        let mut next_fastener = fastener.clone();
        next_fastener.id = fresh_id;
        next_fastener.source = rewrite_grip_ref(&fastener.source, &id_map);
        next_fastener.target = rewrite_grip_ref(&fastener.target, &id_map);
        fresh_fasteners.push(next_fastener);
    }
    (fresh_parts, fresh_fasteners)
}

/// 🧮️ Shifts `part_ids`' 2D board positions and 3D world origins by the given deltas — the puzzle-5d
/// analog of semio_compose_rs's `dragPieces`/`movePieces` (no flatten/re-layout solver here; positions are
/// explicit, so a translate is a direct position write). Mirrors
/// `semio_compose_rs/dev/algorithm/js/index.ts:424,451`. Returns `(index, updated part)` pairs ready for
/// `SetPart` operations.
pub fn translate_parts(projection: &Puzzle5dSnapshot, part_ids: &[String], delta_2d: (f64, f64), delta_3d: [f64; 3]) -> Vec<(usize, Puzzle5dPart)> {
    projection
        .parts
        .iter()
        .enumerate()
        .filter(|(_, part)| part_ids.contains(&part.id))
        .map(|(index, part)| {
            let mut next_part = part.clone();
            next_part.part_2d.x += delta_2d.0;
            next_part.part_2d.y += delta_2d.1;
            next_part.part_3d.origin[0] += delta_3d[0];
            next_part.part_3d.origin[1] += delta_3d[1];
            next_part.part_3d.origin[2] += delta_3d[2];
            (index, next_part)
        })
        .collect()
}

/// 🔍️ Every part-kind id in `kind_catalogs` whose grip kinds are `kind_compatibility`-compatible with
/// `part_id`'s own grip kinds (excluding `part_id`'s current kind) — candidates a "replace kind"
/// picker offers. Mirrors semio_compose_rs's `findReplaceableTypesForSelection` (`semio_compose_rs/dev/algorithm/js/
/// index.ts:84`), computed for real against `kind_catalogs`/`kind_compatibility` instead of a fixture stub.
pub fn find_replaceable_kinds(projection: &Puzzle5dSnapshot, part_id: &str) -> Vec<String> {
    let Some(part) = projection.parts.iter().find(|part| part.id == part_id) else {
        return Vec::new();
    };
    let Some(catalogs) = crate::artifacts::puzzle5d::kind_catalogs_of(&projection.kind_catalogs, &projection.kind_catalogs_extra) else {
        return Vec::new();
    };
    let grip_kinds: HashSet<&str> = part.grips.iter().filter_map(|grip| grip.grip_kind.as_deref()).collect();
    let current_kind = part.part_kind.as_deref().unwrap_or("");
    let mut candidates = Vec::new();
    for candidate in &catalogs.parts {
        if candidate.id == current_kind {
            continue;
        }
        let candidate_grip_kinds: HashSet<&str> = candidate.grips.iter().filter_map(|template| template.grip_kind.as_deref()).collect();
        let compatible = grip_kinds.iter().any(|source_kind| {
            candidate_grip_kinds
                .iter()
                .any(|target_kind| projection.kind_compatibility.iter().any(|rule| (rule.source == *source_kind && rule.target == *target_kind) || (rule.bidirectional && rule.source == *target_kind && rule.target == *source_kind)))
        });
        if compatible {
            candidates.push(candidate.id.clone());
        }
    }
    candidates
}
//#endregion 🔖️CopyPasteTranslate

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle5d::{Puzzle5dCompatSpecificity, Puzzle5dGripTemplate, Puzzle5dCatalogPartKind, Puzzle5dGrip, Puzzle5dKindCatalogs, Puzzle5dKindCompatibility, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor};

    fn part_at(id: &str, x: f64, y: f64) -> Puzzle5dPart {
        Puzzle5dPart {
            id: id.to_string(),
            anchor: Puzzle5dPartAnchor::Fixed,
            part_kind: None,
            part_2d: Puzzle5dPart2d { x, y, ..Default::default() },
            part_3d: Puzzle5dPart3d { origin: [x, y, 0.0], ..Default::default() },
            grips: vec![Puzzle5dGrip { id: "g0".into(), grip_kind: Some("k".into()), grip_2d: Default::default(), grip_3d: Default::default() }],
        }
    }

    fn three_part_projection() -> Puzzle5dSnapshot {
        let mut projection = Puzzle5dSnapshot::default();
        projection.parts.push(part_at("p1", 0.0, 0.0));
        projection.parts.push(part_at("p2", 10.0, 0.0));
        projection.parts.push(part_at("p3", 20.0, 0.0));
        projection.fasteners.push(Puzzle5dFastener { id: "f1".into(), source: "p1:g0".into(), target: "p2:g0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
        projection.fasteners.push(Puzzle5dFastener { id: "f2".into(), source: "p2:g0".into(), target: "p3:g0".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
        projection
    }

    #[test]
    fn copy_selection_pulls_in_fastener_endpoints_and_internal_links() {
        let projection = three_part_projection();
        // Selecting only p1 and p2 (no fastener) should still close over f1 since both endpoints are selected.
        let (parts, fasteners) = copy_selection(&projection, &["p1".into(), "p2".into()], &[]);
        assert_eq!(parts.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["p1", "p2"]);
        assert_eq!(fasteners.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), vec!["f1"]);
    }

    #[test]
    fn copy_selection_expands_parts_from_selected_fastener() {
        let projection = three_part_projection();
        // Selecting only fastener f2 should pull in its endpoint parts p2 and p3.
        let (parts, fasteners) = copy_selection(&projection, &[], &["f2".into()]);
        assert_eq!(parts.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["p2", "p3"]);
        assert_eq!(fasteners.iter().map(|f| f.id.as_str()).collect::<Vec<_>>(), vec!["f2"]);
    }

    #[test]
    fn centroid_2d_averages_positions() {
        let parts = vec![part_at("a", 0.0, 0.0), part_at("b", 10.0, 0.0)];
        assert_eq!(centroid_2d(&parts), Some((5.0, 0.0)));
        assert_eq!(centroid_2d(&[]), None);
    }

    #[test]
    fn paste_selection_mints_fresh_ids_shifts_positions_and_remaps_fastener_endpoints() {
        let projection = three_part_projection();
        let (fragment_parts, fragment_fasteners) = copy_selection(&projection, &["p1".into(), "p2".into()], &[]);
        let (fresh_parts, fresh_fasteners) = paste_selection(&projection, &fragment_parts, &fragment_fasteners, (100.0, 0.0));
        assert_eq!(fresh_parts.len(), 2);
        // Fresh ids must not collide with the existing p1/p2/p3.
        for part in &fresh_parts {
            assert!(!["p1", "p2", "p3"].contains(&part.id.as_str()));
        }
        assert_eq!(fresh_parts[0].part_2d.x, 100.0);
        assert_eq!(fresh_parts[1].part_2d.x, 110.0);
        assert_eq!(fresh_fasteners.len(), 1);
        let fresh_source_part = owning_part_id(&fresh_fasteners[0].source);
        let fresh_target_part = owning_part_id(&fresh_fasteners[0].target);
        assert_eq!(fresh_source_part, fresh_parts[0].id);
        assert_eq!(fresh_target_part, fresh_parts[1].id);
    }

    #[test]
    fn translate_parts_shifts_selected_parts_only() {
        let projection = three_part_projection();
        let updated = translate_parts(&projection, &["p2".into()], (5.0, 5.0), [5.0, 5.0, 5.0]);
        assert_eq!(updated.len(), 1);
        let (index, part) = &updated[0];
        assert_eq!(*index, 1);
        assert_eq!(part.part_2d.x, 15.0);
        assert_eq!(part.part_2d.y, 5.0);
        assert_eq!(part.part_3d.origin, [15.0, 5.0, 5.0]);
    }

    #[test]
    fn find_replaceable_kinds_walks_kind_compatibility() {
        let mut projection = three_part_projection();
        projection.parts[0].part_kind = Some("kind-a".into());
        projection.kind_compatibility.push(Puzzle5dKindCompatibility { source: "k".into(), target: "k2".into(), bidirectional: false, important: false, specificity: Puzzle5dCompatSpecificity::General });
        let (kind_catalogs, kind_catalogs_extra) = crate::artifacts::puzzle5d::split_and_seed_kind_catalogs(Some(Puzzle5dKindCatalogs {
            parts: vec![
                Puzzle5dCatalogPartKind { id: "kind-a".into(), name: "A".into(), label: "A".into(), grips: vec![], ..Default::default() },
                Puzzle5dCatalogPartKind { id: "kind-b".into(), name: "B".into(), label: "B".into(), grips: vec![Puzzle5dGripTemplate { grip_kind: Some("k2".into()), ..Default::default() }], ..Default::default() },
                Puzzle5dCatalogPartKind { id: "kind-c".into(), name: "C".into(), label: "C".into(), grips: vec![Puzzle5dGripTemplate { grip_kind: Some("unrelated".into()), ..Default::default() }], ..Default::default() },
            ],
            grips: vec![],
            fasteners: vec![],
            ropes: vec![],
        }));
        projection.kind_catalogs = kind_catalogs;
        projection.kind_catalogs_extra = kind_catalogs_extra;
        let replaceable = find_replaceable_kinds(&projection, "p1");
        assert_eq!(replaceable, vec!["kind-b".to_string()]);
    }
}
//#endregion 🧪️Tests
