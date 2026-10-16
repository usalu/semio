//! 🛣️ The authenticated inference route handlers and their contexts.

use super::*;
use semio_hub_inference_runtime_contract::{GisMapApprovalIngressAuthorityV1, InferenceMapBaseV1, InferenceOperationAdmissionV1, InferenceProposalV1, InferenceRouteErrorV1, WORK_UNIT_LIMIT, approval_mutation_id, reader};

//#region 🛣️Routes
/// 🛣️ Everything one authenticated inference route call may read; no ambient authority exists.
pub struct InferenceRouteContextV1<'a> {
    pub runtime: &'a Arc<HubInferenceRuntimeV1>,
    pub directory: &'a Arc<semio_hub_directory_dispatch::HubDirectories>,
    pub rebootstrap: &'a Arc<semio_hub_lag_rebootstrap::VerifiedRebootstrapSource>,
    pub scope: DocumentScope,
    pub token: Option<&'a str>,
    pub now_ms: u64,
    pub document_write: Arc<tokio::sync::Mutex<()>>,
}

/// 🛡️ Approval-only route context retaining Hub admission before document-write acquisition.
pub struct InferenceApprovalRouteContextV1<'a> {
    pub route: InferenceRouteContextV1<'a>,
    pub ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
}

async fn authenticated_session(directory: &Arc<semio_hub_directory_dispatch::HubDirectories>, token: Option<&str>) -> Result<semio_hub_directory_contract::model::AuthSessionRecord, InferenceRouteErrorV1> {
    let Some(semio_hub_directory_contract::HubCapability::Session(capability)) = token.and_then(|value| semio_hub_directory_contract::HubCapability::parse(value).ok()) else {
        return Err(InferenceRouteErrorV1::Denied);
    };
    directory.authenticate_session(&capability).await.map_err(|_| InferenceRouteErrorV1::Storage)?.ok_or(InferenceRouteErrorV1::Denied)
}

/// 🗺️ Materializes the current Map base through the verified active checkpoint pair reader alone.
///
/// The retained `MemberFactory::Open` opener is the intended long-term materializer; it is not
/// native-accepted yet, so this boundary reads the authority-verified pack of the active checkpoint
/// and never accepts a client-supplied Map pack.
pub async fn map_base(rebootstrap: &Arc<semio_hub_lag_rebootstrap::VerifiedRebootstrapSource>, scope: &DocumentScope, deadline_ms: u64, control: &InferenceMapBaseControlV1) -> Result<InferenceMapBaseV1, InferenceRouteErrorV1> {
    let context = semio_hub_lag_rebootstrap::RebootstrapContext::new(deadline_ms, control);
    let pair = rebootstrap.active_pair(scope, &context).await.map_err(|_| InferenceRouteErrorV1::Unavailable)?;
    let selection = &pair.selection;
    Ok(InferenceMapBaseV1 {
        frontier: selection.baseline_frontier.clone(),
        descriptor_digest: directory::os_directory::io::binary::artifact_hash::hex_lower(&selection.descriptor_digest_v1.0),
        pack: InferencePrivateBytesV1::new(pair.pair().pack.clone(), semio_hub_inference::schema::INPUT_MAX_BYTES)?,
    })
}

/// ⏱️ Bounded transfer control for one base materialization; it never reports outside its deadline.
pub struct InferenceMapBaseControlV1 {
    pub deadline_ms: u64,
    pub now_ms: u64,
    pub control: Arc<InferenceOperationControlV1>,
}

impl semio_hub_lag_rebootstrap::RebootstrapTransferControl for InferenceMapBaseControlV1 {
    fn now_ms(&self) -> u64 {
        self.now_ms
    }

    fn is_cancelled(&self) -> bool {
        self.control.checkpoint(self.control.progress().0).is_err()
    }

    fn report(&self, _progress: semio_hub_lag_rebootstrap::RebootstrapProgress) {}
}

