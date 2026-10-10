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

    fn commit_document_window_reset(&mut self, replacement: WindowTransientOwnerRegistry) {
        let generation = replacement.document_generation();
        let displaced = std::mem::replace(&mut self.window_transient_store, replacement);
        if !displaced.terminal_is_empty() {
            self.retired_window_transient_stores.insert_admitted(generation, displaced);
        }
        self.tool_cancellations.renew_scope_generation();
        self.cache = None;
    }

    /// 🪟️ Preserves the caller's original full policy through every displaced document generation.
    fn retire_document_windows_step(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let cursor = if closing { &mut self.close_window_retirement_cursor } else { &mut self.maintenance_window_retirement_cursor };
        retire_document_window_registry_step(&mut self.retired_window_transient_stores, cursor, grant, closing)
    }
}

fn document_window_registry_demands(registries: &ArtifactFixedRegistry<WindowTransientOwnerRegistry>, cursor: usize, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
    let Some((_, generation)) = registries.next_id_from(cursor) else { return Ok(Default::default()) };
    let mut demand = registries.get(generation).expect("selected window transient retirement remains owned").retirement_demands(body)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "document window retirement depth overflow"))?;
    Ok(demand)
}

/// ♻️ Advances one displaced document's windows fairly even while another owner is blocked.
fn retire_document_window_registry_step(registries: &mut ArtifactFixedRegistry<WindowTransientOwnerRegistry>, cursor: &mut usize, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
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
        let bytes=std::mem::size_of::<usize>();
        if grant.maximum_copy_bytes<bytes{return Ok(step);}
        *cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
        return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));
    }
    *cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;
    Ok(match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step})
}

/// 📏️ Borrows one original generation or its exact final empty registry allocation.
fn retire_document_window_registry_demands(registries:&ArtifactFixedRegistry<WindowTransientOwnerRegistry>,cursor:usize,body:usize,closing:bool)->Result<RetirementDemand,ValueError>{
    if let Some((_,generation))=registries.next_id_from(cursor){let original=registries.get(generation).expect("selected original window transient generation remains owned");return if original.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:std::mem::size_of::<WindowTransientOwnerRegistry>(),depth:1,..Default::default()})}else{original.retirement_demands(body)};}
    let release_bytes=if closing{registries.empty_backing_byte_demand().unwrap_or(0)}else{0};
    Ok(RetirementDemand{release_bytes,depth:usize::from(release_bytes!=0),..Default::default()})
}
