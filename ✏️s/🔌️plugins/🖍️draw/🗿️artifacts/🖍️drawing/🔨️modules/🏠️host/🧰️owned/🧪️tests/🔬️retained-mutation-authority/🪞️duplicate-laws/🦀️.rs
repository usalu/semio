//! 🪞️ Drawing retained mutation laws over duplication and commit: clone work planning, admission fixture, duplicate hash and name, stale replay cancellation and the committed candidate.

use super::*;
use semio_framework_test_fixtures::fixture_text;

#[test]
fn retained_drawing_duplicate_plans_exact_clone_work_before_overlay_and_source_handoff() {
    let mut source = nested_snapshot();
    let source_owner = native_layer_backing(&source.layers);
    let target = match source.layers.last().expect("Drawing clone-plan group") {
        DrawingLayerNode::Group(group) => group.base.id.clone(),
        _ => unreachable!("Drawing clone-plan group remains exact"),
    };
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target });
    let (workset, actual_clone) = planned_clone_workset(&mut source, &mutation).expect("duplicate clone construction and census finish before binding the overlay");
    assert_eq!(native_layer_backing(&source.layers), source_owner, "clone planning retains the exact source owner");
    assert_eq!((workset.clone_items, workset.clone_bytes), (actual_clone.items, actual_clone.bytes), "workset clone credit equals the retained clone traversal's actual capacities");
    assert!(workset.clone_items > 0 && workset.clone_bytes > size_of::<DrawingLayerCloneAuthority>(), "a nested duplicate carries real subtree backing in addition to its clone cursor");
    assert_eq!(workset.workset_items().expect("duplicate workset items"), workset.arena_items + workset.authority_items + workset.clone_items);
    assert_eq!(workset.workset_bytes().expect("duplicate workset bytes"), workset.arena_bytes + workset.authority_bytes + workset.clone_bytes);

    let initial_layers = source.layers.len();
    let original_id = semio_s_artifact_draw_drawing_model::layer_id(source.layers.last().expect("original duplicate source")).to_string();
    let source = apply(source, &mutation).expect("a planned nested duplicate applies");
    assert_eq!(source.layers.len(), initial_layers + 1);
    assert_ne!(semio_s_artifact_draw_drawing_model::layer_id(source.layers.last().expect("published duplicate")), original_id.as_str());
    drain_mutation(mutation);
    drain_snapshot(source);
}

