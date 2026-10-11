use super::*;
use crate::app::*;
use semio_framework_value::retained_clone::RetainedCloneGrant;

#[test]
fn mounted_owned_child_parent_fixture_requires_exact_original_mutation_admission() {
    assert!(<TestApp as ArtifactApp>::owned_mutation_batch_birth_bytes().is_some(), "the real declared-slot app must own its paired typed admission");
}

#[test]
fn mounted_owned_child_original_preparation_preserves_actual_operation_schema() {
    let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
    let original = vec![TestMutation::SetCount(SetCount { value: 5 }), TestMutation::SetLabel(SetLabel { value: "child 雪\0".into() })];
    let original_pointer = original.as_ptr();
    emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", "child-1", original));
    for _ in 0..100000 {
        let before = emit.child_preparations.front().and_then(ChildEmitPreparation::accepted_prefix).map_or(0, |prefix| prefix.op_schema.0.len());
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| prepare_original_fixture_child(&mut emit,1,262144).unwrap());
        let after = emit.child_preparations.front().and_then(ChildEmitPreparation::accepted_prefix).map_or(before, |prefix| prefix.op_schema.0.len());
        assert!(after.saturating_sub(before) <= 64);
        assert!(heap.requested_bytes <= 262144 && heap.released_bytes <= 262144);
        if matches!(step, ChildEmitPreparationStep::Ready(_)) { break; }
    }
    assert_eq!(emit.owned_child_emits.len(), 1);
    assert!(emit.child_emits.is_empty());
    assert_eq!(emit.owned_child_emits[0].mutations::<TestMutation>().unwrap().as_ptr(), original_pointer);
    let schema = emit.owned_child_emits[0].metadata().unwrap().op_schema.0.clone();
    while !emit.owned_child_emits.is_empty() || emit.owned_child_emits.capacity() != 0 {
        let demand=emit.close_child_demands(262144).expect("original child retirement quote");
        let turn=crate::app::plugin_demand_grant(demand);let step=emit.close_child_one(turn).unwrap();if let Some(PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress))=step{assert!(progress.fits(turn));}
    }
    assert_eq!(schema, "count.set-count", "owned typed source carries its first original operation's authored schema before mounted publication");
    println!("[DEBUG] original owned child schema={schema}, source pointer retained, borrowed schema copies<=64 and whole physical births/releases<=262144 per admitted turn");
}

#[semio_framework_async_macros::async_test]
async fn mounted_owned_child_actual_publisher_commits_original_parent_and_children_before_result_ack() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut app = contract_composed_app_raw(crate::app::artifact_app_laws::fixture_mounted_policy(), &mut crate::app::artifact_app_laws::fixture_identity()).await;
        let count = row["children"].as_u64().unwrap() as usize;
        for index in 0..count { register_test_child(&mut app, &format!("child-{}", index + 1)).await; }
        let mut emit: Emit<TestMutation, TestConfigMutation> = Emit::default();
        if row["parentTouched"].as_bool().unwrap() { emit.artifact_mutations.push(TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() })); }
        for index in 0..count {
            emit.child_preparations.push_back(ChildEmitPreparation::of_owned::<TestSnapshot, TestMutation>("slot", format!("child-{}", index + 1), vec![TestMutation::SetCount(SetCount { value: row["childCounts"][index].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })]));
        }
        for _ in 0..100000 { if matches!(emit.prepare_child_one(1, 262144).unwrap(), ChildEmitPreparationStep::Ready(_)) { break; } }
        assert_eq!(emit.owned_child_emits.len(), count);
        assert!(emit.child_emits.is_empty());
        let (mutations, inverse_group, operation) = crate::app::test_mounted_original_owned_publication(&mut app, emit, meta(), row["parentTouched"].as_bool().unwrap(), |app| (0..count).map(|index| {
            let TestMembers::Child(child) = &app.children.get(&("slot".into(), format!("child-{}", index + 1))).unwrap().member;
            child.snapshot_ref().count != 0
        }).collect()).await;
        assert!(operation != 0);
        assert_eq!(mutations.len(), row["expectedOperations"].as_u64().unwrap() as usize);
        assert_eq!(inverse_group.member_edits.len(), row["expectedMemberEdits"].as_u64().unwrap() as usize);
        assert!(inverse_group.member_edits.iter().all(|member| !member.edit_id.is_empty()));
        let expected: Vec<_> = (row["parentTouched"].as_bool().unwrap().then(|| TestMutation::SetLabel(SetLabel { value: row["parentLabel"].as_str().unwrap().into() }))).into_iter().chain((0..count).flat_map(|index| [TestMutation::SetCount(SetCount { value: row["childCounts"][index].as_i64().unwrap() as i32 }), TestMutation::SetLabel(SetLabel { value: row["childLabel"].as_str().unwrap().into() })])).collect();
        for (mutation, original) in mutations.iter().zip(&expected) {
            let semantics = protocol::SemanticMutation::semantics(original);
            assert_eq!(mutation.diff.schema.0, format!("{}.{}", semantics.entity, semantics.kind));
            assert_eq!(<TestMutation as protocol::OpBinary>::decode_op(&mutation.diff.payload).unwrap(), *original);
            assert!(!mutation.id.0.is_empty());
            let inverses = protocol::decode_ops_vec(&mutation.inverse.inverse_diff.payload).unwrap();
            assert!(!inverses.is_empty());
            for inverse in inverses { assert!(<TestMutation as protocol::OpBinary>::decode_op(&inverse).is_ok()); }
        }
        for index in 0..count {
            let TestMembers::Child(child) = &app.children.get(&("slot".into(), format!("child-{}", index + 1))).unwrap().member;
            assert_eq!(child.snapshot_ref().count, row["childCounts"][index].as_i64().unwrap() as i32);
            assert_eq!(child.snapshot_ref().label, row["childLabel"].as_str().unwrap());
        }
        println!("[DEBUG] actual mounted original owned group parent={} children={count} operations={} memberEdits={} Child page stable until exact ACK", row["parentTouched"], mutations.len(), inverse_group.member_edits.len());
        drop(mutations);
        drop(inverse_group);
        drain_and_close_composed_fixture(&mut app, crate::app::artifact_app_laws::fixture_mounted_policy());
    }
}

#[test]
fn original_pending_child_group_close_preserves_all_sources_and_full_grants(){
 let original:serde_json::Value=serde_json::from_str(include_str!("../../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🧾️fixture-caller/🔣️.json")).unwrap();let policy=original["native"]["grant"].as_array().unwrap();let grant=RetainedCloneGrant{maximum_items:policy[0].as_u64().unwrap()as usize,maximum_copy_bytes:policy[1].as_u64().unwrap()as usize,maximum_capacity_bytes:policy[2].as_u64().unwrap()as usize,maximum_release_bytes:policy[3].as_u64().unwrap()as usize,maximum_depth:policy[4].as_u64().unwrap()as usize};
 for count in[0,1,17]{crate::app::test_original_pending_child_group_closure::<TestApp>(||{(0..count).map(|_|TestMutation::SetLabel(SetLabel{value:"original 雪".repeat(8192)})).collect()},grant);}
}
