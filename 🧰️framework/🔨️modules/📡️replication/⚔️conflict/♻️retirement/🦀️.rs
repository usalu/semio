//! 🧯️ Conflict and causal-envelope owners close without materializing or discarding ungranted payloads.
use crate::{Conflict,ConflictKind,MutationEnvelope,MutationMessageRetirement,ids::{ActorId,MutationId}};
use semio_framework_value::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::{ManuallyDrop,size_of};

fn release_empty<T>(values:&mut Vec<T>)->usize {assert!(values.is_empty());let bytes=values.capacity()*size_of::<T>();drop(std::mem::take(values));bytes}
fn permits(grant:RetainedCloneGrant,copy:usize,release:usize,depth:usize)->bool {grant.maximum_items!=0&&grant.maximum_copy_bytes>=copy&&grant.maximum_release_bytes>=release&&grant.maximum_depth>=depth}

struct MutationEnvelopeRetirement {
    strings:ManuallyDrop<[Option<String>;10]>,
    dependencies:ManuallyDrop<Vec<MutationId>>,
    targets:ManuallyDrop<Vec<String>>,
    payloads:ManuallyDrop<[Option<Vec<u8>>;2]>,
    active:ManuallyDrop<Option<String>>,
    phase:u8,
}

impl MutationEnvelopeRetirement {
    fn new(envelope:MutationEnvelope)->Self {
        let(transaction,tool)=envelope.transaction.map_or((None,None),|transaction|(Some(transaction.id),Some(transaction.tool)));
        Self {strings:ManuallyDrop::new([Some(envelope.mutation_id.0),Some(envelope.document_id.0),Some(envelope.actor.0),Some(envelope.diff.schema.0),Some(envelope.inverse.schema.0),envelope.observed.map(|id|id.0),transaction,tool,envelope.verb,envelope.line]),dependencies:ManuallyDrop::new(envelope.dependencies),targets:ManuallyDrop::new(envelope.target),payloads:ManuallyDrop::new([Some(envelope.diff.payload),Some(envelope.inverse.payload)]),active:ManuallyDrop::new(None),phase:0}
    }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {Ok(if self.active.is_some(){0}else{match self.phase {0=>usize::from(self.strings.iter().any(Option::is_some))*size_of::<String>(),1=>usize::from(!self.dependencies.is_empty())*size_of::<MutationId>(),2=>usize::from(!self.targets.is_empty())*size_of::<String>(),_=>0}})}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {Ok(self.active.as_ref().map_or_else(||match self.phase {1 if self.dependencies.is_empty()=>self.dependencies.capacity()*size_of::<MutationId>(),2 if self.targets.is_empty()=>self.targets.capacity()*size_of::<String>(),3|4=>self.payloads[usize::from(self.phase-3)].as_ref().map_or(0,Vec::capacity),_=>0},String::capacity))}
    fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(usize::from(!self.terminal_is_empty()))}
    fn terminal_is_empty(&self)->bool {self.phase==5&&self.active.is_none()&&self.strings.iter().all(Option::is_none)&&self.payloads.iter().all(Option::is_none)&&self.dependencies.is_empty()&&self.dependencies.capacity()==0&&self.targets.is_empty()&&self.targets.capacity()==0}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let idle=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(idle));}
        if !permits(grant,self.next_copy_byte_demand()?,self.next_release_byte_demand()?,self.next_depth_demand()?){return Ok(RetainedCloneStep::Progress(idle));}
        let progress=if let Some(active)=self.active.as_ref(){let bytes=active.capacity();drop(self.active.take());RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}}
        else {match self.phase {
            0=>if let Some(value)=self.strings.iter_mut().find_map(Option::take){*self.active=Some(value);RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<String>(),..idle}}else{self.phase=1;RetainedCloneProgress {copied_items:1,..idle}},
            1=>if let Some(value)=self.dependencies.pop(){*self.active=Some(value.0);RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<MutationId>(),..idle}}else{let bytes=release_empty(&mut self.dependencies);self.phase=2;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
            2=>if let Some(value)=self.targets.pop(){*self.active=Some(value);RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<String>(),..idle}}else{let bytes=release_empty(&mut self.targets);self.phase=3;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
            3|4=>{let index=usize::from(self.phase-3);let bytes=self.payloads[index].as_ref().map_or(0,Vec::capacity);drop(self.payloads[index].take());self.phase+=1;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"causal retirement lost its exact phase owner")),
        }};
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
impl Drop for MutationEnvelopeRetirement {fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"causal envelope abandoned original payload ownership");if empty{unsafe{ManuallyDrop::drop(&mut self.strings);ManuallyDrop::drop(&mut self.dependencies);ManuallyDrop::drop(&mut self.targets);ManuallyDrop::drop(&mut self.payloads);ManuallyDrop::drop(&mut self.active);}}}}

