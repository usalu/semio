//! 🎛 `flat-position` — one named inference: absolute flatten pose (plane + center) per object.
//! Two independent dependency-hash chains per the closed-ticket merkle design
//! (26/04/17/OPTIMIZE-FLATTEN-DESIGN-WITH-MERKLE-HASH-CACHE) — an object's flatPosition entry
//! changes iff its own vortex, an ancestor's plane/center, or the connecting attraction's params
//! change; nothing else. The result TYPES (`FlattenPlane`/`FlattenPose`) and the low-level per-edge
//! math stay owned by the sibling `🗜️flatten` schema/inference module (rehomed from the former
//! `⚙️engine/📐️geometry/🎛flatten`, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES);
//! this leaf holds the `InferredField` chains that drive incremental per-entity caching over that math.

use crate::standards::v1::subsets::any::schema::inferences::flatten::{compute_child_plane, diagram_center, find_vortex, flatten_objects_with_assignment, orientation_to_plane, vortex_geom, FlattenParent, FlattenPlane};
use crate::{Puzzle3dObjectAnchor, Puzzle3dSnapshot};
use std::collections::HashMap;

//#region 🔖️DependencyHashChains
// 🎯️ First-cut correctness-over-performance: `assignment_for` re-runs the O(objects+attractions)
// BFS on every `dep_input`/`compute` call, so a full `infer_field` pass here is O(n²) rather than
// O(n) — acceptable at pilot/example scale. A later wave can thread the assignment through once per
// `infer_field` call (e.g. by widening `InferredField::plan`'s contract) instead of re-deriving it.
pub(crate) fn assignment_for(snapshot: &Puzzle3dSnapshot) -> HashMap<String, FlattenParent> {
    flatten_objects_with_assignment(&snapshot.objects, &snapshot.attractions, None).2
}

fn push_numbers(values: &mut Vec<semio_framework_value::DslValue>, numbers: impl IntoIterator<Item = f64>) {
    values.extend(numbers.into_iter().map(semio_framework_value::DslValue::float));
}

/// 🎛️ `flatPosition.plane` — root dep = fixed plane (anchor + origin + orientation); chain dep =
/// parent PlaneHash (folded in by the driver via `parents`) + both connectors' point/direction +
/// gap/shift/rise/rotation/turn/tilt. Matches the merkle-hash ticket's `PlaneHash` chain exactly.
pub struct Puzzle3dFlatPlane;

impl store::InferredField<Puzzle3dSnapshot> for Puzzle3dFlatPlane {
    type Dependency = semio_framework_value::DslValue;
    type Key = String;
    type Value = FlattenPlane;
    const FIELD_ID: &'static str = "s.puzzle.puzzle3d.inference.flatPosition.plane";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["objects", "attractions"]
    }

    fn plan(snapshot: &Puzzle3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        let (_, order, assignment) = flatten_objects_with_assignment(&snapshot.objects, &snapshot.attractions, None);
        order
            .into_iter()
            .map(|id| {
                let parents = match assignment.get(&id) {
                    Some(FlattenParent::Child { parent_id, .. }) => vec![parent_id.clone()],
                    _ => Vec::new(),
                };
                store::InferenceStep { key: id, parents }
            })
            .collect()
    }

    fn dep_input(snapshot: &Puzzle3dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Self::Dependency {
        let assignment = assignment_for(snapshot);
        let mut values = Vec::new();
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, child_vortex_id }) => {
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let child_object = snapshot.objects.iter().find(|o| &o.id == key);
                let edge = parent_object.and_then(|p| find_vortex(p, parent_vortex_id)).zip(child_object.and_then(|c| find_vortex(c, child_vortex_id))).zip(snapshot.attractions.get(*attraction_index));
                if let Some(((parent_vortex, child_vortex), attraction)) = edge {
                    let (pp, pd, _) = vortex_geom(parent_vortex);
                    let (cp, cd, _) = vortex_geom(child_vortex);
                    push_numbers(&mut values, pp);
                    push_numbers(&mut values, pd);
                    push_numbers(&mut values, cp);
                    push_numbers(&mut values, cd);
                    push_numbers(&mut values, [attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt]);
                }
            }
            _ => {
                if let Some(object) = snapshot.objects.iter().find(|o| &o.id == key) {
                    values.push(semio_framework_value::DslValue::Bool(matches!(object.anchor, Puzzle3dObjectAnchor::Fixed)));
                    push_numbers(&mut values, object.origin);
                    if let Some(orientation) = object.orientation {
                        push_numbers(&mut values, orientation);
                    }
                }
            }
        }
        semio_framework_value::DslValue::Array(values)
    }

    fn compute(snapshot: &Puzzle3dSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        let assignment = assignment_for(snapshot);
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, child_vortex_id }) => {
                let parent_plane = parents.first().copied().unwrap_or_default();
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let child_object = snapshot.objects.iter().find(|o| &o.id == key);
                let edge = parent_object.and_then(|p| find_vortex(p, parent_vortex_id)).zip(child_object.and_then(|c| find_vortex(c, child_vortex_id))).zip(snapshot.attractions.get(*attraction_index));
                match edge {
                    Some(((parent_vortex, child_vortex), attraction)) => {
                        let (pp, pd, _) = vortex_geom(parent_vortex);
                        let (cp, cd, _) = vortex_geom(child_vortex);
                        compute_child_plane(parent_plane, pp, pd, cp, cd, attraction)
                    }
                    None => FlattenPlane::default(),
                }
            }
            _ => match snapshot.objects.iter().find(|o| &o.id == key) {
                Some(object) if matches!(object.anchor, Puzzle3dObjectAnchor::Fixed) => orientation_to_plane(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])),
                _ => FlattenPlane::default(),
            },
        }
    }
}

