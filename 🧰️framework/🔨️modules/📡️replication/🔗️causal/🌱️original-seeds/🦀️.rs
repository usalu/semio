//! 🌱️ Original local seed identities occupy a private suffix until one allocation-free publication.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,retained_clone_grant_admits};
use std::mem::size_of;
#[derive(Debug,PartialEq)]
pub(super) struct OriginalSeedStage{base:usize,epoch:u64,source:usize,scan_applied:u16,scan_envelope:u16,scan_found:bool,census_complete:bool,seeded:usize,candidate:Option<(usize,usize)>,candidate_applied:u16,candidate_envelope:u16}
/// 🎟️ The defining DAG issues exclusive custody of its actual admitted applied-identity backing.
pub struct OriginalMutationDagSeeds{base:usize,epoch:u64,source:usize,active:bool}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original causal seed reservation belongs to a different backing or frontier")}
fn demand<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
impl MutationDag{
    pub(super) fn visible_applied_len(&self)->usize{self.original_seed_stage.as_ref().map_or(self.applied.len(),|stage|stage.base)}
    pub(super) fn visible_applied(&self)->impl Iterator<Item=&String>{self.applied.iter().take(self.visible_applied_len())}
    pub fn original_seed_is_applied(&self,id:&str)->bool{self.visible_applied().any(|known|known==id)}
    pub fn original_seed_birth_demands()->RetirementDemand{demand::<(Option<OriginalSeedStage>,OriginalMutationDagSeeds,u64)>()}
    pub fn admit_original_seeds(&mut self,grant:RetainedCloneGrant)->Result<Option<(OriginalMutationDagSeeds,RetainedCloneProgress)>,ValueError>{
        if self.original_seed_stage.is_some()||self.applied.birth_bytes()!=0{return Err(invalid())}let d=Self::original_seed_birth_demands();if !retained_clone_grant_admits(grant,d){return Ok(None)}let epoch=self.original_seed_epoch.checked_add(1).ok_or_else(invalid)?;let source=self.applied.slots.as_ptr()as usize;let base=self.applied.len();self.original_seed_epoch=epoch;self.original_seed_stage=Some(OriginalSeedStage{base,epoch,source,scan_applied:self.applied.head,scan_envelope:self.envelopes.head,scan_found:false,census_complete:false,seeded:0,candidate:None,candidate_applied:MUTATION_DAG_SLOT_NONE,candidate_envelope:MUTATION_DAG_SLOT_NONE});Ok(Some((OriginalMutationDagSeeds{base,epoch,source,active:true},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))
    }
    fn check_original_seeds(&self,owner:&OriginalMutationDagSeeds)->Result<(),ValueError>{let stage=self.original_seed_stage.as_ref().ok_or_else(invalid)?;if !owner.active||owner.source!=self.applied.slots.as_ptr()as usize||stage.source!=owner.source||stage.base!=owner.base||stage.epoch!=owner.epoch{return Err(invalid())}Ok(())}
    pub fn original_seed_append_demands()->RetirementDemand{demand::<(OriginalSeedStage,String,usize)>()}
    pub fn stage_original_seed(&mut self,owner:&OriginalMutationDagSeeds,identity:&mut Option<semio_framework_replication_ids::MutationId>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        self.check_original_seeds(owner)?;let id=&identity.as_ref().ok_or_else(invalid)?.0;if id.len()>MUTATION_DAG_IDENTIFIER_BYTES{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original causal identity exceeds its authored byte limit"))}let d=Self::original_seed_append_demands();if !retained_clone_grant_admits(grant,d){return Ok(None)}
        let stage=self.original_seed_stage.as_mut().unwrap();
        if !stage.census_complete{
            if stage.scan_applied==MUTATION_DAG_SLOT_NONE{stage.census_complete=true;}
            else if stage.scan_envelope==MUTATION_DAG_SLOT_NONE{stage.seeded+=usize::from(!stage.scan_found);stage.scan_applied=self.applied.next[usize::from(stage.scan_applied)];stage.scan_envelope=self.envelopes.head;stage.scan_found=false;}
            else{let known=unsafe{self.applied.slot(usize::from(stage.scan_applied)).assume_init_ref()};let envelope=unsafe{self.envelopes.slot(usize::from(stage.scan_envelope)).assume_init_ref()};if known==&envelope.mutation_id.0{stage.scan_found=true;stage.scan_envelope=MUTATION_DAG_SLOT_NONE;}else{stage.scan_envelope=self.envelopes.next[usize::from(stage.scan_envelope)];}}
        }else if let Some(binding)=stage.candidate{
            if binding!=(id.as_ptr()as usize,id.len()){return Err(invalid())}
            if stage.candidate_applied!=MUTATION_DAG_SLOT_NONE{let slot=usize::from(stage.candidate_applied);let known=unsafe{self.applied.slot(slot).assume_init_ref()};if known==id{return Err(invalid())}stage.candidate_applied=self.applied.next[slot];}
            else if stage.candidate_envelope!=MUTATION_DAG_SLOT_NONE{let slot=usize::from(stage.candidate_envelope);let known=unsafe{self.envelopes.slot(slot).assume_init_ref()};if known.mutation_id.0==*id{return Err(invalid())}stage.candidate_envelope=self.envelopes.next[slot];}
            else{if self.envelopes.len()+stage.seeded>=MUTATION_DAG_CAPACITY{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original causal seed capacity is exhausted"))}self.applied.push_reserved(identity.take().unwrap().0);stage.seeded+=1;stage.candidate=None;}
        }else{stage.candidate=Some((id.as_ptr()as usize,id.len()));stage.candidate_applied=self.applied.head;stage.candidate_envelope=self.envelopes.head;}
        Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
    }
    pub fn original_seed_commit_demands()->RetirementDemand{demand::<(Option<OriginalSeedStage>,Option<OriginalMutationDagSeeds>)>()}
    /// 🔎️ Checks the exact original seed suffix and its funded final handoff before any visible parent root moves.
    pub fn can_commit_original_seeds(&self,owner:&OriginalMutationDagSeeds,grant:RetainedCloneGrant)->Result<bool,ValueError>{self.check_original_seeds(owner)?;if self.original_seed_stage.as_ref().is_some_and(|stage|stage.candidate.is_some()){return Err(invalid())}Ok(retained_clone_grant_admits(grant,Self::original_seed_commit_demands()))}
    pub fn commit_original_seeds(&mut self,owner:&mut Option<OriginalMutationDagSeeds>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{if !self.can_commit_original_seeds(owner.as_ref().ok_or_else(invalid)?,grant)?{return Ok(None)}let d=Self::original_seed_commit_demands();self.original_seed_stage=None;let mut original=owner.take().unwrap();original.active=false;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
    pub fn original_seed_abort_demands(&self,owner:&OriginalMutationDagSeeds)->Result<RetirementDemand,ValueError>{self.check_original_seeds(owner)?;Ok(if self.applied.len()>owner.base{demand::<(Option<String>,usize)>()}else{Self::original_seed_commit_demands()})}
    pub fn abort_original_seed_step(&mut self,owner:&mut Option<OriginalMutationDagSeeds>,grant:RetainedCloneGrant)->Result<Option<(Option<String>,RetainedCloneProgress)>,ValueError>{let original=owner.as_ref().ok_or_else(invalid)?;let d=self.original_seed_abort_demands(original)?;if !retained_clone_grant_admits(grant,d){return Ok(None)}if self.applied.len()>original.base{let identity=self.applied.pop().ok_or_else(invalid)?;return Ok(Some((Some(identity),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}self.original_seed_stage=None;let mut original=owner.take().unwrap();original.active=false;Ok(Some((None,RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
}
impl Drop for OriginalMutationDagSeeds{fn drop(&mut self){assert!(!self.active||std::thread::panicking(),"original causal seed reservation requires commit or complete abort");}}
#[cfg(test)]
#[path="🧪️.rs"]
mod tests;
