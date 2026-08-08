//! ➡️ Extrude/revolve/loft/pipe/helical sweep.
//!
//! Native sweep ops that mutate a [`Body`](crate::brep::topo::Body) through
//! [`crate::brep::euler`] editors and attach [`Surface`](crate::brep::surface::Surface)
//! geometry (planes for polygonal sides, cylinders when a circular profile edge is extruded).

use std::f64::consts::TAU;

use crate::brep::arena::{ArenaId, EdgeId, FaceId, SolidId, VertexId};
use crate::brep::curve::Curve3;
use crate::brep::error::KernelError;
use crate::brep::euler::{add_face, add_shell, add_solid, make_edge, make_loop, make_vertex};
use crate::brep::history::OpRecorder;
use crate::brep::mat::Frame3;
use crate::brep::primitives::Wire;
use crate::brep::surface::Surface;
use crate::brep::tolerance::Tol;
use crate::brep::topo::Body;
use crate::brep::vec::{Pnt3, Vec3};

// #region 🔖️Helpers

fn placeholder_face() -> FaceId {
    ArenaId::from_raw(0, 0)
}

fn require_positive(name: &str, value: f64) -> Result<(), KernelError> {
    if value <= Tol::DEFAULT.value() {
        Err(KernelError::InvalidInput(format!("{name} must be positive, got {value}")))
    } else {
        Ok(())
    }
}

fn plane_at(origin: Pnt3, normal: Vec3) -> Surface {
    Surface::Plane {
        frame: Frame3::from_normal(origin, normal).expect("plane frame"),
    }
}

fn attach_face(
    body: &mut Body,
    surface_id: crate::brep::arena::SurfaceId,
    members: &[(EdgeId, bool)],
    flipped: bool,
    tol: Tol,
    rec: &mut OpRecorder,
) -> FaceId {
    let outer = make_loop(body, placeholder_face(), members);
    let face = add_face(body, surface_id, Some(outer), vec![], flipped, tol, rec);
    body.loops.get_mut(outer).unwrap().face = face;
    face
}

fn line_edge(
    body: &mut Body,
    a: Pnt3,
    b: Pnt3,
    va: VertexId,
    vb: VertexId,
    tol: Tol,
    rec: &mut OpRecorder,
) -> EdgeId {
    let curve = body.curves3.insert(Curve3::Line { origin: a, dir: b - a });
    make_edge(body, curve, (0.0, 1.0), va, vb, tol, rec)
}

fn finish_solid(body: &mut Body, faces: Vec<FaceId>, rec: &mut OpRecorder) -> SolidId {
    let shell = add_shell(body, faces, rec);
    add_solid(body, shell, vec![], rec)
}

fn newell_normal(points: &[Pnt3]) -> Option<Vec3> {
    if points.len() < 3 {
        return None;
    }
    let mut n = Vec3::ZERO;
    for i in 0..points.len() {
        let p = points[i];
        let q = points[(i + 1) % points.len()];
        n.x += (p.y - q.y) * (p.z + q.z);
        n.y += (p.z - q.z) * (p.x + q.x);
        n.z += (p.x - q.x) * (p.y + q.y);
    }
    n.normalized()
}

/// ➡️ Ordered outer-loop vertex positions of a face (polyline samples for circular edges).
fn face_outer_polygon(body: &Body, face: FaceId) -> Result<Vec<Pnt3>, KernelError> {
    let face_data = body
        .faces
        .get(face)
        .ok_or_else(|| KernelError::MissingEntity(format!("face {face:?}")))?;
    let outer = face_data
        .outer
        .ok_or_else(|| KernelError::InvalidInput("face has no outer loop".into()))?;
    let coedges = body.loop_coedges(outer);
    if coedges.is_empty() {
        return Err(KernelError::InvalidInput("face outer loop is empty".into()));
    }
    let mut points = Vec::new();
    for cid in coedges {
        let coedge = body
            .coedges
            .get(cid)
            .ok_or_else(|| KernelError::MissingEntity(format!("coedge {cid:?}")))?;
        let edge = body
            .edges
            .get(coedge.edge)
            .ok_or_else(|| KernelError::MissingEntity(format!("edge {:?}", coedge.edge)))?;
        let curve = body
            .curves3
            .get(edge.curve)
            .ok_or_else(|| KernelError::MissingEntity(format!("curve {:?}", edge.curve)))?;
        match curve {
            Curve3::Circle { frame, radius } => {
                let segments = 16usize.max(3);
                let (t0, t1) = edge.range;
                for i in 0..segments {
                    let t = if coedge.forward {
                        t0 + (t1 - t0) * (i as f64) / (segments as f64)
                    } else {
                        t1 + (t0 - t1) * (i as f64) / (segments as f64)
                    };
                    let _ = frame;
                    let _ = radius;
                    points.push(curve.eval(t));
                }
            }
            _ => {
                let (start, _) = body
                    .coedge_endpoints(cid)
                    .ok_or_else(|| KernelError::MissingEntity(format!("coedge endpoints {cid:?}")))?;
                let p = body
                    .vertices
                    .get(start)
                    .ok_or_else(|| KernelError::MissingEntity(format!("vertex {start:?}")))?
                    .position;
                points.push(p);
            }
        }
    }
    if points.len() < 3 {
        return Err(KernelError::InvalidInput(format!(
            "extrude profile needs ≥3 points, got {}",
            points.len()
        )));
    }
    Ok(points)
}

