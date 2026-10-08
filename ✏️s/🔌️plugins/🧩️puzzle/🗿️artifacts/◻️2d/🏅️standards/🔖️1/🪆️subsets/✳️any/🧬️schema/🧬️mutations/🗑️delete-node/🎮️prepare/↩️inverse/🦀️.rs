//! ↩️ Builds the exact native deletion inverse through controlled retained record owners.

use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::{Puzzle2dCloseAxis,close_child_demand,close_demand_methods};
use super::{DeleteNode,Puzzle2dDeleteNodePreparationCursor,Puzzle2dDeleteNodePreparationStep};
use crate::{Puzzle2dSnapshot,Puzzle2dNode,Puzzle2dEdge,standards::v1::subsets::any::schema::mutations::{Puzzle2dMutation,create_node::CreateNode,connect_handles::ConnectHandles,change_edge_visible::ChangeEdgeVisible,change_edge_locked::ChangeEdgeLocked}};
use semio_framework_value::{ValueError,ValueRefusalKind,list::PagedList,paged::PagedUtf8,retirement::controlled::ControlledRetirement,retained_clone::{RetainedClone,RetainedCloneBinding, RetainedFieldCursor,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneStep,RetainedCloneProgress}};
use crate::standards::v1::subsets::any::schema::mutations::native_preparation_child::Puzzle2dPreparationChild;
use std::{mem::{size_of,ManuallyDrop},ops::{Deref,DerefMut}};

pub struct Puzzle2dDeleteNodeInverseCursor { state: ManuallyDrop<Puzzle2dDeleteNodeInverseState> }

#[doc(hidden)]
pub struct Puzzle2dDeleteNodeInverseState {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    preparation: Puzzle2dPreparationChild<Puzzle2dDeleteNodePreparationCursor>,
    node: RetainedFieldCursor<Puzzle2dNode>,
    edge: RetainedFieldCursor<Puzzle2dEdge>,
    identifier: <PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor,
    pending: Option<Puzzle2dMutation>,
    inverse: PagedList<Puzzle2dMutation, {usize::MAX}>,
    pending_close: Option<ControlledRetirement<Puzzle2dMutation>>,
    inverse_close: Option<ControlledRetirement<PagedList<Puzzle2dMutation, {usize::MAX}>>>,
    node_index: usize,
    edge_index: usize,
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
    fn default()->Self { Self { state:ManuallyDrop::new(Puzzle2dDeleteNodeInverseState { source:None,mutation:None,preparation:Default::default(),node:RetainedFieldCursor::<Puzzle2dNode>::default(),edge:RetainedFieldCursor::<Puzzle2dEdge>::default(),identifier:PagedUtf8::retained_clone_cursor(),pending:None,inverse:Default::default(),pending_close:None,inverse_close:None,node_index:0,edge_index:0,visible:None,locked:None,flag:0,resume:0,phase:0,closing:false }) } }
}

