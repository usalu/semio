//! 🔁️ Envelope folds use the actual original coroutine birth, output and rejection recipients.
use super::*;
use semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary;
impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>{
 fn capture_original_fold(failure:&mut Option<semio_framework_job::OriginalTurnFailure>,result:Option<Result<(),OriginalReceivingPrimary>>,receipt_refusal:Option<ValueError>,progress:RetainedCloneProgress){
  let primary=result.and_then(Result::err);
  if primary.is_some()||receipt_refusal.is_some(){*failure=Some(semio_framework_value_native_owned::native_decoding::NativeConstructionStepFailure{primary,receipt_refusal,progress}.into())}
 }
 fn settle_original_fold_scope(&mut self,scope:Option<semio_framework_job::OriginalNativeReceivingOutcome>,result:Result<bool,semio_framework_job::OriginalTurnFailure>)->bool{
  match result{
   Err(failure)=>*self.original_context_failure=Some(failure),
   Ok(false)=>{},
   Ok(true)=>match scope{
    Some(scope) if scope.source_refusal.is_some()||scope.call.is_err()=>*self.original_context_failure=Some(ValueError::literal(ValueRefusalKind::InvalidValue,"original fold receiving scope refused").with_retained_progress(scope.retained_progress()).into()),
    Some(_)=>{},
    None=>*self.original_context_failure=Some(ValueError::literal(ValueRefusalKind::InvariantViolated,"original fold receiving scope vanished").into()),
   },
  }
  self.original_failure.is_none()&&self.original_context_failure.is_none()
 }
 pub(super) fn begin_original_fold(&mut self,cx:&mut StepContext<'_>)->PersistedDocumentHydrationStep<P,M>{
  if self.fold_job.is_none(){
  let history=self.history.as_ref().expect("original fold History remains retained");let output=&mut*self.fold_job;let failure=&mut*self.original_failure;
  let mut runner=Some(|control|{let source=std::sync::Arc::clone(history);async move{source.fold_controlled(semio_os_spr::HistoryShape::Document,&control).await}});
  let mut scope=None;let result=cx.with_native_receiving(&mut scope,|native,_|{let turn=semio_os_spr::HistoryFoldJob::admit_original(&mut runner,output,native);Self::capture_original_fold(failure,turn.result.map(|result|result.map(|_|())),turn.receipt_refusal,turn.progress);Ok(())});
  if !self.settle_original_fold_scope(scope,result){return self.reject(MemberOpenDiagnostic::Replay)}
  }
  if self.fold_job.is_some(){
   let bytes=std::mem::size_of::<Phase>();let grant=cx.retained_grant();
   if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()==0{return PersistedDocumentHydrationStep::Pending(self.progress())}
   self.phase=Phase::Fold;let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);
   if let Err(error)=receipt{*self.original_context_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Replay)}
  }
  PersistedDocumentHydrationStep::Pending(self.progress())
 }
 pub(super) fn advance_original_fold(&mut self,cx:&mut StepContext<'_>)->PersistedDocumentHydrationStep<P,M>{
  if self.fold_output.value.is_some(){
   let bytes=std::mem::size_of::<semio_os_spr::history::RetainedHistoryFold>()+std::mem::size_of_val(&self.fold_job)+std::mem::size_of_val(&self.fold_output)+std::mem::size_of_val(&self.fold)+std::mem::size_of_val(&self.normalized_transitions)+std::mem::size_of_val(&self.replay_ids)+std::mem::size_of_val(&self.normalized_conflicts)+std::mem::size_of::<u64>()+std::mem::size_of::<Phase>();
   let grant=cx.retained_grant();if cx.should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return PersistedDocumentHydrationStep::Pending(self.progress())}
   if !self.fold_job.as_ref().is_some_and(ErasedSnapshotRetirement::terminal_is_empty){return self.reject(MemberOpenDiagnostic::Replay)}
   let(value,transitions,replay,conflicts)=self.fold_output.value.take().unwrap();self.fold_completed=self.fold_job.as_ref().unwrap().completed();self.fold_job.take();*self.fold=Some(value);*self.normalized_transitions=Some(transitions.into());*self.replay_ids=Some(replay);*self.normalized_conflicts=Some(conflicts);self.phase=Phase::BindGenesis;
   let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);
   if let Err(error)=receipt{*self.original_context_failure=Some(error.into());return self.reject(MemberOpenDiagnostic::Replay)}
   return PersistedDocumentHydrationStep::Pending(self.progress());
  }
  let body=usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX).min(4096);
  let job=self.fold_job.as_mut().expect("original fold job remains retained");let output=&mut*self.fold_output.value;let protocol=&mut*self.fold_protocol.value;let failure=&mut*self.original_failure;
  let mut scope=None;let result=cx.with_native_receiving(&mut scope,|native,_|{
   if job.original_rejection_ready(){let turn=job.take_original_rejection(protocol,native);Self::capture_original_fold(failure,turn.result.map(|result|result.map(|_|())),turn.receipt_refusal,turn.progress);}
   else if job.original_output_ready(){let turn=job.take_original_output(output,native);Self::capture_original_fold(failure,turn.result.map(|result|result.map(|_|())),turn.receipt_refusal,turn.progress);}
   else{let turn=job.step_original(native,body);Self::capture_original_fold(failure,turn.result.map(|result|result.map(|_|())),turn.receipt_refusal,turn.progress);}Ok(())
  });
  if !self.settle_original_fold_scope(scope,result)||self.fold_protocol.value.is_some(){return self.reject(MemberOpenDiagnostic::Replay)}
  PersistedDocumentHydrationStep::Pending(self.progress())
 }
}

