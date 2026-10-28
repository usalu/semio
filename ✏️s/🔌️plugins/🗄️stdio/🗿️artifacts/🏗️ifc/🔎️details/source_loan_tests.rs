//! 🧪️ Guided source projection admission retains actual Arc identity under denied currencies.
use super::*;
#[test]
fn guided_source_projection_denial_and_cancelled_retirement_preserve_original_authority(){
    use super::super::source_loan::SourceLoan;
    let snapshot=wall(9);let substitute=wall(9);let mut caller=OriginalWorkFixture::new();let mut other=OriginalWorkFixture::new();
    let mut loan=Some(SourceLoan::default());
    assert!(caller.turn(grant(),|cx|loan.as_mut().unwrap().advance(&substitute,snapshot.as_ref(),[0;32],cx)).is_err());
    assert_eq!(Arc::strong_count(&snapshot),1);
    for _ in 0..4{
        let demand=loan.as_ref().unwrap().demands().unwrap();
        for axis in 0..5{
            let mut denied=grant();match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=demand.copy_bytes.saturating_sub(1),2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,3=>denied.maximum_depth=0,_=>continue}
            let before=Arc::strong_count(&snapshot);
            let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|loan.as_mut().unwrap().advance(&snapshot,snapshot.as_ref(),[0;32],cx)));
            assert_eq!(result.unwrap(),false);assert_eq!(Arc::strong_count(&snapshot),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        }
        if caller.turn(grant(),|cx|loan.as_mut().unwrap().advance(&snapshot,snapshot.as_ref(),[0;32],cx)).unwrap(){break;}
    }
    assert!(loan.as_ref().unwrap().source().is_some());assert_eq!(Arc::strong_count(&snapshot),2);
    assert!(other.turn(grant(),|cx|loan.as_mut().unwrap().advance(&snapshot,snapshot.as_ref(),[0;32],cx)).is_err());
    assert!(caller.turn(grant(),|cx|loan.as_mut().unwrap().advance(&snapshot,snapshot.as_ref(),[1;32],cx)).is_err());
    caller.cancel().cancel_now();let mut close=RetainedCloneClose::default();
    for _ in 0..10000{
        if loan.is_none()&&close.is_empty(){break;}
        caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let demand=close.next_owner_demands::<SourceLoan>(loan.is_some(),&None,4096)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|if close.is_empty(){close.begin_granted(&mut loan,grant)?.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"source loan close missing"))}else{close.step_granted(grant)})?;Ok(())})).unwrap();
    }
    assert!(loan.is_none()&&close.is_empty());assert_eq!(Arc::strong_count(&snapshot),1);caller.finish();other.finish();
}
