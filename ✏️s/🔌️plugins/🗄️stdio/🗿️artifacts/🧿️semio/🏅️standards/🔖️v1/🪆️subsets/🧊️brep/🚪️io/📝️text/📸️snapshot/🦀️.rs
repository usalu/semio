//! 📝️ Text representation codec surface for `stdio.semio.brep` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SemioBrepSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;

/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION brep wave (following the
/// flow pilot's proven template, `ws-codec-workflow-report.md`): real hex/bracket-encoded
/// value primitives backing the hand-rolled `ArtifactDsl` below — same style as this subset's own
/// `🔺️diff`/`🧬️mutations` facets, duplicated here (not imported from `schema::diff`) to keep
/// `snapshot` — the base type `diff`/`mutations` both depend ON — free of a reverse dependency on
/// either sibling facet.
///
/// 🧩️ The `#[derive(dsl::DslArtifact)]` path was reconsidered per this ticket's brief now that the
/// 6 shared `⚙️engine/🧮️geometry` value types (incl. `SemioPoint3`) derive `dsl::DslRecord`. It is
/// still blocked here for a DIFFERENT, new reason than flow's: `BrepCurve`/`BrepSurface` are
/// data-carrying TAGGED ENUMS (`Line`/`Circle`/`Ellipse`/`Nurbs`, `Plane`/`Cylinder`/.../`Nurbs`),
/// several of whose variants hold `Vec<SemioPoint3>`/`Vec<f64>` fields — the derive path has no
/// `DslEnum`-over-heterogeneous-payload-shape mechanism proven to emit a matching TEXT production
/// for a tagged union whose variants carry different field SETS (as opposed to `DslVariants`' one-
/// spec-per-variant binary-only scheme). Hand-rolled instead, matching the established hex/bracket
/// convention this subset's own `🔺️diff` facet already uses for exactly these two enums.
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
/// `N[controlPoints,weights,degree,knots]` — single-letter tag prefix, same convention this
/// subset's own `🔺️diff/🦀️.rs`'s `enc_curve` uses (duplicated here, field-for-field).
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
pub(crate) fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", NativeF64(p.x), NativeF64(p.y))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}

/// 🗺️➰️ `L[origin,direction]` / `C[center,radius]` / `E[center,xAxis,radiusMajor,radiusMinor]` /
/// `N[controlPoints,weights,degree,knots]` — same convention as [`enc_curve`], one dimension down.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_curve2(c: &BrepCurve2) -> String {
    match c {
        BrepCurve2::Line { origin, direction } => format!("L[{},{}]", enc_point2(origin), enc_point2(direction)),
        BrepCurve2::Circle { center, radius } => format!("C[{},{}]", enc_point2(center), NativeF64(*radius)),
        BrepCurve2::Ellipse { center, x_axis, radius_major, radius_minor } => format!("E[{},{},{},{}]", enc_point2(center), enc_point2(x_axis), NativeF64(*radius_major), NativeF64(*radius_minor)),
        BrepCurve2::Nurbs { control_points, weights, degree, knots } => format!("N[{},{},{},{}]", enc_list(control_points, enc_point2), enc_list(weights, |w: &f64| NativeF64(*w).to_string()), degree, enc_list(knots, |k: &f64| NativeF64(*k).to_string()),),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_curve2(s: &str) -> Result<BrepCurve2, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "L" => {
            let [origin, direction] = parts.as_slice() else { return Err(format!("curve2 line: expected 2 fields, got {}", parts.len())) };
            Ok(BrepCurve2::Line { origin: dec_point2(origin)?, direction: dec_point2(direction)? })
        }
        "C" => {
            let [center, radius] = parts.as_slice() else { return Err(format!("curve2 circle: expected 2 fields, got {}", parts.len())) };
            Ok(BrepCurve2::Circle { center: dec_point2(center)?, radius: parse_f64(radius)? })
        }
        "E" => {
            let [center, x_axis, radius_major, radius_minor] = parts.as_slice() else { return Err(format!("curve2 ellipse: expected 4 fields, got {}", parts.len())) };
            Ok(BrepCurve2::Ellipse { center: dec_point2(center)?, x_axis: dec_point2(x_axis)?, radius_major: parse_f64(radius_major)?, radius_minor: parse_f64(radius_minor)? })
        }
        "N" => {
            let [control_points, weights, degree, knots] = parts.as_slice() else { return Err(format!("curve2 nurbs: expected 4 fields, got {}", parts.len())) };
            Ok(BrepCurve2::Nurbs { control_points: dec_list(control_points, dec_point2)?, weights: dec_list(weights, parse_f64)?, degree: parse_u32(degree)?, knots: dec_list(knots, parse_f64)? })
        }
        other => Err(format!("curve2: unknown tag {other:?}")),
    }
}