impl<P,M> RetainedPersistedDocumentHydration<P,M>
where P:ArtifactPack+semio_os_store_text_io::ArtifactStoreSnapshot+semio_os_spr::StoreState,M:semio_os_spr::StoreMutationFor<P>{
 /// ♻️ Closes original fold control with its actual decoder before any grant-only data retirement.
 pub fn close_original_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;
  if self.target!=PersistedDocumentHydrationTarget::Envelope{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original decoder cleanup requires the declared Envelope target").into())}
  if cx.operation()!=self.operation||cx.generation()!=self.generation||!self.original_source.is_some_and(|source|source.matches(cx)){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"hydration cleanup changed its actual original Source").into())}
  if self.original_failure.is_some()||self.original_context_failure.is_some()||!self.fold_protocol.terminal_is_empty()||!self.mutation_domain.terminal_is_empty(){return Ok(false)}
  cx.with_original_retirement(|cx|{
   if cx.retirement_should_yield()||cx.fuel_remaining()==0{return Ok(false)}
   if self.mutation_admission.has_failure()&&self.mutation_refusal.is_none(){if self.mutation_admission.take_failure_into_step(&mut self.mutation_refusal,cx)?{return Ok(false)}}
   if self.mutation_constructing||!self.mutation_admission.terminal_is_empty(){self.mutation_constructing=false;self.mutation_admission.close_step(cx)?;return Ok(false)}
   let grant=cx.retained_grant();let available_work=cx.fuel_remaining();
   if let Some(job)=self.fold_job.as_mut(){
    if job.original_rejection_ready(){
     let output=&mut*self.fold_protocol.value;
     let turn=cx.with_native_retirement(|native|Ok(job.take_original_rejection_retiring(output,native,grant,available_work)))?;
     let progress=match &turn.result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
     let receipt=cx.consume_retained(progress);cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));
     if let Err(original)=turn.result{let mut cause=semio_framework_job::OriginalTurnFailure::from(semio_framework_job::OriginalTurnPrimary::Receiving(original));cause.effects=progress;cause.receipt_refusal=receipt.err();return Err(cause)}
     receipt?;return Ok(false)
    }
    if job.terminal_is_empty(){
     let bytes=std::mem::size_of_val(&self.fold_job);
     if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
     self.fold_job.take();cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(false)
    }
    let demand=job.next_original_close_demand(grant.maximum_copy_bytes)?;
    if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(false)}
    let Some((poll,performed))=cx.with_original_effect(grant,|cx,effect|cx.with_native_retirement(|native|Ok::<_,ValueError>(job.close_original(native,grant.maximum_copy_bytes,effect))))? else{return Ok(false)};
    let turn=match poll?{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn,semio_framework_value::RetainedWorkPoll::Pending{work}=>semio_framework_value::RetainedWorkOutcome{result:Ok(semio_framework_value::RetainedCloneStep::Progress(Default::default())),work}};
    let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
    cx.consume_fuel(u64::try_from(performed.max(turn.work)).unwrap_or(u64::MAX));let receipt=cx.consume_retained(progress);
    if let Err(error)=turn.result{let mut cause=semio_framework_job::OriginalTurnFailure::from(error);cause.effects=progress;cause.receipt_refusal=receipt.err();return Err(cause)}
    receipt?;return Ok(false)
   }
   let demand=self.close_demands(grant.maximum_copy_bytes)?;
   semio_framework_job::close_original_effect_step(cx,demand,|effect|{let body=effect.grant().maximum_copy_bytes;self.close_step(body,effect)})?;
   Ok(self.terminal_is_empty_unbounded())
  })
 }
}
