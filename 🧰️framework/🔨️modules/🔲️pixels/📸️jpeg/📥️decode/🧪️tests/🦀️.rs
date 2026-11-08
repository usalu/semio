//! 🧪️ Shared JPEG vectors and independent decoder witnesses retain all original native owners.
use super::*;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use crate::retirement::tests::{drain,observed};
use semio_framework_io_base64::base64_standard_decode;
#[path="../../../../🌱️value/♻️retirement/📦️owner/🧪️work/effect.rs"]
mod effect;
type Turn=(Result<(JpegDecodeProgress,RetainedCloneProgress),JpegDecodeError>,(usize,usize));
/// 🪢️ One funded decoder turn inside one actual original effect, heap-observed around the decoder action only.
fn advance(decoder:&mut JpegDecodeJob,grant:RetainedCloneGrant)->Turn{effect::with_effect(grant,1<<40,&mut None,|effect|observed(||decoder.advance(effect)))}
fn take(decoder:&mut JpegDecodeJob,grant:RetainedCloneGrant)->(Result<Option<(RasterImage,RetainedCloneProgress)>,JpegDecodeError>,(usize,usize)){effect::with_effect(grant,1<<40,&mut None,|effect|observed(||decoder.take_result(effect)))}
fn fixture()->serde_json::Value{serde_json::from_str(crate::text("📸️jpeg/📥️decode/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn request(row:&serde_json::Value)->JpegDecodeInput{JpegDecodeInput{data:base64_standard_decode(row["base64"].as_str().unwrap()).unwrap(),max_pixels:16777216,max_bytes:67108864,max_segments:65536,max_working_bytes:536870912}}
fn job(input:JpegDecodeInput)->JpegDecodeJob{JpegDecodeJob::new(input).unwrap_or_else(|(error,_)|panic!("JPEG fixture refused: {error}"))}
fn grant(job:&JpegDecodeJob)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:job.next_copy_byte_demand(),maximum_capacity_bytes:job.next_capacity_byte_demand(),maximum_release_bytes:0,maximum_depth:1}}
#[test]
fn jpeg_decode_neutral_samples_match_independent_decoder_and_exact_allocator(){
 let fixture=fixture();let mut laws=0;
 for row in fixture["cases"].as_array().unwrap(){let input=request(row);let source_bytes=input.data.capacity();let pointer=input.data.as_ptr();let reference=image::load_from_memory_with_format(&input.data,image::ImageFormat::Jpeg).unwrap().into_rgba8();let mut decoder=job(input);let mut born=0;
  for _ in 0..1000000{let funded=grant(&decoder);let before=decoder.progress();for axis in 0..4{let mut denied=funded;let demand=match axis{0=>{denied.maximum_items=0;1},1=>{denied.maximum_copy_bytes=funded.maximum_copy_bytes.saturating_sub(1);funded.maximum_copy_bytes},2=>{denied.maximum_capacity_bytes=funded.maximum_capacity_bytes.saturating_sub(1);funded.maximum_capacity_bytes},_=>{denied.maximum_depth=0;1}};if demand==0{continue;}let(result,physical)=advance(&mut decoder,denied);let result=result.unwrap();assert_eq!(physical,(0,0));assert_eq!(result,(before,RetainedCloneProgress::default()));assert_eq!(decoder.source().as_ptr(),pointer);}
   let(result,physical)=advance(&mut decoder,funded);let result=result.unwrap();assert!(result.1.fits(funded));assert_eq!(physical,(result.1.retained_capacity_bytes,0));born+=physical.0;if result.0.done{break;}}
  let image=decoder.result().unwrap();assert_eq!((image.width,image.height),(row["width"].as_u64().unwrap()as u32,row["height"].as_u64().unwrap()as u32));let tolerance=row["oracleTolerance"].as_u64().unwrap()as u8;for (actual,expected)in image.pixels.iter().zip(reference.as_raw()){assert!(actual.abs_diff(*expected)<=tolerance,"{} JPEG oracle difference {}",row["name"],actual.abs_diff(*expected));}
  let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<RasterImage>(),maximum_depth:1,..Default::default()};let (denied,physical)=take(&mut decoder,RetainedCloneGrant{maximum_copy_bytes:size_of::<RasterImage>()-1,..funded});let denied=denied.unwrap();assert!(denied.is_none());assert_eq!(physical,(0,0));let output=take(&mut decoder,funded).0.unwrap().unwrap().0;let(a,r)=drain(decoder);let(a2,r2)=drain(output);assert_eq!(source_bytes+born+a+a2,r+r2);laws+=1;
 }
}
#[test]
fn jpeg_decode_cancelled_and_refused_originals_close_under_actual_grants(){
 let fixture=fixture();let row=&fixture["cases"][0];let mut laws=0;
 for stop in fixture["interruptions"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize){let input=request(row);let original=input.data.capacity();let pointer=input.data.as_ptr();let mut decoder=job(input);let mut born=0;for _ in 0..stop{let funded=grant(&decoder);let(result,physical)=advance(&mut decoder,funded);result.unwrap();born+=physical.0;}let(_,physical)=observed(||decoder.cancel());assert_eq!(physical,(0,0));assert_eq!(decoder.source().as_ptr(),pointer);assert!(matches!(decoder.result(),Err(JpegDecodeError::Cancelled)));let(a,r)=drain(decoder);assert_eq!(original+born+a,r);laws+=1;}
 let mut input=request(row);input.max_pixels=0;let pointer=input.data.as_ptr();let bytes=input.data.capacity();let(result,physical)=observed(||JpegDecodeJob::new(input));assert_eq!(physical,(0,0));let (error,original)=match result{Err(value)=>value,Ok(_)=>panic!("invalid original admitted")};assert_eq!(error,invalid("Invalid JPEG input contract"));assert_eq!(original.data.as_ptr(),pointer);let(a,r)=drain(original);assert_eq!(bytes+a,r);
}
#[test]
fn jpeg_decode_records_the_real_work_of_every_action_and_stops_at_its_authority(){
 let fixture=fixture();
 for row in fixture["cases"].as_array().unwrap(){
  let input=request(row);let bytes=input.data.len()as u64;let mut decoder=job(input);let mut recorded=0u128;let mut actions=0usize;
  for _ in 0..1000000{let funded=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:decoder.next_copy_byte_demand().max(32)*4096,maximum_capacity_bytes:decoder.next_capacity_byte_demand(),maximum_release_bytes:0,maximum_depth:1};
   let(result,performed)=effect::with_effect(funded,1<<40,&mut None,|effect|{let result=decoder.advance(effect);(result,effect.performed_work())});let(progress,receipt)=result.unwrap();recorded+=performed;actions+=receipt.copied_items;assert_eq!(recorded,u128::from(progress.work));if progress.done{break;}}
  let pixels=u64::from(row["width"].as_u64().unwrap())*row["height"].as_u64().unwrap();
  assert!(recorded>=u128::from(bytes+pixels),"{} recorded {recorded} for {bytes} bytes and {pixels} pixels",row["name"]);assert!(recorded>actions as u128,"{} work must exceed one unit per action",row["name"]);
  let(idle,_)=effect::with_effect(RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:0,maximum_depth:1},0,&mut None,|effect|{let mut fresh=job(request(row));(fresh.advance(effect).map(|(progress,receipt)|(progress.work,receipt.copied_items)),())});assert_eq!(idle,Ok((0,0)),"no action runs without original work authority");
 }
}
