use super::*;

#[cfg(not(target_arch="wasm32"))]
#[path="../../🎯️targets/🧊️wgpu/🧊️renderer/🚪️native-io/🎟️invocation/🧪️tests/🦀️.rs"]
mod original_native_io_invocation_tests;

#[cfg(not(target_arch="wasm32"))]
#[test]
fn native_io_original_page_enters_world_without_copy_and_returns_to_its_live_source(){
    use semio_framework_job::{InteractiveJob,JobOutcomeBorrow,JobOutcomeKind,JobOutcomeView,OriginalJobPageReturnRegistry,close_original_retained_step};
    use semio_framework_value::RetirementDemand;
    static RETURNS:OriginalJobPageReturnRegistry<1>=OriginalJobPageReturnRegistry::new();
    let path=std::env::temp_dir().join(format!("semio-renderer-page-return-{}",std::process::id()));std::fs::write(&path,b"original World page").unwrap();let mut job=semio_framework_os_services_native_io::NativeIoJob::new(semio_framework_os_services_native_io::NativeIoRequest::ReadPage{path:path.clone(),offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES});let mut caller=OriginalRendererFixture::new();let source=caller.turn(|cx|Ok(cx.original_source_parent()?.source_identity()?)).unwrap();let mut pointer=None;
    for turn in 0..1024{
        let outcome=caller.turn(|cx|job.step(cx).map(|outcome|outcome.map(JobOutcomeBorrow::into_descriptor))).unwrap();
        if let Some(mut descriptor)=outcome{let complete=descriptor.kind()==JobOutcomeKind::Complete;if complete{let JobOutcomeView::Complete{output:Some(bytes),..}=job.borrow_outcome(&descriptor).unwrap()else{panic!("original native output")};pointer=Some(bytes.single_page().unwrap().as_ptr());}caller.turn(|cx|{close_original_retained_step(cx,RetirementDemand{depth:1,..Default::default()},|grant|Ok(descriptor.acknowledge(grant)))?;Ok(())}).unwrap();assert!(descriptor.is_acknowledged());drop(descriptor);if complete{break}}assert!(turn<1023);
    }
    let pointer=pointer.unwrap();let mut reservation=caller.turn(|cx|Ok(RETURNS.reserve(cx)?.unwrap())).unwrap();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(|cx|job.supply_result_last_page(&mut reservation,cx)));let page=answer.unwrap().unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(page.source_identity(),source);assert_eq!(page.bytes().as_ptr(),pointer);let(world,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||WorldAssetResponsePage::try_from_original(page).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(world.bytes().as_ptr(),pointer);assert_eq!(world.bytes(),b"original World page");
    job.begin_close();for turn in 0..64{if job.terminal_is_empty(){break}caller.turn(|cx|job.close_step(cx)).unwrap();assert!(turn<63);}assert!(job.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(RETURNS.retains_source(source));let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(world));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut released=0;
    for turn in 0..16{if !RETURNS.retains_source(source){break}let before=caller.receipt();let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(|cx|RETURNS.close_returned_step(cx)));let progress=answer.unwrap().progress();let after=caller.receipt();assert_eq!(after.released_bytes-before.released_bytes,progress.released_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));released+=heap.released_bytes;assert!(turn<15);}assert_eq!(released,semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);assert!(!RETURNS.retains_source(source));std::fs::remove_file(path).unwrap();println!("[DEBUG] native file/World actual original page pointer preserved with birth0/copy0 at handoff, consumer return frees0, same live caller pays exact16KiB and source loan before terminal root close");
}

#[cfg(not(target_arch = "wasm32"))]
fn renderer_law_stock() -> semio_framework_job::RetainedCloneGrant { semio_framework_job::RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4096 } }
#[cfg(not(target_arch = "wasm32"))]
struct OriginalRendererFixture { caller: Option<semio_framework_job::OriginalJobFixtureCaller>, decoder: semio_framework_job::OriginalJobDecodeOwner }
#[cfg(not(target_arch = "wasm32"))]
impl OriginalRendererFixture {
    fn new() -> Self { Self::with_identity(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1)) }
    fn with_identity(operation:semio_framework_job::OperationId,generation:semio_framework_job::Generation) -> Self { Self { caller: Some(semio_framework_job::OriginalJobFixtureCaller::admit(operation,generation,renderer_law_stock(),||Some(0),u64::MAX,1 << 48)), decoder:semio_framework_job::OriginalJobDecodeOwner::empty() } }
    fn turn<R>(&mut self, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> Result<R,semio_framework_job::OriginalTurnFailure>) -> Result<R,semio_framework_job::OriginalTurnFailure> { self.limited(renderer_law_stock(),run) }
    fn limited<R>(&mut self, grant:semio_framework_job::RetainedCloneGrant, run:impl FnOnce(&mut semio_framework_job::StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->Result<R,semio_framework_job::OriginalTurnFailure>{ self.caller.as_mut().unwrap().turn_with_decoder(grant,&mut self.decoder,run) }
    fn cancel(&self) { self.caller.as_ref().unwrap().cancel().cancel_now(); }
    fn receipt(&self)->semio_framework_job::RetainedCloneProgress { self.caller.as_ref().unwrap().cumulative_receipt() }
}
#[cfg(not(target_arch = "wasm32"))]
impl Drop for OriginalRendererFixture {
    fn drop(&mut self) {
        let caller=self.caller.as_mut().unwrap();
        for _ in 0..65536 { if self.decoder.terminal_is_empty(){break} caller.turn(renderer_law_stock(),|cx| { let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.decoder.close_step(grant))?;Ok(()) }).unwrap(); }
        assert!(self.decoder.terminal_is_empty());
        self.caller.take().unwrap().retire(renderer_law_stock());
    }
}


include!("🧭️source-readers-and-frame-input-laws/🦀️.rs");
include!("🖼️raster-contract-upload-cache-and-asset-probe/🦀️.rs");
include!("🧱️glb-materials-and-independent-decoder-jobs/🦀️.rs");
include!("👷️decoder-worker-session-recovery-and-asset-transfer/🦀️.rs");
include!("🗜️reference-decode-staging-and-runtime-mailbox/🦀️.rs");
include!("🚦️dispatch-cursors-frame-deferred-and-maintenance/🦀️.rs");
include!("🛰️text-stream-maintenance-queue-and-presenter-retirement/🦀️.rs");
include!("🧊️glb-world-mesh-swapchain-and-original-decoder-close/🦀️.rs");
