//! 📄 Hand-rolled ISO 10303-21 STEP reader/writer subset (MANIFOLD_SOLID_BREP, ADVANCED_FACE,
//! analytic surfaces/curves and B-spline entities). See ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3, nested under
//! `⚙️engine` (not a top-level `📸️snapshot`/`🔺️diff` compute dir — none was pre-mounted for it)
//! since its only real consumer is `⚙️engine`'s own `BrepKernel::export_step`/`import_step`,
//! same pattern as the pre-existing `📦️mesh-io` sibling. KNOWN DUPLICATE, not resolved here:
//! stdio already has a separately-complete, tested AP214 STEP↔SemioBrep walk under
//! `✳️brep/🚪️io` (`SemioBrepToStep`/`SemioBrepFromStep` + `artifacts::step`'s generic Part-21
//! tokenizer). Reconciling the two needs `⚙️engine`'s `BrepKernel` impl rewired — explicitly
//! out of scope for this wave (see `📌️important.md`, "BrepKernel — do NOT attempt").

use std::collections::HashMap;
use std::fmt::Write as _;

use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, Curve3Id, EdgeId, FaceId, SolidId, SurfaceId, VertexId};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::bspline::KnotVector;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::curve::Curve3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::error::StepError;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Frame3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::{Pnt3, Vec3};

// #region 🔖️Api

/// 📄 Serializes one or more manifold solids from `body` to STEP Part 21 (AP203 subset).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_step(body: &Body, solids: &[SolidId]) -> Result<String, StepError> {
    if solids.is_empty() {
        return Err(StepError::Syntax("no solids to export".to_string()));
    }
    let mut ctx = StepWriteContext::new();
    let repr_context_id = ctx.write_geometric_context();
    let product_ids = ctx.write_product_structure();
    let mut brep_ids = Vec::new();
    for &solid_id in solids {
        brep_ids.push(ctx.write_solid(body, solid_id)?);
    }
    let items: Vec<String> = brep_ids.iter().map(|id| format!("#{id}")).collect();
    let shape_repr_id = ctx.next_id();
    ctx.write_entity(shape_repr_id, "ADVANCED_BREP_SHAPE_REPRESENTATION", &format!("'semio export', ({},), #{repr_context_id})", items.join(", ")));
    let prod_def_shape_id = ctx.next_id();
    ctx.write_entity(prod_def_shape_id, "PRODUCT_DEFINITION_SHAPE", &format!("'','',#{})", product_ids.definition));
    let shape_def_repr_id = ctx.next_id();
    ctx.write_entity(shape_def_repr_id, "SHAPE_DEFINITION_REPRESENTATION", &format!("#{prod_def_shape_id}, #{shape_repr_id})"));
    Ok(ctx.finish())
}

/// 📄 Parses STEP Part 21 text into a fresh [`Body`] containing every `MANIFOLD_SOLID_BREP` found.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn read_step(text: &str) -> Result<Body, StepError> {
    let entities = parse_step_entities(text)?;
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let mut builder = StepBuilder::new(&mut body, &entities, &mut rec);
    builder.build_all_solids()?;
    Ok(body)
}

// #endregion 🔖️Api

// #region 🔖️Write

struct ProductIds {
    definition: u64,
}

struct StepWriteContext {
    next: u64,
    entities: String,
    vertex_map: HashMap<u32, u64>,
    edge_map: HashMap<u32, u64>,
}