async fn finish_retained_gis_map_job(owner: RetainedInferenceRunOwnerV1, outcome: Result<InferenceProposalV1, InferenceRouteErrorV1>) {
    let Ok(now_ms) = owner.runtime.fresh_now_ms() else {
        return;
    };
    let _guard = owner.gate.lock().await;
    match outcome {
        Ok(proposal) => {
            let checked = semio_hub_inference_catalog::authorization::check_live_inference_author(&owner.directory, &owner.identity, &owner.scope, || i64::try_from(now_ms).unwrap_or(i64::MAX), &owner.control).await.map_err(InferenceRouteErrorV1::from);
            let checked = match checked {
                Ok(()) => {
                    let base_control = InferenceMapBaseControlV1 { deadline_ms: owner.expires_at_ms, now_ms, control: owner.control.clone() };
                    map_base(&owner.rebootstrap, &owner.scope, base_control.deadline_ms, &base_control).await.and_then(|current| owner.runtime.compare_frozen(&owner.identity, &owner.scope, &current))
                }
                Err(error) => Err(error),
            };
            if checked.is_ok() {
                let _ = owner.runtime.ledger().succeed(&owner.job_id, &owner.identity, owner.claim.run_epoch, &proposal.result, &proposal.proposal, now_ms);
            } else {
                let _ = owner.runtime.ledger().cancel_run(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms);
            }
        }
        Err(InferenceRouteErrorV1::Cancelled | InferenceRouteErrorV1::Expired) => {
            let _ = owner.runtime.ledger().cancel_run(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms);
        }
        Err(_) => {
            let _ = owner.runtime.ledger().fail_run(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms);
        }
    }
}

async fn drive_retained_gis_map_job(owner: RetainedInferenceRunOwnerV1, ready: oneshot::Receiver<()>) {
    if ready.await.is_err() {
        return;
    }
    let terminal_runtime = owner.runtime.clone();
    let terminal_identity = owner.identity.clone();
    let terminal_job_id = owner.job_id.clone();
    let terminal_claim = owner.claim;
    let terminal_control = owner.control.clone();
    let terminal_gate = owner.gate.clone();
    let computed = tokio::task::spawn_blocking(move || {
        let mut appended = 0_u64;
        let mut last = 0_u64;
        let outcome = owner.runtime.infer(&owner.identity, &owner.job_id, &owner.base, &owner.control, &mut |completed, total| {
            let now_ms = owner.runtime.fresh_now_ms()?;
            if completed > last && appended < semio_hub_inference::schema::PROGRESS_MAX_CURSOR {
                let _progress_cursor = owner.runtime.ledger().heartbeat(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, completed, total, now_ms)?;
                appended += 1;
                last = completed;
                #[cfg(feature = "integration-fixtures")]
                if let Some(gate) = owner.runtime.checkpoint_test_gate.lock().ok().and_then(|selected| selected.clone()) {
                    gate.checkpoint(&owner.control, &owner.job_id, _progress_cursor, completed, total)?;
                }
            } else {
                owner.runtime.ledger().renew_claim(&owner.job_id, &reader(&owner.identity), owner.claim.run_epoch, now_ms)?;
            }
            Ok(())
        });
        (owner, outcome)
    })
    .await;
    match computed {
        Ok((owner, outcome)) => finish_retained_gis_map_job(owner, outcome).await,
        Err(_) => {
            let Ok(now_ms) = terminal_runtime.fresh_now_ms() else {
                return;
            };
            let _guard = terminal_gate.lock().await;
            if terminal_control.checkpoint(terminal_control.progress().0).is_err() {
                let _ = terminal_runtime.ledger().cancel_run(&terminal_job_id, &reader(&terminal_identity), terminal_claim.run_epoch, now_ms);
            } else {
                let _ = terminal_runtime.ledger().fail_run(&terminal_job_id, &reader(&terminal_identity), terminal_claim.run_epoch, now_ms);
            }
        }
    }
}