/// ➡️ Prism solid from a closed polygon extruded by `offset` (bottom + top + planar sides).
fn solid_from_prism(body: &mut Body, bottom: &[Pnt3], offset: Vec3) -> Result<SolidId, KernelError> {
    let n = bottom.len();
    if n < 3 {
        return Err(KernelError::InvalidInput("prism needs ≥3 bottom points".into()));
    }
    let top: Vec<Pnt3> = bottom.iter().map(|&p| p + offset).collect();
    let bottom_normal = newell_normal(bottom)
        .ok_or_else(|| KernelError::InvalidInput("bottom polygon is degenerate".into()))?;
    let outward_bottom = if bottom_normal.dot(offset) > 0.0 {
        -bottom_normal
    } else {
        bottom_normal
    };
    let outward_top = -outward_bottom;

    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let v_bot: Vec<VertexId> = bottom.iter().map(|&p| make_vertex(body, p, tol, &mut rec)).collect();
    let v_top: Vec<VertexId> = top.iter().map(|&p| make_vertex(body, p, tol, &mut rec)).collect();

    let mut e_bot = Vec::with_capacity(n);
    let mut e_top = Vec::with_capacity(n);
    let mut e_vert = Vec::with_capacity(n);
    for i in 0..n {
        let j = (i + 1) % n;
        e_bot.push(line_edge(body, bottom[i], bottom[j], v_bot[i], v_bot[j], tol, &mut rec));
        e_top.push(line_edge(body, top[i], top[j], v_top[i], v_top[j], tol, &mut rec));
        e_vert.push(line_edge(body, bottom[i], top[i], v_bot[i], v_top[i], tol, &mut rec));
    }

    let s_bottom = body.surfaces.insert(plane_at(bottom[0], outward_bottom));
    let s_top = body.surfaces.insert(plane_at(top[0], outward_top));
    let bottom_members: Vec<(EdgeId, bool)> = e_bot.iter().rev().map(|&e| (e, false)).collect();
    let top_members: Vec<(EdgeId, bool)> = e_top.iter().map(|&e| (e, true)).collect();
    let bottom_face = attach_face(body, s_bottom, &bottom_members, false, tol, &mut rec);
    let top_face = attach_face(body, s_top, &top_members, false, tol, &mut rec);

    let mut faces = vec![bottom_face, top_face];
    for i in 0..n {
        let j = (i + 1) % n;
        let p0 = bottom[i];
        let p1 = bottom[j];
        let edge_dir = p1 - p0;
        let side_n = edge_dir.cross(offset).normalized().unwrap_or_else(|| {
            if outward_bottom.dot(Vec3::Z).abs() < 0.9 {
                Vec3::Z
            } else {
                Vec3::X
            }
        });
        let side_n = if side_n.dot(outward_bottom.cross(edge_dir).normalized().unwrap_or(side_n)) < 0.0 {
            -side_n
        } else {
            side_n
        };
        // Prefer outward relative to polygon centroid.
        let centroid = {
            let mut c = Vec3::ZERO;
            for p in bottom {
                c = c + p.to_vec();
            }
            Pnt3::from_array((c * (1.0 / n as f64)).to_array())
        };
        let mid = Pnt3::new((p0.x + p1.x) * 0.5, (p0.y + p1.y) * 0.5, (p0.z + p1.z) * 0.5);
        let side_n = if (mid - centroid).dot(side_n) < 0.0 {
            -side_n
        } else {
            side_n
        };
        let s_side = body.surfaces.insert(plane_at(p0, side_n));
        let members = [
            (e_bot[i], true),
            (e_vert[j], true),
            (e_top[i], false),
            (e_vert[i], false),
        ];
        faces.push(attach_face(body, s_side, &members, false, tol, &mut rec));
    }
    Ok(finish_solid(body, faces, &mut rec))
}

