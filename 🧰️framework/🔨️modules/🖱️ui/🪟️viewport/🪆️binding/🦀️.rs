//! 🪟️ Canonical record binding owned by the renderer-neutral viewport types.

use semio_framework_dsl_record::{DslField,FieldSpec,FieldValue,RecordLayout,RecordSpec,RecordValue,Shape,NativeSchemaControl,RecordSpecProducer,RecordFields};
use semio_framework_value::ValueError;
use crate::{Viewport2d,Viewport3dOrbit};

use semio_framework_dsl_record::{BorrowedDslField,BorrowedDslRecord,BorrowedFieldSpec,BorrowedRecordSpec,BorrowedShape};

impl BorrowedDslRecord for Viewport2d {
    const RECORD:BorrowedRecordSpec = BorrowedRecordSpec { keyword:None,layout:RecordLayout::Inline,fields:&[
        BorrowedFieldSpec::new(1,"x",BorrowedShape::Float),BorrowedFieldSpec::new(2,"y",BorrowedShape::Float),BorrowedFieldSpec::new(3,"zoom",BorrowedShape::Float)
    ] };
}
impl BorrowedDslField for Viewport2d { const SHAPE:BorrowedShape = BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>); }

impl BorrowedDslRecord for Viewport3dOrbit {
    const RECORD:BorrowedRecordSpec = {
        const FIELDS:&[BorrowedFieldSpec] = &[
            BorrowedFieldSpec::new(1,"position",BorrowedShape::Tuple(semio_framework_dsl_record::borrowed_field_shape::<f64>,Some(3))),
            BorrowedFieldSpec::new(2,"target",BorrowedShape::Tuple(semio_framework_dsl_record::borrowed_field_shape::<f64>,Some(3))),
            BorrowedFieldSpec::new(3,"zoom",BorrowedShape::Float),
            BorrowedFieldSpec { optional:true,..BorrowedFieldSpec::new(4,"up",BorrowedShape::Tuple(semio_framework_dsl_record::borrowed_field_shape::<f64>,Some(3))) }
        ];
        BorrowedRecordSpec { keyword:None,layout:RecordLayout::Inline,fields:FIELDS }
    };
}
impl BorrowedDslField for Viewport3dOrbit { const SHAPE:BorrowedShape = BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>); }

fn planar_spec() -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "x", Shape::Float), FieldSpec::new(2, "y", Shape::Float), FieldSpec::new(3, "zoom", Shape::Float)])
}

fn orbit_spec() -> RecordSpec {
    let vector = || Shape::Tuple(Box::new(Shape::Float), Some(3));
    RecordSpec::new(None, RecordLayout::Inline, vec![FieldSpec::new(1, "position", vector()), FieldSpec::new(2, "target", vector()), FieldSpec::new(3, "zoom", Shape::Float), FieldSpec::new(4, "up", vector()).optional()])
}

fn planar_spec_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{control.scoped_stage(|control|{control.begin_stage(3)?;let mut fields=control.allocate_vec(3)?;for(id,key)in[(1,"x"),(2,"y"),(3,"zoom")]{fields.push(semio_framework_dsl_record::producer::field(id,key,Shape::Float,control)?);control.step()?;}semio_framework_dsl_record::producer::record(None,RecordLayout::Inline,fields,control)})}
fn planar_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:planar_spec,decoding:|control|planar_spec_controlled(control),encoding:|control|planar_spec_controlled(control)}}
fn orbit_spec_controlled<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,ValueError>{control.scoped_stage(|control|{control.begin_stage(4)?;let mut fields=control.allocate_vec(4)?;for(id,key)in[(1,"position"),(2,"target"),(3,"zoom"),(4,"up")]{let shape=if id==3{Shape::Float}else{Shape::Tuple(semio_framework_dsl_record::producer::boxed(Shape::Float,control)?,Some(3))};let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;if id==4{field.optional=true;}fields.push(field);control.step()?;}semio_framework_dsl_record::producer::record(None,RecordLayout::Inline,fields,control)})}
fn orbit_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:orbit_spec,decoding:|control|orbit_spec_controlled(control),encoding:|control|orbit_spec_controlled(control)}}

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
    fn shape() -> Shape { Shape::Record(planar_producer()) }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Record(planar_producer()))}
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
    fn shape() -> Shape { Shape::Record(orbit_producer()) }
    fn shape_controlled<C:NativeSchemaControl>(control:&mut C)->Result<Shape,ValueError>{control.checkpoint()?;Ok(Shape::Record(orbit_producer()))}
    fn to_value(&self) -> FieldValue {
        let mut fields = [(1, vector_value(&self.position)), (2, vector_value(&self.target)), (3, FieldValue::Float(self.zoom))].into_iter().collect::<RecordFields>();
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