/// 📥️ Accepts one closed client intent, claims it, runs the frozen service, and offers one proposal.
pub async fn submit_gis_map_job(context: InferenceRouteContextV1<'_>, body: &[u8]) -> Result<InferenceJobReceiptV1, InferenceRouteErrorV1> {
    let request = semio_hub_inference::schema::InferenceRequestV1::decode(body)?;
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let control = Arc::new(InferenceOperationControlV1::new(request.lifetime_ms, WORK_UNIT_LIMIT)?);
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control: control.clone() };
    let (receipt, identity, admission) = {
        let _guard = gate.lock().await;
        let descriptor = context.directory.get_document_descriptor(&context.scope).await.map_err(|_| InferenceRouteErrorV1::Storage)?.ok_or(InferenceRouteErrorV1::NotFound)?;
        let base = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
        let identity = semio_hub_inference_catalog::identity_from_frozen_binding(
            runtime.binding(),
            semio_hub_inference_catalog::InferenceIdentitySourceV1 {
                request,
                scope: &context.scope,
                descriptor: &descriptor,
                session: &session,
                frontier: &base.frontier,
                materialized_input: &base.pack,
                now_ms: i64::try_from(context.now_ms).map_err(|_| InferenceRouteErrorV1::Bounds)?,
            },
            &control,
        )
        .await?;
        runtime.compare_frozen(&identity, &context.scope, &base)?;
        semio_hub_inference_catalog::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
        let receipt = runtime.ledger().accept(&identity, &base.pack, context.now_ms)?;
        let runtime_owner = runtime.clone();
        let directory = context.directory.clone();
        let rebootstrap = context.rebootstrap.clone();
        let owner_gate = gate.clone();
        let scope = context.scope.clone();
        let job_id = receipt.job_id.clone();
        let expires_at_ms = receipt.expires_at_ms;
        let owner_identity = identity.clone();
        let owner_control = control.clone();
        let admission = runtime.install_operation(&receipt, &identity, context.now_ms, control.clone(), move |claim, ready, permit| {
            let owner = RetainedInferenceRunOwnerV1 { runtime: runtime_owner, directory, rebootstrap, gate: owner_gate, scope, identity: owner_identity, job_id, expires_at_ms, claim, base, control: owner_control, _permit: permit };
            async move { drive_retained_gis_map_job(owner, ready).await }
        });
        let admission = match admission {
            Ok(admission) => admission,
            Err(error) => {
                let now_ms = runtime.fresh_now_ms().unwrap_or(context.now_ms);
                let _ = runtime.ledger().fail(&receipt.job_id, &reader(&identity), now_ms);
                return Err(error);
            }
        };
        (receipt, identity, admission)
    };
    let now_ms = runtime.fresh_now_ms()?;
    let response = owner_page_receipt(runtime, &receipt.job_id, &identity, receipt.expires_at_ms, now_ms);
    match admission {
        InferenceOperationAdmissionV1::Installed(start) => {
            if response.is_err() {
                control.cancel();
            }
            let _ = start.send(());
        }
        InferenceOperationAdmissionV1::Existing => {}
        InferenceOperationAdmissionV1::Unclaimed => {
            if response.as_ref().is_ok_and(|receipt| receipt.state == semio_hub_inference::schema::InferenceJobStateV1::Running) {
                return Err(InferenceRouteErrorV1::Unavailable);
            }
        }
    }
    response
}

