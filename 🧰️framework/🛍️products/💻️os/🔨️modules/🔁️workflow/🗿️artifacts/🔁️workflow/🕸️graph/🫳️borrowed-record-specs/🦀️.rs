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
 BorrowedFieldSpec::new(0,"id",BorrowedShape::Text),BorrowedFieldSpec::new(1,"port_id",BorrowedShape::Text),BorrowedFieldSpec::new(2,"label",WORKFLOW_LABEL_BORROWED_SHAPE),BorrowedFieldSpec::new(3,"direction",BorrowedShape::Enum(MEDIA_DIRECTION_LABELS)),BorrowedFieldSpec::new(4,"class",BorrowedShape::Enum(MEDIA_CLASS_LABELS)),BorrowedFieldSpec::new(5,"form",BorrowedShape::Enum(MEDIA_FORM_LABELS)),workflow_borrowed_optional(BorrowedFieldSpec::new(6,"kind_id",BorrowedShape::Text)),BorrowedFieldSpec::new(7,"required",BorrowedShape::Bool),BorrowedFieldSpec::new(8,"multiplicity",BorrowedShape::Enum(MEDIA_MULTIPLICITY_LABELS)));

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

fn workflow_native_field<T:semio_framework_dsl_record::DslField>(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<T,semio_framework_value::ValueError>{
 let value=record.get(id).unwrap_or(&semio_framework_dsl_record::FieldValue::Absent);let value=control.scoped_stage(|control|{control.begin_stage(0)?;T::from_value_controlled(value,control)})?;control.step()?;Ok(value)
}

fn workflow_native_optional_field<T:semio_framework_dsl_record::DslField>(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Option<T>,semio_framework_value::ValueError>{
 match record.get(id){None|Some(semio_framework_dsl_record::FieldValue::Absent)=>{control.step()?;Ok(None)},Some(_)=>workflow_native_field(record,id,control).map(Some)}
}

fn workflow_native_enum<T>(record:&semio_framework_dsl_record::RecordValue,id:u16,bind:fn(u32)->Result<T,String>,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<T,semio_framework_value::ValueError>{
 control.step()?;match record.get(id){Some(semio_framework_dsl_record::FieldValue::Enum(ordinal))=>bind(*ordinal).map_err(workflow_invalid),_=>Err(workflow_invalid("expected declared workflow enum"))}
}

fn workflow_native_optional_enum(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Option<MediaForm>,semio_framework_value::ValueError>{
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

pub(super) fn media_contract_from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<MediaContract,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(8)?;let kind_id=workflow_native_field(record,0,control)?;let class=workflow_native_enum(record,1,media_class_from_ordinal,control)?;let form=workflow_native_enum(record,2,media_form_from_ordinal,control)?;
 control.step()?;let kind=match record.get(3){Some(semio_framework_dsl_record::FieldValue::Text(value))=>value.as_str(),_=>return Err(workflow_invalid("expected workflow wire kind"))};let format_kind:Option<String>=workflow_native_optional_field(record,4,control)?;let schema:Option<String>=workflow_native_optional_field(record,5,control)?;
 let wire=match(kind,format_kind,schema){("binary",Some(format_kind),None)=>MediaWireFormat::Binary{format_kind},("document",None,Some(schema))=>MediaWireFormat::Document{schema},("intrinsic",None,Some(schema))=>MediaWireFormat::Intrinsic{schema},_=>return Err(workflow_invalid("invalid workflow wire fields"))};
 let from=workflow_native_optional_enum(record,6,control)?;let to=workflow_native_optional_enum(record,7,control)?;let conversion=match(from,to){(Some(from),Some(to))=>Some((from,to)),(None,None)=>None,_=>return Err(workflow_invalid("workflow conversion needs both forms"))};Ok(MediaContract{kind_id,media_type:MediaType{class,form},wire,conversion})})
}

pub(super) fn workflow_media_port_to_record_controlled(value:&WorkflowMediaPort,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(9)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(9,control)?;workflow_native_project(&mut record,0,&value.id,control)?;workflow_native_project(&mut record,1,&value.spec.id,control)?;workflow_native_project_label(&mut record,2,&value.spec.label,control)?;record.insert(3,semio_framework_dsl_record::FieldValue::Enum(media_port_direction_ordinal(value.spec.direction)))?;control.step()?;record.insert(4,semio_framework_dsl_record::FieldValue::Enum(media_class_ordinal(value.spec.media_type.class)))?;control.step()?;record.insert(5,semio_framework_dsl_record::FieldValue::Enum(media_form_ordinal(value.spec.media_type.form)))?;control.step()?;workflow_native_project_optional(&mut record,6,&value.spec.kind_id,control)?;workflow_native_project(&mut record,7,&value.spec.required,control)?;record.insert(8,semio_framework_dsl_record::FieldValue::Enum(port_multiplicity_ordinal(value.spec.multiplicity)))?;control.step()?;Ok(record.take())})
}

