//! 📦️ Retained cloning and retirement for schema-preserving paged carriers.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_progress};
use crate::{ValueError, ValueRefusalKind, paged::{PAGED_BYTES_CHUNK_BYTES, PagedBytes, PagedMap, PagedUtf8}, retirement::{RetireOwned, RetirementCursor}};
use semio_framework_value_persistent::list::PagedList;
use std::{mem::{ManuallyDrop, size_of}, ops::{Deref, DerefMut}};
type CloseRoute=Option<(crate::OriginalEffectInvocationIdentity,u8,usize,Option<(crate::NativeAllocationIdentity,Option<u64>,crate::OriginalWorkBindingIdentity)>)>;
type CloseReply<'r>=crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
type CloseComplete<'r>=crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
macro_rules! routed{($state:ident,$id:expr,$body:expr,$orig:expr,$effect:ident,$reply:ident,|$child:ident|$call:expr)=>{{
 let invocation=$effect.invocation().ok_or_else(||ValueError::invariant("paged cursor close lacks its original invocation"))?;
 if invocation.mode()!=crate::OriginalEffectCallbackMode::Receive{return Err(ValueError::invariant("paged cursor close is not its original receive invocation"))}
 if $state.close_route.is_some(){return Err(ValueError::invariant("paged cursor has an unresolved original close invocation"))}
 let key=(invocation.identity(),$id,$body,$orig);
 let mut scope=$reply.progress().scope(&mut $state.close_route,key)?;
 let done={let $child=scope.borrow()?;$call}?;drop(done);Ok(scope.forwarded())}}}
macro_rules! recovered{($state:ident,$body:expr,$orig:expr,$effect:ident,$reply:ident,|$route:ident,$child:ident|$call:expr)=>{{
 let (identity,$route,selected_body,selected_original)=$state.close_route.ok_or_else(||ValueError::invariant("paged cursor has no original close invocation"))?;
 if !$effect.invocation().is_some_and(|i|i.identity()==identity&&i.mode()==crate::OriginalEffectCallbackMode::Recover)||selected_body!=$body||selected_original!=$orig{return Err(ValueError::invariant("paged cursor close recovery changed invocation"))}
 let mut scope=$reply.progress().scope(&mut $state.close_route,(identity,$route,selected_body,selected_original))?;
 let done={let $child=scope.borrow()?;$call}?;drop(done);Ok(scope.forwarded())}}}
fn original_key(original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&crate::RetainedAllocationEffect<'_>)->Result<(crate::NativeAllocationIdentity,Option<u64>,crate::OriginalWorkBindingIdentity),ValueError>{let work=original.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("paged cursor original close changed Work"))}Ok((original.ledger_identity(),original.retirement_recipient_identity(),work))}

pub struct PagedBytesCursor<const N: usize> {
    state: ManuallyDrop<PagedBytesCursorState<N>>,
}

#[doc(hidden)]
pub struct PagedBytesCursorState<const N: usize> {
    bytes: PagedList<u8, N>,
    output: Option<PagedBytes<N>>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
    close_route: CloseRoute,
}

impl<const N: usize> Deref for PagedBytesCursor<N> {
    type Target = PagedBytesCursorState<N>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<const N: usize> DerefMut for PagedBytesCursor<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<const N: usize> Default for PagedBytesCursor<N> {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(PagedBytesCursorState { bytes: PagedList::default(), output: None, source: None, phase: 0, closing: false, close: RetainedCloneClose::default(), close_route: None }) }
    }
}

impl<const N: usize> Drop for PagedBytesCursor<N> {
    fn drop(&mut self) {
        let empty = self.bytes.terminal_is_empty() && self.output.is_none() && self.source.is_none() && self.close.is_empty();
        assert!(std::thread::panicking() || empty, "paged octet clone cursor abandoned before bounded terminal close");
        if empty {
            unsafe { ManuallyDrop::drop(&mut self.state) };
        }
    }
}

impl<const N:usize> PagedBytesCursor<N> {
    fn advance_paid(&mut self, source: RetainedCloneRef<'_, PagedBytes<N>>, body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep,crate::OriginalRetirementFailure> {let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "paged octet retained clone cursor is closing")).into());
        }
        let demand = self.advance_demands(source,body,maximum_work_units)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !super::retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,effect);turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        match self.phase {
            0 => {
                let source = source.get().retained_bytes();
                if self.bytes.capacity() < source.len() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(Default::default()));
                    }
                    let required = self.bytes.next_capacity_allocation_bytes(source.len()).map_err(ValueError::from)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "paged octet clone capacity state is inconsistent"))?;
                    if required > grant.maximum_capacity_bytes {
                        return Ok(RetainedCloneStep::Progress(Default::default()));
                    }
                    let turn=self.bytes.reserve_capacity_one_funded(source.len(),effect);
                    return match turn.result{Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Ready(progress))=>Ok(RetainedCloneStep::Progress(progress)),Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Denied)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(semio_framework_value_persistent::list::PagedListReservationFailure::Work(error))=>Err(error.into()),Err(semio_framework_value_persistent::list::PagedListReservationFailure::Allocation{error,..})=>Err((ValueError::from(error.refusal()).with_retained_progress(RetainedCloneProgress{copied_items:usize::from(error.allocated_bytes!=0),retained_capacity_bytes:error.allocated_bytes,..Default::default()})).into())};
                }
                if self.bytes.len() < source.len() {
                    if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 {
                        return Ok(RetainedCloneStep::Progress(Default::default()));
                    }
                    let count = (source.len() - self.bytes.len()).min(body.max(1)).min(PAGED_BYTES_CHUNK_BYTES);
                    let start = self.bytes.len();
                    if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged octet clone exceeded original Work").into())}for index in start..start + count {
                        self.bytes.push_reserved(*source.get(index).ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "paged octet clone source changed"))?).map_err(|_| ValueError::literal(ValueRefusalKind::InvariantViolated, "paged octet clone lost its reserved capacity"))?;
                    }
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, retained_capacity_bytes: 0, released_bytes: 0 }));
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged octet clone exceeded original Work").into())}self.output = Some(PagedBytes::from_retained_bytes(std::mem::take(&mut self.bytes)));
                self.phase = 1;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            }
            1 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "paged octet retained clone cursor is spent")).into()),
        }
    })()}
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if !self.closing{return Ok(Default::default());}
        if !self.bytes.terminal_is_empty(){return self.close.next_owner_demands::<PagedList<u8,N>>(true,&self.source,body);}
        self.close.next_owner_demands::<PagedBytes<N>>(self.output.is_some(),&self.source,body)
    }
}
impl<const N: usize> RetainedCloneCursor<PagedBytes<N>> for PagedBytesCursor<N> {
    fn output_ready(&self) -> bool { !self.closing && self.phase == 1 && self.output.is_some() }
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{PagedBytesCursor::close_demands(self,body)}

