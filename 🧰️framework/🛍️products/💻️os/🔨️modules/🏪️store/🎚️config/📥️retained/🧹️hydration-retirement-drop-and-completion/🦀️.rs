//! 🧹️ Hydration retirement, drop and completion: the remaining hydration operations, the erased snapshot retirement with its controlled closes, the drop and the completed hydration accessors. See [`super`].

use super::*;
use crate::retained_config_hydration::hydration_types_constructor_and_drive::RetainedConfigStoreHydration;

impl<P, M> RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn retirement_demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};
        use semio_framework_value::retirement::RetireOwned;
        if self.terminal_is_empty_unbounded() { return Ok(Default::default()); }
        fn controlled<T: RetireOwned>(_: &T) -> Result<RetirementDemand, ValueError> {
            if !T::controlled_retirement_supported() { return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original config retained value has no controlled retirement declaration")); }
            Ok(RetirementDemand { capacity_bytes: semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<T>(), depth: 2, ..Default::default() })
        }
        fn child(mut demand: RetirementDemand) -> Result<RetirementDemand, ValueError> {
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "original config owner depth overflow"))?;
            Ok(demand)
        }
        if let Some(owner) = self.active.as_ref() { return semio_framework_os_store_read::artifact_retirement_box_demands(owner, copy); }
        if let Some(owner) = self.fold_job.as_ref() { return child(owner.next_demand(copy)?); }
        if let Some(value) = self.normalized_transitions.as_ref() { return controlled(value); }
        if let Some(value) = self.fold.as_ref() { return controlled(value); }
        if let Some(value) = self.fold_auxiliary.as_ref() { return controlled(value); }
        if let Some(value) = self.source_edits.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.source_forwards.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.source_inverse.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.source_metadata.as_ref() { return child(value.demands(copy)?); }
        if let Some(value) = self.pending_source_lane.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_source_edit.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_retired_payload.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_payload.as_ref() { return controlled(value); }
        if let Some(value) = self.pending_metadata.as_ref() { return controlled(value); }
        if self.owners.is_some() {
            if let Some(owner) = self.runtime.as_ref() { return child(owner.initialization_retirement_demands(copy)?); }
            if self.pending_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>(), depth: 2, ..Default::default() }); }
        }
        if self.retired_messages.is_some() || self.pending_messages.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<semio_framework_os_spr::command::MutationMessageLedgerRetirement>(), depth: 2, ..Default::default() }); }
        if self.owners.is_none() {
            if self.runtime.is_some() || self.pending_edit.is_some() || self.pending_snapshot.is_some() || self.validation.is_some() || self.current.is_some() || self.envelope.is_some() || self.initial.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "config root retirement requires its original catalog")); }
            if let Some(value) = self.history.as_ref() { return controlled(value); }
            if let Some(value) = self.edit_lookup.as_ref() { return controlled(value); }
            if let Some(value) = self.expected_id.as_ref() { return controlled(value); }
            if self.actor.0.has_owner() { return controlled(&self.actor.0); }
            if let Some(value) = self.schema.as_ref() { return controlled(value); }
            return Ok(Default::default());
        }
        if let Some(owner) = self.runtime.as_ref() { return child(owner.initialization_retirement_demands(copy)?); }
        if self.pending_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement<M>>(), depth: 2, ..Default::default() }); }
        if let Some(value) = self.pending_snapshot.as_ref().or_else(|| self.validation.as_ref()).or_else(|| self.current.as_ref()) { let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?; return Ok(RetirementDemand { capacity_bytes: owners.initial_snapshot_retirement.retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
        if self.envelope.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<semio_framework_os_store_read::ArtifactStoreEnvelopeRetirement<P, M>>(), depth: 2, ..Default::default() }); }
        if self.initial.is_some() { return Ok(RetirementDemand { capacity_bytes: size_of::<semio_framework_os_store_read::ArtifactGenesisRetirement<P>>(), depth: 2, ..Default::default() }); }
        if let Some(value) = self.history.as_ref() { return controlled(value); }
        if let Some(value) = self.edit_lookup.as_ref() { return controlled(value); }
        if let Some(value) = self.expected_id.as_ref() { return controlled(value); }
        if self.actor.0.has_owner() { return controlled(&self.actor.0); }
        if let Some(value) = self.schema.as_ref() { return controlled(value); }
        let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
        child(owners.uninstalled_owners_demands(copy)?)
    }
}

