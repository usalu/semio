//! 🧪️ Canonical intrinsic Pack boundaries preserve literal bytes and physical caller admission.
use crate::record::{self,intrinsic,EncodeOptions,DecodeOptions};
use semio_framework_value::{DslValue,NativeEncodeControl,NativeDecodeControl,ValueRefusalKind};

fn octets(text:&str)->Vec<u8>{text.as_bytes().chunks_exact(2).map(|word|u8::from_str_radix(std::str::from_utf8(word).unwrap(),16).unwrap()).collect()}
fn tagged(value:&DslValue)->serde_json::Value{serde_json::Value::from(semio_framework_value::intrinsic_json::encode(value))}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn literal(row:&serde_json::Value)->DslValue{semio_framework_value::intrinsic_json::decode(&DslValue::from(&row["value"])).unwrap()}
fn reference_record(value:&DslValue)->(semio_framework_dsl_record::RecordSpec,semio_framework_dsl_record::RecordValue){
    use semio_framework_dsl_record::{RecordSpec,RecordLayout,FieldSpec,Shape,RecordFields,RecordValue,FieldValue};
    let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);
    let mut fields=RecordFields::default();fields.insert(1,FieldValue::Value(value.clone()));(spec,RecordValue{fields})
}

struct OriginalIntrinsic<'a>(&'a DslValue);
impl OriginalIntrinsic<'_>{
    fn at(&self,path:&[usize])->Result<&DslValue,semio_framework_value::ValueError>{
        let Some((0,path))=path.split_first()else{return Err(semio_framework_dsl_record::native_encoding::projection_path_error())};
        let mut value=self.0;for index in path{value=match value{DslValue::Array(items)=>items.get(*index),DslValue::Object(items)=>items.get(*index).map(|(_,value)|value),_=>None}.ok_or_else(semio_framework_dsl_record::native_encoding::projection_path_error)?;}Ok(value)
    }
}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for OriginalIntrinsic<'_>{
    fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{
        use semio_framework_dsl_record::native_encoding::FieldProjectionView as V;
        if path.is_empty(){return Ok(V::Record(&[1]))}Ok(match self.at(path)?{DslValue::Null=>V::IntrinsicNull,DslValue::Bool(value)=>V::IntrinsicBool(*value),DslValue::Number(value)=>V::IntrinsicNumber(*value),DslValue::String(text)=>V::IntrinsicText(text),DslValue::Bytes(bytes)=>V::IntrinsicBytes(bytes),DslValue::Array(items)=>V::IntrinsicArray(items.len()),DslValue::Object(items)=>V::IntrinsicObject(items.len())})
    }
    fn projection_key(&self,path:&[usize],index:usize)->Result<&str,semio_framework_value::ValueError>{match self.at(path)?{DslValue::Object(items)=>items.get(index).map(|(key,_)|key.as_str()).ok_or_else(semio_framework_dsl_record::native_encoding::projection_path_error),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
}
fn occurrence_order_fixture()->serde_json::Value{serde_json::from_str(include_str!("../🔢️occurrence-order/🧫️fixtures/🔣️.json")).unwrap()}
fn close_output(output:&mut protocol::io::binary::operation_bytes::OwnedOperationBytes){use protocol::io::binary::operation_bytes::OperationByteCloseStep as Step;let backing=output.allocated_bytes();let mut retired=0;for _ in 0..17000{let(step,birth,free)=crate::test_allocation::observe_backing(||output.close_one(1,4096).unwrap());assert_eq!(birth,0);let release=match step{Step::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released_bytes},Step::Complete=>0};assert_eq!(free,release);retired+=free;if step==Step::Complete{break}}assert!(output.terminal_is_empty());assert_eq!(output.allocated_bytes(),0);assert_eq!(retired,backing);}

#[test]
fn intrinsic_borrowed_object_order_preserves_original_payload_and_every_equal_key_occurrence(){
    for row in occurrence_order_fixture()["cases"].as_array().unwrap(){
        let value=semio_framework_value::intrinsic_json::decode(&DslValue::from(&row["source"])).unwrap();let source=OriginalIntrinsic(&value);let(spec,_)=reference_record(&DslValue::Null);let expected=octets(row["bodyHex"].as_str().unwrap());
        let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(expected.len(),65536).unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::new(16_777_216,&mut allow);
        assert_eq!(record::encode_projected_record_body_into(&spec,&source,&EncodeOptions::default(),&mut output,&mut control).unwrap(),expected.len());assert!(output.iter().eq(expected.iter().copied()),"{}",row["id"]);close_output(&mut output);assert_eq!(tagged(&value),row["source"]);
    }
    eprintln!("[DEBUG] borrowed intrinsic occurrence order5 original payloads retained duplicate occurrences");
}

