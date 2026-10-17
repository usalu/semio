/// ⚡️ The quick laws module and the non-default proposal binding law.

mod quick {
    use super::*;
use std::sync::Arc;
use directory::os_directory::DocumentScope;
use semio_hub_inference_committer::{GisMapAbandonedRequestTaskV1, GisMapAbandonedRequestV1, GisMapPreparedApprovalTaskV1, GisMapPreparedApprovalV1};
use semio_hub_inference_runtime_contract::{GisMapApprovalCheckpointPublisherV1, GisMapApprovalCommitErrorV1, GisMapApprovalCommitRequestV1, GisMapApprovalIngressAuthorityV1, GisMapApprovalRequestTokenV1, InferenceMapBaseV1, approval_actor, approval_mutation_id, document_key, reader};
use semio_hub_inference::{InferenceOperationControlV1, InferencePrivateBytesV1, sha256};
use std::sync::atomic::Ordering;
use semio_hub_inference::schema::RESULT_MAX_BYTES;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_inference_runtime_contract::last_close_store_step;

    #[tokio::test]
    async fn gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer() {
        let watchdog = RuntimeLawHangWatchdogV1::arm("gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer", std::time::Duration::from_secs(30));
        let (identity, base) = canonical_identity_and_base();
        let ledger = ledger();
        let accepted = ledger.accept(&identity, &base.pack, 1_000).expect("accepted job");
        let claim = ledger.start(&accepted.job_id, &identity, 1_001).expect("claim").expect("owned epoch");
        let (proposal, command) = canonical_approval(&identity, &accepted.job_id, &base, 1_004);
        let result = InferencePrivateBytesV1::new(b"bounded-result".to_vec(), RESULT_MAX_BYTES).expect("bounded result");
        assert!(ledger.succeed(&accepted.job_id, &identity, claim.run_epoch, &result, &proposal, 1_002).expect("offer"));
        let proposal_hash = sha256(proposal.as_slice());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_003).expect("prepared outbox");
        let pool = Arc::new(db::semio_framework_async::WorkerPool::new(db::semio_framework_async::WorkerPoolConfig::new(db::semio_framework_async::ProcessKind::HeadlessBatch, 2)));
        let memory = db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage");
        let backend = Arc::new(db::storage::DbBackend::Memory(memory));
        let database = Arc::new(db::Database::open(pool.clone(), db::DbConfig::for_profile(db::Profile::Test), backend.clone()).await.expect("database"));
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let document = protocol::ArtifactId(document_key(&scope));
        let handle = database.ensure_document(&document).await.expect("document actor");
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));
        let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> =
            Arc::new(OrderedApprovalCheckpointPublisherV1 { ledger: ledger.clone(), identity: identity.clone(), job_id: accepted.job_id.clone(), attempts: std::sync::atomic::AtomicUsize::new(0), order: order.clone(), pause: None });
        let backend_witness = backend.clone();
        let committer = Arc::new(GisRetainedCommitterV1::new(database.clone(), backend, ledger.clone(), publisher));
        let key = document_key(&scope);
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        let composed_children = base.composed_children(&GisMapInferenceBindingV1).expect("exact composed children");
        let actor = approval_actor(&identity);
        let mutation_id = approval_mutation_id(&accepted.job_id, &proposal_hash);
        let command_hash = sha256(command.as_slice());
        let invalid_children = vec!["gismap-drawing".to_owned(), "substituted-value".to_owned()];
        let foreign_scope_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let foreign_scope_ingress: Arc<dyn GisMapApprovalIngressAuthorityV1> = Arc::new(TestApprovalIngressAuthorityV1 {
            scope: DocumentScope::new("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", identity.document_id.clone()),
            user_id: identity.user_id.clone(),
            session_id: identity.session_id.clone(),
            authorization_generation: identity.authorization_generation,
            released: Some(foreign_scope_ingress_released.clone()),
        });
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &composed_children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_004,
                    document_write: gate.clone(),
                    ingress: foreign_scope_ingress,
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Conflict)
        ));
        assert!(foreign_scope_ingress_released.load(Ordering::Acquire));
        let mut substituted_bases = Vec::new();
        let mut descriptor = canonical_map_base(&identity);
        descriptor.descriptor_digest = "9".repeat(64);
        substituted_bases.push(("descriptor", descriptor));
        let mut pack = canonical_map_base(&identity);
        pack.pack = InferencePrivateBytesV1::new(b"substituted-pack".to_vec(), INPUT_MAX_BYTES).expect("bounded substituted pack");
        substituted_bases.push(("pack", pack));
        let mut head_ordinal = canonical_map_base(&identity);
        head_ordinal.frontier.head_edit_ordinal += 1;
        substituted_bases.push(("head-ordinal", head_ordinal));
        let mut head_edit = canonical_map_base(&identity);
        head_edit.frontier.head_edit_id = "substituted-edit".to_owned();
        substituted_bases.push(("head-edit", head_edit));
        let mut commit = canonical_map_base(&identity);
        commit.frontier.last_commit_seq += 1;
        substituted_bases.push(("commit", commit));
        let mut chain = canonical_map_base(&identity);
        chain.frontier.chain_hash = directory::os_directory::ArtifactHash([1; 32]);
        substituted_bases.push(("chain", chain));
        for (name, candidate) in &substituted_bases {
            let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
            assert!(
                matches!(
                    committer
                        .commit(GisMapApprovalCommitRequestV1 {
                            composed_children: &composed_children,
                            scope: &scope,
                            actor: &actor,
                            mutation_id: &mutation_id,
                            command_hash: &command_hash,
                            job_id: &accepted.job_id,
                            proposal_hash: &proposal_hash,
                            command: command.as_slice(),
                            base: candidate,
                            base_frontier: &candidate.frontier,
                            deadline_ms: 60_000,
                            now_ms: 1_004,
                            document_write: gate.clone(),
                            ingress: tracked_approval_ingress(&identity, Some(ingress_released.clone())),
                        })
                        .await,
                    Err(GisMapApprovalCommitErrorV1::Conflict)
                ),
                "{name} substitution reached cleanup admission",
            );
            assert!(ingress_released.load(Ordering::Acquire), "{name} substitution retained ingress");
        }
        let mut substituted_frontier = base.frontier.clone();
        substituted_frontier.head_edit_ordinal += 1;
        let frontier_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &composed_children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &substituted_frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_004,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(frontier_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Conflict)
        ));
        assert!(frontier_ingress_released.load(Ordering::Acquire));
        let substituted_actor = format!("user:{}#session:eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", identity.user_id);
        let actor_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &composed_children,
                    scope: &scope,
                    actor: &substituted_actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_004,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(actor_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Conflict)
        ));
        assert!(actor_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.approval_recovery_by_mutation(&mutation_id).expect("pure admission refusal lookup").is_some(), "pure admission refusals preserve the prepared outbox");
        let substituted_command_hash = sha256(b"substituted-command");
        let substituted_proposal_hash = sha256(b"substituted-proposal");
        let substituted_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(
            matches!(
                committer
                    .commit(GisMapApprovalCommitRequestV1 {
                        composed_children: &composed_children,
                        scope: &scope,
                        actor: &actor,
                        mutation_id: &mutation_id,
                        command_hash: &substituted_command_hash,
                        job_id: &accepted.job_id,
                        proposal_hash: &substituted_proposal_hash,
                        command: command.as_slice(),
                        base: &base,
                        base_frontier: &base.frontier,
                        deadline_ms: 60_000,
                        now_ms: 1_004,
                        document_write: gate.clone(),
                        ingress: tracked_approval_ingress(&identity, Some(substituted_ingress_released.clone())),
                    })
                    .await,
                Err(GisMapApprovalCommitErrorV1::Conflict)
            ),
            "the public committer refuses a same-job substituted command/proposal tuple before cleanup admission",
        );
        assert!(substituted_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        let prepared_after_substitution = ledger.approval_recovery_by_mutation(&mutation_id).expect("substitution outbox lookup").expect("exact prepared outbox remains");
        assert_eq!(
            (prepared_after_substitution.0.command_hash.as_str(), prepared_after_substitution.0.proposal_hash.as_str(), prepared_after_substitution.1.authorization_generation,),
            (command_hash.as_str(), proposal_hash.as_str(), identity.authorization_generation),
        );
        let cleanup_reservations = (0..semio_hub_inference::schema::JOB_CAPACITY).map(|index| format!("{index:064x}")).collect::<Vec<_>>();
        for job_id in &cleanup_reservations {
            committer.reserve_cleanup_job(job_id).expect("bounded cleanup reservation");
        }
        let capacity_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let capacity_refused = committer
            .commit(GisMapApprovalCommitRequestV1 {
                composed_children: &composed_children,
                scope: &scope,
                actor: &actor,
                mutation_id: &mutation_id,
                command_hash: &command_hash,
                job_id: &accepted.job_id,
                proposal_hash: &proposal_hash,
                command: command.as_slice(),
                base: &base,
                base_frontier: &base.frontier,
                deadline_ms: 60_000,
                now_ms: 1_004,
                document_write: gate.clone(),
                ingress: tracked_approval_ingress(&identity, Some(capacity_ingress_released.clone())),
            })
            .await;
        assert!(matches!(capacity_refused, Err(GisMapApprovalCommitErrorV1::Capacity)));
        assert!(capacity_ingress_released.load(Ordering::Acquire));
        assert_eq!(
            ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("capacity outbox query").rows.len(),
            1,
            "cleanup-capacity refusal preserves the caller's exact durable prepared owner",
        );
        for job_id in &cleanup_reservations {
            committer.release_cleanup_job(job_id);
        }
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        let prepared_runtime_shutdown_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        committer.reserve_cleanup_job(&accepted.job_id).expect("prepared cleanup reservation");
        let prepared_runtime_shutdown_owner = GisMapPreparedApprovalV1 {
            scope: scope.clone(),
            job_id: accepted.job_id.clone(),
            mutation_id: mutation_id.clone(),
            command_hash: command_hash.clone(),
            proposal_hash: proposal_hash.clone(),
            ingress: tracked_approval_ingress(&identity, Some(prepared_runtime_shutdown_released.clone())),
        };
        let prepared_runtime_shutdown_key = format!("prepared:{}", accepted.job_id);
        assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(prepared_runtime_shutdown_key.clone()));
        let prepared_runtime_shutdown_task = GisMapPreparedApprovalTaskV1 { committer: committer.as_ref().clone(), maintenance_key: prepared_runtime_shutdown_key, owner: Some(prepared_runtime_shutdown_owner) };
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread().build().expect("temporary prepared-cleanup runtime");
            runtime.spawn(async move {
                let _retained = prepared_runtime_shutdown_task;
                std::future::pending::<()>().await;
            });
            runtime.block_on(tokio::task::yield_now());
        })
        .join()
        .expect("temporary prepared-cleanup runtime shutdown");
        assert!(committer.parked_prepared.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
        assert!(!prepared_runtime_shutdown_released.load(Ordering::Acquire), "runtime shutdown reinserts the exact prepared ingress cursor instead of dropping it");
        resume_parked_cleanup(&committer).await;
        assert!(prepared_runtime_shutdown_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("prepared runtime-shutdown query").rows.is_empty());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after prepared cleanup runtime shutdown");
        let runtime_shutdown_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        committer.reserve_cleanup_job(&accepted.job_id).expect("request cleanup reservation");
        let runtime_shutdown_ingress = tracked_approval_ingress(&identity, Some(runtime_shutdown_ingress_released.clone()));
        let runtime_shutdown_request = Arc::new(GisMapApprovalRequestTokenV1);
        let (runtime_shutdown_identity, _) = GisRetainedCommitterV1::preflight(&GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &actor,
            mutation_id: &mutation_id,
            command_hash: &command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_004,
            document_write: gate.clone(),
            ingress: runtime_shutdown_ingress,
        })
        .expect("runtime-shutdown cursor preflight");
        let runtime_shutdown_owner = GisMapAbandonedRequestV1 { key: key.clone(), identity: runtime_shutdown_identity, request: runtime_shutdown_request };
        let runtime_shutdown_maintenance_key = GisRetainedCommitterV1::abandoned_request_maintenance_key(&runtime_shutdown_owner);
        assert!(committer.maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(runtime_shutdown_maintenance_key.clone()));
        let runtime_shutdown_task = GisMapAbandonedRequestTaskV1 { committer: committer.as_ref().clone(), maintenance_key: runtime_shutdown_maintenance_key, owner: Some(runtime_shutdown_owner) };
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread().build().expect("temporary cleanup runtime");
            runtime.spawn(async move {
                let _retained = runtime_shutdown_task;
                std::future::pending::<()>().await;
            });
            runtime.block_on(tokio::task::yield_now());
        })
        .join()
        .expect("temporary cleanup runtime shutdown");
        assert!(committer.parked_requests.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
        assert!(!runtime_shutdown_ingress_released.load(Ordering::Acquire), "runtime shutdown reinserts the exact ingress cursor instead of dropping it");
        resume_parked_cleanup(&committer).await;
        assert!(runtime_shutdown_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("runtime-shutdown outbox query").rows.is_empty());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after temporary cleanup runtime shutdown");
        let thread_committer = committer.clone();
        let thread_identity = identity.clone();
        let thread_scope = scope.clone();
        let thread_frontier = base.frontier.clone();
        let thread_descriptor = base.descriptor_digest.clone();
        let thread_pack = base.pack.as_slice().to_vec();
        let thread_command = command.as_slice().to_vec();
        let thread_children = composed_children.clone();
        let thread_actor = actor.clone();
        let thread_mutation_id = mutation_id.clone();
        let thread_command_hash = command_hash.clone();
        let thread_job_id = accepted.job_id.clone();
        let thread_proposal_hash = proposal_hash.clone();
        let thread_gate = gate.clone();
        let thread_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let thread_released = thread_ingress_released.clone();
        std::thread::spawn(move || {
            let thread_base = InferenceMapBaseV1 { frontier: thread_frontier, descriptor_digest: thread_descriptor, pack: InferencePrivateBytesV1::new(thread_pack, INPUT_MAX_BYTES).expect("bounded thread base") };
            let thread_command = InferencePrivateBytesV1::new(thread_command, semio_hub_inference::command::COMMAND_MAX_BYTES).expect("bounded thread command");
            let future = thread_committer.commit(GisMapApprovalCommitRequestV1 {
                composed_children: &thread_children,
                scope: &thread_scope,
                actor: &thread_actor,
                mutation_id: &thread_mutation_id,
                command_hash: &thread_command_hash,
                job_id: &thread_job_id,
                proposal_hash: &thread_proposal_hash,
                command: thread_command.as_slice(),
                base: &thread_base,
                base_frontier: &thread_base.frontier,
                deadline_ms: 60_000,
                now_ms: 1_004,
                document_write: thread_gate,
                ingress: tracked_approval_ingress(&thread_identity, Some(thread_released)),
            });
            drop(future);
        })
        .join()
        .expect("no-runtime unpolled owner thread");
        assert!(thread_ingress_released.load(Ordering::Acquire), "no-runtime future Drop synchronously returns its ingress after exact outbox abandonment");
        assert!(ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("no-runtime outbox query").rows.is_empty());
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after no-runtime Drop");
        let unpolled_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let unpolled = committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &actor,
            mutation_id: &mutation_id,
            command_hash: &command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_004,
            document_write: gate.clone(),
            ingress: tracked_approval_ingress(&identity, Some(unpolled_ingress_released.clone())),
        });
        drop(unpolled);
        watchdog.at("awaiting the unpolled abandoned prepared handoff");
        wait_for_abandoned_prepared_handoff(&committer, &ledger, &identity, &accepted.job_id, &key, &gate, unpolled_ingress_released.as_ref()).await;
        ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005).expect("same owner revives after an unpolled future");
        let rejected_ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        assert!(matches!(
            committer
                .commit(GisMapApprovalCommitRequestV1 {
                    composed_children: &invalid_children,
                    scope: &scope,
                    actor: &actor,
                    mutation_id: &mutation_id,
                    command_hash: &command_hash,
                    job_id: &accepted.job_id,
                    proposal_hash: &proposal_hash,
                    command: command.as_slice(),
                    base: &base,
                    base_frontier: &base.frontier,
                    deadline_ms: 60_000,
                    now_ms: 1_006,
                    document_write: gate.clone(),
                    ingress: tracked_approval_ingress(&identity, Some(rejected_ingress_released.clone())),
                })
                .await,
            Err(GisMapApprovalCommitErrorV1::Rejected)
        ),);
        assert!(rejected_ingress_released.load(Ordering::Acquire));
        assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty());
        assert!(ledger.approval_recovery_by_mutation(&mutation_id).expect("rejected preflight lookup").is_some(), "a pure preflight rejection leaves the prepared outbox unchanged");
        for (phase_index, phase) in [AbandonedApprovalPhaseV1::Preflight, AbandonedApprovalPhaseV1::Assembly, AbandonedApprovalPhaseV1::Journal].into_iter().enumerate() {
            let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let ingress = tracked_approval_ingress(&identity, Some(ingress_released.clone()));
            let mut future = committer.commit(GisMapApprovalCommitRequestV1 {
                composed_children: &composed_children,
                scope: &scope,
                actor: &actor,
                mutation_id: &mutation_id,
                command_hash: &command_hash,
                job_id: &accepted.job_id,
                proposal_hash: &proposal_hash,
                command: command.as_slice(),
                base: &base,
                base_frontier: &base.frontier,
                deadline_ms: 60_000,
                now_ms: 1_004,
                document_write: gate.clone(),
                ingress,
            });
            watchdog.at(format!("polling the approval to its retained cancellation phase {phase:?}"));
            tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, phase)).await.unwrap_or_else(|_| panic!("approval reaches its retained cancellation phase {phase:?}"));
            drop(future);
            assert!(committer.cleanup_jobs.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&accepted.job_id));
            watchdog.at(format!("awaiting the abandoned handoff after cancellation phase {phase:?}"));
            wait_for_abandoned_approval_handoff(&committer, &key, &gate, ingress_released.as_ref()).await;
            let actor = handle.checkpoint_publication_snapshot().await.expect("unchanged actor frontier");
            assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id), (0, 0, None));
            let pending = ledger.pending_approvals(None, &InferenceOperationControlV1::new(1_000, 5).expect("bounded approval query")).expect("abandoned outbox query");
            assert!(pending.rows.is_empty(), "the autonomous cancellation owner clears its exact prepared outbox row before releasing Hub ingress");
            let revived = ledger.prepare_approval(&accepted.job_id, &identity, &proposal_hash, &command, 1_005 + u64::try_from(phase_index).expect("bounded phase")).expect("same owner revives its exact abandoned outbox row");
            assert_eq!((revived.mutation_id.as_str(), revived.command_hash.as_str(), revived.proposal_hash.as_str()), (mutation_id.as_str(), command_hash.as_str(), proposal_hash.as_str()));
        }
        let prepared_events = ledger.events(&accepted.job_id, &reader(&identity), 0, 1_008).expect("owner event page").events.into_iter().filter(|event| event.kind == "approval-prepared").count();
        assert_eq!(prepared_events, 1, "three exact cancellation/revival cycles preserve one immutable preparation event");
        assert!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_empty(), "pre-witness cancellation emits neither a public checkpoint nor a peer rebootstrap signal");
        assert_eq!(ledger.read(&accepted.job_id, &reader(&identity), 1_005).expect("offered proposal").proposal_state, semio_hub_inference::schema::InferenceProposalStateV1::Offered);
        let ingress_released = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ingress = tracked_approval_ingress(&identity, Some(ingress_released.clone()));
        let mut future = committer.commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &actor,
            mutation_id: &mutation_id,
            command_hash: &command_hash,
            job_id: &accepted.job_id,
            proposal_hash: &proposal_hash,
            command: command.as_slice(),
            base: &base,
            base_frontier: &base.frontier,
            deadline_ms: 60_000,
            now_ms: 1_006,
            document_write: gate.clone(),
            ingress,
        });
        watchdog.at("polling the final approval to its committed receipt cutover");
        tokio::time::timeout(std::time::Duration::from_secs(5), poll_approval_to_phase(&mut future, &committer, &key, AbandonedApprovalPhaseV1::Committed)).await.expect("approval reaches its committed receipt cutover");
        drop(future);
        watchdog.at("awaiting the autonomous public checkpoint and ledger apply after the receipt cutover");
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            loop {
                let approved = ledger.read(&accepted.job_id, &reader(&identity), 1_007).is_ok_and(|view| view.proposal_state == semio_hub_inference::schema::InferenceProposalStateV1::Approved);
                let maintenance_idle = !committer.maintenance_owns_document(&key);
                if approved && maintenance_idle && gate.try_lock().is_ok() && ingress_released.load(Ordering::Acquire) {
                    return;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("receipt cutover autonomously reaches public checkpoint and ledger apply");
        assert_eq!(order.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_slice(), ["public-checkpoint-attempt", "public-checkpoint-attempt", "public-checkpoint-ack", "peer-rebootstrap"],);
        let actor = handle.checkpoint_publication_snapshot().await.expect("committed actor frontier");
        assert_eq!((actor.frontier.head_seq, actor.frontier.commit_seq, actor.head_edit_id.as_ref().map(|id| id.0.as_str())), (1, 1, Some(mutation_id.as_str())));
        watchdog.at("driving the committer close to its terminal Store handoff");
        committer.close().await.unwrap_or_else(|error| panic!("committer close: {error:?}, last close-store step {:?}", last_close_store_step()));
        drop(committer);
        drop(handle);
        watchdog.at("shutting the database down");
        let mut database = Arc::try_unwrap(database).ok().expect("sole database owner");
        database.shutdown(&db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5))).await.expect("database shutdown");
        drop(database);
        watchdog.at("closing the terminal memory DB I/O backend");
        let db::storage::DbBackend::Memory(memory) = backend_witness.as_ref() else { panic!("the law mounts the exact memory DB I/O backend") };
        memory.close().await.expect("terminal memory DB I/O backend close");
        drop(backend_witness);
        watchdog.at("shutting the worker pool down");
        pool.shutdown().expect("worker pool shutdown");
    }
}

