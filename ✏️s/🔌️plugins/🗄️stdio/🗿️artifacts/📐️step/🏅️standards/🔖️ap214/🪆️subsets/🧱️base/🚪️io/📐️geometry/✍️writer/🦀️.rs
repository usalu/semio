//! ✍️ Writer of the STEP geometry codec: the product ids, the write context that emits the entity graph and the numeric helpers. See [`super`].

use super::*;

// #region 🔖️Write
struct ProductIds {
    pub(super) definition: u64,
}

pub(super) struct StepWriteContext {
    pub(super) next: u64,
    pub(super) entities: String,
    vertex_map: HashMap<u32, u64>,
    edge_map: HashMap<u32, u64>,
}

impl StepWriteContext {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn new() -> Self {
        Self { next: 1, entities: String::new(), vertex_map: HashMap::new(), edge_map: HashMap::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn next_id(&mut self) -> u64 {
        let id = self.next;
        self.next += 1;
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn write_entity(&mut self, id: u64, entity: &str, attrs: &str) {
        let _ = writeln!(self.entities, "#{id} = {entity}({attrs};");
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_point(&mut self, p: Pnt3) -> u64 {
        let id = self.next_id();
        self.write_entity(id, "CARTESIAN_POINT", &format!("'', ({}, {}, {}))", fmt_f64(p.x), fmt_f64(p.y), fmt_f64(p.z)));
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_direction(&mut self, d: Vec3) -> u64 {
        let id = self.next_id();
        self.write_entity(id, "DIRECTION", &format!("'', ({}, {}, {}))", fmt_f64(d.x), fmt_f64(d.y), fmt_f64(d.z)));
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_axis2_placement(&mut self, origin: Pnt3, axis: Vec3, ref_dir: Vec3) -> u64 {
        let origin_id = self.write_point(origin);
        let axis_id = self.write_direction(axis);
        let ref_id = self.write_direction(ref_dir);
        let id = self.next_id();
        self.write_entity(id, "AXIS2_PLACEMENT_3D", &format!("'', #{origin_id}, #{axis_id}, #{ref_id})"));
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn write_geometric_context(&mut self) -> u64 {
        let len_unit = self.next_id();
        let _ = writeln!(self.entities, "#{len_unit} = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) );");
        let angle_unit = self.next_id();
        let _ = writeln!(self.entities, "#{angle_unit} = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) );");
        let solid_angle_unit = self.next_id();
        let _ = writeln!(self.entities, "#{solid_angle_unit} = ( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() );");
        let uncertainty = self.next_id();
        self.write_entity(uncertainty, "UNCERTAINTY_MEASURE_WITH_UNIT", &format!("LENGTH_MEASURE(1.E-07), #{len_unit}, 'distance_accuracy_value', 'confusion accuracy')"));
        let ctx = self.next_id();
        let _ = writeln!(
            self.entities,
            "#{ctx} = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) \
             GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{uncertainty})) \
             GLOBAL_UNIT_ASSIGNED_CONTEXT((#{len_unit},#{angle_unit},#{solid_angle_unit})) \
             REPRESENTATION_CONTEXT('Context3D','3D Context with UNIT and UNCERTAINTY') );"
        );
        ctx
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn write_product_structure(&mut self) -> ProductIds {
        let app_context = self.next_id();
        self.write_entity(app_context, "APPLICATION_CONTEXT", "'configuration controlled 3D design of mechanical parts and assemblies')");
        let mech_context = self.next_id();
        self.write_entity(mech_context, "MECHANICAL_CONTEXT", &format!("'', #{app_context}, 'mechanical')"));
        let protocol_def = self.next_id();
        self.write_entity(protocol_def, "APPLICATION_PROTOCOL_DEFINITION", &format!("'international standard', 'config_control_design', 1994, #{app_context})"));
        let product = self.next_id();
        self.write_entity(product, "PRODUCT", &format!("'semio_solid', 'semio_solid', '', (#{mech_context}))"));
        let formation = self.next_id();
        self.write_entity(formation, "PRODUCT_DEFINITION_FORMATION", &format!("'', '', #{product})"));
        let def_context = self.next_id();
        self.write_entity(def_context, "PRODUCT_DEFINITION_CONTEXT", &format!("'part definition', #{app_context}, 'design')"));
        let definition = self.next_id();
        self.write_entity(definition, "PRODUCT_DEFINITION", &format!("'design', '', #{formation}, #{def_context})"));
        ProductIds { definition }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_vertex(&mut self, body: &Body, vid: VertexId) -> Result<u64, StepError> {
        let key = vid.raw_index();
        if let Some(&cached) = self.vertex_map.get(&key) {
            return Ok(cached);
        }
        let vertex = body.vertices.get(vid).ok_or(StepError::Syntax(format!("missing vertex {vid}")))?;
        let pt_id = self.write_point(vertex.position);
        let vp_id = self.next_id();
        self.write_entity(vp_id, "VERTEX_POINT", &format!("'', #{pt_id})"));
        self.vertex_map.insert(key, vp_id);
        Ok(vp_id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_edge_curve(&mut self, body: &Body, eid: EdgeId) -> Result<u64, StepError> {
        let key = eid.raw_index();
        if let Some(&cached) = self.edge_map.get(&key) {
            return Ok(cached);
        }
        let edge = body.edges.get(eid).ok_or(StepError::Syntax(format!("missing edge {eid}")))?;
        let start_vp = self.write_vertex(body, edge.v0)?;
        let end_vp = self.write_vertex(body, edge.v1)?;
        let start_pt = body.vertices.get(edge.v0).unwrap().position;
        let end_pt = body.vertices.get(edge.v1).unwrap().position;
        let curve = body.curves3.get(edge.curve).ok_or(StepError::Syntax("missing edge curve".to_string()))?;
        let curve_id = match curve {
            Curve3::Line { .. } => {
                let dir = (end_pt - start_pt).normalized().unwrap_or(Vec3::X);
                let length = (end_pt - start_pt).norm();
                let line_origin = self.write_point(start_pt);
                let dir_id = self.write_direction(dir);
                let vector = self.next_id();
                self.write_entity(vector, "VECTOR", &format!("'', #{dir_id}, {})", fmt_f64(length)));
                let line = self.next_id();
                self.write_entity(line, "LINE", &format!("'', #{line_origin}, #{vector})"));
                line
            }
            Curve3::Circle { frame, radius } => {
                let placement = self.write_axis2_placement(frame.origin, frame.z, frame.x);
                let cid = self.next_id();
                self.write_entity(cid, "CIRCLE", &format!("'', #{placement}, {})", fmt_f64(*radius)));
                cid
            }
            Curve3::Ellipse { frame, major_radius, minor_radius } => {
                let placement = self.write_axis2_placement(frame.origin, frame.z, frame.x);
                let eid_step = self.next_id();
                self.write_entity(eid_step, "ELLIPSE", &format!("'', #{placement}, {}, {})", fmt_f64(*major_radius), fmt_f64(*minor_radius)));
                eid_step
            }
            Curve3::Nurbs { knots, controls, weights } => self.write_nurbs_curve(knots, controls, weights),
        };
        let edge_curve = self.next_id();
        self.write_entity(edge_curve, "EDGE_CURVE", &format!("'', #{start_vp}, #{end_vp}, #{curve_id}, .T.)"));
        self.edge_map.insert(key, edge_curve);
        Ok(edge_curve)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_nurbs_curve(&mut self, knots: &KnotVector, controls: &[Pnt3], weights: &[f64]) -> u64 {
        let cp_ids: Vec<u64> = controls.iter().map(|p| self.write_point(*p)).collect();
        let cp_refs: Vec<String> = cp_ids.iter().map(|id| format!("#{id}")).collect();
        let (knot_mults, knot_vals) = compute_knot_multiplicities(&knots.knots);
        let mults_str: Vec<String> = knot_mults.iter().map(ToString::to_string).collect();
        let vals_str: Vec<String> = knot_vals.iter().map(|v| fmt_f64(*v)).collect();
        let id = self.next_id();
        let _ = writeln!(
            self.entities,
            "#{id} = B_SPLINE_CURVE_WITH_KNOTS('', {}, ({}), \
             .UNSPECIFIED., .F., .F., ({}), ({}), .UNSPECIFIED.);",
            knots.degree,
            cp_refs.join(", "),
            mults_str.join(", "),
            vals_str.join(", ")
        );
        let _ = weights;
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_edge_loop(&mut self, body: &Body, loop_id: semio_framework_3d_brep_representation::arena::LoopId) -> Result<u64, StepError> {
        let mut oriented_edge_ids = Vec::new();
        for coedge_id in body.loop_coedges(loop_id) {
            let coedge = body.coedges.get(coedge_id).ok_or(StepError::Syntax("missing coedge".to_string()))?;
            let edge_curve = self.write_edge_curve(body, coedge.edge)?;
            let oriented_edge = self.next_id();
            let orient = if coedge.forward { ".T." } else { ".F." };
            self.write_entity(oriented_edge, "ORIENTED_EDGE", &format!("'', *, *, #{edge_curve}, {orient})"));
            oriented_edge_ids.push(oriented_edge);
        }
        let refs: Vec<String> = oriented_edge_ids.iter().map(|id| format!("#{id}")).collect();
        let step_loop = self.next_id();
        self.write_entity(step_loop, "EDGE_LOOP", &format!("'', ({}))", refs.join(", ")));
        Ok(step_loop)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_surface(&mut self, body: &Body, surface_id: SurfaceId) -> Result<u64, StepError> {
        let surface = body.surfaces.get(surface_id).ok_or(StepError::Syntax("missing surface".to_string()))?;
        Ok(match surface {
            Surface::Plane { frame } => {
                let ref_dir = compute_ref_direction(frame.z);
                let axis = self.write_axis2_placement(frame.origin, frame.z, ref_dir);
                let plane = self.next_id();
                self.write_entity(plane, "PLANE", &format!("'', #{axis})"));
                plane
            }
            Surface::Cylinder { frame, radius } => {
                let ref_dir = compute_ref_direction(frame.z);
                let axis = self.write_axis2_placement(frame.origin, frame.z, ref_dir);
                let id = self.next_id();
                self.write_entity(id, "CYLINDRICAL_SURFACE", &format!("'', #{axis}, {:.15E})", radius));
                id
            }
            Surface::Cone { frame, half_angle } => {
                let ref_dir = compute_ref_direction(frame.z);
                let axis = self.write_axis2_placement(frame.origin, frame.z, ref_dir);
                let id = self.next_id();
                self.write_entity(id, "CONICAL_SURFACE", &format!("'', #{axis}, 0.0E0, {:.15E})", half_angle));
                id
            }
            Surface::Sphere { frame, radius } => {
                let ref_dir = compute_ref_direction(frame.z);
                let axis = self.write_axis2_placement(frame.origin, frame.z, ref_dir);
                let id = self.next_id();
                self.write_entity(id, "SPHERICAL_SURFACE", &format!("'', #{axis}, {:.15E})", radius));
                id
            }
            Surface::Torus { frame, major_radius, minor_radius } => {
                let ref_dir = compute_ref_direction(frame.z);
                let axis = self.write_axis2_placement(frame.origin, frame.z, ref_dir);
                let id = self.next_id();
                self.write_entity(id, "TOROIDAL_SURFACE", &format!("'', #{axis}, {:.15E}, {:.15E})", major_radius, minor_radius));
                id
            }
            Surface::Nurbs { u_knots, v_knots, controls, weights } => self.write_nurbs_surface(u_knots, v_knots, controls, weights)?,
        })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_nurbs_surface(&mut self, u_knots: &KnotVector, v_knots: &KnotVector, controls: &[Vec<Pnt3>], weights: &[Vec<f64>]) -> Result<u64, StepError> {
        if controls.is_empty() {
            return Err(StepError::Syntax("NURBS surface has no control points".to_string()));
        }
        let mut cp_grid_refs = Vec::new();
        for row in controls {
            let row_ids: Vec<u64> = row.iter().map(|p| self.write_point(*p)).collect();
            let row_refs: Vec<String> = row_ids.iter().map(|id| format!("#{id}")).collect();
            cp_grid_refs.push(format!("({})", row_refs.join(", ")));
        }
        let (u_mults, u_vals) = compute_knot_multiplicities(&u_knots.knots);
        let (v_mults, v_vals) = compute_knot_multiplicities(&v_knots.knots);
        let u_mults_str: Vec<String> = u_mults.iter().map(ToString::to_string).collect();
        let u_vals_str: Vec<String> = u_vals.iter().map(|v| fmt_f64(*v)).collect();
        let v_mults_str: Vec<String> = v_mults.iter().map(ToString::to_string).collect();
        let v_vals_str: Vec<String> = v_vals.iter().map(|v| fmt_f64(*v)).collect();
        let id = self.next_id();
        let _ = writeln!(
            self.entities,
            "#{id} = B_SPLINE_SURFACE_WITH_KNOTS('', {}, {}, ({}), \
             .UNSPECIFIED., .F., .F., .F., ({}), ({}), ({}), ({}), .UNSPECIFIED.);",
            u_knots.degree,
            v_knots.degree,
            cp_grid_refs.join(", "),
            u_mults_str.join(", "),
            v_mults_str.join(", "),
            u_vals_str.join(", "),
            v_vals_str.join(", ")
        );
        let _ = weights;
        Ok(id)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_face(&mut self, body: &Body, face_id: FaceId) -> Result<u64, StepError> {
        let face = body.faces.get(face_id).ok_or(StepError::Syntax("missing face".to_string()))?;
        let mut bound_ids = Vec::new();
        if let Some(outer) = face.outer {
            let outer_loop = self.write_edge_loop(body, outer)?;
            let outer_bound = self.next_id();
            self.write_entity(outer_bound, "FACE_OUTER_BOUND", &format!("'', #{outer_loop}, .T.)"));
            bound_ids.push(outer_bound);
        }
        for &inner in &face.inners {
            let inner_loop = self.write_edge_loop(body, inner)?;
            let inner_bound = self.next_id();
            self.write_entity(inner_bound, "FACE_BOUND", &format!("'', #{inner_loop}, .T.)"));
            bound_ids.push(inner_bound);
        }
        let surface_id = self.write_surface(body, face.surface)?;
        let bound_refs: Vec<String> = bound_ids.iter().map(|id| format!("#{id}")).collect();
        let face_orient = if face.flipped { ".F." } else { ".T." };
        let advanced_face = self.next_id();
        self.write_entity(advanced_face, "ADVANCED_FACE", &format!("'', ({}), #{surface_id}, {face_orient})", bound_refs.join(", ")));
        Ok(advanced_face)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_shell(&mut self, body: &Body, shell_id: semio_framework_3d_brep_representation::arena::ShellId) -> Result<u64, StepError> {
        let shell = body.shells.get(shell_id).ok_or(StepError::Syntax("missing shell".to_string()))?;
        let mut face_step_ids = Vec::new();
        for &face_id in &shell.faces {
            face_step_ids.push(self.write_face(body, face_id)?);
        }
        let refs: Vec<String> = face_step_ids.iter().map(|id| format!("#{id}")).collect();
        let closed_shell = self.next_id();
        self.write_entity(closed_shell, "CLOSED_SHELL", &format!("'', ({}))", refs.join(", ")));
        Ok(closed_shell)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn write_solid(&mut self, body: &Body, solid_id: SolidId) -> Result<u64, StepError> {
        let solid = body.solids.get(solid_id).ok_or(StepError::Syntax("missing solid".to_string()))?;
        let shell = self.write_shell(body, solid.outer)?;
        let brep = self.next_id();
        self.write_entity(brep, "MANIFOLD_SOLID_BREP", &format!("'', #{shell})"));
        Ok(brep)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn finish(self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "ISO-10303-21;");
        let _ = writeln!(out, "HEADER;");
        let _ = writeln!(out, "FILE_DESCRIPTION(('semio STEP export'), '2;1');");
        let _ = writeln!(out, "FILE_NAME('output.stp', '2024-01-01T00:00:00', (''), (''), 'semio', 'semio', '');");
        let _ = writeln!(out, "FILE_SCHEMA(('CONFIG_CONTROL_DESIGN'));");
        let _ = writeln!(out, "ENDSEC;");
        let _ = writeln!(out, "DATA;");
        out.push_str(&self.entities);
        let _ = writeln!(out, "ENDSEC;");
        let _ = writeln!(out, "END-ISO-10303-21;");
        out
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn fmt_f64(v: f64) -> String {
    if v.abs() < 1e-15 {
        "0.".to_string()
    } else {
        format!("{v:.15E}")
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn compute_ref_direction(normal: Vec3) -> Vec3 {
    let ax = Vec3::X;
    let ay = Vec3::Y;
    let candidate = if normal.dot(ax).abs() < 0.9 { ax } else { ay };
    normal.cross(candidate).normalized().unwrap_or(ax)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn compute_knot_multiplicities(knots: &[f64]) -> (Vec<u32>, Vec<f64>) {
    if knots.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let mut mults = Vec::new();
    let mut vals = Vec::new();
    let mut current = knots[0];
    let mut count = 1u32;
    for &k in &knots[1..] {
        if (k - current).abs() < 1e-10 {
            count += 1;
        } else {
            mults.push(count);
            vals.push(current);
            current = k;
            count = 1;
        }
    }
    mults.push(count);
    vals.push(current);
    (mults, vals)
}
// #endregion 🔖️Write