#[test]
fn intrinsic_static_borrowed_object_order_keeps_paid_scratch_until_physical_close(){
    use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as S,BorrowedShape as B,RecordLayout};
    static FIELDS:[F;1]=[F::new(1,"value",B::Value)];
    let options=EncodeOptions::default();
    for row in occurrence_order_fixture()["cases"].as_array().unwrap(){
        let value=semio_framework_value::intrinsic_json::decode(&DslValue::from(&row["source"])).unwrap();let source=OriginalIntrinsic(&value);let expected=octets(row["bodyHex"].as_str().unwrap());let mut output=protocol::io::binary::operation_bytes::OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
        assert_eq!(output.allocated_bytes(),0);let mut capsule=record::BorrowedProjectedPackOperation::from_source(&source,S{keyword:None,layout:RecordLayout::Lines,fields:&FIELDS});let mut allow=|_|true;let mut control=NativeEncodeControl::new(16_777_216,&mut allow);
        let(result,birth,free)=crate::test_allocation::observe_backing(||capsule.write_body(&options,&mut output,&mut control));assert_eq!(result.unwrap(),expected.len());assert_eq!(birth,capsule.allocated_bytes()+output.allocated_bytes());assert_eq!(free,0);assert_eq!(control.owned_bytes(),birth);assert!(output.iter().eq(expected.iter().copied()),"{}",row["id"]);
        let scratch=capsule.allocated_bytes();let mut retired=0;for _ in 0..32768{let(step,birth,free)=crate::test_allocation::observe_backing(||capsule.retire_one(1,4096).unwrap());assert_eq!(birth,0);assert_eq!(free,step.2);assert!(step.1<=1&&free<=4096);retired+=free;if !step.0{break}}assert_eq!(retired,scratch);assert_eq!(capsule.allocated_bytes(),0);close_output(&mut output);assert_eq!(tagged(&value),row["source"]);
    }
    eprintln!("[DEBUG] static borrowed intrinsic occurrence order5 retained original payloads exact paid scratch release");
}

#[test]
fn intrinsic_boundary_original_object_order_preserves_every_equal_key_occurrence(){
    let fixture=occurrence_order_fixture();
    let output=EncodeOptions::default();let input=DecodeOptions::default();
    for row in fixture["cases"].as_array().unwrap(){
        let source=semio_framework_value::intrinsic_json::decode(&DslValue::from(&row["source"])).unwrap();
        let expected=octets(row["bodyHex"].as_str().unwrap());let mut allow=|_|true;
        let mut control=NativeEncodeControl::new(16_777_216,&mut allow);
        let body=intrinsic::encode_body(&source,&output,&mut control).unwrap();assert_eq!(body,expected,"{}",row["id"]);
        let mut allow_decode=|_|true;let mut decode=NativeDecodeControl::new(16_777_216,&mut allow_decode);
        assert_eq!(intrinsic::decode_body(&body,&input,&mut decode).unwrap(),source,"{}",row["id"]);
        let(spec,reference)=reference_record(&source);let expected_document=record::encode_document(&spec,&reference,&output).unwrap();
        let mut control=NativeEncodeControl::new(16_777_216,&mut allow);
        let document=intrinsic::encode_document(&source,&output,&mut control).unwrap();assert_eq!(document,expected_document,"{}",row["id"]);
        let oracle:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&source)).unwrap();
        assert_eq!(oracle,serde_json::Value::from(&source),"{}",row["id"]);
    }
    eprintln!("[DEBUG] intrinsic original object order5 neutral vectors body/document/serde oracle preserved duplicate occurrences");
}

