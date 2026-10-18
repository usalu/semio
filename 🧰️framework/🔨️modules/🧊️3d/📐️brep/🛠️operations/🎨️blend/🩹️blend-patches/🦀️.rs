//! 🩹️ Blend patches: the blend spec, the end trim, the patch record and the cylinder, chamfer plane, cone and cap patch builders. See [`super`].

use super::*;

// #region 🔖️Patch
/// 🎨️ What one blend call builds along every selected edge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum BlendSpec {
    /// 🎨️ Rolling ball of one radius.
    Fillet(f64),
    /// 🎨️ Rolling ball whose radius runs linearly from the edge's start to its end.
    VariableFillet(f64, f64),
    /// 🎨️ Cutting plane at the first distance along `faces[0]` and the second along `faces[1]`.
    Chamfer(f64, f64),
}

impl BlendSpec {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn fillet_radius(&self) -> Option<f64> {
        match self {
            BlendSpec::Fillet(r) => Some(*r),
            _ => None,
        }
    }
}

/// 🎨️ How one end of a patch is trimmed: at an iso-line of the run direction (every constant-radius
/// case — the cap plane there is perpendicular to the patch's own axis, so the trim IS a cross
/// section), or, for a variable-radius cone whose axis tilts away from the edge, at the exact conic
/// its cap plane cuts out of it.
#[derive(Clone, Debug)]
pub(super) enum EndTrim {
    Iso(f64),
    Conic(Curve3),
}

/// 🎨️ One blend patch: its analytic support plus the parametric addresses of its four boundaries.
/// `cross` is the parameter held fixed along each face's TANGENCY curve (so that curve is
/// `isocurve(cross_dir, cross[i])`, parametrized by the run parameter); the patch's two END curves
/// run the other way, from `cross[0]` to `cross[1]`. Every analytic kind built here fits that
/// shape, which is what lets one assembly routine serve fillets and chamfers, cylinders, cones,
/// tori and planes alike.
pub(super) struct Patch {
    pub(super) edge: EdgeId,
    pub(super) faces: [FaceId; 2],
    pub(super) surface: Surface,
    cross_dir: IsoDirection,
    pub(super) cross: [f64; 2],
    /// 🎨️ `true` for a band closed in the run direction (a blended circular cap edge), whose two
    /// "ends" are one seam curve traversed twice.
    pub(super) closed: bool,
    /// 🎨️ The original edge's endpoint vertices in run order; both are the seam vertex for a
    /// closed band.
    pub(super) end_vertex: [VertexId; 2],
    /// 🎨️ The run range each face's tangency curve keeps, after trimming. Identical for both faces
    /// except on a variable-radius cone.
    pub(super) run: [(f64, f64); 2],
    pub(super) ends: [EndTrim; 2],
}

impl Patch {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn run_dir(&self) -> IsoDirection {
        match self.cross_dir {
            IsoDirection::U => IsoDirection::V,
            IsoDirection::V => IsoDirection::U,
        }
    }
    /// 🎨️ The tangency curve on `faces[i]`, parametrized by the run parameter.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn tangency(&self, i: usize) -> Curve3 {
        self.surface.isocurve(self.cross_dir, self.cross[i])
    }
    /// 🎨️ The end curve at `slot` and its own parameter range, running from `faces[0]`'s tangency
    /// point to `faces[1]`'s.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn end_curve(&self, slot: usize) -> (Curve3, (f64, f64)) {
        match &self.ends[slot] {
            EndTrim::Iso(w) => (self.surface.isocurve(self.run_dir(), *w), (self.cross[0], self.cross[1])),
            EndTrim::Conic(curve) => (curve.clone(), (0.0, 1.0)),
        }
    }
    /// 🎨️ Whether the ring `[tangency 0, end@hi, tangency 1, end@lo]` walks the patch's `(u, v)`
    /// rectangle counter-clockwise — i.e. with `du × dv`, the outward normal of every support built
    /// here, toward the viewer. Swapping the roles of `u` and `v` reverses a rectangle's
    /// orientation, so the answer flips with `cross_dir`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn ring_is_ccw(&self) -> bool {
        match self.cross_dir {
            IsoDirection::U => self.cross[1] < self.cross[0],
            IsoDirection::V => self.cross[1] > self.cross[0],
        }
    }
}

