//! ↩️ Builds the exact native deletion inverse through controlled retained record owners.

use crate::standards::v1::subsets::any::schema::lookup::borrowed::BorrowedIdentity;
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::{cursor_advance,cursor_retire,cursor_take,identifier::{IdentifierCopy,Text},list_reserve,pay,settled_step};
use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand, RetirementPollExit};
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dCloseAxis;
use super::{Puzzle2dDeleteNodePreparationCursor, Puzzle2dDeleteNodePreparationStep};
use crate::standards::v1::subsets::any::schema::mutations::DeleteNode;
use {crate::Puzzle2dSnapshot, crate::standards::v1::subsets::any::schema::records::Puzzle2dNode, crate::standards::v1::subsets::any::schema::records::Puzzle2dEdge, crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation, crate::standards::v1::subsets::any::schema::mutations::create_node::CreateNode, crate::standards::v1::subsets::any::schema::mutations::connect_handles::ConnectHandles, crate::standards::v1::subsets::any::schema::mutations::change_edge_visible::ChangeEdgeVisible, crate::standards::v1::subsets::any::schema::mutations::change_edge_locked::ChangeEdgeLocked};
use semio_framework_value::{ValueError, retained_clone::{RetainedClone,RetainedCloneClose,RetainedFieldCursor,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress,retained_clone_grant_admits}};
use semio_framework_value_persistent::list::PagedList;
use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::Puzzle2dPreparationChild;
use std::{mem::{size_of,ManuallyDrop},ops::{Deref,DerefMut}};

pub struct Puzzle2dDeleteNodeInverseCursor { state: ManuallyDrop<Puzzle2dDeleteNodeInverseState> }

type Inverse=PagedList<Puzzle2dMutation,{usize::MAX}>;

enum RecordClone {
    Node(RetainedFieldCursor<Puzzle2dNode>),
    Edge(RetainedFieldCursor<Puzzle2dEdge>),
}

impl RecordClone {
    fn node(&mut self)->Result<&mut RetainedFieldCursor<Puzzle2dNode>,ValueError>{
        match self{Self::Node(cursor)=>Ok(cursor),Self::Edge(_)=>Err(refusal("delete inverse node clone is absent"))}
    }

    fn edge(&mut self)->Result<&mut RetainedFieldCursor<Puzzle2dEdge>,ValueError>{
        match self{Self::Edge(cursor)=>Ok(cursor),Self::Node(_)=>Err(refusal("delete inverse edge clone is absent"))}
    }

    fn terminal_is_empty(&self)->bool{
        match self{Self::Node(cursor)=>cursor.terminal_is_empty(),Self::Edge(cursor)=>cursor.terminal_is_empty()}
    }

    fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        match self{Self::Node(cursor)=>cursor_retire(cursor,effect),Self::Edge(cursor)=>cursor_retire(cursor,effect)}
    }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        match self{Self::Node(cursor)=>axis.retained::<Puzzle2dNode,_>(cursor),Self::Edge(cursor)=>axis.retained::<Puzzle2dEdge,_>(cursor)}
    }
}

#[doc(hidden)]
pub struct Puzzle2dDeleteNodeInverseState {
    source: BorrowedIdentity,
    mutation: BorrowedIdentity,
    preparation: Puzzle2dPreparationChild<Puzzle2dDeleteNodePreparationCursor>,
    record: RecordClone,
    identifier: IdentifierCopy,
    identifier_value: Option<Text>,
    pending: Option<Puzzle2dMutation>,
    inverse: Option<Inverse>,
    retirement: RetainedCloneClose,
    node_index: usize,
    edge_index: usize,
    reverse_index: usize,
    visible: Option<bool>,
    locked: Option<bool>,
    flag: u8,
    resume: u8,
    phase: u8,
    closing: bool,
}

impl Deref for Puzzle2dDeleteNodeInverseCursor { type Target=Puzzle2dDeleteNodeInverseState;fn deref(&self)->&Self::Target { &self.state } }
impl DerefMut for Puzzle2dDeleteNodeInverseCursor { fn deref_mut(&mut self)->&mut Self::Target { &mut self.state } }

