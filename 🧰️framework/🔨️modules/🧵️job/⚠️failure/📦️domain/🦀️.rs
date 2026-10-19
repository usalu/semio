//! 📦️ Paid type erasure retains the complete original domain owner in the existing controlled frontier.
use super::{OriginalTurnFailure,OriginalTurnPrimary,StepContext,ValueError};
use semio_framework_value::{RetirementDemand,ValueRefusalKind,retained_clone::{RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement}};
pub use semio_framework_dsl_record::native_encoding::{FieldProjectionSource as OriginalDomainProjection,FieldProjectionView as OriginalDomainView};

/// 🧬️ Domain implementations lend their complete original fields and actual failed effects.
pub trait OriginalDomainFailure:RetireOwned+OriginalDomainProjection+std::fmt::Debug{
 fn domain_code(&self)->&str;
 fn domain_message(&self)->&str;
 fn domain_retained_progress(&self)->RetainedCloneProgress;
}
trait DomainOwner:Send{
 fn original(&self)->&dyn OriginalDomainProjection;
 fn code(&self)->&str;
 fn message(&self)->&str;
 fn progress(&self)->RetainedCloneProgress;
 fn into_retirement(self:Box<Self>)->Box<dyn RetirementCursor>;
}
struct DomainFrame<T:OriginalDomainFailure>{owner:ControlledRetirement<T>}
impl<T:OriginalDomainFailure> DomainOwner for DomainFrame<T>{
 fn original(&self)->&dyn OriginalDomainProjection{self.owner.original().expect("original domain projection precedes retirement")}
 fn code(&self)->&str{self.owner.original().expect("original domain code precedes retirement").domain_code()}
 fn message(&self)->&str{self.owner.original().expect("original domain message precedes retirement").domain_message()}
 fn progress(&self)->RetainedCloneProgress{self.owner.original().expect("original domain receipt precedes retirement").domain_retained_progress()}
 fn into_retirement(self:Box<Self>)->Box<dyn RetirementCursor>{self}
}
impl<T:OriginalDomainFailure> RetirementCursor for DomainFrame<T>{
 fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->RetirementStep{match self.owner.step(grant){Ok(RetainedCloneStep::Complete(progress)|RetainedCloneStep::Progress(progress))=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.owner.selected_demand(body)?.copy_bytes)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.next_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

/// 🫴️ The erased header moves without cloning the admitted typed source or its nested payloads.
pub struct OriginalDomainPrimary{owner:Box<dyn DomainOwner>}
impl std::fmt::Debug for OriginalDomainPrimary{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("OriginalDomainPrimary").field("code",&self.code()).finish_non_exhaustive()}}
impl OriginalDomainPrimary{
 pub fn projection(&self)->&dyn OriginalDomainProjection{self.owner.original()}
 pub fn code(&self)->&str{self.owner.code()}
 pub fn message(&self)->&str{self.owner.message()}
 pub fn retained_progress(&self)->RetainedCloneProgress{self.owner.progress()}
}
impl RetireOwned for OriginalDomainPrimary{
 fn retirement(self)->Box<dyn RetirementCursor>{self.owner.into_retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(0)}
 fn controlled_retirement_supported()->bool{true}
}

/// 📏️ Quotes the complete physical frame before the original failure leaves its producer slot.
pub const fn original_domain_failure_demand<T:OriginalDomainFailure>()->RetirementDemand{RetirementDemand{copy_bytes:0,capacity_bytes:size_of::<DomainFrame<T>>(),release_bytes:0,depth:1}}

/// 🎟️ Moves the whole source after paid admission without posting its already recorded failed effects again.
pub fn admit_original_domain_failure<T:OriginalDomainFailure>(original:&mut Option<T>,cx:&mut StepContext<'_>)->Result<Option<OriginalTurnFailure>,OriginalTurnFailure>{
 if original.is_none(){return Ok(None)}
 if !T::controlled_retirement_supported(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original domain requires its controlled producer").into())}
 let grant=cx.retained_grant();let demand=original_domain_failure_demand::<T>();
 if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(None)}
 let layout=std::alloc::Layout::new::<DomainFrame<T>>();
 cx.consume_fuel(1);
 let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<DomainFrame<T>>())else{return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"original domain frame allocation refused").into())};
 let owner=match ControlledRetirement::new(original.take().expect("observed original domain source")){Ok(owner)=>owner,Err((error,value))=>{*original=Some(value);unsafe{std::alloc::dealloc(pointer.as_ptr().cast(),layout)};return Err(OriginalTurnFailure::debit_original(error.with_retained_progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),released_bytes:layout.size(),..Default::default()}),cx))}};
 let owner=unsafe{pointer.as_ptr().write(DomainFrame{owner});Box::from_raw(pointer.as_ptr())};
 let mut failure={let original=OriginalTurnFailure::from(OriginalTurnPrimary::Domain(OriginalDomainPrimary{owner}));OriginalTurnFailure{receipt_refusal:None,context_refusal:None,..original}};
 failure.receipt_refusal=cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:layout.size(),..Default::default()}).err();
 Ok(Some(failure))
}

#[path="🤝️closure/🦀️.rs"]
mod closure;
pub use closure::OriginalClosureFailure;