impl Puzzle2dDeleteNodeInverseCursor {
    pub fn advance(&mut self,source:RetainedCloneRef<'_,Puzzle2dSnapshot>,mutation:RetainedCloneRef<'_,DeleteNode>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if self.closing || self.phase==21 { return Err(refusal("delete inverse is closing or spent")); }
        if grant.maximum_items==0 || grant.maximum_depth==0 { return Ok(progress(0,0,0)); }
        source.bind(&mut self.source)?;mutation.bind(&mut self.mutation)?;
        match self.phase {
            0 => {
                if let Some(step)=self.preparation.ensure(grant)? {return Ok(step);}
                let step=self.preparation.owner_mut().advance(source,mutation,grant)?;
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
                let step=self.node.advance(source.project(1,|snapshot|&snapshot.nodes).project(1,|nodes|nodes.get(index).expect("immutable inverse node ordinal")),grant)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {self.phase=2;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            2 => {
                if grant.maximum_copy_bytes<size_of::<Puzzle2dMutation>() { return Ok(progress(0,0,0)); }
                let node=self.node.take().ok_or_else(||refusal("delete inverse lost completed native node"))?;
                self.pending=Some(Puzzle2dMutation::CreateNode(CreateNode {node,index:Some(self.node_index)}));
                self.node.begin_close();self.phase=3;
                Ok(progress(1,size_of::<Puzzle2dMutation>(),0))
            }
            3 => {
                let step=self.node.close_step(grant)?;
                if self.node.terminal_is_empty() {self.resume=0;self.phase=8;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            4 => {
                let bytes=size_of::<RetainedFieldCursor<Puzzle2dEdge>>();
                if grant.maximum_copy_bytes<bytes {return Ok(progress(0,0,0));}
                self.edge=RetainedFieldCursor::<Puzzle2dEdge>::default();self.phase=5;
                Ok(progress(1,bytes,0))
            }
            5 => {
                let index=self.edge_index;
                let step=self.edge.advance(source.project(1,|snapshot|&snapshot.edges).project(1,|edges|edges.get(index).expect("immutable inverse edge ordinal")),grant)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {self.phase=6;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            6 => {
                let bytes=size_of::<Puzzle2dMutation>()+size_of::<[Option<bool>;2]>();
                if grant.maximum_copy_bytes<bytes {return Ok(progress(0,0,0));}
                let edge=self.edge.take().ok_or_else(||refusal("delete inverse lost completed native edge"))?;
                self.visible=edge.visible;self.locked=edge.locked;self.flag=0;
                self.pending=Some(Puzzle2dMutation::ConnectHandles(ConnectHandles {id:edge.id,source:edge.source,target:edge.target,edge_kind:edge.edge_kind,gap:edge.gap,shift:edge.shift,rise:edge.rise,rotation:edge.rotation,turn:edge.turn,tilt:edge.tilt,x:edge.x,y:edge.y,source_tip:edge.source_tip,target_tip:edge.target_tip,tolerance:None,index:Some(self.edge_index)}));
                self.edge.begin_close();self.phase=7;
                Ok(progress(1,bytes,0))
            }
            7 => {
                let step=self.edge.close_step(grant)?;
                if self.edge.terminal_is_empty() {self.resume=10;self.phase=8;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            8 => {
                if !self.inverse.has_reserved_slot() {
                    let demand=self.inverse.next_capacity_allocation_bytes(1).map_err(ValueError::from)?.ok_or_else(||refusal("delete inverse lost native page demand"))?;
                    if demand>grant.maximum_capacity_bytes {return Ok(progress(0,0,0));}
                    let admitted=self.inverse.reserve_one(grant.maximum_capacity_bytes).map_err(|error|ValueError::from(error.refusal()))?;
                    return Ok(progress(usize::from(admitted.progressed),0,admitted.allocated_bytes));
                }
                if grant.maximum_copy_bytes<size_of::<Puzzle2dMutation>() {return Ok(progress(0,0,0));}
                let operation=self.pending.take().ok_or_else(||refusal("delete inverse lost pending native mutation"))?;
                if let Err(operation)=self.inverse.push_reserved(operation) {self.pending=Some(operation);return Err(refusal("delete inverse lost admitted output page"));}
                self.phase=self.resume;
                Ok(progress(1,size_of::<Puzzle2dMutation>(),0))
            }
            10 => {
                if self.flag==2 {self.phase=0;return Ok(progress(1,0,0));}
                let flag=if self.flag==0 {self.visible}else{self.locked};
                if flag.is_none() {self.flag+=1;return Ok(progress(1,0,0));}
                let bytes=size_of::<<PagedUtf8<{usize::MAX}> as RetainedClone>::Cursor>();
                if grant.maximum_copy_bytes<bytes {return Ok(progress(0,0,0));}
                self.identifier=PagedUtf8::retained_clone_cursor();self.phase=11;
                Ok(progress(1,bytes,0))
            }
            11 => {
                let index=self.edge_index;
                let step=self.identifier.advance(source.project(1,|snapshot|&snapshot.edges).project(1,|edges|edges.get(index).expect("immutable inverse flag edge")).project(1,|edge|&edge.id),grant)?;
                if matches!(step,RetainedCloneStep::Complete(_)) {self.phase=12;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            12 => {
                if grant.maximum_copy_bytes<size_of::<Puzzle2dMutation>() {return Ok(progress(0,0,0));}
                let id=self.identifier.take().ok_or_else(||refusal("delete inverse lost literal flag identifier"))?;
                self.pending=Some(if self.flag==0 {Puzzle2dMutation::ChangeEdgeVisible(ChangeEdgeVisible {id,new_visible:self.visible})}else{Puzzle2dMutation::ChangeEdgeLocked(ChangeEdgeLocked {id,new_locked:self.locked})});
                self.identifier.begin_close();self.phase=13;
                Ok(progress(1,size_of::<Puzzle2dMutation>(),0))
            }
            13 => {
                let step=self.identifier.close_step(grant)?;
                if self.identifier.terminal_is_empty() {self.flag+=1;self.resume=10;self.phase=8;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            15 => {
                let step=self.preparation.close_granted(grant)?;
                if self.preparation.terminal_is_empty() {self.phase=16;}
                Ok(RetainedCloneStep::Progress(step.progress()))
            }
            16 => {
                if grant.maximum_copy_bytes<size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>() {return Ok(progress(0,0,0));}
                self.phase=20;
                Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>(),retained_capacity_bytes:0, released_bytes: 0 }))
            }
            20 => Ok(RetainedCloneStep::Complete(Default::default())),
            _ => Err(refusal("delete inverse has an unknown ownership phase")),
        }
    }

    pub fn take(&mut self)->Option<PagedList<Puzzle2dMutation,{usize::MAX}>> {if self.closing || self.phase!=20 {return None;}self.phase=21;Some(std::mem::take(&mut self.inverse))}
    pub fn begin_close(&mut self) {self.closing=true;self.preparation.begin_close();self.node.begin_close();self.edge.begin_close();self.identifier.begin_close();}

    fn close_demand(&self,axis:Puzzle2dCloseAxis)->Result<usize,ValueError>{
        if !self.preparation.terminal_is_empty(){return close_child_demand!(axis,self.preparation)}
        if !self.node.terminal_is_empty(){return axis.retained::<Puzzle2dNode,_>(&self.node)}
        if !self.edge.terminal_is_empty(){return axis.retained::<Puzzle2dEdge,_>(&self.edge)}
        if !self.identifier.terminal_is_empty(){return axis.retained::<PagedUtf8<{usize::MAX}>,_>(&self.identifier)}
        if let Some(owner)=self.pending_close.as_ref(){return axis.retirement(owner)}
        if self.pending.is_some(){return axis.inline::<Puzzle2dMutation>()}
        if let Some(owner)=self.inverse_close.as_ref(){return axis.retirement(owner)}
        if !self.inverse.terminal_is_empty(){return axis.inline::<PagedList<Puzzle2dMutation,{usize::MAX}>>()}
        axis.binding(if self.mutation.is_some(){&self.mutation}else{&self.source})
    }

    close_demand_methods!(next_close_copy_byte_demand,next_close_capacity_byte_demand,next_close_release_byte_demand,next_close_depth_demand);

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        if !self.closing {return Err(refusal("delete inverse closure was not started"));}
        if grant.maximum_items==0 || grant.maximum_depth==0 {return Ok(progress(0,0,0));}
        if !self.preparation.terminal_is_empty() {return self.preparation.close_granted(grant);}
        if !self.node.terminal_is_empty() {return self.node.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if !self.edge.terminal_is_empty() {return self.edge.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if !self.identifier.terminal_is_empty() {return self.identifier.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some(owner)=self.pending_close.as_mut() {let step=owner.step(grant)?;if owner.terminal_is_empty(){self.pending_close=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.pending.is_some() {
            if grant.maximum_copy_bytes<size_of::<Puzzle2dMutation>() {return Ok(progress(0,0,0));}
            match ControlledRetirement::new(self.pending.take().unwrap()) {Ok(owner)=>self.pending_close=Some(owner),Err((error,owner))=>{self.pending=Some(owner);return Err(error);}}
            return Ok(progress(1,size_of::<Puzzle2dMutation>(),0));
        }
        if let Some(owner)=self.inverse_close.as_mut() {let step=owner.step(grant)?;if owner.terminal_is_empty(){self.inverse_close=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if !self.inverse.terminal_is_empty() {
            if grant.maximum_copy_bytes<size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>() {return Ok(progress(0,0,0));}
            match ControlledRetirement::new(std::mem::take(&mut self.inverse)) {Ok(owner)=>self.inverse_close=Some(owner),Err((error,owner))=>{self.inverse=owner;return Err(error);}}
            return Ok(progress(1,size_of::<PagedList<Puzzle2dMutation,{usize::MAX}>>(),0));
        }
        let step=RetainedCloneBinding::close_one(if self.mutation.is_some(){&mut self.mutation}else{&mut self.source},grant)?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{retirement_progress(step)})
    }

    pub fn terminal_is_empty(&self)->bool {self.closing&&self.preparation.terminal_is_empty()&&self.node.terminal_is_empty()&&self.edge.terminal_is_empty()&&self.identifier.terminal_is_empty()&&self.pending.is_none()&&self.inverse.terminal_is_empty()&&self.pending_close.is_none()&&self.inverse_close.is_none()&&self.source.is_none()&&self.mutation.is_none()}
}

impl Drop for Puzzle2dDeleteNodeInverseCursor {fn drop(&mut self){let empty=self.terminal_is_empty();assert!(std::thread::panicking()||empty,"delete inverse abandoned before controlled closure");if empty{unsafe{ManuallyDrop::drop(&mut self.state);}}}}
fn progress(items:usize,bytes:usize,capacity:usize)->RetainedCloneStep {RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:items,copied_bytes:bytes,retained_capacity_bytes:capacity, released_bytes: 0 })}
fn retirement_progress(step:RetainedCloneStep)->RetainedCloneStep {RetainedCloneStep::Progress(step.progress())}
fn refusal(message:&str)->ValueError {ValueError::new(ValueRefusalKind::InvariantViolated,message)}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
