//! 📦️ Pack scan and fold phases: the pack scan, decode, begin, fold, initialization catalog and genesis binding of the hydration step. See [`super`].

use super::*;

impl<P, M> RetainedPersistedDocumentHydration<P, M>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static + semio_framework_os_spr::ColdMutation<P>,
{
    /// 🔍 Scans the retained pack for the next record.
    pub(super) fn step_scan_pack(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let pack = self.pack.as_ref().expect("persisted Pack remains retained through scan");
        let remaining = pack.len().saturating_sub(self.pack_scanned);
        if remaining == 0 {
            self.phase = Phase::DecodePack;
            cx.consume_fuel(1);
            return PersistedDocumentHydrationStep::Pending(self.progress());
        }
        let scanned = remaining.min(semio_framework_os_store_owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
        if scanned == 0 {
            return PersistedDocumentHydrationStep::Pending(self.progress());
        }
        self.pack_hasher.update(&self.pack.as_ref().expect("retained Pack remains present")[self.pack_scanned..self.pack_scanned + scanned]);
        self.pack_scanned += scanned;
        cx.consume_fuel(scanned as u64);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🧬 Decodes the scanned pack record.
    pub(super) fn step_decode_pack(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        let initial = match P::decode_pack(self.pack.as_ref().expect("scanned Pack remains retained")) {
            Ok(initial) => initial,
            Err(_) => return self.reject(MemberOpenDiagnostic::Decode),
        };
        *self.initial = Some(initial);
        self.phase = Phase::Begin;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🚦 Begins the hydration.
    pub(super) fn step_begin(&mut self, cx: &mut StepContext<'_>) -> PersistedDocumentHydrationStep<P, M> {
        use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};
        if self.actor.0.trim().is_empty() { return self.reject(MemberOpenDiagnostic::Initialization); }
        let history = self.history.as_ref().expect("decoded history remains retained");
        let expected = self.expected.as_ref().expect("document identity remains retained");
        let schema = self.schema.as_ref().expect("document schema remains retained");
        let Some(composition) = history.composition.as_ref() else { return self.reject(MemberOpenDiagnostic::Identity) };
        let dialect_matches = composition.dialect.as_ref().is_some_and(|(artifact_kind, standard, subset)| artifact_kind == &expected.dialect.artifact_kind && standard == &expected.dialect.standard && subset == &expected.dialect.subset);
        let history_owner = match composition.owner.as_ref() {
            Some((parent, slot, child_id)) => match ArtifactRef::parse_uri(parent) {
                Ok(parent) => Some(OwnerRef { parent, slot: slot.clone(), child_id: child_id.clone() }),
                Err(_) => return self.reject(MemberOpenDiagnostic::Owner),
            },
            None => None,
        };
        if history.doc_id != expected.artifact_id || history.schema != *schema || !dialect_matches || history_owner.as_ref() != self.owner.as_ref() {
            return self.reject(MemberOpenDiagnostic::Identity);
        }
        if semio_framework_os_io::publish_native_snapshot::<P>().is_err() { return self.reject(MemberOpenDiagnostic::Initialization); }
        *self.fold_job = Some(semio_framework_os_spr::HistoryLog::fold_job(history.clone(), semio_framework_os_spr::HistoryShape::Document));
        self.phase = Phase::Fold;
        cx.consume_fuel(1);
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🪄 Folds the decoded records into the history.
    pub(super) fn step_fold(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let mut fold_grant = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
                        let body = fold_grant.maximum_copy_bytes;
    let job = self.fold_job.as_mut().expect("history fold job remains retained");
        let required_copy = match job.next_step_grant(1, fold_grant.maximum_copy_bytes) { Ok(required) => required.maximum_copy_bytes, Err(_) => return self.reject(MemberOpenDiagnostic::Replay) };
        fold_grant.maximum_copy_bytes = fold_grant.maximum_copy_bytes.max(required_copy.min(grant.maximum_copy_bytes));
        let result = job.step(fold_grant, body, &mut || cx.should_yield());
        self.fold_completed = job.completed();
        match result {
            Ok(semio_framework_os_spr::HistoryFoldJobStep::Pending { progress, .. }) if progress.fits(fold_grant) => {
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
            }
            Ok(semio_framework_os_spr::HistoryFoldJobStep::Ready { value: (fold, transitions, replay_order, conflicts), progress }) => {
                *self.replay_ids = Some(replay_order);
                *self.fold = Some(fold);
                *self.normalized_transitions = Some(transitions);
                *self.normalized_conflicts = Some(conflicts);
                if !progress.fits(fold_grant) || !self.fold_job.as_ref().is_some_and(ErasedSnapshotRetirement::terminal_is_empty) { return self.reject(MemberOpenDiagnostic::Replay); }
                if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
                self.fold_job.take();
                *self.initialization_catalog = Some(semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog::empty());
                self.phase = Phase::AdmitInitializationCatalog;
            }
            _ => return self.reject(MemberOpenDiagnostic::Replay),
        }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 📇 Admits the initialization catalog.
    pub(super) fn step_admit_initialization_catalog(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        let child = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
        let catalog = self.initialization_catalog.as_mut().expect("initialization retains its original page catalog");
        let progress = match catalog.admit_next(child) { Ok(progress) => progress, Err(_) => return self.reject(MemberOpenDiagnostic::Initialization) };
        if !progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); }
        let ready = catalog.admission_is_complete();
        if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
        if ready { self.phase = Phase::BindGenesis; }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }

    /// 🌱 Binds the genesis snapshot.
    pub(super) fn step_bind_genesis(&mut self, cx: &mut StepContext<'_>, grant: RetainedCloneGrant) -> PersistedDocumentHydrationStep<P, M> {
        if !self.initialization_catalog.as_ref().is_some_and(semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog::admission_is_complete) { return self.reject(MemberOpenDiagnostic::Initialization); }
        let child = match hydration_child_grant(grant, cx.fuel_remaining()) { Ok(child) => child, Err(diagnostic) => return self.reject(diagnostic) };
        let birth = match semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis::<P>::verified_pack_birth_demand() { Ok(birth) => birth, Err(_) => return self.reject(MemberOpenDiagnostic::Capacity) };
        if birth.admit(child).is_err() { return PersistedDocumentHydrationStep::Pending(self.progress()); }
        let history = std::sync::Arc::get_mut(self.history.as_mut().expect("decoded history remains retained")).expect("fold aliases close before genesis binding");
        let fold = self.fold.as_mut().expect("derived history remains retained");
        let initial = self.initial.take().expect("typed initial snapshot remains retained");
        let initial_digest = self.pack_digest.unwrap_or_else(|| *self.pack_hasher.finalize().as_bytes());
        let pack = self.pack.take().expect("stored genesis Pack remains retained");
        let (genesis, progress) = match semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis::admit_verified_pack(initial, pack, initial_digest, child) {
            Ok(admitted) => admitted,
            Err((_, initial, pack)) => { *self.initial = Some(initial); *self.pack = Some(pack); return self.reject(MemberOpenDiagnostic::Initialization); }
        };
        let expected = self.expected.take().expect("document identity remains retained");
        let schema = self.schema.take().expect("document schema remains retained");
        let mut envelope = crate::empty_document_envelope_from_genesis_owners::<P, M>(schema, expected.artifact_id, genesis, None);
        envelope.dialect = Some(expected.dialect);
        envelope.owner = self.owner.take();
        envelope.active_alternative_id = fold.alternative.take();
        envelope.viewer_checkpoint_id = history.viewer_checkpoint.take();
        envelope.cursor.as_mut().expect("new document retains its original cursor pages").checkpoint_id = fold.checkpoint.take();
        envelope.transitions = self.normalized_transitions.take().expect("normalized transition owners remain retained");
        let supersessions = std::mem::take(&mut fold.supersessions);
        envelope.conflicts = self.normalized_conflicts.take().expect("normalized conflict owners remain retained");
        let current = envelope.vcs.genesis.facts().share_snapshot();
        let mut runtime = ArtifactStoreInitializationRuntime::new_with_owner_catalog(&envelope.id, &envelope.schema, current, initial_digest, semio_framework_os_spr::ActorId(std::mem::take(&mut self.actor.0)), self.initialization_catalog.take().expect("fully admitted initialization catalog remains original"));
        let installed = runtime.set_supersessions(supersessions);
        *self.runtime = Some(runtime);
        *self.envelope = Some(envelope);
        if installed.is_err() {
            return self.reject(MemberOpenDiagnostic::Initialization);
        }
        self.phase = Phase::AdmitEnvelope;
        self.operation_index = 0;
        if !progress.fits(child) { return self.reject(MemberOpenDiagnostic::Initialization); }
        if let Err(diagnostic) = semio_framework_os_store_member_open::record_member_step(cx, progress) { return self.reject(diagnostic); }
        PersistedDocumentHydrationStep::Pending(self.progress())
    }
}
