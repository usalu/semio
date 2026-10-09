use super::*;
use semio_framework_value::{retirement::{RetireOwned,controlled::{ControlledRetirement,ErasedControlledRetirement}},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},RetirementDemand,ValueError,ValueRefusalKind};

#[derive(Default)]
struct RemovedEntity {
    vertices:Option<ControlledRetirement<Vertex>>,edges:Option<ControlledRetirement<Edge>>,coedges:Option<ControlledRetirement<Coedge>>,loops:Option<ControlledRetirement<Loop>>,faces:Option<ControlledRetirement<Face>>,shells:Option<ControlledRetirement<Shell>>,solids:Option<ControlledRetirement<Solid>>,curves3:Option<ControlledRetirement<Curve3>>,curves2:Option<ControlledRetirement<Curve2>>,surfaces:Option<ControlledRetirement<Surface>>,
}
semio_framework_value::artifact_retire_struct!(RemovedEntity {vertices,edges,coedges,loops,faces,shells,solids,curves3,curves2,surfaces});
impl RemovedEntity {
    fn owner(&self)->Option<&dyn ErasedControlledRetirement>{
        macro_rules! active {($($field:ident),+)=>{$(if let Some(owner)=&self.$field{return Some(owner);})+};}
        active!(vertices,edges,coedges,loops,faces,shells,solids,curves3,curves2,surfaces);None
    }
    fn close_step(&mut self,grant:RetainedCloneGrant,receipt:&mut RetainedCloneProgress)->Result<RetainedCloneStep,ValueError>{
        macro_rules! close {($($field:ident),+)=>{$(if let Some(owner)=&mut self.$field{let result=owner.step(grant);*receipt=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.$field=None;}return Ok(step);})+};}
        close!(vertices,edges,coedges,loops,faces,shells,solids,curves3,curves2,surfaces);Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

/// 🧹️ Sweeps one original arena slot or one removed original payload frontier per independently funded turn.
pub struct BodyCompactionJob {keep:ControlledRetirement<ReachSet>,removed:RemovedEntity,store:usize,slot:usize,freed:Remap,cancelled:bool,receipt:CompactionReceipt}
#[derive(Clone,Copy)]
struct CompactionReceipt(RetainedCloneProgress);
semio_framework_value::artifact_retire_leaf!(CompactionReceipt);
semio_framework_value::artifact_retire_leaf!(Remap);
semio_framework_value::artifact_retire_struct!(BodyCompactionJob {keep,removed,store,slot,freed,cancelled,receipt});
impl BodyCompactionJob {
    /// 🫴️ Captures exact original membership inline and returns it unchanged on unsupported admission.
    pub fn new(keep:ReachSet)->Result<Self,(ValueError,ReachSet)>{
        if !Vertex::controlled_retirement_supported()||!Edge::controlled_retirement_supported()||!Coedge::controlled_retirement_supported()||!Loop::controlled_retirement_supported()||!Face::controlled_retirement_supported()||!Shell::controlled_retirement_supported()||!Solid::controlled_retirement_supported()||!Curve3::controlled_retirement_supported()||!Curve2::controlled_retirement_supported()||!Surface::controlled_retirement_supported(){return Err((ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original topology entity lacks physical retirement authority"),keep));}
        let keep=ControlledRetirement::new(keep)?;Ok(Self {keep,removed:RemovedEntity::default(),store:0,slot:0,freed:Remap::default(),cancelled:false,receipt:CompactionReceipt(Default::default())})
    }
    pub fn freed(&self)->Remap{self.freed}
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.receipt.0}
    pub fn cancel(&mut self){self.cancelled=true;}
    pub fn terminal_is_empty(&self)->bool{(self.cancelled||self.store==10)&&self.removed.owner().is_none()&&self.keep.terminal_is_empty()}
    fn scratch(&self)->Option<&dyn ErasedControlledRetirement>{self.removed.owner().or_else(||(self.cancelled||self.store==10).then_some(&self.keep as &dyn ErasedControlledRetirement))}
    fn demand(&self,body:&Body,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}
        if let Some(owner)=self.scratch(){return Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?});}
        let keep=self.keep.original().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original compact membership entered closure before sweep completion"))?;
        macro_rules! quote {($field:ident)=>{{let original=body.$field.slot_at(self.slot);let(birth,release)=original.filter(|(id,_)|!keep.$field.contains(id)).map_or((0,0),|(id,_)|body.$field.remove_backing_demand(id));let membership=original.map_or(Ok(0),|(id,_)|keep.$field.next_contains_depth_demand(&id))?;RetirementDemand {capacity_bytes:birth,release_bytes:release,depth:body.$field.next_slot_depth_demand(self.slot)?.max(membership).max(1),..Default::default()}}};}
        Ok(match self.store{0=>quote!(vertices),1=>quote!(edges),2=>quote!(coedges),3=>quote!(loops),4=>quote!(faces),5=>quote!(shells),6=>quote!(solids),7=>quote!(curves3),8=>quote!(curves2),9=>quote!(surfaces),_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original compact store is invalid"))})
    }
    pub fn next_copy_byte_demand(&self,body:&Body)->Result<usize,ValueError>{Ok(self.demand(body,0)?.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,body:&Body,copy:usize)->Result<usize,ValueError>{Ok(self.demand(body,copy)?.capacity_bytes)}
    pub fn next_release_byte_demand(&self,body:&Body)->Result<usize,ValueError>{Ok(self.demand(body,0)?.release_bytes)}
    pub fn next_depth_demand(&self,body:&Body)->Result<usize,ValueError>{Ok(self.demand(body,0)?.depth)}
    /// 📏️ Borrows exact closure demands without cancelling or scanning another source slot.
    pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let owner=self.removed.owner().unwrap_or(&self.keep);Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})
    }
    /// 🎟️ Advances physical closure without borrowing or sweeping any further source slot.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.receipt.0=Default::default();self.cancelled=true;if self.removed.owner().is_some(){return self.removed.close_step(grant,&mut self.receipt.0);}let result=self.keep.step(grant);self.receipt.0=self.keep.step_progress();result
    }
    /// ⏱️ Keeps source generations and all physically removed ownership at the same admitted boundary.
    pub fn step(&mut self,body:&mut Body,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.receipt.0=Default::default();let result=self.step_retained(body,grant);if let Ok(step)=&result{self.receipt.0=step.progress();}result
    }
    fn step_retained(&mut self,body:&mut Body,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if self.removed.owner().is_some(){return self.removed.close_step(grant,&mut self.receipt.0);}
        if self.cancelled||self.store==10{let result=self.keep.step(grant);self.receipt.0=self.keep.step_progress();return result;}
        let demand=self.demand(body,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let keep=self.keep.original().unwrap();
        macro_rules! sweep {($field:ident,$count:ident)=>{{
            if self.slot>=body.$field.slot_count(){self.store+=1;self.slot=0;RetainedCloneProgress {copied_items:1,..Default::default()}}
            else {let id=body.$field.slot_at(self.slot).filter(|(id,_)|!keep.$field.contains(id)).map(|(id,_)|id);
                if let Some(id)=id{let(value,progress)=body.$field.remove_granted(id,grant)?;if let Some(value)=value{self.removed.$field=Some(ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("validated original entity retirement: {error}")));self.freed.$count+=1;self.slot+=1;}progress}
                else{self.slot+=1;RetainedCloneProgress {copied_items:1,..Default::default()}}
            }
        }};}
        let progress=match self.store{0=>sweep!(vertices,freed_vertices),1=>sweep!(edges,freed_edges),2=>sweep!(coedges,freed_coedges),3=>sweep!(loops,freed_loops),4=>sweep!(faces,freed_faces),5=>sweep!(shells,freed_shells),6=>sweep!(solids,freed_solids),7=>sweep!(curves3,freed_curves3),8=>sweep!(curves2,freed_curves2),9=>sweep!(surfaces,freed_surfaces),_=>unreachable!()};Ok(RetainedCloneStep::Progress(progress))
    }
}
