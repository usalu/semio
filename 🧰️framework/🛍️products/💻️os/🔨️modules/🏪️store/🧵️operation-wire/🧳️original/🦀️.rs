//! 🧳️ Original typed operation custody survives bounded encoding, protocol handoff and cancellation.
use super::{ArtifactOperationWire,ArtifactPreparedOperationCursor,ArtifactPreparedOperationOutput};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedOwnedProjection,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits};
use std::mem::{ManuallyDrop,size_of};

/// 📦️ The genuine original mutation stays retained until its actual codec aliases close.
pub struct ArtifactOriginalOperationBytes<M:Sync>{
    original:ManuallyDrop<Option<RetainedOwnedProjection<M>>>,
    wire:ArtifactOperationWire<M>,
    cursor:ArtifactPreparedOperationCursor,
    output:ArtifactPreparedOperationOutput,
    pending:[u8;64],
    length:usize,
    offset:usize,
    phase:u8,
    closing:bool,
    paged:bool,
}

fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original operation bytes have a different ownership frontier")}
fn progress(step:super::ArtifactPreparedOperationProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:step.processed_items,copied_bytes:step.copied_bytes,retained_capacity_bytes:step.retained_capacity_bytes,released_bytes:step.released_bytes}}

impl<M:Sync> ArtifactOriginalOperationBytes<M>{
    /// 📏️ Quotes only the original alias transfer and empty native codec/output scaffolds.
    pub fn birth_demands()->RetirementDemand{header::<Self>()}