struct FixedMembersBindingV1;

impl MapInferenceProposalBindingV1 for FixedMembersBindingV1 {
    fn inference_service(&self) -> semio_framework_plugin::ArtifactInferenceService {
        GisMapInferenceBindingV1.inference_service()
    }

    fn commit_members(&self) -> [&'static str; 2] {
        ["fixed-drawing", "fixed-value"]
    }

    fn composed_children(&self, _pack: &[u8]) -> Result<Vec<String>, MapBindingRefusalV1> {
        Ok(vec!["fixed-drawing".to_owned(), "fixed-value".to_owned()])
    }

    fn infer(&self, _pack: &[u8], _job_id: &str, _budget: &semio_framework_plugin::WireArtifactInferenceBudget, _checkpoint: &mut dyn FnMut(u64) -> Result<(), semio_framework_plugin::ArtifactInferenceExecutionError>) -> Result<MapInferenceRunV1, MapBindingRefusalV1> {
        Err(MapBindingRefusalV1::Execution)
    }

    fn approval_effect(&self, _pack: &[u8], _job_id: &str, _budget: &semio_framework_plugin::WireArtifactInferenceBudget) -> Result<MapProposalEffectV1, MapBindingRefusalV1> {
        Err(MapBindingRefusalV1::Composition)
    }

    fn undo_effect(&self, _pack: &[u8], _original_inverse_payload: &[u8], _original_job_id: &str) -> Result<MapUndoEffectV1, MapBindingRefusalV1> {
        Err(MapBindingRefusalV1::Arity)
    }

    fn preview(&self, job_id: &str, _proposal: &[u8]) -> Result<MapRegionPreviewV1, MapBindingRefusalV1> {
        Ok(MapRegionPreviewV1 { region_id: format!("fixed-{job_id}"), ring: [[1.0, 2.0]; 5] })
    }
}

#[test]
fn the_inference_runtime_honours_a_non_default_proposal_binding() {
    let (_identity, base) = canonical_identity_and_base();
    assert_eq!(base.composed_children(&FixedMembersBindingV1), Ok(vec!["fixed-drawing".to_owned(), "fixed-value".to_owned()]));
    assert_ne!(base.composed_children(&GisMapInferenceBindingV1), base.composed_children(&FixedMembersBindingV1), "the members come from the registered binding, not from a hub literal");
    let preview = gis_map_inference_preview(&FixedMembersBindingV1, "job", &sha256(b"any proposal"), b"any proposal").expect("the registered binding decodes the preview");
    assert_eq!(serde_json::to_value(preview).expect("preview wire"), serde_json::json!({"schema": "semio.hub.gis-map-inference-preview/v1", "jobId": "job", "proposalHash": sha256(b"any proposal"), "regionId": "fixed-job", "ring": [[1.0, 2.0], [1.0, 2.0], [1.0, 2.0], [1.0, 2.0], [1.0, 2.0]]}));
    assert_eq!(gis_map_inference_preview(&FixedMembersBindingV1, "job", &"0".repeat(64), b"any proposal").err(), Some(InferenceRouteErrorV1::Conflict), "the hub still owns the exact proposal hash check");
    assert_eq!(InferenceRouteErrorV1::from(MapBindingRefusalV1::Bounds), InferenceRouteErrorV1::Bounds);
    assert_eq!(InferenceRouteErrorV1::from(MapBindingRefusalV1::Arity), InferenceRouteErrorV1::Conflict);
    assert_eq!(InferenceRouteErrorV1::from(MapBindingRefusalV1::Undecodable), InferenceRouteErrorV1::Invalid);
}
