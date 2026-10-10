//! 🧪️ Original borrowed source witnesses match independent byte oracles and physical receipts.
use super::*;
use base64::Engine;
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::RetainedCloneStep};
use std::{alloc::{GlobalAlloc,Layout,System},cell::Cell};
thread_local!{static TRACK:Cell<Option<(usize,usize)>>=const{Cell::new(None)};}
struct PhysicalAllocator;
#[global_allocator]
static ALLOCATOR:PhysicalAllocator=PhysicalAllocator;
unsafe impl GlobalAlloc for PhysicalAllocator{
 unsafe fn alloc(&self,layout:Layout)->*mut u8{let pointer=unsafe{System.alloc(layout)};if !pointer.is_null(){let _=TRACK.try_with(|track|if let Some((allocated,released))=track.get(){track.set(Some((allocated+layout.size(),released)));});}pointer}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout){let _=TRACK.try_with(|track|if let Some((allocated,released))=track.get(){track.set(Some((allocated,released+layout.size())));});unsafe{System.dealloc(pointer,layout);}}
}
fn observed<T>(step:impl FnOnce()->T)->(T,(usize,usize)){TRACK.with(|track|{assert!(track.get().is_none());track.set(Some((0,0)));});let output=step();let bytes=TRACK.with(|track|track.replace(None).unwrap());(output,bytes)}
fn drain<T:RetireOwned>(value:T)->(usize,usize){
 let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("unsupported original: {error}"));let(mut allocated,mut released)=(0,0);
 for _ in 0..2000000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};let(step,physical)=observed(||owner.step(grant).unwrap());let(RetainedCloneStep::Progress(receipt)|RetainedCloneStep::Complete(receipt))=step;assert!(receipt.fits(grant));assert_eq!(physical,(receipt.retained_capacity_bytes,receipt.released_bytes));allocated+=physical.0;released+=physical.1;}
 assert!(owner.terminal_is_empty());assert_eq!(observed(||drop(owner)).1,(0,0));(allocated,released)
}
fn parsed_input(value:&serde_json::Value)->Option<BinarySourceInput<'_>>{Some(BinarySourceInput{mime:value["mime"].as_str()?,data:value["data"].as_str()?,min_bytes:value["minBytes"].as_u64()?as usize,max_bytes:value["maxBytes"].as_u64()?as usize,max_source_bytes:value["maxSourceBytes"].as_u64()?as usize,max_work:value["maxWork"].as_u64()?})}
fn input(value:&serde_json::Value)->BinarySourceInput<'_>{parsed_input(value).unwrap()}
fn funded(job:&BinarySourceJob)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:job.next_copy_byte_demand(),maximum_capacity_bytes:job.next_capacity_byte_demand(),maximum_release_bytes:0,maximum_depth:1}}
fn finish(job:&mut BinarySourceJob,source:&str,items:usize)->BinarySourceProgress{for _ in 0..2000000{let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:items*32,maximum_capacity_bytes:67108864,maximum_release_bytes:0,maximum_depth:1};let(progress,receipt)=job.advance(source,grant).unwrap();assert!(receipt.fits(grant));if progress.done{return progress;}}panic!("source did not finish")}
fn output(job:&mut BinarySourceJob)->Vec<u8>{job.take_result(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<u8>>(),maximum_depth:1,..Default::default()}).unwrap().unwrap().0}
fn oracle(data:&str)->Vec<u8>{if data.get(..5).is_some_and(|value|value.eq_ignore_ascii_case("data:")){let(header,body)=data.split_once(',').unwrap();let bytes=percent_encoding::percent_decode_str(body).collect::<Vec<_>>();if header.to_ascii_lowercase().ends_with(";base64"){base64::engine::general_purpose::STANDARD.decode(bytes).unwrap()}else{bytes}}else{base64::engine::general_purpose::STANDARD.decode(data).unwrap()}}
#[test]
fn binary_sources_match_neutral_font_image_bytes_and_independent_oracles(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){let input=input(&row["input"]);let expected:Vec<u8>=serde_json::from_value(row["expected"].clone()).unwrap();assert_eq!(oracle(input.data),expected);for items in [1,7,4096]{let mut job=BinarySourceJob::new(input).unwrap();finish(&mut job,input.data,items);assert_eq!(job.result().unwrap(),expected);let output=output(&mut job);drain(job);drain(output);}}
 eprintln!("[DEBUG] Native borrowed binary sources matched shared bytes and independent base64/percent oracles");
}
#[test]
fn binary_sources_refuse_malformed_contracts_and_private_failures(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️invalid/🔣️.json")).unwrap();
 for row in rows.as_array().unwrap(){let Some(input)=parsed_input(&row["input"])else{continue;};if let Ok(mut job)=BinarySourceJob::new(input){let mut error=None;for _ in 0..2000000{let grant=funded(&job);match job.advance(input.data,grant){Ok((p,_))=>assert!(!p.done,"{}",row["name"]),Err(value)=>{error=Some(value);break;}}}let error=error.unwrap();assert_eq!(job.advance(input.data,funded(&job)).unwrap_err(),error);assert!(job.result().is_err());drain(job);}}
 eprintln!("[DEBUG] Native malformed sources kept private original source and refused output");
}
#[test]
fn original_binary_source_funding_preserves_axes_and_allocator_receipts(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🎟️funding.json")).unwrap();let input=input(&fixture["input"]);let original_pointer=input.data.as_ptr();let original_bytes=input.data.as_bytes();
 for row in fixture["cases"].as_array().unwrap(){let mut job=BinarySourceJob::new(input).unwrap();let stage=row["stage"].as_str().unwrap();let mut born=0;for _ in 0..256{if stage=="header"||stage=="allocation"&&job.next_capacity_byte_demand()>0||stage=="handoff"&&job.progress().done{break;}let grant=funded(&job);let((_,receipt),physical)=observed(||job.advance(input.data,grant).unwrap());assert_eq!(physical,(receipt.retained_capacity_bytes,0));born+=physical.0;}
  let value=&row["grant"];let grant=RetainedCloneGrant{maximum_items:value["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:value["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:value["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:value["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:value["maximumDepth"].as_u64().unwrap()as usize};let before=job.progress();let expected=row["accepted"].as_bool().unwrap();
  if stage=="handoff"{let(result,physical)=observed(||job.take_result(grant).unwrap());assert_eq!(physical,(0,0));assert_eq!(result.is_some(),expected);if let Some((output,receipt))=result{assert!(receipt.fits(grant));assert_eq!(output,oracle(input.data));let(a,r)=drain(output);born+=a;let(a2,r2)=drain(job);assert_eq!(born+a2,r+r2);}else{assert_eq!(job.progress(),before);let(a,r)=drain(job);assert_eq!(born+a,r);}}
  else{let((progress,receipt),physical)=observed(||job.advance(input.data,grant).unwrap());assert!(receipt.fits(grant));assert_eq!(physical,(receipt.retained_capacity_bytes,0));born+=physical.0;assert_eq!(progress!=before,expected);let(a,r)=drain(job);assert_eq!(born+a,r);}
  assert_eq!(input.data.as_ptr(),original_pointer);assert_eq!(input.data.as_bytes(),original_bytes);
 }
 for phase in fixture["cancelPhases"].as_array().unwrap(){let mut job=BinarySourceJob::new(input).unwrap();let mut born=0;while job.progress().phase!=phase.as_str().unwrap(){let grant=funded(&job);let((_,receipt),physical)=observed(||job.advance(input.data,grant).unwrap());assert_eq!(physical,(receipt.retained_capacity_bytes,0));born+=physical.0;}assert_eq!(observed(||job.cancel()).1,(0,0));assert_eq!(job.result().unwrap_err(),BinarySourceError::Cancelled);assert_eq!(input.data.as_ptr(),original_pointer);let(a,r)=drain(job);assert_eq!(born+a,r);}
 let equal_content=input.data.to_owned();let mut job=BinarySourceJob::new(input).unwrap();assert_eq!(observed(||job.advance(&equal_content,funded(&job))).1,(0,0));assert_eq!(job.progress().work,0);drain(job);
 eprintln!("[DEBUG] Original binary source9neutral grants,4cancel phases,changed-source refusal,independent oracle and exact allocator conservation");
}
#[test]
fn binary_sources_load_actual_bundled_font_resources(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔤️fonts/🔣️.json")).unwrap();let fonts:&[&[u8]]=&[include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🪉️supplement/⚖️medium/🔤️outline.ttf"),include_bytes!("../../../../📚️compiler/🌍️world/🔤️fonts/⌨️LibertinusMono-Regular.otf")];
 for(row,bytes)in rows.as_array().unwrap().iter().zip(fonts){assert_eq!(bytes.len(),row["bytes"].as_u64().unwrap()as usize);let mime=row["mime"].as_str().unwrap();let data=format!("data:{mime};name=Document%20Font;base64,{}",base64::engine::general_purpose::STANDARD.encode(bytes));let input=BinarySourceInput{mime,data:&data,min_bytes:bytes.len(),max_bytes:bytes.len(),max_source_bytes:data.len(),max_work:data.len()as u64*2+2};let mut job=BinarySourceJob::new(input).unwrap();finish(&mut job,&data,4096);assert_eq!(job.result().unwrap(),*bytes);assert_eq!(oracle(&data),*bytes);let output=output(&mut job);drain(job);drain(output);}
 eprintln!("[DEBUG] Original bundled native fonts retained exact bytes through granted source decoding");
}