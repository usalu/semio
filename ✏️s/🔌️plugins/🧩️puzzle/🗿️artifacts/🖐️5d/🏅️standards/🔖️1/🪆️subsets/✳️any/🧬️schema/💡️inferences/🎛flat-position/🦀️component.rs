//! 🎛 `flat-position` — one named inference: absolute flatten pose (plane + center) per part.
//! `FlattenPlane`/`FlattenPose` (the result types) stay owned by `puzzle3d::schema::inferences::flatten`
//! (that sibling artifact's own low-level pose math); this leaf owns the puzzle5d-side projection —
//! mapping parts/grips/fasteners onto the 3d object/vortex/attraction graph and running puzzle3d's
//! solver — directly, so `📦️glue.rs` has a `flat_position` mount matching puzzle3d's own shape.
//!
//! 🚚️ Relocated from the deleted `⚙️engine/📐️flatten` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `flatten_snapshot`/`flatten_snapshot_inplace`
//! are a pure `Puzzle5dSnapshot -> projection` derived-compute pair, already the sole compute this
//! artifact's own `Puzzle5dInference` (the family-root `🦀️component.rs`) calls to fill
//! `flatPositions` — the mirror-side of the `puzzle2d_manifest_fragment` pattern (a
//! `Snapshot -> Value` manifest-projection pure fn living in `🧬️schema/💡️inferences/`), so the math
//! belongs here beside its one real consumer instead of behind an engine facade.

use crate::artifacts::puzzle3d::schema::inferences::flatten::{self, DIAGRAM_HORIZONTAL_SCALE, DIAGRAM_RADIUS, DIAGRAM_VERTICAL_V_EXTRA};
use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dVortex};
use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dPart, Puzzle5dPartAnchor, Puzzle5dScale, Puzzle5dSnapshot};
use std::collections::HashMap;

// 🔗️ Kept public (the pre-relocation shim's own surface): the result TYPES stay owned by
// puzzle3d's own low-level geometry, re-exported here so `flat_position::FlattenPose`/`FlattenPlane`
// keep resolving for any caller reaching through this slug's own name.
pub use crate::artifacts::puzzle3d::schema::inferences::flatten::{FlattenPlane, FlattenPose};

//#region 🔖️SnapshotToObjectGraph
async fn parse_endpoint(endpoint: &str) -> Option<(&str, &str)> {
    endpoint.split_once(':')
}

async fn part_to_object(part: &Puzzle5dPart) -> Puzzle3dObject {
    let anchor = match part.anchor {
        Puzzle5dPartAnchor::Fixed => Puzzle3dObjectAnchor::Fixed,
        Puzzle5dPartAnchor::Derived => Puzzle3dObjectAnchor::Derived,
    };
    Puzzle3dObject {
        id: part.id.clone(),
        label: part.part_3d.label.clone(),
        object_kind: part.part_kind.clone(),
        anchor,
        origin: part.part_3d.origin,
        orientation: part.part_3d.orientation,
        scale: part.part_3d.scale.as_ref().map(|scale| match scale {
            Puzzle5dScale::Uniform(value) => crate::artifacts::puzzle3d::Puzzle3dScale::Uniform(*value),
            Puzzle5dScale::Vec3(value) => crate::artifacts::puzzle3d::Puzzle3dScale::Vec3(*value),
        }),
        mesh_url: part.part_3d.mesh_url.clone(),
        vortices: part.grips.iter().map(grip_to_vortex).collect(),
        hidden: part.part_2d.hidden.unwrap_or(false),
        locked: part.part_2d.locked.unwrap_or(false),
    }
}

async fn grip_to_vortex(grip: &Puzzle5dGrip) -> Puzzle3dVortex {
    Puzzle3dVortex {
        id: grip.id.clone(),
        vortex_kind: grip.grip_kind.clone(),
        label: grip.grip_3d.label.clone(),
        position: grip.grip_3d.position,
        direction: grip.grip_3d.direction,
        radius: grip.grip_3d.radius.or(grip.grip_2d.radius),
        hidden: false,
        locked: false,
    }
}

async fn fastener_to_attraction(fastener: &Puzzle5dFastener) -> Puzzle3dAttraction {
    Puzzle3dAttraction {
        id: fastener.id.clone(),
        attracting: fastener.source.clone(),
        attracted: fastener.target.clone(),
        gap: fastener.gap,
        shift: fastener.shift,
        rise: fastener.rise,
        rotation: fastener.rotation,
        turn: fastener.turn,
        tilt: fastener.tilt,
        x: fastener.x,
        y: fastener.y,
    }
}

