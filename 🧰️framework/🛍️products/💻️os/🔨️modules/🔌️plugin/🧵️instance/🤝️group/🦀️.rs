//! 🤝️ The instance's private child group publication and mounted command log turns.
use crate::child_content::MountedChildGroupIdentitySource;
use crate::child_content::*;
use crate::child_content::mounted_child_group_receipt_group::{MountedGroupMemberReceipts,MountedCommandEntry,MountedCommandEntrySource};
use super::*;

impl<A:ArtifactApp,M:SpaceMember+MemberFactory+'static> VcsArtifactApp<A,M>{
    /// 🧪️ Borrows original completed-output ownership and refusal observations without widening production visibility.
    #[cfg(test)]
    pub(crate) fn test_original_owned_emit_observation(&self,operation:u64)->Option<(usize,usize,usize,bool,bool)>{
        let mounted=self.tool_operations.get(operation)?;
        let ArtifactToolCompletionValue::Emit(Ok(emit),_)=mounted.publication.as_ref()? else{return None;};
        Some((emit.artifact_mutations.len(),emit.owned_child_emits.len(),emit.owned_child_emits.as_ptr() as usize,mounted.terminal_fault.is_some(),mounted.cancellation_lease.as_ref()?.token.is_cancelled_now()))
    }

    pub(crate) async fn mount_original_emit_publication(&mut self,verb:&str,emit:Emit<A::Mutation,A::ConfigMutation,A::DraftMutation>,meta:&ActionMeta,authority:Option<(semio_framework_job::Operation,ToolCancellationLease,semio_framework_value::native_encoding::NativeEncodeContinuation)>, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        self.mount_original_reserved_owners(verb,Some(ArtifactToolCompletionValue::Emit(Ok(emit),EphemeralEmit::default())),None,None,meta,authority, cx).await
    }

    /// 🎒️ Installs original reserved producer and completion owners before fallible publication capture.
    pub(crate) async fn mount_original_reserved_owners(&mut self,verb:&str,publication:Option<ArtifactToolCompletionValue<A>>,reserved_producer:Option<ArtifactReservedToolJob>,completion:Option<ArtifactToolCompletion<A>>,meta:&ActionMeta,authority:Option<(semio_framework_job::Operation,ToolCancellationLease,semio_framework_value::native_encoding::NativeEncodeContinuation)>, cx:&mut semio_framework_job::StepContext<'_>)->Result<InvocationResult,Fault>{
        let revision=self.store.content_revision_now();
        let generation=semio_framework_job::Generation(self.store.generation_now());
        let base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().unwrap()));
        let(operation,lease,identity)=match authority{
            Some((operation,lease,identity))=>(operation,Some(lease),Some(identity)),
            None=>{let operation_id=self.admit_typed_operation_slot().ok_or_else(||plugin_sdk_fault("every fixed publication slot already retains an original operation"))?;let operation=semio_framework_job::Operation::new(operation_id,base_revision,generation,operation_id.0);(operation,None,None)}
        };
        let operation_id=operation.operation.0;
        let reservation=self.typed_operation_reservations[operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
        if !self.tool_operations.can_insert(operation_id)||reservation.is_some_and(|owner|owner!=operation_id){return Err(plugin_sdk_fault("original emission lost its exact pre-admitted publication slot"));}
        let mounted=MountedTypedCommandFullOperation::<A>{
            verb:verb.into(),meta:meta.clone(),operation,source_identity:(cx.operation(),cx.generation()),canonical_revision:revision,artifact_generation:generation.0,
            config_generation:self.config_store.generation_now(),draft_generation:self.draft_store.generation_now(),presence_generation:0,transient_generation:0,
            window_config_authority:None,window_transient_authority:None,publication_lanes:&[],
            session:None,session_rejected:None,reserved_producer,completion,completion_retirement:None,publication_retirement:None,output_retirement:None,raw_input:None,output_chunks:None,cancellation_lease:lease,identity,identity_progress:semio_framework_value::native_encoding::NativeEncodeProgress{completed:0,total:0,owned_bytes:0},terminal_seen:true,worker_semantic_pending:false,worker_outcome_pending:false,worker_fault_capture:None,pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,original_retirement_receipt:None,
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
                if let Some(lease)=mounted.cancellation_lease.as_ref().filter(|_|mounted.identity.is_none()){let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|!lease.token.is_cancelled_now()&&!lease.publication_claim.is_cancelled();mounted.identity=Some(semio_framework_value::NativeEncodeControl::new(maximum_identity_bytes,&mut observer).pause().map_err(|error|plugin_sdk_fault(error.to_string()))?);}
                mounted.publication_lanes=lanes;mounted.window_config_authority=window_config;mounted.window_transient_authority=window_transient;
                mounted.presence_generation=presence;mounted.transient_generation=transient;
                if let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut(){let inline=take_inline_interaction_verbs(&mut emit.effects);if !inline.is_empty(){self.typed_inline_interaction_verbs.push((operation_id,inline));}}
            }
            Err(fault)=>{mounted.terminal_fault.get_or_insert_with(||ArtifactBoundedToolFault::from_fault(&fault));if let Some(lease)=mounted.cancellation_lease.as_ref(){lease.cancel();}}
        }
        let mut result=Self::empty_result(verb,meta,Vec::new(),Vec::new(),UiDirtyScope::None).await;
        result.output=DslValue::Object(vec![("operationId".into(),DslValue::String(operation_id.to_string())),("generation".into(),DslValue::String(operation.generation.0.to_string()))]);
        Ok(result)
    }

    fn advance_private_child_group(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
        if self.private_child_groups.get(mounted.operation.operation.0).is_some_and(|owner|owner.phase>=9){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,RetirementDemand{depth:1,..Default::default()},|same,grant|{let step=self.advance_private_child_group_granted(mounted,same,grant)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;
        let ready=self.private_child_groups.get(mounted.operation.operation.0).is_some_and(|owner|owner.phase>=9);
        Ok(if ready{RetainedCloneStep::Complete(step.progress().unwrap_or_default())}else{RetainedCloneStep::Progress(step.progress().unwrap_or_default())})
    }

    fn advance_private_child_group_granted(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        let step=(||->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
        let operation=mounted.operation.operation.0;
        let kind=self.declared_dispatch_kind(&mounted.verb).unwrap_or(ActionKind::Mutation);
        let owner=self.private_child_groups.get_mut(operation).ok_or_else(||plugin_sdk_fault("mounted publication lost its original admitted group frame"))?;
        if owner.closing{return Err(plugin_sdk_fault("mounted private publication requires bounded retirement after cancellation").into());}
        if owner.phase>=9{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let structural=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
        let error=crate::PluginLifecycleFailure::from;
        if owner.phase==0{
            if owner.identity.ready(){*owner.group_id=owner.identity.take_ready(grant);owner.phase=1;return Ok(structural());}
            let source=MountedChildGroupIdentitySource{parent:&self.store.envelope().id,actor:&mounted.meta.actor,revision:mounted.canonical_revision,instance:mounted.meta.instance_id,operation};
            return owner.identity.advance(source,grant).map_err(error);
        }
        if owner.phase==1{
            if owner.parent_issuer.ready(){owner.parent_handle=owner.parent_issuer.take(grant);owner.parent_issuer_closed=true;owner.phase=2;return Ok(structural());}
            return owner.parent_issuer.advance(&self.store.envelope().id,grant).map_err(error);
        }
        if owner.phase==2{
            let _birth=A::owned_mutation_batch_birth_bytes().ok_or_else(||plugin_sdk_fault("owned child publication requires the parent app's exact retained mutation factory"))?;
            
            let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut()else{return Err(plugin_sdk_fault("mounted group lost its original typed emission").into());};
            owner.parent_touched=!emit.artifact_mutations.is_empty();
            let mut original=Some(std::mem::take(&mut emit.artifact_mutations));
            match A::admit_owned_mutation_batch(&mut original,grant){
                Ok(Some((batch,progress)))=>{assert!(original.is_none(),"admitted parent batch owns its original vector");*owner.parent_batch=Some(batch);owner.phase=3;return Ok(RetainedCloneStep::Progress(progress));},
                Ok(None)=>{emit.artifact_mutations=original.take().expect("refused parent birth retains its exact original vector");return Ok(RetainedCloneStep::Progress(Default::default()));},
                Err(reason)=>{emit.artifact_mutations=original.take().expect("rejected parent birth retains its exact original vector");return Err(error(reason));}
            }
        }
        if owner.phase==3{
            let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut()else{return Err(plugin_sdk_fault("mounted group lost its original typed emission").into());};
            if emit.owned_child_emits.is_empty()||emit.owned_child_emits.len()>PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN{return Err(plugin_sdk_fault("mounted group exceeds its original declared child admission").into());}
            let source=PrivateChildGroupSource{parent:owner.parent_batch.take().unwrap(),parent_touched:owner.parent_touched,children:std::mem::take(&mut emit.owned_child_emits),transaction:emit.transaction.take(),group_id:owner.group_id.take().unwrap()};
            match PrivateChildPublicationGroup::try_new(source){Ok(group)=>{*owner.publication=Some(group);owner.phase=4;return Ok(structural());},Err(source)=>{*owner.parent_batch=Some(source.parent);*owner.group_id=Some(source.group_id);emit.owned_child_emits=source.children;emit.transaction=source.transaction;return Err(plugin_sdk_fault("mounted private group refused its exact original child source").into());}}
        }
        let group=owner.publication.as_mut().expect("admitted private publication retains its original group");
        if !group.inputs_ready(){let envelope=self.store.envelope();let dialect=envelope.dialect.as_ref().ok_or_else(||plugin_sdk_fault("private group requires its parent's original declared dialect"))?;let _capacity=group.next_input_capacity_byte_demand(&self.children,&envelope.id,dialect,&mounted.meta.actor).map_err(error)?;return group.advance_inputs(&self.children,&envelope.id,dialect,&mounted.meta.actor,mounted.operation.operation,mounted.operation.generation,owner.expires_at_us,cx.now_us().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private child source requires the incoming caller clock"))?,grant).map_err(error);}
        if !group.openings_ready(){
            let demand=group.next_open_retirement_demands(grant.maximum_copy_bytes)?;let birth=group.next_open_birth_demand()?;if demand.depth>grant.maximum_depth||birth.depth>grant.maximum_depth{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"private member opening exceeds its original caller depth").into())}return group.advance_openings(cx,grant);
        }
        if owner.receipts.is_none(){let birth=MountedGroupMemberReceipts::frame_birth_bytes();if grant.maximum_capacity_bytes<birth{return Ok(RetainedCloneStep::Progress(Default::default()))}let receipts=MountedGroupMemberReceipts::new(group,&mounted.meta.actor,owner.parent_handle.unwrap()).map_err(error)?;*owner.receipts=Some(Box::new(receipts));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:grant.maximum_items,retained_capacity_bytes:birth,..Default::default()}));}
        let receipts=owner.receipts.as_mut().unwrap();
        if let Some(index)=receipts.receipt_index(){if group.prepared_publication(index).is_some(){let _capacity=receipts.next_capacity_byte_demand(group,&mounted.meta.actor).map_err(error)?;let _release=receipts.next_release_byte_demand(group,&mounted.meta.actor).map_err(error)?;return receipts.advance_receipt(group,&self.children,&mounted.meta.actor,grant).map_err(error);}}
        if !group.publications_ready(){let _capacity=group.next_publication_capacity_byte_demand(&self.store,&self.children)?.max(4096);return group.advance_publications(&mut self.store,&mut self.children,mounted.operation.operation,grant).map_err(Into::into);}
        if !group.projection_ready(){return group.advance_projection(&self.store,grant).map_err(Into::into);}
        if !group.entries_ready(){return group.advance_entries(&mut self.children,&self.child_content_root,grant).map_err(Into::into);}
        let step=group.close_displaced_view_step(&self.child_content_root,grant)?;semio_framework_value::retained_clone::admit_retained_clone_progress(grant,step.progress(),"mounted private displaced view")?;if step.progress()!=Default::default()||!matches!(step,RetainedCloneStep::Complete(_)){return Ok(RetainedCloneStep::Progress(step.progress()));}
        let graph=self.composition.ownership_graph_mut();if !group.ready(graph){let demand=group.next_graph_demands(graph,&self.children)?;if demand.copy_bytes>4096{return Err(plugin_sdk_fault("mounted graph source words exceed the admitted per-turn work frontier").into());}return group.advance_graph(graph,&self.children,grant).map_err(Into::into);}
        if !receipts.final_ready(){let _capacity=receipts.next_final_capacity_byte_demand(group.group_id()).map_err(error)?;let _release=receipts.next_final_release_byte_demand();return receipts.advance_final(group.group_id(),grant).map_err(error);}
        let label=self.registry.get(&mounted.verb).map(|definition|&definition.label).or_else(||self.registry.get_command(&mounted.verb).map(|definition|&definition.label));
        let source=MountedCommandEntrySource{action_id:&mounted.verb,label,captured_millis:owner.captured_millis,kind,count:1,parent_touched:group.parent_touched(),child_edit_count:group.child_count()};
        if owner.command.is_none(){owner.command=Some(MountedCommandEntry::new(source).map_err(|_|plugin_sdk_fault("mounted command entry refused its original timestamp authority"))?);return Ok(structural());}
        let command=owner.command.as_mut().unwrap();if !command.ready(){let _capacity=command.next_capacity_byte_demand(source).map_err(|_|plugin_sdk_fault("mounted command entry source changed before prestage"))?;return command.advance(source,grant).map_err(|_|crate::PluginLifecycleFailure::from(plugin_sdk_fault("mounted command entry lost its original borrowed label source")));}
        if owner.append.is_none(){owner.append=Some(self.command_log.prepare_append().map_err(|_|plugin_sdk_fault("mounted command history refused its next original page"))?);return Ok(structural());}
        let append=owner.append.as_mut().unwrap();match self.command_log.validate_append(append){Ok(())=>{},Err(mounted_child_group_receipt_group::CommandAppendRefusal::Unfunded)=>{return Ok(append.admit_step(grant));},Err(mounted_child_group_receipt_group::CommandAppendRefusal::Stale)=>{let step=append.close_step(grant);owner.append=None;return Ok(step);},Err(_)=>return Err(plugin_sdk_fault("mounted command append lost its exact original history authority").into())}
        owner.phase=9;Ok(RetainedCloneStep::Complete(Default::default()))
        })()?;
        let ready=self.private_child_groups.get(mounted.operation.operation.0).is_some_and(|owner|owner.phase>=9);
        Ok(if ready{step}else{RetainedCloneStep::Progress(step.progress())})
    }
    pub(crate) fn admit_private_child_group_frame(&mut self,operation:u64,captured_millis:i64,expires_at_us:u64,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.private_child_groups.get(operation).is_some(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if !self.private_child_groups.can_insert(operation){return Err(plugin_sdk_fault("mounted child group lacks its exact original fixed registry slot"));}
        let bytes=MountedPrivateChildGroup::<M>::frame_birth_bytes();
        if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.private_child_groups.insert_admitted(operation,Box::new(MountedPrivateChildGroup::new(captured_millis,expires_at_us)));
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}))
    }
    pub(crate) async fn publish_mounted_owned_child_operation_unit(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let operation=mounted.owned_child_group.ok_or_else(||plugin_sdk_fault("mounted private publication lost its exact operation identity"))?;
        let Some(owner)=self.private_child_groups.get(operation)else{mounted.owned_child_group=None;return Ok(());};
        let lease=mounted.cancellation_lease.as_ref().ok_or_else(||plugin_sdk_fault("mounted private publication requires its original cancellation lease"))?;
        let cancelled=lease.token.is_cancelled_now()||lease.publication_claim.is_cancelled()||lease.handle.publication_scope.is_cancelled()||lease.document_claim.as_ref().is_some_and(|claim|claim.is_cancelled());
        if cancelled||(owner.closing&&!mounted.owned_child_result_pending)||(mounted.owned_child_committed&&!mounted.owned_child_result_pending){
            let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,self.mounted_policy.close);
            let demand=self.private_child_group_operation_close_demands(operation,grant.maximum_copy_bytes)?;
            if demand.copy_bytes>grant.maximum_copy_bytes||demand.depth>grant.maximum_depth{return Err(plugin_sdk_fault("mounted retirement exceeds its original copy or depth admission").into());}
            self.close_private_child_group_operation_step(operation,cx,grant)?;
            if self.private_child_groups.get(operation).is_none(){mounted.owned_child_group=None;if !mounted.owned_child_committed{mounted.reject_cancelled_publication()?;}}
            return Ok(());
        }
        if mounted.owned_child_committed{return Ok(());}
        let Some(_permit)=lease.try_claim_publication()else{return Ok(());};
        let captured=mounted.captured_child_content.as_ref().ok_or_else(||plugin_sdk_fault("mounted child group lost its captured immutable child frontier"))?;
        let fresh=typed_operation_document_is_fresh(&mounted.operation,mounted.canonical_revision,self.store.content_revision_now(),self.store.generation_now())&&mounted.captured_child_content_generation==self.child_content_generation&&captured.identity_digest()==self.child_content_root.identity_digest();
        if !fresh{let fault=plugin_sdk_fault("mounted private group rejected its stale original parent or child frontier");self.private_child_groups.get_mut(operation).unwrap().closing=true;mounted.terminal_fault=Some(ArtifactBoundedToolFault::from_fault(&fault));lease.cancel();return Err(fault.into());}
        let step=match self.advance_private_child_group(mounted,cx,self.mounted_policy.maintenance){Ok(step)=>step,Err(failure)=>{self.private_child_groups.get_mut(operation).unwrap().closing=true;mounted.cancellation_lease.as_ref().unwrap().cancel();return Err(failure);}};
        if step.progress()!=Default::default(){mounted.publication_progress=mounted.publication_progress.saturating_add(1);}
        if !matches!(step,RetainedCloneStep::Complete(_)){return Ok(());}
        if cx.should_yield()||!self.typed_composed_outbox.can_push(){return Ok(());}
        let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,self.mounted_policy.maintenance);if grant.maximum_items==0||grant.maximum_depth==0{return Ok(())}
        let generation=self.child_content_generation.checked_add(1).ok_or_else(||plugin_sdk_fault("private child frontier generation exhausted"))?;
        if !self.child_content_root.is_empty()&&!self.child_content_retirements.can_insert(generation){return Ok(());}
        let sequence=self.next_command_seq.checked_add(1).ok_or_else(||plugin_sdk_fault("private command history sequence exhausted"))?;
        let log_generation=self.log_generation.checked_add(1).ok_or_else(||plugin_sdk_fault("private command history generation exhausted"))?;
        let mut page=TypedOperationResultPage::try_new(mounted.next_token(),TypedOperationResultLane::Child,b"")?;
        let owner=self.private_child_groups.get_mut(operation).unwrap();
        let append=owner.append.as_mut().unwrap();if self.command_log.validate_append(append).is_err(){owner.phase=8;return Ok(());}
        let group=owner.publication.as_mut().unwrap();let graph=self.composition.ownership_graph_mut();if !group.ready(graph){return Ok(());}
        if owner.output.is_none(){*owner.output=owner.receipts.as_mut().unwrap().take_final(grant);}
        let output=owner.output.as_mut().ok_or_else(||plugin_sdk_fault("preborn private receipt refused its exact original handoff"))?;
        if owner.record.is_none(){*owner.record=owner.command.as_mut().unwrap().take(&mut output.command_edit_id,&mut output.command_child_edit_ids,sequence,grant);}
        owner.record.as_mut().ok_or_else(||plugin_sdk_fault("preborn private command row refused its original receipt cardinality"))?.seq=sequence;
        let Some(next)=group.commit(&mut self.store,&mut self.children,graph,store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth})?else{return Ok(());};
        let previous=std::mem::replace(&mut*self.child_content_root,next);if !previous.is_empty(){self.child_content_retirements.insert_admitted(generation,ChildContentRetirement::new(previous,false));}
        self.child_content_generation=generation;
        self.command_log.commit_append(append,&mut owner.record).expect("common private decision retains the validated original command prefix");self.next_command_seq=sequence;self.log_generation=log_generation;*owner.displaced_shell_redo=std::mem::take(&mut self.shell_redo);
        let output=owner.output.take().unwrap();self.typed_composed_outbox.push(ComposedGestureResult{operation,mutations:output.mutations,inverse_group:output.inverse_group}).unwrap_or_else(|_|unreachable!("validated fixed private receiver retains its exact original free slot"));
        owner.closing=true;mounted.owned_child_committed=true;mounted.owned_child_result_pending=true;mounted.command_logged=true;mounted.published_artifact=true;mounted.artifact_generation=self.store.generation_now();mounted.canonical_revision=self.store.content_revision_now();mounted.operation.base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().unwrap()));mounted.operation.generation=semio_framework_job::Generation(mounted.artifact_generation);page.token=mounted.next_token();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);mounted.queue_page(page).map_err(Into::into)
    }
    pub(crate) fn private_child_group_close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let Some((_,operation))=self.private_child_groups.next_id_from(0) else {
            let release_bytes=self.private_child_groups.empty_backing_byte_demand().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private group registry still owns its original slot"))?;
            return Ok(RetirementDemand { release_bytes, depth:usize::from(release_bytes!=0), ..Default::default() });
        };
        self.private_child_group_operation_close_demands(operation,body)
    }
    pub(crate) fn next_closing_private_child_group(&self)->Option<u64>{(0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index|self.private_child_groups.entry(index).and_then(|(operation,owner)|(owner.closing&&self.tool_operations.get(*operation).is_none_or(|mounted|!mounted.owned_child_result_pending)).then_some(*operation)))}
    pub(crate) fn private_child_group_operation_close_demands(&self, operation:u64, body:usize) -> Result<RetirementDemand, ValueError> {
        let Some(owner)=self.private_child_groups.get(operation)else{return Ok(Default::default());};
        if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:MountedPrivateChildGroup::<M>::frame_birth_bytes(),depth:1,..Default::default()});}
        mounted_child_group_receipt::mounted_nested_retirement_demand(owner.retirement_demands(body)?,1)
    }
    pub(crate) fn private_child_groups_terminal_is_empty(&self)->bool{self.private_child_groups.is_empty()&&self.private_child_groups.empty_backing_byte_demand()==Some(0)}
    pub(crate) fn close_private_child_group_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,crate::PluginLifecycleFailure>{
        let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,grant);
        let demand=self.private_child_group_close_demands(grant.maximum_copy_bytes)?;
        if self.private_child_groups_terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});}
        if !plugin_grant_funds(demand,grant){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        let Some((_,operation))=self.private_child_groups.next_id_from(0)else{
            if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
            let step=self.private_child_groups.close_empty_backing_step(grant)?;cx.consume_retained(step.progress())?;cx.consume_fuel(1);
            return match step{
                RetainedCloneStep::Progress(progress)=>Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress}),
                RetainedCloneStep::Complete(progress)=>Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress}),
            };
        };
        self.close_private_child_group_operation_step(operation,cx,grant)
    }
    pub(crate) fn close_private_child_group_operation_step(&mut self,operation:u64,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,crate::PluginLifecycleFailure>{
        let demand=self.private_child_group_operation_close_demands(operation,loan.maximum_copy_bytes)?;
        let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,grant|{
            let step=self.close_private_child_group_operation_granted(operation,same,grant)?;
            let terminal=self.private_child_groups.get(operation).is_none();
            Ok((match step{semio_framework_job::InteractiveJobCloseStep::Pending{progress}=>PluginLifecycleStep::Progress(progress),semio_framework_job::InteractiveJobCloseStep::Complete{progress}=>if terminal{PluginLifecycleStep::Complete(progress)}else{PluginLifecycleStep::Progress(progress)},semio_framework_job::InteractiveJobCloseStep::Blocked=>PluginLifecycleStep::Blocked{reason:"private group retains its original source"}},terminal))
        })?;
        Ok(match step{PluginLifecycleStep::Progress(progress)=>semio_framework_job::InteractiveJobCloseStep::Pending{progress},PluginLifecycleStep::Complete(progress)=>semio_framework_job::InteractiveJobCloseStep::Complete{progress},PluginLifecycleStep::AwaitingInput{..}|PluginLifecycleStep::Blocked{..}=>semio_framework_job::InteractiveJobCloseStep::Blocked})
    }
    fn close_private_child_group_operation_granted(&mut self,operation:u64,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,crate::PluginLifecycleFailure>{
        let Some(original)=self.private_child_groups.get(operation)else{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});};
        let terminal=original.terminal_is_empty();
        let demand=self.private_child_group_operation_close_demands(operation,grant.maximum_copy_bytes)?;
        if !plugin_grant_funds(demand,grant){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        if terminal{drop(self.private_child_groups.remove(operation));return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()}});}
        let owner=self.private_child_groups.get_mut(operation).ok_or_else(||plugin_sdk_fault("private group changed during its exact bounded close"))?;
        let child_grant=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        let step=owner.close_step(&mut self.store,&mut self.children,self.composition.ownership_graph_mut(),&self.child_content_root,cx,child_grant)?;
        Ok(step.admit(child_grant,owner.terminal_is_empty())?)
    }

}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    pub(crate) fn command_prune_requested(&self) -> bool { self.command_prune_generation != self.command_prune_processed }
    /// 📏️ Quotes the next prune turn: a cancellation or begin hand-off, a funded hidden owner release, or one bounded comparison.
    pub(crate) fn command_prune_demands(&self, closing: bool, _body: usize) -> Result<RetirementDemand, ValueError> {
        let item = RetirementDemand { depth: 1, ..Default::default() };
        if closing && (self.pending_command_prune.is_some() || self.command_prune_requested()) { return Ok(item); }
        if self.pending_command_prune.is_none() && !self.command_log.pruned_terminal_is_empty() {
            return Ok(RetirementDemand { release_bytes: self.command_log.next_pruned_close_byte_demand()?, depth: 1, ..Default::default() });
        }
        if self.pending_command_prune.is_none() { return Ok(if self.command_prune_requested() { item } else { RetirementDemand::default() }); }
        Ok(RetirementDemand { copy_bytes: 64, depth: 1, ..Default::default() })
    }
    /// 🧹️ Advances one original metadata comparison, checked visibility decision or funded hidden owner.
    pub(crate) fn advance_command_prune_step(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<PluginLifecycleStep, Fault> {
        let pending = PluginLifecycleStep::Progress(RetainedCloneProgress::default());
        if grant.maximum_items == 0 { return Ok(pending); }
        let demand = self.command_prune_demands(closing, grant.maximum_copy_bytes).map_err(plugin_retirement_fault)?;
        if !plugin_turn_admitted(demand, grant)? { return Ok(pending); }
        if closing {
            if let Some(mut owner) = self.pending_command_prune.take() {
                self.command_log.cancel_prune(&mut owner.cursor, 1).map_err(|_| plugin_sdk_fault("mounted command visibility lost its original cancellation lease"))?;
                self.command_prune_processed = self.command_prune_generation;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            if self.command_prune_requested() {
                self.command_prune_processed = self.command_prune_generation;
                return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
        }
        if self.pending_command_prune.is_none() && !self.command_log.pruned_terminal_is_empty() {
            if let Some(record) = self.command_log.next_pruned_record() {
                self.shell_undone.remove(&record.seq);
                self.history_dirty_sequences.remove(&record.seq);
            }
            let step = self.command_log.close_pruned_step(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: 0, maximum_copy_bytes: 0, maximum_release_bytes: grant.maximum_release_bytes, maximum_depth: grant.maximum_depth }).map_err(|error| plugin_sdk_fault(error.into_message()))?;
            return Ok(PluginLifecycleStep::Progress(step.progress()));
        }
        if self.pending_command_prune.is_none() {
            if !self.command_prune_requested() { return Ok(PluginLifecycleStep::Complete(RetainedCloneProgress::default())); }
            self.refresh_supersede_ledger();
            let cursor = self.command_log.begin_prune().map_err(|_| plugin_sdk_fault("mounted command visibility requires its original idle history owner"))?;
            self.pending_command_prune = Some(MountedCommandPrune { cursor, requested: self.command_prune_generation, revision: self.store.content_revision_now(), generation: self.store.generation_now(), edits: self.store.envelope().vcs.edits.len(), transitions: self.supersedes.records.len(), field: 0, index: 0, offset: 0, keep: true });
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let mut owner = self.pending_command_prune.take().unwrap();
        let stale = owner.requested != self.command_prune_generation || owner.revision != self.store.content_revision_now() || owner.generation != self.store.generation_now() || owner.edits != self.store.envelope().vcs.edits.len() || owner.transitions != self.supersedes.records.len();
        let record = if stale { None } else { self.command_log.prune_current(&owner.cursor).ok() };
        let Some(record) = record else {
            self.command_log.cancel_prune(&mut owner.cursor, 1).map_err(|_| plugin_sdk_fault("stale command visibility requires its exact retained cancellation"))?;
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        };
        let Some(record) = record else {
            let removed = self.command_log.commit_prune(&mut owner.cursor).map_err(|_| plugin_sdk_fault("mounted command visibility changed before its constant decision"))?;
            self.command_prune_processed = owner.requested;
            if removed != 0 { self.history_backfill = None; self.log_generation += 1; }
            return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        };
        match owner.field {
            0 => match record.edit_id.as_deref() {
                None => { owner.field = 1; owner.index = 0; owner.offset = 0; },
                Some(id) => match self.store.envelope().vcs.edits.get(owner.index) {
                    None => { owner.keep = false; owner.field = 2; },
                    Some(edit) => match MountedCommandPrune::equal_prefix(id, &edit.id, &mut owner.offset) {
                        Some(true) => { owner.field = 1; owner.index = 0; owner.offset = 0; },
                        Some(false) => { owner.index += 1; },
                        None => {},
                    },
                },
            },
            1 => match record.transition_id.as_deref() {
                None => { owner.field = 2; },
                Some(id) => match self.supersedes.records.get(owner.index) {
                    None => { owner.keep = false; owner.field = 2; },
                    Some(transition) => match MountedCommandPrune::equal_prefix(id, &transition.transition_id, &mut owner.offset) {
                        Some(true) => { owner.field = 2; },
                        Some(false) => { owner.index += 1; },
                        None => {},
                    },
                },
            },
            _ => {
                self.command_log.advance_prune(&mut owner.cursor, owner.keep, RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: 0, maximum_copy_bytes: 64, maximum_release_bytes: 0, maximum_depth: 64 }).map_err(|_| plugin_sdk_fault("prepared command visibility changed before its original row link"))?;
                owner.field = 0; owner.index = 0; owner.offset = 0; owner.keep = true;
            },
        }
        self.pending_command_prune = Some(owner);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }
}
