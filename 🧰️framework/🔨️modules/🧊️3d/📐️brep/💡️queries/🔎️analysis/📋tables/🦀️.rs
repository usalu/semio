//! 📋 Per-face and per-edge tables for picking and inspection: persistent label, geometry kind, measure,
//! centroid and normal, the labels of neighbouring faces, and the dihedral angle between faces.

use super::ShapeScope;
use crate::brep::queries::mass_properties::{area_centroid_from_moments, area_from_moments, faces_moments, surface_uv};
use crate::brep::queries::validation::is_point_edge;
use crate::brep::representation::arena::{ArenaId, EdgeId, FaceId};
use crate::brep::representation::curve::curve_ops;
use crate::brep::representation::curve::Curve3;
use crate::brep::representation::error::KernelError;
use crate::brep::representation::surface::{surface_ops, Surface};
use crate::brep::representation::topology::Body;
use crate::brep::representation::vector::{Pnt3, Vec3};
use std::collections::{BTreeSet, HashMap};

/// 🏄️ The geometry of a face's supporting surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
#[value(rename_all = "camelCase")]
pub enum SurfaceKind {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
    Nurbs,
}

impl SurfaceKind {
    pub fn of(surface: &Surface) -> Self {
        match surface {
            Surface::Plane { .. } => Self::Plane,
            Surface::Cylinder { .. } => Self::Cylinder,
            Surface::Cone { .. } => Self::Cone,
            Surface::Sphere { .. } => Self::Sphere,
            Surface::Torus { .. } => Self::Torus,
            Surface::Nurbs { .. } => Self::Nurbs,
        }
    }
}

/// ➰️ The geometry of an edge's supporting curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
#[value(rename_all = "camelCase")]
pub enum CurveKind {
    Line,
    Circle,
    Ellipse,
    Nurbs,
}

impl CurveKind {
    pub fn of(curve: &Curve3) -> Self {
        match curve {
            Curve3::Line { .. } => Self::Line,
            Curve3::Circle { .. } => Self::Circle,
            Curve3::Ellipse { .. } => Self::Ellipse,
            Curve3::Nurbs { .. } => Self::Nurbs,
        }
    }
}

/// 📋 One face of the shape.
///
/// `normal` is the outward normal at the surface point nearest the centroid. It is `None` when that point
/// is not unique: the centroid lies within 0.1% of the radius of a cylinder's or cone's axis, a sphere's
/// centre or a torus's axis or core circle (a full cylinder's centroid is on its axis).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct FaceRow {
    pub label: u64,
    pub surface_kind: SurfaceKind,
    pub area: f64,
    pub centroid: [f64; 3],
    pub normal: Option<[f64; 3]>,
    pub loops: usize,
    pub adjacent: Vec<u64>,
}

/// ⛰️ Whether an edge between two faces bulges outwards (convex), dents inwards (concave) or is smooth.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
#[value(rename_all = "camelCase")]
pub enum Convexity {
    Convex,
    Concave,
    Flat,
}

/// 📐 The angle between the two faces meeting at an edge.
///
/// `normal_angle` is the angle between the faces' outward normals (0 for a smooth join). `interior_angle` is
/// the angle measured through the material: `pi - normal_angle` when convex (a box edge is `pi/2`),
/// `pi + normal_angle` when concave, `pi` when flat.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Dihedral {
    pub normal_angle: f64,
    pub interior_angle: f64,
    pub convexity: Convexity,
}

/// 🧵 One edge of the shape. `dihedral` is present only for an edge shared by exactly two distinct faces.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct EdgeRow {
    pub label: u64,
    pub curve_kind: CurveKind,
    pub length: f64,
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub degenerate: bool,
    pub faces: Vec<u64>,
    pub dihedral: Option<Dihedral>,
}

fn array(p: Pnt3) -> [f64; 3] {
    [p.x, p.y, p.z]
}

fn edge_faces(body: &Body) -> HashMap<EdgeId, Vec<FaceId>> {
    let mut map: HashMap<EdgeId, Vec<FaceId>> = HashMap::new();
    for (_, coedge) in body.coedges.iter() {
        if let Some(lp) = body.loops.get(coedge.loop_id) {
            map.entry(coedge.edge).or_default().push(lp.face);
        }
    }
    map
}

fn nearest_point_is_ambiguous(surface: &Surface, point: Pnt3) -> bool {
    const NEAR: f64 = 1e-3;
    match surface {
        Surface::Plane { .. } | Surface::Nurbs { .. } => false,
        Surface::Cylinder { frame, radius } => {
            let l = frame.to_local(point);
            l.x.hypot(l.y) <= NEAR * radius.abs()
        }
        Surface::Cone { frame, half_angle } => {
            let l = frame.to_local(point);
            l.x.hypot(l.y) <= NEAR * (l.z.abs() * half_angle.tan()).max(1e-12)
        }
        Surface::Sphere { frame, radius } => (point - frame.origin).norm() <= NEAR * radius.abs(),
        Surface::Torus { frame, major_radius, minor_radius } => {
            let l = frame.to_local(point);
            let radial = l.x.hypot(l.y);
            radial <= NEAR * major_radius.abs() || (radial - major_radius).hypot(l.z) <= NEAR * minor_radius.abs()
        }
    }
}

fn outward_normal(surface: &Surface, flipped: bool, u: f64, v: f64) -> Option<Vec3> {
    surface.normal(u, v).map(|n| if flipped { -n } else { n })
}

