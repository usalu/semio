//! 🫳️ Borrowed record specs of the workflow graph: the borrowed carrier macro and its three invocations, the label tables and the controlled native field helpers. See [`super`].

use super::*;

//#region 🔖️MediaContractDsl
const fn workflow_borrowed_optional(mut field:BorrowedFieldSpec)->BorrowedFieldSpec{field.optional=true;field}

/// 🫳️ Borrows the flattened handwritten workflow carrier metadata.
macro_rules! workflow_borrowed_record {
 ($owner:ty; $($field:expr),* $(,)?)=>{
  impl semio_framework_dsl_record::BorrowedDslRecord for $owner{
   const RECORD:semio_framework_dsl_record::BorrowedRecordSpec={
    semio_framework_dsl_record::BorrowedRecordSpec{keyword:None,layout:semio_framework_dsl_record::RecordLayout::Inline,fields:&[$($field),*]}
   };
  }
  impl semio_framework_dsl_record::BorrowedDslField for $owner{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Record(semio_framework_dsl_record::borrowed_record::<Self>);}
 };
}

workflow_borrowed_record!(MediaContract;
 BorrowedFieldSpec::new(0,"kind_id",BorrowedShape::Text),BorrowedFieldSpec::new(1,"class",BorrowedShape::Enum(MEDIA_CLASS_LABELS)),BorrowedFieldSpec::new(2,"form",BorrowedShape::Enum(MEDIA_FORM_LABELS)),BorrowedFieldSpec::new(3,"wire_kind",BorrowedShape::Text),workflow_borrowed_optional(BorrowedFieldSpec::new(4,"wire_format",BorrowedShape::Text)),workflow_borrowed_optional(BorrowedFieldSpec::new(5,"wire_schema",BorrowedShape::Text)),workflow_borrowed_optional(BorrowedFieldSpec::new(6,"conversion_from",BorrowedShape::Enum(MEDIA_FORM_LABELS))),workflow_borrowed_optional(BorrowedFieldSpec::new(7,"conversion_to",BorrowedShape::Enum(MEDIA_FORM_LABELS))));

workflow_borrowed_record!(WorkflowMediaPort;
 BorrowedFieldSpec::new(0,"id",BorrowedShape::Text),BorrowedFieldSpec::new(1,"port_id",BorrowedShape::Text),BorrowedFieldSpec::new(2,"label",BorrowedShape::Text),BorrowedFieldSpec::new(3,"direction",BorrowedShape::Enum(MEDIA_DIRECTION_LABELS)),BorrowedFieldSpec::new(4,"class",BorrowedShape::Enum(MEDIA_CLASS_LABELS)),BorrowedFieldSpec::new(5,"form",BorrowedShape::Enum(MEDIA_FORM_LABELS)),workflow_borrowed_optional(BorrowedFieldSpec::new(6,"kind_id",BorrowedShape::Text)),BorrowedFieldSpec::new(7,"required",BorrowedShape::Bool),BorrowedFieldSpec::new(8,"multiplicity",BorrowedShape::Enum(MEDIA_MULTIPLICITY_LABELS)));

workflow_borrowed_record!(WorkflowInput;
 BorrowedFieldSpec::new(0,"id",BorrowedShape::Text),BorrowedFieldSpec::new(1,"kind_id",BorrowedShape::Text),BorrowedFieldSpec::new(2,"selector",BorrowedShape::Text),BorrowedFieldSpec::new(3,"required",BorrowedShape::Bool),BorrowedFieldSpec::new(4,"multiplicity",BorrowedShape::Enum(MEDIA_MULTIPLICITY_LABELS)));

pub(super) fn workflow_enum_shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(labels:&[(&str,u32)],control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(labels.len())?;let mut values=control.allocate_vec::<(String,u32)>(labels.len())?;for(label,ordinal)in labels{values.push((control.copy_text(label)?,*ordinal));control.step()?;}Ok(semio_framework_dsl_record::Shape::Enum(values))})
}

pub(super) const MEDIA_CLASS_LABELS:&[(&str,u32)]=&[("twoD",0),("threeD",1),("text",2),("data",3),("graph",4),("kit",5),("computation",6),("presentation",7)];

pub(super) const MEDIA_FORM_LABELS:&[(&str,u32)]=&[("any",0),("vector",1),("raster",2),("brep",3),("mesh",4),("document",5),("value",6),("dag",7),("trinity",8),("type",9),("design",10),("kit",11),("flow",12),("sequence",13),("procedure",14),("deck",15)];

