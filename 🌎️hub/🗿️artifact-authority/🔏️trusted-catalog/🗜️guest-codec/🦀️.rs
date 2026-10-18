//! 🗜️ Guest codec calls: a verified package's lazily compiled actor, its pinned codec rows and the off-worker interpreter with cancellation.

use super::*;

/// 🗜️ One verified package's actor, compiled when a document operation needs it and kept within the
/// catalog's residency budget ([`GuestResidencyLedgerV1`]). Its codec rows that neither a linked native
/// codec nor this engine's verification memory pinned at load (`rows`) are pinned against the component's
/// own `pack-schema-hash` once, before its first codec call — by the catalog's background verification or
/// by that call, whichever comes first — so a hub serves without interpreting its whole catalog first and
/// no codec call ever runs on a row its component did not answer as the trust record says.
pub(super) struct GuestArtifactComponent {
    pub(super) runtime: Arc<semio_framework_plugin_interpreter_runtime::OwnedRuntime>,
    pub(super) package: PackageRef,
    pub(super) component: TrustedCatalogAsset,
    pub(super) compiled: GuestResidencyV1<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>,
    pub(super) position: usize,
    pub(super) component_sha256: [u8; 32],
    pub(super) rows: Vec<(String, [u8; 32])>,
    pub(super) verified: tokio::sync::OnceCell<Result<(), String>>,
    pub(super) verifications: Arc<GuestCodecVerificationCacheV1>,
    pub(super) progress: Arc<TrustedCatalogLoadProgressCellV1>,
}

impl GuestArtifactComponent {
    async fn compiled(&self, context: &OperationContext<'_>) -> Result<Arc<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>, AuthorityError> {
        self.compiled
            .acquire(context, || async {
                context.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCompiling, completed_units: 0, total_units: 1 })?;
                let bytes = self.component.read(context).await?;
                let runtime = Arc::clone(&self.runtime);
                let compiled = interpret_off_worker(context, move |_handle, _progress, _cancellation| runtime.compile_component(&bytes).map_err(semio_framework_plugin_guest_contract::TurnFault::Host)).await?.map_err(|error| catalog_error(format!("{}: {error}", self.package.package.0)))?;
                context.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCompiling, completed_units: 1, total_units: 1 })?;
                Ok(compiled)
            })
            .await
    }

    /// 🔐️ Every row of `rows` pinned against this component's own answer, once for the catalog's lifetime. A
    /// caller that cancels or stalls leaves the rows for the next caller; a component that answers a row
    /// differently from its trust record, or does not compile, refuses every codec call of its package.
    pub(super) async fn verified(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        if self.rows.is_empty() {
            return Ok(());
        }
        let outcome = self.verified.get_or_try_init(|| self.verify_once(context)).await?;
        outcome.clone().map_err(AuthorityError::Catalog)
    }

    /// 🔐️ One verification attempt: its phase in the catalog progress, and its terminal outcome — `Ok(Err(..))`
    /// is a refusal the package keeps, `Err` a cancellation or stall the next caller retries.
    async fn verify_once(&self, context: &OperationContext<'_>) -> Result<Result<(), String>, AuthorityError> {
        self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Verifying);
        match self.verify_rows(context).await {
            Ok(()) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Ready);
                Ok(Ok(()))
            }
            Err(error @ (AuthorityError::Cancelled | AuthorityError::Stalled | AuthorityError::DeadlineExceeded)) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Staged);
                Err(error)
            }
            Err(AuthorityError::Catalog(refusal)) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Refused);
                Ok(Err(refusal))
            }
            Err(error) => {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Refused);
                Ok(Err(error.to_string()))
            }
        }
    }

    /// 🔐️ Interprets every pending row on one compile, several at once ([`guest_verification_concurrency`]).
    async fn verify_rows(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        use futures::StreamExt;
        let schemas = self.rows.iter().map(|(schema, _)| schema.as_str()).collect::<Vec<_>>().join(", ");
        let compiled = self.compiled(context).await.map_err(|error| catalog_error(format!("{schemas}: {error}")))?;
        let pending: Vec<_> = self.rows.iter().map(|(schema, expected)| self.verify_row(Arc::clone(&compiled), schema, *expected, context)).collect();
        let mut running = futures::stream::iter(pending).buffer_unordered(guest_verification_concurrency());
        let mut verified = Ok(());
        while let Some(row) = running.next().await {
            if let Err(error) = row {
                verified = Err(error);
                break;
            }
        }
        drop(running);
        self.compiled.remeasure(&compiled, context).await;
        verified
    }

    /// 🔐️ One row: the component's own `pack-schema-hash`, compared with the trust record and remembered for this engine.
    async fn verify_row(&self, compiled: Arc<semio_framework_plugin_interpreter_runtime::OwnedCompiledGuest>, schema: &str, expected: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let (runtime, row_schema) = (Arc::clone(&self.runtime), schema.to_string());
        let observed = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_pack_schema_hash_observed(&compiled, &row_schema, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation)))
            .await?
            .map_err(|error| catalog_error(format!("{schema}: {error}")))?;
        context.checkpoint()?;
        if observed != expected {
            return Err(catalog(&format!("{schema}: guest artifact codec schema hash differs from its trust record")));
        }
        self.verifications.remember(&self.component_sha256, schema, &observed).await;
        self.progress.package(self.position, |package| package.rows_verified = package.rows_verified.saturating_add(1));
        Ok(())
    }
}

