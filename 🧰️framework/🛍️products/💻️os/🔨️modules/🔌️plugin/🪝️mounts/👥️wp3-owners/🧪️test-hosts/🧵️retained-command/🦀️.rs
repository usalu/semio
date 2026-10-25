//! 🧪️ Facade test host of `retained-command`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[doc(hidden)]
pub mod retained_command_unit_1 {
    use semio_framework_os_kernel::one_item_publication;
    use semio_framework_actor::typed_operation_page::{TypedOperationResultToken,TYPED_OPERATION_RESULT_PAGE_BYTES};
    use semio_framework_plugin_job::reserved_jobs::{plugin_job_yield_once,QualifiedToolProof};
use semio_framework_plugin_instance_app::MAINTENANCE_STAGES;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_plugin_window_kits::AppActionRegistry;
    use serde_json::Value;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_decl_emission::ActionMeta;
    use semio_framework::Fault;
    use semio_framework::RetainedCloneGrant;
    use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
    use semio_framework_os_store_command::ArtifactCommand;
    use semio_framework_plugin_composition_publication::MountedTypedCommandFullOperationStage;
    use semio_framework_plugin_instance_app::VcsArtifactApp;
    use semio_framework_plugin_testing_laws::artifact_app_laws;
    use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
    use semio_framework_plugin_reactor_app::PluginApp;
    use semio_framework_plugin_retained_command::typed_operation::PendingPublicationOutcome;
    #[doc(hidden)]
pub async fn test_retained_keyed_dispatch<A: ArtifactApp + Default>(
        registry: AppActionRegistry,
        command: fn(&str, i32) -> A::Command,
        mutation: fn(i32) -> A::Mutation,
        observe: fn(&A::Snapshot) -> i32,
        clock: Option<fn() -> Option<u64>>,
        mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1,
        identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
    ) {
        let fixture: Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🔗️tool-latest-wins-integration.json")).unwrap();
        let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, std::thread::available_parallelism().map(std::num::NonZeroUsize::get).unwrap_or(1)));
        for case in fixture["cases"].as_array().unwrap() {
            let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry.clone(), protocol::ActorId("fixture".into()), mounted_policy, identity).await;
            artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
            app.tools.test_tool_clock = clock;
            app.bind_instance_id(7).await;
            let meta = ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None };
            let first = Box::new(command(case["firstTarget"].as_str().unwrap(), case["firstValue"].as_i64().unwrap() as i32));
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&first).unwrap();
            let admission = app.tools.admit_command_wire(&app.stores, "compositeEdit", &wire, 1).await.unwrap();
            assert!(matches!(admission.proof, QualifiedToolProof::AppOwned(_)));
            app.dispatch_typed_command_inner(first, admission, &meta, identity).await.unwrap();
            let first_id = *app.tools.latest_wins_order.items.front().unwrap();
            assert!(!app.tools.can_admit_typed_operation(first_id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64));
            for _ in 0..100_000 {
                if app.tools.tool_operations.get(first_id).is_some() {
                    break;
                }
                app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
                if let Some((_,grant,progress))=app.take_typed_operation_preparation_receipt(){assert_eq!(grant,mounted_policy.preparation);assert!(progress.fits(grant));}
                plugin_job_yield_once().await;
            }
            let first_token = app.tools.tool_operations.get(first_id).expect("actual registered first worker").cancellation_lease.as_ref().unwrap().token.clone();
            let next = Box::new(command(case["nextTarget"].as_str().unwrap(), case["nextValue"].as_i64().unwrap() as i32));
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&next).unwrap();
            let admission = app.tools.admit_command_wire(&app.stores, "compositeEdit", &wire, 1).await.unwrap();
            app.dispatch_typed_command_inner(next, admission, &meta, identity).await.unwrap();
            let next_id = *app.tools.latest_wins_order.items.front().unwrap();
            let old_key = app.tools.latest_wins_commands.get(next_id).unwrap().lease.as_ref().unwrap().key.clone();
            app.stores.store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation(13)], transaction: None }, identity).await.unwrap();
            let fresh_revision = app.stores.store.content_revision_now();
            for _ in 0..100_000 {
                if app.tools.tool_operations.get(next_id).is_some() {
                    break;
                }
                app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
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
                    app.tools.tool_operations.get_mut(next_id).unwrap().drive_worker_step(app.mounted_policy.maintenance).unwrap();
                } else {
                    { let __a0 = app.mounted_policy.maintenance; app.typed_ctx().advance_typed_operation_publication_one(__a0).await }.unwrap();
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
                state.result_page.as_ref().map(|page| String::from_utf8_lossy(&page.bytes[..page.len]))
            );
            assert_eq!(serde_json::json!(observe(app.stores.store.snapshot_root().as_ref())), serde_json::json!(expected));
            assert!(app.tools.tool_cancellations.cancel(&fresh_key).unwrap());
            for _ in 0..100_000 {
                if app.close_terminal_is_empty() {
                    break;
                }
                match fixture_close_receipt(&mut app, mounted_policy).unwrap() {
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
            let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry.clone(), protocol::ActorId("fixture".into()), mounted_policy, identity).await;
            artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
            app.bind_instance_id(7).await;
            let meta = ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None };
            let command = Box::new(command("aä🧵", 42));
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).unwrap();
            let admission = app.tools.admit_command_wire(&app.stores, "compositeEdit", &wire, 1).await.unwrap();
            app.dispatch_typed_command_inner(command, admission, &meta, identity).await.unwrap();
            let id = *app.tools.latest_wins_order.items.front().unwrap();
            let pending = app.tools.latest_wins_commands.get_mut(id).unwrap();
            let copy = pending.key_copy.take();
            let key = pending.key.take();
            pending.retired_key = Some(fixture["keyRetirement"]["text"].as_str().unwrap().into());
            let pointer=pending.retired_key.as_ref().unwrap().as_ptr();let capacity=pending.retired_key.as_ref().unwrap().capacity();
            for denied in[RetainedCloneGrant{maximum_items:0,..mounted_policy.close},RetainedCloneGrant{maximum_release_bytes:capacity-1,..mounted_policy.close},RetainedCloneGrant{maximum_depth:0,..mounted_policy.close}]{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pending.close_key_step(denied).unwrap());assert_eq!(step.progress().unwrap(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(pending.retired_key.as_ref().unwrap().as_ptr(),pointer);}
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||pending.close_key_step(mounted_policy.close).unwrap());assert_eq!(step.progress().unwrap().released_bytes,capacity);assert_eq!(step.progress().unwrap().copied_bytes,0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,capacity));assert_eq!(serde_json::json!(capacity),fixture["keyRetirement"]["utf8Bytes"]);
            pending.key_copy = copy;
            pending.key = key;
            let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
            let foreign = id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64;
            app.tools.typed_operation_reservations[slot] = case["foreignReservation"].as_bool().unwrap().then_some(foreign);
            for _ in 0..8 {
                app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
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
                app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
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
                let _ = fixture_close_receipt(&mut app, mounted_policy).unwrap();
                plugin_job_yield_once().await;
            }
            assert!(app.close_terminal_is_empty());
            eprintln!("[TRACE] actual registered keyed dispatch {} preserved foreign reservation, retired seven UTF-8 bytes exactly, and delivered rejection after vacancy", case["id"]);
        }
    }
    #[doc(hidden)]