    fn advance_demands(&self, source: RetainedCloneRef<'_, PagedBytes<N>>, body:usize,maximum_work_units:u64) -> Result<crate::RetirementDemand, ValueError> {
        if self.closing || self.phase == 2 { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "paged bytes clone is closing or spent")); }
        if !source.check_binding(&self.source)? { return Ok(crate::RetirementDemand { copy_bytes: source.binding_copy_bytes(), depth: 1, ..Default::default() }); }
        if self.output_ready() { return Ok(Default::default()); }
        let length = source.get().len();
        Ok(if self.bytes.capacity() < length { crate::RetirementDemand { capacity_bytes: self.bytes.next_capacity_allocation_bytes(length)?.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "paged bytes capacity action absent"))?, depth: 1, ..Default::default() } } else { crate::RetirementDemand { copy_bytes: if self.bytes.len() < length { (length-self.bytes.len()).min(body.max(1)).min(PAGED_BYTES_CHUNK_BYTES) } else { size_of::<PagedBytes<N>>() }, depth: 1, ..Default::default() } })
    }


    fn native_terminal_release(&self)->Option<crate::retained_clone::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{crate::retained_clone::RetainedCursorRelease::native(|cursor:&Self|cursor.terminal_is_empty())})}
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,PagedBytes<N>>,body:usize,maximum:u64)->Result<u64,ValueError>{
        if self.closing||self.phase==2{return Err(ValueError::invariant("paged clone is closing or spent"))}
        if maximum==0||self.output_ready(){return Ok(0)}
        if self.source.is_none(){return source.binding_work_demand(&self.source)}
        let length=source.get().len();
        if self.bytes.capacity()<length{return self.bytes.reserve_capacity_one_work_demand(length).map_err(ValueError::from)}
        Ok(1)
    }
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.closing{return Ok(0)}
        if !self.bytes.terminal_is_empty(){return self.close.next_owner_work::<PagedList<u8,N>>(true,&self.source,body)}
        self.close.next_owner_work::<PagedBytes<N>>(self.output.is_some(),&self.source,body)
    }
    fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<PagedBytes<N>,u8>()}
    fn take<'reply>(&mut self,output:&mut Option<PagedBytes<N>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.phase,2,effect,reply,PagedBytes<N>,u8)}

    

    

    fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,PagedBytes<N>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Ok(reply.complete(self.advance_paid(source,body,maximum,effect)))}
    fn recover_advance_into<'r>(&mut self,_source:RetainedCloneRef<'_,PagedBytes<N>>,_body:usize,_maximum:u64,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Ok(reply.pending())}
    fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{let turn=self.begin_close_paid(effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
    fn recover_begin_close_into<'r>(&mut self,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Ok(reply.pending())}
    fn requires_original_birth(&self)->bool{if self.close_route.is_some(){return false}if !self.close.is_empty(){self.close.requires_original_birth()}else if !self.bytes.terminal_is_empty()||self.output.is_some(){false}else{RetainedCloneBinding::requires_original_birth(&self.source)}}
    fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.close.is_empty(){self.close.step_original_birth(original,grant)}else if !self.bytes.terminal_is_empty()||self.output.is_some(){crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original Birth is not the selected cursor frontier").into()),work:0})}else{RetainedCloneBinding::close_original_birth_step(&mut self.source,original,grant)}}
    fn requires_original_control(&self)->bool{if let Some((_,route,_,_))=self.close_route{return route>=3}if !self.close.is_empty(){self.close.requires_original_control()}else if !self.bytes.terminal_is_empty()||self.output.is_some(){false}else{RetainedCloneBinding::requires_original_control(&self.source)}}
    fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        let grant=effect.grant();
        if !self.closing{return Err(ValueError::invariant("paged owner must begin close before granted retirement"))}
        if self.requires_original_birth()||self.requires_original_control(){return Err(ValueError::invariant("paged owner requires selected original close"))}
        if grant.maximum_items==0||!super::retained_clone_grant_admits(grant,PagedBytesCursor::close_demands(self,grant.maximum_copy_bytes)?){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
        let state=&mut *self.state;
        if !state.close.is_empty(){return routed!(state,1,body,None,effect,reply,|child|state.close.step_into(body,effect,child))}
        if !state.bytes.terminal_is_empty(){if let Some(step)=state.close.begin_empty_granted(&mut state.bytes,effect).result?{return Ok(reply.complete(Ok(step)))}}
        if let Some(step)=state.close.begin_granted(&mut state.output,effect).result?{return Ok(reply.complete(Ok(step)))}
        routed!(state,2,body,None,effect,reply,|child|RetainedCloneBinding::close_into(&mut state.source,body,effect,child))
    }
    fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        let state=&mut *self.state;
        recovered!(state,body,None,effect,reply,|route,child|match route{1=>state.close.recover_step_into(body,effect,child),2=>RetainedCloneBinding::recover_close_into(&mut state.source,body,effect,child),_=>Err(ValueError::invariant("paged cursor close route changed"))})
    }
    fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if self.requires_original_birth(){return Err(ValueError::invariant("paged cursor Birth must settle before original close"))}
        if !self.requires_original_control(){return self.close_into(body,effect,reply)}
        let key=Some(original_key(original,effect)?);
        let state=&mut *self.state;
        if !state.close.is_empty(){return routed!(state,3,body,key,effect,reply,|child|state.close.step_original_into(original,body,effect,child))}
        routed!(state,4,body,key,effect,reply,|child|RetainedCloneBinding::close_original_into(&mut state.source,original,body,effect,child))
    }
    fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if matches!(self.close_route,Some((_,1|2,_,_))){return self.recover_close_into(body,effect,reply)}
        let key=Some(original_key(original,effect)?);
        let state=&mut *self.state;
        recovered!(state,body,key,effect,reply,|route,child|match route{3=>state.close.recover_step_original_into(original,body,effect,child),4=>RetainedCloneBinding::recover_close_original_into(&mut state.source,original,body,effect,child),_=>Err(ValueError::invariant("paged cursor close route changed"))})
    }

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.bytes.terminal_is_empty() && self.output.is_none() && self.close.is_empty() && self.source.is_none() && self.close_route.is_none() }
}

