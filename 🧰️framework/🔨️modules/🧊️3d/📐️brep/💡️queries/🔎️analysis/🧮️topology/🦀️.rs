//! 🧮️ Topology counts with Euler characteristic and genus, and the closed / manifold / watertight verdict.

use super::{ScopeMembers, ShapeKind, ShapeScope};
use crate::brep::queries::validation::is_point_edge;
use crate::brep::representation::arena::{ArenaId, EdgeId, FaceId};
use crate::brep::representation::error::KernelError;
use crate::brep::representation::topology::Body;
use std::collections::{BTreeMap, BTreeSet};

/// 🧮️ Entity counts of a shape. `edges` counts every edge; `degenerate_edges` of them collapse to a point
/// (a sphere's poles) and carry no length, so the Euler characteristic ignores them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct TopologyCounts {
    pub solids: usize,
    pub shells: usize,
    pub closed_shells: usize,
    pub faces: usize,
    pub loops: usize,
    pub inner_loops: usize,
    pub coedges: usize,
    pub wires: usize,
    pub edges: usize,
    pub degenerate_edges: usize,
    pub vertices: usize,
    pub euler_characteristic: i64,
    pub genus: Option<i64>,
}

/// 💧 How watertight a shape is, worst problem first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
#[value(rename_all = "camelCase")]
pub enum Watertightness {
    NonManifold,
    Open,
    Misoriented,
    Watertight,
}

/// 🧷 Edge-use census of a shape's faces. Every non-degenerate edge of a watertight shape is used by exactly
/// two coedges that run along it in opposite senses as seen from outside.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ManifoldReport {
    pub closed: bool,
    pub manifold: bool,
    pub consistently_oriented: bool,
    pub boundary_edges: usize,
    pub non_manifold_edges: usize,
    pub misoriented_edges: usize,
    pub verdict: Watertightness,
}

fn effective_senses(body: &Body, faces: &[FaceId]) -> BTreeMap<u32, (EdgeId, Vec<bool>)> {
    let mut uses: BTreeMap<u32, (EdgeId, Vec<bool>)> = BTreeMap::new();
    for &face in faces {
        let flipped = body.faces.get(face).is_some_and(|f| f.flipped);
        for coedge in body.face_coedges(face) {
            if let Some(c) = body.coedges.get(coedge) {
                if is_point_edge(body, c.edge) {
                    continue;
                }
                uses.entry(c.edge.raw_index()).or_insert_with(|| (c.edge, Vec::new())).1.push(c.forward != flipped);
            }
        }
    }
    uses
}

fn census(body: &Body, faces: &[FaceId]) -> ManifoldReport {
    let (mut boundary, mut non_manifold, mut misoriented) = (0, 0, 0);
    for (_, senses) in effective_senses(body, faces).values() {
        match senses.len() {
            2 if senses[0] == senses[1] => misoriented += 1,
            2 => {}
            1 => boundary += 1,
            _ => non_manifold += 1,
        }
    }
    let verdict = if non_manifold > 0 {
        Watertightness::NonManifold
    } else if boundary > 0 || faces.is_empty() {
        Watertightness::Open
    } else if misoriented > 0 {
        Watertightness::Misoriented
    } else {
        Watertightness::Watertight
    };
    ManifoldReport { closed: !faces.is_empty() && boundary == 0, manifold: non_manifold == 0, consistently_oriented: misoriented == 0, boundary_edges: boundary, non_manifold_edges: non_manifold, misoriented_edges: misoriented, verdict }
}

/// 🚰 The closed / manifold / watertight census of `scope`'s faces.
///
/// A shape with no faces (a vertex, an edge, a wire) is `Open`.
pub fn manifold_report(body: &Body, scope: &ShapeScope) -> Result<ManifoldReport, KernelError> {
    let members = scope.members(body)?;
    Ok(census(body, &members.faces))
}

struct ShellFigures {
    euler: i64,
    closed: bool,
}

fn shell_figures(body: &Body, faces: &[FaceId]) -> ShellFigures {
    let mut vertices = BTreeSet::new();
    let mut edges = BTreeSet::new();
    let mut inner = 0i64;
    for &face in faces {
        inner += body.faces.get(face).map_or(0, |f| f.inners.len()) as i64;
        for coedge in body.face_coedges(face) {
            let Some(c) = body.coedges.get(coedge) else { continue };
            let Some(edge) = body.edges.get(c.edge) else { continue };
            vertices.insert(edge.v0.raw_index());
            vertices.insert(edge.v1.raw_index());
            if !is_point_edge(body, c.edge) {
                edges.insert(c.edge.raw_index());
            }
        }
    }
    let report = census(body, faces);
    ShellFigures { euler: vertices.len() as i64 - edges.len() as i64 + faces.len() as i64 - inner, closed: report.closed && report.manifold }
}

/// 🔢️ Counts the entities of `scope` and derives Euler characteristic and genus.
///
/// `euler_characteristic = V - E + F - R` over the non-degenerate edges, with `R` the inner loops (a face
/// with a hole is an annulus, not a disk). `genus` is the sum over the shape's shells of
/// `(2 - chi) / 2`, present only when every shell is closed and manifold.
pub fn topology_counts(body: &Body, scope: &ShapeScope) -> Result<TopologyCounts, KernelError> {
    let members: ScopeMembers = scope.members(body)?;
    let degenerate = members.edges.iter().filter(|&&edge| is_point_edge(body, edge)).count();
    let inner_loops: usize = members.faces.iter().map(|&face| body.faces.get(face).map_or(0, |f| f.inners.len())).sum();
    let euler = members.vertices.len() as i64 - (members.edges.len() - degenerate) as i64 + members.faces.len() as i64 - inner_loops as i64;
    let mut closed_shells = 0;
    let mut genus_sum = Some(0i64);
    for &shell in &members.shells {
        let figures = shell_figures(body, &body.shell_faces(shell));
        if figures.closed {
            closed_shells += 1;
        }
        let shell_genus = (figures.closed && (2 - figures.euler) % 2 == 0 && figures.euler <= 2).then(|| (2 - figures.euler) / 2);
        genus_sum = match (genus_sum, shell_genus) {
            (Some(total), Some(g)) => Some(total + g),
            _ => None,
        };
    }
    let has_shells = !members.shells.is_empty();
    Ok(TopologyCounts {
        solids: members.solids.len(),
        shells: members.shells.len(),
        closed_shells,
        faces: members.faces.len(),
        loops: members.loops.len(),
        inner_loops,
        coedges: members.coedges.len(),
        wires: usize::from(scope.kind == ShapeKind::Wire),
        edges: members.edges.len(),
        degenerate_edges: degenerate,
        vertices: members.vertices.len(),
        euler_characteristic: euler,
        genus: if has_shells { genus_sum } else { None },
    })
}
