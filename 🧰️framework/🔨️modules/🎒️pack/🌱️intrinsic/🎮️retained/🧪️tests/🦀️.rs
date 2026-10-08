//! 🧪️ Neutral wire vectors and original intrinsic values exercise bounded reconstruction.
use super::*;
use serde_json::Value;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneStep};

fn corpus()->Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn hex(text:&str)->Vec<u8>{text.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect()}
fn close(cursor:&mut RetainedIntrinsicBody<'_>){
 for _ in 0..100000{
  if cursor.terminal_is_empty(){return}
  let copy=cursor.next_close_copy_byte_demand().unwrap();let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();
  cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(3),maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap();
 }
 panic!("retained intrinsic close did not terminate");
}
#[test]
fn retained_intrinsic_body_neutral_vectors_preserve_serde_and_one_unit_progress(){
 for row in corpus()["cases"].as_array().unwrap(){for units in [1,3,256]{
  let bytes=hex(row["hex"].as_str().unwrap());let mut cursor=RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<20).unwrap();
  assert_eq!(cursor.advance(0,false).unwrap().units,0);
  for _ in 0..100000{let step=cursor.advance(units,false).unwrap();assert!(step.units<=units);if step.complete{break}}
  let value=cursor.take_output().expect("complete output");assert_eq!(Value::from(value),row["expected"],"{}",row["id"]);assert!(cursor.admitted_bytes()<=1<<20);close(&mut cursor);
 }}
}
#[test]
fn retained_intrinsic_body_original_corpus_and_partial_cancel_close(){
 let rows:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows["cases"].as_array().unwrap(){
  let value=if row["kind"]=="bytes"{DslValue::Bytes(row["expected"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect())}else{DslValue::from(&row["expected"])};
  let mut callback=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1<<24,&mut callback);let bytes=crate::record::intrinsic::encode_body(&value,&Default::default(),&mut control).unwrap();
  let mut cursor=RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<24).unwrap();for _ in 0..100000{if cursor.advance(1,false).unwrap().complete{break}}assert_eq!(cursor.take_output().unwrap(),value);close(&mut cursor);
  for cutoff in [0,1,3,bytes.len(),bytes.len()*2]{let mut cursor=RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<24).unwrap();cursor.advance(cutoff,false).unwrap();assert!(cursor.advance(1,true).is_err());close(&mut cursor)}
 }
}
#[test]
fn retained_intrinsic_body_malformed_and_capacity_refusals_retain_close_ownership(){
 for text in corpus()["malformed"].as_array().unwrap(){let bytes=hex(text.as_str().unwrap());let mut cursor=RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<20).unwrap();let mut refused=false;for _ in 0..10000{match cursor.advance(1,false){Ok(step)=>assert!(!step.complete),Err(_)=>{refused=true;break}}}close(&mut cursor);assert!(refused,"{text}")}
 let bytes=hex("0107e99baaf09f98800101110600");let mut cursor=RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1).unwrap();assert!(cursor.advance(1,false).is_err());assert_eq!(cursor.admitted_bytes(),0);close(&mut cursor);
}

#[test]
fn retained_intrinsic_body_exact_requests_and_typed_physical_close_receipts(){
 let bytes=hex("0107e99baaf09f98800101110600");
 let (mut cursor,requests,releases)=crate::test_allocation::observe_backing(||RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<20).unwrap());assert_eq!((requests,releases),(0,0));
 for _ in 0..100000{let before=cursor.admitted_bytes();let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(1,false).unwrap());assert_eq!(releases,0,"partial construction retains every born allocation");assert_eq!(cursor.admitted_bytes(),step.admitted_bytes);assert_eq!(requests,step.admitted_bytes-before);if step.complete{break}}
 let admitted=cursor.admitted_bytes();
 let(_,requests,releases)=crate::test_allocation::observe_backing(||cursor.close_step(RetainedCloneGrant::default()).unwrap());assert_eq!((requests,releases),(0,0));assert_eq!(cursor.admitted_bytes(),admitted);
 let mut actual_releases=0;
 for _ in 0..100000{
  if cursor.terminal_is_empty(){break}
  let copy=cursor.next_close_copy_byte_demand().unwrap().max(3);let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();
  let(step,requests,releases)=crate::test_allocation::observe_backing(||cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap());
  let receipt=match step{RetainedCloneStep::Complete(receipt)|RetainedCloneStep::Progress(receipt)=>receipt};assert_eq!(requests,receipt.retained_capacity_bytes);assert_eq!(releases,receipt.released_bytes);actual_releases+=releases;
 }
 assert!(cursor.terminal_is_empty());assert!(actual_releases>=admitted);
 eprintln!("[DEBUG] retained intrinsic Body actual allocator requests and terminal releases match independent typed receipts");
}