impl<const N: usize> RetireOwned for PagedBytes<N> {
    type RetirementCursor=<PagedList<u8,N> as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{
        self.into_retained_bytes().retirement_seed()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<crate::retirement::PagedListRetirement<u8, N>>()) }
    fn controlled_retirement_supported() -> bool { true }
}

impl<const N: usize> RetainedClone for PagedBytes<N> {
    type Cursor = PagedBytesCursor<N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedBytesCursor::default()
    }
}

pub struct PagedUtf8Cursor<const N: usize> {
    inner: <PagedList<String, N> as RetainedClone>::Cursor,
    chunks: Option<PagedList<String, N>>,
    output: Option<PagedUtf8<N>>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
    close_route: CloseRoute,
    child_pending: u8,
}

impl<const N: usize> Default for PagedUtf8Cursor<N> {
    fn default() -> Self {
        Self { inner: PagedList::retained_clone_cursor(), chunks: None, output: None, source: None, phase: 0, closing: false, close: RetainedCloneClose::default(), close_route: None, child_pending: 0 }
    }
}

impl<const N:usize> PagedUtf8Cursor<N> {
    fn advance_paid(&mut self, source: RetainedCloneRef<'_, PagedUtf8<N>>, body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep,crate::OriginalRetirementFailure> {let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is closing")).into());
        }
        let demand = self.advance_demands(source,body,maximum_work_units)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !super::retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,effect);turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.phase==0&&self.inner.output_ready(){self.phase=5;}
        match self.phase {
            0 => {self.child_pending=10;Ok(RetainedCloneStep::Progress(Default::default()))},
            5 => {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let nested=super::nested_retained_clone_grant(grant)?;let Some(progress)=super::take_child_output(&mut self.inner,&mut self.chunks,nested,effect)? else{return Ok(RetainedCloneStep::Progress(Default::default()))};
                
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,progress,"paged UTF-8 chunk cursor take")?))
            }
            1 => {
                if !self.inner.terminal_is_empty() {
                    if !self.inner.close_started(){let turn=self.inner.begin_close_paid(effect);return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
                    self.child_pending=11;return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 2;if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged clone exceeded original Work").into())}
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            }
            2 => {
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < size_of::<usize>() {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged clone exceeded original Work").into())}self.output = Some(PagedUtf8::from_retained_chunks_cloned(self.chunks.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone lost its chunks"))?, source.get().len()));
                self.phase = 3;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: 0, released_bytes: 0 }))
            }
            3 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "paged UTF-8 retained clone cursor is spent")).into()),
        }
    })()}
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if !self.closing{return Ok(Default::default());}
        if !self.inner.terminal_is_empty(){return super::nested_retained_clone_close_demands::<PagedList<String,N>>(&self.inner,body);}
        if self.chunks.is_some(){return self.close.next_owner_demands::<PagedList<String,N>>(true,&self.source,body);}
        self.close.next_owner_demands::<PagedUtf8<N>>(self.output.is_some(),&self.source,body)
    }
}
impl<const N: usize> RetainedCloneCursor<PagedUtf8<N>> for PagedUtf8Cursor<N> {
    fn output_ready(&self) -> bool { !self.closing && self.phase == 3 && self.output.is_some() }
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{PagedUtf8Cursor::close_demands(self,body)}

