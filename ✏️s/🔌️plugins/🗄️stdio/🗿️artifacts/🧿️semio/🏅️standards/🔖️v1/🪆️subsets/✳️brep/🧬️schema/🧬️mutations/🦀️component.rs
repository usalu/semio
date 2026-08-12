//! 🧬️ SemioBrepMutation — named-variant enum (one `Add*`/`Remove*`/`Set*` triad per collection,
//! plus `SetSnapshot`), following the gif 89a / docx precedent. Every variant's `diff()`/
//! `inverse()` is HAND-WRITTEN below (never apply-and-capture — schema-design.md's warning: svg's
//! original bug was computing diffs via clone+apply+re-diff causing infinite mutual recursion
//! once `mutate` was flipped to return `(Self, Diff)`).

use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::engine::triples::{split_top_level, strip_brackets, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{
    dec_curve, dec_edge, dec_face, dec_list, dec_loop, dec_loop_edge, dec_point3, dec_shell, dec_shell_face, dec_solid, dec_solid_shell, dec_str,
    dec_surface, dec_vertex, enc_bool, enc_curve, enc_edge, enc_face, enc_list, enc_loop, enc_loop_edge, enc_point3, enc_shell, enc_shell_face,
    enc_solid, enc_solid_shell, enc_str, enc_surface, enc_vertex, parse_bool, BrepEdgeDiff, BrepFaceDiff, BrepLoopDiff, BrepShellDiff, BrepSolidDiff,
    BrepVertexDiff, SemioBrepDiff,
};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{
    BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell,
    BrepSurface, BrepVertex, SemioBrepSnapshot,
};
use protocol::command::DiffAlgebra;
use protocol::Mutation;
/// 🔧️ Unconditional — the non-test `impl protocol::OpBinary for SemioBrepMutation` block below
/// calls `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in scope
/// in production code too, not merely under `#[cfg(test)]` (same fix workflow's own mutations
/// facet needed).
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
/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION brep wave: real hand-rolled
/// `OpText`/`OpBinary`, replacing the old whole-enum compact-`serde_json` passthrough. Grammar:
/// `keyword arg=value ...` (space-separated), reusing the sibling `🔺️diff` facet's now-`pub(crate)`
/// hex/value primitives (`enc_str`/`enc_point3`/`enc_curve`/`enc_surface`/`enc_vertex`/`enc_edge`/
/// `enc_loop`/`enc_face`/`enc_shell`/`enc_solid`/`enc_list`/...) rather than re-deriving a second
/// independent copy — one source of truth for the entity encoding, same convention workflow's own
/// mutations facet established (importing from its sibling `schema::diff`).
fn enc_brep_snapshot(s: &SemioBrepSnapshot) -> String {
    format!(
        "[{},{},{},{},{},{},{}]",
        enc_str(&s.schema),
        enc_list(&s.vertices, enc_vertex),
        enc_list(&s.edges, enc_edge),
        enc_list(&s.loops, enc_loop),
        enc_list(&s.faces, enc_face),
        enc_list(&s.shells, enc_shell),
        enc_list(&s.solids, enc_solid),
    )
}
fn dec_brep_snapshot(s: &str) -> Result<SemioBrepSnapshot, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [schema, vertices, edges, loops, faces, shells, solids] = parts.as_slice() else {
        return Err(format!("snapshot: expected 7 fields, got {}", parts.len()));
    };
    Ok(SemioBrepSnapshot {
        schema: dec_str(schema)?,
        vertices: dec_list(vertices, dec_vertex)?,
        edges: dec_list(edges, dec_edge)?,
        loops: dec_list(loops, dec_loop)?,
        faces: dec_list(faces, dec_face)?,
        shells: dec_list(shells, dec_shell)?,
        solids: dec_list(solids, dec_solid)?,
    })
}

