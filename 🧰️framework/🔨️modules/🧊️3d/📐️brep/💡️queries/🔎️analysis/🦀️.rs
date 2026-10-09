//! 🔎️ Pure B-Rep analysis queries over a [`Body`] and a [`ShapeScope`]: mass properties with principal
//! inertia, tight bounds, topology counts with Euler characteristic and genus, scoped validity, the
//! closed/manifold/watertight verdict, surface differential geometry, per-face and per-edge tables and
//! distances. A session handle resolves to a scope (see `Brep::analyze_sync`); a consumer holding only a
//! `Body` builds the scope itself, so nothing here needs a kernel session.
//!
//! Conventions: unit density; normals are the face's outward normal (`flipped` applied); curvature is
//! positive where the surface bends away from its outward normal (a solid sphere is `+1/r`).
//!
//! 🔗️ [Euler-Poincare formula](https://en.wikipedia.org/wiki/Euler_characteristic) ·
//! [Principal curvature](https://en.wikipedia.org/wiki/Principal_curvature)

use crate::brep::representation::arena::{ArenaId, CoedgeId, EdgeId, FaceId, LoopId, ShellId, SolidId, VertexId};
use crate::brep::representation::error::KernelError;
use crate::brep::representation::topology::{Body, EntityRef};

#[path = "🛞️mass/🦀️.rs"]
mod mass;
pub use mass::{mass_properties, ShapeMassProperties, VolumetricMass};
#[path = "📦️bounds/🦀️.rs"]
mod bounds;
pub use bounds::{bounding_box, ShapeBounds};
#[path = "🧮️topology/🦀️.rs"]
mod topology;
pub use topology::{manifold_report, topology_counts, ManifoldReport, TopologyCounts, Watertightness};
#[path = "🩺validity/🦀️.rs"]
mod validity;
pub use validity::{validity, IssueSeverity, ValidityIssue, ValidityReport};
#[path = "🌀curvature/🦀️.rs"]
mod curvature;
pub use curvature::{face_differential, surface_differential, SurfaceDifferential};
#[path = "📋tables/🦀️.rs"]
mod tables;
pub use tables::{edge_table, face_table, Convexity, CurveKind, Dihedral, EdgeRow, FaceRow, SurfaceKind};
#[path = "📏distance/🦀️.rs"]
mod distance;
pub use distance::{point_distance, shape_distance, ClosestPair, PointDistance};

/// 🧭️ What kind of shape a [`ShapeScope`] names.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
#[value(rename_all = "camelCase")]
pub enum ShapeKind {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    Compound,
}

/// 🎯️ The part of a [`Body`] one analysis is about: its kind and the arena roots it reaches from.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapeScope {
    pub kind: ShapeKind,
    pub roots: Vec<EntityRef>,
}

/// 🗂️ Every arena entity a scope reaches, each list ascending by arena index so output order is stable.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScopeMembers {
    pub solids: Vec<SolidId>,
    pub shells: Vec<ShellId>,
    pub faces: Vec<FaceId>,
    pub loops: Vec<LoopId>,
    pub coedges: Vec<CoedgeId>,
    pub edges: Vec<EdgeId>,
    pub vertices: Vec<VertexId>,
}

impl ShapeScope {
    pub fn vertex(id: VertexId) -> Self {
        Self { kind: ShapeKind::Vertex, roots: vec![EntityRef::Vertex(id)] }
    }
    pub fn edge(id: EdgeId) -> Self {
        Self { kind: ShapeKind::Edge, roots: vec![EntityRef::Edge(id)] }
    }
    pub fn wire(edges: &[EdgeId], vertices: &[VertexId]) -> Self {
        Self { kind: ShapeKind::Wire, roots: edges.iter().map(|id| EntityRef::Edge(*id)).chain(vertices.iter().map(|id| EntityRef::Vertex(*id))).collect() }
    }
    pub fn face(id: FaceId) -> Self {
        Self { kind: ShapeKind::Face, roots: vec![EntityRef::Face(id)] }
    }
    pub fn shell(id: ShellId) -> Self {
        Self { kind: ShapeKind::Shell, roots: vec![EntityRef::Shell(id)] }
    }
    pub fn solid(id: SolidId) -> Self {
        Self { kind: ShapeKind::Solid, roots: vec![EntityRef::Solid(id)] }
    }
    pub fn compound(solids: &[SolidId]) -> Self {
        Self { kind: ShapeKind::Compound, roots: solids.iter().map(|id| EntityRef::Solid(*id)).collect() }
    }