    /// 🎟️ Moves the actual admitted mutation alias only after the complete constructor quote.
    pub fn admit(source:&mut Option<RetainedOwnedProjection<M>>,wire:ArtifactOperationWire<M>,maximum_payload:usize,maximum_capacity:usize,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{
        let d=Self::birth_demands();if !retained_clone_grant_admits(grant,d){return Ok(None)}
        let original=source.as_ref().ok_or_else(invalid)?;let borrowed=original.borrow()?;wire.source(borrowed.get()).ok_or_else(invalid)?;
        let output=ArtifactPreparedOperationOutput::empty(maximum_payload,maximum_capacity)?;
        Ok(Some((Self{original:ManuallyDrop::new(source.take()),wire,cursor:Default::default(),output,pending:[0;64],length:0,offset:0,phase:0,closing:false,paged:false},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
    }

    /// 📄️ Keeps encoded bytes in their original paid pages for a bounded enclosing protocol writer.
    pub fn admit_paged(source:&mut Option<RetainedOwnedProjection<M>>,wire:ArtifactOperationWire<M>,maximum_payload:usize,maximum_capacity:usize,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{let owner=Self::admit(source,wire,maximum_payload,maximum_capacity,grant)?;Ok(owner.map(|(mut owner,p)|{owner.paged=true;(owner,p)}))}

    pub fn payload_ready(&self)->bool{self.phase==5&&!self.closing}
    pub fn payload_len(&self)->Result<usize,ValueError>{if !self.paged||!self.payload_ready(){return Err(invalid())}Ok(self.output.len())}
    pub fn read_payload(&self,offset:usize,output:&mut[u8])->Result<usize,ValueError>{let length=self.payload_len()?;if offset>length{return Err(invalid())}let count=(length-offset).min(output.len());for(index,byte)in output[..count].iter_mut().enumerate(){*byte=self.output.byte_at(offset+index).ok_or_else(invalid)?;}Ok(count)}

    /// 📋️ Borrows the initialized original output extent before its exact protocol transfer.
    pub fn bytes(&self)->Option<&[u8]>{if self.phase==5&&!self.closing{self.output.contiguous_prefix()}else{None}}

    /// 📏️ Every quote selects the actual codec, page, contiguous or source-return frontier.
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing{return self.close_demands(body)}
        let mut d=match self.phase{
            0 if self.offset<self.length=>self.output.append_demands((self.length-self.offset).min(body))?,
            0 if !self.cursor.complete=>RetirementDemand{copy_bytes:self.cursor.next_minimum_copy_bytes().max(64),capacity_bytes:self.cursor.next_capacity_byte_demand()?,depth:64,..Default::default()},
            0=>header::<u8>(),
            1=>self.cursor.retirement_demands()?,
            2 if self.paged=>header::<u8>(),
            2=>self.output.contiguous_demands(body)?,
            3 if self.original.as_ref().is_some_and(|owner|!owner.terminal_is_empty())=>self.original.as_ref().unwrap().next_close_demand(body)?,
            3=>header::<Option<RetainedOwnedProjection<M>>>(),
            4=>header::<u8>(),
            5=>header::<Vec<u8>>(),
            _=>return Err(invalid()),
        };
        d.copy_bytes=d.copy_bytes.checked_add(size_of::<usize>()*2+size_of::<u8>()).ok_or_else(invalid)?;d.depth=d.depth.max(1);Ok(d)
    }

    /// ✍️ Advances one actual bounded native action; its embedding original issuer owns the receipt.
    pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.closing{return Err(invalid())}if self.phase==5{return Ok(RetainedCloneStep::Complete(Default::default()))}
        let d=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
        let metadata=size_of::<usize>()*2+size_of::<u8>();let child=RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-metadata,..grant};
        let p=match self.phase{
            0 if self.offset<self.length=>{let p=self.output.append(&self.pending[self.offset..self.length],child)?;self.offset+=p.copied_bytes;p},
            0 if !self.cursor.complete=>{let original=self.original.as_ref().ok_or_else(invalid)?.borrow()?;let source=self.wire.source(original.get()).ok_or_else(invalid)?;let step=self.cursor.advance(source,&mut self.pending,child).map_err(|error|error.reason)?;self.length=step.written_bytes;self.offset=0;progress(step)},
            0=>{self.phase=1;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}},
            1=>{let step=self.cursor.close(child).map_err(|error|error.reason)?;if step.complete{self.phase=2}progress(step)},
            2 if self.paged=>{self.phase=3;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}},
            2=>{let step=self.output.prepare_contiguous(child)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=3}step.progress()},
            3 if self.original.as_ref().is_some_and(|owner|!owner.terminal_is_empty())=>self.original.as_mut().unwrap().close_step(child)?.progress(),
            3=>{self.original.take();self.phase=4;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<RetainedOwnedProjection<M>>>(),..Default::default()}},
            4=>{self.phase=5;RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u8>(),..Default::default()}},
            _=>return Err(invalid()),
        };
        let p=p.checked_add(RetainedCloneProgress{copied_items:usize::from(p.copied_items==0),copied_bytes:metadata,..Default::default()})?;
        if !p.fits(grant){return Err(invalid().with_retained_progress(p))}Ok(if self.phase==5{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
    }

    /// 🤝️ Transfers the same paid contiguous bytes only after all original read aliases return.
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<u8>,RetainedCloneProgress)>,ValueError>{
        if self.phase!=5||self.closing||self.paged{return Err(invalid())}
        let d=self.demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(None)}
        let Some((bytes,mut p))=self.output.take_contiguous(grant)?else{return Ok(None)};self.phase=6;p.copied_bytes+=size_of::<usize>()*2+size_of::<u8>();Ok(Some((bytes,p)))
    }

    /// 🛑️ Stops production while retaining every initialized byte and the actual original source.
    pub fn begin_close(&mut self){self.closing=true;self.output.begin_close();}

    /// 📏️ Quotes the actual retained byte owner, codec frontier and original source lease.
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.cursor.closed{return self.cursor.retirement_demands()}
        if !self.output.terminal_is_empty(){return self.output.retirement_demands()}
        if let Some(original)=self.original.as_ref(){return if original.terminal_is_empty(){Ok(header::<Option<RetainedOwnedProjection<M>>>())}else{original.next_close_demand(body)}}
        Ok(Default::default())
    }

    /// ♻️ Returns actual backings and original source aliases under independently supplied close currencies.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if !self.closing{return Err(invalid())}let d=self.close_demands(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !self.cursor.closed{return self.cursor.close(grant).map(|step|RetainedCloneStep::Progress(progress(step))).map_err(|error|error.reason)}
        if !self.output.terminal_is_empty(){return self.output.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}
        if let Some(original)=self.original.as_mut(){if !original.terminal_is_empty(){return original.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()))}self.original.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }

    /// 🫙️ Confirms byte handoff or physical return together with every actual source lease.
    pub fn terminal_is_empty(&self)->bool{self.cursor.closed&&self.output.terminal_is_empty()&&self.original.is_none()}
}
impl<M:Sync> Drop for ArtifactOriginalOperationBytes<M>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original operation encoder requires complete paid closure");}}

#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