pub(super) const MEDIA_DIRECTION_LABELS:&[(&str,u32)]=&[("in",0),("out",1)];

pub(super) const MEDIA_MULTIPLICITY_LABELS:&[(&str,u32)]=&[("one",0),("many",1)];

/// 🚫️ Declared-schema refusal of the controlled Workflow lane, typed at its actual failure site.
pub fn workflow_invalid(message:impl Into<String>)->semio_framework_value::ValueError{semio_framework_value::ValueError::new(::semio_framework_value::ValueRefusalKind::InvalidValue,message)}

fn workflow_native_field<T:semio_framework_dsl_record::DslField>(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T,semio_framework_value::ValueError>{
 let value=record.get(id).unwrap_or(&semio_framework_dsl_record::FieldValue::Absent);let value=control.scoped_stage(|control|{control.begin_stage(0)?;T::from_value_controlled(value,control)})?;control.step()?;Ok(value)
}

fn workflow_native_optional_field<T:semio_framework_dsl_record::DslField>(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<T>,semio_framework_value::ValueError>{
 match record.get(id){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>{control.step()?;Ok(None)},Some(_)=>workflow_native_field(record,id,control).map(Some)}
}

fn workflow_native_enum<T>(record:&semio_framework_dsl_record::RecordValue,id:u16,bind:fn(u32)->Result<T,String>,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T,semio_framework_value::ValueError>{
 control.step()?;match record.get(id){Some(semio_framework_dsl_record::FieldValue::Enum(ordinal))=>bind(*ordinal).map_err(workflow_invalid),_=>Err(workflow_invalid("expected declared workflow enum"))}
}

fn workflow_native_optional_enum(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Option<MediaForm>,semio_framework_value::ValueError>{
 control.step()?;match record.get(id){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(None),Some(semio_framework_dsl_record::FieldValue::Enum(ordinal))=>media_form_from_ordinal(*ordinal).map(Some).map_err(workflow_invalid),_=>Err(workflow_invalid("expected optional workflow media form"))}
}

fn workflow_native_project<T:semio_framework_dsl_record::DslField>(record:&mut semio_framework_dsl_record::native_encoding::EncodedRecord,id:u16,value:&T,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),semio_framework_value::ValueError>{
 let value=control.scoped_stage(|control|{control.begin_stage(0)?;value.to_value_controlled(control)})?;record.insert(id,value)?;control.step()
}

fn workflow_native_project_optional<T:semio_framework_dsl_record::DslField>(record:&mut semio_framework_dsl_record::native_encoding::EncodedRecord,id:u16,value:&Option<T>,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),semio_framework_value::ValueError>{
 match value{Some(value)=>workflow_native_project(record,id,value,control),None=>{record.insert(id,semio_framework_dsl_record::FieldValue::Absent)?;control.step()}}
}

pub(super) fn media_contract_to_record_controlled(value:&MediaContract,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(8)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(8,control)?;
 workflow_native_project(&mut record,0,&value.kind_id,control)?;record.insert(1,semio_framework_dsl_record::FieldValue::Enum(media_class_ordinal(value.media_type.class)))?;control.step()?;record.insert(2,semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(value.media_type.form)))?;control.step()?;
 match &value.wire{
 MediaWireFormat::Binary{format_kind}=>{record.insert(3,semio_framework_dsl_record::FieldValue::Text(control.copy_text("binary")?))?;control.step()?;workflow_native_project(&mut record,4,format_kind,control)?;record.insert(5,semio_framework_dsl_record::FieldValue::Absent)?;control.step()?;},
 MediaWireFormat::Document{schema}|MediaWireFormat::Intrinsic{schema}=>{let kind=if matches!(&value.wire,MediaWireFormat::Intrinsic{..}){"intrinsic"}else{"document"};record.insert(3,semio_framework_dsl_record::FieldValue::Text(control.copy_text(kind)?))?;control.step()?;record.insert(4,semio_framework_dsl_record::FieldValue::Absent)?;control.step()?;workflow_native_project(&mut record,5,schema,control)?;}
 }
 match value.conversion{Some((from,to))=>{record.insert(6,semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(from)))?;record.insert(7,semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(to)))?;},None=>{record.insert(6,semio_framework_dsl_record::FieldValue::Absent)?;record.insert(7,semio_framework_dsl_record::FieldValue::Absent)?;}}control.step()?;control.step()?;Ok(record.take())})
}