/// ➡️ Cylinder solid when extruding a single closed circular edge along its plane normal.
fn try_extrude_circle_cylinder(
    body: &mut Body,
    face: FaceId,
    direction: Vec3,
    distance: f64,
) -> Result<Option<SolidId>, KernelError> {
    let face_data = body
        .faces
        .get(face)
        .ok_or_else(|| KernelError::MissingEntity(format!("face {face:?}")))?;
    let Some(outer) = face_data.outer else {
        return Ok(None);
    };
    let coedges = body.loop_coedges(outer);
    if coedges.len() != 1 {
        return Ok(None);
    }
    let coedge = body.coedges.get(coedges[0]).unwrap();
    let edge = body.edges.get(coedge.edge).unwrap();
    let Some(Curve3::Circle { frame, radius }) = body.curves3.get(edge.curve).cloned() else {
        return Ok(None);
    };
    let dir = direction
        .normalized()
        .ok_or_else(|| KernelError::InvalidInput("extrude direction is zero".into()))?;
    if dir.cross(frame.z).norm() > 1e-6 {
        return Ok(None);
    }
    let height = if dir.dot(frame.z) >= 0.0 { distance } else { -distance };
    if height.abs() <= Tol::DEFAULT.value() {
        return Err(KernelError::InvalidInput("extrude distance is zero".into()));
    }
    let abs_h = height.abs();
    let z_sign = height.signum();
    let mut rec = OpRecorder::new();
    let tol = Tol::DEFAULT;
    let bot_center = frame.origin;
    let top_center = bot_center + frame.z * (abs_h * z_sign);
    let bot_pt = frame.to_world(Pnt3::new(radius, 0.0, 0.0));
    let top_pt = bot_pt + frame.z * (abs_h * z_sign);
    let v_bot = make_vertex(body, bot_pt, tol, &mut rec);
    let v_top = make_vertex(body, top_pt, tol, &mut rec);
    let e_bot = {
        let curve = body.curves3.insert(Curve3::Circle {
            frame: Frame3 {
                origin: bot_center,
                x: frame.x,
                y: frame.y,
                z: frame.z,
            },
            radius,
        });
        make_edge(body, curve, (0.0, TAU), v_bot, v_bot, tol, &mut rec)
    };
    let e_top = {
        let curve = body.curves3.insert(Curve3::Circle {
            frame: Frame3 {
                origin: top_center,
                x: frame.x,
                y: frame.y,
                z: frame.z,
            },
            radius,
        });
        make_edge(body, curve, (0.0, TAU), v_top, v_top, tol, &mut rec)
    };
    let e_seam = line_edge(body, bot_pt, top_pt, v_bot, v_top, tol, &mut rec);
    let cyl_frame = Frame3 {
        origin: if z_sign >= 0.0 { bot_center } else { top_center },
        x: frame.x,
        y: frame.y,
        z: if z_sign >= 0.0 { frame.z } else { -frame.z },
    };
    let cyl = body.surfaces.insert(Surface::Cylinder {
        frame: cyl_frame,
        radius,
    });
    let lateral = attach_face(
        body,
        cyl,
        &[(e_bot, true), (e_seam, true), (e_top, false), (e_seam, false)],
        false,
        tol,
        &mut rec,
    );
    let s_bottom = body.surfaces.insert(plane_at(bot_center, -frame.z * z_sign));
    let s_top = body.surfaces.insert(plane_at(top_center, frame.z * z_sign));
    let bottom = attach_face(body, s_bottom, &[(e_bot, false)], false, tol, &mut rec);
    let top = attach_face(body, s_top, &[(e_top, true)], false, tol, &mut rec);
    Ok(Some(finish_solid(body, vec![lateral, bottom, top], &mut rec)))
}

// #endregion 🔖️Helpers

// #region 🔖️Api

