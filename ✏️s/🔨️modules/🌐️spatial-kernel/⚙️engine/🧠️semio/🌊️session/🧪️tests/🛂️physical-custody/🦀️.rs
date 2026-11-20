//! 🎟️ Checks physical session allocation custody through terminal retirement.

use super::*;
pub(super) fn observe<T>(work:impl FnOnce()->T)->(T,(usize,usize)) {let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(work);(value,(heap.requested_bytes,heap.released_bytes))}
#[test]
fn original_session_capture_keeps_same_allocation_until_full_typed_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    let row=&fixture["box"];let custody=&fixture["physicalCustody"];
    let identity=semio_os_flow::geometry::GeometryWorkIdentity{operation:61,generation:1};
    let mut caller=super::original_tessellation_authority_tests::original_geometry_caller(identity);
    let stock=super::original_tessellation_authority_tests::original_geometry_source_grant();
    let (mut capture,factory)=observe(|| {
        let source=Session::new();let handle=source.with_kernel(|kernel|kernel.box_prim_sync(row["width"].as_f64().unwrap(),row["depth"].as_f64().unwrap(),row["height"].as_f64().unwrap()).map_err(|error|map_kernel_error(&error))).unwrap();
        let probe=caller.turn(stock,|cx|{let mut result=None;cx.with_original_work_loan(0,cx.deadline_us(),|same|{result=Some(source.tessellate_step(identity,handle.as_str(),0.05,same));Ok(())})?;Ok(result.unwrap())}).unwrap().unwrap();
        assert!(matches!(probe.0,semio_os_flow::geometry::GeometryStep::Working {..}));
        let mut warmed=false;
        for _ in 0..custody["maximumWarmTurns"].as_u64().unwrap() {
            let(step,_)=caller.turn(stock,|cx|Ok(source.tessellate_step(identity,handle.as_str(),0.05,cx))).unwrap().unwrap();
            match step {semio_os_flow::geometry::GeometryStep::Working {..}=>{let owner=source.tessellation_jobs().try_lock().unwrap();let registry=owner.original().unwrap();let faces=registry.jobs.iter().find(|(key,_)|key.0==handle.0&&key.1==0.05f64.to_bits()).map_or(0,|(_,row)|row.job.progress().faces_done);if faces>=custody["warmFaces"].as_u64().unwrap()as usize {warmed=true;break}},other=>panic!("original partial tessellation did not remain owned: {other:?}")}
        }
        assert!(warmed,"original job must retain sampled edges and a partial mesh");
        let capture=source.capture();drop(handle);drop(source);capture
    });
    let retained=factory.0.checked_sub(factory.1).unwrap();let original=Arc::as_ptr(&capture.session.as_ref().unwrap().state);
    let (count,cancel)=observe(||capture.cancel_all_tessellations());assert_eq!(count,1);
    assert_eq!(cancel,(custody["cancelCapacityBytes"].as_u64().unwrap() as usize,custody["cancelReleaseBytes"].as_u64().unwrap() as usize));
    let handoff=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
    let (step,heap)=observe(||capture.close_step(handoff).unwrap());assert!(step.progress().fits(handoff));
    assert_eq!(heap,(custody["handoffCapacityBytes"].as_u64().unwrap() as usize,custody["handoffReleaseBytes"].as_u64().unwrap() as usize));
    assert_eq!(capture.allocation.as_ref().unwrap().as_ptr(),original);
    let (mut born,mut freed,mut refusals,mut turns,mut final_shell)=(0,0,0,0,0);
    while !capture.terminal_is_empty() {
        let copy=capture.next_close_copy_byte_demand().unwrap();let capacity=capture.next_close_capacity_byte_demand(copy).unwrap();let release=capture.next_close_release_byte_demand().unwrap();let depth=capture.next_close_depth_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        if release>custody["insufficientReleaseBytes"].as_u64().unwrap() as usize {
            for _ in 0..2 {let (step,heap)=observe(||capture.close_step(RetainedCloneGrant {maximum_release_bytes:8,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(capture.next_close_release_byte_demand().unwrap(),release);refusals+=1;}
        }
        let parent=capture.orphan.is_none() && capture.allocation.is_some();
        let (step,heap)=observe(||capture.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<1000000);
        if parent && capture.terminal_is_empty() {assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(heap.1,release);final_shell+=1;}
    }
    assert_eq!(retained+born,freed);assert_eq!(final_shell,1);assert!(refusals>0);
    caller.retire(stock);
}
