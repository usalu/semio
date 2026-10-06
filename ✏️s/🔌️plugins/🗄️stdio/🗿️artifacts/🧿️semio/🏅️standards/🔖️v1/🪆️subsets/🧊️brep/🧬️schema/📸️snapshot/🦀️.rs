//! 🧬️ SemioBrepSnapshot — id-keyed b-rep topology graph (vertices/edges/loops/faces/shells/
//! solids) with typed `BrepSurface`/`BrepCurve` value enums. Informed by step's
//! `⚙️engine/🧱️brep` analyzer view (`BrepMesh`: vertices + polygon faces) and `StepSnapshot`'s
//! generic Part-21 entity graph (`CARTESIAN_POINT`/`VERTEX_POINT`/`EDGE_CURVE`/`ORIENTED_EDGE`/
//! `EDGE_LOOP`/`FACE_BOUND`/`ADVANCED_FACE`/`CLOSED_SHELL`/`MANIFOLD_SOLID_BREP`) — this snapshot
//! generalizes that analyzer's planar-only mesh into a full typed b-rep: every edge carries its
//! own curve (not just a straight-line control polygon) and every face its own surface (not just
//! `PLANE`), matching AP214's real vocabulary of surface/curve kinds.

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3};


use framework_schema::ArtifactSchema;
use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;

//#region 🔖️Ids
pub const STDIO_SEMIOBREP_DOCUMENT_SCHEMA: &str = "stdio.semio.brep";
//#endregion 🔖️Ids

//#region 🔖️Curve
/// 📈️ A b-rep edge's underlying 3D curve. Owned by `brep` (`w1b-type-ownership.md`).
///
/// 🔣️ Container `rename_all` cases the variant names (`ellipse`, `nurbs`); `rename_all_fields` cases every
/// struct-variant member (`radiusMajor`, `controlPoints`), exactly as the brep schema (`📸️snapshot/🔣️.json`)
/// and serde state them (`🌱️value/✨️derive/🧪️tests/🐫️variant-field-casing`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
/// (`📸️snapshot/➰️curve/🦀️.rs`) field-for-field so [`Body::to_snapshot`]/[`crate::standards::v1::subsets::brep::schema::snapshot::body::body_from_snapshot`]
/// (`📸️snapshot/🔁️body/🦀️.rs`) round-trip it exactly, never approximated.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum BrepCurve2 {
    Line { origin: SemioPoint2, direction: SemioPoint2 },
    Circle { center: SemioPoint2, radius: f64 },
    Ellipse { center: SemioPoint2, x_axis: SemioPoint2, radius_major: f64, radius_minor: f64 },
    Nurbs { control_points: Vec<SemioPoint2>, weights: Vec<f64>, degree: u32, knots: Vec<f64> },
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
/// 🔣️ Cased like [`BrepCurve`]: variant names by `rename_all`, members by `rename_all_fields`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
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
    /// 🎚️ Native `Vertex::tol` containment radius in model units, persisted explicitly.
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
    /// 🎚️ Native `Edge::tol` tube radius in model units, persisted explicitly.
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
    /// 🎚️ Native `Face::tol` shell thickness in model units, persisted explicitly.
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
    /// this field existed (STEP import, hand-authored fixtures): [`crate::standards::v1::subsets::brep::schema::snapshot::body::crate::standards::v1::subsets::brep::schema::snapshot::body::body_from_snapshot`]
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
        Self {
            schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(),
            vertices: Default::default(),
            edges: Default::default(),
            loops: Default::default(),
            faces: Default::default(),
            shells: Default::default(),
            solids: Default::default(),
            coedges: Default::default(),
            next_label: 0,
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextPrimitives


































































//#endregion 🔖️TextPrimitives

//#region 🔖️BinaryPrimitives














































//#endregion 🔖️BinaryPrimitives

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge











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
        BrepEdge {
            id: "e3".into(),
            start_vertex: "v3".into(),
            end_vertex: "v1".into(),
            curve: BrepCurve::Nurbs { control_points: vec![s.vertices[2].point, s.vertices[0].point], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] },
            tol: 1e-7,
        },
    ];
    s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }] }];
    // 🧱️ Coedges mirror `loops[0].edges` one-for-one, in ring order, with a p-curve stored on the
    // first coedge only — exercising both the `Some(pcurve)` and `None` (fallback-to-projection)
    // arms of `crate::standards::v1::subsets::brep::schema::snapshot::body::body_from_snapshot` in one fixture.
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests





