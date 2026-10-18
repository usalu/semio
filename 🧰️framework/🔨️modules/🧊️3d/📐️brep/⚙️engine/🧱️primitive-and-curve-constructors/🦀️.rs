//! 🧱️ Primitive and curve constructors: the revision bookkeeping and the synchronous primitive, curve, wire and surface constructors of the session. See [`super`].

use super::*;

// #region 🔖️SyncApi
impl Brep {
    pub(super) fn advance_live_revision(&mut self){self.live_revision=self.live_revision.checked_add(1).expect("original live registry revision overflow");}
    pub(super) fn retention_source_revision(&self)->(u64,u64){(self.live_revision,self.body.labels.next())}
    /// 👀️ Borrows the original live registry without constructing a handle owner.
    pub fn contains_live_handle(&self,handle:&str)->bool{self.live.get(handle).is_some()}
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn box_prim_sync(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut self.body, width, depth, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sphere_prim_sync(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_sphere(&mut self.body, radius, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cylinder_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cylinder(&mut self.body, radius, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn cone_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cone(&mut self.body, radius, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn torus_prim_sync(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_torus(&mut self.body, major, minor, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn line_curve_sync(&mut self, start: EVec3, end: EVec3) -> Result<GeometryHandle, BrepError> {
        Ok(self.register_curve(Curve3::Line { origin: pnt(start), dir: pnt(end) - pnt(start) }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn circle_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad circle frame".into()))?;
        Ok(self.register_curve(Curve3::Circle { frame, radius }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn arc_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        if !(radius.is_finite() && radius > 0.0) {
            return Err(BrepError::InvalidInput("arc radius must be positive".into()));
        }
        if !start_angle.is_finite() || !end_angle.is_finite() {
            return Err(BrepError::InvalidInput("arc angles must be finite".into()));
        }
        if (end_angle - start_angle).abs() <= 1e-15 {
            return Err(BrepError::InvalidInput("arc start and end angles must differ".into()));
        }
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad arc frame".into()))?;
        let circle = Curve3::Circle { frame, radius };
        let nurbs = circle.to_nurbs((start_angle, end_angle));
        Ok(self.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ellipse_curve_sync(&mut self, center: EVec3, normal: EVec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad ellipse frame".into()))?;
        Ok(self.register_curve(Curve3::Ellipse { frame, major_radius: semi_major, minor_radius: semi_minor }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn polyline_wire_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let wire = make_polyline_wire(&mut self.body, &pts, false, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rectangle_wire_sync(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut self.body, width, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn regular_polygon_wire_sync(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_regular_polygon_wire(&mut self.body, radius, sides, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_wire(wire))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn interpolate_curve_sync(&mut self, points: &[EVec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        if points.len() < 2 {
            return Err(BrepError::InvalidInput("interpolate_curve needs at least 2 points".into()));
        }
        let controls: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let nurbs = interpolate_curve(&controls, degree, ParamMethod::Centripetal, None, false).ok_or_else(|| BrepError::InvalidInput("interpolate_curve: degenerate or coincident points".into()))?;
        Ok(self.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn approximate_curve_sync(&mut self, points: &[EVec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        if points.len() < 2 {
            return Err(BrepError::InvalidInput("approximate_curve needs at least 2 points".into()));
        }
        let controls: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let target = control_points.clamp(2, controls.len());
        let (nurbs, _err) = approximate_curve_with_count(&controls, degree, target).ok_or_else(|| BrepError::InvalidInput("approximate_curve: degenerate input or infeasible control-point count".into()))?;
        Ok(self.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn helix_curve_sync(&mut self, origin: EVec3, axis: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let mut pts = Vec::new();
        let n = ((turns.abs() * 32.0).ceil() as usize).max(8);
        let axis_v = vec3(axis).normalized().unwrap_or(NativeVec3::Z);
        let frame = Frame3::from_normal(pnt(origin), axis_v).ok_or_else(|| BrepError::InvalidInput("bad helix".into()))?;
        for i in 0..=n {
            let t = i as f64 / n as f64 * turns;
            let ang = t * std::f64::consts::TAU;
            let p = frame.origin + frame.x * (radius * ang.cos()) + frame.y * (radius * ang.sin()) + axis_v * (pitch * t);
            pts.push(evec(p));
        }
        self.polyline_wire_sync(&pts)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn plane_surface_sync(&mut self, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(origin), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad plane".into()))?;
        Ok(self.register_surface(Surface::Plane { frame }))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn planar_face_from_points_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let face = make_planar_face_from_points(&mut self.body, &pts, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_face(face))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn planar_face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let wire = self.wire_ref(wire)?.clone();
        let mut rec = OpRecorder::new();
        let w = transform_wire(&mut self.body, &wire, &Affine3::IDENTITY, &mut rec);
        let origin = self.body.vertices.get(w.vertices[0]).map_or(Pnt3::new(0.0, 0.0, 0.0), |v| v.position);
        let face = make_planar_face_from_wire(&mut self.body, &w, origin, NativeVec3::Z, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_face(face))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn nurbs_surface_from_grid_sync(&mut self, points: &[Vec<EVec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        if points.is_empty() || points[0].is_empty() {
            return Err(BrepError::InvalidInput("nurbs grid requires a non-empty control net".into()));
        }
        let grid: Vec<Vec<Pnt3>> = points.iter().map(|row| row.iter().copied().map(pnt).collect()).collect();
        let surface = interpolate_surface_grid(&grid, degree_u, degree_v).ok_or_else(|| BrepError::InvalidInput("nurbs_surface_from_grid: ragged rows or degenerate net".into()))?;
        Ok(self.register_surface(surface))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn coons_patch_sync(&mut self, curves: &[Vec<EVec3>]) -> Result<GeometryHandle, BrepError> {
        if curves.len() != 4 {
            return Err(BrepError::InvalidInput("coons_patch requires exactly 4 boundary polylines".into()));
        }
        let mut boundary_curves = Vec::with_capacity(4);
        for (idx, curve) in curves.iter().enumerate() {
            if curve.len() < 2 {
                return Err(BrepError::InvalidInput(format!("coons_patch boundary {idx} needs at least 2 points")));
            }
            let pts: Vec<Pnt3> = curve.iter().copied().map(pnt).collect();
            let degree = pts.len().saturating_sub(1).min(3);
            let nurbs = interpolate_curve(&pts, degree, ParamMethod::Centripetal, None, false).ok_or_else(|| BrepError::InvalidInput(format!("coons_patch boundary {idx} is degenerate")))?;
            boundary_curves.push(nurbs);
        }
        let [c0, d1, c1, d0] = [&boundary_curves[0], &boundary_curves[1], &boundary_curves[2], &boundary_curves[3]];
        let surface = coons_patch_nurbs(c0, c1, d0, d1, 1e-6).ok_or_else(|| BrepError::InvalidInput("coons_patch: boundary corners do not match or bases are incompatible".into()))?;
        Ok(self.register_surface(surface))
    }
}
// #endregion 🔖️SyncApi
