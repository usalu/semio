use super::*;
use semio_framework_replication_codec::PackEncodeOptions as EncodeOptions;
fn encode_document(spec:&semio_framework_dsl_record::RecordSpec,record:&semio_framework_dsl_record::RecordValue,options:&EncodeOptions)->Result<Vec<u8>,semio_framework_pack_error::PackRefusal>{
 semio_framework_pack_record_encode::encode_document_controlled(spec,record,options,&mut semio_framework_value::native_encoding::NativeEncodeControl::new(1<<30,&mut |_|true))
}
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedShape as B,RecordLayout,DslField,FieldValue};
use semio_framework_dsl_record_derive::DslRecord;
#[derive(Debug,PartialEq,DslRecord)]
struct Original{name:String,enabled:bool}

#[derive(Debug,PartialEq,DslRecord)]
struct ExpressionOriginal{formula:semio_framework_dsl_record::ExprValue,text:String}
#[test]
fn borrowed_pack_document_derived_expression_joins_original_symbol_and_default_codec(){
 use semio_framework_dsl_record::{ExprValue as E,ExprOp,BorrowedDslRecord};let source=ExpressionOriginal{formula:E::Binary(ExprOp::Add,Box::new(E::Num(1.0)),Box::new(E::Binary(ExprOp::Add,Box::new(E::Num(2.0)),Box::new(E::Num(3.0))))),text:"1 + (2 + 3)".into()};let semio_framework_dsl_record::Shape::Record(make)=ExpressionOriginal::shape()else{panic!("derived Record")};let FieldValue::Record(record)=source.to_value()else{panic!("derived Record")};let expected=encode_document(&(make.ordinary)(),&record,&Default::default()).unwrap();let mut cursor=BorrowedPackDocumentCursor::default();let mut actual=Vec::new();let mut output=[0;1];for _ in 0..2000000{let grant=exact(&cursor);let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,ExpressionOriginal::RECORD,&mut output,grant).unwrap());assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));assert!(step.progress.fits(grant));actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}let complete=cursor.is_complete();close(&mut cursor);assert!(complete);assert_eq!(actual,expected);let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut accepted);assert_eq!(source.to_value_controlled(&mut control).unwrap(),FieldValue::Record(record));
}
static FIELDS:[F;2]=[F::new(0,"name",B::Text),F::new(1,"enabled",B::Bool)];
fn spec()->BorrowedRecordSpec{BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS}}
fn exact(cursor:&BorrowedPackDocumentCursor)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:cursor.next_copy_byte_demand().unwrap(),maximum_capacity_bytes:cursor.next_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand().unwrap()}}
fn close(cursor:&mut BorrowedPackDocumentCursor){for _ in 0..300000{let demand=cursor.retirement_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.close(grant).unwrap());assert_eq!(birth,0);assert_eq!(free,step.progress.released_bytes);assert!(step.progress.fits(grant));if step.complete{assert!(cursor.terminal_is_empty());return;}}panic!("document owners did not close");}
#[test]
fn borrowed_pack_document_default_codec_exact_original_output_and_cancellation(){
 assert!(std::mem::size_of::<BorrowedPackDocumentCursor>()<=4096);
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["records"].as_array().unwrap(){
  let source=Original{name:row["name"].as_str().unwrap().into(),enabled:row["enabled"].as_bool().unwrap()};let Bowned=Original::shape();let semio_framework_dsl_record::Shape::Record(make)=Bowned else{panic!("original Record")};let FieldValue::Record(record)=source.to_value()else{panic!("original Record")};let expected=encode_document(&(make.ordinary)(),&record,&Default::default()).unwrap();
  let mut cursor=BorrowedPackDocumentCursor::default();let mut output=[0;1];let mut actual=Vec::new();let mut requested=false;
  for _ in 0..2000000{let grant=exact(&cursor);let source_request=cursor.phase==Phase::Segment&&!cursor.hash_pending&&cursor.compressed.is_none()&&!cursor.deflate.is_complete()&&!cursor.need_raw&&cursor.deflate.phase().as_str()=="input"&&cursor.pending.is_none()&&!cursor.raw_done;if source_request{assert!(grant.maximum_copy_bytes>=fixture["controlBookkeeping"]["minimumGrantCopyBytes"].as_u64().unwrap()as usize);}
if grant.maximum_copy_bytes!=0{let(zero,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec(),&mut output,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant}).unwrap());assert_eq!(zero.progress,Default::default());assert_eq!((birth,free),(0,0));}
   let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec(),&mut output,grant).unwrap());assert_eq!(birth,step.progress.retained_capacity_bytes);assert_eq!(free,0);assert!(step.progress.fits(grant));if source_request{assert!(cursor.need_raw);assert_eq!(step.progress.copied_items,1);assert_eq!(step.progress.copied_bytes,fixture["controlBookkeeping"]["copyBytes"].as_u64().unwrap()as usize);requested=true;}actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}
  assert!(requested);assert!(cursor.is_complete());assert_eq!(cursor.output_len(),Some(expected.len()));assert_eq!(actual,expected);close(&mut cursor);
  let code="const z=require('node:zlib');const a=Buffer.from(process.argv[1],'hex');let p=32;const v=()=>{let n=0,s=0,b;do{b=a[p++];n+=(b&127)*2**s;s+=7;}while(b&128);return n;};let count=0;while(p<a.length-84){const k=a[p++],f=a[p++],n=v(),raw=f&1?v():n;const body=a.subarray(p,p+n);if(f&1){if(f!==3)throw Error('codec');if(z.inflateRawSync(body).length!==raw)throw Error('raw');count++;}p+=n+4;}if(count!==3)throw Error('segments');";
  let hex: String=actual.iter().map(|byte|format!("{byte:02x}")).collect();let oracle=std::process::Command::new("bun").args(["-e",code,&hex]).output().unwrap();assert!(oracle.status.success(),"{}",String::from_utf8_lossy(&oracle.stderr));
  for stop in fixture["cancelStops"].as_array().unwrap(){let mut cursor=BorrowedPackDocumentCursor::default();let mut prefix=Vec::new();for _ in 0..stop.as_u64().unwrap(){let step=cursor.advance(&source,spec(),&mut output,exact(&cursor)).unwrap();prefix.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}assert!(expected.starts_with(&prefix));close(&mut cursor);}
 }
}