/// 🧭 Reconciles one previously submitted request without accepting or restarting work.
pub async fn reconcile_gis_map_job(context: InferenceRouteContextV1<'_>, body: &[u8]) -> Result<InferenceJobReconcileResultV1, InferenceRouteErrorV1> {
    let request = semio_hub_inference::schema::decode_inference_job_reconcile_request_v1(body)?;
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let reader = session_reader(&session, &context.scope);
    let job = runtime.ledger().reconcile_request(&request.request_id, &reader, context.now_ms)?;
    semio_hub_inference_catalog::authorization::check_live_inference_session_author(context.directory, &session.id, &session.user_id, session.authorization_generation, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    Ok(InferenceJobReconcileResultV1 { schema: "semio.framework.job-reconcile-result/v1", version: 1, request_id: request.request_id, found: job.is_some(), job })
}

fn owner_page_receipt(runtime: &Arc<HubInferenceRuntimeV1>, job_id: &str, identity: &InferenceIdentityV1, expires_at_ms: u64, now_ms: u64) -> Result<InferenceJobReceiptV1, InferenceRouteErrorV1> {
    let page = runtime.ledger().events(job_id, &reader(identity), 0, now_ms)?;
    Ok(InferenceJobReceiptV1 { schema: "semio.hub.inference-job-receipt/v1", job_id: job_id.to_owned(), state: page.state, proposal_state: page.proposal_state, proposal_hash: page.proposal_hash, cursor: page.next_cursor, expires_at_ms })
}

/// 📤️ Returns the owner-private bounded event page and marks a drifted offer stale to its owner.
pub async fn read_gis_map_job_events(context: InferenceRouteContextV1<'_>, job_id: &str, after: u64) -> Result<InferenceEventPageV1, InferenceRouteErrorV1> {
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let identity = runtime.ledger().identity_of(job_id, &session_reader(&session, &context.scope))?;
    semio_hub_inference_catalog::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    let page = runtime.ledger().events(job_id, &reader(&identity), after, context.now_ms)?;
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control };
    let current = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
    let stale = runtime.compare_frozen(&identity, &context.scope, &current).is_err();
    let preview = if !stale && !page.cancel_requested && page.state == semio_hub_inference::schema::InferenceJobStateV1::Succeeded && page.proposal_state == semio_hub_inference::schema::InferenceProposalStateV1::Offered {
        let proposal_hash = page.proposal_hash.as_deref().ok_or(InferenceRouteErrorV1::Conflict)?;
        let view = runtime.ledger().read(job_id, &reader(&identity), context.now_ms)?;
        Some(gis_map_inference_preview(runtime.proposals().as_ref(), job_id, proposal_hash, view.proposal.as_slice())?)
    } else {
        None
    };
    Ok(InferenceEventPageV1 {
        schema: "semio.hub.inference-job-events/v1",
        job_id: job_id.to_owned(),
        state: page.state,
        proposal_state: page.proposal_state,
        cancel_requested: page.cancel_requested,
        stale,
        proposal_hash: page.proposal_hash,
        preview,
        events: page.events.into_iter().map(|row| InferenceEventV1 { ordinal: row.ordinal, kind: row.kind, at_ms: row.at_ms }).collect(),
        progress: page.progress.into_iter().map(|row| InferenceProgressV1 { cursor: row.cursor, run_epoch: row.run_epoch, completed: row.completed, total: row.total, at_ms: row.at_ms }).collect(),
        next_cursor: page.next_cursor,
    })
}

/// 🛑️ Records the owner's durable cancel request and interrupts its retained bounded work.
pub async fn cancel_gis_map_job(context: InferenceRouteContextV1<'_>, job_id: &str) -> Result<InferenceEventPageV1, InferenceRouteErrorV1> {
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let identity = runtime.ledger().identity_of(job_id, &session_reader(&session, &context.scope))?;
    semio_hub_inference_catalog::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    runtime.interrupt_operation(job_id, &identity.digest()?)?;
    runtime.ledger().request_cancel(job_id, &reader(&identity), context.now_ms)?;
    let page = runtime.ledger().events(job_id, &reader(&identity), 0, context.now_ms)?;
    Ok(InferenceEventPageV1 {
        schema: "semio.hub.inference-job-events/v1",
        job_id: job_id.to_owned(),
        state: page.state,
        proposal_state: page.proposal_state,
        cancel_requested: page.cancel_requested,
        stale: false,
        proposal_hash: page.proposal_hash,
        preview: None,
        events: page.events.into_iter().map(|row| InferenceEventV1 { ordinal: row.ordinal, kind: row.kind, at_ms: row.at_ms }).collect(),
        progress: page.progress.into_iter().map(|row| InferenceProgressV1 { cursor: row.cursor, run_epoch: row.run_epoch, completed: row.completed, total: row.total, at_ms: row.at_ms }).collect(),
        next_cursor: page.next_cursor,
    })
}

