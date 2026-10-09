use super::*;
#[test]
fn nullable_selection_serde_text_and_binary_round_trip() {
    for value in [None, Some("null".into()), Some("".into()), Some("☃\n".into())] {
        let mutation = ChangeTestConfigSelection { selected: value };
        assert_eq!(serde_json::from_str::<ChangeTestConfigSelection>(&serde_json::to_string(&mutation).unwrap()).unwrap(), mutation);
        assert_eq!(ChangeTestConfigSelection::parse_op(&mutation.print_op()).unwrap(), mutation);
        assert_eq!(ChangeTestConfigSelection::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
    }
    for raw in ["{}", "{\"selected\":1}", "{\"selected\":null,\"other\":true}"] {
        assert!(serde_json::from_str::<ChangeTestConfigSelection>(raw).is_err());
    }
}
#[test]
fn structural_config_diff_serde_preserves_identity_clear_and_set() {
    for diff in [TestConfigDiff::Identity, TestConfigDiff::Clear, TestConfigDiff::Set("next".into())] {
        assert_eq!(serde_json::from_str::<TestConfigDiff>(&serde_json::to_string(&diff).unwrap()).unwrap(), diff);
    }
}

#[test]
fn test_config_retirement_original_nullable_selection() {
    for value in [None,Some(""),Some("α\0😀")] { for capacity in [0,8192,65536] { for body in [1,17,4096] {
        for scope in ["snapshot","leaf","mutation"] {
            let(original,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||value.map(|value|{let mut text=String::with_capacity(capacity);text.push_str(value);text}));assert_eq!(heap.released_bytes,0);let held=heap.requested_bytes;
            assert_eq!(serde_json::to_value(&original).unwrap(),serde_json::to_value(value).unwrap());
            match scope {
                "snapshot"=>close_config_original(TestConfig{selected:original},held,body,scope),
                "leaf"=>close_config_original(ChangeTestConfigSelection{selected:original},held,body,scope),
                _=>close_config_original(TestConfigMutation::ChangeTestConfigSelection(ChangeTestConfigSelection{selected:original}),held,body,scope),
            }
        }
    }}}
}

/// 🧮️ Measures original nullable fixture fields and their declared native cursor scaffold separately.
fn close_config_original<T:semio_framework_value::retirement::RetireOwned>(original:T,held:usize,body:usize,scope:&str) {
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
    use semio_framework_trace::observe_heap_allocations_on_this_thread;
    let(mut owner,heap)=observe_heap_allocations_on_this_thread(||ControlledRetirement::new(original).unwrap_or_else(|_|panic!("native config owner")));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut births=0;let mut released=0;let mut turns=0;
    while !owner.terminal_is_empty() {
        let((copy,capacity,release,depth),heap)=observe_heap_allocations_on_this_thread(||(owner.next_copy_byte_demand().unwrap(),owner.next_capacity_byte_demand(body).unwrap(),owner.next_release_byte_demand().unwrap(),owner.next_depth_demand().unwrap()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.max(copy),maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
        for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(copy>0).then_some(RetainedCloneGrant{maximum_copy_bytes:copy.saturating_sub(1),..grant}),(capacity>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:capacity.saturating_sub(1),..grant}),(release>0).then_some(RetainedCloneGrant{maximum_release_bytes:release.saturating_sub(1),..grant}),(depth>0).then_some(RetainedCloneGrant{maximum_depth:depth.saturating_sub(1),..grant})].into_iter().flatten(){let(step,heap)=observe_heap_allocations_on_this_thread(||owner.step(denied));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Ok(step)=step{assert_eq!(step.progress(),Default::default());}}
        let(step,heap)=observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);assert!(step.progress().copied_items>0||matches!(step,RetainedCloneStep::Complete(_)));
    }
    assert_eq!(released,held+births);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Actual config nullable owner={scope} body={body} original={held} births={births} released={released} terminalDrop=0");
}