#[test]
fn borrowed_pack_document_multiple_original_frames_match_canonical_oracle(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let frame=fixture["frameSplitBytes"].as_u64().unwrap()as usize;
 let source=Original{name:fixture["frameText"].as_str().unwrap().repeat(fixture["frameRepeat"].as_u64().unwrap()as usize),enabled:true};let semio_framework_dsl_record::Shape::Record(make)=Original::shape()else{panic!("Record")};let FieldValue::Record(record)=source.to_value()else{panic!("Record")};let mut options=EncodeOptions::default();options.frame_size=frame as u64;let expected=encode_document(&(make.ordinary)(),&record,&options).unwrap();
 let mut cursor=BorrowedPackDocumentCursor::with_options(&options).unwrap();let mut output=[0;3];let mut actual=Vec::new();
 for _ in 0..3000000{let step=cursor.advance(&source,spec(),&mut output,exact(&cursor)).unwrap();actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}
 assert!(cursor.is_complete());assert_eq!(actual,expected);close(&mut cursor);
}

#[test]
fn borrowed_pack_document_raw_body_limit_retains_refusal_receipt_and_output_custody(){
 let source=Original{name:"compressible".repeat(100),enabled:true};let mut cursor=BorrowedPackDocumentCursor::default().with_maximum_body_len(7);let mut output=[0xa5;1];let mut failed=false;
 for _ in 0..1000000{let grant=exact(&cursor);let(result,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec(),&mut output,grant));match result{Ok(step)=>{assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));assert_eq!(step.written_bytes,0);},Err(failure)=>{assert_eq!(failure.reason.kind,ValueRefusalKind::OwnershipLimit);assert!(failure.reason.retained_progress().fits(grant));assert_eq!((birth,free),(failure.reason.retained_progress().retained_capacity_bytes,0));assert_eq!(failure.written_bytes,0);assert_eq!(output,[0xa5]);failed=true;break;}}}
 close(&mut cursor);assert!(failed);
}
#[test]
fn borrowed_pack_document_source_depth_refuses_explicitly_without_grant_stall(){
 let source=Original{name:"one real child".into(),enabled:true};let mut cursor=BorrowedPackDocumentCursor::default().with_maximum_source_depth(0);let mut output=[0xa5;1];let mut failed=false;
 for _ in 0..200000{let grant=exact(&cursor);match cursor.advance(&source,spec(),&mut output,grant){Ok(step)=>assert_eq!(step.written_bytes,0),Err(failure)=>{assert_eq!(failure.reason.kind,ValueRefusalKind::DepthLimit);assert!(failure.reason.retained_progress().fits(grant));assert_eq!(failure.written_bytes,0);failed=true;break;}}}close(&mut cursor);assert!(failed);assert_eq!(output,[0xa5]);
}

