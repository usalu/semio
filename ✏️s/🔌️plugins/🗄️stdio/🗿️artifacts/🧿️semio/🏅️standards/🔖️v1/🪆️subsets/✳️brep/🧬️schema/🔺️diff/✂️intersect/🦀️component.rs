//! ✂️ Curve/curve, curve/surface, and surface/surface intersection math — three source files
//! folded into one compute subdir per the "one compute subdir, not a 1:1 file mapping"
//! precedent (see the `🔺️euler`/imprint compute dir). Each stays namespaced in its own inner
//! module because `newton_refine`/`intersect_general` private helper names collide between
//! `curve_curve` and `curve_surface`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/{✂️int-cc,✂️int-cs,✂️int-ss}/🦀️component.rs` in
//! ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL2.

pub mod curve_curve {
    //! ✂️ Curve/curve intersection (analytic + Bézier clipping).
    //!
    //! Analytic fast paths cover [`Curve3::Line`]/[`Curve3::Circle`] pairs; every other combination
    //! falls through to a NURBS representation that is either clipped with [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bezier`]
    //! control-hull subdivision or refined by Newton on sample seeds.
    //!
    //! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bezier::RationalBezier3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::insert_knot;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::{Curve3, NurbsCurve3};
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::IntersectError;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

    // #region 🔖️Api