/// ✅️ Explicit approval: rebuilds the typed effect server-side and hands it to the composition port.
/// A retry of an approval whose outbox is still prepared carries that exact durable command, so every
/// duplicate reaches the same boundary instead of a fresh wall-clock stamp conflicting with it.
pub async fn approve_gis_map_job(context: InferenceApprovalRouteContextV1<'_>, job_id: &str, body: &[u8]) -> Result<InferenceApprovalReceiptV1, InferenceRouteErrorV1> {
    let InferenceApprovalRouteContextV1 { route: context, ingress } = context;
    let approval = semio_hub_inference::schema::InferenceApprovalRequestV1::decode(body)?;
    if approval.job_id != job_id {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    runtime.reap_finished_operations().await?;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let control = Arc::new(InferenceOperationControlV1::new(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    let identity = runtime.ledger().identity_of(&approval.job_id, &session_reader(&session, &context.scope))?;
    semio_hub_inference_catalog::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    let view = runtime.ledger().read(&approval.job_id, &reader(&identity), context.now_ms)?;
    if view.proposal.as_slice().is_empty() || sha256(view.proposal.as_slice()) != approval.proposal_hash {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control };
    let base = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
    runtime.compare_frozen(&identity, &context.scope, &base)?;
    let command = match runtime.ledger().approval_recovery_by_mutation(&approval_mutation_id(&approval.job_id, &approval.proposal_hash))? {
        Some((outbox, _, false)) if outbox.job_id == approval.job_id && outbox.proposal_hash == approval.proposal_hash => outbox.command,
        _ => {
            let document_clock = runtime.committer().document_clock(&context.scope).await;
            runtime.server_stamped_command(&identity, &approval.job_id, &base, &approval.proposal_hash, document_clock, context.now_ms)?
        }
    };
    let prepared = runtime.ledger().prepare_approval(&approval.job_id, &identity, &approval.proposal_hash, &command, context.now_ms)?;
    let committed = runtime.commit_approval(&identity, &approval.job_id, &approval.proposal_hash, &command, &base, base_control.deadline_ms, context.now_ms, context.document_write, ingress).await?;
    Ok(InferenceApprovalReceiptV1 {
        schema: "semio.hub.inference-approval-receipt/v1",
        job_id: approval.job_id,
        mutation_id: prepared.mutation_id,
        command_hash: prepared.command_hash,
        proposal_hash: prepared.proposal_hash,
        applied: committed.applied,
        undo: committed.undo,
    })
}

/// ↩️ Applies the server-retained fixed-three inverse against the exact current durable tail.
pub async fn undo_gis_map_approval(context: InferenceApprovalRouteContextV1<'_>, body: &[u8]) -> Result<semio_hub_inference::schema::GisMapApprovalUndoReceiptV1, InferenceRouteErrorV1> {
    let InferenceApprovalRouteContextV1 { route: context, ingress } = context;
    if body.is_empty() || body.len() > semio_hub_inference::schema::REQUEST_MAX_BYTES {
        return Err(InferenceRouteErrorV1::Bounds);
    }
    let request: semio_hub_inference::schema::GisMapApprovalUndoRequestV1 = serde_json::from_slice(body).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    if !request.validate() || request.expected_current.document_id != context.scope.document_id {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    let session = authenticated_session(context.directory, context.token).await?;
    let runtime = context.runtime;
    let gate = runtime.document_gate(&context.scope)?;
    let _guard = gate.lock().await;
    let reader = session_reader(&session, &context.scope);
    let original_job_id = runtime.ledger().gis_map_approval_undo_job_id(&request.target_id, &reader)?;
    let identity = runtime.ledger().identity_of(&original_job_id, &reader)?;
    let control = Arc::new(InferenceOperationControlV1::new(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS, WORK_UNIT_LIMIT)?);
    semio_hub_inference_catalog::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &control).await?;
    if let Some(receipt) = runtime.ledger().replayed_gis_map_approval_undo(&request.target_id, &request.idempotency_key, &reader)? {
        return Ok(receipt);
    }
    let target = runtime.ledger().gis_map_approval_undo_target(&request.target_id, &reader)?;
    if target.after_frontier != request.expected_current || identity.descriptor_digest.is_empty() || target.descriptor_digest != identity.descriptor_digest {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let base_control = InferenceMapBaseControlV1 { deadline_ms: context.now_ms.saturating_add(semio_hub_inference::schema::JOB_MAX_LIFETIME_MS), now_ms: context.now_ms, control };
    let base = map_base(context.rebootstrap, &context.scope, base_control.deadline_ms, &base_control).await?;
    let current = directory::os_directory::EditedArtifactFrontierV1 {
        document_id: base.frontier.document_id.clone(),
        head_edit_ordinal: base.frontier.head_edit_ordinal,
        head_edit_id: base.frontier.head_edit_id.clone(),
        last_commit_seq: base.frontier.last_commit_seq,
        chain_sha256: directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&base.frontier.chain_hash),
    };
    if current != request.expected_current || base.descriptor_digest != identity.descriptor_digest || base.digest() != target.after_base_digest {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    semio_hub_inference_catalog::authorization::check_live_inference_author(context.directory, &identity, &context.scope, || i64::try_from(context.now_ms).unwrap_or(i64::MAX), &base_control.control).await?;
    let document_clock = runtime.committer().document_clock(&context.scope).await;
    let prepared = runtime.server_stamped_undo_command(&target, &request.idempotency_key, &base, document_clock, context.now_ms)?;
    match runtime.ledger().prepare_gis_map_approval_undo(&target, &request.idempotency_key, &prepared.operation_id, &prepared.proposal_hash, &prepared.mutation_id, &prepared.command_hash, &prepared.command)? {
        semio_hub_inference_ledger::GisMapApprovalUndoAdmissionV1::Replayed(receipt) => return Ok(receipt),
        semio_hub_inference_ledger::GisMapApprovalUndoAdmissionV1::Prepared => {}
    }
    let committed = runtime.commit_undo(&target, &request.idempotency_key, &prepared, &base, base_control.deadline_ms, context.now_ms, context.document_write, ingress).await?;
    Ok(semio_hub_inference::schema::GisMapApprovalUndoReceiptV1 {
        schema: "semio.hub.gis-map-approval-undo-receipt/v1".into(),
        target_id: target.target_id,
        original_job_id: target.original_job_id,
        mutation_id: prepared.mutation_id,
        command_hash: prepared.command_hash,
        applied: committed.applied,
        replayed: false,
        frontier: committed.frontier,
    })
}

/// 🖼️ Publishes the owner preview of one offered proposal: the exact hash, then the registered binding's region.
pub fn gis_map_inference_preview(proposals: &dyn MapInferenceProposalBindingV1, job_id: &str, proposal_hash: &str, proposal: &[u8]) -> Result<GisMapInferencePreviewV1, InferenceRouteErrorV1> {
    if sha256(proposal) != proposal_hash {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    let preview = proposals.preview(job_id, proposal)?;
    Ok(GisMapInferencePreviewV1 { schema: "semio.hub.gis-map-inference-preview/v1", job_id: job_id.to_owned(), proposal_hash: proposal_hash.to_owned(), region_id: preview.region_id, ring: preview.ring })
}

fn session_reader<'a>(session: &'a semio_hub_directory_contract::model::AuthSessionRecord, scope: &'a DocumentScope) -> InferenceReaderV1<'a> {
    InferenceReaderV1 { user_id: &session.user_id, session_id: &session.id, authorization_generation: session.authorization_generation, space_id: &scope.space_id, document_id: &scope.document_id }
}
//#endregion 🛣️Routes
