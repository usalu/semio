//! 💧️ Change hydration phases: the changes, checkpoints, alternatives and pins hydration of the hydration step. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: ArtifactPack + semio_os_store_text_io::ArtifactStoreSnapshot + semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
{
    /// 🔄 Hydrates the change records.
    pub(super) fn step_hydrate_changes(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        if self.target==PersistedDocumentHydrationTarget::Envelope{return self.step_original_changes(cx)}
        let fold = self.fold.as_mut().expect("history fold remains retained");
        if let Some(change) = fold.changes.get_mut(self.operation_index) {
            let change = semio_os_vcs::Change { id: std::mem::take(&mut change.id), edit_ids: std::mem::take(&mut change.edit_ids), description: change.description.take(), saved_at: std::mem::take(&mut change.saved_at) };
            if let Err(change) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.changes.try_push(change) {
                *self.active = Some(Box::new(semio_os_store_read::ArtifactStoreHistoryMetadataRetirement::change(change)));
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = Phase::HydrateCheckpoints;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 📍 Hydrates the checkpoints.
    pub(super) fn step_hydrate_checkpoints(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        if self.target==PersistedDocumentHydrationTarget::Envelope{return self.step_original_checkpoints(cx)}
        let fold = self.fold.as_mut().expect("history fold remains retained");
        if let Some(source) = fold.checkpoints.get_mut(self.operation_index) {
            let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
            if self.pin_index == 0 {
                let checkpoint = semio_os_vcs::Checkpoint { id: std::mem::take(&mut source.id), change_ids: std::mem::take(&mut source.change_ids), parent_id: source.parent_id.take(), authors: Vec::with_capacity(source.authors.len()), message: source.message.take(), timestamp: std::mem::take(&mut source.timestamp), composition_pins: Vec::new() };
                if let Err(checkpoint) = envelope.vcs.checkpoints.try_push(checkpoint) {
                    *self.active = Some(Box::new(semio_os_store_read::ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint)));
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                self.pin_index = 1;
            } else if let Some(author) = source.authors.get_mut(self.pin_index - 1) {
                envelope.vcs.checkpoints.get_mut(self.operation_index).expect("hydrated checkpoint remains indexed").authors.push(semio_os_vcs::Author { id: std::mem::take(&mut author.id), name: std::mem::take(&mut author.name), avatar: author.avatar.take() });
                self.pin_index += 1;
            } else {
                self.operation_index += 1;
                self.pin_index = 0;
            }
        } else {
            self.operation_index = 0;
            self.pin_index = 0;
            self.phase = Phase::HydrateAlternatives;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🔀 Hydrates the alternatives.
    pub(super) fn step_hydrate_alternatives(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        if self.target==PersistedDocumentHydrationTarget::Envelope{return self.step_original_alternatives(cx)}
        let fold = self.fold.as_mut().expect("history fold remains retained");
        if let Some(alternative) = fold.alternatives.get_mut(self.operation_index) {
            let alternative = semio_os_vcs::Alternative { id: std::mem::take(&mut alternative.id), name: std::mem::take(&mut alternative.name), checkpoint_ids: std::mem::take(&mut alternative.checkpoint_ids) };
            if let Err(alternative) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.alternatives.try_push(alternative) {
                *self.active = Some(Box::new(semio_os_store_read::ArtifactStoreHistoryMetadataRetirement::alternative(alternative)));
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = Phase::HydratePins;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 📌 Hydrates the pins.
    pub(super) fn step_hydrate_pins(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};
        let Some(source) = self.fold.as_ref().expect("history fold remains retained").checkpoints.get(self.pin_group_index) else {
            self.record_index = 0;
            self.operation_index = 0;
            self.phase = if self.target==PersistedDocumentHydrationTarget::Envelope{Phase::RetireHistory}else{Phase::PrepareReplay};
            cx.consume_fuel(1);
            return PersistedDocumentHydrationStep::Pending(self.progress());
        };
        let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
        let Some(checkpoint) = envelope.vcs.checkpoints.get_mut(self.pin_group_index) else {
            return self.reject(MemberOpenDiagnostic::Replay);
        };
        if self.pin_index == 0 && checkpoint.composition_pins.try_reserve_exact(source.pins.len()).is_err() {
            return self.reject(MemberOpenDiagnostic::Capacity);
        }
        if let Some(pin) = source.pins.get(self.pin_index) {
            let child_ref = match ArtifactRef::parse_uri(&pin.child_uri) {
                Ok(reference) => reference,
                Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
            };
            if !self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained").insert(child_ref.clone()) {
                return self.reject(MemberOpenDiagnostic::Replay);
            }
            checkpoint.composition_pins.push(CompositionPin { child_ref, checkpoint_id: pin.checkpoint_id.clone() });
            self.pin_index += 1;
        } else {
            if !owned_frame_funded::<std::collections::HashSet<ArtifactRef>>(grant) { return PersistedDocumentHydrationStep::Pending(self.progress()); }
            let previous = std::mem::take(self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained"));
            if let Err(diagnostic) = self.admit_active(previous, cx, grant) { return self.reject(diagnostic); }
            self.pin_group_index += 1;
            self.pin_index = 0;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }
}

fn take_change(source:&mut protocol::FoldChange)->semio_os_vcs::Change{
 semio_os_vcs::Change{id:std::mem::take(&mut source.id),edit_ids:std::mem::take(&mut source.edit_ids),description:source.description.take(),saved_at:std::mem::take(&mut source.saved_at)}
}
fn restore_change(source:&mut protocol::FoldChange,value:semio_os_vcs::Change){source.id=value.id;source.edit_ids=value.edit_ids;source.description=value.description;source.saved_at=value.saved_at;}
impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>{
 fn step_original_changes(&mut self,cx:&mut StepContext<'_>)->PersistedDocumentHydrationStep<P,M>{
  if let Err(error)=cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)){*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Replay)}
  if cx.should_yield()||cx.fuel_remaining()<3{return PersistedDocumentHydrationStep::Pending(self.progress())}
  let grant=cx.retained_grant();let fold=self.fold.as_mut().expect("history fold remains retained");
  let Some(source)=fold.changes.get_mut(self.operation_index)else{
   let bytes=std::mem::size_of::<usize>()+std::mem::size_of::<Phase>();
   if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
   self.operation_index=0;self.phase=Phase::HydrateCheckpoints;
   let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);
   if let Err(error)=receipt{*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}
   return PersistedDocumentHydrationStep::Pending(self.progress());
  };
  let ledger=&mut self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.changes;
  let overhead=2*std::mem::size_of_val(source)+2*std::mem::size_of::<Option<semio_os_vcs::Change>>()+std::mem::size_of::<usize>();
  let mut demand=ledger.owned_row_admission_demands();demand.copy_bytes=match demand.copy_bytes.checked_add(overhead){Some(bytes)=>bytes,None=>return self.reject(MemberOpenDiagnostic::Capacity)};
  if grant.maximum_items<2||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return PersistedDocumentHydrationStep::Pending(self.progress())}
  let mut row=Some(take_change(source));
  let turn=ledger.admit_owned_row(&mut row,RetainedCloneGrant{maximum_items:grant.maximum_items-1,maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant});
  let donated=row.is_none();if let Some(value)=row{restore_change(source,value)}
  if donated{self.operation_index+=1}
  let mut progress=match &turn.result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
  progress.copied_items+=1;progress.copied_bytes+=overhead;
  let receipt=cx.consume_retained(progress);cx.consume_fuel(turn.work+2);
  if let Err(error)=turn.result{let mut failure=semio_framework_job::OriginalTurnFailure::from(error);failure.effects=progress;failure.receipt_refusal=receipt.err();*self.original_failure=Some(failure);return self.reject(MemberOpenDiagnostic::Capacity)}
  if let Err(error)=receipt{*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}
  PersistedDocumentHydrationStep::Pending(self.progress())
 }
}
#[cfg(test)]
mod original_change_laws{
 use super::*;
 #[test]
 fn actual_change_donation_and_refusal_restore_preserve_the_same_string_and_vector_backings(){
  let law:serde_json::Value=serde_json::from_str(include_str!("donor.json")).unwrap();
  for row in law["cases"].as_array().unwrap(){
   let mut source=protocol::FoldChange{id:row["id"].as_str().unwrap().into(),edit_ids:row["editIds"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().into()).collect(),description:row["description"].as_str().map(str::to_owned),saved_at:row["savedAt"].as_str().unwrap().into()};
   let id=source.id.as_ptr();let ids=source.edit_ids.as_ptr();let saved=source.saved_at.as_ptr();let description=source.description.as_ref().map(String::as_ptr);
   let value=take_change(&mut source);assert!(source.id.is_empty()&&source.edit_ids.is_empty()&&source.description.is_none()&&source.saved_at.is_empty());
   assert_eq!(value.id.as_ptr(),id);assert_eq!(value.edit_ids.as_ptr(),ids);assert_eq!(value.saved_at.as_ptr(),saved);assert_eq!(value.description.as_ref().map(String::as_ptr),description);
   assert_eq!(serde_json::json!({"id":value.id,"editIds":value.edit_ids,"description":value.description,"savedAt":value.saved_at}),*row);
   restore_change(&mut source,value);assert_eq!(source.id.as_ptr(),id);assert_eq!(source.edit_ids.as_ptr(),ids);assert_eq!(source.saved_at.as_ptr(),saved);
  }
 }
}

fn take_checkpoint(source:&mut protocol::FoldCheckpoint)->semio_os_vcs::Checkpoint{
 semio_os_vcs::Checkpoint{id:std::mem::take(&mut source.id),change_ids:std::mem::take(&mut source.change_ids),parent_id:source.parent_id.take(),authors:Vec::new(),message:source.message.take(),timestamp:std::mem::take(&mut source.timestamp),composition_pins:Vec::new()}
}
fn restore_checkpoint(source:&mut protocol::FoldCheckpoint,value:semio_os_vcs::Checkpoint){source.id=value.id;source.change_ids=value.change_ids;source.parent_id=value.parent_id;source.message=value.message;source.timestamp=value.timestamp;assert!(value.authors.is_empty()&&value.composition_pins.is_empty());}
impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>{
 fn post_original_metadata(&mut self,cx:&mut StepContext<'_>,turn:semio_framework_value::RetainedWorkOutcome<RetainedCloneProgress>)->PersistedDocumentHydrationStep<P,M>{
  let progress=match &turn.result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};let receipt=cx.consume_retained(progress);cx.consume_fuel(turn.work);
  if let Err(error)=turn.result{let mut failure=semio_framework_job::OriginalTurnFailure::from(error);failure.effects=progress;failure.receipt_refusal=receipt.err();*self.original_failure=Some(failure);return self.reject(MemberOpenDiagnostic::Capacity)}
  if let Err(error)=receipt{*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Capacity)}
  PersistedDocumentHydrationStep::Pending(self.progress())
 }
 fn step_original_checkpoints(&mut self,cx:&mut StepContext<'_>)->PersistedDocumentHydrationStep<P,M>{
  if let Err(error)=cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)){*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Replay)}
  if cx.should_yield()||cx.fuel_remaining()<4{return PersistedDocumentHydrationStep::Pending(self.progress())}
  let grant=cx.retained_grant();let fold=self.fold.as_mut().expect("history fold remains retained");
  let Some(source)=fold.checkpoints.get_mut(self.operation_index)else{
   let bytes=2*std::mem::size_of::<usize>()+std::mem::size_of::<Phase>();
   if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
   self.operation_index=0;self.pin_index=0;self.phase=Phase::HydrateAlternatives;
   return self.post_original_metadata(cx,semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}),work:1});
  };
  let ledger=&mut self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.checkpoints;
  if self.pin_index==0{
   let overhead=2*std::mem::size_of_val(source)+2*std::mem::size_of::<Option<semio_os_vcs::Checkpoint>>()+std::mem::size_of::<usize>();
   let mut demand=ledger.owned_row_admission_demands();demand.copy_bytes=match demand.copy_bytes.checked_add(overhead){Some(bytes)=>bytes,None=>return self.reject(MemberOpenDiagnostic::Capacity)};
   if grant.maximum_items<2||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return PersistedDocumentHydrationStep::Pending(self.progress())}
   let mut row=Some(take_checkpoint(source));let turn=ledger.admit_owned_row(&mut row,RetainedCloneGrant{maximum_items:grant.maximum_items-1,maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant});
   if let Some(value)=row{restore_checkpoint(source,value)}else{self.pin_index=1}
   let mut progress=match &turn.result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};progress.copied_items+=1;progress.copied_bytes+=overhead;
   return self.post_original_metadata(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|_|progress).map_err(|error|error.with_retained_progress(progress)),work:turn.work+2});
  }
  if ledger.len()!=self.operation_index+1{return self.reject(MemberOpenDiagnostic::Replay)}
  let Some(target)=ledger.last_mut()else{return self.reject(MemberOpenDiagnostic::Replay)};
  let mut work=semio_framework_value_persistent::work::RetainedWorkLedger::new(grant);
  let result=(||->Result<(),ValueError>{
   let author_count=source.authors.len();if let Some(author)=source.authors.get_mut(self.pin_index-1){
    if !work.reserve_vec(&mut target.authors,author_count).map_err(|reason|ValueError::literal(ValueRefusalKind::OwnershipLimit,reason))?{return Ok(())}
    if !work.vec_slot(&mut target.authors).map_err(|reason|ValueError::literal(ValueRefusalKind::OwnershipLimit,reason))?||!work.copy_bytes(std::mem::size_of_val(author)+std::mem::size_of::<usize>()){return Ok(())}
    let value=semio_os_vcs::Author{id:std::mem::take(&mut author.id),name:std::mem::take(&mut author.name),avatar:author.avatar.take()};
    if let Err((reason,value))=work.vec_push(&mut target.authors,value){author.id=value.id;author.name=value.name;author.avatar=value.avatar;return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,reason))}
    self.pin_index+=1;
   }else if work.copy_bytes(2*std::mem::size_of::<usize>()){self.operation_index+=1;self.pin_index=0;}
   Ok(())
  })();
  let progress=work.receipt();self.post_original_metadata(cx,semio_framework_value::RetainedWorkOutcome{result:result.map(|()|progress).map_err(|error|error.with_retained_progress(progress)),work:work.actual_work()+1})
 }
}

