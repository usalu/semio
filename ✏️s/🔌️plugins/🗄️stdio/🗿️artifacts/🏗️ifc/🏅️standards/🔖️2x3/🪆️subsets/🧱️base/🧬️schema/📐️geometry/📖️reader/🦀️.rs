//! 📚️ Borrowed IFC2x3 access over its native Part-21 instance/value graph.
use super::shared::graph::{GeometryConstituent, GeometryEntity, GeometryGraph, GeometryStandard};
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Value};

pub struct Ifc2x3GeometryGraph<'a>(pub &'a Part21Document);


impl GeometryGraph for Ifc2x3GeometryGraph<'_> {
    type Value = Part21Value;
    fn standard(&self) -> GeometryStandard { GeometryStandard::Ifc2x3 }
    fn source_identity(&self) -> usize { std::ptr::from_ref(self.0) as usize }
    fn len(&self) -> usize { self.0.instances.len() }
    fn entity(&self, index: usize) -> Option<GeometryEntity<'_, Self::Value>> {
        let instance = self.0.instances.get(index)?;
        let (name, args) = instance.entities.first()?;
        Some(GeometryEntity { id: instance.id, name, args, constituent_count: instance.entities.len() })
    }
    fn constituent(&self, instance: usize, index: usize) -> Option<GeometryConstituent<'_, Self::Value>> {
        let (name, args) = self.0.instances.get(instance)?.entities.get(index)?;
        Some(GeometryConstituent { name, args })
    }
}