pub async fn test_typed_operation_slot_preadmission<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        const VACANT: usize = 11;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        app.bind_instance_id(7).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: 7, view_state: None };
        for slot in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            app.tools.typed_operation_reservations[slot] = (slot != VACANT).then_some((slot + 3 * ARTIFACT_LIVE_OUTPUT_SLOTS) as u64);
        }
        for _ in 0..ARTIFACT_LIVE_OUTPUT_SLOTS {
            let admitted = app.media.admit_typed_operation_slot(&app.pending, &app.tools).expect("one vacant residue class still pre-admits an exact operation slot");
            assert_eq!(admitted.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS, VACANT, "ingress must mint its operation id FOR the vacant slot instead of testing whichever slot the shared counter reached");
        }
        let first = Box::new(command("aä🧵", 42));
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&first).unwrap();
        let admission = app.tools.admit_command_wire(&app.stores, verb, &wire, 1).await.unwrap();
        app.dispatch_typed_command_inner(first, admission, &meta, identity).await.expect("a typed action must dispatch while unrelated operations hold every other slot");
        let operation_id = *app.tools.latest_wins_order.items.front().expect("the admitted operation owns the vacant slot");
        assert_eq!(operation_id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS, VACANT);
        assert_eq!(app.tools.typed_operation_reservations[VACANT], Some(operation_id));
        let saturated = Box::new(command("bö🧶", 43));
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&saturated).unwrap();
        let admission = app.tools.admit_command_wire(&app.stores, verb, &wire, 1).await.unwrap();
        let fault = app.dispatch_typed_command_inner(saturated, admission, &meta, identity).await.expect_err("only true saturation of every fixed slot may refuse a typed action");
        assert_eq!(fault.code.0, "interactive-job.typed-operation-capacity");
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
            app.advance_latest_wins_command_one(mounted_policy.preparation).await.unwrap();
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
            let _ = fixture_close_receipt(&mut app, mounted_policy).unwrap();
            plugin_job_yield_once().await;
        }
        assert!(app.close_terminal_is_empty());
        eprintln!("[TRACE] actual typed ingress pre-admitted slot {VACANT} under 63 live foreign reservations and refused only at true saturation");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_slot_retirement_under_storm<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        const ACTIONS: usize = 200;
        const HOST_CONTINUATIONS_PER_ACTION: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        let mut highest = 0;
        for index in 0..ACTIONS {
            let target = format!("storm-{index}");
            let payload = Box::new(command(&target, index as i32));
            let wire = <A::Command as ::protocol::OpBinary>::encode_op(&payload).unwrap();
            let admission = app.tools.admit_command_wire(&app.stores, verb, &wire, 1).await.expect("every storm action retains its exact command admission");
            if let Err(fault) = app.dispatch_typed_command_inner(payload, admission, &meta, identity).await {
                panic!("action {index} was refused with '{}' ({}) after {highest} of {ARTIFACT_LIVE_OUTPUT_SLOTS} typed-operation slots were never released by the host continuation", fault.message, fault.code.0);
            }
            drive_host_typed_continuations(&mut app, RECEIVER, HOST_CONTINUATIONS_PER_ACTION, mounted_policy, identity).await;
            let live = app.live_typed_operation_slots();
            highest = highest.max(live);
            assert_eq!(live, 0, "action {index} {}; an admitted slot must have exactly one release path the same turn driver walks", released_slot_census(&app, HOST_CONTINUATIONS_PER_ACTION));
        }
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
        eprintln!("[TRACE] actual {ACTIONS} storm actions each released their typed-operation slot within {HOST_CONTINUATIONS_PER_ACTION} host continuations, peak occupancy {highest}");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_slot_release_on_every_outcome<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("settled", 1)), &meta, identity).await;
        drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity).await;
        assert_eq!(app.live_typed_operation_slots(), 0, "a settled operation {}", released_slot_census(&app, CONTINUATIONS));
        for value in 0..3 {
            admit_fixture_typed_action(&mut app, verb, Box::new(command("replaced", value)), &meta, identity).await;
        }
        drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity).await;
        assert_eq!(app.live_typed_operation_slots(), 0, "a latest-wins replacement {}", released_slot_census(&app, CONTINUATIONS));
        admit_fixture_typed_action(&mut app, verb, Box::new(command("cancelled", 4)), &meta, identity).await;
        let pending = *app.tools.latest_wins_order.items.front().expect("the admitted keyed command owns its exact pending slot");
        app.tools.latest_wins_commands.get_mut(pending).expect("the admitted keyed command").lease.as_ref().expect("pending cancellation lease").cancel();
        drive_host_typed_continuations(&mut app, RECEIVER, CONTINUATIONS, mounted_policy, identity).await;
        assert_eq!(app.live_typed_operation_slots(), 0, "a cancelled operation {}", released_slot_census(&app, CONTINUATIONS));
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
        eprintln!("[TRACE] actual settled, latest-wins-replaced and cancelled typed operations each released their exact slot within {CONTINUATIONS} host continuations");
    }
    async fn admit_fixture_typed_action<A: ArtifactApp + Default>(app: &mut VcsArtifactApp<A>, verb: &str, command: Box<A::Command>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(command.as_ref()).unwrap();
        let admission = app.tools.admit_command_wire(&app.stores, verb, &wire, 1).await.expect("every fixture action retains its exact command admission");
        if let Err(fault) = app.dispatch_typed_command_inner(command, admission, meta, identity).await {
            panic!("typed ingress refused '{verb}' with '{}' ({})", fault.message, fault.code.0);
        }
    }
    /// 🔁️ The host's OWN typed-operation continuation, unit for unit: exactly what
    /// `plugin_runtime::advance_typed_operation_output` drives per turn — one publication unit, the result
    /// page for the bound receiver plus its mandatory ACK, then the effect, event, UI-scope and completion
    /// outboxes. Deliberately calls NO `maintenance_step`: a slot the host cannot release on this path alone
    /// is a leak, whatever the cooperative rotation would eventually have done about it.
    #[cfg(test)]
    async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt> {
        let mut receiving_job=semio_framework_plugin_testing_laws::artifact_app_laws::FixtureOriginalJob::new(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_retained_turn());
        let mut ownership_receipts = Vec::new();
        artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture admission receipts");
        for _ in 0..continuations {
            if !app.has_runnable_typed_operations() {
                return ownership_receipts;
            }
            receiving_job.receive(mounted_policy.preparation,|receiving|::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,mounted_policy.preparation,receiving))).expect("one host continuation unit");
            artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture continuation receipts");
            while let Some(mut page) = semio_framework_plugin_testing_laws::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
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
    #[cfg(test)]
    async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, stop_at_idle: bool, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> HostCallCensus {
        let mut receiving_job=semio_framework_plugin_testing_laws::artifact_app_laws::FixtureOriginalJob::new(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_retained_turn());
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
            receiving_job.receive(mounted_policy.preparation,|receiving|::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(identity,mounted_policy.preparation,receiving))).expect("one host continuation unit");
            artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original deferred continuation receipts");
            while let Some(mut page) = semio_framework_plugin_testing_laws::artifact_app_laws::take_fixture_result_page(app,receiver,mounted_policy.preparation).expect("original fixture page presentation") {
                if matches!(page.lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault) {
                    terminal = Some((page.lane, String::from_utf8_lossy(page.bytes()).into_owned()));
                }
                acknowledgements.push(semio_framework_plugin_testing_laws::artifact_app_laws::close_fixture_result_page(&mut page,mounted_policy.close));
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

    #[cfg(test)]
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
pub async fn test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("own-call", 1)), &meta, identity).await;
        let HostCallCensus { spent, parked, terminal, .. } = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, false, mounted_policy, identity).await;
        assert_eq!(parked, None, "continuation {parked:?} of {spent} ended owing typed-operation work while the turn reported none runnable; it {}", released_slot_census(&app, spent));
        assert!(!app.has_pending_typed_operations(), "the admitting call ended owing typed-operation work; it {}", released_slot_census(&app, spent));
        assert_landing_terminal_lane(terminal.as_ref(), verb, spent);
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
        eprintln!("[TRACE] actual one mutating typed operation kept its turn runnable through all {spent} continuations of the call that admitted it");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_completes_under_a_status_only_host_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("drain-call", 2)), &meta, identity).await;
        let HostCallCensus { spent, parked, terminal, .. } = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, true, mounted_policy, identity).await;
        let drain = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, true, mounted_policy, identity).await;
        let drained = drain.spent;
        assert!(
            !app.has_pending_typed_operations(),
            "a status-only call of {spent} continuations (parked at {parked:?}) and one eventless drain call of {drained} left typed-operation work for the next command; it {}",
            released_slot_census(&app, spent + drained)
        );
        assert_landing_terminal_lane(terminal.as_ref().or(drain.terminal.as_ref()), verb, spent + drained);
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
        eprintln!("[TRACE] actual a status-only host call finished its typed operation in {spent} continuations, with {drained} drain continuations after it");
    }
    #[doc(hidden)]
