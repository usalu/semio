//! 🧬️ SemioBrepSnapshot — id-keyed b-rep topology graph (vertices/edges/loops/faces/shells/
//! solids) with typed `BrepSurface`/`BrepCurve` value enums. Informed by step's
//! `⚙️engine/🧱️brep` analyzer view (`BrepMesh`: vertices + polygon faces) and `StepSnapshot`'s
//! generic Part-21 entity graph (`CARTESIAN_POINT`/`VERTEX_POINT`/`EDGE_CURVE`/`ORIENTED_EDGE`/
//! `EDGE_LOOP`/`FACE_BOUND`/`ADVANCED_FACE`/`CLOSED_SHELL`/`MANIFOLD_SOLID_BREP`) — this snapshot
//! generalizes that analyzer's planar-only mesh into a full typed b-rep: every edge carries its
//! own curve (not just a straight-line control polygon) and every face its own surface (not just
//! `PLANE`), matching AP214's real vocabulary of surface/curve kinds.

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};
use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{split_top_level, strip_brackets};
use schema::ArtifactSchema;

//#region 🔖️Ids
pub const STDIO_SEMIOBREP_DOCUMENT_SCHEMA: &str = "stdio.semio.brep";
//#endregion 🔖️Ids

//#region 🔖️Curve
/// 📈️ A b-rep edge's underlying 3D curve. Owned by `brep` (`w1b-type-ownership.md`).
///
/// 🔣️ Unlike `serde`, this derive's own `rename_all` already applies to BOTH the variant name AND
/// every struct-variant member name (see `🌱️value/✨️derive`'s module docs) — no separate
/// `rename_all_fields` needed; `radiusMajor`/`controlPoints` etc. come out correctly from
/// `rename_all = "camelCase"` alone.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum BrepCurve {
    Line {
        origin: SemioPoint3,
        direction: SemioPoint3,
    },
    Circle {
        center: SemioPoint3,
        axis: SemioPoint3,
        radius: f64,
    },
    Ellipse {
        center: SemioPoint3,
        axis: SemioPoint3,
        radius_major: f64,
        radius_minor: f64,
    },
    /// 🎛️ Rational B-spline curve: a flat `control_points`/`weights` run alongside `knots`
    /// (length `control_points.len() + degree + 1`, per the standard open-uniform-or-not knot
    /// vector convention) — no nested fixed arrays, per the f6 §4.3 gap.
    Nurbs {
        control_points: Vec<SemioPoint3>,
        weights: Vec<f64>,
        degree: u32,
        knots: Vec<f64>,
    },
}

/// 🩹️ Needed ONLY so `BrepEdge`/entity structs can derive `Default` (in turn needed only because
/// `serde_derive`'s `#[value(default)]` on a `Vec<T>` field spuriously infers `T: Default` for the
/// SHARED `🧰️triples::NamedTripleDiff<K,D,T>`'s `added: Vec<T>` — see the "shared infra gaps" note
/// in the wave report; never constructed as a meaningful default in real code paths.
impl Default for BrepCurve {
    fn default() -> Self {
        BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3::default() }
    }
}

/// 🗺️➰️ A p-curve: a coedge's edge, reparametrized into its owning face's `(u, v)` domain — the
/// 2D twin of [`BrepCurve`], same variant vocabulary, matching the native kernel's `Curve2`
/// (`📸️snapshot/➰️curve/🦀️.rs`) field-for-field so [`Body::to_snapshot`]/[`Body::from_snapshot`]
/// (`📸️snapshot/🔁️body/🦀️.rs`) round-trip it exactly, never approximated.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum BrepCurve2 {
    Line {
        origin: SemioPoint2,
        direction: SemioPoint2,
    },
    Circle {
        center: SemioPoint2,
        radius: f64,
    },
    Ellipse {
        center: SemioPoint2,
        x_axis: SemioPoint2,
        radius_major: f64,
        radius_minor: f64,
    },
    Nurbs {
        control_points: Vec<SemioPoint2>,
        weights: Vec<f64>,
        degree: u32,
        knots: Vec<f64>,
    },
}

/// 🩹️ See `BrepCurve`'s `Default` impl doc comment — same reason (needed only so `BrepCoedge` can
/// derive `Default`, never a meaningful default in real code paths).
impl Default for BrepCurve2 {
    fn default() -> Self {
        BrepCurve2::Line { origin: SemioPoint2::default(), direction: SemioPoint2::default() }
    }
}
//#endregion 🔖️Curve