#[test]
fn drawing_mutation_admission_fixture_matches_native_dispositions_and_serde_json_carriers() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧰️owned/🧪️tests/🧫️fixtures/🧮️mutation-admission/🔣️.json")).expect("neutral Drawing mutation-admission fixture parses through serde_json 1");
    assert_eq!(fixture["schema"], "drawing.mutation-admission/v1");
    assert_eq!(fixture["limits"]["documentBytes"].as_u64(), Some(DRAWING_MAXIMUM_NESTED_BYTES as u64));
    assert_eq!(fixture["limits"]["fieldBytes"].as_u64(), Some(DRAWING_OWNED_FIELD_BYTES as u64));
    let cases = fixture["cases"].as_array().expect("neutral admission cases");
    let case = |id: &str| cases.iter().find(|value| value["id"] == id).expect("declared neutral admission case");
    assert_eq!(case("exact-field-page")["expected"]["workClasses"], serde_json::json!(["fixed-arena", "overlay-page"]));
    assert_eq!(case("field-page-plus-one")["expected"]["fault"], "drawing-store.mutation-field-capacity");
    assert_eq!(case("source-document-plus-one")["expected"]["fault"], "drawing-store.preflight-byte-capacity");
    assert_eq!(case("duplicate-subtree")["expected"]["workClasses"], serde_json::json!(["fixed-arena", "clone-subtree"]));

    let exact_source = nested_snapshot();
    let exact_target = match exact_source.layers.last().expect("neutral exact-field group") {
        DrawingLayerNode::Group(group) => semio_s_artifact_draw_drawing_model::layer_id(&group.children[0]).clone(),
        _ => unreachable!("neutral exact-field group remains exact"),
    };
    let exact_mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: exact_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES).into() });
    let exact_source = apply(exact_source, &exact_mutation).expect("neutral exact field-page disposition is applied");
    let exact_carrier: serde_json::Value = serde_json::from_str(&serde_json::to_string(&exact_source).expect("serde_json carrier writes exact-field result")).expect("serde_json carrier reads exact-field result");
    assert!(exact_carrier.to_string().contains(&format!("\"{}\"", "x".repeat(DRAWING_OWNED_FIELD_BYTES))), "third-party carrier exposes the changed layer name");
    drain_mutation(exact_mutation);
    drain_snapshot(exact_source);

    let plus_source = nested_snapshot();
    let plus_before = serde_json::to_value(&plus_source).expect("serde_json carrier records +1 source");
    let plus_target = match plus_source.layers.last().expect("neutral +1 group") {
        DrawingLayerNode::Group(group) => semio_s_artifact_draw_drawing_model::layer_id(&group.children[0]).clone(),
        _ => unreachable!("neutral +1 group remains exact"),
    };
    let plus_mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: plus_target, new_name: "x".repeat(DRAWING_OWNED_FIELD_BYTES + 1).into() });
    let (plus_source, plus_fault) = apply(plus_source, &plus_mutation).expect_err("neutral field-page +1 disposition is rejected");
    assert_eq!(plus_fault, case("field-page-plus-one")["expected"]["fault"].as_str().expect("neutral +1 fault"));
    assert_eq!(serde_json::to_value(&plus_source).expect("serde_json carrier records returned +1 source"), plus_before);
    drain_mutation(plus_mutation);
    drain_snapshot(plus_source);

    let bounded_source = document_over_byte_bound_snapshot();
    let bounded_target = semio_s_artifact_draw_drawing_model::layer_id(&bounded_source.layers[0]).clone();
    let bounded_before = serde_json::to_value(&bounded_source).expect("serde_json carrier records D + 1 source");
    let bounded_mutation = DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: bounded_target, visible: false });
    let (bounded_source, bounded_fault) = apply(bounded_source, &bounded_mutation).expect_err("neutral D + 1 disposition is rejected");
    assert_eq!(bounded_fault, case("source-document-plus-one")["expected"]["fault"].as_str().expect("neutral source fault"));
    assert_eq!(serde_json::to_value(&bounded_source).expect("serde_json carrier records returned D + 1 source"), bounded_before);
    drain_mutation(bounded_mutation);
    drain_snapshot(bounded_source);

    let duplicate_source = nested_snapshot();
    let initial_layers = duplicate_source.layers.len();
    let duplicate_target = semio_s_artifact_draw_drawing_model::layer_id(duplicate_source.layers.last().expect("neutral duplicate source")).clone();
    let duplicate_mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&duplicate_source,&duplicate_target), layer_id: duplicate_target.clone() });
    let duplicate_source = apply(duplicate_source, &duplicate_mutation).expect("neutral duplicate disposition is applied");
    let duplicate_carrier: serde_json::Value = serde_json::from_str(&serde_json::to_string(&duplicate_source).expect("serde_json carrier writes duplicate result")).expect("serde_json carrier reads duplicate result");
    assert_eq!(duplicate_carrier["layers"].as_array().expect("carrier layers").len(), initial_layers + 1);
    assert_ne!(semio_s_artifact_draw_drawing_model::layer_id(duplicate_source.layers.last().expect("neutral published duplicate")), &duplicate_target);
    drain_mutation(duplicate_mutation);
    drain_snapshot(duplicate_source);
}

#[test]
fn retained_drawing_duplicate_hash_frames_domain_id_and_name_lengths_without_concatenation_collision() {
    fn duplicate_id(id: &str, name: &str) -> String {
        let mut source = semio_s_artifact_draw_drawing_model::default_drawing_document("duplicate-framing", None);
        let mut layer = semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit(((name)).to_string().into()).expect("nonempty authored identity"), name, Default::default());
        let base = semio_s_artifact_draw_drawing_model::layer_base_mut(&mut layer);
        base.id.clear();
        base.id.push_str(id);
        admit_layer_string_destinations(&mut layer);
        source.layers.clear();
        reserve_native_layer_slots(&mut source.layers, 2);
        source.layers.push(layer);
        let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&semio_framework_value::paged::PagedUtf8::<{usize::MAX}>::from(id)), layer_id: id.into() });
        let source = apply(source, &mutation).expect("framed duplicate mutation applies");
        let duplicate = source.layers.get(1).map(semio_s_artifact_draw_drawing_model::layer_id).expect("duplicated layer remains retained").to_string();
        drain_mutation(mutation);
        drain_snapshot(source);
        duplicate
    }

    assert_ne!(duplicate_id("ab", "c"), duplicate_id("a", "bc"), "separate id/name length frames prevent concatenation collisions");
}

