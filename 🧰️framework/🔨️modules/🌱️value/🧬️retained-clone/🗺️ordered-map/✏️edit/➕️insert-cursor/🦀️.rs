//! ➕️ Native insertion retains the original workspace through lookup, movement and cancellation.
use super::*;
use super::edit_input::{EditInput,EditWorkspace};
use super::edit_retirement::{CloseRoute,close_key};
use crate::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementCloseLoan};
use std::mem::ManuallyDrop;
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>;
type Effect<'a>=crate::RetainedAllocationEffect<'a>;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum RetainedOrderedMapInsertStep{Progress(RetainedOrderedMapInsertProgress),Complete{ordinal:usize,progress:RetainedOrderedMapInsertProgress}}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedOrderedMapInsertGrant{pub retirement:RetainedCloneGrant,pub comparison:BoundedOrdGrant,pub maximum_moved_items:usize,pub maximum_moved_bytes:usize,pub maximum_capacity_bytes:usize}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct RetainedOrderedMapInsertProgress{pub retirement:RetainedCloneProgress,pub comparison:BoundedOrdProgress,pub moved_items:usize,pub moved_bytes:usize,pub retained_capacity_bytes:usize,pub released_bytes:usize}
impl RetainedOrderedMapInsertProgress{pub fn fits(self,g:RetainedOrderedMapInsertGrant)->bool{self.retirement.fits(g.retirement)&&self.comparison.fits(g.comparison)&&self.moved_items<=g.maximum_moved_items&&self.moved_bytes<=g.maximum_moved_bytes&&self.retained_capacity_bytes<=g.maximum_capacity_bytes&&self.released_bytes<=g.retirement.maximum_release_bytes}}
pub(super) struct InsertState<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{
 input:EditInput<K,V>,workspace:Option<EditWorkspace<K,V>>,lookup_result:Option<RetainedOrderedMapLookup>,
 pub(super) map:Option<RetainedOrderedMap<K,V>>,pub(super) output:Option<RetainedOrderedMap<K,V>>,pub(super) key:Option<K>,pub(super) value:Option<V>,pub(super) target:Option<usize>,
 shift_page:Option<usize>,old_directory:Option<Vec<Vec<(K,V)>>>,new_directory:Option<Vec<Vec<(K,V)>>>,directory_index:usize,
 pub(super) phase:u8,pub(super) spent:bool,pub(super) closing:bool,pub(super) close:RetainedCloneClose,close_route:Option<CloseRoute>,movement:RetainedOrderedMapInsertProgress,
}
/// 🧯️ Owns every unpublished page and candidate until its actual funded output or retirement.
pub struct RetainedOrderedMapInsertCursor<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync>{pub(super) state:ManuallyDrop<InsertState<K,V>>}
impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> RetainedOrderedMapInsertCursor<K,V>{
 pub fn new(map:RetainedOrderedMap<K,V>,key:K,value:V)->Self{Self{state:ManuallyDrop::new(InsertState{input:EditInput::new(map,key,Some(value)),workspace:None,lookup_result:None,map:None,output:None,key:None,value:None,target:None,shift_page:None,old_directory:None,new_directory:None,directory_index:0,phase:0,spent:false,closing:false,close:RetainedCloneClose::vacant(),close_route:None,movement:Default::default()})}}
 fn invalid()->ValueError{ValueError::invariant("native insertion changed its original ownership")}
 fn overflow()->ValueError{ValueError::literal(crate::ValueRefusalKind::OwnershipLimit,"native insertion extent overflow")}
 fn fixed(bytes:usize)->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()}}
 fn quote(d:crate::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
 fn unpack_demand()->crate::RetirementDemand{Self::fixed(size_of::<Option<EditWorkspace<K,V>>>()+size_of::<EditWorkspace<K,V>>()+size_of::<Option<RetainedOrderedMap<K,V>>>()+size_of::<Option<K>>()+size_of::<Option<V>>()+size_of::<Option<RetainedOrderedMapLookup>>()+size_of::<Option<usize>>()+size_of::<u8>())}
 fn prefix_demand(&self,comparison:BoundedOrdGrant,body:usize)->Result<crate::RetirementDemand,ValueError>{let s=&*self.state;if s.workspace.is_some(){Ok(Self::unpack_demand())}else if s.input.ready(){s.input.take_demand()}else{s.input.next_demand(comparison,body)}}
 fn funded(effect:&Effect<'_>,d:crate::RetirementDemand,work:u64)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),d)&&effect.remaining_work_units()?>=work)}
 pub fn birth_complete(&self)->bool{self.state.phase!=0||self.state.input.birth_complete()}
    pub fn next_advance_demands(&self, maximum_copy_bytes: usize) -> Result<RetainedOrderedMapInsertGrant, ValueError> {
        let state = &*self.state;
        let overflow = || ValueError::literal(crate::ValueRefusalKind::OwnershipLimit, "ordered-map insertion next-action extent overflow");
        let mut grant = RetainedOrderedMapInsertGrant::default();
        if state.closing || state.spent {
            return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion cannot advance after close or transfer"));
        }
        match state.phase {
            0 => {grant.comparison=BoundedOrdGrant{maximum_items:1,maximum_bytes:maximum_copy_bytes.min(64)};grant.retirement=Self::quote(self.prefix_demand(grant.comparison,maximum_copy_bytes)?);}
            1 => {
                let map = state.map.as_ref().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no map"))?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 && map.pages.len() < map.pages.capacity() {
                    grant.maximum_capacity_bytes = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
                } else { grant.maximum_moved_items = 1; }
            }
            2 => {
                let ordinal = state.target.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no target"))?;
                let shift = state.shift_page.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no shift page"))?;
                if shift == ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY {grant.maximum_moved_items=1;} else {
                    let page = state.map.as_ref().and_then(|map| map.pages.get(shift)).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no shift owner"))?;
                    grant.maximum_moved_items = page.len().checked_add(1).ok_or_else(overflow)?;
                    grant.maximum_moved_bytes = grant.maximum_moved_items.checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
                }
            }
            3 => {
                let ordinal = state.target.ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no target"))?;
                let page = state.map.as_ref().and_then(|map| map.pages.get(ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY)).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no target owner"))?;
                grant.maximum_moved_items = page.len().saturating_sub(ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY).checked_add(1).ok_or_else(overflow)?;
                grant.maximum_moved_bytes = grant.maximum_moved_items.checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
            }
            14 => {
                let tail = state.map.as_ref().and_then(|map| map.pages.last()).ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has no tail page"))?;
                grant.maximum_capacity_bytes = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
                grant.maximum_moved_items = tail.len().max(1);
                grant.maximum_moved_bytes = tail.len().checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
                grant.retirement.maximum_release_bytes = tail.capacity().checked_mul(size_of::<(K,V)>()).ok_or_else(overflow)?;
            }
            10 => {
                let map = state.map.as_ref().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map directory quote has no map"))?;
                grant.maximum_capacity_bytes = map.pages.len().checked_mul(2).and_then(|n| n.checked_add(1)).and_then(|n| n.checked_mul(size_of::<Vec<(K,V)>>())).ok_or_else(overflow)?;
            }
            11 | 13 => { grant.maximum_moved_items = 1; grant.maximum_moved_bytes = size_of::<Vec<Vec<(K,V)>>>(); }
            12 => {
                let old = state.old_directory.as_ref().ok_or_else(|| ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map directory quote has no original owner"))?;
                grant.maximum_moved_items=1;if state.directory_index != old.len() {grant.maximum_moved_bytes=size_of::<Vec<(K,V)>>();}
            }
            4 | 255 => {}
            _ => return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion quote has an invalid phase")),
        }
        Ok(grant)
    }
 pub fn advance_demands(&self,body:usize,grant:RetainedOrderedMapInsertGrant)->Result<crate::RetirementDemand,ValueError>{let s=&*self.state;if s.closing||s.spent||s.close_route.is_some(){return Err(Self::invalid())}if s.phase==0{return self.prefix_demand(grant.comparison,body)}if s.phase==4||s.phase==255{return Ok(Default::default())}let plan=self.next_advance_demands(body)?;let metadata=match s.phase{1=>size_of::<Option<usize>>()+size_of::<u8>()+if plan.maximum_capacity_bytes!=0{3*size_of::<Vec<(K,V)>>()+size_of::<Option<usize>>()}else{0},2=>2*size_of::<Vec<(K,V)>>()+size_of::<Option<usize>>()+size_of::<u8>(),3=>size_of::<Vec<(K,V)>>()+size_of::<Option<K>>()+size_of::<Option<V>>()+size_of::<usize>()+2*size_of::<Option<RetainedOrderedMap<K,V>>>()+size_of::<u8>(),14=>3*size_of::<Vec<(K,V)>>()+size_of::<Option<usize>>()+size_of::<u8>(),10=>size_of::<Option<Vec<Vec<(K,V)>>>>()+size_of::<Option<usize>>()+size_of::<u8>(),11=>size_of::<Vec<Vec<(K,V)>>>()+size_of::<Option<Vec<Vec<(K,V)>>>>()+size_of::<u8>(),12=>2*size_of::<Vec<(K,V)>>()+size_of::<Vec<Vec<(K,V)>>>()+size_of::<usize>()+size_of::<u8>(),13=>size_of::<Vec<Vec<(K,V)>>>()+size_of::<Option<Vec<Vec<(K,V)>>>>()+size_of::<usize>()+size_of::<u8>(),4|255=>0,_=>return Err(Self::invalid())};let copy_bytes=plan.maximum_moved_bytes.checked_add(metadata).and_then(|n|n.checked_add(size_of::<RetainedOrderedMapInsertProgress>())).ok_or_else(Self::overflow)?;Ok(crate::RetirementDemand{copy_bytes,capacity_bytes:plan.maximum_capacity_bytes,release_bytes:plan.retirement.maximum_release_bytes,depth:usize::from(copy_bytes!=0||plan.maximum_capacity_bytes!=0),..Default::default()})}
 pub fn advance_work_demand(&self,body:usize,grant:RetainedOrderedMapInsertGrant)->Result<u64,ValueError>{self.advance_demands(body,grant)?;let s=&*self.state;if s.phase==0&&!s.workspace.is_some()&&!s.input.ready(){s.input.next_work_demand(grant.comparison,body)}else{Ok(u64::from(s.phase!=4&&s.phase!=255))}}
 pub fn advance_into<'r>(&mut self,body:usize,grant:RetainedOrderedMapInsertGrant,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.advance_dispatch(body,grant,None,effect,reply,false)}
 pub fn recover_advance_into<'r>(&mut self,body:usize,grant:RetainedOrderedMapInsertGrant,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.advance_dispatch(body,grant,None,effect,reply,true)}
 pub fn advance_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,grant:RetainedOrderedMapInsertGrant,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.advance_dispatch(body,grant,Some(original),effect,reply,false)}
 pub fn recover_advance_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,grant:RetainedOrderedMapInsertGrant,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.advance_dispatch(body,grant,Some(original),effect,reply,true)}
 fn advance_dispatch<'r>(&mut self,body:usize,grant:RetainedOrderedMapInsertGrant,original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut Effect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  self.state.input.validate(effect)?;if self.state.closing||self.state.spent||self.state.close_route.is_some(){return Err(Self::invalid())}if recover{if self.state.phase!=0{return Err(Self::invalid())}return self.state.input.advance_into(grant.comparison,body,original,effect,reply.progress(),true)}
  let demand=self.advance_demands(body,grant)?;let units=self.advance_work_demand(body,grant)?;if !Self::funded(effect,demand,units)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if self.state.phase==0{if !retained_clone_grant_admits(grant.retirement,demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let s=&mut*self.state;if let Some(workspace)=s.workspace.as_ref(){if workspace.value.is_none()||s.lookup_result.is_none(){return Err(Self::invalid())}if !effect.performed(1)?{return Err(Self::invalid())}let EditWorkspace{map,key,value}=s.workspace.take().expect("original insertion workspace");s.map=Some(map);s.key=Some(key);s.value=value;match s.lookup_result.take().expect("original lookup result"){RetainedOrderedMapLookup::Found(_)=>s.phase=255,RetainedOrderedMapLookup::Missing(ordinal)=>{s.target=Some(ordinal);s.phase=1;}}return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}if s.input.ready(){return s.input.take_into(&mut s.workspace,&mut s.lookup_result,effect,reply.progress())}return s.input.advance_into(grant.comparison,body,original,effect,reply.progress(),false)}
  if self.state.phase==255{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::InvalidValue,"retained ordered-map insertion refused a duplicate key").into())))}if self.state.phase==4{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let plan=self.next_advance_demands(body)?;if plan.maximum_moved_items>grant.maximum_moved_items||plan.maximum_moved_bytes>grant.maximum_moved_bytes||plan.maximum_capacity_bytes>grant.maximum_capacity_bytes{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if demand.capacity_bytes!=0&&effect.actual().is_some(){return Err(Self::invalid())}if self.state.phase==3&&(self.state.key.is_none()||self.state.value.is_none()){return Err(Self::invalid())}if !effect.performed(1)?{return Err(Self::invalid())}if demand.capacity_bytes!=0{*effect.actual()=Some(0)}let result=self.move_local(grant,effect);match result{Ok(step)=>{let(domain,complete)=match step{RetainedOrderedMapInsertStep::Progress(p)=>(p,false),RetainedOrderedMapInsertStep::Complete{progress,..}=>(progress,true)};self.state.movement=domain;let p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:domain.retained_capacity_bytes,released_bytes:domain.released_bytes};Ok(reply.complete(Ok(if complete{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})))},Err(error)=>{let p=RetainedCloneProgress{copied_items:1,copied_bytes:if demand.capacity_bytes!=0&&effect.actual()==&mut Some(0){size_of::<Option<usize>>()}else{demand.copy_bytes},retained_capacity_bytes:effect.actual().unwrap_or(0),..Default::default()};Ok(reply.complete(Err(error.with_retained_progress(p).into())))}}
 }
 fn move_local(&mut self,grant:RetainedOrderedMapInsertGrant,effect:&mut Effect<'_>)->Result<RetainedOrderedMapInsertStep,ValueError>{let state=&mut*self.state;match state.phase{
            255=>Err(ValueError::literal(crate::ValueRefusalKind::InvalidValue,"retained ordered-map insertion refused a duplicate key")),
            1 => {
                let map = state.map.as_mut().ok_or_else(|| Self::invalid())?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 {
                    if map.pages.len() == map.pages.capacity() {
                        if grant.maximum_moved_items==0{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}state.phase = 10;
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let capacity = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                    let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                    if !progress.fits(grant) {
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let mut page = Vec::new();
                    page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| ValueError::literal(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map insertion page allocation failed"))?;
                    let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                    *effect.actual()=Some(actual);
                    map.pages.push(page);
                    if actual > grant.maximum_capacity_bytes {state.shift_page=map.pages.len().checked_sub(1);state.phase=2;return Err(Self::invalid());}
                    state.shift_page = map.pages.len().checked_sub(1);
                    state.phase = 2;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }));
                }
                if grant.maximum_moved_items == 0 {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.shift_page = map.pages.len().checked_sub(1);
                state.phase = if map.pages.last().is_some_and(|tail| tail.capacity() == tail.len()) { 14 } else { 2 };
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()))
            }
            2 => {
                let map = state.map.as_mut().ok_or_else(|| Self::invalid())?;
                let ordinal = state.target.ok_or_else(|| Self::invalid())?;
                let target_page = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let shift_page = state.shift_page.ok_or_else(|| Self::invalid())?;
                if shift_page == target_page {
                    if grant.maximum_moved_items==0{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}state.phase = 3;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()));
                }
                let (left, right) = map.pages.split_at_mut(shift_page);
                let previous = left.get_mut(shift_page - 1).ok_or_else(|| Self::invalid())?;
                let current = right.first_mut().ok_or_else(|| Self::invalid())?;
                if current.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(Self::invalid());
                }
                let moved_items = current.len().checked_add(1).ok_or_else(Self::overflow)?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let entry = previous.pop().ok_or_else(|| Self::invalid())?;
                current.insert(0, entry);
                state.shift_page = Some(shift_page - 1);
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            3 => {
                let map = state.map.as_mut().ok_or_else(|| Self::invalid())?;
                let ordinal = state.target.ok_or_else(|| Self::invalid())?;
                let page_index = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let entry_index = ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let page = map.pages.get_mut(page_index).ok_or_else(|| Self::invalid())?;
                if page.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(Self::invalid());
                }
                let moved_items = page.len().saturating_sub(entry_index).checked_add(1).ok_or_else(Self::overflow)?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let new_len = map.len.checked_add(1).ok_or_else(Self::overflow)?;
                page.insert(
                    entry_index,
                    (
                        state.key.take().ok_or_else(|| Self::invalid())?,
                        state.value.take().ok_or_else(|| Self::invalid())?,
                    ),
                );
                map.len = new_len;
                state.output = state.map.take();
                state.phase = 4;
                Ok(RetainedOrderedMapInsertStep::Complete { ordinal, progress })
            }
            4 => Ok(RetainedOrderedMapInsertStep::Complete {
                ordinal: state.target.ok_or_else(|| Self::invalid())?,
                progress: RetainedOrderedMapInsertProgress::default(),
            }),
            14 => {
                let tail = state.map.as_mut().and_then(|map| map.pages.last_mut()).ok_or_else(|| Self::invalid())?;
                let capacity = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                let moved_bytes = tail.len().checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                let released_bytes = tail.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                let progress = RetainedOrderedMapInsertProgress { moved_items: tail.len(), moved_bytes, retained_capacity_bytes: capacity, released_bytes, ..Default::default() };
                if !progress.fits(grant) || grant.maximum_moved_items == 0 {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let mut page = Vec::new();
                page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| ValueError::literal(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map tail growth allocation failed"))?;
                let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(Self::overflow)?;
                *effect.actual() = Some(actual);
                page.append(tail);
                drop(std::mem::replace(tail, page));
                state.phase = 2;
                if actual > grant.maximum_capacity_bytes {
                    return Err(Self::invalid());
                }
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }))
            }
            10 => {
                let map = state.map.as_ref().ok_or_else(|| Self::invalid())?;
                let planned = map.pages.len().checked_mul(2).and_then(|value| value.checked_add(1)).ok_or_else(Self::overflow)?;
                let capacity = planned.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(Self::overflow)?;
                let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let mut directory = Vec::new();
                directory.try_reserve_exact(planned).map_err(|_| ValueError::literal(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory growth allocation failed"))?;
                let actual = directory.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(Self::overflow)?;
                *effect.actual()=Some(actual);
                    state.new_directory=Some(directory);
                if actual>grant.maximum_capacity_bytes{state.phase=11;return Err(Self::invalid());}
                state.phase = 11;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }))
            }
            11 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let map = state.map.as_mut().ok_or_else(|| Self::invalid())?;
                state.old_directory = Some(std::mem::take(&mut map.pages));
                state.phase = 12;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            12 => {
                let old = state.old_directory.as_mut().ok_or_else(|| Self::invalid())?;
                if state.directory_index == old.len() {
                    if grant.maximum_moved_items==0{return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));}state.phase = 13;
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<(K, V)>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.new_directory.as_mut().ok_or_else(|| Self::invalid())?.push(std::mem::take(&mut old[state.directory_index]));
                state.directory_index += 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            13 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                state.map.as_mut().ok_or_else(|| Self::invalid())?.pages =
                    state.new_directory.take().ok_or_else(|| Self::invalid())?;
                state.directory_index = 0;
                state.phase = 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            _ => Err(Self::invalid()),
        }
}
 pub fn output_ready(&self)->bool{self.state.output.is_some()&&!self.state.closing&&!self.state.spent}
 pub fn refused_workspace_ready(&self)->bool{self.state.phase==255&&self.state.map.is_some()&&!self.state.closing&&!self.state.spent}
 pub fn inserted_ordinal(&self)->Option<usize>{(self.state.phase==4).then_some(self.state.target).flatten()}
 #[cfg(test)]
 pub(super) fn original_key(&self)->Option<&K>{self.state.input.original_key()}
 pub fn comparison_progress(&self)->BoundedOrdProgress{self.state.input.comparison_progress()}
 pub fn movement_progress(&self)->RetainedOrderedMapInsertProgress{self.state.movement}
 pub fn take_output_demand(&self)->crate::RetirementDemand{Self::fixed(2*size_of::<Option<RetainedOrderedMap<K,V>>>()+size_of::<bool>())}
 pub fn take_output_work_demand(&self)->u64{1}
 pub fn take_into<'r>(&mut self,output:&mut Option<RetainedOrderedMap<K,V>>,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.take_dispatch(output,effect,reply,false)}
 pub fn take_refused_workspace_into<'r>(&mut self,output:&mut Option<RetainedOrderedMap<K,V>>,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.take_dispatch(output,effect,reply,true)}
 fn take_dispatch<'r>(&mut self,output:&mut Option<RetainedOrderedMap<K,V>>,effect:&mut Effect<'_>,reply:Reply<'r>,refused:bool)->Result<Complete<'r>,ValueError>{self.state.input.validate(effect)?;if output.is_some()||if refused{!self.refused_workspace_ready()}else{!self.output_ready()}{return Err(Self::invalid())}let d=self.take_output_demand();if !Self::funded(effect,d,1)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(Self::invalid())}*output=if refused{self.state.map.take()}else{self.state.output.take()};self.state.spent=true;Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 pub fn begin_close_demand(&self)->crate::RetirementDemand{Self::fixed(usize::from(!self.state.closing)*size_of::<bool>())}
 pub fn begin_close_work_demand(&self)->u64{u64::from(!self.state.closing)}
 pub fn begin_close_into<'r>(&mut self,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.state.input.validate(effect)?;if self.state.close_route.is_some()||self.state.input.has_pending(){return Err(Self::invalid())}let d=self.begin_close_demand();if !Self::funded(effect,d,self.begin_close_work_demand())?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if self.state.closing{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Err(Self::invalid())}self.state.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}
 pub fn close_demands(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{let s=&*self.state;if let Some(route)=s.close_route{if route.key.body!=body{return Err(Self::invalid())}return Ok(route.demand)}if !s.closing{if s.input.has_pending(){return Err(Self::invalid())}return Ok(self.begin_close_demand())}if !s.close.is_empty(){return s.close.next_owner_demands::<K>(false,&None,body)}if s.lookup_result.is_some(){return Ok(Self::fixed(size_of::<Option<RetainedOrderedMapLookup>>()))}if s.workspace.is_some(){return s.close.next_owner_demands::<EditWorkspace<K,V>>(true,&None,body)}if s.key.is_some(){return s.close.next_owner_demands::<K>(true,&None,body)}if s.value.is_some(){return s.close.next_owner_demands::<V>(true,&None,body)}if s.old_directory.is_some()||s.new_directory.is_some(){return s.close.next_owner_demands::<Vec<Vec<(K,V)>>>(true,&None,body)}if s.map.is_some()||s.output.is_some(){return s.close.next_owner_demands::<RetainedOrderedMap<K,V>>(true,&None,body)}s.input.close_demand(body)}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{let s=&*self.state;if let Some(route)=s.close_route{if route.key.body!=body{return Err(Self::invalid())}return Ok(route.work)}if !s.closing{return Ok(self.begin_close_work_demand())}if !s.close.is_empty(){return s.close.next_owner_work::<K>(false,&None,body)}if s.lookup_result.is_some(){return Ok(1)}if s.workspace.is_some(){return s.close.next_owner_work::<EditWorkspace<K,V>>(true,&None,body)}if s.key.is_some(){return s.close.next_owner_work::<K>(true,&None,body)}if s.value.is_some(){return s.close.next_owner_work::<V>(true,&None,body)}if s.old_directory.is_some()||s.new_directory.is_some(){return s.close.next_owner_work::<Vec<Vec<(K,V)>>>(true,&None,body)}if s.map.is_some()||s.output.is_some(){return s.close.next_owner_work::<RetainedOrderedMap<K,V>>(true,&None,body)}s.input.close_work_demand(body)}
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close_dispatch(body,None,effect,reply,false)}
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close_dispatch(body,None,effect,reply,true)}
 pub fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close_dispatch(body,Some(original),effect,reply,false)}
 pub fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut Effect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.close_dispatch(body,Some(original),effect,reply,true)}
 pub(super) fn close_dispatch<'r>(&mut self,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut Effect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  self.state.input.validate(effect)?;if !recover&&!self.state.closing{return self.begin_close_into(effect,reply)}if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let key=close_key(body,original.as_deref_mut(),effect,recover)?;let route=if recover{let route=self.state.close_route.ok_or_else(Self::invalid)?;if route.key!=key{return Err(Self::invalid())}route}else{if self.state.close_route.is_some(){return Err(Self::invalid())}let d=self.close_demands(body)?;let work=self.close_work_demand(body)?;if !Self::funded(effect,d,work)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}let s=&mut*self.state;if s.close.is_empty()&&s.lookup_result.is_some(){if !effect.performed(1)?{return Err(Self::invalid())}s.lookup_result=None;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))))}if s.close.is_empty(){macro_rules! begin{($field:ident)=>{if s.$field.is_some(){let turn=s.close.begin_granted(&mut s.$field,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}};}begin!(workspace);begin!(key);begin!(value);begin!(old_directory);begin!(new_directory);begin!(map);begin!(output);}CloseRoute{key,operation:u8::from(!s.close.is_empty()),demand:d,work}};
  let s=&mut*self.state;let mut scope=reply.progress().scope(&mut s.close_route,route)?;let done=if route.operation==1{match(original,recover){(None,false)=>s.close.step_into(body,effect,scope.borrow()?),(None,true)=>s.close.recover_step_into(body,effect,scope.borrow()?),(Some(original),false)=>s.close.step_original_into(original,body,effect,scope.borrow()?),(Some(original),true)=>s.close.recover_step_original_into(original,body,effect,scope.borrow()?)}}else{s.input.close_into(body,original,effect,scope.borrow()?,recover)}?;drop(done);Ok(scope.forwarded())
 }
 /// 🪜️ Drives one advance through a direct reply slot and resumes a recorded callback by itself; `None` keeps the callback pending.
 pub fn advance_turn(&mut self,body:usize,grant:RetainedOrderedMapInsertGrant,effect:&mut Effect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{
  let recover=self.state.phase==0&&self.state.input.has_pending();let mut slot=None;let reply=Reply::borrow(&mut slot)?;
  drop(if recover{self.recover_advance_into(body,grant,effect,reply)?}else{self.advance_into(body,grant,effect,reply)?});
  slot.map(|result|result.map_err(crate::OriginalRetirementFailure::into_value_error)).transpose()
 }
 /// 📤️ Drives the output donation through a direct reply slot.
 pub fn take_turn(&mut self,output:&mut Option<RetainedOrderedMap<K,V>>,effect:&mut Effect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{
  let refused=self.refused_workspace_ready();let mut slot=None;let reply=Reply::borrow(&mut slot)?;
  drop(self.take_dispatch(output,effect,reply,refused)?);
  slot.map(|result|result.map_err(crate::OriginalRetirementFailure::into_value_error)).transpose()
 }
 /// ♻️ Drives one close (starting it first) through a direct reply slot and resumes a recorded callback by itself.
 pub fn close_turn(&mut self,body:usize,effect:&mut Effect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{
  let recover=self.state.close_route.is_some();let mut slot=None;let reply=Reply::borrow(&mut slot)?;
  drop(self.close_dispatch(body,None,effect,reply,recover)?);
  slot.map(|result|result.map_err(crate::OriginalRetirementFailure::into_value_error)).transpose()
 }
 pub fn requires_original_birth(&self)->bool{if !self.state.close.is_empty(){self.state.close.requires_original_birth()}else{self.state.input.requires_original_birth()}}
 pub fn requires_original_control(&self)->bool{if !self.state.close.is_empty(){self.state.close.requires_original_control()}else{self.state.input.requires_original_control()}}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.state.close.is_empty(){self.state.close.step_original_birth(original,grant)}else{self.state.input.original_birth_step(original,grant)}}
 pub fn terminal_is_empty(&self)->bool{let s=&*self.state;s.closing&&s.close_route.is_none()&&s.input.terminal_is_empty()&&s.workspace.is_none()&&s.lookup_result.is_none()&&s.key.is_none()&&s.value.is_none()&&s.old_directory.is_none()&&s.new_directory.is_none()&&s.map.is_none()&&s.output.is_none()&&s.close.is_empty()}
}
impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> Drop for RetainedOrderedMapInsertCursor<K,V>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.state)}}else if !std::thread::panicking(){panic!("native insertion retains original owners")}}}