/// 🎨️ The distinct solid faces that use `edge` — one for a seam a single face uses twice, two for an
/// ordinary manifold edge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn edge_faces(body: &Body, solid_faces: &BTreeSet<FaceId>, edge: EdgeId) -> Vec<FaceId> {
    let mut faces = Vec::new();
    for cid in body.edge_coedges(edge) {
        if let Some(c) = body.coedges.get(cid) {
            if let Some(lp) = body.loops.get(c.loop_id) {
                if solid_faces.contains(&lp.face) && !faces.contains(&lp.face) {
                    faces.push(lp.face);
                }
            }
        }
    }
    faces
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn edge_two_faces(body: &Body, solid_faces: &BTreeSet<FaceId>, edge: EdgeId) -> Result<(FaceId, FaceId), KernelError> {
    let faces = edge_faces(body, solid_faces, edge);
    if faces.len() != 2 {
        return Err(KernelError::Operation("blend edge must be shared by exactly two distinct faces".into()));
    }
    Ok((faces[0], faces[1]))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn coedge_on_face(body: &Body, edge: EdgeId, face: FaceId) -> Option<CoedgeId> {
    body.edge_coedges(edge).into_iter().find(|&cid| body.coedges.get(cid).and_then(|c| body.loops.get(c.loop_id)).map(|lp| lp.face) == Some(face))
}

/// 🎨️ The direction `edge` is traversed in as seen by `face`'s own loop.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn traversal_direction(body: &Body, edge: EdgeId, face: FaceId) -> Result<Vec3, KernelError> {
    let cid = coedge_on_face(body, edge, face).ok_or_else(|| KernelError::Operation("blend edge has no coedge on its own face".into()))?;
    let forward = body.coedges.get(cid).unwrap().forward;
    let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
    let curve = body.curves3.get(ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
    // 🧭️ An edge runs from `v0` at `range.0` to `v1` at `range.1`, and a rebuilt edge's range can
    // DECREASE (`↔️offset`'s `rebuild_topology` reuses its source curve's own parametrization) — so
    // the direction `v0 → v1` is the curve's tangent only when the range increases.
    let sense = if ent.range.1 >= ent.range.0 { 1.0 } else { -1.0 };
    let tangent = curve.d1(0.5 * (ent.range.0 + ent.range.1)) * sense;
    unit(if forward { tangent } else { -tangent }, "edge traversal direction")
}

/// 🎨️ Builds the analytic patch for one selected edge, choosing the support from the pair of
/// adjacent surfaces and refusing — never approximating — anything else.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn build_patch(body: &Body, edge: EdgeId, solid_faces: &BTreeSet<FaceId>, spec: BlendSpec) -> Result<Patch, KernelError> {
    let (f0, f1) = edge_two_faces(body, solid_faces, edge)?;
    let ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?.clone();
    let curve = body.curves3.get(ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?.clone();
    let mid = curve.eval(0.5 * (ent.range.0 + ent.range.1));
    let m0 = outward_normal_at(body, f0, mid)?;
    let m1 = outward_normal_at(body, f1, mid)?;
    let s0 = body.surfaces.get(body.faces.get(f0).unwrap().surface).unwrap().clone();
    let s1 = body.surfaces.get(body.faces.get(f1).unwrap().surface).unwrap().clone();
    let base = Patch {
        edge,
        faces: [f0, f1],
        surface: Surface::Plane { frame: Frame3::WORLD },
        cross_dir: IsoDirection::U,
        cross: [0.0, 0.0],
        closed: ent.v0 == ent.v1,
        end_vertex: [ent.v0, ent.v1],
        run: [(0.0, 0.0); 2],
        ends: [EndTrim::Iso(0.0), EndTrim::Iso(0.0)],
    };
    match (&s0, &s1, &curve) {
        (Surface::Plane { .. }, Surface::Plane { .. }, Curve3::Line { origin, dir }) => {
            let along = traversal_direction(body, edge, f0)?;
            let axis = unit(m0.cross(m1), "dihedral axis")?;
            if axis.dot(along) <= 0.0 {
                return Err(KernelError::Operation("blend: only convex edges have a rolling-ball blend on this side".into()));
            }
            let point = *origin + *dir * ent.range.0;
            match spec {
                BlendSpec::Fillet(r) => cylinder_patch(base, point, axis, m0, m1, r),
                BlendSpec::VariableFillet(r0, r1) => cone_fillet_patch(base, &curve, ent.range, m0, m1, r0, r1),
                BlendSpec::Chamfer(d0, d1) => chamfer_plane_patch(base, point, axis, m0, m1, d0, d1),
            }
        }
        (Surface::Plane { .. }, Surface::Cylinder { .. }, Curve3::Circle { .. }) => cap_patch(body, base, 1, 0, spec),
        (Surface::Cylinder { .. }, Surface::Plane { .. }, Curve3::Circle { .. }) => cap_patch(body, base, 0, 1, spec),
        _ => Err(KernelError::Operation("blend: only plane/plane and plane/cylinder-cap edges have an exact analytic blend".into())),
    }
}

/// 🎨️ Constant-radius rolling ball between two planes: the ball centres run along the line at
/// distance `r` inside both, so the envelope is exactly the cylinder of radius `r` about that line
/// and the two tangency curves are its `u = const` rulings. `centre = p − r(m0+m1)/(1+m0·m1)` is
/// the closed form for "distance `r` from both planes" (each dot product collapses the fraction
/// back to `r`); the frame is pinned with `x = m0`, putting face 0's tangency at `u = 0` and face
/// 1's at the dihedral's own exterior angle.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cylinder_patch(base: Patch, point: Pnt3, axis: Vec3, m0: Vec3, m1: Vec3, r: f64) -> Result<Patch, KernelError> {
    let denom = 1.0 + m0.dot(m1);
    if denom <= 1e-6 {
        return Err(KernelError::Operation("blend: the dihedral angle is too flat for a rolling ball".into()));
    }
    let origin = point - (m0 + m1) * (r / denom);
    let frame = Frame3 { origin, x: m0, y: axis.cross(m0), z: axis };
    let theta = m1.dot(frame.y).atan2(m1.dot(frame.x));
    Ok(Patch { surface: Surface::Cylinder { frame, radius: r }, cross_dir: IsoDirection::U, cross: [0.0, theta], ..base })
}

