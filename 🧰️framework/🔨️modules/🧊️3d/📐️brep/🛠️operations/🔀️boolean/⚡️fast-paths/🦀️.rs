//! ⚡️ Fast paths: the trivial disjoint and contained topology fast path and the exact box fast path with their inside and box tests. See [`super`].

use super::*;

// #region 🔖️TrivialFastPath
/// 🔀 Checks that don't depend on box-ness at all: disjoint operands (gap ≥ tol) and full
/// containment (one operand's boundary lies entirely inside/outside the other), verified with a
/// real [`point_in_solid`] probe of several boundary points, not just AABB containment (AABB
/// containment alone is necessary but not sufficient — used only to decide whether the probe is
/// worth running).
/// 🐛 Every branch that "returns an operand" returns a DEEP COPY of it ([`copy_solid`],
/// [`detached_copy_of_outer_faces`]). The former `clone_solid_shells` minted a second solid/shell
/// wrapper over the operand's OWN faces, so one face belonged to two shells at once and the
/// operand's lifetime became the result's — see this module's header.
/// 🕳️ A tool strictly inside the target cuts a CAVITY: the tool's faces become an inner shell wound
/// inward (every face flipped), so the void's signed volume opposes the outer shell's. Unflipped,
/// every such cut (`sphere-cut-with-torus` at any radius past the torus) failed validation with
/// `void-shell-not-inverted` and the preview had nothing to paint.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn trivial_topology_fast_path(body: &mut Body, a: SolidId, b: SolidId, (bb_a, bb_b): (&AxisAlignedBox, &AxisAlignedBox), op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
    let gap = aabb_gap(bb_a, bb_b);
    if gap >= tol {
        return match op {
            BooleanOp::Unite => {
                let mut faces = detached_copy_of_outer_faces(body, a, rec)?;
                faces.extend(detached_copy_of_outer_faces(body, b, rec)?);
                Ok(Some(solid_from_outer_faces(body, faces, Vec::new(), rec)?))
            }
            BooleanOp::Cut => Ok(Some(copy_solid(body, a, rec)?)),
            BooleanOp::Intersect => Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty (operands disjoint)".into()))),
        };
    }
    if aabb_contains(bb_b, bb_a, tol) && solid_wholly_inside(body, a, b, tol)? {
        return match op {
            BooleanOp::Unite => Ok(Some(copy_solid(body, b, rec)?)),
            BooleanOp::Intersect => Ok(Some(copy_solid(body, a, rec)?)),
            BooleanOp::Cut => Err(KernelError::Boolean(BooleanError::InvalidResult("boolean cut is empty (tool contains target)".into()))),
        };
    }
    if aabb_contains(bb_a, bb_b, tol) && solid_wholly_inside(body, b, a, tol)? {
        return match op {
            BooleanOp::Unite => Ok(Some(copy_solid(body, a, rec)?)),
            BooleanOp::Intersect => Ok(Some(copy_solid(body, b, rec)?)),
            BooleanOp::Cut if solid_strictly_inside(body, b, a, tol)? => {
                let outer = detached_copy_of_outer_faces(body, a, rec)?;
                let inner = detached_copy_of_outer_faces(body, b, rec)?;
                for &face in &inner {
                    flip_face(body, face);
                }
                Ok(Some(solid_from_outer_faces(body, outer, vec![inner], rec)?))
            }
            BooleanOp::Cut => Ok(None),
        };
    }
    Ok(None)
}

/// 🔀 `true` when every boundary vertex of `inner` classifies strictly `Inside` `outer` — no vertex on
/// its boundary — so `inner` can only ever be a genuine cavity of `outer`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn solid_strictly_inside(body: &Body, inner: SolidId, outer: SolidId, tol: f64) -> Result<bool, KernelError> {
    let points = solid_probe_points(body, inner, tol);
    if points.is_empty() {
        return Ok(false);
    }
    for p in points {
        if !matches!(local_point_in_solid(body, outer, p, tol)?, PointClassification::Inside) {
            return Ok(false);
        }
    }
    Ok(true)
}