/// 🗒️ One [`FaceRow`] per face of `scope`, ascending by arena index; `tolerance` is the mass-integral tolerance.
pub fn face_table(body: &Body, scope: &ShapeScope, tolerance: f64) -> Result<Vec<FaceRow>, KernelError> {
    let members = scope.members(body)?;
    let neighbours = edge_faces(body);
    let mut rows = Vec::with_capacity(members.faces.len());
    for &face in &members.faces {
        let face_ent = body.faces.get(face).ok_or_else(|| KernelError::MissingEntity("face".into()))?;
        let surface = body.surfaces.get(face_ent.surface).ok_or_else(|| KernelError::MissingEntity("surface".into()))?;
        let (totals, _) = faces_moments(body, &[face], tolerance)?;
        let centroid = area_centroid_from_moments(&totals).unwrap_or(Pnt3::new(0.0, 0.0, 0.0));
        let normal = if nearest_point_is_ambiguous(surface, centroid) {
            None
        } else {
            let at = surface_ops::closest_uv(surface, surface.domain(), centroid, 1e-9);
            outward_normal(surface, face_ent.flipped, at.u, at.v).map(|n| [n.x, n.y, n.z])
        };
        let mut adjacent = BTreeSet::new();
        for coedge in body.face_coedges(face) {
            let Some(c) = body.coedges.get(coedge) else { continue };
            for &other in neighbours.get(&c.edge).map(Vec::as_slice).unwrap_or_default() {
                if other != face {
                    if let Some(other_face) = body.faces.get(other) {
                        adjacent.insert(other_face.label.0);
                    }
                }
            }
        }
        rows.push(FaceRow { label: face_ent.label.0, surface_kind: SurfaceKind::of(surface), area: area_from_moments(&totals), centroid: array(centroid), normal, loops: body.face_loops(face).len(), adjacent: adjacent.into_iter().collect() });
    }
    Ok(rows)
}

fn dihedral(body: &Body, edge: EdgeId, faces: &[FaceId]) -> Option<Dihedral> {
    let edge_ent = body.edges.get(edge)?;
    let curve = body.curves3.get(edge_ent.curve)?;
    let mid = 0.5 * (edge_ent.range.0 + edge_ent.range.1);
    let point = curve.eval(mid);
    let tangent = curve.d1(mid).normalized()?;
    let mut sides = Vec::new();
    for &face in faces {
        let face_ent = body.faces.get(face)?;
        let surface = body.surfaces.get(face_ent.surface)?;
        let uv = surface_uv(surface, point);
        let normal = outward_normal(surface, face_ent.flipped, uv.x, uv.y)?;
        let coedge = body.face_coedges(face).into_iter().filter_map(|id| body.coedges.get(id)).find(|c| c.edge == edge)?;
        let travel = if coedge.forward != face_ent.flipped { tangent } else { -tangent };
        sides.push((normal, normal.cross(travel)));
    }
    let [(n_a, _), (n_b, d_b)] = sides.try_into().ok()?;
    let normal_angle = n_a.dot(n_b).clamp(-1.0, 1.0).acos();
    let side = d_b.dot(n_a);
    let (convexity, interior_angle) = if normal_angle < 1e-9 || side.abs() < 1e-12 {
        (Convexity::Flat, std::f64::consts::PI)
    } else if side < 0.0 {
        (Convexity::Convex, std::f64::consts::PI - normal_angle)
    } else {
        (Convexity::Concave, std::f64::consts::PI + normal_angle)
    };
    Some(Dihedral { normal_angle, interior_angle, convexity })
}

/// 📑 One [`EdgeRow`] per edge of `scope`, ascending by arena index.
pub fn edge_table(body: &Body, scope: &ShapeScope) -> Result<Vec<EdgeRow>, KernelError> {
    let members = scope.members(body)?;
    let neighbours = edge_faces(body);
    let mut rows = Vec::with_capacity(members.edges.len());
    for &edge in &members.edges {
        let edge_ent = body.edges.get(edge).ok_or_else(|| KernelError::MissingEntity("edge".into()))?;
        let curve = body.curves3.get(edge_ent.curve).ok_or_else(|| KernelError::MissingEntity("curve".into()))?;
        let position = |id| body.vertices.get(id).map(|v| array(v.position)).ok_or_else(|| KernelError::MissingEntity("vertex".into()));
        let degenerate = is_point_edge(body, edge);
        let mut faces: Vec<FaceId> = neighbours.get(&edge).cloned().unwrap_or_default();
        faces.sort_unstable_by_key(|face| face.raw_index());
        faces.dedup();
        let labels: BTreeSet<u64> = faces.iter().filter_map(|&face| body.faces.get(face)).map(|f| f.label.0).collect();
        let uses = neighbours.get(&edge).map_or(0, Vec::len);
        let length = if degenerate { 0.0 } else { curve_ops::arc_length(curve, edge_ent.range.0, edge_ent.range.1, 1e-9) };
        rows.push(EdgeRow {
            label: edge_ent.label.0,
            curve_kind: CurveKind::of(curve),
            length,
            start: position(edge_ent.v0)?,
            end: position(edge_ent.v1)?,
            degenerate,
            faces: labels.into_iter().collect(),
            dihedral: if !degenerate && uses == 2 && faces.len() == 2 { dihedral(body, edge, &faces) } else { None },
        });
    }
    Ok(rows)
}