    /// ✂️ An isolated curve/curve intersection: world point plus parameters on each operand.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct CurveCurveHit {
        pub point: Pnt3,
        pub t_a: f64,
        pub t_b: f64,
    }

    /// ✂️ Intersect two 3D curves within `tol`. Analytic for line/line and line/circle; general
    /// NURBS otherwise via Bézier control-hull clipping plus Newton refinement.
    pub async fn intersect_curve_curve(a: &Curve3, b: &Curve3, tol: f64) -> Result<Vec<CurveCurveHit>, IntersectError> {
        if !(tol.is_finite() && tol > 0.0) {
            return Err(IntersectError::Degenerate("tolerance must be positive and finite".into()));
        }
        match (a, b) {
            (Curve3::Line { origin: o1, dir: d1 }, Curve3::Line { origin: o2, dir: d2 }) => intersect_line_line(*o1, *d1, *o2, *d2, tol),
            (Curve3::Line { origin, dir }, Curve3::Circle { frame, radius }) => intersect_line_circle(*origin, *dir, frame, *radius, tol, false),
            (Curve3::Circle { frame, radius }, Curve3::Line { origin, dir }) => intersect_line_circle(*origin, *dir, frame, *radius, tol, true),
            (Curve3::Circle { frame: f1, radius: r1 }, Curve3::Circle { frame: f2, radius: r2 }) => intersect_circle_circle(f1, *r1, f2, *r2, tol),
            _ => intersect_general(a, b, tol),
        }
    }

    // #endregion 🔖️Api

    // #region 🔖️Analytic

    async fn intersect_line_line(o1: Pnt3, d1: Vec3, o2: Pnt3, d2: Vec3, tol: f64) -> Result<Vec<CurveCurveHit>, IntersectError> {
        let n1 = d1.norm();
        let n2 = d2.norm();
        if n1 <= tol || n2 <= tol {
            return Err(IntersectError::Degenerate("zero-length line direction".into()));
        }
        let w0 = o1 - o2;
        let a = d1.dot(d1);
        let b = d1.dot(d2);
        let c = d2.dot(d2);
        let d = d1.dot(w0);
        let e = d2.dot(w0);
        let denom = a * c - b * b;
        if denom.abs() <= tol * tol * a * c {
            let dist = w0.cross(d1).norm() / n1;
            if dist <= tol {
                return Err(IntersectError::Tangent);
            }
            return Ok(vec![]);
        }
        let t_a = (b * e - c * d) / denom;
        let t_b = (a * e - b * d) / denom;
        let p_a = o1 + d1 * t_a;
        let p_b = o2 + d2 * t_b;
        if p_a.distance(p_b) > tol {
            return Ok(vec![]);
        }
        Ok(vec![CurveCurveHit { point: Pnt3::new((p_a.x + p_b.x) * 0.5, (p_a.y + p_b.y) * 0.5, (p_a.z + p_b.z) * 0.5), t_a, t_b }])
    }

    async fn intersect_line_circle(origin: Pnt3, dir: Vec3, frame: &Frame3, radius: f64, tol: f64, swap: bool) -> Result<Vec<CurveCurveHit>, IntersectError> {
        if radius <= tol || dir.norm() <= tol {
            return Err(IntersectError::Degenerate("degenerate line or circle".into()));
        }
        let o = frame.to_local(origin);
        let d = frame.to_local_vector(dir);
        if d.z.abs() <= tol {
            if o.z.abs() > tol {
                return Ok(vec![]);
            }
            return intersect_line2_circle(o.x, o.y, d.x, d.y, radius, origin, dir, frame, tol, swap);
        }
        let t_plane = -o.z / d.z;
        let x = o.x + d.x * t_plane;
        let y = o.y + d.y * t_plane;
        let rho = (x * x + y * y).sqrt();
        if (rho - radius).abs() <= tol {
            let point = origin + dir * t_plane;
            let t_circle = y.atan2(x).rem_euclid(std::f64::consts::TAU);
            return Ok(vec![pack_hit(point, t_plane, t_circle, swap)]);
        }
        if rho > radius + tol {
            return Ok(vec![]);
        }
        intersect_line2_circle(o.x, o.y, d.x, d.y, radius, origin, dir, frame, tol, swap)
    }

    async fn intersect_line2_circle(ox: f64, oy: f64, dx: f64, dy: f64, radius: f64, origin: Pnt3, dir: Vec3, frame: &Frame3, tol: f64, swap: bool) -> Result<Vec<CurveCurveHit>, IntersectError> {
        let a = dx * dx + dy * dy;
        if a <= tol * tol {
            return Err(IntersectError::Degenerate("line direction parallel to circle normal with zero in-plane speed".into()));
        }
        let b = 2.0 * (ox * dx + oy * dy);
        let c = ox * ox + oy * oy - radius * radius;
        let disc = b * b - 4.0 * a * c;
        if disc < -(tol * tol) * a * a {
            return Ok(vec![]);
        }
        let sqrt_disc = disc.max(0.0).sqrt();
        let mut hits = Vec::with_capacity(2);
        for sign in [-1.0, 1.0] {
            let t = (-b + sign * sqrt_disc) / (2.0 * a);
            let x = ox + dx * t;
            let y = oy + dy * t;
            let point = origin + dir * t;
            if frame.to_local(point).z.abs() > tol * 10.0 {
                continue;
            }
            let t_circle = y.atan2(x).rem_euclid(std::f64::consts::TAU);
            let hit = pack_hit(point, t, t_circle, swap);
            if hits.iter().all(|h: &CurveCurveHit| h.point.distance(hit.point) > tol) {
                hits.push(hit);
            }
        }
        Ok(hits)
    }

    async fn pack_hit(point: Pnt3, t_line: f64, t_circle: f64, swap: bool) -> CurveCurveHit {
        if swap {
            CurveCurveHit { point, t_a: t_circle, t_b: t_line }
        } else {
            CurveCurveHit { point, t_a: t_line, t_b: t_circle }
        }
    }

    async fn intersect_circle_circle(f1: &Frame3, r1: f64, f2: &Frame3, r2: f64, tol: f64) -> Result<Vec<CurveCurveHit>, IntersectError> {
        if r1 <= tol || r2 <= tol {
            return Err(IntersectError::Degenerate("non-positive circle radius".into()));
        }
        if f1.z.cross(f2.z).norm() > tol {
            return intersect_general(&Curve3::Circle { frame: *f1, radius: r1 }, &Curve3::Circle { frame: *f2, radius: r2 }, tol);
        }
        let c1 = f1.origin;
        let c2 = f2.origin;
        let d_vec = c2 - c1;
        let d = d_vec.norm();
        if d <= tol && (r1 - r2).abs() <= tol {
            return Err(IntersectError::Tangent);
        }
        if d > r1 + r2 + tol || d < (r1 - r2).abs() - tol {
            return Ok(vec![]);
        }
        let den = d.max(tol);
        let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * den);
        let h_sq = r1 * r1 - a * a;
        if h_sq < -(tol * tol) {
            return Ok(vec![]);
        }
        let h = h_sq.max(0.0).sqrt();
        let mid = c1 + d_vec * (a / den);
        let n = f1.z.normalized().unwrap_or(Vec3::Z);
        let radial = (d_vec - n * d_vec.dot(n)).normalized().unwrap_or(f1.x);
        let perp = n.cross(radial);
        let mut hits = Vec::new();
        for sign in [-1.0, 1.0] {
            let point = mid + perp * (h * sign);
            let local1 = f1.to_local(point);
            let local2 = f2.to_local(point);
            let t_a = local1.y.atan2(local1.x).rem_euclid(std::f64::consts::TAU);
            let t_b = local2.y.atan2(local2.x).rem_euclid(std::f64::consts::TAU);
            let hit = CurveCurveHit { point, t_a, t_b };
            if hits.iter().all(|existing: &CurveCurveHit| existing.point.distance(hit.point) > tol) {
                hits.push(hit);
            }
        }
        Ok(hits)
    }

    // #endregion 🔖️Analytic

    // #region 🔖️General

    async fn intersect_general(a: &Curve3, b: &Curve3, tol: f64) -> Result<Vec<CurveCurveHit>, IntersectError> {
        let (dom_a, nurbs_a) = curve_as_nurbs(a, b, tol)?;
        let (dom_b, nurbs_b) = curve_as_nurbs(b, a, tol)?;
        let segs_a = nurbs_to_bezier_segments(&nurbs_a)?;
        let segs_b = nurbs_to_bezier_segments(&nurbs_b)?;
        let mut hits = Vec::new();
        for (bez_a, a0, a1) in &segs_a {
            for (bez_b, b0, b1) in &segs_b {
                clip_pair(bez_a, *a0, *a1, bez_b, *b0, *b1, a, b, tol, 0, &mut hits)?;
            }
        }
        if hits.is_empty() {
            sample_newton(a, dom_a, b, dom_b, tol, &mut hits);
        }
        merge_hits(&mut hits, tol);
        Ok(hits)
    }

    async fn curve_as_nurbs(curve: &Curve3, other: &Curve3, tol: f64) -> Result<((f64, f64), NurbsCurve3), IntersectError> {
        let domain = match curve {
            Curve3::Line { origin, dir } => line_domain_against(origin, dir, other, tol)?,
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => curve.domain(),
            Curve3::Nurbs { knots, .. } => knots.domain(),
        };
        if !domain.0.is_finite() || !domain.1.is_finite() || domain.1 <= domain.0 {
            return Err(IntersectError::Degenerate("unable to form a finite NURBS domain".into()));
        }
        Ok((domain, curve.to_nurbs(domain)))
    }

    async fn line_domain_against(origin: &Pnt3, dir: &Vec3, other: &Curve3, tol: f64) -> Result<(f64, f64), IntersectError> {
        let n = dir.norm();
        if n <= tol {
            return Err(IntersectError::Degenerate("zero-length line direction".into()));
        }
        let unit = *dir * (1.0 / n);
        let (t0, t1) = other.domain();
        let samples = if t0.is_finite() && t1.is_finite() {
            let mut ts = Vec::new();
            for i in 0..=16 {
                ts.push(t0 + (t1 - t0) * (i as f64 / 16.0));
            }
            ts
        } else {
            vec![0.0]
        };
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for t in samples {
            let p = other.eval(t);
            let s = (p - *origin).dot(unit);
            lo = lo.min(s);
            hi = hi.max(s);
        }
        let pad = ((hi - lo).abs() + 1.0).max(1.0);
        lo -= pad;
        hi += pad;
        Ok((lo / n, hi / n))
    }

    async fn nurbs_to_bezier_segments(nurbs: &NurbsCurve3) -> Result<Vec<(RationalBezier3, f64, f64)>, IntersectError> {
        let mut knots = nurbs.knots.clone();
        let mut hx: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.x * w).collect();
        let mut hy: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.y * w).collect();
        let mut hz: Vec<f64> = nurbs.controls.iter().zip(&nurbs.weights).map(|(p, w)| p.z * w).collect();
        let mut hw = nurbs.weights.clone();
        let p = knots.degree;
        let (d0, d1) = knots.domain();
        let mut unique: Vec<f64> = Vec::new();
        for &k in &knots.knots {
            if k > d0 + 1e-15 && k < d1 - 1e-15 && unique.last().map(|&u| (u - k).abs() > 1e-15).unwrap_or(true) {
                unique.push(k);
            }
        }
        for u in unique {
            while knots.multiplicity(u) < p {
                let (nk, nx) = insert_knot(&knots, &hx, u);
                let (_, ny) = insert_knot(&knots, &hy, u);
                let (_, nz) = insert_knot(&knots, &hz, u);
                let (_, nw) = insert_knot(&knots, &hw, u);
                knots = nk;
                hx = nx;
                hy = ny;
                hz = nz;
                hw = nw;
            }
        }
        let mut spans = Vec::new();
        let mut i = p;
        let last = knots.knots.len() - p - 1;
        while i < last {
            let u0 = knots.knots[i];
            let u1 = knots.knots[i + 1];
            if (u1 - u0).abs() > 1e-15 {
                let mut controls = Vec::with_capacity(p + 1);
                let mut weights = Vec::with_capacity(p + 1);
                for j in 0..=p {
                    let idx = i - p + j;
                    let w = hw[idx];
                    if w.abs() <= 1e-300 {
                        return Err(IntersectError::Degenerate("zero weight in NURBS segment".into()));
                    }
                    controls.push(Pnt3::new(hx[idx] / w, hy[idx] / w, hz[idx] / w));
                    weights.push(w);
                }
                spans.push((RationalBezier3::new(controls, weights), u0, u1));
            }
            i += 1;
            while i < last && (knots.knots[i + 1] - knots.knots[i]).abs() <= 1e-15 {
                i += 1;
            }
        }
        if spans.is_empty() {
            return Err(IntersectError::Unresolved("NURBS produced no Bézier spans".into()));
        }
        Ok(spans)
    }

    async fn boxes_overlap3(a: (Pnt3, Pnt3), b: (Pnt3, Pnt3), tol: f64) -> bool {
        a.0.x - tol <= b.1.x && b.0.x - tol <= a.1.x && a.0.y - tol <= b.1.y && b.0.y - tol <= a.1.y && a.0.z - tol <= b.1.z && b.0.z - tol <= a.1.z
    }

    async fn clip_pair(bez_a: &RationalBezier3, a0: f64, a1: f64, bez_b: &RationalBezier3, b0: f64, b1: f64, curve_a: &Curve3, curve_b: &Curve3, tol: f64, depth: u32, hits: &mut Vec<CurveCurveHit>) -> Result<(), IntersectError> {
        if !boxes_overlap3(bez_a.control_hull_box(), bez_b.control_hull_box(), tol) {
            return Ok(());
        }
        let span_a = (a1 - a0).abs();
        let span_b = (b1 - b0).abs();
        let (lo_a, hi_a) = bez_a.control_hull_box();
        let (lo_b, hi_b) = bez_b.control_hull_box();
        let size_a = (hi_a.x - lo_a.x).max(hi_a.y - lo_a.y).max(hi_a.z - lo_a.z);
        let size_b = (hi_b.x - lo_b.x).max(hi_b.y - lo_b.y).max(hi_b.z - lo_b.z);
        if (size_a <= tol && size_b <= tol) || depth >= 32 || (span_a <= tol && span_b <= tol) {
            let t_a = 0.5 * (a0 + a1);
            let t_b = 0.5 * (b0 + b1);
            if let Some(hit) = newton_refine(curve_a, curve_b, t_a, t_b, tol) {
                hits.push(hit);
            }
            return Ok(());
        }
        if span_a >= span_b {
            let (left, right) = bez_a.subdivide(0.5);
            let mid = 0.5 * (a0 + a1);
            clip_pair(&left, a0, mid, bez_b, b0, b1, curve_a, curve_b, tol, depth + 1, hits)?;
            clip_pair(&right, mid, a1, bez_b, b0, b1, curve_a, curve_b, tol, depth + 1, hits)?;
        } else {
            let (left, right) = bez_b.subdivide(0.5);
            let mid = 0.5 * (b0 + b1);
            clip_pair(bez_a, a0, a1, &left, b0, mid, curve_a, curve_b, tol, depth + 1, hits)?;
            clip_pair(bez_a, a0, a1, &right, mid, b1, curve_a, curve_b, tol, depth + 1, hits)?;
        }
        Ok(())
    }

    async fn sample_newton(a: &Curve3, dom_a: (f64, f64), b: &Curve3, dom_b: (f64, f64), tol: f64, hits: &mut Vec<CurveCurveHit>) {
        const N: usize = 24;
        for i in 0..=N {
            let t_a = dom_a.0 + (dom_a.1 - dom_a.0) * (i as f64 / N as f64);
            let pa = a.eval(t_a);
            let mut best_t = dom_b.0;
            let mut best_d = f64::INFINITY;
            for j in 0..=N {
                let t_b = dom_b.0 + (dom_b.1 - dom_b.0) * (j as f64 / N as f64);
                let d = pa.distance(b.eval(t_b));
                if d < best_d {
                    best_d = d;
                    best_t = t_b;
                }
            }
            if best_d <= tol * 50.0 {
                if let Some(hit) = newton_refine(a, b, t_a, best_t, tol) {
                    if hits.iter().all(|h| h.point.distance(hit.point) > tol) {
                        hits.push(hit);
                    }
                }
            }
        }
    }

    async fn newton_refine(a: &Curve3, b: &Curve3, mut t_a: f64, mut t_b: f64, tol: f64) -> Option<CurveCurveHit> {
        for _ in 0..12 {
            let pa = a.eval(t_a);
            let pb = b.eval(t_b);
            let f = pa - pb;
            if f.norm() <= tol {
                return Some(CurveCurveHit { point: pa, t_a, t_b });
            }
            let da = a.d1(t_a);
            let db = b.d1(t_b);
            let j11 = da.dot(da);
            let j12 = -da.dot(db);
            let j22 = db.dot(db);
            let r1 = -da.dot(f);
            let r2 = db.dot(f);
            let det = j11 * j22 - j12 * j12;
            if det.abs() <= 1e-30 {
                return None;
            }
            let du1 = (j22 * r1 - j12 * r2) / det;
            let du2 = (j11 * r2 - j12 * r1) / det;
            t_a += du1;
            t_b += du2;
        }
        let pa = a.eval(t_a);
        let pb = b.eval(t_b);
        if pa.distance(pb) <= tol * 10.0 {
            Some(CurveCurveHit { point: Pnt3::new((pa.x + pb.x) * 0.5, (pa.y + pb.y) * 0.5, (pa.z + pb.z) * 0.5), t_a, t_b })
        } else {
            None
        }
    }

    async fn merge_hits(hits: &mut Vec<CurveCurveHit>, tol: f64) {
        hits.sort_by(|x, y| x.t_a.partial_cmp(&y.t_a).unwrap_or(std::cmp::Ordering::Equal));
        let mut out = Vec::new();
        for hit in hits.drain(..) {
            if out.iter().all(|h: &CurveCurveHit| h.point.distance(hit.point) > tol) {
                out.push(hit);
            }
        }
        *hits = out;
    }

    // #endregion 🔖️General

    // #region 🔖️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        async fn perpendicular_lines_meet_at_origin() {
            let a = Curve3::Line { origin: Pnt3::new(-1.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
            let b = Curve3::Line { origin: Pnt3::new(0.0, -1.0, 0.0), dir: Vec3::new(0.0, 1.0, 0.0) };
            let hits = intersect_curve_curve(&a, &b, 1e-9).unwrap();
            assert_eq!(hits.len(), 1);
            assert!(hits[0].point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-9);
            assert!((hits[0].t_a - 1.0).abs() < 1e-9);
            assert!((hits[0].t_b - 1.0).abs() < 1e-9);
        }

        #[test]
        async fn unit_circle_with_diameter_line() {
            let circle = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
            let line = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
            let hits = intersect_curve_curve(&circle, &line, 1e-9).unwrap();
            assert_eq!(hits.len(), 2);
            let mut xs: Vec<f64> = hits.iter().map(|h| h.point.x).collect();
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            assert!((xs[0] + 1.0).abs() < 1e-9);
            assert!((xs[1] - 1.0).abs() < 1e-9);
            for h in &hits {
                assert!(h.point.y.abs() < 1e-9);
                assert!(h.point.z.abs() < 1e-9);
                assert!((h.point.x * h.point.x + h.point.y * h.point.y - 1.0).abs() < 1e-9);
            }
        }

        #[test]
        async fn line_circle_order_preserves_parameters() {
            let circle = Curve3::Circle { frame: Frame3::WORLD, radius: 1.0 };
            let line = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
            let hits = intersect_curve_curve(&line, &circle, 1e-9).unwrap();
            assert_eq!(hits.len(), 2);
            for h in &hits {
                let on_line = line.eval(h.t_a);
                let on_circle = circle.eval(h.t_b);
                assert!(on_line.distance(h.point) < 1e-8);
                assert!(on_circle.distance(h.point) < 1e-8);
            }
        }

        mod quick {
            use super::*;

            #[test]
            async fn skew_lines_do_not_intersect() {
                let a = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 0.0), dir: Vec3::X };
                let b = Curve3::Line { origin: Pnt3::new(0.0, 1.0, 1.0), dir: Vec3::Y };
                let hits = intersect_curve_curve(&a, &b, 1e-9).unwrap();
                assert!(hits.is_empty());
            }
        }
    }
    // #endregion 🔖️Tests
}

