//! 🧬️ SemioBrepMutation — named-variant enum (one `Add*`/`Remove*`/`Set*` triad per collection,
//! plus `SetSnapshot`), following the gif 89a / docx precedent. Every variant's `diff()`/
//! `inverse()` is HAND-WRITTEN below (never apply-and-capture — schema-design.md's warning: svg's
//! original bug was computing diffs via clone+apply+re-diff causing infinite mutual recursion
//! once `mutate` was flipped to return `(Self, Diff)`).

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::engine::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{
    BrepEdgeDiff, BrepFaceDiff, BrepLoopDiff, BrepShellDiff, BrepSolidDiff, BrepVertexDiff, SemioBrepDiff,
};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{
    BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell,
    BrepSurface, BrepVertex, SemioBrepSnapshot,
};
use protocol::command::DiffAlgebra;
use protocol::Mutation;
#[cfg(test)]
use protocol::{OpBinary, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioBrepMutation {
    #[default]
    NoMutation,
    SetSnapshot { snapshot: SemioBrepSnapshot },
    AddVertex { vertex: BrepVertex },
    RemoveVertex { id: String },
    SetVertexPoint { id: String, point: SemioPoint3 },
    AddEdge { edge: BrepEdge },
    RemoveEdge { id: String },
    SetEdgeEndpoints { id: String, start_vertex: String, end_vertex: String },
    SetEdgeCurve { id: String, curve: BrepCurve },
    AddLoop { brep_loop: BrepLoop },
    RemoveLoop { id: String },
    SetLoopEdges { id: String, edges: Vec<BrepLoopEdge> },
    AddFace { face: BrepFace },
    RemoveFace { id: String },
    SetFaceSurface { id: String, surface: BrepSurface },
    SetFaceOrientation { id: String, orientation: bool },
    SetFaceLoops { id: String, outer_loop: String, inner_loops: Vec<String> },
    AddShell { shell: BrepShell },
    RemoveShell { id: String },
    SetShellFaces { id: String, faces: Vec<BrepShellFace> },
    AddSolid { solid: BrepSolid },
    RemoveSolid { id: String },
    SetSolidShells { id: String, shells: Vec<BrepSolidShell> },
}
//#endregion 🔖️Mutation

//#region 🔖️DiffWrapHelpers
/// 🧭️ Lowers a single-collection triple into a full `SemioBrepDiff` — the sparse-write analog of
/// bcf's `wrap_topic_diff`. One generic helper, instantiated per collection via the small
/// `diff_add_*`/`diff_remove_*`/`diff_modify_*` functions below (mirrors the diff facet's own
/// per-entity function-per-collection style rather than a single mega-generic, since each
/// collection sits behind its own named `SemioBrepDiff` field).
fn wrap_vertices(triple: NamedTripleDiff<String, BrepVertexDiff, BrepVertex>) -> SemioBrepDiff {
    SemioBrepDiff { vertices: Some(triple), ..Default::default() }
}
fn wrap_edges(triple: NamedTripleDiff<String, BrepEdgeDiff, BrepEdge>) -> SemioBrepDiff {
    SemioBrepDiff { edges: Some(triple), ..Default::default() }
}
fn wrap_loops(triple: NamedTripleDiff<String, BrepLoopDiff, BrepLoop>) -> SemioBrepDiff {
    SemioBrepDiff { loops: Some(triple), ..Default::default() }
}
fn wrap_faces(triple: NamedTripleDiff<String, BrepFaceDiff, BrepFace>) -> SemioBrepDiff {
    SemioBrepDiff { faces: Some(triple), ..Default::default() }
}
fn wrap_shells(triple: NamedTripleDiff<String, BrepShellDiff, BrepShell>) -> SemioBrepDiff {
    SemioBrepDiff { shells: Some(triple), ..Default::default() }
}
fn wrap_solids(triple: NamedTripleDiff<String, BrepSolidDiff, BrepSolid>) -> SemioBrepDiff {
    SemioBrepDiff { solids: Some(triple), ..Default::default() }
}

fn diff_add_vertex(v: &BrepVertex) -> SemioBrepDiff { wrap_vertices(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![v.clone()] }) }
fn diff_remove_vertex(id: &str) -> SemioBrepDiff { wrap_vertices(NamedTripleDiff { removed: vec![id.to_string()], modified: vec![], added: vec![] }) }
fn diff_modify_vertex(id: &str, d: BrepVertexDiff) -> SemioBrepDiff { wrap_vertices(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: id.to_string(), diff: d }], added: vec![] }) }

