//! 🖇️ Imprint face pairs: the pending imprint record, the face pair imprint through the surface intersection and the range splitting. See [`super`].

use super::*;

// #region 🔖️ExactImprintEngine
/// 🔀 One clipped piece of an intersection curve queued for imprint on one original face: `edge_id` is shared verbatim between the
/// two originating faces' queues (built once, used twice), so the two resulting halves share topology, not just geometry.
/// `pcurve_id` over `prange` is this face's own p-curve of that edge, in the edge's own order.
pub(super) struct Pending {
    pub(super) edge_id: EdgeId,
    pub(super) pcurve_id: Curve2Id,
    pub(super) prange: (f64, f64),
}

/// 🔀 One (face-of-A, face-of-B) imprint unit: intersect the two supports, clip every resulting
/// curve to both trims, and queue the imprint each side owes. The atomic unit of
/// [`BooleanPhase::Imprint`] — extracted verbatim out of the former `exact_imprint_boolean`'s inner
/// loop so the budgeted walk and the unbudgeted one run the SAME code (ticket
/// `26/09/09/PROCEDURAL-3D-END-TO-END`). A pair with nothing to imprint returns `Ok(())`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
#[allow(clippy::too_many_arguments)]
pub(super) fn imprint_face_pair(
    body: &mut Body,
    (fa, fb): (FaceId, FaceId),
    (sa, bb_a): (&Surface, &semio_framework_3d_brep_representation::contract::Aabb),
    tol: f64,
    weld: &mut Vec<(Pnt3, VertexId)>,
    (pending_a, pending_b): (&mut HashMap<FaceId, Vec<Pending>>, &mut HashMap<FaceId, Vec<Pending>>),
    rec: &mut OpRecorder,
) -> Result<(), KernelError> {
    let Ok(bb_b) = face_aabb(body, fb) else { return Ok(()) };
    if !aabb_overlap(bb_a, &bb_b, tol) {
        return Ok(());
    }
    let Some(face_b) = body.faces.get(fb).cloned() else { return Ok(()) };
    let Some(sb) = body.surfaces.get(face_b.surface).cloned() else { return Ok(()) };
    let Ok(curves) = intersect_surface_surface(sa, &sb, tol) else { return Ok(()) };
    for ic in &curves {
        // A near-tangent pair (e.g. two spheres offset by barely more than the sum of
        // their radii) can still produce a genuine but near-zero-radius contact circle
        // from `intersect_surface_surface`'s own tolerant overlap test — a real curve
        // object, not a numerical error, but one whose enclosed area/length is below the
        // tolerance at which its precise topology (which side of the seam it grazes, its
        // own trim gap) is even meaningful. Physically this is single-point external
        // tangency: zero intersection area, nothing to imprint. Skipping it here (rather
        // than imprinting a degenerate sliver) is what keeps the tangent-sphere union's
        // volume the exact, un-carved sum of both spheres.
        let bracket = intcurve_finite_bracket(body, ic, fa, fb);
        if curve3_extent(&ic.curve3, bracket) <= tol {
            continue;
        }
        for (t0, t1, full_period, touches) in clip_intcurve_to_faces(body, ic, fa, fb, tol) {
            if (t1 - t0).abs() < 1e-9 {
                continue;
            }
            let p0 = ic.curve3.eval(t0);
            let p1 = ic.curve3.eval(t1);
            if !full_period && p0.distance(p1) <= tol.max(1e-9) {
                continue; // degenerate near-zero chord — nothing useful to imprint
            }
            let along_a = coincident_boundary_edge(body, fa, ic, (t0, t1), tol);
            let along_b = coincident_boundary_edge(body, fb, ic, (t0, t1), tol);
            let (t0, t1) = if full_period {
                if let Some(edge) = along_a.or(along_b).and_then(|id| body.edges.get(id)) {
                    let point = body.vertices.get(edge.v0).ok_or_else(|| KernelError::MissingEntity(format!("vertex {}", edge.v0)))?.position;
                    let anchor = closest_parameter(&ic.curve3, (t0, t1), point, tol).t;
                    (anchor, anchor + (t1 - t0))
                } else { (t0, t1) }
            } else { (t0, t1) };
            // A segment can run exactly ALONG one operand's pre-existing boundary edge (a plane through a sphere's centre
            // containing its axis meets it in that sphere's own seam meridian). Imprinting a second, geometrically identical
            // edge there would leave duplicate topology, so the existing edge is subdivided and REUSED as the shared edge,
            // and that operand queues no imprint of its own — the boundary it needs is already there.
            let endpoints = (ic.curve3.eval(t0), ic.curve3.eval(t1));
            // 🧱 A segment running along BOTH operands' boundaries (two boxes flush on a face: the stock's side face meets the
            // tool's top face exactly along the stock's own top edge) is already topology on each side; neither face needs an
            // imprint, only the two boundary subedges must become ONE edge so the pieces stitch by shared-edge adjacency.
            if let (Some(existing_a), Some(existing_b)) = (along_a, along_b) {
                let ea = if full_period { existing_a } else { boundary_subedge(body, fa, endpoints, (tol, weld), rec)? };
                let eb = if full_period { existing_b } else { boundary_subedge(body, fb, endpoints, (tol, weld), rec)? };
                if full_period { unify_closed_boundary_vertices(body, eb, ea, tol, weld)?; }
                if ea != eb {
                    unify_boundary_edges(body, eb, ea)?;
                }
                continue;
            }
            if along_a.is_some() || along_b.is_some() {
                let edge_id = match (along_a, along_b) {
                    (Some(existing), _) => if full_period { existing } else { boundary_subedge(body, fa, endpoints, (tol, weld), rec)? },
                    (None, Some(existing)) => if full_period { existing } else { boundary_subedge(body, fb, endpoints, (tol, weld), rec)? },
                    (None, None) => unreachable!("one operand runs along the segment"),
                };
                let reversed = if full_period {
                    let tangent = body.edges.get(edge_id).and_then(|edge| body.curves3.get(edge.curve).and_then(|curve| curve.tangent(edge.range.0)));
                    tangent.zip(ic.curve3.tangent(t0)).is_some_and(|(a, b)| a.dot(b) < 0.0)
                } else {
                    oriented_prange(body, edge_id, endpoints, (t0, t1)).0 != t0
                };
                let orient = |(s0, s1): (f64, f64)| if reversed { (s1, s0) } else { (s0, s1) };
                if along_a.is_none() {
                    let (curve, range) = pcurve_for_clip(sa, &ic.curve3, &ic.pcurve_a, (t0, t1), tol);
                    pending_a.entry(fa).or_default().push(Pending { edge_id, pcurve_id: body.curves2.insert(curve), prange: orient(range) });
                }
                if along_b.is_none() {
                    let (curve, range) = pcurve_for_clip(&sb, &ic.curve3, &ic.pcurve_b, (t0, t1), tol);
                    pending_b.entry(fb).or_default().push(Pending { edge_id, pcurve_id: body.curves2.insert(curve), prange: orient(range) });
                }
                continue;
            }
            // 🧭️ A fresh imprint is cut into pieces wherever it meets either face's boundary — a periodic seam it crosses, a pole it
            // passes through — so every piece lies inside one chart of both faces and the arrangement sees a plain planar map.
            let marks = if full_period { touches } else { boundary_touch_parameters(body, ic, (fa, fb), (t0, t1), false, tol) };
            for (r0, r1, closed) in split_ranges((t0, t1), full_period, &marks) {
                let edge_id = build_imprint_edge(body, ic, (r0, r1), closed, (tol, weld), rec);
                let (curve_a, range_a) = pcurve_for_clip(sa, &ic.curve3, &ic.pcurve_a, (r0, r1), tol);
                let (curve_b, range_b) = pcurve_for_clip(&sb, &ic.curve3, &ic.pcurve_b, (r0, r1), tol);
                pending_a.entry(fa).or_default().push(Pending { edge_id, pcurve_id: body.curves2.insert(curve_a), prange: range_a });
                pending_b.entry(fb).or_default().push(Pending { edge_id, pcurve_id: body.curves2.insert(curve_b), prange: range_b });
            }
        }
    }
    Ok(())
}

