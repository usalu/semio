//! 🧪️ Original-bound retained parameter decisions, inverses and cancellation against neutral vectors.
use super::*;
use protocol::{ColdMutation, Mutation, RetainedMutationAdvance, RetainedMutationCloseContext, RetainedMutationDecision, RetainedMutationLeaf, RetainedMutationLeafPreparation};
use semio_framework_value::{retirement::RetireOwned, RetirementDemand, retained_clone::{RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep}};

#[global_allocator]
static REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

macro_rules! test_wrapper {
    ($leaf:ty, $variant:ident) => {
        impl protocol::RetainedLeafWrapper<$leaf> for RemodelingMutation {
            fn wrap_work_demand() -> u64 { 1 }
            fn wrap(value: $leaf) -> (Self, u64) { (Self::$variant(value), 1) }
        }
    };
}
test_wrapper!(UpdateIngestParams, UpdateIngestParams);
test_wrapper!(UpdateFeatureParams, UpdateFeatureParams);
test_wrapper!(UpdateMatchParams, UpdateMatchParams);
test_wrapper!(UpdateSfmParams, UpdateSfmParams);
test_wrapper!(UpdateDenseParams, UpdateDenseParams);
test_wrapper!(UpdateMeshParams, UpdateMeshParams);
test_wrapper!(UpdateMotionParams, UpdateMotionParams);
test_wrapper!(UpdateGeoParams, UpdateGeoParams);

fn grant(d: RetirementDemand) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: d.copy_bytes, maximum_capacity_bytes: d.capacity_bytes, maximum_release_bytes: d.release_bytes, maximum_depth: d.depth } }
fn denials(d: RetirementDemand) -> Vec<RetainedCloneGrant> {
    let g = grant(d);
    [Some(RetainedCloneGrant { maximum_items: 0, ..g }), (d.copy_bytes > 0).then_some(RetainedCloneGrant { maximum_copy_bytes: d.copy_bytes.saturating_sub(1), ..g }), (d.capacity_bytes > 0).then_some(RetainedCloneGrant { maximum_capacity_bytes: d.capacity_bytes.saturating_sub(1), ..g }), (d.release_bytes > 0).then_some(RetainedCloneGrant { maximum_release_bytes: d.release_bytes.saturating_sub(1), ..g }), (d.depth > 0).then_some(RetainedCloneGrant { maximum_depth: d.depth.saturating_sub(1), ..g })].into_iter().flatten().collect()
}
fn source<T: RetireOwned + Sync + 'static>(value: T) -> RetainedCloneSource<T> {
    let mut admission = RetainedCloneSource::admission_owned(value, ());
    let mut output = None;
    for _ in 0..1000 {
        if admission.terminal_is_empty() { break; }
        let g = grant(admission.next_demand().unwrap());
        if admission.phase() == "ready" { admission.take_output(&mut output, g).result.unwrap(); }
        else { let mut actual = None; let turn = admission.advance(g, &mut actual); let progress = turn.result.unwrap().progress(); if let Some(bytes) = actual { assert_eq!(bytes, progress.retained_capacity_bytes); } }
    }
    output.expect("real original source admission")
}
fn close_source<T: RetireOwned + Sync + 'static>(source: &mut RetainedCloneSource<T>) {
    while !source.terminal_is_empty() { source.close_step(grant(source.next_close_demand(4096).unwrap())).result.unwrap(); }
}
fn close_messages(rows: Vec<protocol::MutationMessage>) {
    let mut ledger = protocol::MutationMessageLedgerRetirement::new(String::new(), rows);
    while !ledger.terminal_is_empty() {
        let d = RetirementDemand { copy_bytes: ledger.next_copy_byte_demand(), capacity_bytes: ledger.next_capacity_byte_demand(4096).unwrap(), release_bytes: ledger.next_release_byte_demand().unwrap(), depth: ledger.next_depth_demand().unwrap() };
        let g = grant(d); assert!(ledger.close_step(g).unwrap().progress().fits(g));
    }
}

