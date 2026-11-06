use super::*;
use crate::{DslValue,source_fixture::receive};
#[path="📥️driver/🦀️.rs"]mod driver;
use driver::*;
#[test]
fn native_shared_original_receive_exact_prefix_cancellation_and_transfer(){let vectors=fixture();for law in vectors["cases"].as_array().unwrap(){let mut complete=0;for cut in 0..1000{let mut bank=bank();let original=source(DslValue::String(law["original"].as_str().unwrap().to_owned()),&mut bank);let foreign=source(DslValue::String(law["original"].as_str().unwrap().to_owned()),&mut bank);let(mut receiver,h)=crate::observe_retirement_allocations(<SealedShared<String>as NativeReceive<DslValue>>::receiver);assert_eq!(h,(0,0));let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);let mut turns=0;while !receiver.output_ready()&&turns<cut{let d=receiver.demands(original.borrow(),4096,bank.work_remaining()).unwrap();deny(&mut receiver,original.borrow(),&mut control,&mut bank,d);let g=bank.loan(1,4096);original_turn(&mut receiver,original.borrow(),&mut control,g,&mut bank).unwrap();turns+=1;if receiver.binding.is_some()&&!receiver.output_ready(){assert!(receiver.demands(foreign.borrow(),4096,bank.work_remaining()).is_err());}}if receiver.output_ready(){complete=turns;assert!(!receiver.child.reserved());assert!(receiver.retired_header.is_none());assert!(receiver.raw.is_none());let pointer=receiver.output.as_ref().unwrap().get().as_ptr();let value=take(&mut receiver,&mut bank);assert_eq!(value.get().as_ptr(),pointer);assert_eq!(value.get(),law["expected"].as_str().unwrap());drain(value,&mut bank);}close(&mut receiver,&mut bank);drain(original,&mut bank);drain(foreign,&mut bank);if complete>0&&cut>complete{break}}assert!(complete>0)}}
#[test]
fn native_shared_original_callback_refusal_retains_paid_child_authority(){let mut bank=bank();let original=source(DslValue::String("authored original".to_owned()),&mut bank);let mut receiver=<SealedShared<String>as NativeReceive<DslValue>>::receiver();let admitted=std::cell::Cell::new(true);let mut callback=|_|admitted.get();let mut control=NativeDecodeControl::new(1<<20,&mut callback);for _ in 0..100{if receiver.phase==Phase::Receive{break}let g=bank.loan(1,4096);original_turn(&mut receiver,original.borrow(),&mut control,g,&mut bank).unwrap();}assert!(receiver.child.initialized());let pointer=receiver.child.address();admitted.set(false);let g=bank.loan(1,4096);let(t,h)=crate::observe_retirement_allocations(||receive(&mut control,g,bank.work_loan(bank.work_remaining()),|c|{let maximum=c.available_work_units().unwrap();receiver.advance(original.borrow(),c,4096,maximum,g)}));assert!(t.result.is_err());assert_eq!(h,(0,0));let p=t.result.as_ref().unwrap_err().retained_progress();bank.post(p,t.work,g,h);assert_eq!(receiver.child.address(),pointer);assert!(!receiver.output_ready());close(&mut receiver,&mut bank);drain(original,&mut bank);}