/// ➡️ Extrude a face along `direction` by `distance`, producing a closed solid prism (or cylinder).
///
/// Planar polygonal faces yield an axis-aligned-style prism of planar faces. A single closed
/// circular outer edge extruded along the circle axis becomes an analytic cylinder solid.
pub fn extrude_face(
    body: &mut Body,
    face: FaceId,
    direction: Vec3,
    distance: f64,
) -> Result<SolidId, KernelError> {
    require_positive("extrude distance", distance.abs())?;
    let dir = direction
        .normalized()
        .ok_or_else(|| KernelError::InvalidInput("extrude direction is zero-length".into()))?;
    let offset = dir * distance;
    if let Some(solid) = try_extrude_circle_cylinder(body, face, dir, distance)? {
        return Ok(solid);
    }
    let polygon = face_outer_polygon(body, face)?;
    solid_from_prism(body, &polygon, offset)
}

/// ➡️ Revolves a planar face about an axis by sampling section polygons into a solid.
pub fn revolve_face(
    body: &mut Body,
    face: FaceId,
    axis_origin: Pnt3,
    axis_direction: Vec3,
    angle: f64,
) -> Result<SolidId, KernelError> {
    let axis = axis_direction
        .normalized()
        .ok_or_else(|| KernelError::InvalidInput("revolve axis is zero-length".into()))?;
    if !angle.is_finite() || angle.abs() <= 1e-12 {
        return Err(KernelError::InvalidInput("revolve angle must be non-zero".into()));
    }
    let profile = face_outer_polygon(body, face)?;
    let steps = ((angle.abs() / std::f64::consts::FRAC_PI_4).ceil() as usize).clamp(8, 64);
    let mut sections = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let a = angle * t;
        let section: Vec<Pnt3> = profile
            .iter()
            .map(|&p| rotate_around_axis(p, axis_origin, axis, a))
            .collect();
        sections.push(section);
    }
    solid_from_lofted_sections(body, &sections)
}

/// ➡️ Lofts profile faces into a solid by connecting successive outer polygons.
pub fn loft_profiles(
    body: &mut Body,
    profiles: &[FaceId],
    _smooth: bool,
) -> Result<SolidId, KernelError> {
    if profiles.len() < 2 {
        return Err(KernelError::InvalidInput("loft requires at least two profiles".into()));
    }
    let mut sections = Vec::with_capacity(profiles.len());
    for &face in profiles {
        sections.push(face_outer_polygon(body, face)?);
    }
    solid_from_lofted_sections(body, &sections)
}

/// ➡️ Sweeps a profile face along a wire path.
pub fn sweep_along_path(
    body: &mut Body,
    profile: FaceId,
    path: &Wire,
) -> Result<SolidId, KernelError> {
    let polygon = face_outer_polygon(body, profile)?;
    let samples = sample_wire_points(body, path, 16)?;
    if samples.len() < 2 {
        return Err(KernelError::InvalidInput("sweep path needs at least two samples".into()));
    }
    let origin = samples[0];
    let mut sections = Vec::with_capacity(samples.len());
    for i in 0..samples.len() {
        let tangent = if i + 1 < samples.len() {
            samples[i + 1] - samples[i]
        } else {
            samples[i] - samples[i - 1]
        };
        let xdir = tangent.normalized().unwrap_or(Vec3::X);
        let zdir = xdir.cross(Vec3::Z).normalized().unwrap_or_else(|| xdir.cross(Vec3::Y).normalized().unwrap_or(Vec3::Y));
        let ydir = zdir.cross(xdir).normalized().unwrap_or(Vec3::Z);
        let section = polygon
            .iter()
            .map(|&p| {
                let local = p - origin;
                samples[i] + xdir * local.x + ydir * local.y + zdir * local.z
            })
            .collect::<Vec<_>>();
        sections.push(section);
    }
    solid_from_lofted_sections(body, &sections)
}

/// ➡️ Pipes a profile along a path (guide currently ignored — constant scale).
pub fn pipe(
    body: &mut Body,
    profile: FaceId,
    path: &Wire,
    _guide: Option<&Wire>,
) -> Result<SolidId, KernelError> {
    sweep_along_path(body, profile, path)
}

