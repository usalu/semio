mod semantic_document_tests {
    use super::*;

    fn recovery_job(owner: &MountedProductReplayRecoveryOwner) -> u64 {
        match owner {
            MountedProductReplayRecoveryOwner::Request(request) | MountedProductReplayRecoveryOwner::Claim { request, .. } | MountedProductReplayRecoveryOwner::Authority { request, .. } => request.raw.job,
        }
    }

    fn document(generation: u64, surface: &str) -> UiDocumentLease {
        let id = ui_contract::UiNodeId(1);
        let surface = SurfaceId::try_from(surface).expect("bounded command retirement surface");
        let mut builder = UiDocumentBuilder::try_new(generation, surface, UiRevision(generation), Some(id), generation).expect("command retirement document slot");
        builder
            .try_push(ui_contract::UiNodeRecord {
                id,
                key: ui_contract::UiText::try_from_str("root").expect("bounded command retirement key"),
                component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
                layout: Default::default(),
                style: Default::default(),
                activity: Default::default(),
                disabled: false,
                transition: None,
                accessibility: Default::default(),
                bindings: Default::default(),
                menu: None,
                children: Default::default(),
            })
            .expect("command retirement root");
        builder.finish().expect("command retirement document")
    }

    fn retained_patch(surface: SurfaceId, base: u64, revision: u64) -> KernelUiPatch {
        let id = ui_contract::UiNodeId(1);
        let record = ui_contract::UiNodeRecord {
            id,
            key: ui_contract::UiText::try_from("root").expect("bounded hostile key"),
            component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
            layout: Default::default(),
            style: Default::default(),
            activity: Default::default(),
            disabled: false,
            transition: None,
            accessibility: Default::default(),
            bindings: Default::default(),
            menu: None,
            children: Default::default(),
        };
        let mut ops = ui_contract::UiPatchOps::default();
        ops.try_push(ui_contract::UiPatchOp::Upsert(record)).expect("hostile upsert owner");
        ops.try_push(ui_contract::UiPatchOp::SetRoot { id }).expect("hostile root owner");
        KernelUiPatch { surface, base_revision: UiRevision(base), revision: UiRevision(revision), ops }
    }

    #[test]
    fn renderer_sequence_maximum_is_once_then_permanently_exhausted_and_rollback_is_transactional() {
        let mut authority = RendererSequenceAuthority { committed: u64::MAX - 1, reserved: None, exhausted: false };
        let maximum = authority.try_reserve().expect("maximum is admitted once");
        assert_eq!(maximum, u64::MAX);
        authority.commit(maximum).expect("maximum commits once");
        let witness = (authority.committed, authority.reserved, authority.exhausted);
        assert!(authority.try_reserve().is_err());
        assert!(authority.try_reserve().is_err());
        assert_eq!((authority.committed, authority.reserved, authority.exhausted), witness);

        let mut rollback = RendererSequenceAuthority::default();
        let first = rollback.try_reserve().expect("first reservation");
        rollback.rollback(first);
        assert_eq!(rollback.try_reserve().expect("rolled back identity is reusable before commit"), first);
    }

    #[test]
    fn renderer_fixed_registries_return_exact_max_plus_one_surface_and_patch_owners() {
        let mut retained = RetainedSurfaceRegistry::new();
        for index in 0..RETAINED_SURFACE_CAPACITY {
            retained.try_admit(1, SurfaceId::try_from(format!("surface-{index}")).expect("bounded surface")).expect("maximum retained surface");
        }
        let overflow = SurfaceId::try_from("surface-overflow").expect("bounded surface");
        assert_eq!(retained.try_admit(1, overflow.clone()), Err(overflow));

        let mut rejections = PendingSurfaceRejectionRegistry::new();
        for index in 0..RETAINED_SURFACE_CAPACITY {
            let surface = SurfaceId::try_from(format!("rejection-{index}")).expect("bounded surface");
            rejections
                .try_admit(PendingSurfaceRejection {
                    generation: index as u64 + 1,
                    instance: 1,
                    surface: surface.clone(),
                    revision: UiRevision(index as u64),
                    reason: ui_contract::UiText::try_from_str("capacity").expect("bounded reason"),
                    patch: Some(retained_patch(surface, 0, index as u64 + 1)),
                    receipt: None,
                })
                .unwrap_or_else(|_| panic!("maximum pending rejection"));
        }
        let surface = SurfaceId::try_from("rejection-overflow").expect("bounded surface");
        let rejected = rejections
            .try_admit(PendingSurfaceRejection {
                generation: 999,
                instance: 1,
                surface: surface.clone(),
                revision: UiRevision(999),
                reason: ui_contract::UiText::try_from_str("capacity").expect("bounded reason"),
                patch: Some(retained_patch(surface, 0, 999)),
                receipt: None,
            })
            .expect_err("maximum plus one rejection");
        assert_eq!(rejected.patch.as_ref().map(|patch| patch.revision), Some(UiRevision(999)));
        while !rejections.terminal_is_empty() {
            let _ = rejections.take_any_one();
        }
        while !retained.advance_realm_close_one() {}
    }

    #[test]
    fn command_batch_ninth_document_is_retained_after_nonterminal_close_and_exactly_returned_or_retired() {
        let first = document(91_001, "command.retirement.first");
        let mut owners = UiFixedList::<ExchangeSurfaceDocument, 9>::default();
        for ordinal in 0..7 {
            owners
                .try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from(format!("command.retirement.alias-{ordinal}")).expect("bounded alias surface"), document: first.try_alias().expect("fixed command document alias") })
                .unwrap_or_else(|_| panic!("command alias owner"));
        }
        owners.try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from("command.retirement.primary").expect("bounded primary surface"), document: first }).unwrap_or_else(|_| panic!("command primary owner"));
        owners
            .try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from("command.retirement.ninth").expect("bounded ninth surface"), document: document(91_002, "command.retirement.ninth-document") })
            .unwrap_or_else(|_| panic!("command ninth owner"));
        let mut registry = CommandDocumentRetirementRegistry::new();
        let mut first_page = registry.try_reserve_page(7, 91_101).expect("first fixed page destinations");
        for _ in 0..UI_DOCUMENT_LEASE_SLOTS {
            registry.admit(&mut first_page, owners.swap_remove(0).expect("first page owner"));
        }
        registry.release(&mut first_page);
        let mut second_page = registry.try_reserve_page(7, 91_101).expect("second fixed page destinations");
        registry.admit(&mut second_page, owners.swap_remove(0).expect("ninth page owner"));
        registry.release(&mut second_page);
        registry.begin_close_batch(7, 91_100);
        assert!(!registry.has_close_work(), "stale batch generation cannot mutate pending document owners");
        let mut output = UiFixedList::default();
        registry.publish_batch(7, 91_101, &mut output);
        assert_eq!(output.len(), UI_DOCUMENT_LEASE_SLOTS);
        assert!(!registry.batch_is_empty(7, 91_101), "the ninth owner remains qualified in the close lane");
        while !registry.close_one() {}
        assert!(!registry.batch_is_empty(7, 91_101), "one nonterminal close opportunity cannot discard the ninth owner");
        let mut cleanup = registry.try_reserve_page(8, 91_102).expect("output cleanup destinations");
        for owner in output {
            registry.admit(&mut cleanup, owner);
        }
        registry.release(&mut cleanup);
        registry.begin_close_batch(8, 91_102);
        let mut opportunities = 1;
        while !registry.batch_is_empty(7, 91_101) || !registry.batch_is_empty(8, 91_102) {
            let _ = registry.close_one();
            opportunities += 1;
            assert!(opportunities < 16_384, "command document retirement must converge");
        }
    }

    #[test]
    fn command_document_page_saturation_refuses_before_turn_owner_production() {
        let mut registry = CommandDocumentRetirementRegistry::new();
        let mut reservations = UiFixedList::<CommandDocumentPageReservation, { COMMAND_DOCUMENT_RETIREMENT_CAPACITY / UI_DOCUMENT_LEASE_SLOTS }>::default();
        for generation in 1..=COMMAND_DOCUMENT_RETIREMENT_CAPACITY / UI_DOCUMENT_LEASE_SLOTS {
            reservations.try_push(registry.try_reserve_page(17, generation as u64).expect("absolute command document page credit")).unwrap_or_else(|_| panic!("fixed reservation owner"));
        }
        assert!(registry.try_reserve_page(17, 999).is_err(), "max plus one refuses before a guest turn can produce another document owner");
        for mut reservation in reservations {
            registry.release(&mut reservation);
        }
        assert!(registry.instance_is_empty(17));
    }

    #[test]
    fn stale_cancel_and_fault_document_builds_retain_closer_until_terminal() {
        let surface = SurfaceId::try_from("hostile.document-close").expect("bounded hostile surface");
        let mut retained = RetainedSurface::new(surface.clone());
        retained.admit_patch(retained_patch(surface, 0, 1)).expect("retained patch");
        while retained.patch.is_some() {
            let _ = retained.advance_patch_one();
        }
        retained.begin_document_build();
        retained.advance_document_one();
        let build = retained.build.as_mut().expect("builder retained");
        build.closing = true;
        retained.advance_document_one();
        assert!(retained.build.is_some(), "cancel/stale/fault first close opportunity retains a nonterminal closer");
        while retained.build.is_some() {
            retained.advance_document_one();
        }
        while !retained.close_step() {}
    }

    #[test]
    fn first_render_requires_multiple_fixed_opportunities_stale_patch_keeps_last_valid_and_close_reaches_terminal() {
        let surface_id = SurfaceId::try_from("hostile.first-render").expect("bounded hostile surface");
        let mut retained = RetainedSurface::new(surface_id.clone());
        retained.admit_patch(retained_patch(surface_id.clone(), 0, 1)).expect("first retained patch");
        assert!(retained.try_alias().is_none());
        let mut opportunities = 0;
        while retained.try_alias().is_none() {
            assert!(retained.advance_patch_one().is_none());
            if retained.patch.is_none() {
                retained.advance_document_one();
            }
            opportunities += 1;
            assert!(opportunities <= ui_contract::UI_DOCUMENT_PATCH_OPS + ui_contract::UI_DOCUMENT_NODES * ui_contract::UI_DOCUMENT_NODES);
        }
        assert!(opportunities > 1);
        let mut last_valid = retained.try_alias().expect("published first document");
        let generation = last_valid.generation();
        retained.admit_patch(retained_patch(surface_id, 0, 2)).expect("stale patch owner retained");
        while retained.advance_patch_one().is_none() {}
        assert_eq!(retained.try_alias().expect("last valid survives stale patch").generation(), generation);
        let _ = last_valid.close_step();
        drop(last_valid);
        while !retained.close_step() {}
    }

    #[test]
    fn mounted_job_progress_presentation_bridge_is_fixed_fifo_and_generation_checked() {
        let mut bridge = JobProgressPresentationBridge::new();
        let identity = |ordinal: usize| JobProgressIdentity { actor: ActorId(ordinal as u64 + 1), job: 7, operation: ordinal as u64 + 11, base_revision: 13, generation: 17, step_sequence: 0, preview_sequence: 1 };
        let mut tokens = Vec::new();
        for ordinal in 0..JOB_PROGRESS_PRESENTATION_CAPACITY {
            let token = bridge.reserve(identity(ordinal), JobProgressKind::Preview, ordinal as u64).expect("fixed presentation slot");
            assert!(bridge.publish(token));
            tokens.push(token);
        }
        assert!(bridge.reserve(identity(999), JobProgressKind::Preview, 0).is_none(), "capacity +1 must reject before publication ownership moves");
        for (ordinal, token) in tokens.into_iter().enumerate() {
            let mut lease = bridge.take().expect("FIFO presentation lease");
            assert_eq!(bridge.slots[lease.token.index].identity, identity(ordinal));
            assert_eq!(lease.token, token);
            assert!(bridge.presented(token));
            lease.terminal = true;
            assert!(bridge.release_presented(token));
            assert!(!bridge.release_presented(token), "duplicate generation release is rejected");
        }
        assert!(bridge.take().is_none());
        assert!(bridge.terminal_is_empty(), "realm witness requires every fixed presentation slot to be vacant");
    }

    #[test]
    fn cancelled_head_does_not_strand_later_ready_presentations() {
        let mut bridge = JobProgressPresentationBridge::new();
        let identity = |actor: u64| JobProgressIdentity { actor: ActorId(actor), job: 7, operation: actor + 11, base_revision: 13, generation: 17, step_sequence: 0, preview_sequence: 1 };
        let head = bridge.reserve(identity(1), JobProgressKind::Preview, 1).expect("head");
        let middle = bridge.reserve(identity(2), JobProgressKind::Preview, 2).expect("middle");
        let tail = bridge.reserve(identity(3), JobProgressKind::Preview, 3).expect("tail");
        assert!(bridge.publish(head));
        assert!(bridge.publish(middle));
        assert!(bridge.publish(tail));
        assert!(bridge.cancel(head));

        let mut returned = bridge.take().expect("cancelled head is skipped");
        assert_eq!(returned.token, middle);
        assert!(bridge.return_lease(middle));
        returned.terminal = true;
        let mut middle_lease = bridge.take().expect("returned oldest lease retains admitted order");
        assert_eq!(middle_lease.token, middle);
        assert!(bridge.presented(middle));
        middle_lease.terminal = true;
        assert!(bridge.release_presented(middle));

        let mut tail_lease = bridge.take().expect("tail remains reachable after cancelled hole");
        assert_eq!(tail_lease.token, tail);
        assert!(bridge.presented(tail));
        tail_lease.terminal = true;
        assert!(bridge.release_presented(tail));
        assert!(bridge.terminal_is_empty());
    }

    #[test]
    fn mounted_replay_recovery_capacity_plus_one_refuses_without_touching_reserved_identity() {
        let mut registry = MountedReplayRecoveryRegistry::new();
        let tokens: [_; JOB_PROGRESS_ACTIVE_CAPACITY] = std::array::from_fn(|_| registry.reserve().expect("fixed recovery reservation"));
        assert!(registry.reserve().is_none());
        for token in tokens {
            assert!(registry.release(token));
            assert!(!registry.release(token));
        }
        assert!(!registry.has_close_work());
    }

    #[test]
    fn rejected_production_replay_submit_retries_exact_restore_start_identity_before_job_step() {
        let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved submit recovery");
        let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, document: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
        let authority = JobTurn { job: 19, operation: JobOperation { operation: 23, base_revision: 29, generation: 31, preview_sequence: 0, seed: 37 }, step_sequence: 0 };
        let request = JobReplayRequest::from_spawn("fixture.production-retry", b"retained-input");
        let mut mounted = MountedJobReplay {
            actor: ActorId(41),
            authority,
            request,
            placement: JobPlacement::Isolated,
            log: ManuallyDrop::new(JobReplayLog::new(route, 31).expect("generation-qualified replay")),
            captured: None,
            policy: None,
            terminal_seen: true,
            replay_requested: true,
            replay_started: false,
            replay_submit_sequence: None,
            accepted_replay_sequence: None,
            replay_worker_count: 4,
            replay_worker_slot: 3,
            recovery: Some(recovery),
        };
        let first = mounted.replay_submission_sequence().expect("first retained restore/start packet");
        let rejected_identity = (mounted.actor, mounted.authority, mounted.request, mounted.replay_worker_count, mounted.replay_worker_slot, first);
        assert!(!mounted.replay_started);
        assert!(mounted.pending_replay_start_is_exact(4, 3));
        assert!(!mounted.pending_replay_start_is_exact(2, 3));
        assert!(!mounted.pending_replay_start_is_exact(4, 2));
        let retry = mounted.replay_submission_sequence().expect("backpressure retries retained restore/start packet");
        assert_eq!((mounted.actor, mounted.authority, mounted.request, mounted.replay_worker_count, mounted.replay_worker_slot, retry), rejected_identity);
        assert!(!mounted.replay_started, "a rejected runtime submit cannot make the next opportunity a JobStep");
        mounted.accept_replay_submission(retry).expect("accepted exact restore/start packet");
        assert!(mounted.replay_started);
        assert_eq!(mounted.replay_submit_sequence, None);
        drop(mounted);
        while mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
            assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
        }
    }

    fn admitted_product_request(instance: u32, job: u64, kind: &str, input: &[u8], placement: JobPlacement) -> MountedProductReplayRequest {
        let recovery = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve().expect("fixed product replay request recovery");
        MountedProductReplayRequest::from_admitted_effect(instance, 0, Effect::SpawnJob { job, kind: kind.to_string(), input: input.to_vec(), placement }, recovery)
    }

    fn product_admission_permit(instance: u32) -> MountedProductReplayAdmissionPermit {
        let token = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(instance).expect("fixed product refusal permit");
        MountedProductReplayAdmissionPermit { token: Some(token) }
    }

    fn drain_product_recovery() -> usize {
        let mut opportunities = 0;
        while mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
            assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one());
            opportunities += 1;
        }
        opportunities
    }

    fn product_authority_fixture() -> (MountedProductReplayAuthority, MountedProductReplayExpected) {
        let kind = "action-bus.production";
        let mut request_owner = admitted_product_request(59, 7, kind, b"fixed-input", JobPlacement::Isolated);
        let request = request_owner.request;
        let route =
            JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: request.controller, tool: request.tool, window: 5, document: [6; 32], request_schema: request.schema, request_version: request.version, request_digest: request.digest };
        let operation = JobOperation { operation: 11, base_revision: 13, generation: 17, preview_sequence: 0, seed: 19 };
        let turn = JobTurn { job: 7, operation, step_sequence: 23 };
        let terminal = JobReplayRecordHeader {
            actor: ActorId(29),
            turn,
            ordinal: 31,
            granted_fuel: 37,
            deadline_class_ms: 4,
            worker_count: 7,
            worker_slot: 3,
            cancellation_observed: false,
            kind: JobReplayPublicationKind::Commit,
            policy: JobReplayPublicationPolicy::Accepted,
            applied_progress: 41,
            payload_items: 2,
            payload_bytes: 43,
            payload_digest: 47,
            prefix_digest: 53,
        };
        let checkpoint = Some(MountedProductReplayCheckpoint { ordinal: 61, digest: 67, pages: 2, applied_progress: 69 });
        request_owner.raw_mut().acknowledge_mount();
        request_owner.raw_mut().acknowledge_qualification(checkpoint);
        let expected = MountedProductReplayExpected {
            instance: 59,
            actor: terminal.actor,
            turn,
            request,
            placement: JobPlacement::Isolated,
            route,
            checkpoint,
            terminal,
            process_worker_count: 7,
            process_worker_slot: 3,
            logical_worker_count: 7,
            logical_worker_slot: 3,
            begin: true,
            restore_start_ordinal: Some(71),
        };
        (
            MountedProductReplayAuthority {
                request_owner: Some(request_owner),
                actor: terminal.actor,
                route,
                operation,
                generation: operation.generation,
                seed: operation.seed,
                checkpoint,
                terminal,
                worker_count: 7,
                worker_slot: 3,
                begin: true,
                restore_start_ordinal: Some(71),
                profile_cursor: 0,
                profile_started: false,
            },
            expected,
        )
    }

    #[test]
    fn production_action_bus_product_authority_carries_complete_identity_and_returns_exact_owner_on_wrong_field() {
        let (authority, expected) = product_authority_fixture();
        let authority = authority.validate(expected).expect("complete typed authority");
        assert_eq!(authority.request_owner().job_kind(), b"action-bus.production");
        assert_eq!((authority.instance(), authority.job(), authority.operation, authority.generation, authority.seed), (59, 7, expected.turn.operation, 17, 19));
        assert_eq!(authority.request_owner().placement, JobPlacement::Isolated);
        assert_eq!(authority.checkpoint, Some(MountedProductReplayCheckpoint { ordinal: 61, digest: 67, pages: 2, applied_progress: 69 }));
        assert_eq!((authority.worker_count, authority.worker_slot, authority.begin, authority.restore_start_ordinal), (7, 3, true, Some(71)));
        authority.retire();

        let (authority, mut wrong) = product_authority_fixture();
        wrong.checkpoint.as_mut().expect("checkpoint witness").digest ^= 1;
        let token = authority.request_owner().recovery.expect("pre-reserved exact handback");
        let rejected = authority.validate(wrong).expect_err("wrong checkpoint returns exact authority");
        assert_eq!((rejected.instance(), rejected.job(), rejected.generation, rejected.seed, rejected.terminal.prefix_digest), (59, 7, 17, 19, 53));
        rejected.retire();
        {
            let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert_eq!(registry.slots[token.index].generation, token.generation);
            assert_eq!(registry.slots[token.index].owner.as_ref().map(recovery_job), Some(7));
        }
        while mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
            assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one());
        }
    }

    fn assert_product_authority_field_rejected(mutate: fn(&mut MountedProductReplayAuthority, &mut MountedProductReplayExpected)) {
        let (mut authority, mut expected) = product_authority_fixture();
        let token = authority.request_owner().recovery.expect("pre-reserved field rejection");
        mutate(&mut authority, &mut expected);
        let rejected = authority.validate(expected).expect_err("wrong product authority field must return the exact shell");
        assert_eq!((rejected.instance(), rejected.job()), (59, 7));
        rejected.retire();
        let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert_eq!((registry.slots[token.index].generation, registry.slots[token.index].owner.as_ref().map(recovery_job)), (token.generation, Some(7)));
        while registry.has_close_work() {
            assert!(registry.close_one());
        }
    }

    #[test]
    fn production_product_authority_qualifies_every_minted_field_before_replay_work() {
        assert_product_authority_field_rejected(|authority, _| authority.request_owner.as_mut().expect("request").job_kind[0] ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.request.digest[0] ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.placement = JobPlacement::Exclusive);
        assert_product_authority_field_rejected(|_, expected| expected.route.request_digest[0] ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.turn.operation.operation ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.turn.operation.generation ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.turn.operation.seed ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").ordinal ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").digest ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").pages ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").applied_progress ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.terminal.ordinal ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.terminal.payload_digest ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.terminal.prefix_digest ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.process_worker_count ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.process_worker_slot ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.logical_worker_count ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.logical_worker_slot ^= 1);
        assert_product_authority_field_rejected(|_, expected| expected.begin = false);
        assert_product_authority_field_rejected(|_, expected| expected.restore_start_ordinal = Some(72));
    }

    #[test]
    fn product_authority_fault_abort_and_close_rediscover_exact_generation_once() {
        let (authority, _) = product_authority_fixture();
        let token = authority.request_owner().recovery.expect("pre-reserved abort handback");
        let mut authorities: [Option<MountedProductReplayAuthority>; JOB_PROGRESS_ACTIVE_CAPACITY] = std::array::from_fn(|_| None);
        authorities[0] = Some(authority);
        authorities[0].take().expect("fault retains exact authority").retire();
        let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert_eq!((registry.slots[token.index].generation, registry.slots[token.index].owner.as_ref().map(recovery_job)), (token.generation, Some(7)));
        while registry.has_close_work() {
            assert!(registry.close_one());
        }
        assert!(!registry.has_close_work());
    }

    #[test]
    fn product_claim_fault_retains_exact_request_cursor_checkpoint_and_generation() {
        let request = admitted_product_request(83, 89, "action-bus.claim", b"claim-input", JobPlacement::Exclusive);
        let token = request.recovery.expect("pre-reserved claim handback");
        MountedProductReplayClaim { request: Some(request), actor: ActorId(97), record_cursor: 101, checkpoint: Some(MountedProductReplayCheckpoint { ordinal: 103, digest: 107, pages: 2, applied_progress: 109 }) }.retire();
        let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
        assert_eq!(registry.slots[token.index].generation, token.generation);
        assert!(matches!(
            registry.slots[token.index].owner.as_ref(),
            Some(MountedProductReplayRecoveryOwner::Claim {
                request,
                actor: ActorId(97),
                record_cursor: 101,
                checkpoint: Some(MountedProductReplayCheckpoint { ordinal: 103, digest: 107, pages: 2, applied_progress: 109 }),
            }) if request.instance == 83 && request.raw.job == 89 && request.placement == JobPlacement::Exclusive
        ));
        while registry.has_close_work() {
            assert!(registry.close_one());
        }
        assert!(!registry.has_close_work());
    }

    #[test]
    fn product_effect_after_normal_exchange_is_moved_into_fixed_authority_without_json_escape_hatch() {
        let mut outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![
                Effect::Notify { message: "before".to_string() },
                Effect::SpawnJob { job: 73, kind: "action-bus.production".to_string(), input: vec![79, 83], placement: JobPlacement::Inline },
                Effect::Notify { message: "after".to_string() },
            ],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let (kind_pointer, input_pointer) = match &outcome.effects[1] {
            Effect::SpawnJob { kind, input, .. } => (kind.as_ptr(), input.as_ptr()),
            _ => panic!("selected product source remains SpawnJob"),
        };
        let authority = match outcome.take_product_replay_authority(89, product_admission_permit(89)) {
            MountedProductReplayAdmission::Admitted(authority) => authority,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("fixed product exchange must admit"),
        };
        assert_eq!((authority.instance, authority.job, authority.job_kind(), authority.placement), (89, 73, b"action-bus.production".as_slice(), JobPlacement::Inline));
        assert_eq!(authority.request, JobReplayRequest::from_spawn("action-bus.production", &[79, 83]));
        assert_eq!((authority.raw().kind().as_ptr(), authority.raw().input().as_ptr(), authority.raw().selected_index), (kind_pointer, input_pointer, 1));
        assert!(matches!(&outcome.effects[..], [Effect::Notify { message: before }, Effect::Notify { message: after }] if before == "before" && after == "after"));
        authority.retire();
        assert_eq!(drain_product_recovery(), 2);
    }

    #[test]
    fn admitted_product_mount_refusal_returns_exact_raw_allocation_then_closes_one_backing_per_grant() {
        let mut outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![Effect::SpawnJob { job: 91, kind: "raw.mount-refusal".to_string(), input: vec![2, 3, 5, 7], placement: JobPlacement::Exclusive }],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let (kind_pointer, input_pointer) = match &outcome.effects[0] {
            Effect::SpawnJob { kind, input, .. } => (kind.as_ptr(), input.as_ptr()),
            _ => panic!("mount-refusal source remains SpawnJob"),
        };
        let request = match outcome.take_product_replay_authority(93, product_admission_permit(93)) {
            MountedProductReplayAdmission::Admitted(owner) => owner,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("bounded raw request must admit"),
        };
        let token = request.recovery.expect("admitted request keeps exact recovery");
        let rejected: Result<(), MountedProductReplayRequest> = Err(request);
        let request = rejected.expect_err("unregistered mount returns the same request owner");
        assert_eq!((request.raw().kind().as_ptr(), request.raw().input().as_ptr(), request.raw().selected_index), (kind_pointer, input_pointer, 0));
        request.retire();
        {
            let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert!(
                matches!(registry.slots[token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Request(request)) if request.raw.kind().as_ptr() == kind_pointer && request.raw.input().as_ptr() == input_pointer && request.raw.disposition == RawSpawnJobDisposition::Rejected)
            );
        }
        assert_eq!(drain_product_recovery(), 2, "raw input and kind backing close on separate grants");
    }

    #[test]
    fn admitted_product_request_claim_and_authority_drop_preserve_raw_allocation_identity() {
        let request = admitted_product_request(95, 97, "raw.request-drop", b"request", JobPlacement::Inline);
        let request_token = request.recovery.expect("request drop recovery");
        let request_identity = (request.raw().kind().as_ptr(), request.raw().input().as_ptr());
        drop(request);
        {
            let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert!(matches!(registry.slots[request_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Request(request)) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == request_identity));
        }
        assert_eq!(drain_product_recovery(), 2);

        let mut request = admitted_product_request(101, 103, "raw.claim-drop", b"claim", JobPlacement::Isolated);
        request.raw_mut().acknowledge_mount();
        let claim_token = request.recovery.expect("claim drop recovery");
        let claim_identity = (request.raw().kind().as_ptr(), request.raw().input().as_ptr());
        drop(MountedProductReplayClaim { request: Some(request), actor: ActorId(107), record_cursor: 0, checkpoint: None });
        {
            let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert!(matches!(registry.slots[claim_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Claim { request, .. }) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == claim_identity));
        }
        assert_eq!(drain_product_recovery(), 2);

        let (authority, _) = product_authority_fixture();
        let authority_token = authority.request_owner().recovery.expect("authority drop recovery");
        let authority_identity = (authority.request_owner().raw().kind().as_ptr(), authority.request_owner().raw().input().as_ptr());
        drop(authority);
        {
            let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert!(matches!(registry.slots[authority_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Authority { request, .. }) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == authority_identity));
        }
        assert_eq!(drain_product_recovery(), 2);
    }

    #[test]
    fn admitted_raw_backing_retires_only_after_fixed_mount_qualification_and_all_replay_acks() {
        let (mut authority, _) = product_authority_fixture();
        assert!(authority.acknowledge_raw_retirement_boundary().is_err(), "raw backing cannot retire before every replay profile ACK");
        authority.profile_cursor = PRODUCT_REPLAY_PROFILE_COUNT;
        authority.profile_started = false;
        authority.restore_start_ordinal = Some(113);
        authority.acknowledge_raw_retirement_boundary().expect("all fixed, mount, qualification, terminal, and replay ACKs admit retirement");
        let raw = authority.request_owner().raw();
        assert_eq!(
            (raw.fixed_witness_acked, raw.mount_acked, raw.qualification_acked, raw.replay_acked, raw.terminal_ordinal, raw.terminal_prefix, raw.accepted_replay_ordinal, raw.disposition),
            (true, true, true, true, Some(authority.terminal.ordinal), Some(authority.terminal.prefix_digest), Some(113), RawSpawnJobDisposition::Accepted)
        );
        authority.retire();
        assert_eq!(drain_product_recovery(), 2);
    }

    #[test]
    fn product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder() {
        let kind_max = "k".repeat(PRODUCT_REPLAY_KIND_BYTES);
        let mut kind_max_outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![Effect::SpawnJob { job: 101, kind: kind_max, input: vec![1], placement: JobPlacement::Inline }],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let admitted = match kind_max_outcome.take_product_replay_authority(103, product_admission_permit(103)) {
            MountedProductReplayAdmission::Admitted(owner) => owner,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("kind MAX must admit"),
        };
        admitted.retire();
        assert_eq!(drain_product_recovery(), 2);

        let input_max = vec![2; PRODUCT_REPLAY_INPUT_BYTES];
        let mut input_max_outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![Effect::SpawnJob { job: 107, kind: "input.max".to_string(), input: input_max, placement: JobPlacement::Isolated }],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let admitted = match input_max_outcome.take_product_replay_authority(109, product_admission_permit(109)) {
            MountedProductReplayAdmission::Admitted(owner) => owner,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("input MAX must admit"),
        };
        admitted.retire();
        assert_eq!(drain_product_recovery(), 2);

        let kind_max_plus_one = "z".repeat(PRODUCT_REPLAY_KIND_BYTES + 1);
        let mut kind_refusal_outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![
                Effect::Notify { message: "before-kind".to_string() },
                Effect::SpawnJob { job: 113, kind: kind_max_plus_one.clone(), input: vec![3, 5], placement: JobPlacement::Exclusive },
                Effect::Notify { message: "after-kind".to_string() },
            ],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let refusal = match kind_refusal_outcome.take_product_replay_authority(127, product_admission_permit(127)) {
            MountedProductReplayAdmission::Refused(owner) => owner,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("kind MAX+1 must return typed refusal"),
        };
        assert_eq!((refusal.instance, refusal.cause, refusal.selected_index), (127, MountedProductReplayRefusalCause::KindCapacity, 1));
        assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 113, kind, input, placement: JobPlacement::Exclusive }) if kind == &kind_max_plus_one && input == &[3, 5]));
        assert!(matches!(refusal.remaining_effects.as_deref(), Some([Effect::Notify { message: before }, Effect::Notify { message: after }]) if before == "before-kind" && after == "after-kind"));
        assert!(kind_refusal_outcome.effects.is_empty());
        refusal.retire();
        let mut kind_close_opportunities = 0;
        while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
            assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
            kind_close_opportunities += 1;
            assert!(kind_close_opportunities <= 8);
        }
        assert!(kind_close_opportunities >= 6);

        let input_max_plus_one = vec![11; PRODUCT_REPLAY_INPUT_BYTES + 1];
        let mut input_refusal_outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![Effect::SpawnJob { job: 131, kind: "input.max-plus-one".to_string(), input: input_max_plus_one, placement: JobPlacement::Inline }],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let refusal = match input_refusal_outcome.take_product_replay_authority(137, product_admission_permit(137)) {
            MountedProductReplayAdmission::Refused(owner) => owner,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("input MAX+1 must return typed refusal"),
        };
        assert_eq!(refusal.cause, MountedProductReplayRefusalCause::InputCapacity);
        assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 131, input, .. }) if input.len() == PRODUCT_REPLAY_INPUT_BYTES + 1));
        refusal.retire();
        while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
            assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
        }
    }

    #[test]
    fn product_ingress_recovery_max_plus_one_retries_exact_spawn_then_closes_remainder_incrementally() {
        let reservations: [MountedProductReplayRecoveryToken; JOB_PROGRESS_ACTIVE_CAPACITY] = {
            let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            std::array::from_fn(|_| registry.reserve().expect("fill every product recovery slot"))
        };
        let mut outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![
                Effect::Notify { message: "before-recovery".to_string() },
                Effect::SpawnJob { job: 139, kind: "recovery.max-plus-one".to_string(), input: vec![13, 17, 19], placement: JobPlacement::Isolated },
                Effect::Notify { message: "after-recovery".to_string() },
            ],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let refusal = match outcome.take_product_replay_authority(149, product_admission_permit(149)) {
            MountedProductReplayAdmission::Refused(owner) => owner,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("recovery MAX+1 must return typed refusal"),
        };
        assert_eq!((refusal.cause, refusal.selected_index), (MountedProductReplayRefusalCause::RecoveryCapacity, 1));
        assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 139, kind, input, placement: JobPlacement::Isolated }) if kind == "recovery.max-plus-one" && input == &[13, 17, 19]));
        assert_eq!(refusal.remaining_effects.as_ref().map(Vec::len), Some(2));
        refusal.retire();
        assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(reservations[0]));
        let step = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(true);
        let request = step.request.expect("one recovery opportunity retries the exact fixed request");
        assert_eq!(
            (request.instance, request.job, request.job_kind(), request.request, request.placement),
            (149, 139, b"recovery.max-plus-one".as_slice(), JobReplayRequest::from_spawn("recovery.max-plus-one", &[13, 17, 19]), JobPlacement::Isolated)
        );
        request.retire();
        for token in reservations.into_iter().skip(1) {
            assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(token));
        }
        let raw_close_opportunities = drain_product_recovery();
        assert_eq!(raw_close_opportunities, 2);
        let mut close_opportunities = 0;
        while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
            assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
            close_opportunities += 1;
            assert!(close_opportunities <= 8);
        }
        assert!(raw_close_opportunities + close_opportunities >= 6, "raw input, kind, two effects, and remainder backing close remain distinct opportunities");
    }

    #[test]
    fn product_refusal_slot_max_plus_one_stops_before_exchange_and_drop_publishes_exact_generation() {
        let permits: [MountedProductReplayRefusalToken; JOB_PROGRESS_ACTIVE_CAPACITY] = {
            let mut registry = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock");
            std::array::from_fn(|index| registry.reserve(index as u32 + 1).expect("fill fixed refusal permits"))
        };
        assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(u32::MAX).is_none(), "refusal MAX+1 is denied before ActionBus exchange owns a SpawnJob");
        for token in permits {
            assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token));
        }

        let mut outcome = ExchangeOutcome {
            frames: Vec::new(),
            surfaces: UiFixedList::default(),
            effects: vec![
                Effect::Notify { message: "drop-before".to_string() },
                Effect::SpawnJob { job: 151, kind: "d".repeat(PRODUCT_REPLAY_KIND_BYTES + 1), input: vec![23, 29], placement: JobPlacement::Exclusive },
                Effect::Notify { message: "drop-after".to_string() },
            ],
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
        };
        let refusal = match outcome.take_product_replay_authority(157, product_admission_permit(157)) {
            MountedProductReplayAdmission::Refused(owner) => owner,
            MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("oversized kind must refuse"),
        };
        let token = refusal.token.expect("typed refusal retains its pre-reserved generation");
        drop(refusal);
        {
            let registry = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock");
            let owner = registry.slots[token.index].owner.as_ref().expect("ordinary refusal Drop publishes exact owner");
            assert_eq!((registry.slots[token.index].generation, owner.instance, owner.cause, owner.selected_index), (token.generation, 157, MountedProductReplayRefusalCause::KindCapacity, 1));
            assert!(matches!(owner.spawn.as_ref(), Some(Effect::SpawnJob { job: 151, input, placement: JobPlacement::Exclusive, .. }) if input == &[23, 29]));
            assert_eq!(owner.remaining_effects.as_ref().map(Vec::len), Some(2));
        }
        let mut opportunities = 0;
        while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
            assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
            opportunities += 1;
            assert!(opportunities <= 8);
        }
        assert!(opportunities >= 6);
    }

    #[test]
    fn production_mounted_process_replay_supports_one_two_four_and_default_without_private_pool() {
        let profiles: [_; PRODUCT_REPLAY_PROFILE_COUNT] = std::array::from_fn(|cursor| mounted_product_replay_profile(7, cursor).expect("same-process logical replay profile"));
        assert_eq!(profiles, [1, 2, 4, 7]);
        assert_eq!(profiles.map(|worker_count| mounted_product_replay_worker_slot(11, worker_count).expect("deterministic logical slot")), [0, 1, 3, 4]);
        assert_eq!(mounted_product_replay_profile(7, PRODUCT_REPLAY_PROFILE_COUNT), None);
        assert_eq!(mounted_product_replay_profile(0, 3), None);
        assert_eq!(mounted_product_replay_worker_slot(u16::MAX, 4), None);
    }

    #[test]
    fn production_product_authority_opportunities_remain_sub_eight_ms() {
        for cursor in 0..PRODUCT_REPLAY_PROFILE_COUNT {
            let started = std::time::Instant::now();
            let worker_count = mounted_product_replay_profile(7, cursor).expect("fixed production profile");
            let _worker_slot = mounted_product_replay_worker_slot(11, worker_count).expect("fixed production slot");
            assert!(started.elapsed() < Duration::from_millis(8), "one product authority control opportunity exceeded 8ms");
        }
        let started = std::time::Instant::now();
        let (authority, expected) = product_authority_fixture();
        let authority = authority.validate(expected).expect("one fixed identity opportunity");
        assert!(started.elapsed() < Duration::from_millis(8), "one product authority validation opportunity exceeded 8ms");
        authority.retire();
        assert_eq!(drain_product_recovery(), 2);
    }

    #[test]
    fn populated_mounted_replay_drop_publishes_exact_generation_and_drains_incrementally() {
        let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved recovery");
        let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, document: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
        let authority = JobTurn { job: 19, operation: JobOperation { operation: 23, base_revision: 29, generation: 31, preview_sequence: 0, seed: 37 }, step_sequence: 0 };
        drop(MountedJobReplay {
            actor: ActorId(41),
            authority,
            request: JobReplayRequest::from_spawn("fixture.replay", b"seed"),
            placement: JobPlacement::Inline,
            log: ManuallyDrop::new(JobReplayLog::new(route, 31).expect("generation-qualified replay")),
            captured: None,
            policy: None,
            terminal_seen: false,
            replay_requested: false,
            replay_started: false,
            replay_submit_sequence: None,
            accepted_replay_sequence: None,
            replay_worker_count: 0,
            replay_worker_slot: u16::MAX,
            recovery: Some(recovery),
        });
        {
            let registry = mounted_replay_recovery_registry().lock().expect("replay recovery lock");
            let owner = registry.slots[recovery.index].owner.as_ref().expect("drop published recovery owner");
            assert_eq!((owner.generation, owner.actor, owner.job), (31, ActorId(41), 19));
        }
        let mut opportunities = 0;
        while mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
            assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
            opportunities += 1;
            assert!(opportunities <= 4);
        }
        assert!(opportunities >= 2, "owner transfer and terminal slot release are distinct grants");
    }

    #[test]
    fn panic_after_mounted_capture_transfers_the_exact_generation_to_incremental_recovery() {
        let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved panic recovery");
        let route = JobReplayRoute { plugin: [9; 32], package: [8; 32], controller: [7; 32], tool: [6; 32], window: 5, document: [4; 32], request_schema: [3; 32], request_version: 1, request_digest: [2; 32] };
        let authority = JobTurn { job: 43, operation: JobOperation { operation: 47, base_revision: 53, generation: 59, preview_sequence: 0, seed: 61 }, step_sequence: 0 };
        let caught = std::panic::catch_unwind(|| {
            let _mounted = MountedJobReplay {
                actor: ActorId(67),
                authority,
                request: JobReplayRequest::from_spawn("fixture.panic", b"owner"),
                placement: JobPlacement::Exclusive,
                log: ManuallyDrop::new(JobReplayLog::new(route, 59).expect("generation-qualified panic replay")),
                captured: None,
                policy: None,
                terminal_seen: false,
                replay_requested: false,
                replay_started: false,
                replay_submit_sequence: None,
                accepted_replay_sequence: None,
                replay_worker_count: 0,
                replay_worker_slot: u16::MAX,
                recovery: Some(recovery),
            };
            panic!("mounted replay panic fixture");
        });
        assert!(caught.is_err());
        {
            let registry = mounted_replay_recovery_registry().lock().expect("replay recovery lock");
            let owner = registry.slots[recovery.index].owner.as_ref().expect("panic published exact owner");
            assert_eq!((owner.generation, owner.actor, owner.job), (59, ActorId(67), 43));
        }
        let mut opportunities = 0;
        while mounted_replay_recovery_registry().lock().expect("replay recovery lock").slots[recovery.index].reserved {
            assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
            opportunities += 1;
            assert!(opportunities <= JOB_PROGRESS_ACTIVE_CAPACITY * 4);
        }
        assert!(opportunities >= 2);
    }

    fn destroy_request(instance: u32) -> KernelRequest {
        KernelRequest::DestroyApp {
            owner: Arc::new(KernelCloseSubmission {
                instance,
                realm: false,
                generation: u64::from(instance) + 1,
                queue: Arc::new(KernelRequestQueue::default()),
                pool: crate::renderer_worker_pool(),
                registry: std::sync::Weak::new(),
                phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_QUEUED),
            }),
        }
    }

    fn close_submission(registry: &Arc<KernelCloseSubmissionRegistry>, instance: u32, generation: u64) -> Arc<KernelCloseSubmission> {
        Arc::new(KernelCloseSubmission {
            instance,
            realm: false,
            generation,
            queue: Arc::new(KernelRequestQueue::default()),
            pool: crate::renderer_worker_pool(),
            registry: Arc::downgrade(registry),
            phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_UNADMITTED),
        })
    }

    fn command_request(instance: u32, generation: u64, page_count: usize) -> KernelRequest {
        let mut pages = semio_framework::kernel::CommandPageSet::try_new(page_count).unwrap();
        for index in 0..page_count {
            let page = if index + 1 == page_count {
                semio_framework::kernel::FixedCommandPage::try_copy_from(b"tail").unwrap()
            } else {
                semio_framework::kernel::FixedCommandPage::try_copy_from(&[3; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).unwrap()
            };
            pages.try_push(page).unwrap();
        }
        let command = semio_framework::kernel::PagedCommand::try_from_pages(pages).unwrap();
        let mut commands = semio_framework::kernel::CommandEnvelopeSet::try_new().unwrap();
        commands.try_push(semio_framework::kernel::CommandEnvelope { instance, seq: generation, command }).unwrap();
        let batch = semio_framework::kernel::CommandBatch::try_new(generation, commands).unwrap();
        KernelRequest::ExchangeCommands { instance, driver: semio_framework::kernel::CommandBatchDriver::new(generation, batch) }
    }

    #[test]
    fn fixed_kernel_request_queue_returns_capacity_plus_one_owner_and_preserves_fifo() {
        let queue = KernelRequestQueue::default();
        for instance in 0..KERNEL_REQUEST_QUEUE_CAPACITY as u32 {
            queue.try_push(destroy_request(instance), Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
        }
        let (rejected, _) = queue.try_push(destroy_request(999), Arc::new(ResponseSlot::default()), None).unwrap_err();
        assert!(matches!(rejected, KernelRequest::DestroyApp { ref owner } if owner.instance == 999));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        for expected in 0..KERNEL_REQUEST_QUEUE_CAPACITY as u32 {
            let Poll::Ready((request, _)) = queue.poll(&mut context) else { panic!("fixed request was ready") };
            assert!(matches!(request, KernelRequest::DestroyApp { ref owner } if owner.instance == expected));
        }
    }

    #[test]
    fn fixed_kernel_request_queue_rejects_aggregate_page_credit_plus_one_exactly() {
        let queue = KernelRequestQueue::default();
        queue.try_push(command_request(1, 7, semio_framework::kernel::COMMAND_MAXIMUM_PAGES), Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
        let (rejected, _) = queue.try_push(command_request(2, 8, 1), Arc::new(ResponseSlot::default()), None).unwrap_err();
        assert!(matches!(rejected, KernelRequest::ExchangeCommands { instance: 2, ref driver } if driver.remaining_pages() == 1));
    }

    #[test]
    fn fixed_kernel_request_queue_contention_returns_the_untouched_owner() {
        let queue = KernelRequestQueue::default();
        let guard = queue.state.lock().unwrap();
        let (rejected, _) = queue.try_push(command_request(7, 11, 1), Arc::new(ResponseSlot::default()), None).unwrap_err();
        assert!(matches!(rejected, KernelRequest::ExchangeCommands { instance: 7, ref driver } if driver.generation() == 11 && driver.remaining_pages() == 1));
        drop(guard);
    }

    #[test]
    fn fixed_kernel_close_registry_returns_the_exact_modulo_collision_and_reuses_only_after_terminal_generation() {
        let registry = Arc::new(KernelCloseSubmissionRegistry::new());
        let first = close_submission(&registry, 3, 11);
        assert!(first.try_admit());
        assert!(registry.contains(3, 11));
        let collision = close_submission(&registry, 3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32, 12);
        assert!(!collision.try_admit());
        assert_eq!(collision.instance, 3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32);
        assert_eq!(collision.generation, 12);
        assert!(registry.contains(3, 11));
        first.finish(KernelCloseStatus::Complete);
        assert!(!registry.contains(3, 11));
        assert!(collision.try_admit());
        assert!(registry.contains(3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32, 12));
    }

    #[test]
    fn fixed_kernel_close_registry_contention_returns_unadmitted_owner_for_exact_retry() {
        let registry = Arc::new(KernelCloseSubmissionRegistry::new());
        let owner = close_submission(&registry, 8, 21);
        let guard = registry.slots.lock().unwrap();
        assert!(!owner.try_admit());
        assert_eq!(owner.phase.load(std::sync::atomic::Ordering::Acquire), KERNEL_CLOSE_UNADMITTED);
        assert_eq!(owner.instance, 8);
        assert_eq!(owner.generation, 21);
        drop(guard);
        assert!(owner.try_admit());
        assert!(registry.contains(8, 21));
    }

    #[test]
    fn fixed_kernel_request_shutdown_faults_the_retained_close_handle_before_terminal_removal() {
        let queue = KernelRequestQueue::default();
        let owner = match destroy_request(19) {
            KernelRequest::DestroyApp { owner } => owner,
            _ => unreachable!(),
        };
        queue.try_push(KernelRequest::DestroyApp { owner: owner.clone() }, Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
        assert!(queue.begin_shutdown());
        assert_eq!(queue.shutdown_step(0), (true, 1, 0));
        assert_eq!(owner.terminal_status(), Some(KernelCloseStatus::Fault));
    }

    #[test]
    fn fixed_kernel_request_queue_shutdown_releases_one_real_page_per_grant() {
        let queue = KernelRequestQueue::default();
        queue.try_push(command_request(3, 12, 2), Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
        assert!(queue.begin_shutdown());
        assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1), (false, 0, 0));
        assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (false, 1, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES));
        assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (true, 1, 4));
    }

    #[test]
    fn fixed_kernel_request_queue_shutdown_releases_create_fields_one_owner_per_grant() {
        let queue = KernelRequestQueue::default();
        queue
            .try_push(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(PathBuf::from("path"), "plugin".to_string(), "app".to_string()) }, Arc::new(ResponseSlot::default()), None)
            .unwrap_or_else(|_| panic!("fixture request queue admission"));
        assert!(queue.begin_shutdown());
        assert_eq!(queue.shutdown_step(3), (false, 0, 0));
        assert_eq!(queue.shutdown_step(4), (false, 1, 4));
        assert_eq!(queue.shutdown_step(6), (false, 1, 6));
        assert_eq!(queue.shutdown_step(3), (true, 1, 3));
    }

    #[test]
    fn fixed_kernel_request_queue_shutdown_releases_surface_and_rejected_events_in_fifo_units() {
        let queue = KernelRequestQueue::default();
        queue.try_push(KernelRequest::Exchange { instance: 1, event: QueuedKernelEvent { surface_visible: Some("surface".to_string()), surface_body_key: None, surface_view_state: None } }, Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fixture request queue admission"));
        queue
            .try_push(KernelRequest::CloseRejectedEvents { owner: RejectedKernelEvents { events: std::collections::VecDeque::from([Event::Wake, Event::Wake]) } }, Arc::new(ResponseSlot::default()), None)
            .unwrap_or_else(|_| panic!("fixture request queue admission"));
        assert!(queue.begin_shutdown());
        assert_eq!(queue.shutdown_step(6), (false, 0, 0));
        assert_eq!(queue.shutdown_step(7), (false, 1, 7));
        assert_eq!(queue.shutdown_step(0), (false, 1, 0));
        assert_eq!(queue.shutdown_step(0), (true, 1, 0));
    }
}

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn kernel_runtime_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "renderer::kernel_runtime")
        .map(|table| semio_framework_async::FixedSlotTableBudget::new(table["owner"].as_str().expect("owner"), table["capacity"].as_u64().expect("capacity") as usize, table["elementSizeBytes"].as_u64().expect("element bytes") as usize, table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize))
        .collect();
    let measured = vec![
        semio_framework_async::FixedSlotTableBudget::new("kernel_runtime::RetainedSurfaceRegistry", RETAINED_SURFACE_CAPACITY, size_of::<Option<RetainedSurfaceSlot>>(), size_of::<RetainedSurfaceRegistry>()),
        semio_framework_async::FixedSlotTableBudget::new("kernel_runtime::CommandDocumentRetirementRegistry", COMMAND_DOCUMENT_RETIREMENT_CAPACITY, size_of::<Option<CommandDocumentRetirementState>>(), size_of::<CommandDocumentRetirementRegistry>()),
        semio_framework_async::FixedSlotTableBudget::new("kernel_runtime::MountedTypedOperationResultExchange", MOUNTED_TYPED_OPERATION_RESULT_PAGES, size_of::<Option<MountedTypedOperationResultPage>>(), size_of::<MountedTypedOperationResultExchange>()),
    ];
    semio_framework_async::assert_fixed_slot_tables(
        "renderer::kernel_runtime",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(RetainedSurfaceRegistry::new());
            drop(CommandDocumentRetirementRegistry::new());
            drop(MountedTypedOperationResultExchange::new());
        },
    );
}