fn diff_add_edge(e: &BrepEdge) -> SemioBrepDiff { wrap_edges(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![e.clone()] }) }
fn diff_remove_edge(id: &str) -> SemioBrepDiff { wrap_edges(NamedTripleDiff { removed: vec![id.to_string()], modified: vec![], added: vec![] }) }
fn diff_modify_edge(id: &str, d: BrepEdgeDiff) -> SemioBrepDiff { wrap_edges(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: id.to_string(), diff: d }], added: vec![] }) }

fn diff_add_loop(l: &BrepLoop) -> SemioBrepDiff { wrap_loops(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![l.clone()] }) }
fn diff_remove_loop(id: &str) -> SemioBrepDiff { wrap_loops(NamedTripleDiff { removed: vec![id.to_string()], modified: vec![], added: vec![] }) }
fn diff_modify_loop(id: &str, d: BrepLoopDiff) -> SemioBrepDiff { wrap_loops(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: id.to_string(), diff: d }], added: vec![] }) }

fn diff_add_face(f: &BrepFace) -> SemioBrepDiff { wrap_faces(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![f.clone()] }) }
fn diff_remove_face(id: &str) -> SemioBrepDiff { wrap_faces(NamedTripleDiff { removed: vec![id.to_string()], modified: vec![], added: vec![] }) }
fn diff_modify_face(id: &str, d: BrepFaceDiff) -> SemioBrepDiff { wrap_faces(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: id.to_string(), diff: d }], added: vec![] }) }

fn diff_add_shell(s: &BrepShell) -> SemioBrepDiff { wrap_shells(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![s.clone()] }) }
fn diff_remove_shell(id: &str) -> SemioBrepDiff { wrap_shells(NamedTripleDiff { removed: vec![id.to_string()], modified: vec![], added: vec![] }) }
fn diff_modify_shell(id: &str, d: BrepShellDiff) -> SemioBrepDiff { wrap_shells(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: id.to_string(), diff: d }], added: vec![] }) }

fn diff_add_solid(s: &BrepSolid) -> SemioBrepDiff { wrap_solids(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![s.clone()] }) }
fn diff_remove_solid(id: &str) -> SemioBrepDiff { wrap_solids(NamedTripleDiff { removed: vec![id.to_string()], modified: vec![], added: vec![] }) }
fn diff_modify_solid(id: &str, d: BrepSolidDiff) -> SemioBrepDiff { wrap_solids(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: id.to_string(), diff: d }], added: vec![] }) }
//#endregion 🔖️DiffWrapHelpers

//#region 🔖️Diff
impl Mutation<SemioBrepSnapshot> for SemioBrepMutation {
    type Diff = SemioBrepDiff;

