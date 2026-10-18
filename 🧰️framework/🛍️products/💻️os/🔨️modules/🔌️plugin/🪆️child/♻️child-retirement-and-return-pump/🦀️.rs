//! ♻️ Child retirement and return pump of the child members: the content member metadata and retirement, the snapshot read return pump, the admission test state and the member metadata, retirement and store admission. See [`super`].

use super::*;

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ChildContentMemberMetadata {
    pub key: MemberKey,
    pub artifact_id: String,
    pub dialect: ArtifactDialect,
}

#[doc(hidden)]
pub struct ChildContentRetirement {
    pub view: ManuallyDrop<ChildContentView>,
    pub pending: ManuallyDrop<Option<ChildContentEntry>>,
    pub active: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub active_member: ManuallyDrop<Option<ChildContentMemberMetadata>>,
    pub metadata_close: ManuallyDrop<Option<ControlledRetirement<ChildContentMemberMetadata>>>,
    require_member_terminal: bool,
}

impl ChildContentRetirement {
    pub fn new(view: ChildContentView, require_member_terminal: bool) -> Self {
        Self { view: ManuallyDrop::new(view), pending: ManuallyDrop::new(None), active: ManuallyDrop::new(None), active_member: ManuallyDrop::new(None), metadata_close: ManuallyDrop::new(None), require_member_terminal }
    }

    fn snapshot_handoff_copy_bytes()->usize{
        size_of::<Option<ChildContentEntry>>()+size_of::<ChildContentEntry>()+size_of::<Option<store::os_store::snapshot_read::ErasedSnapshotRead>>()+size_of::<Option<ChildContentMemberMetadata>>()+size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>()
    }

    pub fn parent_snapshot_demand(mut child:RetirementDemand,parent_copy:usize)->Result<RetirementDemand,ValueError>{
        child.copy_bytes=child.copy_bytes.checked_add(parent_copy).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original child snapshot header overflow"))?;
        child.depth=child.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original child snapshot depth overflow"))?;Ok(child)
    }

