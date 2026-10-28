//! 🧳️ Precommit custody returns the exact original boxed edit beside its prepared causal message.
use super::*;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneSourceTake,RetainedCloneClose};

/// 📤️ One issuer owns the original edit, its bounded encoder and every cancellation frontier.
pub struct ArtifactOriginalEditMessage<M:RetireOwned+Send+Sync+'static>{
    source:ManuallyDrop<Option<RetainedCloneSource<Box<Edit<M>>>>>,
    projection:ManuallyDrop<Option<RetainedOwnedProjection<Edit<M>>>>,
    writer:ManuallyDrop<Option<Box<ArtifactOriginalEditWire<M>>>>,
    returned:ManuallyDrop<Option<Box<Edit<M>>>>,
    returned_close:RetainedCloneClose,
    wire:ArtifactOperationWire<M>,phase:u8,closing:bool,
}
impl<M:RetireOwned+Send+Sync+'static> ArtifactOriginalEditMessage<M>{
    pub fn birth_demands()->RetirementDemand{let d=RetainedCloneSource::<Box<Edit<M>>>::owned_constructor_demand::<()>();RetirementDemand{copy_bytes:size_of::<Self>()+RetainedCloneSource::<Box<Edit<M>>>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()}}
    pub fn admit(edit:&mut Option<Box<Edit<M>>>,wire:ArtifactOperationWire<M>,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let d=Self::birth_demands();if !retained_clone_grant_admits(grant,d){return Ok(None)}
        let original=edit.take().ok_or_else(invalid)?;let(source,mut p)=match RetainedCloneSource::admit_owned(original,(),grant){Ok(value)=>value,Err((error,original,()))=>{*edit=Some(original);return Err(error)}};p.copied_bytes+=size_of::<Self>();
        Ok(Some((Self{source:ManuallyDrop::new(Some(source)),projection:ManuallyDrop::new(None),writer:ManuallyDrop::new(None),returned:ManuallyDrop::new(None),returned_close:Default::default(),wire,phase:0,closing:false},p)))
    }
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing{return self.close_demands(body)}
        let mut d=match self.phase{
            0=>RetainedOwnedProjection::<Edit<M>>::constructor_demand(),
            1=>{let mut d=ArtifactOriginalEditWire::<M>::birth_demands();d.capacity_bytes+=size_of::<ArtifactOriginalEditWire<M>>();d.copy_bytes+=size_of::<Option<Box<ArtifactOriginalEditWire<M>>>>();d},
            2=>self.writer.as_ref().ok_or_else(invalid)?.demands(body)?,
            3=>self.source.as_ref().ok_or_else(invalid)?.next_take_demand(body)?,
            4=>{let mut d=header::<PagedBytes<{isize::MAX as usize}>>();d.copy_bytes+=size_of::<Option<Box<Edit<M>>>>();d},
            _=>return Err(invalid()),
        };d.copy_bytes=d.copy_bytes.checked_add(size_of::<u8>()).ok_or_else(invalid)?;Ok(d)
    }
    pub fn advance(&mut self,headers:OriginalEditWireHeaders<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.closing{return Err(invalid())}if self.phase==4{return Ok(RetainedCloneStep::Complete(Default::default()))}
        let d=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-size_of::<u8>(),..grant};
        let mut p=match self.phase{
            0=>{let(owner,p)=self.source.as_ref().ok_or_else(invalid)?.project_owned(0,|edit|&**edit,g)?;*self.projection=Some(owner);self.phase=1;p},
            1=>{let(owner,mut p)=ArtifactOriginalEditWire::admit(&mut self.projection,self.wire,semio_framework_replication_causal::binary::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES,4*semio_framework_replication_causal::binary::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES,g)?.ok_or_else(invalid)?;*self.writer=Some(Box::new(owner));p.retained_capacity_bytes+=size_of::<ArtifactOriginalEditWire<M>>();p.copied_bytes+=size_of::<Option<Box<ArtifactOriginalEditWire<M>>>>();self.phase=2;p},
            2=>{let step=self.writer.as_mut().ok_or_else(invalid)?.advance(headers,g)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=3}step.progress()},
            3=>match self.source.as_mut().ok_or_else(invalid)?.take_authority(g)?{RetainedCloneSourceTake::Pending(p)=>p,RetainedCloneSourceTake::Ready(edit,p)=>{*self.returned=Some(edit);self.phase=4;p}},
            _=>return Err(invalid()),
        };p.copied_bytes+=size_of::<u8>();p.copied_items=p.copied_items.max(1);if !p.fits(grant){return Err(invalid().with_retained_progress(p))}Ok(if self.phase==4{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
    }
    pub fn ready(&self)->bool{self.phase==4&&!self.closing}
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Box<Edit<M>>,PagedBytes<{isize::MAX as usize}>,RetainedCloneProgress)>,ValueError>{
        if !self.ready(){return Err(invalid())}let d=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(None)}let Some((bytes,mut p))=self.writer.as_mut().ok_or_else(invalid)?.take(grant)?else{return Ok(None)};let edit=self.returned.take().ok_or_else(invalid)?;self.phase=5;p.copied_bytes+=size_of::<Option<Box<Edit<M>>>>()+size_of::<u8>();Ok(Some((edit,bytes,p)))
    }
    pub fn begin_close(&mut self){self.closing=true;if let Some(writer)=self.writer.as_mut(){writer.begin_close()}}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(writer)=self.writer.as_ref(){return if writer.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of::<ArtifactOriginalEditWire<M>>(),..header::<Option<Box<ArtifactOriginalEditWire<M>>>>()})}else{writer.close_demands(body)}}
        if let Some(owner)=self.projection.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Edit<M>>>>())}else{owner.next_close_demand(body)}}
        if let Some(source)=self.source.as_ref(){return if source.terminal_is_empty(){Ok(header::<Option<RetainedCloneSource<Box<Edit<M>>>>>())}else{source.next_close_demand(body)}}
        self.returned_close.next_owner_demands::<Box<Edit<M>>>(self.returned.is_some(),&None,body)
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.closing{return Err(invalid())}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
        let p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()};
        if let Some(writer)=self.writer.as_mut(){if !writer.terminal_is_empty(){return writer.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}drop(self.writer.take());return Ok(RetainedCloneStep::Progress(p))}
        if let Some(owner)=self.projection.as_mut(){if !owner.terminal_is_empty(){return owner.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.projection.take();return Ok(RetainedCloneStep::Progress(p))}
        if let Some(source)=self.source.as_mut(){if !source.terminal_is_empty(){return source.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.source.take();return Ok(RetainedCloneStep::Progress(p))}
        if let Some(step)=self.returned_close.begin_granted(&mut self.returned,grant)?{return Ok(step)}self.returned_close.step_granted(grant)
    }
    pub fn terminal_is_empty(&self)->bool{self.writer.is_none()&&self.projection.is_none()&&self.source.is_none()&&self.returned.is_none()&&self.returned_close.is_empty()}
}
impl<M:RetireOwned+Send+Sync+'static> Drop for ArtifactOriginalEditMessage<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original edit message requires complete paid closure");}}