fn exercise<L>(base: RemodelingSnapshot, payload: L, operation: RemodelingMutation, cancel: Option<usize>, inverse_cancel: Option<usize>) -> (usize, usize)
where
    L: RetireOwned + Sync + 'static + RetainedMutationLeaf<RemodelingSnapshot, RemodelingMutation>,
{
    let cold = operation.diff(&base);
    let expected_diff = cold.diff().clone();
    let expected_messages = cold.messages().to_vec();
    let expected_refusal = expected_messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal));
    let expected_inverse = operation.inverse(&base).unwrap();
    let mut original = source(base);
    let mut authored = source(payload);
    let mut owner = <<L as RetainedMutationLeaf<RemodelingSnapshot, RemodelingMutation>>::Preparation as RetainedMutationLeafPreparation<RemodelingSnapshot, RemodelingMutation, L>>::new();
    let factory = semio_framework_value::factory::OwnedValueRetirementFactory::<RemodelingMutation>::default();
    let context = RetainedMutationCloseContext { mutations: &factory };
    let mut retained_diff = None;
    let mut ready = false;
    let mut decision_turns = 0;
    let mut inverse_turns = 0;
    for turn_index in 0..2000 {
        if cancel == Some(turn_index) { break; }
        let demand = owner.decision_demand(authored.borrow(), original.borrow(), 8, 4096).unwrap();
        let work = owner.decision_work_demand(authored.borrow(), original.borrow(), 8, 4096).unwrap();
        let g = grant(demand);
        let (denied_work, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.decide(authored.borrow(), original.borrow(), 8, 0, g)); assert_eq!((born, released), (0, 0)); assert_eq!(denied_work.work, 0); assert_eq!(denied_work.result.unwrap().progress(), Default::default());
        for denied in denials(demand) { let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.decide(authored.borrow(), original.borrow(), 8, 4096, denied)); assert_eq!((born, released), (0, 0)); assert_eq!(turn.work, 0); assert_eq!(turn.result.unwrap().progress(), Default::default()); }
        let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.decide(authored.borrow(), original.borrow(), 8, 4096, g));
        decision_turns += 1;
        assert!(turn.work <= work);
        let step = turn.result.unwrap(); assert!(step.progress().fits(g)); assert_eq!((born, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
        if matches!(step, RetainedMutationAdvance::Ready(_)) { ready = true; break; }
    }
    if ready {
        match owner.take_decision() {
            RetainedMutationDecision::Refused(rows) => { assert!(expected_refusal); assert_eq!(rows, expected_messages); close_messages(rows); },
            RetainedMutationDecision::Admitted(diff, rows) => {
                assert!(!expected_refusal);
                assert_eq!(diff, expected_diff); assert_eq!(rows, expected_messages); close_messages(rows);
                let mut actual_diff = source(diff);
                let mut inverse_ready = false;
                for index in 0..2000 {
                    if inverse_cancel == Some(index) { break; }
                    let demand = owner.inverse_demand(authored.borrow(), original.borrow(), actual_diff.borrow(), 8, 4096).unwrap();
                    let work = owner.inverse_work_demand(authored.borrow(), original.borrow(), actual_diff.borrow(), 8, 4096).unwrap();
                    let g = grant(demand);
                    let (denied_work, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.inverse(authored.borrow(), original.borrow(), actual_diff.borrow(), 8, 0, g)); assert_eq!((born, released), (0, 0)); assert_eq!(denied_work.work, 0); assert_eq!(denied_work.result.unwrap().progress(), Default::default());
                    for denied in denials(demand) { let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.inverse(authored.borrow(), original.borrow(), actual_diff.borrow(), 8, 4096, denied)); assert_eq!((born, released), (0, 0)); assert_eq!(turn.work, 0); assert_eq!(turn.result.unwrap().progress(), Default::default()); }
                    let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.inverse(authored.borrow(), original.borrow(), actual_diff.borrow(), 8, 4096, g)); assert!(turn.work <= work);
                    inverse_turns += 1;
                    let step = turn.result.unwrap(); assert!(step.progress().fits(g)); assert_eq!((born, released), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
                    if matches!(step, RetainedMutationAdvance::Ready(_)) { inverse_ready = true; break; }
                }
                if inverse_ready { let inverse = owner.take_inverse(); assert_eq!(inverse, expected_inverse); let mut inverse_source = source(inverse); close_source(&mut inverse_source); }
                else { assert!(inverse_cancel.is_some(), "funded inverse failed to become ready"); }
                retained_diff = Some(actual_diff);
            },
        }
    } else { assert!(cancel.is_some(), "funded decision failed to become ready"); }
    owner.begin_close();
    for _ in 0..4000 {
        if owner.terminal_is_empty() { break; }
        let d = owner.close_demand(context, 8).unwrap(); let work = owner.close_work_demand(context, 8).unwrap(); let g = grant(d);
        for denied in denials(d) { let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(context, denied)); assert_eq!((born, released), (0, 0)); assert_eq!(turn.work, 0); assert_eq!(turn.result.unwrap().progress(), Default::default()); }
        let (turn, born, released) = semio_framework_trace_memory_testing::observe_backing(|| owner.close_step(context, g)); assert!(turn.work <= work); let progress = turn.result.unwrap().progress(); assert!(progress.fits(g)); assert_eq!((born, released), (progress.retained_capacity_bytes, progress.released_bytes));
    }
    assert!(owner.terminal_is_empty());
    if let Some(diff) = retained_diff.as_mut() { close_source(diff); }
    close_source(&mut authored); close_source(&mut original);
    (decision_turns, inverse_turns)
}

