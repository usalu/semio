//! 🧪️ A real Window partition lends its exact native source to bounded capture and returns it on cancellation.
use super::*;
use semio_framework_job::*;
use semio_framework_value::RetirementPollExit;

fn turn<T>(
    caller: &mut OriginalJobFixtureCaller,
    decoder: &mut OriginalJobDecodeOwner,
    grant: RetainedCloneGrant,
    run: impl FnOnce(&mut StepContext<'_>) -> Result<T, OriginalTurnFailure>,
) -> T {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        caller.refreshed_turn_with_decoder(grant, decoder, run)
    });
    let receipt = caller.receipt();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
    result.unwrap()
}
#[test]
fn original_window_authority_capture_preserves_native_read_and_cancels_partial_addresses(){
 type O=crate::original_genesis_tests::Owner;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:4096};
 for cut in [0,1,3,10,100,usize::MAX]{let mut caller=OriginalJobFixtureCaller::admit(OperationId(219),Generation(9),RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,..grant},default_now_us,u64::MAX,1<<28);
 caller.retain_dispatch().unwrap();
 let mut decoder=caller.admitted_decoder();
 let mut registry=WindowConfigOwnerRegistry::new(protocol::ActorId("capture".into()));registry.register::<O>().unwrap();let window="Ω".repeat(137);let mut view=semio_framework::ViewModel::new(semio_framework::Locale::En,semio_framework::Terminology::Native);view.window_id=Some(window.clone());view.window_instances=vec![semio_framework::ViewWindowInstance{id:window,window_kind_id:O::WINDOW_KIND_ID.into()}];
 for index in 0..262144{if turn(&mut caller,&mut decoder,grant,|cx|registry.ensure_for_view(Some(&view),cx)){break}assert!(index<262143,"native Window partition did not initialize");}
 let mut source=None;turn(&mut caller,&mut decoder,grant,|cx|OriginalWindowConfigCapture::admit(cx,&mut source));let mut ready=false;for index in 0..262144{if index==cut{caller.cancel().cancel_now();break}ready=turn(&mut caller,&mut decoder,grant,|cx|source.as_mut().unwrap().advance(&mut registry,Some(&view),cx));if ready{break}assert!(index<262143,"native Window capture stalled");}
 if ready {
     let mut authority = None;
     turn(&mut caller, &mut decoder, grant, |cx| source.as_mut().unwrap().take_into(&mut authority, cx));
     let authority = authority.unwrap();
     assert_eq!(authority.window_id, view.window_id.as_deref().unwrap());
     assert_eq!(authority.window_kind_id, O::WINDOW_KIND_ID);
     assert_eq!(authority.snapshot.get::<O>().unwrap(), &crate::original_genesis_tests::State::default());
     let mut close = semio_framework_value::retained_clone::RetainedCloneClose::default();
     let mut owner = Some(authority);
     const BODY_BYTES: usize = 4096;
     for _ in 0..262144 {
         if owner.is_none() && close.is_empty() { break; }
         turn(&mut caller, &mut decoder, grant, |cx| cx.with_original_retirement(|cx| {
             let demand = close.next_owner_demands::<WindowConfigAuthority>(owner.is_some(), &None, BODY_BYTES)?;
             close_original_effect_step(cx, demand, |effect| {
                 if close.is_empty() {
                     let begin = close.begin_granted(&mut owner, effect);
                     RetirementPollExit::settle(effect, begin.result
                         .map(|step| step.unwrap_or(RetainedCloneStep::Progress(Default::default())))
                         .map_err(Into::into))
                 } else {
                     close.step_granted(BODY_BYTES, effect)
                 }
             })?;
             Ok(())
         }));
     }
     assert!(owner.is_none() && close.is_empty());
 }
 while !source.as_ref().unwrap().terminal_is_empty(){turn(&mut caller,&mut decoder,grant,|cx|cx.with_original_retirement(|cx|source.as_mut().unwrap().close_step(cx)));}drop(source);
 for index in 0..262144{if registry.terminal_is_empty(){break}turn(&mut caller,&mut decoder,grant,|cx|cx.with_original_retirement(|cx|registry.close_step_context(grant,cx).map(|_|()).map_err(|failure|failure.into())));assert!(index<262143,"native captured registry did not close");}drop(registry);
 for _ in 0..262144 {
     if decoder.terminal_is_empty() { break; }
     caller.refreshed_turn(grant, |cx| cx.with_original_retirement(|cx| decoder.close_on_source(cx))).unwrap();
 }
 assert!(decoder.terminal_is_empty(), "capture fixture decoder did not close");
 caller.retire(grant);
 }
}

/// 🪺️ Every mounted Window of the roster owns its config partition without being addressed, so a background render and a mounted reader of an untouched Window resolve its authority.
#[test]
fn roster_partitions_open_for_every_unaddressed_window(){
 type O=crate::original_genesis_tests::Owner;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:4096};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(221),Generation(3),RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,..grant},default_now_us,u64::MAX,1<<28);
 caller.retain_dispatch().unwrap();
 let mut decoder=caller.admitted_decoder();
 let mut registry=WindowConfigOwnerRegistry::new(protocol::ActorId("roster".into()));registry.register::<O>().unwrap();
 let mut view=semio_framework::ViewModel::new(semio_framework::Locale::En,semio_framework::Terminology::Native);
 view.window_instances=["model","results"].map(|id|semio_framework::ViewWindowInstance{id:id.into(),window_kind_id:O::WINDOW_KIND_ID.into()}).to_vec();
 assert!(registry.borrow(O::WINDOW_KIND_ID,"results").is_none());
 for index in 0..262144{if turn(&mut caller,&mut decoder,grant,|cx|registry.ensure_for_roster(Some(&view),cx)){break}assert!(index<262143,"roster partitions did not initialize");}
 assert!(["model","results"].iter().all(|id|registry.borrow(O::WINDOW_KIND_ID,id).is_some()));
 for index in 0..262144{if registry.terminal_is_empty(){break}turn(&mut caller,&mut decoder,grant,|cx|cx.with_original_retirement(|cx|registry.close_step_context(grant,cx).map(|_|()).map_err(|failure|failure.into())));assert!(index<262143,"roster registry did not close");}drop(registry);
 for _ in 0..262144{if decoder.terminal_is_empty(){break}caller.refreshed_turn(grant,|cx|cx.with_original_retirement(|cx|decoder.close_on_source(cx))).unwrap();}
 assert!(decoder.terminal_is_empty(),"roster fixture decoder did not close");
 caller.retire(grant);
}