#[cfg(test)]
mod original_checkpoint_laws{
 use super::*;
 #[test]
 fn actual_checkpoint_header_donation_preserves_the_separate_author_source_until_paid_growth(){
  let law:serde_json::Value=serde_json::from_str(include_str!("checkpoint.json")).unwrap();
  for row in law["cases"].as_array().unwrap(){
   let mut source=protocol::FoldCheckpoint{id:row["id"].as_str().unwrap().into(),change_ids:row["changeIds"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().into()).collect(),parent_id:row["parentId"].as_str().map(str::to_owned),authors:row["authors"].as_array().unwrap().iter().map(|author|protocol::Author{id:author["id"].as_str().unwrap().into(),name:author["name"].as_str().unwrap().into(),avatar:author["avatar"].as_str().map(str::to_owned)}).collect(),message:row["message"].as_str().map(str::to_owned),timestamp:row["timestamp"].as_str().unwrap().into(),pins:Vec::new()};
   let id=source.id.as_ptr();let changes=source.change_ids.as_ptr();let authors=source.authors.as_ptr();let timestamp=source.timestamp.as_ptr();
   let value=take_checkpoint(&mut source);assert_eq!(value.id.as_ptr(),id);assert_eq!(value.change_ids.as_ptr(),changes);assert_eq!(value.timestamp.as_ptr(),timestamp);assert_eq!(source.authors.as_ptr(),authors);assert!(value.authors.is_empty()&&value.composition_pins.is_empty());
   assert_eq!(serde_json::json!({"id":value.id,"changeIds":value.change_ids,"parentId":value.parent_id,"authors":source.authors.iter().map(|author|serde_json::json!({"id":author.id,"name":author.name,"avatar":author.avatar})).collect::<Vec<_>>(),"message":value.message,"timestamp":value.timestamp}),*row);
   restore_checkpoint(&mut source,value);assert_eq!(source.id.as_ptr(),id);assert_eq!(source.change_ids.as_ptr(),changes);assert_eq!(source.authors.as_ptr(),authors);
  }
 }
}

