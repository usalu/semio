//! 🏆️ Staged `child-group` members of the `instance-typed-ops` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🪆️ Private child group publication.
use super::*;
use crate::child_content::MountedChildGroupIdentitySource;
use crate::child_content::*;
use crate::child_content::mounted_child_group_receipt_group::MountedGroupMemberReceipts;
use crate::child_content::mounted_child_group_receipt_group::MountedCommandEntry;
use crate::child_content::mounted_child_group_receipt_group::MountedCommandEntrySource;
use super::docs_instance_root::typed_operation_document_is_fresh;

impl<'ctx, A:ArtifactApp,M:SpaceMember+MemberFactory+'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    fn advance_private_child_group(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
        if self.child_composition.private_child_groups.get(mounted.operation.operation.0).is_some_and(|owner|owner.phase>=9){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,RetirementDemand{depth:1,..Default::default()},|same,grant|{let step=self.advance_private_child_group_granted(mounted,same,grant)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;
        let ready=self.child_composition.private_child_groups.get(mounted.operation.operation.0).is_some_and(|owner|owner.phase>=9);
        Ok(if ready{RetainedCloneStep::Complete(step.progress().unwrap_or_default())}else{RetainedCloneStep::Progress(step.progress().unwrap_or_default())})
    }

    fn advance_private_child_group_granted(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        let step=(||->Result<RetainedCloneStep,crate::PluginLifecycleFailure>{
        let operation=mounted.operation.operation.0;
        let kind=self.tools.declared_dispatch_kind(&*self.host_events, &mounted.verb).unwrap_or(ActionKind::Mutation);
        let owner=self.child_composition.private_child_groups.get_mut(operation).ok_or_else(||plugin_sdk_fault("mounted publication lost its original admitted group frame"))?;
        if owner.closing{return Err(plugin_sdk_fault("mounted private publication requires bounded retirement after cancellation").into());}
        if owner.phase>=9{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let structural=||RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
        let error=crate::PluginLifecycleFailure::from;
        if owner.phase==0{
            if owner.identity.ready(){*owner.group_id=owner.identity.take_ready(grant);owner.phase=1;return Ok(structural());}
            let source=MountedChildGroupIdentitySource{parent:&self.stores.store.envelope().id,actor:&mounted.meta.actor,revision:mounted.canonical_revision,instance:mounted.meta.instance_id,operation};
            return owner.identity.advance(source,grant).map_err(error);
        }
        if owner.phase==1{
            if owner.parent_issuer.ready(){owner.parent_handle=owner.parent_issuer.take(grant);owner.parent_issuer_closed=true;owner.phase=2;return Ok(structural());}
            return owner.parent_issuer.advance(&self.stores.store.envelope().id,grant).map_err(error);
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
        if !group.inputs_ready(){let envelope=self.stores.store.envelope();let dialect=envelope.dialect.as_ref().ok_or_else(||plugin_sdk_fault("private group requires its parent's original declared dialect"))?;let _capacity=group.next_input_capacity_byte_demand(&self.child_composition.children,&envelope.id,dialect,&mounted.meta.actor).map_err(error)?;return group.advance_inputs(&self.child_composition.children,&envelope.id,dialect,&mounted.meta.actor,mounted.operation.operation,mounted.operation.generation,owner.expires_at_us,cx.now_us().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private child source requires the incoming caller clock"))?,grant).map_err(error);}
        if !group.openings_ready(){
            let demand=group.next_open_retirement_demands(grant.maximum_copy_bytes)?;let birth=group.next_open_birth_demand()?;if demand.depth>grant.maximum_depth||birth.depth>grant.maximum_depth{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"private member opening exceeds its original caller depth").into())}return group.advance_openings(cx,grant);
        }
        if owner.receipts.is_none(){let birth=MountedGroupMemberReceipts::frame_birth_bytes();if grant.maximum_capacity_bytes<birth{return Ok(RetainedCloneStep::Progress(Default::default()))}let receipts=MountedGroupMemberReceipts::new(group,&mounted.meta.actor,owner.parent_handle.unwrap()).map_err(error)?;*owner.receipts=Some(Box::new(receipts));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:grant.maximum_items,retained_capacity_bytes:birth,..Default::default()}));}
        let receipts=owner.receipts.as_mut().unwrap();
        if let Some(index)=receipts.receipt_index(){if group.prepared_publication(index).is_some(){let _capacity=receipts.next_capacity_byte_demand(group,&mounted.meta.actor).map_err(error)?;let _release=receipts.next_release_byte_demand(group,&mounted.meta.actor).map_err(error)?;return receipts.advance_receipt(group,&self.child_composition.children,&mounted.meta.actor,grant).map_err(error);}}
        if !group.publications_ready(){let _capacity=group.next_publication_capacity_byte_demand(&self.stores.store,&self.child_composition.children)?.max(4096);return group.advance_publications(&mut self.stores.store,&mut self.child_composition.children,mounted.operation.operation,grant).map_err(Into::into);}
        if !group.projection_ready(){return group.advance_projection(&self.stores.store,grant).map_err(Into::into);}
        if !group.entries_ready(){return group.advance_entries(&mut self.child_composition.children,&self.child_composition.child_content_root,grant).map_err(Into::into);}
        let step=group.close_displaced_view_step(&self.child_composition.child_content_root,grant)?;semio_framework_value::retained_clone::admit_retained_clone_progress(grant,step.progress(),"mounted private displaced view")?;if step.progress()!=Default::default()||!matches!(step,RetainedCloneStep::Complete(_)){return Ok(RetainedCloneStep::Progress(step.progress()));}
        let graph=self.child_composition.composition.ownership_graph_mut();if !group.ready(graph){let demand=group.next_graph_demands(graph,&self.child_composition.children)?;if demand.copy_bytes>4096{return Err(plugin_sdk_fault("mounted graph source words exceed the admitted per-turn work frontier").into());}return group.advance_graph(graph,&self.child_composition.children,grant).map_err(Into::into);}
        if !receipts.final_ready(){let _capacity=receipts.next_final_capacity_byte_demand(group.group_id()).map_err(error)?;let _release=receipts.next_final_release_byte_demand();return receipts.advance_final(group.group_id(),grant).map_err(error);}
        let label=self.tools.registry.get(&mounted.verb).map(|definition|&definition.label).or_else(||self.tools.registry.get_command(&mounted.verb).map(|definition|&definition.label));
        let source=MountedCommandEntrySource{action_id:&mounted.verb,label,captured_millis:owner.captured_millis,kind,count:1,parent_touched:group.parent_touched(),child_edit_count:group.child_count()};
        if owner.command.is_none(){owner.command=Some(MountedCommandEntry::new(source).map_err(|_|plugin_sdk_fault("mounted command entry refused its original timestamp authority"))?);return Ok(structural());}
        let command=owner.command.as_mut().unwrap();if !command.ready(){let _capacity=command.next_capacity_byte_demand(source).map_err(|_|plugin_sdk_fault("mounted command entry source changed before prestage"))?;return command.advance(source,grant).map_err(|_|crate::PluginLifecycleFailure::from(plugin_sdk_fault("mounted command entry lost its original borrowed label source")));}
        if owner.append.is_none(){owner.append=Some(self.history.command_log.prepare_append().map_err(|_|plugin_sdk_fault("mounted command history refused its next original page"))?);return Ok(structural());}
        let append=owner.append.as_mut().unwrap();match self.history.command_log.validate_append(append){Ok(())=>{},Err(mounted_child_group_receipt_group::CommandAppendRefusal::Unfunded)=>{return Ok(append.admit_step(grant));},Err(mounted_child_group_receipt_group::CommandAppendRefusal::Stale)=>{let step=append.close_step(grant);owner.append=None;return Ok(step);},Err(_)=>return Err(plugin_sdk_fault("mounted command append lost its exact original history authority").into())}
        owner.phase=9;Ok(RetainedCloneStep::Complete(Default::default()))
        })()?;
        let ready=self.child_composition.private_child_groups.get(mounted.operation.operation.0).is_some_and(|owner|owner.phase>=9);
        Ok(if ready{step}else{RetainedCloneStep::Progress(step.progress())})
    }

    pub(crate) async fn publish_mounted_owned_child_operation_unit(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let operation=mounted.owned_child_group.ok_or_else(||plugin_sdk_fault("mounted private publication lost its exact operation identity"))?;
        let Some(owner)=self.child_composition.private_child_groups.get(operation)else{mounted.owned_child_group=None;return Ok(());};
        let lease=mounted.cancellation_lease.as_ref().ok_or_else(||plugin_sdk_fault("mounted private publication requires its original cancellation lease"))?;
        let cancelled=lease.token.is_cancelled_now()||lease.publication_claim.is_cancelled()||lease.handle.publication_scope.is_cancelled()||lease.document_claim.as_ref().is_some_and(|claim|claim.is_cancelled());
        if cancelled||(owner.closing&&!mounted.owned_child_result_pending)||(mounted.owned_child_committed&&!mounted.owned_child_result_pending){
            let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,self.mounted_policy.close);
            let demand=self.child_composition.private_child_group_operation_close_demands(operation, grant.maximum_copy_bytes)?;
            if demand.copy_bytes>grant.maximum_copy_bytes||demand.depth>grant.maximum_depth{return Err(plugin_sdk_fault("mounted retirement exceeds its original copy or depth admission").into());}
            self.child_composition.close_private_child_group_operation_step(&mut *self.stores, operation, cx, grant)?;
            if self.child_composition.private_child_groups.get(operation).is_none(){mounted.owned_child_group=None;if !mounted.owned_child_committed{mounted.reject_cancelled_publication()?;}}
            return Ok(());
        }
        if mounted.owned_child_committed{return Ok(());}
        let Some(_permit)=lease.try_claim_publication()else{return Ok(());};
        let captured=mounted.captured_child_content.as_ref().ok_or_else(||plugin_sdk_fault("mounted child group lost its captured immutable child frontier"))?;
        let fresh=typed_operation_document_is_fresh(&mounted.operation,mounted.canonical_revision,self.stores.store.content_revision_now(),self.stores.store.generation_now())&&mounted.captured_child_content_generation==self.child_composition.child_content_generation&&captured.identity_digest()==self.child_composition.child_content_root.identity_digest();
        if !fresh{let fault=plugin_sdk_fault("mounted private group rejected its stale original parent or child frontier");self.child_composition.private_child_groups.get_mut(operation).unwrap().closing=true;mounted.terminal_fault=Some(ArtifactBoundedToolFault::from_fault(&fault));lease.cancel();return Err(fault.into());}
        let step=match self.advance_private_child_group(mounted,cx,self.mounted_policy.maintenance){Ok(step)=>step,Err(failure)=>{self.child_composition.private_child_groups.get_mut(operation).unwrap().closing=true;mounted.cancellation_lease.as_ref().unwrap().cancel();return Err(failure);}};
        if step.progress()!=Default::default(){mounted.publication_progress=mounted.publication_progress.saturating_add(1);}
        if !matches!(step,RetainedCloneStep::Complete(_)){return Ok(());}
        if cx.should_yield()||!self.tools.typed_composed_outbox.can_push(){return Ok(());}
        let grant=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_grant(cx,self.mounted_policy.maintenance);if grant.maximum_items==0||grant.maximum_depth==0{return Ok(())}
        let generation=self.child_composition.child_content_generation.checked_add(1).ok_or_else(||plugin_sdk_fault("private child frontier generation exhausted"))?;
        if !self.child_composition.child_content_root.is_empty()&&!self.child_composition.child_content_retirements.can_insert(generation){return Ok(());}
        let sequence=self.history.next_command_seq.checked_add(1).ok_or_else(||plugin_sdk_fault("private command history sequence exhausted"))?;
        let log_generation=self.history.log_generation.checked_add(1).ok_or_else(||plugin_sdk_fault("private command history generation exhausted"))?;
        let mut page=TypedOperationResultPage::try_new(mounted.next_token(),TypedOperationResultLane::Child,b"")?;
        let owner=self.child_composition.private_child_groups.get_mut(operation).unwrap();
        let append=owner.append.as_mut().unwrap();if self.history.command_log.validate_append(append).is_err(){owner.phase=8;return Ok(());}
        let group=owner.publication.as_mut().unwrap();let graph=self.child_composition.composition.ownership_graph_mut();if !group.ready(graph){return Ok(());}
        if owner.output.is_none(){*owner.output=owner.receipts.as_mut().unwrap().take_final(grant);}
        let output=owner.output.as_mut().ok_or_else(||plugin_sdk_fault("preborn private receipt refused its exact original handoff"))?;
        if owner.record.is_none(){*owner.record=owner.command.as_mut().unwrap().take(&mut output.command_edit_id,&mut output.command_child_edit_ids,sequence,grant);}
        owner.record.as_mut().ok_or_else(||plugin_sdk_fault("preborn private command row refused its original receipt cardinality"))?.seq=sequence;
        let Some(next)=group.commit(&mut self.stores.store,&mut self.child_composition.children,graph,store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth})?else{return Ok(());};
        let previous=std::mem::replace(&mut*self.child_composition.child_content_root,next);if !previous.is_empty(){self.child_composition.child_content_retirements.insert_admitted(generation,ChildContentRetirement::new(previous,false));}
        self.child_composition.child_content_generation=generation;
        self.history.command_log.commit_append(append,&mut owner.record).expect("common private decision retains the validated original command prefix");self.history.next_command_seq=sequence;self.history.log_generation=log_generation;*owner.displaced_shell_redo=std::mem::take(&mut self.history.shell_redo);
        let output=owner.output.take().unwrap();self.tools.typed_composed_outbox.push(ComposedGestureResult{operation,mutations:output.mutations,inverse_group:output.inverse_group}).unwrap_or_else(|_|unreachable!("validated fixed private receiver retains its exact original free slot"));
        owner.closing=true;mounted.owned_child_committed=true;mounted.owned_child_result_pending=true;mounted.command_logged=true;mounted.published_artifact=true;mounted.artifact_generation=self.stores.store.generation_now();mounted.canonical_revision=self.stores.store.content_revision_now();mounted.operation.base_revision=semio_framework_job::RevisionId(u64::from_be_bytes(mounted.canonical_revision[..8].try_into().unwrap()));mounted.operation.generation=semio_framework_job::Generation(mounted.artifact_generation);page.token=mounted.next_token();cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);mounted.queue_page(page).map_err(Into::into)
    }
}
