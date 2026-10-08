//! 📝️ Text representation codec surface for `stdio.semio.brep` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");



#[allow(unused_imports)]
mod diff_codec {
/// 📥️ The `pack::from_json_str` inverse of `SemioBrepDiff`'s `ToValue` — decodes the committed
/// `../🧬️mutations/<kind>/🧪️tests/<fixture>/🔺️diff/🔣️.json` specification vectors into a real
/// [`SemioBrepDiff`], mirroring `📸️snapshot/🦀️.rs`'s `decode_semio_brep_snapshot_json` and
/// `🧬️mutations/🦀️.rs`'s `decode_semio_brep_mutation_json` so no fixture test needs `serde_json`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_brep_diff_json(text: &str) -> Result<SemioBrepDiff, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

use super::*;
use crate::standards::v1::subsets::brep::io::binary::diff::{encode_option, decode_option};
use crate::standards::v1::subsets::brep::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_named_added, dec_named_triple, enc_named_added, enc_named_triple};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_face};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_face};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_edge};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_edge};
use crate::standards::v1::subsets::brep::io::text::snapshot::{dec_vertex};
use crate::standards::v1::subsets::brep::io::text::snapshot::{enc_vertex};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_brep_diff(d: &SemioBrepDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.vertices {
        tokens.push(format!("vertices={}", enc_named_triple(v, |k: &String| enc_str(k), enc_vertex_diff, |a| enc_named_added(a, enc_vertex))));
    }
    if let Some(v) = &d.edges {
        tokens.push(format!("edges={}", enc_named_triple(v, |k: &String| enc_str(k), enc_edge_diff, |a| enc_named_added(a, enc_edge))));
    }
    if let Some(v) = &d.loops {
        tokens.push(format!("loops={}", enc_named_triple(v, |k: &String| enc_str(k), enc_loop_diff, |a| enc_named_added(a, enc_loop))));
    }
    if let Some(v) = &d.faces {
        tokens.push(format!("faces={}", enc_named_triple(v, |k: &String| enc_str(k), enc_face_diff, |a| enc_named_added(a, enc_face))));
    }
    if let Some(v) = &d.shells {
        tokens.push(format!("shells={}", enc_named_triple(v, |k: &String| enc_str(k), enc_shell_diff, |a| enc_named_added(a, enc_shell))));
    }
    if let Some(v) = &d.solids {
        tokens.push(format!("solids={}", enc_named_triple(v, |k: &String| enc_str(k), enc_solid_diff, |a| enc_named_added(a, enc_solid))));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_brep_diff(line: &str) -> Result<SemioBrepDiff, String> {
    let mut d = SemioBrepDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("vertices=") {
            d.vertices = Some(dec_named_triple(rest, dec_str, dec_vertex_diff, |t| dec_named_added(t, dec_vertex))?);
        } else if let Some(rest) = token.strip_prefix("edges=") {
            d.edges = Some(dec_named_triple(rest, dec_str, dec_edge_diff, |t| dec_named_added(t, dec_edge))?);
        } else if let Some(rest) = token.strip_prefix("loops=") {
            d.loops = Some(dec_named_triple(rest, dec_str, dec_loop_diff, |t| dec_named_added(t, dec_loop))?);
        } else if let Some(rest) = token.strip_prefix("faces=") {
            d.faces = Some(dec_named_triple(rest, dec_str, dec_face_diff, |t| dec_named_added(t, dec_face))?);
        } else if let Some(rest) = token.strip_prefix("shells=") {
            d.shells = Some(dec_named_triple(rest, dec_str, dec_shell_diff, |t| dec_named_added(t, dec_shell))?);
        } else if let Some(rest) = token.strip_prefix("solids=") {
            d.solids = Some(dec_named_triple(rest, dec_str, dec_solid_diff, |t| dec_named_added(t, dec_solid))?);
        } else {
            return Err(format!("brep diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioBrepDiff {
fn print_diff(&self) -> String {
    print_brep_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_brep_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    crate::standards::v1::subsets::base::schema::geometry::native::parse(s)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", NativeF64(p.x), NativeF64(p.y), NativeF64(p.z))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

/// 📈️ `L[origin,direction]` / `C[center,axis,radius]` / `E[center,axis,radiusMajor,radiusMinor]` /
/// `N[controlPoints,weights,degree,knots]` — single-letter tag prefix, same convention as bcf's
/// `enc_camera`/svg's `enc_xml_node`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_curve(c: &BrepCurve) -> String {
    match c {
        BrepCurve::Line { origin, direction } => format!("L[{},{}]", enc_point3(origin), enc_point3(direction)),
        BrepCurve::Circle { center, axis, radius } => format!("C[{},{},{}]", enc_point3(center), enc_point3(axis), NativeF64(*radius)),
        BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => {
            format!("E[{},{},{},{}]", enc_point3(center), enc_point3(axis), NativeF64(*radius_major), NativeF64(*radius_minor))
        }
        BrepCurve::Nurbs { control_points, weights, degree, knots } => format!("N[{},{},{},{}]", enc_list(control_points, enc_point3), enc_list(weights, |w: &f64| NativeF64(*w).to_string()), degree, enc_list(knots, |k: &f64| NativeF64(*k).to_string()),),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_curve(s: &str) -> Result<BrepCurve, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "L" => {
            let [origin, direction] = parts.as_slice() else { return Err(format!("curve line: expected 2 fields, got {}", parts.len())) };
            Ok(BrepCurve::Line { origin: dec_point3(origin)?, direction: dec_point3(direction)? })
        }
        "C" => {
            let [center, axis, radius] = parts.as_slice() else { return Err(format!("curve circle: expected 3 fields, got {}", parts.len())) };
            Ok(BrepCurve::Circle { center: dec_point3(center)?, axis: dec_point3(axis)?, radius: parse_f64(radius)? })
        }
        "E" => {
            let [center, axis, radius_major, radius_minor] = parts.as_slice() else { return Err(format!("curve ellipse: expected 4 fields, got {}", parts.len())) };
            Ok(BrepCurve::Ellipse { center: dec_point3(center)?, axis: dec_point3(axis)?, radius_major: parse_f64(radius_major)?, radius_minor: parse_f64(radius_minor)? })
        }
        "N" => {
            let [control_points, weights, degree, knots] = parts.as_slice() else { return Err(format!("curve nurbs: expected 4 fields, got {}", parts.len())) };
            Ok(BrepCurve::Nurbs { control_points: dec_list(control_points, dec_point3)?, weights: dec_list(weights, parse_f64)?, degree: parse_u32(degree)?, knots: dec_list(knots, parse_f64)? })
        }
        other => Err(format!("curve: unknown tag {other:?}")),
    }
}

/// 🗺️ `P[origin,normal]` / `C[origin,axis,radius]` (cylinder) / `O[origin,axis,radius,halfAngle]`
/// (cone) / `S[center,radius]` (sphere) / `T[center,axis,majorRadius,minorRadius]` (torus) /
/// `N[controlPoints,weights,uCount,vCount,degreeU,degreeV,knotsU,knotsV]`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_surface(s: &BrepSurface) -> String {
    match s {
        BrepSurface::Plane { origin, normal } => format!("P[{},{}]", enc_point3(origin), enc_point3(normal)),
        BrepSurface::Cylinder { origin, axis, radius } => format!("C[{},{},{}]", enc_point3(origin), enc_point3(axis), NativeF64(*radius)),
        BrepSurface::Cone { origin, axis, radius, half_angle } => format!("O[{},{},{},{}]", enc_point3(origin), enc_point3(axis), NativeF64(*radius), NativeF64(*half_angle)),
        BrepSurface::Sphere { center, radius } => format!("S[{},{}]", enc_point3(center), NativeF64(*radius)),
        BrepSurface::Torus { center, axis, major_radius, minor_radius } => format!("T[{},{},{},{}]", enc_point3(center), enc_point3(axis), NativeF64(*major_radius), NativeF64(*minor_radius)),
        BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => format!(
            "N[{},{},{},{},{},{},{},{}]",
            enc_list(control_points, enc_point3),
            enc_list(weights, |w: &f64| NativeF64(*w).to_string()),
            u_count,
            v_count,
            degree_u,
            degree_v,
            enc_list(knots_u, |k: &f64| NativeF64(*k).to_string()),
            enc_list(knots_v, |k: &f64| NativeF64(*k).to_string()),
        ),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_surface(s: &str) -> Result<BrepSurface, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "P" => {
            let [origin, normal] = parts.as_slice() else { return Err(format!("surface plane: expected 2 fields, got {}", parts.len())) };
            Ok(BrepSurface::Plane { origin: dec_point3(origin)?, normal: dec_point3(normal)? })
        }
        "C" => {
            let [origin, axis, radius] = parts.as_slice() else { return Err(format!("surface cylinder: expected 3 fields, got {}", parts.len())) };
            Ok(BrepSurface::Cylinder { origin: dec_point3(origin)?, axis: dec_point3(axis)?, radius: parse_f64(radius)? })
        }
        "O" => {
            let [origin, axis, radius, half_angle] = parts.as_slice() else { return Err(format!("surface cone: expected 4 fields, got {}", parts.len())) };
            Ok(BrepSurface::Cone { origin: dec_point3(origin)?, axis: dec_point3(axis)?, radius: parse_f64(radius)?, half_angle: parse_f64(half_angle)? })
        }
        "S" => {
            let [center, radius] = parts.as_slice() else { return Err(format!("surface sphere: expected 2 fields, got {}", parts.len())) };
            Ok(BrepSurface::Sphere { center: dec_point3(center)?, radius: parse_f64(radius)? })
        }
        "T" => {
            let [center, axis, major_radius, minor_radius] = parts.as_slice() else { return Err(format!("surface torus: expected 4 fields, got {}", parts.len())) };
            Ok(BrepSurface::Torus { center: dec_point3(center)?, axis: dec_point3(axis)?, major_radius: parse_f64(major_radius)?, minor_radius: parse_f64(minor_radius)? })
        }
        "N" => {
            let [control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v] = parts.as_slice() else {
                return Err(format!("surface nurbs: expected 8 fields, got {}", parts.len()));
            };
            Ok(BrepSurface::Nurbs {
                control_points: dec_list(control_points, dec_point3)?,
                weights: dec_list(weights, parse_f64)?,
                u_count: parse_u32(u_count)?,
                v_count: parse_u32(v_count)?,
                degree_u: parse_u32(degree_u)?,
                degree_v: parse_u32(degree_v)?,
                knots_u: dec_list(knots_u, parse_f64)?,
                knots_v: dec_list(knots_v, parse_f64)?,
            })
        }
        other => Err(format!("surface: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_loop_edge(le: &BrepLoopEdge) -> String {
    format!("[{},{}]", enc_str(&le.edge), enc_bool(le.orientation))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_loop_edge(s: &str) -> Result<BrepLoopEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [edge, orientation] = parts.as_slice() else { return Err(format!("loop edge: expected 2 fields, got {}", parts.len())) };
    Ok(BrepLoopEdge { edge: dec_str(edge)?, orientation: parse_bool(orientation)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_shell_face(sf: &BrepShellFace) -> String {
    format!("[{},{}]", enc_str(&sf.face), enc_bool(sf.orientation))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_shell_face(s: &str) -> Result<BrepShellFace, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [face, orientation] = parts.as_slice() else { return Err(format!("shell face: expected 2 fields, got {}", parts.len())) };
    Ok(BrepShellFace { face: dec_str(face)?, orientation: parse_bool(orientation)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_solid_shell(ss: &BrepSolidShell) -> String {
    format!("[{},{}]", enc_str(&ss.shell), enc_bool(ss.is_void))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_solid_shell(s: &str) -> Result<BrepSolidShell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [shell, is_void] = parts.as_slice() else { return Err(format!("solid shell: expected 2 fields, got {}", parts.len())) };
    Ok(BrepSolidShell { shell: dec_str(shell)?, is_void: parse_bool(is_void)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_loop(l: &BrepLoop) -> String {
    format!("[{},{}]", enc_str(&l.id), enc_list(&l.edges, enc_loop_edge))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_loop(s: &str) -> Result<BrepLoop, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, edges] = parts.as_slice() else { return Err(format!("loop: expected 2 fields, got {}", parts.len())) };
    Ok(BrepLoop { id: dec_str(id)?, edges: dec_list(edges, dec_loop_edge)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_shell(sh: &BrepShell) -> String {
    format!("[{},{}]", enc_str(&sh.id), enc_list(&sh.faces, enc_shell_face))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_shell(s: &str) -> Result<BrepShell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, faces] = parts.as_slice() else { return Err(format!("shell: expected 2 fields, got {}", parts.len())) };
    Ok(BrepShell { id: dec_str(id)?, faces: dec_list(faces, dec_shell_face)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_solid(so: &BrepSolid) -> String {
    format!("[{},{}]", enc_str(&so.id), enc_list(&so.shells, enc_solid_shell))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_solid(s: &str) -> Result<BrepSolid, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, shells] = parts.as_slice() else { return Err(format!("solid: expected 2 fields, got {}", parts.len())) };
    Ok(BrepSolid { id: dec_str(id)?, shells: dec_list(shells, dec_solid_shell)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_vertex_diff(d: &BrepVertexDiff) -> String {
    format!("[{},{}]", encode_option(&d.point, enc_point3), encode_option(&d.tol, |v: &f64| NativeF64(*v).to_string()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_vertex_diff(s: &str) -> Result<BrepVertexDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [point, tol] = parts.as_slice() else { return Err(format!("vertex diff: expected 2 fields, got {}", parts.len())) };
    Ok(BrepVertexDiff { point: decode_option(point, dec_point3)?, tol: decode_option(tol, parse_f64)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_edge_diff(d: &BrepEdgeDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.start_vertex, |v: &String| enc_str(v)), encode_option(&d.end_vertex, |v: &String| enc_str(v)), encode_option(&d.curve, enc_curve), encode_option(&d.tol, |v: &f64| NativeF64(*v).to_string()))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_edge_diff(s: &str) -> Result<BrepEdgeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [start_vertex, end_vertex, curve, tol] = parts.as_slice() else { return Err(format!("edge diff: expected 4 fields, got {}", parts.len())) };
    Ok(BrepEdgeDiff { start_vertex: decode_option(start_vertex, dec_str)?, end_vertex: decode_option(end_vertex, dec_str)?, curve: decode_option(curve, dec_curve)?, tol: decode_option(tol, parse_f64)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_loop_diff(d: &BrepLoopDiff) -> String {
    format!("[{}]", encode_option(&d.edges, |v: &Vec<BrepLoopEdge>| enc_list(v, enc_loop_edge)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_loop_diff(s: &str) -> Result<BrepLoopDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BrepLoopDiff { edges: decode_option(inner, |s| dec_list(s, dec_loop_edge))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_face_diff(d: &BrepFaceDiff) -> String {
    format!(
        "[{},{},{},{},{}]",
        encode_option(&d.outer_loop, |v: &String| enc_str(v)),
        encode_option(&d.inner_loops, |v: &Vec<String>| enc_list(v, |s: &String| crate::standards::v1::subsets::brep::io::text::snapshot::enc_loop_id(s))),
        encode_option(&d.surface, enc_surface),
        encode_option(&d.orientation, |b: &bool| enc_bool(*b).to_string()),
        encode_option(&d.tol, |v: &f64| NativeF64(*v).to_string()),
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_face_diff(s: &str) -> Result<BrepFaceDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [outer_loop, inner_loops, surface, orientation, tol] = parts.as_slice() else { return Err(format!("face diff: expected 5 fields, got {}", parts.len())) };
    Ok(BrepFaceDiff {
        outer_loop: decode_option(outer_loop, dec_str)?,
        inner_loops: decode_option(inner_loops, |s| dec_list(s, crate::standards::v1::subsets::brep::io::text::snapshot::dec_loop_id))?,
        surface: decode_option(surface, dec_surface)?,
        orientation: decode_option(orientation, parse_bool)?,
        tol: decode_option(tol, parse_f64)?,
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_shell_diff(d: &BrepShellDiff) -> String {
    format!("[{}]", encode_option(&d.faces, |v: &Vec<BrepShellFace>| enc_list(v, enc_shell_face)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_shell_diff(s: &str) -> Result<BrepShellDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BrepShellDiff { faces: decode_option(inner, |s| dec_list(s, dec_shell_face))? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_solid_diff(d: &BrepSolidDiff) -> String {
    format!("[{}]", encode_option(&d.shells, |v: &Vec<BrepSolidShell>| enc_list(v, enc_solid_shell)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_solid_diff(s: &str) -> Result<BrepSolidDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BrepSolidDiff { shells: decode_option(inner, |s| dec_list(s, dec_solid_shell))? })
}
}
pub use diff_codec::*;
