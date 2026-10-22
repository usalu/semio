//! 🧹️ Member retirement and close of the instance tool run turns: the member retire step and the retire, entry close and close steps of the tool run ledger. See [`super`].

use super::*;
use semio_framework::Fault;
use semio_framework_plugin_decl_emission::ChildEmit;
use semio_framework_plugin_decl_views::child_view::ChildContentView;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunCtx<'ctx, A, M, X> {
    /// ♻️ One bounded unit of member-run retirement: one alias of a member run's typed owners handed to its member store's
    /// alias retirement (displaced aliases of live runs, every alias of a run that left its slot), else the composed reads of
    /// a fully retired member run. `None` when nothing member-owned is retiring or an alias retirement is still stepping.
    fn tool_run_member_retire_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,Fault>{
        if self.tool_runs.snapshot_retirement.is_some(){return Ok(None);}
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(index)=self.tool_runs.retired_members.iter().position(|member|!member.stale.is_empty()||member.children.is_some()){
            let copy=size_of::<ChildContentView>();
            if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            let mut member=self.tool_runs.retired_members.remove(index);
            let mut views=std::mem::take(&mut member.stale);
            views.extend(member.children.take());
            let outcome=self.child_composition.retire_tool_run_views(views, &mut member.stale);
            self.tool_runs.retired_members.insert(index,member);
            return match outcome{
                Ok(())=>Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))),
                Err(fault) if fault.retryable=>Ok(Some(RetainedCloneStep::Progress(Default::default()))),
                Err(fault)=>Err(fault),
            };
        }
        let ToolRunCtx{tool_runs,child_composition,..}=self;
        let children=&mut child_composition.children;
        let live=tool_runs.entries.iter_mut().filter_map(|entry|entry.member.as_mut()).map(|member|(member,false));
        let retired=tool_runs.retired_members.iter_mut().map(|member|(member,true));
        for(member,everything)in live.chain(retired){
            let queued_owner=!member.retired_emission_owners.is_empty();
            let owner=if queued_owner{member.retired_emission_owners.front_mut()}else if everything{member.emission_owner.as_mut()}else{None};
            if let Some(owner)=owner{
                if owner.is_empty(){
                    let demand=RetirementDemand{copy_bytes:size_of::<Option<Box<dyn ToolRunMemberEmissionOwner>>>(),release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()};
                    if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
                    if queued_owner{drop(member.retired_emission_owners.pop_front());}else{drop(member.emission_owner.take());}
                    return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})));
                }
                let step=match children.get(&(member.slot.clone(),member.child_id.clone())){
                    Some(child)=>child.member.visit_member(ToolRunMemberEmissionRetire{owner:owner.as_mut(),grant})?,
                    None=>owner.close_step(grant)?,
                };return Ok(Some(step));
            }
            if !member.retired_emits.is_empty()||everything&&member.emit.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool member metadata retains its granted native emission retirement authority").into_fault());}
            let queued_ops=!member.retired_ops.is_empty();
            let ops=if queued_ops{member.retired_ops.front_mut()}else if everything&&(!member.ops.is_empty()||member.ops.capacity()!=0){Some(&mut member.ops)}else{None};
            if let Some(ops)=ops{
                let copy=2*size_of::<Vec<u8>>();let release=ops.last().map_or_else(||ops.capacity().checked_mul(size_of::<Vec<u8>>()),|op|Some(op.capacity())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original member operation backing overflow").into_fault())?;
                if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,release_bytes:release,depth:1,..Default::default()}){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
                if !ops.is_empty(){drop(ops.pop());}else{drop(std::mem::take(ops));if queued_ops{drop(member.retired_ops.pop_front());}}
                return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})));
            }
            macro_rules! close_member_backing{
                ($field:ident,$item:ty)=>{
                    if member.$field.is_empty()&&member.$field.capacity()!=0{
                        let release=member.$field.capacity().checked_mul(std::mem::size_of::<$item>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original member queue backing overflow").into_fault())?;
                        let copy=2*std::mem::size_of::<std::collections::VecDeque<$item>>();
                        if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,release_bytes:release,depth:1,..Default::default()}){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
                        drop(std::mem::take(&mut member.$field));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()})));
                    }
                };
            }
            close_member_backing!(retired_emission_owners,Box<dyn ToolRunMemberEmissionOwner>);
            close_member_backing!(retired_emits,ChildEmit);
            close_member_backing!(retired_ops,Vec<Vec<u8>>);
            let Some(owners)=member.owners.as_deref_mut().filter(|owners|everything||owners.has_displaced())else{continue};
            let child=children.get(&(member.slot.clone(),member.child_id.clone())).ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool member original snapshot retains its missing exact Store issuer").into_fault())?;
            let(owner,progress)=child.member.visit_member(ToolRunMemberRetire{owners,everything,grant})?;
            if let Some(owner)=owner{tool_runs.snapshot_retirement=Some(owner);}
            if progress!=RetainedCloneProgress::default(){return Ok(Some(RetainedCloneStep::Progress(progress)));}
        }
        if let Some(index)=tool_runs.retired_members.iter().position(ToolRunMemberRun::is_retired){
            let release=tool_runs.retired_members[index].address_backing_bytes();
            let demand=RetirementDemand{copy_bytes:size_of::<ToolRunMemberRun>(),release_bytes:release,depth:1,..Default::default()};
            if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            drop(tool_runs.retired_members.remove(index));
            return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})));
        }
        Ok(None)
    }

    pub fn tool_run_retire_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<RetainedCloneStep>,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();if cx.should_yield(){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        for entry in [false,true]{
            let step=if entry{self.tool_run_entry_close_step(grant)}else{self.tool_run_member_retire_step(grant)}.map_err(|fault|semio_framework_job::OriginalTurnFailure::debit_original(fault,cx))?;
            if let Some(step)=step{cx.consume_fuel(step.progress().copied_items as u64);cx.consume_retained(step.progress())?;return Ok(Some(step))}
        }
        self.tool_runs.retire_step(&mut self.stores.store,cx)
    }

    /// 🚪️ Closes the newest live entry of a closing ledger one funded unit at a time; its job, member and provisional owners move to the ledger's retiring lists.
    fn tool_run_entry_close_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,Fault>{
        let Some(demand)=self.shared().tool_run_entry_close_demand(grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?else{return Ok(None)};
        if self.tool_runs.has_original_receipt()||grant.maximum_items==0||!tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let entry=self.tool_runs.entries.last_mut().expect("a quoted close unit has its entry");
        if let Some(finalize)=entry.finalize.as_mut(){
            if let Some(publication)=finalize.publication.as_mut(){
                if publication.terminal_is_empty(){drop(finalize.publication.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
                publication.begin_close();
                let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
                let step=publication.close_step(child).map_err(FaultFrom::into_fault)?;
                let step=admit_retained_clone_close(child,step,publication.terminal_is_empty(),"tool entry final publication").map_err(FaultFrom::into_fault)?;
                return Ok(Some(RetainedCloneStep::Progress(step.progress())));
            }
            if !finalize.ingress.terminal_is_empty(){return finalize.ingress.close_step(grant).map(|step|Some(RetainedCloneStep::Progress(step.progress()))).map_err(FaultFrom::into_fault);}
        }
        let entry=self.tool_runs.entries.pop().expect("a quoted close unit has its entry");
        self.tool_runs.selected=None;
        self.tool_runs.retire_entry(*entry);
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})))
    }

    /// 🎟️ Forwards the original full grant and retains every unsupported cleanup frontier in place.
    pub fn tool_run_close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
        if !self.tool_runs.closing{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"ToolRun close requires genuine granted entry cancellation admission").into())}
        Ok(self.tool_run_retire_step(cx)?.unwrap_or_else(||if self.tool_runs.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())}))
    }
}