struct ConflictRetirement {
    value:ManuallyDrop<Option<Conflict>>,
    message:Option<MutationMessageRetirement>,
    envelope:Option<MutationEnvelopeRetirement>,
    active:ManuallyDrop<Option<String>>,
    phase:u8,
}
impl ConflictRetirement {
    fn new(value:Conflict)->Self {Self {value:ManuallyDrop::new(Some(value)),message:None,envelope:None,active:ManuallyDrop::new(None),phase:0}}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {
        if let Some(message)=self.message.as_ref(){return Ok(message.next_copy_byte_demand());}if let Some(envelope)=self.envelope.as_ref(){return envelope.next_copy_byte_demand();}if self.active.is_some()||self.terminal_is_empty(){return Ok(0);}
        let value=self.value.as_ref().expect("live original conflict owner");Ok(match self.phase {0=>usize::from(!value.messages.is_empty())*size_of::<crate::MutationMessage>(),1=>match &value.kind {ConflictKind::Quarantined {envelopes}=>usize::from(!envelopes.is_empty())*size_of::<MutationEnvelope>(),ConflictKind::Degraded {edit_ids}=>usize::from(!edit_ids.is_empty())*size_of::<String>()},2=>usize::from(!value.actors.is_empty())*size_of::<ActorId>(),_=>0})
    }
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {
        if let Some(message)=self.message.as_ref(){return Ok(message.next_release_byte_demand());}if let Some(envelope)=self.envelope.as_ref(){return envelope.next_release_byte_demand();}if let Some(active)=self.active.as_ref(){return Ok(active.capacity());}if self.terminal_is_empty(){return Ok(0);}
        let value=self.value.as_ref().expect("live original conflict owner");Ok(match self.phase {0 if value.messages.is_empty()=>value.messages.capacity()*size_of::<crate::MutationMessage>(),1=>match &value.kind {ConflictKind::Quarantined {envelopes} if envelopes.is_empty()=>envelopes.capacity()*size_of::<MutationEnvelope>(),ConflictKind::Degraded {edit_ids} if edit_ids.is_empty()=>edit_ids.capacity()*size_of::<String>(),_=>0},2 if value.actors.is_empty()=>value.actors.capacity()*size_of::<ActorId>(),3=>value.id.0.capacity(),_=>0})
    }
    fn next_depth_demand(&self)->Result<usize,ValueError> {if self.terminal_is_empty(){return Ok(0);}if let Some(message)=self.message.as_ref(){return message.next_depth_demand().map(|depth|depth+1);}self.envelope.as_ref().map_or(Ok(1),|envelope|envelope.next_depth_demand().map(|depth|depth+1))}
    fn terminal_is_empty(&self)->bool {self.value.is_none()&&self.message.is_none()&&self.envelope.is_none()&&self.active.is_none()}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let idle=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(idle));}
        if !permits(grant,self.next_copy_byte_demand()?,self.next_release_byte_demand()?,self.next_depth_demand()?){return Ok(RetainedCloneStep::Progress(idle));}
        let progress=if let Some(message)=self.message.as_mut(){let progress=message.close_step(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant});if message.terminal_is_empty(){self.message=None;}progress}
        else if let Some(envelope)=self.envelope.as_mut(){let step=envelope.close_step(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant})?;if matches!(step,RetainedCloneStep::Complete(_))&&!envelope.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"causal child reported completion while retaining ownership"));}if envelope.terminal_is_empty(){self.envelope=None;}step.progress()}
        else if let Some(active)=self.active.as_ref(){let bytes=active.capacity();drop(self.active.take());RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}}
        else {let value=self.value.as_mut().expect("live original conflict owner");match self.phase {
            0=>if let Some(message)=value.messages.pop(){self.message=Some(MutationMessageRetirement::new(message));RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<crate::MutationMessage>(),..idle}}else{let bytes=release_empty(&mut value.messages);self.phase=1;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
            1=>match &mut value.kind {
                ConflictKind::Quarantined {envelopes}=>if let Some(envelope)=envelopes.pop(){self.envelope=Some(MutationEnvelopeRetirement::new(envelope));RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<MutationEnvelope>(),..idle}}else{let bytes=release_empty(envelopes);self.phase=2;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
                ConflictKind::Degraded {edit_ids}=>if let Some(identity)=edit_ids.pop(){*self.active=Some(identity);RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<String>(),..idle}}else{let bytes=release_empty(edit_ids);self.phase=2;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
            },
            2=>if let Some(actor)=value.actors.pop(){*self.active=Some(actor.0);RetainedCloneProgress {copied_items:1,copied_bytes:size_of::<ActorId>(),..idle}}else{let bytes=release_empty(&mut value.actors);self.phase=3;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
            3=>{let bytes=value.id.0.capacity();drop(self.value.take());self.phase=4;RetainedCloneProgress {copied_items:1,released_bytes:bytes,..idle}},
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"conflict retirement lost its original phase owner")),
        }};
        if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"conflict retirement exceeded its independent grant"));}
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
}
impl Drop for ConflictRetirement {fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"conflict abandoned original nested owners");if empty{unsafe{ManuallyDrop::drop(&mut self.value);ManuallyDrop::drop(&mut self.active);}}}}

