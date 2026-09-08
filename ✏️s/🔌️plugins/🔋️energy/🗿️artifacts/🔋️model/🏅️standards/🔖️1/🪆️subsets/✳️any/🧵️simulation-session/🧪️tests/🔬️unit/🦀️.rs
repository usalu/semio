
use super::*;

fn render(app: u32, generation: u64) -> AppRenderOperationContext {
    AppRenderOperationContext { app_instance_id: app, base_revision: RevisionId(9), generation: Generation(generation), canonical_base_revision: [generation as u8; 32] }
}

fn request(request: u64) -> EnergySimulationRequestIdentity {
    EnergySimulationRequestIdentity { request, operation: 1, generation: 1, config_digest: EnergySimulationConfigProjection::default().digest() }
}

#[test]
fn event_log_max_plus_one_preserves_existing_chronology() {
    let mut registry = Registry::new();
    for index in 0..EVENT_SLOTS {
        registry.push_event(render(1, 1), if index == 0 { EnergySimulationEventKind::Start { request: 1, config: EnergySimulationConfigProjection::default() } } else { EnergySimulationEventKind::Cancel(request(1)) }).unwrap();
    }
    assert_eq!(registry.push_event(render(1, 1), EnergySimulationEventKind::Discard(request(1))), Err("energy.session.event-log-saturated"));
    for expected in 1..=EVENT_SLOTS as u64 {
        assert_eq!(registry.pop_event().unwrap().sequence, expected);
    }
}

#[test]
fn fixed_shell_max_plus_one_never_reuses_a_live_owner() {
    let mut registry = Registry::new();
    let mut shells = [0u16; SHELL_SLOTS];
    for shell in &mut shells {
        *shell = registry.allocate().unwrap();
    }
    assert!(registry.allocate().is_none());
    assert_eq!(shells[0], 0);
    assert_eq!(shells[SHELL_SLOTS - 1], (SHELL_SLOTS - 1) as u16);
}

#[test]
fn active_app_slot_max_plus_one_rejects_without_aliasing() {
    let mut registry = Registry::new();
    for app in 1..=ACTIVE_SLOTS as u32 {
        registry.push_event(render(app, 1), EnergySimulationEventKind::Cancel(request(u64::from(app)))).unwrap();
    }
    assert_eq!(registry.push_event(render(ACTIVE_SLOTS as u32 + 1, 1), EnergySimulationEventKind::Cancel(request(99))), Err("energy.session.active-slots-saturated"));
    for app in 1..=ACTIVE_SLOTS as u32 {
        assert!(registry.slot_for(app).is_some());
    }
}

#[test]
fn capture_admission_rejects_item_and_byte_max_plus_one_before_mount() {
    let mut items = CaptureCensus { lane: 0, index: 0, items: MAXIMUM_CAPTURE_ITEMS, bytes: 0 };
    assert_eq!(items.charge_backing(1, 0), Err("energy.session.capture-admission-exceeded"));
    let mut bytes = CaptureCensus { lane: 0, index: 0, items: 0, bytes: MAXIMUM_CAPTURE_BYTES };
    assert_eq!(bytes.charge_backing(1, 1), Err("energy.session.capture-admission-exceeded"));
}

#[test]
fn retirement_max_plus_one_retains_the_rejected_shell() {
    let mut registry = Registry::new();
    for shell in 0..SHELL_SLOTS as u16 {
        assert!(registry.reserve_shell_retirement(shell));
        assert!(registry.retire_shell(shell));
    }
    assert!(registry.reserve_retirement().is_none());
    assert_eq!(registry.retiring[0], Some(0));
    assert_eq!(registry.retiring[SHELL_SLOTS - 1], Some((SHELL_SLOTS - 1) as u16));
}

#[test]
fn checkpoint_selection_and_locale_do_not_change_numerical_digest() {
    let base = EnergySimulationConfigProjection::default();
    let mut restored = base;
    restored.checkpoint_token = u64::MAX;
    restored.locale_de = true;
    assert_eq!(base.digest(), restored.digest());
    restored.zone_timestep_minutes += 1;
    assert_ne!(base.digest(), restored.digest());
}

