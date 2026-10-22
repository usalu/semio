//! 💧️ Change hydration phases: the changes, checkpoints, alternatives and pins hydration of the hydration step. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static + semio_framework_os_spr::ColdMutation<P>,
{
    /// 🔄 Hydrates the change records.
    pub(super) fn step_hydrate_changes(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let fold = self.fold.as_mut().expect("history fold remains retained");
        if let Some(change) = fold.changes.get_mut(self.operation_index) {
            let change = semio_framework_os_vcs::Change { id: std::mem::take(&mut change.id), edit_ids: std::mem::take(&mut change.edit_ids), description: change.description.take(), saved_at: std::mem::take(&mut change.saved_at) };
            if let Err(change) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.changes.try_push(change) {
                *self.active = Some(Box::new(semio_framework_os_store_read::ArtifactStoreHistoryMetadataRetirement::change(change)));
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = Phase::HydrateCheckpoints;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 📍 Hydrates the checkpoints.
    pub(super) fn step_hydrate_checkpoints(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let fold = self.fold.as_mut().expect("history fold remains retained");
        if let Some(source) = fold.checkpoints.get_mut(self.operation_index) {
            let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
            if self.pin_index == 0 {
                let checkpoint = semio_framework_os_vcs::Checkpoint { id: std::mem::take(&mut source.id), change_ids: std::mem::take(&mut source.change_ids), parent_id: source.parent_id.take(), authors: Vec::with_capacity(source.authors.len()), message: source.message.take(), timestamp: std::mem::take(&mut source.timestamp), composition_pins: Vec::new() };
                if let Err(checkpoint) = envelope.vcs.checkpoints.try_push(checkpoint) {
                    *self.active = Some(Box::new(semio_framework_os_store_read::ArtifactStoreHistoryMetadataRetirement::checkpoint(checkpoint)));
                    return self.reject(MemberOpenDiagnostic::Capacity);
                }
                self.pin_index = 1;
            } else if let Some(author) = source.authors.get_mut(self.pin_index - 1) {
                envelope.vcs.checkpoints.get_mut(self.operation_index).expect("hydrated checkpoint remains indexed").authors.push(semio_framework_os_vcs::Author { id: std::mem::take(&mut author.id), name: std::mem::take(&mut author.name), avatar: author.avatar.take() });
                self.pin_index += 1;
            } else {
                self.operation_index += 1;
                self.pin_index = 0;
            }
        } else {
            self.operation_index = 0;
            self.pin_index = 0;
            self.phase = Phase::HydrateAlternatives;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🔀 Hydrates the alternatives.
    pub(super) fn step_hydrate_alternatives(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let fold = self.fold.as_mut().expect("history fold remains retained");
        if let Some(alternative) = fold.alternatives.get_mut(self.operation_index) {
            let alternative = semio_framework_os_vcs::Alternative { id: std::mem::take(&mut alternative.id), name: std::mem::take(&mut alternative.name), checkpoint_ids: std::mem::take(&mut alternative.checkpoint_ids) };
            if let Err(alternative) = self.envelope.as_mut().expect("hydrated envelope remains retained").vcs.alternatives.try_push(alternative) {
                *self.active = Some(Box::new(semio_framework_os_store_read::ArtifactStoreHistoryMetadataRetirement::alternative(alternative)));
                return self.reject(MemberOpenDiagnostic::Capacity);
            }
            self.operation_index += 1;
        } else {
            self.operation_index = 0;
            self.phase = Phase::HydratePins;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 📌 Hydrates the pins.
    pub(super) fn step_hydrate_pins(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};
        let Some(source) = self.fold.as_ref().expect("history fold remains retained").checkpoints.get(self.pin_group_index) else {
            self.record_index = 0;
            self.operation_index = 0;
            self.phase = Phase::PrepareReplay;
            cx.consume_fuel(1);
            return PersistedDocumentHydrationStep::Pending(self.progress());
        };
        let envelope = self.envelope.as_mut().expect("hydrated envelope remains retained");
        let Some(checkpoint) = envelope.vcs.checkpoints.get_mut(self.pin_group_index) else {
            return self.reject(MemberOpenDiagnostic::Replay);
        };
        if self.pin_index == 0 && checkpoint.composition_pins.try_reserve_exact(source.pins.len()).is_err() {
            return self.reject(MemberOpenDiagnostic::Capacity);
        }
        if let Some(pin) = source.pins.get(self.pin_index) {
            let child_ref = match ArtifactRef::parse_uri(&pin.child_uri) {
                Ok(reference) => reference,
                Err(_) => return self.reject(MemberOpenDiagnostic::Replay),
            };
            if !self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained").insert(child_ref.clone()) {
                return self.reject(MemberOpenDiagnostic::Replay);
            }
            checkpoint.composition_pins.push(CompositionPin { child_ref, checkpoint_id: pin.checkpoint_id.clone() });
            self.pin_index += 1;
        } else {
            if !owned_frame_funded::<std::collections::HashSet<ArtifactRef>>(grant) { return PersistedDocumentHydrationStep::Pending(self.progress()); }
            let previous = std::mem::take(self.pin_refs.as_mut().expect("hydrated checkpoint pin index remains retained"));
            if let Err(diagnostic) = self.admit_active(previous, cx, grant) { return self.reject(diagnostic); }
            self.pin_group_index += 1;
            self.pin_index = 0;
        }
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }
}
