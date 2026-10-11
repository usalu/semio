use mounted_child_group_receipt_group::{MountedGroupMemberReceipts,MountedCommandEntry,MountedCommandEntrySource,MountedGroupReceiptOutput};
use mounted_child_group_receipt::MountedArtifactHandleIssuer;
use std::mem::ManuallyDrop;

struct MountedPrivateChildGroup<M:SpaceMember+MemberFactory> {
    identity:MountedChildGroupIdentityIssuer,
    parent_issuer:MountedArtifactHandleIssuer,
    parent_issuer_closed:bool,
    parent_handle:Option<ArtifactHandle>,
    parent_touched:bool,
    group_id:ManuallyDrop<Option<String>>,
    parent_batch:ManuallyDrop<Option<store::MemberStoreOwnedBatch>>,
    publication:ManuallyDrop<Option<PrivateChildPublicationGroup<M>>>,
    receipts:ManuallyDrop<Option<Box<MountedGroupMemberReceipts>>>,
    command:Option<MountedCommandEntry>,
    append:Option<CommandAppend>,
    output:ManuallyDrop<Option<MountedGroupReceiptOutput>>,
    record:ManuallyDrop<Option<CommandLogEntry>>,
    record_close_field:usize,
    displaced_shell_redo:ManuallyDrop<Vec<ShellHistoryReplay>>,
    phase:u8,
    captured_millis:i64,
    expires_at_us:u64,
    open_sequence:u64,
    context:Option<semio_framework_job::StepContextOwner>,
    closing:bool,
}