/// 🧭️ The sub-ranges a fresh imprint over `(t0, t1)` is built from when it must be cut at `marks`: an open run splits at every
/// interior mark; a closed one is re-anchored at its first mark and splits at the rest (a single mark leaves one closed edge
/// whose vertex sits on the boundary, none leaves the plain closed hole).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn split_ranges((t0, t1): (f64, f64), closed: bool, marks: &[f64]) -> Vec<(f64, f64, bool)> {
    let period = t1 - t0;
    let mut cuts: Vec<f64> = marks.iter().map(|&m| if closed { t0 + (m - t0).rem_euclid(period) } else { m }).filter(|&m| closed || (m > t0 + 1e-9 && m < t1 - 1e-9)).collect();
    cuts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    if closed {
        match cuts.len() {
            0 => vec![(t0, t1, true)],
            1 => vec![(cuts[0], cuts[0] + period, true)],
            n => (0..n).map(|i| (cuts[i], if i + 1 < n { cuts[i + 1] } else { cuts[0] + period }, false)).collect(),
        }
    } else {
        let mut bounds = vec![t0];
        bounds.extend(cuts);
        bounds.push(t1);
        bounds.windows(2).map(|w| (w[0], w[1], false)).collect()
    }
}
// #endregion 🔖️Imprint
