//! 🧪️ The defining inline Node retains real source custody, semantic descriptors and actual returned page backing.
use super::{OriginalRendererFixture,renderer_law_stock,WorldAssetResponsePage};
use semio_framework_os_renderer_wgpu_host_plumbing::native_io::RendererOriginalIoNode;
use semio_framework_job::{OriginalJobPageReturnRegistry,InteractiveJobCloseStep};
use semio_framework_os_services_native_io::{NativeIoRequest, NativeIoResultStatus};

static RETURNS:OriginalJobPageReturnRegistry<3>=OriginalJobPageReturnRegistry::new();
static STREAM_RETURNS:OriginalJobPageReturnRegistry<1>=OriginalJobPageReturnRegistry::new();
static CANCEL_RETURNS:OriginalJobPageReturnRegistry<1>=OriginalJobPageReturnRegistry::new();

#[test]
fn renderer_original_io_cancelled_partial_admission_returns_loan_without_taking_request(){
 let mut caller=OriginalRendererFixture::new();let path=std::path::PathBuf::from("preborn-native-request");let pointer=path.as_os_str().as_encoded_bytes().as_ptr();let mut request=Some(NativeIoRequest::ReadBytes(path));let mut output=None;
 assert!(!caller.limited(semio_framework_job::RetainedCloneGrant{maximum_items:1,..renderer_law_stock()},|cx|RendererOriginalIoNode::admit(&mut request,&mut output,&CANCEL_RETURNS,cx)).unwrap());let node=output.as_mut().unwrap();assert!(node.producer_is_empty());assert!(request.is_some());caller.cancel();node.begin_close();
 for turn in 0..16{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(|cx|node.close_step(cx)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if matches!(answer.unwrap(),InteractiveJobCloseStep::Complete{..}){break}assert!(turn<15)}assert!(node.terminal_is_empty());let NativeIoRequest::ReadBytes(path)=request.as_ref().unwrap()else{panic!("original preborn request")};assert_eq!(path.as_os_str().as_encoded_bytes().as_ptr(),pointer);assert!(caller.turn(|cx|Ok(cx.original_source_parent()?.original_loans_are_returned())).unwrap());
}

#[test]
fn renderer_original_io_invocation_keeps_source_until_real_world_pages_return(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
 let path=std::env::temp_dir().join(format!("semio-original-node-{}",std::process::id()));
 let contents=vec![37u8;2*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES+7];std::fs::write(&path,&contents).unwrap();
 let request_path=path.clone();let request_pointer=request_path.as_os_str().as_encoded_bytes().as_ptr();let mut request=Some(NativeIoRequest::ReadBytes(request_path));let mut node=None;
 let mut caller=OriginalRendererFixture::new();let source=caller.turn(|cx|Ok(cx.original_source_parent()?.source_identity()?)).unwrap();
 for denied in [semio_framework_job::RetainedCloneGrant{maximum_items:0,..renderer_law_stock()},semio_framework_job::RetainedCloneGrant{maximum_copy_bytes:0,..renderer_law_stock()},semio_framework_job::RetainedCloneGrant{maximum_depth:0,..renderer_law_stock()}]{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.limited(denied,|cx|RendererOriginalIoNode::admit(&mut request,&mut node,&RETURNS,cx)));assert!(!answer.unwrap());assert!(node.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let NativeIoRequest::ReadBytes(path)=request.as_ref().unwrap()else{panic!("same request")};assert_eq!(path.as_os_str().as_encoded_bytes().as_ptr(),request_pointer);}
 for turn in 0..4{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.limited(semio_framework_job::RetainedCloneGrant{maximum_items:1,..renderer_law_stock()},|cx|RendererOriginalIoNode::admit(&mut request,&mut node,&RETURNS,cx)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if answer.unwrap(){break}assert!(request.is_some());assert!(turn<3)}assert!(request.is_none());let node=node.as_mut().unwrap();assert_eq!(node.request_backing_identity(),Some(request_pointer));
 let identity=caller.turn(|cx|Ok((cx.operation(),cx.generation()))).unwrap();let mut foreign=OriginalRendererFixture::with_identity(identity.0,identity.1);
 let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(|cx|node.advance(cx)));assert!(answer.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(node.request_backing_identity(),Some(request_pointer));drop(foreign);
 let mut original_pointers=[std::ptr::null();3];
 for turn in 0..4096{let complete=caller.turn(|cx|node.advance(cx)).unwrap();if let Some(semio_framework_job::JobOutcomeView::Complete{output:Some(bytes),..})=node.borrow_outcome().unwrap(){original_pointers=std::array::from_fn(|index|bytes.page(index).unwrap().as_ptr());}if complete{break}assert!(turn<4095)}
 assert_eq!(node.result_status(),Some(NativeIoResultStatus::Bytes{length:contents.len(),pages:3}));assert!(node.outcome_is_acknowledged());
 let mut world_pages:[Option<WorldAssetResponsePage>;3]=std::array::from_fn(|_|None);
 for ordinal in law["pages"].as_array().unwrap(){let ordinal=ordinal.as_u64().unwrap()as usize;let mut output=None;for turn in 0..16{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(|cx|node.supply_next_page(&mut output,cx)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if answer.unwrap(){break}assert!(turn<15)}let page=output.unwrap();assert_eq!(page.original_page_index(),ordinal);assert_eq!(page.source_identity(),source);assert_eq!(page.bytes().as_ptr(),original_pointers[ordinal]);let pointer=page.bytes().as_ptr();let world=WorldAssetResponsePage::try_from_original(page).unwrap();assert_eq!(world.bytes().as_ptr(),pointer);assert_eq!(world.bytes(),&contents[ordinal*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES..((ordinal+1)*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES).min(contents.len())]);world_pages[ordinal]=Some(world);}
 node.begin_close();for _ in 0..128{caller.turn(|cx|node.close_step(cx)).unwrap();if node.producer_is_empty(){break}}assert!(node.producer_is_empty());assert!(!node.terminal_is_empty());assert!(RETURNS.retains_source(source));assert!(!caller.turn(|cx|Ok(cx.original_source_parent()?.original_loans_are_returned())).unwrap());
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(world_pages));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.cancel();
 for denied in [semio_framework_job::RetainedCloneGrant{maximum_items:0,..renderer_law_stock()},semio_framework_job::RetainedCloneGrant{maximum_copy_bytes:0,..renderer_law_stock()},semio_framework_job::RetainedCloneGrant{maximum_release_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES-1,..renderer_law_stock()},semio_framework_job::RetainedCloneGrant{maximum_depth:0,..renderer_law_stock()}]{let before=caller.receipt();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.limited(denied,|cx|node.close_step(cx)));assert_eq!(answer.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),before);assert!(RETURNS.retains_source(source));}
 let mut released=0;
 for turn in 0..128{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(|cx|node.close_step(cx)));let step=answer.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));released+=heap.released_bytes;if node.terminal_is_empty(){assert!(matches!(step,InteractiveJobCloseStep::Complete{..}));break}assert!(!matches!(step,InteractiveJobCloseStep::Complete{..}));assert!(turn<127)}
 assert_eq!(released,3*semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);assert!(node.terminal_is_empty());assert!(caller.turn(|cx|Ok(cx.original_source_parent()?.original_loans_are_returned())).unwrap());std::fs::remove_file(path).unwrap();
}

