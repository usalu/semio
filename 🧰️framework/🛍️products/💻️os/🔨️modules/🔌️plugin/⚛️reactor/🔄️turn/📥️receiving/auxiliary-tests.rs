//! 🧪️ The actual unissued Pack owner remains stationary under denied auxiliary custody.
use semio_framework_plugin_reactor_turn::original_receiving::{staged::StagedRequestInput,auxiliary::*};
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_plugin_reactor_app::NoPluginApp;
use semio_framework::action_staging::StagedFormLimits;
use semio_framework_value::{DslValue,RetainedCloneGrant};
fn source()->Option<Vec<u8>>{Some(semio_framework_os_kernel::os_pack::pack_rt::encode_wire_value(&DslValue::from(&serde_json::json!({"token":{"action":"applyIfcTemplate","generation":"1","revision":"0".repeat(64),"window":"missing-real-window"},"args":{"id":"9007199254740993"},"locale":"de"}))))}
#[test]
fn auxiliary_original_pack_donor_denied_axes_and_cancelled_prefixes_retire(){
    assert!(!STAGED_AUXILIARY_TRANSPORT_ADMITTED);let runtime=PluginRuntime::<NoPluginApp>::new(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy()).unwrap();
    for cut in [0,1,2,8,32,128,512]{let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let stock=cx.retained_grant();let mut wire=source();let mut donor=StagedRequestInput::admit(&mut wire,StagedFormLimits::FRAMEWORK,1<<20,&mut cx).unwrap();assert!(wire.is_none());
        for denied in [RetainedCloneGrant{maximum_items:0,..stock},RetainedCloneGrant{maximum_copy_bytes:0,..stock},RetainedCloneGrant{maximum_depth:0,..stock}]{let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|cx|StagedAuxiliaryInput::admit(&mut donor,971,"framework.action.form",cx)));assert!(result.unwrap().is_none());assert!(donor.is_some());assert_eq!(cx.retained_progress(),before);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
        let mut owner=StagedAuxiliaryInput::admit(&mut donor,971,"framework.action.form",&mut cx).unwrap().unwrap();assert!(donor.is_none());for _ in 0..cut{let result=semio_framework_async::poll::resolve_ready(owner.step(&runtime,&mut |_,_|panic!("missing actor cannot publish"),&mut cx));if result.is_err(){break;}assert_eq!(result.unwrap(),StagedAuxiliaryProgress::Pending);}cx.original_cancel_token().cancel_now();
        for turn in 0..semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break;}let closed=cx.with_original_retirement(|cx|semio_framework_async::poll::resolve_ready(owner.close_step(&runtime,cx))).unwrap();if closed{break;}assert!(turn+1<semio_framework_plugin_testing_laws::OriginalFixtureCaller::maximum_turns());}assert!(owner.terminal_is_empty());drop(owner);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
    }
}