#[test]
fn intrinsic_boundary_literal_body_and_document_match_the_owned_schema(){
    let fixture=fixture();let output=EncodeOptions::default();let input=DecodeOptions::default();
    for row in fixture["cases"].as_array().unwrap(){
        let value=literal(row);let expected=octets(row["bodyHex"].as_str().unwrap());let mut allow=|_|true;let mut allow_decode=|_|true;
        let mut encode=NativeEncodeControl::new(usize::MAX,&mut allow);let bytes=intrinsic::encode_body(&value,&output,&mut encode).unwrap();assert_eq!(bytes,expected,"{}",row["id"]);
        let mut decode=NativeDecodeControl::new(usize::MAX,&mut allow_decode);let decoded=intrinsic::decode_body(&bytes,&input,&mut decode).unwrap();assert_eq!(tagged(&decoded),row["value"]);
        let(spec,record)=reference_record(&value);let reference=record::encode_document(&spec,&record,&output).unwrap();
        let mut encode=NativeEncodeControl::new(usize::MAX,&mut allow);let document=intrinsic::encode_document(&value,&output,&mut encode).unwrap();assert_eq!(document,reference,"{}",row["id"]);
        let mut decode=NativeDecodeControl::new(usize::MAX,&mut allow_decode);let decoded=intrinsic::decode_document(&document,&input,&mut decode).unwrap();assert_eq!(tagged(&decoded),row["value"]);
        let semantic:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&decoded)).unwrap();assert_eq!(semantic,serde_json::Value::from(&value));
    }
    eprintln!("[DEBUG] canonical intrinsic Pack literal/body/document/serde oracle cases={}",fixture["cases"].as_array().unwrap().len());
}

#[test]
fn intrinsic_object_order_scratch_is_paid_by_exact_cumulative_caller_authority(){
    let options=EncodeOptions::default();
    for row in occurrence_order_fixture()["cases"].as_array().unwrap(){
        let value=semio_framework_value::intrinsic_json::decode(&DslValue::from(&row["source"])).unwrap();let expected=octets(row["bodyHex"].as_str().unwrap());let mut allow=|_|true;let mut control=NativeEncodeControl::new(16_777_216,&mut allow);
        let(body,requested)=crate::test_allocation::observe(||intrinsic::encode_body(&value,&options,&mut control));assert_eq!(body.unwrap(),expected);assert_eq!(control.owned_bytes(),requested,"{}",row["id"]);
        let mut exact=NativeEncodeControl::new(requested,&mut allow);let(body,actual)=crate::test_allocation::observe(||intrinsic::encode_body(&value,&options,&mut exact));assert_eq!(body.unwrap(),expected);assert_eq!(actual,requested);
        let mut short=NativeEncodeControl::new(requested-1,&mut allow);assert_eq!(intrinsic::encode_body(&value,&options,&mut short).unwrap_err().kind(),ValueRefusalKind::OwnershipLimit);
        let mut cumulative=NativeEncodeControl::new(requested*2-1,&mut allow);assert_eq!(intrinsic::encode_body(&value,&options,&mut cumulative).unwrap(),expected);assert_eq!(intrinsic::encode_body(&value,&options,&mut cumulative).unwrap_err().kind(),ValueRefusalKind::OwnershipLimit);
        let mut deny=|_|false;let mut canceled=NativeEncodeControl::new(16_777_216,&mut deny);assert_eq!(intrinsic::encode_body(&value,&options,&mut canceled).unwrap_err().kind(),ValueRefusalKind::Canceled);assert_eq!(canceled.owned_bytes(),0);assert_eq!(tagged(&value),row["source"]);
    }
    eprintln!("[DEBUG] intrinsic object order5 actual index/frontier/output allocations exactly charged; short cumulative and canceled authority refused");
}

#[test]
fn intrinsic_boundary_refuses_extra_missing_unknown_and_suffix_fields(){
    let fixture=fixture();let input=DecodeOptions::default();
    for row in fixture["refusals"].as_array().unwrap(){let bytes=octets(row["bodyHex"].as_str().unwrap());let mut allow=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);let error=intrinsic::decode_body(&bytes,&input,&mut control).unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::InvalidValue,"{}",row["id"]);}
    let(spec,mut record)=reference_record(&DslValue::Null);record.fields.insert(2,semio_framework_dsl_record::FieldValue::Value(DslValue::Bool(true)));let bytes=record::encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let mut allow=|_|true;let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(intrinsic::decode_document(&bytes,&input,&mut control).unwrap_err().kind(),ValueRefusalKind::InvalidValue);
    let(spec,mut record)=reference_record(&DslValue::Null);record.fields.remove(&1);let bytes=record::encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(intrinsic::decode_document(&bytes,&input,&mut control).unwrap_err().kind(),ValueRefusalKind::InvalidValue);
    let(mut spec,record)=reference_record(&DslValue::Null);spec.fields[0].key="other".into();let bytes=record::encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let mut control=NativeDecodeControl::new(usize::MAX,&mut allow);assert_eq!(intrinsic::decode_document(&bytes,&input,&mut control).unwrap_err().kind(),ValueRefusalKind::InvalidValue);
}

