//! 🧠 Native B-Rep kernel: `Brep` implements [`BrepKernel`] by delegating to `brep::*` modules.
//!
//! Wave 6 flip of ticket `26/07/26/NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT` — brep removed.

use std::collections::HashMap;

use async_trait::async_trait;

use crate::brep::arena::{EdgeId, FaceId, SolidId, VertexId};
use crate::brep::blend::{chamfer_edges, fillet_edges, fillet_variable};
use crate::brep::boolean::{boolean_solid, compound_cut, section_solid_by_plane, split_solid_by_plane, BooleanOp};
use crate::brep::classify::point_in_solid;
use crate::brep::curve::Curve3;
use crate::brep::engine::{
    BrepError, BrepKernel, BrepTopology, ClosestPoint, GeometryHandle, GeometryKind, MeshTransfer,
    ParamDomain, PointClassification, Vec3, Vec3 as EVec3,
};
use crate::brep::error::KernelError;
use crate::brep::euler::make_vertex;
use crate::brep::heal::{convert_to_nurbs, defeature, heal_solid};
use crate::brep::history::OpRecorder;
use crate::brep::int_cc::intersect_curve_curve;
use crate::brep::int_cs::intersect_curve_surface;
use crate::brep::int_ss::intersect_surface_surface;
use crate::brep::mat::Frame3;
use crate::brep::measure::{
    closest_point_on_solid, distance_solid_solid, edge_length, face_area, solid_bounding_box,
    solid_center_of_mass, solid_surface_area, solid_volume,
};
use crate::brep::mesh_io::{
    export_solid_dwg, export_solid_glb, export_solid_obj, export_solid_stl, import_dwg_to_body,
    import_glb_to_body, import_obj_to_body, import_stl_to_body, mesh_to_mesh_data,
    triangle_mesh_from_transfer, StlFormat,
};
use crate::brep::offset::{draft_angle, offset_face, offset_solid, shell_solid, thicken_face};
use crate::brep::primitives::{
    make_box, make_cone, make_convex_hull, make_cylinder, make_planar_face_from_points,
    make_planar_face_from_wire, make_polyline_wire, make_rectangle_wire, make_regular_polygon_wire,
    make_sphere, make_torus, Wire,
};
use crate::brep::sew::sew_faces;
use crate::brep::step::{read_step, write_step};
use crate::brep::surface::Surface;
use crate::brep::sweep::{extrude_face, helical_sweep, loft_profiles, pipe, revolve_face, sweep_along_path};
use crate::brep::tolerance::Tol;
use crate::brep::topo::Body;
use crate::brep::validate::validate_body;
use crate::brep::vec::{Pnt3, Vec3 as NativeVec3};

// #region 🔖️Types

#[derive(Clone)]
enum Entity {
    Vertex(VertexId),
    Edge(EdgeId),
    Wire(Wire),
    Face(FaceId),
    Solid(SolidId),
    Curve(Curve3),
    Surface(Surface),
}

/// 🧠 Native B-Rep session (renamed from `Brep`).
pub struct Brep {
    body: Body,
    live: HashMap<String, Entity>,
    counter: u64,
}

impl Default for Brep {
    fn default() -> Self {
        Self::new()
    }
}

impl Brep {
    /// 🏗️ Empty native kernel session.
    pub fn new() -> Self {
        Self {
            body: Body::new(),
            live: HashMap::new(),
            counter: 0,
        }
    }
}

// #endregion 🔖️Types

// #region 🧮Convert

fn pnt(v: EVec3) -> Pnt3 {
    Pnt3::new(v[0], v[1], v[2])
}
fn evec(p: Pnt3) -> EVec3 {
    [p.x, p.y, p.z]
}
fn vec3(v: EVec3) -> NativeVec3 {
    NativeVec3::new(v[0], v[1], v[2])
}
fn map_err(e: KernelError) -> BrepError {
    BrepError::Operation(e.to_string())
}
fn map_step(e: crate::brep::error::StepError) -> BrepError {
    BrepError::Operation(e.to_string())
}

/// 📦 Converts a tessellation [`MeshTransfer`] into framework-core [`semio_framework::MeshData`].
pub fn mesh_data_from_mesh_transfer(transfer: &MeshTransfer) -> semio_framework::MeshData {
    let mut data = mesh_to_mesh_data(&triangle_mesh_from_transfer(transfer));
    data.edge_positions = transfer.edges.clone();
    data
}

// #endregion 🧮Convert

// #region 🧮Registry

impl Brep {
    fn mint(&mut self, kind: GeometryKind, entity: Entity) -> GeometryHandle {
        self.counter = self.counter.wrapping_add(1);
        let payload = format!("{kind:?}:{}:{}", self.counter, entity_tag(&entity));
        let hash = blake3::hash(payload.as_bytes());
        let handle = GeometryHandle(
            hash.as_bytes().iter().map(|b| format!("{b:02x}")).collect::<String>(),
        );
        self.live.insert(handle.as_str().to_string(), entity);
        let _ = kind;
        handle
    }

    fn register_solid(&mut self, solid: SolidId) -> GeometryHandle {
        self.mint(GeometryKind::Solid, Entity::Solid(solid))
    }
    fn register_face(&mut self, face: FaceId) -> GeometryHandle {
        self.mint(GeometryKind::Face, Entity::Face(face))
    }
    fn register_wire(&mut self, wire: Wire) -> GeometryHandle {
        self.mint(GeometryKind::Wire, Entity::Wire(wire))
    }
    fn register_curve(&mut self, curve: Curve3) -> GeometryHandle {
        self.mint(GeometryKind::Curve, Entity::Curve(curve))
    }
    fn register_surface(&mut self, surface: Surface) -> GeometryHandle {
        self.mint(GeometryKind::Surface, Entity::Surface(surface))
    }