/// 🎨️ Cutting-plane chamfer between two planes: the two cut lines sit at their distances INSIDE
/// each face (`m_i × traversal_i` is that in-face inward direction) and the chamfer face is the
/// plane through both. `x` runs from cut line 0 to cut line 1 — making them its `u = 0` and
/// `u = |A1 − A0|` iso-lines — and `y` is the edge direction, which makes `z = x × y` outward.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn chamfer_plane_patch(base: Patch, point: Pnt3, axis: Vec3, m0: Vec3, m1: Vec3, d0: f64, d1: f64) -> Result<Patch, KernelError> {
    let a0 = point + unit(m0.cross(axis), "chamfer in-face direction")? * d0;
    let a1 = point + unit(m1.cross(-axis), "chamfer in-face direction")? * d1;
    let cross = a1.distance(a0);
    if cross <= BLEND_EPS {
        return Err(KernelError::Operation("blend: the chamfer distances collapse the cut face".into()));
    }
    let x = unit(a1 - a0, "chamfer cross direction")?;
    let frame = Frame3 { origin: a0, x, y: axis, z: x.cross(axis) };
    Ok(Patch { surface: Surface::Plane { frame }, cross_dir: IsoDirection::U, cross: [0.0, cross], ..base })
}

/// 🎨️ Variable-radius rolling ball between two planes: the centres still run along a LINE (both the
/// edge point and the radius are affine in the edge's parameter) with a radius growing linearly
/// along it, so the envelope is exactly the cone of revolution about that line with
/// `sin(half_angle) = dr/ds` — a canal surface of a line is a cone — and the tangency curves are
/// two of its rulings. `v` is distance from the apex, so both tangencies stay iso-lines; the ends,
/// whose cap planes are NOT perpendicular to this tilted axis, are the exact conics of §
/// [`conic_end_trim`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cone_fillet_patch(base: Patch, curve: &Curve3, range: (f64, f64), m0: Vec3, m1: Vec3, r0: f64, r1: f64) -> Result<Patch, KernelError> {
    let denom = 1.0 + m0.dot(m1);
    if denom <= 1e-6 {
        return Err(KernelError::Operation("blend: the dihedral angle is too flat for a rolling ball".into()));
    }
    let bisector = (m0 + m1) * (1.0 / denom);
    let c0 = curve.eval(range.0) - bisector * r0;
    let c1 = curve.eval(range.1) - bisector * r1;
    let span = c1.distance(c0);
    if span <= BLEND_EPS {
        return Err(KernelError::Operation("blend: the variable-radius spine collapses".into()));
    }
    let slope = (r1 - r0) / span;
    if slope.abs() >= 1.0 - DIR_EPS || slope.abs() <= DIR_EPS {
        return Err(KernelError::Operation("blend: this radius variation has no cone envelope — use a constant radius".into()));
    }
    let (spine, start_radius, base_center) = if slope > 0.0 { (unit(c1 - c0, "variable-fillet spine")?, r0, c0) } else { (unit(c0 - c1, "variable-fillet spine")?, r1, c1) };
    let half_angle = slope.abs().asin();
    let apex = base_center - spine * (start_radius / slope.abs());
    let radial0 = unit(m0 - spine * m0.dot(spine), "cone radial reference")?;
    let frame = Frame3 { origin: apex, x: radial0, y: spine.cross(radial0), z: spine };
    let radial1 = m1 - spine * m1.dot(spine);
    // 🧭️ The spine points along GROWING radius, which is the edge's own direction or its opposite
    // depending on which end carries the larger radius, so face 1's tangency ruling can sit on
    // either side of face 0's; the patch simply spans the shorter way between them, and
    // `ring_is_ccw` reads the sign back off `cross` so the ring still walks `(u, v)` positively.
    let theta = radial1.dot(frame.y).atan2(radial1.dot(frame.x));
    if !(theta.abs() > DIR_EPS && theta.abs() < PI - DIR_EPS) {
        return Err(KernelError::Operation("blend: the dihedral angle is too flat for a rolling ball".into()));
    }
    Ok(Patch { surface: Surface::Cone { frame, half_angle }, cross_dir: IsoDirection::U, cross: [0.0, theta], ..base })
}

