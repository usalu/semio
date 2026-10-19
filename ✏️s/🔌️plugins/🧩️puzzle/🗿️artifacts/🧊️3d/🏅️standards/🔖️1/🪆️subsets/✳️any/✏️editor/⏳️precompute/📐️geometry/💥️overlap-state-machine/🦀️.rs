//! 💥️ Overlap state machine of the precompute geometry layer: the incremental penetration test between two collision bodies.

use super::*;

//#region 🔖️Collision
//#region ⏳️OverlapStateMachine
pub trait CollisionStepContext {
    fn is_cancelled(&self) -> bool;
    fn should_yield(&self) -> bool;
    fn consume_fuel(&mut self, units: u64);
}

impl CollisionStepContext for semio_framework_job::StepContext<'_> {
    fn is_cancelled(&self) -> bool {
        semio_framework_job::StepContext::is_cancelled(self)
    }

    fn should_yield(&self) -> bool {
        semio_framework_job::StepContext::should_yield(self)
    }

    fn consume_fuel(&mut self, units: u64) {
        semio_framework_job::StepContext::consume_fuel(self, units);
    }
}

/// 🕳️ Where a [`CollisionPenetrationState`] stands. `A`/`B` name the two bodies; `Vertices*` probe one body's vertices
/// against the other solid, `Faces*` probe the parts of one body's triangles that pass behind the other body's faces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub enum CollisionPenetrationStage {
    BroadPhase,
    VerticesA,
    VerticesB,
    FacesA,
    FacesB,
    InsetA,
    InsetB,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollisionStepResult {
    Pending,
    Cancelled,
    /// 🌐️ `depth`: how far (m) one body's surface dives into the other solid, measured to that solid's nearest surface —
    /// `0` for bodies that only touch. `rejected_early` when the measure stopped at the first probe past the tolerance.
    Complete { depth: f64, rejected_early: bool },
}

/// 🕳️ Resumable PENETRATION-DEPTH test of one body pair: every step probes one vertex or one triangle, so a fill or brush
/// search interleaves it with its own fuel and cancellation.
///
/// The measure is the deepest point of one body's SURFACE inside the other SOLID, as its distance to that solid's surface.
/// Probes: the vertices of each body that lie inside the other, and the corners of every part of one body's triangle that
/// passes behind a triangle of the other inside that triangle's prism ([`collision::clip_behind_triangle`]) — the exact
/// extremes of a surface diving under a face. A probe only counts when it is farther than `tolerance` from the other
/// surface AND inside the other solid, which also keeps flush contact (the docking of two faces) at depth `0`. Surfaces
/// that COINCIDE (a duplicate at the same pose, a face lying inside another body's face) have no probe off the other
/// surface, so each triangle also probes its centroid moved inward by twice the tolerance: inside its own body, and for a
/// flush-docked neighbour outside the other one, but deep inside a coinciding solid.
///
/// 🐛️ It replaced a volume estimate (512 random points in the bounding-box intersection against an `m³` budget) that
/// accepted a slab 5.5 cm deep inside its neighbour and, because the docking host was skipped, a placement 1 m deep inside
/// its host (ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS, dev directive: only tiny collisions on the touching
/// surfaces).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub struct CollisionPenetrationState {
    pub stage: CollisionPenetrationStage,
    pub(super) part_a: usize,
    pub(super) part_b: usize,
    pub(super) cursor: usize,
    tolerance: f64,
    pub(super) depth: f64,
    pub(super) rejected_early: bool,
}

impl CollisionPenetrationState {
    pub fn new(tolerance: f64) -> Self {
        Self { stage: CollisionPenetrationStage::BroadPhase, part_a: 0, part_b: 0, cursor: 0, tolerance, depth: 0.0, rejected_early: false }
    }

    #[cfg(test)]
    pub fn checkpoint(&self) -> Self {
        self.clone()
    }

    #[cfg(test)]
    pub fn resume(checkpoint: Self) -> Self {
        checkpoint
    }

