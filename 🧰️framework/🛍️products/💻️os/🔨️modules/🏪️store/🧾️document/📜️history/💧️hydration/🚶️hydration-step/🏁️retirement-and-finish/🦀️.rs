//! 🏁️ Retirement and finish phases: the history and pack retirement, the envelope close and the finish of the hydration step. See [`super`].

use super::*;


impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: ArtifactPack + semio_os_store_text_io::ArtifactStoreSnapshot + semio_os_spr::StoreState,
    M: semio_os_spr::StoreMutationFor<P>+protocol::OpBinaryReceiving,
{
    /// 🧹 Retires the history runtime.
    pub(super) fn step_retire_history(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        macro_rules! retire {
            ($field:ident) => { if self.$field.is_some() { let step = owned_outcome(cx, grant, &mut self.$field, &mut self.active); return self.admitted_step(step.result, cx); } };
        }
        retire!(history);
        retire!(mutation_lookup);
        retire!(pin_refs);
        retire!(edit_lookup);
        retire!(fold);
        if self.target==PersistedDocumentHydrationTarget::Envelope&&self.actor.0.has_owner(){if !owned_frame_funded::<semio_framework_value::SharedUtf8>(grant){return PersistedDocumentHydrationStep::Pending(self.progress())}let actor=std::mem::take(&mut self.actor.0);if let Err(error)=self.admit_active(actor,cx,grant){return self.reject(error)}return PersistedDocumentHydrationStep::Pending(self.progress())}
        self.phase = Phase::RetirePack;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🗑 Retires the pack.
    pub(super) fn step_retire_pack(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        if self.pack.is_some() {
            let step = owned_outcome(cx, grant, &mut self.pack, &mut self.active);
            return self.admitted_step(step.result, cx);
        }
        self.phase = if self.target == PersistedDocumentHydrationTarget::Envelope { Phase::CloseEnvelopeRuntime } else { Phase::Finish };
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🔒 Closes the envelope runtime.
    pub(super) fn step_close_envelope_runtime(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        if self.runtime.is_none(){self.phase=Phase::CloseEnvelopeOwners;cx.consume_fuel(1);return PersistedDocumentHydrationStep::Pending(self.progress())}
        let runtime = self.runtime.as_mut().expect("envelope hydration runtime remains retained");
        let owners = self.owners.as_ref().expect("envelope hydration owner catalog remains retained");
        match drive_hydration_runtime(runtime, &owners.initial_snapshot_retirement, cx, grant) {
            Ok(true) => {
                self.runtime.take();
                self.phase = Phase::CloseEnvelopeOwners;
            }
            Ok(false) => {}
            Err(error) => return self.reject(error),
        }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🔐 Closes the envelope owners.
    pub(super) fn step_close_envelope_owners(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let owners = self.owners.as_mut().expect("envelope hydration owner catalog remains retained");
        let child = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        let (step, performed) = match cx.with_original_effect(child, |_, effect| owners.close_uninstalled_owners_step(effect)) {
            Ok(Some((Ok(step), performed))) => (step, performed),
            Ok(Some((Err(_), _))) | Err(_) => return self.reject(MemberOpenDiagnostic::Initialization),
            Ok(None) => return PersistedDocumentHydrationStep::Pending(self.progress()),
        };
        if semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "hydration envelope owner catalog").is_err() { return self.reject(MemberOpenDiagnostic::Initialization); }
        match step {
            RetainedCloneStep::Complete(_) => {
                self.owners.take();
                let envelope = self.envelope.take().expect("hydrated envelope remains retained until owner-catalog close");
                assert!(self.runtime.is_none(), "envelope hydration runtime closes before handoff");
                self.terminal = true;
                return PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Envelope(envelope));
            }
            RetainedCloneStep::Progress(progress) => if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step_performed(cx, progress, performed) { return self.reject(diagnostic); },
        }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🏆 Finishes the hydration.
    pub(super) fn step_finish(&mut self) -> PersistedDocumentHydrationStep<P, M> {
        let envelope = self.envelope.take().expect("hydrated envelope remains retained");
        let output = match self.target {
            PersistedDocumentHydrationTarget::Store { generation } => {
                let mut runtime = self.runtime.take().expect("hydrated runtime remains retained");
                let cursor = envelope.cursor.as_ref().expect("persisted cursor remains retained");
                runtime.set_current_checkpoint_id(cursor.checkpoint_id.clone());
                let owners = self.owners.take().expect("store hydration owners remain retained");
                PersistedDocumentHydrationOutput::Store(Box::new(ArtifactStore::from_initialized_runtime_with_owners(envelope, runtime, generation, owners)))
            }
            PersistedDocumentHydrationTarget::Envelope => unreachable!("envelope hydration hands off while closing its uninstalled owner catalog"),
        };
        self.terminal = true;
        PersistedDocumentHydrationStep::Ready(output)
    }
}
