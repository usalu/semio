//! 📚️ Borrowed IFC2x3 access over its native Part-21 instance/value graph.
use super::shared::graph::{GeometryConstituent, GeometryEntity, GeometryGraph, GeometryStandard, GeometryValue};
use semio_s_artifact_stdio_contract::part21::{Part21Document, Part21Value};

pub struct Ifc2x3GeometryGraph<'a>(pub &'a Part21Document);

impl GeometryValue for Part21Value {
    fn as_reference(&self) -> Option<u64> { self.as_ref_id() }
    fn as_str(&self) -> Option<&str> { Part21Value::as_str(self) }
    fn as_enum(&self) -> Option<&str> { Part21Value::as_enum(self) }
    fn as_real(&self) -> Option<f64> { Part21Value::as_real(self) }
    fn as_integer(&self) -> Option<i64> { if let Self::Int(value) = self { Some(*value) } else { None } }
    fn as_aggregate(&self) -> Option<&[Self]> { self.as_list() }
    fn as_typed(&self)->Option<(&str,&[Self])>{Part21Value::as_typed(self)}
    fn is_absent(&self) -> bool { matches!(self, Self::Unset | Self::Derived) }
    fn is_unset(&self) -> bool { matches!(self, Self::Unset) }
    fn is_derived(&self) -> bool { matches!(self, Self::Derived) }
}

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
