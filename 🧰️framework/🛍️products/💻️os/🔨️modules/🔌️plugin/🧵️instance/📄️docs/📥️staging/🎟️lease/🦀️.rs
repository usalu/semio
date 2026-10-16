//! 🪮️ Staged slice of the `lease` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
use super::*;
use super::jobs_lease::{OriginalToolLeaseAdmission};

impl<A: ArtifactApp> MediaTransfers<A> {
    pub(crate) fn original_tool_lease_close_demands(&self, tools: &ToolOperations<A>,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        use semio_framework_value::RetirementDemand;
        if tools.pending_tool_lease_admission.is_none()&&self.pending_media_import_permit.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<(FrameworkReservedCommitPermit,(semio_framework_job::OperationId,semio_framework_job::Generation,usize))>()+size_of::<OriginalToolLeaseAdmission>(),depth:1,..Default::default()})}
        let Some(pending)=tools.pending_tool_lease_admission.as_ref()else{return Ok(Default::default())};
        if let Some(owner)=pending.retirement.as_ref(){return owner.retirement_demands(tools.tool_cancellations.handle().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending lease lost its issuer"))?,body)}
        if pending.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),depth:1,..Default::default()})}
        if let Some(verb)=pending.verb.as_ref(){return Ok(RetirementDemand{release_bytes:verb.capacity(),depth:1,..Default::default()})}
        if pending.operation.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_job::Operation>(),depth:1,..Default::default()})}
        if let Some(source)=pending.source.as_ref(){return Ok(source.retirement_demands())}
        Ok(RetirementDemand{copy_bytes:size_of::<OriginalToolLeaseAdmission>(),depth:1,..Default::default()})
    }

    pub(crate) fn close_original_tool_lease(&mut self, tools: &mut ToolOperations<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
        let demand=self.original_tool_lease_close_demands(tools, grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||!semio_framework_plugin_ownership::bounded_ownership::plugin_grant_funds(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if tools.pending_tool_lease_admission.is_none(){if let Some((permit,identity))=self.pending_media_import_permit.take(){
            let FrameworkReservedCommitPermit{operation,lease,..}=permit;let instance=lease.key.app_instance_id;
            *tools.pending_tool_lease_admission=Some(OriginalToolLeaseAdmission{source:ManuallyDrop::new(None),verb:ManuallyDrop::new(None),operation:Some(operation),lease:ManuallyDrop::new(Some(lease)),retirement:ManuallyDrop::new(None),instance,identity,keyed:true,copied:true,returning:true});
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
        }}
        let Some(pending)=tools.pending_tool_lease_admission.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        if pending.identity!=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool lease close requires its original receiving caller").into())}
        if !pending.returning{if let Some(source)=pending.source.as_ref(){source.validate_context(cx)?;}}
        if let Some(owner)=pending.retirement.as_mut(){
            if owner.terminal_is_empty(){pending.retirement.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
            return owner.close_step(tools.tool_cancellations.handle().unwrap(),grant).map_err(Into::into)
        }
        if let Some(lease)=pending.lease.take(){lease.cancel();*pending.retirement=Some(crate::tool_cancellation::ToolCancellationLeaseRetirement::new(lease));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
        if pending.verb.is_some(){drop(pending.verb.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))}
        if let Some(operation)=pending.operation.take(){tools.release_reserved_emit_slot(operation.operation.0);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
        if let Some(source)=pending.source.as_mut(){pending.returning=true;let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){pending.source.take();}return Ok(RetainedCloneStep::Progress(step.progress()))}
        tools.pending_tool_lease_admission.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
}