#[test]
fn original_effect_recovery_gates_native_receiver_close() {
    use crate::native_decoding::{NativeConstructionRecipientIssuer,NativeDecodeAllocationEvent,NativeDecodeAllocationIssuer,NativeDecodeConstructionRecipient,NativeDecodeWorkIssuer,NativeDecodeWorkRecipient};
    struct Construction{grant:RetainedCloneGrant,submitted:RetainedCloneProgress}
    impl NativeDecodeConstructionRecipient for Construction {
        fn submitted(&self)->RetainedCloneProgress{self.submitted}
        fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{if !crate::retained_clone::retained_clone_grant_admits(self.grant,d){return Err(ValueError::invariant("native fixture exceeds its declared construction stock"))}Ok(self.grant)}
        fn accept(&mut self,g:RetainedCloneGrant,p:RetainedCloneProgress)->Result<(),ValueError>{assert!(p.fits(g)&&p.fits(self.grant));let total=self.submitted.checked_add(p)?;if !total.fits(self.grant){return Err(ValueError::invariant("native fixture cumulative construction stock exhausted"))}self.submitted=total;Ok(())}
    }
    struct Fuel;
    impl NativeDecodeWorkRecipient for Fuel{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{8u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed fixture work exceeds its eight-unit scope"))}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=8))}}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🏭️construction/📦️effect/🧪️fixtures/🛟️recovery/🔣️.json")).unwrap();
    let pending=fixture["cases"].as_array().unwrap().iter().find(|row|row["phase"]=="EffectStarted").unwrap();
    assert_eq!(pending["closeAllowed"],false);
    let mut receiver=<SealedShared<String> as NativeReceive<DslValue>>::receiver();
    let mut original=Some("owned recovery input".to_owned());
    let pointer=original.as_ref().unwrap().as_ptr();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
    assert!(receiver.slots.capture(&mut original,grant).result.is_ok());
    let mut callback=|_|true;
    let mut allocation=|event|match event{NativeDecodeAllocationEvent::Post{..}=>panic!("original post interrupted"),_=>Ok(())};
    let mut allocation_issuer=NativeDecodeAllocationIssuer::new();
    let mut allocation_loan=allocation_issuer.loan(&mut allocation);
    let mut control=NativeDecodeControl::new_forwarded(1<<20,&mut callback,&mut allocation_loan);
    let mut recipient=Construction{grant,submitted:Default::default()};
    let mut recipient_issuer=NativeConstructionRecipientIssuer::new();
    let mut recipient_loan=recipient_issuer.loan(&mut recipient);
    let mut fuel=Fuel;
    let mut work_issuer=NativeDecodeWorkIssuer::new();
    let mut work_loan=work_issuer.loan(&mut fuel);
    let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.with_work_recipient(&mut work_loan,|control|control.with_construction_recipient(&mut recipient_loan,|control|Ok::<_,ValueError>(receiver.slots.advance(control,SharedIssuer::owned()))))));
    assert!(interrupted.is_err());
    assert!(receiver.has_pending_recovery());
    assert!(receiver.begin_close_step(grant,8).result.is_err());
    assert!(!receiver.closing);
    assert_eq!(receiver.slots.input().unwrap().as_ptr(),pointer);
    let original_frame=match receiver.pending_recovery().unwrap(){crate::owned::NativeRecoveryLoan::Value(frame)=>frame as *mut _,_=>panic!("unexpected recovery type")};
    assert_eq!(match receiver.pending_recovery().unwrap(){crate::owned::NativeRecoveryLoan::Value(frame)=>frame as *mut _,_=>panic!("unexpected recovery type")},original_frame);
    let mut foreign=Construction{grant,submitted:Default::default()};
    let mut foreign_issuer=NativeConstructionRecipientIssuer::new();
    let foreign_loan=foreign_issuer.loan(&mut foreign);
    let saved=std::mem::replace(&mut recipient_loan,foreign_loan);
    let refused=control.with_work_recipient(&mut work_loan,|control|control.with_construction_recipient(&mut recipient_loan,|control|Ok::<_,ValueError>(match receiver.pending_recovery().unwrap(){crate::owned::NativeRecoveryLoan::Value(frame)=>frame.resolve_allocation_callback(control,Ok(())),_=>panic!("unexpected recovery type")}))).unwrap();
    assert!(refused.is_err());
    let _foreign=std::mem::replace(&mut recipient_loan,saved);
    let result=control.with_work_recipient(&mut work_loan,|control|control.with_construction_recipient(&mut recipient_loan,|control|{
        match receiver.pending_recovery().unwrap(){crate::owned::NativeRecoveryLoan::Value(frame)=>frame.resolve_allocation_callback(control,Ok(())),_=>panic!("unexpected recovery type")}.map_err(|(error,_)|error)?;
        Ok::<_,ValueError>(receiver.resume_recovery(control))
    })).unwrap();
    assert!(result.result.is_ok());
    assert!(!receiver.has_pending_recovery());
    drop(work_loan);
    assert!(!work_issuer.has_pending_work());
    let mut close_bank=bank();let g=close_bank.loan(1,4096);let q=receiver.begin_close_work_demand().unwrap();assert!(q<=close_bank.work_remaining());let(turn,heap)=crate::observe_retirement_allocations(||receiver.begin_close_step(g,q));post(&mut close_bank,turn,g,heap,|e|panic!("unexpected close-intent refusal: {e:?}")).unwrap();let mut actual=None;
    for _ in 0..10000{if receiver.terminal_is_empty()&&actual.is_none(){break}let grant=close_bank.loan(1,4096);let(turn,heap)=crate::observe_retirement_allocations(||crate::source_fixture::close_step(&mut actual,grant,close_bank.work_remaining(),receiver.close_demands(4096),receiver.close_work_demand(4096),|g,a|receiver.close_step(g,a)));post(&mut close_bank,turn,grant,heap,|error|error.retained_progress()).unwrap();}
    assert!(receiver.terminal_is_empty());
}