pub mod curve_surface {
    //! ✂️ Curve/surface intersection (analytic + Newton).
    //!
    //! Analytic fast paths cover [`Curve3::Line`] against [`Surface::Plane`], [`Surface::Sphere`], and
    //! [`Surface::Cylinder`]. Every other combination falls through to sample seeding plus Newton on
    //! the coupled 3×3 system `C(t) − S(u, v) = 0`.
    //!
    //! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::IntersectError;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::surface_ops::closest_point;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

    // #region 🔖️Api

    /// ✂️ An isolated curve/surface intersection: world point plus curve and surface parameters.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct CurveSurfaceHit {
        pub point: Pnt3,
        pub t: f64,
        pub u: f64,
        pub v: f64,
    }

    /// ✂️ Intersect a 3D curve with a parametric surface within `tol`. Analytic for line/plane,
    /// line/sphere, and line/cylinder; general otherwise via sample seeds plus Newton refinement.
    pub async fn intersect_curve_surface(curve: &Curve3, surface: &Surface, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
        if !(tol.is_finite() && tol > 0.0) {
            return Err(IntersectError::Degenerate("tolerance must be positive and finite".into()));
        }
        match (curve, surface) {
            (Curve3::Line { origin, dir }, Surface::Plane { frame }) => intersect_line_plane(*origin, *dir, frame, tol),
            (Curve3::Line { origin, dir }, Surface::Sphere { frame, radius }) => intersect_line_sphere(*origin, *dir, frame, *radius, tol),
            (Curve3::Line { origin, dir }, Surface::Cylinder { frame, radius }) => intersect_line_cylinder(*origin, *dir, frame, *radius, tol),
            _ => intersect_general(curve, surface, tol),
        }
    }

    // #endregion 🔖️Api

    // #region 🔖️Analytic

    async fn intersect_line_plane(origin: Pnt3, dir: Vec3, frame: &Frame3, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
        let n = dir.norm();
        if n <= tol {
            return Err(IntersectError::Degenerate("zero-length line direction".into()));
        }
        let normal = frame.z;
        let denom = dir.dot(normal);
        let local = frame.to_local(origin);
        if denom.abs() <= tol * n {
            if local.z.abs() <= tol {
                return Err(IntersectError::Tangent);
            }
            return Ok(vec![]);
        }
        let t = -local.z / (frame.to_local_vector(dir).z);
        let point = origin + dir * t;
        let uv = frame.to_local(point);
        Ok(vec![CurveSurfaceHit { point, t, u: uv.x, v: uv.y }])
    }

    async fn intersect_line_sphere(origin: Pnt3, dir: Vec3, frame: &Frame3, radius: f64, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
        if radius <= tol || dir.norm() <= tol {
            return Err(IntersectError::Degenerate("degenerate line or sphere".into()));
        }
        let o = frame.to_local(origin).to_vec();
        let d = frame.to_local_vector(dir);
        let a = d.dot(d);
        let b = 2.0 * o.dot(d);
        let c = o.dot(o) - radius * radius;
        let disc = b * b - 4.0 * a * c;
        if disc < -(tol * tol) * a * a {
            return Ok(vec![]);
        }
        let sqrt_disc = disc.max(0.0).sqrt();
        let mut hits = Vec::with_capacity(2);
        for sign in [-1.0, 1.0] {
            let t = (-b + sign * sqrt_disc) / (2.0 * a);
            let point = origin + dir * t;
            let local = frame.to_local(point).to_vec();
            let n = local.normalized().unwrap_or(Vec3::Z);
            let v = n.z.clamp(-1.0, 1.0).asin();
            let u = n.y.atan2(n.x).rem_euclid(std::f64::consts::TAU);
            let hit = CurveSurfaceHit { point, t, u, v };
            if hits.iter().all(|h: &CurveSurfaceHit| h.point.distance(hit.point) > tol) {
                hits.push(hit);
            }
        }
        if hits.len() == 1 && disc.abs() <= (tol * tol) * a * a * 4.0 {
            return Err(IntersectError::Tangent);
        }
        Ok(hits)
    }

    async fn intersect_line_cylinder(origin: Pnt3, dir: Vec3, frame: &Frame3, radius: f64, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
        if radius <= tol || dir.norm() <= tol {
            return Err(IntersectError::Degenerate("degenerate line or cylinder".into()));
        }
        let o = frame.to_local(origin);
        let d = frame.to_local_vector(dir);
        let a = d.x * d.x + d.y * d.y;
        let b = 2.0 * (o.x * d.x + o.y * d.y);
        let c = o.x * o.x + o.y * o.y - radius * radius;
        if a <= tol * tol {
            if c.abs() <= tol * (2.0 * radius).max(1.0) {
                return Err(IntersectError::Tangent);
            }
            return Ok(vec![]);
        }
        let disc = b * b - 4.0 * a * c;
        if disc < -(tol * tol) * a * a {
            return Ok(vec![]);
        }
        let sqrt_disc = disc.max(0.0).sqrt();
        let mut hits = Vec::with_capacity(2);
        for sign in [-1.0, 1.0] {
            let t = (-b + sign * sqrt_disc) / (2.0 * a);
            let point = origin + dir * t;
            let local = frame.to_local(point);
            let u = local.y.atan2(local.x).rem_euclid(std::f64::consts::TAU);
            let hit = CurveSurfaceHit { point, t, u, v: local.z };
            if hits.iter().all(|h: &CurveSurfaceHit| h.point.distance(hit.point) > tol) {
                hits.push(hit);
            }
        }
        if hits.len() == 1 && disc.abs() <= (tol * tol) * a * a * 4.0 {
            return Err(IntersectError::Tangent);
        }
        Ok(hits)
    }

    // #endregion 🔖️Analytic

    // #region 🔖️General

    async fn intersect_general(curve: &Curve3, surface: &Surface, tol: f64) -> Result<Vec<CurveSurfaceHit>, IntersectError> {
        let domain_t = curve_sample_domain(curve, surface, tol)?;
        let surf_domain = finite_surface_domain(surface);
        let n_samples = 32usize;
        let mut hits = Vec::new();
        for i in 0..=n_samples {
            let t = domain_t.0 + (domain_t.1 - domain_t.0) * (i as f64 / n_samples as f64);
            let pt = curve.eval(t);
            let (u, v, dist) = closest_point(surface, surf_domain, pt, 8);
            if dist <= tol * 50.0 {
                if let Some(hit) = newton_refine(curve, surface, t, u, v, domain_t, surf_domain, tol) {
                    push_unique(&mut hits, hit, tol);
                }
            }
        }
        hits.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(std::cmp::Ordering::Equal));
        Ok(hits)
    }

    async fn curve_sample_domain(curve: &Curve3, surface: &Surface, tol: f64) -> Result<(f64, f64), IntersectError> {
        match curve {
            Curve3::Line { origin, dir } => line_domain_against_surface(origin, dir, surface, tol),
            Curve3::Circle { .. } | Curve3::Ellipse { .. } => Ok(curve.domain()),
            Curve3::Nurbs { knots, .. } => {
                let d = knots.domain();
                if !d.0.is_finite() || !d.1.is_finite() || d.1 <= d.0 {
                    return Err(IntersectError::Degenerate("unable to form a finite curve domain".into()));
                }
                Ok(d)
            }
        }
    }

    async fn line_domain_against_surface(origin: &Pnt3, dir: &Vec3, surface: &Surface, tol: f64) -> Result<(f64, f64), IntersectError> {
        let n = dir.norm();
        if n <= tol {
            return Err(IntersectError::Degenerate("zero-length line direction".into()));
        }
        let unit = *dir * (1.0 / n);
        let ((u0, u1), (v0, v1)) = finite_surface_domain(surface);
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for i in 0..=8 {
            for j in 0..=8 {
                let u = u0 + (u1 - u0) * (i as f64 / 8.0);
                let v = v0 + (v1 - v0) * (j as f64 / 8.0);
                let p = surface.eval(u, v);
                let s = (p - *origin).dot(unit);
                lo = lo.min(s);
                hi = hi.max(s);
            }
        }
        if !lo.is_finite() || !hi.is_finite() {
            return Err(IntersectError::Degenerate("unable to bound line against surface".into()));
        }
        let pad = ((hi - lo).abs() + 1.0).max(1.0);
        Ok(((lo - pad) / n, (hi + pad) / n))
    }

    async fn finite_surface_domain(surface: &Surface) -> ((f64, f64), (f64, f64)) {
        let ((u0, u1), (v0, v1)) = surface.domain();
        let u_hi = if u1.is_finite() { u1 } else { u0 + std::f64::consts::TAU };
        let u_lo = if u0.is_finite() { u0 } else { u_hi - std::f64::consts::TAU };
        let v_hi = if v1.is_finite() { v1 } else { 10.0 };
        let v_lo = if v0.is_finite() { v0 } else { -10.0 };
        ((u_lo, u_hi), (v_lo, v_hi))
    }

    async fn wrap_or_clamp(x: f64, lo: f64, hi: f64, periodic: bool) -> f64 {
        if periodic {
            let period = hi - lo;
            if period.abs() <= f64::EPSILON {
                return lo;
            }
            let mut w = (x - lo) % period;
            if w < 0.0 {
                w += period;
            }
            lo + w
        } else if lo.is_finite() && hi.is_finite() {
            x.clamp(lo, hi)
        } else {
            x
        }
    }

    async fn newton_refine(curve: &Curve3, surface: &Surface, mut t: f64, mut u: f64, mut v: f64, domain_t: (f64, f64), surf_domain: ((f64, f64), (f64, f64)), tol: f64) -> Option<CurveSurfaceHit> {
        let ((u_lo, u_hi), (v_lo, v_hi)) = surf_domain;
        let u_periodic = surface.is_u_periodic();
        let v_periodic = surface.is_v_periodic();
        let t_periodic = curve.is_periodic();
        for _ in 0..16 {
            let c_pt = curve.eval(t);
            let d = surface.derivatives(u, v);
            let residual = c_pt - d.point;
            if residual.norm() <= tol {
                return Some(CurveSurfaceHit { point: c_pt, t, u, v });
            }
            let ct = curve.d1(t);
            let col0 = ct;
            let col1 = -d.du;
            let col2 = -d.dv;
            let det = col0.x * (col1.y * col2.z - col1.z * col2.y) - col1.x * (col0.y * col2.z - col0.z * col2.y) + col2.x * (col0.y * col1.z - col0.z * col1.y);
            let (dt, du, dv) = if det.abs() < 1e-30 {
                let lambda = 1e-6;
                let jtj = [[col0.dot(col0) + lambda, col0.dot(col1), col0.dot(col2)], [col1.dot(col0), col1.dot(col1) + lambda, col1.dot(col2)], [col2.dot(col0), col2.dot(col1), col2.dot(col2) + lambda]];
                let jtr = [col0.dot(residual), col1.dot(residual), col2.dot(residual)];
                solve_3x3(&jtj, &jtr)?
            } else {
                let inv_det = 1.0 / det;
                let neg_r = -residual;
                let dt = inv_det * (neg_r.x * (col1.y * col2.z - col1.z * col2.y) - col1.x * (neg_r.y * col2.z - neg_r.z * col2.y) + col2.x * (neg_r.y * col1.z - neg_r.z * col1.y));
                let du = inv_det * (col0.x * (neg_r.y * col2.z - neg_r.z * col2.y) - neg_r.x * (col0.y * col2.z - col0.z * col2.y) + col2.x * (col0.y * neg_r.z - col0.z * neg_r.y));
                let dv = inv_det * (col0.x * (col1.y * neg_r.z - col1.z * neg_r.y) - col1.x * (col0.y * neg_r.z - col0.z * neg_r.y) + neg_r.x * (col0.y * col1.z - col0.z * col1.y));
                (dt, du, dv)
            };
            t = if t_periodic {
                wrap_or_clamp(t + dt, domain_t.0, domain_t.1, true)
            } else if domain_t.0.is_finite() && domain_t.1.is_finite() {
                (t + dt).clamp(domain_t.0, domain_t.1)
            } else {
                t + dt
            };
            u = wrap_or_clamp(u + du, u_lo, u_hi, u_periodic);
            v = wrap_or_clamp(v + dv, v_lo, v_hi, v_periodic);
        }
        let c_pt = curve.eval(t);
        let s_pt = surface.eval(u, v);
        if c_pt.distance(s_pt) <= tol * 10.0 {
            Some(CurveSurfaceHit { point: c_pt, t, u, v })
        } else {
            None
        }
    }

    async fn solve_3x3(a: &[[f64; 3]; 3], b: &[f64; 3]) -> Option<(f64, f64, f64)> {
        let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0]) + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
        if det.abs() < 1e-30 {
            return None;
        }
        let inv = 1.0 / det;
        let x = inv * (b[0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1]) - a[0][1] * (b[1] * a[2][2] - a[1][2] * b[2]) + a[0][2] * (b[1] * a[2][1] - a[1][1] * b[2]));
        let y = inv * (a[0][0] * (b[1] * a[2][2] - a[1][2] * b[2]) - b[0] * (a[1][0] * a[2][2] - a[1][2] * a[2][0]) + a[0][2] * (a[1][0] * b[2] - b[1] * a[2][0]));
        let z = inv * (a[0][0] * (a[1][1] * b[2] - b[1] * a[2][1]) - a[0][1] * (a[1][0] * b[2] - b[1] * a[2][0]) + b[0] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]));
        Some((x, y, z))
    }

    async fn push_unique(hits: &mut Vec<CurveSurfaceHit>, hit: CurveSurfaceHit, tol: f64) {
        let dedup = tol.max(1e-6) * 10.0;
        if hits.iter().all(|h| h.point.distance(hit.point) > dedup) {
            hits.push(hit);
        }
    }

    // #endregion 🔖️General

    // #region 🔖️Tests

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        async fn line_pierces_plane_z0() {
            let curve = Curve3::Line { origin: Pnt3::new(0.0, 0.0, -1.0), dir: Vec3::new(0.0, 0.0, 1.0) };
            let surface = Surface::Plane { frame: Frame3::WORLD };
            let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
            assert_eq!(hits.len(), 1);
            assert!(hits[0].point.distance(Pnt3::new(0.0, 0.0, 0.0)) < 1e-9);
            assert!((hits[0].t - 1.0).abs() < 1e-9);
            assert!(hits[0].u.abs() < 1e-9);
            assert!(hits[0].v.abs() < 1e-9);
        }

        #[test]
        async fn line_through_sphere() {
            let curve = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
            let surface = Surface::Sphere { frame: Frame3::WORLD, radius: 1.0 };
            let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
            assert_eq!(hits.len(), 2);
            let mut xs: Vec<f64> = hits.iter().map(|h| h.point.x).collect();
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            assert!((xs[0] + 1.0).abs() < 1e-9);
            assert!((xs[1] - 1.0).abs() < 1e-9);
            for h in &hits {
                assert!(h.point.y.abs() < 1e-9);
                assert!(h.point.z.abs() < 1e-9);
                assert!((h.point.to_vec().norm() - 1.0).abs() < 1e-9);
                let on_curve = curve.eval(h.t);
                let on_surf = surface.eval(h.u, h.v);
                assert!(on_curve.distance(h.point) < 1e-8);
                assert!(on_surf.distance(h.point) < 1e-8);
            }
        }

        #[test]
        async fn line_through_cylinder() {
            let curve = Curve3::Line { origin: Pnt3::new(-2.0, 0.0, 1.0), dir: Vec3::new(1.0, 0.0, 0.0) };
            let surface = Surface::Cylinder { frame: Frame3::WORLD, radius: 1.0 };
            let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
            assert_eq!(hits.len(), 2);
            for h in &hits {
                assert!((h.point.x * h.point.x + h.point.y * h.point.y - 1.0).abs() < 1e-9);
                assert!((h.point.z - 1.0).abs() < 1e-9);
                assert!((h.v - 1.0).abs() < 1e-9);
            }
        }

        mod quick {
            use super::*;

            #[test]
            async fn parallel_line_misses_plane() {
                let curve = Curve3::Line { origin: Pnt3::new(0.0, 0.0, 1.0), dir: Vec3::X };
                let surface = Surface::Plane { frame: Frame3::WORLD };
                let hits = intersect_curve_surface(&curve, &surface, 1e-9).unwrap();
                assert!(hits.is_empty());
            }

            #[test]
            async fn circle_plane_equator() {
                let curve = Curve3::Circle { frame: Frame3::WORLD, radius: 2.0 };
                let surface = Surface::Plane { frame: Frame3::WORLD };
                let hits = intersect_curve_surface(&curve, &surface, 1e-6).unwrap();
                assert!(!hits.is_empty());
                for h in &hits {
                    assert!(h.point.z.abs() < 1e-5);
                    assert!((h.point.x * h.point.x + h.point.y * h.point.y - 4.0).abs() < 1e-4);
                }
            }
        }
    }

    // #endregion 🔖️Tests
}

