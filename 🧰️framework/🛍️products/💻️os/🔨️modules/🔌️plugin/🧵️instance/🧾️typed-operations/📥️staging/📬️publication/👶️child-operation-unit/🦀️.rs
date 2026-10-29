//! 👶️ Converts exact encoded child sources through installed native decoders before the common private group ladder.
use super::*;
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_plugin_ownership::bounded_ownership::{PluginLifecycleFailure,plugin_grant_funds};
use semio_framework_value::{RetirementDemand,ValueRefusalKind};
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_decl_emission::{ChildEmit,child_emit_preparation::OwnedChildEmit};
use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
fn original_failure(detail:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,detail)}
fn original_header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn post_original<T>(cx:&mut StepContext<'_>,result:Result<(T,RetainedCloneProgress),ValueError>)->Result<T,OriginalTurnFailure>{let p=result.as_ref().map(|(_,p)|*p).unwrap_or_else(|e|e.retained_progress());if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)?;}else{cx.consume_retained(p)?;}cx.consume_fuel(p.copied_items as u64);Ok(result?.0)}
impl<'ctx,A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+'static,X:InstanceExtension> TypedCtx<'ctx,A,M,X>{
    /// 🧳️ Native parsing keeps the original child vectors and hands the resulting typed owners to the installed group pipeline.
    pub(super) async fn publish_mounted_typed_child_operation_unit(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,cx:&mut StepContext<'_>)->Result<(),PluginLifecycleFailure>{
        if cx.should_yield(){return Ok(())}
        let mut pending=mounted.pending_child_publication.take().ok_or_else(||original_failure("mounted child receiving lost its original owner"))?;
        let result=self.advance_original_child_receiving(mounted,&mut pending,cx);
        if pending.terminal_is_empty(){drop(pending);}else{mounted.pending_child_publication=Some(pending);}
        result
    }
    fn advance_original_child_receiving(&mut self,mounted:&mut MountedTypedCommandFullOperation<A>,pending:&mut PendingChildGroupPublication<A>,cx:&mut StepContext<'_>)->Result<(),PluginLifecycleFailure>{
        let grant=cx.retained_grant();
        if pending.phase==PendingChildGroupPublicationPhase::Closing{
            let d=pending.retirement_demands(grant.maximum_copy_bytes)?;
            if !plugin_grant_funds(d,grant){return Ok(())}
            if pending.requires_original_control(){cx.with_original_retirement(|cx|{let result=cx.with_native_retirement(|control|pending.close_step_controlled(grant,control));let p=result.as_ref().map(|step|step.progress()).unwrap_or_else(|e|e.retained_progress());cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);result?;Ok::<_,OriginalTurnFailure>(())})?;}else{let result=pending.close_step(grant);let p=result.as_ref().ok().and_then(|step|step.progress()).unwrap_or_default();cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);result?;}
            return Ok(())
        }
        if pending.phase!=PendingChildGroupPublicationPhase::Ready{return Err(original_failure("encoded child source cannot reenter a committed publication").into())}
        if !pending.native_reserved{
            let count=pending.child_emits.len();if count==0||count>PRIVATE_CHILD_GROUP_MAXIMUM_CHILDREN{return Err(original_failure("encoded child source exceeds the native group member bound").into())}
            let d=RetirementDemand{capacity_bytes:count.checked_mul(size_of::<OwnedChildEmit>()).ok_or_else(||original_failure("native child source capacity overflow"))?,..original_header::<(Vec<OwnedChildEmit>,bool)>()};if !plugin_grant_funds(d,grant){return Ok(())}
            pending.native_children.try_reserve_exact(count).map_err(|_|original_failure("original typed child vector allocation failed"))?;pending.native_reserved=true;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())
        }
        if let Some(receiver)=pending.native_receiver.as_mut(){
            if pending.native_batch.is_some(){
                let d=if receiver.terminal_is_empty(){RetirementDemand{release_bytes:receiver.allocation_bytes(),depth:1,..Default::default()}}else{receiver.close_demands(grant.maximum_copy_bytes)?};if !plugin_grant_funds(d,grant){return Ok(())}
                if receiver.terminal_is_empty(){pending.native_receiver=None;cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})?;cx.consume_fuel(1);}else{cx.with_original_retirement(|cx|{let result=cx.with_native_retirement(|control|receiver.close_step(control,grant));let p=result.as_ref().map(|step|step.progress()).unwrap_or_else(|e|e.retained_progress());cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);result?;Ok::<_,OriginalTurnFailure>(())})?;}
                return Ok(())
            }
            if receiver.ready(){let d=receiver.take_demands();if !plugin_grant_funds(d,grant){return Ok(())}let result=cx.with_native_receiving(|control|receiver.take(&mut pending.native_batch,&mut pending.native_wires,control,grant).map(|step|((),step.progress())));post_original(cx,result)?;return Ok(())}
            let d=receiver.demands(grant.maximum_copy_bytes)?;if !plugin_grant_funds(d,grant){return Ok(())}
            if receiver.settling(){cx.with_original_retirement(|cx|{let result=cx.with_native_retirement(|control|receiver.settle(control,grant));let p=result.as_ref().map(|step|step.progress()).unwrap_or_else(|e|e.retained_progress());cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);result?;Ok::<_,OriginalTurnFailure>(())})?;}else{let result=cx.with_native_receiving(|control|receiver.advance(control,grant).map(|step|((),step.progress())));post_original(cx,result)?;}
            return Ok(())
        }
        if pending.native_batch.is_some(){
            if pending.native_wires.is_some(){let d=original_header::<Vec<Vec<u8>>>();if !plugin_grant_funds(d,grant){return Ok(())}let child=pending.child_emits.get_mut(pending.native_child_index).ok_or_else(||original_failure("original child wire metadata slot is absent"))?;if !child.ops.is_empty()||child.ops.capacity()!=0{return Err(original_failure("original child wire metadata slot is occupied").into())}child.ops=pending.native_wires.take().unwrap();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())}
            let d=original_header::<(ChildEmit,OwnedChildEmit,usize)>();if !plugin_grant_funds(d,grant){return Ok(())}
            let empty=ChildEmit{genesis:None,owner:String::new(),slot:String::new(),child_id:String::new(),ops:Vec::new(),op_schema:protocol::SchemaId(String::new()),labels:Vec::new()};
            let metadata=std::mem::replace(&mut pending.child_emits[pending.native_child_index],empty);pending.native_children.push(OwnedChildEmit::new(metadata,pending.native_batch.take().unwrap()));pending.native_child_index+=1;
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())
        }
        if pending.native_child_index==pending.child_emits.len(){
            let d=original_header::<(Vec<A::Mutation>,Vec<OwnedChildEmit>,Option<protocol::TransactionRef>)>();if !plugin_grant_funds(d,grant){return Ok(())}
            let Some(ArtifactToolCompletionValue::Emit(Ok(emit),_))=mounted.publication.as_mut()else{return Err(original_failure("original child conversion lost its same emission recipient").into())};
            if !emit.artifact_mutations.is_empty()||emit.artifact_mutations.capacity()!=0||!emit.owned_child_emits.is_empty()||emit.owned_child_emits.capacity()!=0||emit.transaction.is_some(){return Err(original_failure("original child conversion recipient is occupied").into())}
            emit.artifact_mutations=std::mem::take(&mut*pending.artifact_mutations);emit.owned_child_emits=std::mem::take(&mut pending.native_children);emit.transaction=pending.transaction.take();pending.begin_close();
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())
        }
        let child=&mut pending.child_emits[pending.native_child_index];
        if pending.native_wires.is_none(){let d=original_header::<Vec<Vec<u8>>>();if !plugin_grant_funds(d,grant){return Ok(())}pending.native_wires=Some(std::mem::take(&mut child.ops));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(())}
        let member=self.child_composition.children.member(MemberKeyRef{owner:&child.owner,slot:&child.slot,child_id:&child.child_id}).ok_or_else(||original_failure("encoded child publication requires its exact live member; declared genesis uses typed child preparation"))?;
        let d=member.member.original_batch_receiving_demands()?;if !plugin_grant_funds(d,grant){return Ok(())}
        let result=cx.with_native_receiving(|control|member.member.admit_original_batch_receiving(&mut pending.native_wires,control,grant).map(|admitted|match admitted{Some((owner,p))=>{pending.native_receiver=Some(owner);((),p)},None=>((),Default::default())}));post_original(cx,result)?;Ok(())
    }
}
