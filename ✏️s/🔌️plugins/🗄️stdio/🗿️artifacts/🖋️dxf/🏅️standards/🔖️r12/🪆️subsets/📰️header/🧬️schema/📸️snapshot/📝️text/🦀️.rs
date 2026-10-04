//! 📝️ Complete six-root owned DXF snapshot persistence.
use super::*;
use semio_framework_value::{ValueError, ValueRefusalKind};
use pack::value::{DslValue,FromValue,ToValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio";
const FIELDS:[(u16,&str,semio_framework_dsl_record::Shape,bool);6]=[
    (1,"schema",semio_framework_dsl_record::Shape::Text,false),
    (2,"headerVars",semio_framework_dsl_record::Shape::Value,false),
    (3,"tables",semio_framework_dsl_record::Shape::Value,false),
    (4,"otherTables",semio_framework_dsl_record::Shape::Value,false),
    (5,"blocks",semio_framework_dsl_record::Shape::Value,false),
    (6,"entities",semio_framework_dsl_record::Shape::Value,false),
];
pub(super) fn spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=semio_framework_dsl_record::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,ValueError>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=semio_framework_dsl_record::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub(super) fn spec_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}

pub(super) fn to_record(snapshot:&DxfSnapshot)->semio_framework_dsl_record::RecordValue{
    use semio_framework_dsl_record::FieldValue as V;
    semio_framework_dsl_record::RecordValue{fields:[(1,semio_framework_dsl_record::FieldValue::Text(snapshot.schema.clone())),(2,semio_framework_dsl_record::FieldValue::Value(snapshot.header_vars.to_value())),(3,semio_framework_dsl_record::FieldValue::Value(snapshot.tables.to_value())),(4,semio_framework_dsl_record::FieldValue::Value(snapshot.other_tables.to_value())),(5,semio_framework_dsl_record::FieldValue::Value(snapshot.blocks.to_value())),(6,semio_framework_dsl_record::FieldValue::Value(snapshot.entities.to_value()))].into_iter().collect()}
}

pub(super) fn to_record_controlled(snapshot:&DxfSnapshot,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<_,ValueError>{
        control.begin_stage(6)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(6,control)?;
        record.insert(1,semio_framework_dsl_record::FieldValue::Text(control.copy_text(&snapshot.schema)?))?;control.step()?;
        record.insert(2,project(&snapshot.header_vars,control)?)?;control.step()?;
        record.insert(3,project(&snapshot.tables,control)?)?;control.step()?;
        record.insert(4,project(&snapshot.other_tables,control)?)?;control.step()?;
        record.insert(5,project(&snapshot.blocks,control)?)?;control.step()?;
        record.insert(6,project(&snapshot.entities,control)?)?;control.step()?;
        Ok(record.take())
    }))
}
fn project<T:ToValue>(value:&T,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,ValueError>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(semio_framework_dsl_record::FieldValue::Value)})}
pub(super) fn from_record(record:&semio_framework_dsl_record::RecordValue)->Result<DxfSnapshot,semio_framework_diagnostic::TextError>{
    if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "DXF snapshot contains an undeclared root field",semio_framework_diagnostic::TextSpan::at(1,1)));}
    let mut fields=Vec::with_capacity(6);
    for(id,key)in[(1,"schema"),(2,"headerVars"),(3,"tables"),(4,"otherTables"),(5,"blocks"),(6,"entities")]{
        let value=match record.get(id){Some(semio_framework_dsl_record::FieldValue::Text(value))if id==1=>DslValue::String(value.clone()),Some(semio_framework_dsl_record::FieldValue::Value(value))if id>1=>value.clone(),_=>return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DXF snapshot field {key} is missing or has a different shape"),semio_framework_diagnostic::TextSpan::at(1,1)))};
        fields.push((key.into(),value));
    }
    DxfSnapshot::from_value(DslValue::Object(fields)).map_err(|error|semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1,1)))
}

fn owned<T:FromValue>(record:&semio_framework_dsl_record::RecordValue,id:u16,key:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,ValueError>{
    let Some(semio_framework_dsl_record::FieldValue::Value(value))=record.get(id)else{return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DXF snapshot field {key} is missing or has a different shape")))};
    let value=T::from_value_controlled(value,control).map_err(|error|error.under(key))?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step()?;Ok(owner)
}
pub(super) fn from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<DxfSnapshot,ValueError>{
    control.scoped_stage(|control|->Result<_,ValueError>{
        control.begin_stage(6)?;if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue, "DXF snapshot contains an undeclared root field"));}
        let Some(semio_framework_dsl_record::FieldValue::Text(schema))=record.get(1)else{return Err(semio_framework_value::ValueError::new(ValueRefusalKind::InvalidValue, "DXF snapshot schema is missing or has a different shape"));};let schema=control.copy_text(schema)?;control.step()?;
        let header_vars=owned::<Vec<DxfHeaderVar>>(record,2,"headerVars",control)?;let tables=owned::<DxfTables>(record,3,"tables",control)?;let other_tables=owned::<Vec<DxfOtherTable>>(record,4,"otherTables",control)?;let blocks=owned::<Vec<DxfBlock>>(record,5,"blocks",control)?;let entities=owned::<Vec<DxfEntity>>(record,6,"entities",control)?;
        Ok(DxfSnapshot{schema,header_vars:header_vars.take(),tables:tables.take(),other_tables:other_tables.take(),blocks:blocks.take(),entities:entities.take()})
    })
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;