pub mod surface_surface {
    //! ✂️ Surface/surface intersection emitting [`IntCurve`].
    //!
    //! Analytic fast paths for plane/plane, plane/cylinder, plane/sphere, sphere/sphere;
    //! remaining pairs use a dense UV sampling fallback that emits a degree-1 NURBS through the hits.
    //!
    //! See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.

    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::IntersectError;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

    // #region 🔖️Api

    /// ✂️ One surface/surface intersection branch (space curve; pcurves land later).
    #[derive(Clone, Debug, PartialEq)]
    pub struct IntCurve {
        pub curve3: Curve3,
    }

    /// ✂️ Intersect two parametric surfaces within `tol`.
    pub async fn intersect_surface_surface(a: &Surface, b: &Surface, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
        if !(tol.is_finite() && tol > 0.0) {
            return Err(IntersectError::Degenerate("tolerance must be positive and finite".into()));
        }
        match (a, b) {
            (Surface::Plane { frame: fa }, Surface::Plane { frame: fb }) => intersect_plane_plane(fa, fb, tol),
            (Surface::Plane { frame }, Surface::Cylinder { frame: cf, radius }) => intersect_plane_cylinder(frame, cf, *radius, tol),
            (Surface::Cylinder { frame: cf, radius }, Surface::Plane { frame }) => intersect_plane_cylinder(frame, cf, *radius, tol),
            (Surface::Plane { frame }, Surface::Sphere { frame: sf, radius }) => intersect_plane_sphere(frame, sf, *radius, tol),
            (Surface::Sphere { frame: sf, radius }, Surface::Plane { frame }) => intersect_plane_sphere(frame, sf, *radius, tol),
            (Surface::Sphere { frame: fa, radius: ra }, Surface::Sphere { frame: fb, radius: rb }) => intersect_sphere_sphere(fa, *ra, fb, *rb, tol),
            _ => intersect_surfaces_sampled(a, b, tol),
        }
    }