impl<M:SpaceMember+MemberFactory> MountedPrivateChildGroup<M> {
    fn new(captured_millis:i64,expires_at_us:u64)->Self{Self{identity:MountedChildGroupIdentityIssuer::new(),parent_issuer:MountedArtifactHandleIssuer::new(),parent_issuer_closed:false,parent_handle:None,parent_touched:false,group_id:ManuallyDrop::new(None),parent_batch:ManuallyDrop::new(None),publication:ManuallyDrop::new(None),receipts:ManuallyDrop::new(None),command:None,append:None,output:ManuallyDrop::new(None),record:ManuallyDrop::new(None),record_close_field:0,displaced_shell_redo:ManuallyDrop::new(Vec::new()),phase:0,captured_millis,expires_at_us,open_sequence:0,context:None,closing:false}}
    fn frame_birth_bytes()->usize{std::mem::size_of::<Self>()}
    fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        use mounted_child_group_receipt::mounted_nested_retirement_demand as nested;
        let release = |release_bytes, depth| RetirementDemand { release_bytes, depth, ..Default::default() };
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if self.output.is_some() { return Ok(release(0, 2)); }
        if let Some(record) = self.record.as_ref() { return nested(PagedCommandLog::record_retirement_demands(record,self.record_close_field)?,1); }
        if let Some(replay) = self.displaced_shell_redo.last() {
            return if replay.redo.action_id.capacity() != 0 { Ok(release(replay.redo.action_id.capacity(),2)) } else if let Some(value)=replay.redo.args.as_ref() { nested(PagedCommandLog::value_retirement_demands(value)?,2) } else { Ok(release(0,2)) };
        }
        if self.displaced_shell_redo.capacity() != 0 {
            let bytes = std::alloc::Layout::array::<ShellHistoryReplay>(self.displaced_shell_redo.capacity()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "mounted replay backing exceeds addressable ownership"))?.size();
            return Ok(release(bytes, 1));
        }
        if let Some(append) = self.append.as_ref() { return Ok(release(append.next_close_byte_demand(), 2)); }
        if let Some(command) = self.command.as_ref() { return Ok(release(command.next_close_byte_demand(), 2)); }
        if let Some(receipts) = self.receipts.as_ref() { return if receipts.terminal_is_empty() { Ok(release(MountedGroupMemberReceipts::frame_birth_bytes(), 1)) } else { nested(receipts.retirement_demands()?, 2) }; }
        if let Some(group) = self.publication.as_ref() { return nested(group.retirement_demands(maximum_body_bytes)?, 1); }
        if let Some(context) = self.context.as_ref() {
            return nested(RetirementDemand { copy_bytes: context.next_close_copy_byte_demand()?, capacity_bytes: context.next_close_capacity_byte_demand(maximum_body_bytes)?, release_bytes: context.next_close_release_byte_demand()?, depth: context.next_close_depth_demand()? }, 1);
        }
        if let Some(batch) = self.parent_batch.as_ref() { return nested(batch.next_demands(maximum_body_bytes)?, 1); }
        if let Some(id) = self.group_id.as_ref() { return Ok(release(id.capacity(), 1)); }
        if !self.identity.terminal_is_empty() { return Ok(release(self.identity.next_close_byte_demand(), 2)); }
        Ok(release(0, 2))
    }
    fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.record.is_none()&&self.displaced_shell_redo.capacity()==0&&self.append.is_none()&&self.command.is_none()&&self.receipts.is_none()&&self.publication.is_none()&&self.context.is_none()&&self.parent_batch.is_none()&&self.group_id.is_none()&&self.identity.terminal_is_empty()&&self.parent_issuer_closed}
    fn close_step(&mut self,parent:&mut impl SpaceMember,registry:&mut ChildMemberRegistry<M>,graph:&mut store::CompositionGraph,live:&ChildContentView,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,Fault>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        use semio_framework_job::InteractiveJobCloseStep;
        let pending=|step:RetainedCloneStep| InteractiveJobCloseStep::Pending{progress:step.progress()};
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()});}
        let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        if !mounted_child_group_receipt::mounted_retirement_grant_funds(demand, grant){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()});}
        let bytes=demand.release_bytes;
        let child_grant=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        self.closing=true;
        if self.output.is_some(){return Ok(pending(self.receipts.as_mut().and_then(|receipts|receipts.final_receipt_mut()).expect("transferred output retains its exact empty receipt owner").retain_output(&mut self.output,child_grant)));}
        if let Some(record)=self.record.as_mut(){if self.record_close_field==8{self.record.take();}else if PagedCommandLog::record_close_one(record,self.record_close_field,child_grant.maximum_release_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?{self.record_close_field+=1;}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if let Some(replay)=self.displaced_shell_redo.last_mut(){if replay.redo.action_id.capacity()!=0{drop(std::mem::take(&mut replay.redo.action_id));}else if let Some(value)=replay.redo.args.as_mut(){if matches!(value,DslValue::Null){replay.redo.args.take();}else{PagedCommandLog::value_close_one(value);}}else{self.displaced_shell_redo.pop();}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if self.displaced_shell_redo.capacity()!=0{drop(std::mem::take(&mut*self.displaced_shell_redo));return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if let Some(append)=self.append.as_mut(){let step=append.close_step(child_grant);self.append=None;return Ok(pending(step));}
        if let Some(command)=self.command.as_mut(){let step=command.close_step(child_grant);if command.terminal_is_empty(){self.command=None;}return Ok(pending(step));}
        if let Some(receipts)=self.receipts.as_mut(){if !receipts.terminal_is_empty(){return receipts.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 2, ..child_grant }).map(pending).map_err(|error|plugin_sdk_fault(error.to_string()));}drop(self.receipts.take());return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if let Some(group)=self.publication.as_mut(){let step=group.close_step(parent,registry,graph,live,child_grant)?;if group.terminal_is_empty(){self.publication.take();}return Ok(pending(step));}
        if let Some(context)=self.context.as_mut(){
            let step=context.close_step(child_grant).admit(child_grant,context.terminal_is_empty());
            return Ok(match step{
                InteractiveJobCloseStep::Complete{progress}=>{self.context=None;InteractiveJobCloseStep::Pending{progress}},
                step=>step,
            });
        }
        if let Some(batch)=self.parent_batch.as_mut(){let step=batch.close_granted(child_grant).map_err(|error|plugin_sdk_fault(error.to_string()))?;if batch.terminal_is_empty(){self.parent_batch.take();}return Ok(pending(step));}
        if self.group_id.is_some(){drop(self.group_id.take());return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if !self.identity.terminal_is_empty(){return Ok(pending(self.identity.close_granted(child_grant)));}
        if !self.parent_issuer_closed{let step=self.parent_issuer.close_step(child_grant);self.parent_issuer_closed=true;return Ok(pending(step));}
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }
}
impl<M:SpaceMember+MemberFactory> Drop for MountedPrivateChildGroup<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"mounted private child frame retains original inputs and staged owners until funded terminal close");}}


impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> VcsArtifactApp<A,M>{
    /// 🧪️ Borrows original completed-output ownership and refusal observations without widening production visibility.
    #[cfg(test)]
    pub(crate) fn test_original_owned_emit_observation(&self,operation:u64)->Option<(usize,usize,usize,bool,bool)>{
        let mounted=self.tool_operations.get(operation)?;
        let ArtifactToolCompletionValue::Emit(Ok(emit),_)=mounted.publication.as_ref()? else{return None;};
        Some((emit.artifact_mutations.len(),emit.owned_child_emits.len(),emit.owned_child_emits.as_ptr() as usize,mounted.terminal_fault.is_some(),mounted.cancellation_lease.as_ref()?.token.is_cancelled_now()))
    }

    async fn mount_original_emit_publication(&mut self,verb:&str,emit:Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>,meta:&ActionMeta,authority:Option<(semio_framework_job::Operation,ToolCancellationLease,semio_framework_value::native_encoding::NativeEncodeContinuation)>)->Result<InvocationResult,Fault>{
        self.mount_original_reserved_owners(verb,Some(ArtifactToolCompletionValue::Emit(Ok(emit),EphemeralEmit::default())),None,None,meta,authority).await
    }

    /// 🎒️ Installs original reserved producer and completion owners before fallible publication capture.
    async fn mount_original_reserved_owners(&mut self,verb:&str,publication:Option<ArtifactToolCompletionValue<A>>,reserved_producer:Option<ArtifactReservedToolJob>,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,authority:Option<(semio_framework_job::Operation,ToolCancellationLease,semio_framework_value::native_encoding::NativeEncodeContinuation)>)->Result<InvocationResult,Fault>{
        let revision=self.store.content_revision_now();
        let generation=semio_framework_job::Generation(self.store.generation_now());
        let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));
        let(operation,lease,identity)=match authority{Some((operation,lease,identity))=>(operation,lease,Some(identity)),None=>{let operation_id=self.admit_typed_operation_slot().ok_or_else(||plugin_sdk_fault("every fixed publication slot already retains an original operation"))?;let operation=semio_framework_job::Operation::new(operation_id,base_revision,generation,operation_id.0);let lease=self.tool_cancellations.begin_keyed(ToolOperationKey{app_instance_id:meta.instance_id,document:ArtifactDocumentAuthority(meta.instance_id),operation_id,base_revision,generation})?;(operation,lease,None)}};
        let operation_id=operation.operation.0;
        let reservation=self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
        if !self.tool_operations.can_insert(operation_id)||reservation.is_some_and(|owner|owner!=operation_id){return Err(plugin_sdk_fault("original emission lost its exact pre-admitted publication slot"));}
        let mounted=MountedTypedCommandFullOperation::<A>{
            verb:verb.into(),meta:meta.clone(),operation,canonical_revision:revision,artifact_generation:generation.0,
            config_generation:self.config_store.generation_now(),draft_generation:self.draft_store.generation_now(),presence_generation:0,transient_generation:0,
            window_config_authority:None,window_transient_authority:None,publication_lanes:&[],
            session:None,session_rejected:None,reserved_producer,completion,completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input:None,output_chunks:None,cancellation_lease:Some(lease),identity,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},terminal_seen:true,worker_semantic_pending:false,worker_outcome_pending:false,worker_fault_capture:None,pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,original_retirement_receipt:None,
            publication,publication_ownership_progress:None,actor_capture:None,worker_resume_pending:false,pending_artifact_publication:None,pending_child_publication:None,
            owned_child_group:None,owned_child_committed:false,owned_child_result_pending:false,
            captured_child_content:Some(std::sync::Arc::new(ChildContentView::clone(&*self.child_content_root))),captured_child_content_generation:self.child_content_generation,
            result_page:None,result_page_presented:false,result_sequence:0,publication_progress:0,publication_checkpoint:None,publication_attempt:0,
            ui_pending:true,progress:None,progress_pending:false,user_cancel_requested:false,published_artifact:false,published_config:false,published_window_config:false,
            command_logged:false,interaction_revalidated:false,retained_close_fault: None,retained_close_fault_retirement:None,retained_close_fault_refusal:None, terminal_fault:None,stage:MountedTypedCommandFullOperationStage::Publishing,
        };
        self.tool_operations.insert_admitted(operation_id,mounted);
        self.release_reserved_emit_slot(operation_id);
        let capture:Result<_,Fault>=async{
            if self.live_runtime_instance_id!=Some(meta.instance_id){return Err(plugin_sdk_fault("original emission does not belong to its mounted live app instance"));}
            let proof=self.qualified_tool_proof(verb)?;
            let window_config=self.window_config_store.capture(meta.view_state.as_ref()).await?;
            let window_transient=self.window_transient_store.capture(meta.view_state.as_ref())?;
            Ok((proof.publication_lanes(),proof.contract().max_output_bytes,window_config,window_transient,self.presence_store.generation().await,self.transient_store.generation().await))
        }.await;
        let mounted=self.tool_operations.get_mut(operation_id).expect("original completed output remains in its exact admitted slot");
        match capture{
            Ok((lanes,maximum_identity_bytes,window_config,window_transient,presence,transient))=>{
                if mounted.identity.is_none(){let lease=mounted.cancellation_lease.as_ref().expect("original publication owns its real lease");let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum_identity_bytes,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);}
                mounted.publication_lanes=lanes;mounted.window_config_authority=window_config;mounted.window_transient_authority=window_transient;
                mounted.presence_generation=presence;mounted.transient_generation=transient;
                if let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut(){let inline=take_inline_interaction_verbs(&mut emit.effects);if !inline.is_empty(){self.typed_inline_interaction_verbs.push((operation_id,inline));}}
            }
            Err(fault)=>{mounted.terminal_fault.get_or_insert_with(||ArtifactBoundedToolFault::from_fault(&fault));mounted.cancellation_lease.as_ref().expect("original keyed lease remains retained").cancel();}
        }
        let mut result=Self::empty_result(verb,meta,Vec::new(),Vec::new(),UiDirtyScope::None).await;
        result.output=DslValue::Object(vec![("operationId".into(),DslValue::String(operation_id.to_string())),("generation".into(),DslValue::String(operation.generation.0.to_string()))]);
        Ok(result)
    }

    fn advance_private_child_group(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>)->Result<semio_framework_value::retained_clone::RetainedCloneStep,Fault>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        let step=(||->Result<RetainedCloneStep,Fault>{
        let operation=mounted.operation.operation.0;
        let kind=self.declared_dispatch_kind(&mounted.verb).unwrap_or(ActionKind::Mutation);
        let owner=self.private_child_groups.get_mut(operation).ok_or_else(||plugin_sdk_fault("mounted publication lost its original admitted group frame"))?;
        if owner.closing{return Err(plugin_sdk_fault("mounted private publication requires bounded retirement after cancellation"));}
        if owner.phase>=9{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let structural=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
        let error=|error:semio_framework_value::ValueError|plugin_sdk_fault(error.to_string());
        if owner.phase==0{
            if owner.identity.ready(){*owner.group_id=owner.identity.take_ready(self.mounted_policy.maintenance);owner.phase=1;return Ok(structural());}
            let source=MountedChildGroupIdentitySource{parent:&self.store.envelope().id,actor:&mounted.meta.actor,revision:mounted.canonical_revision,instance:mounted.meta.instance_id,operation};
            return owner.identity.advance(source,self.mounted_policy.maintenance).map_err(error);
        }
        if owner.phase==1{
            if owner.parent_issuer.ready(){owner.parent_handle=owner.parent_issuer.take(self.mounted_policy.maintenance);owner.parent_issuer_closed=true;owner.phase=2;return Ok(structural());}
            return owner.parent_issuer.advance(&self.store.envelope().id,self.mounted_policy.maintenance).map_err(error);
        }
        if owner.phase==2{
            let birth=A::owned_mutation_batch_birth_bytes().ok_or_else(||plugin_sdk_fault("owned child publication requires the parent app's exact retained mutation factory"))?;
            let grant=self.mounted_policy.maintenance;
            let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut()else{return Err(plugin_sdk_fault("mounted group lost its original typed emission"));};
            owner.parent_touched=!emit.artifact_mutations.is_empty();
            let mut original=Some(std::mem::take(&mut emit.artifact_mutations));
            match A::admit_owned_mutation_batch(&mut original,grant){
                Ok(Some((batch,progress)))=>{assert!(original.is_none(),"admitted parent batch owns its original vector");*owner.parent_batch=Some(batch);owner.phase=3;return Ok(RetainedCloneStep::Progress(progress));},
                Ok(None)=>{emit.artifact_mutations=original.take().expect("refused parent birth retains its exact original vector");return Ok(RetainedCloneStep::Progress(Default::default()));},
                Err(reason)=>{emit.artifact_mutations=original.take().expect("rejected parent birth retains its exact original vector");return Err(error(reason));}
            }
        }
        if owner.phase==3{
            let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut()else{return Err(plugin_sdk_fault("mounted group lost its original typed emission"));};
            if emit.owned_child_emits.is_empty()||emit.owned_child_emits.len()>PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN{return Err(plugin_sdk_fault("mounted group exceeds its original declared child admission"));}
            let source=PrivateChildGroupSource{parent:owner.parent_batch.take().unwrap(),parent_touched:owner.parent_touched,children:std::mem::take(&mut emit.owned_child_emits),transaction:emit.transaction.take(),group_id:owner.group_id.take().unwrap()};
            match PrivateChildPublicationGroup::try_new(source){Ok(group)=>{*owner.publication=Some(group);owner.phase=4;return Ok(structural());},Err(source)=>{*owner.parent_batch=Some(source.parent);*owner.group_id=Some(source.group_id);emit.owned_child_emits=source.children;emit.transaction=source.transaction;return Err(plugin_sdk_fault("mounted private group refused its exact original child source"));}}
        }
        let group=owner.publication.as_mut().expect("admitted private publication retains its original group");
        if !group.inputs_ready(){let envelope=self.store.envelope();let dialect=envelope.dialect.as_ref().ok_or_else(||plugin_sdk_fault("private group requires its parent's original declared dialect"))?;let capacity=group.next_input_capacity_byte_demand(&self.children,&envelope.id,dialect,&mounted.meta.actor).map_err(error)?;return group.advance_inputs(&self.children,&envelope.id,dialect,&mounted.meta.actor,mounted.operation.operation,mounted.operation.generation,owner.expires_at_us,semio_framework_job::default_now_us().unwrap_or(0),self.mounted_policy.maintenance).map_err(error);}
        if !group.openings_ready(){
            if owner.context.is_none(){let birth=semio_framework_job::StepContextOwner::birth_bytes();let grant=self.mounted_policy.maintenance;let(context,progress)=semio_framework_job::StepContextOwner::new(mounted.operation.operation,mounted.operation.generation,grant).map_err(error)?;owner.context=Some(context);return Ok(RetainedCloneStep::Progress(progress));}
            let demand=group.next_open_retirement_demands(64).map_err(error)?;if demand.copy_bytes>64||demand.depth>64{return Err(plugin_sdk_fault("private open retirement exceeds its original copy or depth admission"));}let birth=group.next_open_birth_demand()?;if birth.depth>64{return Err(plugin_sdk_fault("private open birth exceeds its original depth admission"));}let capacity=birth.capacity_bytes.max(demand.capacity_bytes);let grant=self.mounted_policy.maintenance;let cancel=mounted.cancellation_lease.as_ref().ok_or_else(||plugin_sdk_fault("private child opening requires its original operation lease"))?.token.clone();let deadline=semio_framework_job::default_now_us().unwrap_or(0).saturating_add(INTERACTIVE_TURN_WORKER_WALL_US);let mut actual_retained_progress=Default::default();let mut cx=owner.context.as_ref().unwrap().context(semio_framework_job::StepBudget::new(1,deadline,grant),cancel,semio_framework_job::default_now_us,&mut owner.open_sequence,&mut actual_retained_progress).ok_or_else(||plugin_sdk_fault("private child opening context is already closing"))?;return group.advance_openings(&mut cx,grant,M::begin_open);
        }
        if owner.receipts.is_none(){let birth=MountedGroupMemberReceipts::frame_birth_bytes();let grant=self.mounted_policy.maintenance;let receipts=MountedGroupMemberReceipts::new(group,&mounted.meta.actor,owner.parent_handle.unwrap()).map_err(error)?;*owner.receipts=Some(Box::new(receipts));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:grant.maximum_items,retained_capacity_bytes:birth,..Default::default()}));}
        let receipts=owner.receipts.as_mut().unwrap();
        if let Some(index)=receipts.receipt_index(){if group.prepared_publication(index).is_some(){let capacity=receipts.next_capacity_byte_demand(group,&mounted.meta.actor).map_err(error)?;let release=receipts.next_release_byte_demand(group,&mounted.meta.actor).map_err(error)?;return receipts.advance_receipt(group,&self.children,&mounted.meta.actor,self.mounted_policy.maintenance).map_err(error);}}
        if !group.publications_ready(){let capacity=group.next_publication_capacity_byte_demand(&self.store,&self.children)?.max(4096);return group.advance_publications(&mut self.store,&mut self.children,mounted.operation.operation,self.mounted_policy.maintenance);}
        if !group.projection_ready(){return group.advance_projection(&self.store,self.mounted_policy.maintenance);}
        if !group.entries_ready(){return group.advance_entries(&mut self.children,&self.child_content_root,self.mounted_policy.maintenance);}
        let grant=self.mounted_policy.maintenance;let step=group.close_displaced_view_step(&self.child_content_root,grant)?;semio_framework_value::retained_clone::admit_retained_clone_progress(grant,step.progress(),"mounted private displaced view").map_err(semio_framework_value::ValueError::into_fault)?;if step.progress()!=Default::default()||!matches!(step,RetainedCloneStep::Complete(_)){return Ok(RetainedCloneStep::Progress(step.progress()));}
        let graph=self.composition.ownership_graph_mut();if !group.ready(graph){let demand=group.next_graph_demands(graph,&self.children)?;if demand.copy_bytes>4096{return Err(plugin_sdk_fault("mounted graph source words exceed the admitted per-turn work frontier"));}let grant=self.mounted_policy.maintenance;return group.advance_graph(graph,&self.children,grant);}
        if !receipts.final_ready(){let capacity=receipts.next_final_capacity_byte_demand(group.group_id()).map_err(error)?;let release=receipts.next_final_release_byte_demand();return receipts.advance_final(group.group_id(),self.mounted_policy.maintenance).map_err(error);}
        let label=self.registry.get(&mounted.verb).map(|definition|&definition.label).or_else(||self.registry.get_command(&mounted.verb).map(|definition|&definition.label));
        let source=MountedCommandEntrySource{action_id:&mounted.verb,label,captured_millis:owner.captured_millis,kind,count:1,parent_touched:group.parent_touched(),child_edit_count:group.child_count()};
        if owner.command.is_none(){owner.command=Some(MountedCommandEntry::new(source).map_err(|_|plugin_sdk_fault("mounted command entry refused its original timestamp authority"))?);return Ok(structural());}
        let command=owner.command.as_mut().unwrap();if !command.ready(){let capacity=command.next_capacity_byte_demand(source).map_err(|_|plugin_sdk_fault("mounted command entry source changed before prestage"))?;return command.advance(source,self.mounted_policy.maintenance).map_err(|_|plugin_sdk_fault("mounted command entry lost its original borrowed label source"));}
        if owner.append.is_none(){owner.append=Some(self.command_log.prepare_append().map_err(|_|plugin_sdk_fault("mounted command history refused its next original page"))?);return Ok(structural());}
        let append=owner.append.as_mut().unwrap();match self.command_log.validate_append(append){Ok(())=>{},Err(mounted_child_group_receipt_group::CommandAppendRefusal::Unfunded)=>{return Ok(append.admit_step(self.mounted_policy.maintenance));},Err(mounted_child_group_receipt_group::CommandAppendRefusal::Stale)=>{let step=append.close_step(self.mounted_policy.maintenance);owner.append=None;return Ok(step);},Err(_)=>return Err(plugin_sdk_fault("mounted command append lost its exact original history authority"))}
        owner.phase=9;Ok(RetainedCloneStep::Complete(Default::default()))
        })()?;
        let ready=self.private_child_groups.get(mounted.operation.operation.0).is_some_and(|owner|owner.phase>=9);
        Ok(if ready{step}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn admit_private_child_group_frame(&mut self,operation:u64,captured_millis:i64,expires_at_us:u64,grant:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,Fault>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.private_child_groups.get(operation).is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.private_child_groups.can_insert(operation){return Err(plugin_sdk_fault("mounted child group lacks its exact original fixed registry slot"));}
        let bytes=MountedPrivateChildGroup::<M>::frame_birth_bytes();
        if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.private_child_groups.insert_admitted(operation,Box::new(MountedPrivateChildGroup::new(captured_millis,expires_at_us)));
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
    }
    async fn publish_mounted_owned_child_operation_unit(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>)->Result<(),Fault>{
        let operation=mounted.owned_child_group.ok_or_else(||plugin_sdk_fault("mounted private publication lost its exact operation identity"))?;
        let Some(owner)=self.private_child_groups.get(operation)else{mounted.owned_child_group=None;return Ok(());};
        let lease=mounted.cancellation_lease.as_ref().ok_or_else(||plugin_sdk_fault("mounted private publication requires its original cancellation lease"))?;
        let cancelled=lease.token.is_cancelled_now()||lease.publication_claim.is_cancelled()||lease.handle.publication_scope.is_cancelled()||lease.document_claim.as_ref().is_some_and(|claim|claim.is_cancelled());
        if cancelled||(owner.closing&&!mounted.owned_child_result_pending)||(mounted.owned_child_committed&&!mounted.owned_child_result_pending){
            let grant=self.mounted_policy.close;
            let demand=self.private_child_group_operation_close_demands(operation,grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
            if demand.copy_bytes>grant.maximum_copy_bytes||demand.depth>grant.maximum_depth{return Err(plugin_sdk_fault("mounted retirement exceeds its original copy or depth admission"));}
            self.close_private_child_group_operation_step(operation,grant)?;
            if self.private_child_groups.get(operation).is_none(){mounted.owned_child_group=None;if !mounted.owned_child_committed{mounted.reject_cancelled_publication()?;}}
            return Ok(());
        }
        if mounted.owned_child_committed{return Ok(());}
        let Some(_permit)=lease.try_claim_publication()else{return Ok(());};
        let captured=mounted.captured_child_content.as_ref().ok_or_else(||plugin_sdk_fault("mounted child group lost its captured immutable child frontier"))?;
        let fresh=typed_operation_document_is_fresh(&mounted.operation,mounted.canonical_revision,self.store.content_revision_now(),self.store.generation_now())&&mounted.captured_child_content_generation==self.child_content_generation&&captured.identity_digest()==self.child_content_root.identity_digest();
        if !fresh{let fault=plugin_sdk_fault("mounted private group rejected its stale original parent or child frontier");self.private_child_groups.get_mut(operation).unwrap().closing=true;mounted.terminal_fault=Some(ArtifactBoundedToolFault::from_fault(&fault));lease.cancel();return Err(fault);}
        let step=match self.advance_private_child_group(mounted){Ok(step)=>step,Err(fault)=>{self.private_child_groups.get_mut(operation).unwrap().closing=true;mounted.terminal_fault=Some(ArtifactBoundedToolFault::from_fault(&fault));mounted.cancellation_lease.as_ref().unwrap().cancel();return Err(fault);}};
        if step.progress()!=Default::default(){mounted.publication_progress=mounted.publication_progress.saturating_add(1);}
        if !matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){return Ok(());}
        if !self.typed_composed_outbox.can_push(){return Ok(());}
        let generation=self.child_content_generation.checked_add(1).ok_or_else(||plugin_sdk_fault("private child frontier generation exhausted"))?;
        if !self.child_content_root.is_empty()&&!self.child_content_retirements.can_insert(generation){return Ok(());}
        let sequence=self.next_command_seq.checked_add(1).ok_or_else(||plugin_sdk_fault("private command history sequence exhausted"))?;
        let log_generation=self.log_generation.checked_add(1).ok_or_else(||plugin_sdk_fault("private command history generation exhausted"))?;
        let mut page=TypedOperationResultPage::try_new(mounted.next_token(),TypedOperationResultLane::Child,b"")?;
        let owner=self.private_child_groups.get_mut(operation).unwrap();
        let append=owner.append.as_mut().unwrap();if self.command_log.validate_append(append).is_err(){owner.phase=8;return Ok(());}
        let group=owner.publication.as_mut().unwrap();let graph=self.composition.ownership_graph_mut();if !group.ready(graph){return Ok(());}
        if owner.output.is_none(){*owner.output=owner.receipts.as_mut().unwrap().take_final(self.mounted_policy.maintenance);}
        let output=owner.output.as_mut().ok_or_else(||plugin_sdk_fault("preborn private receipt refused its exact original handoff"))?;
        if owner.record.is_none(){*owner.record=owner.command.as_mut().unwrap().take(&mut output.command_edit_id,&mut output.command_child_edit_ids,sequence,self.mounted_policy.maintenance);}
        owner.record.as_mut().ok_or_else(||plugin_sdk_fault("preborn private command row refused its original receipt cardinality"))?.seq=sequence;
        let Some(next)=group.commit(&mut self.store,&mut self.children,graph,store::ArtifactStoreOneItemGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:64})?else{return Ok(());};
        let previous=std::mem::replace(&mut*self.child_content_root,next);if !previous.is_empty(){self.child_content_retirements.insert_admitted(generation,ChildContentRetirement::new(previous,false));}
        self.child_content_generation=generation;
        self.command_log.commit_append(append,&mut owner.record).expect("common private decision retains the validated original command prefix");self.next_command_seq=sequence;self.log_generation=log_generation;*owner.displaced_shell_redo=std::mem::take(&mut self.shell_redo);
        let output=owner.output.take().unwrap();self.typed_composed_outbox.push(ComposedGestureResult{operation,mutations:output.mutations,inverse_group:output.inverse_group}).unwrap_or_else(|_|unreachable!("validated fixed private receiver retains its exact original free slot"));
        owner.closing=true;mounted.owned_child_committed=true;mounted.owned_child_result_pending=true;mounted.command_logged=true;mounted.published_artifact=true;mounted.artifact_generation=self.store.generation_now();mounted.canonical_revision=self.store.content_revision_now();mounted.operation.base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().unwrap()));mounted.operation.generation=semio_framework_job::Generation(mounted.artifact_generation);page.token=mounted.next_token();mounted.queue_page(page)
    }
    fn private_child_group_close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let Some((_,operation))=self.private_child_groups.next_id_from(0) else {
            let release_bytes=self.private_child_groups.empty_backing_byte_demand().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private group registry still owns its original slot"))?;
            return Ok(semio_framework_value::RetirementDemand { release_bytes, depth:usize::from(release_bytes!=0), ..Default::default() });
        };
        self.private_child_group_operation_close_demands(operation,body)
    }
    fn next_closing_private_child_group(&self)->Option<u64>{(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index|self.private_child_groups.entry(index).and_then(|(operation,owner)|(owner.closing&&self.tool_operations.get(*operation).is_none_or(|mounted|!mounted.owned_child_result_pending)).then_some(*operation)))}
    fn private_child_group_operation_close_demands(&self, operation:u64, body:usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let Some(owner)=self.private_child_groups.get(operation)else{return Ok(Default::default());};
        if owner.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{release_bytes:MountedPrivateChildGroup::<M>::frame_birth_bytes(),depth:1,..Default::default()});}
        mounted_child_group_receipt::mounted_nested_retirement_demand(owner.retirement_demands(body)?,1)
    }
    fn private_child_groups_terminal_is_empty(&self)->bool{self.private_child_groups.is_empty()&&self.private_child_groups.empty_backing_byte_demand()==Some(0)}
    fn close_private_child_group_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,Fault>{
        let demand=self.private_child_group_close_demands(grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        if self.private_child_groups_terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});}
        if !mounted_child_group_receipt::mounted_retirement_grant_funds(demand,grant){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        let Some((_,operation))=self.private_child_groups.next_id_from(0)else{
            return match self.private_child_groups.close_empty_backing_step(RetainedCloneGrant{maximum_items:grant.maximum_items.min(1),..grant})?{
                RetainedCloneStep::Progress(progress)=>Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress}),
                RetainedCloneStep::Complete(progress)=>Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress}),
            };
        };
        self.close_private_child_group_operation_step(operation,grant)
    }
    fn close_private_child_group_operation_step(&mut self,operation:u64,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,Fault>{
        let Some(original)=self.private_child_groups.get(operation)else{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});};
        let terminal=original.terminal_is_empty();
        let demand=self.private_child_group_operation_close_demands(operation,grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        if !mounted_child_group_receipt::mounted_retirement_grant_funds(demand,grant){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        if terminal{drop(self.private_child_groups.remove(operation));return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}});}
        let owner=self.private_child_groups.get_mut(operation).ok_or_else(||plugin_sdk_fault("private group changed during its exact bounded close"))?;
        let child_grant=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        let step=owner.close_step(&mut self.store,&mut self.children,self.composition.ownership_graph_mut(),&self.child_content_root,child_grant)?;
        Ok(step.admit(child_grant,owner.terminal_is_empty()))
    }

}

#[cfg(test)]
include!("🧪️tests/🦀️.rs");

#[cfg(test)]
include!("../../../../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs");
