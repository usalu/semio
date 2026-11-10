//! ♻️ Exact original History test recipient retirement.
use semio_framework_value::{RetainedCloneGrant, RetirementDemand, admit_owned_retirement, owned_retirement_birth_bytes};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity};
use std::mem::size_of_val;
/// 🔬️ Measures the actual requested and released backing bytes of one operation through the shared trace allocator observation.
pub fn observe_backing<T>(operation:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(operation);assert!(!heap.overflowed,"requested allocation observation overflow");(value,heap.requested_bytes,heap.released_bytes)}
fn observe<T>(operation:impl FnOnce()->T)->(T,usize,usize){observe_backing(operation)}
fn permits(g:RetainedCloneGrant,d:RetirementDemand)->bool{g.maximum_items>0&&g.maximum_depth>=d.depth&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes}
pub fn close_original<T:semio_framework_value::retirement::RetireOwned>(original:T,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant)->(usize,usize){let birth=owned_retirement_birth_bytes::<T>();assert!(birth<=g.maximum_capacity_bytes);let mut wallet=NativeWorkWallet::new(1_000_000,4096);let mut slot=Some(original);let(result,born,released)=observe(||effected(control,&mut wallet,g,birth,|effect|admit_owned_retirement(&mut slot,effect)));let(mut owner,p)=result.result.unwrap().unwrap();assert!(p.fits(g));assert_eq!((born,released),(p.retained_capacity_bytes,p.released_bytes));let(mut births,mut releases)=(born,released);for _ in 0..100000{if owner.terminal_is_empty(){let bytes=size_of_val(owner.as_ref());assert!(bytes<=g.maximum_release_bytes);let(_,born,released)=observe(||drop(owner));assert_eq!((born,released),(0,bytes));return(births,releases+released)}let d=owner.next_demand(g.maximum_copy_bytes).unwrap();assert!(permits(g,d));let step=semio_framework_value_clone_source_testing::drive::drive(g,64,|effect|owner.close_step(g.maximum_copy_bytes,effect)).result.unwrap();assert!(step.progress().fits(g));let(born,released)=(step.progress().retained_capacity_bytes,step.progress().released_bytes);births+=born;releases+=released}panic!("original transition recipient retirement failed to terminate")}

/// 🧪️ Runs one funded effect of the grant inside the wallet's work turn.
fn effected<R>(control:&mut NativeDecodeControl<'_>,wallet:&mut NativeWorkWallet,g:RetainedCloneGrant,quote:usize,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->R{let demand=RetirementDemand{copy_bytes:g.maximum_copy_bytes,capacity_bytes:quote,release_bytes:g.maximum_release_bytes,depth:g.maximum_depth};native_turn::<_,semio_framework_value::ValueError>(control,wallet,|native|native.with_allocated_effect(demand,64,|_,effect|run(effect))).unwrap().expect("History test effect work quote was not admitted")}

/// 🎟️ One explicit finite native work wallet spans all observations of its owning fixture.
pub struct NativeWorkWallet{budget:WalletBudget,authority:OriginalWorkAuthority<WalletDriver>}
pub struct WalletBudget{remaining:u64,quantum:u64}
pub struct WalletDriver;
impl NativeWorkWallet{
 pub fn new(remaining:u64,quantum:u64)->Self{Self{budget:WalletBudget{remaining,quantum},authority:OriginalWorkAuthority::new(WalletDriver)}}
}
impl OriginalWorkRecipient for WalletDriver{
 type Context<'t,'s>=WalletBudget where 's:'t;
 fn validate_context<'t,'s>(&self,_:&WalletBudget)->Result<(),semio_framework_value::ValueError> where 's:'t{Ok(())}
 fn source_turn<'t,'s>(&self,_:&WalletBudget)->Option<OriginalSourceTurnIdentity> where 's:'t{None}
 fn admits<'r,'t,'s>(&mut self,c:&mut WalletBudget,quote:OriginalWorkQuote<'r>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'r> where 's:'t{quote.complete(Ok(observed.checked_add(additional).is_some_and(|work|work<=c.remaining.min(c.quantum))))}
 fn settle_partial<'r,'t,'s>(&mut self,_:&mut WalletBudget,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{check.complete(Ok(()))}
 fn recover_partial<'r,'t,'s>(&mut self,_:&mut WalletBudget,check:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r> where 's:'t{check.pending()}
}
/// 🔬️ Observes only the actual native counter under the same wallet; it makes no claim about nonnative callback work.
pub fn native_turn<T,E:From<semio_framework_value::ValueError>>(control:&mut NativeDecodeControl<'_>,wallet:&mut NativeWorkWallet,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 let before=control.actual_work();let result=wallet.authority.with_loan(&mut wallet.budget,|loan|control.with_work_recipient(loan,operation));let actual=u64::try_from(control.actual_work()-before).unwrap();assert!(actual<=wallet.budget.remaining.min(wallet.budget.quantum));wallet.budget.remaining-=actual;result.map_err(E::from).and_then(|inner|inner)
}
