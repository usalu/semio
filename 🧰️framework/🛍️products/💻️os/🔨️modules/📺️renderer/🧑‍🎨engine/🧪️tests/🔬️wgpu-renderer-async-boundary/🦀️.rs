use super::*;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_job::{OriginalJobReturn, OriginalJobReturnOrigin, drive_step};

#[cfg(not(target_arch="wasm32"))]
#[path="../../🎯️targets/🧊️wgpu/🧊️renderer/🚪️native-io/🎟️invocation/🧪️tests/🦀️.rs"]
mod original_native_io_invocation_tests;

#[cfg(not(target_arch="wasm32"))]
#[test]
fn native_io_original_page_enters_world_without_copy_and_returns_to_its_live_source(){
    use semio_framework_job::{InteractiveJob, JobOutcomeKind, JobOutcomeView, OriginalJobPageReturnRegistry};
    static RETURNS:OriginalJobPageReturnRegistry<1>=OriginalJobPageReturnRegistry::new();
    let path=std::env::temp_dir().join(format!("semio-renderer-page-return-{}",std::process::id()));std::fs::write(&path,b"original World page").unwrap();let mut job=semio_os_services_native_io::NativeIoJob::new(semio_os_services_native_io::NativeIoRequest::ReadPage{path:path.clone(),offset:0,max_bytes:semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES});let mut caller=OriginalRendererFixture::new();let source=caller.turn(|cx|Ok(cx.original_source_parent()?.source_identity()?)).unwrap();let mut pointer=None;
    let mut origin = OriginalJobReturnOrigin::empty();
    caller.admit_job_origin(&mut origin);
    for _ in 0..1024 {
        caller.turn(|cx| {
            let mut verdict = None;
            match drive_step(&mut job, cx, origin.receiver()?, "renderer.native-io-page", InteractiveStage::InteractiveStep, &mut verdict)? {
                OriginalJobReturn::Preparing | OriginalJobReturn::Pending => {}
                OriginalJobReturn::Outcome(outcome) => {
                    let mut outcome = Some(outcome.into_descriptor());
                    assert!(origin.receive_outcome(cx, &mut outcome)?);
                    assert!(outcome.is_none());
                }
                OriginalJobReturn::Continuation(continuation) => {
                    assert!(origin.receive_continuation(cx, &mut Some(continuation))?);
                    panic!("original native page read must return a normal outcome");
                }
            }
            Ok(())
        }).unwrap();
        let Some(descriptor) = origin.outcome() else { continue };
        if descriptor.kind() == JobOutcomeKind::Complete {
            let JobOutcomeView::Complete { output: Some(bytes), .. } = job.borrow_outcome(descriptor).unwrap() else {
                panic!("original native output");
            };
            pointer = Some(bytes.single_page().unwrap().as_ptr());
        }
        for _ in 0..16 {
            if caller.turn(|cx| origin.acknowledge_outcome(cx)).unwrap() { break; }
        }
        assert!(origin.outcome().is_none());
        if pointer.is_some() { break; }
    }
    caller.close_job_origin(&mut origin);
    let pointer = pointer.expect("original native page read completed");
    let mut reserved = None;
    for _ in 0..16 {
        if caller.turn(|cx| RETURNS.reserve(&mut reserved, cx)).unwrap() { break; }
    }
    let mut reservation = reserved.expect("the original page reservation completes");
    let mut supplied = None;
    for _ in 0..16 {
        let admitted = caller.turn(|cx| {
            let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.supply_result_last_page(&mut reservation, &mut supplied, cx));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            answer
        }).unwrap();
        if admitted { break; }
    }
    let page = supplied.expect("the original page reaches its supplier return");
    assert_eq!(page.source_identity(), source);
    assert_eq!(page.bytes().as_ptr(), pointer);
    let (world, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| WorldAssetResponsePage::try_from_original(page).unwrap());
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    assert_eq!(world.bytes().as_ptr(), pointer);
    assert_eq!(world.bytes(), b"original World page");
    job.begin_close();for turn in 0..64{if job.terminal_is_empty(){break}caller.turn(|cx|job.close_step(cx)).unwrap();assert!(turn<63);}assert!(job.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(RETURNS.retains_source(source));let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(world));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut released=0;
    for _ in 0..16 {
        if !RETURNS.retains_source(source) { break; }
        released += caller.turn(|cx| {
            let before = cx.retained_progress();
            let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| RETURNS.close_returned_step(cx));
            let progress = answer?.progress();
            let after = cx.retained_progress();
            assert_eq!(after.released_bytes - before.released_bytes, progress.released_bytes);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            Ok(heap.released_bytes)
        }).unwrap();
    }
    assert_eq!(released, semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);
    assert!(!RETURNS.retains_source(source));
    std::fs::remove_file(path).unwrap();
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
    fn admit_job_origin(&mut self, origin: &mut OriginalJobReturnOrigin) {
        for _ in 0..64 {
            if self.turn(|cx| origin.admit(cx)).unwrap() {
                assert!(origin.body_ready());
                return;
            }
        }
        panic!("renderer job return origin was not admitted");
    }
    fn close_job_origin(&mut self, origin: &mut OriginalJobReturnOrigin) {
        for _ in 0..64 {
            if origin.terminal_is_empty() { break; }
            self.turn(|cx| origin.close_step(cx)).unwrap();
        }
        assert!(origin.terminal_is_empty());
    }
}
#[cfg(not(target_arch = "wasm32"))]
impl Drop for OriginalRendererFixture {
    fn drop(&mut self) {
        if std::thread::panicking() { return; }
        let caller=self.caller.as_mut().unwrap();
        for _ in 0..65536 { if self.decoder.terminal_is_empty(){break} caller.turn(renderer_law_stock(),|cx| { let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;self.decoder.close_on_source(cx)?;Ok(()) }).unwrap(); }
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