#[test]
fn retained_drawing_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner() {
    let mut source = semio_s_artifact_draw_drawing_model::default_drawing_document("duplicate-name-owner", None);
    let mut layer = semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Layer")).to_string().into()).expect("nonempty authored identity"), "Layer", Default::default());
    admit_layer_string_destinations(&mut layer);
    let target = semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
    let original_name_owner = native_text_backing(&semio_s_artifact_draw_drawing_model::layer_base(&layer).name);
    source.layers.clear();
    reserve_native_layer_slots(&mut source.layers, 2);
    source.layers.push(layer);
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target });
    let source = apply(source, &mutation).expect("duplicate name suffix uses only pre-admitted destination and fixed scratch page");
    assert_eq!(native_text_backing(&semio_s_artifact_draw_drawing_model::layer_base(&source.layers[0]).name), original_name_owner, "last-valid name backing remains exact");
    assert_eq!(semio_s_artifact_draw_drawing_model::layer_base(&source.layers[1]).name, "Layer copy");
    drain_mutation(mutation);
    drain_snapshot(source);

    let mut rejected = semio_s_artifact_draw_drawing_model::default_drawing_document("duplicate-name-rejected", None);
    let layer = semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Layer")).to_string().into()).expect("nonempty authored identity"), "Layer", Default::default());
    let target = semio_s_artifact_draw_drawing_model::layer_id(&layer).clone();
    rejected.layers = vec![layer].into();
    let exact_owner = native_layer_backing(&rejected.layers);
    let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&rejected,&target), layer_id: target });
    let (rejected, error) = apply(rejected, &mutation).expect_err("unadmitted duplicate destination rejects without allocating after operation admission");
    assert_eq!(error, "drawing-store.duplicate-destination-capacity");
    assert_eq!(native_layer_backing(&rejected.layers), exact_owner, "duplicate rejection returns the exact source native container owner");
    drain_mutation(mutation);
    drain_snapshot(rejected);
}

