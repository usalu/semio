//! 🗣️ Typed map records retain native payload schemas and explicit controlled field ownership.
use super::*;
use semio_framework_dsl_record::{DslField, FieldValue, FieldSpec, RecordValue, RecordSpec, RecordLayout, Shape, RecordSpecProducer, NativeSchemaControl, producer, native_encoding::{self, FieldProjectionView as View, EncodedRecord}};
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, DecodedValue};

fn invalid() -> ValueError { controlled::invalid("map DSL record has incorrect fields") }
fn fields(record: &RecordValue, ids: &[u16]) -> Result<(), ValueError> { if record.fields.len() != ids.len() || ids.iter().any(|id| record.get(*id).is_none()) { Err(invalid()) } else { Ok(()) } }
fn text(record: &RecordValue, id: u16) -> Result<&str, ValueError> { match record.get(id) { Some(FieldValue::Text(value)) => Ok(value), _ => Err(invalid()) } }
fn cold_record(fields: impl IntoIterator<Item = (u16, FieldValue)>) -> RecordValue { RecordValue { fields: fields.into_iter().collect() } }
fn field(id: u16, key: &str, shape: Shape, optional: bool) -> FieldSpec { FieldSpec { id, key: key.into(), position: None, shape, optional, flatten: false, defines: None, is_call_name: false } }


impl<V: semio_framework_dsl_record::BorrowedDslField> semio_framework_dsl_record::BorrowedDslRecord for MapDelta<V> {
    const RECORD: semio_framework_dsl_record::BorrowedRecordSpec = semio_framework_dsl_record::BorrowedRecordSpec {
        keyword: None, layout: RecordLayout::Inline,
        fields: &[semio_framework_dsl_record::BorrowedFieldSpec::new(0, "entries", semio_framework_dsl_record::BorrowedShape::Map(|| semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<MapEntryDelta<V>>)))],
    };
}
impl<V: semio_framework_dsl_record::BorrowedDslField> semio_framework_dsl_record::BorrowedDslRecord for MapEntryDelta<V> {
    const RECORD: semio_framework_dsl_record::BorrowedRecordSpec = semio_framework_dsl_record::BorrowedRecordSpec {
        keyword: None, layout: RecordLayout::Inline,
        fields: &[
            semio_framework_dsl_record::BorrowedFieldSpec::new(0, "operation", semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<MapEntryOperation<V>>)),
            semio_framework_dsl_record::BorrowedFieldSpec::new(1, "precondition", semio_framework_dsl_record::BorrowedShape::Text),
        ],
    };
}
impl<V: semio_framework_dsl_record::BorrowedDslField> semio_framework_dsl_record::BorrowedDslRecord for MapEntryOperation<V> {
    const RECORD: semio_framework_dsl_record::BorrowedRecordSpec = semio_framework_dsl_record::BorrowedRecordSpec {
        keyword: None, layout: RecordLayout::Inline,
        fields: &[
            semio_framework_dsl_record::BorrowedFieldSpec::new(0, "kind", semio_framework_dsl_record::BorrowedShape::Text),
            semio_framework_dsl_record::BorrowedFieldSpec { optional: true, ..semio_framework_dsl_record::BorrowedFieldSpec::new(1, "value", V::SHAPE) },
        ],
    };
}
macro_rules! borrowed_field_owner {
    ($owner:ident) => {
        impl<V: semio_framework_dsl_record::BorrowedDslField> semio_framework_dsl_record::BorrowedDslField for $owner<V> {
            const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);
        }
    };
}
borrowed_field_owner!(MapDelta);
borrowed_field_owner!(MapEntryDelta);
borrowed_field_owner!(MapEntryOperation);