/// 🔀 `true` when every boundary vertex of `inner` classifies as `Inside` or `OnBoundary` against
/// `outer` — a real (if sampling-based) containment proof, not an AABB heuristic.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn solid_wholly_inside(body: &Body, inner: SolidId, outer: SolidId, tol: f64) -> Result<bool, KernelError> {
    let points = solid_probe_points(body, inner, tol);
    if points.is_empty() {
        return Ok(false);
    }
    for p in points {
        if !matches!(local_point_in_solid(body, outer, p, tol)?, PointClassification::Inside | PointClassification::OnBoundary) {
            return Ok(false);
        }
    }
    Ok(true)
}
// #endregion 🔖️TrivialFastPath

// #region 🔖️BoxFastPath
/// 🔀 When BOTH operands are genuinely axis boxes (6 faces, volume matching their AABB volume),
/// `Intersect` is always another axis box (the AABB intersection), and `Unite` is one too — but
/// ONLY when the two boxes agree on two of the three axis intervals and merely extend each other
/// along the third ([`boxes_union_is_a_box`]); a general partial overlap unions into an L/cross
/// shape, which this shortcut must hand back to the general imprint engine rather than silently
/// returning the (strictly larger) AABB union. `Cut` is likewise left to the general engine.
/// 🐛 The result is built at the union/intersection's own min corner: [`make_box`] always builds
/// at the origin, so the shortcut used to return a correctly-SIZED box in the wrong PLACE
/// whenever either operand was not itself origin-anchored — invisible to a volume-only assertion.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn box_fast_path(body: &mut Body, a: SolidId, b: SolidId, (bb_a, bb_b): (&AxisAlignedBox, &AxisAlignedBox), op: BooleanOp, tol: f64, rec: &mut OpRecorder) -> Result<Option<SolidId>, KernelError> {
    if !matches!(op, BooleanOp::Unite | BooleanOp::Intersect) {
        return Ok(None);
    }
    if !(is_aabb_box_solid(body, a, bb_a)? && is_aabb_box_solid(body, b, bb_b)?) {
        return Ok(None);
    }
    let target = match op {
        BooleanOp::Unite => {
            if !boxes_union_is_a_box(bb_a, bb_b, tol) {
                return Ok(None);
            }
            aabb_union(bb_a, bb_b)
        }
        BooleanOp::Intersect => {
            let Some(inter) = aabb_intersection(bb_a, bb_b) else {
                return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty".into())));
            };
            let (w, d, h) = aabb_dims(&inter);
            if w <= tol || d <= tol || h <= tol {
                return Err(KernelError::Boolean(BooleanError::InvalidResult("boolean intersect is empty within tolerance".into())));
            }
            inter
        }
        BooleanOp::Cut => unreachable!(),
    };
    let (w, d, h) = aabb_dims(&target);
    let built = make_box(body, w, d, h, rec)?;
    let origin = Vec3::new(target.min.x, target.min.y, target.min.z);
    if origin.norm_sq() <= 0.0 {
        return Ok(Some(built));
    }
    Ok(Some(transform_solid(body, built, &Affine3::translation(origin), rec)?))
}

/// 🔀 `true` when two axis boxes' union is itself an axis box: they must coincide on two of the
/// three axis intervals and overlap or abut along the remaining one. Any other configuration
/// unions into a non-box (L, cross, or two disjoint lumps).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn boxes_union_is_a_box(a: &AxisAlignedBox, b: &AxisAlignedBox, tol: f64) -> bool {
    let spans = [(a.min.x, a.max.x, b.min.x, b.max.x), (a.min.y, a.max.y, b.min.y, b.max.y), (a.min.z, a.max.z, b.min.z, b.max.z)];
    let mut extended = 0usize;
    for (a0, a1, b0, b1) in spans {
        if (a0 - b0).abs() <= tol && (a1 - b1).abs() <= tol {
            continue;
        }
        if gap_1d(a0, a1, b0, b1) > tol {
            return false;
        }
        extended += 1;
    }
    extended <= 1
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_aabb_box_solid(body: &Body, solid: SolidId, bb: &AxisAlignedBox) -> Result<bool, KernelError> {
    let faces = body.solid_faces(solid);
    if faces.len() != 6 {
        return Ok(false);
    }
    let bv = aabb_volume(bb);
    if !(bv.is_finite() && bv > 0.0) {
        return Ok(false);
    }
    let v = solid_volume(body, solid, 1e-6)?;
    Ok((v - bv).abs() <= 1e-6)
}
// #endregion 🔖️BoxFastPath