#[derive(Debug,PartialEq,DslRecord)]
struct ChunkOriginal{#[dsl(base64)]data:Vec<u8>}
static CHUNK_FIELDS:[F;1]=[F::new(0,"data",B::Bytes64)];
struct PagedChunkOriginal{data:semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>}
impl semio_framework_dsl_record::native_encoding::FieldProjectionSource for PagedChunkOriginal{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,semio_framework_value::ValueError>{use semio_framework_dsl_record::native_encoding::FieldProjectionView as V;match path{[]=>Ok(V::Record(&[0])),[0]=>Ok(V::BytesSource(&self.data)),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
}

#[test]
fn borrowed_pack_document_standard_chunks_preserve_original_and_independent_hash(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧱️chunks/🔣️.json")).unwrap();let pattern:Vec<u8>=fixture["pattern"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();let source=ChunkOriginal{data:pattern.repeat(fixture["repeat"].as_u64().unwrap()as usize)};let original=source.data.as_ptr();
 let semio_framework_dsl_record::Shape::Record(make)=ChunkOriginal::shape()else{panic!("Record")};let FieldValue::Record(record)=source.to_value()else{panic!("Record")};let expected=encode_document(&(make.ordinary)(),&record,&Default::default()).unwrap();let spec=BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&CHUNK_FIELDS};let native=PagedChunkOriginal{data:semio_framework_value::paged::PagedBytes::try_from_slice(&source.data).unwrap()};let native_pointer=native.data.retained_bytes().backing_ptr(0);for source_view in [&source as &dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,&native]{let mut cursor=BorrowedPackDocumentCursor::default();let mut actual=Vec::new();let mut output=[0;64];
 for _ in 0..10000000{let grant=exact(&cursor);let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(source_view,spec,&mut output,grant).unwrap());assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));assert!(step.progress.fits(grant));assert!(birth<=4096);actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}
 let complete=cursor.is_complete();let entries=cursor.chunk_entries.len();if complete{assert_eq!(entries,fixture["chunks"].as_u64().unwrap()as usize);assert_eq!(cursor.chunk_entries[0].hash,*blake3::hash(&source.data).as_bytes());}close(&mut cursor);assert!(complete);assert_eq!(actual.len(),expected.len());assert!(actual==expected,"first chunk document mismatch at {:?}",actual.iter().zip(&expected).position(|(actual,expected)|actual!=expected));assert_eq!(source.data.as_ptr(),original);eprintln!("[DEBUG] retained original{}octet chunk matches canonical container and third-party BLAKE3 with exact paid custody",source.data.len());assert_eq!(native.data.retained_bytes().backing_ptr(0),native_pointer);}
}

#[test]
fn borrowed_pack_document_chunk_frontiers_cancel_with_exact_original_retirement(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧱️chunks/🔣️.json")).unwrap();let pattern:Vec<u8>=fixture["pattern"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();let source=ChunkOriginal{data:pattern.repeat(fixture["repeat"].as_u64().unwrap()as usize)};let original=source.data.as_ptr();let spec=BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&CHUNK_FIELDS};
 let native=PagedChunkOriginal{data:semio_framework_value::paged::PagedBytes::try_from_slice(&source.data).unwrap()};let native_pointer=native.data.retained_bytes().backing_ptr(0);for source_view in [&source as &dyn semio_framework_dsl_record::native_encoding::FieldProjectionSource,&native]{
 for target in fixture["cancelFrontiers"].as_array().unwrap(){let target=target.as_str().unwrap();let frontier=match target{"ChunkBirth"=>Phase::ChunkBirth,"ChunkRead"=>Phase::ChunkRead,"ChunkHash"=>Phase::ChunkHash,"ChunkCatalog"=>Phase::ChunkCatalog,"ChunkPrefix"=>Phase::ChunkPrefix,"ChunkEmit"=>Phase::ChunkEmit,"ChunkChecksum"=>Phase::ChunkChecksum,"TablePrefix"=>Phase::TablePrefix,"TableBody"=>Phase::TableBody,"TableChecksum"=>Phase::TableChecksum,_=>panic!("unknown cancellation frontier")};let mut cursor=BorrowedPackDocumentCursor::default();let mut output=[0;1];let mut reached=false;
  for _ in 0..10000000{if cursor.phase==frontier{reached=true;break;}let grant=exact(&cursor);let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(source_view,spec,&mut output,grant).unwrap());assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));assert!(step.progress.fits(grant));if step.complete{break;}}
  let demand=cursor.retirement_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant}];if demand.copy_bytes!=0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.release_bytes!=0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}if demand.depth!=0{denied.push(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant});}
  for denied in denied{let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.close(denied).unwrap());assert_eq!(step.progress,Default::default());assert_eq!((birth,free),(0,0));}close(&mut cursor);assert!(reached,"missing original frontier{target}");assert_eq!(source.data.as_ptr(),original);
 }
 assert_eq!(native.data.retained_bytes().backing_ptr(0),native_pointer);}

}
