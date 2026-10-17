//! 🚚 Session side of the standalone shape value: [`Brep::export_shape`] captures the closure of one live handle as an owned
//! [`ShapeValue`] and [`Brep::import_shape`] / [`Brep::import_shape_mapped`] registers one in a session again.

use super::{entity_roots, label_of_entity, Brep, BrepError, Entity, GeometryHandle, GeometryKind};
use semio_framework_3d_brep_representation::topology::history::{LabelSource, PersistentLabel};
use semio_framework_3d_brep_representation::topology::{Body, Wire};
use semio_framework_3d_brep_shape_value::{ImportedShape, ShapeRoot, ShapeValue, ShapeWire};

/// 🎯️ Highest label carried by a body's entities or the root, plus one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn label_high_water(body: &Body, root: PersistentLabel) -> u64 {
    body.vertices.iter().map(|(_, vertex)| vertex.label)
        .chain(body.edges.iter().map(|(_, edge)| edge.label))
        .chain(body.faces.iter().map(|(_, face)| face.label))
        .chain(body.shells.iter().map(|(_, shell)| shell.label))
        .chain(body.solids.iter().map(|(_, solid)| solid.label))
        .chain(std::iter::once(root))
        .map(|label| label.0 + 1)
        .max()
        .unwrap_or(0)
}

impl Brep {
    /// 📤️ Exports `handle` as a session-free [`ShapeValue`] holding exactly the closure of its roots, every [`PersistentLabel`] preserved and the arena compacted to dense ids in index order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn export_shape(&self, handle: &GeometryHandle) -> Result<ShapeValue, BrepError> {
        let entity = self.entity(handle)?;
        let label = label_of_entity(&self.body, entity).ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))?;
        let reach = self.body.reachable_from(&entity_roots(entity));
        let mut body = Body::new();
        let map = body.merge_selected(&self.body, Some(&reach));
        let lost = || BrepError::MissingHandle(handle.as_str().to_string());
        let root = match entity {
            Entity::Vertex(id) => ShapeRoot::Vertex(*map.vertices.get(id).ok_or_else(lost)?),
            Entity::Edge(id) => ShapeRoot::Edge(*map.edges.get(id).ok_or_else(lost)?),
            Entity::Face(id) => ShapeRoot::Face(*map.faces.get(id).ok_or_else(lost)?),
            Entity::Shell(id) => ShapeRoot::Shell(*map.shells.get(id).ok_or_else(lost)?),
            Entity::Solid(id) => ShapeRoot::Solid(*map.solids.get(id).ok_or_else(lost)?),
            Entity::Wire(wire, label) => ShapeRoot::Wire(ShapeWire {
                members: wire.members.iter().map(|(edge, forward)| map.edges.get(edge).map(|edge| (*edge, *forward)).ok_or_else(lost)).collect::<Result<_, _>>()?,
                vertices: wire.vertices.iter().map(|vertex| map.vertices.get(vertex).copied().ok_or_else(lost)).collect::<Result<_, _>>()?,
                closed: wire.closed,
                label: *label,
            }),
            Entity::Compound(solids, label) => ShapeRoot::Compound { solids: solids.iter().map(|solid| map.solids.get(solid).copied().ok_or_else(lost)).collect::<Result<_, _>>()?, label: *label },
            Entity::Curve(curve, label) => ShapeRoot::Curve { curve: curve.clone(), label: *label },
            Entity::Surface(surface, label) => ShapeRoot::Surface { surface: surface.clone(), label: *label },
        };
        body.labels = LabelSource::from_next(label_high_water(&body, label));
        Ok(ShapeValue { root, body })
    }

    /// 📥️ Imports `shape` into this session and returns the root handle; in a fresh session every label, and therefore every handle, equals the exported one.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_shape(&mut self, shape: &ShapeValue) -> Result<GeometryHandle, BrepError> {
        self.import_shape_mapped(shape).map(|imported| imported.handle)
    }

    /// 📥️ [`Brep::import_shape`] that also reports the label shift, so labels held against the value resolve in a non-fresh session.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn import_shape_mapped(&mut self, shape: &ShapeValue) -> Result<ImportedShape, BrepError> {
        shape.check()?;
        let offset = self.body.labels.next();
        let map = self.body.merge(&shape.body);
        let relabel = |label: PersistentLabel| PersistentLabel(label.0 + offset);
        let lost = || BrepError::InvalidInput("shape value root does not resolve".into());
        let handle = match &shape.root {
            ShapeRoot::Vertex(id) => self.mint(GeometryKind::Vertex, Entity::Vertex(*map.vertices.get(id).ok_or_else(lost)?)),
            ShapeRoot::Edge(id) => self.mint(GeometryKind::Edge, Entity::Edge(*map.edges.get(id).ok_or_else(lost)?)),
            ShapeRoot::Face(id) => self.register_face(*map.faces.get(id).ok_or_else(lost)?),
            ShapeRoot::Shell(id) => self.register_shell(*map.shells.get(id).ok_or_else(lost)?),
            ShapeRoot::Solid(id) => self.register_solid(*map.solids.get(id).ok_or_else(lost)?),
            ShapeRoot::Wire(wire) => {
                let wire_value = Wire {
                    members: wire.members.iter().map(|(edge, forward)| map.edges.get(edge).map(|edge| (*edge, *forward)).ok_or_else(lost)).collect::<Result<_, _>>()?,
                    vertices: wire.vertices.iter().map(|vertex| map.vertices.get(vertex).copied().ok_or_else(lost)).collect::<Result<_, _>>()?,
                    closed: wire.closed,
                };
                self.mint(GeometryKind::Wire, Entity::Wire(wire_value, relabel(wire.label)))
            }
            ShapeRoot::Compound { solids, label } => {
                let solids = solids.iter().map(|solid| map.solids.get(solid).copied().ok_or_else(lost)).collect::<Result<_, _>>()?;
                self.mint(GeometryKind::Compound, Entity::Compound(solids, relabel(*label)))
            }
            ShapeRoot::Curve { curve, label } => self.mint(GeometryKind::Curve, Entity::Curve(curve.clone(), relabel(*label))),
            ShapeRoot::Surface { surface, label } => self.mint(GeometryKind::Surface, Entity::Surface(surface.clone(), relabel(*label))),
        };
        Ok(ImportedShape { handle, label_offset: offset })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