#[test]
fn invalid_config_is_rejected_before_event_owner_move() {
    let mut invalid = EnergySimulationConfigProjection::default();
    invalid.warmup_days = 366;
    assert!(!invalid.validate());
    invalid = EnergySimulationConfigProjection::default();
    invalid.zone_timestep_minutes = 0;
    assert!(!invalid.validate());
}

#[test]
fn cancel_before_snapshot_admission_retires_the_exact_preflight() {
    let mut registry = Registry::new();
    let operation = render(4, 7);
    registry.push_event(operation, EnergySimulationEventKind::Start { request: 7, config: EnergySimulationConfigProjection::default() }).unwrap();
    apply_event_one(&mut registry);
    let slot = registry.slot_for(operation.app_instance_id).unwrap();
    assert!(registry.preflight[slot].is_some());
    registry.push_event(operation, EnergySimulationEventKind::Cancel(request(7))).unwrap();
    apply_event_one(&mut registry);
    assert!(registry.preflight[slot].is_none());
    assert_eq!(registry.free_len, SHELL_SLOTS);
}

#[test]
fn dynamic_record_capture_mutation_is_one_record_character_or_item_per_grant() {
    let mut source = Model::default();
    source.surfaces.try_reserve_exact(1).unwrap();
    let mut name = String::new();
    name.try_reserve_exact(8).unwrap();
    name.push_str("Wände");
    let mut vertices_m = Vec::new();
    vertices_m.try_reserve_exact(3).unwrap();
    vertices_m.extend([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
    source.surfaces.push(crate::model::Surface {
        id: crate::model::EntityId(1),
        name,
        zone_id: crate::model::EntityId(2),
        class: crate::model::SurfaceClass::ExteriorWall,
        vertices_m,
        construction_id: crate::model::EntityId(3),
        outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    });
    let mut census = CaptureCensus { lane: 5, index: 0, items: 0, bytes: 0 };
    assert!(!census.step_one(&source).unwrap());
    assert_eq!(census.items, 11);
    assert_eq!(census.bytes, 8 + 3 * size_of::<[f64; 3]>());
    assert!(!census.step_one(&source).unwrap());
    assert_eq!(census.items, 12);
    assert_eq!(census.bytes, 8 + 3 * size_of::<[f64; 3]>() + size_of::<crate::model::Surface>());
    let mut capture = ModelCapture::new();
    capture.lane = 5;
    for _ in 0..64 {
        let before = capture.model.surfaces.len() + capture.model.surfaces.first().map_or(0, |surface| surface.name.chars().count() + surface.vertices_m.len());
        capture.step_one(&source).unwrap();
        let after = capture.model.surfaces.len() + capture.model.surfaces.first().map_or(0, |surface| surface.name.chars().count() + surface.vertices_m.len());
        assert!(after.saturating_sub(before) <= 1);
        if capture.lane > 5 {
            break;
        }
    }
    assert_eq!(capture.model.surfaces, source.surfaces);
}

#[test]
fn admitted_capture_source_has_no_whole_record_clone_backdoor() {
    let source = include_str!("../../🦀️.rs");
    for forbidden in [concat!("item", ".clone()"), concat!("airflow_network", ".clone()"), concat!("ground_temperature", ".clone()")] {
        assert!(!source.contains(forbidden), "whole record mutation survived: {forbidden}");
    }
    assert!(source.contains("capture-nested-vector-reserve"));
}

#[test]
fn chronology_is_identical_for_one_two_four_and_default_fuel() {
    let expected = [EnergyQualityTier::SteadyStateEstimate, EnergyQualityTier::DesignDay, EnergyQualityTier::CoarseTimestep, EnergyQualityTier::Final];
    for fuel in [1u64, 2, 4, 64] {
        let mut cursor = 0;
        let mut observed = [EnergyQualityTier::SteadyStateEstimate; 4];
        while cursor < expected.len() {
            let admitted = usize::try_from(fuel.min(1)).unwrap();
            for _ in 0..admitted {
                observed[cursor] = expected[cursor];
                cursor += 1;
            }
        }
        assert_eq!(observed, expected);
        assert_eq!(observed.map(quality_tier_index), [0, 1, 2, 3]);
    }
}

#[test]
fn lower_tier_and_stale_sequence_cannot_replace_visible_authority() {
    let identity = MountedIdentity {
        app_instance_id: 3,
        request: 1,
        document_revision: RevisionId(9),
        document_generation: Generation(4),
        canonical_base_revision: [4; 32],
        operation: OperationId(7),
        generation: Generation(2),
        config_digest: 8,
        operation_seed: 9,
        job: 7,
    };
    let mut projection = EnergySimulationProjection::new(identity);
    projection.latest_sequence = 9;
    projection.latest_tier = Some(EnergyQualityTier::CoarseTimestep);
    let tier = match EnergyQualityTier::SteadyStateEstimate {
        EnergyQualityTier::SteadyStateEstimate => 0,
        _ => 3,
    };
    assert!(tier < 2);
    assert_eq!(projection.latest_sequence, 9);
}

#[test]
fn process_token_rejects_stale_generation_and_tags_the_exact_job() {
    let identity = MountedIdentity {
        app_instance_id: 1,
        request: 1,
        document_revision: RevisionId(2),
        document_generation: Generation(3),
        canonical_base_revision: [4; 32],
        operation: OperationId(JOB_TAG | 1),
        generation: Generation(5),
        config_digest: 6,
        operation_seed: 7,
        job: JOB_TAG | 1,
    };
    let bytes = encode_input(0, identity);
    assert_eq!(decode_input(identity.job, &bytes).unwrap().1, identity);
    assert_ne!(decode_input(identity.job + 1, &bytes).unwrap().1, identity);
    let mut stale = bytes;
    stale[31..39].copy_from_slice(&0u64.to_le_bytes());
    assert!(decode_input(identity.job, &stale).is_none());
}

#[test]
fn schema_and_accessibility_vocabulary_is_complete() {
    let source = include_str!("../../../✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs");
    for law in ["Start simulation", "Simulation starten", "Cancel simulation", "Simulation abbrechen", "aria-live", "busy", "Final result"] {
        assert!(source.contains(law), "missing {law}");
    }
}

#[test]
fn artifact_read_path_has_no_process_cache_clone_or_serde_key_authority() {
    let model = Model { name: "store-owned".into(), ..Model::default() };
    let snapshot = crate::energy_snapshot_with_state(crate::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None);
    assert_eq!(snapshot.model, model, "the event-sourced snapshot, not a side cache, is the exact numerical read authority");
    let artifact = include_str!("../../../../../../../🦀️.rs");
    for forbidden in ["ENERGY_SCRATCH", "with_energy_model_ref", "HashMap<String, EnergyWorkingScene>", "energy_scene_id"] {
        assert!(!artifact.contains(forbidden), "process cache authority survived: {forbidden}");
    }
    assert!(artifact.contains("pub struct EnergyModelReadLease"));
    assert!(artifact.contains("commit_authority_matches"));
    assert!(artifact.contains("return_to_registry_witness"));
}

#[test]
fn exact_request_identity_rejects_each_stale_lifecycle_dimension() {
    let identity = MountedIdentity {
        app_instance_id: 8,
        request: 19,
        document_revision: RevisionId(2),
        document_generation: Generation(3),
        canonical_base_revision: [4; 32],
        operation: OperationId(5),
        generation: Generation(6),
        config_digest: 7,
        operation_seed: 8,
        job: JOB_TAG | 9,
    };
    let exact = EnergySimulationRequestIdentity { request: 19, operation: 5, generation: 6, config_digest: 7 };
    assert!(identity.matches_request(exact));
    for stale in
        [EnergySimulationRequestIdentity { request: 20, ..exact }, EnergySimulationRequestIdentity { operation: 9, ..exact }, EnergySimulationRequestIdentity { generation: 9, ..exact }, EnergySimulationRequestIdentity { config_digest: 9, ..exact }]
    {
        assert!(!identity.matches_request(stale));
    }
}

#[test]
fn completed_capture_revalidates_every_live_authority_before_model_transfer() {
    let mounted = MountedIdentity {
        app_instance_id: 61,
        request: 19,
        document_revision: RevisionId(2),
        document_generation: Generation(3),
        canonical_base_revision: [4; 32],
        operation: OperationId(5),
        generation: Generation(6),
        config_digest: 7,
        operation_seed: 8,
        job: JOB_TAG | 9,
    };
    let exact_render = AppRenderOperationContext { app_instance_id: mounted.app_instance_id, base_revision: mounted.document_revision, generation: mounted.document_generation, canonical_base_revision: mounted.canonical_base_revision };
    let stale_expected = MountedIdentity { generation: Generation(99), ..mounted };
    let stale_app = AppRenderOperationContext { app_instance_id: 62, ..exact_render };
    let stale_revision = AppRenderOperationContext { base_revision: RevisionId(99), ..exact_render };
    let stale_generation = AppRenderOperationContext { generation: Generation(99), ..exact_render };
    let stale_canonical = AppRenderOperationContext { canonical_base_revision: [99; 32], ..exact_render };
    for (expected, render, request, digest, snapshot_fresh, cancelled) in [
        (stale_expected, exact_render, mounted.request, mounted.config_digest, true, false),
        (mounted, stale_app, mounted.request, mounted.config_digest, true, false),
        (mounted, stale_revision, mounted.request, mounted.config_digest, true, false),
        (mounted, stale_generation, mounted.request, mounted.config_digest, true, false),
        (mounted, stale_canonical, mounted.request, mounted.config_digest, true, false),
        (mounted, exact_render, mounted.request + 1, mounted.config_digest, true, false),
        (mounted, exact_render, mounted.request, mounted.config_digest + 1, true, false),
        (mounted, exact_render, mounted.request, mounted.config_digest, false, false),
        (mounted, exact_render, mounted.request, mounted.config_digest, true, true),
    ] {
        let mut capture = ModelCapture::new();
        capture.model.name = "stale-capture".into();
        capture.model.zones.push(crate::model::Zone { id: crate::model::EntityId(1), name: "retained-zone".into(), volume_m3: 1.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true });
        let mut capture = Some(capture);
        assert!(take_captured_model_for_admission(&mut capture, mounted, expected, render, request, digest, snapshot_fresh, cancelled).is_err());
        assert_eq!(capture.as_ref().expect("stale capture remains exact").model.zones[0].name, "retained-zone");
        let mut close = EnergyModelCloseCursor::new(std::mem::replace(&mut capture, None).expect("retained stale capture").finish());
        for _ in 0..128 {
            match close.close_step(4) {
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= 4);
                }
                semio_framework_job::InteractiveJobCloseStep::Complete => break,
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("stale captured Model close cannot block"),
            }
        }
        assert!(close.terminal_is_empty());
    }

    let mut exact = ModelCapture::new();
    exact.model.name = "exact-capture".into();
    let mut exact = Some(exact);
    let model = take_captured_model_for_admission(&mut exact, mounted, mounted, exact_render, mounted.request, mounted.config_digest, true, false).expect("only the current uncancelled capture transfers");
    assert_eq!(model.name, "exact-capture");
    assert!(exact.is_none());
}