    fn advance_demands(&self, source: RetainedCloneRef<'_, PagedUtf8<N>>, body:usize,maximum_work_units:u64) -> Result<crate::RetirementDemand, ValueError> {
        if self.closing || self.phase == 4 { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "paged text clone is closing or spent")); }
        if !source.check_binding(&self.source)? { return Ok(crate::RetirementDemand { copy_bytes: source.binding_copy_bytes(), depth: 1, ..Default::default() }); }
        if self.output_ready() { return Ok(Default::default()); }
        match if self.phase==0&&self.inner.output_ready(){5}else{self.phase} {
            0 => super::nested_retained_clone_demands::<PagedList<String,N>>(&self.inner,source.project(1,PagedUtf8::retained_chunks),body,maximum_work_units),
            5 => super::nested_retained_clone_take_demands(&self.inner),
            1 if !self.inner.terminal_is_empty() => super::nested_retained_clone_close_demands::<PagedList<String,N>>(&self.inner,body),
            1 => Ok(crate::RetirementDemand { depth:1,..Default::default() }),
            2 => Ok(crate::RetirementDemand { copy_bytes:size_of::<PagedUtf8<N>>(),depth:1,..Default::default() }),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"paged text clone phase invalid")),
        }
    }


    fn native_terminal_release(&self)->Option<crate::retained_clone::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{crate::retained_clone::RetainedCursorRelease::native(|cursor:&Self|cursor.terminal_is_empty())})}
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,PagedUtf8<N>>,body:usize,maximum:u64)->Result<u64,ValueError>{
        if self.closing||self.phase==4{return Err(ValueError::invariant("paged clone is closing or spent"))}
        if maximum==0||self.output_ready(){return Ok(0)}
        if self.source.is_none(){return source.binding_work_demand(&self.source)}
        match if self.phase==0&&self.inner.output_ready(){5}else{self.phase}{
            0=>self.inner.advance_work_demand(source.project(1,PagedUtf8::retained_chunks),body,maximum),
            5=>self.inner.take_work_demand(),
            1 if !self.inner.terminal_is_empty()=>if !self.inner.close_started(){self.inner.begin_close_work_demand()}else{self.inner.close_work_demand(body)},
            1|2=>Ok(1),
            _=>Err(ValueError::invariant("paged clone phase invalid")),
        }
    }
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.closing{return Ok(0)}
        if !self.inner.terminal_is_empty(){return if !self.inner.close_started(){self.inner.begin_close_work_demand()}else{self.inner.close_work_demand(body)}}
        if self.chunks.is_some(){return self.close.next_owner_work::<PagedList<String,N>>(true,&self.source,body)}
        self.close.next_owner_work::<PagedUtf8<N>>(self.output.is_some(),&self.source,body)
    }
    fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<PagedUtf8<N>,u8>()}
    fn take<'reply>(&mut self,output:&mut Option<PagedUtf8<N>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.phase,4,effect,reply,PagedUtf8<N>,u8)}

    

    

    fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,PagedUtf8<N>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if self.close_route.is_some(){return Err(ValueError::invariant("paged clone has an unresolved original invocation"))}
        let step=self.advance_paid(source,body,maximum,effect);
        match std::mem::take(&mut self.child_pending){
            10=>{let maximum=maximum.min(effect.maximum_work_units());routed!(self,10,body,None,effect,reply,|child|self.inner.advance_into(source.project(1,PagedUtf8::retained_chunks),body,maximum,effect,child))}
            11=>routed!(self,11,body,None,effect,reply,|child|self.inner.close_into(body,effect,child)),
            _=>Ok(reply.complete(step)),
        }
    }
    fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,PagedUtf8<N>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        let maximum=maximum.min(effect.maximum_work_units());
        recovered!(self,body,None,effect,reply,|route,child|match route{10=>self.inner.recover_advance_into(source.project(1,PagedUtf8::retained_chunks),body,maximum,effect,child),11=>self.inner.recover_close_into(body,effect,child),_=>Err(ValueError::invariant("paged clone route changed"))})
    }
    fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{let turn=self.begin_close_paid(effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
    fn recover_begin_close_into<'r>(&mut self,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Ok(reply.pending())}
    fn requires_original_birth(&self)->bool{if self.close_route.is_some(){return false}if !self.inner.terminal_is_empty(){self.inner.requires_original_birth()}else if !self.close.is_empty(){self.close.requires_original_birth()}else if self.chunks.is_some()||self.output.is_some(){false}else{RetainedCloneBinding::requires_original_birth(&self.source)}}
    fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if self.close_route.is_some()||self.chunks.is_some()||self.output.is_some()&&self.close.is_empty()&&self.inner.terminal_is_empty(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original Birth is not the selected cursor frontier").into()),work:0})}if !self.inner.terminal_is_empty(){self.inner.close_original_birth_step(original,grant)}else if !self.close.is_empty(){self.close.step_original_birth(original,grant)}else{RetainedCloneBinding::close_original_birth_step(&mut self.source,original,grant)}}
    fn requires_original_control(&self)->bool{if let Some((_,route,_,_))=self.close_route{return matches!(route,3|4|6)}if !self.inner.terminal_is_empty(){self.inner.requires_original_control()}else if !self.close.is_empty(){self.close.requires_original_control()}else if self.chunks.is_some()||self.output.is_some(){false}else{RetainedCloneBinding::requires_original_control(&self.source)}}
    fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        let grant=effect.grant();
        if !self.closing{return Err(ValueError::invariant("paged owner must begin close before granted retirement"))}
        if self.requires_original_birth()||self.requires_original_control(){return Err(ValueError::invariant("paged owner requires selected original close"))}
        if grant.maximum_items==0||!super::retained_clone_grant_admits(grant,PagedUtf8Cursor::close_demands(self,grant.maximum_copy_bytes)?){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
        if !self.inner.terminal_is_empty(){if !self.inner.close_started(){let turn=self.inner.begin_close_paid(effect);return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}return routed!(self,5,body,None,effect,reply,|child|self.inner.close_into(body,effect,child))}
        if !self.close.is_empty(){return routed!(self,1,body,None,effect,reply,|child|self.close.step_into(body,effect,child))}
        if let Some(step)=self.close.begin_granted(&mut self.chunks,effect).result?{return Ok(reply.complete(Ok(step)))}
        if let Some(step)=self.close.begin_granted(&mut self.output,effect).result?{return Ok(reply.complete(Ok(step)))}
        routed!(self,2,body,None,effect,reply,|child|RetainedCloneBinding::close_into(&mut self.source,body,effect,child))
    }
    fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        recovered!(self,body,None,effect,reply,|route,child|match route{1=>self.close.recover_step_into(body,effect,child),2=>RetainedCloneBinding::recover_close_into(&mut self.source,body,effect,child),5=>self.inner.recover_close_into(body,effect,child),_=>Err(ValueError::invariant("paged cursor close route changed"))})
    }
    fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if self.requires_original_birth(){return Err(ValueError::invariant("paged cursor Birth must settle before original close"))}
        if !self.requires_original_control(){return self.close_into(body,effect,reply)}
        let key=Some(original_key(original,effect)?);
        if !self.inner.terminal_is_empty(){return routed!(self,6,body,key,effect,reply,|child|self.inner.close_original_into(original,body,effect,child))}
        if !self.close.is_empty(){return routed!(self,3,body,key,effect,reply,|child|self.close.step_original_into(original,body,effect,child))}
        routed!(self,4,body,key,effect,reply,|child|RetainedCloneBinding::close_original_into(&mut self.source,original,body,effect,child))
    }
    fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if matches!(self.close_route,Some((_,1|2|5,_,_))){return self.recover_close_into(body,effect,reply)}
        let key=Some(original_key(original,effect)?);
        recovered!(self,body,key,effect,reply,|route,child|match route{6=>self.inner.recover_close_original_into(original,body,effect,child),3=>self.close.recover_step_original_into(original,body,effect,child),4=>RetainedCloneBinding::recover_close_original_into(&mut self.source,original,body,effect,child),_=>Err(ValueError::invariant("paged cursor close route changed"))})
    }

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.inner.terminal_is_empty() && self.chunks.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() && self.close_route.is_none() }
}

impl<const N: usize> RetireOwned for PagedUtf8<N> {
    type RetirementCursor=<PagedList<String,N> as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{
        self.into_retained_chunks().retirement_seed()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<crate::retirement::PagedListRetirement<String, N>>()) }
    fn controlled_retirement_supported() -> bool { true }
}

impl<const N: usize> RetainedClone for PagedUtf8<N> {
    type Cursor = PagedUtf8Cursor<N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedUtf8Cursor::default()
    }
}

/// 🔎️ Compares original paged UTF-8 identifiers with stable chunk and byte offsets.
#[derive(Default)]
pub struct PagedUtf8BoundedOrdCursor<const N: usize> {
    initialized:bool,
    #[doc(hidden)] pub left: Option<RetainedCloneBinding>,
    #[doc(hidden)] pub right: Option<RetainedCloneBinding>,
    left_chunk: usize,
    right_chunk: usize,
    left_byte: usize,
    right_byte: usize,
    complete: Option<std::cmp::Ordering>,
    closing: bool,
    last: super::ordered_map::BoundedOrdProgress,
    total: super::ordered_map::BoundedOrdProgress,
    taken: bool,
}