//#region 🔖️Surface
/// 🗺️ A b-rep face's underlying surface. Owned by `brep`.
///
/// 🔣️ Same story as `BrepCurve` above — this derive's `rename_all` already covers struct-variant
/// member names, so a bare `rename_all = "camelCase"` is enough.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum BrepSurface {
    Plane {
        origin: SemioPoint3,
        normal: SemioPoint3,
    },
    Cylinder {
        origin: SemioPoint3,
        axis: SemioPoint3,
        radius: f64,
    },
    Cone {
        origin: SemioPoint3,
        axis: SemioPoint3,
        radius: f64,
        half_angle: f64,
    },
    Sphere {
        center: SemioPoint3,
        radius: f64,
    },
    Torus {
        center: SemioPoint3,
        axis: SemioPoint3,
        major_radius: f64,
        minor_radius: f64,
    },
    /// 🎛️ Rational B-spline surface: `control_points` is a flat `u_count * v_count` row-major
    /// grid (never a nested `Vec<Vec<_>>`/fixed array — f6 §4.3).
    Nurbs {
        control_points: Vec<SemioPoint3>,
        weights: Vec<f64>,
        u_count: u32,
        v_count: u32,
        degree_u: u32,
        degree_v: u32,
        knots_u: Vec<f64>,
        knots_v: Vec<f64>,
    },
}

/// 🩹️ See `BrepCurve`'s `Default` impl doc comment — same reason.
impl Default for BrepSurface {
    fn default() -> Self {
        BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3::default() }
    }
}
//#endregion 🔖️Surface

//#region 🔖️Topology
/// 📍️ A b-rep vertex — corresponds to STEP's `VERTEX_POINT`/`CARTESIAN_POINT` pair collapsed
/// into one id-keyed entity.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepVertex {
    pub id: String,
    pub point: SemioPoint3,
    /// 🎚️ Native `Vertex::tol` (containment ball radius, model units) — `0.0` (the Rust default)
    /// means "unspecified"; [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::body::Body::from_snapshot`]
    /// treats `<= 0.0` as "use the kernel default" rather than a literal zero tolerance, so
    /// pre-this-wave fixture JSON (missing this field) still reconstructs a valid `Body`.
    #[value(default)]
    pub tol: f64,
}

/// ➡️ A b-rep edge — corresponds to STEP's `EDGE_CURVE`, always resolved between two vertices.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepEdge {
    pub id: String,
    pub start_vertex: String,
    pub end_vertex: String,
    pub curve: BrepCurve,
    /// 🎚️ Native `Edge::tol` (tube radius, model units) — same "`<= 0.0` means unspecified"
    /// convention as [`BrepVertex::tol`].
    #[value(default)]
    pub tol: f64,
}

/// 🔁️ A loop-member reference to an edge, carrying the traversal orientation — STEP's
/// `ORIENTED_EDGE.orientation`. A named weak struct, never a bare `(String, bool)` tuple.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepLoopEdge {
    pub edge: String,
    pub orientation: bool,
}

/// ⭕️ A closed edge loop — corresponds to STEP's `EDGE_LOOP`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepLoop {
    pub id: String,
    #[value(default)]
    pub edges: Vec<BrepLoopEdge>,
}

/// 🧱️ A b-rep coedge — one face's directed USE of one edge within one loop, matching the native
/// kernel's `Coedge` (`📸️snapshot/🕸️topology/🦀️.rs`) field-for-field: `edge`/`forward` duplicate
/// `BrepLoopEdge`'s `edge`/`orientation` (kept as a SEPARATE, purely-additive top-level collection
/// — `SemioBrepSnapshot::coedges` — rather than widening `BrepLoopEdge` itself, so every existing
/// producer of `BrepLoopEdge` literals, in this facet's own STEP im/export siblings included,
/// keeps compiling unchanged), plus the two fields `BrepLoopEdge` cannot carry: the p-curve
/// (`pcurve`/`prange`, `None`/`(0.0, 0.0)` when the producer has not stored one) and the coedge's
/// position in its loop's ring (`loop_id`, `next`, `prev` — ids into this same collection,
/// matching native `Coedge::{loop_id,next,prev}`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepCoedge {
    pub id: String,
    pub edge: String,
    pub forward: bool,
    #[value(default)]
    pub pcurve: Option<BrepCurve2>,
    #[value(default)]
    pub prange: (f64, f64),
    pub loop_id: String,
    pub next: String,
    pub prev: String,
}