    fn diff(&self, base: &SemioBrepSnapshot) -> Self::Diff {
        match self {
            SemioBrepMutation::NoMutation => SemioBrepDiff::default(),
            SemioBrepMutation::SetSnapshot { snapshot } => SemioBrepDiff::between(base, snapshot),
            SemioBrepMutation::AddVertex { vertex } => diff_add_vertex(vertex),
            SemioBrepMutation::RemoveVertex { id } => diff_remove_vertex(id),
            SemioBrepMutation::SetVertexPoint { id, point } => diff_modify_vertex(id, BrepVertexDiff { point: Some(*point) }),
            SemioBrepMutation::AddEdge { edge } => diff_add_edge(edge),
            SemioBrepMutation::RemoveEdge { id } => diff_remove_edge(id),
            SemioBrepMutation::SetEdgeEndpoints { id, start_vertex, end_vertex } => {
                diff_modify_edge(id, BrepEdgeDiff { start_vertex: Some(start_vertex.clone()), end_vertex: Some(end_vertex.clone()), curve: None })
            }
            SemioBrepMutation::SetEdgeCurve { id, curve } => diff_modify_edge(id, BrepEdgeDiff { start_vertex: None, end_vertex: None, curve: Some(curve.clone()) }),
            SemioBrepMutation::AddLoop { brep_loop } => diff_add_loop(brep_loop),
            SemioBrepMutation::RemoveLoop { id } => diff_remove_loop(id),
            SemioBrepMutation::SetLoopEdges { id, edges } => diff_modify_loop(id, BrepLoopDiff { edges: Some(edges.clone()) }),
            SemioBrepMutation::AddFace { face } => diff_add_face(face),
            SemioBrepMutation::RemoveFace { id } => diff_remove_face(id),
            SemioBrepMutation::SetFaceSurface { id, surface } => diff_modify_face(id, BrepFaceDiff { outer_loop: None, inner_loops: None, surface: Some(surface.clone()), orientation: None }),
            SemioBrepMutation::SetFaceOrientation { id, orientation } => diff_modify_face(id, BrepFaceDiff { outer_loop: None, inner_loops: None, surface: None, orientation: Some(*orientation) }),
            SemioBrepMutation::SetFaceLoops { id, outer_loop, inner_loops } => {
                diff_modify_face(id, BrepFaceDiff { outer_loop: Some(outer_loop.clone()), inner_loops: Some(inner_loops.clone()), surface: None, orientation: None })
            }
            SemioBrepMutation::AddShell { shell } => diff_add_shell(shell),
            SemioBrepMutation::RemoveShell { id } => diff_remove_shell(id),
            SemioBrepMutation::SetShellFaces { id, faces } => diff_modify_shell(id, BrepShellDiff { faces: Some(faces.clone()) }),
            SemioBrepMutation::AddSolid { solid } => diff_add_solid(solid),
            SemioBrepMutation::RemoveSolid { id } => diff_remove_solid(id),
            SemioBrepMutation::SetSolidShells { id, shells } => diff_modify_solid(id, BrepSolidDiff { shells: Some(shells.clone()) }),
        }
    }

