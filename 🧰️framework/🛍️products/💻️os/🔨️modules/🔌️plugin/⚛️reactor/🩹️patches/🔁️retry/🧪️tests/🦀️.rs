use semio_framework_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey;
use crate::{PatchTracker, has_work};
use semio_framework_ui_contract as ui_contract;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[doc(hidden)]
use semio_framework_plugin_testing_caller::OriginalFixtureCaller;

#[cfg_attr(test,test)]
pub fn pending_mounted_render_survives_denied_original_retry_without_heap_or_lost_schedule(){
    let mut caller=OriginalFixtureCaller::admit().unwrap();
    let mut cx=caller.context();
    let recipient=cx.original_recipient_identity();
    let patches=PatchTracker::new();
    let key=NativeCloseKey::fixture(941,1);
    patches.reserve_mounted(ui_contract::SurfaceId::try_from("941:original").unwrap(),key).unwrap_or_else(|_|panic!("original fixed surface admission")).cancel_and_defer();
    for _ in 0..OriginalFixtureCaller::maximum_turns(){
        if patches.state.borrow().ready.iter().all(Option::is_none){break}
        patches.close_step(&mut cx).unwrap();
    }
    assert!(patches.state.borrow().ready.iter().all(Option::is_none));
    assert!(patches.has_publishable_work()&&patches.has_drivable_work()&&patches.has_work());
    let original=cx.retained_grant();
    let bytes=size_of::<ui_contract::SurfaceId>();
    for loan in [RetainedCloneGrant{maximum_items:0,..original},RetainedCloneGrant{maximum_copy_bytes:bytes-1,..original}]{
        let before=cx.retained_progress();let fuel=cx.fuel_remaining();
        let mut offered=None;let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{offered=patches.take_pending_render_on_original(same)?;Ok(offered.is_some())}));
        assert!(!result.unwrap());assert!(offered.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
        assert!(patches.has_publishable_work()&&patches.has_drivable_work()&&patches.has_work());
    }
    let loan=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
    let before=cx.retained_progress();let fuel=cx.fuel_remaining();
    let mut offered=None;let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(loan,|same|{offered=patches.take_pending_render_on_original(same)?;Ok(offered.is_some())}));
    assert!(result.unwrap());assert_eq!(offered.unwrap().as_ref(),"941:original");assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    assert_eq!(cx.retained_progress(),before.checked_add(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).unwrap());
    assert_eq!(cx.fuel_remaining(),fuel-1);assert_eq!(cx.original_recipient_identity(),recipient);
    assert!(!patches.has_publishable_work()&&!patches.has_drivable_work()&&!patches.has_work());
    patches.reserve_close_instance(key).unwrap();patches.activate_close_instance(key).unwrap();
    for _ in 0..OriginalFixtureCaller::maximum_turns(){if patches.close_instance_complete(key).unwrap(){break}patches.close_step(&mut cx).unwrap();}
    assert!(patches.close_instance_complete(key).unwrap());patches.release_close_instance(key).unwrap();
    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
}