    async fn intersect_plane_sphere(plane: &Frame3, sphere: &Frame3, radius: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
        if !(radius.is_finite() && radius > tol) {
            return Err(IntersectError::Degenerate("sphere radius must be positive".into()));
        }
        let n = plane.z;
        let n_n = n.norm();
        if n_n <= tol {
            return Err(IntersectError::Degenerate("plane normal degenerate".into()));
        }
        let n_u = n * (1.0 / n_n);
        let dist = n_u.dot(sphere.origin - plane.origin);
        let abs_d = dist.abs();
        if abs_d > radius + tol {
            return Ok(Vec::new());
        }
        let h2 = radius * radius - dist * dist;
        let r = if h2 <= 0.0 { 0.0 } else { h2.sqrt() };
        let center = sphere.origin - n_u * dist;
        let x = plane.x - n_u * plane.x.dot(n_u);
        let x = x.normalized().unwrap_or(plane.y);
        let y = n_u.cross(x);
        Ok(vec![IntCurve { curve3: Curve3::Circle { frame: Frame3 { origin: center, x, y, z: n_u }, radius: r.max(tol * 0.5) } }])
    }

    async fn intersect_sphere_sphere(fa: &Frame3, ra: f64, fb: &Frame3, rb: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
        if !(ra.is_finite() && rb.is_finite() && ra > tol && rb > tol) {
            return Err(IntersectError::Degenerate("sphere radii must be positive".into()));
        }
        let d_vec = fb.origin - fa.origin;
        let d = d_vec.norm();
        if d <= tol {
            return if (ra - rb).abs() <= tol { Err(IntersectError::Unresolved("coincident spheres".into())) } else { Ok(Vec::new()) };
        }
        if d > ra + rb + tol || d + tol < (ra - rb).abs() {
            return Ok(Vec::new());
        }
        let dir = d_vec * (1.0 / d);
        let a = (ra * ra - rb * rb + d * d) / (2.0 * d);
        let h2 = ra * ra - a * a;
        let h = if h2 <= 0.0 { 0.0 } else { h2.sqrt() };
        let center = fa.origin + dir * a;
        let x = dir.cross(Vec3::new(0.0, 0.0, 1.0)).normalized().or_else(|| dir.cross(Vec3::new(0.0, 1.0, 0.0)).normalized()).ok_or_else(|| IntersectError::Degenerate("sphere intersection frame".into()))?;
        let y = dir.cross(x);
        Ok(vec![IntCurve { curve3: Curve3::Circle { frame: Frame3 { origin: center, x, y, z: dir }, radius: h.max(tol * 0.5) } }])
    }