    fn inverse(&self, base: &SemioBrepSnapshot) -> Vec<Self> {
        match self {
            SemioBrepMutation::NoMutation => vec![SemioBrepMutation::NoMutation],
            SemioBrepMutation::SetSnapshot { .. } => vec![SemioBrepMutation::SetSnapshot { snapshot: base.clone() }],
            SemioBrepMutation::AddVertex { vertex } => vec![SemioBrepMutation::RemoveVertex { id: vertex.id.clone() }],
            SemioBrepMutation::RemoveVertex { id } => base.vertices.iter().find(|v| &v.id == id).map(|v| vec![SemioBrepMutation::AddVertex { vertex: v.clone() }]).unwrap_or_default(),
            SemioBrepMutation::SetVertexPoint { id, .. } => base.vertices.iter().find(|v| &v.id == id).map(|v| vec![SemioBrepMutation::SetVertexPoint { id: id.clone(), point: v.point }]).unwrap_or_default(),
            SemioBrepMutation::AddEdge { edge } => vec![SemioBrepMutation::RemoveEdge { id: edge.id.clone() }],
            SemioBrepMutation::RemoveEdge { id } => base.edges.iter().find(|e| &e.id == id).map(|e| vec![SemioBrepMutation::AddEdge { edge: e.clone() }]).unwrap_or_default(),
            SemioBrepMutation::SetEdgeEndpoints { id, .. } => base.edges.iter().find(|e| &e.id == id)
                .map(|e| vec![SemioBrepMutation::SetEdgeEndpoints { id: id.clone(), start_vertex: e.start_vertex.clone(), end_vertex: e.end_vertex.clone() }])
                .unwrap_or_default(),
            SemioBrepMutation::SetEdgeCurve { id, .. } => base.edges.iter().find(|e| &e.id == id).map(|e| vec![SemioBrepMutation::SetEdgeCurve { id: id.clone(), curve: e.curve.clone() }]).unwrap_or_default(),
            SemioBrepMutation::AddLoop { brep_loop } => vec![SemioBrepMutation::RemoveLoop { id: brep_loop.id.clone() }],
            SemioBrepMutation::RemoveLoop { id } => base.loops.iter().find(|l| &l.id == id).map(|l| vec![SemioBrepMutation::AddLoop { brep_loop: l.clone() }]).unwrap_or_default(),
            SemioBrepMutation::SetLoopEdges { id, .. } => base.loops.iter().find(|l| &l.id == id).map(|l| vec![SemioBrepMutation::SetLoopEdges { id: id.clone(), edges: l.edges.clone() }]).unwrap_or_default(),
            SemioBrepMutation::AddFace { face } => vec![SemioBrepMutation::RemoveFace { id: face.id.clone() }],
            SemioBrepMutation::RemoveFace { id } => base.faces.iter().find(|f| &f.id == id).map(|f| vec![SemioBrepMutation::AddFace { face: f.clone() }]).unwrap_or_default(),
            SemioBrepMutation::SetFaceSurface { id, .. } => base.faces.iter().find(|f| &f.id == id).map(|f| vec![SemioBrepMutation::SetFaceSurface { id: id.clone(), surface: f.surface.clone() }]).unwrap_or_default(),
            SemioBrepMutation::SetFaceOrientation { id, .. } => base.faces.iter().find(|f| &f.id == id).map(|f| vec![SemioBrepMutation::SetFaceOrientation { id: id.clone(), orientation: f.orientation }]).unwrap_or_default(),
            SemioBrepMutation::SetFaceLoops { id, .. } => base.faces.iter().find(|f| &f.id == id)
                .map(|f| vec![SemioBrepMutation::SetFaceLoops { id: id.clone(), outer_loop: f.outer_loop.clone(), inner_loops: f.inner_loops.clone() }])
                .unwrap_or_default(),
            SemioBrepMutation::AddShell { shell } => vec![SemioBrepMutation::RemoveShell { id: shell.id.clone() }],
            SemioBrepMutation::RemoveShell { id } => base.shells.iter().find(|s| &s.id == id).map(|s| vec![SemioBrepMutation::AddShell { shell: s.clone() }]).unwrap_or_default(),
            SemioBrepMutation::SetShellFaces { id, .. } => base.shells.iter().find(|s| &s.id == id).map(|s| vec![SemioBrepMutation::SetShellFaces { id: id.clone(), faces: s.faces.clone() }]).unwrap_or_default(),
            SemioBrepMutation::AddSolid { solid } => vec![SemioBrepMutation::RemoveSolid { id: solid.id.clone() }],
            SemioBrepMutation::RemoveSolid { id } => base.solids.iter().find(|s| &s.id == id).map(|s| vec![SemioBrepMutation::AddSolid { solid: s.clone() }]).unwrap_or_default(),
            SemioBrepMutation::SetSolidShells { id, .. } => base.solids.iter().find(|s| &s.id == id).map(|s| vec![SemioBrepMutation::SetSolidShells { id: id.clone(), shells: s.shells.clone() }]).unwrap_or_default(),
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
pub fn apply_semio_brep_mutation(snapshot: &mut SemioBrepSnapshot, mutation: &SemioBrepMutation) -> SemioBrepDiff {
    let diff = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(mutation, snapshot);
    *snapshot = <SemioBrepDiff as protocol::MutationDiff<SemioBrepSnapshot>>::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Diff

//#region OpCodecs
/// 🎙️ Handcrafted `OpText`/`OpBinary`: plain `serde_json` round-trip of the whole enum (one line
/// of compact JSON per op), the same "JSON-pack passthrough" honesty boundary the subset's own
/// `ArtifactPack` impl uses. Deliberately NOT `#[derive(dsl::DslOps)]` + `#[dsl(block)]` (the
/// grammar/hand-rolled-op-triple path every OTHER artifact's real mutation vocabulary uses) —
/// that path requires the embedded snapshot type to itself implement `dsl::DslField` (via
/// `dsl::DslRecord`), which spans every nested type in the snapshot tree and is out of this
/// wave's scope per the f6 §4 dsl-derive gaps (generics E0107 on collection-diff types, no
/// `DslField` for `NamedTripleDiff`).
impl protocol::OpText for SemioBrepMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl protocol::OpBinary for SemioBrepMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
//#endregion OpCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧱️ One populated item per collection, self-referentially consistent, so every `Remove*`/
    /// `Set*` variant below has something real to act against.
    fn populated_snapshot() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } }];
        s.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }];
        s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
        s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true }];
        s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
        s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
        s
    }

    fn all_mutations(base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
        vec![
            SemioBrepMutation::SetSnapshot { snapshot: { let mut s = base.clone(); s.vertices[0].point = SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }; s } },
            SemioBrepMutation::AddVertex { vertex: BrepVertex { id: "v-new".into(), point: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } } },
            SemioBrepMutation::RemoveVertex { id: "v1".into() },
            SemioBrepMutation::SetVertexPoint { id: "v1".into(), point: SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 } },
            SemioBrepMutation::AddEdge { edge: BrepEdge { id: "e-new".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.0 } } },
            SemioBrepMutation::RemoveEdge { id: "e1".into() },
            SemioBrepMutation::SetEdgeEndpoints { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into() },
            SemioBrepMutation::SetEdgeCurve { id: "e1".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 3.0 } },
            SemioBrepMutation::AddLoop { brep_loop: BrepLoop { id: "l-new".into(), edges: vec![] } },
            SemioBrepMutation::RemoveLoop { id: "l1".into() },
            SemioBrepMutation::SetLoopEdges { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] },
            SemioBrepMutation::AddFace { face: BrepFace { id: "f-new".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Sphere { center: SemioPoint3::default(), radius: 1.0 }, orientation: true } },
            SemioBrepMutation::RemoveFace { id: "f1".into() },
            SemioBrepMutation::SetFaceSurface { id: "f1".into(), surface: BrepSurface::Sphere { center: SemioPoint3::default(), radius: 2.0 } },
            SemioBrepMutation::SetFaceOrientation { id: "f1".into(), orientation: false },
            SemioBrepMutation::SetFaceLoops { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec!["l1".into()] },
            SemioBrepMutation::AddShell { shell: BrepShell { id: "s-new".into(), faces: vec![] } },
            SemioBrepMutation::RemoveShell { id: "s1".into() },
            SemioBrepMutation::SetShellFaces { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: false }] },
            SemioBrepMutation::AddSolid { solid: BrepSolid { id: "so-new".into(), shells: vec![] } },
            SemioBrepMutation::RemoveSolid { id: "so1".into() },
            SemioBrepMutation::SetSolidShells { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: true }] },
        ]
    }

    /// 🧪️ mutation_diff_law: ∀ variant, `apply_semio_brep_mutation`'s returned diff equals
    /// `mutation.diff(base)`, and applying that diff to `base` equals the in-place mutated
    /// snapshot.
    #[test]
    fn mutation_diff_law_covers_every_variant() {
        let base = populated_snapshot();
        for m in all_mutations(&base) {
            let expected_diff = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(&m, &base);
            let mut applied = base.clone();
            let returned_diff = apply_semio_brep_mutation(&mut applied, &m);
            assert_eq!(returned_diff, expected_diff, "mutation {m:?} returned diff mismatch");
            assert_eq!(<SemioBrepDiff as protocol::MutationDiff<SemioBrepSnapshot>>::apply(&expected_diff, &base), applied, "mutation {m:?} apply mismatch");
        }
    }

    /// 🧪️ inverse_law (mutation level): ∀ variant, applying the mutation then every inverse
    /// mutation restores `base`.
    #[test]
    fn inverse_law_mutation_level_round_trips_every_variant() {
        let base = populated_snapshot();
        for m in all_mutations(&base) {
            let mut s = base.clone();
            let _ = apply_semio_brep_mutation(&mut s, &m);
            let invs = <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::inverse(&m, &base);
            assert!(!invs.is_empty(), "mutation {m:?} produced no inverse");
            for inv in invs {
                let _ = apply_semio_brep_mutation(&mut s, &inv);
            }
            assert_eq!(s, base, "mutation {m:?} inverse did not restore base");
        }
    }

    /// 🧪️ op_text_binary_roundtrip_law: handcrafted `OpText`/`OpBinary` JSON round-trip, covering
    /// every variant (incl. `NoMutation`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = populated_snapshot();
        let mut ms = vec![SemioBrepMutation::NoMutation];
        ms.extend(all_mutations(&base));
        for m in ms {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioBrepMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioBrepMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
}
//#endregion 🔖️Tests