fn map_producer<V: DslField>() -> RecordSpecProducer { RecordSpecProducer { ordinary: map_spec::<V>, decoding: map_spec_decode::<V>, encoding: map_spec_encode::<V> } }
fn entry_producer<V: DslField>() -> RecordSpecProducer { RecordSpecProducer { ordinary: entry_spec::<V>, decoding: entry_spec_decode::<V>, encoding: entry_spec_encode::<V> } }
fn operation_producer<V: DslField>() -> RecordSpecProducer { RecordSpecProducer { ordinary: operation_spec::<V>, decoding: operation_spec_decode::<V>, encoding: operation_spec_encode::<V> } }
fn map_spec<V: DslField>() -> RecordSpec { RecordSpec::new(None, RecordLayout::Inline, vec![field(0, "entries", Shape::Map(Box::new(Shape::Record(entry_producer::<V>()))), false)]) }
fn entry_spec<V: DslField>() -> RecordSpec { RecordSpec::new(None, RecordLayout::Inline, vec![field(0, "operation", Shape::Record(operation_producer::<V>()), false), field(1, "precondition", Shape::Text, false)]) }
fn operation_spec<V: DslField>() -> RecordSpec { RecordSpec::new(None, RecordLayout::Inline, vec![field(0, "kind", Shape::Text, false), field(1, "value", V::shape(), true)]) }
fn spec<V: DslField, C: NativeSchemaControl>(kind: u8, control: &mut C) -> Result<RecordSpec, ValueError> {
    let mut fields = control.allocate_vec(if kind == 0 { 1 } else { 2 })?;
    match kind {
        0 => { let child = producer::boxed(Shape::Record(entry_producer::<V>()), control)?; fields.push(producer::field(0, "entries", Shape::Map(child), control)?); }
        1 => { fields.push(producer::field(0, "operation", Shape::Record(operation_producer::<V>()), control)?); fields.push(producer::field(1, "precondition", Shape::Text, control)?); }
        _ => { fields.push(producer::field(0, "kind", Shape::Text, control)?); let shape = V::shape_controlled(control)?; let mut value = producer::field(1, "value", shape, control)?; value.optional = true; fields.push(value); }
    }
    producer::record(None, RecordLayout::Inline, fields, control)
}
fn map_spec_decode<V: DslField>(c: &mut NativeDecodeControl<'_>) -> Result<RecordSpec, ValueError> { spec::<V, _>(0, c) }
fn map_spec_encode<V: DslField>(c: &mut NativeEncodeControl<'_>) -> Result<RecordSpec, ValueError> { spec::<V, _>(0, c) }
fn entry_spec_decode<V: DslField>(c: &mut NativeDecodeControl<'_>) -> Result<RecordSpec, ValueError> { spec::<V, _>(1, c) }
fn entry_spec_encode<V: DslField>(c: &mut NativeEncodeControl<'_>) -> Result<RecordSpec, ValueError> { spec::<V, _>(1, c) }
fn operation_spec_decode<V: DslField>(c: &mut NativeDecodeControl<'_>) -> Result<RecordSpec, ValueError> { spec::<V, _>(2, c) }
fn operation_spec_encode<V: DslField>(c: &mut NativeEncodeControl<'_>) -> Result<RecordSpec, ValueError> { spec::<V, _>(2, c) }

fn map_view<'a, V: DslField>(value: &'a MapDelta<V>, path: &[usize]) -> Result<View<'a>, ValueError> { match path.split_first() { None => Ok(View::Record(&[0])), Some((0, rest)) => DslField::projection_view(&value.entries, rest), _ => Err(native_encoding::projection_path_error()) } }
fn map_key<'a, V: DslField>(value: &'a MapDelta<V>, path: &[usize], index: usize) -> Result<&'a str, ValueError> { match path.split_first() { Some((0, rest)) => DslField::projection_key(&value.entries, rest, index), _ => Err(native_encoding::projection_path_error()) } }
fn entry_view<'a, V: DslField>(value: &'a MapEntryDelta<V>, path: &[usize]) -> Result<View<'a>, ValueError> { match path.split_first() { None => Ok(View::Record(&[0, 1])), Some((0, rest)) => operation_view(&value.operation, rest), Some((1, [])) => Ok(View::Text(value.precondition.name())), _ => Err(native_encoding::projection_path_error()) } }
fn entry_key<'a, V: DslField>(value: &'a MapEntryDelta<V>, path: &[usize], index: usize) -> Result<&'a str, ValueError> { match path.split_first() { Some((0, rest)) => operation_key(&value.operation, rest, index), _ => Err(native_encoding::projection_path_error()) } }
fn operation_view<'a, V: DslField>(value: &'a MapEntryOperation<V>, path: &[usize]) -> Result<View<'a>, ValueError> {
    match (path.split_first(), value) { (None, MapEntryOperation::Set(_)) => Ok(View::Record(&[0, 1])), (None, _) => Ok(View::Record(&[0])), (Some((0, [])), _) => Ok(View::Text(kind(value))), (Some((1, rest)), MapEntryOperation::Set(value)) => value.projection_view(rest), _ => Err(native_encoding::projection_path_error()) }
}
fn operation_key<'a, V: DslField>(value: &'a MapEntryOperation<V>, path: &[usize], index: usize) -> Result<&'a str, ValueError> { match (path.split_first(), value) { (Some((1, rest)), MapEntryOperation::Set(value)) => value.projection_key(rest, index), _ => Err(native_encoding::projection_path_error()) } }
fn kind<V>(value: &MapEntryOperation<V>) -> &'static str { match value { MapEntryOperation::Set(_) => "set", MapEntryOperation::Remove => "remove", MapEntryOperation::Reject => "reject" } }