impl Default for Puzzle2dDeleteNodeInverseCursor {
    fn default()->Self { Self { state:ManuallyDrop::new(Puzzle2dDeleteNodeInverseState { source:Default::default(),mutation:Default::default(),preparation:Default::default(),record:RecordClone::Node(Default::default()),identifier:Default::default(),identifier_value:None,pending:None,inverse:Some(PagedList::with_page_items(2)),retirement:Default::default(),node_index:0,edge_index:0,reverse_index:0,visible:None,locked:None,flag:0,resume:0,phase:0,closing:false }) } }
}

impl Puzzle2dDeleteNodeInverseCursor {
    pub fn advance(&mut self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,mutation:RetainedCloneRef<'_,DeleteNode>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError> {
        let grant=effect.grant();
        if self.closing || self.phase==21 { return Err(refusal("delete inverse is closing or spent")); }
        if grant.maximum_items==0 || grant.maximum_depth==0 { return Ok(progress(0,0,0)); }
        if let Some(progress)=self.source.bind(source,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        if let Some(progress)=self.mutation.bind(mutation,effect)?{return Ok(RetainedCloneStep::Progress(progress))}
        match self.phase {
            0 => {
                if let Some(step)=self.preparation.ensure(effect)? {return Ok(step);}
                let step=self.preparation.owner_mut().advance(source,mutation,effect)?;
                let result=match step {
                    Puzzle2dDeleteNodePreparationStep::Pending(p)=>p,
                    Puzzle2dDeleteNodePreparationStep::Node { index,progress }=>{ if let Some(index)=index { self.node_index=index;self.phase=1; } progress },
                    Puzzle2dDeleteNodePreparationStep::Edge { index,progress }=>{self.edge_index=index;self.phase=4;progress},
                    Puzzle2dDeleteNodePreparationStep::Complete(p)=>{self.preparation.begin_close();self.phase=15;p},
                };
                Ok(RetainedCloneStep::Progress(result))
            }
            1 => {
                let index=self.node_index;
                let step=cursor_advance(self.record.node()?,node_source(source,index)?,body,effect)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {self.phase=2;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            2 => {
                let (node,progress)=capture(self.record.node()?,size_of::<Option<Puzzle2dMutation>>(),effect)?;
                if let Some(node)=node{
                    self.pending=Some(Puzzle2dMutation::CreateNode(CreateNode{node,index:Some(self.node_index)}));
                    self.phase=3;
                }
                Ok(RetainedCloneStep::Progress(progress))
            }
            3 => {
                let step=self.record.close(effect)?;
                if self.record.terminal_is_empty() {self.resume=0;self.phase=8;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            4 => {
                if !self.record.terminal_is_empty(){return self.record.close(effect).map(retirement_progress);}
                let bytes=size_of::<RecordClone>();
                if grant.maximum_copy_bytes<bytes||!pay(effect)? {return Ok(progress(0,0,0));}
                self.record=RecordClone::Edge(Default::default());self.phase=5;
                Ok(progress(1,bytes,0))
            }
            5 => {
                let index=self.edge_index;
                let step=cursor_advance(self.record.edge()?,edge_source(source,index)?,body,effect)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {self.phase=6;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            6 => {
                let (edge,progress)=capture(self.record.edge()?,edge_placement_bytes(),effect)?;
                let Some(edge)=edge else{return Ok(RetainedCloneStep::Progress(progress));};
                self.visible=edge.visible;self.locked=edge.locked;self.flag=0;
                self.pending=Some(Puzzle2dMutation::ConnectHandles(ConnectHandles {id:edge.id,source:edge.source,target:edge.target,edge_kind:edge.edge_kind,gap:edge.gap,shift:edge.shift,rise:edge.rise,rotation:edge.rotation,turn:edge.turn,tilt:edge.tilt,x:edge.x,y:edge.y,source_tip:edge.source_tip,target_tip:edge.target_tip,tolerance:None,index:Some(self.edge_index)}));
                self.phase=7;
                Ok(RetainedCloneStep::Progress(progress))
            }
            7 => {
                let step=self.record.close(effect)?;
                if self.record.terminal_is_empty() {self.resume=10;self.phase=8;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            8 => {
                let state=&mut *self.state;
                let inverse=state.inverse.as_mut().ok_or_else(||refusal("delete inverse output is absent"))?;
                if !inverse.has_reserved_slot() { return Ok(RetainedCloneStep::Progress(list_reserve(inverse,effect)?.unwrap_or_default())); }
                let bytes=size_of::<Puzzle2dMutation>()+size_of::<Option<Puzzle2dMutation>>();
                if grant.maximum_copy_bytes<bytes||!pay(effect)? {return Ok(progress(0,0,0));}
                let operation=state.pending.take().ok_or_else(||refusal("delete inverse lost pending native mutation"))?;
                if let Err(operation)=inverse.push_reserved(operation) {state.pending=Some(operation);return Err(refusal("delete inverse lost admitted output page"));}
                state.phase=state.resume;
                Ok(progress(1,bytes,0))
            }
            10 => {
                let flag=if self.flag==0 {self.visible}else{self.locked};
                if self.flag==2||flag.is_none(){
                    if grant.maximum_copy_bytes<size_of::<u8>()||!pay(effect)?{return Ok(progress(0,0,0));}
                    if self.flag==2{self.phase=0;}else{self.flag+=1;}
                    return Ok(progress(1,size_of::<u8>(),0));
                }
                let bytes=size_of::<IdentifierCopy>();
                if grant.maximum_copy_bytes<bytes||!pay(effect)? {return Ok(progress(0,0,0));}
                self.identifier=Default::default();self.phase=11;
                Ok(progress(1,bytes,0))
            }
            11 => {
                let edge=edge_source(source,self.edge_index)?;
                let step=self.identifier.advance(&edge.get().id,body,effect)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {self.phase=12;}
                Ok(retirement_progress(step))
            }
            12 => {
                let state=&mut *self.state;
                let step=state.identifier.take(&mut state.identifier_value,effect)?;
                if state.identifier_value.is_some(){state.phase=13;}
                Ok(retirement_progress(step))
            }
            13 => {
                let state=&mut *self.state;
                let step=state.identifier.close(&mut state.identifier_value,effect)?;
                if state.identifier.terminal_is_empty(){state.phase=14;}
                Ok(retirement_progress(step))
            }
            14 => {
                let bytes=size_of::<Option<Puzzle2dMutation>>()+size_of::<Option<Text>>();
                if grant.maximum_copy_bytes<bytes||!pay(effect)?{return Ok(progress(0,0,0));}
                let id=self.identifier_value.take().ok_or_else(||refusal("delete inverse lost its flag identifier"))?;
                self.pending=Some(if self.flag==0{Puzzle2dMutation::ChangeEdgeVisible(ChangeEdgeVisible{id,new_visible:self.visible})}
                    else{Puzzle2dMutation::ChangeEdgeLocked(ChangeEdgeLocked{id,new_locked:self.locked})});
                self.flag+=1;self.resume=10;self.phase=8;
                Ok(progress(1,bytes,0))
            }
            15 => {
                let step=settled_step(self.preparation.close_step(effect))?;
                if self.preparation.terminal_is_empty() {self.phase=16;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            16 => {
                if let Some((right,demand))=self.reverse_demand()?{
                    let state=&mut *self.state;
                    let inverse=state.inverse.as_mut().ok_or_else(||refusal("delete inverse output is absent"))?;
                    let work=inverse.swap_source_work_demand(state.reverse_index,right)?;
                    let available=effect.remaining_work_units()?;
                    if !retained_clone_grant_admits(grant,demand)||available<=work{return Ok(progress(0,0,0));}
                    let swap_grant=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-size_of::<usize>(),..grant};
                    if !inverse.swap_source_step(state.reverse_index,right,&mut effect.reborrow(swap_grant,work)?).result?{return Ok(progress(0,0,0));}
                    if !pay(effect)?{return Err(refusal("delete inverse reverse index lost its Work"))}
                    state.reverse_index+=1;
                    return Ok(progress(1,demand.copy_bytes,0));
                }
                if grant.maximum_copy_bytes<size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>()||!pay(effect)? {return Ok(progress(0,0,0));}
                self.phase=20;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>(),retained_capacity_bytes:0, released_bytes: 0 }))
            }
            20 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("delete inverse has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self)->Option<PagedList<Puzzle2dMutation,{usize::MAX}>> {if self.closing || self.phase!=20 {return None;}self.phase=21;self.inverse.take()}
    pub fn begin_close(&mut self) {self.closing=true;self.preparation.begin_close();}

    fn reverse_demand(&self)->Result<Option<(usize,RetirementDemand)>,ValueError>{
        let inverse=self.inverse.as_ref().ok_or_else(||refusal("delete inverse output is absent"))?;
        if self.reverse_index>=inverse.len()/2{return Ok(None);}
        let right=inverse.len()-1-self.reverse_index;
        let mut demand=inverse.swap_source_demand(self.reverse_index,right)?;
        demand.copy_bytes+=size_of::<usize>();
        Ok(Some((right,demand)))
    }

    #[cfg(feature="testkit")]
    #[doc(hidden)]
    pub fn advance_test_demand(&self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::cursor_retire_demand;
        Ok(match (&self.record,self.phase){
            (RecordClone::Node(cursor),1)=>Some(cursor.advance_demands(node_source(source,self.node_index)?,body,u64::MAX)?),
            (RecordClone::Node(cursor),2)=>Some(capture_demand(cursor,size_of::<Option<Puzzle2dMutation>>())?),
            (RecordClone::Node(cursor),3)=>Some(cursor_retire_demand::<Puzzle2dNode,_>(cursor,body)?),
            (RecordClone::Edge(cursor),5)=>Some(cursor.advance_demands(edge_source(source,self.edge_index)?,body,u64::MAX)?),
            (RecordClone::Edge(cursor),6)=>Some(capture_demand(cursor,edge_placement_bytes())?),
            (RecordClone::Edge(cursor),7)=>Some(cursor_retire_demand::<Puzzle2dEdge,_>(cursor,body)?),
            (_,8)=>self.inverse.as_ref().filter(|list|!list.has_reserved_slot()).map(|list|Ok::<_,ValueError>(RetirementDemand{
                copy_bytes:list.next_reserve_copy_byte_demand()?,
                capacity_bytes:list.next_allocation_bytes()?,
                depth:list.next_reserve_depth_demand()?,
                ..Default::default()
            })).transpose()?,
            (_,11)=>Some(self.identifier.advance_demand(&edge_source(source,self.edge_index)?.get().id,body)?),
            (_,13)=>Some(self.identifier.close_demand(body)?),
            (_,16)=>Some(self.reverse_demand()?.map_or(RetirementDemand{copy_bytes:size_of::<Inverse>(),depth:1,..Default::default()},|(_,demand)|demand)),
            _=>None,
        })
    }

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return match axis {Puzzle2dCloseAxis::Copy=>self.preparation.next_close_copy_byte_demand(),Puzzle2dCloseAxis::Capacity(body)=>self.preparation.next_close_capacity_byte_demand(body),Puzzle2dCloseAxis::Release=>self.preparation.next_close_release_byte_demand(),Puzzle2dCloseAxis::Depth=>self.preparation.next_close_depth_demand()}}
        if !self.record.terminal_is_empty(){return self.record.close_demand(axis)}
        if !self.identifier.terminal_is_empty(){
            let body=match axis{Puzzle2dCloseAxis::Capacity(body)=>body,_=>0};
            return Ok(axis.select(self.identifier.close_demand(body)?))
        }
        if !self.retirement.is_empty(){return axis.owned::<()>(&self.retirement,false)}
        if self.identifier_value.is_some(){return axis.owned::<Text>(&self.retirement,true)}
        if self.pending.is_some(){return axis.owned::<Puzzle2dMutation>(&self.retirement,true)}
        if self.inverse.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){return axis.owned::<Inverse>(&self.retirement,true)}
        Ok(axis.select(if self.mutation.is_empty(){self.source.close_demand()}else{self.mutation.close_demand()}))
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Copy)}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Capacity(body))}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Release)}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.close_demand(Puzzle2dCloseAxis::Depth)}

    pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let result=self.close_stepped(effect);RetirementPollExit::settle(effect,result)}
    fn close_stepped(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant=effect.grant();
        if !self.closing {return Err(refusal("delete inverse closure was not started").into());}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(progress(0,0,0));}
        if !self.preparation.terminal_is_empty() {return Ok(RetainedCloneStep::Progress(RetirementPollExit::resume(self.preparation.close_step(effect))?.progress()));}
        let state=&mut *self.state;
        if !state.record.terminal_is_empty(){return Ok(retirement_progress(state.record.close(effect)?))}
        if !state.identifier.terminal_is_empty(){return Ok(retirement_progress(state.identifier.close(&mut state.identifier_value,effect)?))}
        if !state.retirement.is_empty(){return RetirementPollExit::resume(state.retirement.step_granted(grant.maximum_copy_bytes,effect)).map(retirement_progress)}
        let birth=if state.identifier_value.is_some(){Some(state.retirement.begin_granted(&mut state.identifier_value,effect))}
            else if state.pending.is_some(){Some(state.retirement.begin_granted(&mut state.pending,effect))}
            else if state.inverse.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){Some(state.retirement.begin_granted(&mut state.inverse,effect))}
            else{None};
        if let Some(turn)=birth{return Ok(RetainedCloneStep::Progress(turn.result?.map(|step|step.progress()).unwrap_or_default()))}
        let progress=if self.mutation.is_empty(){self.source.close(effect)?}else{self.mutation.close(effect)?};
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }

    pub fn terminal_is_empty(&self)->bool {self.closing&&self.preparation.terminal_is_empty()&&self.record.terminal_is_empty()&&self.identifier.terminal_is_empty()&&self.identifier_value.is_none()&&self.pending.is_none()&&self.inverse.as_ref().is_none_or(PagedList::terminal_is_empty)&&self.retirement.is_empty()&&self.source.is_empty()&&self.mutation.is_empty()}
}

