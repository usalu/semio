//! 🪟️ OS text and Pack binding for the actual renderer-neutral viewport records.

use super::{DslField, FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
use semio_framework_ui_viewport::{Viewport2d, Viewport3dOrbit};

fn planar_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "x", Shape::Float), FieldSpec::new(2, "y", Shape::Float), FieldSpec::new(3, "zoom", Shape::Float)])
}

fn orbit_spec() -> RecordSpec {
    let vector = || Shape::Tuple(Box::new(Shape::Float), Some(3));
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "position", vector()), FieldSpec::new(2, "target", vector()), FieldSpec::new(3, "zoom", Shape::Float), FieldSpec::new(4, "up", vector()).optional()])
}

fn record(value: &FieldValue, maximum: u16) -> Result<&RecordValue, String> {
    let FieldValue::Record(record) = value else { return Err("expected viewport record".into()); };
    if record.fields.keys().any(|id| *id == 0 || *id > maximum) { return Err("unknown viewport record field".into()); }
    Ok(record)
}

fn scalar(record: &RecordValue, id: u16) -> Result<f64, String> {
    f64::from_value(record.get(id).ok_or_else(|| format!("missing viewport field {id}"))?)
}

fn vector(record: &RecordValue, id: u16) -> Result<[f64; 3], String> {
    match record.get(id) {
        Some(FieldValue::Tuple(values)) if values.len() == 3 => Ok([f64::from_value(&values[0])?, f64::from_value(&values[1])?, f64::from_value(&values[2])?]),
        _ => Err(format!("expected three coordinates for viewport field {id}")),
    }
}

fn vector_value(value: &[f64; 3]) -> FieldValue { FieldValue::Tuple(value.iter().map(|value| FieldValue::Float(*value)).collect()) }

impl DslField for Viewport2d {
    fn shape() -> Shape { Shape::Record(planar_spec) }
    fn to_value(&self) -> FieldValue {
        FieldValue::Record(RecordValue { fields: [(1, FieldValue::Float(self.x)), (2, FieldValue::Float(self.y)), (3, FieldValue::Float(self.zoom))].into_iter().collect() })
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        let record = record(value, 3)?;
        let pose = Self { x: scalar(record, 1)?, y: scalar(record, 2)?, zoom: scalar(record, 3)? };
        pose.validate().map_err(|error| error.to_string())?;
        Ok(pose)
    }
}

impl DslField for Viewport3dOrbit {
    fn shape() -> Shape { Shape::Record(orbit_spec) }
    fn to_value(&self) -> FieldValue {
        let mut fields = [(1, vector_value(&self.position)), (2, vector_value(&self.target)), (3, FieldValue::Float(self.zoom))].into_iter().collect::<std::collections::HashMap<_, _>>();
        if let Some(up) = &self.up { fields.insert(4, vector_value(up)); }
        FieldValue::Record(RecordValue { fields })
    }
    fn from_value(value: &FieldValue) -> Result<Self, String> {
        let record = record(value, 4)?;
        let up = match record.get(4) { None | Some(FieldValue::Absent) => None, _ => Some(vector(record, 4)?) };
        let pose = Self { position: vector(record, 1)?, target: vector(record, 2)?, zoom: scalar(record, 3)?, up };
        pose.validate().map_err(|error| error.to_string())?;
        Ok(pose)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🪟️poses/🦀️.rs"]
mod tests;