/// ➡️ Helical sweep of a profile about an axis.
pub fn helical_sweep(
    body: &mut Body,
    profile: FaceId,
    axis_origin: Pnt3,
    axis_dir: Vec3,
    radius: f64,
    pitch: f64,
    turns: f64,
) -> Result<SolidId, KernelError> {
    require_positive("helical radius", radius)?;
    if !turns.is_finite() || turns.abs() <= 1e-12 {
        return Err(KernelError::InvalidInput("helical turns must be non-zero".into()));
    }
    let axis = axis_dir
        .normalized()
        .ok_or_else(|| KernelError::InvalidInput("helical axis is zero-length".into()))?;
    let polygon = face_outer_polygon(body, profile)?;
    let steps = ((turns.abs() * 16.0).ceil() as usize).clamp(16, 128);
    let mut sections = Vec::with_capacity(steps + 1);
    let start = polygon[0];
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let angle = turns * std::f64::consts::TAU * t;
        let along = pitch * turns * t;
        let radial = rotate_around_axis(start, axis_origin, axis, angle);
        let center = axis_origin + axis * along;
        // Place profile in a frame along the helix.
        let offset = (radial - axis_origin) - axis * (radial - axis_origin).dot(axis);
        let xdir = if offset.norm() > 1e-9 {
            offset.normalized().unwrap()
        } else {
            axis.cross(Vec3::X).normalized().unwrap_or(Vec3::Y)
        };
        let ydir = axis.cross(xdir).normalized().unwrap_or(Vec3::Z);
        let section = polygon
            .iter()
            .map(|&p| {
                let local = p - start;
                center + xdir * (radius + local.x) + ydir * local.y + axis * local.z
            })
            .collect::<Vec<_>>();
        sections.push(section);
    }
    solid_from_lofted_sections(body, &sections)
}

fn rotate_around_axis(point: Pnt3, origin: Pnt3, axis: Vec3, angle: f64) -> Pnt3 {
    let v = point - origin;
    let cos = angle.cos();
    let sin = angle.sin();
    let parallel = axis * v.dot(axis);
    let lateral = v - parallel;
    let rotated = lateral * cos + axis.cross(lateral) * sin + parallel;
    origin + rotated
}

fn sample_wire_points(body: &Body, wire: &Wire, samples_per_edge: usize) -> Result<Vec<Pnt3>, KernelError> {
    let mut points = Vec::new();
    for (edge_id, forward) in &wire.members {
        let edge = body
            .edges
            .get(*edge_id)
            .ok_or_else(|| KernelError::MissingEntity(format!("edge {edge_id}")))?;
        let curve = body
            .curves3
            .get(edge.curve)
            .ok_or_else(|| KernelError::MissingEntity(format!("curve {}", edge.curve)))?;
        let (a, b) = edge.range;
        for i in 0..samples_per_edge {
            let t = if samples_per_edge <= 1 {
                0.0
            } else {
                i as f64 / (samples_per_edge as f64 - 1.0)
            };
            let u = if *forward { a + (b - a) * t } else { b + (a - b) * t };
            let p = curve_point(curve, u);
            if points.last().map(|q: &Pnt3| (*q - p).norm() > 1e-9).unwrap_or(true) {
                points.push(p);
            }
        }
    }
    Ok(points)
}

fn curve_point(curve: &crate::brep::curve::Curve3, u: f64) -> Pnt3 {
    use crate::brep::curve::Curve3;
    match curve {
        Curve3::Line { origin, dir } => *origin + *dir * u,
        Curve3::Circle { frame, radius } => {
            let c = u.cos();
            let s = u.sin();
            frame.origin + frame.x * (*radius * c) + frame.y * (*radius * s)
        }
        Curve3::Ellipse { frame, major_radius, minor_radius } => {
            let c = u.cos();
            let s = u.sin();
            frame.origin + frame.x * (*major_radius * c) + frame.y * (*minor_radius * s)
        }
        Curve3::Nurbs { .. } => Pnt3::new(0.0, 0.0, 0.0),
    }
}

fn solid_from_lofted_sections(body: &mut Body, sections: &[Vec<Pnt3>]) -> Result<SolidId, KernelError> {
    if sections.len() < 2 {
        return Err(KernelError::InvalidInput("loft/sweep needs at least two sections".into()));
    }
    let n = sections[0].len();
    if n < 3 {
        return Err(KernelError::InvalidInput("section needs ≥3 points".into()));
    }
    for section in sections {
        if section.len() != n {
            return Err(KernelError::InvalidInput("loft sections must have equal vertex counts".into()));
        }
    }
    // Side quads as two triangles + capped ends via solid_from_prism-like construction.
    let mut triangles: Vec<[Pnt3; 3]> = Vec::new();
    for s in 0..sections.len() - 1 {
        let a = &sections[s];
        let b = &sections[s + 1];
        for i in 0..n {
            let j = (i + 1) % n;
            triangles.push([a[i], b[i], b[j]]);
            triangles.push([a[i], b[j], a[j]]);
        }
    }
    // Caps
    let bottom = &sections[0];
    let top = &sections[sections.len() - 1];
    for i in 1..n - 1 {
        triangles.push([bottom[0], bottom[i], bottom[i + 1]]);
        triangles.push([top[0], top[i + 1], top[i]]);
    }
    crate::brep::primitives::solid_from_triangle_soup(body, &triangles)
}