/// 🧩️ The component half of one artifact identity: the package's own actor plus the document
/// schema that selects the owning app inside it. Every verified codec carries one, because `genesis`
/// now comes from the component for EVERY package (ticket 26/09/18 slice TC3b) — the compiled-in
/// stdio/gis/vcs genesis table it replaced could only ever answer for three packages.
pub struct GuestArtifactCodecBinding {
    pub component: Arc<GuestArtifactComponent>,
    pub(super) artifact_schema: String,
}

impl GuestArtifactCodecBinding {
    /// 🌱️ The guest's canonical empty document. The call is the longest single thing a creation
    /// does — the interpreter walks a ≈ 48 MB component's whole app bundle — so the guest's own
    /// fuel progress is reported into the caller's context as it happens: under a stall bound a
    /// checkpoint is what says "still moving", and without these an honest interpreter would look
    /// exactly like a wedged one. A caller that stops waiting ends the interpretation at its next
    /// step ([`interpret_off_worker`]).
    pub async fn genesis(&self, document_id: &str, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, document_id, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), document_id.to_string(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_genesis_observed(&guest, &schema, &document_id, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))).await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let pair = answer.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: pair.pack, spr: pair.spr })
    }

    pub(super) async fn print_mirror(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        let (runtime, schema, pack, spr, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), pair.pack.clone(), pair.spr.clone(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_print_mirror_observed(&guest, &schema, &pack, &spr, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))).await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let mirror = answer.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        if mirror.dsl.len().checked_add(mirror.ops.len()).is_none_or(|length| length > AUTHORITY_MAX_CODEC_TEXT_BYTES) {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        Ok(())
    }

    pub(super) async fn apply_ops(&self, pair: ArtifactPair, encoded: Vec<u8>, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| handle.block_on(runtime.codec_apply_ops_observed(&guest, &schema, &pair.pack, &pair.spr, &encoded, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))).await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let next = answer.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: next.pack, spr: next.spr })
    }

    /// 📜️ The guest's own replica fold of a ledger stream; fuel progress reaches the caller's stall
    /// bound exactly as [`Self::genesis`]'s does.
    pub(super) async fn replay_envelopes(&self, pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, envelopes, guest) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), envelopes.to_vec(), Arc::clone(&compiled));
        let answer = interpret_off_worker(context, move |handle, progress, cancellation| {
            handle.block_on(runtime.codec_replay_envelopes_observed(&guest, &schema, &pair.pack, &pair.spr, &envelopes, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel), cancellation))
        })
        .await?;
        self.component.compiled.remeasure(&compiled, context).await;
        let next = answer.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: next.pack, spr: next.spr })
    }
}

/// 🧵️ Interprets one guest codec call on the hub runtime's blocking pool, never on the async worker
/// that awaits it: a call interprets for seconds to minutes, and on a worker it would stall every
/// request, socket and timer queued behind it (a sign-in, a presence heartbeat, an edit's ack). The
/// call's fuel observations reach the caller's context while it runs — each one a checkpoint of its
/// stall bound — and the call observes its caller: once the caller is released — refused at an
/// observation, cancelled (seen within [`GUEST_CALL_CANCELLATION_POLL`] between observations), or
/// dropped mid-await like the background verification a hub shutdown aborts — the interpreter ends
/// the call at its next step, so no interpretation outlives the operation that asked for it and a
/// runtime being dropped waits for none. The outer result is the caller's own outcome, the inner one
/// the guest's answer. Guest codec calls belong to the hub's runtime; outside one they are refused.
pub async fn interpret_off_worker<T, F>(context: &OperationContext<'_>, call: F) -> Result<Result<T, semio_framework_plugin_guest_contract::TurnFault>, AuthorityError>
where
    T: Send + 'static,
    F: FnOnce(&tokio::runtime::Handle, &mut dyn FnMut(u64), &semio_framework_plugin_guest_contract::GuestCallCancellation) -> Result<T, semio_framework_plugin_guest_contract::TurnFault> + Send + 'static,
{
    let handle = tokio::runtime::Handle::try_current().map_err(|_| catalog("guest codec calls run on the hub runtime"))?;
    let (sender, mut observations) = tokio::sync::mpsc::unbounded_channel::<u64>();
    let released = GuestCallRelease(semio_framework_plugin_guest_contract::GuestCallCancellation::default());
    let (blocking, cancellation) = (handle.clone(), released.0.clone());
    let joined = handle.spawn_blocking(move || {
        call(
            &blocking,
            &mut |fuel| {
                let _ = sender.send(fuel);
            },
            &cancellation,
        )
    });
    loop {
        match tokio::time::timeout(GUEST_CALL_CANCELLATION_POLL, observations.recv()).await {
            Ok(Some(fuel)) => context.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: fuel.min(GUEST_CODEC_BUDGET.fuel), total_units: GUEST_CODEC_BUDGET.fuel })?,
            Ok(None) => break,
            Err(_) if context.is_cancelled() => return Err(AuthorityError::Cancelled),
            Err(_) => {}
        }
    }
    Ok(joined.await.unwrap_or_else(|error| Err(semio_framework_plugin_guest_contract::TurnFault::Trapped(format!("guest codec call ended abnormally: {error}")))))
}

/// ⏲️ How often a caller awaiting a guest call between two fuel observations (one per 25 M fuel or 5 s)
/// looks at its own cancellation.
pub const GUEST_CALL_CANCELLATION_POLL: std::time::Duration = std::time::Duration::from_millis(50);

/// 🧯️ The awaiting side of one guest call: however its caller is released — answered, refused, cancelled or
/// dropped mid-await — the call is cancelled, which ends its interpretation at the next step (a call that already
/// returned is unaffected).
struct GuestCallRelease(semio_framework_plugin_guest_contract::GuestCallCancellation);

impl Drop for GuestCallRelease {
    fn drop(&mut self) {
        self.0.cancel();
    }
}