impl<const N: usize> PagedUtf8BoundedOrdCursor<N> {
 fn binding(&self)->&Option<RetainedCloneBinding>{if self.left.is_some(){&self.left}else{&self.right}}
 fn binding_mut(&mut self)->&mut Option<RetainedCloneBinding>{if self.left.is_some(){&mut self.left}else{&mut self.right}}
 fn selected(&self,l:RetainedCloneRef<'_,PagedUtf8<N>>,r:RetainedCloneRef<'_,PagedUtf8<N>>,g:super::ordered_map::BoundedOrdGrant,body:usize)->Result<(crate::RetirementDemand,u64),ValueError>{
  if self.closing{return Err(ValueError::invariant("paged identifier comparator is closing"))}
  l.binding_demand(&self.left)?;r.binding_demand(&self.right)?;
  if self.complete.is_some()||self.taken||g.maximum_items==0{return Ok((Default::default(),0))}
  if let Some(d)=l.binding_demand(&self.left)?{return Ok((d,l.binding_work_demand(&self.left)?))}
  if let Some(d)=r.binding_demand(&self.right)?{return Ok((d,r.binding_work_demand(&self.right)?))}
  let copy=2*size_of::<super::ordered_map::BoundedOrdProgress>()+4*size_of::<usize>()+size_of::<Option<std::cmp::Ordering>>()+size_of::<bool>();
  let work=RetainedCloneRef::<PagedUtf8<N>>::check_binding_work(&self.left).checked_add(RetainedCloneRef::<PagedUtf8<N>>::check_binding_work(&self.right)).and_then(|n|n.checked_add(g.maximum_bytes.min(body.max(1)) as u64)).and_then(|n|n.checked_add(4)).ok_or_else(||ValueError::invariant("comparison Work overflow"))?;
  Ok((crate::RetirementDemand{copy_bytes:copy,depth:1,..Default::default()},work))
 }
 fn permits(effect:&crate::RetainedAllocationEffect<'_>,d:crate::RetirementDemand,work:u64)->Result<bool,ValueError>{Ok(super::retained_clone_grant_admits(effect.grant(),d)&&effect.remaining_work_units()?>=work)}
 fn note(&mut self,p:super::ordered_map::BoundedOrdProgress){self.last=p;self.total.compared_items+=p.compared_items;self.total.compared_bytes+=p.compared_bytes}
    fn compare_paid(&mut self, left: RetainedCloneRef<'_, PagedUtf8<N>>, right: RetainedCloneRef<'_, PagedUtf8<N>>, grant: super::ordered_map::BoundedOrdGrant, effect:&mut crate::RetainedAllocationEffect<'_>) -> Result<super::ordered_map::BoundedOrdStep, ValueError> {(||{
        use super::ordered_map::{BoundedOrdProgress, BoundedOrdStep};
        if self.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "paged identifier comparator is closing")); }
        let first = !self.initialized;
        if first && grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
        if let Some(progress)=left.bind(&mut self.left,effect).result?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(progress)=right.bind(&mut self.right,effect).result?{return Ok(BoundedOrdStep::Authority(progress));}
        if let Some(ordering) = self.complete { return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress::default() }); }
        if first {if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged identifier comparison exceeded original Work"))}self.initialized=true; return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 })); }
        let left = left.get().retained_chunks();
        let right = right.get().retained_chunks();
        if left.get(self.left_chunk).is_some_and(|chunk| chunk.len() == self.left_byte) {
            if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged identifier comparison exceeded original Work"))}self.left_chunk += 1;
            self.left_byte = 0;
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        if right.get(self.right_chunk).is_some_and(|chunk| chunk.len() == self.right_byte) {
            if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged identifier comparison exceeded original Work"))}self.right_chunk += 1;
            self.right_byte = 0;
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        let left_available = left.get(self.left_chunk).is_some();
        let right_available = right.get(self.right_chunk).is_some();
        if !left_available || !right_available {
            if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default())); }
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged identifier comparison exceeded original Work"))}let ordering = left_available.cmp(&right_available);
            self.complete = Some(ordering);
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: 0 } });
        }
        let mut progress = BoundedOrdProgress::default();
        while progress.compared_bytes < grant.maximum_bytes {
            let l = left.get(self.left_chunk).and_then(|chunk| chunk.as_bytes().get(self.left_byte)).copied();
            let r = right.get(self.right_chunk).and_then(|chunk| chunk.as_bytes().get(self.right_byte)).copied();
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged identifier comparison exceeded original Work"))}let ordering = match (l, r) {
                (None, None) | (None, Some(_)) | (Some(_), None) => {
                    if grant.maximum_items == 0 { return Ok(BoundedOrdStep::Progress(progress)); }
                    progress.compared_items = 1;
                    l.cmp(&r)
                }
                (Some(l), Some(r)) => {
                    progress.compared_bytes += 1;
                    self.left_byte += 1;
                    self.right_byte += 1;
                    if self.left_byte == left[self.left_chunk].len() { self.left_chunk += 1; self.left_byte = 0; }
                    if self.right_byte == right[self.right_chunk].len() { self.right_chunk += 1; self.right_byte = 0; }
                    if l == r { continue; }
                    l.cmp(&r)
                }
            };
            self.complete = Some(ordering);
            return Ok(BoundedOrdStep::Complete { ordering, progress });
        }
        Ok(BoundedOrdStep::Progress(progress))
    })()}
}
impl<const N: usize> super::ordered_map::BoundedOrdCursor<PagedUtf8<N>> for PagedUtf8BoundedOrdCursor<N> {
 fn compare_demands(&self,l:RetainedCloneRef<'_,PagedUtf8<N>>,r:RetainedCloneRef<'_,PagedUtf8<N>>,g:super::ordered_map::BoundedOrdGrant,b:usize)->Result<crate::RetirementDemand,ValueError>{Ok(self.selected(l,r,g,b)?.0)}
 fn compare_work_demand(&self,l:RetainedCloneRef<'_,PagedUtf8<N>>,r:RetainedCloneRef<'_,PagedUtf8<N>>,g:super::ordered_map::BoundedOrdGrant,b:usize)->Result<u64,ValueError>{Ok(self.selected(l,r,g,b)?.1)}
 fn compare_into<'r>(&mut self,l:RetainedCloneRef<'_,PagedUtf8<N>>,r:RetainedCloneRef<'_,PagedUtf8<N>>,g:super::ordered_map::BoundedOrdGrant,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
  use super::ordered_map::BoundedOrdStep;
  let(d,work)=match self.selected(l,r,g,body){Ok(selected)=>selected,Err(error)=>return Ok(reply.complete(Err(error.into())))};
  if self.complete.is_some()||self.taken{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  if work==0||!Self::permits(effect,d,work)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  let paid=|p:super::ordered_map::BoundedOrdProgress|if p==Default::default(){RetainedCloneProgress::default()}else{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}};
  let step=match self.compare_paid(l,r,g,effect){Ok(step)=>step,Err(error)=>return Ok(reply.complete(Err(error.into())))};
  let out=match step{BoundedOrdStep::Authority(p)=>RetainedCloneStep::Progress(p),BoundedOrdStep::Progress(p)=>{self.note(p);RetainedCloneStep::Progress(paid(p))},BoundedOrdStep::Complete{progress:p,..}=>{self.note(p);RetainedCloneStep::Complete(paid(p))}};
  Ok(reply.complete(Ok(out)))
 }
 fn recover_compare_into<'r>(&mut self,_l:RetainedCloneRef<'_,PagedUtf8<N>>,_r:RetainedCloneRef<'_,PagedUtf8<N>>,_g:super::ordered_map::BoundedOrdGrant,_body:usize,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Err(ValueError::invariant("paged comparison has no unrecorded callback"))}
 fn ordering(&self)->Option<std::cmp::Ordering>{self.complete}
 fn comparison_progress(&self)->super::ordered_map::BoundedOrdProgress{self.last}
 fn comparison_totals(&self)->super::ordered_map::BoundedOrdProgress{self.total}
 fn take_ordering_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*size_of::<Option<std::cmp::Ordering>>()+size_of::<bool>(),depth:1,..Default::default()}}
 fn take_ordering_work_demand(&self)->u64{1}
 fn take_ordering_into<'r>(&mut self,output:&mut Option<std::cmp::Ordering>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{if output.is_some()||self.complete.is_none(){return Err(ValueError::invariant("paged ordering output is not vacant and ready"))}let d=self.take_ordering_demand();if !Self::permits(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(ValueError::invariant("ordering transfer lost original Work"))}*output=self.complete.take();self.taken=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 fn recover_take_ordering_into<'r>(&mut self,_output:&mut Option<std::cmp::Ordering>,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Err(ValueError::invariant("paged ordering transfer has no unrecorded callback"))}
 fn close_started(&self)->bool{self.closing}
 fn begin_close_demand(&self)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:usize::from(!self.closing)*size_of::<bool>(),depth:usize::from(!self.closing),..Default::default()}}
 fn begin_close_work_demand(&self)->u64{u64::from(!self.closing)}
 fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{if self.closing{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let d=self.begin_close_demand();if !Self::permits(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(ValueError::invariant("paged close selection lost original Work"))}self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 fn recover_begin_close_into<'r>(&mut self,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Err(ValueError::invariant("paged close selection has no unrecorded callback"))}
 fn close_demands(&self,b:usize)->Result<crate::RetirementDemand,ValueError>{RetainedCloneBinding::close_demand(self.binding(),b)}
 fn close_work_demand(&self,b:usize)->Result<u64,ValueError>{RetainedCloneBinding::close_work_demand(self.binding(),b)}
 fn close_into<'r>(&mut self,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{if !self.closing{return Err(ValueError::invariant("comparator must begin paid close"))}RetainedCloneBinding::close_into(self.binding_mut(),b,e,r.progress())}
 fn recover_close_into<'r>(&mut self,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{RetainedCloneBinding::recover_close_into(self.binding_mut(),b,e,r.progress())}
 fn close_original_into<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{if !self.closing{return Err(ValueError::invariant("comparator must begin paid close"))}RetainedCloneBinding::close_original_into(self.binding_mut(),o,b,e,r.progress())}
 fn recover_close_original_into<'r>(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,b:usize,e:&mut crate::RetainedAllocationEffect<'_>,r:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{RetainedCloneBinding::recover_close_original_into(self.binding_mut(),o,b,e,r.progress())}
 fn requires_original_birth(&self)->bool{RetainedCloneBinding::requires_original_birth(self.binding())}
 fn requires_original_control(&self)->bool{RetainedCloneBinding::requires_original_control(self.binding())}
 fn close_original_birth_step(&mut self,o:&mut crate::OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->crate::OriginalRetirementPoll{RetainedCloneBinding::close_original_birth_step(self.binding_mut(),o,g)}
 fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
}

impl<const N: usize> super::ordered_map::BoundedOrd for PagedUtf8<N> {
    type Cursor = PagedUtf8BoundedOrdCursor<N>;
    fn bounded_ord_birth_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:2*size_of::<Self::Cursor>(),depth:1,..Default::default()}}
    fn bounded_ord_birth_work_demand()->u64{1}
    fn admit_bounded_ord_cursor_into<'r>(output:&mut Option<Self::Cursor>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{if output.is_some(){return Err(ValueError::invariant("bounded comparison output is occupied"))}let d=Self::bounded_ord_birth_demand();if !PagedUtf8BoundedOrdCursor::<N>::permits(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(ValueError::invariant("paged comparison admission lost original Work"))}*output=Some(PagedUtf8BoundedOrdCursor::default());Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
    fn recover_admit_bounded_ord_cursor_into<'r>(_output:&mut Option<Self::Cursor>,_effect:&mut crate::RetainedAllocationEffect<'_>,_reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Err(ValueError::invariant("paged cursor admission has no unrecorded callback"))}
}

