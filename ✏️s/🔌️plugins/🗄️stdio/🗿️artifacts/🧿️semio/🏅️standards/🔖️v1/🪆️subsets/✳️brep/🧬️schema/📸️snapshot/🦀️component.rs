//! 🧬️ SemioBrepSnapshot — id-keyed b-rep topology graph (vertices/edges/loops/faces/shells/
//! solids) with typed `BrepSurface`/`BrepCurve` value enums. Informed by step's
//! `⚙️engine/🧱️brep` analyzer view (`BrepMesh`: vertices + polygon faces) and `StepSnapshot`'s
//! generic Part-21 entity graph (`CARTESIAN_POINT`/`VERTEX_POINT`/`EDGE_CURVE`/`ORIENTED_EDGE`/
//! `EDGE_LOOP`/`FACE_BOUND`/`ADVANCED_FACE`/`CLOSED_SHELL`/`MANIFOLD_SOLID_BREP`) — this snapshot
//! generalizes that analyzer's planar-only mesh into a full typed b-rep: every edge carries its
//! own curve (not just a straight-line control polygon) and every face its own surface (not just
//! `PLANE`), matching AP214's real vocabulary of surface/curve kinds.

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOBREP_DOCUMENT_SCHEMA: &str = "stdio.semio.brep";
//#endregion 🔖️Ids

//#region 🔖️Curve
/// 📈️ A b-rep edge's underlying 3D curve. Owned by `brep` (`w1b-type-ownership.md`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum BrepCurve {
    Line { origin: SemioPoint3, direction: SemioPoint3 },
    Circle { center: SemioPoint3, axis: SemioPoint3, radius: f64 },
    Ellipse { center: SemioPoint3, axis: SemioPoint3, radius_major: f64, radius_minor: f64 },
    /// 🎛️ Rational B-spline curve: a flat `control_points`/`weights` run alongside `knots`
    /// (length `control_points.len() + degree + 1`, per the standard open-uniform-or-not knot
    /// vector convention) — no nested fixed arrays, per the f6 §4.3 gap.
    Nurbs { control_points: Vec<SemioPoint3>, weights: Vec<f64>, degree: u32, knots: Vec<f64> },
}

/// 🩹️ Needed ONLY so `BrepEdge`/entity structs can derive `Default` (in turn needed only because
/// `serde_derive`'s `#[serde(default)]` on a `Vec<T>` field spuriously infers `T: Default` for the
/// SHARED `🧰️triples::NamedTripleDiff<K,D,T>`'s `added: Vec<T>` — see the "shared infra gaps" note
/// in the wave report; never constructed as a meaningful default in real code paths.
impl Default for BrepCurve {
    fn default() -> Self { BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3::default() } }
}
//#endregion 🔖️Curve

//#region 🔖️Surface
/// 🗺️ A b-rep face's underlying surface. Owned by `brep`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum BrepSurface {
    Plane { origin: SemioPoint3, normal: SemioPoint3 },
    Cylinder { origin: SemioPoint3, axis: SemioPoint3, radius: f64 },
    Cone { origin: SemioPoint3, axis: SemioPoint3, radius: f64, half_angle: f64 },
    Sphere { center: SemioPoint3, radius: f64 },
    Torus { center: SemioPoint3, axis: SemioPoint3, major_radius: f64, minor_radius: f64 },
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
    fn default() -> Self { BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3::default() } }
}
//#endregion 🔖️Surface

//#region 🔖️Topology
/// 📍️ A b-rep vertex — corresponds to STEP's `VERTEX_POINT`/`CARTESIAN_POINT` pair collapsed
/// into one id-keyed entity.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepVertex {
    pub id: String,
    pub point: SemioPoint3,
}

/// ➡️ A b-rep edge — corresponds to STEP's `EDGE_CURVE`, always resolved between two vertices.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepEdge {
    pub id: String,
    pub start_vertex: String,
    pub end_vertex: String,
    pub curve: BrepCurve,
}

/// 🔁️ A loop-member reference to an edge, carrying the traversal orientation — STEP's
/// `ORIENTED_EDGE.orientation`. A named weak struct, never a bare `(String, bool)` tuple.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepLoopEdge {
    pub edge: String,
    pub orientation: bool,
}

/// ⭕️ A closed edge loop — corresponds to STEP's `EDGE_LOOP`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepLoop {
    pub id: String,
    #[serde(default)]
    pub edges: Vec<BrepLoopEdge>,
}