pub(super) fn media_contract_from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<MediaContract,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(8)?;let kind_id=workflow_native_field(record,0,control)?;let class=workflow_native_enum(record,1,media_class_from_ordinal,control)?;let form=workflow_native_enum(record,2,media_form_from_ordinal,control)?;
 control.step()?;let kind=match record.get(3){Some(semio_framework_dsl_record::FieldValue::Text(value))=>value.as_str(),_=>return Err(workflow_invalid("expected workflow wire kind"))};let format_kind:Option<String>=workflow_native_optional_field(record,4,control)?;let schema:Option<String>=workflow_native_optional_field(record,5,control)?;
 let wire=match(kind,format_kind,schema){("binary",Some(format_kind),None)=>MediaWireFormat::Binary{format_kind},("document",None,Some(schema))=>MediaWireFormat::Document{schema},("intrinsic",None,Some(schema))=>MediaWireFormat::Intrinsic{schema},_=>return Err(workflow_invalid("invalid workflow wire fields"))};
 let from=workflow_native_optional_enum(record,6,control)?;let to=workflow_native_optional_enum(record,7,control)?;let conversion=match(from,to){(Some(from),Some(to))=>Some((from,to)),(None,None)=>None,_=>return Err(workflow_invalid("workflow conversion needs both forms"))};Ok(MediaContract{kind_id,media_type:MediaType{class,form},wire,conversion})})
}

pub(super) fn workflow_media_port_to_record_controlled(value:&WorkflowMediaPort,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(9)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(9,control)?;workflow_native_project(&mut record,0,&value.id,control)?;workflow_native_project(&mut record,1,&value.spec.id,control)?;workflow_native_project(&mut record,2,&value.spec.label,control)?;record.insert(3,semio_framework_dsl_record::FieldValue::Enum(media_port_direction_ordinal(value.spec.direction)))?;control.step()?;record.insert(4,semio_framework_dsl_record::FieldValue::Enum(media_class_ordinal(value.spec.media_type.class)))?;control.step()?;record.insert(5,semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(value.spec.media_type.form)))?;control.step()?;workflow_native_project_optional(&mut record,6,&value.spec.kind_id,control)?;workflow_native_project(&mut record,7,&value.spec.required,control)?;record.insert(8,semio_framework_dsl_record::FieldValue::Enum(port_multiplicity_ordinal(value.spec.multiplicity)))?;control.step()?;Ok(record.take())})
}

pub(super) fn workflow_media_port_from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<WorkflowMediaPort,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(9)?;let id=workflow_native_field(record,0,control)?;let port_id=workflow_native_field(record,1,control)?;let label=workflow_native_field(record,2,control)?;let direction=workflow_native_enum(record,3,media_port_direction_from_ordinal,control)?;let class=workflow_native_enum(record,4,media_class_from_ordinal,control)?;let form=workflow_native_enum(record,5,media_form_from_ordinal,control)?;let kind_id=workflow_native_optional_field(record,6,control)?;let required=workflow_native_field(record,7,control)?;let multiplicity=workflow_native_enum(record,8,port_multiplicity_from_ordinal,control)?;Ok(WorkflowMediaPort{id,spec:MediaPortSpec{id:port_id,label,direction,media_type:MediaType{class,form},kind_id,required,multiplicity}})})
}

pub(super) fn workflow_input_to_record_controlled(value:&WorkflowInput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(5)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(5,control)?;workflow_native_project(&mut record,0,&value.id,control)?;workflow_native_project(&mut record,1,&value.kind_id,control)?;workflow_native_project(&mut record,2,&value.selector,control)?;workflow_native_project(&mut record,3,&value.required,control)?;record.insert(4,semio_framework_dsl_record::FieldValue::Enum(port_multiplicity_ordinal(value.multiplicity)))?;control.step()?;Ok(record.take())})
}

pub(super) fn workflow_input_from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<WorkflowInput,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(5)?;let id=workflow_native_field(record,0,control)?;let kind_id=workflow_native_field(record,1,control)?;let selector=workflow_native_field(record,2,control)?;let required=workflow_native_field(record,3,control)?;let multiplicity=workflow_native_enum(record,4,port_multiplicity_from_ordinal,control)?;Ok(WorkflowInput{id,kind_id,selector,required,multiplicity})})
}
//#endregion 🔖️MediaContractDsl
