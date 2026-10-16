//! 🧪️ Facade test host of `retained-command`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
pub(crate) mod retained_command_unit_1 {
    use semio_framework_plugin_retained_command::typed_operation::PendingPublicationOutcome;
    #[cfg(test)]
    pub(crate) async fn test_retained_keyed_dispatch<A: ArtifactApp + Default>(
        registry: AppActionRegistry,
        command: fn(&str, i32) -> A::Command,
        mutation: fn(i32) -> A::Mutation,
        observe: fn(&A::Snapshot) -> i32,
        clock: Option<fn() -> Option<u64>>,
        mounted_policy: crate::MountedOwnerPolicyV1,
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
    #[cfg(test)]
    pub(crate) async fn test_typed_operation_slot_preadmission<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
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
    #[cfg(test)]
    pub(crate) async fn test_typed_operation_slot_retirement_under_storm<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
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
    #[cfg(test)]
    pub(crate) async fn test_typed_operation_slot_release_on_every_outcome<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
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
    #[cfg(test)]
    async fn admit_fixture_typed_action<A: ArtifactApp + Default>(app: &mut VcsArtifactApp<A>, verb: &str, command: Box<A::Command>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(command.as_ref()).unwrap();
        let admission = app.tools.admit_command_wire(&app.stores, verb, &wire, 1).await.expect("every fixture action retains its exact command admission");
        if let Err(fault) = app.dispatch_typed_command_inner(command, admission, meta, identity).await {
            panic!("typed ingress refused '{verb}' with '{}' ({})", fault.message, fault.code.0);
        }
    }
    #[cfg(test)]
    async fn drive_host_typed_continuations<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt> {
        let mut ownership_receipts = Vec::new();
        artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture admission receipts");
        for _ in 0..continuations {
            if !app.has_runnable_typed_operations() {
                return ownership_receipts;
            }
            app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
            artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original fixture continuation receipts");
            while let Some(page) = app.take_typed_operation_result_page(receiver) {
                assert!(app.acknowledge_typed_operation_result(page.token).expect("presented result page accepts its exact token"));
            }
            while app.take_typed_operation_effect().is_some() {}
            while app.take_typed_operation_event().is_some() {}
            while app.take_typed_operation_ui_scope().is_some() {}
            while app.take_typed_operation_completion().await.expect("one terminal completion witness").is_some() {}
            plugin_job_yield_once().await;
        }
        ownership_receipts
    }
    #[cfg(test)]
    async fn drive_host_call_with_deferred_acks<P: PluginApp>(app: &mut P, receiver: u32, continuations: usize, stop_at_idle: bool, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> HostCallCensus {
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
            app.advance_typed_operation_publication(identity,mounted_policy.preparation).await.expect("one host continuation unit");
            artifact_app_laws::retain_original_ownership_receipts(app, mounted_policy, &mut ownership_receipts).expect("original deferred continuation receipts");
            while let Some(page) = app.take_typed_operation_result_page(receiver) {
                if matches!(page.lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault) {
                    terminal = Some((page.lane, String::from_utf8_lossy(page.bytes()).into_owned()));
                }
                acknowledgements.push(page.token);
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
        ownership_receipts: Vec<artifact_app_laws::TypedOperationFixtureOwnershipReceipt>,
        completions: usize,
        history_patches: usize,
        spent: usize,
        parked: Option<usize>,
        terminal: Option<(TypedOperationResultLane, String)>,
    }
    #[cfg(test)]
    fn assert_landing_terminal_lane(terminal: Option<&(TypedOperationResultLane, String)>, verb: &str, spent: usize) {
        let Some((lane, body)) = terminal else {
            panic!("'{verb}' reached no terminal lane at all within {spent} host continuations, so this law asserted nothing about a landing mutation");
        };
        assert_eq!(*lane, TypedOperationResultLane::Terminal, "'{verb}' terminated on the {lane:?} lane ({body:?}) within {spent} host continuations; a refused ladder publishes nothing, so this law would be vacuous over it");
    }
    #[cfg(test)]
    pub(crate) async fn test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
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
    #[cfg(test)]
    pub(crate) async fn test_typed_operation_completes_under_a_status_only_host_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
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
    #[cfg(test)]
    pub(crate) async fn test_typed_operation_lands_its_edit_inside_the_admitting_call<A: ArtifactApp + Default>(registry: AppActionRegistry, verb: &str, command: fn(&str, i32) -> A::Command, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
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
    #[cfg(test)]
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
    #[cfg(test)]
    fn fixture_close_receipt<A:ArtifactApp>(app:&mut VcsArtifactApp<A>,mounted_policy:crate::MountedOwnerPolicyV1)->Result<PluginLifecycleStep,Fault>{
        let grant=mounted_policy.close;
        let demand=app.close_retirement_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.to_string()))?;
        semio_framework_plugin_ownership::bounded_ownership::runtime_lifecycle_step(demand,grant,|grant|Ok((app.close_step(grant)?,app.close_terminal_is_empty())))
    }
    #[cfg(test)]
    async fn close_fixture_app_to_terminal_emptiness<A: ArtifactApp>(app: &mut VcsArtifactApp<A>, mounted_policy: crate::MountedOwnerPolicyV1) {
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
    #[cfg(test)]
    pub(crate) async fn test_retained_cancellation_publication_boundaries<A: ArtifactApp<Presence = crate::publication_fixture::PublicationPresence, PresenceMutation = crate::publication_fixture::PublicationPresenceMutation> + Default>() {
        typed_command_full_operation_tests::retained_cancellation_publication_boundaries::<A>().await;
    }
    #[cfg(test)]
    fn fixture_close_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant) -> PluginLifecycleStep {
        let _demand = app.close_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        app.close_step(original_grant).unwrap()
    }
    #[cfg(test)]
    fn fixture_maintenance_turn<P: PluginApp>(app: &mut P, original_grant:RetainedCloneGrant) -> PluginLifecycleStep {
        let _demand = app.maintenance_retirement_demands(TYPED_OPERATION_RESULT_PAGE_BYTES).unwrap();
        app.maintenance_step(original_grant).unwrap()
    }
    #[cfg(test)]
    pub(crate) async fn test_retained_latest_wins_slot_and_publication_fairness<A: ArtifactApp<Presence = crate::publication_fixture::PublicationPresence, PresenceMutation = crate::publication_fixture::PublicationPresenceMutation> + Default>() {
        typed_command_full_operation_tests::retained_latest_wins_slot_and_publication_fairness::<A>().await;
    }
    #[cfg(test)]
    pub(crate) async fn test_retained_document_cancellation<A: ArtifactApp + Default>(
        factory: std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<A::Snapshot, A::Mutation>>,
        mutation: fn() -> A::Mutation,
        observe: fn(&A::Snapshot) -> i32,
    ) {
        typed_command_full_operation_tests::retained_document_cancellation::<A>(factory, mutation, observe).await;
    }
    #[cfg(test)]
    include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs");
}

#[cfg(test)]
pub(crate) mod retained_command_unit_2 {
    #[cfg(test)]
    use semio_framework_plugin_decl_emission::ChildEmit;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use semio_framework_ui_locale::LocalizedLabel;
    use semio_framework_value::retained_clone::RetainedCloneGrant;
    use semio_framework_value::retained_clone::RetainedCloneProgress;
    use semio_framework_value::retained_clone::RetainedCloneStep;
    #[cfg(any(test, feature = "artifact-app-testing"))]
    use serde_json::Value;
    use semio_framework_value::ValueError;
    use semio_framework_plugin_decl_app::ArtifactApp;
    use semio_framework_plugin_decl_emission::Emit;
    use store::Mutation;
    use crate::plugin_app::PluginApp;
    #[cfg(test)]
    mod child_complete_group_candidate_tests{
        mod paged_encoder{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🛫️encoder/🦀️.rs");}
        mod paged_owner{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🫙️owner/🦀️.rs");}
        mod candidate{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🧩️projection/🦀️.rs");}
        mod reader{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📖️reader/🦀️.rs");}
        use candidate::{ChildGroupSources,ChildGroupSource};
        use semio_framework_os_kernel::os_spr::operation_bytes::{OwnedOperationBytes,OperationByteOutput,OperationByteCloseStep};
        use semio_framework_value::{ErasedSnapshotRetirement,NativeEncodeControl,list::PagedList};
        use semio_framework_ui_locale::LocalizedLabel;
        struct Group{owner:String,slot:String,child_id:String,schema:String,ops:PagedList<OwnedOperationBytes,2>,labels:PagedList<LocalizedLabel,2>}
        struct Groups(Vec<Group>);
        impl ChildGroupSources for Groups{
            fn len(&self)->usize{self.0.len()}
            fn group_at(&self,index:usize)->Option<ChildGroupSource<'_>>{self.0.get(index).map(|group|ChildGroupSource{owner:&group.owner,slot:&group.slot,child_id:&group.child_id,schema:&group.schema,operations:&group.ops,labels:&group.labels})}
        }
        fn drain(bytes:&mut OwnedOperationBytes){for _ in 0..bytes.len()+256{if bytes.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break}}assert!(bytes.terminal_is_empty());}
        fn groups(fixture:&serde_json::Value)->Groups{
            let mut groups=Vec::new();
            for value in fixture["groups"].as_array().unwrap(){
                let mut ops=PagedList::<OwnedOperationBytes,2>::empty();let mut labels=PagedList::<LocalizedLabel,2>::empty();
                while !ops.has_reserved_slot(){assert!(ops.reserve_one(4096).unwrap().progressed)}
                while !labels.has_reserved_slot(){assert!(labels.reserve_one(4096).unwrap().progressed)}
                let mut bytes=OwnedOperationBytes::try_new(8194,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);
                for byte in value["ops"][0].as_array().unwrap(){bytes.write_bytes(&[byte.as_u64().unwrap()as u8],&mut encoding).unwrap();}
                assert!(ops.push_reserved(bytes).is_ok());
                let label=LocalizedLabel::from_fn(|terminology,locale|value["labels"][0][terminology.as_str()][locale.as_str()].as_str().unwrap().to_owned());
                assert!(labels.push_reserved(label).is_ok());
                groups.push(Group{owner:value["owner"].as_str().unwrap().into(),slot:value["slot"].as_str().unwrap().into(),child_id:value["child_id"].as_str().unwrap().into(),schema:value["op_schema"].as_str().unwrap().into(),ops,labels});
            }
            Groups(groups)
        }
        struct Visitor<'a>{expected:&'a Groups,group:usize,operation:usize,byte:usize,label:usize,partial:Option<OwnedOperationBytes>,complete:PagedList<OwnedOperationBytes,4>}
        impl<'a> Visitor<'a>{
            fn new(expected:&'a Groups)->Self{Self{expected,group:0,operation:0,byte:0,label:0,partial:None,complete:PagedList::empty()}}
            fn close(&mut self){if let Some(source)=self.partial.as_mut(){drain(source);}self.partial.take();while let Some(source)=self.complete.get_mut(self.complete.len().saturating_sub(1)){drain(source);self.complete.pop();}while !self.complete.terminal_is_empty(){assert!(self.complete.release_empty_page(4096).unwrap().progressed)}}
        }
        impl reader::ChildGroupDecodeVisitor for Visitor<'_>{
            fn begin_groups(&mut self,count:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0.len());Ok(())}
            fn begin_group(&mut self,index:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{self.group=index;Ok(())}
            fn text(&mut self,field:reader::ChildGroupText,source:semio_framework_os_kernel::os_pack::codec::ByteSpan<'_>,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{let group=&self.expected.0[self.group];let expected=match field{reader::ChildGroupText::Owner=>&group.owner,reader::ChildGroupText::Slot=>&group.slot,reader::ChildGroupText::ChildId=>&group.child_id,reader::ChildGroupText::Schema=>&group.schema};assert!(source.iter().eq(expected.bytes()));Ok(())}
            fn begin_operations(&mut self,count:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0[self.group].ops.len());Ok(())}
            fn begin_operation(&mut self,index:usize,count:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{
                self.operation=index;self.byte=0;assert!(self.partial.is_none());assert_eq!(count,self.expected.0[self.group].ops[index].len());
                self.partial=Some(OwnedOperationBytes::try_new(count.max(1),control.maximum_bytes()).unwrap());Ok(())
            }
            fn operation_byte(&mut self,byte:u8,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{
                assert_eq!(self.expected.0[self.group].ops[self.operation].byte_at(self.byte),Some(byte));
                let source=self.partial.as_mut().unwrap();
                while !source.has_reserved_slot(){let required=source.next_allocation_bytes().map_err(|fault|semio_framework_value::ValueError::new(fault.kind,fault.reason))?;assert!(required<=4096);control.checkpoint()?;control.charge(required)?;assert!(source.reserve_one(4096).map_err(|fault|semio_framework_value::ValueError::new(fault.kind,fault.reason))?.progressed)}
                source.push_reserved(byte).unwrap();self.byte+=1;Ok(())
            }
            fn end_operation(&mut self,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{
                while !self.complete.has_reserved_slot(){let required=self.complete.next_allocation_bytes().map_err(semio_framework_value::ValueError::from)?;assert!(required<=4096);control.charge(required)?;assert!(self.complete.reserve_one(4096).unwrap().progressed)}
                assert!(self.complete.push_reserved(self.partial.take().unwrap()).is_ok());Ok(())
            }
            fn begin_labels(&mut self,count:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert_eq!(count,self.expected.0[self.group].labels.len());Ok(())}
            fn begin_label(&mut self,index:usize,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{self.label=index;Ok(())}
            fn label_text(&mut self,terminology:semio_framework_ui_locale::Terminology,locale:semio_framework_ui_locale::Locale,source:semio_framework_os_kernel::os_pack::codec::ByteSpan<'_>,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert!(source.iter().eq(self.expected.0[self.group].labels[self.label].resolve(terminology,locale).bytes()));Ok(())}
            fn end_label(&mut self,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{Ok(())}
            fn end_group(&mut self,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{Ok(())}
            fn end_groups(&mut self,_:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{assert!(self.partial.is_none());Ok(())}
        }
        fn close_groups(groups:&mut Groups){for group in &mut groups.0{while let Some(source)=group.ops.get_mut(group.ops.len().saturating_sub(1)){drain(source);group.ops.pop();}while !group.ops.terminal_is_empty(){assert!(group.ops.release_empty_page(4096).unwrap().progressed)}while let Some(label)=group.labels.get_mut(group.labels.len().saturating_sub(1)){while label.close_owned_cell_one(4096).unwrap().is_some(){}group.labels.pop();}while !group.labels.terminal_is_empty(){assert!(group.labels.release_empty_page(4096).unwrap().progressed)}}}
        #[test]
        fn child_complete_group_source_candidate_reader_retains_actual_partial8194_pages(){
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🎒️child-group-wire/🔣️.json")).unwrap();let mut groups=groups(&fixture);
            let mut output=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);candidate::encode_groups_into(&groups,&semio_framework_os_kernel::os_pack::codec::PackEncodeOptions::default(),&mut output,&mut encoding).unwrap();
            let options=semio_framework_os_kernel::os_pack::codec::PackDecodeOptions::default();
            let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);reader::visit_groups_span(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).unwrap();assert_eq!(visitor.complete.len(),2);assert!(visitor.complete[0].iter().eq(groups.0[0].ops[0].iter()));assert_eq!(visitor.complete[0].len(),8194);assert!(visitor.complete[1].iter().eq(groups.0[1].ops[0].iter()));let decoded_owned=decoding.owned_bytes();assert_eq!(decoded_owned,visitor.complete.allocated_bytes()+visitor.complete.iter().map(OwnedOperationBytes::allocated_bytes).sum::<usize>());visitor.close();
            let mut visitor=Visitor::new(&groups);let mut cancel=|progress:semio_framework_value::native_decoding::NativeDecodeProgress|progress.owned_bytes<8192;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut cancel);assert!(reader::visit_groups_span(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).is_err());let partial=visitor.partial.as_ref().unwrap();assert!(partial.len()>0&&partial.len()<8194);assert!(partial.allocated_bytes()>=8192);assert_eq!(partial.allocated_bytes(),decoding.owned_bytes());let retained=partial.len();visitor.close();
            let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(4096,&mut allow);assert!(reader::visit_groups_span(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output),&options,&mut decoding,&mut visitor).is_err());assert!(visitor.partial.as_ref().unwrap().is_empty());assert!(visitor.partial.as_ref().unwrap().allocated_bytes()>0);assert!(decoding.owned_bytes()<=4096);visitor.close();
            let mut visitor=Visitor::new(&groups);let mut allow=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(65536,&mut allow);let short=semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output).slice(0,output.len()-1).unwrap();assert!(reader::visit_groups_span(short,&options,&mut decoding,&mut visitor).is_err());assert_eq!(visitor.complete.len(),2);visitor.close();
            drain(&mut output);close_groups(&mut groups);
            eprintln!("[DEBUG] held group span reader reconstructed all8194 exact octets into caller-retained real pages; cancellation retains{retained} partial octets; initial4096 rejects real next page; truncation retains both completed sources; all owners drain1/4096");
        }
        #[test]
        fn child_complete_group_source_candidate_matches_independent_wire(){
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🎒️child-group-wire/🔣️.json")).unwrap();
            let mut groups=groups(&fixture);assert_eq!(groups.0[0].ops[0].len(),8194);
            let options=semio_framework_os_kernel::os_pack::codec::PackEncodeOptions::default();
            let mut output=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);
            let count=candidate::encode_groups_into(&groups,&options,&mut output,&mut control).unwrap();assert_eq!(count,output.len());
            let expected=fixture["wire"].as_array().unwrap();assert_eq!(output.len(),expected.len());for(index,byte)in expected.iter().enumerate(){assert_eq!(output.byte_at(index),Some(byte.as_u64().unwrap()as u8),"exact original group octet {index}")}
            let mut comparison=semio_framework_os_kernel::os_spr::operation_bytes::OperationByteComparison::new(semio_framework_os_kernel::os_pack::codec::ByteSpan::from_source(&output));let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);candidate::encode_groups_into(&groups,&options,&mut comparison,&mut control).unwrap();comparison.finish().unwrap();
            let mut empty=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);assert_eq!(candidate::encode_groups_into(&Groups(Vec::new()),&options,&mut empty,&mut control).unwrap(),0);assert_eq!(empty.len(),0);assert_eq!(empty.allocated_bytes(),0);drain(&mut empty);
            let mut partial=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|progress.owned_bytes<8192;let mut control=NativeEncodeControl::new(65536,&mut allow);assert!(candidate::encode_groups_into(&groups,&options,&mut partial,&mut control).is_err());assert!(partial.allocated_bytes()>0);let retained=partial.allocated_bytes();drain(&mut partial);
            let mut refused=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut small=options.clone();small.limits.max_total_alloc=128;let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);assert!(candidate::encode_groups_into(&groups,&small,&mut refused,&mut control).is_err());assert!(control.owned_bytes()<=128);drain(&mut refused);
            drain(&mut output);
            close_groups(&mut groups);
            eprintln!("[DEBUG] held complete child group candidate equals independent JSON/TextEncoder/Ajv wire; actual paged input8194 and paged output drain1/4096; cancellation retains{retained} physical bytes; production ChildEmit remains held");
        }
        #[test]
        fn child_complete_original_semantic_metadata8194_grant4096(){
            let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🔤️semantic-text-wire/🔣️.json")).unwrap();
            for case in corpus["cases"].as_array().unwrap(){
                let field=case["field"].as_str().unwrap();let large="m".repeat(8194);assert_eq!(large.len(),8194);let mut child=super::ChildEmit::open("","",0);
                match field{"owner"=>child.owner=large,"slot"=>child.slot=large,"child_id"=>child.child_id=large,"op_schema"=>child.op_schema.0=large,_=>{let keys:Vec<&str>=field.split('.').collect();assert_eq!(keys.len(),3);let label=semio_framework_ui_locale::LocalizedLabel::from_fn(|terminology,locale|if terminology.as_str()==keys[1]&&locale.as_str()==keys[2]{large.clone()}else{String::new()});child.labels.push(label);}}
                use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};let page=RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:semio_framework_value::ordered::MAX_AVL_HEIGHT * 5 + 16 };
                assert_eq!(child.close_one(RetainedCloneGrant{maximum_items:0,..page}).unwrap(),RetainedCloneStep::Progress(RetainedCloneProgress::default()));let mut complete=false;let mut physical=0;
                if matches!(field,"owner"|"slot"|"child_id"|"op_schema"){for _ in 0..64{if matches!(child.close_one(page).unwrap(),RetainedCloneStep::Complete(_)){panic!("a String backing releases whole: a 4096-byte release grant can never retire the 8194-byte owner: {field}")}}}
                let whole=RetainedCloneGrant{maximum_release_bytes:8194+4096,..page};
                for _ in 0..8194+128{match child.close_one(whole).unwrap(){RetainedCloneStep::Progress(progress)=>{assert!(progress.fits(whole));physical+=progress.released_bytes;},RetainedCloneStep::Complete(progress)=>{assert!(progress.fits(whole));physical+=progress.released_bytes;complete=true;break}}}
                assert!(complete,"actual ChildEmit full8194 semantic owner remained held under the whole-backing release grant: {field}");assert!(physical>=8194);assert!(child.ops.is_empty());assert!(child.labels.is_empty());assert!(child.owner.is_empty());assert!(child.slot.is_empty());assert!(child.child_id.is_empty());assert!(child.op_schema.0.is_empty());
            }
        }
        fn drain_retained(owner:&mut dyn semio_framework_value::ErasedSnapshotRetirement)->usize{
            use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep};
            let mut released=0;for _ in 0..100000{match owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Complete(_)=>break,RetainedCloneStep::Progress(progress)=>{let released_items=progress.copied_items;let released_bytes=progress.released_bytes;assert!(progress.copied_bytes<=4096);assert_eq!(progress.retained_capacity_bytes,0);assert!(released_items<=1);assert!(released_bytes<=4096);released+=released_bytes;},step=>panic!("retained child owner refused its genuine physical page grant: {step:?}")}}assert!(owner.terminal_is_empty());released
        }
        fn drain_typed_groups(groups:&mut semio_framework_value::list::PagedList<paged_owner::PagedChildOwner,{isize::MAX as usize}>)->usize{
            let mut released=0;while let Some(child)=groups.get_mut(groups.len().saturating_sub(1)){released+=drain_retained(child);groups.pop();}while !groups.terminal_is_empty(){let step=groups.release_empty_page(4096).unwrap();assert!(step.progressed);assert!(step.released_allocation_bytes<=4096);released+=step.released_allocation_bytes;}released
        }
        #[test]
        fn child_complete_group_source_candidate_paged_semantic8194_metadata_and_labels(){
            use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,native_decoding::NativeDecodeRetirementRecipient,ErasedSnapshotRetirement};
            use semio_framework_os_kernel::{os_pack::codec::{ByteSpan,PackEncodeOptions,PackDecodeOptions},os_spr::operation_bytes::{OperationByteOutput,OperationByteComparison}};
            let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🔤️semantic-text-wire/🔣️.json")).unwrap();assert_eq!(corpus["maximumItems"],1);assert_eq!(corpus["maximumBytes"],4096);assert_eq!(corpus["maximumAllocationBytes"],65536);
            let mut cases=0;for case in corpus["cases"].as_array().unwrap(){
                let wire:Vec<u8>=case["wire"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let mut source=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);source.write_bytes(&wire,&mut encoding).unwrap();let source_bytes=source.allocated_bytes();
                let mut recipient=NativeDecodeRetirementRecipient::new();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(65536,&mut allow);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);let mut groups=paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).unwrap();let admitted=decoding.owned_bytes();drop(decoding);assert!(recipient.has_owner());assert_eq!(groups.len(),1);assert_eq!(groups[0].operations[0].len(),8194);assert!(groups[0].operations[0].iter().eq("17".bytes().chain(std::iter::repeat_n(b' ',8192))));
                let typed_bytes=groups.allocated_bytes()+groups.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>();assert_eq!(admitted,std::mem::size_of::<paged_owner::PagedChildDecodeOwner>()+typed_bytes);let source_group=paged_encoder::PagedChildGroups::group(&groups,0).unwrap();let field=case["field"].as_str().unwrap();let text=match field{"owner"=>source_group.owner,"slot"=>source_group.slot,"child_id"=>source_group.child_id,"op_schema"=>source_group.schema,"label.native.en"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[0]).unwrap(),"label.native.de"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[1]).unwrap(),"label.reuse.en"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[1],semio_framework_ui_locale::Locale::ALL[0]).unwrap(),"label.reuse.de"=>source_group.labels.text(0,semio_framework_ui_locale::Terminology::ALL[1],semio_framework_ui_locale::Locale::ALL[1]).unwrap(),_=>panic!("closed semantic field changed")};assert_eq!(text.len(),8194);for index in 0..8194{assert_eq!(text.byte(index).unwrap(),b'm')}
                let scaffold_released=drain_retained(&mut recipient);assert!(scaffold_released>=std::mem::size_of::<paged_owner::PagedChildDecodeOwner>());assert_eq!(source.allocated_bytes(),source_bytes);assert_eq!(groups.allocated_bytes()+groups.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>(),typed_bytes);
                let mut symbols=paged_encoder::ChildSymbolOwner::empty();let mut comparison=OperationByteComparison::new(ByteSpan::from_source(&source));let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert_eq!(paged_encoder::encode_paged_groups_into(&groups,&PackEncodeOptions::default(),&mut comparison,&mut symbols,&mut encoding).unwrap(),wire.len());comparison.finish().unwrap();drain_retained(&mut symbols);assert_eq!(drain_typed_groups(&mut groups),typed_bytes);assert_eq!(source.allocated_bytes(),source_bytes);
                let mut recipient=NativeDecodeRetirementRecipient::new();let mut stages=0;let mut cancel=|progress:semio_framework_value::native_decoding::NativeDecodeProgress|{if progress.total==8194&&progress.completed==0{stages+=1;}!(progress.total==8194&&stages>=2&&progress.completed>=256)};let mut decoding=NativeDecodeControl::new(65536,&mut cancel);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());let partial_admitted=decoding.owned_bytes();drop(decoding);assert!(recipient.has_owner());assert!(partial_admitted>=std::mem::size_of::<paged_owner::PagedChildDecodeOwner>()+4096);assert_eq!(drain_retained(&mut recipient),partial_admitted);
                let mut recipient=NativeDecodeRetirementRecipient::new();let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(4096,&mut allow);decoding.install_retirement_recipient(&mut recipient).unwrap();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());let small_admitted=decoding.owned_bytes();drop(decoding);assert!(small_admitted<=4096);assert_eq!(drain_retained(&mut recipient),small_admitted);
                let mut allow=|_|true;let mut decoding=NativeDecodeControl::new(65536,&mut allow);let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);assert!(paged_owner::decode_paged_groups_span(ByteSpan::from_source(&source),&PackDecodeOptions::default(),&PackEncodeOptions::default(),&mut decoding,&mut encoding).is_err());assert_eq!(decoding.owned_bytes(),0);drain(&mut source);cases+=1;
            }assert_eq!(cases,8);eprintln!("[DEBUG] held paged semantic child metadata/locale family preserves all8 full8194 positive fields, actual partial cancellation and initial4096 recipient refusal; source, returned typed text and canonical scaffolds drain independently1/4096");
        }
        #[test]
        fn child_complete_group_source_candidate_direct_producer_and_parent(){
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
            use semio_framework_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
            use reader::ChildGroupText;
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/📤️child-group-producer/🔣️.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);let large=fixture["text"].as_str().unwrap().repeat(8194);
            fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;let padding=[b' ';256];for _ in 0..32{output.write_bytes(&padding,control)?;}Ok(())}
            fn initialize(owner:&mut paged_owner::PagedChildOwner,text:&str,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>){for(field,value)in[(ChildGroupText::Owner,text),(ChildGroupText::Slot,"slot"),(ChildGroupText::ChildId,"child"),(ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&value,options,control).unwrap();}}
            let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);initialize(&mut owner,&large,&options,&mut control);let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let text=if terminology==semio_framework_ui_locale::Terminology::ALL[0]&&locale==semio_framework_ui_locale::Locale::ALL[0]{large.as_str()}else{"small"};receipt.read_label_from_source(terminology,locale,&text,&mut control).unwrap();}}receipt.commit(&mut control).unwrap();assert_eq!(owner.operations.len(),1);assert_eq!(owner.operations[0].len(),8194);assert!(owner.operations[0].iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192))));assert_eq!(owner.allocated_bytes(),control.owned_bytes());let allocated=owner.allocated_bytes();let mut parent=ParentAllocationReturn::<128>::try_new(4096,allocated).unwrap();assert!(!owner.return_one(&mut parent,0,4096).unwrap().progressed);let mut returned=0;
            for _ in 0..8194*3+256{if owner.terminal_is_empty(){break;}let before=owner.allocated_bytes();let step=owner.return_one(&mut parent,1,4096).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}assert!(owner.terminal_is_empty());assert_eq!(returned,allocated);assert_eq!(parent.retained_bytes(),allocated);assert!(!parent.terminal_is_empty());let mut released=0;for _ in 0..256{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(released,allocated);
            for mode in["encoder","label","file","segment"]{let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);initialize(&mut owner,"root",&options,&mut control);let mut limits=options.clone();if mode=="file"{limits.limits.max_file_len=1;}if mode=="segment"{limits.limits.max_segment_len=128;}
                let result=owner.produce_owned_operation(8194,&limits,&mut control,|_,output,control|{encode(output,control)?;if mode=="encoder"{return Err(semio_framework_os_kernel::os_pack::PackRefusal::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"neutral direct encoder refused after source")).into())}Ok(())});
                match result{Err(_)=>{assert!(mode=="encoder"||mode=="file")},Ok(mut receipt)=>{receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();assert!(mode=="label"||mode=="segment");if mode=="segment"{assert!(receipt.read_label_from_source(semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[0],&large.as_str(),&mut control).is_err());}else{receipt.read_label_from_source(semio_framework_ui_locale::Terminology::ALL[0],semio_framework_ui_locale::Locale::ALL[0],&large.as_str(),&mut control).unwrap();}assert!(receipt.commit(&mut control).is_err());}}
                assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);for(index,byte)in b"child.empty".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain_retained(&mut owner),allocated);
            }
            eprintln!("[DEBUG] held direct child source receipt preserves exact8194 octets and independent original8194 owner/locale text under64k, admits both prefix pages before transfer, retains encoder/label/file/segment refusal and pending schema, real parent alone physically disposes1/4096");
        }
        #[test]
        fn child_complete_group_source_candidate_repeated_prefix_preserves_accepted_owners(){
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::ParentAllocationReturn};
            use semio_framework_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🔁️repeated-prefix/🔣️.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["acceptedOperations"],2);
            fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}
            let options=PackEncodeOptions::default();let original_large_schema=std::iter::repeat_n('s',8194).collect::<String>();for original_schema in["child.empty",original_large_schema.as_str()]{for refused in[false,true]{let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,original_schema)]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
                let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"first",&mut control).unwrap();}}receipt.commit(&mut control).unwrap();let source_pointer=owner.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let allocated=owner.allocated_bytes();let mut parent=ParentAllocationReturn::<16>::try_new(4096,65536).unwrap();assert!(!owner.return_committed_scaffold_one(&mut parent,0).unwrap().progressed);assert_eq!(owner.allocated_bytes(),allocated);
                let mut returned=0;for _ in 0..8194+128{let before=owner.allocated_bytes();let step=owner.return_committed_scaffold_one(&mut parent,1).unwrap();if !step.progressed{break;}assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;assert_eq!(owner.operations.len(),1);assert_eq!(owner.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(source_pointer));assert_eq!(owner.view().unwrap().labels.len(),1);}assert_eq!(parent.retained_bytes(),returned);if original_schema.len()==8194{assert!(returned>8194);assert!(!parent.terminal_is_empty());}else{assert_eq!(returned,0);assert!(parent.terminal_is_empty());}let mut schema_released=0;for _ in 0..128{match parent.close_step(1,4096){semio_framework_value::retirement::allocation_return::AllocationReturnStep::Complete=>break,semio_framework_value::retirement::allocation_return::AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);schema_released+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(schema_released,returned);
                let result=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|{encode(output,control)?;if refused{return Err(semio_framework_os_kernel::os_pack::PackRefusal::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::Canceled,"second direct encoder refused")).into())}Ok(())});match result{Err(_)=>assert!(refused),Ok(mut receipt)=>{assert!(!refused);for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"second",&mut control).unwrap();}}receipt.commit(&mut control).unwrap();}}
                assert_eq!(owner.operations.len(),if refused{1}else{2});assert_eq!(owner.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(source_pointer));assert!(owner.operations.iter().all(|source|source.len()==8194&&source.iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192)))));let view=owner.view().unwrap();assert_eq!(view.labels.len(),if refused{1}else{2});for(index,byte)in b"schema.actual".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}assert_eq!(owner.allocated_bytes()+schema_released,control.owned_bytes());let allocated=owner.allocated_bytes();assert_eq!(drain_retained(&mut owner),allocated);
            }}
            eprintln!("[DEBUG] repeated child source8194 accepted prefix preserves pointer, schema, both locale matrices and first source on second refusal; only committed scaffold returns before next producer; every actual allocation retires1/4096");
        }
        #[test]
        fn child_complete_group_source_candidate_refuses_partial_last_locale_before_commit(){
            use semio_framework_value::{NativeEncodeControl,native_encoding::NativeEncodeProgress};
            use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🌓️partial-label/🔣️.json")).unwrap();
            assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["partialBytes"],256);assert_eq!(fixture["completeCells"],3);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);
            for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
            let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}).unwrap();receipt.read_first_schema_from_source(&"schema.actual",&mut control).unwrap();
            let mut axes=Vec::new();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{axes.push((terminology,locale));}}
            for(terminology,locale)in &axes[..3]{receipt.read_label_from_source(*terminology,*locale,&"first",&mut control).unwrap();}
            let baseline=control.owned_bytes();let continuation=control.pause().unwrap();let mut cancel=|progress:NativeEncodeProgress|progress.owned_bytes<=baseline||progress.completed<256;let mut control=NativeEncodeControl::resume(continuation,&mut cancel).unwrap();let large="x".repeat(8194);assert!(receipt.read_label_from_source(axes[3].0,axes[3].1,&large.as_str(),&mut control).is_err());assert!(control.owned_bytes()>baseline);
            let continuation=control.pause().unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::resume(continuation,&mut allow).unwrap();assert!(receipt.commit(&mut control).is_err(),"partial last locale was published despite lacking complete text authority");
            assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);for(index,byte)in b"child.empty".iter().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),*byte)}let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain_retained(&mut owner),allocated);
            eprintln!("[DEBUG] three complete locale cells and fourth partial256/8194 retain their real owners; commit refuses and original schema/prefix remain unchanged; all physical owners drain1/4096");
        }
        #[test]
        fn child_complete_actual_authored_label_producer_preserves_all_axes_and_retained_source(){
            use semio_framework_value::{paged_text::{PagedText,InlineTextBuffer},NativeEncodeControl,ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneStep};
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/✍️authored-label/🔣️.json")).unwrap();assert_eq!(fixture["englishBytes"],8194);assert_eq!(fixture["germanBytes"],8205);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["initialAllocationBytes"],4096);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            fn drain<const N:usize>(paged:&mut PagedText<N>,inline:&mut InlineTextBuffer)->usize{if !inline.terminal_is_empty(){assert!(inline.close_one(1));}let mut released=0;for _ in 0..8205+128{match paged.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Complete(_)=>break,RetainedCloneStep::Progress(progress)=>{let released_items=progress.copied_items;let released_bytes=progress.released_bytes;assert!(progress.copied_bytes<=4096);assert_eq!(progress.retained_capacity_bytes,0);assert!(released_items<=1&&released_bytes<=4096);released+=released_bytes;}}}assert!(paged.terminal_is_empty()&&inline.terminal_is_empty());released}
            for large in[false,true]{let operation=crate::test_app_mutation_fixture::SetLabel{value:if large{std::iter::repeat_n('x',8181).collect()}else{fixture["shortValue"].as_str().unwrap().to_owned()}};for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let(prefix,suffix)=match locale{semio_framework_ui_locale::Locale::En=>(fixture["englishPrefix"].as_str().unwrap(),fixture["englishSuffix"].as_str().unwrap()),semio_framework_ui_locale::Locale::De=>(fixture["germanPrefix"].as_str().unwrap(),fixture["germanSuffix"].as_str().unwrap())};let mut paged=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut active=false;let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);paged.encode_with_inline(&mut inline,&mut active,65536,&mut control,|output|operation.label_into(terminology,locale,output)).unwrap();let expected=prefix.as_bytes().iter().copied().chain(operation.value.as_bytes().iter().copied()).chain(suffix.as_bytes().iter().copied());if large{assert!(active&&inline.borrow().is_err());assert!(paged.borrow().unwrap().bytes().eq(expected));}else{assert!(!active&&paged.borrow().is_err());assert!(inline.borrow().unwrap().bytes().eq(expected));assert_eq!(control.owned_bytes(),0);}let allocated=paged.allocated_bytes();assert_eq!(allocated,control.owned_bytes());assert_eq!(drain(&mut paged,&mut inline),allocated);
                if large{let mut paged=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut active=false;let mut allow=|_|true;let mut control=NativeEncodeControl::new(4096,&mut allow);assert!(paged.encode_with_inline(&mut inline,&mut active,65536,&mut control,|output|operation.label_into(terminology,locale,output)).is_err());assert!(paged.borrow().is_err()&&inline.borrow().is_err());assert!(active&&paged.allocated_bytes()>0&&paged.allocated_bytes()<=4096);assert_eq!(paged.allocated_bytes(),control.owned_bytes());let allocated=paged.allocated_bytes();assert_eq!(drain(&mut paged,&mut inline),allocated);}
            }}}
            eprintln!("[DEBUG] actual authored SetLabel directly streams every locale/terminology at original64k with full8194 English/original8205 German; short native cells own zero heap; initial4096 preserves actual prefix+metadata; no label String producer or flattening adapter; typed input remains a separate lifetime");
        }
        #[test]
        fn child_complete_actual_authored_label_receipt_joins_original8194_source_and_prefix(){
            use semio_framework_value::NativeEncodeControl;
            use semio_framework_os_kernel::{os_pack::codec::PackEncodeOptions,os_spr::operation_bytes::OperationByteOutput};
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🧾️label-receipt/🔣️.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            fn encode(output:&mut dyn OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_spr::ProtocolError>{output.write_bytes(b"17",control)?;for _ in 0..32{output.write_bytes(&[b' ';256],control)?;}Ok(())}
            let operation=crate::test_app_mutation_fixture::SetLabel{value:fixture["typedValue"].as_str().unwrap().to_owned()};for refused in[false,true]{let mut options=PackEncodeOptions::default();if refused{options.limits.max_segment_len=11;}let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);for(field,text)in[(reader::ChildGroupText::Owner,"root"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}let mut receipt=owner.produce_owned_operation(8194,&options,&mut control,|_,output,control|encode(output,control)).unwrap();receipt.read_first_schema_from_source(&if refused{"schema"}else{fixture["acceptedSchema"].as_str().unwrap()},&mut control).unwrap();let mut label_failed=false;for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let result=receipt.produce_label(terminology,locale,&mut control,|output|operation.label_into(terminology,locale,output));if refused{assert!(result.is_err());label_failed=true;break;}result.unwrap();}if label_failed{break;}}if refused{assert!(receipt.commit(&mut control).is_err());assert_eq!(owner.operations.len(),0);let view=owner.view().unwrap();assert_eq!(view.labels.len(),0);assert_eq!(view.schema.len(),11);}else{receipt.commit(&mut control).unwrap();assert_eq!(owner.operations.len(),1);assert!(owner.operations[0].iter().eq(b"17".iter().copied().chain(std::iter::repeat_n(b' ',8192))));let view=owner.view().unwrap();assert_eq!(view.labels.len(),1);for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let expected=match locale{semio_framework_ui_locale::Locale::En=>"Set label to target",semio_framework_ui_locale::Locale::De=>"Beschriftung auf target setzen"};let text=view.labels.text(0,terminology,locale).unwrap();assert_eq!(text.len(),expected.len());for(index,byte)in expected.as_bytes().iter().enumerate(){assert_eq!(text.byte(index).unwrap(),*byte)}}}}
                assert_eq!(owner.allocated_bytes(),control.owned_bytes());let allocated=owner.allocated_bytes();assert_eq!(drain_retained(&mut owner),allocated);
            }
            eprintln!("[DEBUG] actual authored locale producer joins the retained original8194 wire success receipt; intrinsic labels and first schema commit atomically at original64k; real segment refusal preserves zero prefix and all partial owners until1/4096; neutral byte producer is not credited as actual typed OpBinary");
        }
        #[test]
        fn child_complete_group_source_candidate_owned_publication_preserves_empty_original_and_parent(){
            mod publication{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/📢️publication/🦀️.rs");}
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep},paged_text::TextReadSource};
            use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
            use paged_encoder::PagedChildGroups;
            const CAPACITY:usize=isize::MAX as usize;
            struct Repeated;impl TextReadSource for Repeated{fn byte_len(&self)->usize{8194}fn byte_at(&self,index:usize)->Option<u8>{(index<8194).then_some(b'e')}}
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/📢️owned-publication/🔣️.json")).unwrap();let wire:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🎒️child-group-wire/🔣️.json")).unwrap();assert_eq!(fixture["originalGroups"],3);assert_eq!(fixture["originalSourceBytes"],8194);assert_eq!(fixture["emptyOwnerTextBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            fn append(children:&mut PagedList<paged_owner::PagedChildOwner,CAPACITY>,child:paged_owner::PagedChildOwner,control:&mut NativeEncodeControl<'_>){while !children.has_reserved_slot(){let required=children.next_allocation_bytes().unwrap();assert!(required<=4096);control.charge(required).unwrap();let step=children.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);}assert!(children.push_reserved(child).is_ok());}
            fn child(value:&serde_json::Value,options:&PackEncodeOptions,control:&mut NativeEncodeControl<'_>)->paged_owner::PagedChildOwner{let mut child=paged_owner::PagedChildOwner::empty();for(field,text)in[(reader::ChildGroupText::Owner,value["owner"].as_str().unwrap()),(reader::ChildGroupText::Slot,value["slot"].as_str().unwrap()),(reader::ChildGroupText::ChildId,value["child_id"].as_str().unwrap()),(reader::ChildGroupText::Schema,"child.empty")]{child.read_metadata_from_encoding_source(field,&text,options,control).unwrap();}let bytes=value["ops"][0].as_array().unwrap();let mut receipt=child.produce_owned_operation(bytes.len(),options,control,|_,output,control|{for byte in bytes{output.write_bytes(&[byte.as_u64().unwrap()as u8],control)?;}Ok(())}).unwrap();receipt.read_first_schema_from_source(&value["op_schema"].as_str().unwrap(),control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&value["labels"][0][terminology.as_str()][locale.as_str()].as_str().unwrap(),control).unwrap();}}receipt.commit(control).unwrap();child}
            for canceled in[false,true]{let options=PackEncodeOptions::default();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut children=PagedList::<paged_owner::PagedChildOwner,CAPACITY>::empty();let first=child(&wire["groups"][0],&options,&mut control);append(&mut children,first,&mut control);let mut empty=paged_owner::PagedChildOwner::empty();empty.read_metadata_from_encoding_source(reader::ChildGroupText::Owner,&Repeated,&options,&mut control).unwrap();for(field,text)in[(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"empty-child"),(reader::ChildGroupText::Schema,"child.empty")]{empty.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}append(&mut children,empty,&mut control);let second=child(&wire["groups"][1],&options,&mut control);append(&mut children,second,&mut control);let pointer=children[0].operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let allocated=children.allocated_bytes()+children.iter().map(paged_owner::PagedChildOwner::allocated_bytes).sum::<usize>();assert_eq!(allocated,control.owned_bytes());let mut incoming=Some(children);let mut owner=publication::ChildGroupPublicationOwner::take(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert!(owner.sources().is_err());assert!(!owner.step(0,4096,&mut control).unwrap());assert_eq!(owner.allocated_bytes(),allocated);assert_eq!(owner.original_count(),3);assert_eq!(owner.original(0).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(pointer));
                if canceled{let continuation=control.pause().unwrap();let mut reject=|_|false;let mut control=NativeEncodeControl::resume(continuation,&mut reject).unwrap();assert!(owner.step(1,4096,&mut control).is_err());assert!(owner.sources().is_err());assert_eq!(owner.original_count(),3);assert_eq!(owner.allocated_bytes(),allocated);assert_eq!(drain_retained(&mut owner),allocated);continue;}
                for _ in 0..32{if owner.step(1,4096,&mut control).unwrap(){break;}}let selected=owner.sources().unwrap();assert_eq!(selected.len(),2);assert_eq!(owner.original_count(),3);assert_eq!(owner.original(1).unwrap().operations.len(),0);assert_eq!(owner.original(1).unwrap().view().unwrap().owner.len(),8194);assert_eq!(owner.original(0).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(pointer));
                let mut encoded=OwnedOperationBytes::try_new(65536,65536).unwrap();let mut symbols=paged_encoder::ChildSymbolOwner::empty();let mut allow=|_|true;let mut encoding=NativeEncodeControl::new(65536,&mut allow);paged_encoder::encode_paged_groups_into(&selected,&options,&mut encoded,&mut symbols,&mut encoding).unwrap();let expected=wire["wire"].as_array().unwrap();assert_eq!(encoded.len(),expected.len());for(index,byte)in expected.iter().enumerate(){assert_eq!(encoded.byte_at(index),Some(byte.as_u64().unwrap()as u8));}assert_eq!(owner.original_count(),3);assert_eq!(owner.original(0).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(pointer));let writer_owned=encoded.allocated_bytes()+symbols.allocated_bytes();assert_eq!(writer_owned,encoding.owned_bytes());assert_eq!(drain_retained(&mut symbols)+{let bytes=encoded.allocated_bytes();drain(&mut encoded);bytes},writer_owned);
                let allocated=owner.allocated_bytes();assert_eq!(allocated,control.owned_bytes());let mut parent=ParentAllocationReturn::<128>::try_new(4096,allocated).unwrap();assert!(!owner.return_one(&mut parent,0,4096).unwrap().progressed);assert_eq!(owner.allocated_bytes(),allocated);let mut returned=0;for _ in 0..32768{if owner.terminal_is_empty(){break;}let before=owner.allocated_bytes();let step=owner.return_one(&mut parent,1,4096).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;assert!(owner.sources().is_err());}assert!(owner.terminal_is_empty());assert_eq!(returned,allocated);assert_eq!(parent.retained_bytes(),allocated);let mut physical=0;for _ in 0..256{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1&&released_bytes<=4096);physical+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(physical,allocated);
            }
            eprintln!("[DEBUG] owned publication binds original3 children to selected0,2 and exact16722 framing; original8194 source pointer and empty8194 semantic metadata survive; canceled selection retains all originals; real parent alone disposes every index/child/outer allocation1/4096");
        }
        #[test]
        fn child_complete_group_source_candidate_accepted_ledger_retains_original_registry_sources(){
            mod registry{include!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/🪪️registry/🦀️.rs");}
            use semio_framework_value::{ErasedSnapshotRetirement,RetainedCloneGrant,RetainedCloneStep};
            use semio_framework_tool_machine::{ScrubLedger,ScrubInput,ToolStep,ToolAbortReason};
            use semio_framework_os_kernel::{ActorId,HybridLogicalTimestamp};
            use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🪪️retained-source-registry/🔣️.json")).unwrap();
            assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            let reuse:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🔁️retained-source-registry-reuse/🔣️.json")).unwrap();assert_eq!(reuse["sourceBytes"],8194);assert_eq!(reuse["maximumAllocationBytes"],65536);assert_eq!(reuse["maximumItems"],1);assert_eq!(reuse["maximumBytes"],4096);assert_eq!(reuse["reusedIdentity"],3);
            fn child(control:&mut NativeEncodeControl<'_>)->paged_owner::PagedChildOwner{
                let options=PackEncodeOptions::default();let mut owner=paged_owner::PagedChildOwner::empty();
                for(field,text)in[(reader::ChildGroupText::Owner,"owner"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{owner.read_metadata_from_encoding_source(field,&text,&options,control).unwrap();}
                let mut receipt=owner.produce_owned_operation(8194,&options,control,|_,output,control|{output.write_bytes(b"17",control)?;for _ in 0..8192{output.write_bytes(b" ",control)?;}Ok(())}).unwrap();
                receipt.read_first_schema_from_source(&"label.set-label",control).unwrap();
                for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.read_label_from_source(terminology,locale,&"label",control).unwrap();}}
                eprintln!("[DEBUG] registry source before outer commit cumulative admitted bytes={}",control.owned_bytes());receipt.commit(control).unwrap();eprintln!("[DEBUG] registry full8194 source after commit cumulative={} retained={}",control.owned_bytes(),owner.allocated_bytes());owner
            }
            fn reconcile(owner:&mut registry::RetainedSourceRegistry<paged_owner::PagedChildOwner>,ledger:&ScrubLedger<registry::RetainedSourceIdentity>,control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_value::ValueError>{
                owner.reconcile(|identity,control|{for accepted in ledger.provisional(){control.checkpoint()?;control.step()?;if *accepted==identity{return Ok(true)}}Ok(false)},control)
            }
            for host_abort in[false,true]{
                let actor=ActorId("actor".into());let clock=|physical_ms|HybridLogicalTimestamp{actor:0,physical_ms,logical:0};let mut ledger=ScrubLedger::default();
                let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut owner=registry::RetainedSourceRegistry::empty();
                let first=child(&mut control);let first_allocated=first.allocated_bytes();let first_pointer=first.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let mut incoming=Some(first);let mut allow_zero=|_|true;let mut zero=NativeEncodeControl::new(0,&mut allow_zero);assert!(owner.insert(&mut incoming,&mut zero).is_err());assert_eq!(owner.retained_count(),0);assert!(owner.terminal_is_empty());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));let one=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());let scalar_clone=one.clone();let another_scalar_clone=scalar_clone.clone();assert_eq!(another_scalar_clone,one);
                ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"press".into(),leaves:vec![one]},clock(1)).unwrap();reconcile(&mut owner,&ledger,&mut control).unwrap();
                assert_eq!(owner.borrow(one).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));assert_eq!(owner.retained_count(),1);
                let second=child(&mut control);let second_pointer=second.operations[0].byte_ref(0).map(|byte|byte as *const u8).unwrap();let second_allocated=second.allocated_bytes();let mut incoming=Some(second);let continuation=control.pause().unwrap();let mut reject=|_|false;let mut rejected=NativeEncodeControl::resume(continuation,&mut reject).unwrap();
                assert!(owner.insert(&mut incoming,&mut rejected).is_err());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert!(reconcile(&mut owner,&ledger,&mut rejected).is_err());assert_eq!(owner.borrow(one).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(first_pointer));
                let continuation=rejected.pause().unwrap();let mut allow=|_|true;let mut control=NativeEncodeControl::resume(continuation,&mut allow).unwrap();let two=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert_eq!(owner.retained_count(),2);
                ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"press".into(),leaves:vec![two]},clock(2)).unwrap();
                assert!(owner.borrow(one).is_some());assert_eq!(ledger.provisional().copied().collect::<Vec<_>>(),vec![two]);reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.borrow(one).is_none());assert_eq!(owner.borrow(two).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert!(owner.take(one,&mut control).is_err());
                assert_eq!(owner.close_step(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(),RetainedCloneStep::Progress(Default::default()));let mut displaced=0;
                for _ in 0..16384{if owner.retained_count()==1{break}match owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Progress(progress)=>{let released_items=progress.copied_items;let released_bytes=progress.released_bytes;assert!(progress.copied_bytes<=4096);assert_eq!(progress.retained_capacity_bytes,0);assert!(released_items<=1&&released_bytes<=4096);displaced+=released_bytes;},RetainedCloneStep::Complete(_)=>panic!("live accepted source disappeared")}}
                assert_eq!(owner.retained_count(),1);assert_eq!(displaced,first_allocated);assert_eq!(owner.borrow(two).unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));
                if host_abort{
                    assert!(matches!(ledger.abort("window",Some("press"),ToolAbortReason::CaptureLost),ToolStep::Aborted(..)));assert!(ledger.is_empty());let mut incoming=Some(owner.take(two,&mut control).unwrap());assert_eq!(incoming.as_ref().unwrap().operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.borrow(two).is_none());let metadata=owner.allocated_bytes();let mut second_released=0;
                    for _ in 0..16384{match owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Progress(progress)=>{let released_items=progress.copied_items;let released_bytes=progress.released_bytes;assert!(progress.copied_bytes<=4096);assert_eq!(progress.retained_capacity_bytes,0);assert!(released_items<=1&&released_bytes<=4096);second_released+=released_bytes;if released_items==0&&released_bytes==0{break}},RetainedCloneStep::Complete(_)=>break}}
                    assert_eq!(owner.retained_count(),0);let third_pointer=second_pointer;let third_allocated=second_allocated;let before_readmission=control.owned_bytes();let three=owner.insert(&mut incoming,&mut control).unwrap();assert!(incoming.is_none());assert!(control.owned_bytes()>=before_readmission&&control.owned_bytes()<=65536);
                    ledger.send("window","tool",&actor,"base",ScrubInput::Tick{gesture:"new-press".into(),leaves:vec![three]},clock(3)).unwrap();let retained=owner.borrow(three).expect("open registry lost monotone identity after abort and bounded source drain");assert_eq!(retained.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(second_released,0);assert!(owner.allocated_bytes()>=metadata);reconcile(&mut owner,&ledger,&mut control).unwrap();
                    let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]},clock(4)).unwrap()else{panic!("reused open registry did not commit actual new press")};assert_eq!(leaves,vec![three]);let mut published=owner.take(three,&mut control).unwrap();assert_eq!(published.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(third_pointer));assert_eq!(published.operations[0].len(),8194);reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(three,&mut control).is_err());assert_eq!(drain_retained(&mut published),third_allocated);let metadata=owner.allocated_bytes();owner.begin_close();assert_eq!(drain_retained(&mut owner),metadata);
                    assert!(matches!(ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"new-press".into(),leaves:vec![three]},clock(5)).unwrap(),ToolStep::Idle));
                }else{
                    let ToolStep::Committed(_,leaves)=ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]},clock(3)).unwrap()else{panic!("accepted release did not commit")};assert_eq!(leaves,vec![two]);assert!(ledger.is_empty());
                    let mut published=owner.take(leaves[0],&mut control).unwrap();assert_eq!(published.operations[0].byte_ref(0).map(|byte|byte as *const u8),Some(second_pointer));assert_eq!(published.operations[0].len(),8194);assert_eq!(published.operations[0].byte_at(0),Some(b'1'));assert_eq!(published.operations[0].byte_at(1),Some(b'7'));for at in 2..8194{assert_eq!(published.operations[0].byte_at(at),Some(b' '));}
                    reconcile(&mut owner,&ledger,&mut control).unwrap();assert!(owner.take(two,&mut control).is_err());assert!(matches!(ledger.send("window","tool",&actor,"base",ScrubInput::Commit{gesture:"press".into(),leaves:vec![two]},clock(4)).unwrap(),ToolStep::Idle));
                    assert_eq!(drain_retained(&mut published),second_allocated);let metadata=owner.allocated_bytes();owner.begin_close();assert_eq!(drain_retained(&mut owner),metadata);
                }
                assert!(owner.terminal_is_empty());assert_eq!(owner.retained_count(),0);
            }
            eprintln!("[DEBUG] actual ScrubLedger scalar clones preserve original8194 sources; accepted tick displacement retires only absent original; extraction precedes accepted-ledger reconcile; commit transfers pointer once, late commit stays idle, host abort revokes authority first; all original/registry/publication allocations independently drain1/4096");
        }
        #[test]
        fn child_complete_actual_authored_operation_producer_joins_original_projection_and_retained_receipt(){
            use semio_framework_value::{ErasedSnapshotRetirement,NativeEncodeControl};
            use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
            use std::sync::atomic::{AtomicUsize,Ordering};
            struct Observed<'a>{inner:&'a mut dyn OperationByteOutput,accepted:&'a AtomicUsize}
            impl OperationByteOutput for Observed<'_>{fn write_bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{for byte in bytes{self.inner.write_bytes(&[*byte],control)?;self.accepted.fetch_add(1,Ordering::Relaxed);}Ok(())}}
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/📨️authored-operation-source/🔣️.json")).unwrap();assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["frameBytes"],8202);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            for mode in["complete","file-one-short","depth-zero","initial4096","interior-cancel"]{
                let mutation=crate::test_app_mutation_fixture::TestMutation::from(crate::test_app_mutation_fixture::SetLabel{value:"x".repeat(8194)});let input_pointer=match &mutation{crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=>leaf.value.as_ptr(),_=>unreachable!()};
                let accepted=AtomicUsize::new(0);let mut continue_encoding=|_:semio_framework_value::native_encoding::NativeEncodeProgress|mode!="interior-cancel"||accepted.load(Ordering::Relaxed)<256;let mut control=NativeEncodeControl::new(if mode=="initial4096"{4096}else{65536},&mut continue_encoding);let mut options=PackEncodeOptions::default();options.limits.max_file_len=if mode=="file-one-short"{8201}else{8202};if mode=="depth-zero"{options.limits.max_depth=0;}
                let mut child=paged_owner::PagedChildOwner::empty();for(field,text)in[(reader::ChildGroupText::Owner,"owner"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{child.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
                let mut capsule=mutation.retained_pack_operation();
                let result=child.produce_owned_operation(8202,&options,&mut control,|options,output,control|semio_framework_os_kernel::variants_binary::encode_op_into(&mut capsule,options,&mut Observed{inner:output,accepted:&accepted},control));
                match result{Ok(mut receipt)=>{assert_eq!(mode,"complete");receipt.read_first_schema_from_source(&fixture["expected"]["schema"].as_str().unwrap(),&mut control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{let crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=&mutation else{unreachable!()};receipt.produce_label(terminology,locale,&mut control,|output|leaf.label_into(terminology,locale,output)).unwrap();}}receipt.commit(&mut control).unwrap();},Err(_)=>assert_ne!(mode,"complete")}
                assert_eq!(match &mutation{crate::test_app_mutation_fixture::TestMutation::SetLabel(leaf)=>leaf.value.as_ptr(),_=>unreachable!()},input_pointer);
                if mode=="complete"{assert_eq!(child.operations.len(),1);assert_eq!(child.operations[0].len(),8202);let canonical=<crate::test_app_mutation_fixture::TestMutation as protocol::OpBinary>::encode_op(&mutation).unwrap();assert_eq!(canonical.len(),8202);for(index,byte)in canonical.iter().copied().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}let prefix=[1,1,0,1,0,7,130,64];for(index,byte)in prefix.into_iter().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}for index in 8..8202{assert_eq!(child.operations[0].byte_at(index),Some(b'x'));}let schema=fixture["expected"]["schema"].as_str().unwrap();let view=child.view().unwrap();assert_eq!(view.schema.len(),schema.len());for(index,byte)in schema.bytes().enumerate(){assert_eq!(view.schema.byte(index).unwrap(),byte);}}
                else{assert_eq!(child.operations.len(),0);assert!(child.retained_partial_source().is_some());if mode=="interior-cancel"{assert_eq!(accepted.load(Ordering::Relaxed),256);assert_eq!(child.retained_partial_source().unwrap().len(),256);}assert!(child.retained_partial_source().unwrap().len()<8202);}
                let scratch=capsule.allocated_bytes();let before=semio_framework_trace::retained_heap_bytes_on_this_thread();assert_eq!(capsule.retire_one(0,4096).unwrap(),(false,0,0));assert_eq!(semio_framework_trace::retained_heap_bytes_on_this_thread(),before);assert_eq!(capsule.allocated_bytes(),scratch);
                let mut released=0;for _ in 0..32768{let before=semio_framework_trace::retained_heap_bytes_on_this_thread();let step=capsule.retire_one(1,4096).unwrap();let after=semio_framework_trace::retained_heap_bytes_on_this_thread();assert_eq!(usize::try_from(before-after).unwrap(),step.2);assert!(step.1<=1&&step.2<=4096);released+=step.2;if !step.0{break}}assert_eq!(released,scratch);assert_eq!(capsule.allocated_bytes(),0);assert_eq!(capsule.variant_identity().unwrap_err().kind(),semio_framework_value::ValueRefusalKind::InvariantViolated);
                let allocated=child.allocated_bytes();assert_eq!(drain_retained(&mut child),allocated);assert!(child.terminal_is_empty());
            }
            eprintln!("[DEBUG] actual authored TestMutation direct projection emits exact8202 frame with full8194 payload into retained receipt and actual four locale cells; caller input pointer survives full-policy success/refusal; whole-file/depth/initial4096/interior256 cancellation retains actual source prefix and explicit1/4096 child closure; typed String and producer metadata/scaffold lifetime remain separate unqualified owners");
        }
        #[test]
        fn child_complete_actual_authored_typed_source_event_observer_matches_system_backing(){
            let(mut bytes,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||vec![b'x';8194]);assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8194,0));assert!(!allocated.overflowed);
            let(_,shortened)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.truncate(4096));assert_eq!((shortened.requested_bytes,shortened.released_bytes),(0,0));assert_eq!(bytes.capacity(),8194);
            let(_,resized)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.shrink_to_fit());assert_eq!(bytes.capacity(),4096);assert_eq!((resized.requested_bytes,resized.released_bytes,resized.largest_release_bytes),(4096,8194,8194));assert!(!resized.overflowed);
            let(_,released)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bytes));assert_eq!((released.requested_bytes,released.released_bytes,released.largest_release_bytes),(0,4096,4096));
            let(pages,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(vec![0u8;4096],vec![0u8;4096]));assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8192,0));let(inner,outer)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pages)).1);assert_eq!(inner,outer);assert_eq!((outer.requested_bytes,outer.released_bytes,outer.largest_release_bytes),(0,8192,4096));assert!(!outer.overflowed);
            eprintln!("[DEBUG] actual system truncate releases0; realloc8194-to4096 records original8194 release+new4096 request; nested two4096 releases total8192 with largest4096");
        }
        #[test]
        fn child_complete_actual_authored_typed_source_physical8194_grant4096(){
            use semio_framework_value::retirement::{RetireOwned,RetirementStep};
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/♻️typed-source-physical/🔣️.json")).unwrap();
            let backing_bytes=fixture["textBytes"].as_u64().unwrap() as usize;
            let work_bytes=fixture["maximumBytes"].as_u64().unwrap() as usize;
            let admission=fixture["maximumAllocationBytes"].as_u64().unwrap() as usize;
            assert_eq!((backing_bytes,work_bytes,admission),(8194,4096,65536));assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["textByte"],b'x');
            let mutation=crate::test_app_mutation_fixture::SetLabel{value:"x".repeat(backing_bytes)};
            let pointer=mutation.value.as_ptr();let backing=mutation.value.capacity();assert_eq!(backing,backing_bytes);
            let ((mut cursor,source_pointer),construction)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let source_pointer=mutation.value.as_ptr();(mutation.value.retirement(),source_pointer)});
            assert_eq!(pointer,source_pointer);assert_eq!(construction.released_bytes,0);assert!(!construction.overflowed);
            let (step,zero)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(Default::default()));
            assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((zero.requested_bytes,zero.released_bytes),(0,0));assert!(!cursor.terminal_is_empty());
            assert_eq!(cursor.next_work_byte_demand().unwrap(),0,"a String retires release-only: its bytes are never logically drained");
            let (step,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work_bytes,maximum_depth:1,..Default::default()}));
            assert!(matches!(step,RetirementStep::Advanced),"one unit truncates the logical text without copying or touching the backing");
            assert!(!observed.overflowed);assert_eq!((observed.requested_bytes,observed.released_bytes),(0,0));
            eprintln!("[DEBUG] actual typed String logical truncate copy=0 physicalRelease=0 originalBacking={backing}");
            for grant in fixture["physicalRelease"]["deniedBytes"].as_array().unwrap(){
                let bytes=grant.as_u64().unwrap() as usize;assert_eq!(cursor.next_close_byte_demand(),Some(backing));
                let (step,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_depth:1,maximum_release_bytes:bytes,..Default::default()}));
                assert!(matches!(step,RetirementStep::BudgetExhausted));assert!(!cursor.terminal_is_empty());
                assert_eq!(cursor.next_close_byte_demand(),Some(backing));assert_eq!((observed.requested_bytes,observed.released_bytes),(0,0));assert!(!observed.overflowed);
                eprintln!("[DEBUG] actual typed String physical denied grant={bytes} retainedDemand={backing} released=0");
            }
            let demand=cursor.next_close_byte_demand().expect("retained physical source demand");
            assert_eq!(demand,fixture["physicalRelease"]["demandBytes"].as_u64().unwrap() as usize);assert!(demand<=admission);
            let (step,released)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_depth:1,maximum_release_bytes:demand,..Default::default()}));
            let RetirementStep::Bytes(reported)=step else{panic!("whole funded source release must publish its exact physical receipt")};
            assert_eq!(reported,fixture["physicalRelease"]["reportedBytes"].as_u64().unwrap() as usize);
            assert_eq!((released.requested_bytes,released.released_bytes,released.largest_release_bytes),(0,reported,reported));assert!(!released.overflowed);assert_eq!(reported,backing);
            let (step,terminal)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work_bytes,maximum_depth:1,..Default::default()}));
            assert!(matches!(step,RetirementStep::Complete));assert!(cursor.terminal_is_empty());assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));assert!(!terminal.overflowed);
            let frame=cursor.terminal_release_bytes().expect("separate terminal cursor extent");
            assert_eq!(frame,std::mem::size_of_val(cursor.as_ref()));assert_eq!(frame,construction.requested_bytes);assert!(frame<=admission);assert_eq!(fixture["physicalRelease"]["terminalFrameSeparate"],true);
            let (_,scaffold)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));
            assert_eq!((scaffold.requested_bytes,scaffold.released_bytes,scaffold.largest_release_bytes),(0,frame,frame));assert!(!scaffold.overflowed);
            eprintln!("[DEBUG] actual typed String whole release grant={demand} physical={reported}; separate terminal frame funded={frame} physical={}",scaffold.released_bytes);
        }
        #[test]
        fn child_complete_actual_authored_paged_source8194_frame8202_physical4096(){
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,paged_text::TextReadSource,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
            use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
            struct Original;impl TextReadSource for Original{fn byte_len(&self)->usize{8194}fn byte_at(&self,index:usize)->Option<u8>{(index<8194).then_some(b'x')}}
            struct Observed<'a>{inner:&'a mut dyn OperationByteOutput,accepted:&'a std::sync::atomic::AtomicUsize}
            impl OperationByteOutput for Observed<'_>{fn write_bytes(&mut self,bytes:&[u8],control:&mut NativeEncodeControl<'_>)->Result<(),semio_framework_os_kernel::os_pack::PackRefusal>{for byte in bytes{self.inner.write_bytes(&[*byte],control)?;self.accepted.fetch_add(1,std::sync::atomic::Ordering::Relaxed);}Ok(())}}
            fn settle(owner:&mut dyn ErasedSnapshotRetirement)->usize{let mut total=0;for _ in 0..100000{let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_release_bytes:4096,maximum_depth:64,..Default::default()}).unwrap());let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};assert!(!event.overflowed);assert_eq!(event.requested_bytes,0);assert_eq!(event.released_bytes,progress.released_bytes);assert!(progress.copied_items<=1&&event.released_bytes<=4096&&event.largest_release_bytes<=4096);total+=event.released_bytes;if owner.terminal_is_empty(){break}}assert!(owner.terminal_is_empty());total}
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/🌱️authored-paged-source/🔣️.json")).unwrap();assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["frameBytes"],8202);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["metadataWindowBytes"],512);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
            for mode in fixture["modes"].as_array().unwrap().iter().map(|mode|mode.as_str().unwrap()){
                let mut source=crate::test_app_mutation_fixture::SetLabel::source_owner();let accepted=std::sync::atomic::AtomicUsize::new(0);let mut allow=|_|mode!="interior256"||accepted.load(std::sync::atomic::Ordering::Relaxed)<256;let mut control=NativeEncodeControl::new(if mode=="initial4096"{4096}else{65536},&mut allow);
                let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.read_from_encoding_source(&Original,&mut control));assert!(!event.overflowed);assert_eq!(event.released_bytes,0);assert_eq!(event.requested_bytes,source.allocated_bytes());assert_eq!(event.requested_bytes,control.owned_bytes());let original=source.allocated_bytes();
                if mode=="initial4096"{assert!(result.is_err());assert!(source.value_view().is_err());assert_eq!(settle(&mut source),original);continue}result.unwrap();let original_byte=source.value_view().unwrap().byte_at(0);assert_eq!(original_byte,Some(b'x'));
                let(mut capsule,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.retained_pack_operation());assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));assert_eq!(capsule.variant_identity().unwrap(),("set-label",1));
                if mode=="complete"{let mut measurement=semio_framework_os_kernel::os_spr::operation_bytes::OperationByteMeasurement::new(8202);let window=control.owned_bytes()+512;let(result,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||control.scoped_maximum(window,|control|Ok::<_,semio_framework_value::ValueError>(semio_framework_os_kernel::variants_binary::encode_op_into(&mut capsule,&PackEncodeOptions::default(),&mut measurement,control))));result.unwrap().unwrap();assert_eq!(measurement.exact_length().unwrap(),8202);assert_eq!((event.requested_bytes,event.released_bytes),(0,0));}
                let mut child=paged_owner::PagedChildOwner::empty();let mut options=PackEncodeOptions::default();options.limits.max_file_len=if mode=="file-one-short"{8201}else{8202};if mode=="depth-zero"{options.limits.max_depth=0;}
                for(field,text)in[(reader::ChildGroupText::Owner,"owner"),(reader::ChildGroupText::Slot,"slot"),(reader::ChildGroupText::ChildId,"child"),(reader::ChildGroupText::Schema,"child.empty")]{child.read_metadata_from_encoding_source(field,&text,&options,&mut control).unwrap();}
                let result=child.produce_owned_operation(8202,&options,&mut control,|options,output,control|semio_framework_os_kernel::variants_binary::encode_op_into(&mut capsule,options,&mut Observed{inner:output,accepted:&accepted},control));
                match result{Ok(mut receipt)=>{assert_eq!(mode,"complete");receipt.read_first_schema_from_source(&"plugin.testkit.document",&mut control).unwrap();for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{receipt.produce_label(terminology,locale,&mut control,|output|source.label_into(terminology,locale,output)).unwrap();}}receipt.commit(&mut control).unwrap();},Err(_)=>assert_ne!(mode,"complete")}
                assert_eq!(source.allocated_bytes(),original);assert_eq!(source.value_view().unwrap().byte_at(0),original_byte);assert!(control.owned_bytes()<=65536);
                if mode=="complete"{assert_eq!(child.operations.len(),1);assert_eq!(child.operations[0].len(),8202);for(index,byte)in[1,1,0,1,0,7,130,64].into_iter().enumerate(){assert_eq!(child.operations[0].byte_at(index),Some(byte));}for index in 8..8202{assert_eq!(child.operations[0].byte_at(index),Some(b'x'));}let view=child.view().unwrap();assert_eq!(view.schema.len(),"plugin.testkit.document".len());for terminology in semio_framework_ui_locale::Terminology::ALL{for(locale_index,locale)in semio_framework_ui_locale::Locale::ALL.into_iter().enumerate(){let label=view.labels.text(0,terminology,locale).unwrap();let prefix=fixture["labelPrefixes"][locale_index].as_str().unwrap();let suffix=fixture["labelSuffixes"][locale_index].as_str().unwrap();assert_eq!(label.len(),prefix.len()+8194+suffix.len());for(index,byte)in prefix.bytes().chain(std::iter::repeat_n(b'x',8194)).chain(suffix.bytes()).enumerate(){assert_eq!(label.byte(index).unwrap(),byte);}}}}
                else{assert_eq!(child.operations.len(),0);if mode=="interior256"{assert_eq!(accepted.load(std::sync::atomic::Ordering::Relaxed),256);assert_eq!(child.retained_partial_source().unwrap().len(),256);}}
                let scratch=capsule.allocated_bytes();let mut released=0;for _ in 0..32768{let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||capsule.retire_one(1,4096).unwrap());assert_eq!(event.requested_bytes,0);assert_eq!(event.released_bytes,step.2);assert!(step.1<=1&&event.released_bytes<=4096&&event.largest_release_bytes<=4096);released+=event.released_bytes;if !step.0{break}}assert_eq!(released,scratch);drop(capsule);let child_bytes=child.allocated_bytes();assert_eq!(settle(&mut child),child_bytes);assert_eq!(settle(&mut source),original);
                let(_,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{drop(child);drop(source)});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));eprintln!("[DEBUG] actual authored SetLabel origin mode={mode} originalBacking={original} frame8202 fourLocaleCells cumulative={} metadata512 source/capsule/child physical1/4096; original String TestMutation and eight ChildEmit carriers unqualified",control.owned_bytes());
            }
        }
        #[test]
        fn child_complete_candidate_retirement_keeps_currencies_independent(){
            use semio_framework_value::{NativeEncodeControl,ErasedSnapshotRetirement,paged_text::TextReadSource,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
            use semio_framework_os_kernel::os_pack::codec::PackEncodeOptions;
            struct Original;impl std::fmt::Display for Original{fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{for _ in 0..8194{formatter.write_str("x")?;}Ok(())}}impl TextReadSource for Original{fn byte_len(&self)->usize{8194}fn byte_at(&self,index:usize)->Option<u8>{(index<8194).then_some(b'x')}}
            let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/🔬️app-typed-command-full-operation/🧫️fixtures/♻️child-currencies/🔣️.json")).unwrap();assert_eq!(fixture["semanticBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumCopyBytes"],4096);assert_eq!(fixture["maximumCapacityBytes"],0);assert_eq!(fixture["maximumReleaseBytes"],4096);assert_eq!(fixture["maximumDepth"],64);
            for carrier in 0..8{
                let mut owner=paged_owner::PagedChildOwner::empty();let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let options=PackEncodeOptions::default();
                for(index,field)in[reader::ChildGroupText::Owner,reader::ChildGroupText::Slot,reader::ChildGroupText::ChildId,reader::ChildGroupText::Schema].into_iter().enumerate(){if index==carrier{owner.read_metadata_from_encoding_source(field,&Original,&options,&mut control).unwrap();}else{owner.read_metadata_from_encoding_source(field,&"cell",&options,&mut control).unwrap();}}
                if carrier>=4{let mut receipt=owner.produce_owned_operation(1,&options,&mut control,|_,output,control|output.write_bytes(&[0],control).map_err(Into::into)).unwrap();receipt.read_first_schema_from_source(&"schema",&mut control).unwrap();let mut index=4;for terminology in semio_framework_ui_locale::Terminology::ALL{for locale in semio_framework_ui_locale::Locale::ALL{if index==carrier{receipt.produce_label(terminology,locale,&mut control,|output|semio_framework_value::paged_text::write_encoding_format(output,format_args!("{Original}"))).unwrap();}else{receipt.produce_label(terminology,locale,&mut control,|output|semio_framework_value::paged_text::write_encoding_format(output,format_args!("cell"))).unwrap();}index+=1;}}receipt.commit(&mut control).unwrap();}
                let backing=owner.allocated_bytes();assert!(backing>=8194&&backing<=65536);assert_eq!(control.owned_bytes(),backing);let(mut copied,mut disposed)=(0,0);
                for _ in 0..100000{
                    if owner.terminal_is_empty(){break}let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();assert_eq!(owner.next_capacity_byte_demand(4096).unwrap(),0);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64};
                    if copy>0||release>0{let denied=RetainedCloneGrant{maximum_copy_bytes:if copy>0{0}else{4096},maximum_release_bytes:if release>0{release-1}else{4096},..grant};let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));}
                    let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};assert!(progress.fits(grant));assert!(!event.overflowed);assert_eq!(event.requested_bytes,0);assert_eq!(event.released_bytes,progress.released_bytes);assert!(event.released_bytes<=4096&&event.largest_release_bytes<=4096);copied+=progress.copied_bytes;disposed+=progress.released_bytes;
                }
                assert!(owner.terminal_is_empty());assert_eq!(disposed,backing);assert!(copied>=8194);assert_eq!(owner.next_copy_byte_demand().unwrap(),0);assert_eq!(owner.next_release_byte_demand().unwrap(),0);assert_eq!(owner.next_depth_demand().unwrap(),0);let(_,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));eprintln!("[DEBUG] original child candidate carrier={carrier} semantic8194 backing={backing} copied={copied} disposed={disposed}; independent1/4096 currencies observed; production ChildEmit carrier remains separate");
            }
        }
    }
}
