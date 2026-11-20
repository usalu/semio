//! 📐️ Recognizes an exact rectangular extrusion as a certified primitive.

use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::{arena::FaceId,topology::{Body,Wire}};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 📋️ Borrows one possible original rectangular boundary.
pub enum RectangleProfile<'a> { Wire(&'a Wire), Face(FaceId) }

/// 📦️ Recognizes one horizontal rectangular line ring without copying topology.
pub fn rectangular_extrusion(body: &Body, source: RectangleProfile<'_>, vector: [f64;3]) -> Result<Option<SolidPrimitive>, ValueError> {
    let invalid = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original polygon extrusion requires a rectangular profile along the world Z axis");
    let mut corners = [Pnt3::new(0., 0., 0.); 4];
    let get_point = |edge_id, forward| -> Result<Pnt3, ValueError> {
        let edge = body.edges.get(edge_id).ok_or_else(invalid)?;
        if !matches!(body.curves3.get(edge.curve), Some(Curve3::Line { .. })) { return Err(invalid()); }
        body.vertices.get(if forward { edge.v0 } else { edge.v1 }).map(|vertex| vertex.position).ok_or_else(invalid)
    };
    match source {
        RectangleProfile::Wire(wire) if wire.closed && wire.members.len() == 4 => {
            for (point, &(edge, forward)) in corners.iter_mut().zip(&wire.members) {
                *point = get_point(edge, forward)?;
            }
        }
        RectangleProfile::Face(face) => {
            let face = body.faces.get(face).ok_or_else(invalid)?;
            if !face.inners.is_empty() || !matches!(body.surfaces.get(face.surface), Some(Surface::Plane { .. })) { return Ok(None); }
            let ring = face.outer.and_then(|id| body.loops.get(id)).ok_or_else(invalid)?;
            let mut use_id = ring.first;
            for point in &mut corners {
                let use_ = body.coedges.get(use_id).ok_or_else(invalid)?;
                *point = get_point(use_.edge, use_.forward)?;
                use_id = use_.next;
            }
            if use_id != ring.first { return Ok(None); }
        }
        _ => return Ok(None),
    }
    let [x0, y0, z] = corners[0].to_array();
    let mut bounds = [x0, y0, x0, y0];
    for i in 0..4 {
        let p = corners[i];
        let q = corners[(i + 1) % 4];
        if p.z != z || (p.x == q.x) == (p.y == q.y) { return Ok(None); }
        bounds = [bounds[0].min(p.x), bounds[1].min(p.y), bounds[2].max(p.x), bounds[3].max(p.y)];
    }
    let [min_x, min_y, max_x, max_y] = bounds;
    let mut visited = 0u8;
    for p in corners {
        if (p.x != min_x && p.x != max_x) || (p.y != min_y && p.y != max_y) { return Ok(None); }
        visited |= 1 << (u8::from(p.x == max_x) + 2 * u8::from(p.y == max_y));
    }
    if visited != 0b1111 { return Ok(None); }
    let [dx, dy, dz] = vector;
    if dx != 0. || dy != 0. || !dz.is_finite() || dz.abs() <= Tol::DEFAULT.value() { return Ok(None); }
    let origin = Pnt3::new(min_x, min_y, z + dz.min(0.));
    let primitive = SolidPrimitive::Box { origin, width: max_x - min_x, depth: max_y - min_y, height: dz.abs() };
    Ok(Some(primitive))
}