enum ProtocolOwner {Conflict(ConflictRetirement),Envelope(MutationEnvelopeRetirement)}
/// 🔒️ An inline typed owner retains the full original fatal conflict or causal envelope until exact cleanup.
pub struct ProtocolConflictRetirement {owner:ProtocolOwner}
impl ProtocolConflictRetirement {
    pub fn new(conflict:Conflict)->Self {Self {owner:ProtocolOwner::Conflict(ConflictRetirement::new(conflict))}}
    pub fn causal_envelope(envelope:MutationEnvelope)->Self {Self {owner:ProtocolOwner::Envelope(MutationEnvelopeRetirement::new(envelope))}}
}
impl ErasedSnapshotRetirement for ProtocolConflictRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {match &mut self.owner {ProtocolOwner::Conflict(owner)=>owner.close_step(grant),ProtocolOwner::Envelope(owner)=>owner.close_step(grant)}}
    fn terminal_is_empty(&self)->bool {match &self.owner {ProtocolOwner::Conflict(owner)=>owner.terminal_is_empty(),ProtocolOwner::Envelope(owner)=>owner.terminal_is_empty()}}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError> {match &self.owner {ProtocolOwner::Conflict(owner)=>owner.next_copy_byte_demand(),ProtocolOwner::Envelope(owner)=>owner.next_copy_byte_demand()}}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError> {Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError> {match &self.owner {ProtocolOwner::Conflict(owner)=>owner.next_release_byte_demand(),ProtocolOwner::Envelope(owner)=>owner.next_release_byte_demand()}}
    fn next_depth_demand(&self)->Result<usize,ValueError> {match &self.owner {ProtocolOwner::Conflict(owner)=>owner.next_depth_demand(),ProtocolOwner::Envelope(owner)=>owner.next_depth_demand()}}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
