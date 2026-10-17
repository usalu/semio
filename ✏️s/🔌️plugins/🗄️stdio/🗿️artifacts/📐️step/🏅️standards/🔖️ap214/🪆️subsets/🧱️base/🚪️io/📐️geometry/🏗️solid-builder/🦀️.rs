//! 🏗️ Builder of the STEP geometry codec: the step builder that resolves the parsed entities into a solid. See [`super`].

use super::*;

// #region 🔖️Read
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

pub(super) struct StepBuilder<'a> {
    pub(super) body: &'a mut Body,
    pub(super) entities: &'a HashMap<u64, StepEntity>,
    pub(super) rec: &'a mut OpRecorder,
    vertex_cache: HashMap<u64, VertexId>,
    edge_cache: HashMap<u64, EdgeId>,
    tol: Tol,
}

impl<'a> StepBuilder<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn new(body: &'a mut Body, entities: &'a HashMap<u64, StepEntity>, rec: &'a mut OpRecorder) -> Self {
        Self { body, entities, rec, vertex_cache: HashMap::new(), edge_cache: HashMap::new(), tol: Tol::DEFAULT }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn build_all_solids(&mut self) -> Result<(), StepError> {
        let brep_ids: Vec<u64> = self.entities.iter().filter(|(_, e)| e.entity_type == "MANIFOLD_SOLID_BREP").map(|(&id, _)| id).collect();
        for brep_id in brep_ids {
            self.build_solid(brep_id)?;
        }
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_solid(&mut self, brep_id: u64) -> Result<(), StepError> {
        let attrs = self.get_entity(brep_id)?.attrs.clone();
        let shell_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax(format!("MANIFOLD_SOLID_BREP #{brep_id} missing shell")))?;
        let shell_id = self.build_shell(shell_ref)?;
        add_solid(self.body, shell_id, vec![], self.rec);
        Ok(())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_shell(&mut self, shell_ref: u64) -> Result<semio_framework_3d_brep_representation::arena::ShellId, StepError> {
        let attrs = self.get_entity(shell_ref)?.attrs.clone();
        let face_refs = parse_list_refs(&attrs);
        let mut face_ids = Vec::new();
        for face_ref in face_refs {
            face_ids.push(self.build_face(face_ref)?);
        }
        Ok(add_shell(self.body, face_ids, self.rec))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_face(&mut self, face_ref: u64) -> Result<FaceId, StepError> {
        let attrs = self.get_entity(face_ref)?.attrs.clone();
        let orient_tail = attrs.trim_end_matches(')').trim();
        let face_reversed = orient_tail.ends_with(".F.") || orient_tail.ends_with(".FALSE.");
        let all_refs = parse_refs(&attrs);
        let list_refs = parse_list_refs(&attrs);
        let list_set: std::collections::HashSet<u64> = list_refs.iter().copied().collect();
        let surface_ref = all_refs.iter().rev().find(|r| !list_set.contains(r)).copied().ok_or(StepError::Syntax(format!("ADVANCED_FACE #{face_ref} missing surface")))?;
        let surface_id = self.build_surface(surface_ref)?;
        let mut outer_loop = None;
        let mut inner_loops = Vec::new();
        for &bound_ref in &list_refs {
            let bound_entity = self.get_entity(bound_ref)?;
            let is_outer = bound_entity.entity_type == "FACE_OUTER_BOUND";
            let bound_attrs = bound_entity.attrs.clone();
            let loop_ref = parse_refs(&bound_attrs).first().copied().ok_or(StepError::Syntax("face bound missing loop".to_string()))?;
            let members = self.build_loop_members(loop_ref)?;
            let loop_id = make_loop(self.body, placeholder_face(), &members);
            if is_outer && outer_loop.is_none() {
                outer_loop = Some(loop_id);
            } else {
                inner_loops.push(loop_id);
            }
        }
        let outer = outer_loop.or_else(|| if inner_loops.is_empty() { None } else { Some(inner_loops.remove(0)) });
        let outer = outer.ok_or(StepError::Syntax(format!("ADVANCED_FACE #{face_ref} has no bounds")))?;
        let face_id = add_face(self.body, surface_id, Some(outer), inner_loops.clone(), face_reversed, self.tol, self.rec);
        self.body.loops.get_mut(outer).unwrap().face = face_id;
        for loop_id in inner_loops {
            self.body.loops.get_mut(loop_id).unwrap().face = face_id;
        }
        self.attach_pcurves(face_id, surface_id);
        Ok(face_id)
    }

    /// 🎯️ Gives every coedge of a freshly read face its p-curve (`Surface::project_curve` of the
    /// edge's own 3D curve over the edge's own `(0, 1)` range, exact for lines and aligned conics on
    /// planes) — the same contract the kernel's own primitives honour, and the one every boolean's
    /// imprint/classification samples through; a STEP body without them could be tessellated and
    /// measured but never cut, fused or classified.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn attach_pcurves(&mut self, face_id: FaceId, surface_id: SurfaceId) {
        let Some(surface) = self.body.surfaces.get(surface_id).cloned() else { return };
        for coedge_id in self.body.face_coedges(face_id) {
            let Some(coedge) = self.body.coedges.get(coedge_id).cloned() else { continue };
            let Some(edge) = self.body.edges.get(coedge.edge).cloned() else { continue };
            let Some(curve) = self.body.curves3.get(edge.curve).cloned() else { continue };
            let pcurve = self.body.curves2.insert(surface.project_curve(&curve, edge.range, self.tol.0));
            if let Some(coedge) = self.body.coedges.get_mut(coedge_id) {
                coedge.pcurve = Some(pcurve);
                coedge.prange = (0.0, 1.0);
            }
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_loop_members(&mut self, loop_ref: u64) -> Result<Vec<(EdgeId, bool)>, StepError> {
        let attrs = self.get_entity(loop_ref)?.attrs.clone();
        let oe_refs = parse_list_refs(&attrs);
        let mut members = Vec::new();
        for oe_ref in oe_refs {
            members.push(self.build_oriented_edge(oe_ref)?);
        }
        Ok(members)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_oriented_edge(&mut self, oe_ref: u64) -> Result<(EdgeId, bool), StepError> {
        let attrs = self.get_entity(oe_ref)?.attrs.clone();
        let refs = parse_refs(&attrs);
        let forward = attrs.contains(".T.");
        let edge_curve_ref = refs.last().copied().ok_or(StepError::Syntax(format!("ORIENTED_EDGE #{oe_ref} missing edge curve")))?;
        let edge_id = self.build_edge_curve(edge_curve_ref)?;
        Ok((edge_id, forward))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_edge_curve(&mut self, ec_ref: u64) -> Result<EdgeId, StepError> {
        if let Some(&cached) = self.edge_cache.get(&ec_ref) {
            return Ok(cached);
        }
        let attrs = self.get_entity(ec_ref)?.attrs.clone();
        let refs = parse_refs(&attrs);
        if refs.len() < 3 {
            return Err(StepError::Syntax(format!("EDGE_CURVE #{ec_ref} needs at least 3 references")));
        }
        let v0 = self.build_vertex_point(refs[0])?;
        let v1 = self.build_vertex_point(refs[1])?;
        let curve_id = self.build_curve_geometry(refs[2], v0, v1)?;
        let edge_id = make_edge(self.body, curve_id, (0.0, 1.0), v0, v1, self.tol, self.rec);
        self.edge_cache.insert(ec_ref, edge_id);
        Ok(edge_id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_curve_geometry(&mut self, curve_ref: u64, v0: VertexId, v1: VertexId) -> Result<Curve3Id, StepError> {
        let entity = self.get_entity(curve_ref)?;
        let entity_type = entity.entity_type.clone();
        let attrs = entity.attrs.clone();
        let p0 = self.body.vertices.get(v0).unwrap().position;
        let p1 = self.body.vertices.get(v1).unwrap().position;
        let curve = match entity_type.as_str() {
            "LINE" => Curve3::Line { origin: p0, dir: p1 - p0 },
            "CIRCLE" => {
                let axis_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax("CIRCLE missing axis".to_string()))?;
                let radius = parse_floats(&attrs).first().copied().ok_or(StepError::Syntax("CIRCLE missing radius".to_string()))?;
                let (center, normal, u_axis) = self.build_axis2_placement(axis_ref)?;
                let frame = Frame3::from_x_z(center, u_axis, normal).ok_or(StepError::Syntax("invalid circle frame".to_string()))?;
                Curve3::Circle { frame, radius }
            }
            "ELLIPSE" => {
                let axis_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax("ELLIPSE missing axis".to_string()))?;
                let floats = parse_floats(&attrs);
                if floats.len() < 2 {
                    return Err(StepError::Syntax("ELLIPSE needs semi axes".to_string()));
                }
                let (center, normal, u_axis) = self.build_axis2_placement(axis_ref)?;
                let frame = Frame3::from_x_z(center, u_axis, normal).ok_or(StepError::Syntax("invalid ellipse frame".to_string()))?;
                Curve3::Ellipse { frame, major_radius: floats[0], minor_radius: floats[1] }
            }
            "B_SPLINE_CURVE_WITH_KNOTS" => self.build_bspline_curve(curve_ref, &attrs, p0, p1)?,
            _ if entity_type.is_empty() || attrs.contains("B_SPLINE_CURVE_WITH_KNOTS") => {
                let bspline_attrs = find_composite_bspline_attrs(&attrs, "B_SPLINE_CURVE").ok_or_else(|| StepError::Unsupported(format!("composite curve #{curve_ref}")))?;
                self.build_bspline_curve(curve_ref, bspline_attrs, p0, p1)?
            }
            other => return Err(StepError::Unsupported(other.to_string())),
        };
        Ok(self.body.curves3.insert(curve))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_bspline_curve(&self, curve_ref: u64, attrs: &str, p0: Pnt3, p1: Pnt3) -> Result<Curve3, StepError> {
        let (degree, cp_refs, mults, knot_vals) = parse_bspline_curve_attrs(attrs).ok_or_else(|| StepError::Syntax(format!("B_SPLINE_CURVE #{curve_ref} parse failed")))?;
        let mut control_points = Vec::with_capacity(cp_refs.len());
        for &cp_ref in &cp_refs {
            control_points.push(self.build_cartesian_point(cp_ref)?);
        }
        if straight_control_polygon(&control_points) {
            return Ok(Curve3::Line { origin: p0, dir: p1 - p0 });
        }
        let knots = expand_knots(&mults, &knot_vals);
        let n = control_points.len();
        let knot_vec = KnotVector::new(knots, degree, n).ok_or(StepError::Syntax("invalid B-spline knots".to_string()))?;
        let weights = vec![1.0; n];
        Ok(Curve3::Nurbs { knots: knot_vec, controls: control_points, weights })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_surface(&mut self, surface_ref: u64) -> Result<SurfaceId, StepError> {
        let entity = self.get_entity(surface_ref)?;
        let entity_type = entity.entity_type.clone();
        let attrs = entity.attrs.clone();
        let surface = match entity_type.as_str() {
            "PLANE" => {
                let axis_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax("PLANE missing axis".to_string()))?;
                let (origin, axis, ref_dir) = self.build_axis2_placement(axis_ref)?;
                let frame = Frame3::from_x_z(origin, ref_dir, axis).ok_or(StepError::Syntax("invalid plane frame".to_string()))?;
                Surface::Plane { frame }
            }
            "CYLINDRICAL_SURFACE" => {
                let axis_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax("CYLINDRICAL_SURFACE missing axis".to_string()))?;
                let radius = parse_floats(&attrs).first().copied().ok_or(StepError::Syntax("CYLINDRICAL_SURFACE missing radius".to_string()))?;
                let (origin, axis, ref_dir) = self.build_axis2_placement(axis_ref)?;
                let frame = Frame3::from_x_z(origin, ref_dir, axis).ok_or(StepError::Syntax("invalid cylinder frame".to_string()))?;
                Surface::Cylinder { frame, radius }
            }
            "CONICAL_SURFACE" => {
                let axis_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax("CONICAL_SURFACE missing axis".to_string()))?;
                let half_angle = parse_floats(&attrs).last().copied().ok_or(StepError::Syntax("CONICAL_SURFACE missing half_angle".to_string()))?;
                let (origin, axis, ref_dir) = self.build_axis2_placement(axis_ref)?;
                let frame = Frame3::from_x_z(origin, ref_dir, axis).ok_or(StepError::Syntax("invalid cone frame".to_string()))?;
                Surface::Cone { frame, half_angle }
            }
            "SPHERICAL_SURFACE" => {
                let axis_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax("SPHERICAL_SURFACE missing axis".to_string()))?;
                let radius = parse_floats(&attrs).first().copied().ok_or(StepError::Syntax("SPHERICAL_SURFACE missing radius".to_string()))?;
                let (center, axis, ref_dir) = self.build_axis2_placement(axis_ref)?;
                let frame = Frame3::from_x_z(center, ref_dir, axis).ok_or(StepError::Syntax("invalid sphere frame".to_string()))?;
                Surface::Sphere { frame, radius }
            }
            "TOROIDAL_SURFACE" => {
                let axis_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax("TOROIDAL_SURFACE missing axis".to_string()))?;
                let floats = parse_floats(&attrs);
                if floats.len() < 2 {
                    return Err(StepError::Syntax("TOROIDAL_SURFACE missing radii".to_string()));
                }
                let (center, axis, ref_dir) = self.build_axis2_placement(axis_ref)?;
                let frame = Frame3::from_x_z(center, ref_dir, axis).ok_or(StepError::Syntax("invalid torus frame".to_string()))?;
                Surface::Torus { frame, major_radius: floats[0], minor_radius: floats[1] }
            }
            "B_SPLINE_SURFACE_WITH_KNOTS" | "BOUNDED_SURFACE" | "B_SPLINE_SURFACE" => self.build_bspline_surface(surface_ref, &attrs)?,
            _ if entity_type.is_empty() || attrs.contains("B_SPLINE_SURFACE_WITH_KNOTS") => {
                let bspline_attrs = find_composite_bspline_attrs(&attrs, "B_SPLINE_SURFACE").ok_or_else(|| StepError::Unsupported(format!("composite surface #{surface_ref}")))?;
                self.build_bspline_surface(surface_ref, bspline_attrs)?
            }
            other => return Err(StepError::Unsupported(other.to_string())),
        };
        Ok(self.body.surfaces.insert(surface))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_bspline_surface(&self, surface_ref: u64, attrs: &str) -> Result<Surface, StepError> {
        let (degree_u, degree_v, cp_grid_refs, u_mults, v_mults, u_knots, v_knots) = parse_bspline_surface_attrs(attrs).ok_or_else(|| StepError::Syntax(format!("B_SPLINE_SURFACE #{surface_ref} parse failed")))?;
        let mut cp_grid: Vec<Vec<Pnt3>> = Vec::new();
        for row_refs in &cp_grid_refs {
            let mut row: Vec<Pnt3> = Vec::new();
            for &cp_ref in row_refs {
                row.push(self.build_cartesian_point(cp_ref)?);
            }
            cp_grid.push(row);
        }
        let knots_u = expand_knots(&u_mults, &u_knots);
        let knots_v = expand_knots(&v_mults, &v_knots);
        let n_rows = cp_grid.len();
        let n_cols = cp_grid.first().map_or(0, Vec::len);
        let u_kv = KnotVector::new(knots_u, degree_u, n_cols).ok_or(StepError::Syntax("invalid surface u knots".to_string()))?;
        let v_kv = KnotVector::new(knots_v, degree_v, n_rows).ok_or(StepError::Syntax("invalid surface v knots".to_string()))?;
        let weights = vec![vec![1.0; n_cols]; n_rows];
        Ok(Surface::Nurbs { u_knots: u_kv, v_knots: v_kv, controls: cp_grid, weights })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_vertex_point(&mut self, vp_ref: u64) -> Result<VertexId, StepError> {
        if let Some(&cached) = self.vertex_cache.get(&vp_ref) {
            return Ok(cached);
        }
        let attrs = self.get_entity(vp_ref)?.attrs.clone();
        let cp_ref = parse_refs(&attrs).first().copied().ok_or(StepError::Syntax(format!("VERTEX_POINT #{vp_ref} missing point")))?;
        let point = self.build_cartesian_point(cp_ref)?;
        let vid = make_vertex(self.body, point, self.tol, self.rec);
        self.vertex_cache.insert(vp_ref, vid);
        Ok(vid)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_cartesian_point(&self, cp_ref: u64) -> Result<Pnt3, StepError> {
        let coords = parse_floats(&self.get_entity(cp_ref)?.attrs);
        if coords.len() < 3 {
            return Err(StepError::Syntax(format!("CARTESIAN_POINT #{cp_ref} needs 3 coordinates")));
        }
        Ok(Pnt3::new(coords[0], coords[1], coords[2]))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_direction(&self, dir_ref: u64) -> Result<Vec3, StepError> {
        let coords = parse_floats(&self.get_entity(dir_ref)?.attrs);
        if coords.len() < 3 {
            return Err(StepError::Syntax(format!("DIRECTION #{dir_ref} needs 3 components")));
        }
        Ok(Vec3::new(coords[0], coords[1], coords[2]))
    }

    /// 🧭️ `AXIS2_PLACEMENT_3D(name, location, axis, ref_direction)` with ISO 10303-42's optional
    /// slots honoured: an absent (`$`) `axis` is `+Z`, an absent `ref_direction` is the standard's
    /// `first_proj_axis` derivation (`+X` projected onto the axis plane, `+Y` when the axis is `±X`) —
    /// real Rhino/ST-Developer exports write `$` for the ref direction of every planar face.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_axis2_placement(&self, axis_ref: u64) -> Result<(Pnt3, Vec3, Vec3), StepError> {
        let attrs = self.get_entity(axis_ref)?.attrs.clone();
        let slots = parse_slots(&attrs);
        let slot_ref = |index: usize| slots.get(index).and_then(|slot| parse_entity_id(slot));
        let location = slot_ref(1).ok_or_else(|| StepError::Syntax(format!("AXIS2_PLACEMENT_3D #{axis_ref} needs a location reference")))?;
        let origin = self.build_cartesian_point(location)?;
        let axis = match slot_ref(2) {
            Some(direction) => self.build_direction(direction)?,
            None => Vec3::Z,
        };
        let ref_direction = match slot_ref(3) {
            Some(direction) => self.build_direction(direction)?,
            None => first_proj_axis(axis),
        };
        Ok((origin, axis, ref_direction))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn get_entity(&self, id: u64) -> Result<&StepEntity, StepError> {
        self.entities.get(&id).ok_or(StepError::UnresolvedReference(id))
    }
}
// #endregion 🔖️Read
