use super::*;
use crate::brep::representation::topology::{ReachabilityJob,ReachabilityStep,BodyCompactionJob};
use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},RetirementDemand,ValueError,ValueRefusalKind};

/// 🧹️ Retains original live registry rows and their borrowed topology roots before the original bounded arena sweep.
pub struct BrepRetentionJob {walk:Option<ReachabilityJob>,compact:Option<BodyCompactionJob>,removed:Option<ControlledRetirement<(String,Entity)>>,closing_walk:Option<ControlledRetirement<ReachabilityJob>>,slot:usize,root:usize,decision:Option<bool>,revision:(u64,u64),restarting:bool,cancelled:bool,receipt:BrepRetentionReceipt}
#[derive(Clone,Copy)]
struct BrepRetentionReceipt(RetainedCloneProgress);
semio_framework_value::artifact_retire_leaf!(BrepRetentionReceipt);
semio_framework_value::artifact_retire_struct!(BrepRetentionJob {walk,compact,removed,closing_walk,slot,root,decision,revision,restarting,cancelled,receipt});
impl BrepRetentionJob {
    /// 🌱️ Borrows the original source revision and starts with inline empty traversal scratch.
    pub fn new(kernel:&Brep)->Self{Self {walk:Some(ReachabilityJob::new()),compact:None,removed:None,closing_walk:None,slot:0,root:0,decision:None,revision:kernel.retention_source_revision(),restarting:false,cancelled:false,receipt:BrepRetentionReceipt(Default::default())}}
    pub fn cancel(&mut self){self.cancelled=true;}
    /// 🔄️ Restarts borrowed decisions when the original external claim authority changes.
    pub fn restart(&mut self){self.restarting=true;}
    pub fn terminal_is_empty(&self)->bool{(!self.restarting||self.cancelled)&&self.walk.is_none()&&self.compact.is_none()&&self.removed.is_none()&&self.closing_walk.is_none()}
    /// 👀️ Borrows the exact next original handle while its caller scans original family claims.
    pub fn candidate_handle<'a>(&self,kernel:&'a Brep)->Option<&'a str>{
        if self.cancelled||self.restarting||self.revision!=kernel.retention_source_revision()||self.compact.is_some()||self.removed.is_some()||self.closing_walk.is_some()||self.decision.is_some()||self.walk.as_ref().is_none_or(|walk|!walk.walk_is_complete()){return None;}
        kernel.live.slot_entry(self.slot).filter(|(_,entity)|label_of_entity(&kernel.body,entity).is_some()).map(|(handle,_)|handle.as_str())
    }
    /// 🫴️ Captures one scalar decision only for the unchanged original candidate.
    pub fn decide(&mut self,kernel:&Brep,keep:bool)->Result<(),ValueError>{if self.candidate_handle(kernel).is_none(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original retain candidate changed before its claim decision"));}self.decision=Some(keep);Ok(())}
    fn closing_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,copy:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
    fn demand(&self,kernel:&Brep,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}
        if let Some(owner)=&self.removed{return Self::closing_demand(owner,copy);}
        if let Some(owner)=&self.closing_walk{return Self::closing_demand(owner,copy);}
        if self.cancelled||self.restarting||self.revision!=kernel.retention_source_revision(){return self.compact.as_ref().map_or(Ok(RetirementDemand {depth:1,..Default::default()}),|job|job.close_demands(copy));}
        if let Some(job)=&self.compact{return Ok(RetirementDemand {copy_bytes:job.next_copy_byte_demand(&kernel.body)?,capacity_bytes:job.next_capacity_byte_demand(&kernel.body,copy)?,release_bytes:job.next_release_byte_demand(&kernel.body)?,depth:job.next_depth_demand(&kernel.body)?});}
        let walk=self.walk.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original retain walk disappeared"))?;
        if !walk.walk_is_complete(){return Ok(RetirementDemand {copy_bytes:walk.next_copy_byte_demand()?,capacity_bytes:walk.next_capacity_byte_demand(copy)?,release_bytes:walk.next_release_byte_demand()?,depth:walk.next_depth_demand()?});}
        let depth=if self.decision==Some(false)||kernel.live.slot_entry(self.slot).is_some_and(|(_,entity)|label_of_entity(&kernel.body,entity).is_none()){kernel.live.next_extract_slot_depth_demand(self.slot)?}else{1};Ok(RetirementDemand {depth,..Default::default()})
    }
    pub fn next_copy_byte_demand(&self,kernel:&Brep)->Result<usize,ValueError>{Ok(self.demand(kernel,0)?.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,kernel:&Brep,copy:usize)->Result<usize,ValueError>{Ok(self.demand(kernel,copy)?.capacity_bytes)}
    pub fn next_release_byte_demand(&self,kernel:&Brep)->Result<usize,ValueError>{Ok(self.demand(kernel,0)?.release_bytes)}
    pub fn next_depth_demand(&self,kernel:&Brep)->Result<usize,ValueError>{Ok(self.demand(kernel,0)?.depth)}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.receipt.0}
    /// ⏱️ Owns each transferred row and retained topology allocation until its exact physical closure.
    pub fn step(&mut self,kernel:&mut Brep,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.receipt.0=Default::default();let step=self.step_retained(kernel,grant);if let Ok(step)=&step{self.receipt.0=step.progress();}step
    }
    fn step_retained(&mut self,kernel:&mut Brep,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if kernel.pending_mutations!=0&&!self.cancelled{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(owner)=&mut self.removed{let result=owner.step(grant);self.receipt.0=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.removed=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(owner)=&mut self.closing_walk{let result=owner.step(grant);self.receipt.0=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.closing_walk=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())});}
        if self.revision!=kernel.retention_source_revision(){self.restarting=true;}
        if let Some(job)=&mut self.compact{let result=if self.cancelled||self.restarting{job.close_step(grant)}else{job.step(&mut kernel.body,grant)};self.receipt.0=job.normal_step_progress();let step=result?;if job.terminal_is_empty(){self.compact=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())});}
        if !self.cancelled&&!self.restarting{if let Some(walk)=&mut self.walk{if !walk.walk_is_complete(){return match walk.step(&kernel.body,grant)?{ReachabilityStep::Failed{error,progress}=>{self.receipt.0=progress;Err(error)},step=>Ok(RetainedCloneStep::Progress(step.progress()))};}}}
        let demand=self.demand(kernel,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.cancelled||self.restarting{
            if self.cancelled{if let Some(walk)=self.walk.take(){self.closing_walk=Some(ControlledRetirement::new(walk).unwrap_or_else(|(error,_)|panic!("original retain walk closure: {error}")));}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
            if let Some(walk)=&mut self.walk{walk.restart();}else{self.walk=Some(ReachabilityJob::new());}self.slot=0;self.root=0;self.decision=None;self.revision=kernel.retention_source_revision();self.restarting=false;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));
        }
        let walk=self.walk.as_mut().unwrap();
        if self.slot>=kernel.live.slot_count(){let keep=self.walk.take().unwrap().into_set();self.compact=Some(BodyCompactionJob::new(keep).unwrap_or_else(|(error,_)|panic!("original retained compact admission: {error}")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        let Some((_,entity))=kernel.live.slot_entry(self.slot)else{self.slot+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));};
        if self.decision==Some(false)||label_of_entity(&kernel.body,entity).is_none(){let original=kernel.live.extract_slot_if(self.slot,|_,_|false);if let Some(original)=original{self.removed=Some(ControlledRetirement::new(original).unwrap_or_else(|(error,_)|panic!("original retained registry row: {error}")));kernel.advance_live_revision();self.revision=kernel.retention_source_revision();}self.slot+=1;self.root=0;self.decision=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        if self.decision.is_none(){return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(root)=entity_root_at(entity,self.root){walk.begin_root(root).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"original retain root admission busy"))?;self.root+=1;}else{self.slot+=1;self.root=0;self.decision=None;}
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()}))
    }
}

fn entity_root_at(entity:&Entity,index:usize)->Option<EntityRef>{match entity{
    Entity::Vertex(id)=>(index==0).then_some(EntityRef::Vertex(*id)),Entity::Edge(id)=>(index==0).then_some(EntityRef::Edge(*id)),Entity::Face(id)=>(index==0).then_some(EntityRef::Face(*id)),Entity::Shell(id)=>(index==0).then_some(EntityRef::Shell(*id)),Entity::Solid(id)=>(index==0).then_some(EntityRef::Solid(*id)),
    Entity::Compound(solids,_)=>solids.get(index).copied().map(EntityRef::Solid),Entity::Wire(wire,_)=>wire.members.get(index).map(|(edge,_)|EntityRef::Edge(*edge)).or_else(||index.checked_sub(wire.members.len()).and_then(|index|wire.vertices.get(index)).copied().map(EntityRef::Vertex)),Entity::Curve(_,_)|Entity::Surface(_,_)=>None,
}}