pub(super) fn workflow_media_port_from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<WorkflowMediaPort,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(9)?;let id=workflow_native_field(record,0,control)?;let port_id=workflow_native_field(record,1,control)?;let label=workflow_native_field_label(record,2,control)?;let direction=workflow_native_enum(record,3,media_port_direction_from_ordinal,control)?;let class=workflow_native_enum(record,4,media_class_from_ordinal,control)?;let form=workflow_native_enum(record,5,media_form_from_ordinal,control)?;let kind_id=workflow_native_optional_field(record,6,control)?;let required=workflow_native_field(record,7,control)?;let multiplicity=workflow_native_enum(record,8,port_multiplicity_from_ordinal,control)?;Ok(WorkflowMediaPort{id,spec:MediaPortSpec{id:port_id,label,direction,media_type:MediaType{class,form},kind_id,required,multiplicity}})})
}

pub(super) fn workflow_input_to_record_controlled(value:&WorkflowInput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(5)?;let mut record=semio_framework_dsl_record::native_encoding::EncodedRecord::new(5,control)?;workflow_native_project(&mut record,0,&value.id,control)?;workflow_native_project(&mut record,1,&value.kind_id,control)?;workflow_native_project(&mut record,2,&value.selector,control)?;workflow_native_project(&mut record,3,&value.required,control)?;record.insert(4,semio_framework_dsl_record::FieldValue::Enum(port_multiplicity_ordinal(value.multiplicity)))?;control.step()?;Ok(record.take())})
}

pub(super) fn workflow_input_from_record_controlled(record:&semio_framework_dsl_record::RecordValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<WorkflowInput,semio_framework_value::ValueError>{
 control.scoped_stage(|control|{control.begin_stage(5)?;let id=workflow_native_field(record,0,control)?;let kind_id=workflow_native_field(record,1,control)?;let selector=workflow_native_field(record,2,control)?;let required=workflow_native_field(record,3,control)?;let multiplicity=workflow_native_enum(record,4,port_multiplicity_from_ordinal,control)?;Ok(WorkflowInput{id,kind_id,selector,required,multiplicity})})
}

//#region 🏷️LocalizedLabelRecord
const _: () = assert!(semio_ui_locale::Locale::COUNT == 2 && semio_ui_locale::Terminology::COUNT == 2, "🏷️ the borrowed label carrier below spells the axes `native`/`reuse` x `en`/`de`; extend it with the axes");

struct WorkflowLabelRow;
workflow_borrowed_record!(WorkflowLabelRow;
 BorrowedFieldSpec::new(0,"en",BorrowedShape::Text),BorrowedFieldSpec::new(1,"de",BorrowedShape::Text));

struct WorkflowLabelMatrix;
workflow_borrowed_record!(WorkflowLabelMatrix;
 BorrowedFieldSpec::new(0,"native",<WorkflowLabelRow as semio_framework_dsl_record::BorrowedDslField>::SHAPE),BorrowedFieldSpec::new(1,"reuse",<WorkflowLabelRow as semio_framework_dsl_record::BorrowedDslField>::SHAPE));

/// 🏷️ The shape of a port label in the borrowed carrier.
pub(super) const WORKFLOW_LABEL_BORROWED_SHAPE:semio_framework_dsl_record::BorrowedShape=<WorkflowLabelMatrix as semio_framework_dsl_record::BorrowedDslField>::SHAPE;

fn workflow_label_row_spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,semio_ui_locale::Locale::ALL.iter().map(|locale|semio_framework_dsl_record::FieldSpec::new(locale.index() as u16,locale.as_str(),semio_framework_dsl_record::Shape::Text)).collect())
}