#[test]
fn intrinsic_boundary_settles_actual_requests_and_cumulative_cancellation(){
    let value=DslValue::String("a".repeat(131072));let output=EncodeOptions::default();let input=DecodeOptions::default();
    for document in [false,true]{
        let mut allow=|_|true;let mut control=NativeEncodeControl::new(usize::MAX,&mut allow);
        let(encoded,requested)=crate::test_allocation::observe(||if document{intrinsic::encode_document(&value,&output,&mut control)}else{intrinsic::encode_body(&value,&output,&mut control)});let bytes=encoded.unwrap();assert_eq!(control.owned_bytes(),requested,"complete physical producer allowance");
        let mut exact=NativeEncodeControl::new(requested,&mut allow);let(encoded,actual)=crate::test_allocation::observe(||if document{intrinsic::encode_document(&value,&output,&mut exact)}else{intrinsic::encode_body(&value,&output,&mut exact)});assert_eq!(encoded.unwrap(),bytes);assert_eq!(actual,requested);
        let mut short=NativeEncodeControl::new(requested-1,&mut allow);let error=if document{intrinsic::encode_document(&value,&output,&mut short)}else{intrinsic::encode_body(&value,&output,&mut short)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut cumulative=NativeEncodeControl::new(requested*2-1,&mut allow);let first=if document{intrinsic::encode_document(&value,&output,&mut cumulative)}else{intrinsic::encode_body(&value,&output,&mut cumulative)}.unwrap();assert_eq!(first,bytes);let error=if document{intrinsic::encode_document(&value,&output,&mut cumulative)}else{intrinsic::encode_body(&value,&output,&mut cumulative)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut deny=|_|false;let mut canceled=NativeEncodeControl::new(usize::MAX,&mut deny);let error=if document{intrinsic::encode_document(&value,&output,&mut canceled)}else{intrinsic::encode_body(&value,&output,&mut canceled)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::Canceled);assert_eq!(canceled.owned_bytes(),0);
        let mut allow=|_|true;let mut decode=NativeDecodeControl::new(usize::MAX,&mut allow);let(decoded,requested)=crate::test_allocation::observe(||if document{intrinsic::decode_document(&bytes,&input,&mut decode)}else{intrinsic::decode_body(&bytes,&input,&mut decode)});assert_eq!(decoded.unwrap(),value);assert_eq!(decode.owned_bytes(),requested,"complete physical decoder allowance");
        let mut exact=NativeDecodeControl::new(requested,&mut allow);let(decoded,actual)=crate::test_allocation::observe(||if document{intrinsic::decode_document(&bytes,&input,&mut exact)}else{intrinsic::decode_body(&bytes,&input,&mut exact)});assert_eq!(decoded.unwrap(),value);assert_eq!(actual,requested);
        let mut short=NativeDecodeControl::new(requested-1,&mut allow);let error=if document{intrinsic::decode_document(&bytes,&input,&mut short)}else{intrinsic::decode_body(&bytes,&input,&mut short)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut cumulative=NativeDecodeControl::new(requested*2-1,&mut allow);let first=if document{intrinsic::decode_document(&bytes,&input,&mut cumulative)}else{intrinsic::decode_body(&bytes,&input,&mut cumulative)}.unwrap();assert_eq!(first,value);let error=if document{intrinsic::decode_document(&bytes,&input,&mut cumulative)}else{intrinsic::decode_body(&bytes,&input,&mut cumulative)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::OwnershipLimit);
        let mut deny=|_|false;let mut canceled=NativeDecodeControl::new(usize::MAX,&mut deny);let error=if document{intrinsic::decode_document(&bytes,&input,&mut canceled)}else{intrinsic::decode_body(&bytes,&input,&mut canceled)}.unwrap_err();assert_eq!(error.kind(),ValueRefusalKind::Canceled);assert_eq!(canceled.owned_bytes(),0);
        eprintln!("[DEBUG] intrinsic Pack document={document} caller ownership requests={requested} exact/short/cancel executed");
    }
}