fn print_brep_mutation(m: &SemioBrepMutation) -> String {
    match m {
        SemioBrepMutation::NoMutation => "no-mutation".to_string(),
        SemioBrepMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_brep_snapshot(snapshot)),
        SemioBrepMutation::AddVertex { vertex } => format!("add-vertex vertex={}", enc_vertex(vertex)),
        SemioBrepMutation::RemoveVertex { id } => format!("remove-vertex id={}", enc_str(id)),
        SemioBrepMutation::SetVertexPoint { id, point } => format!("set-vertex-point id={} point={}", enc_str(id), enc_point3(point)),
        SemioBrepMutation::AddEdge { edge } => format!("add-edge edge={}", enc_edge(edge)),
        SemioBrepMutation::RemoveEdge { id } => format!("remove-edge id={}", enc_str(id)),
        SemioBrepMutation::SetEdgeEndpoints { id, start_vertex, end_vertex } => format!("set-edge-endpoints id={} start={} end={}", enc_str(id), enc_str(start_vertex), enc_str(end_vertex)),
        SemioBrepMutation::SetEdgeCurve { id, curve } => format!("set-edge-curve id={} curve={}", enc_str(id), enc_curve(curve)),
        SemioBrepMutation::AddLoop { brep_loop } => format!("add-loop loop={}", enc_loop(brep_loop)),
        SemioBrepMutation::RemoveLoop { id } => format!("remove-loop id={}", enc_str(id)),
        SemioBrepMutation::SetLoopEdges { id, edges } => format!("set-loop-edges id={} edges={}", enc_str(id), enc_list(edges, enc_loop_edge)),
        SemioBrepMutation::AddFace { face } => format!("add-face face={}", enc_face(face)),
        SemioBrepMutation::RemoveFace { id } => format!("remove-face id={}", enc_str(id)),
        SemioBrepMutation::SetFaceSurface { id, surface } => format!("set-face-surface id={} surface={}", enc_str(id), enc_surface(surface)),
        SemioBrepMutation::SetFaceOrientation { id, orientation } => format!("set-face-orientation id={} orientation={}", enc_str(id), enc_bool(*orientation)),
        SemioBrepMutation::SetFaceLoops { id, outer_loop, inner_loops } => format!("set-face-loops id={} outer={} inner={}", enc_str(id), enc_str(outer_loop), enc_list(inner_loops, |s: &String| enc_str(s))),
        SemioBrepMutation::AddShell { shell } => format!("add-shell shell={}", enc_shell(shell)),
        SemioBrepMutation::RemoveShell { id } => format!("remove-shell id={}", enc_str(id)),
        SemioBrepMutation::SetShellFaces { id, faces } => format!("set-shell-faces id={} faces={}", enc_str(id), enc_list(faces, enc_shell_face)),
        SemioBrepMutation::AddSolid { solid } => format!("add-solid solid={}", enc_solid(solid)),
        SemioBrepMutation::RemoveSolid { id } => format!("remove-solid id={}", enc_str(id)),
        SemioBrepMutation::SetSolidShells { id, shells } => format!("set-solid-shells id={} shells={}", enc_str(id), enc_list(shells, enc_solid_shell)),
    }
}
fn parse_brep_mutation(line: &str) -> Result<SemioBrepMutation, String> {
    if line == "no-mutation" {
        return Ok(SemioBrepMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("brep mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("brep mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(SemioBrepMutation::SetSnapshot { snapshot: dec_brep_snapshot(arg("snapshot")?)? }),
        "add-vertex" => Ok(SemioBrepMutation::AddVertex { vertex: dec_vertex(arg("vertex")?)? }),
        "remove-vertex" => Ok(SemioBrepMutation::RemoveVertex { id: dec_str(arg("id")?)? }),
        "set-vertex-point" => Ok(SemioBrepMutation::SetVertexPoint { id: dec_str(arg("id")?)?, point: dec_point3(arg("point")?)? }),
        "add-edge" => Ok(SemioBrepMutation::AddEdge { edge: dec_edge(arg("edge")?)? }),
        "remove-edge" => Ok(SemioBrepMutation::RemoveEdge { id: dec_str(arg("id")?)? }),
        "set-edge-endpoints" => Ok(SemioBrepMutation::SetEdgeEndpoints { id: dec_str(arg("id")?)?, start_vertex: dec_str(arg("start")?)?, end_vertex: dec_str(arg("end")?)? }),
        "set-edge-curve" => Ok(SemioBrepMutation::SetEdgeCurve { id: dec_str(arg("id")?)?, curve: dec_curve(arg("curve")?)? }),
        "add-loop" => Ok(SemioBrepMutation::AddLoop { brep_loop: dec_loop(arg("loop")?)? }),
        "remove-loop" => Ok(SemioBrepMutation::RemoveLoop { id: dec_str(arg("id")?)? }),
        "set-loop-edges" => Ok(SemioBrepMutation::SetLoopEdges { id: dec_str(arg("id")?)?, edges: dec_list(arg("edges")?, dec_loop_edge)? }),
        "add-face" => Ok(SemioBrepMutation::AddFace { face: dec_face(arg("face")?)? }),
        "remove-face" => Ok(SemioBrepMutation::RemoveFace { id: dec_str(arg("id")?)? }),
        "set-face-surface" => Ok(SemioBrepMutation::SetFaceSurface { id: dec_str(arg("id")?)?, surface: dec_surface(arg("surface")?)? }),
        "set-face-orientation" => Ok(SemioBrepMutation::SetFaceOrientation { id: dec_str(arg("id")?)?, orientation: parse_bool(arg("orientation")?)? }),
        "set-face-loops" => Ok(SemioBrepMutation::SetFaceLoops { id: dec_str(arg("id")?)?, outer_loop: dec_str(arg("outer")?)?, inner_loops: dec_list(arg("inner")?, dec_str)? }),
        "add-shell" => Ok(SemioBrepMutation::AddShell { shell: dec_shell(arg("shell")?)? }),
        "remove-shell" => Ok(SemioBrepMutation::RemoveShell { id: dec_str(arg("id")?)? }),
        "set-shell-faces" => Ok(SemioBrepMutation::SetShellFaces { id: dec_str(arg("id")?)?, faces: dec_list(arg("faces")?, dec_shell_face)? }),
        "add-solid" => Ok(SemioBrepMutation::AddSolid { solid: dec_solid(arg("solid")?)? }),
        "remove-solid" => Ok(SemioBrepMutation::RemoveSolid { id: dec_str(arg("id")?)? }),
        "set-solid-shells" => Ok(SemioBrepMutation::SetSolidShells { id: dec_str(arg("id")?)?, shells: dec_list(arg("shells")?, dec_solid_shell)? }),
        other => Err(format!("brep mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioBrepMutation {
    fn print_op(&self) -> String {
        print_brep_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_brep_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🏷️ Ordinal table, same declaration order as `SemioBrepMutation`'s own enum variants and
/// `parse_brep_mutation`'s keyword match — the real binary `tag` field's source of truth.
const OP_KEYWORDS: [&str; 23] = [
    "no-mutation",
    "set-snapshot",
    "add-vertex",
    "remove-vertex",
    "set-vertex-point",
    "add-edge",
    "remove-edge",
    "set-edge-endpoints",
    "set-edge-curve",
    "add-loop",
    "remove-loop",
    "set-loop-edges",
    "add-face",
    "remove-face",
    "set-face-surface",
    "set-face-orientation",
    "set-face-loops",
    "add-shell",
    "remove-shell",
    "set-shell-faces",
    "add-solid",
    "remove-solid",
    "set-solid-shells",
];
fn variant_ordinal(m: &SemioBrepMutation) -> u8 {
    match m {
        SemioBrepMutation::NoMutation => 0,
        SemioBrepMutation::SetSnapshot { .. } => 1,
        SemioBrepMutation::AddVertex { .. } => 2,
        SemioBrepMutation::RemoveVertex { .. } => 3,
        SemioBrepMutation::SetVertexPoint { .. } => 4,
        SemioBrepMutation::AddEdge { .. } => 5,
        SemioBrepMutation::RemoveEdge { .. } => 6,
        SemioBrepMutation::SetEdgeEndpoints { .. } => 7,
        SemioBrepMutation::SetEdgeCurve { .. } => 8,
        SemioBrepMutation::AddLoop { .. } => 9,
        SemioBrepMutation::RemoveLoop { .. } => 10,
        SemioBrepMutation::SetLoopEdges { .. } => 11,
        SemioBrepMutation::AddFace { .. } => 12,
        SemioBrepMutation::RemoveFace { .. } => 13,
        SemioBrepMutation::SetFaceSurface { .. } => 14,
        SemioBrepMutation::SetFaceOrientation { .. } => 15,
        SemioBrepMutation::SetFaceLoops { .. } => 16,
        SemioBrepMutation::AddShell { .. } => 17,
        SemioBrepMutation::RemoveShell { .. } => 18,
        SemioBrepMutation::SetShellFaces { .. } => 19,
        SemioBrepMutation::AddSolid { .. } => 20,
        SemioBrepMutation::RemoveSolid { .. } => 21,
        SemioBrepMutation::SetSolidShells { .. } => 22,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_brep_mutation` (empty for `no-mutation`) —
/// the binary frame's `tag` byte already carries the keyword, so the text keyword itself is
/// redundant in the binary payload.
fn print_brep_mutation_args(m: &SemioBrepMutation) -> String {
    match print_brep_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old whole-enum compact-`serde_json::to_vec` shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`OP_KEYWORDS`]) are two REAL fixed fields; the variant's own `key=value ...` argument payload
/// follows as one opaque trailing `bytes` chain — reusing the already-real, already-tested
/// `print_brep_mutation`/`parse_brep_mutation` text codec rather than re-deriving a second
/// independent encoding.
impl protocol::OpBinary for SemioBrepMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_brep_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Shared fixture + representative `SemioBrepMutation` cases (one per variant, incl.
/// `NoMutation`) — single source of truth for this facet's own tests AND
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
fn fixture() -> SemioBrepSnapshot {
    let mut s = SemioBrepSnapshot::default();
    s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } }];
    s.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }];
    s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
    s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true }];
    s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
    s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
    s
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<SemioBrepMutation> {
    let base = fixture();
    vec![
        SemioBrepMutation::NoMutation,
        SemioBrepMutation::SetSnapshot { snapshot: base.clone() },
        SemioBrepMutation::AddVertex { vertex: BrepVertex { id: "v-new".into(), point: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } } },
        SemioBrepMutation::RemoveVertex { id: "v1".into() },
        SemioBrepMutation::SetVertexPoint { id: "v1".into(), point: SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 } },
        SemioBrepMutation::AddEdge { edge: BrepEdge { id: "e-new".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.0 } } },
        SemioBrepMutation::RemoveEdge { id: "e1".into() },
        SemioBrepMutation::SetEdgeEndpoints { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into() },
        SemioBrepMutation::SetEdgeCurve { id: "e1".into(), curve: BrepCurve::Nurbs { control_points: vec![SemioPoint3::default(), SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] } },
        SemioBrepMutation::AddLoop { brep_loop: BrepLoop { id: "l-new".into(), edges: vec![] } },
        SemioBrepMutation::RemoveLoop { id: "l1".into() },
        SemioBrepMutation::SetLoopEdges { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] },
        SemioBrepMutation::AddFace { face: BrepFace { id: "f-new".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Sphere { center: SemioPoint3::default(), radius: 1.0 }, orientation: true } },
        SemioBrepMutation::RemoveFace { id: "f1".into() },
        SemioBrepMutation::SetFaceSurface { id: "f1".into(), surface: BrepSurface::Torus { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, major_radius: 3.0, minor_radius: 1.0 } },
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
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧪️ mutation_diff_law: ∀ variant, `apply_semio_brep_mutation`'s returned diff equals
    /// `mutation.diff(base)`, and applying that diff to `base` equals the in-place mutated
    /// snapshot.
    #[test]
    fn mutation_diff_law_covers_every_variant() {
        let base = fixture();
        for m in demo_mutation_cases() {
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
        let base = fixture();
        for m in demo_mutation_cases() {
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

    /// 🧪️ op_text_binary_roundtrip_law: real `OpText`/`OpBinary` round-trip, covering every
    /// variant (incl. `NoMutation`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        for m in demo_mutation_cases() {
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