impl Drop for Puzzle2dDeleteNodeInverseCursor {fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"delete inverse abandoned before controlled closure");if empty{unsafe{ManuallyDrop::drop(&mut self.state);}}}}

fn node_source(source:RetainedCloneRef<'_,Puzzle2dSnapshot>,index:usize)->Result<RetainedCloneRef<'_,Puzzle2dNode>,ValueError>{
    source.project(1,|snapshot|&snapshot.nodes).try_project(1,|nodes|nodes.get(index).ok_or_else(||refusal("immutable inverse node ordinal is absent")))
}

fn edge_source(source:RetainedCloneRef<'_,Puzzle2dSnapshot>,index:usize)->Result<RetainedCloneRef<'_,Puzzle2dEdge>,ValueError>{
    source.project(1,|snapshot|&snapshot.edges).try_project(1,|edges|edges.get(index).ok_or_else(||refusal("immutable inverse edge ordinal is absent")))
}

fn edge_placement_bytes()->usize{size_of::<Option<Puzzle2dMutation>>()+size_of::<[Option<bool>;2]>()+size_of::<u8>()}

fn capture_demand<T:RetainedClone>(cursor:&RetainedFieldCursor<T>,extra:usize)->Result<RetirementDemand,ValueError>{
    let mut demand=cursor.take_demand()?;
    demand.copy_bytes=demand.copy_bytes.checked_add(extra).ok_or_else(||refusal("delete inverse capture quote overflow"))?;
    Ok(demand)
}

fn capture<T:RetainedClone>(cursor:&mut RetainedFieldCursor<T>,extra:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<(Option<T>,RetainedCloneProgress),ValueError>{
    let grant=effect.grant();
    let demand=capture_demand(cursor,extra)?;
    let available=effect.remaining_work_units()?;
    let work=cursor.take_work_demand()?.checked_add(1).ok_or_else(||refusal("delete inverse capture Work overflow"))?;
    if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand)||available<work{return Ok((None,Default::default()))}
    let mut output=None;
    let mut progress={
        let mut nested=effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-extra,..grant},available-1)?;
        cursor_take(cursor,&mut output,&mut nested)?.progress()
    };
    if output.is_some(){
        if !pay(effect)?{return Err(refusal("delete inverse capture lost its Work"))}
        progress.copied_bytes+=extra;
        progress.copied_items=progress.copied_items.max(1);
    }
    Ok((output,progress))
}

fn progress(items:usize,bytes:usize,capacity:usize)->RetainedCloneStep {RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:items,copied_bytes:bytes,retained_capacity_bytes:capacity, released_bytes: 0 })}
fn retirement_progress(step:RetainedCloneStep)->RetainedCloneStep {RetainedCloneStep::Progress(step.progress())}
fn refusal(message:&'static str)->ValueError {ValueError::invariant(message)}