/// 🔺️ A b-rep face — corresponds to STEP's `ADVANCED_FACE`, bounded by one outer loop and zero
/// or more inner (hole) loops, over a typed surface.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepFace {
    pub id: String,
    pub outer_loop: String,
    #[value(default)]
    pub inner_loops: Vec<String>,
    pub surface: BrepSurface,
    pub orientation: bool,
    /// 🎚️ Native `Face::tol` (shell thickness, model units) — same "`<= 0.0` means unspecified"
    /// convention as [`BrepVertex::tol`].
    #[value(default)]
    pub tol: f64,
}

/// 🔁️ A shell-member reference to a face, carrying orientation — STEP's face-in-shell sense.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepShellFace {
    pub face: String,
    pub orientation: bool,
}

/// 🐚️ A closed (or open) shell — corresponds to STEP's `CLOSED_SHELL`/`OPEN_SHELL`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepShell {
    pub id: String,
    #[value(default)]
    pub faces: Vec<BrepShellFace>,
}

/// 🔁️ A solid-member reference to a shell, flagging whether it bounds a void (an internal
/// cavity) rather than the solid's outer boundary — STEP's `MANIFOLD_SOLID_BREP.voids`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepSolidShell {
    pub shell: String,
    pub is_void: bool,
}

/// 🧊️ A manifold solid — corresponds to STEP's `MANIFOLD_SOLID_BREP`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepSolid {
    pub id: String,
    #[value(default)]
    pub shells: Vec<BrepSolidShell>,
}
//#endregion 🔖️Topology

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep")]
pub struct SemioBrepSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub vertices: Vec<BrepVertex>,
    #[state(artifact)]
    #[value(default)]
    pub edges: Vec<BrepEdge>,
    #[state(artifact)]
    #[value(default)]
    pub loops: Vec<BrepLoop>,
    #[state(artifact)]
    #[value(default)]
    pub faces: Vec<BrepFace>,
    #[state(artifact)]
    #[value(default)]
    pub shells: Vec<BrepShell>,
    #[state(artifact)]
    #[value(default)]
    pub solids: Vec<BrepSolid>,
    /// 🧱️ First-class coedges — see [`BrepCoedge`]'s own doc comment for why this is a separate
    /// collection rather than a widened `BrepLoopEdge`. Empty for every snapshot produced before
    /// this field existed (STEP import, hand-authored fixtures): [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::body::Body::from_snapshot`]
    /// falls back to reconstructing coedges from `BrepLoop.edges` (no pcurve) when this is empty.
    #[state(artifact)]
    #[value(default)]
    pub coedges: Vec<BrepCoedge>,
    /// 📜️ The native `Body::labels`/`LabelSource` high-water mark — MUST be carried forward
    /// (never reset to 0) across a `to_snapshot`/`from_snapshot` round trip so two independent
    /// mutation constructions against the same document never mint colliding persistent labels
    /// (see `topology::history::LabelSource`'s own doc comment). `0` for any snapshot produced
    /// before this field existed, meaning "mint fresh labels for everything" — safe because such a
    /// snapshot has no persistent-label history to preserve in the first place.
    #[state(artifact)]
    #[value(default)]
    pub next_label: u64,
}

impl Default for SemioBrepSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices: Default::default(), edges: Default::default(), loops: Default::default(), faces: Default::default(), shells: Default::default(), solids: Default::default(), coedges: Default::default(), next_label: 0 }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives
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
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