#[test]
fn retained_intrinsic_body_owned_input_returns_on_refusal_and_closes_original_pointer(){
 let bytes=hex("0107e99baaf09f98800101110600");let pointer=bytes.as_ptr();let backing=bytes.capacity();
 let((error,input),requests,releases)=crate::test_allocation::observe_backing(||match RetainedIntrinsicBody::new(RetainedIntrinsicInput::OwnedBytes(bytes),Default::default(),backing-1){Err(result)=>result,Ok(mut cursor)=>{close(&mut cursor);panic!("one-short source admission accepted")}});assert_eq!((requests,releases),(0,0));assert!(matches!(error,PackRefusal::LimitExceeded{kind:semio_framework_value::ValueRefusalKind::OwnershipLimit,..}));let RetainedIntrinsicInput::OwnedBytes(bytes)=input else{panic!("owned source lost")};assert_eq!(bytes.as_ptr(),pointer);
 let(mut cursor,requests,releases)=crate::test_allocation::observe_backing(||RetainedIntrinsicBody::new(RetainedIntrinsicInput::OwnedBytes(bytes),Default::default(),1<<20).unwrap());assert_eq!((requests,releases),(0,0));assert_eq!(cursor.input().unwrap().as_ptr(),pointer);assert_eq!(cursor.admitted_bytes(),backing);
 for cutoff in [1,3,7]{cursor.advance(cutoff,false).unwrap();assert_eq!(cursor.input().unwrap().as_ptr(),pointer)}assert!(cursor.advance(1,true).is_err());close(&mut cursor);assert!(cursor.terminal_is_empty());
}

#[test]
fn retained_intrinsic_body_logical_depth_matches_original_intrinsic_boundary(){
 for(text,depth,accepted)in[("0001011112",1,true),("0001011110010701780c0204121000",2,true),("0001011110010701780c0204121000",1,false)]{
  let bytes=hex(text);let mut limits=crate::PackLimits::default();limits.max_depth=depth;let mut cursor=RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),limits,1<<20).unwrap();let mut complete=false;let mut refused=false;
  for _ in 0..100000{match cursor.advance(1,false){Ok(step)=>if step.complete{complete=true;break},Err(_)=>{refused=true;break}}}close(&mut cursor);assert_eq!(complete,accepted,"{text} depth {depth}");assert_eq!(refused,!accepted);
 }
}

#[test]
fn retained_intrinsic_body_terminal_cancellation_withholds_output_until_typed_close(){
 let bytes=hex("0107e99baaf09f98800101110600");let mut cursor=RetainedIntrinsicBody::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<20).unwrap();for _ in 0..100000{if cursor.advance(1,false).unwrap().complete{break}}
 assert!(cursor.advance(1,true).is_err());let withheld=cursor.take_output();let present=withheld.is_some();if let Some(value)=withheld{drop(value)}close(&mut cursor);assert!(!present,"a canceled operation cannot publish its terminal retained value");
}

fn close_document(cursor:&mut RetainedIntrinsicDocument<'_>){
 for _ in 0..1000000{if cursor.terminal_is_empty(){return}let copy=cursor.next_close_copy_byte_demand().unwrap().max(3);let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap();}panic!("retained intrinsic Document close did not terminate");
}

#[test]
fn retained_intrinsic_document_original_corpus_framing_compression_and_chunk_custody(){
 let rows:Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();let plan=corpus();
 for row in rows["cases"].as_array().unwrap(){for codec in plan["document"]["codecs"].as_array().unwrap(){for frame in plan["document"]["frameBytes"].as_array().unwrap(){
  let value=if row["kind"]=="bytes"{DslValue::Bytes(row["expected"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect())}else{DslValue::from(&row["expected"])};let mut options=crate::record::EncodeOptions::default();options.codec=crate::CodecId(codec.as_u64().unwrap()as u8);options.frame_size=frame.as_u64().unwrap();options.chunk_threshold=plan["document"]["chunkThreshold"].as_u64().unwrap();options.chunk_size=1;
  let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1<<24,&mut allow);let bytes=crate::record::intrinsic::encode_document(&value,&options,&mut control).unwrap();let mut cursor=RetainedIntrinsicDocument::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<24).unwrap();assert_eq!(cursor.advance(0,false).unwrap().units,0);
  for _ in 0..1000000{let step=cursor.advance(1,false).unwrap();assert!(step.units<=1);if step.complete{break}}
  let output=cursor.take_output();close_document(&mut cursor);assert_eq!(output,Some(value),"{} codec {codec} frame {frame}",row["id"]);
 }}}
}

#[test]
fn retained_intrinsic_document_explicit_body_refusal_and_partial_cancel_close(){
 let body=hex("0001011112");let mut cursor=RetainedIntrinsicDocument::new(RetainedIntrinsicInput::Borrowed(&body),Default::default(),1<<20).unwrap();let mut refused=false;for _ in 0..100000{match cursor.advance(1,false){Ok(step)=>assert!(!step.complete),Err(_)=>{refused=true;break}}}close_document(&mut cursor);assert!(refused,"Document cannot fall back to Body");
 let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1<<24,&mut allow);let bytes=crate::record::intrinsic::encode_document(&DslValue::String("雪😀".repeat(1024)),&Default::default(),&mut control).unwrap();for cutoff in [0,1,3,256,bytes.len(),bytes.len()*3]{let mut cursor=RetainedIntrinsicDocument::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<24).unwrap();cursor.advance(cutoff,false).unwrap();assert!(cursor.advance(1,true).is_err());assert!(cursor.take_output().is_none());close_document(&mut cursor)}
}