impl<P, M> ErasedSnapshotRetirement for RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.retirement_demands(0).map(|demand| demand.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.retirement_demands(copy).map(|demand| demand.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.retirement_demands(0).map(|demand| demand.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.retirement_demands(0).map(|demand| demand.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
        use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress, admit_retained_clone_close}};
        use semio_framework_value::retirement::controlled::admit_typed_controlled_retirement;
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "original config hydration depth refused")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        self.request_cancel();
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() { return semio_framework_os_store_read::artifact_retirement_box_close_step(&mut self.active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.fold_job.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config fold")?;
            if terminal { drop(self.fold_job.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        macro_rules! controlled {
            ($field:ident) => {
                if let Some(value) = self.$field.take() {
                    match admit_typed_controlled_retirement(value, child) {
                        Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                        Err((error, value)) => { *self.$field = Some(value); return Err(error); }
                    }
                }
            };
        }
        controlled!(normalized_transitions);
        controlled!(fold);
        controlled!(fold_auxiliary);
        if let Some(owner) = self.source_edits.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_edits")?;
            if terminal { drop(self.source_edits.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.source_forwards.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_forwards")?;
            if terminal { drop(self.source_forwards.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.source_inverse.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_inverse")?;
            if terminal { drop(self.source_inverse.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(owner) = self.source_metadata.as_mut() {
            let step = owner.close_step(child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config source_metadata")?;
            if terminal { drop(self.source_metadata.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        controlled!(pending_source_lane);
        controlled!(pending_source_edit);
        controlled!(pending_retired_payload);
        controlled!(pending_payload);
        controlled!(pending_metadata);
        if self.owners.is_none() {
            if let Some(messages) = self.retired_messages.take() {
                match semio_framework_value::retirement::frame::admit_retirement_frame(messages, child, |messages| semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                    Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                    Err((error, messages)) => { *self.retired_messages = Some(messages); return Err(error); }
                }
            }
            if let Some(messages) = self.pending_messages.take() {
                match semio_framework_value::retirement::frame::admit_retirement_frame(messages, child, |messages| semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                    Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                    Err((error, messages)) => { *self.pending_messages = Some(messages); return Err(error); }
                }
            }
            if self.runtime.is_some() || self.pending_edit.is_some() || self.pending_snapshot.is_some() || self.validation.is_some() || self.current.is_some() || self.envelope.is_some() || self.initial.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "config root retirement requires its original catalog")); }
            controlled!(history);
            controlled!(edit_lookup);
            controlled!(expected_id);
            if self.actor.0.has_owner() {
                let original = std::mem::take(&mut self.actor.0);
                match admit_typed_controlled_retirement(original, child) {
                    Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                    Err((error, original)) => { self.actor.0 = original; return Err(error); }
                }
            }
            controlled!(schema);
            self.terminal = true;
            return Ok(RetainedCloneStep::Complete(empty));
        }
        if let Some(owner) = self.runtime.as_mut() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let step = owner.close_step(&owners.initial_snapshot_retirement, child)?;
            let terminal = owner.terminal_is_empty();
            admit_retained_clone_close(child, step, terminal, "original config initialization")?;
            if terminal { drop(self.runtime.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending_edit.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let edit = self.pending_edit.take().expect("observed original config pending_edit");
            match owners.retire_decoded_edit(edit, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, edit)) => { *self.pending_edit = Some(edit); return Err(error); }
            }
        }
        if let Some(messages) = self.retired_messages.take() {
            match semio_framework_value::retirement::frame::admit_retirement_frame(messages, child, |messages| semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, messages)) => { *self.retired_messages = Some(messages); return Err(error); }
            }
        }
        if let Some(messages) = self.pending_messages.take() {
            match semio_framework_value::retirement::frame::admit_retirement_frame(messages, child, |messages| semio_framework_os_spr::command::MutationMessageLedgerRetirement::new(messages.edit_id, messages.messages)) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, messages)) => { *self.pending_messages = Some(messages); return Err(error); }
            }
        }
        if self.pending_snapshot.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.pending_snapshot.take().expect("observed original config pending_snapshot");
            match owners.retire_initial_snapshot_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.pending_snapshot = Some(value); return Err(error); }
            }
        }
        if self.validation.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.validation.take().expect("observed original config validation");
            match owners.retire_initial_snapshot_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.validation = Some(value); return Err(error); }
            }
        }
        if self.current.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.current.take().expect("observed original config current");
            match owners.retire_initial_snapshot_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.current = Some(value); return Err(error); }
            }
        }
        if self.envelope.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.envelope.take().expect("observed original config envelope");
            match owners.retire_decoded_envelope(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.envelope = Some(value); return Err(error); }
            }
        }
        if self.initial.is_some() {
            let owners = self.owners.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "original config retirement lost its catalog"))?;
            let value = self.initial.take().expect("observed original config initial");
            match owners.retire_genesis_owned(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { *self.initial = Some(value); return Err(error); }
            }
        }
        controlled!(history);
        controlled!(edit_lookup);
        controlled!(expected_id);
        if self.actor.0.has_owner() {
            let value = std::mem::take(&mut self.actor.0);
            match admit_typed_controlled_retirement(value, child) {
                Ok((owner, progress)) => { *self.active = Some(owner); return Ok(RetainedCloneStep::Progress(progress)); }
                Err((error, value)) => { self.actor.0 = value; return Err(error); }
            }
        }
        controlled!(schema);
        let owners = self.owners.as_mut().expect("original config catalog remains retained");
        let step = owners.close_uninstalled_owners_step(child)?;
        let terminal = owners.uninstalled_owners_terminal_is_empty();
        admit_retained_clone_close(child, step, terminal, "original config catalog")?;
        if terminal { drop(self.owners.take()); self.terminal = true; }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal
            && !self.actor.0.has_owner()
            && self.initial.is_none()
            && self.validation.is_none()
            && self.current.is_none()
            && self.history.is_none()
            && self.fold_job.is_none()
            && self.fold.is_none()
            && self.normalized_transitions.is_none()
            && self.fold_auxiliary.is_none()
            && self.source_edits.is_none()
            && self.source_forwards.is_none()
            && self.source_inverse.is_none()
            && self.source_metadata.is_none()
            && self.pending_source_lane.is_none()
            && self.pending_snapshot.is_none()
            && self.pending_source_edit.is_none()
            && self.pending_retired_payload.is_none()
            && self.pending_payload.is_none()
            && self.pending_metadata.is_none()
            && self.expected_id.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.retired_messages.is_none()
            && self.edit_lookup.is_none()
            && self.active.is_none()
    }
}

