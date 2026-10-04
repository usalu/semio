//! 🎞️ Canonical one-field intrinsic Pack documents and exact terminal record bodies.
use super::{EncodeOptions,DecodeOptions,PackRefusal};
use semio_framework_value::{DslValue,NativeEncodeControl,NativeDecodeControl,ValueRefusalKind};
use semio_framework_dsl_record::{NativeSchemaControl,RecordSpec,RecordLayout,Shape,FieldValue};

const FIELD_ID:u16=1;
fn schema<C:NativeSchemaControl>(control:&mut C)->Result<RecordSpec,PackRefusal>{
    let mut fields=control.allocate_vec(1)?;fields.push(semio_framework_dsl_record::producer::field(FIELD_ID,"value",Shape::Value,control)?);
    Ok(semio_framework_dsl_record::producer::record(None,RecordLayout::Lines,fields,control)?)
}
fn invalid(detail:&'static str)->PackRefusal{PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"intrinsic document",offset:0,detail}}

/// 🛫️ Borrows the complete value through canonical terminal body emission and one caller control.
pub fn encode_body(value:&DslValue,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{
    super::encode_value_record_body_controlled(FIELD_ID,value,options,control)
}

/// 🛬️ Materializes exactly one intrinsic field without accepting unknown fields or suffix bytes.
pub fn decode_body(bytes:&[u8],options:&DecodeOptions,control:&mut NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    super::decode_value_record_body_exact_controlled(bytes,FIELD_ID,options,control)
}

/// 📦️ Writes the borrowed value with the authored one-field schema and cumulative physical admission.
pub fn encode_document(value:&DslValue,options:&EncodeOptions,control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX);
    control.scoped_maximum(maximum,|control|{control.checkpoint()?;let spec=schema(control)?;super::controlled_encoding::intrinsic_document(&spec,FIELD_ID,value,options,control)})
}

/// 📤️ Moves the one decoded value from its declared document field after exact schema admission.
pub fn decode_document(bytes:&[u8],options:&DecodeOptions,control:&mut NativeDecodeControl<'_>)->Result<DslValue,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX);
    control.scoped_maximum(maximum,|control|{
        control.checkpoint()?;let spec=schema(control)?;let(mut record,report)=super::decode_document_controlled(bytes,&spec,options,control)?;
        if report.schema_drift||!report.unknown_field_ids.is_empty()||record.fields.len()!=1{return Err(invalid("expected the declared single Value field and exact schema"))}
        let Some(FieldValue::Value(value))=record.fields.remove(&FIELD_ID)else{return Err(invalid("declared intrinsic field has a different shape"))};control.checkpoint().map_err(PackRefusal::from)?;Ok(value)
    })
}
