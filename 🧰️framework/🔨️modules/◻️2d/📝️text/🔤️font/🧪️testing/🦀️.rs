//! 🧰️ Dev-only support of the font crate family tests: authored font bytes and funded physical close with heap witnesses.
use semio_framework_test_fixtures::fixture_path;
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneStep},work::RetainedWorkLedger,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,ValueError};
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
/// 🔤️ Reads one shipped font asset outline by its `🖼️assets/🔤️fonts` family path.
pub fn bytes(font:&str)->Vec<u8>{let path=fixture_path!("../../../../../../..").join("🖼️assets/🔤️fonts").join(font).join("🔤️outline.ttf");std::fs::read(path).unwrap()}
/// 🧹️ Closes one owner with single-item grants and returns the physically released bytes.
pub fn close<T:RetireOwned>(value:T)->usize{close_with_items(value,1)}
/// 🧮️ Closes one owner with `items`-item grants over one original effect bank, proving every step against the heap witness, and returns the released bytes.
pub fn close_with_items<T:RetireOwned>(value:T,items:usize)->usize{
 let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("{error}"));
 let mut bank=EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},1<<62));
 let mut released=0;
 for _ in 0..if items>1{10000000}else{2000000}{
  if owner.terminal_is_empty(){break;}
  let copy=owner.next_copy_byte_demand().unwrap();
  let release=owner.next_release_byte_demand().unwrap();
  let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};
  let(zero,physical)=turn(&mut bank,&mut owner,copy,RetainedCloneGrant{maximum_items:0,..grant});
  assert!(!physical.overflowed);
  assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));
  assert!(matches!(zero,RetainedCloneStep::Progress(p)if p==Default::default()));
  let(step,physical)=turn(&mut bank,&mut owner,copy,grant);
  let(RetainedCloneStep::Progress(receipt)|RetainedCloneStep::Complete(receipt))=step;
  assert!(receipt.fits(grant));
  assert!(!physical.overflowed);
  assert_eq!(physical.requested_bytes,receipt.retained_capacity_bytes);
  assert_eq!(physical.released_bytes,receipt.released_bytes);
  released+=physical.released_bytes;
 }
 assert!(owner.terminal_is_empty());
 let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));
 assert_eq!(physical.released_bytes,0);
 released
}
fn turn<T:RetireOwned>(bank:&mut EffectBank,owner:&mut ControlledRetirement<T>,body:usize,grant:RetainedCloneGrant)->(RetainedCloneStep,semio_framework_trace::HeapAllocationObservation){
 let mut observed=None;
 let(poll,_)=bank.run(grant,WORK,|effect|{let(poll,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(body,effect));observed=Some(heap);poll});
 assert_ne!(bank.take_admitted(),Some(false));
 let step=match poll{RetainedWorkPoll::Complete(turn)=>turn.result.unwrap_or_else(|_|panic!("font retirement step failed")),RetainedWorkPoll::Pending{..}=>RetainedCloneStep::Progress(Default::default())};
 (step,observed.expect("effect ran"))
}
/// ⏱️ Fixture work ceiling every close turn may record.
pub const WORK:u64=1<<20;

/// 🎟️ Fixture authority is explicit and independent of a font child's quote.
pub fn work_grant(maximum_items:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:128}}

/// 🧵️ Runs one producer turn over a fresh ledger and the actual original effect of an effect bank, both granted by [`work_grant`].
pub fn with_work<R>(maximum_items:usize,operation:impl FnOnce(&mut RetainedWorkLedger,&mut RetainedAllocationEffect<'_>)->R)->R{
 let grant=work_grant(maximum_items);
 let mut bank=EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},1<<62));
 let mut output=None;
 let(poll,_)=bank.run(grant,WORK,|effect|{let boundary=effect.performed_boundary();output=Some(operation(&mut RetainedWorkLedger::new(grant),effect));let work=effect.performed_since(boundary).unwrap();RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})});
 assert!(matches!(poll,RetainedWorkPoll::Complete(_)));
 assert_ne!(bank.take_admitted(),Some(false));
 output.expect("producer turn ran")
}
/// 🎫️ Runs one producer turn over the actual original effect of an effect bank with a generous finite stock.
pub fn with_effect<R>(operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{
 let grant=RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};
 let mut bank=EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},1<<62));
 let mut output=None;
 let(poll,_)=bank.run(grant,WORK,|effect|{let boundary=effect.performed_boundary();output=Some(operation(effect));let work=effect.performed_since(boundary).unwrap();RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})});
 assert!(matches!(poll,RetainedWorkPoll::Complete(_)));
 assert_ne!(bank.take_admitted(),Some(false));
 output.expect("producer turn ran")
}
