/// 🪪️ A persistent original loan owns one window renewal admission until its atomic publication and paid return.
struct DocumentWindowResetAdmission{source:std::mem::ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,generation:Option<u64>,committed:bool,returning:bool}
impl DocumentWindowResetAdmission{
 fn validate<A:ArtifactApp,M:SpaceMember+MemberFactory+'static>(&self,app:&VcsArtifactApp<A,M>,cx:&semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{if !self.returning{self.source.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original window renewal loan is absent"))?.validate_context(cx)?;}if self.generation.is_some_and(|generation|generation!=app.tool_cancellations.scope_generation()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original ready window renewal changed its exact generation").into())}Ok(())}
}
/// 🔁️ Shared document replacement invalidates concrete window transients and retains displaced owners.
impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    fn prepare_document_window_reset(&self) -> Result<WindowTransientOwnerRegistry, Fault> {
        if self.close_started {
            return Err(plugin_sdk_fault("a closing app cannot replace its document window owners"));
        }
        let generation = self.window_transient_store.document_generation().checked_add(1).ok_or_else(|| plugin_sdk_fault("document window transient generation is exhausted"))?;
        if !self.window_transient_store.terminal_is_empty() && !self.retired_window_transient_stores.can_insert(generation) {
            return Err(plugin_sdk_fault("document replacement awaits bounded window transient retirement capacity"));
        }
        let mut replacement = WindowTransientOwnerRegistry::for_document_generation(generation);
        A::register_window_transient_owners(&mut replacement)?;
        Ok(replacement)
    }

    pub(crate) fn admit_document_window_reset(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.pending_document_window_admission.is_none(){let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(false)};*self.pending_document_window_admission=Some(DocumentWindowResetAdmission{source:std::mem::ManuallyDrop::new(Some(source)),generation:None,committed:false,returning:false});return Ok(false)}
        let admission=self.pending_document_window_admission.as_ref().unwrap();admission.validate(self,cx)?;if admission.committed{return Ok(false)}if admission.generation.is_some(){return Ok(true)}
        if !self.tool_cancellations.renew_scope_generation(cx)?{return Ok(false)}self.pending_document_window_admission.as_mut().unwrap().generation=Some(self.tool_cancellations.scope_generation());Ok(false)
    }

    fn commit_document_window_reset(&mut self,replacement:&mut Option<WindowTransientOwnerRegistry>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let admission=self.pending_document_window_admission.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"window publication has no original renewal admission"))?;admission.validate(self,cx)?;if admission.generation.is_none()||admission.committed{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"window publication requires one ready unused original renewal" ).into())}
        let replacement=replacement.take().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"document reset lost its retained replacement"))?;
        let generation=replacement.document_generation();let displaced=std::mem::replace(&mut self.window_transient_store,replacement);if !displaced.terminal_is_empty(){self.retired_window_transient_stores.insert_admitted(generation,displaced)}self.pending_document_window_admission.as_mut().unwrap().committed=true;self.cache=None;Ok(())
    }

    pub(crate) fn document_window_admission_demands(&self)->semio_framework_value::RetirementDemand{self.pending_document_window_admission.as_ref().map_or(Default::default(),|admission|admission.source.as_ref().map_or(semio_framework_value::RetirementDemand{depth:1,..Default::default()},semio_framework_job::OriginalCallerSourceLoan::retirement_demands))}

    pub(crate) fn close_document_window_admission(&mut self,cx:&mut semio_framework_job::StepContext<'_>,closing:bool)->Result<crate::PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let Some(admission)=self.pending_document_window_admission.as_mut()else{return Ok(crate::PluginLifecycleStep::Complete(Default::default()))};if !closing&&!admission.committed{return Ok(crate::PluginLifecycleStep::AwaitingInput{reason:"original window renewal awaits its candidate publication"})}
        if let Some(source)=admission.source.as_mut(){admission.returning=true;let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){admission.source.take();}return Ok(crate::PluginLifecycleStep::Progress(step.progress()))}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(crate::PluginLifecycleStep::Progress(Default::default()))}self.pending_document_window_admission.take();cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(crate::PluginLifecycleStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}))
    }

    /// 🪟️ Preserves the caller's original full policy through every displaced document generation.
    fn retire_document_windows_step(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let cursor = if closing { &mut self.close_window_retirement_cursor } else { &mut self.maintenance_window_retirement_cursor };
        retire_document_window_registry_step(&mut self.retired_window_transient_stores, cursor, grant, closing)
    }

    /// 📏️ Quotes the next displaced document window retirement under the same cursor [`Self::retire_document_windows_step`] advances.
    fn document_windows_retirement_demands(&self, body: usize, closing: bool) -> Result<RetirementDemand, ValueError> {
        let cursor = if closing { self.close_window_retirement_cursor } else { self.maintenance_window_retirement_cursor };
        retire_document_window_registry_demands(&self.retired_window_transient_stores, cursor, body, closing)
    }
}

/// ♻️ Advances one displaced document's windows fairly even while another owner is blocked.
pub(crate) fn retire_document_window_registry_step(registries: &mut ArtifactFixedRegistry<WindowTransientOwnerRegistry>, cursor: &mut usize, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
    let demand=retire_document_window_registry_demands(registries,*cursor,grant.maximum_copy_bytes,closing).map_err(|error|plugin_sdk_fault(error.into_message()))?;
    if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(PluginLifecycleStep::Progress(Default::default()));}
    let Some((index,generation))=registries.next_id_from(*cursor)else{
        if demand.release_bytes==0{return Ok(PluginLifecycleStep::Complete(Default::default()));}
        registries.slots=Box::default();registries.allocation_admitted=false;
        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}));
    };
    let retired=registries.get_mut(generation).expect("selected original window transient generation remains owned");
    if retired.terminal_is_empty(){
        drop(registries.remove(generation));*cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));
    }
    let step=retired.close_step(grant)?;
    if step.progress().is_some_and(|progress|!progress.fits(grant))||matches!(step,PluginLifecycleStep::Complete(_))&&!retired.terminal_is_empty(){return Err(plugin_sdk_fault("displaced window transient violated its original grant or terminal ownership"));}
    if step.progress().is_none_or(|progress|progress==RetainedCloneProgress::default()){
        let bytes=size_of::<usize>();
        if grant.maximum_copy_bytes<bytes{return Ok(step);}
        *cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
    }
    *cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
    Ok(match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step})
}

/// 📏️ Borrows one original generation or its exact final empty registry allocation.
pub(crate) fn retire_document_window_registry_demands(registries:&ArtifactFixedRegistry<WindowTransientOwnerRegistry>,cursor:usize,body:usize,closing:bool)->Result<RetirementDemand,ValueError>{
    if let Some((_,generation))=registries.next_id_from(cursor){let original=registries.get(generation).expect("selected original window transient generation remains owned");return if original.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<WindowTransientOwnerRegistry>(),depth:1,..Default::default()})}else{original.retirement_demands(body)};}
    let release_bytes=if closing{registries.empty_backing_byte_demand().unwrap_or(0)}else{0};
    Ok(RetirementDemand{release_bytes,depth:usize::from(release_bytes!=0),..Default::default()})
}