fn take_alternative(source:&mut protocol::FoldAlternative)->semio_os_vcs::Alternative{semio_os_vcs::Alternative{id:std::mem::take(&mut source.id),name:std::mem::take(&mut source.name),checkpoint_ids:std::mem::take(&mut source.checkpoint_ids)}}
fn restore_alternative(source:&mut protocol::FoldAlternative,value:semio_os_vcs::Alternative){source.id=value.id;source.name=value.name;source.checkpoint_ids=value.checkpoint_ids;}
impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>{
 fn step_original_alternatives(&mut self,cx:&mut StepContext<'_>)->PersistedDocumentHydrationStep<P,M>{
  if let Err(error)=cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)){*self.original_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Replay)}
  if cx.should_yield()||cx.fuel_remaining()<3{return PersistedDocumentHydrationStep::Pending(self.progress())}
  let grant=cx.retained_grant();let fold=self.fold.as_mut().expect("history fold remains retained");
  let Some(source)=fold.alternatives.get_mut(self.operation_index)else{
   let bytes=std::mem::size_of::<usize>()+std::mem::size_of::<Phase>();
   if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
   self.operation_index=0;self.phase=Phase::HydratePins;
   return self.post_original_metadata(cx,semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}),work:1});
  };
  let ledger=&mut self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.alternatives;
  let overhead=2*std::mem::size_of_val(source)+2*std::mem::size_of::<Option<semio_os_vcs::Alternative>>()+std::mem::size_of::<usize>();
  let mut demand=ledger.owned_row_admission_demands();demand.copy_bytes=match demand.copy_bytes.checked_add(overhead){Some(bytes)=>bytes,None=>return self.reject(MemberOpenDiagnostic::Capacity)};
  if grant.maximum_items<2||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return PersistedDocumentHydrationStep::Pending(self.progress())}
  let mut row=Some(take_alternative(source));let turn=ledger.admit_owned_row(&mut row,RetainedCloneGrant{maximum_items:grant.maximum_items-1,maximum_copy_bytes:grant.maximum_copy_bytes-overhead,..grant});
  if let Some(value)=row{restore_alternative(source,value)}else{self.operation_index+=1}
  let mut progress=match &turn.result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};progress.copied_items+=1;progress.copied_bytes+=overhead;
  self.post_original_metadata(cx,semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|_|progress).map_err(|error|error.with_retained_progress(progress)),work:turn.work+2})
 }
}
#[cfg(test)]
mod original_alternative_laws{
 use super::*;
 #[test]
 fn actual_alternative_donation_and_refusal_restore_keep_the_original_backings(){
  let law:serde_json::Value=serde_json::from_str(include_str!("alternative.json")).unwrap();
  for row in law["cases"].as_array().unwrap(){
   let mut source=protocol::FoldAlternative{id:row["id"].as_str().unwrap().into(),name:row["name"].as_str().unwrap().into(),checkpoint_ids:row["checkpointIds"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().into()).collect()};
   let id=source.id.as_ptr();let name=source.name.as_ptr();let checkpoints=source.checkpoint_ids.as_ptr();let value=take_alternative(&mut source);
   assert!(source.id.is_empty()&&source.name.is_empty()&&source.checkpoint_ids.is_empty());assert_eq!(value.id.as_ptr(),id);assert_eq!(value.name.as_ptr(),name);assert_eq!(value.checkpoint_ids.as_ptr(),checkpoints);
   assert_eq!(serde_json::json!({"id":value.id,"name":value.name,"checkpointIds":value.checkpoint_ids}),*row);
   restore_alternative(&mut source,value);assert_eq!(source.id.as_ptr(),id);assert_eq!(source.name.as_ptr(),name);assert_eq!(source.checkpoint_ids.as_ptr(),checkpoints);
  }
 }
}
