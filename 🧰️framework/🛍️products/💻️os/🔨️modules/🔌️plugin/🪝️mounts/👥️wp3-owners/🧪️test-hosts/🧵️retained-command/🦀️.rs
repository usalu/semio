//! 🧪️ Facade test host of `retained-command`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[doc(hidden)]
pub mod retained_command_unit_1 {
    use semio_os_kernel::one_item_publication;
    use semio_framework_actor::typed_operation_page::{TypedOperationResultToken,TYPED_OPERATION_RESULT_PAGE_BYTES};
    use semio_plugin_job::reserved_jobs::{plugin_job_yield_once,QualifiedToolProof};
use semio_plugin_instance_app::MAINTENANCE_STAGES;
use semio_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_plugin_window_kits::AppActionRegistry;
    use serde_json::Value;
    use semio_plugin_decl_app::ArtifactApp;
    use semio_plugin_decl_emission::ActionMeta;
    use semio_framework::Fault;
    use semio_framework::RetainedCloneGrant;
    use semio_plugin_decl_app::tool_contract::TypedOperationResultLane;
    use semio_os_store_command::ArtifactCommand;
    use semio_plugin_composition_publication::MountedTypedCommandFullOperationStage;
    use semio_plugin_instance_app::VcsArtifactApp;
    use semio_plugin_testing_laws::artifact_app_laws;
    use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_plugin_reactor_app::PluginApp;
    use semio_plugin_retained_command::typed_operation::PendingPublicationOutcome;
    #[doc(hidden)]
pub async fn test_retained_keyed_dispatch<A, F, R>(
        mut registry: F,
        command: fn(&str, i32) -> A::Command,
        mutation: fn(i32) -> A::Mutation,
        observe: fn(&A::Snapshot) -> i32,
        clock: Option<fn() -> Option<u64>>,
        mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1,
        identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    ) where A: ArtifactApp + Default, F: FnMut() -> R, R: std::future::Future<Output = AppActionRegistry> {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        let fixture: Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)));
        for case in fixture["cases"].as_array().unwrap() {
            let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry().await, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
            artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
            if let Some(clock) = clock { assert_eq!((cx.clock_source())(), clock(), "the original caller supplies the exact authored clock"); }
            app.bind_instance_id(7).await;
            let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: 7, view_state: None };
            let first = Box::new(command(case["firstTarget"].as_str().unwrap(), case["firstValue"].as_i64().unwrap() as i32));
            admit_fixture_typed_action(&mut app, "compositeEdit", first, &meta, identity, cx).await;
            let first_id = *app.tools.latest_wins_order.items.front().unwrap();
            assert!(!app.tools.can_admit_typed_operation(first_id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64));
            for _ in 0..100_000 {
                if app.tools.tool_operations.get(first_id).is_some() {
                    break;
                }
                app.typed_ctx().advance_latest_wins_command_one(mounted_policy.preparation, cx).await.unwrap();
                if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
                plugin_job_yield_once().await;
            }
            let first_token = app.tools.tool_operations.get(first_id).expect("actual registered first worker").cancellation_lease.as_ref().unwrap().token.clone();
            let next = Box::new(command(case["nextTarget"].as_str().unwrap(), case["nextValue"].as_i64().unwrap() as i32));
            admit_fixture_typed_action(&mut app, "compositeEdit", next, &meta, identity, cx).await;
            let next_id = *app.tools.latest_wins_order.items.front().unwrap();
            let old_key = app.tools.latest_wins_commands.get(next_id).unwrap().lease.as_ref().unwrap().key.clone();
            app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation(13)], transaction: None }, identity).await.unwrap();
            let fresh_revision = app.stores.store.content_revision_now();
            for _ in 0..100_000 {
                if app.tools.tool_operations.get(next_id).is_some() {
                    break;
                }
                app.typed_ctx().advance_latest_wins_command_one(mounted_policy.preparation, cx).await.unwrap();
                if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
                plugin_job_yield_once().await;
            }
            let mounted = app.tools.tool_operations.get(next_id).expect("accepted key creates actual fresh worker");
            assert_eq!(mounted.canonical_revision, fresh_revision);
            assert_eq!(mounted.operation.generation.0, fixture["rebase"]["afterGeneration"].as_u64().unwrap());
            let fresh_key = mounted.cancellation_lease.as_ref().unwrap().key.clone();
            assert_eq!(fresh_key.generation, mounted.operation.generation);
            assert!(!app.tools.tool_cancellations.cancel(&old_key).unwrap());
            assert_eq!(serde_json::json!(first_token.is_cancelled_now()), case["firstCancelled"]);
            let expected = 13 + case["nextValue"].as_i64().unwrap() as i32;
            for _ in 0..100_000 {
                if observe(app.stores.store.snapshot_root().as_ref()) == expected {
                    break;
                }
                if app.tools.tool_operations.get(next_id).unwrap().stage == MountedTypedCommandFullOperationStage::Worker {
                    cx.with_retained_loan(app.mounted_policy.maintenance, |same| app.tools.tool_operations.get_mut(next_id).unwrap().drive_worker_step(same)).unwrap();
                } else {
                    { let __a0 = app.mounted_policy.maintenance; cx.with_retained_loan(__a0, |same| ::semio_framework_async::poll::resolve_ready(app.typed_ctx().advance_typed_operation_publication_one(identity,same)).map_err(semio_framework_job::OriginalTurnFailure::from)) }.unwrap();
                }
                plugin_job_yield_once().await;
            }
            let state = app.tools.tool_operations.get(next_id).unwrap();
            eprintln!(
                "[TRACE] registered keyed dispatch progress stage={:?} terminal_seen={} pending={} completion={} result={:?}",
                state.stage,
                state.terminal_seen,
                state.pending_artifact_publication.is_some(),
                state.publication.is_some(),
                state.result_page.as_ref().map(|page| String::from_utf8_lossy(page.bytes()))
            );
            assert_eq!(serde_json::json!(observe(app.stores.store.snapshot_root().as_ref())), serde_json::json!(expected));
            assert!(app.tools.tool_cancellations.cancel(&fresh_key).unwrap());
            for _ in 0..100_000 {
                if app.close_terminal_is_empty() {
                    break;
                }
                match fixture_close_receipt(&mut app, mounted_policy, cx).unwrap() {
                    PluginLifecycleStep::Progress(progress) => {
                        assert!(progress.copied_items <= 1);
                        assert!(progress.copied_bytes <= TYPED_OPERATION_RESULT_PAGE_BYTES);
                        assert!(progress.fits(mounted_policy.close));
                    }
                    PluginLifecycleStep::Complete(progress) => {assert!(progress.fits(mounted_policy.close));break;},
                    PluginLifecycleStep::AwaitingInput { reason } => panic!("registered keyed dispatch close awaited input: {reason}"),
                    PluginLifecycleStep::Blocked { .. } => {}
                }
                plugin_job_yield_once().await;
            }
            assert!(app.close_terminal_is_empty());
            eprintln!("[TRACE] actual registered keyed dispatch {} preserved exact target supersession, rebased its worker, published count{expected}, and closed every retained owner", case["id"]);
        }
        for case in fixture["lostReservations"].as_array().unwrap() {
            let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry().await, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
            artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
            app.bind_instance_id(7).await;
            let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: 7, view_state: None };
            let command = Box::new(command("aä🧵", 42));
            admit_fixture_typed_action(&mut app, "compositeEdit", command, &meta, identity, cx).await;
            let id = *app.tools.latest_wins_order.items.front().unwrap();
            let pending = app.tools.latest_wins_commands.get_mut(id).unwrap();
            let copy = pending.key_copy.take();
            let pointer = pending.key.as_ref().map(|key| std::ptr::from_ref(key) as usize);
            for denied in [RetainedCloneGrant { maximum_items: 0, ..mounted_policy.close }, RetainedCloneGrant { maximum_depth: 0, ..mounted_policy.close }] {
                let mut bank = semio_framework_value_testing_admission::EffectBank::new(semio_framework_value_testing_admission::BankWallet::new(denied, 1 << 20));
                let (poll, _) = semio_framework_value_testing_admission::bank_run(&mut bank, denied, 1 << 20, |effect| { let result = pending.close_key_step(effect); semio_framework_value::RetainedWorkPoll::Complete(semio_framework_value::RetainedWorkOutcome { result, work: effect.performed_work() }) });
                assert!(bank.into_wallet().is_ok());
                let semio_framework_value::RetainedWorkPoll::Complete(turn) = poll else { panic!("a denied key close stayed pending") };
                assert_eq!(turn.result.unwrap().progress().unwrap(), Default::default());
                assert_eq!(pending.key.as_ref().map(|key| std::ptr::from_ref(key) as usize), pointer);
            }
            pending.key_copy = copy;
            let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
            let foreign = id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64;
            app.tools.typed_operation_reservations[slot] = case["foreignReservation"].as_bool().unwrap().then_some(foreign);
            for _ in 0..8 {
                app.typed_ctx().advance_latest_wins_command_one(mounted_policy.preparation, cx).await.unwrap();
                if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
                assert!(app.tools.tool_operations.get(id).is_none());
                if case["foreignReservation"].as_bool().unwrap() {
                    assert_eq!(app.tools.typed_operation_reservations[slot], Some(foreign));
                }
            }
            app.tools.typed_operation_reservations[slot] = None;
            for _ in 0..100_000 {
                if app.tools.tool_operations.get(id).is_some() {
                    break;
                }
                app.typed_ctx().advance_latest_wins_command_one(mounted_policy.preparation, cx).await.unwrap();
                if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
            }
            let mounted = app.tools.tool_operations.get_mut(id).expect("vacant rejected slot receives its retained terminal result");
            assert!(mounted.session.is_none());
            let page = mounted.result_page.as_ref().unwrap();
            assert_eq!(page.lane, TypedOperationResultLane::Fault);
            assert!(mounted.acknowledge_result_page(page.token).unwrap());
            assert_eq!(observe(app.stores.store.snapshot_root().as_ref()), 0);
            for _ in 0..100_000 {
                if app.close_terminal_is_empty() {
                    break;
                }
                let key_open = app.tools.latest_wins_commands.get(id).is_some_and(|pending| pending.key.is_some());
                let quote = app.close_retirement_demands(mounted_policy.close.maximum_copy_bytes).unwrap();
                let step = fixture_close_receipt(&mut app, mounted_policy, cx).unwrap();
                if let (true, PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress)) = (key_open, &step) {
                    if progress.released_bytes > 0 {
                        assert_eq!(progress.released_bytes, quote.release_bytes, "a retained key close releases exactly what its close quote names");
                    }
                }
                plugin_job_yield_once().await;
            }
            assert!(app.close_terminal_is_empty());
            eprintln!("[TRACE] actual registered keyed dispatch {} preserved foreign reservation, retired seven UTF-8 bytes exactly, and delivered rejection after vacancy", case["id"]);
        }
    }
    #[doc(hidden)]