    /// 🧱 The entities this scope reaches; fails when a root no longer exists in `body`.
    pub fn members(&self, body: &Body) -> Result<ScopeMembers, KernelError> {
        for root in &self.roots {
            let live = match *root {
                EntityRef::Vertex(id) => body.vertices.get(id).is_some(),
                EntityRef::Edge(id) => body.edges.get(id).is_some(),
                EntityRef::Face(id) => body.faces.get(id).is_some(),
                EntityRef::Shell(id) => body.shells.get(id).is_some(),
                EntityRef::Solid(id) => body.solids.get(id).is_some(),
                _ => true,
            };
            if !live {
                return Err(KernelError::MissingEntity(format!("{:?} root no longer exists", self.kind)));
            }
        }
        let reach = body.reachable_from(&self.roots);
        fn sorted<T: ArenaId + Copy + Ord>(set: &crate::brep::representation::topology::HistoryFoldSet<T>) -> Vec<T> {
            let mut ids: Vec<T> = set.iter().copied().collect();
            ids.sort_unstable_by_key(|id| id.raw_index());
            ids
        }
        Ok(ScopeMembers { solids: sorted(&reach.solids), shells: sorted(&reach.shells), faces: sorted(&reach.faces), loops: sorted(&reach.loops), coedges: sorted(&reach.coedges), edges: sorted(&reach.edges), vertices: sorted(&reach.vertices) })
    }
}

/// 🔎️ Everything the analysis queries know about one shape, so an inference can hold one value.
///
/// A part that does not apply to the shape (the volume of an open shell, the faces of an edge) is
/// `None` or empty and its reason is listed in `unavailable`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct ShapeAnalysis {
    pub kind: ShapeKind,
    pub mass: Option<ShapeMassProperties>,
    pub bounds: Option<ShapeBounds>,
    pub topology: TopologyCounts,
    pub manifold: ManifoldReport,
    pub validity: ValidityReport,
    pub faces: Vec<FaceRow>,
    pub edges: Vec<EdgeRow>,
    pub unavailable: Vec<String>,
}

/// 🗃️ Runs every analysis query on `scope`; `tolerance` is the relative quadrature tolerance of the mass integrals.
pub fn analyze_shape(body: &Body, scope: &ShapeScope, tolerance: f64) -> Result<ShapeAnalysis, KernelError> {
    let mut unavailable = Vec::new();
    let mass = keep(&mut unavailable, "mass", mass_properties(body, scope, tolerance));
    let bounds = keep(&mut unavailable, "bounds", bounding_box(body, scope));
    let faces = keep(&mut unavailable, "faces", face_table(body, scope, tolerance)).unwrap_or_default();
    let edges = keep(&mut unavailable, "edges", edge_table(body, scope)).unwrap_or_default();
    Ok(ShapeAnalysis { kind: scope.kind, mass, bounds, topology: topology_counts(body, scope)?, manifold: manifold_report(body, scope)?, validity: validity(body, scope)?, faces, edges, unavailable })
}

fn keep<T>(unavailable: &mut Vec<String>, what: &str, outcome: Result<T, KernelError>) -> Option<T> {
    match outcome {
        Ok(value) => Some(value),
        Err(error) => {
            unavailable.push(format!("{what}: {error}"));
            None
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