pub struct PagedMapCursor<V: RetainedClone, const N: usize> {
    inner: <PagedList<(PagedUtf8<{usize::MAX}>, V), N> as RetainedClone>::Cursor,
    entries: Option<PagedList<(PagedUtf8<{usize::MAX}>, V), N>>,
    output: Option<PagedMap<V, N>>,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    closing: bool,
    close: RetainedCloneClose,
    close_route: CloseRoute,
    child_pending: u8,
}

impl<V: RetainedClone, const N: usize> Default for PagedMapCursor<V, N> {
    fn default() -> Self {
        Self { inner: PagedList::retained_clone_cursor(), entries: None, output: None, source: None, phase: 0, closing: false, close: RetainedCloneClose::default(), close_route: None, child_pending: 0 }
    }
}

impl<V:RetainedClone,const N:usize> PagedMapCursor<V,N> {
    fn advance_paid(&mut self, source: RetainedCloneRef<'_, PagedMap<V, N>>, body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep,crate::OriginalRetirementFailure> {let grant=effect.grant();let maximum_work_units=maximum_work_units.min(effect.maximum_work_units());(|| -> Result<RetainedCloneStep,crate::OriginalRetirementFailure>{
        if self.closing {
            return Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "paged map retained clone cursor is closing")).into());
        }
        let demand = self.advance_demands(source,body,maximum_work_units)?;
        if self.output_ready() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !super::retained_clone_grant_admits(grant,demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(progress)={let turn=source.bind(&mut self.source,effect);turn.result}?{return Ok(RetainedCloneStep::Progress(progress));}
        if self.phase==0&&self.inner.output_ready(){self.phase=5;}
        match self.phase {
            0 => {self.child_pending=10;Ok(RetainedCloneStep::Progress(Default::default()))},
            5 => {
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                let nested=super::nested_retained_clone_grant(grant)?;let Some(progress)=super::take_child_output(&mut self.inner,&mut self.entries,nested,effect)? else{return Ok(RetainedCloneStep::Progress(Default::default()))};
                
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(nested,progress,"paged map entry cursor take")?))
            }
            1 => {
                if !self.inner.terminal_is_empty() {
                    if !self.inner.close_started(){let turn=self.inner.begin_close_paid(effect);return turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)}
                    self.child_pending=11;return Ok(RetainedCloneStep::Progress(Default::default()));
                }
                if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
                self.phase = 2;if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged clone exceeded original Work").into())}
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            }
            2 => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged clone exceeded original Work").into())}self.output = Some(PagedMap::from_retained_entries_cloned(self.entries.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "paged map retained clone lost its entries"))?));
                self.phase = 3;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
            }
            3 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err((ValueError::literal(ValueRefusalKind::InvariantViolated, "paged map retained clone cursor is spent")).into()),
        }
    })()}
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
        if !self.closing{return Ok(Default::default());}
        if !self.inner.terminal_is_empty(){return super::nested_retained_clone_close_demands::<PagedList<(PagedUtf8<{usize::MAX}>,V),N>>(&self.inner,body);}
        if self.entries.is_some(){return self.close.next_owner_demands::<PagedList<(PagedUtf8<{usize::MAX}>,V),N>>(true,&self.source,body);}
        self.close.next_owner_demands::<PagedMap<V,N>>(self.output.is_some(),&self.source,body)
    }
}
impl<V: RetainedClone, const N: usize> RetainedCloneCursor<PagedMap<V, N>> for PagedMapCursor<V, N> {
    fn output_ready(&self) -> bool { !self.closing && self.phase == 3 && self.output.is_some() }
    fn begin_close_demand(&self)->Result<crate::RetirementDemand,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_demand(self.closing))}