pub async fn test_typed_operation_slot_preadmission<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        const VACANT: usize = 11;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
        app.bind_instance_id(7).await;
        let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: 7, view_state: None };
        for slot in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            app.tools.typed_operation_reservations[slot] = (slot != VACANT).then_some((slot + 3 * ARTIFACT_LIVE_OUTPUT_SLOTS) as u64);
        }
        for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            let admitted = app.media.admit_typed_operation_slot(&app.pending, &app.tools).expect("one vacant residue class still pre-admits an exact operation slot");
            assert_eq!(admitted.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS, VACANT, "ingress must mint its operation id FOR the vacant slot instead of testing whichever slot the shared counter reached");
        }
        let first = Box::new(command("aä🧵", 42));
        admit_fixture_typed_action(&mut app, verb, first, &meta, identity, cx).await;
        let operation_id = *app.tools.latest_wins_order.items.front().expect("the admitted operation owns the vacant slot");
        assert_eq!(operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS, VACANT);
        assert_eq!(app.tools.typed_operation_reservations[VACANT], Some(operation_id));
        let mut saturated = Some(Box::new(command("bö🧶", 43)));
        let mut waiting = semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource::<A>::default();
        let mut metadata = Some(meta.clone());
        let mut tag = None;
        for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            let mut result = None;
            cx.with_retained_loan(mounted_policy.preparation, |same| { result = ::semio_framework_async::poll::resolve_ready(app.dispatch_ctx().drive_runtime_typed_command(&mut waiting, || saturated.take(), &mut tag, &mut metadata, identity, same))?; Ok(()) }).expect("full capacity retains its original admission");
            assert!(result.is_none());
            assert!(waiting.operation.is_none(), "saturation cannot mint an operation without a vacant exact slot");
        }
        assert!(waiting.pending() && saturated.is_none() && metadata.is_none());
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            if !waiting.pending() { break; }
            cx.with_retained_loan(mounted_policy.close, |same| same.with_original_retirement(|same| waiting.close_step(same))).expect("the saturated input closes under its same caller");
        }
        assert!(!waiting.pending());
        assert!(app.media.admit_typed_operation_slot(&app.pending, &app.tools).is_none());
        for slot in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            if slot != VACANT {
                app.tools.typed_operation_reservations[slot] = None;
            }
        }
        app.tools.latest_wins_commands.get_mut(operation_id).expect("the pending keyed command").lease.as_ref().expect("pending cancellation lease").cancel();
        for _ in 0..100_000 {
            if app.tools.tool_operations.get(operation_id).is_some() {
                break;
            }
            app.typed_ctx().advance_latest_wins_command_one(mounted_policy.preparation, cx).await.unwrap();
                if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
            plugin_job_yield_once().await;
        }
        let mounted = app.tools.tool_operations.get_mut(operation_id).expect("the cancelled pending command lands its retained terminal result");
        let page = mounted.result_page.as_ref().expect("terminal fault page");
        assert_eq!(page.lane, TypedOperationResultLane::Fault);
        let token = page.token;
        assert!(mounted.acknowledge_result_page(token).unwrap());
        for _ in 0..100_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            let _ = fixture_close_receipt(&mut app, mounted_policy, cx).unwrap();
            plugin_job_yield_once().await;
        }
        assert!(app.close_terminal_is_empty());
        eprintln!("[TRACE] actual typed ingress pre-admitted slot {VACANT} under 63 live foreign reservations and retained its funded input without minting an operation at true saturation");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_slot_retirement_under_storm<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        const ACTIONS: usize = 200;
        const HOST_CONTINUATIONS_PER_ACTION: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        let mut highest = 0;
        for index in 0..ACTIONS {
            let target = format!("storm-{index}");
            let payload = Box::new(command(&target, index as i32));
            admit_fixture_typed_action(&mut app, verb, payload, &meta, identity, cx).await;
            drive_host_typed_continuations(&mut app, RECEIVER, HOST_CONTINUATIONS_PER_ACTION, mounted_policy, identity, cx).await;
            let live = app.live_typed_operation_slots();
            highest = highest.max(live);
            assert_eq!(live, 0, "action {index} {}; an admitted slot must have exactly one release path the same turn driver walks", released_slot_census(&app, HOST_CONTINUATIONS_PER_ACTION));
        }
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy, cx).await;
        eprintln!("[TRACE] actual {ACTIONS} storm actions each released their typed-operation slot within {HOST_CONTINUATIONS_PER_ACTION} host continuations, peak occupancy {highest}");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_slot_release_on_every_outcome<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("settled", 1)), &meta, identity, cx).await;
        drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity, cx).await;
        assert_eq!(app.live_typed_operation_slots(), 0, "a settled operation {}", released_slot_census(&app, CONTINUATIONS));
        for value in 0..3 {
            admit_fixture_typed_action(&mut app, verb, Box::new(command("replaced", value)), &meta, identity, cx).await;
        }
        drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity, cx).await;
        assert_eq!(app.live_typed_operation_slots(), 0, "a latest-wins replacement {}", released_slot_census(&app, CONTINUATIONS));
        admit_fixture_typed_action(&mut app, verb, Box::new(command("cancelled", 4)), &meta, identity, cx).await;
        let pending = *app.tools.latest_wins_order.items.front().expect("the admitted keyed command owns its exact pending slot");
        app.tools.latest_wins_commands.get_mut(pending).expect("the admitted keyed command").lease.as_ref().expect("pending cancellation lease").cancel();
        drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity, cx).await;
        assert_eq!(app.live_typed_operation_slots(), 0, "a cancelled operation {}", released_slot_census(&app, CONTINUATIONS));
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy, cx).await;
        eprintln!("[TRACE] actual settled, latest-wins-replaced and cancelled typed operations each released their exact slot within {CONTINUATIONS} host continuations");
    }
    fn fixture_cancellation_lease<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, key: &semio_plugin_operation::ToolOperationKey, cx: &mut semio_framework_job::StepContext<'_>) -> semio_plugin_operation::ToolCancellationLease {
        artifact_app_laws::ensure_fixture_cancellation_issuer(app, cx).expect("original cancellation issuer");
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            if let Some(lease) = app.tools.tool_cancellations.begin_keyed(key, cx).expect("original keyed cancellation lease") { return lease; }
        }
        panic!("original keyed cancellation lease did not settle");
    }

    fn retire_fixture_invocation(pending: &mut Option<semio_framework::kernel::InvocationResult>, grant: RetainedCloneGrant, cx: &mut semio_framework_job::StepContext<'_>) {
        let mut active = None;
        assert!(artifact_app_laws::retire_fixture_owned(pending, &mut active, grant, cx).expect("original invocation result retires through its unchanged caller"));
        assert!(pending.is_none() && active.is_none());
    }

    async fn admit_fixture_typed_action<A: ArtifactApp + Default>(app: &mut VcsArtifactApp<A>, verb: &str, command: Box<A::Command>, meta: &ActionMeta, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) {
        assert_eq!(A::command_id(command.as_ref()).await, verb);
        artifact_app_laws::ensure_fixture_cancellation_issuer(app, cx).expect("original fixture issuer admission");
        let mut source = semio_plugin_command::command_admission::original_typed_command::OriginalTypedCommandSource::<A>::default();
        let mut command = Some(command);
        let mut metadata = Some(meta.clone());
        let mut tag = None;
        let mut qualified = false;
        let mut completed = false;
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            let grant = app.mounted_policy.preparation;
            let mut result = None;
            cx.with_retained_loan(grant, |same| { result = ::semio_framework_async::poll::resolve_ready(app.dispatch_ctx().drive_runtime_typed_command(&mut source, || command.take(), &mut tag, &mut metadata, identity, same))?; Ok(()) }).expect("original typed fixture ingress");
            qualified |= source.admission.as_ref().is_some_and(|admission| matches!(&admission.proof, QualifiedToolProof::AppOwned(_)));
            if result.is_some() { retire_fixture_invocation(&mut result, app.mounted_policy.close, cx); completed = true; break; }
        }
        assert!(completed && qualified, "the actual declared application producer admits the typed operation");
        for _ in 0..artifact_app_laws::OriginalFixtureCaller::maximum_turns() {
            if !source.pending() { return; }
            cx.with_retained_loan(app.mounted_policy.close, |same| same.with_original_retirement(|same| source.close_step(same))).expect("same original typed fixture source closes");
        }
        panic!("typed fixture ingress retains an unclosed original source");
    }
    /// 🔁️ The host's OWN typed-operation continuation, unit for unit: exactly what
    /// `plugin_runtime::advance_typed_operation_output` drives per turn — one publication unit, the result
    /// page for the bound receiver plus its mandatory ACK, then the effect, event, UI-scope and completion
    /// outboxes. Deliberately calls NO `maintenance_step`: a slot the host cannot release on this path alone
    /// is a leak, whatever the cooperative rotation would eventually have done about it.
    async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt> {
        
        let mut ownership_receipts = Vec::new();
        artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture admission receipts");
        for _ in 0..continuations {
            if !app.has_runnable_typed_operations() {
                return ownership_receipts;
            }
            cx.with_retained_loan(mounted_policy.preparation, |same| ::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity, same)).map_err(semio_framework_job::OriginalTurnFailure::from)).expect("same original host continuation unit");
            artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture continuation receipts");
            while let Some(mut page) = semio_plugin_testing_laws::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
                assert!(app.acknowledge_typed_operation_result(crate::app_testing::artifact_app_laws::close_fixture_result_page(&mut page,mounted_policy.close)).expect("presented result page accepts its exact token"));
            }
            while app.take_typed_operation_effect().is_some() {}
            while app.take_typed_operation_event().is_some() {}
            while app.take_typed_operation_ui_scope().is_some() {}
            while app.take_typed_operation_completion().await.expect("one terminal completion witness").is_some() {}
            plugin_job_yield_once().await;
        }
        ownership_receipts
    }

    /// 🕰️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B16: the host's OWN call, with the ACK round trip the
    /// reactor really has. `poll` routes an `Event::Message` acknowledgement at the TOP of a turn and only
    /// then advances the operation, so the ACK a turn's presented page earns is submitted with the NEXT
    /// continuation, never inside the turn that produced it. `stop_at_idle` picks which host call this is:
    /// `false` is the renderer's `settlePluginTurn`, which also continues while it owes an acknowledgement;
    /// `true` is every call that can only read the turn's STATUS — the eventless `drainTypedOperations` poll
    /// and a `refresh-ui` settle whose surfaces are already published. Answers
    /// `(terminal completions, continuations spent, first parked continuation)`, where "parked" is a
    /// continuation that ended owing typed-operation work while the turn reported none runnable.
    async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, stop_at_idle: bool, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> HostCallCensus {
        
        let mut ownership_receipts = Vec::new();
        artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original deferred admission receipts");
        let mut acknowledgements: Vec<TypedOperationResultToken> = Vec::new();
        let (mut completions, mut spent, mut parked) = (0usize, 0usize, None);
        let mut history_patches = 0usize;
        let mut terminal: Option<(TypedOperationResultLane, String)> = None;
        while spent < continuations {
            for token in std::mem::take(&mut acknowledgements) {
                app.acknowledge_typed_operation_result(token).expect("a presented result page accepts its exact token");
            }
            cx.with_retained_loan(mounted_policy.preparation, |same| ::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity, same)).map_err(semio_framework_job::OriginalTurnFailure::from)).expect("same original host continuation unit");
            artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original deferred continuation receipts");
            while let Some(mut page) = semio_plugin_testing_laws::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
                if matches!(page.lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault) {
                    terminal = Some((page.lane, String::from_utf8_lossy(page.bytes()).into_owned()));
                }
                acknowledgements.push(semio_plugin_testing_laws::artifact_app_laws::close_fixture_result_page(&mut page,mounted_policy.close));
            }
            while app.take_typed_operation_effect().is_some() {}
            while app.take_typed_operation_event().is_some() {}
            while app.take_typed_operation_ui_scope().is_some() {}
            while let Some(completion) = app.take_typed_operation_completion().await.expect("one terminal completion witness") {
                completions += 1;
                history_patches += usize::from(completion.history_patch.is_some());
            }
            let runnable = app.has_runnable_typed_operations();
            if !runnable && app.has_pending_typed_operations() {
                parked.get_or_insert(spent);
            }
            spent += 1;
            if !runnable && (stop_at_idle || acknowledgements.is_empty()) {
                break;
            }
            plugin_job_yield_once().await;
        }
        HostCallCensus { ownership_receipts, completions, history_patches, spent, parked, terminal }
    }

    struct HostCallCensus {
        pub ownership_receipts: Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt>,
        pub completions: usize,
        history_patches: usize,
        pub spent: usize,
        pub parked: Option<usize>,
        pub terminal: Option<(TypedOperationResultLane, String)>,
    }
    fn assert_landing_terminal_lane(terminal: Option<&(TypedOperationResultLane, String)>, verb: &str, spent: usize) {
        let Some((lane, body)) = terminal else {
            panic!("'{verb}' reached no terminal lane at all within {spent} host continuations, so this law asserted nothing about a landing mutation");
        };
        assert_eq!(*lane, TypedOperationResultLane::Terminal, "'{verb}' terminated on the {lane:?} lane ({body:?}) within {spent} host continuations; a refused ladder publishes nothing, so this law would be vacuous over it");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("own-call", 1)), &meta, identity, cx).await;
        let HostCallCensus { spent, parked, terminal, .. } = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, false, mounted_policy, identity, cx).await;
        assert_eq!(parked, None, "continuation {parked:?} of {spent} ended owing typed-operation work while the turn reported none runnable; it {}", released_slot_census(&app, spent));
        assert!(!app.has_pending_typed_operations(), "the admitting call ended owing typed-operation work; it {}", released_slot_census(&app, spent));
        assert_landing_terminal_lane(terminal.as_ref(), verb, spent);
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy, cx).await;
        eprintln!("[TRACE] actual one mutating typed operation kept its turn runnable through all {spent} continuations of the call that admitted it");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_completes_under_a_status_only_host_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("drain-call", 2)), &meta, identity, cx).await;
        let HostCallCensus { spent, parked, terminal, .. } = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, true, mounted_policy, identity, cx).await;
        let drain = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, true, mounted_policy, identity, cx).await;
        let drained = drain.spent;
        assert!(
            !app.has_pending_typed_operations(),
            "a status-only call of {spent} continuations (parked at {parked:?}) and one eventless drain call of {drained} left typed-operation work for the next command; it {}",
            released_slot_census(&app, spent + drained)
        );
        assert_landing_terminal_lane(terminal.as_ref().or(drain.terminal.as_ref()), verb, spent + drained);
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy, cx).await;
        eprintln!("[TRACE] actual a status-only host call finished its typed operation in {spent} continuations, with {drained} drain continuations after it");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_lands_its_edit_inside_the_admitting_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let mut original_scope = artifact_app_laws::OriginalFixtureScope::admit();
        let cx = original_scope.cx();
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await.expect("the reference snapshot births");
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { expected_document_revision:None, actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("admitting-call", 3)), &meta, identity, cx).await;
        let census = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, false, mounted_policy, identity, cx).await;
        let (spent, parked) = (census.spent, census.parked);
        let Some((lane, body)) = census.terminal.clone() else {
            panic!("the admitting call of {spent} continuations (parked at {parked:?}) handed back no terminal-lane page for '{verb}' — the ladder finishes on a later host call; it {}", released_slot_census(&app, spent));
        };
        let expected = usize::from(lane == TypedOperationResultLane::Terminal);
        assert_eq!(
            census.completions,
            expected,
            "the admitting call of {spent} continuations carried back a {lane:?} page ({body:?}) but {} completion witness(es); a Terminal page owes exactly one, a Fault page none; it {}",
            census.completions,
            released_slot_census(&app, spent)
        );
        assert_eq!(
            census.history_patches,
            expected,
            "the admitting call of {spent} continuations handed back a {lane:?} page ({body:?}) and {} completion witness(es) but {} of them carried a history patch; the `create-object` row is what the renderer turns into the document frame, so a Terminal completion owes exactly one; it {}",
            census.completions,
            census.history_patches,
            released_slot_census(&app, spent)
        );
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy, cx).await;
        eprintln!("[TRACE] actual the admitting call handed back the {lane:?} lane ({body:?}), {} completion witness(es) and {} history patch(es) within {spent} continuations", census.completions, census.history_patches);
    }
    fn released_slot_census<A: ArtifactApp>(app: &VcsArtifactApp<A>, continuations: usize) -> String {
        let owners: Vec<String> = (0..ARTIFACT_LIVE_OUTPUT_SLOTS)
            .filter(|slot| !app.media.typed_operation_slot_is_vacant(&app.pending, &app.tools, *slot))
            .map(|slot| {
                let mounted = app.tools.tool_operations.entry(slot).map(|(id, operation)| format!("mounted {id}:{:?}:presented={}", operation.stage, operation.result_page_presented));
                let reserved = app.tools.typed_operation_reservations[slot].map(|id| format!("reserved {id}"));
                let pending = app.tools.latest_wins_commands.entry(slot).map(|(id, _)| format!("latest-wins {id}"));
                let download = (!app.media.segmented_downloads.slot_is_vacant(slot)).then(|| "segmented-download".to_string());
                let closure = (!app.media.segmented_closures.slot_is_vacant(slot)).then(|| "segmented-closure".to_string());
                format!("{slot}:{}", [mounted, reserved, pending, download, closure].into_iter().flatten().collect::<Vec<String>>().join("+"))
            })
            .collect();
        format!("left {} of {ARTIFACT_LIVE_OUTPUT_SLOTS} typed-operation slots live after {continuations} host continuations {owners:?}", app.live_typed_operation_slots())
    }
    fn fixture_close_receipt<A:ArtifactApp>(app:&mut VcsArtifactApp<A>,mounted_policy:semio_plugin_mounted_owner::MountedOwnerPolicyV1, cx: &mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let grant=mounted_policy.close;
        let demand=app.close_retirement_demands(grant.maximum_copy_bytes)?;
        semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant|Ok((app.close_step(same,grant)?,app.close_terminal_is_empty())))
    }
    async fn close_fixture_app_to_terminal_emptiness<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, mounted_policy: semio_plugin_mounted_owner::MountedOwnerPolicyV1, cx: &mut semio_framework_job::StepContext<'_>) {
        for _ in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
            if !semio_framework_job_worker::worker_job_retirements_are_parked() {
                break;
            }
            let _ = PluginApp::maintenance_step(app, cx, mounted_policy.maintenance).unwrap();
            plugin_job_yield_once().await;
        }
        for _ in 0..1_000_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            let _ = fixture_close_receipt(app, mounted_policy, cx).unwrap();
            plugin_job_yield_once().await;
        }
        assert!(app.close_terminal_is_empty());
    }
    #[doc(hidden)]
