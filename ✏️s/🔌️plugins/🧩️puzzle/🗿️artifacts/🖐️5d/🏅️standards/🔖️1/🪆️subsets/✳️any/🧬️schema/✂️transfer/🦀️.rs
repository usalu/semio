//! ✂️ Puzzle 5d schema — the typed copy/paste/translate transfer rules over a `Puzzle5dSnapshot`:
//! the closure selection a copy fragment expands to, its centroid, the fresh-id materialization a
//! paste performs, the direct position write a translate is, and the replace-kind candidate walk.
//! Mirrors semio-compose's `copyDesign`/`pasteDesign`/`dragPieces`/`findReplaceableTypesForSelection`.
//!
//! 🚚️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES):
//! pure helpers over the document schema (no app/AppIo/wasm_bindgen dependency) belong beside the
//! rest of the artifact's schema, not behind an engine facade.

use crate::standards::v1::subsets::any::schema::next_id;
use crate::{Puzzle5dFastener, Puzzle5dPart, Puzzle5dSnapshot};
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
    let Some(catalogs) = crate::kind_catalogs_of(&projection.kind_catalogs, &projection.kind_catalogs_extra) else {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