fn exercise_case(case: &serde_json::Value, cancel: Option<usize>, inverse_cancel: Option<usize>) -> (usize, usize) {
    let base = semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::default_remodeling_scene();
    let change = case["change"].as_object().unwrap();
    match case["facet"].as_str().unwrap() {
        "ingest" => { let mut params = base.params.ingest.clone(); if let Some(value) = change.get("frame_sample_stride") { params.frame_sample_stride = value.as_u64().unwrap() as u32; } let payload = UpdateIngestParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateIngestParams(payload), cancel, inverse_cancel) },
        "feature" => { let mut params = base.params.feature.clone(); if let Some(value) = change.get("target_count") { params.target_count = value.as_u64().unwrap() as u32; } let payload = UpdateFeatureParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateFeatureParams(payload), cancel, inverse_cancel) },
        "matching" => { let mut params = base.params.matching.clone(); if let Some(value) = change.get("ratio_test") { params.ratio_test = value.as_f64().unwrap() as f32; } if let Some(value) = change.get("sequential_window") { params.sequential_window = value.as_u64().unwrap() as u32; } let payload = UpdateMatchParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateMatchParams(payload), cancel, inverse_cancel) },
        "sfm" => { let mut params = base.params.sfm.clone(); if let Some(value) = change.get("ransac_iterations") { params.ransac_iterations = value.as_u64().unwrap() as u32; } if change.contains_key("ransac_threshold_px") { params.ransac_threshold_px = f32::NAN; } let payload = UpdateSfmParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateSfmParams(payload), cancel, inverse_cancel) },
        "dense" => { let mut params = base.params.dense.clone(); if let Some(value) = change.get("max_points") { params.max_points = value.as_u64().unwrap() as u32; } if change.contains_key("confidence_threshold") { params.confidence_threshold = f32::NAN; } let payload = UpdateDenseParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateDenseParams(payload), cancel, inverse_cancel) },
        "mesh" => { let mut params = base.params.mesh.clone(); if let Some(value) = change.get("texture_size") { params.texture_size = value.as_u64().unwrap() as u32; } if change.contains_key("tsdf_voxel_size_mm") { params.tsdf_voxel_size_mm = f32::NAN; } let payload = UpdateMeshParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateMeshParams(payload), cancel, inverse_cancel) },
        "motion" => { let mut params = base.params.motion.clone(); if let Some(value) = change.get("enabled") { params.enabled = value.as_bool().unwrap(); } if change.contains_key("min_track_quality") { params.min_track_quality = f32::NAN; } let payload = UpdateMotionParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateMotionParams(payload), cancel, inverse_cancel) },
        "geo" => { let mut params = base.params.geo.clone(); if let Some(value) = change.get("enabled") { params.enabled = value.as_bool().unwrap(); } if let Some(value) = change.get("gsd_m") { params.gsd_m = value.as_f64().unwrap() as f32; } let payload = UpdateGeoParams { params }; exercise(base, payload.clone(), RemodelingMutation::UpdateGeoParams(payload), cancel, inverse_cancel) },
        other => panic!("unknown parameter facet {other}"),
    }
}

#[test]
fn original_parameter_decisions_and_inverses_match_neutral_and_cold_oracles() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️retained-params/🧪️.json")).unwrap();
    for case in cases.as_array().unwrap() { exercise_case(case, None, None); }
}

#[test]
fn original_parameter_decisions_close_at_every_cancelled_frontier() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️retained-params/🧪️.json")).unwrap();
    for expected in ["unchanged", "admitted", "refused"] {
        let case = cases.as_array().unwrap().iter().find(|case| case["facet"] == "feature" && case["expected"] == expected).unwrap();
        let (turns, _) = exercise_case(case, None, None);
        assert!(turns > 0);
        for frontier in 0..=turns { exercise_case(case, Some(frontier), None); }
    }
}

#[test]
fn original_parameter_inverse_closes_at_every_cancelled_frontier() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../🧫️retained-params/🧪️.json")).unwrap();
    for expected in ["unchanged", "admitted"] {
        let case = cases.as_array().unwrap().iter().find(|case| case["facet"] == "feature" && case["expected"] == expected).unwrap();
        let (_, turns) = exercise_case(case, None, None);
        assert!(turns > 0);
        for frontier in 0..=turns { exercise_case(case, None, Some(frontier)); }
    }
}