/// 📈️ `L[origin,direction]` / `C[center,axis,radius]` / `E[center,axis,radiusMajor,radiusMinor]` /
/// `N[controlPoints,weights,degree,knots]` — single-letter tag prefix, same convention this
/// subset's own `🔺️diff/🦀️.rs`'s `enc_curve` uses (duplicated here, field-for-field).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_curve(c: &BrepCurve) -> String {
    match c {
        BrepCurve::Line { origin, direction } => format!("L[{},{}]", enc_point3(origin), enc_point3(direction)),
        BrepCurve::Circle { center, axis, radius } => format!("C[{},{},{}]", enc_point3(center), enc_point3(axis), radius),
        BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => {
            format!("E[{},{},{},{}]", enc_point3(center), enc_point3(axis), radius_major, radius_minor)
        }
        BrepCurve::Nurbs { control_points, weights, degree, knots } => format!("N[{},{},{},{}]", enc_list(control_points, enc_point3), enc_list(weights, |w: &f64| w.to_string()), degree, enc_list(knots, |k: &f64| k.to_string()),),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_curve(s: &str) -> Result<BrepCurve, String> {
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
fn enc_surface(s: &BrepSurface) -> String {
    match s {
        BrepSurface::Plane { origin, normal } => format!("P[{},{}]", enc_point3(origin), enc_point3(normal)),
        BrepSurface::Cylinder { origin, axis, radius } => format!("C[{},{},{}]", enc_point3(origin), enc_point3(axis), radius),
        BrepSurface::Cone { origin, axis, radius, half_angle } => format!("O[{},{},{},{}]", enc_point3(origin), enc_point3(axis), radius, half_angle),
        BrepSurface::Sphere { center, radius } => format!("S[{},{}]", enc_point3(center), radius),
        BrepSurface::Torus { center, axis, major_radius, minor_radius } => format!("T[{},{},{},{}]", enc_point3(center), enc_point3(axis), major_radius, minor_radius),
        BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => format!(
            "N[{},{},{},{},{},{},{},{}]",
            enc_list(control_points, enc_point3),
            enc_list(weights, |w: &f64| w.to_string()),
            u_count,
            v_count,
            degree_u,
            degree_v,
            enc_list(knots_u, |k: &f64| k.to_string()),
            enc_list(knots_v, |k: &f64| k.to_string()),
        ),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_surface(s: &str) -> Result<BrepSurface, String> {
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
fn enc_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", p.x, p.y)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_point2(s: &str) -> Result<SemioPoint2, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: parse_f64(x)?, y: parse_f64(y)? })
}

/// 🗺️➰️ `L[origin,direction]` / `C[center,radius]` / `E[center,xAxis,radiusMajor,radiusMinor]` /
/// `N[controlPoints,weights,degree,knots]` — same convention as [`enc_curve`], one dimension down.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_curve2(c: &BrepCurve2) -> String {
    match c {
        BrepCurve2::Line { origin, direction } => format!("L[{},{}]", enc_point2(origin), enc_point2(direction)),
        BrepCurve2::Circle { center, radius } => format!("C[{},{}]", enc_point2(center), radius),
        BrepCurve2::Ellipse { center, x_axis, radius_major, radius_minor } => format!("E[{},{},{},{}]", enc_point2(center), enc_point2(x_axis), radius_major, radius_minor),
        BrepCurve2::Nurbs { control_points, weights, degree, knots } => format!("N[{},{},{},{}]", enc_list(control_points, enc_point2), enc_list(weights, |w: &f64| w.to_string()), degree, enc_list(knots, |k: &f64| k.to_string()),),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_curve2(s: &str) -> Result<BrepCurve2, String> {
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
fn enc_opt_curve2(c: &Option<BrepCurve2>) -> String {
    match c {
        Some(curve) => format!("~{}", enc_curve2(curve)),
        None => "-".to_string(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_opt_curve2(s: &str) -> Result<Option<BrepCurve2>, String> {
    if s == "-" {
        return Ok(None);
    }
    let rest = s.strip_prefix('~').ok_or_else(|| format!("optional curve2: expected '-' or a '~'-prefixed curve, got {s:?}"))?;
    Ok(Some(dec_curve2(rest)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_prange(r: &(f64, f64)) -> String {
    format!("[{},{}]", r.0, r.1)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_prange(s: &str) -> Result<(f64, f64), String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [a, b] = parts.as_slice() else { return Err(format!("prange: expected 2 fields, got {}", parts.len())) };
    Ok((parse_f64(a)?, parse_f64(b)?))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_coedge(c: &BrepCoedge) -> String {
    format!("[{},{},{},{},{},{},{},{}]", enc_str(&c.id), enc_str(&c.edge), enc_bool(c.forward), enc_opt_curve2(&c.pcurve), enc_prange(&c.prange), enc_str(&c.loop_id), enc_str(&c.next), enc_str(&c.prev))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_coedge(s: &str) -> Result<BrepCoedge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, edge, forward, pcurve, prange, loop_id, next, prev] = parts.as_slice() else { return Err(format!("coedge: expected 8 fields, got {}", parts.len())) };
    Ok(BrepCoedge { id: dec_str(id)?, edge: dec_str(edge)?, forward: parse_bool(forward)?, pcurve: dec_opt_curve2(pcurve)?, prange: dec_prange(prange)?, loop_id: dec_str(loop_id)?, next: dec_str(next)?, prev: dec_str(prev)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_loop_edge(le: &BrepLoopEdge) -> String {
    format!("[{},{}]", enc_str(&le.edge), enc_bool(le.orientation))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_loop_edge(s: &str) -> Result<BrepLoopEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [edge, orientation] = parts.as_slice() else { return Err(format!("loop edge: expected 2 fields, got {}", parts.len())) };
    Ok(BrepLoopEdge { edge: dec_str(edge)?, orientation: parse_bool(orientation)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_shell_face(sf: &BrepShellFace) -> String {
    format!("[{},{}]", enc_str(&sf.face), enc_bool(sf.orientation))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_shell_face(s: &str) -> Result<BrepShellFace, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [face, orientation] = parts.as_slice() else { return Err(format!("shell face: expected 2 fields, got {}", parts.len())) };
    Ok(BrepShellFace { face: dec_str(face)?, orientation: parse_bool(orientation)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_solid_shell(ss: &BrepSolidShell) -> String {
    format!("[{},{}]", enc_str(&ss.shell), enc_bool(ss.is_void))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_solid_shell(s: &str) -> Result<BrepSolidShell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [shell, is_void] = parts.as_slice() else { return Err(format!("solid shell: expected 2 fields, got {}", parts.len())) };
    Ok(BrepSolidShell { shell: dec_str(shell)?, is_void: parse_bool(is_void)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_vertex(v: &BrepVertex) -> String {
    format!("[{},{},{}]", enc_str(&v.id), enc_point3(&v.point), v.tol)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_vertex(s: &str) -> Result<BrepVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, point, tol] = parts.as_slice() else { return Err(format!("vertex: expected 3 fields, got {}", parts.len())) };
    Ok(BrepVertex { id: dec_str(id)?, point: dec_point3(point)?, tol: parse_f64(tol)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_edge(e: &BrepEdge) -> String {
    format!("[{},{},{},{},{}]", enc_str(&e.id), enc_str(&e.start_vertex), enc_str(&e.end_vertex), enc_curve(&e.curve), e.tol)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_edge(s: &str) -> Result<BrepEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, start_vertex, end_vertex, curve, tol] = parts.as_slice() else { return Err(format!("edge: expected 5 fields, got {}", parts.len())) };
    Ok(BrepEdge { id: dec_str(id)?, start_vertex: dec_str(start_vertex)?, end_vertex: dec_str(end_vertex)?, curve: dec_curve(curve)?, tol: parse_f64(tol)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_loop(l: &BrepLoop) -> String {
    format!("[{},{}]", enc_str(&l.id), enc_list(&l.edges, enc_loop_edge))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_loop(s: &str) -> Result<BrepLoop, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, edges] = parts.as_slice() else { return Err(format!("loop: expected 2 fields, got {}", parts.len())) };
    Ok(BrepLoop { id: dec_str(id)?, edges: dec_list(edges, dec_loop_edge)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_face(f: &BrepFace) -> String {
    format!("[{},{},{},{},{},{}]", enc_str(&f.id), enc_str(&f.outer_loop), enc_list(&f.inner_loops, |s: &String| enc_str(s)), enc_surface(&f.surface), enc_bool(f.orientation), f.tol)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_face(s: &str) -> Result<BrepFace, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, outer_loop, inner_loops, surface, orientation, tol] = parts.as_slice() else { return Err(format!("face: expected 6 fields, got {}", parts.len())) };
    Ok(BrepFace { id: dec_str(id)?, outer_loop: dec_str(outer_loop)?, inner_loops: dec_list(inner_loops, dec_str)?, surface: dec_surface(surface)?, orientation: parse_bool(orientation)?, tol: parse_f64(tol)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_shell(sh: &BrepShell) -> String {
    format!("[{},{}]", enc_str(&sh.id), enc_list(&sh.faces, enc_shell_face))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_shell(s: &str) -> Result<BrepShell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, faces] = parts.as_slice() else { return Err(format!("shell: expected 2 fields, got {}", parts.len())) };
    Ok(BrepShell { id: dec_str(id)?, faces: dec_list(faces, dec_shell_face)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_solid(so: &BrepSolid) -> String {
    format!("[{},{}]", enc_str(&so.id), enc_list(&so.shells, enc_solid_shell))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_solid(s: &str) -> Result<BrepSolid, String> {
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
fn print_brep_snapshot_body(s: &SemioBrepSnapshot) -> String {
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
fn parse_brep_snapshot_body(body: &str) -> Result<SemioBrepSnapshot, String> {
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
//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers `stdio.semio.flow`'s upgraded `OpBinary`/`DiffCodec`
/// reuse) backing the real `ArtifactPack` below — replaces the old `serde_json::to_vec`-in-
/// envelope shortcut.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_point3(out: &mut Vec<u8>, p: &SemioPoint3) {
    out.extend_from_slice(&p.x.to_le_bytes());
    out.extend_from_slice(&p.y.to_le_bytes());
    out.extend_from_slice(&p.z.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_point3(reader: &mut store::ByteReader<'_>) -> Result<SemioPoint3, String> {
    let x = reader.read_f64_le().map_err(|e| e.to_string())?;
    let y = reader.read_f64_le().map_err(|e| e.to_string())?;
    let z = reader.read_f64_le().map_err(|e| e.to_string())?;
    Ok(SemioPoint3 { x, y, z })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_f64_vec(out: &mut Vec<u8>, v: &[f64]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for x in v {
        out.extend_from_slice(&x.to_le_bytes());
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_f64_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<f64>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(reader.read_f64_le().map_err(|e| e.to_string())?);
    }
    Ok(v)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_point3_vec(out: &mut Vec<u8>, v: &[SemioPoint3]) {
    store::pack_rt::write_varint_u64(out, v.len() as u64);
    for p in v {
        write_point3(out, p);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_point3_vec(reader: &mut store::ByteReader<'_>) -> Result<Vec<SemioPoint3>, String> {
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(read_point3(reader)?);
    }
    Ok(v)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}

/// 🏷️ `BrepCurve` variant tags — 0=Line, 1=Circle, 2=Ellipse, 3=Nurbs (declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_curve(out: &mut Vec<u8>, c: &BrepCurve) {
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
fn read_curve(reader: &mut store::ByteReader<'_>) -> Result<BrepCurve, String> {
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
fn write_surface(out: &mut Vec<u8>, s: &BrepSurface) {
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
fn read_surface(reader: &mut store::ByteReader<'_>) -> Result<BrepSurface, String> {
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
fn write_vertex(out: &mut Vec<u8>, v: &BrepVertex) {
    write_str_lp(out, &v.id);
    write_point3(out, &v.point);
    out.extend_from_slice(&v.tol.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_vertex(reader: &mut store::ByteReader<'_>) -> Result<BrepVertex, String> {
    Ok(BrepVertex { id: read_str_lp(reader)?, point: read_point3(reader)?, tol: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_edge(out: &mut Vec<u8>, e: &BrepEdge) {
    write_str_lp(out, &e.id);
    write_str_lp(out, &e.start_vertex);
    write_str_lp(out, &e.end_vertex);
    write_curve(out, &e.curve);
    out.extend_from_slice(&e.tol.to_le_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_edge(reader: &mut store::ByteReader<'_>) -> Result<BrepEdge, String> {
    Ok(BrepEdge { id: read_str_lp(reader)?, start_vertex: read_str_lp(reader)?, end_vertex: read_str_lp(reader)?, curve: read_curve(reader)?, tol: reader.read_f64_le().map_err(|e| e.to_string())? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_loop_edge(out: &mut Vec<u8>, le: &BrepLoopEdge) {
    write_str_lp(out, &le.edge);
    write_bool(out, le.orientation);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_loop_edge(reader: &mut store::ByteReader<'_>) -> Result<BrepLoopEdge, String> {
    Ok(BrepLoopEdge { edge: read_str_lp(reader)?, orientation: read_bool(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_loop(out: &mut Vec<u8>, l: &BrepLoop) {
    write_str_lp(out, &l.id);
    store::pack_rt::write_varint_u64(out, l.edges.len() as u64);
    for le in &l.edges {
        write_loop_edge(out, le);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_loop(reader: &mut store::ByteReader<'_>) -> Result<BrepLoop, String> {
    let id = read_str_lp(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut edges = Vec::with_capacity(n as usize);
    for _ in 0..n {
        edges.push(read_loop_edge(reader)?);
    }
    Ok(BrepLoop { id, edges })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_face(out: &mut Vec<u8>, f: &BrepFace) {
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
fn read_face(reader: &mut store::ByteReader<'_>) -> Result<BrepFace, String> {
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
fn write_shell_face(out: &mut Vec<u8>, sf: &BrepShellFace) {
    write_str_lp(out, &sf.face);
    write_bool(out, sf.orientation);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_shell_face(reader: &mut store::ByteReader<'_>) -> Result<BrepShellFace, String> {
    Ok(BrepShellFace { face: read_str_lp(reader)?, orientation: read_bool(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_shell(out: &mut Vec<u8>, sh: &BrepShell) {
    write_str_lp(out, &sh.id);
    store::pack_rt::write_varint_u64(out, sh.faces.len() as u64);
    for sf in &sh.faces {
        write_shell_face(out, sf);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_shell(reader: &mut store::ByteReader<'_>) -> Result<BrepShell, String> {
    let id = read_str_lp(reader)?;
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut faces = Vec::with_capacity(n as usize);
    for _ in 0..n {
        faces.push(read_shell_face(reader)?);
    }
    Ok(BrepShell { id, faces })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_solid_shell(out: &mut Vec<u8>, ss: &BrepSolidShell) {
    write_str_lp(out, &ss.shell);
    write_bool(out, ss.is_void);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_solid_shell(reader: &mut store::ByteReader<'_>) -> Result<BrepSolidShell, String> {
    Ok(BrepSolidShell { shell: read_str_lp(reader)?, is_void: read_bool(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_solid(out: &mut Vec<u8>, so: &BrepSolid) {
    write_str_lp(out, &so.id);
    store::pack_rt::write_varint_u64(out, so.shells.len() as u64);
    for ss in &so.shells {
        write_solid_shell(out, ss);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_solid(reader: &mut store::ByteReader<'_>) -> Result<BrepSolid, String> {
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
fn write_curve2(out: &mut Vec<u8>, c: &BrepCurve2) {
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
fn read_curve2(reader: &mut store::ByteReader<'_>) -> Result<BrepCurve2, String> {
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
fn write_coedge(out: &mut Vec<u8>, c: &BrepCoedge) {
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
fn read_coedge(reader: &mut store::ByteReader<'_>) -> Result<BrepCoedge, String> {
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
fn encode_brep_snapshot_binary(s: &SemioBrepSnapshot) -> Vec<u8> {
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
fn decode_brep_snapshot_binary(bytes: &[u8]) -> Result<SemioBrepSnapshot, String> {
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
//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 Real structured text/binary codecs (brep wave — off the old hex-dump-of-`serde_json`
/// shortcut, following the flow pilot's proven template). Wrapped in the repo-wide
/// `store::semio_format` envelope, unchanged.
impl store::ArtifactDsl for SemioBrepSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIOBREP_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_brep_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_brep_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioBrepSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_brep_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_brep_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
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
    pack::to_json_string(snapshot)
}

/// 📥️ The `pack::from_json_str` inverse of [`encode_semio_brep_snapshot_json`] — decodes the
/// committed `../🧬️mutations/<kind>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors into real [`SemioBrepSnapshot`] values, so `🧊️mutate-semio-brep`'s adapter reads the
/// committed fixture instead of re-declaring it as a Rust literal beside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_brep_snapshot_json(text: &str) -> Result<SemioBrepSnapshot, String> {
    pack::from_json_str(text).map_err(|error| error.to_string())
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
//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.brep` document — one triangular face bounding one shell bounding one
/// solid, exercising every collection AND every `BrepCurve`/`BrepSurface` variant at least once
/// (incl. the `Nurbs` variants, whose `Vec<SemioPoint3>`/`Vec<f64>` fields are the shapes most
/// likely to expose an encoder/grammar mismatch). Single source of truth for
/// `📚️examples/🧊️solid/🖼️assets/🗣️.dsl.semio`/`🎒️.pack.semio` and for the
/// conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_brep_snapshot() -> SemioBrepSnapshot {
    let mut s = SemioBrepSnapshot::default();
    s.vertices = vec![
        BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 1e-7 },
        BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 4.0, y: 0.0, z: 0.0 }, tol: 1e-7 },
        BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 4.0, y: 3.0, z: 0.0 }, tol: 1e-7 },
    ];
    s.edges = vec![
        BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: s.vertices[0].point, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }, tol: 1e-7 },
        BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Circle { center: SemioPoint3 { x: 4.0, y: 1.5, z: 0.0 }, axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.5 }, tol: 1e-7 },
        BrepEdge { id: "e3".into(), start_vertex: "v3".into(), end_vertex: "v1".into(), curve: BrepCurve::Nurbs { control_points: vec![s.vertices[2].point, s.vertices[0].point], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] }, tol: 1e-7 },
    ];
    s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }] }];
    // 🧱️ Coedges mirror `loops[0].edges` one-for-one, in ring order, with a p-curve stored on the
    // first coedge only — exercising both the `Some(pcurve)` and `None` (fallback-to-projection)
    // arms of `Body::from_snapshot` in one fixture.
    s.coedges = vec![
        BrepCoedge {
            id: "co1".into(),
            edge: "e1".into(),
            forward: true,
            pcurve: Some(BrepCurve2::Line { origin: SemioPoint2 { x: 0.0, y: 0.0 }, direction: SemioPoint2 { x: 1.0, y: 0.0 } }),
            prange: (0.0, 4.0),
            loop_id: "l1".into(),
            next: "co2".into(),
            prev: "co3".into(),
        },
        BrepCoedge { id: "co2".into(), edge: "e2".into(), forward: true, pcurve: None, prange: (0.0, 0.0), loop_id: "l1".into(), next: "co3".into(), prev: "co1".into() },
        BrepCoedge { id: "co3".into(), edge: "e3".into(), forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: "l1".into(), next: "co1".into(), prev: "co2".into() },
    ];
    s.faces = vec![BrepFace {
        id: "f1".into(),
        outer_loop: "l1".into(),
        inner_loops: vec![],
        surface: BrepSurface::Nurbs {
            control_points: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 4.0, y: 3.0, z: 0.0 }],
            weights: vec![1.0, 1.0],
            u_count: 2,
            v_count: 1,
            degree_u: 1,
            degree_v: 1,
            knots_u: vec![0.0, 0.0, 1.0, 1.0],
            knots_v: vec![0.0, 1.0],
        },
        orientation: true,
        tol: 1e-7,
    }];
    s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
    s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
    s.next_label = 100;
    s
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧱️ A small but fully-populated, self-referentially-consistent b-rep: one triangular face
    /// bounding one shell bounding one solid. Reused by the codec_retention_law test below.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn populated_snapshot() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![
            BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 1e-7 },
            BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 4.0, y: 0.0, z: 0.0 }, tol: 1e-7 },
            BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 4.0, y: 3.0, z: 0.0 }, tol: 1e-7 },
        ];
        s.edges = vec![
            BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: s.vertices[0].point, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }, tol: 1e-7 },
            BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Line { origin: s.vertices[1].point, direction: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 } }, tol: 1e-7 },
            BrepEdge {
                id: "e3".into(),
                start_vertex: "v3".into(),
                end_vertex: "v1".into(),
                curve: BrepCurve::Nurbs { control_points: vec![s.vertices[2].point, s.vertices[0].point], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] },
                tol: 1e-7,
            },
        ];
        s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }] }];
        s.coedges = vec![
            BrepCoedge { id: "co1".into(), edge: "e1".into(), forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: "l1".into(), next: "co2".into(), prev: "co3".into() },
            BrepCoedge { id: "co2".into(), edge: "e2".into(), forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: "l1".into(), next: "co3".into(), prev: "co1".into() },
            BrepCoedge { id: "co3".into(), edge: "e3".into(), forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: "l1".into(), next: "co1".into(), prev: "co2".into() },
        ];
        s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true, tol: 1e-7 }];
        s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
        s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
        s.next_label = 42;
        s
    }

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = SemioBrepSnapshot::default();
        let bytes = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = SemioBrepSnapshot::default();
        let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: a fully-populated snapshot (every collection non-empty, every
    /// `BrepSurface`/`BrepCurve` variant represented at least once) survives a pack AND a dsl
    /// round trip byte-for-byte (structurally — every field, incl. every `Nurbs` variant's
    /// `Vec<SemioPoint3>`/`Vec<f64>` runs, round-trips exactly).
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law_populated_snapshot_round_trips_pack_and_dsl() {
        let snap = populated_snapshot();
        let packed = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&snap);
        assert_eq!(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), snap);
        let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert_eq!(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snap);
    }

    /// 🧪️ Every `BrepCurve`/`BrepSurface` variant (incl. both `Nurbs` shapes) round-trips through
    /// both the pack binary and the dsl text codec — the demo fixture used by the fixture-honesty
    /// conformance law.
    #[semio_framework_async_macros::async_test]
    async fn demo_snapshot_round_trips_pack_and_dsl() {
        let demo = demo_brep_snapshot();
        let packed = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&demo);
        assert_eq!(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), demo);
        let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&demo);
        assert_eq!(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), demo);
    }
}
//#endregion 🔖️Tests
