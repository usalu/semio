//! 🚶️ Hydration step: the single step of the retained persisted document hydration through its phases. See [`super`].

use super::*;

#[path = "📦️pack-scan-and-fold/🦀️.rs"]
mod pack_scan_and_fold;

#[path = "✉️edit-decoding/🦀️.rs"]
mod edit_decoding;

#[path = "💧️change-hydration/🦀️.rs"]
mod change_hydration;

#[path = "🔁️replay-and-targets/🦀️.rs"]
mod replay_and_targets;

#[path = "🏁️retirement-and-finish/🦀️.rs"]
mod retirement_and_finish;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static + semio_framework_os_spr::ColdMutation<P>,
{
    pub fn step(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let grant = semio_framework_os_store_member_open::member_step_grant(cx, grant);
        if let Some(diagnostic) = self.diagnostic {
            return PersistedDocumentHydrationStep::Rejected(diagnostic);
        }
        if let Err(diagnostic) = self.check_authority(cx) {
            return self.reject(diagnostic);
        }
        match self.drive_active(cx, grant) {
            Ok(true) => return PersistedDocumentHydrationStep::Pending(self.progress()),
            Ok(false) => {}
            Err(diagnostic) => return self.reject(diagnostic),
        }
        if cx.should_yield() {
            return PersistedDocumentHydrationStep::Pending(self.progress());
        }
        match self.phase {
            Phase::ScanPack => self.step_scan_pack(cx),
            Phase::DecodePack => self.step_decode_pack(cx),
            Phase::Begin => self.step_begin(cx),
            Phase::Fold => self.step_fold(cx, grant),
            Phase::AdmitInitializationCatalog => self.step_admit_initialization_catalog(cx, grant),
            Phase::BindGenesis => self.step_bind_genesis(cx, grant),
            Phase::AdmitEnvelope => self.step_admit_envelope(cx, grant),
            Phase::BuildAppliedCursor | Phase::BuildRedoCursor => self.step_build_applied_cursor(cx),
            Phase::BeginEdit => self.step_begin_edit(cx),
            Phase::DecodeForward | Phase::DecodeInverse => self.step_decode_forward(cx),
            Phase::DecodeMetadata => self.step_decode_metadata(cx),
            Phase::FinishEdit => self.step_finish_edit(cx),
            Phase::HydrateChanges => self.step_hydrate_changes(cx),
            Phase::HydrateCheckpoints => self.step_hydrate_checkpoints(cx),
            Phase::HydrateAlternatives => self.step_hydrate_alternatives(cx),
            Phase::HydratePins => self.step_hydrate_pins(cx, grant),
            Phase::PrepareReplay => self.step_prepare_replay(cx),
            Phase::ReplayCursor => self.step_replay_cursor(cx),
            Phase::AdoptInverse => self.step_adopt_inverse(cx),
            Phase::AdoptMessages => self.step_adopt_messages(cx, grant),
            Phase::SettleReplay => self.step_settle_replay(cx),
            Phase::BeginTargets => self.step_begin_targets(cx, grant),
            Phase::DecodeTarget => self.step_decode_target(cx, grant),
            Phase::TargetInputs => self.step_target_inputs(cx, grant),
            Phase::SeedApplied | Phase::SeedRedo => self.step_seed_applied(cx),
            Phase::RetireHistory => self.step_retire_history(cx, grant),
            Phase::RetirePack => self.step_retire_pack(cx, grant),
            Phase::CloseEnvelopeRuntime => self.step_close_envelope_runtime(cx, grant),
            Phase::CloseEnvelopeOwners => self.step_close_envelope_owners(cx, grant),
            Phase::Finish => self.step_finish(),
            Phase::Rejected => PersistedDocumentHydrationStep::Rejected(self.diagnostic.unwrap_or(MemberOpenDiagnostic::Stale)),
        }
    }
}
