//! 🧱️ Primitive and curve constructors: the revision bookkeeping and the synchronous primitive, curve, wire and surface constructors of the session. See [`super`].

use super::*;

// #region 🔖️SyncApi
impl Brep {
    pub fn box_prim_sync(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut self.native.body, width, depth, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_solid_primitive(solid, SolidPrimitive::axis_box(width, depth, height)))
    }
    pub fn sphere_prim_sync(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_sphere(&mut self.native.body, radius, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_solid_primitive(solid, SolidPrimitive::sphere(radius)))
    }
    pub fn cylinder_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cylinder(&mut self.native.body, radius, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_solid_primitive(solid, SolidPrimitive::cylinder(radius, height)))
    }
    pub fn cone_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_cone(&mut self.native.body, radius, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_solid_primitive(solid, SolidPrimitive::cone(radius, height)))
    }
    pub fn torus_prim_sync(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let solid = make_torus(&mut self.native.body, major, minor, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_solid_primitive(solid, SolidPrimitive::torus(major, minor)))
    }
    pub fn line_curve_sync(&mut self, start: EVec3, end: EVec3) -> Result<GeometryHandle, BrepError> {
        Ok(self.native.register_curve(Curve3::Line { origin: pnt(start), dir: pnt(end) - pnt(start) }))
    }
    pub fn circle_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad circle frame".into()))?;
        Ok(self.native.register_curve(Curve3::Circle { frame, radius }))
    }
    pub fn arc_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        let recipe=analytic_arc_recipe(center,normal,radius,start_angle,end_angle).map_err(|error|BrepError::InvalidInput(error.to_string()))?;
        let (controls,weights)=(0..recipe.control_count()).map(|i|recipe.control(i)).unzip();
        let knots=(0..recipe.knot_count()).map(|i|recipe.knot(i)).collect();
        let knots=semio_framework_3d_brep_curve::bspline::KnotVector{knots,degree:2};
        Ok(self.native.register_curve(Curve3::Nurbs{knots,controls,weights}))
    }
    pub fn ellipse_curve_sync(&mut self, center: EVec3, normal: EVec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad ellipse frame".into()))?;
        Ok(self.native.register_curve(Curve3::Ellipse { frame, major_radius: semi_major, minor_radius: semi_minor }))
    }
    pub fn polyline_wire_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let closed = points.len() > 1 && points.first() == points.last();
        let unique = &points[..points.len() - usize::from(closed)];
        let pts: Vec<Pnt3> = unique.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let wire = make_polyline_wire(&mut self.native.body, &pts, closed, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_wire(wire))
    }
    pub fn rectangle_wire_sync(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_rectangle_wire(&mut self.native.body, width, height, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_wire(wire))
    }
    pub fn regular_polygon_wire_sync(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let wire = make_regular_polygon_wire(&mut self.native.body, radius, sides, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_wire(wire))
    }
    pub fn interpolate_curve_sync(&mut self, points: &[EVec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        if points.len() < 2 {
            return Err(BrepError::InvalidInput("interpolate_curve needs at least 2 points".into()));
        }
        let controls: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let nurbs = interpolate_curve(&controls, degree, ParamMethod::Centripetal, None, false).ok_or_else(|| BrepError::InvalidInput("interpolate_curve: degenerate or coincident points".into()))?;
        Ok(self.native.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    pub fn approximate_curve_sync(&mut self, points: &[EVec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        if points.len() < 2 {
            return Err(BrepError::InvalidInput("approximate_curve needs at least 2 points".into()));
        }
        let controls: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let target = control_points.clamp(2, controls.len());
        let (nurbs, _err) = approximate_curve_with_count(&controls, degree, target).ok_or_else(|| BrepError::InvalidInput("approximate_curve: degenerate input or infeasible control-point count".into()))?;
        Ok(self.native.register_curve(Curve3::Nurbs { knots: nurbs.knots, controls: nurbs.controls, weights: nurbs.weights }))
    }
    pub fn helix_curve_sync(&mut self, origin: EVec3, axis: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let recipe = HelixWireRecipe::new(origin, axis, radius, pitch, turns).map_err(|error| BrepError::InvalidInput(error.to_string()))?;
        let mut pts = Vec::new();
        pts.try_reserve_exact(recipe.sample_count()).map_err(|error| BrepError::InvalidInput(error.to_string()))?;
        pts.extend((0..recipe.sample_count()).map(|index| recipe.sample(index)));
        self.polyline_wire_sync(&pts)
    }
    pub fn plane_surface_sync(&mut self, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(origin), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad plane".into()))?;
        Ok(self.native.register_surface(Surface::Plane { frame }))
    }
    pub fn planar_face_from_points_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let mut rec = OpRecorder::new();
        let face = make_planar_face_from_points(&mut self.native.body, &pts, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_face(face))
    }
    pub fn planar_face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boundary_face_sync(wire, true)
    }
    pub fn nurbs_surface_from_grid_sync(&mut self, points: &[Vec<EVec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        if points.is_empty() || points[0].is_empty() {
            return Err(BrepError::InvalidInput("nurbs grid requires a non-empty control net".into()));
        }
        let grid: Vec<Vec<Pnt3>> = points.iter().map(|row| row.iter().copied().map(pnt).collect()).collect();
        let surface = interpolate_surface_grid(&grid, degree_u, degree_v).ok_or_else(|| BrepError::InvalidInput("nurbs_surface_from_grid: ragged rows or degenerate net".into()))?;
        Ok(self.native.register_surface(surface))
    }
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
        Ok(self.native.register_surface(surface))
    }
}
// #endregion 🔖️SyncApi