pub async fn test_retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = semio_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence, PresenceMutation = semio_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation> + Default>() {
        typed_command_full_operation_tests::retained_cancellation_publication_boundaries::<A>().await;
    }
    fn fixture_close_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant, cx: &mut semio_framework_job::StepContext<'_>) -> PluginLifecycleStep {
        let _demand = app.close_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        app.close_step(cx, original_grant).unwrap()
    }
    fn fixture_maintenance_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant, cx: &mut semio_framework_job::StepContext<'_>) -> PluginLifecycleStep {
        let _demand = app.maintenance_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES,cx.fuel_remaining()).unwrap();
        app.maintenance_step(cx, original_grant).unwrap()
    }
    #[doc(hidden)]
pub async fn test_retained_latest_wins_slot_and_publication_fairness<A: ArtifactApp<Presence = semio_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence, PresenceMutation = semio_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation> + Default>() {
        typed_command_full_operation_tests::retained_latest_wins_slot_and_publication_fairness::<A>().await;
    }
    #[doc(hidden)]
pub async fn test_retained_document_cancellation<A: ArtifactApp + Default>(
        factory: std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>,
        mutation: fn() -> A::Mutation,
        observe: fn(&A::Snapshot) -> i32,
    ) {
        typed_command_full_operation_tests::retained_document_cancellation::<A>(factory, mutation, observe).await;
    }
    include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs");
}