    /// 📏️ Quotes the next disposer step, lease verification, entry disposal, or Arc release against its exact owning member.
    pub fn retirement_demands<M: SpaceMember>(&self, children: &ChildMemberRegistry<M>, retiring: Option<&ArtifactFixedRegistry<ChildMemberRetirement<M>>>, body: usize) -> Result<RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { let parent_copy=if active.terminal_is_empty(){size_of_val(&*self.active)}else{0};return Self::parent_snapshot_demand(store::os_store::snapshot_read::artifact_retirement_box_demands(active,body.saturating_sub(parent_copy))?,parent_copy); }
        if let Some(owner)=self.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata_close),depth:1,..Default::default()})}else{Self::parent_snapshot_demand(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?},0)}}
        if let Some(original) = self.active_member.as_ref() {
            if self.require_member_terminal{
                let Some(member)=child_snapshot_owner_ref(children,retiring,original.key.borrowed(),&original.artifact_id)else{return Ok(RetirementDemand{depth:1,..Default::default()})};
                let returned=member.returned_snapshot_read_retirement_demand()?;
                if returned!=RetirementDemand::default(){return Self::parent_snapshot_demand(returned,size_of_val(&*self.active))}
                if !member.snapshot_read_leases_terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}
            }
            return Ok(RetirementDemand{copy_bytes:size_of_val(&*self.active_member)+size_of_val(&*self.metadata_close),depth:1,..Default::default()});
        }
        if let Some(entry) = self.pending.as_ref() {
            let Some(member) = child_snapshot_owner_ref(children, retiring, entry.key(), &entry.artifact_id) else { return Ok(RetirementDemand { depth: 1, ..Default::default() }); };
            return Self::parent_snapshot_demand(member.snapshot_read_retirement_demand(),Self::snapshot_handoff_copy_bytes());
        }
        if self.terminal_is_empty() { return Ok(RetirementDemand::default()); }
        Ok(RetirementDemand { copy_bytes: self.view.next_take_copy_bytes(), release_bytes: self.view.next_take_release_bytes(), depth: 1, ..Default::default() })
    }

    pub fn close_step<M: SpaceMember>(
        &mut self,
        children: &mut ChildMemberRegistry<M>,
        retiring: Option<&mut ArtifactFixedRegistry<ChildMemberRetirement<M>>>,
        current: &ChildContentView,
        owners: &dyn ChildContentRetainers,
        grant: RetainedCloneGrant,
    ) -> Result<RetainedCloneStep, Fault> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(children, retiring.as_deref(), grant.maximum_copy_bytes).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        if grant.maximum_depth < demand.depth { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let admit = |progress: RetainedCloneProgress| semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "child snapshot retirement").map_err(|error| plugin_sdk_fault(error.to_string()));
        if self.active.is_some() {
            let parent_copy=if self.active.as_ref().unwrap().terminal_is_empty(){size_of_val(&*self.active)}else{0};
            let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
            let step = store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child)
                .map_err(|reason| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-snapshot-disposer-refused"), format!("child snapshot disposer refused its own bounded step: {reason}")))?;
            let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original child header preflight");return Ok(RetainedCloneStep::Progress(admit(receipt)?));
        }
        if let Some(owner)=self.metadata_close.as_mut(){
            if owner.terminal_is_empty(){drop(self.metadata_close.take());return Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty})?))}
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original child member metadata").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(admit(step.progress())?))
        }
        if let Some(original) = self.active_member.as_ref() {
            if self.require_member_terminal {
                let Some(member) = child_snapshot_owner(children, retiring, original.key.borrowed(), &original.artifact_id) else { return Ok(RetainedCloneStep::Progress(empty)); };
                if member.returned_snapshot_read_retirement_demand().map_err(|error|plugin_sdk_fault(error.into_message()))?!=RetirementDemand::default(){
                    let parent_copy=size_of_val(&*self.active);let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let(owner,progress)=member.take_returned_snapshot_read_retirement(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original returned child snapshot").map_err(|error|plugin_sdk_fault(error.into_message()))?;
                    let mut receipt=progress;if let Some(owner)=owner{*self.active=Some(owner);receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original returned header preflight")};return Ok(RetainedCloneStep::Progress(admit(receipt)?));
                }
                if !member.snapshot_read_leases_terminal_is_empty() { return Ok(RetainedCloneStep::Progress(empty)); }
            }
            if !<ChildContentMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child metadata has no controlled producer"))}
            let original=self.active_member.take().unwrap();match ControlledRetirement::new(original){Ok(owner)=>*self.metadata_close=Some(owner),Err((error,original))=>{*self.active_member=Some(original);return Err(plugin_sdk_fault(error.into_message()))}};
            return Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty})?));
        }
        if self.pending.is_none() {
            return match self.view.take_one(current, owners) {
                ChildContentTake::Blocked => Ok(RetainedCloneStep::Progress(empty)),
                ChildContentTake::Released(released_bytes) => Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes, ..empty })?)),
                ChildContentTake::Snapshot(entry, released_bytes) => {
                    *self.pending = Some(entry);
                    Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes, ..empty })?))
                }
                ChildContentTake::Complete(released_bytes) => Ok(RetainedCloneStep::Complete(admit(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes, ..empty })?)),
            };
        }
        let entry=self.pending.as_ref().unwrap();let Some(member)=child_snapshot_owner(children,retiring,entry.key(),&entry.artifact_id)else{return Ok(RetainedCloneStep::Progress(empty))};
        let parent_copy=Self::snapshot_handoff_copy_bytes();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
        let entry = self.pending.take().expect("pending child snapshot authority exists");
        let ChildContentEntry { owner: member_owner, slot, child_id, artifact_id, dialect, revision, snapshot } = entry;
        let mut read = Some(snapshot);
        let restore = |read: &mut Option<store::os_store::snapshot_read::ErasedSnapshotRead>, member_owner, slot, child_id, artifact_id, dialect, revision| ChildContentEntry { owner: member_owner, slot, child_id, artifact_id, dialect, revision, snapshot: read.take().expect("refused snapshot read remains with its caller") };
        match member.retire_snapshot_read_erased(&mut read, child) {
            Ok((Some(owner), progress)) => {
                semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original child snapshot handoff").map_err(|error|plugin_sdk_fault(error.into_message()))?;
                *self.active_member = Some(ChildContentMemberMetadata{key:MemberKey { owner: member_owner, slot, child_id }, artifact_id, dialect});
                *self.active = Some(owner);
                Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:progress.copied_items.max(1),copied_bytes:progress.copied_bytes.checked_add(parent_copy).expect("original child handoff preflight"),..progress})?))
            }
            Ok((None, progress)) => {
                *self.pending = Some(restore(&mut read, member_owner, slot, child_id, artifact_id, dialect, revision));
                semio_framework_value::retained_clone::admit_retained_clone_progress(child,progress,"original refused child snapshot").map_err(|error|plugin_sdk_fault(error.into_message()))?;
                Ok(RetainedCloneStep::Progress(admit(RetainedCloneProgress{copied_items:progress.copied_items.max(1),copied_bytes:progress.copied_bytes.checked_add(parent_copy).expect("original child restore preflight"),..progress})?))
            }
            Err(error) => {
                *self.pending = Some(restore(&mut read, member_owner, slot, child_id, artifact_id, dialect, revision));
                Err(plugin_sdk_fault(error.to_string()))
            }
        }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.view.root.is_none() && self.pending.is_none() && self.active.is_none() && self.active_member.is_none() && self.metadata_close.is_none()
    }
}