/// 🎛️ `flatPosition.center` — root dep = fixed center (always `[0, 0]` today, `flatten_snapshot`
/// never seeds centers); chain dep = parent CenterHash (folded in via `parents`) + parent connector
/// `direction.z`/`t` + attraction `x`/`y`. Deliberately a SEPARATE chain from `Puzzle3dFlatPlane`: a
/// center-only change (attraction `x`/`y`) must never invalidate the plane chain, and vice versa.
pub struct Puzzle3dFlatCenter;

impl store::InferredField<Puzzle3dSnapshot> for Puzzle3dFlatCenter {
    type Dependency = semio_framework_value::DslValue;
    type Key = String;
    type Value = [f64; 2];
    const FIELD_ID: &'static str = "s.puzzle.puzzle3d.inference.flatPosition.center";
    const SCHEMA_VERSION: u32 = 1;

    fn reads() -> &'static [&'static str] {
        &["objects", "attractions"]
    }

    fn plan(snapshot: &Puzzle3dSnapshot) -> Vec<store::InferenceStep<Self::Key>> {
        Puzzle3dFlatPlane::plan(snapshot)
    }

    fn dep_input(snapshot: &Puzzle3dSnapshot, key: &Self::Key, _parents: &[Self::Key]) -> Self::Dependency {
        let assignment = assignment_for(snapshot);
        let mut values = Vec::new();
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, .. }) => {
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let edge = parent_object.and_then(|p| find_vortex(p, parent_vortex_id)).zip(snapshot.attractions.get(*attraction_index));
                if let Some((parent_vortex, attraction)) = edge {
                    let (_, pd, pt) = vortex_geom(parent_vortex);
                    push_numbers(&mut values, pd);
                    push_numbers(&mut values, [pt, attraction.x, attraction.y]);
                }
            }
            _ => values.push(semio_framework_value::DslValue::Null),
        }
        semio_framework_value::DslValue::Array(values)
    }

    fn compute(snapshot: &Puzzle3dSnapshot, key: &Self::Key, parents: &[Self::Value]) -> Self::Value {
        let assignment = assignment_for(snapshot);
        match assignment.get(key) {
            Some(FlattenParent::Child { parent_id, attraction_index, parent_vortex_id, .. }) => {
                let parent_center = parents.first().copied().unwrap_or([0.0, 0.0]);
                let parent_object = snapshot.objects.iter().find(|o| &o.id == parent_id);
                let edge = parent_object.and_then(|p| find_vortex(p, parent_vortex_id)).zip(snapshot.attractions.get(*attraction_index));
                match edge {
                    Some((parent_vortex, attraction)) => {
                        let (_, pd, pt) = vortex_geom(parent_vortex);
                        diagram_center(parent_center, pd, pt, attraction)
                    }
                    None => [0.0, 0.0],
                }
            }
            _ => [0.0, 0.0],
        }
    }
}
//#endregion 🔖️DependencyHashChains

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