fn workflow_label_row_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Locale::COUNT)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(semio_ui_locale::Locale::COUNT)?;
        for locale in semio_ui_locale::Locale::ALL{let shape=semio_framework_dsl_record::Shape::Text;fields.push(semio_framework_dsl_record::producer::field(locale.index() as u16,locale.as_str(),shape,control)?);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn workflow_label_row_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:workflow_label_row_spec,decoding:|control|workflow_label_row_spec_controlled(control),encoding:|control|workflow_label_row_spec_controlled(control)}}

fn workflow_label_matrix_spec()->semio_framework_dsl_record::RecordSpec{
    semio_framework_dsl_record::RecordSpec::new(None,semio_framework_dsl_record::RecordLayout::Inline,semio_ui_locale::Terminology::ALL.iter().map(|terminology|semio_framework_dsl_record::FieldSpec::new(terminology.index() as u16,terminology.as_str(),semio_framework_dsl_record::Shape::Record(workflow_label_row_producer()))).collect())
}

fn workflow_label_matrix_spec_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::RecordSpec,semio_framework_value::ValueError>{
    control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Terminology::COUNT)?;let mut fields=control.allocate_vec::<semio_framework_dsl_record::FieldSpec>(semio_ui_locale::Terminology::COUNT)?;
        for terminology in semio_ui_locale::Terminology::ALL{let shape=semio_framework_dsl_record::Shape::Record(workflow_label_row_producer());fields.push(semio_framework_dsl_record::producer::field(terminology.index() as u16,terminology.as_str(),shape,control)?);control.step()?;}
        semio_framework_dsl_record::producer::record(None,semio_framework_dsl_record::RecordLayout::Inline,fields,control)
    })
}

fn workflow_label_matrix_producer()->semio_framework_dsl_record::RecordSpecProducer{semio_framework_dsl_record::RecordSpecProducer{ordinary:workflow_label_matrix_spec,decoding:|control|workflow_label_matrix_spec_controlled(control),encoding:|control|workflow_label_matrix_spec_controlled(control)}}

/// 🏷️ The shape of a port label in the record DSL.
pub(super) fn workflow_label_shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Record(workflow_label_matrix_producer())}

/// 🏷️ The shape of a port label in the controlled schema lane.
pub(super) fn workflow_label_shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Record(workflow_label_matrix_producer()))}

/// 🏷️ A port label as its record value.
pub(super) fn workflow_label_to_value(label:&semio_ui_locale::LocalizedLabel)->semio_framework_dsl_record::FieldValue{
    let mut matrix=semio_framework_dsl_record::RecordValue::default();
    for terminology in semio_ui_locale::Terminology::ALL{
        let mut row=semio_framework_dsl_record::RecordValue::default();
        for locale in semio_ui_locale::Locale::ALL{row.fields.insert(locale.index() as u16,semio_framework_dsl_record::FieldValue::Text(label.resolve(terminology,locale).to_string()));}
        matrix.fields.insert(terminology.index() as u16,semio_framework_dsl_record::FieldValue::Record(row));
    }
    semio_framework_dsl_record::FieldValue::Record(matrix)
}