impl Drop for ChildContentRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "incomplete child-content retirement reached Drop without bounded disposal");
    }
}

#[doc(hidden)]
pub struct SnapshotReadReturnPump {
    pub active: ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl SnapshotReadReturnPump {
    pub fn new() -> Self {
        Self { active: ManuallyDrop::new(None) }
    }

    /// 📏️ Quotes the active disposer's next step, else the caller's quote of the next returned-read admission.
    pub fn retirement_demands(&self, body: usize, next: impl FnOnce() -> Result<RetirementDemand, ValueError>) -> Result<RetirementDemand, ValueError> {
        let parent_copy=size_of_val(&*self.active);
        match self.active.as_ref(){Some(active)=>{let copied=if active.terminal_is_empty(){parent_copy}else{0};ChildContentRetirement::parent_snapshot_demand(store::os_store::snapshot_read::artifact_retirement_box_demands(active,body.saturating_sub(copied))?,copied)},None=>{let child=next()?;if child==RetirementDemand::default(){Ok(child)}else{ChildContentRetirement::parent_snapshot_demand(child,parent_copy)}}}
    }

    pub fn drive(&mut self,next_demand:RetirementDemand,next:impl FnOnce(RetainedCloneGrant)->Result<(Option<Box<dyn store::ErasedSnapshotRetirement>>,RetainedCloneProgress),Fault>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(active)=self.active.as_ref(){
            let parent_copy=if active.terminal_is_empty(){size_of_val(&*self.active)}else{0};if grant.maximum_copy_bytes<parent_copy{return Ok(RetainedCloneStep::Progress(empty))}
            let demand=ChildContentRetirement::parent_snapshot_demand(store::os_store::snapshot_read::artifact_retirement_box_demands(active,grant.maximum_copy_bytes-parent_copy).map_err(|error|plugin_sdk_fault(error.into_message()))?,parent_copy).map_err(|error|plugin_sdk_fault(error.into_message()))?;
            if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
            let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let mut receipt=step.progress();receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original returned snapshot shell preflight");let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original returned snapshot shell").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(receipt));
        }
        if next_demand==RetirementDemand::default(){return Ok(RetainedCloneStep::Complete(empty))}
        let parent_copy=size_of_val(&*self.active);let demand=ChildContentRetirement::parent_snapshot_demand(next_demand,parent_copy).map_err(|error|plugin_sdk_fault(error.into_message()))?;if grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let(owner,progress)=next(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "returned snapshot-read admission").map_err(|error| plugin_sdk_fault(error.to_string()))?;
        match owner {
            Some(owner) => {
                *self.active = Some(owner);
                let mut receipt=progress;receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original returned snapshot handoff preflight");let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original returned snapshot handoff").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
            }
            None if progress == empty => Ok(RetainedCloneStep::Complete(empty)),
            None => Ok(RetainedCloneStep::Progress(progress)),
        }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.active.is_none()
    }
}