impl<P, M> Drop for RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty_unbounded(), "retained config hydration reached Drop before exact handoff or bounded retirement");
    }
}

impl<P, M> RetainedConfigStoreHydration<P, M>
where
    P: Clone + ToValue + FromValue,
    M: Clone + ToValue + FromValue + Mutation<P>,
{
    pub fn terminal_is_empty_unbounded(&self) -> bool {
        self.terminal
            && !self.actor.0.has_owner()
            && self.initial.is_none()
            && self.validation.is_none()
            && self.current.is_none()
            && self.history.is_none()
            && self.fold_job.is_none()
            && self.fold.is_none()
            && self.normalized_transitions.is_none()
            && self.fold_auxiliary.is_none()
            && self.source_edits.is_none()
            && self.source_forwards.is_none()
            && self.source_inverse.is_none()
            && self.source_metadata.is_none()
            && self.pending_source_lane.is_none()
            && self.pending_snapshot.is_none()
            && self.pending_source_edit.is_none()
            && self.pending_retired_payload.is_none()
            && self.pending_payload.is_none()
            && self.pending_metadata.is_none()
            && self.expected_id.is_none()
            && self.schema.is_none()
            && self.envelope.is_none()
            && self.runtime.is_none()
            && self.owners.is_none()
            && self.pending_edit.is_none()
            && self.pending_messages.is_none()
            && self.retired_messages.is_none()
            && self.edit_lookup.is_none()
            && self.active.is_none()
    }
}
