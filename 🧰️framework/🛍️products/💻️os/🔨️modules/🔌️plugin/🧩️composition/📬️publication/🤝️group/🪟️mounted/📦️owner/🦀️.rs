use mounted_child_group_receipt_group::{MountedGroupMemberReceipts,MountedGroupReceiptOutput};
use mounted_child_group_receipt::MountedArtifactHandleIssuer;
use std::mem::ManuallyDrop;

pub(crate) struct MountedPrivateChildGroup<M:SpaceMember+MemberFactory> {
    pub(crate) identity:MountedChildGroupIdentityIssuer,
    pub(crate) parent_issuer:MountedArtifactHandleIssuer,
    pub(crate) parent_issuer_closed:bool,
    pub(crate) parent_handle:Option<ArtifactHandle>,
    pub(crate) parent_touched:bool,
    pub(crate) group_id:ManuallyDrop<Option<String>>,
    pub(crate) parent_batch:ManuallyDrop<Option<store::MemberStoreOwnedBatch>>,
    pub(crate) publication:ManuallyDrop<Option<PrivateChildPublicationGroup<M>>>,
    pub(crate) receipts:ManuallyDrop<Option<Box<MountedGroupMemberReceipts>>>,
    pub(crate) command:Option<MountedCommandEntry>,
    pub(crate) append:Option<CommandAppend>,
    pub(crate) output:ManuallyDrop<Option<MountedGroupReceiptOutput>>,
    pub(crate) record:ManuallyDrop<Option<CommandLogEntry>>,
    record_close_field:usize,
    pub(crate) displaced_shell_redo:ManuallyDrop<Vec<ShellHistoryReplay>>,
    pub(crate) phase:u8,
    pub(crate) captured_millis:i64,
    pub(crate) expires_at_us:u64,
    pub(crate) closing:bool,
}

impl<M:SpaceMember+MemberFactory> MountedPrivateChildGroup<M> {
    pub(crate) fn new(captured_millis:i64,expires_at_us:u64)->Self{Self{identity:MountedChildGroupIdentityIssuer::new(),parent_issuer:MountedArtifactHandleIssuer::new(),parent_issuer_closed:false,parent_handle:None,parent_touched:false,group_id:ManuallyDrop::new(None),parent_batch:ManuallyDrop::new(None),publication:ManuallyDrop::new(None),receipts:ManuallyDrop::new(None),command:None,append:None,output:ManuallyDrop::new(None),record:ManuallyDrop::new(None),displaced_shell_redo:ManuallyDrop::new(Vec::new()),record_close_field:0,phase:0,captured_millis,expires_at_us,closing:false}}
    pub(crate) fn frame_birth_bytes()->usize{size_of::<Self>()}
    pub(crate) fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
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
        if let Some(batch) = self.parent_batch.as_ref() { return nested(batch.next_demands(maximum_body_bytes)?, 1); }
        if let Some(id) = self.group_id.as_ref() { return Ok(release(id.capacity(), 1)); }
        if !self.identity.terminal_is_empty() { return Ok(release(self.identity.next_close_byte_demand(), 2)); }
        Ok(release(0, 2))
    }
    pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.output.is_none()&&self.record.is_none()&&self.displaced_shell_redo.capacity()==0&&self.append.is_none()&&self.command.is_none()&&self.receipts.is_none()&&self.publication.is_none()&&self.parent_batch.is_none()&&self.group_id.is_none()&&self.identity.terminal_is_empty()&&self.parent_issuer_closed}
    pub(crate) fn close_step(&mut self,parent:&mut impl SpaceMember,registry:&mut ChildMemberRegistry<M>,graph:&mut store::os_store::composition_coordinator::CompositionGraph,live:&ChildContentView,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,crate::PluginLifecycleFailure>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        use semio_framework_job::InteractiveJobCloseStep;
        let pending=|step:RetainedCloneStep| InteractiveJobCloseStep::Pending{progress:step.progress()};
        if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()});}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if !plugin_grant_funds(demand, grant){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()});}
        let bytes=demand.release_bytes;
        let child_grant=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        self.closing=true;
        if self.output.is_some(){return Ok(pending(self.receipts.as_mut().and_then(|receipts|receipts.final_receipt_mut()).expect("transferred output retains its exact empty receipt owner").retain_output(&mut self.output,child_grant)));}
        if let Some(record)=self.record.as_mut(){if self.record_close_field==8{self.record.take();}else if PagedCommandLog::record_close_one(record,self.record_close_field,child_grant.maximum_release_bytes)?{self.record_close_field+=1;}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if let Some(replay)=self.displaced_shell_redo.last_mut(){if replay.redo.action_id.capacity()!=0{drop(std::mem::take(&mut replay.redo.action_id));}else if let Some(value)=replay.redo.args.as_mut(){if matches!(value,DslValue::Null){replay.redo.args.take();}else{PagedCommandLog::value_close_one(value);}}else{self.displaced_shell_redo.pop();}return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if self.displaced_shell_redo.capacity()!=0{drop(std::mem::take(&mut*self.displaced_shell_redo));return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if let Some(append)=self.append.as_mut(){let step=append.close_step(child_grant);self.append=None;return Ok(pending(step));}
        if let Some(command)=self.command.as_mut(){let step=command.close_step(child_grant);if command.terminal_is_empty(){self.command=None;}return Ok(pending(step));}
        if let Some(receipts)=self.receipts.as_mut(){if !receipts.terminal_is_empty(){return receipts.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 2, ..child_grant }).map(pending).map_err(Into::into);}drop(self.receipts.take());return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if let Some(group)=self.publication.as_mut(){let step=group.close_step(parent,registry,graph,live,cx,child_grant)?;if group.terminal_is_empty(){self.publication.take();}return Ok(pending(step));}
        if let Some(batch)=self.parent_batch.as_mut(){let step=batch.close_granted(child_grant)?;if batch.terminal_is_empty(){self.parent_batch.take();}return Ok(pending(step));}
        if self.group_id.is_some(){drop(self.group_id.take());return Ok(InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}});}
        if !self.identity.terminal_is_empty(){return Ok(pending(self.identity.close_granted(child_grant)));}
        if !self.parent_issuer_closed{let step=self.parent_issuer.close_step(child_grant);self.parent_issuer_closed=true;return Ok(pending(step));}
        Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }
}
impl<M:SpaceMember+MemberFactory> Drop for MountedPrivateChildGroup<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"mounted private child frame retains original inputs and staged owners until funded terminal close");}}

#[cfg(test)]
include!("🧪️tests/🦀️.rs");

#[cfg(test)]
include!("../../../../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/📦️driver/🦀️.rs");