fn begin_close_work_demand(&self)->Result<u64,crate::ValueError>{Ok(crate::retained_clone::retained_close_start_work_demand(self.closing))}
fn begin_close_paid(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneProgress>{crate::retained_clone::start_retained_close(&mut self.closing,effect)}
fn close_started(&self) -> bool { self.closing }
    fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{PagedMapCursor::close_demands(self,body)}

    fn advance_demands(&self, source: RetainedCloneRef<'_, PagedMap<V,N>>, body:usize,maximum_work_units:u64) -> Result<crate::RetirementDemand, ValueError> {
        if self.closing || self.phase == 4 { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "paged map clone is closing or spent")); }
        if !source.check_binding(&self.source)? { return Ok(crate::RetirementDemand { copy_bytes: source.binding_copy_bytes(), depth: 1, ..Default::default() }); }
        if self.output_ready() { return Ok(Default::default()); }
        match if self.phase==0&&self.inner.output_ready(){5}else{self.phase} {
            0 => super::nested_retained_clone_demands::<PagedList<(PagedUtf8<{usize::MAX}>,V),N>>(&self.inner,source.project(1,PagedMap::retained_entries),body,maximum_work_units),
            5 => super::nested_retained_clone_take_demands(&self.inner),
            1 if !self.inner.terminal_is_empty() => super::nested_retained_clone_close_demands::<PagedList<(PagedUtf8<{usize::MAX}>,V),N>>(&self.inner,body),
            1 => Ok(crate::RetirementDemand { depth:1,..Default::default() }),
            2 => Ok(crate::RetirementDemand { copy_bytes:size_of::<PagedMap<V,N>>(),depth:1,..Default::default() }),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"paged map clone phase invalid")),
        }
    }


    fn native_terminal_release(&self)->Option<crate::retained_clone::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{crate::retained_clone::RetainedCursorRelease::native(|cursor:&Self|cursor.terminal_is_empty())})}
    fn advance_work_demand(&self,source:RetainedCloneRef<'_,PagedMap<V,N>>,body:usize,maximum:u64)->Result<u64,ValueError>{
        if self.closing||self.phase==4{return Err(ValueError::invariant("paged clone is closing or spent"))}
        if maximum==0||self.output_ready(){return Ok(0)}
        if self.source.is_none(){return source.binding_work_demand(&self.source)}
        match if self.phase==0&&self.inner.output_ready(){5}else{self.phase}{
            0=>self.inner.advance_work_demand(source.project(1,PagedMap::retained_entries),body,maximum),
            5=>self.inner.take_work_demand(),
            1 if !self.inner.terminal_is_empty()=>if !self.inner.close_started(){self.inner.begin_close_work_demand()}else{self.inner.close_work_demand(body)},
            1|2=>Ok(1),
            _=>Err(ValueError::invariant("paged clone phase invalid")),
        }
    }
    fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(self.output_ready()))}
    fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.closing{return Ok(0)}
        if !self.inner.terminal_is_empty(){return if !self.inner.close_started(){self.inner.begin_close_work_demand()}else{self.inner.close_work_demand(body)}}
        if self.entries.is_some(){return self.close.next_owner_work::<PagedList<(PagedUtf8<{usize::MAX}>,V),N>>(true,&self.source,body)}
        self.close.next_owner_work::<PagedMap<V,N>>(self.output.is_some(),&self.source,body)
    }
    fn take_demand(&self)->Result<crate::RetirementDemand,ValueError>{crate::retained_clone::retained_clone_cursor_take_demand::<PagedMap<V, N>,u8>()}
    fn take<'reply>(&mut self,output:&mut Option<PagedMap<V, N>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::retained_clone::OriginalEffectReply<'reply,RetainedCloneStep,ValueError>)->Result<crate::retained_clone::OriginalEffectReplyComplete<'reply,RetainedCloneStep,ValueError>,ValueError>{crate::retained_clone::take_retained_clone_cursor_output!(self.output_ready(),self.output,output,self.phase,4,effect,reply,PagedMap<V, N>,u8)}

    

    

    fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,PagedMap<V, N>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if self.close_route.is_some(){return Err(ValueError::invariant("paged clone has an unresolved original invocation"))}
        let step=self.advance_paid(source,body,maximum,effect);
        match std::mem::take(&mut self.child_pending){
            10=>{let maximum=maximum.min(effect.maximum_work_units());routed!(self,10,body,None,effect,reply,|child|self.inner.advance_into(source.project(1,PagedMap::retained_entries),body,maximum,effect,child))}
            11=>routed!(self,11,body,None,effect,reply,|child|self.inner.close_into(body,effect,child)),
            _=>Ok(reply.complete(step)),
        }
    }
    fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,PagedMap<V, N>>,body:usize,maximum:u64,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        let maximum=maximum.min(effect.maximum_work_units());
        recovered!(self,body,None,effect,reply,|route,child|match route{10=>self.inner.recover_advance_into(source.project(1,PagedMap::retained_entries),body,maximum,effect,child),11=>self.inner.recover_close_into(body,effect,child),_=>Err(ValueError::invariant("paged clone route changed"))})
    }
    fn begin_close_into<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{let turn=self.begin_close_paid(effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
    fn recover_begin_close_into<'r>(&mut self,_effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{Ok(reply.pending())}
    fn requires_original_birth(&self)->bool{if self.close_route.is_some(){return false}if !self.inner.terminal_is_empty(){self.inner.requires_original_birth()}else if !self.close.is_empty(){self.close.requires_original_birth()}else if self.entries.is_some()||self.output.is_some(){false}else{RetainedCloneBinding::requires_original_birth(&self.source)}}
    fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if self.close_route.is_some()||self.entries.is_some()||self.output.is_some()&&self.close.is_empty()&&self.inner.terminal_is_empty(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::invariant("original Birth is not the selected cursor frontier").into()),work:0})}if !self.inner.terminal_is_empty(){self.inner.close_original_birth_step(original,grant)}else if !self.close.is_empty(){self.close.step_original_birth(original,grant)}else{RetainedCloneBinding::close_original_birth_step(&mut self.source,original,grant)}}
    fn requires_original_control(&self)->bool{if let Some((_,route,_,_))=self.close_route{return matches!(route,3|4|6)}if !self.inner.terminal_is_empty(){self.inner.requires_original_control()}else if !self.close.is_empty(){self.close.requires_original_control()}else if self.entries.is_some()||self.output.is_some(){false}else{RetainedCloneBinding::requires_original_control(&self.source)}}
    fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        let grant=effect.grant();
        if !self.closing{return Err(ValueError::invariant("paged owner must begin close before granted retirement"))}
        if self.requires_original_birth()||self.requires_original_control(){return Err(ValueError::invariant("paged owner requires selected original close"))}
        if grant.maximum_items==0||!super::retained_clone_grant_admits(grant,PagedMapCursor::close_demands(self,grant.maximum_copy_bytes)?){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
        if !self.inner.terminal_is_empty(){if !self.inner.close_started(){let turn=self.inner.begin_close_paid(effect);return Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}return routed!(self,5,body,None,effect,reply,|child|self.inner.close_into(body,effect,child))}
        if !self.close.is_empty(){return routed!(self,1,body,None,effect,reply,|child|self.close.step_into(body,effect,child))}
        if let Some(step)=self.close.begin_granted(&mut self.entries,effect).result?{return Ok(reply.complete(Ok(step)))}
        if let Some(step)=self.close.begin_granted(&mut self.output,effect).result?{return Ok(reply.complete(Ok(step)))}
        routed!(self,2,body,None,effect,reply,|child|RetainedCloneBinding::close_into(&mut self.source,body,effect,child))
    }
    fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        recovered!(self,body,None,effect,reply,|route,child|match route{1=>self.close.recover_step_into(body,effect,child),2=>RetainedCloneBinding::recover_close_into(&mut self.source,body,effect,child),5=>self.inner.recover_close_into(body,effect,child),_=>Err(ValueError::invariant("paged cursor close route changed"))})
    }
    fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if self.requires_original_birth(){return Err(ValueError::invariant("paged cursor Birth must settle before original close"))}
        if !self.requires_original_control(){return self.close_into(body,effect,reply)}
        let key=Some(original_key(original,effect)?);
        if !self.inner.terminal_is_empty(){return routed!(self,6,body,key,effect,reply,|child|self.inner.close_original_into(original,body,effect,child))}
        if !self.close.is_empty(){return routed!(self,3,body,key,effect,reply,|child|self.close.step_original_into(original,body,effect,child))}
        routed!(self,4,body,key,effect,reply,|child|RetainedCloneBinding::close_original_into(&mut self.source,original,body,effect,child))
    }
    fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{
        if matches!(self.close_route,Some((_,1|2|5,_,_))){return self.recover_close_into(body,effect,reply)}
        let key=Some(original_key(original,effect)?);
        recovered!(self,body,key,effect,reply,|route,child|match route{6=>self.inner.recover_close_original_into(original,body,effect,child),3=>self.close.recover_step_original_into(original,body,effect,child),4=>RetainedCloneBinding::recover_close_original_into(&mut self.source,original,body,effect,child),_=>Err(ValueError::invariant("paged cursor close route changed"))})
    }

    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.depth)}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(0)?.release_bytes)}
    fn terminal_is_empty(&self) -> bool { self.closing && self.inner.terminal_is_empty() && self.entries.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none() && self.close_route.is_none() }
}

impl<V: RetireOwned + Send + Sync + 'static, const N: usize> RetireOwned for PagedMap<V, N> {
    type RetirementCursor=<PagedList<(PagedUtf8<{usize::MAX}>,V),N> as RetireOwned>::RetirementCursor;
    fn retirement_seed(self)->Self::RetirementCursor{
        self.into_retained_entries().retirement_seed()
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<crate::retirement::PagedListRetirement<(PagedUtf8<{usize::MAX}>, V), N>>()) }
    fn controlled_retirement_supported() -> bool { crate::retirement::support::controlled_retirement_supported::<Self>() }
    fn controlled_retirement_supported_in(path: &crate::retirement::RetirementSupport<'_>) -> bool { path.supports::<V>() }
}

impl<V: RetainedClone, const N: usize> RetainedClone for PagedMap<V, N> {
    type Cursor = PagedMapCursor<V, N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedMapCursor::default()
    }
}

