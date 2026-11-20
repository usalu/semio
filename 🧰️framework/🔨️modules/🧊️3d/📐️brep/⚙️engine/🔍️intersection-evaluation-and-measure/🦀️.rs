//! 🔍️ Intersection, evaluation and measure: the curve and surface intersections, the curve and surface evaluation, the measures, the closest point, the classification, the validation and the vertex, face and sew constructors. See [`super`].

use super::*;
use semio_framework_3d_brep_analysis::{self as analysis, nearest_analytic};
use semio_framework_value::{ValueError, ValueRefusalKind};


#[path = "🎯️nearest-planar/🦀️.rs"]
mod nearest_planar;
pub use nearest_planar::PlanarClosestJob;

#[path = "✂️line-intersections/🦀️.rs"]
mod line_intersections;

// #region 🔖️SyncApi
impl Brep {
    pub fn curve_curve_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let ca = self.native.curve_ref(a)?;
        let cb = self.native.curve_ref(b)?;
        let hits = intersect_curve_curve(ca, cb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    pub fn curve_surface_intersect_sync(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let c = self.native.curve_ref(curve)?;
        let s = self.native.surface_ref(surface)?;
        let hits = intersect_curve_surface(c, s, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    pub fn surface_surface_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let sa = self.native.surface_ref(a)?;
        let sb = self.native.surface_ref(b)?;
        let hits = intersect_surface_surface(sa, sb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|hit| self.native.register_curve(hit.curve3)).collect())
    }
    pub fn curve_point_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        Ok(evec(self.native.curve_ref(curve)?.eval(parameter)))
    }
    pub fn curve_tangent_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        let tangent = self.native.curve_ref(curve)?.d1(parameter);
        Ok([tangent.x, tangent.y, tangent.z])
    }

    fn original_curve(&self, handle: &str) -> Result<&Curve3, ValueError> {
        match self.native.original_query_entity(handle)? {
            Entity::Curve(curve, _) => Ok(curve),
            _ => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve evaluation requires a curve")),
        }
    }

    /// 📍️ Evaluates a fixed-size analytic curve without staging a temporary handle or spline work.
    pub fn original_curve_sample(&self, handle: &str, parameter: f64, tangent: bool) -> Result<EVec3, ValueError> {
        if !parameter.is_finite() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve sampling requires a finite parameter"));
        }
        let curve = self.original_curve(handle)?;
        analysis::analytic_curve_sample(curve, parameter, tangent)
    }

    /// 📏️ Reads the bounded parameter span without evaluating or cloning the curve.
    pub fn original_curve_span(&self, handle: &str) -> Result<f64, ValueError> {
        analysis::analytic_curve_span(self.original_curve(handle)?)
    }

    /// 🪝️ Reads curvature from an analytic curve's fixed derivatives.
    pub fn original_curve_curvature(&self, handle: &str, parameter: f64) -> Result<f64, ValueError> {
        if !parameter.is_finite() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve curvature requires a finite parameter"));
        }
        let curve = self.original_curve(handle)?;
        analysis::analytic_curve_curvature(curve, parameter)
    }

    /// 📐️ Measures vertices, analytic line or circular edges, and complete ellipses with fixed work.
    pub fn original_geometry_length(&self, handle: &str) -> Result<f64, ValueError> {
        let (curve, range) = match self.native.original_query_entity(handle)? {
            Entity::Vertex(_) => return Ok(0.0),
            Entity::Curve(curve, _) => (curve, curve.domain()),
            Entity::Edge(id) => {
                let edge = self.native.body.edges.get(*id).ok_or_else(|| ValueError::invariant("live edge record is missing"))?;
                let curve = self.native.body.curves3.get(edge.curve).ok_or_else(|| ValueError::invariant("live edge curve is missing"))?;
                (curve, edge.range)
            }
            _ => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "boundary length requires an original topology traversal")),
        };
        analysis::analytic_curve_length(curve, range)
    }

    pub fn curve_domain_sync(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        let c = self.native.curve_ref(curve)?;
        let (min, max) = c.domain();
        Ok(ParamDomain { min, max })
    }
    pub fn curve_curvature_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        Ok(self.native.curve_ref(curve)?.curvature(parameter))
    }
    pub fn surface_point_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.native.surface_ref(surface)?;
        Ok(evec(s.eval(u, v)))
    }
    pub fn surface_normal_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.native.surface_ref(surface)?;
        let n = s.normal(u, v).ok_or_else(|| BrepError::Operation("surface normal undefined".into()))?;
        Ok([n.x, n.y, n.z])
    }
    /// 🧭️ Evaluates one analytic surface point or normal without copying spline control nets.
    pub fn original_surface_sample(&self, handle: &str, u: f64, v: f64, normal: bool) -> Result<EVec3, ValueError> {
        if !u.is_finite() || !v.is_finite() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "surface sampling requires finite parameters"));
        }
        let Entity::Surface(surface, _) = self.native.original_query_entity(handle)? else {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "surface evaluation requires a surface"));
        };
        analysis::analytic_surface_sample(surface, u, v, normal)
    }

    /// 🎯️ Certified closest parameter on `curve` to `point` — see [`BrepKernel::curve_closest_parameter`].
    pub fn curve_closest_parameter_sync(&self, curve: &GeometryHandle, point: EVec3) -> Result<(f64, EVec3, f64), BrepError> {
        let c = self.native.curve_ref(curve)?;
        let cp = curve_closest_parameter_fn(c, c.domain(), pnt(point), 1e-9);
        Ok((cp.t, evec(cp.point), cp.distance))
    }
    /// 🧲️ Certified closest `(u, v)` on `surface` to `point` — see [`BrepKernel::surface_closest_uv`].
    pub fn surface_closest_uv_sync(&self, surface: &GeometryHandle, point: EVec3) -> Result<(f64, f64, EVec3, f64), BrepError> {
        let s = self.native.surface_ref(surface)?;
        let cp = surface_closest_uv_fn(s, s.domain(), pnt(point), 1e-9);
        Ok((cp.u, cp.v, evec(cp.point), cp.distance))
    }
    /// 🎯️ Finds the original nearest parameter on a native analytic curve.
    pub fn original_curve_closest_parameter(&self,handle:&str,target:EVec3)->Result<(f64,EVec3,f64),ValueError>{
        let ([parameter],point,distance)=nearest_analytic::curve_nearest(self.original_curve(handle)?,pnt(target))?;
        Ok((parameter,evec(point),distance))
    }

    /// 🏄️ Finds the original nearest UV on a native analytic surface.
    pub fn original_surface_closest_uv(&self,handle:&str,target:EVec3)->Result<(f64,f64,EVec3,f64),ValueError>{
        let Entity::Surface(surface,_)=self.native.original_query_entity(handle)? else{
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"nearest UV requires a surface"));
        };
        let ([u,v],point,distance)=nearest_analytic::surface_nearest(surface,pnt(target))?;
        Ok((u,v,evec(point),distance))
    }

    /// 🧪️ Reads exact volume from a live constructor-certified solid under bounded original work.
    pub fn original_primitive_volume(&self, handle: &str) -> Result<f64, semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind};
        let volume = self.live_certified_volume(handle)
            .ok_or_else(|| ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original volume requires a constructor-certified solid"))?;
        if !volume.is_finite() { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "primitive volume exceeds finite numeric range")); }
        Ok(volume)
    }

    /// 📏️ Measures a certified solid or a single complete planar conic trim without resampling.
    pub fn original_analytic_area(&self, handle:&str) -> Result<Option<f64>,semio_framework_value::ValueError> {
        use semio_framework_value::{ValueError,ValueRefusalKind};
        let area = match self.native.original_query_entity(handle)? {
            Entity::Face(id) => match self.conic_face_preview_source(*id).map(|(curve,_,_)| curve) {
                Some(Curve3::Circle{radius,..}) => std::f64::consts::PI * radius.powi(2),
                Some(Curve3::Ellipse{major_radius,minor_radius,..}) => std::f64::consts::PI * major_radius * minor_radius,
                _ => return Ok(None),
            },
            _ => match self.live_solid_primitive(handle) { Some(shape)=>shape.area(),None=>return Ok(None) },
        };
        if !area.is_finite() { return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"analytic area exceeds finite numeric range")); }
        Ok(Some(area))
    }

    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.native.entity(shape)? {
            Entity::Solid(id, _) => solid_volume(&self.native.body, *id, 1e-6).map_err(|error| map_err(&error)),
            entity @ Entity::Compound(_, _) => {
                let mut solids: Vec<_> = self.native.body.reachable_from(&entity_roots(entity)).solids.into_iter().collect();
                solids.sort_unstable_by_key(|id| id.raw_index());
                solids.into_iter().try_fold(0.0, |total, id| solid_volume(&self.native.body, id, 1e-6).map(|value| total + value).map_err(|error| map_err(&error)))
            }
            _ => Err(BrepError::InvalidInput("volume requires a solid or compound".into())),
        }
    }
    pub fn area_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.native.entity(shape)? {
            Entity::Solid(id, _) => solid_surface_area(&self.native.body, *id, 1e-4).map_err(|error| map_err(&error)),
            Entity::Face(id) => face_area(&self.native.body, *id, 1e-4).map_err(|error| map_err(&error)),
            Entity::Shell(id)=>self.native.body.shells.get(*id).ok_or_else(||BrepError::InvalidInput("missing shell".into()))?.faces.iter().try_fold(0.0,|area,face|face_area(&self.native.body,*face,1e-4).map(|value|area+value).map_err(|error|map_err(&error))),
            entity @ Entity::Compound(_, _) => {
                let mut faces: Vec<_> = self.native.body.reachable_from(&entity_roots(entity)).faces.into_iter().collect();
                faces.sort_unstable_by_key(|id| id.raw_index());
                faces.into_iter().try_fold(0.0, |total, id| face_area(&self.native.body, id, 1e-4).map(|value| total + value).map_err(|error| map_err(&error)))
            }
            _ => Err(BrepError::InvalidInput("area requires a face, shell, solid, or compound".into())),
        }
    }
    pub fn length_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.native.entity(shape)? {
            Entity::Vertex(_) => Ok(0.0),
            Entity::Edge(id) => edge_length(&self.native.body, *id).map_err(|error| map_err(&error)),
            Entity::Curve(curve, _) => {
                let (start, end) = curve.domain();
                if !start.is_finite() || !end.is_finite() { return Err(BrepError::InvalidInput("length requires a bounded curve".into())); }
                use semio_framework_3d_brep_curve::curve_ops::{arc_length, ellipse_perimeter};
                let length = match curve {
                    Curve3::Circle { radius, .. } => std::f64::consts::TAU * radius.abs(),
                    Curve3::Ellipse { major_radius, minor_radius, .. } => ellipse_perimeter(*major_radius, *minor_radius),
                    _ => arc_length(curve, start, end, 1e-9),
                };
                if !length.is_finite() { return Err(BrepError::InvalidInput("curve length is not finite".into())); }
                Ok(length)
            }
            Entity::Surface(_, _) => Err(BrepError::InvalidInput("length requires a curve or topological boundary".into())),
            entity => {
                let mut edges: Vec<_> = self.native.body.reachable_from(&entity_roots(entity)).edges.into_iter().collect();
                edges.sort_unstable_by_key(|id| id.raw_index());
                edges.into_iter().try_fold(0.0, |total, id| edge_length(&self.native.body, id).map(|value| total + value).map_err(|error| map_err(&error)))
            }
        }
    }
    pub fn center_of_mass_sync(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        let solid = self.native.solid_id(shape)?;
        Ok(evec(solid_center_of_mass(&self.native.body, solid, 1e-4).map_err(|error| map_err(&error))?))
    }
    pub fn bounding_box_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let [min, max] = match self.native.original_axis_bounds(shape.as_str()) {
            Ok(Some(bounds)) => bounds,
            _ => {
                let solid = self.native.solid_id(shape)?;
                let bounds = solid_bounding_box(&self.native.body, solid).map_err(|error| map_err(&error))?;
                [bounds.min, bounds.max]
            }
        };
        let corners = [
            evec(min),
            evec(Pnt3::new(max.x, min.y, min.z)),
            evec(Pnt3::new(max.x, max.y, min.z)),
            evec(Pnt3::new(min.x, max.y, min.z)),
            evec(Pnt3::new(min.x, min.y, max.z)),
            evec(Pnt3::new(max.x, min.y, max.z)),
            evec(max),
            evec(Pnt3::new(min.x, max.y, max.z)),
        ];
        self.convex_hull_sync(&corners)
    }
    pub fn distance_sync(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        let sa = self.native.solid_id(a)?;
        let sb = self.native.solid_id(b)?;
        distance_solid_solid(&self.native.body, sa, sb).map_err(|error| map_err(&error))
    }
    /// 🪜️ Quotes the topology rows needed for the bounded closest-point query.
    pub fn original_closest_point_depth_demand(&self,handle:&str)->Result<usize,ValueError>{
        let depth=self.native.tessellation_source_depth_demand(handle)?;
        let body=&self.native.body;
        let shape_depth=match self.native.original_query_entity(handle)?{
            Entity::Vertex(id)=>body.vertices.next_remove_depth_demand(*id)?,
            Entity::Edge(id)=>{
                let edge=body.edges.get(*id).ok_or_else(||ValueError::invariant("live edge is missing"))?;
                body.edges.next_remove_depth_demand(*id)?.max(body.curves3.next_remove_depth_demand(edge.curve)?)
            },
            Entity::Face(id)=>analysis::closest_face_depth(body,*id)?,
            _=>1,
        };
        Ok(depth.max(shape_depth))
    }

    /// 📌️ Projects onto analytic geometry, fitted circular trims and their native planar disks.
    pub fn original_geometry_closest_point(&self, handle: &str, target: EVec3) -> Result<EVec3, ValueError> {
        if target.iter().any(|value| !value.is_finite()) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "closest point requires finite coordinates"));
        }
        let target = pnt(target);
        let result = match self.native.original_query_entity(handle)? {
            Entity::Vertex(id) => self.native.body.vertices.get(*id).ok_or_else(|| ValueError::invariant("live vertex is missing"))?.position,
            Entity::Curve(curve,_)=>nearest_analytic::curve_nearest(curve,target)?.1,
            Entity::Edge(id) => {
                let edge = self.native.body.edges.get(*id).ok_or_else(|| ValueError::invariant("live edge is missing"))?;
                let curve = self.native.body.curves3.get(edge.curve).ok_or_else(|| ValueError::invariant("live edge curve is missing"))?;
                nearest_analytic::edge_nearest(curve,target,edge.range)?
            }
            Entity::Face(id)=>{
                let (curve,_,_)=self.conic_face_preview_source(*id).ok_or_else(||
                    ValueError::literal(ValueRefusalKind::UnsupportedOwner,"closest point on a trimmed face requires original boundary traversal"))?;
                nearest_analytic::conic_disk_nearest(curve,target)?
            }
            Entity::Surface(surface,_)=>nearest_analytic::surface_nearest(surface,target)?.1,
            Entity::Solid(_,Some(SolidProvenance::Primitive(provenance)))=>nearest_analytic::primitive_nearest(provenance.primitive,target)?,
            _ => return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "geometry requires original closest-point traversal")),
        };
        let axes = evec(result);
        if axes.iter().any(|value| !value.is_finite()) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "closest point must have finite coordinates"));
        }
        Ok(axes)
    }

    pub fn closest_point_sync(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        let vertex = |id| {
            let position = self.native.body.vertices.get(id).ok_or_else(|| BrepError::InvalidInput("missing vertex".into()))?.position;
            Ok(ClosestPoint { distance: (position - pnt(point)).norm(), point: evec(position), parameter: None, uv: None })
        };
        let edge = |id| {
            let edge = self.native.body.edges.get(id).ok_or_else(|| BrepError::InvalidInput("missing edge".into()))?;
            let curve = self.native.body.curves3.get(edge.curve).ok_or_else(|| BrepError::InvalidInput("missing edge curve".into()))?;
            let closest = curve_closest_parameter_fn(curve, edge.range, pnt(point), 1e-9);
            Ok(ClosestPoint { distance: closest.distance, point: evec(closest.point), parameter: Some(closest.t), uv: None })
        };
        match self.native.entity(shape)? {
            Entity::Vertex(id) => vertex(*id),
            Entity::Edge(id) => edge(*id),
            Entity::Curve(_, _) => {
                let (t, p, d) = self.curve_closest_parameter_sync(shape, point)?;
                Ok(ClosestPoint { distance: d, point: p, parameter: Some(t), uv: None })
            }
            Entity::Surface(_, _) => {
                let (u, v, p, d) = self.surface_closest_uv_sync(shape, point)?;
                Ok(ClosestPoint { distance: d, point: p, parameter: None, uv: Some([u, v]) })
            }
            Entity::Solid(id, _) => {
                let (p, d) = closest_point_on_solid(&self.native.body, *id, pnt(point)).map_err(|error| map_err(&error))?;
                Ok(ClosestPoint { distance: d, point: evec(p), parameter: None, uv: None })
            }
            entity => {
                let reach = self.native.body.reachable_from(&entity_roots(entity));
                let mut best: Option<ClosestPoint> = None;
                let mut consider = |candidate: ClosestPoint| {
                    if best.as_ref().is_none_or(|best| candidate.distance < best.distance) { best = Some(candidate); }
                };
                let mut faces: Vec<_> = reach.faces.into_iter().collect();
                faces.sort_by_key(|id| id.raw_index());
                for face in &faces {
                    let (position, distance) = closest_point_on_face(&self.native.body, *face, pnt(point)).map_err(|error| map_err(&error))?;
                    consider(ClosestPoint { distance, point: evec(position), parameter: None, uv: None });
                }
                if faces.is_empty() {
                    let mut edges: Vec<_> = reach.edges.into_iter().collect();
                    edges.sort_by_key(|id| id.raw_index());
                    for id in edges { consider(edge(id)?); }
                    let mut vertices: Vec<_> = reach.vertices.into_iter().collect();
                    vertices.sort_by_key(|id| id.raw_index());
                    for id in vertices { consider(vertex(id)?); }
                }
                best.ok_or_else(|| BrepError::InvalidInput("shape has no closest-point support".into()))
            }
        }
    }
    pub fn classify_point_sync(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        let id = self.native.solid_id(solid)?;
        point_in_solid(&self.native.body, id, pnt(point), 1e-6).map_err(|error| map_err(&error))
    }
    pub fn validate_sync(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        let validity = self.validity_sync(shape)?;
        let report = semio_framework_pack_json::object([
            ("ok".into(), semio_framework_pack_json::Value::Bool(validity.ok)),
            ("issueCount".into(), semio_framework_pack_json::Value::from(validity.issues.len() as u64)),
            ("issues".into(), semio_framework_pack_json::Value::Array(validity.issues.iter().map(|issue| semio_framework_pack_json::object([
                ("entity".into(), semio_framework_pack_json::Value::String(issue.entity.clone())),
                ("code".into(), semio_framework_pack_json::Value::String(issue.code.clone())),
                ("message".into(), semio_framework_pack_json::Value::String(issue.message.clone())),
            ])).collect())),
        ]);
        Ok(semio_framework_pack_json::to_string(&report))
    }
    pub fn vertex_sync(&mut self, point: EVec3) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let id = make_vertex(&mut self.native.body, pnt(point), Tol::DEFAULT, &mut rec);
        Ok(self.native.mint(GeometryKind::Vertex, Entity::Vertex(id)))
    }
    pub fn face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.boundary_face_sync(wire, false)
    }
    pub fn sew_faces_sync(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mut fids = Vec::new();
        for f in faces {
            fids.push(self.native.face_id(f)?);
        }
        let mut rec = OpRecorder::new();
        let solid = sew_faces(&mut self.native.body, &fids, tolerance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.native.register_solid(solid))
    }
}
// #endregion 🔖️SyncApi