    pub fn step<C: CollisionStepContext>(&mut self, context: &mut C, a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> CollisionStepResult {
        if context.is_cancelled() {
            return CollisionStepResult::Cancelled;
        }
        if context.should_yield() {
            return CollisionStepResult::Pending;
        }
        let result = match self.stage {
            CollisionPenetrationStage::BroadPhase => {
                if CollisionAabb::from_body(a, world_a).intersects(&CollisionAabb::from_body(b, world_b)) {
                    self.advance(CollisionPenetrationStage::VerticesA)
                } else {
                    self.finish()
                }
            }
            CollisionPenetrationStage::VerticesA => self.step_vertex(a, world_a, b, world_b, CollisionPenetrationStage::VerticesB),
            CollisionPenetrationStage::VerticesB => self.step_vertex(b, world_b, a, world_a, CollisionPenetrationStage::FacesA),
            CollisionPenetrationStage::FacesA => self.step_face(a, world_a, b, world_b, CollisionPenetrationStage::FacesB),
            CollisionPenetrationStage::FacesB => self.step_face(b, world_b, a, world_a, CollisionPenetrationStage::InsetA),
            CollisionPenetrationStage::InsetA => self.step_inset(a, world_a, b, world_b, CollisionPenetrationStage::InsetB),
            CollisionPenetrationStage::InsetB => self.step_inset(b, world_b, a, world_a, CollisionPenetrationStage::Complete),
            CollisionPenetrationStage::Complete => CollisionStepResult::Complete { depth: self.depth, rejected_early: self.rejected_early },
        };
        context.consume_fuel(1);
        if context.is_cancelled() { CollisionStepResult::Cancelled } else { result }
    }

    fn advance(&mut self, stage: CollisionPenetrationStage) -> CollisionStepResult {
        self.stage = stage;
        self.part_a = 0;
        self.part_b = 0;
        self.cursor = 0;
        if stage == CollisionPenetrationStage::Complete {
            return self.finish();
        }
        CollisionStepResult::Pending
    }

    fn finish(&mut self) -> CollisionStepResult {
        self.stage = CollisionPenetrationStage::Complete;
        CollisionStepResult::Complete { depth: self.depth, rejected_early: self.rejected_early }
    }

    /// 📍️ Records `point` as a depth probe into `solid`'s part, keeping the deepest one; answers `true` once the depth is past
    /// the tolerance, which ends the measure (an infinite tolerance measures the full depth).
    fn probe(&mut self, point: rigid::Point3, solid_bounds: &CollisionAabb, solid_pose: rigid::Isometry3, solid: &CollisionShape) -> bool {
        // 📦️ A point outside the solid's world bounds cannot be inside it: the surface query and the containment test are only
        // paid for probes where the two bodies' bounds meet, and containment (BVH ray parity) only for a probe whose surface
        // distance would deepen the measure.
        let outside = |axis: usize, value: f32| value < solid_bounds.min[axis] || value > solid_bounds.max[axis];
        if outside(0, point.x) || outside(1, point.y) || outside(2, point.z) {
            return false;
        }
        precompute_work(1);
        let distance = f64::from(collision::distance_to_surface(solid_pose, &solid.shape, point));
        if distance <= COLLISION_CONTACT_NOISE_M || distance <= self.depth {
            return false;
        }
        precompute_work(1);
        if !collision::contains_point_fast(solid_pose, &solid.shape, point) {
            return false;
        }
        self.depth = distance;
        self.rejected_early = self.depth > self.tolerance;
        self.rejected_early
    }

    /// 🧭️ The current `(probe part, solid part)` pair, advancing the part cursors past exhausted `count`s; `None` once every
    /// pair is done.
    pub(super) fn parts<'a>(&mut self, probe: &'a CollisionBody, solid: &'a CollisionBody, count: impl Fn(&CollisionShape) -> usize) -> Option<(&'a CollisionMeshPart, &'a CollisionMeshPart)> {
        loop {
            let probe_part = probe.parts.get(self.part_a)?;
            let Some(solid_part) = solid.parts.get(self.part_b) else {
                self.part_a += 1;
                self.part_b = 0;
                self.cursor = 0;
                continue;
            };
            if self.cursor >= count(&probe_part.shape) {
                self.part_b += 1;
                self.cursor = 0;
                continue;
            }
            return Some((probe_part, solid_part));
        }
    }

    fn step_vertex(&mut self, probe: &CollisionBody, probe_world: &Pose3d, solid: &CollisionBody, solid_world: &Pose3d, next: CollisionPenetrationStage) -> CollisionStepResult {
        let Some((probe_part, solid_part)) = self.parts(probe, solid, |shape| shape.shape.vertex_count()) else { return self.advance(next) };
        let probe_pose = probe_world.semio_compose_rs(&probe_part.local_pose).0;
        let solid_pose = solid_world.semio_compose_rs(&solid_part.local_pose).0;
        let point = probe_part.shape.shape.world_vertex(probe_pose, self.cursor);
        self.cursor += 1;
        if self.probe(point, &CollisionAabb::from_body(solid, solid_world), solid_pose, &solid_part.shape) { self.finish() } else { CollisionStepResult::Pending }
    }