async fn grip_t(grip: &Puzzle5dGrip) -> f64 {
    grip.grip_2d.angle / (2.0 * std::f64::consts::PI)
}
//#endregion 🔖️SnapshotToObjectGraph

//#region 🔖️Flatten
/// 🌤️ Flatten a 5d snapshot in place: updates part 3d origins/orientations and 2d x/y from the attraction graph.
pub async fn flatten_snapshot_inplace(snapshot: &mut Puzzle5dSnapshot) {
    let objects: Vec<Puzzle3dObject> = snapshot.parts.iter().map(part_to_object).collect();
    let attractions: Vec<Puzzle3dAttraction> = snapshot.fasteners.iter().map(fastener_to_attraction).collect();
    let seed_centers: HashMap<String, [f64; 2]> = snapshot.parts.iter().map(|part| (part.id.clone(), [part.part_2d.x, part.part_2d.y])).collect();
    let poses = flatten::flatten_objects(&objects, &attractions, Some(&seed_centers));
    // Recompute diagram centers with grip `t` from 2d angle (3d vortices do not carry t).
    let centers = diagram_centers_with_grip_t(snapshot, &poses);
    for part in &mut snapshot.parts {
        if let Some(pose) = poses.get(&part.id) {
            part.part_3d.origin = pose.plane.origin;
            part.part_3d.orientation = Some(pose.orientation);
            let center = centers.get(&part.id).copied().unwrap_or(pose.center);
            part.part_2d.x = center[0];
            part.part_2d.y = center[1];
        }
    }
}

/// 🌤️ Flatten a 5d snapshot, returning poses keyed by part id.
pub async fn flatten_snapshot(snapshot: &Puzzle5dSnapshot) -> HashMap<String, FlattenPose> {
    let objects: Vec<Puzzle3dObject> = snapshot.parts.iter().map(part_to_object).collect();
    let attractions: Vec<Puzzle3dAttraction> = snapshot.fasteners.iter().map(fastener_to_attraction).collect();
    let seed_centers: HashMap<String, [f64; 2]> = snapshot.parts.iter().map(|part| (part.id.clone(), [part.part_2d.x, part.part_2d.y])).collect();
    let mut poses = flatten::flatten_objects(&objects, &attractions, Some(&seed_centers));
    let centers = diagram_centers_with_grip_t(snapshot, &poses);
    for (id, center) in centers {
        if let Some(pose) = poses.get_mut(&id) {
            pose.center = center;
        }
    }
    poses
}

