//! 🪢️ Cancellation after retirement reservation closes the real unpublished key and its physical cursor.
use super::*;
use semio_framework_value::{native_decoding::owned::tree::NativeSequenceCollection,retirement::controlled::ControlledRetirement,RetainedWorkPoll,RetainedCloneProgress};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
use semio_framework_value_testing_admission as admission;
use std::cell::RefCell;
thread_local!{static BANK:RefCell<Option<admission::EffectBank>>=const{RefCell::new(None)};}
fn new_bank()->admission::EffectBank{admission::EffectBank::new(admission::BankWallet::from_profile(&serde_json::json!({"items":1u64<<30,"copyBytes":1u64<<40,"capacityBytes":1u64<<40,"releaseBytes":1u64<<40,"depth":1u64<<20,"fuel":1u64<<60})))}
fn warm(){BANK.with(|slot|{let mut slot=slot.borrow_mut();if slot.is_none(){*slot=Some(new_bank())}})}
fn fx<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->R{
 BANK.with(|slot|{
  let mut bank=slot.borrow_mut().take().unwrap_or_else(new_bank);let mut result=None;
  let _=bank.run(grant,1<<40,|effect|{result=Some(run(effect));let work=effect.performed_work();admission::RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})});
  bank.take_admitted();*slot.borrow_mut()=Some(bank);result.expect("effect call ran")
 })
}
fn reserve(payer:&mut admission::Authorities,control:&mut NativeDecodeControl<'_>,storage:&mut NativeTurnPatchStorage,maximum:u64,grant:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep,OriginalReceivingPrimary>,(usize,usize)){
 let mut heap=(0,0);
 let poll=payer.nested(control,grant,|control,_|{let(turn,observed)=observe(||UiTurnPatches::reserve_step(storage,0,UI_TURN_PATCHES_MAXIMUM,&mut (),control,maximum,grant));heap=observed;RetainedWorkPoll::Complete(turn)});
 match poll{RetainedWorkPoll::Complete(turn)=>(turn,heap),RetainedWorkPoll::Pending{..}=>panic!("original patch reservation stayed pending")}
}
#[test]
fn original_ui_turn_patch_reservation_keeps_unpublished_ownership_until_paid_close(){
 warm();
 let mut bank=RetainedCloneGrant{maximum_items:1000000,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:64};let mut fuel=1000000u64;let mut born=0;let mut freed=0;let(mut storage,heap)=observe(||NativeTurnPatchStorage{patches:UiTurnPatches::default(),candidate:0});assert_eq!(heap,(0,0));
 let post=|bank:&mut RetainedCloneGrant,fuel:&mut u64,p:RetainedCloneProgress,work:u64,heap:(usize,usize)|{assert!(p.fits(*bank));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;*fuel=fuel.checked_sub(work).unwrap();};
 let mut payer=admission::Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<20,&mut callback);
 let _=payer.nested(&mut control,bank,|control,_|{control.prepare_work(1).unwrap();control.commit_work(1).unwrap();RetainedWorkPoll::<RetainedCloneStep,OriginalReceivingPrimary>::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:1})});
  for at in 0..UI_TURN_PATCH_RETIRE_SLOTS+1{
  if storage.patches.entries[0].retirement.is_some(){break}
  let d=UiTurnPatches::reserve_demand(&storage,0,UI_TURN_PATCHES_MAXIMUM,&()).unwrap().unwrap();
  for denied in [RetainedCloneGrant{maximum_items:0,..bank},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..bank},RetainedCloneGrant{maximum_depth:0,..bank}]{
   let before=storage.candidate;let maximum=fuel;
   let(turn,heap)=reserve(&mut payer,&mut control,&mut storage,maximum,denied);
   assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(storage.candidate,before);
  }
  let maximum=fuel;let offer=bank;
  let(turn,heap)=reserve(&mut payer,&mut control,&mut storage,maximum,offer);
  let spent=u64::try_from(turn.work).unwrap();post(&mut bank,&mut fuel,turn.result.unwrap().progress(),spent,heap);assert!(at<UI_TURN_PATCH_RETIRE_SLOTS);
 }
 let key=storage.patches.entries[0].retirement.expect("actual reservation remains in unpublished Source storage");assert_eq!(storage.patches.length,1);assert!(storage.patches.entries[0].contents.terminal_is_empty());assert!(storage.patches.iter().next().is_none());let(mut close,heap)=observe(||ControlledRetirement::new(storage).unwrap_or_else(|_|panic!("native patch storage supports paid retirement")));assert_eq!(heap,(0,0));
 for at in 0..100000{
  if close.terminal_is_empty(){break}
  let body=bank.maximum_copy_bytes;let(polled,heap)=observe(||fx(bank,|effect|{let poll=close.step(body,effect);(poll,effect.performed_work())}));let(poll,work)=polled;
  let result=match poll{RetainedWorkPoll::Complete(turn)=>turn.result,RetainedWorkPoll::Pending{..}=>panic!("original patch close stayed pending")};
  let p=result.unwrap().progress();post(&mut bank,&mut fuel,p,u64::try_from(work).unwrap(),heap);born+=p.retained_capacity_bytes;freed+=p.released_bytes;assert!(at<99999)
 }
 assert!(close.terminal_is_empty());assert_eq!(born,freed);assert!(fuel>0);let arena=UI_TURN_PATCH_RETIRE_ARENA.lock().unwrap();assert!(!arena.slots[key.slot].reserved||arena.slots[key.slot].epoch!=key.epoch);
}