/// 🏷️ A port label from its record value; refuses a missing terminology row or locale cell.
pub(super) fn workflow_label_from_value(value:&semio_framework_dsl_record::FieldValue)->Result<semio_ui_locale::LocalizedLabel,String>{
    let semio_framework_dsl_record::FieldValue::Record(matrix)=value else{return Err("expected label record".to_string())};
    let mut refusal=None;
    let label=semio_ui_locale::LocalizedLabel::from_fn(|terminology,locale|match matrix.get(terminology.index() as u16){
        Some(semio_framework_dsl_record::FieldValue::Record(row))=>match row.get(locale.index() as u16){Some(semio_framework_dsl_record::FieldValue::Text(text))=>text.clone(),_=>{refusal.get_or_insert(format!("expected label text for {}.{}",terminology.as_str(),locale.as_str()));String::new()}},
        _=>{refusal.get_or_insert(format!("expected label row for {}",terminology.as_str()));String::new()}
    });
    refusal.map_or(Ok(label),Err)
}

fn workflow_native_project_label(record:&mut semio_framework_dsl_record::native_encoding::EncodedRecord,id:u16,label:&semio_ui_locale::LocalizedLabel,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),semio_framework_value::ValueError>{
    let matrix=control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Terminology::COUNT)?;let mut matrix=semio_framework_dsl_record::native_encoding::EncodedRecord::new(semio_ui_locale::Terminology::COUNT,control)?;
        for terminology in semio_ui_locale::Terminology::ALL{
            let row=control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Locale::COUNT)?;let mut row=semio_framework_dsl_record::native_encoding::EncodedRecord::new(semio_ui_locale::Locale::COUNT,control)?;
                for locale in semio_ui_locale::Locale::ALL{row.insert(locale.index() as u16,semio_framework_dsl_record::FieldValue::Text(control.copy_text(label.resolve(terminology,locale))?))?;control.step()?;}
                Ok::<_,semio_framework_value::ValueError>(row.take())
            })?;
            matrix.insert(terminology.index() as u16,semio_framework_dsl_record::FieldValue::Record(row))?;control.step()?;
        }
        Ok::<_,semio_framework_value::ValueError>(matrix.take())
    })?;
    record.insert(id,semio_framework_dsl_record::FieldValue::Record(matrix))?;control.step()
}

fn workflow_native_field_label(record:&semio_framework_dsl_record::RecordValue,id:u16,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<semio_ui_locale::LocalizedLabel,semio_framework_value::ValueError>{
    let Some(semio_framework_dsl_record::FieldValue::Record(matrix))=record.get(id) else{return Err(workflow_invalid("expected declared label record"))};
    let mut cells:[[String;semio_ui_locale::Locale::COUNT];semio_ui_locale::Terminology::COUNT]=Default::default();
    control.scoped_stage(|control|{control.begin_stage(semio_ui_locale::Terminology::COUNT*semio_ui_locale::Locale::COUNT)?;
        for terminology in semio_ui_locale::Terminology::ALL{
            let Some(semio_framework_dsl_record::FieldValue::Record(row))=matrix.get(terminology.index() as u16) else{return Err(workflow_invalid("expected declared label row"))};
            for locale in semio_ui_locale::Locale::ALL{
                let Some(semio_framework_dsl_record::FieldValue::Text(text))=row.get(locale.index() as u16) else{return Err(workflow_invalid("expected declared label text"))};
                cells[terminology.index()][locale.index()]=control.copy_text(text)?;control.step()?;
            }
        }
        Ok::<(),semio_framework_value::ValueError>(())
    })?;
    Ok(semio_ui_locale::LocalizedLabel::from_fn(|terminology,locale|std::mem::take(&mut cells[terminology.index()][locale.index()])))
}
//#endregion 🏷️LocalizedLabelRecord
//#endregion 🔖️MediaContractDsl