fn map_to<V: DslField>(value: &MapDelta<V>) -> RecordValue { cold_record([(0, DslField::to_value(&value.entries))]) }
fn entry_to<V: DslField>(value: &MapEntryDelta<V>) -> RecordValue { cold_record([(0, FieldValue::Record(operation_to(&value.operation))), (1, FieldValue::Text(value.precondition.name().into()))]) }
fn operation_to<V: DslField>(value: &MapEntryOperation<V>) -> RecordValue { let mut record = cold_record([(0, FieldValue::Text(kind(value).into()))]); if let MapEntryOperation::Set(value) = value { record.fields.insert(1, DslField::to_value(value)); } record }
fn map_from<V: DslField>(record: &RecordValue) -> Result<MapDelta<V>, String> {
    fields(record, &[0]).map_err(|e| e.to_string())?;
    let Some(FieldValue::Map(entries)) = record.get(0) else { return Err(invalid().to_string()); };
    let mut result = MapDelta::default();
    for (key, value) in entries { if result.entries.contains_key(key) { return Err("duplicate map delta key".into()); } result.entries.insert(key.clone(), <MapEntryDelta<V> as DslField>::from_value(value)?); }
    Ok(result)
}
fn entry_from<V: DslField>(record: &RecordValue) -> Result<MapEntryDelta<V>, String> { fields(record, &[0, 1]).map_err(|e| e.to_string())?; let precondition = controlled::presence(Some(text(record, 1).map_err(|e| e.to_string())?)).map_err(|e| e.to_string())?; let operation = <MapEntryOperation<V> as DslField>::from_value(record.get(0).unwrap())?; controlled::compatible(precondition, &operation).map_err(|e| e.to_string())?; Ok(MapEntryDelta { precondition, operation }) }
fn operation_from<V: DslField>(record: &RecordValue) -> Result<MapEntryOperation<V>, String> {
    let kind = text(record, 0).map_err(|e| e.to_string())?;
    fields(record, if kind == "set" { &[0, 1] } else { &[0] }).map_err(|e| e.to_string())?;
    match kind { "set" => Ok(MapEntryOperation::Set(V::from_value(record.get(1).unwrap())?)), "remove" => Ok(MapEntryOperation::Remove), "reject" => Ok(MapEntryOperation::Reject), _ => Err("unknown map operation".into()) }
}