#[test]
fn retained_intrinsic_document_schema_graph_agrees_with_canonical_and_third_party_hash(){
 use semio_framework_dsl_record::{FieldSpec,RecordSpec,RecordLayout,Shape};
 let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);let hash=crate::record::schema_hash(&spec);assert_eq!(hash,*blake3::hash(super::retained::schema_graph()).as_bytes());
 let mut own=semio_framework_hash::Hasher::new();for byte in super::retained::schema_graph(){own.update(&[*byte]);}assert_eq!(hash,*own.finalize().as_bytes());
}

fn document_octets()->Vec<u8>{let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1<<24,&mut allow);let mut options=crate::record::EncodeOptions::default();options.codec=crate::CodecId(0);options.chunk_threshold=2;options.chunk_size=1;crate::record::intrinsic::encode_document(&DslValue::Bytes(vec![0,127,255]),&options,&mut control).unwrap()}
fn word(bytes:&[u8],offset:&mut usize)->usize{let mut result=0;let mut shift=0;loop{let byte=bytes[*offset];*offset+=1;result|=usize::from(byte&127)<<shift;if byte&128==0{return result}shift+=7;}}

#[test]
fn retained_intrinsic_document_referenced_chunk_catalog_integrity_is_mandatory(){
 for tamper in corpus()["document"]["catalogTamper"].as_array().unwrap(){
  let mut bytes=document_octets();let mut offset=crate::format::HEADER_SIZE;
  loop{let start=offset;let kind=bytes[offset];offset+=1;let flags=bytes[offset];offset+=1;assert_eq!(flags,0);let stored=word(&bytes,&mut offset);let payload=offset;let end=payload+stored;
   if kind==crate::KIND_CHUNK_TABLE{assert_eq!(word(&bytes,&mut offset),3);for _ in 0..3{word(&bytes,&mut offset);}if tamper=="blake3"{offset+=4;}bytes[offset]^=1;let crc=crate::crc32c(&bytes[start..end]);bytes[end..end+4].copy_from_slice(&crc.to_le_bytes());break}offset=end+4;assert!(offset<bytes.len()-crate::format::FOOTER_SIZE);
  }
  let mut cursor=RetainedIntrinsicDocument::new(RetainedIntrinsicInput::Borrowed(&bytes),Default::default(),1<<24).unwrap();let mut refused=false;for _ in 0..1000000{match cursor.advance(1,false){Ok(step)=>if step.complete{break},Err(_)=>{refused=true;break}}}let output=cursor.take_output();close_document(&mut cursor);assert!(refused,"{tamper} catalog witness accepted");assert!(output.is_none());
 }
}

#[test]
fn retained_intrinsic_document_allocator_birth_and_terminal_physical_grants_are_exact(){
 let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(1<<24,&mut allow);let mut options=crate::record::EncodeOptions::default();options.codec=crate::CodecId(1);options.chunk_threshold=2;options.chunk_size=1;let bytes=crate::record::intrinsic::encode_document(&DslValue::Array(vec![DslValue::String("雪😀".into()),DslValue::Bytes(vec![0,127,255])]),&options,&mut control).unwrap();let pointer=bytes.as_ptr();let backing=bytes.capacity();
 let (mut cursor,requests,releases)=crate::test_allocation::observe_backing(||RetainedIntrinsicDocument::new(RetainedIntrinsicInput::OwnedBytes(bytes),Default::default(),1<<24).unwrap());assert_eq!((requests,releases),(0,0));assert_eq!(cursor.input().unwrap().as_ptr(),pointer);assert_eq!(cursor.admitted_bytes(),backing);
 for _ in 0..1000000{let before=cursor.admitted_bytes();let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.advance(1,false));if let Err(error)=result{close_document(&mut cursor);panic!("unexpected document refusal: {error:?}")}let step=result.unwrap();assert_eq!(requests,step.admitted_bytes-before);assert_eq!(releases,0);assert_eq!(cursor.input().unwrap().as_ptr(),pointer);if step.complete{break}}
 let admitted=cursor.admitted_bytes();assert!(cursor.advance(1,true).is_err());assert!(cursor.take_output().is_none());let mut actual_releases=0;
 for _ in 0..1000000{if cursor.terminal_is_empty(){break}let copy=cursor.next_close_copy_byte_demand().unwrap().max(3);let release=cursor.next_close_release_byte_demand().unwrap();let capacity=cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap();let depth=cursor.next_close_depth_demand().unwrap();let(result,requests,releases)=crate::test_allocation::observe_backing(||cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}));let receipt=match result.unwrap(){RetainedCloneStep::Complete(receipt)|RetainedCloneStep::Progress(receipt)=>receipt};assert_eq!(requests,receipt.retained_capacity_bytes);assert_eq!(releases,receipt.released_bytes);actual_releases+=releases;}
 assert!(cursor.terminal_is_empty());assert!(actual_releases>=admitted);eprintln!("[DEBUG] retained Document original source, inflater, catalog, chunk-reference and partial-value allocations close under independent exact receipts");
}