// #endregion 🔖️Api

#[cfg(test)]
mod tests {
    use super::*;
    use crate::brep::measure::solid_volume;
    use crate::brep::primitives::{make_box, make_planar_face_from_points, make_rectangle_wire};
    use crate::brep::validate::validate_body;

    fn solid_counts(body: &Body, solid: SolidId) -> (usize, usize, usize) {
        let faces = body.solid_faces(solid);
        let mut edge_ids = std::collections::HashSet::new();
        let mut vertex_ids = std::collections::HashSet::new();
        for face in &faces {
            for coedge in body.face_coedges(*face) {
                let edge = body.coedges.get(coedge).unwrap().edge;
                edge_ids.insert(edge);
                let e = body.edges.get(edge).unwrap();
                vertex_ids.insert(e.v0);
                vertex_ids.insert(e.v1);
            }
        }
        (vertex_ids.len(), edge_ids.len(), faces.len())
    }

    #[test]
    fn extrude_rectangle_matches_box_topology_and_volume() {
        let mut body = Body::new();
        let face = make_planar_face_from_points(
            &mut body,
            &[
                Pnt3::new(0.0, 0.0, 0.0),
                Pnt3::new(2.0, 0.0, 0.0),
                Pnt3::new(2.0, 3.0, 0.0),
                Pnt3::new(0.0, 3.0, 0.0),
            ],
        )
        .unwrap();
        let solid = extrude_face(&mut body, face, Vec3::Z, 4.0).unwrap();
        let (v, e, f) = solid_counts(&body, solid);
        assert_eq!((v, e, f), (8, 12, 6));
        assert_eq!(v as i64 - e as i64 + f as i64, 2);
        let vol = solid_volume(&body, solid, 0.1).unwrap();
        assert!((vol - 24.0).abs() < 1e-6, "expected volume 24, got {vol}");

        let mut ref_body = Body::new();
        let ref_solid = make_box(&mut ref_body, 2.0, 3.0, 4.0).unwrap();
        let ref_vol = solid_volume(&ref_body, ref_solid, 0.1).unwrap();
        assert!((vol - ref_vol).abs() < 1e-6, "extrude vol {vol} vs make_box {ref_vol}");
        let issues = validate_body(&body);
        let ring_issues: Vec<_> = issues
            .iter()
            .filter(|i| matches!(i.code, "empty-loop" | "broken-ring" | "loop-not-closed" | "next-prev-mismatch"))
            .collect();
        assert!(ring_issues.is_empty(), "{ring_issues:?}");
    }

    #[test]
    fn extrude_rejects_zero_direction_and_distance() {
        let mut body = Body::new();
        let face = make_planar_face_from_points(
            &mut body,
            &[
                Pnt3::new(0.0, 0.0, 0.0),
                Pnt3::new(1.0, 0.0, 0.0),
                Pnt3::new(0.0, 1.0, 0.0),
            ],
        )
        .unwrap();
        assert!(extrude_face(&mut body, face, Vec3::ZERO, 1.0).is_err());
        assert!(extrude_face(&mut body, face, Vec3::Z, 0.0).is_err());
    }

    #[test]
    fn revolve_face_produces_solid() {
        let mut body = Body::new();
        let face = make_planar_face_from_points(
            &mut body,
            &[
                Pnt3::new(1.0, 0.0, 0.0),
                Pnt3::new(2.0, 0.0, 0.0),
                Pnt3::new(2.0, 0.0, 1.0),
                Pnt3::new(1.0, 0.0, 1.0),
            ],
        )
        .unwrap();
        let solid = revolve_face(&mut body, face, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, TAU).expect("revolve");
        assert!(solid_volume(&body, solid, 1e-3).unwrap() > 0.0);
    }

    #[test]
    fn extrude_uses_rectangle_wire_helper() {
        let mut body = Body::new();
        let wire = make_rectangle_wire(&mut body, 1.0, 1.0).unwrap();
        let face = crate::brep::primitives::make_planar_face_from_wire(&mut body, &wire, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z)
            .unwrap();
        let solid = extrude_face(&mut body, face, Vec3::Z, 1.0).unwrap();
        let vol = solid_volume(&body, solid, 0.1).unwrap();
        assert!((vol - 1.0).abs() < 1e-6, "got {vol}");
    }
}