impl Drop for SnapshotReadReturnPump {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "returned snapshot-read pump reached Drop before its exact disposer was terminal-empty");
    }
}

#[doc(hidden)]
#[cfg(any(test, feature = "artifact-app-testing"))]
pub struct ChildAdmissionTestState {
    pub children_empty: bool,
    pub generation: u64,
    pub abort_generation: u64,
    pub abort_empty: bool,
    pub requested_abort_retained: bool,
    pub content_empty: bool,
    pub roots_retiring_empty: bool,
}

#[doc(hidden)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ChildMemberMetadata {
    pub reference: ArtifactRef,
    pub owner: store::OwnerRef,
}

#[doc(hidden)]
pub struct ChildMemberRetirement<M> {
    pub entry: ManuallyDrop<Option<ChildMemberEntry<M>>>,
    pub metadata:ManuallyDrop<Option<ControlledRetirement<ChildMemberMetadata>>>,
}

#[doc(hidden)]
pub struct ChildStoreAdmission {
    pub key: MemberKey,
    pub expected: ArtifactRef,
    pub owner: store::OwnerRef,
    pub member: ChildMemberAdmission,
    pub graph: store::os_store::composition_coordinator::OwnsAdmission,
    pub root_index: usize,
    pub generation: u64,
}

impl<M> ChildMemberRetirement<M> {
    pub(super) fn retires(&self, artifact_id: &str) -> bool {
        self.entry.as_ref().is_some_and(|entry| entry.reference.artifact_id == artifact_id)
    }
}

impl<M: SpaceMember> ChildMemberRetirement<M> {
    pub fn new(entry: ChildMemberEntry<M>) -> Self {
        Self { entry: ManuallyDrop::new(Some(entry)),metadata:ManuallyDrop::new(None) }
    }

    pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(owner)=self.metadata.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of_val(&*self.metadata),depth:1,..Default::default()})}else{ChildContentRetirement::parent_snapshot_demand(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?},0)}}
        let Some(entry)=self.entry.as_ref()else{return Ok(Default::default())};
        if !entry.member.snapshot_read_leases_terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if !entry.member.close_owned_terminal_is_empty(){return ChildContentRetirement::parent_snapshot_demand(entry.member.close_owned_demands(body)?,0)}
        Ok(RetirementDemand{copy_bytes:size_of_val(&*self.entry)+size_of::<ChildMemberEntry<M>>()+size_of_val(&*self.metadata),depth:1,..Default::default()})
    }

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}
        let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.into_message()))?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.metadata.as_mut(){if owner.terminal_is_empty(){drop(self.metadata.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original child member metadata").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(step.progress()))}
        let entry=self.entry.as_mut().unwrap();let member=&mut entry.member;
        if !member.snapshot_read_leases_terminal_is_empty(){return Ok(RetainedCloneStep::Progress(empty))}
        if !member.close_owned_terminal_is_empty(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=member.close_owned_step(child).map_err(|error|plugin_sdk_fault(error.into_message()))?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,member.close_owned_terminal_is_empty(),"original child member store").map_err(|error|plugin_sdk_fault(error.into_message()))?;return Ok(RetainedCloneStep::Progress(step.progress()))}
        if !<ChildMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(plugin_sdk_fault("original child member metadata requires its native controlled producer"))}
        let ChildMemberEntry{reference,owner,member}=self.entry.take().unwrap();let original=ChildMemberMetadata{reference,owner};match ControlledRetirement::new(original){Ok(owner)=>*self.metadata=Some(owner),Err((error,original))=>{*self.entry=Some(ChildMemberEntry{reference:original.reference,owner:original.owner,member});return Err(plugin_sdk_fault(error.into_message()))}};drop(member);
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.entry.is_none()&&self.metadata.is_none()
    }
}

impl<M> Drop for ChildMemberRetirement<M> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.entry.is_none()&&self.metadata.is_none()), "child-member retirement reached Drop before its exact owner was terminal-empty");
    }
}
