//! 📤️ Original edit metadata and operation pages feed the bounded causal message writer.
use super::{ArtifactOperationWire,ArtifactOriginalOperationBytes,ArtifactPreparedOperationOutput};
use semio_framework_os_spr::Edit;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use semio_framework_replication_causal::binary::native_encoding::{CausalEnvelopeField,CausalEnvelopeShape,CausalEnvelopeWireSource,CausalEnvelopeWireCursor};
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value::paged::PagedBytes;
#[path="👁️.rs"]
mod view;
pub use view::OriginalEditWireHeaders;
use view::EditWireView;
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original edit wire frontier differs from its retained source")}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}

/// 🧵️ The original edit remains owned by its issuer until all encoded field aliases return.
pub struct ArtifactOriginalEditWire<M:Sync>{
    original:ManuallyDrop<Option<RetainedOwnedProjection<Edit<M>>>>,
    projection:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
    child:ManuallyDrop<Option<Box<ArtifactOriginalOperationBytes<M>>>>,
    forward:ManuallyDrop<Option<Box<ArtifactOriginalOperationBytes<M>>>>,
    inverse:ManuallyDrop<Option<Box<ArtifactOriginalOperationBytes<M>>>>,
    wire:ArtifactOperationWire<M>,
    output:ArtifactPreparedOperationOutput,
    cursor:CausalEnvelopeWireCursor,
    pending:[u8;64],length:usize,offset:usize,row:usize,phase:u8,closing:bool,
}
impl<M:Sync> ArtifactOriginalEditWire<M>{
    pub fn birth_demands()->RetirementDemand{header::<Self>()}
    pub fn admit(source:&mut Option<RetainedOwnedProjection<Edit<M>>>,wire:ArtifactOperationWire<M>,maximum_payload:usize,maximum_capacity:usize,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let d=Self::birth_demands();if !retained_clone_grant_admits(grant,d){return Ok(None)}
        let original=source.as_ref().ok_or_else(invalid)?.borrow()?;let edit=original.get();if edit.forwards.len()!=edit.mutation_meta.len(){return Err(invalid())}
        let output=ArtifactPreparedOperationOutput::empty(maximum_payload,maximum_capacity)?;
        Ok(Some((Self{original:ManuallyDrop::new(source.take()),projection:ManuallyDrop::new(None),child:ManuallyDrop::new(None),forward:ManuallyDrop::new(None),inverse:ManuallyDrop::new(None),wire,output,cursor:Default::default(),pending:[0;64],length:0,offset:0,row:0,phase:0,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
    }
    fn child_birth()->RetirementDemand{let mut d=ArtifactOriginalOperationBytes::<M>::birth_demands();d.capacity_bytes=size_of::<ArtifactOriginalOperationBytes<M>>();d.copy_bytes+=size_of::<Option<Box<ArtifactOriginalOperationBytes<M>>>>();d}
    fn child_drop()->RetirementDemand{RetirementDemand{release_bytes:size_of::<ArtifactOriginalOperationBytes<M>>(),..header::<Option<Box<ArtifactOriginalOperationBytes<M>>>>()}}
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing{return self.close_demands(body)}
        let mut d=match self.phase{
            0|3=>RetainedOwnedProjection::<M>::constructor_demand(),
            1|4=>Self::child_birth(),2|5=>self.child.as_ref().ok_or_else(invalid)?.demands(body)?,
            6 if self.offset<self.length=>self.output.append_demands((self.length-self.offset).min(body))?,
            6=>self.cursor.demand(body),
            7=>self.forward.as_ref().map_or(Ok(header::<u8>()),|owner|if owner.terminal_is_empty(){Ok(Self::child_drop())}else{owner.close_demands(body)})?,
            8=>self.inverse.as_ref().map_or(Ok(header::<u8>()),|owner|if owner.terminal_is_empty(){Ok(Self::child_drop())}else{owner.close_demands(body)})?,
            9 if self.offset<self.length=>self.output.append_demands((self.length-self.offset).min(body))?,
            9 if !self.cursor.is_complete()=>header::<(usize,u8)>(),
            9=>header::<u8>(),
            10=>self.original.as_ref().map_or(Ok(header::<u8>()),|owner|if owner.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Edit<M>>>>())}else{owner.next_close_demand(body)})?,
            11=>header::<PagedBytes<{isize::MAX as usize}>>(),_=>return Err(invalid()),
        };d.copy_bytes=d.copy_bytes.checked_add(size_of::<usize>()*3+size_of::<u8>()).ok_or_else(invalid)?;d.depth=d.depth.max(1);Ok(d)
    }
    pub fn advance(&mut self,headers:OriginalEditWireHeaders<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.closing{return Err(invalid())}if self.phase==11{return Ok(RetainedCloneStep::Complete(Default::default()))}
        let d=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
        let metadata=size_of::<usize>()*3+size_of::<u8>();let g=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-metadata,..grant};
        let p=match self.phase{
            0|3=>{let original=self.original.as_ref().ok_or_else(invalid)?;let edit=original.borrow()?;let inverse=self.phase==3;if self.row>=edit.get().forwards.len(){self.phase=6;RetainedCloneProgress{copied_items:1,..Default::default()}}else if inverse&&edit.get().inverse.get(self.row).is_none(){self.phase=6;RetainedCloneProgress{copied_items:1,..Default::default()}}else{let row=self.row;let(owner,p)=original.project(row*2+usize::from(inverse),|edit|if inverse{&edit.inverse[row]}else{&edit.forwards[row]},g)?;*self.projection=Some(owner);self.phase+=1;p}},
            1|4=>{let(owner,mut p)=ArtifactOriginalOperationBytes::admit_paged(&mut self.projection,self.wire,262144,1<<20,g)?.ok_or_else(invalid)?;*self.child=Some(Box::new(owner));p.retained_capacity_bytes+=size_of::<ArtifactOriginalOperationBytes<M>>();p.copied_bytes+=size_of::<Option<Box<ArtifactOriginalOperationBytes<M>>>>();self.phase+=1;p},
            2|5=>{let step=self.child.as_mut().ok_or_else(invalid)?.advance(g)?;if matches!(step,RetainedCloneStep::Complete(_)){if self.phase==2{*self.forward=self.child.take();self.phase=3}else{*self.inverse=self.child.take();self.phase=6}}step.progress()},
            6 if self.offset<self.length=>{let p=self.output.append(&self.pending[self.offset..self.length],g)?;self.offset+=p.copied_bytes;p},
            6=>{let original=self.original.as_ref().ok_or_else(invalid)?.borrow()?;let view=EditWireView{edit:original.get(),wire:self.wire,headers,current:self.row,forward:self.forward.as_deref(),inverse:self.inverse.as_deref()};let step=self.cursor.advance(&view,&mut self.pending,g)?;self.length=step.written_bytes;self.offset=0;if self.cursor.row_index()>self.row||self.cursor.is_complete(){if let Some(owner)=self.forward.as_mut(){owner.begin_close()}if let Some(owner)=self.inverse.as_mut(){owner.begin_close()}self.phase=7}step.progress},
            7|8=>{let slot=if self.phase==7{&mut self.forward}else{&mut self.inverse};if let Some(owner)=slot.as_mut(){if !owner.terminal_is_empty(){owner.close_step(g)?.progress()}else{drop(slot.take());self.phase+=1;RetainedCloneProgress{copied_items:1,copied_bytes:Self::child_drop().copy_bytes,released_bytes:Self::child_drop().release_bytes,..Default::default()}}}else{self.phase+=1;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}}},
            9 if self.offset<self.length=>{let p=self.output.append(&self.pending[self.offset..self.length],g)?;self.offset+=p.copied_bytes;p},
            9 if !self.cursor.is_complete()=>{self.row=self.cursor.row_index();self.phase=0;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>()+size_of::<u8>(),..Default::default()}},
            9=>{self.phase=10;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}},
            10=>{if let Some(original)=self.original.as_mut(){if !original.terminal_is_empty(){original.close_step(g)?.progress()}else{self.original.take();self.phase=11;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<RetainedOwnedProjection<Edit<M>>>>(),..Default::default()}}}else{return Err(invalid())}},
            _=>return Err(invalid()),
        };let p=p.checked_add(RetainedCloneProgress{copied_items:usize::from(p.copied_items==0),copied_bytes:metadata,..Default::default()})?;if !p.fits(grant){return Err(invalid().with_retained_progress(p))}Ok(if self.phase==11{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
    }
    pub fn payload_ready(&self)->bool{self.phase==11&&!self.closing}
    pub fn payload_len(&self)->Result<usize,ValueError>{if !self.payload_ready(){return Err(invalid())}Ok(self.output.len())}
    pub fn read_payload(&self,offset:usize,output:&mut[u8])->Result<usize,ValueError>{let len=self.payload_len()?;if offset>len{return Err(invalid())}let count=(len-offset).min(output.len());for(index,byte)in output[..count].iter_mut().enumerate(){*byte=self.output.byte_at(offset+index).ok_or_else(invalid)?}Ok(count)}
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(PagedBytes<{isize::MAX as usize}>,RetainedCloneProgress)>,ValueError>{if self.phase!=11||self.closing{return Err(invalid())}self.output.take_bytes(grant)}
    pub fn begin_close(&mut self){self.closing=true;self.output.begin_close();for owner in [&mut self.child,&mut self.forward,&mut self.inverse]{if let Some(owner)=owner.as_mut(){owner.begin_close()}}}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        for owner in [&self.child,&self.forward,&self.inverse]{if let Some(owner)=owner.as_ref(){return if owner.terminal_is_empty(){Ok(Self::child_drop())}else{owner.close_demands(body)}}}
        if let Some(owner)=self.projection.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{owner.next_close_demand(body)}}
        if let Some(owner)=self.original.as_ref(){return if owner.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<Edit<M>>>>())}else{owner.next_close_demand(body)}}
        self.output.retirement_demands()
    }
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.closing{return Err(invalid())}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
        for owner in [&mut self.child,&mut self.forward,&mut self.inverse]{if let Some(child)=owner.as_mut(){if !child.terminal_is_empty(){return child.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}drop(owner.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}}
        if let Some(owner)=self.projection.as_mut(){if !owner.terminal_is_empty(){return owner.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.projection.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
        if let Some(owner)=self.original.as_mut(){if !owner.terminal_is_empty(){return owner.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.original.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
        self.output.close_step(grant)
    }
    pub fn terminal_is_empty(&self)->bool{self.child.is_none()&&self.forward.is_none()&&self.inverse.is_none()&&self.projection.is_none()&&self.original.is_none()&&self.output.terminal_is_empty()}
}
impl<M:Sync> Drop for ArtifactOriginalEditWire<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original edit encoder requires paid closure");}}
#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
#[path="🧳️custody/🦀️.rs"]
mod custody;
pub use custody::ArtifactOriginalEditMessage;
#[path="🚪️admission/🦀️.rs"]
mod admission;
pub use admission::with_original_edit_message;
