//! 🏁️ Retirement and finish phases: the history and pack retirement, the envelope close and the finish of the hydration step. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static + semio_framework_os_spr::ColdMutation<P>,
{
    /// 🧹 Retires the history runtime.
    pub(super) fn step_retire_history(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        macro_rules! retire {
            ($field:ident) => { if self.$field.is_some() { let step = semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.$field, &mut self.active, RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant }); return self.admitted_step(step, cx); } };
        }
        retire!(history);
        retire!(mutation_lookup);
        retire!(pin_refs);
        retire!(edit_lookup);
        retire!(fold);
        self.phase = Phase::RetirePack;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🗑 Retires the pack.
    pub(super) fn step_retire_pack(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        if self.pack.is_some() {
            let step = semio_framework_os_store_read::artifact_retirement_admit_owned(&mut self.pack, &mut self.active, RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant });
            return self.admitted_step(step, cx);
        }
        self.phase = if self.target == PersistedDocumentHydrationTarget::Envelope { Phase::CloseEnvelopeRuntime } else { Phase::Finish };
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🔒 Closes the envelope runtime.
    pub(super) fn step_close_envelope_runtime(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
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
        let Ok(step) = owners.close_uninstalled_owners_step(child) else { return self.reject(MemberOpenDiagnostic::Initialization) };
        if semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owners.uninstalled_owners_terminal_is_empty(), "hydration envelope owner catalog").is_err() { return self.reject(MemberOpenDiagnostic::Initialization); }
        match step {
            RetainedCloneStep::Complete(_) => {
                self.owners.take();
                let envelope = self.envelope.take().expect("hydrated envelope remains retained until owner-catalog close");
                assert!(self.runtime.is_none(), "envelope hydration runtime closes before handoff");
                self.terminal = true;
                return PersistedDocumentHydrationStep::Ready(PersistedDocumentHydrationOutput::Envelope(envelope));
            }
            RetainedCloneStep::Progress(progress) => if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); },
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