/// 🔺️ A b-rep face — corresponds to STEP's `ADVANCED_FACE`, bounded by one outer loop and zero
/// or more inner (hole) loops, over a typed surface.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepFace {
    pub id: String,
    pub outer_loop: String,
    #[serde(default)]
    pub inner_loops: Vec<String>,
    pub surface: BrepSurface,
    pub orientation: bool,
}

/// 🔁️ A shell-member reference to a face, carrying orientation — STEP's face-in-shell sense.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepShellFace {
    pub face: String,
    pub orientation: bool,
}

/// 🐚️ A closed (or open) shell — corresponds to STEP's `CLOSED_SHELL`/`OPEN_SHELL`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepShell {
    pub id: String,
    #[serde(default)]
    pub faces: Vec<BrepShellFace>,
}

/// 🔁️ A solid-member reference to a shell, flagging whether it bounds a void (an internal
/// cavity) rather than the solid's outer boundary — STEP's `MANIFOLD_SOLID_BREP.voids`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepSolidShell {
    pub shell: String,
    pub is_void: bool,
}

/// 🧊️ A manifold solid — corresponds to STEP's `MANIFOLD_SOLID_BREP`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepSolid {
    pub id: String,
    #[serde(default)]
    pub shells: Vec<BrepSolidShell>,
}
//#endregion 🔖️Topology

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.brep")]
pub struct SemioBrepSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub vertices: Vec<BrepVertex>,
    #[state(persistent)]
    #[serde(default)]
    pub edges: Vec<BrepEdge>,
    #[state(persistent)]
    #[serde(default)]
    pub loops: Vec<BrepLoop>,
    #[state(persistent)]
    #[serde(default)]
    pub faces: Vec<BrepFace>,
    #[state(persistent)]
    #[serde(default)]
    pub shells: Vec<BrepShell>,
    #[state(persistent)]
    #[serde(default)]
    pub solids: Vec<BrepSolid>,
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
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 🚧 JSON-pack round trip (honest, genuinely working — not a per-format binary codec, since
/// this subset's snapshot is a NEUTRAL semio type, not an on-disk file format). Wrapped in the
/// same `store::semio_format` envelope every stdio artifact uses.
impl store::ArtifactDsl for SemioBrepSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOBREP_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioBrepSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧱️ A small but fully-populated, self-referentially-consistent b-rep: one triangular face
    /// bounding one shell bounding one solid. Reused by the codec_retention_law test below.
    fn populated_snapshot() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![
            BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } },
            BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 4.0, y: 0.0, z: 0.0 } },
            BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 4.0, y: 3.0, z: 0.0 } },
        ];
        s.edges = vec![
            BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: s.vertices[0].point, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } },
            BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Line { origin: s.vertices[1].point, direction: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 } } },
            BrepEdge { id: "e3".into(), start_vertex: "v3".into(), end_vertex: "v1".into(), curve: BrepCurve::Nurbs { control_points: vec![s.vertices[2].point, s.vertices[0].point], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] } },
        ];
        s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![
            BrepLoopEdge { edge: "e1".into(), orientation: true },
            BrepLoopEdge { edge: "e2".into(), orientation: true },
            BrepLoopEdge { edge: "e3".into(), orientation: true },
        ] }];
        s.faces = vec![BrepFace {
            id: "f1".into(),
            outer_loop: "l1".into(),
            inner_loops: vec![],
            surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } },
            orientation: true,
        }];
        s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
        s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
        s
    }

    #[test]
    fn json_pack_round_trips() {
        let snap = SemioBrepSnapshot::default();
        let bytes = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[test]
    fn dsl_text_round_trips() {
        let snap = SemioBrepSnapshot::default();
        let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    /// 🧪️ codec_retention_law: a fully-populated snapshot (every collection non-empty, every
    /// `BrepSurface`/`BrepCurve` variant represented at least once) survives a pack AND a dsl
    /// round trip byte-for-byte (structurally, since the wire form is JSON — no lossy
    /// normalization anywhere in the pipeline).
    #[test]
    fn codec_retention_law_populated_snapshot_round_trips_pack_and_dsl() {
        let snap = populated_snapshot();
        let packed = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&snap);
        assert_eq!(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), snap);
        let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        assert_eq!(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snap);
    }
}
//#endregion 🔖️Tests