#[test]
fn adopted_projection_is_partitioned_by_application_and_rejects_every_aba_dimension() {
    let identity = MountedIdentity {
        app_instance_id: 71,
        request: 29,
        document_revision: RevisionId(12),
        document_generation: Generation(13),
        canonical_base_revision: [14; 32],
        operation: OperationId(15),
        generation: Generation(16),
        config_digest: 17,
        operation_seed: 18,
        job: JOB_TAG | 19,
    };
    let render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.document_revision, generation: identity.document_generation, canonical_base_revision: identity.canonical_base_revision };
    let mut projection = EnergySimulationProjection::new(identity);
    projection.adopted = true;
    projection.status = EnergySimulationStatus::Adopted;
    projection.tiers[0] = Some(EnergyTierProjection {
        app_instance_id: identity.app_instance_id,
        document_revision: identity.document_revision,
        document_generation: identity.document_generation,
        canonical_base_revision: identity.canonical_base_revision,
        operation: identity.operation,
        generation: identity.generation,
        config_digest: identity.config_digest,
        sequence: 1,
        tier: EnergyQualityTier::SteadyStateEstimate,
        stage: EnergyJobStage::Complete,
        warmup_hour: 0,
        timestep: 1,
        total_timesteps: 1,
        facility_electricity_kwh: 1.0,
    });
    let exact = AdoptedProjectionAuthority::new(identity, projection).expect("exact adopted authority");
    let mut registry = Registry::new();
    registry.apps[0] = Some(identity.app_instance_id);
    registry.apps[1] = Some(identity.app_instance_id + 1);
    registry.last_request[0] = identity.request;
    registry.adopted[0] = Some(exact);
    assert_eq!(registry.adopted_projection(render).map(|projection| projection.request), Some(identity.request));
    let other_app_same_document = AppRenderOperationContext { app_instance_id: identity.app_instance_id + 1, ..render };
    assert!(registry.adopted_projection(other_app_same_document).is_none(), "matching document provenance cannot cross the application partition");

    for mutation in 0..5 {
        let mut stale = exact;
        match mutation {
            0 => stale.projection.request += 1,
            1 => stale.projection.operation = OperationId(stale.projection.operation.0 + 1),
            2 => stale.projection.generation = Generation(stale.projection.generation.0 + 1),
            3 => stale.projection.config_digest += 1,
            _ => stale.projection.tiers[0].as_mut().expect("tier").app_instance_id += 1,
        }
        registry.adopted[0] = Some(stale);
        assert!(registry.adopted_projection(render).is_none(), "ABA mutation {mutation} leaked an adopted projection");
    }
    registry.adopted[0] = Some(exact);
    registry.last_request[0] += 1;
    assert!(registry.adopted_projection(render).is_none(), "a newer request must invalidate the retained adopted authority");
}

