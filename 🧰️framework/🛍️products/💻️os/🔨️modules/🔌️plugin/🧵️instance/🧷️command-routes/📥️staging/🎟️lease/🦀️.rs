//! 🪭️ Staged slice of the `lease` band of the mounted artifact instance: its `instance-commands` members (WP6 step S0 of the instance family).
use super::*;
use super::jobs_lease::{OriginalToolLeaseAdmission};

impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> VcsArtifactApp<A,M> {
    pub(crate) fn admit_original_tool_lease(&mut self,verb:&str,meta:&ActionMeta,keyed:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(semio_framework_job::Operation,ToolCancellationLease)>,semio_framework_job::OriginalTurnFailure>{
        if self.tools.pending_tool_lease_admission.is_none(){
            let bytes=size_of::<OriginalToolLeaseAdmission>();let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
            *self.tools.pending_tool_lease_admission=Some(OriginalToolLeaseAdmission{source:ManuallyDrop::new(None),verb:ManuallyDrop::new(Some(String::new())),operation:None,lease:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),instance:meta.instance_id,identity:(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?),keyed,copied:false,returning:false});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        let pending=self.tools.pending_tool_lease_admission.as_mut().unwrap();
        if pending.identity!=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?)||pending.instance!=meta.instance_id||pending.keyed!=keyed{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool admission requires its retained receiving lane").into())}
        if !pending.returning{
            if pending.source.is_none(){let Some(source)=cx.original_source_parent()?.admit_loan(cx)?else{return Ok(None)};*pending.source=Some(source);return Ok(None)}
            pending.source.as_ref().unwrap().validate_context(cx)?;
        }
        if pending.copied&&pending.verb.as_ref().is_some_and(|original|original!=verb){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool admission requires its retained verb").into())}
        if !pending.copied{
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<verb.len()||grant.maximum_capacity_bytes<verb.len()||grant.maximum_depth==0{return Ok(None)}
            let target=pending.verb.as_mut().unwrap();target.try_reserve_exact(verb.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original tool verb reservation refused"))?;
            target.push_str(verb);pending.copied=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:verb.len(),retained_capacity_bytes:target.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        if pending.operation.is_none(){
            let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<semio_framework_job::Operation>()||grant.maximum_depth==0{return Ok(None)}
            let Some(operation_id)=self.media.admit_typed_operation_slot(&self.pending, &self.tools)else{return Ok(None)};
            let revision=self.stores.store.content_revision_now();let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));let generation=semio_framework_job::Generation(self.stores.store.generation_now());
            let operation=semio_framework_job::Operation::new(operation_id,base_revision,generation,operation_id.0);
            self.tools.typed_operation_reservations[operation_id.0 as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]=Some(operation_id.0);self.tools.pending_tool_lease_admission.as_mut().unwrap().operation=Some(operation);
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<semio_framework_job::Operation>(),..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        let pending=self.tools.pending_tool_lease_admission.as_mut().unwrap();
        if pending.lease.is_none(){
            let operation=pending.operation.unwrap();let key=ToolOperationKey{app_instance_id:pending.instance,document:ArtifactDocumentAuthority(pending.instance),operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation};
            let lease=if keyed{self.tools.tool_cancellations.begin_keyed(&key,cx)?}else{self.tools.tool_cancellations.begin(&key,cx)?};
            *pending.lease=lease;return Ok(None)
        }
        if let Some(original)=pending.verb.as_ref(){
            let bytes=original.capacity();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
            drop(pending.verb.take());cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        if let Some(source)=pending.source.as_mut(){pending.returning=true;source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){pending.source.take();}return Ok(None)}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<OriginalToolLeaseAdmission>()||grant.maximum_depth==0{return Ok(None)}
        let mut pending=self.tools.pending_tool_lease_admission.take().unwrap();let result=(pending.operation.take().unwrap(),pending.lease.take().unwrap());cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<OriginalToolLeaseAdmission>(),..Default::default()})?;cx.consume_fuel(1);Ok(Some(result))
    }
}