/// 🌀️ `[hex-tag]` around `Some`'s inner encoding, or the literal token `-` for `None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_opt_curve2(c: &Option<BrepCurve2>) -> String {
    match c {
        Some(curve) => format!("~{}", enc_curve2(curve)),
        None => "-".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_opt_curve2(s: &str) -> Result<Option<BrepCurve2>, String> {
    if s == "-" {
        return Ok(None);
    }
    let rest = s.strip_prefix('~').ok_or_else(|| format!("optional curve2: expected '-' or a '~'-prefixed curve, got {s:?}"))?;
    Ok(Some(dec_curve2(rest)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_prange(r: &(f64, f64)) -> String {
    format!("[{},{}]", NativeF64(r.0), NativeF64(r.1))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_prange(s: &str) -> Result<(f64, f64), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [a, b] = parts.as_slice() else { return Err(format!("prange: expected 2 fields, got {}", parts.len())) };
    Ok((parse_f64(a)?, parse_f64(b)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_coedge(c: &BrepCoedge) -> String {
    format!("[{},{},{},{},{},{},{},{}]", enc_str(&c.id), enc_str(&c.edge), enc_bool(c.forward), enc_opt_curve2(&c.pcurve), enc_prange(&c.prange), enc_str(&c.loop_id), enc_str(&c.next), enc_str(&c.prev))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_coedge(s: &str) -> Result<BrepCoedge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, edge, forward, pcurve, prange, loop_id, next, prev] = parts.as_slice() else { return Err(format!("coedge: expected 8 fields, got {}", parts.len())) };
    Ok(BrepCoedge { id: dec_str(id)?, edge: dec_str(edge)?, forward: parse_bool(forward)?, pcurve: dec_opt_curve2(pcurve)?, prange: dec_prange(prange)?, loop_id: dec_str(loop_id)?, next: dec_str(next)?, prev: dec_str(prev)? })
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
pub(in crate::standards::v1::subsets::brep::io) fn enc_vertex(v: &BrepVertex) -> String {
    format!("[{},{},{}]", enc_str(&v.id), enc_point3(&v.point), NativeF64(v.tol))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(in crate::standards::v1::subsets::brep::io) fn dec_vertex(s: &str) -> Result<BrepVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, point, tol] = parts.as_slice() else { return Err(format!("vertex: expected 3 fields, got {}", parts.len())) };
    Ok(BrepVertex { id: dec_str(id)?, point: dec_point3(point)?, tol: parse_f64(tol)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(in crate::standards::v1::subsets::brep::io) fn enc_edge(e: &BrepEdge) -> String {
    format!("[{},{},{},{},{}]", enc_str(&e.id), enc_str(&e.start_vertex), enc_str(&e.end_vertex), enc_curve(&e.curve), NativeF64(e.tol))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(in crate::standards::v1::subsets::brep::io) fn dec_edge(s: &str) -> Result<BrepEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, start_vertex, end_vertex, curve, tol] = parts.as_slice() else { return Err(format!("edge: expected 5 fields, got {}", parts.len())) };
    Ok(BrepEdge { id: dec_str(id)?, start_vertex: dec_str(start_vertex)?, end_vertex: dec_str(end_vertex)?, curve: dec_curve(curve)?, tol: parse_f64(tol)? })
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
pub(crate) fn enc_loop_id(value:&str)->String{format!("[{}]",enc_str(value))}

pub(crate) fn dec_loop_id(value:&str)->Result<String,String>{dec_str(strip_brackets(value)?)}

pub(in crate::standards::v1::subsets::brep::io) fn enc_face(f: &BrepFace) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&f.id), enc_str(&f.outer_loop), enc_list(&f.inner_loops, |s: &String| enc_loop_id(s)), enc_surface(&f.surface), enc_bool(f.orientation), NativeF64(f.tol))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(in crate::standards::v1::subsets::brep::io) fn dec_face(s: &str) -> Result<BrepFace, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, outer_loop, inner_loops, surface, orientation, tol] = parts.as_slice() else { return Err(format!("face: expected 6 fields, got {}", parts.len())) };
    Ok(BrepFace { id: dec_str(id)?, outer_loop: dec_str(outer_loop)?, inner_loops: dec_list(inner_loops, dec_loop_id)?, surface: dec_surface(surface)?, orientation: parse_bool(orientation)?, tol: parse_f64(tol)? })
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

/// 📄️ The real structured text body: seven lines — `schema=<hex>`, `vertices=[...]`,
/// `edges=[...]`, `loops=[...]`, `faces=[...]`, `shells=[...]`, `solids=[...]` — matching the
/// grammar's `document = artifact-mark schema-line vertices-line edges-line loops-line faces-line
/// shells-line solids-line`. Newlines are pure lexer trivia in the shared dialect, so this is
/// genuinely recognizable by `dsl::Recognizer`, not merely readable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_brep_snapshot_body(s: &SemioBrepSnapshot) -> String {
    format!(
        "schema={}\nvertices=[{}]\nedges=[{}]\nloops=[{}]\nfaces=[{}]\nshells=[{}]\nsolids=[{}]\ncoedges=[{}]\nnextLabel={}",
        enc_str(&s.schema),
        s.vertices.iter().map(enc_vertex).collect::<Vec<_>>().join(","),
        s.edges.iter().map(enc_edge).collect::<Vec<_>>().join(","),
        s.loops.iter().map(enc_loop).collect::<Vec<_>>().join(","),
        s.faces.iter().map(enc_face).collect::<Vec<_>>().join(","),
        s.shells.iter().map(enc_shell).collect::<Vec<_>>().join(","),
        s.solids.iter().map(enc_solid).collect::<Vec<_>>().join(","),
        s.coedges.iter().map(enc_coedge).collect::<Vec<_>>().join(","),
        s.next_label,
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_brep_snapshot_body(body: &str) -> Result<SemioBrepSnapshot, String> {
    let mut schema = None;
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut loops = Vec::new();
    let mut faces = Vec::new();
    let mut shells = Vec::new();
    let mut solids = Vec::new();
    let mut coedges = Vec::new();
    let mut next_label = 0u64;
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("vertices=") {
            vertices = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_vertex).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("edges=") {
            edges = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_edge).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("loops=") {
            loops = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_loop).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("faces=") {
            faces = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_face).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("shells=") {
            shells = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_shell).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("solids=") {
            solids = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_solid).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("coedges=") {
            coedges = split_top_level(strip_brackets(rest)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_coedge).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("nextLabel=") {
            next_label = rest.parse::<u64>().map_err(|e| e.to_string())?;
        } else {
            return Err(format!("brep snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "brep snapshot: missing schema line".to_string())?;
    Ok(SemioBrepSnapshot { schema, vertices, edges, loops, faces, shells, solids, coedges, next_label })
}

/// 🎁 Real structured text/binary codecs (brep wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow pilot's proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioBrepSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOBREP_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_brep_snapshot_body(body).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_brep_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📤️ This subset's own `#[value(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.brep` — the shape `🧊️mutate-semio-brep` compares under `ordered-json-v1`, derived from the
/// snapshot type itself rather than hand-written a second time in the adapter, where it could drift
/// away from the type it claims to project. The projection is not flat: `BrepCurve` and `BrepSurface` are `#[value(tag = "kind",
/// rename_all = "camelCase")]` enums, so every edge carries a discriminated `curve` object and every
/// face a discriminated `surface` one — a shape no hand-written adapter projection would reproduce
/// reliably by eye.
/// A thin `pack::to_json_string` wrapper (first-party, over `ToValue`/`DslValue`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_brep_snapshot_json(snapshot: &SemioBrepSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_brep_snapshot_json`] — decodes the
/// committed `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioBrepSnapshot`] values, so `🧊️mutate-semio-brep`'s adapter reads the
/// committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_brep_snapshot_json(text: &str) -> Result<SemioBrepSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📥️ Parses this subset's own committed `.dsl.semio` text into a real [`SemioBrepSnapshot`] — a
/// thin wrapper over `store::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `🧊️mutate-semio-brep` test adapter, whose
/// `identity-round-trip` scenario reads the REAL committed `📚️examples/🧊️solid` artifact) can still
/// drive the same codec production does. Same shape and same rationale as `🌊️flow`'s own bridge.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_brep_dsl(text: &str) -> Result<SemioBrepSnapshot, String> {
    <SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactDsl::print_dsl` inverse of [`parse_semio_brep_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_brep_dsl(snapshot: &SemioBrepSnapshot) -> String {
    <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1::subsets::brep::schema::snapshot::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;




}
pub use snapshot_wire_codec::*;