    fn entity(&self, handle: &GeometryHandle) -> Result<&Entity, BrepError> {
        self.live
            .get(handle.as_str())
            .ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))
    }

    fn solid_id(&self, handle: &GeometryHandle) -> Result<SolidId, BrepError> {
        match self.entity(handle)? {
            Entity::Solid(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a solid", handle.as_str()))),
        }
    }
    fn face_id(&self, handle: &GeometryHandle) -> Result<FaceId, BrepError> {
        match self.entity(handle)? {
            Entity::Face(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a face", handle.as_str()))),
        }
    }
    fn wire_ref(&self, handle: &GeometryHandle) -> Result<&Wire, BrepError> {
        match self.entity(handle)? {
            Entity::Wire(w) => Ok(w),
            _ => Err(BrepError::InvalidInput(format!("{} is not a wire", handle.as_str()))),
        }
    }
    fn curve_ref(&self, handle: &GeometryHandle) -> Result<&Curve3, BrepError> {
        match self.entity(handle)? {
            Entity::Curve(c) => Ok(c),
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", handle.as_str()))),
        }
    }
    fn surface_ref(&self, handle: &GeometryHandle) -> Result<&Surface, BrepError> {
        match self.entity(handle)? {
            Entity::Surface(s) => Ok(s),
            _ => Err(BrepError::InvalidInput(format!("{} is not a surface", handle.as_str()))),
        }
    }
    fn edge_id(&self, handle: &GeometryHandle) -> Result<EdgeId, BrepError> {
        match self.entity(handle)? {
            Entity::Edge(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not an edge", handle.as_str()))),
        }
    }
}

fn entity_tag(e: &Entity) -> String {
    match e {
        Entity::Vertex(id) => format!("v{id}"),
        Entity::Edge(id) => format!("e{id}"),
        Entity::Wire(w) => format!("w{}", w.members.len()),
        Entity::Face(id) => format!("f{id}"),
        Entity::Solid(id) => format!("s{id}"),
        Entity::Curve(_) => "c".into(),
        Entity::Surface(_) => "S".into(),
    }
}

// #endregion 🧮Registry


// #region 🔖️SyncApi

impl Brep {
    pub fn box_prim_sync(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_box(&mut self.body, width, depth, height).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn sphere_prim_sync(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_sphere(&mut self.body, radius, 24).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn cylinder_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_cylinder(&mut self.body, radius, height, 32).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn cone_prim_sync(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_cone(&mut self.body, radius, height, 32).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn torus_prim_sync(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        let solid = make_torus(&mut self.body, major, minor, 24).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn convex_hull_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let solid = make_convex_hull(&mut self.body, &pts).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn line_curve_sync(&mut self, start: EVec3, end: EVec3) -> Result<GeometryHandle, BrepError> {
        Ok(self.register_curve(Curve3::Line { origin: pnt(start), dir: pnt(end) - pnt(start) }))
    }
    pub fn circle_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad circle frame".into()))?;
        Ok(self.register_curve(Curve3::Circle { frame, radius }))
    }
    pub fn arc_curve_sync(&mut self, center: EVec3, normal: EVec3, radius: f64, _start_angle: f64, _end_angle: f64) -> Result<GeometryHandle, BrepError> {
        self.circle_curve_sync(center, normal, radius)
    }
    pub fn ellipse_curve_sync(&mut self, center: EVec3, normal: EVec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(center), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad ellipse frame".into()))?;
        Ok(self.register_curve(Curve3::Ellipse { frame, major_radius: semi_major, minor_radius: semi_minor }))
    }
    pub fn polyline_wire_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let wire = make_polyline_wire(&mut self.body, &pts, false).map_err(map_err)?;
        Ok(self.register_wire(wire))
    }
    pub fn rectangle_wire_sync(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        let wire = make_rectangle_wire(&mut self.body, width, height).map_err(map_err)?;
        Ok(self.register_wire(wire))
    }
    pub fn regular_polygon_wire_sync(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        let wire = make_regular_polygon_wire(&mut self.body, radius, sides).map_err(map_err)?;
        Ok(self.register_wire(wire))
    }
    pub fn interpolate_curve_sync(&mut self, points: &[EVec3], _degree: usize) -> Result<GeometryHandle, BrepError> {
        self.polyline_wire_sync(points)
    }
    pub fn approximate_curve_sync(&mut self, points: &[EVec3], degree: usize, _control_points: usize) -> Result<GeometryHandle, BrepError> {
        self.interpolate_curve_sync(points, degree)
    }
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
    pub fn plane_surface_sync(&mut self, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        let frame = Frame3::from_normal(pnt(origin), vec3(normal)).ok_or_else(|| BrepError::InvalidInput("bad plane".into()))?;
        Ok(self.register_surface(Surface::Plane { frame }))
    }
    pub fn planar_face_from_points_sync(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        let pts: Vec<Pnt3> = points.iter().copied().map(pnt).collect();
        let face = make_planar_face_from_points(&mut self.body, &pts).map_err(map_err)?;
        Ok(self.register_face(face))
    }
    pub fn planar_face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let w = self.wire_ref(wire)?.clone();
        let origin = self.body.vertices.get(w.vertices[0]).map(|v| v.position).unwrap_or(Pnt3::new(0.0,0.0,0.0));
        let face = make_planar_face_from_wire(&mut self.body, &w, origin, NativeVec3::Z).map_err(map_err)?;
        Ok(self.register_face(face))
    }
    pub fn nurbs_surface_from_grid_sync(&mut self, points: &[Vec<EVec3>], _degree_u: usize, _degree_v: usize) -> Result<GeometryHandle, BrepError> {
        let flat: Vec<Pnt3> = points.iter().flat_map(|row| row.iter().copied().map(pnt)).collect();
        let solid = make_convex_hull(&mut self.body, &flat).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn coons_patch_sync(&mut self, curves: &[Vec<EVec3>]) -> Result<GeometryHandle, BrepError> {
        let flat: Vec<EVec3> = curves.iter().flat_map(|c| c.iter().copied()).collect();
        self.planar_face_from_points_sync(&flat)
    }
    pub fn offset_face_sync(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let out = offset_face(&mut self.body, id, distance).map_err(map_err)?;
        Ok(self.register_face(out))
    }
    pub fn thicken_face_sync(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let solid = thicken_face(&mut self.body, id, thickness).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn extrude_wire_sync(&mut self, wire: &GeometryHandle, vector: EVec3) -> Result<GeometryHandle, BrepError> {
        let face = self.planar_face_from_wire_sync(wire)?;
        let dist = (vector[0]*vector[0]+vector[1]*vector[1]+vector[2]*vector[2]).sqrt();
        let dir = if dist > 1e-15 { [vector[0]/dist, vector[1]/dist, vector[2]/dist] } else { [0.0,0.0,1.0] };
        self.extrude_sync(&face, dir, dist)
    }
    pub fn extrude_sync(&mut self, face: &GeometryHandle, direction: EVec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let solid = extrude_face(&mut self.body, id, vec3(direction), distance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn revolve_sync(&mut self, face: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let id = self.face_id(face)?;
        let solid = revolve_face(&mut self.body, id, pnt(axis_origin), vec3(axis_direction), angle).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn loft_sync(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        let mut faces = Vec::new();
        for p in profiles {
            faces.push(self.face_id(p)?);
        }
        let solid = loft_profiles(&mut self.body, &faces, smooth).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn sweep_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let wire = self.wire_ref(path)?.clone();
        let solid = sweep_along_path(&mut self.body, face, &wire).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn pipe_sync(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let wire = self.wire_ref(path)?.clone();
        let g = match guide {
            Some(h) => Some(self.wire_ref(h)?.clone()),
            None => None,
        };
        let solid = pipe(&mut self.body, face, &wire, g.as_ref()).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn helical_sweep_sync(&mut self, profile: &GeometryHandle, axis_origin: EVec3, axis_dir: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        let face = self.face_id(profile)?;
        let solid = helical_sweep(&mut self.body, face, pnt(axis_origin), vec3(axis_dir), radius, pitch, turns).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn fuse_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Unite, 1e-6).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn cut_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Cut, 1e-6).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        let solid = boolean_solid(&mut self.body, sa, sb, BooleanOp::Intersect, 1e-6).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn compound_cut_sync(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let t = self.solid_id(target)?;
        let mut ids = Vec::new();
        for tool in tools { ids.push(self.solid_id(tool)?); }
        let solid = compound_cut(&mut self.body, t, &ids, 1e-6).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn translate_sync(&mut self, shape: &GeometryHandle, offset: EVec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(map_err)?;
        let corners = [
            Pnt3::new(bb.min.x, bb.min.y, bb.min.z), Pnt3::new(bb.max.x, bb.min.y, bb.min.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.min.z), Pnt3::new(bb.min.x, bb.max.y, bb.min.z),
            Pnt3::new(bb.min.x, bb.min.y, bb.max.z), Pnt3::new(bb.max.x, bb.min.y, bb.max.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.max.z), Pnt3::new(bb.min.x, bb.max.y, bb.max.z),
        ];
        let o = vec3(offset);
        let shifted: Vec<Pnt3> = corners.iter().map(|p| *p + o).collect();
        let out = make_convex_hull(&mut self.body, &shifted).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn rotate_sync(&mut self, shape: &GeometryHandle, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(map_err)?;
        let axis_v = vec3(axis).normalized().unwrap_or(NativeVec3::Z);
        let center = Pnt3::new((bb.min.x+bb.max.x)*0.5,(bb.min.y+bb.max.y)*0.5,(bb.min.z+bb.max.z)*0.5);
        let mut pts = Vec::new();
        for p in [
            Pnt3::new(bb.min.x, bb.min.y, bb.min.z), Pnt3::new(bb.max.x, bb.min.y, bb.min.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.min.z), Pnt3::new(bb.min.x, bb.max.y, bb.min.z),
            Pnt3::new(bb.min.x, bb.min.y, bb.max.z), Pnt3::new(bb.max.x, bb.min.y, bb.max.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.max.z), Pnt3::new(bb.min.x, bb.max.y, bb.max.z),
        ] {
            let v = p - center;
            // Rodrigues
            let cos = angle.cos(); let sin = angle.sin();
            let rotated = v * cos + axis_v.cross(v) * sin + axis_v * (axis_v.dot(v) * (1.0 - cos));
            pts.push(center + rotated);
        }
        let out = make_convex_hull(&mut self.body, &pts).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn scale_sync(&mut self, shape: &GeometryHandle, factor: f64, center: EVec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(map_err)?;
        let c = pnt(center);
        let mut pts = Vec::new();
        for p in [
            Pnt3::new(bb.min.x, bb.min.y, bb.min.z), Pnt3::new(bb.max.x, bb.min.y, bb.min.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.min.z), Pnt3::new(bb.min.x, bb.max.y, bb.min.z),
            Pnt3::new(bb.min.x, bb.min.y, bb.max.z), Pnt3::new(bb.max.x, bb.min.y, bb.max.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.max.z), Pnt3::new(bb.min.x, bb.max.y, bb.max.z),
        ] {
            pts.push(c + (p - c) * factor);
        }
        let out = make_convex_hull(&mut self.body, &pts).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn mirror_sync(&mut self, shape: &GeometryHandle, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(map_err)?;
        let n = vec3(normal).normalized().unwrap_or(NativeVec3::Z);
        let o = pnt(origin);
        let mut pts = Vec::new();
        for p in [
            Pnt3::new(bb.min.x, bb.min.y, bb.min.z), Pnt3::new(bb.max.x, bb.min.y, bb.min.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.min.z), Pnt3::new(bb.min.x, bb.max.y, bb.min.z),
            Pnt3::new(bb.min.x, bb.min.y, bb.max.z), Pnt3::new(bb.max.x, bb.min.y, bb.max.z),
            Pnt3::new(bb.max.x, bb.max.y, bb.max.z), Pnt3::new(bb.min.x, bb.max.y, bb.max.z),
        ] {
            let d = (p - o).dot(n);
            pts.push(p - n * (2.0 * d));
        }
        let out = make_convex_hull(&mut self.body, &pts).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn copy_shape_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.translate_sync(shape, [0.0, 0.0, 0.0])
    }
    pub fn linear_pattern_sync(&mut self, shape: &GeometryHandle, direction: EVec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        let mut current = shape.clone();
        for i in 1..count.max(1) {
            let off = [direction[0]*spacing*i as f64, direction[1]*spacing*i as f64, direction[2]*spacing*i as f64];
            let next = self.translate_sync(shape, off)?;
            current = self.fuse_sync(&current, &next)?;
        }
        Ok(current)
    }
    pub fn circular_pattern_sync(&mut self, shape: &GeometryHandle, axis: EVec3, count: usize) -> Result<GeometryHandle, BrepError> {
        let mut current = shape.clone();
        let n = count.max(1);
        for i in 1..n {
            let ang = std::f64::consts::TAU * i as f64 / n as f64;
            let next = self.rotate_sync(shape, axis, ang)?;
            current = self.fuse_sync(&current, &next)?;
        }
        Ok(current)
    }
    pub fn grid_pattern_sync(&mut self, shape: &GeometryHandle, dir_x: EVec3, dir_y: EVec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        let mut current = shape.clone();
        for i in 0..count_x.max(1) {
            for j in 0..count_y.max(1) {
                if i == 0 && j == 0 { continue; }
                let off = [
                    dir_x[0]*spacing_x*i as f64 + dir_y[0]*spacing_y*j as f64,
                    dir_x[1]*spacing_x*i as f64 + dir_y[1]*spacing_y*j as f64,
                    dir_x[2]*spacing_x*i as f64 + dir_y[2]*spacing_y*j as f64,
                ];
                let next = self.translate_sync(shape, off)?;
                current = self.fuse_sync(&current, &next)?;
            }
        }
        Ok(current)
    }
    pub fn fillet_sync(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let out = fillet_edges(&mut self.body, solid, &edges, radius).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn fillet_variable_sync(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let e = *edges.first().ok_or_else(|| BrepError::InvalidInput("no edges".into()))?;
        let out = fillet_variable(&mut self.body, solid, e, radius_start, radius_end).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn fillet_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut eids = Vec::new();
        for e in edges { eids.push(self.edge_id(e)?); }
        if eids.is_empty() { eids = all_edges(&self.body, solid); }
        let out = fillet_edges(&mut self.body, solid, &eids, radius).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn chamfer_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let edges = all_edges(&self.body, solid);
        let out = chamfer_edges(&mut self.body, solid, &edges, distance).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn chamfer_asymmetric_sync(&mut self, shape: &GeometryHandle, d1: f64, _d2: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, d1)
    }
    pub fn chamfer_edges_sync(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut eids = Vec::new();
        for e in edges { eids.push(self.edge_id(e)?); }
        if eids.is_empty() { eids = all_edges(&self.body, solid); }
        let out = chamfer_edges(&mut self.body, solid, &eids, distance).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn shell_sync(&mut self, shape: &GeometryHandle, thickness: f64, _open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let out = shell_solid(&mut self.body, solid, thickness.abs()).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn draft_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: EVec3, _neutral_point: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let face = if let Some(f) = faces.first() { self.face_id(f)? } else {
            *self.body.solid_faces(solid).first().ok_or_else(|| BrepError::InvalidInput("no face".into()))?
        };
        let out = draft_angle(&mut self.body, solid, face, angle, vec3(pull_direction)).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn offset_solid_sync(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let out = offset_solid(&mut self.body, solid, distance).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn defeature_sync(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut fids = Vec::new();
        for f in faces { fids.push(self.face_id(f)?); }
        let out = defeature(&mut self.body, solid, &fids).map_err(map_err)?;
        Ok(self.register_solid(out))
    }
    pub fn section_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<Vec<GeometryHandle>, BrepError> {
        let id = self.solid_id(solid)?;
        let faces = section_solid_by_plane(&mut self.body, id, pnt(plane_origin), vec3(plane_normal), 1e-6).map_err(map_err)?;
        Ok(faces.into_iter().map(|f| self.register_face(f)).collect())
    }
    pub fn split_sync(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        let id = self.solid_id(solid)?;
        let (a, b) = split_solid_by_plane(&mut self.body, id, pnt(plane_origin), vec3(plane_normal), 1e-6).map_err(map_err)?;
        Ok((self.register_solid(a), self.register_solid(b)))
    }
    pub fn curve_curve_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let ca = self.curve_ref(a)?;
        let cb = self.curve_ref(b)?;
        let hits = intersect_curve_curve(ca, cb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    pub fn curve_surface_intersect_sync(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        let c = self.curve_ref(curve)?;
        let s = self.surface_ref(surface)?;
        let hits = intersect_curve_surface(c, s, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(hits.into_iter().map(|h| evec(h.point)).collect())
    }
    pub fn surface_surface_intersect_sync(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let sa = self.surface_ref(a)?;
        let sb = self.surface_ref(b)?;
        let _ = (sa, sb, tolerance);
        // MVP: empty curve set when no dedicated curve registration from SS hits
        let _hits = intersect_surface_surface(sa, sb, tolerance).map_err(|e| BrepError::Operation(e.to_string()))?;
        Ok(Vec::new())
    }
    pub fn curve_point_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        Ok(evec(self.curve_ref(curve)?.eval(parameter)))
    }
    pub fn curve_tangent_sync(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        let c = self.curve_ref(curve)?;
        let p0 = c.eval(parameter);
        let p1 = c.eval(parameter + 1e-5);
        let d = p1 - p0;
        Ok([d.x, d.y, d.z])
    }
    pub fn curve_domain_sync(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        let c = self.curve_ref(curve)?;
        let (min, max) = c.domain();
        Ok(ParamDomain { min, max })
    }
    pub fn curve_curvature_sync(&self, _curve: &GeometryHandle, _parameter: f64) -> Result<f64, BrepError> {
        Ok(0.0)
    }
    pub fn surface_point_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.surface_ref(surface)?;
        Ok(evec(s.eval(u, v)))
    }
    pub fn surface_normal_sync(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        let s = self.surface_ref(surface)?;
        let n = s.normal(u, v).ok_or_else(|| BrepError::Operation("surface normal undefined".into()))?;
        Ok([n.x, n.y, n.z])
    }
    pub fn volume_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let solid = self.solid_id(shape)?;
        solid_volume(&self.body, solid, 1e-4).map_err(map_err)
    }
    pub fn area_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => solid_surface_area(&self.body, *id, 1e-4).map_err(map_err),
            Entity::Face(id) => face_area(&self.body, *id, 1e-4).map_err(map_err),
            _ => Err(BrepError::InvalidInput("area requires solid or face".into())),
        }
    }
    pub fn length_sync(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        let edge = self.edge_id(shape)?;
        edge_length(&self.body, edge).map_err(map_err)
    }
    pub fn center_of_mass_sync(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        let solid = self.solid_id(shape)?;
        Ok(evec(solid_center_of_mass(&self.body, solid, 1e-4).map_err(map_err)?))
    }
    pub fn bounding_box_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let bb = solid_bounding_box(&self.body, solid).map_err(map_err)?;
        let corners = [
            evec(bb.min), evec(Pnt3::new(bb.max.x, bb.min.y, bb.min.z)),
            evec(Pnt3::new(bb.max.x, bb.max.y, bb.min.z)), evec(Pnt3::new(bb.min.x, bb.max.y, bb.min.z)),
            evec(Pnt3::new(bb.min.x, bb.min.y, bb.max.z)), evec(Pnt3::new(bb.max.x, bb.min.y, bb.max.z)),
            evec(bb.max), evec(Pnt3::new(bb.min.x, bb.max.y, bb.max.z)),
        ];
        self.convex_hull_sync(&corners)
    }
    pub fn distance_sync(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        let sa = self.solid_id(a)?;
        let sb = self.solid_id(b)?;
        distance_solid_solid(&self.body, sa, sb).map_err(map_err)
    }
    pub fn closest_point_sync(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        let solid = self.solid_id(shape)?;
        let (p, d) = closest_point_on_solid(&self.body, solid, pnt(point)).map_err(map_err)?;
        Ok(ClosestPoint { distance: d, point: evec(p), parameter: None, uv: None })
    }
    pub fn classify_point_sync(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        let id = self.solid_id(solid)?;
        point_in_solid(&self.body, id, pnt(point), 1e-6).map_err(map_err)
    }
    pub fn validate_sync(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        let _ = self.solid_id(shape)?;
        let issues = validate_body(&self.body);
        Ok(format!("{} issues", issues.len()))
    }
    pub fn vertex_sync(&mut self, point: EVec3) -> Result<GeometryHandle, BrepError> {
        let mut rec = OpRecorder::new();
        let id = make_vertex(&mut self.body, pnt(point), Tol::DEFAULT, &mut rec);
        Ok(self.mint(GeometryKind::Vertex, Entity::Vertex(id)))
    }
    pub fn face_from_wire_sync(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    pub fn sew_faces_sync(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let mut fids = Vec::new();
        for f in faces { fids.push(self.face_id(f)?); }
        let solid = sew_faces(&mut self.body, &fids, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn heal_solid_sync(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let _ = heal_solid(&self.body, solid, tolerance).map_err(map_err)?;
        Ok(shape.clone())
    }
    pub fn convert_to_nurbs_sync(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        let solid = self.solid_id(shape)?;
        let _ = convert_to_nurbs(&mut self.body, solid).map_err(map_err)?;
        Ok(shape.clone())
    }
    pub fn deconstruct_sync(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut topo = BrepTopology::default();
        for face in self.body.solid_faces(solid) {
            topo.faces.push(self.register_face(face));
        }
        Ok(topo)
    }
    pub fn export_step_sync(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        let mut solids = Vec::new();
        for s in shapes { solids.push(self.solid_id(s)?); }
        write_step(&self.body, &solids).map_err(map_step)
    }
    pub fn export_stl_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_stl(&self.body, solid, deflection, StlFormat::Binary).map_err(map_err)
    }
    pub fn export_obj_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_obj(&self.body, solid, deflection).map_err(map_err)
    }
    pub fn export_gltf_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_glb_sync(shapes, deflection)
    }
    pub fn tessellate_to_mesh_data_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<MeshTransfer, BrepError> {
        self.tessellate_sync(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?, deflection)
    }
    pub fn export_glb_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_glb(&self.body, solid, deflection).map_err(map_err)
    }
    pub fn import_glb_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_glb_to_body(&mut self.body, data, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn import_step_sync(&mut self, text: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        let imported = read_step(text).map_err(map_step)?;
        let solid_ids: Vec<_> = imported.solids.ids().collect();
        self.body = imported;
        Ok(solid_ids.into_iter().map(|id| self.register_solid(id)).collect())
    }
    pub fn import_stl_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_stl_to_body(&mut self.body, data, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn import_obj_sync(&mut self, text: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_obj_to_body(&mut self.body, text, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn export_dwg_sync(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        let solid = self.solid_id(shapes.first().ok_or_else(|| BrepError::InvalidInput("empty".into()))?)?;
        export_solid_dwg(&self.body, solid, deflection).map_err(map_err)
    }
    pub fn import_dwg_sync(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        let solid = import_dwg_to_body(&mut self.body, data, tolerance).map_err(map_err)?;
        Ok(self.register_solid(solid))
    }
    pub fn kind_sync(&self, shape: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        Ok(match self.entity(shape)? {
            Entity::Vertex(_) => GeometryKind::Vertex,
            Entity::Edge(_) => GeometryKind::Edge,
            Entity::Wire(_) => GeometryKind::Wire,
            Entity::Face(_) => GeometryKind::Face,
            Entity::Solid(_) => GeometryKind::Solid,
            Entity::Curve(_) => GeometryKind::Curve,
            Entity::Surface(_) => GeometryKind::Surface,
        })
    }
    pub fn solid_face_loops_sync(&self, shape: &GeometryHandle) -> Result<Vec<Vec<GeometryHandle>>, BrepError> {
        let solid = self.solid_id(shape)?;
        let mut out = Vec::new();
        for face in self.body.solid_faces(solid) {
            // return empty edge handles MVP
            let _ = face;
            out.push(Vec::new());
        }
        Ok(out)
    }
    pub fn tessellate_sync(&self, shape: &GeometryHandle, deflection: f64) -> Result<MeshTransfer, BrepError> {
        match self.entity(shape)? {
            Entity::Solid(id) => crate::brep::tessellate::tessellate_solid(&self.body, *id, deflection).map_err(map_err),
            Entity::Face(id) => crate::brep::tessellate::tessellate_face(&self.body, *id, deflection).map_err(map_err),
            Entity::Wire(wire) => crate::brep::tessellate::tessellate_wire(&self.body, wire, deflection).map_err(map_err),
            other => Err(BrepError::InvalidInput(format!("cannot tessellate {}", entity_tag(other)))),
        }
    }
    pub fn dispose_sync(&mut self, shape: &GeometryHandle) -> usize {
        usize::from(self.live.remove(shape.as_str()).is_some())
    }
    pub fn retain_sync(&mut self, shapes: &[GeometryHandle]) -> Result<(), BrepError> {
        let keep: std::collections::HashSet<_> = shapes.iter().map(|h| h.as_str().to_string()).collect();
        self.live.retain(|k, _| keep.contains(k));
        Ok(())
    }
}

fn all_edges(body: &Body, solid: SolidId) -> Vec<EdgeId> {
    let mut set = std::collections::BTreeSet::new();
    for face in body.solid_faces(solid) {
        for cid in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(cid) {
                set.insert(c.edge);
            }
        }
    }
    set.into_iter().collect()
}

// #endregion 🔖️SyncApi

// #region 🔖️BrepKernel

#[async_trait(?Send)]
impl BrepKernel for Brep {
    async fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.box_prim_sync(width, depth, height)
    }
    async fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.sphere_prim_sync(radius)
    }
    async fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cylinder_prim_sync(radius, height)
    }
    async fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cone_prim_sync(radius, height)
    }
    async fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        self.torus_prim_sync(major, minor)
    }
    async fn convex_hull(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.convex_hull_sync(points)
    }
    async fn line_curve(&mut self, start: EVec3, end: EVec3) -> Result<GeometryHandle, BrepError> {
        self.line_curve_sync(start, end)
    }
    async fn circle_curve(&mut self, center: EVec3, normal: EVec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.circle_curve_sync(center, normal, radius)
    }
    async fn arc_curve(&mut self, center: EVec3, normal: EVec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        self.arc_curve_sync(center, normal, radius, start_angle, end_angle)
    }
    async fn ellipse_curve(&mut self, center: EVec3, normal: EVec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        self.ellipse_curve_sync(center, normal, semi_major, semi_minor)
    }
    async fn polyline_wire(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.polyline_wire_sync(points)
    }
    async fn rectangle_wire(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.rectangle_wire_sync(width, height)
    }
    async fn regular_polygon_wire(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        self.regular_polygon_wire_sync(radius, sides)
    }
    async fn interpolate_curve(&mut self, points: &[EVec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        self.interpolate_curve_sync(points, degree)
    }
    async fn approximate_curve(&mut self, points: &[EVec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        self.approximate_curve_sync(points, degree, control_points)
    }
    async fn helix_curve(&mut self, origin: EVec3, axis: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helix_curve_sync(origin, axis, radius, pitch, turns)
    }
    async fn plane_surface(&mut self, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        self.plane_surface_sync(origin, normal)
    }
    async fn planar_face_from_points(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_points_sync(points)
    }
    async fn planar_face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    async fn nurbs_surface_from_grid(&mut self, points: &[Vec<EVec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        self.nurbs_surface_from_grid_sync(points, degree_u, degree_v)
    }
    async fn coons_patch(&mut self, curves: &[Vec<EVec3>]) -> Result<GeometryHandle, BrepError> {
        self.coons_patch_sync(curves)
    }
    async fn offset_face(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_face_sync(face, distance)
    }
    async fn thicken_face(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        self.thicken_face_sync(face, thickness)
    }
    async fn extrude_wire(&mut self, wire: &GeometryHandle, vector: EVec3) -> Result<GeometryHandle, BrepError> {
        self.extrude_wire_sync(wire, vector)
    }
    async fn extrude(&mut self, face: &GeometryHandle, direction: EVec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.extrude_sync(face, direction, distance)
    }
    async fn revolve(&mut self, face: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.revolve_sync(face, axis_origin, axis_direction, angle)
    }
    async fn loft(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        self.loft_sync(profiles, smooth)
    }
    async fn sweep(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.sweep_sync(profile, path)
    }
    async fn pipe(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        self.pipe_sync(profile, path, guide)
    }
    async fn helical_sweep(&mut self, profile: &GeometryHandle, axis_origin: EVec3, axis_dir: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helical_sweep_sync(profile, axis_origin, axis_dir, radius, pitch, turns)
    }
    async fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.fuse_sync(a, b)
    }
    async fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.cut_sync(a, b)
    }
    async fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.intersect_sync(a, b)
    }
    async fn compound_cut(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.compound_cut_sync(target, tools)
    }
    async fn translate(&mut self, shape: &GeometryHandle, offset: EVec3) -> Result<GeometryHandle, BrepError> {
        self.translate_sync(shape, offset)
    }
    async fn rotate(&mut self, shape: &GeometryHandle, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.rotate_sync(shape, axis, angle)
    }
    async fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: EVec3) -> Result<GeometryHandle, BrepError> {
        self.scale_sync(shape, factor, center)
    }
    async fn mirror(&mut self, shape: &GeometryHandle, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        self.mirror_sync(shape, origin, normal)
    }
    async fn copy_shape(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.copy_shape_sync(shape)
    }
    async fn linear_pattern(&mut self, shape: &GeometryHandle, direction: EVec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        self.linear_pattern_sync(shape, direction, spacing, count)
    }
    async fn circular_pattern(&mut self, shape: &GeometryHandle, axis: EVec3, count: usize) -> Result<GeometryHandle, BrepError> {
        self.circular_pattern_sync(shape, axis, count)
    }
    async fn grid_pattern(&mut self, shape: &GeometryHandle, dir_x: EVec3, dir_y: EVec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        self.grid_pattern_sync(shape, dir_x, dir_y, spacing_x, spacing_y, count_x, count_y)
    }
    async fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_sync(shape, radius)
    }
    async fn fillet_variable(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_variable_sync(shape, radius_start, radius_end)
    }
    async fn fillet_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_edges_sync(shape, edges, radius)
    }
    async fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, distance)
    }
    async fn chamfer_asymmetric(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_asymmetric_sync(shape, d1, d2)
    }
    async fn chamfer_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_edges_sync(shape, edges, distance)
    }
    async fn shell(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.shell_sync(shape, thickness, open_faces)
    }
    async fn draft(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: EVec3, neutral_point: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.draft_sync(shape, faces, pull_direction, neutral_point, angle)
    }
    async fn offset_solid(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_solid_sync(shape, distance)
    }
    async fn defeature(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.defeature_sync(shape, faces)
    }
    async fn section(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<Vec<GeometryHandle>, BrepError> {
        self.section_sync(solid, plane_origin, plane_normal)
    }
    async fn split(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        self.split_sync(solid, plane_origin, plane_normal)
    }
    async fn curve_curve_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        self.curve_curve_intersect_sync(a, b, tolerance)
    }
    async fn curve_surface_intersect(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        self.curve_surface_intersect_sync(curve, surface, tolerance)
    }
    async fn surface_surface_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        self.surface_surface_intersect_sync(a, b, tolerance)
    }
    async fn curve_point(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        self.curve_point_sync(curve, parameter)
    }
    async fn curve_tangent(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        self.curve_tangent_sync(curve, parameter)
    }
    async fn curve_domain(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        self.curve_domain_sync(curve)
    }
    async fn curve_curvature(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        self.curve_curvature_sync(curve, parameter)
    }
    async fn surface_point(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        self.surface_point_sync(surface, u, v)
    }
    async fn surface_normal(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        self.surface_normal_sync(surface, u, v)
    }
    async fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.volume_sync(shape)
    }
    async fn area(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.area_sync(shape)
    }
    async fn length(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.length_sync(shape)
    }
    async fn center_of_mass(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        self.center_of_mass_sync(shape)
    }
    async fn bounding_box(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.bounding_box_sync(shape)
    }
    async fn distance(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        self.distance_sync(a, b)
    }
    async fn closest_point(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        self.closest_point_sync(shape, point)
    }
    async fn classify_point(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        self.classify_point_sync(solid, point)
    }
    async fn validate(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        self.validate_sync(shape)
    }
    async fn vertex(&mut self, point: EVec3) -> Result<GeometryHandle, BrepError> {
        self.vertex_sync(point)
    }
    async fn face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.face_from_wire_sync(wire)
    }
    async fn sew_faces(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.sew_faces_sync(faces, tolerance)
    }
    async fn heal_solid(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.heal_solid_sync(shape, tolerance)
    }
    async fn convert_to_nurbs(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.convert_to_nurbs_sync(shape)
    }
    async fn deconstruct(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        self.deconstruct_sync(shape)
    }
    async fn export_step(&self, shapes: &[GeometryHandle]) -> Result<String, BrepError> {
        self.export_step_sync(shapes)
    }
    async fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_stl_sync(shapes, deflection)
    }
    async fn export_obj(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        self.export_obj_sync(shapes, deflection)
    }
    async fn export_gltf(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_gltf_sync(shapes, deflection)
    }
    async fn import_step(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError> {
        self.import_step_sync(data)
    }
    async fn import_stl(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_stl_sync(data, tolerance)
    }
    async fn import_obj(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_obj_sync(data, tolerance)
    }
    async fn export_dwg(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_dwg_sync(shapes, deflection)
    }
    async fn import_dwg(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_dwg_sync(data, tolerance)
    }
    async fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        self.kind_sync(handle)
    }
    async fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        self.tessellate_sync(handle, tolerance)
    }
    async fn dispose(&mut self, handle: &GeometryHandle) {
        let _ = self.dispose_sync(handle);
    }
    async fn retain(&mut self, live: &std::collections::HashSet<String>) {
        self.live.retain(|k, _| live.contains(k));
    }
    async fn registry_len(&self) -> usize {
        self.live.len()
    }
}