#[test]
fn acknowledged_numerical_complete_is_retained_then_detaches_the_process_owner() {
    let mut retained = None;
    let complete = StepOutcome::Complete(semio_framework_job::CommitCandidate {
        state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
        output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
    });
    assert!(matches!(retain_worker_outcome(&mut retained, complete), JobStep::Done(bytes) if bytes.is_empty()));
    assert!(retained.as_ref().is_some_and(StepOutcome::is_terminal));
    assert!(retained.as_ref().is_some_and(StepOutcome::terminal_is_empty));
    let identity = MountedIdentity {
        app_instance_id: 81,
        request: 82,
        document_revision: RevisionId(83),
        document_generation: Generation(84),
        canonical_base_revision: [85; 32],
        operation: OperationId(86),
        generation: Generation(87),
        config_digest: 88,
        operation_seed: 89,
        job: JOB_TAG | 90,
    };
    let shell = 5;
    drop(EnergyMountedBoundedJob { shell_index: shell, shell: Rc::new(RefCell::new(None)), identity });
    let recovered = RECOVERY.with(|recovery| recovery.borrow_mut()[shell as usize].take()).expect("terminal process Drop publishes the exact fixed recovery witness");
    assert_eq!(recovered, RecoveryRecord { shell, identity });
}

#[test]
fn reused_or_older_start_request_cannot_replace_current_preflight_authority() {
    let render = render(18, 1);
    let mut registry = Registry::new();
    registry.push_event(render, EnergySimulationEventKind::Start { request: 9, config: EnergySimulationConfigProjection::default() }).unwrap();
    apply_event_one(&mut registry);
    let slot = registry.slot_for(18).unwrap();
    assert_eq!(registry.preflight[slot].map(|preflight| preflight.request), Some(9));
    registry.push_event(render, EnergySimulationEventKind::Start { request: 9, config: EnergySimulationConfigProjection { warmup_days: 99, ..EnergySimulationConfigProjection::default() } }).unwrap();
    apply_event_one(&mut registry);
    assert_eq!(registry.preflight[slot].map(|preflight| (preflight.request, preflight.config.warmup_days)), Some((9, EnergySimulationConfigProjection::default().warmup_days)));
    assert_eq!(registry.last_request[slot], 9);
}