fn map_encode<V: DslField>(value: &MapDelta<V>, c: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> { let mut output = EncodedRecord::new(1, c)?; output.insert(0, native_encoding::project_map(&value.entries, c)?)?; Ok(output.take()) }
fn entry_encode<V: DslField>(value: &MapEntryDelta<V>, c: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> { let mut output = EncodedRecord::new(2, c)?; output.insert(0, FieldValue::Record(operation_encode(&value.operation, c)?))?; output.insert(1, FieldValue::Text(c.copy_text(value.precondition.name())?))?; Ok(output.take()) }
fn operation_encode<V: DslField>(value: &MapEntryOperation<V>, c: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> { let mut output = EncodedRecord::new(if matches!(value, MapEntryOperation::Set(_)) { 2 } else { 1 }, c)?; output.insert(0, FieldValue::Text(c.copy_text(kind(value))?))?; if let MapEntryOperation::Set(value) = value { output.insert(1, c.scoped_stage(|c| V::to_value_controlled(value, c))?)?; } Ok(output.take()) }
fn map_decode<V: DslField>(record: &RecordValue, c: &mut NativeDecodeControl<'_>) -> Result<MapDelta<V>, ValueError> {
    fields(record, &[0])?;
    let Some(FieldValue::Map(entries)) = record.get(0) else { return Err(invalid()); };
    c.scoped_depth(64, |c| c.scoped_stage(|c| {
        c.begin_stage(entries.len())?;
        let mut output = DecodedValue::new(MapDelta::<V>::default(), map_retire::<V>);
        for (key, value) in entries { if output.get().entries.contains_key(key) { return Err(controlled::invalid("duplicate map delta key")); } controlled::charge_entry::<V>(c)?; let key = c.copy_text(key)?; let entry = <MapEntryDelta<V> as DslField>::from_value_controlled(value, c)?; output.get_mut().entries.insert(key, entry); c.step()?; }
        Ok(output.take())
    }))
}
fn entry_decode<V: DslField>(record: &RecordValue, c: &mut NativeDecodeControl<'_>) -> Result<MapEntryDelta<V>, ValueError> {
    c.checkpoint()?; fields(record, &[0, 1])?;
    let precondition = controlled::presence(Some(text(record, 1)?))?;
    let operation = <MapEntryOperation<V> as DslField>::from_value_controlled(record.get(0).unwrap(), c)?;
    let operation = DecodedValue::new(operation, operation_retire::<V>);
    controlled::compatible(precondition, operation.get())?;
    Ok(MapEntryDelta { precondition, operation: operation.take() })
}
fn operation_decode<V: DslField>(record: &RecordValue, c: &mut NativeDecodeControl<'_>) -> Result<MapEntryOperation<V>, ValueError> {
    c.checkpoint()?;
    let kind = text(record, 0)?; fields(record, if kind == "set" { &[0, 1] } else { &[0] })?;
    match kind { "set" => Ok(MapEntryOperation::Set(c.scoped_stage(|c| V::from_value_controlled(record.get(1).unwrap(), c))?)), "remove" => Ok(MapEntryOperation::Remove), "reject" => Ok(MapEntryOperation::Reject), _ => Err(controlled::invalid("unknown map operation")) }
}
fn operation_retire<V: DslField>(value: MapEntryOperation<V>) { if let MapEntryOperation::Set(value) = value { V::retire_decoded(value); } }
fn entry_retire<V: DslField>(value: MapEntryDelta<V>) { operation_retire(value.operation); }
fn map_retire<V: DslField>(value: MapDelta<V>) { for (_, entry) in value.entries { entry_retire(entry); } }

macro_rules! field_owner {
    ($owner:ident,$producer:ident,$view:ident,$key:ident,$to:ident,$from:ident,$encode:ident,$decode:ident,$retire:ident) => {
        impl<V: DslField> DslField for $owner<V> {
            fn projection_view(&self, path: &[usize]) -> Result<View<'_>, ValueError> { $view(self, path) }
            fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ValueError> { $key(self, path, index) }
            fn shape() -> Shape { Shape::Record($producer::<V>()) }
            fn shape_controlled<C: NativeSchemaControl>(c: &mut C) -> Result<Shape, ValueError> { c.checkpoint()?; Ok(Self::shape()) }
            fn to_value(&self) -> FieldValue { FieldValue::Record($to(self)) }
            fn from_value(value: &FieldValue) -> Result<Self, String> { match value { FieldValue::Record(record) => $from(record), _ => Err(invalid().to_string()) } }
            fn to_value_controlled(&self, c: &mut NativeEncodeControl<'_>) -> Result<FieldValue, ValueError> { $encode(self, c).map(FieldValue::Record) }
            fn to_record_controlled(&self, c: &mut NativeEncodeControl<'_>) -> Result<RecordValue, ValueError> { $encode(self, c) }
            fn from_value_controlled(value: &FieldValue, c: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { match value { FieldValue::Record(record) => $decode(record, c), _ => Err(invalid()) } }
            fn from_record_controlled(record: &RecordValue, c: &mut NativeDecodeControl<'_>) -> Result<Self, ValueError> { $decode(record, c) }
            fn retire_decoded(self) { $retire(self); }
        }
    };
}
field_owner!(MapDelta,map_producer,map_view,map_key,map_to,map_from,map_encode,map_decode,map_retire);
field_owner!(MapEntryDelta,entry_producer,entry_view,entry_key,entry_to,entry_from,entry_encode,entry_decode,entry_retire);
field_owner!(MapEntryOperation,operation_producer,operation_view,operation_key,operation_to,operation_from,operation_encode,operation_decode,operation_retire);