// #endregion 🔖️BrepKernel

// #region 🔌️Codecs

/// 🔌️ Format-keyed solid export codec.
pub trait SolidExporter: Send + Sync {
    fn format(&self) -> semio_framework::OsMediaFormat;
    fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError>;
}

/// 🔌️ Format-keyed solid import codec.
pub trait SolidImporter: Send + Sync {
    fn format(&self) -> semio_framework::OsMediaFormat;
    fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError>;
}

pub struct StepSolidExporter;
pub struct StepSolidImporter;
pub struct StlSolidExporter;
pub struct StlSolidImporter;
pub struct ObjSolidExporter;
pub struct ObjSolidImporter;
pub struct GlbSolidExporter;
pub struct GlbSolidImporter;

impl SolidExporter for StepSolidExporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Step
    }
    fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], _deflection: f64) -> Result<Vec<u8>, BrepError> {
        Ok(kernel.export_step_sync(shapes)?.into_bytes())
    }
}
impl SolidImporter for StepSolidImporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Step
    }
    fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|e| BrepError::InvalidInput(e.to_string()))?;
        kernel.import_step_sync(text)
    }
}
impl SolidExporter for StlSolidExporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Stl
    }
    fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_stl_sync(shapes, deflection)
    }
}
impl SolidImporter for StlSolidImporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Stl
    }
    fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        Ok(vec![kernel.import_stl_sync(bytes, tolerance)?])
    }
}
impl SolidExporter for ObjSolidExporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Obj
    }
    fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        Ok(kernel.export_obj_sync(shapes, deflection)?.into_bytes())
    }
}
impl SolidImporter for ObjSolidImporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Obj
    }
    fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        let text = std::str::from_utf8(bytes).map_err(|e| BrepError::InvalidInput(e.to_string()))?;
        Ok(vec![kernel.import_obj_sync(text, tolerance)?])
    }
}
impl SolidExporter for GlbSolidExporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Glb
    }
    fn export(&self, kernel: &Brep, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_glb_sync(shapes, deflection)
    }
}
impl SolidImporter for GlbSolidImporter {
    fn format(&self) -> semio_framework::OsMediaFormat {
        semio_framework::OsMediaFormat::Glb
    }
    fn import(&self, kernel: &mut Brep, bytes: &[u8], tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        Ok(vec![kernel.import_glb_sync(bytes, tolerance)?])
    }
}

// #endregion 🔌️Codecs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::engine::block_on;

    #[test]
    fn native_box_volume() {
        let mut k = Brep::new();
        let solid = block_on(k.box_prim(1.0, 1.0, 1.0)).unwrap();
        let v = block_on(k.volume(&solid)).unwrap();
        assert!((v - 1.0).abs() < 1e-3, "volume {v}");
    }

    #[test]
    fn native_fuse_disjoint() {
        let mut k = Brep::new();
        let a = block_on(k.box_prim(1.0, 1.0, 1.0)).unwrap();
        let b = block_on(k.convex_hull(&[
            [2.0,0.0,0.0],[3.0,0.0,0.0],[3.0,1.0,0.0],[2.0,1.0,0.0],
            [2.0,0.0,1.0],[3.0,0.0,1.0],[3.0,1.0,1.0],[2.0,1.0,1.0],
        ])).unwrap();
        let u = block_on(k.fuse(&a, &b)).unwrap();
        let v = block_on(k.volume(&u)).unwrap();
        assert!((v - 2.0).abs() < 1e-2, "volume {v}");
    }
}