    /// ✂️ Dense UV sampling fallback: keep samples of `a` near `b`, then emit a polyline.
    async fn intersect_surfaces_sampled(a: &Surface, b: &Surface, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
        let nu = 24usize;
        let nv = 24usize;
        let mut pts = Vec::new();
        for iu in 0..=nu {
            let u = (iu as f64) / (nu as f64) * std::f64::consts::TAU;
            for iv in 0..=nv {
                let v = ((iv as f64) / (nv as f64) - 0.5) * 4.0;
                let p = a.eval(u, v);
                if let Some(q) = project_point_to_surface(b, p, tol) {
                    if (q - p).norm() <= tol * 4.0 {
                        pts.push(p);
                    }
                }
            }
        }
        if pts.len() < 2 {
            return Ok(Vec::new());
        }
        let mut ordered = vec![pts.remove(0)];
        while !pts.is_empty() {
            let last = *ordered.last().unwrap();
            let (idx, _) = pts.iter().enumerate().min_by(|(_, aa), (_, bb)| (last - **aa).norm().partial_cmp(&(last - **bb).norm()).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
            ordered.push(pts.swap_remove(idx));
        }
        // Dedup near-duplicates
        let mut controls = Vec::<Pnt3>::new();
        for p in ordered {
            if controls.last().map(|q| (*q - p).norm() > tol).unwrap_or(true) {
                controls.push(p);
            }
        }
        if controls.len() < 2 {
            return Ok(Vec::new());
        }
        if controls.len() == 2 {
            let origin = controls[0];
            let dir = controls[1] - origin;
            if dir.norm() <= tol {
                return Ok(Vec::new());
            }
            return Ok(vec![IntCurve { curve3: Curve3::Line { origin, dir } }]);
        }
        let n = controls.len();
        let knots = crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector::clamped_uniform(n, 1);
        let weights = vec![1.0; n];
        Ok(vec![IntCurve { curve3: Curve3::Nurbs { knots, controls, weights } }])
    }

    async fn project_point_to_surface(surface: &Surface, point: Pnt3, tol: f64) -> Option<Pnt3> {
        match surface {
            Surface::Plane { frame } => {
                let n = frame.z;
                let n_n = n.norm();
                if n_n <= tol {
                    return None;
                }
                let n_u = n * (1.0 / n_n);
                Some(point - n_u * n_u.dot(point - frame.origin))
            }
            Surface::Sphere { frame, radius } => {
                let v = point - frame.origin;
                let n = v.norm();
                if n <= tol {
                    return None;
                }
                Some(frame.origin + v * (*radius / n))
            }
            Surface::Cylinder { frame, radius } => {
                let w = point - frame.origin;
                let axis = frame.z;
                let axial = axis * axis.dot(w);
                let radial = w - axial;
                let rn = radial.norm();
                if rn <= tol {
                    return None;
                }
                Some(frame.origin + axial + radial * (*radius / rn))
            }
            _ => {
                let mut u = 0.0;
                let mut v = 0.0;
                for _ in 0..8 {
                    let p = surface.eval(u, v);
                    let r = point - p;
                    if r.norm() <= tol {
                        return Some(p);
                    }
                    let pu = surface.eval(u + 1e-3, v) - p;
                    let pv = surface.eval(u, v + 1e-3) - p;
                    let gu = pu.dot(r);
                    let gv = pv.dot(r);
                    let du = pu.dot(pu);
                    let dv = pv.dot(pv);
                    if du > tol * tol {
                        u += gu / du;
                    }
                    if dv > tol * tol {
                        v += gv / dv;
                    }
                }
                Some(surface.eval(u, v))
            }
        }
    }

    // #endregion 🔖️Api

    // #region 🔖️Analytic

    async fn intersect_plane_plane(fa: &Frame3, fb: &Frame3, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
        let n1 = fa.z;
        let n2 = fb.z;
        let dir = n1.cross(n2);
        let dir_n = dir.norm();
        if dir_n <= tol {
            let dist = n1.dot(fb.origin - fa.origin).abs();
            if dist <= tol {
                return Err(IntersectError::Tangent);
            }
            return Ok(vec![]);
        }
        let d1 = n1.dot(fa.origin.to_vec());
        let d2 = n2.dot(fb.origin.to_vec());
        let point = (n2.cross(dir) * d1 + dir.cross(n1) * d2) * (1.0 / (dir_n * dir_n));
        let origin = Pnt3::new(point.x, point.y, point.z);
        let unit = dir * (1.0 / dir_n);
        Ok(vec![IntCurve { curve3: Curve3::Line { origin, dir: unit } }])
    }

    async fn intersect_plane_cylinder(plane: &Frame3, cyl: &Frame3, radius: f64, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
        if radius <= tol {
            return Err(IntersectError::Degenerate("non-positive cylinder radius".into()));
        }
        let n = plane.z.normalized().unwrap_or(Vec3::Z);
        let axis = cyl.z.normalized().unwrap_or(Vec3::Z);
        let cos_theta = n.dot(axis).abs();
        if cos_theta <= tol {
            return plane_cylinder_parallel(plane, cyl, radius, n, axis, tol);
        }
        let n_dot_axis = n.dot(axis);
        let t = n.dot(plane.origin - cyl.origin) / n_dot_axis;
        let center = cyl.origin + axis * t;
        if (1.0 - cos_theta) <= tol {
            let frame = Frame3::from_normal(center, axis).ok_or_else(|| IntersectError::Degenerate("degenerate circle frame on cylinder".into()))?;
            return Ok(vec![IntCurve { curve3: Curve3::Circle { frame, radius } }]);
        }
        let minor = radius;
        let major = radius / cos_theta;
        let major_dir = (axis - n * axis.dot(n)).normalized().unwrap_or_else(|| n.any_orthogonal());
        let frame = Frame3::from_x_z(center, major_dir, n).ok_or_else(|| IntersectError::Degenerate("degenerate ellipse frame on cylinder".into()))?;
        Ok(vec![IntCurve { curve3: Curve3::Ellipse { frame, major_radius: major, minor_radius: minor } }])
    }

    async fn plane_cylinder_parallel(plane: &Frame3, cyl: &Frame3, radius: f64, n: Vec3, axis: Vec3, tol: f64) -> Result<Vec<IntCurve>, IntersectError> {
        let signed = n.dot(cyl.origin - plane.origin);
        let dist = signed.abs();
        if dist > radius + tol {
            return Ok(vec![]);
        }
        let h_sq = radius * radius - dist * dist;
        if h_sq < -(tol * tol) {
            return Ok(vec![]);
        }
        let h = h_sq.max(0.0).sqrt();
        let foot = cyl.origin - n * signed;
        let perp = n.cross(axis).normalized().unwrap_or_else(|| axis.any_orthogonal());
        if h <= tol {
            return Err(IntersectError::Tangent);
        }
        Ok(vec![IntCurve { curve3: Curve3::Line { origin: foot + perp * (-h), dir: axis } }, IntCurve { curve3: Curve3::Line { origin: foot + perp * h, dir: axis } }])
    }

    // #endregion 🔖️Analytic

    // #region 🔖️Tests

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        async fn orthogonal_planes_intersect_in_line() {
            let xy = Surface::Plane { frame: Frame3::WORLD };
            let xz = Surface::Plane { frame: Frame3::from_normal(Pnt3::new(0.0, 0.0, 0.0), Vec3::Y).unwrap() };
            let curves = intersect_surface_surface(&xy, &xz, 1e-8).expect("planes intersect");
            assert_eq!(curves.len(), 1);
            match &curves[0].curve3 {
                Curve3::Line { origin, dir } => {
                    assert!(origin.y.abs() < 1e-8 && origin.z.abs() < 1e-8);
                    let u = dir.normalized().unwrap();
                    assert!((u.x.abs() - 1.0).abs() < 1e-8);
                    assert!(u.y.abs() < 1e-8 && u.z.abs() < 1e-8);
                }
                other => panic!("expected line, got {other:?}"),
            }
            for t in [-2.0_f64, -0.5, 0.0, 1.5, 3.0] {
                let p = curves[0].curve3.eval(t);
                assert!(p.y.abs() < 1e-8 && p.z.abs() < 1e-8);
            }
        }

        #[test]
        async fn parallel_planes_empty_or_tangent() {
            let a = Surface::Plane { frame: Frame3::WORLD };
            let b = Surface::Plane { frame: Frame3 { origin: Pnt3::new(0.0, 0.0, 2.0), ..Frame3::WORLD } };
            assert!(intersect_surface_surface(&a, &b, 1e-8).unwrap().is_empty());
            let c = Surface::Plane { frame: Frame3 { origin: Pnt3::new(1.0, 2.0, 0.0), ..Frame3::WORLD } };
            assert!(matches!(intersect_surface_surface(&a, &c, 1e-8), Err(IntersectError::Tangent)));
        }

        #[test]
        async fn plane_cylinder_perpendicular_is_circle() {
            let plane = Surface::Plane { frame: Frame3 { origin: Pnt3::new(0.0, 0.0, 3.0), ..Frame3::WORLD } };
            let cyl = Surface::Cylinder { frame: Frame3::WORLD, radius: 2.0 };
            let curves = intersect_surface_surface(&plane, &cyl, 1e-8).expect("plane/cyl");
            assert_eq!(curves.len(), 1);
            match &curves[0].curve3 {
                Curve3::Circle { frame, radius } => {
                    assert!((radius - 2.0).abs() < 1e-8);
                    assert!(frame.origin.distance(Pnt3::new(0.0, 0.0, 3.0)) < 1e-8);
                }
                other => panic!("expected circle, got {other:?}"),
            }
            for i in 0..16 {
                let t = i as f64 * std::f64::consts::TAU / 16.0;
                let p = curves[0].curve3.eval(t);
                let r = (p.x * p.x + p.y * p.y).sqrt();
                assert!((r - 2.0).abs() < 1e-6);
                assert!((p.z - 3.0).abs() < 1e-6);
            }
        }

        #[test]
        async fn plane_cylinder_parallel_two_lines() {
            let plane = Surface::Plane { frame: Frame3::from_x_z(Pnt3::new(0.0, 0.0, 0.0), Vec3::Y, Vec3::X).unwrap() };
            let cyl = Surface::Cylinder { frame: Frame3::WORLD, radius: 2.0 };
            let curves = intersect_surface_surface(&plane, &cyl, 1e-8).expect("parallel plane/cyl");
            assert_eq!(curves.len(), 2);
            for c in &curves {
                match &c.curve3 {
                    Curve3::Line { origin, dir } => {
                        assert!(origin.x.abs() < 1e-6);
                        assert!((origin.y.abs() - 2.0).abs() < 1e-6);
                        assert!(dir.normalized().unwrap().z.abs() > 0.99);
                    }
                    other => panic!("expected line, got {other:?}"),
                }
            }
        }
    }

    // #endregion 🔖️Tests
}

// #region 🔖️Reexports
pub use curve_curve::{intersect_curve_curve, CurveCurveHit};
pub use curve_surface::{intersect_curve_surface, CurveSurfaceHit};
pub use surface_surface::{intersect_surface_surface, IntCurve};
// #endregion 🔖️Reexports