/// 🎨️ The blend of a circular cap edge between a planar cap and its coaxial cylinder: the ball
/// centres run along a CIRCLE coaxial with the cylinder, so a constant-radius fillet's envelope is
/// exactly a torus (tube radius = the fillet radius, tube angle `v` running from the cylinder's
/// tangency at `v = 0` to the cap's at `v = π/2`) and a chamfer's is exactly a cone frustum (whose
/// two tangency circles are `v = const` iso-lines too). The band is closed in the azimuth, so its
/// two ends are one seam curve used twice — the same device `make_cylinder`'s own lateral face uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn cap_patch(body: &Body, base: Patch, lateral_index: usize, plane_index: usize, spec: BlendSpec) -> Result<Patch, KernelError> {
    let lateral = base.faces[lateral_index];
    let plane_face = base.faces[plane_index];
    if !base.closed {
        return Err(KernelError::Operation("blend: a plane/cylinder edge must be the cylinder's own full cap circle".into()));
    }
    let ent = body.edges.get(base.edge).unwrap().clone();
    let Curve3::Circle { frame: circle_frame, radius: cap_radius } = body.curves3.get(ent.curve).unwrap().clone() else {
        return Err(KernelError::Operation("blend: expected a circular cap edge".into()));
    };
    let Surface::Cylinder { frame: cyl_frame, radius: cyl_radius } = body.surfaces.get(body.faces.get(lateral).unwrap().surface).unwrap().clone() else {
        return Err(KernelError::Operation("blend: expected a cylindrical face".into()));
    };
    if (cyl_radius - cap_radius).abs() > 1e-9 {
        return Err(KernelError::Operation("blend: the cap circle does not lie on its own cylinder".into()));
    }
    let (_, plane_normal) = face_plane(body, plane_face)?;
    if cyl_frame.z.cross(plane_normal).norm() > DIR_EPS {
        return Err(KernelError::Operation("blend: an exact cap blend needs the cap plane perpendicular to the cylinder axis".into()));
    }
    let seam_point = body.vertices.get(ent.v0).ok_or_else(|| KernelError::MissingEntity("vertex".into()))?.position;
    let radial = unit(seam_point - circle_frame.origin, "cap seam radial")?;
    if outward_normal_at(body, lateral, seam_point)?.dot(radial) <= 0.0 {
        return Err(KernelError::Operation("blend: only convex cap edges have a blend on this side".into()));
    }
    let into_solid = -plane_normal;
    let (surface, cross) = match spec {
        BlendSpec::Fillet(r) => {
            if r >= cap_radius {
                return Err(KernelError::Operation("blend: the fillet radius exceeds the cap radius".into()));
            }
            let frame = Frame3 { origin: circle_frame.origin + into_solid * r, x: radial, y: plane_normal.cross(radial), z: plane_normal };
            (Surface::Torus { frame, major_radius: cap_radius - r, minor_radius: r }, [0.0, FRAC_PI_2])
        }
        BlendSpec::Chamfer(first, second) => {
            let (d_lateral, d_plane) = if lateral_index == 0 { (first, second) } else { (second, first) };
            if d_plane >= cap_radius {
                return Err(KernelError::Operation("blend: the chamfer distance exceeds the cap radius".into()));
            }
            let apex_offset = (cap_radius - d_plane) * d_lateral / d_plane;
            let frame = Frame3 { origin: circle_frame.origin - into_solid * apex_offset, x: radial, y: into_solid.cross(radial), z: into_solid };
            (Surface::Cone { frame, half_angle: (d_plane / d_lateral).atan() }, [apex_offset + d_lateral, apex_offset])
        }
        BlendSpec::VariableFillet(..) => return Err(KernelError::Operation("blend: a variable radius needs a straight edge between two planes".into())),
    };
    let cross = if lateral_index == 0 { cross } else { [cross[1], cross[0]] };
    Ok(Patch { surface, cross_dir: IsoDirection::V, cross, ..base })
}
// #endregion 🔖️Patch