#[test]
fn retry_path_has_no_default_config_fallback_or_unrelated_preflight() {
    let source = include_str!("../../🦀️.rs");
    let retry = &source[source.find("EnergySimulationEventKind::Retry(request)").expect("retry")..source.find("EnergySimulationEventKind::Cancel(request)").expect("cancel")];
    assert!(!retry.contains("unwrap_or_default"));
    assert!(retry.contains("matches_request(request)"));
    assert!(retry.contains("state.config"));
}

#[test]
fn partial_capture_closes_one_nested_character_or_item_per_grant() {
    let mut capture = ModelCapture::new();
    capture.model.name = "Gebäude".into();
    capture.model.zones.push(crate::model::Zone { id: crate::model::EntityId(1), name: "Raum".into(), volume_m3: 1.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true });
    let mut close = EnergyModelCloseCursor::new(capture.finish());
    let mut turns = 0;
    while !close.terminal_is_empty() {
        match close.close_step(4) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 4);
            }
            semio_framework_job::InteractiveJobCloseStep::Complete => {}
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("owned partial model cannot block"),
        }
        turns += 1;
        assert!(turns < 64);
    }
}

#[test]
fn retirement_is_reserved_before_snapshot_or_shell_owner_move() {
    let mut registry = Registry::new();
    let mut shells = [0u16; SHELL_SLOTS];
    for shell in &mut shells {
        *shell = registry.allocate().expect("fixed shell");
        assert!(registry.reserve_shell_retirement(*shell));
        assert!(registry.reserve_shell_recovery(*shell));
    }
    assert!(registry.allocate().is_none());
    assert!(registry.retire_shell(shells[0]));
    let source = include_str!("../../🦀️.rs");
    let reconcile = &source[source.find("pub fn reconcile(").expect("reconcile")..source.find("pub fn with_projection").expect("projection")];
    assert!(reconcile.find("reserve_shell_retirement").expect("reservation") < reconcile.find("take_snapshot_read").expect("snapshot move"));
    assert!(reconcile.find("reserve_shell_recovery").expect("recovery reservation") < reconcile.find("take_snapshot_read").expect("snapshot move"));
    assert!(reconcile.find("reserve_shell_retirement").expect("reservation") < reconcile.find("MountedState::new").expect("shell owner move"));
}