    fn step_inset(&mut self, probe: &CollisionBody, probe_world: &Pose3d, solid: &CollisionBody, solid_world: &Pose3d, next: CollisionPenetrationStage) -> CollisionStepResult {
        let Some((probe_part, solid_part)) = self.parts(probe, solid, |shape| shape.shape.triangle_count()) else { return self.advance(next) };
        let probe_pose = probe_world.semio_compose_rs(&probe_part.local_pose).0;
        let solid_pose = solid_world.semio_compose_rs(&solid_part.local_pose).0;
        let [a, b, c] = probe_part.shape.shape.world_triangle_outward(probe_pose, self.cursor);
        self.cursor += 1;
        let Some(outward) = (b - a).cross(c - a).try_normalize(1e-12) else { return CollisionStepResult::Pending };
        let centroid = rigid::Point3::new((a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0, (a.z + b.z + c.z) / 3.0);
        let inset = centroid + outward * -(2.0 * self.tolerance.clamp(COLLISION_INSET_MIN_M, COLLISION_INSET_MAX_M) as f32);
        if self.probe(inset, &CollisionAabb::from_body(solid, solid_world), solid_pose, &solid_part.shape) { self.finish() } else { CollisionStepResult::Pending }
    }

    fn step_face(&mut self, probe: &CollisionBody, probe_world: &Pose3d, solid: &CollisionBody, solid_world: &Pose3d, next: CollisionPenetrationStage) -> CollisionStepResult {
        let Some((probe_part, solid_part)) = self.parts(probe, solid, |shape| shape.shape.triangle_count()) else { return self.advance(next) };
        let probe_pose = probe_world.semio_compose_rs(&probe_part.local_pose).0;
        let solid_pose = solid_world.semio_compose_rs(&solid_part.local_pose).0;
        let triangle = probe_part.shape.shape.world_triangle(probe_pose, self.cursor);
        self.cursor += 1;
        let bounds = CollisionAabb::from_body(solid, solid_world);
        let mut near = Vec::new();
        solid_part.shape.shape.triangles_near(solid_pose, triangle, self.tolerance as f32, &mut near);
        precompute_work(near.len() + 1);
        for index in near {
            let face = solid_part.shape.shape.world_triangle_outward(solid_pose, index as usize);
            let clipped = collision::clip_behind_triangle(triangle, face);
            // 📏️ A clipped point sits behind `face` inside its prism, so its distance to the solid's surface is at most its
            // distance to that face's plane: a polygon whose every corner is within the tolerance of the plane cannot probe
            // deeper, and skips the surface queries.
            let normal = (face[1] - face[0]).cross(face[2] - face[0]);
            let Some(unit) = normal.try_normalize(1e-12) else { continue };
            let deepest = clipped.points().iter().map(|point| f64::from(-unit.dot(*point - face[0]))).fold(0.0, f64::max);
            if deepest <= COLLISION_CONTACT_NOISE_M || deepest <= self.depth {
                continue;
            }
            // 📍️ Depth into a solid is the smallest distance over ALL its faces, which peaks inside the polygon (an edge cutting
            // into a face between two other faces reads 0 at both ends and its full depth at the middle), so the polygon's
            // edges and centroid are probed too.
            let points = clipped.points();
            let mut centroid = rigid::Vector3::new(0.0, 0.0, 0.0);
            for (index, point) in points.iter().enumerate() {
                centroid = centroid + point.coords();
                let to = points[(index + 1) % points.len()];
                for step in 0..COLLISION_EDGE_PROBES {
                    let along = *point + (to - *point) * (step as f32 / COLLISION_EDGE_PROBES as f32);
                    if self.probe(along, &bounds, solid_pose, &solid_part.shape) {
                        return self.finish();
                    }
                }
            }
            if self.probe(rigid::Point3::from_coords(centroid * (1.0 / points.len() as f32)), &bounds, solid_pose, &solid_part.shape) {
                return self.finish();
            }
        }
        CollisionStepResult::Pending
    }
}
//#endregion ⏳️OverlapStateMachine
//#endregion 🔖️Collision