impl StepWriteContext {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new() -> Self {
        Self { next: 1, entities: String::new(), vertex_map: HashMap::new(), edge_map: HashMap::new() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn next_id(&mut self) -> u64 {
        let id = self.next;
        self.next += 1;
        id
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn write_entity(&mut self, id: u64, entity: &str, attrs: &str) {
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
    fn write_geometric_context(&mut self) -> u64 {
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
    fn write_product_structure(&mut self) -> ProductIds {
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
    fn write_edge_loop(&mut self, body: &Body, loop_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::LoopId) -> Result<u64, StepError> {
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
    fn write_shell(&mut self, body: &Body, shell_id: crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::ShellId) -> Result<u64, StepError> {
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
    fn write_solid(&mut self, body: &Body, solid_id: SolidId) -> Result<u64, StepError> {
        let solid = body.solids.get(solid_id).ok_or(StepError::Syntax("missing solid".to_string()))?;
        let shell = self.write_shell(body, solid.outer)?;
        let brep = self.next_id();
        self.write_entity(brep, "MANIFOLD_SOLID_BREP", &format!("'', #{shell})"));
        Ok(brep)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn finish(self) -> String {
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

// #region 🔖️Parse

#[derive(Debug)]
struct StepEntity {
    entity_type: String,
    attrs: String,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_step_entities(input: &str) -> Result<HashMap<u64, StepEntity>, StepError> {
    let mut entities = HashMap::new();
    let data_start = input.find("DATA;").ok_or_else(|| StepError::Syntax("no DATA section found".to_string()))?;
    let data_end = input[data_start..].find("ENDSEC;").ok_or_else(|| StepError::Syntax("no ENDSEC after DATA".to_string()))?;
    let data_section = &input[data_start + 5..data_start + data_end];
    let joined = data_section.replace(['\n', '\r'], " ");
    for statement in joined.split(';') {
        let stmt = statement.trim();
        if stmt.is_empty() {
            continue;
        }
        if let Some(eq_pos) = stmt.find('=') {
            let id_part = stmt[..eq_pos].trim();
            let rest = stmt[eq_pos + 1..].trim();
            if let Some(id) = parse_entity_id(id_part) {
                if let Some(paren_pos) = rest.find('(') {
                    let entity_type = rest[..paren_pos].trim().to_uppercase();
                    let attrs = rest[paren_pos + 1..].trim().to_string();
                    entities.insert(id, StepEntity { entity_type, attrs });
                }
            }
        }
    }
    Ok(entities)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_entity_id(s: &str) -> Option<u64> {
    s.trim().strip_prefix('#')?.parse().ok()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_refs(attrs: &str) -> Vec<u64> {
    let mut refs = Vec::new();
    let bytes = attrs.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i > start {
                if let Ok(num) = attrs[start..i].parse::<u64>() {
                    refs.push(num);
                }
            }
        } else {
            i += 1;
        }
    }
    refs
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_list_refs(attrs: &str) -> Vec<u64> {
    if let Some(start) = attrs.find('(') {
        if let Some(end) = attrs[start..].find(')') {
            return parse_refs(&attrs[start + 1..start + end]);
        }
    }
    Vec::new()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_floats(attrs: &str) -> Vec<f64> {
    let mut result = Vec::new();
    if let Some(start) = attrs.find('(') {
        if let Some(end) = attrs[start..].find(')') {
            let inner = &attrs[start + 1..start + end];
            for part in inner.split(',') {
                if let Ok(v) = part.trim().parse::<f64>() {
                    result.push(v);
                }
            }
        }
    }
    if result.is_empty() {
        for part in attrs.split(',') {
            let trimmed = part.trim().trim_matches('\'').trim_end_matches(')');
            if trimmed.starts_with('#') || trimmed.starts_with('.') || trimmed.is_empty() {
                continue;
            }
            if let Ok(v) = trimmed.parse::<f64>() {
                result.push(v);
            }
        }
    }
    result
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_ints_in_parens(s: &str) -> Vec<u32> {
    let mut result = Vec::new();
    for part in s.split(',') {
        let trimmed = part.trim().trim_matches('(').trim_matches(')').trim();
        if let Ok(v) = trimmed.parse::<u32>() {
            result.push(v);
        }
    }
    result
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_knots(mults: &[u32], vals: &[f64]) -> Vec<f64> {
    let mut knots = Vec::new();
    for (&m, &v) in mults.iter().zip(vals.iter()) {
        for _ in 0..m {
            knots.push(v);
        }
    }
    knots
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn find_composite_bspline_attrs<'a>(attrs: &'a str, base_name: &str) -> Option<&'a str> {
    let with_knots = format!("{base_name}_WITH_KNOTS");
    if let Some(pos) = attrs.find(&with_knots) {
        return Some(&attrs[pos + with_knots.len()..]);
    }
    let anchored = format!("{base_name}(");
    if let Some(pos) = attrs.find(&anchored) {
        return Some(&attrs[pos + base_name.len()..]);
    }
    None
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_bspline_curve_attrs(attrs: &str) -> Option<(usize, Vec<u64>, Vec<u32>, Vec<f64>)> {
    let mut tokens = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    let mut groups: Vec<String> = Vec::new();
    for ch in attrs.chars() {
        match ch {
            '(' => {
                if depth == 0 && !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                    current.clear();
                }
                depth += 1;
                current.push(ch);
            }
            ')' => {
                current.push(ch);
                depth -= 1;
                if depth == 0 {
                    groups.push(current.clone());
                    current.clear();
                }
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    tokens.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    let mut degree = None;
    for tok in &tokens {
        if tok.starts_with('\'') || tok.starts_with('.') {
            continue;
        }
        if let Ok(d) = tok.parse::<usize>() {
            degree = Some(d);
            break;
        }
    }
    let degree = degree?;
    if groups.len() < 3 {
        return None;
    }
    Some((degree, parse_refs(&groups[0]), parse_ints_in_parens(&groups[1]), parse_floats(&groups[2])))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_nested_refs(s: &str) -> Vec<Vec<u64>> {
    let mut rows: Vec<Vec<u64>> = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in s.chars() {
        match ch {
            '(' => {
                depth += 1;
                if depth >= 2 {
                    current.push(ch);
                }
            }
            ')' => {
                if depth >= 2 {
                    current.push(ch);
                }
                depth -= 1;
                if depth == 1 && !current.is_empty() {
                    rows.push(parse_refs(&current));
                    current.clear();
                }
            }
            ',' if depth == 1 => {
                if !current.is_empty() {
                    rows.push(parse_refs(&current));
                    current.clear();
                }
            }
            _ => {
                if depth >= 2 {
                    current.push(ch);
                }
            }
        }
    }
    rows
}

#[allow(clippy::type_complexity)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_bspline_surface_attrs(attrs: &str) -> Option<(usize, usize, Vec<Vec<u64>>, Vec<u32>, Vec<u32>, Vec<f64>, Vec<f64>)> {
    let mut tokens = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    let mut groups: Vec<String> = Vec::new();
    for ch in attrs.chars() {
        match ch {
            '(' => {
                if depth == 0 && !current.trim().is_empty() {
                    tokens.push(current.trim().to_string());
                    current.clear();
                }
                depth += 1;
                current.push(ch);
            }
            ')' => {
                current.push(ch);
                depth -= 1;
                if depth == 0 {
                    groups.push(current.clone());
                    current.clear();
                }
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    tokens.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        tokens.push(current.trim().to_string());
    }
    let mut degrees: Vec<usize> = Vec::new();
    for tok in &tokens {
        if tok.starts_with('\'') || tok.starts_with('.') {
            continue;
        }
        if let Ok(d) = tok.parse::<usize>() {
            degrees.push(d);
        }
    }
    if degrees.len() < 2 || groups.len() < 5 {
        return None;
    }
    Some((degrees[0], degrees[1], parse_nested_refs(&groups[0]), parse_ints_in_parens(&groups[1]), parse_ints_in_parens(&groups[2]), parse_floats(&groups[3]), parse_floats(&groups[4])))
}

// #endregion 🔖️Parse

// #region 🔖️Read

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

struct StepBuilder<'a> {
    body: &'a mut Body,
    entities: &'a HashMap<u64, StepEntity>,
    rec: &'a mut OpRecorder,
    vertex_cache: HashMap<u64, VertexId>,
    edge_cache: HashMap<u64, EdgeId>,
    tol: Tol,
}

impl<'a> StepBuilder<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(body: &'a mut Body, entities: &'a HashMap<u64, StepEntity>, rec: &'a mut OpRecorder) -> Self {
        Self { body, entities, rec, vertex_cache: HashMap::new(), edge_cache: HashMap::new(), tol: Tol::DEFAULT }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_all_solids(&mut self) -> Result<(), StepError> {
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
    fn build_shell(&mut self, shell_ref: u64) -> Result<crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::arena::ShellId, StepError> {
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
        Ok(face_id)
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
            "B_SPLINE_CURVE_WITH_KNOTS" => self.build_bspline_curve(curve_ref, &attrs)?,
            _ if entity_type.is_empty() || attrs.contains("B_SPLINE_CURVE_WITH_KNOTS") => {
                let bspline_attrs = find_composite_bspline_attrs(&attrs, "B_SPLINE_CURVE").ok_or_else(|| StepError::Unsupported(format!("composite curve #{curve_ref}")))?;
                self.build_bspline_curve(curve_ref, bspline_attrs)?
            }
            other => return Err(StepError::Unsupported(other.to_string())),
        };
        Ok(self.body.curves3.insert(curve))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_bspline_curve(&self, curve_ref: u64, attrs: &str) -> Result<Curve3, StepError> {
        let (degree, cp_refs, mults, knot_vals) = parse_bspline_curve_attrs(attrs).ok_or_else(|| StepError::Syntax(format!("B_SPLINE_CURVE #{curve_ref} parse failed")))?;
        let mut control_points = Vec::with_capacity(cp_refs.len());
        for &cp_ref in &cp_refs {
            control_points.push(self.build_cartesian_point(cp_ref)?);
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

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn build_axis2_placement(&self, axis_ref: u64) -> Result<(Pnt3, Vec3, Vec3), StepError> {
        let attrs = self.get_entity(axis_ref)?.attrs.clone();
        let refs = parse_refs(&attrs);
        if refs.len() < 3 {
            return Err(StepError::Syntax(format!("AXIS2_PLACEMENT_3D #{axis_ref} needs 3 references")));
        }
        Ok((self.build_cartesian_point(refs[0])?, self.build_direction(refs[1])?, self.build_direction(refs[2])?))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn get_entity(&self, id: u64) -> Result<&StepEntity, StepError> {
        self.entities.get(&id).ok_or(StepError::UnresolvedReference(id))
    }
}

// #endregion 🔖️Read

// #region 🔖️Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::primitives::make_box;

    #[semio_framework_async_macros::async_test]
    async fn box_round_trip_topology_counts() {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let solid = make_box(&mut body, 2.0, 3.0, 4.0, &mut rec).unwrap();
        let step = write_step(&body, &[solid]).unwrap();
        assert!(step.contains("MANIFOLD_SOLID_BREP"));
        assert!(step.contains("ADVANCED_FACE"));
        let read = read_step(&step).unwrap();
        assert_eq!(read.vertices.len(), 8);
        assert_eq!(read.edges.len(), 12);
        assert_eq!(read.faces.len(), 6);
        assert_eq!(read.solids.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn write_step_rejects_empty_solids() {
        let body = Body::new();
        assert!(write_step(&body, &[]).is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn read_step_rejects_missing_data_section() {
        assert!(read_step("ISO-10303-21;").is_err());
    }
}

// #endregion 🔖️Tests