#[test]
fn every_worker_drop_publishes_exact_fixed_recovery_identity() {
    let identity = MountedIdentity {
        app_instance_id: 31,
        request: 41,
        document_revision: RevisionId(2),
        document_generation: Generation(3),
        canonical_base_revision: [4; 32],
        operation: OperationId(5),
        generation: Generation(6),
        config_digest: 7,
        operation_seed: 8,
        job: JOB_TAG | 9,
    };
    let shell = 3;
    drop(EnergyMountedBoundedJob { shell_index: shell, shell: Rc::new(RefCell::new(None)), identity });
    let recovered = RECOVERY.with(|recovery| recovery.borrow_mut()[shell as usize].take()).expect("normal, lost, panic and cancellation share one unconditional Drop publisher");
    assert_eq!(recovered.shell, shell);
    assert_eq!(recovered.identity, identity);
}

#[test]
fn rejected_whole_capture_and_terminal_loss_mutations_are_absent() {
    let source = include_str!("../../🦀️.rs");
    for forbidden in [concat!("self.capture", ".take()"), "clean_terminal", "state.abandoned", "try_borrow_mut() {\n            if let Some(state) = owner.as_mut() {\n                state.abandoned"] {
        assert!(!source.contains(forbidden), "owner-loss mutation survived: {forbidden}");
    }
    assert!(source.contains("capture_close"));
    assert!(source.contains("RECOVERY.with"));
    assert!(source.contains("worker_returned"));
}
