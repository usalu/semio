//! 🪪️ Native metadata consumes original actor leases and ordered mutations through independent turns.
use super::*;
use semio_framework_hash::Hasher;
use semio_framework_value::{RetirementDemand,list::PagedList,retirement::{RetirementCursor,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes}};

fn refuses(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items!=0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
fn owned_text(original:&str)->Result<String,ValueError>{let mut copy=String::new();copy.try_reserve_exact(original.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native metadata text allocation failed"))?;copy.push_str(original);Ok(copy)}
fn text_demand(original:Option<&str>)->RetirementDemand{let bytes=original.map_or(0,str::len);RetirementDemand{copy_bytes:bytes+size_of::<Option<String>>(),capacity_bytes:bytes,depth:1,..Default::default()}}

pub(super) struct NativeEditMetadata<M:RetireOwned>{hasher:Hasher,actor_offset:usize,phase:u8,edit:std::mem::ManuallyDrop<Option<Edit<M>>>,meta:std::mem::ManuallyDrop<Option<MutationMeta>>,inverse_pending:std::mem::ManuallyDrop<Option<M>>}
impl<M:RetireOwned> NativeEditMetadata<M>{
    pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>()+size_of::<Box<Self>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
    pub fn admit(authority:&ArtifactStoreOneItemLiveAuthority,grant:RetainedCloneGrant)->Result<(Box<Self>,RetainedCloneProgress),ValueError>{
        let d=Self::constructor_demand();if !permits(d,grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native metadata frame exceeds original caller grant"));}
        let edit=Edit{id:String::new(),actor:None,line:None,forwards:Vec::new(),inverse:PagedList::default(),mutation_meta:Vec::new(),verb:None,sequence_number:authority.next_sequence_number(),started_at:String::new(),finished_at:None};
        let meta=MutationMeta{mutation_id:None,dependencies:Vec::new(),base_version:authority.base_applied_edit_count()as u64,author_id:None,timestamp:authority.next_clock(),undo_policy:UndoPolicy::ExactBaseOnly,payload_hash:None,semantic_kind:None,label:None,group_id:None,origin:Default::default(),transaction:None};
        Ok((Box::new(Self{hasher:Hasher::new(),actor_offset:0,phase:if authority.stamped_edit_id().is_some(){3}else{0},edit:std::mem::ManuallyDrop::new(Some(edit)),meta:std::mem::ManuallyDrop::new(Some(meta)),inverse_pending:std::mem::ManuallyDrop::new(None)}),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:0}))
    }
    pub fn next_demand(&self,authority:&ArtifactStoreOneItemLiveAuthority,inverse:&OriginalVectorCursor<M>)->Result<RetirementDemand,ValueError>{
        let edit=self.edit.as_ref().ok_or_else(||refuses("native metadata lost original edit"))?;
        Ok(match self.phase{
            0=>RetirementDemand{copy_bytes:8,depth:1,..Default::default()},
            1=>RetirementDemand{copy_bytes:authority.actor().len().saturating_sub(self.actor_offset).min(256).max(size_of::<u8>()),depth:1,..Default::default()},
            2=>RetirementDemand{copy_bytes:28,depth:1,..Default::default()},
            3=>{let bytes=authority.stamped_edit_id().map_or(21,str::len);RetirementDemand{copy_bytes:bytes+size_of::<String>(),capacity_bytes:bytes,depth:1,..Default::default()}},
            4|5=>RetirementDemand{copy_bytes:size_of::<semio_framework_value::SharedUtf8>()*2,depth:1,..Default::default()},
            6=>text_demand(authority.line_id()),7=>text_demand(authority.group_id()),
            8=>{let bytes=edit.id.len().checked_add(2).ok_or_else(||refuses("native mutation identity overflow"))?;RetirementDemand{copy_bytes:bytes+size_of::<MutationId>(),capacity_bytes:bytes,depth:1,..Default::default()}},
            9=>RetirementDemand{copy_bytes:size_of::<Vec<MutationMeta>>(),capacity_bytes:size_of::<MutationMeta>(),depth:1,..Default::default()},
            10=>RetirementDemand{copy_bytes:size_of::<MutationMeta>()*2,depth:1,..Default::default()},
            11=>RetirementDemand{copy_bytes:size_of::<Vec<M>>(),capacity_bytes:size_of::<M>(),depth:1,..Default::default()},
            12=>RetirementDemand{copy_bytes:size_of::<M>()*2,depth:1,..Default::default()},
            13 if inverse.remaining().is_empty()=>RetirementDemand{copy_bytes:size_of::<u8>(),depth:1,..Default::default()},
            13 if !edit.inverse.has_reserved_slot()=>RetirementDemand{capacity_bytes:edit.inverse.next_allocation_bytes().map_err(|_|refuses("native inverse page quote failed"))?,depth:edit.inverse.next_reserve_depth_demand().map_err(|_|refuses("native inverse depth quote failed"))?,..Default::default()},
            13=>{let mut demand=inverse.next_take_demand();demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<M>()).ok_or_else(||refuses("native inverse original transfer overflow"))?;demand},
            14=>RetirementDemand{copy_bytes:size_of::<M>(),depth:edit.inverse.next_push_depth_demand().map_err(|_|refuses("native inverse original placement depth failed"))?,..Default::default()},
            15=>RetirementDemand{copy_bytes:size_of::<Edit<M>>()+size_of::<Option<Edit<M>>>(),depth:1,..Default::default()},
            _=>return Err(refuses("native metadata phase has no original authority")),
        })
    }
    pub fn step(&mut self,authority:&ArtifactStoreOneItemLiveAuthority,forward:&mut Option<M>,inverse:&mut OriginalVectorCursor<M>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if self.phase==15{return Ok(Default::default());}let d=self.next_demand(authority,inverse)?;if !permits(d,grant){return Ok(Default::default());}
        let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes};
        match self.phase{
            0=>{self.hasher.update(&(authority.actor().len()as u64).to_le_bytes());self.phase=1;},
            1=>{let bytes=authority.actor().as_bytes();if self.actor_offset==bytes.len(){self.phase=2;}else{let end=self.actor_offset+d.copy_bytes;self.hasher.update(&bytes[self.actor_offset..end]);self.actor_offset=end;}},
            2=>{self.hasher.update(&authority.next_sequence_number().to_le_bytes());let clock=authority.next_clock();for part in [clock.actor,clock.physical_ms,clock.logical]{self.hasher.update(&part.to_le_bytes());}self.phase=3;},
            3=>{let id=if let Some(original)=authority.stamped_edit_id(){owned_text(original)?}else{let digest=self.hasher.finalize();let mut id=String::new();id.try_reserve_exact(21).map_err(|_|refuses("native edit identity allocation failed"))?;id.push_str("edit-");let hex=b"0123456789abcdef";for byte in &digest.as_bytes()[..8]{id.push(hex[usize::from(byte>>4)]as char);id.push(hex[usize::from(byte&15)]as char);}id};self.edit.as_mut().unwrap().id=id;self.phase=4;},
            4=>{let(owner,receipt)=authority.admit_actor_lease(grant)?;self.edit.as_mut().unwrap().actor=Some(owner);p.copied_bytes=receipt.copied_bytes+size_of::<semio_framework_value::SharedUtf8>();self.phase=5;},
            5=>{let(owner,receipt)=authority.admit_actor_lease(grant)?;self.meta.as_mut().unwrap().author_id=Some(ActorId(owner));p.copied_bytes=receipt.copied_bytes+size_of::<semio_framework_value::SharedUtf8>();self.phase=6;},
            6=>{self.edit.as_mut().unwrap().line=authority.line_id().map(owned_text).transpose()?;self.phase=7;},
            7=>{self.meta.as_mut().unwrap().group_id=authority.group_id().map(owned_text).transpose()?;self.phase=8;},
            8=>{let original=self.edit.as_ref().unwrap().id.as_str();let mut identity=String::new();identity.try_reserve_exact(original.len()+2).map_err(|_|refuses("native mutation identity allocation failed"))?;identity.push_str(original);identity.push_str("#0");self.meta.as_mut().unwrap().mutation_id=Some(MutationId(identity));self.phase=9;},
            9=>{self.edit.as_mut().unwrap().mutation_meta.try_reserve_exact(1).map_err(|_|refuses("native metadata row allocation failed"))?;self.phase=10;},
            10=>{let edit=self.edit.as_mut().unwrap();if edit.mutation_meta.capacity()<1{return Err(refuses("native metadata row has no admitted backing"));}edit.mutation_meta.push(self.meta.take().ok_or_else(||refuses("native metadata lost original row"))?);self.phase=11;},
            11=>{self.edit.as_mut().unwrap().forwards.try_reserve_exact(1).map_err(|_|refuses("native forward allocation failed"))?;self.phase=12;},
            12=>{let edit=self.edit.as_mut().unwrap();if edit.forwards.capacity()<1{return Err(refuses("native forward has no admitted backing"));}edit.forwards.push(forward.take().ok_or_else(||refuses("native metadata lost original forward"))?);self.phase=13;},
            13 if inverse.remaining().is_empty()=>self.phase=15,
            13 if !self.edit.as_ref().unwrap().inverse.has_reserved_slot()=>{let progress=self.edit.as_mut().unwrap().inverse.reserve_one(grant.maximum_capacity_bytes).map_err(|_|refuses("native inverse page allocation failed"))?;p.retained_capacity_bytes=progress.allocated_bytes;},
            13=>{let(original,receipt)=inverse.take_front(grant)?.ok_or_else(||refuses("native inverse lost original front"))?;*self.inverse_pending=Some(original);p.copied_bytes=receipt.copied_bytes+size_of::<M>();self.phase=14;},
            14=>{let progress=self.edit.as_mut().unwrap().inverse.place_reserved(&mut self.inverse_pending,grant.maximum_copy_bytes).map_err(|_|refuses("native inverse original placement failed"))?;if !progress.progressed{return Ok(Default::default());}p.copied_bytes=progress.placed_bytes;self.phase=13;},
            _=>return Err(refuses("native metadata lacks original phase")),
        }
        admit_retained_clone_progress(grant,p,"original native edit metadata")
    }
    pub fn ready(&self)->bool{self.phase==15&&self.edit.is_some()&&self.meta.is_none()&&self.inverse_pending.is_none()}
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<Option<(Edit<M>,RetainedCloneProgress)>,ValueError>{if !self.ready(){return Ok(None);}let d=RetirementDemand{copy_bytes:size_of::<Edit<M>>()+size_of::<Option<Edit<M>>>(),depth:1,..Default::default()};if !permits(d,grant){return Ok(None);}Ok(self.edit.take().map(|original|(original,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
    pub fn terminal_is_empty(&self)->bool{self.edit.is_none()&&self.meta.is_none()&&self.inverse_pending.is_none()}
}
impl<M:RetireOwned+Send+'static> RetireOwned for NativeEditMetadata<M>{
    fn retirement(mut self)->Box<dyn RetirementCursor>{sequence(vec![deferred(self.edit.take()),deferred(self.meta.take()),deferred(self.inverse_pending.take())])}
    fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&*self.edit),deferred_birth_bytes_for(&*self.meta),deferred_birth_bytes_for(&*self.inverse_pending)])}
    fn controlled_retirement_supported()->bool{M::controlled_retirement_supported()}
}
impl<M:RetireOwned> Drop for NativeEditMetadata<M>{fn drop(&mut self){let empty=self.terminal_is_empty();assert!(empty||std::thread::panicking(),"native metadata abandoned original retained fields");if empty{unsafe{std::mem::ManuallyDrop::drop(&mut self.edit);std::mem::ManuallyDrop::drop(&mut self.meta);std::mem::ManuallyDrop::drop(&mut self.inverse_pending);}}}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
