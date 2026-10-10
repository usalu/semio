use super::*;
use crate::app::*;

#[semio_framework_async_macros::async_test]
async fn direct_owned_child_dispatch_retains_original_parent_and_children() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let mut app = contract_composed_app_raw().await;
        register_test_child(&mut app, "child-1").await;
        let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
        if row["parentTouched"].as_bool().unwrap() { emit.artifact_mutations.push(TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() })); }
        emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", "child-1", vec![TestMutation::SetCount(SetCount { value: row["childCount"].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })]));
        for _ in 0..law["maximumTurns"].as_u64().unwrap() { if matches!(emit.prepare_child_one(law["maximumItems"].as_u64().unwrap() as usize, law["maximumBytes"].as_u64().unwrap() as usize).unwrap(), ChildEmitPreparationStep::Ready(_)) { break; } }
        assert_eq!(emit.owned_child_emits.len(), 1);
        let result = app.test_dispatch_emit("compositeEdit", emit, &meta()).await.expect("direct dispatch retains owned publication under a genuine mounted operation");
        assert!(result.output.get("operationId").and_then(DslValue::as_str).is_some(), "direct dispatch returns its actual admitted operation identity");
        assert!(result.mutations.is_empty(), "publication remains staged at admission");
        drain_and_close_composed_fixture(&mut app);
        println!("[DEBUG] direct original owned dispatch parent={} actualoperationreceipt, source retained for bounded continuation/cancellation", row["parentTouched"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn direct_owned_child_dispatch_retains_original_output_when_authority_capture_refuses() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for refused in [false, true] {
        let row = &law["cases"][1];
        let mut app = contract_composed_app_raw().await;
        register_test_child(&mut app, "child-1").await;
        let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
        emit.artifact_mutations.push(TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() }));
        emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", "child-1", vec![TestMutation::SetCount(SetCount { value: row["childCount"].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })]));
        for _ in 0..law["maximumTurns"].as_u64().unwrap() { if matches!(emit.prepare_child_one(law["maximumItems"].as_u64().unwrap() as usize, law["maximumBytes"].as_u64().unwrap() as usize).unwrap(), ChildEmitPreparationStep::Ready(_)) { break; } }
        let original = emit.owned_child_emits.as_ptr() as usize;
        assert_eq!(emit.owned_child_emits.len(), 1);
        let verb = if refused { "unregisteredOriginalChildVerb" } else { "compositeEdit" };
        let result = app.test_dispatch_emit(verb, emit, &meta()).await.expect("original output remains mounted even after authority capture refusal");
        let operation = result.output.get("operationId").and_then(DslValue::as_str).unwrap().parse::<u64>().unwrap();
        let (observation, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| app.test_original_owned_emit_observation(operation).unwrap());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(observation, (1, 1, original, refused, refused));
        assert!(result.mutations.is_empty());
        drain_and_close_composed_fixture(&mut app);
        println!("[DEBUG] original completed output operation={operation} capture-refused={refused} unchanged-parent/child-pointer=true retained-before-refusal=true original-keyed-cancellation={refused}");
    }
}

mod reserved_emission_admission_laws { include!("🪪️admission/🦀️.rs"); }