pub async fn test_typed_operation_lands_its_edit_inside_the_admitting_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        const CONTINUATIONS: usize = 512;
        const RECEIVER: u32 = 7;
        let mut app = VcsArtifactApp::<A>::with_registry(A::default(), registry, protocol::ActorId("fixture".into()), mounted_policy, identity).await;
        app.bind_instance_id(RECEIVER).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: RECEIVER, view_state: None };
        admit_fixture_typed_action(&mut app, verb, Box::new(command("admitting-call", 3)), &meta, identity).await;
        let census = drive_host_call_with_deferred_acks(&mut app, RECEIVER, CONTINUATIONS, false, mounted_policy, identity).await;
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
        close_fixture_app_to_terminal_emptiness(&mut app, mounted_policy).await;
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
    fn fixture_close_receipt<A:ArtifactApp>(app:&mut VcsArtifactApp<A>,mounted_policy:semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1)->Result<PluginLifecycleStep,Fault>{
        let grant=mounted_policy.close;
        let demand=app.close_retirement_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step(demand,grant,|grant|Ok((app.close_step(grant)?,app.close_terminal_is_empty())))
    }
    async fn close_fixture_app_to_terminal_emptiness<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1) {
        for _ in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
            if !semio_framework_job_worker::worker_job_retirements_are_parked() {
                break;
            }
            let _ = PluginApp::maintenance_step(app, mounted_policy.maintenance).unwrap();
            plugin_job_yield_once().await;
        }
        for _ in 0..1_000_000 {
            if app.close_terminal_is_empty() {
                break;
            }
            let _ = fixture_close_receipt(app, mounted_policy).unwrap();
            plugin_job_yield_once().await;
        }
        assert!(app.close_terminal_is_empty());
    }
    #[doc(hidden)]
pub async fn test_retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence, PresenceMutation = semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation> + Default>() {
        typed_command_full_operation_tests::retained_cancellation_publication_boundaries::<A>().await;
    }
    fn fixture_close_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant) -> PluginLifecycleStep {
        let _demand = app.close_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        app.close_step(original_grant).unwrap()
    }
    fn fixture_maintenance_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant) -> PluginLifecycleStep {
        let _demand = app.maintenance_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        app.maintenance_step(original_grant).unwrap()
    }
    #[doc(hidden)]
pub async fn test_retained_latest_wins_slot_and_publication_fairness<A: ArtifactApp<Presence = semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence, PresenceMutation = semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation> + Default>() {
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