#[test]
fn renderer_original_io_stream_reuses_one_source_for_a_file_larger_than_live_payload_stock(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();let length=law["streamedFileBytes"].as_u64().unwrap();assert!(length>law["liveOperationBytes"].as_u64().unwrap());
 let path=std::env::temp_dir().join(format!("semio-original-stream-{}",std::process::id()));let file=std::fs::File::create(&path).unwrap();file.set_len(length).unwrap();drop(file);
 let input=path.clone();let pointer=input.as_os_str().as_encoded_bytes().as_ptr();let mut request=Some(NativeIoRequest::ReadPage{path:input,offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES});let mut node=None;let mut caller=OriginalRendererFixture::new();let source=caller.turn(|cx|Ok(cx.original_source_parent()?.source_identity()?)).unwrap();
 for turn in 0..4{if caller.turn(|cx|RendererOriginalIoNode::admit(&mut request,&mut node,&STREAM_RETURNS,cx)).unwrap(){break}assert!(turn<3)}let node=node.as_mut().unwrap();
 for(index,offset)in [0,length-semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES as u64].into_iter().enumerate(){
  if index!=0{for turn in 0..64{if caller.turn(|cx|node.restart_read_page(offset,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,cx)).unwrap(){break}assert!(turn<63)}}
  assert_eq!(node.request_backing_identity(),Some(pointer));for turn in 0..4096{if caller.turn(|cx|node.advance(cx)).unwrap(){break}assert!(turn<4095)}
  assert_eq!(node.result_status(),Some(NativeIoResultStatus::Page{length:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES,pages:1,eof:index==1}));let mut output=None;
  for turn in 0..16{if caller.turn(|cx|node.supply_next_page(&mut output,cx)).unwrap(){break}assert!(turn<15)}let page=output.unwrap();assert_eq!(page.source_identity(),source);assert!(page.bytes().iter().all(|byte|*byte==0));drop(page);
 }
 node.begin_close();for turn in 0..128{caller.turn(|cx|node.close_step(cx)).unwrap();if node.terminal_is_empty(){break}assert!(turn<127)}assert_eq!(node.request_backing_identity(),None);assert!(caller.turn(|cx|Ok(cx.original_source_parent()?.original_loans_are_returned())).unwrap());std::fs::remove_file(path).unwrap();
}