#[test]
fn retained_drawing_cancel_stale_each_precommit_replay_candidate_container_stage_preserves_last_valid() {
    let stages = [
        DrawingMutationCandidatePhase::PreflightSource,
        DrawingMutationCandidatePhase::PreflightMutation,
        DrawingMutationCandidatePhase::LocateCloneSource,
        DrawingMutationCandidatePhase::PrepareOwnedValue,
        DrawingMutationCandidatePhase::PlanOwnedValue,
        DrawingMutationCandidatePhase::BindOverlay,
        DrawingMutationCandidatePhase::LocatePrimary,
        DrawingMutationCandidatePhase::LocateSecondary,
        DrawingMutationCandidatePhase::Apply,
        DrawingMutationCandidatePhase::RebuildSource,
        DrawingMutationCandidatePhase::LocateDestination,
        DrawingMutationCandidatePhase::RebuildDestination,
    ];
    for stage in stages {
        for stale in [false, true] {
            let mut source = nested_snapshot();
            let (group_id, target) = match source.layers.last().expect("Drawing group") {
                DrawingLayerNode::Group(group) => (group.base.id.clone(), semio_s_artifact_draw_drawing_model::layer_id(&group.children[0]).clone()),
                _ => unreachable!("Drawing fixture group remains exact"),
            };
            let last_valid_id = source.id.clone();
            let mutation = match stage {
                DrawingMutationCandidatePhase::LocateSecondary => DrawingMutation::CreateLayer(CreateLayer { parent_id: Some(group_id), index: Some(0), layer: Box::new(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("cancel-create")).to_string().into()).expect("nonempty authored identity"), "cancel-create", Default::default())) }),
                DrawingMutationCandidatePhase::LocatePrimary => DrawingMutation::SetLayerVisible(SetLayerVisible { layer_id: target, visible: false }),
                DrawingMutationCandidatePhase::RebuildSource | DrawingMutationCandidatePhase::LocateDestination => DrawingMutation::ReorderLayer(ReorderLayer { layer_id: target, parent_id: Some(group_id), index: 2 }),
                _ => DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target }),
            };
            let operation = semio_framework_job::OperationId(8_003);
            let generation = semio_framework_job::Generation(83);
            let mut authority = borrowed_candidate(operation, generation).expect("Drawing candidate fixed owner arenas admit");
            let cancel = semio_framework_job::root_cancel_token();
            let mut preview_sequence = 0;
            for _ in 0..100_000 {
                if authority.phase == stage {
                    break;
                }
                let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
                let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
                assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing stage fixture advances"));
            }
            assert_eq!(authority.phase, stage, "fixture reaches each replay/candidate/container stage exactly");
            if !stale {
                cancel.cancel_now();
            }
            let mut rejected_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut rejected = semio_framework_job::StepContext::new(
                operation,
                if stale { semio_framework_job::Generation(84) } else { generation },
                semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT),
                cancel,
                semio_framework_job::default_now_us,
                &mut preview_sequence,
                &mut rejected_retained,
            );
            assert_eq!(authority.step(&mut source, &mutation, &mut rejected), Err(if stale { "drawing-store.mutation-candidate-stale-authority" } else { "drawing-store.mutation-candidate-cancelled" }),);
            close_candidate(&mut authority, Some(&mut source));
            drop(authority);
            assert_eq!(source.id, last_valid_id, "cancel/stale close never publishes a partial candidate");
            drain_mutation(mutation);
            drain_snapshot(source);
        }
    }
}

#[test]
fn retained_drawing_committed_candidate_finishes_exact_owner_return_after_late_cancel_or_stale_context() {
    for stale in [false, true] {
        let mut source = nested_snapshot();
        let initial_layers = source.layers.len();
        let target = semio_s_artifact_draw_drawing_model::layer_id(source.layers.last().expect("Drawing duplicate source")).clone();
        let mutation = DrawingMutation::DuplicateLayer(DuplicateLayer { identities: duplicate_assignments(&source,&target), layer_id: target });
        let operation = semio_framework_job::OperationId(8_004);
        let generation = semio_framework_job::Generation(84);
        let mut authority = borrowed_candidate(operation, generation).expect("Drawing candidate fixed owner arenas admit");
        let cancel = semio_framework_job::root_cancel_token();
        let mut preview_sequence = 0;
        for _ in 0..100_000 {
            if authority.phase == DrawingMutationCandidatePhase::Complete {
                break;
            }
            let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(operation, generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
            assert!(!authority.step(&mut source, &mutation, &mut context).expect("Drawing candidate reaches its committed phase"));
        }
        assert_eq!(authority.phase, DrawingMutationCandidatePhase::Complete);
        if !stale {
            cancel.cancel_now();
        }
        let terminal_generation = if stale { semio_framework_job::Generation(generation.0 + 1) } else { generation };
        let mut terminal = false;
        for _ in 0..100_000 {
            let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
            let mut context = semio_framework_job::StepContext::new(operation, terminal_generation, semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), cancel.clone(), semio_framework_job::default_now_us, &mut preview_sequence, &mut context_retained);
            if authority.step(&mut source, &mutation, &mut context).expect("a committed candidate finishes exact arena return despite a late context change") {
                terminal = true;
                break;
            }
        }
        assert!(terminal, "committed Drawing candidate reaches its terminal witness");
        authority.take().expect("committed Drawing candidate exact terminal owner handoff");
        assert!(authority.terminal_is_empty());
        assert_eq!(source.layers.len(), initial_layers + 1, "late cancellation or staleness cannot roll back an already-published duplicate");
        drop(authority);
        drain_mutation(mutation);
        drain_snapshot(source);
    }
}
