//! 📝️ Complete six-root owned DXF snapshot persistence.
use super::*;
use pack::value::{DslValue,FromValue,ToValue};
pub const COMPONENT_GRAMMAR_SEMIO:&str=include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH:&str="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio";
const FIELDS:[(u16,&str,dsl::Shape,bool);6]=[
    (1,"schema",dsl::Shape::Text,false),
    (2,"headerVars",dsl::Shape::Value,false),
    (3,"tables",dsl::Shape::Value,false),
    (4,"otherTables",dsl::Shape::Value,false),
    (5,"blocks",dsl::Shape::Value,false),
    (6,"entities",dsl::Shape::Value,false),
];
pub(super) fn spec()->dsl::RecordSpec{
    dsl::RecordSpec::new(None,dsl::RecordLayout::Lines,FIELDS.into_iter().map(|(id,key,shape,optional)|{let mut field=dsl::FieldSpec::new(id,key,shape);field.optional=optional;field}).collect())
}
fn spec_controlled<C:dsl::NativeSchemaControl>(control:&mut C)->Result<dsl::RecordSpec,String>{
    control.scoped_stage(|control|{
        control.begin_stage(FIELDS.len())?;let mut fields=control.allocate_vec::<dsl::FieldSpec>(FIELDS.len())?;
        for(id,key,shape,optional)in FIELDS{control.checkpoint()?;let mut field=dsl::schema::producer::field(id,key,shape,control)?;field.optional=optional;fields.push(field);control.step()?;}
        dsl::schema::producer::record(None,dsl::RecordLayout::Lines,fields,control)
    })
}
/// 🏭️ Owns literal snapshot metadata independently under either native allocation controller.
pub(super) fn spec_producer()->dsl::RecordSpecProducer{dsl::RecordSpecProducer{ordinary:spec,decoding:|control|spec_controlled(control),encoding:|control|spec_controlled(control)}}

pub(super) fn to_record(snapshot:&DxfSnapshot)->dsl::RecordValue{
    use dsl::FieldValue as V;
    dsl::RecordValue{fields:[(1,V::Text(snapshot.schema.clone())),(2,V::Value(snapshot.header_vars.to_value())),(3,V::Value(snapshot.tables.to_value())),(4,V::Value(snapshot.other_tables.to_value())),(5,V::Value(snapshot.blocks.to_value())),(6,V::Value(snapshot.entities.to_value()))].into_iter().collect()}
}

pub(super) fn to_record_controlled(snapshot:&DxfSnapshot,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::RecordValue,store::TextError>{
    control.scoped_depth(64,|control|control.scoped_stage(|control|->Result<_,String>{
        control.begin_stage(6)?;let mut record=dsl::native_encoding::EncodedRecord::new(6,control)?;
        record.insert(1,dsl::FieldValue::Text(control.copy_text(&snapshot.schema)?));control.step()?;
        record.insert(2,project(&snapshot.header_vars,control)?);control.step()?;
        record.insert(3,project(&snapshot.tables,control)?);control.step()?;
        record.insert(4,project(&snapshot.other_tables,control)?);control.step()?;
        record.insert(5,project(&snapshot.blocks,control)?);control.step()?;
        record.insert(6,project(&snapshot.entities,control)?);control.step()?;
        Ok(record.take())
    })).map_err(|message|store::TextError::new(message,dsl::TextSpan::at(1,1)))
}
fn project<T:ToValue>(value:&T,control:&mut dsl::NativeEncodeControl<'_>)->Result<dsl::FieldValue,String>{control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control).map(dsl::FieldValue::Value).map_err(|error|error.to_string())})}
pub(super) fn from_record(record:&dsl::RecordValue)->Result<DxfSnapshot,store::TextError>{
    if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(store::TextError::new("DXF snapshot contains an undeclared root field",dsl::TextSpan::at(1,1)));}
    let mut fields=Vec::with_capacity(6);
    for(id,key)in[(1,"schema"),(2,"headerVars"),(3,"tables"),(4,"otherTables"),(5,"blocks"),(6,"entities")]{
        let value=match record.get(id){Some(dsl::FieldValue::Text(value))if id==1=>DslValue::String(value.clone()),Some(dsl::FieldValue::Value(value))if id>1=>value.clone(),_=>return Err(store::TextError::new(format!("DXF snapshot field {key} is missing or has a different shape"),dsl::TextSpan::at(1,1)))};
        fields.push((key.into(),value));
    }
    DxfSnapshot::from_value(DslValue::Object(fields)).map_err(|error|store::TextError::new(error.to_string(),dsl::TextSpan::at(1,1)))
}

fn owned<T:FromValue>(record:&dsl::RecordValue,id:u16,key:&str,control:&mut dsl::NativeDecodeControl<'_>)->Result<pack::value::DecodedValue<T>,store::TextError>{
    let Some(dsl::FieldValue::Value(value))=record.get(id)else{return Err(store::TextError::new(format!("DXF snapshot field {key} is missing or has a different shape"),dsl::TextSpan::at(1,1)))};
    let value=T::from_value_controlled(value,control).map_err(|error|store::TextError::new(error.under(key).to_string(),dsl::TextSpan::at(1,1)))?;let owner=pack::value::DecodedValue::new(value,T::retire_decoded);control.step().map_err(|error|store::TextError::new(error,dsl::TextSpan::at(1,1)))?;Ok(owner)
}
pub(super) fn from_record_controlled(record:&dsl::RecordValue,control:&mut dsl::NativeDecodeControl<'_>)->Result<DxfSnapshot,store::TextError>{
    let error=|message:String|store::TextError::new(message,dsl::TextSpan::at(1,1));
    control.scoped_stage(|control|{
        control.begin_stage(6).map_err(error)?;if record.fields.keys().any(|id|!(1..=6).contains(id)){return Err(error("DXF snapshot contains an undeclared root field".into()));}
        let Some(dsl::FieldValue::Text(schema))=record.get(1)else{return Err(error("DXF snapshot schema is missing or has a different shape".into()));};let schema=control.copy_text(schema).map_err(error)?;control.step().map_err(error)?;
        let header_vars=owned::<Vec<DxfHeaderVar>>(record,2,"headerVars",control)?;let tables=owned::<DxfTables>(record,3,"tables",control)?;let other_tables=owned::<Vec<DxfOtherTable>>(record,4,"otherTables",control)?;let blocks=owned::<Vec<DxfBlock>>(record,5,"blocks",control)?;let entities=owned::<Vec<DxfEntity>>(record,6,"entities",control)?;
        Ok(DxfSnapshot{schema,header_vars:header_vars.take(),tables:tables.take(),other_tables:other_tables.take(),blocks:blocks.take(),entities:entities.take()})
    })
}

#[cfg(test)]
#[path="🧪️tests/🏭️producer/🦀️.rs"]
mod producer_tests;