async fn diagram_centers_with_grip_t(snapshot: &Puzzle5dSnapshot, seed_poses: &HashMap<String, FlattenPose>) -> HashMap<String, [f64; 2]> {
    let part_map: HashMap<&str, &Puzzle5dPart> = snapshot.parts.iter().map(|part| (part.id.as_str(), part)).collect();
    let mut adjacency: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for (index, fastener) in snapshot.fasteners.iter().enumerate() {
        let Some((parent_id, _)) = parse_endpoint(&fastener.source) else { continue };
        let Some((child_id, _)) = parse_endpoint(&fastener.target) else { continue };
        if part_map.contains_key(parent_id) && part_map.contains_key(child_id) {
            adjacency.entry(parent_id.to_string()).or_default().push((child_id.to_string(), index));
            adjacency.entry(child_id.to_string()).or_default().push((parent_id.to_string(), index));
        }
    }
    let mut centers: HashMap<String, [f64; 2]> = HashMap::new();
    let mut visited = std::collections::HashSet::new();
    use std::collections::VecDeque;
    for part in &snapshot.parts {
        if visited.contains(&part.id) {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back(part.id.clone());
        visited.insert(part.id.clone());
        let seed = seed_poses.get(&part.id).map(|pose| pose.center).unwrap_or([part.part_2d.x, part.part_2d.y]);
        centers.insert(part.id.clone(), seed);
        while let Some(current_id) = queue.pop_front() {
            let parent_center = *centers.get(&current_id).unwrap_or(&[0.0, 0.0]);
            let neighbors = adjacency.get(&current_id).cloned().unwrap_or_default();
            for (neighbor_id, fastener_index) in neighbors {
                if visited.contains(&neighbor_id) {
                    continue;
                }
                visited.insert(neighbor_id.clone());
                let fastener = &snapshot.fasteners[fastener_index];
                let Some((design_parent_id, design_parent_grip)) = parse_endpoint(&fastener.source) else {
                    centers.insert(neighbor_id.clone(), [0.0, 0.0]);
                    queue.push_back(neighbor_id);
                    continue;
                };
                let current_grip_id = if design_parent_id == current_id { design_parent_grip } else { parse_endpoint(&fastener.target).map(|(_, grip)| grip).unwrap_or("") };
                let current_part = part_map.get(current_id.as_str()).expect("current");
                let grip = current_part.grips.iter().find(|grip| grip.id == current_grip_id);
                let parent_t = grip.map(grip_t).unwrap_or(0.0);
                let mut parent_direction = grip.and_then(|grip| grip.grip_3d.direction).unwrap_or([0.0, 0.0, 1.0]);
                let len = (parent_direction[0].powi(2) + parent_direction[1].powi(2) + parent_direction[2].powi(2)).sqrt();
                if len > 0.0 {
                    parent_direction[0] /= len;
                    parent_direction[1] /= len;
                    parent_direction[2] /= len;
                }
                let connection_x = fastener.x;
                let connection_y = fastener.y;
                let (child_x, child_y) = if parent_center[0] == 0.0 && parent_center[1] == 0.0 {
                    let angle = 2.0 * std::f64::consts::PI * parent_t;
                    (DIAGRAM_RADIUS * angle.sin(), DIAGRAM_RADIUS * angle.cos())
                } else if parent_direction[2].abs() > 0.5 {
                    (parent_center[0] + connection_x, parent_center[1] + connection_y + DIAGRAM_VERTICAL_V_EXTRA)
                } else {
                    (parent_center[0] + connection_x * DIAGRAM_HORIZONTAL_SCALE, parent_center[1] + connection_y * DIAGRAM_HORIZONTAL_SCALE)
                };
                let round = |v: f64| (v * 1_000_000.0).round() / 1_000_000.0;
                centers.insert(neighbor_id.clone(), [round(child_x), round(child_y)]);
                queue.push_back(neighbor_id);
            }
        }
    }
    centers
}
//#endregion 🔖️Flatten

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dGrip, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor, Puzzle5dSnapshot};

    #[semio_framework_async_macros::async_test]
    async fn flatten_writes_diagram_offsets_onto_part_2d() {
        let mut snapshot = Puzzle5dSnapshot {
            schema: "puzzle.5d".into(),
            domain: "architecture".into(),
            label: None,
            meta: Puzzle5dMeta { description: String::new() },
            kind_catalogs: None,
            kind_catalogs_extra: None,
            kind_compatibility: Vec::new(),
            parts: vec![
                Puzzle5dPart {
                    id: "p".into(),
                    part_kind: None,
                    anchor: Puzzle5dPartAnchor::Fixed,
                    part_2d: Puzzle5dPart2d { x: 10.0, y: 20.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
                    part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
                    grips: vec![Puzzle5dGrip {
                        id: "top".into(),
                        grip_kind: None,
                        grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                        grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, 1.0], direction: Some([0.0, 0.0, 1.0]), radius: None, label: None },
                    }],
                },
                Puzzle5dPart {
                    id: "c".into(),
                    part_kind: None,
                    anchor: Puzzle5dPartAnchor::Derived,
                    part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: None, radius: None, width: None, height: None, text: None, icon_kind: None, hidden: None, locked: None },
                    part_3d: Puzzle5dPart3d { origin: [0.0, 0.0, 0.0], mesh_url: None, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
                    grips: vec![Puzzle5dGrip {
                        id: "bottom".into(),
                        grip_kind: None,
                        grip_2d: Puzzle5dGrip2d { angle: 0.0, grip_kind: None, radius: None },
                        grip_3d: Puzzle5dGrip3d { position: [0.0, 0.0, -1.0], direction: Some([0.0, 0.0, -1.0]), radius: None, label: None },
                    }],
                },
            ],
            fasteners: vec![Puzzle5dFastener { id: "f".into(), source: "p:top".into(), target: "c:bottom".into(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 1.5, y: 2.5 }],
        };
        flatten_snapshot_inplace(&mut snapshot);
        let child = snapshot.parts.iter().find(|part| part.id == "c").expect("c");
        assert_eq!(child.part_2d.x, 10.0 + 1.5);
        assert_eq!(child.part_2d.y, 20.0 + 2.5 + DIAGRAM_VERTICAL_V_EXTRA);
    }
}
//#endregion 🧪️Tests
