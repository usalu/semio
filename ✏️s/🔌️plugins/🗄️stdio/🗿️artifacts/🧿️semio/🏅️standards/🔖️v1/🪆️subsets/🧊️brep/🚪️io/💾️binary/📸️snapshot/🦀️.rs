//! 💾️ Binary (pack) representation codec surface for `stdio.semio.brep` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};
use crate::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.semio.flow`'s upgraded `OpBinary`/`DiffCodec`
/// reuse) backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-
/// envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point3(out: &mut Vec<u8>, p: &SemioPoint3) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point3(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint3, String> {
    let x = reader.read_f64_le().map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().map_err(|e| e.to_string())?;
    let z = reader.read_f64_le().map_err(|e| e.to_string())?;
    Ok(SemioPoint3 { x, y, z })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_f64_vec(out: &mut Vec<u8>, v: &[f64]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_f64_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<f64>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(reader.read_f64_le().map_err(|e| e.to_string())?);
    }
    Ok(v)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_point3_vec(out: &mut Vec<u8>, v: &[SemioPoint3]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for p in v {
        write_point3(out, p);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_point3_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioPoint3>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(read_point3(reader)?);
    }
    Ok(v)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

/// 🏷️ `BrepCurve` variant tags — 0=Line, 1=Circle, 2=Ellipse, 3=Nurbs (declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_curve(out: &mut Vec<u8>, c: &BrepCurve) {
    match c {
        BrepCurve::Line { origin, direction } => {
            out.push(0);
            write_point3(out, origin);
            write_point3(out, direction);
        }
        BrepCurve::Circle { center, axis, radius } => {
            out.push(1);
            write_point3(out, center);
            write_point3(out, axis);
            out.extend_from_slice(&radius.to_le_bytes());
        }
        BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => {
            out.push(2);
            write_point3(out, center);
            write_point3(out, axis);
            out.extend_from_slice(&radius_major.to_le_bytes());
            out.extend_from_slice(&radius_minor.to_le_bytes());
        }
        BrepCurve::Nurbs { control_points, weights, degree, knots } => {
            out.push(3);
            write_point3_vec(out, control_points);
            write_f64_vec(out, weights);
            store::pack_rt::write_varint_u64(out, *degree as u64);
            write_f64_vec(out, knots);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_curve(reader: &mut store::ByteReader<'_>) -> Result<BrepCurve, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(BrepCurve::Line { origin: read_point3(reader)?, direction: read_point3(reader)? }),
        1 => Ok(BrepCurve::Circle { center: read_point3(reader)?, axis: read_point3(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(BrepCurve::Ellipse { center: read_point3(reader)?, axis: read_point3(reader)?, radius_major: reader.read_f64_le().map_err(|e| e.to_string())?, radius_minor: reader.read_f64_le().map_err(|e| e.to_string())? }),
        3 => Ok(BrepCurve::Nurbs { control_points: read_point3_vec(reader)?, weights: read_f64_vec(reader)?, degree: reader.read_varint_u64().map_err(|e| e.to_string())? as u32, knots: read_f64_vec(reader)? }),
        other => Err(format!("curve: unknown binary tag {other}")),
    }
}

/// 🏷️ `BrepSurface` variant tags — 0=Plane, 1=Cylinder, 2=Cone, 3=Sphere, 4=Torus, 5=Nurbs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_surface(out: &mut Vec<u8>, s: &BrepSurface) {
    match s {
        BrepSurface::Plane { origin, normal } => {
            out.push(0);
            write_point3(out, origin);
            write_point3(out, normal);
        }
        BrepSurface::Cylinder { origin, axis, radius } => {
            out.push(1);
            write_point3(out, origin);
            write_point3(out, axis);
            out.extend_from_slice(&radius.to_le_bytes());
        }
        BrepSurface::Cone { origin, axis, radius, half_angle } => {
            out.push(2);
            write_point3(out, origin);
            write_point3(out, axis);
            out.extend_from_slice(&radius.to_le_bytes());
            out.extend_from_slice(&half_angle.to_le_bytes());
        }
        BrepSurface::Sphere { center, radius } => {
            out.push(3);
            write_point3(out, center);
            out.extend_from_slice(&radius.to_le_bytes());
        }
        BrepSurface::Torus { center, axis, major_radius, minor_radius } => {
            out.push(4);
            write_point3(out, center);
            write_point3(out, axis);
            out.extend_from_slice(&major_radius.to_le_bytes());
            out.extend_from_slice(&minor_radius.to_le_bytes());
        }
        BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => {
            out.push(5);
            write_point3_vec(out, control_points);
            write_f64_vec(out, weights);
            store::pack_rt::write_varint_u64(out, *u_count as u64);
            store::pack_rt::write_varint_u64(out, *v_count as u64);
            store::pack_rt::write_varint_u64(out, *degree_u as u64);
            store::pack_rt::write_varint_u64(out, *degree_v as u64);
            write_f64_vec(out, knots_u);
            write_f64_vec(out, knots_v);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_surface(reader: &mut store::ByteReader<'_>) -> Result<BrepSurface, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(BrepSurface::Plane { origin: read_point3(reader)?, normal: read_point3(reader)? }),
        1 => Ok(BrepSurface::Cylinder { origin: read_point3(reader)?, axis: read_point3(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        2 => Ok(BrepSurface::Cone { origin: read_point3(reader)?, axis: read_point3(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())?, half_angle: reader.read_f64_le().map_err(|e| e.to_string())? }),
        3 => Ok(BrepSurface::Sphere { center: read_point3(reader)?, radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        4 => Ok(BrepSurface::Torus { center: read_point3(reader)?, axis: read_point3(reader)?, major_radius: reader.read_f64_le().map_err(|e| e.to_string())?, minor_radius: reader.read_f64_le().map_err(|e| e.to_string())? }),
        5 => Ok(BrepSurface::Nurbs {
            control_points: read_point3_vec(reader)?,
            weights: read_f64_vec(reader)?,
            u_count: reader.read_varint_u64().map_err(|e| e.to_string())? as u32,
            v_count: reader.read_varint_u64().map_err(|e| e.to_string())? as u32,
            degree_u: reader.read_varint_u64().map_err(|e| e.to_string())? as u32,
            degree_v: reader.read_varint_u64().map_err(|e| e.to_string())? as u32,
            knots_u: read_f64_vec(reader)?,
            knots_v: read_f64_vec(reader)?,
        }),
        other => Err(format!("surface: unknown binary tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_vertex(out: &mut Vec<u8>, v: &BrepVertex) {
    write_str_lp(out, &v.id);
    write_point3(out, &v.point);
    out.extend_from_slice(&v.tol.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_vertex(reader: &mut store::ByteReader<'_>) -> Result<BrepVertex, String> {
    Ok(BrepVertex { id: read_str_lp(reader)?, point: read_point3(reader)?, tol: reader.read_f64_le().map_err(|e| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_edge(out: &mut Vec<u8>, e: &BrepEdge) {
    write_str_lp(out, &e.id);
    write_str_lp(out, &e.start_vertex);
    write_str_lp(out, &e.end_vertex);
    write_curve(out, &e.curve);
    out.extend_from_slice(&e.tol.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_edge(reader: &mut store::ByteReader<'_>) -> Result<BrepEdge, String> {
    Ok(BrepEdge { id: read_str_lp(reader)?, start_vertex: read_str_lp(reader)?, end_vertex: read_str_lp(reader)?, curve: read_curve(reader)?, tol: reader.read_f64_le().map_err(|e| e.to_string())? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_loop_edge(out: &mut Vec<u8>, le: &BrepLoopEdge) {
    write_str_lp(out, &le.edge);
    write_bool(out, le.orientation);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_loop_edge(reader: &mut store::ByteReader<'_>) -> Result<BrepLoopEdge, String> {
    Ok(BrepLoopEdge { edge: read_str_lp(reader)?, orientation: read_bool(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_loop(out: &mut Vec<u8>, l: &BrepLoop) {
    write_str_lp(out, &l.id);
    store::pack_rt::write_varint_u64(out, l.edges.len() as u64);
    for le in &l.edges {
        write_loop_edge(out, le);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_loop(reader: &mut store::ByteReader<'_>) -> Result<BrepLoop, String> {
    let id = read_str_lp(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut edges = Vec::with_capacity(n as usize);
    for _ in 0..n {
        edges.push(read_loop_edge(reader)?);
    }
    Ok(BrepLoop { id, edges })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_face(out: &mut Vec<u8>, f: &BrepFace) {
    write_str_lp(out, &f.id);
    write_str_lp(out, &f.outer_loop);
    store::pack_rt::write_varint_u64(out, f.inner_loops.len() as u64);
    for il in &f.inner_loops {
        write_str_lp(out, il);
    }
    write_surface(out, &f.surface);
    write_bool(out, f.orientation);
    out.extend_from_slice(&f.tol.to_le_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_face(reader: &mut store::ByteReader<'_>) -> Result<BrepFace, String> {
    let id = read_str_lp(reader)?;
    let outer_loop = read_str_lp(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut inner_loops = Vec::with_capacity(n as usize);
    for _ in 0..n {
        inner_loops.push(read_str_lp(reader)?);
    }
    let surface = read_surface(reader)?;
    let orientation = read_bool(reader)?;
    let tol = reader.read_f64_le().map_err(|e| e.to_string())?;
    Ok(BrepFace { id, outer_loop, inner_loops, surface, orientation, tol })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_shell_face(out: &mut Vec<u8>, sf: &BrepShellFace) {
    write_str_lp(out, &sf.face);
    write_bool(out, sf.orientation);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_shell_face(reader: &mut store::ByteReader<'_>) -> Result<BrepShellFace, String> {
    Ok(BrepShellFace { face: read_str_lp(reader)?, orientation: read_bool(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_shell(out: &mut Vec<u8>, sh: &BrepShell) {
    write_str_lp(out, &sh.id);
    store::pack_rt::write_varint_u64(out, sh.faces.len() as u64);
    for sf in &sh.faces {
        write_shell_face(out, sf);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_shell(reader: &mut store::ByteReader<'_>) -> Result<BrepShell, String> {
    let id = read_str_lp(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut faces = Vec::with_capacity(n as usize);
    for _ in 0..n {
        faces.push(read_shell_face(reader)?);
    }
    Ok(BrepShell { id, faces })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_solid_shell(out: &mut Vec<u8>, ss: &BrepSolidShell) {
    write_str_lp(out, &ss.shell);
    write_bool(out, ss.is_void);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_solid_shell(reader: &mut store::ByteReader<'_>) -> Result<BrepSolidShell, String> {
    Ok(BrepSolidShell { shell: read_str_lp(reader)?, is_void: read_bool(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_solid(out: &mut Vec<u8>, so: &BrepSolid) {
    write_str_lp(out, &so.id);
    store::pack_rt::write_varint_u64(out, so.shells.len() as u64);
    for ss in &so.shells {
        write_solid_shell(out, ss);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_solid(reader: &mut store::ByteReader<'_>) -> Result<BrepSolid, String> {
    let id = read_str_lp(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut shells = Vec::with_capacity(n as usize);
    for _ in 0..n {
        shells.push(read_solid_shell(reader)?);
    }
    Ok(BrepSolid { id, shells })
}

/// 🏷️ `BrepCurve2` variant tags — 0=Line, 1=Circle, 2=Ellipse, 3=Nurbs (declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_curve2(out: &mut Vec<u8>, c: &BrepCurve2) {
    match c {
        BrepCurve2::Line { origin, direction } => {
            out.push(0);
            out.extend_from_slice(&origin.x.to_le_bytes());
            out.extend_from_slice(&origin.y.to_le_bytes());
            out.extend_from_slice(&direction.x.to_le_bytes());
            out.extend_from_slice(&direction.y.to_le_bytes());
        }
        BrepCurve2::Circle { center, radius } => {
            out.push(1);
            out.extend_from_slice(&center.x.to_le_bytes());
            out.extend_from_slice(&center.y.to_le_bytes());
            out.extend_from_slice(&radius.to_le_bytes());
        }
        BrepCurve2::Ellipse { center, x_axis, radius_major, radius_minor } => {
            out.push(2);
            out.extend_from_slice(&center.x.to_le_bytes());
            out.extend_from_slice(&center.y.to_le_bytes());
            out.extend_from_slice(&x_axis.x.to_le_bytes());
            out.extend_from_slice(&x_axis.y.to_le_bytes());
            out.extend_from_slice(&radius_major.to_le_bytes());
            out.extend_from_slice(&radius_minor.to_le_bytes());
        }
        BrepCurve2::Nurbs { control_points, weights, degree, knots } => {
            out.push(3);
            store::pack_rt::write_varint_u64(out, control_points.len() as u64);
            for p in control_points {
                out.extend_from_slice(&p.x.to_le_bytes());
                out.extend_from_slice(&p.y.to_le_bytes());
            }
            write_f64_vec(out, weights);
            store::pack_rt::write_varint_u64(out, *degree as u64);
            write_f64_vec(out, knots);
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_curve2(reader: &mut store::ByteReader<'_>) -> Result<BrepCurve2, String> {
    let read_f64 = |reader: &mut store::ByteReader<'_>| reader.read_f64_le().map_err(|e| e.to_string());
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => Ok(BrepCurve2::Line { origin: SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? }, direction: SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? } }),
        1 => Ok(BrepCurve2::Circle { center: SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? }, radius: read_f64(reader)? }),
        2 => Ok(BrepCurve2::Ellipse { center: SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? }, x_axis: SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? }, radius_major: read_f64(reader)?, radius_minor: read_f64(reader)? }),
        3 => {
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut control_points = Vec::with_capacity(n as usize);
            for _ in 0..n {
                control_points.push(SemioPoint2 { x: read_f64(reader)?, y: read_f64(reader)? });
            }
            let weights = read_f64_vec(reader)?;
            let degree = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
            let knots = read_f64_vec(reader)?;
            Ok(BrepCurve2::Nurbs { control_points, weights, degree, knots })
        }
        other => Err(format!("curve2: unknown binary tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_coedge(out: &mut Vec<u8>, c: &BrepCoedge) {
    write_str_lp(out, &c.id);
    write_str_lp(out, &c.edge);
    write_bool(out, c.forward);
    match &c.pcurve {
        Some(curve) => {
            write_bool(out, true);
            write_curve2(out, curve);
        }
        None => write_bool(out, false),
    }
    out.extend_from_slice(&c.prange.0.to_le_bytes());
    out.extend_from_slice(&c.prange.1.to_le_bytes());
    write_str_lp(out, &c.loop_id);
    write_str_lp(out, &c.next);
    write_str_lp(out, &c.prev);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_coedge(reader: &mut store::ByteReader<'_>) -> Result<BrepCoedge, String> {
    let id = read_str_lp(reader)?;
    let edge = read_str_lp(reader)?;
    let forward = read_bool(reader)?;
    let has_pcurve = read_bool(reader)?;
    let pcurve = if has_pcurve { Some(read_curve2(reader)?) } else { None };
    let prange = (reader.read_f64_le().map_err(|e| e.to_string())?, reader.read_f64_le().map_err(|e| e.to_string())?);
    let loop_id = read_str_lp(reader)?;
    let next = read_str_lp(reader)?;
    let prev = read_str_lp(reader)?;
    Ok(BrepCoedge { id, edge, forward, pcurve, prange, loop_id, next, prev })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_brep_snapshot_binary(s: &SemioBrepSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.vertices.len() as u64);
    for v in &s.vertices {
        write_vertex(&mut out, v);
    }
    store::pack_rt::write_varint_u64(&mut out, s.edges.len() as u64);
    for e in &s.edges {
        write_edge(&mut out, e);
    }
    store::pack_rt::write_varint_u64(&mut out, s.loops.len() as u64);
    for l in &s.loops {
        write_loop(&mut out, l);
    }
    store::pack_rt::write_varint_u64(&mut out, s.faces.len() as u64);
    for f in &s.faces {
        write_face(&mut out, f);
    }
    store::pack_rt::write_varint_u64(&mut out, s.shells.len() as u64);
    for sh in &s.shells {
        write_shell(&mut out, sh);
    }
    store::pack_rt::write_varint_u64(&mut out, s.solids.len() as u64);
    for so in &s.solids {
        write_solid(&mut out, so);
    }
    store::pack_rt::write_varint_u64(&mut out, s.coedges.len() as u64);
    for c in &s.coedges {
        write_coedge(&mut out, c);
    }
    store::pack_rt::write_varint_u64(&mut out, s.next_label);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_brep_snapshot_binary(bytes: &[u8]) -> Result<SemioBrepSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let vertex_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut vertices = Vec::with_capacity(vertex_count as usize);
    for _ in 0..vertex_count {
        vertices.push(read_vertex(&mut reader)?);
    }
    let edge_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut edges = Vec::with_capacity(edge_count as usize);
    for _ in 0..edge_count {
        edges.push(read_edge(&mut reader)?);
    }
    let loop_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut loops = Vec::with_capacity(loop_count as usize);
    for _ in 0..loop_count {
        loops.push(read_loop(&mut reader)?);
    }
    let face_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut faces = Vec::with_capacity(face_count as usize);
    for _ in 0..face_count {
        faces.push(read_face(&mut reader)?);
    }
    let shell_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut shells = Vec::with_capacity(shell_count as usize);
    for _ in 0..shell_count {
        shells.push(read_shell(&mut reader)?);
    }
    let solid_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut solids = Vec::with_capacity(solid_count as usize);
    for _ in 0..solid_count {
        solids.push(read_solid(&mut reader)?);
    }
    let coedge_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut coedges = Vec::with_capacity(coedge_count as usize);
    for _ in 0..coedge_count {
        coedges.push(read_coedge(&mut reader)?);
    }
    let next_label = reader.read_varint_u64().map_err(|e| e.to_string())?;
    Ok(SemioBrepSnapshot { schema, vertices, edges, loops, faces, shells, solids, coedges, next_label })
}

impl store::ArtifactPack for SemioBrepSnapshot {
    /// 🪶️ Publishes the owned typed relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_brep_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let _ = options;
        decode_brep_snapshot_binary(&inner).map_err(|detail| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, detail)))
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};
use crate::audio::io::text::diff::{strip_brackets};
use crate::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;
use crate::standards::v1::subsets::brep::io::text::snapshot::*;
/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioBrepSnapshot`] —
/// the binary half of the same bridge, so a caller outside this crate can check the two codecs
/// against each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_brep_pack(bytes: &[u8]) -> Result<SemioBrepSnapshot, String> {
    <SemioBrepSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `store::ArtifactPack::encode_pack` inverse of [`decode_semio_brep_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_brep_pack(snapshot: &SemioBrepSnapshot) -> Vec<u8> {
    <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
