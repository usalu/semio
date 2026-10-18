//! 🔍️ Intersection, evaluation and measure: the curve and surface intersections, the curve and surface evaluation, the measures, the closest point, the classification, the validation and the vertex, face and sew constructors. See [`super`].

use super::*;

// #region 🔖️SyncApi
impl Brep {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_curve_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let ca = self.curve_ref(a)?;
        let cb = self.curve_ref(b)?;
        let hits = intersect_curve_curve(ca, cb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_surface_intersect_sync(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let c = self.curve_ref(curve)?;
        let s = self.surface_ref(surface)?;
        let hits = intersect_curve_surface(c, s, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_surface_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let sa = self.surface_ref(a)?;
        let sb = self.surface_ref(b)?;
        let hits = intersect_surface_surface(sa, sb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|hit| self.register_curve(hit.curve3)).collect())
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_point_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        Ok(evec(self.curve_ref(curve)?.eval(parameter)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_tangent_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        let c = self.curve_ref(curve)?;
        let p0 = c.eval(parameter);
        let p1 = c.eval(parameter + 1e-5);
        let d = p1 - p0;
        Ok([d.x, d.y, d.z])
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_domain_sync(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        let c = self.curve_ref(curve)?;
        let (min, max) = c.domain();
        Ok(ParamDomain { min, max })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_curvature_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        Ok(self.curve_ref(curve)?.curvature(parameter))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_point_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.surface_ref(surface)?;
        Ok(evec(s.eval(u, v)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_normal_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.surface_ref(surface)?;
        let n = s.normal(u, v).ok_or_else(|| BrepError::Operation("surface normal undefined".into()))?;
        Ok([n.x, n.y, n.z])
    }
    /// 🧭️ Certified closest parameter on `curve` to `point` — see [`BrepKernel::curve_closest_parameter`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn curve_closest_parameter_sync(&self, curve: &GeometryHandle, point: EVec3) -> Result<(f64, EVec3, f64), BrepError> {
        let c = self.curve_ref(curve)?;
        let cp = curve_closest_parameter_fn(c, c.domain(), pnt(point), 1e-9);
        Ok((cp.t, evec(cp.point), cp.distance))
    }
    /// 🧭️ Certified closest `(u, v)` on `surface` to `point` — see [`BrepKernel::surface_closest_uv`].
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn surface_closest_uv_sync(&self, surface: &GeometryHandle, point: EVec3) -> Result<(f64, f64, EVec3, f64), BrepError> {
        let s = self.surface_ref(surface)?;
        let cp = surface_closest_uv_fn(s, s.domain(), pnt(point), 1e-9);
        Ok((cp.u, cp.v, evec(cp.point), cp.distance))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => solid_volume(&self.body, *id, 1e-6).map_err(|error| map_err(&error)),
            entity @ Entity::Compound(_, _) => {
                let mut solids: Vec<_> = self.body.reachable_from(&entity_roots(entity)).solids.into_iter().collect();
                solids.sort_unstable_by_key(|id| id.raw_index());
                solids.into_iter().try_fold(0.0, |total, id| solid_volume(&self.body, id, 1e-6).map(|value| total + value).map_err(|error| map_err(&error)))
            }
            _ => Err(BrepError::InvalidInput("volume requires a solid or compound".into())),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn area_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => solid_surface_area(&self.body, *id, 1e-4).map_err(|error| map_err(&error)),
            Entity::Face(id) => face_area(&self.body, *id, 1e-4).map_err(|error| map_err(&error)),
            Entity::Shell(id)=>self.body.shells.get(*id).ok_or_else(||BrepError::InvalidInput("missing shell".into()))?.faces.iter().try_fold(0.0,|area,face|face_area(&self.body,*face,1e-4).map(|value|area+value).map_err(|error|map_err(&error))),
            entity @ Entity::Compound(_, _) => {
                let mut faces: Vec<_> = self.body.reachable_from(&entity_roots(entity)).faces.into_iter().collect();
                faces.sort_unstable_by_key(|id| id.raw_index());
                faces.into_iter().try_fold(0.0, |total, id| face_area(&self.body, id, 1e-4).map(|value| total + value).map_err(|error| map_err(&error)))
            }
            _ => Err(BrepError::InvalidInput("area requires a face, shell, solid, or compound".into())),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn length_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.entity(shape)? {
            Entity::Vertex(_) => Ok(0.0),
            Entity::Edge(id) => edge_length(&self.body, *id).map_err(|error| map_err(&error)),
            Entity::Curve(curve, _) => {
                let (start, end) = curve.domain();
                if !start.is_finite() || !end.is_finite() { return Err(BrepError::InvalidInput("length requires a bounded curve".into())); }
                let length = semio_framework_3d_brep_curve::curve_ops::arc_length(curve, start, end, 1e-9);
                if !length.is_finite() { return Err(BrepError::InvalidInput("curve length is not finite".into())); }
                Ok(length)
            }
            Entity::Surface(_, _) => Err(BrepError::InvalidInput("length requires a curve or topological boundary".into())),
            entity => {
                let mut edges: Vec<_> = self.body.reachable_from(&entity_roots(entity)).edges.into_iter().collect();
                edges.sort_unstable_by_key(|id| id.raw_index());
                edges.into_iter().try_fold(0.0, |total, id| edge_length(&self.body, id).map(|value| total + value).map_err(|error| map_err(&error)))
            }
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn center_of_mass_sync(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        let solid = self.solid_id(shape)?;
        Ok(evec(solid_center_of_mass(&self.body, solid, 1e-4).map_err(|error| map_err(&error))?))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn bounding_box_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(|error| map_err(&error))?;
        let corners = [
            evec(bb.min),
            evec(Pnt3::new(bb.max.x, bb.min.y, bb.min.z)),
            evec(Pnt3::new(bb.max.x, bb.max.y, bb.min.z)),
            evec(Pnt3::new(bb.min.x, bb.max.y, bb.min.z)),
            evec(Pnt3::new(bb.min.x, bb.min.y, bb.max.z)),
            evec(Pnt3::new(bb.max.x, bb.min.y, bb.max.z)),
            evec(bb.max),
            evec(Pnt3::new(bb.min.x, bb.max.y, bb.max.z)),
        ];
        self.convex_hull_sync(&corners)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn distance_sync(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        distance_solid_solid(&self.body, sa, sb).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn closest_point_sync(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        let vertex = |id| {
            let position = self.body.vertices.get(id).ok_or_else(|| BrepError::InvalidInput("missing vertex".into()))?.position;
            Ok(ClosestPoint { distance: (position - pnt(point)).norm(), point: evec(position), parameter: None, uv: None })
        };
        let edge = |id| {
            let edge = self.body.edges.get(id).ok_or_else(|| BrepError::InvalidInput("missing edge".into()))?;
            let curve = self.body.curves3.get(edge.curve).ok_or_else(|| BrepError::InvalidInput("missing edge curve".into()))?;
            let closest = curve_closest_parameter_fn(curve, edge.range, pnt(point), 1e-9);
            Ok(ClosestPoint { distance: closest.distance, point: evec(closest.point), parameter: Some(closest.t), uv: None })
        };
        match self.entity(shape)? {
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
            Entity::Solid(id) => {
                let (p, d) = closest_point_on_solid(&self.body, *id, pnt(point)).map_err(|error| map_err(&error))?;
                Ok(ClosestPoint { distance: d, point: evec(p), parameter: None, uv: None })
            }
            entity => {
                let reach = self.body.reachable_from(&entity_roots(entity));
                let mut best: Option<ClosestPoint> = None;
                let mut consider = |candidate: ClosestPoint| {
                    if best.as_ref().is_none_or(|best| candidate.distance < best.distance) { best = Some(candidate); }
                };
                let mut faces: Vec<_> = reach.faces.into_iter().collect();
                faces.sort_by_key(|id| id.raw_index());
                for face in &faces {
                    let (position, distance) = closest_point_on_face(&self.body, *face, pnt(point)).map_err(|error| map_err(&error))?;
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn classify_point_sync(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        let id = self.solid_id(solid)?;
        point_in_solid(&self.body, id, pnt(point), 1e-6).map_err(|error| map_err(&error))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn vertex_sync(&mut self, point: EVec3) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let id = make_vertex(&mut self.body, pnt(point), Tol::DEFAULT, &mut rec);
        Ok(self.mint(GeometryKind::Vertex, Entity::Vertex(id)))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sew_faces_sync(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mut fids = Vec::new();
        for f in faces {
            fids.push(self.face_id(f)?);
        }
        let mut rec = OpRecorder::new();
        let solid = sew_faces(&mut self.body, &fids, tolerance, &mut rec).map_err(|error| map_err(&error))?;
        Ok(self.register_solid(solid))
    }
}
// #endregion 🔖️SyncApi
