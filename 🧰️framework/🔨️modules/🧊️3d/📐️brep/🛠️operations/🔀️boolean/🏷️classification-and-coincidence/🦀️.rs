//! 🏷️ Classification and coincidence: the interior face point, the local point in solid test, the face classification and keep rules and the coincident face detection. See [`super`].

use super::*;

// #region 🔖️Scoping
// #region 🔖️Classify
/// 🔀 A UV point inside `face`'s own trim (outer minus every hole), for use as a representative
/// 3D sample when classifying the whole piece: the outer loop's sampled centroid first, then a
/// coarse grid scan of the loop's UV bounding box.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn interior_point_of_face(body: &Body, face: FaceId, tol: f64) -> Option<Pnt3> {
    let face_data = body.faces.get(face)?;
    let surface = body.surfaces.get(face_data.surface)?.clone();
    let outer = face_data.outer?;
    let pts = sample_loop_uv(body, outer, &surface, 12);
    if pts.is_empty() {
        return None;
    }
    let cx = pts.iter().map(|p| p.x).sum::<f64>() / pts.len() as f64;
    let cy = pts.iter().map(|p| p.y).sum::<f64>() / pts.len() as f64;
    let (mut umin, mut umax, mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    for p in &pts {
        umin = umin.min(p.x);
        umax = umax.max(p.x);
        vmin = vmin.min(p.y);
        vmax = vmax.max(p.y);
    }
    if point_in_face_uv_periodic(body, face, Pnt2::new(cx, cy), tol) {
        return Some(surface.eval(cx, cy));
    }
    const STEPS: usize = 16;
    for i in 0..STEPS {
        for j in 0..STEPS {
            let u = umin + (umax - umin) * (i as f64 + 0.5) / STEPS as f64;
            let v = vmin + (vmax - vmin) * (j as f64 + 0.5) / STEPS as f64;
            if point_in_face_uv_periodic(body, face, Pnt2::new(u, v), tol) {
                return Some(surface.eval(u, v));
            }
        }
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn sample_loop_uv(body: &Body, loop_id: LoopId, surface: &Surface, per_edge: usize) -> Vec<Pnt2> {
    let mut pts = Vec::new();
    let mut prev_u: Option<f64> = None;
    let mut prev_v: Option<f64> = None;
    for cid in body.loop_coedges(loop_id) {
        let Some(co) = body.coedges.get(cid) else { continue };
        let Some(pc_id) = co.pcurve else { continue };
        let Some(pc) = body.curves2.get(pc_id) else { continue };
        let (t0, t1) = if co.forward { co.prange } else { (co.prange.1, co.prange.0) };
        for k in 0..per_edge {
            let s = k as f64 / per_edge as f64;
            let mut p = pc.eval(t0 + (t1 - t0) * s);
            // Unwrap continuously across coedges (matching `classification::loop_uv_polygon_sampled`'s
            // own periodic handling) so a loop that grazes a periodic seam still yields a compact,
            // consistently-signed UV bounding box instead of a bogus near-full-domain one.
            if surface.is_u_periodic() {
                if let Some(pu) = prev_u {
                    p.x = unwrap_to(pu, p.x);
                }
                prev_u = Some(p.x);
            }
            if surface.is_v_periodic() {
                if let Some(pv) = prev_v {
                    p.y = unwrap_to(pv, p.y);
                }
                prev_v = Some(p.y);
            }
            pts.push(p);
        }
    }
    pts
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn unwrap_to(prev: f64, u: f64) -> f64 {
    let tau = std::f64::consts::TAU;
    let diff = u - prev;
    prev + diff - tau * ((diff + std::f64::consts::PI) / tau).floor()
}

/// 🔀 Six fixed irrational-ish ray directions for [`local_point_in_solid`]'s retry consensus —
/// same role as `classification.rs`'s own private `RAY_RETRY_DIRS` (kept independent since that
/// one isn't `pub`).
/// 🐛 All six used to have a POSITIVE `z`, so every retry left the query point through the same
/// hemisphere: one damaged face in that hemisphere loses every vote at once, and the classifier
/// reports `Outside` unanimously rather than disagreeing. Half of them are negated here so the set
/// spans both hemispheres and no single face can shadow the whole consensus.
const LOCAL_RAY_RETRY_DIRS: [[f64; 3]; 6] = [
    [0.573_576_436_351_046, 0.740_535_693_464_567_5, 0.350_889_803_483_932_2],
    [0.350_889_803_483_932_2, -0.573_576_436_351_046, -0.740_535_693_464_567_5],
    [0.267_261_241_941_149_4, 0.534_522_483_882_298_8, 0.801_783_725_737_219],
    [-0.577_350_269_189_625_8, -0.577_350_269_189_625_8, -0.577_350_269_189_625_7],
    [0.308_608_313_448_298, 0.904_511_432_523_735, 0.293_892_626_045_885],
    [-0.843_391_445_261_857, -0.214_298_755_144_806, -0.491_975_172_042_98],
];

/// 🔀 Local replacement for `classification::point_in_solid`, needed to work around a confirmed
/// bug there rather than editing that file (outside this file's ownership per the ticket brief —
/// see the report for the exact diff `classification.rs` would need instead).
/// `point_in_face_trim_status`'s `Surface::Sphere` branch builds its own trim "polygon" from the
/// face's real 3D VERTICES (`face_boundary_points`) — correct for the pre-this-ticket
/// two-hemisphere sphere topology, but wrong for W1-E's new single-face/one-seam sphere (only 2
/// real vertices: the two poles), where that "polygon" degenerates to two collinear points, so
/// `point_in_polygon_3d` returns `false` for every hit and EVERY ray reports zero crossings.
/// Confirmed directly (not assumed): `point_in_solid` on a bare, unsplit unit sphere returns
/// `Outside` for `(0,0,0)` (its own center) and for `(2,0,0)` (clearly outside) identically. This
/// reimplements the same multi-ray-parity vote, but routes every face's trim test through
/// `classification::point_in_face_uv` (the UV-sampled path, unaffected by the bug above) using the
/// `(u, v)` [`intersect_curve_surface`] already returns per hit, instead of the broken 3D-vertex
/// shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn local_point_in_solid(body: &Body, solid: SolidId, point: Pnt3, tol: f64) -> Result<PointClassification, KernelError> {
    let (_, dist) = closest_point_on_solid(body, solid, point)?;
    if dist <= tol {
        return Ok(PointClassification::OnBoundary);
    }
    let faces = body.solid_faces(solid);
    let mut inside_votes = 0u32;
    let mut outside_votes = 0u32;
    for raw_dir in LOCAL_RAY_RETRY_DIRS {
        let dir = Vec3::new(raw_dir[0], raw_dir[1], raw_dir[2]);
        let ray = semio_framework_3d_brep_curve::Curve3::Line { origin: point, dir };
        let mut grazing = false;
        let mut hits: Vec<f64> = Vec::new();
        for &face in &faces {
            let Some(face_data) = body.faces.get(face) else { continue };
            let Some(surface) = body.surfaces.get(face_data.surface) else { continue };
            let face_hits = match intersect_curve_surface(&ray, surface, tol) {
                Ok(h) => h,
                Err(_) => {
                    grazing = true;
                    break;
                }
            };
            for h in face_hits {
                if h.t <= 1e-9 {
                    continue;
                }
                if let Some(n) = surface.normal(h.u, h.v) {
                    if let Some(nn) = n.normalized() {
                        if nn.dot(dir).abs() < 1e-6 {
                            grazing = true;
                            break;
                        }
                    }
                }
                // 🌀 The trim test must try the hit's UV at every period shift: a split piece of a
                // periodic face keeps its own boundary polygon on whatever `2π` branch its imprint
                // settled on (the piece ABOVE a seam-crossing circle on a cylinder), and a single
                // canonically-wrapped probe missed it — every ray through a blind bore's upper
                // lateral piece counted zero crossings and the bore's floor disc classified Outside.
                if point_in_face_uv_periodic(body, face, Pnt2::new(h.u, h.v), tol) {
                    hits.push(h.t);
                }
            }
            if grazing {
                break;
            }
        }
        if grazing {
            continue;
        }
        hits.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let merge_tol = tol * 10.0;
        let mut crossings = 0u32;
        let mut last: Option<f64> = None;
        for t in hits {
            if last.is_none_or(|l| (t - l).abs() > merge_tol) {
                crossings += 1;
                last = Some(t);
            }
        }
        if crossings % 2 == 1 {
            inside_votes += 1;
        } else {
            outside_votes += 1;
        }
        if inside_votes >= 2 {
            return Ok(PointClassification::Inside);
        }
        if outside_votes >= 2 {
            return Ok(PointClassification::Outside);
        }
    }
    if inside_votes > outside_votes {
        Ok(PointClassification::Inside)
    } else if outside_votes > inside_votes {
        Ok(PointClassification::Outside)
    } else {
        Err(KernelError::Operation("local point classification: every retry direction was grazing or degenerate".into()))
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn classify_face_against_solid(body: &Body, face: FaceId, other: SolidId, tol: f64) -> Result<PointClassification, KernelError> {
    let Some(p) = interior_point_of_face(body, face, tol) else {
        return Err(KernelError::Boolean(BooleanError::ClassificationAmbiguous(format!("no interior UV sample found for face {face}"))));
    };
    local_point_in_solid(body, other, p, tol)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn keep_face(op: BooleanOp, from_a: bool, class: PointClassification) -> bool {
    match op {
        BooleanOp::Unite => matches!(class, PointClassification::Outside | PointClassification::OnBoundary),
        BooleanOp::Intersect => matches!(class, PointClassification::Inside | PointClassification::OnBoundary),
        BooleanOp::Cut => {
            if from_a {
                matches!(class, PointClassification::Outside | PointClassification::OnBoundary)
            } else {
                matches!(class, PointClassification::Inside)
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn flip_face(body: &mut Body, face: FaceId) {
    if let Some(f) = body.faces.get_mut(face) {
        f.flipped = !f.flipped;
    }
}
// #endregion 🔖️Classify

// #region 🔖️Coincident
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
/// ↔️ Selects the surviving operand boundary for a coincident face pair.
pub(super) fn keep_coincident_boundary(op: BooleanOp, aligned: bool) -> bool {
    match op { BooleanOp::Unite | BooleanOp::Intersect => aligned, BooleanOp::Cut => !aligned }
}

/// 🧭️ Compares outward normals at the same boundary point on both analytic surfaces.
fn coincident_face_alignment(body: &Body, fa: FaceId, fb: FaceId, tol: f64) -> Option<bool> {
    let a = body.faces.get(fa)?;
    let b = body.faces.get(fb)?;
    let sa = body.surfaces.get(a.surface)?;
    let sb = body.surfaces.get(b.surface)?;
    body.face_coedges(fa).into_iter().find_map(|coedge| {
        let edge = body.edges.get(body.coedges.get(coedge)?.edge)?;
        let point = body.curves3.get(edge.curve)?.eval((edge.range.0 + edge.range.1) * 0.5);
        let ua = closest_uv(sa, sa.domain(), point, tol);
        let ub = closest_uv(sb, sb.domain(), point, tol);
        let na = sa.normal(ua.u, ua.v)? * if a.flipped { -1.0 } else { 1.0 };
        let nb = sb.normal(ub.u, ub.v)? * if b.flipped { -1.0 } else { 1.0 };
        Some(na.dot(nb) > 0.0)
    })
}

pub(super) fn find_coincident_face_pairs(body: &Body, faces_a: &[FaceId], faces_b: &[FaceId], tol: f64) -> Vec<(FaceId, FaceId, bool)> {
    let mut out = Vec::new();
    for &fa in faces_a {
        let Some(face_a) = body.faces.get(fa) else { continue };
        let Some(outer_a) = face_a.outer else { continue };
        let Some(sa) = body.surfaces.get(face_a.surface) else { continue };
        for &fb in faces_b {
            let Some(face_b) = body.faces.get(fb) else { continue };
            let Some(outer_b) = face_b.outer else { continue };
            let Some(sb) = body.surfaces.get(face_b.surface) else { continue };
            if surfaces_equal(sa, sb, tol) && loops_coincide(body, outer_a, outer_b, tol) {
                if let Some(aligned) = coincident_face_alignment(body, fa, fb, tol) { out.push((fa, fb, aligned)); }
            }
        }
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surfaces_equal(sa: &Surface, sb: &Surface, tol: f64) -> bool {
    match (sa, sb) {
        (Surface::Plane { frame: fa }, Surface::Plane { frame: fb }) => fa.z.dot(fb.z).abs() > 1.0 - 1e-9 && (fa.origin - fb.origin).dot(fa.z).abs() < tol,
        (Surface::Cylinder { frame: fa, radius: ra }, Surface::Cylinder { frame: fb, radius: rb }) => (ra - rb).abs() < tol && fa.z.cross(fb.z).norm() < 1e-9 && (fa.origin - fb.origin).cross(fa.z).norm() < tol,
        (Surface::Sphere { frame: fa, radius: ra }, Surface::Sphere { frame: fb, radius: rb }) => (ra - rb).abs() < tol && (fa.origin - fb.origin).norm() < tol,
        _ => false,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loop_positions(body: &Body, loop_id: LoopId) -> Vec<Pnt3> {
    body.loop_coedges(loop_id).into_iter().filter_map(|c| body.coedge_endpoints(c)).filter_map(|(v, _)| body.vertices.get(v).map(|x| x.position)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn loops_coincide(body: &Body, la: LoopId, lb: LoopId, tol: f64) -> bool {
    let pa = loop_positions(body, la);
    let pb = loop_positions(body, lb);
    if pa.is_empty() || pa.len() != pb.len() {
        return false;
    }
    pa.iter().all(|p| pb.iter().any(|q| p.distance(*q) <= tol))
}
// #endregion 🔖️Coincident
// #endregion 🔖️ExactImprintEngine
