//! 🧪️ Actual auxiliary context admission keeps denied and canceled original turns allocation-free.
use semio_framework_plugin_runtime_ops::ops_surface_render::staged_context;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_plugin_reactor_app::NoPluginApp;
use semio_framework_value::RetainedCloneGrant;

#[test]
fn staged_window_context_denied_original_axes_never_consult_actor_authority(){
 let runtime=PluginRuntime::<NoPluginApp>::new(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy()).unwrap();
 let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut capture=staged_context::StagedWindowContextCapture::default();let mut cx=caller.context();let grant=cx.retained_grant();let recipient=cx.original_recipient_identity();let deadline=cx.deadline_us();
 let authority=runtime.guest_lifetimes.borrow_mut();
 let before=cx.retained_progress();let fuel=cx.fuel_remaining();
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(0,deadline,|same|{assert!(semio_framework_async::poll::resolve_ready(staged_context::capture(&runtime,971,"left",semio_framework::Locale::En,&mut capture,same))?.is_none());Ok(())}));
 result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
 for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
  let before=cx.retained_progress();let fuel=cx.fuel_remaining();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|{assert!(semio_framework_async::poll::resolve_ready(staged_context::capture(&runtime,971,"left",semio_framework::Locale::En,&mut capture,same))?.is_none());Ok(())}));
  result.unwrap();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(cx.original_recipient_identity(),recipient);
 }
 drop(authority);let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).unwrap();
}

#[test]
fn staged_window_context_missing_busy_and_cancelled_sources_keep_original_receipts(){
 let runtime=PluginRuntime::<NoPluginApp>::new(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy()).unwrap();
 let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().unwrap();let mut cx=caller.context();let mut capture=staged_context::StagedWindowContextCapture::default();
 for busy in [false,true]{
  let authority=busy.then(||runtime.guest_lifetimes.borrow_mut());let before=cx.retained_progress();let fuel=cx.fuel_remaining();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(staged_context::capture(&runtime,971,"left",semio_framework::Locale::En,&mut capture,&mut cx)));
  assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);drop(authority);
 }
 cx.original_cancel_token().cancel_now();let before=cx.retained_progress();let fuel=cx.fuel_remaining();let authority=runtime.guest_lifetimes.borrow_mut();
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(staged_context::capture(&runtime,971,"left",semio_framework::Locale::En,&mut capture,&mut cx)));
 assert!(result.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);
 drop(authority);drop(cx);caller.retire(fuel).unwrap();
}
