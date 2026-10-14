/// 🎟️ Original caller admission of one retained tool key before producer donation.
struct OriginalToolLeaseAdmission {
    source:ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,
    verb:ManuallyDrop<Option<String>>,
    operation:Option<semio_framework_job::Operation>,
    lease:ManuallyDrop<Option<ToolCancellationLease>>,
    retirement:ManuallyDrop<Option<crate::tool_cancellation::ToolCancellationLeaseRetirement>>,
    instance:u32,
    identity:(semio_framework_job::OperationId,semio_framework_job::Generation,usize),
    keyed:bool,
    copied:bool,
    returning:bool,
}
impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> VcsArtifactApp<A,M> {
    fn admit_original_tool_lease(&mut self,verb:&str,meta:&ActionMeta,keyed:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<(semio_framework_job::Operation,ToolCancellationLease)>,semio_framework_job::OriginalTurnFailure>{
        if self.pending_tool_lease_admission.is_none(){
            let bytes=size_of::<OriginalToolLeaseAdmission>();let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
            *self.pending_tool_lease_admission=Some(OriginalToolLeaseAdmission{source:ManuallyDrop::new(None),verb:ManuallyDrop::new(Some(String::new())),operation:None,lease:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),instance:meta.instance_id,identity:(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?),keyed,copied:false,returning:false});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        let pending=self.pending_tool_lease_admission.as_mut().unwrap();
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
            let Some(operation_id)=self.admit_typed_operation_slot()else{return Ok(None)};
            let revision=self.store.content_revision_now();let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));let generation=semio_framework_job::Generation(self.store.generation_now());
            let operation=semio_framework_job::Operation::new(operation_id,base_revision,generation,operation_id.0);
            self.typed_operation_reservations[operation_id.0 as usize%ARTIFACT_LIVE_OUTPUT_SLOTS]=Some(operation_id.0);self.pending_tool_lease_admission.as_mut().unwrap().operation=Some(operation);
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<semio_framework_job::Operation>(),..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        let pending=self.pending_tool_lease_admission.as_mut().unwrap();
        if pending.lease.is_none(){
            let operation=pending.operation.unwrap();let key=ToolOperationKey{app_instance_id:pending.instance,document:ArtifactDocumentAuthority(pending.instance),operation_id:operation.operation,base_revision:operation.base_revision,generation:operation.generation};
            let lease=if keyed{self.tool_cancellations.begin_keyed(&key,cx)?}else{self.tool_cancellations.begin(&key,cx)?};
            *pending.lease=lease;return Ok(None)
        }
        if let Some(original)=pending.verb.as_ref(){
            let bytes=original.capacity();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(None)}
            drop(pending.verb.take());cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(None)
        }
        if let Some(source)=pending.source.as_mut(){pending.returning=true;source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){pending.source.take();}return Ok(None)}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<OriginalToolLeaseAdmission>()||grant.maximum_depth==0{return Ok(None)}
        let mut pending=self.pending_tool_lease_admission.take().unwrap();let result=(pending.operation.take().unwrap(),pending.lease.take().unwrap());cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<OriginalToolLeaseAdmission>(),..Default::default()})?;cx.consume_fuel(1);Ok(Some(result))
    }
}

impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> VcsArtifactApp<A,M> {
    fn original_tool_lease_close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        use semio_framework_value::RetirementDemand;
        if self.pending_tool_lease_admission.is_none()&&self.pending_media_import_permit.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<(FrameworkReservedCommitPermit,(semio_framework_job::OperationId,semio_framework_job::Generation,usize))>()+size_of::<OriginalToolLeaseAdmission>(),depth:1,..Default::default()})}
        let Some(pending)=self.pending_tool_lease_admission.as_ref()else{return Ok(Default::default())};
        if let Some(owner)=pending.retirement.as_ref(){return owner.retirement_demands(self.tool_cancellations.handle().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original pending lease lost its issuer"))?,body)}
        if pending.lease.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ToolCancellationLease>(),depth:1,..Default::default()})}
        if let Some(verb)=pending.verb.as_ref(){return Ok(RetirementDemand{release_bytes:verb.capacity(),depth:1,..Default::default()})}
        if pending.operation.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_job::Operation>(),depth:1,..Default::default()})}
        if let Some(source)=pending.source.as_ref(){return Ok(source.retirement_demands())}
        Ok(RetirementDemand{copy_bytes:size_of::<OriginalToolLeaseAdmission>(),depth:1,..Default::default()})
    }
    fn close_original_tool_lease(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
        let demand=self.original_tool_lease_close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||!crate::bounded_ownership::plugin_grant_funds(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if self.pending_tool_lease_admission.is_none(){if let Some((permit,identity))=self.pending_media_import_permit.take(){
            let FrameworkReservedCommitPermit{operation,lease,..}=permit;let instance=lease.key.app_instance_id;
            *self.pending_tool_lease_admission=Some(OriginalToolLeaseAdmission{source:ManuallyDrop::new(None),verb:ManuallyDrop::new(None),operation:Some(operation),lease:ManuallyDrop::new(Some(lease)),retirement:ManuallyDrop::new(None),instance,identity,keyed:true,copied:true,returning:true});
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
        }}
        let Some(pending)=self.pending_tool_lease_admission.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()))};
        if pending.identity!=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original tool lease close requires its original receiving caller").into())}
        if !pending.returning{if let Some(source)=pending.source.as_ref(){source.validate_context(cx)?;}}
        if let Some(owner)=pending.retirement.as_mut(){
            if owner.terminal_is_empty(){pending.retirement.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
            return owner.close_step(self.tool_cancellations.handle().unwrap(),grant).map_err(Into::into)
        }
        if let Some(lease)=pending.lease.take(){lease.cancel();*pending.retirement=Some(crate::tool_cancellation::ToolCancellationLeaseRetirement::new(lease));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
        if pending.verb.is_some(){drop(pending.verb.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}))}
        if let Some(operation)=pending.operation.take(){self.release_reserved_emit_slot(operation.operation.0);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
        if let Some(source)=pending.source.as_mut(){pending.returning=true;let step=source.close_step(cx.original_source_parent()?,cx)?;if source.terminal_is_empty(){pending.source.take();}return Ok(RetainedCloneStep::Progress(step.progress()))}
        self.pending_tool_lease_admission.take();Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
}
impl Drop for OriginalToolLeaseAdmission {
    fn drop(&mut self){assert!(std::thread::panicking()||(self.source.is_none()&&self.verb.is_none()&&self.lease.is_none()&&self.retirement.is_none()),"original tool lease admission still retains its issuing source")}
}
