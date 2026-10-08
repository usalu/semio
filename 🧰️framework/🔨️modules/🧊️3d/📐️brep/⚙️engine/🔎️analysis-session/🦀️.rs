//! 🔎️ Analysis queries on a live [`Brep`] session: every handle resolves to a [`ShapeScope`] and the pure
//! queries of `queries::analysis` answer for exactly that shape. The pure functions need no session, so a
//! consumer holding only a [`Body`] calls them directly with a scope it builds itself.

use super::{entity_roots, map_err, Brep, BrepError, Entity, GeometryHandle};
use crate::brep::queries::analysis::{self, ClosestPair, ManifoldReport, PointDistance, ShapeAnalysis, ShapeBounds, ShapeKind, ShapeMassProperties, ShapeScope, SurfaceDifferential, TopologyCounts, ValidityReport};
use crate::brep::representation::vector::Pnt3;

impl Brep {
    /// 🪝 The analysis scope a handle names; curves and surfaces have no topology and are refused.
    pub fn shape_scope(&self, shape: &GeometryHandle) -> Result<ShapeScope, BrepError> {
        match self.entity(shape)? {
            Entity::Vertex(id) => Ok(ShapeScope::vertex(*id)),
            Entity::Edge(id) => Ok(ShapeScope::edge(*id)),
            Entity::Wire(wire, _) => Ok(ShapeScope::wire(&wire.members.iter().map(|(edge, _)| *edge).collect::<Vec<_>>(), &wire.vertices)),
            Entity::Face(id) => Ok(ShapeScope::face(*id)),
            Entity::Shell(id) => Ok(ShapeScope::shell(*id)),
            Entity::Solid(id) => Ok(ShapeScope::solid(*id)),
            entity @ Entity::Compound(_, _) => Ok(ShapeScope { kind: ShapeKind::Compound, roots: entity_roots(entity) }),
            Entity::Curve(_, _) | Entity::Surface(_, _) => Err(BrepError::InvalidInput(format!("{} has no topology to analyse", shape.as_str()))),
        }
    }

    /// 🔎️ Every analysis query for one shape in one value, see [`analysis::analyze_shape`].
    pub fn analyze_sync(&self, shape: &GeometryHandle, tolerance: f64) -> Result<ShapeAnalysis, BrepError> {
        analysis::analyze_shape(&self.body, &self.shape_scope(shape)?, tolerance).map_err(|error| map_err(&error))
    }

    /// 🧮 Area, volume, centroid, inertia tensor and principal moments of a face, shell, solid or compound.
    pub fn mass_properties_sync(&self, shape: &GeometryHandle, tolerance: f64) -> Result<ShapeMassProperties, BrepError> {
        analysis::mass_properties(&self.body, &self.shape_scope(shape)?, tolerance).map_err(|error| map_err(&error))
    }

    /// 🪟 Tight bounds of the selected shape alone, see [`analysis::bounding_box`].
    pub fn bounds_sync(&self, shape: &GeometryHandle) -> Result<ShapeBounds, BrepError> {
        analysis::bounding_box(&self.body, &self.shape_scope(shape)?).map_err(|error| map_err(&error))
    }

    /// 🔢 Entity counts, Euler characteristic and genus of the selected shape.
    pub fn topology_counts_sync(&self, shape: &GeometryHandle) -> Result<TopologyCounts, BrepError> {
        analysis::topology_counts(&self.body, &self.shape_scope(shape)?).map_err(|error| map_err(&error))
    }

    /// 💊 Validity of the selected shape alone; unrelated broken shapes in the session do not count.
    pub fn validity_sync(&self, shape: &GeometryHandle) -> Result<ValidityReport, BrepError> {
        analysis::validity(&self.body, &self.shape_scope(shape)?).map_err(|error| map_err(&error))
    }

    /// 🛁 The closed / manifold / watertight verdict of the selected shape.
    pub fn watertightness_sync(&self, shape: &GeometryHandle) -> Result<ManifoldReport, BrepError> {
        analysis::manifold_report(&self.body, &self.shape_scope(shape)?).map_err(|error| map_err(&error))
    }

    /// 🥄 Principal curvatures, Gaussian and mean curvature and principal directions of a face or a bare
    /// surface at `(u, v)`; a face honours its orientation, a bare surface its natural normal.
    pub fn surface_differential_sync(&self, shape: &GeometryHandle, u: f64, v: f64) -> Result<SurfaceDifferential, BrepError> {
        match self.entity(shape)? {
            Entity::Face(id) => analysis::face_differential(&self.body, *id, u, v),
            Entity::Surface(surface, _) => analysis::surface_differential(surface, false, u, v),
            _ => return Err(BrepError::InvalidInput(format!("{} is not a face or a surface", shape.as_str()))),
        }
        .map_err(|error| map_err(&error))
    }

    /// 🗒 Label, surface kind, area, centroid, normal and neighbouring faces of every face of the shape.
    pub fn face_table_sync(&self, shape: &GeometryHandle, tolerance: f64) -> Result<Vec<analysis::FaceRow>, BrepError> {
        analysis::face_table(&self.body, &self.shape_scope(shape)?, tolerance).map_err(|error| map_err(&error))
    }

    /// 🔗 Label, curve kind, length, adjacent faces and dihedral angle of every edge of the shape.
    pub fn edge_table_sync(&self, shape: &GeometryHandle) -> Result<Vec<analysis::EdgeRow>, BrepError> {
        analysis::edge_table(&self.body, &self.shape_scope(shape)?).map_err(|error| map_err(&error))
    }

    /// 🛤️ Minimum distance between any two shapes with the nearest point on each.
    pub fn shape_distance_sync(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<ClosestPair, BrepError> {
        analysis::shape_distance(&self.body, &self.shape_scope(a)?, &self.shape_scope(b)?).map_err(|error| map_err(&error))
    }

    /// 🪜 Nearest point of any shape to `point`; signed (negative inside) for solids and compounds.
    pub fn point_distance_sync(&self, shape: &GeometryHandle, point: [f64; 3]) -> Result<PointDistance, BrepError> {
        analysis::point_distance(&self.body, &self.shape_scope(shape)?, Pnt3::new(point[0], point[1], point[2])).map_err(|error| map_err(&error))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
