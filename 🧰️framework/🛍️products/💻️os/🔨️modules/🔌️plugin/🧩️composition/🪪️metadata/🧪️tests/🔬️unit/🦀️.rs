use super::*;

fn grant(copy: usize, capacity: usize, release: usize) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: 8 } }
fn reference(value: &serde_json::Value) -> ArtifactRef { ArtifactRef { artifact_id: value["artifact_id"].as_str().unwrap().into(), dialect: ArtifactDialect { artifact_kind: value["dialect"]["artifact_kind"].as_str().unwrap().into(), standard: value["dialect"]["standard"].as_str().unwrap().into(), subset: value["dialect"]["subset"].as_str().unwrap().into() } } }

fn retire(owner: &mut PrivateChildMemberMetadataIssuer) {
    let mut count = 0;
    while !owner.terminal_is_empty() {
        let demand = owner.next_close_byte_demand();
        if demand != 0 {
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(grant(0, 0, demand - 1)).unwrap());
            assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            assert_eq!(owner.next_close_byte_demand(), demand);
        }
        let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_granted(grant(0, 0, demand)).unwrap());
        let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
        assert_eq!((events.requested_bytes, events.released_bytes), (0, progress.released_bytes));
        assert!(progress.released_bytes <= demand);
        count += 1;
        assert!(count <= 33);
    }
}

#[test]
fn private_child_metadata_copies_all_thirty_one_original_fields_and_admits_each_whole_capacity() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🏪️store/🧩️composition/🚪️open/🌱️genesis/🧫️fixtures/🔣️.json")).unwrap();
    let publication_fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
      for transaction in publication_fixture["transactions"].as_array().unwrap() {
        let transaction = (!transaction.is_null()).then(|| protocol::TransactionRef { id: transaction["id"].as_str().unwrap().into(), tool: transaction["tool"].as_str().unwrap().into() });
        let expected = reference(&row["expected"]);
        let parent = reference(&row["owner"]["parent"]);
        let key = MemberKeyRef::root(row["owner"]["slot"].as_str().unwrap(), row["owner"]["child_id"].as_str().unwrap());
        let source = PrivateChildMemberMetadataSource { expected: &expected, parent_id: &parent.artifact_id, parent_dialect: &parent.dialect, key, actor: publication_fixture["actor"].as_str().unwrap(), transaction: transaction.as_ref(), group_id: transaction.as_ref().map(|transaction| transaction.id.as_str()) };
        let original = source.fields().map(str::to_owned);
        let mut issuer = PrivateChildMemberMetadataIssuer::new();
        let mut turns = 0;
        while !issuer.ready() {
            let capacity = issuer.next_capacity_byte_demand(source).unwrap();
            if capacity != 0 {
                for denied in [RetainedCloneGrant { maximum_items: 0, ..grant(64, capacity, 0) }, grant(64, capacity - 1, 0)] {
                    let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| issuer.advance(source, denied).unwrap());
                    assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
                    assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                    assert_eq!(issuer.next_capacity_byte_demand(source), Some(capacity));
                }
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| issuer.advance(source, grant(64, capacity, 0)).unwrap());
            let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
            assert!(progress.copied_bytes <= 64 && progress.copied_items <= 1);
            assert_eq!((events.requested_bytes, events.released_bytes), (progress.retained_capacity_bytes, 0));
            turns += 1;
            assert!(turns <= 80);
        }
        assert!(issuer.take_ready(RetainedCloneGrant { maximum_items: 0, ..grant(0, 0, 0) }).unwrap().is_none());
        let mut metadata = issuer.take_ready(grant(0, 0, 0)).unwrap().unwrap();
        assert!(issuer.terminal_is_empty());
        let (parts, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| metadata.take_ready(grant(0, 0, 0)).unwrap().unwrap());
        assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
        assert!(metadata.terminal_is_empty());
        let actual = parts.fields().map(|field| field.map_or("", String::as_str));
        assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(&original).unwrap());
        assert_eq!(parts.transaction, transaction);
        assert_eq!(parts.group_id.as_deref(), transaction.as_ref().map(|transaction| transaction.id.as_str()));
        assert_eq!(parts.publication_actor, source.actor);
        assert_eq!(parts.registry_owner, parts.owner);
        println!("[DEBUG] private child metadata case={} fields=31 transaction-present={} UTF8-originals=true admitted-turns={turns} max-copy=64", row["id"], transaction.is_some());
      }
    }
}

#[test]
fn private_child_metadata_cancellation_retains_every_original_allocation_until_whole_release() {
    let expected = ArtifactRef { artifact_id: "child-δ".into(), dialect: ArtifactDialect { artifact_kind: "kind".into(), standard: "1".into(), subset: "*".into() } };
    let key = MemberKeyRef::root("slot-α", "child-δ");
    let transaction = protocol::TransactionRef { id: "txn:δ".into(), tool: "cad#verschieben-ä".into() };
    let source = PrivateChildMemberMetadataSource { expected: &expected, parent_id: "parent-β", parent_dialect: &expected.dialect, key, actor: "actor:private-child", transaction: Some(&transaction), group_id: Some(&transaction.id) };
    for stop in [0, 1, 3, 17, 31, 62] {
        let mut issuer = PrivateChildMemberMetadataIssuer::new();
        for _ in 0..stop { let capacity = issuer.next_capacity_byte_demand(source).unwrap(); issuer.advance(source, grant(64, capacity, 0)).unwrap(); }
        retire(&mut issuer);
    }
    let mut issuer = PrivateChildMemberMetadataIssuer::new();
    while !issuer.ready() { let capacity = issuer.next_capacity_byte_demand(source).unwrap(); issuer.advance(source, grant(64, capacity, 0)).unwrap(); }
    let mut metadata = issuer.take_ready(grant(0, 0, 0)).unwrap().unwrap();
    for _ in 0..32 {
        let demand = metadata.next_close_byte_demand();
        if demand != 0 {
            for denied in [RetainedCloneGrant { maximum_items: 0, ..grant(0, 0, demand) }, grant(0, 0, demand - 1)] {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| metadata.close_granted(denied).unwrap());
                assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(metadata.next_close_byte_demand(), demand);
            }
        }
        let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| metadata.close_granted(grant(0, 0, demand)).unwrap());
        let progress = match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress };
        assert_eq!((events.requested_bytes, events.released_bytes), (0, progress.released_bytes));
        if metadata.terminal_is_empty() { break; }
    }
    assert!(metadata.terminal_is_empty());
    println!("[DEBUG] private child metadata six cancellation phases: all capacities remain underfunded and exact paid releases match native allocator events");
}

#[test]
fn private_parent_metadata_shares_exact_bounded_birth_copy_and_typed_transfer() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["transactions"].as_array().unwrap() {
        let transaction = (!row.is_null()).then(|| protocol::TransactionRef { id: row["id"].as_str().unwrap().into(), tool: row["tool"].as_str().unwrap().into() });
        let source = PrivatePublicationMetadataSource { actor: fixture["actor"].as_str().unwrap(), transaction: transaction.as_ref(), group_id: transaction.as_ref().map(|transaction| transaction.id.as_str()) };
        let mut issuer = PrivatePublicationMetadataIssuer::new();
        let mut turns = 0;
        while !issuer.ready() {
            let capacity = issuer.next_capacity_byte_demand(source).unwrap();
            if capacity != 0 {
                for denied in [RetainedCloneGrant { maximum_items: 0, ..grant(64, capacity, 0) }, grant(64, capacity - 1, 0)] {
                    let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| issuer.advance(source, denied).unwrap());
                    assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
                    assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                    assert_eq!(issuer.next_capacity_byte_demand(source), Some(capacity));
                }
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| issuer.advance(source, grant(64, capacity, 0)).unwrap());
            let progress = step.progress();
            assert!(progress.copied_bytes <= 64 && progress.copied_items <= 1);
            assert_eq!((events.requested_bytes, events.released_bytes), (progress.retained_capacity_bytes, 0));
            turns += 1;
            assert!(turns <= 16);
        }
        let (metadata, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| issuer.take_ready(grant(0, 0, 0)).unwrap().unwrap());
        assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
        assert!(issuer.terminal_is_empty());
        let mut metadata = metadata;
        assert!(metadata.take_ready(RetainedCloneGrant { maximum_items: 0, ..grant(0, 0, 0) }).unwrap().is_none());
        let (parts, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| metadata.take_ready(grant(0, 0, 0)).unwrap().unwrap());
        assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
        assert_eq!(parts.actor, source.actor);
        assert_eq!(parts.transaction, transaction);
        assert_eq!(parts.group_id.as_deref(), transaction.as_ref().map(|transaction| transaction.id.as_str()));
        assert!(metadata.terminal_is_empty());
        let pointer = parts.actor.as_ptr();
        let (mut restored, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| PrivatePublicationMetadata::from_parts(parts));
        assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
        assert_eq!(restored.parts_mut().unwrap().actor.as_ptr(), pointer);
        for _ in 0..5 {
            let demand = restored.next_close_byte_demand();
            if demand != 0 {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| restored.close_granted(grant(0, 0, demand - 1)).unwrap());
                assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(restored.next_close_byte_demand(), demand);
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| restored.close_granted(grant(0, 0, demand)).unwrap());
            assert_eq!((events.requested_bytes, events.released_bytes), (0, step.progress().released_bytes));
            if restored.terminal_is_empty() { break; }
        }
        assert!(restored.terminal_is_empty());
        println!("[DEBUG] parent metadata transaction-present={} fields=4 max-copy=64 transfer-birth=0 exact-close=true", transaction.is_some());
    }
}

#[test]
fn private_parent_metadata_cancellation_never_frees_a_fractionally_funded_original() {
    let transaction = protocol::TransactionRef { id: "txn:δ".into(), tool: "cad#verschieben-ä".into() };
    let source = PrivatePublicationMetadataSource { actor: "actor:private-child", transaction: Some(&transaction), group_id: Some(&transaction.id) };
    for stop in [0, 1, 3, 7, 8] {
        let mut issuer = PrivatePublicationMetadataIssuer::new();
        for _ in 0..stop { let capacity = issuer.next_capacity_byte_demand(source).unwrap(); issuer.advance(source, grant(64, capacity, 0)).unwrap(); }
        for _ in 0..6 {
            if issuer.terminal_is_empty() { break; }
            let demand = issuer.next_close_byte_demand();
            if demand != 0 {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| issuer.close_granted(grant(0, 0, demand - 1)).unwrap());
                assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(issuer.next_close_byte_demand(), demand);
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| issuer.close_granted(grant(0, 0, demand)).unwrap());
            assert_eq!((events.requested_bytes, events.released_bytes), (0, step.progress().released_bytes));
        }
        assert!(issuer.terminal_is_empty());
    }
    println!("[DEBUG] parent metadata five cancellation phases preserve whole physical allocations under denied grants");
}

#[test]
fn private_publication_metadata_preserves_group_id_independently_of_transaction() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let expected = ArtifactRef { artifact_id: "child:δ".into(), dialect: ArtifactDialect { artifact_kind: "flow".into(), standard: "1".into(), subset: "*".into() } };
    for transaction in fixture["transactions"].as_array().unwrap() {
        let transaction = (!transaction.is_null()).then(|| protocol::TransactionRef { id: transaction["id"].as_str().unwrap().into(), tool: transaction["tool"].as_str().unwrap().into() });
        for group_id in fixture["groupIds"].as_array().unwrap() {
            let group_id = group_id.as_str();
            let parent_source = PrivatePublicationMetadataSource { actor: fixture["actor"].as_str().unwrap(), transaction: transaction.as_ref(), group_id };
            let child_source = PrivateChildMemberMetadataSource { expected: &expected, parent_id: "parent:β", parent_dialect: &expected.dialect, key: MemberKeyRef::root("slot:α", "local:δ"), actor: parent_source.actor, transaction: transaction.as_ref(), group_id };
            let mut parent = PrivatePublicationMetadataIssuer::new();
            let mut child = PrivateChildMemberMetadataIssuer::new();
            for _ in 0..80 {
                if !parent.ready() {
                    let capacity = parent.next_capacity_byte_demand(parent_source).unwrap();
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| parent.advance(parent_source, grant(64, capacity, 0)).unwrap());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, 0));
                }
                if !child.ready() {
                    let capacity = child.next_capacity_byte_demand(child_source).unwrap();
                    let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| child.advance(child_source, grant(64, capacity, 0)).unwrap());
                    assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, 0));
                }
                if parent.ready() && child.ready() { break; }
            }
            assert!(parent.ready() && child.ready());
            let mut parent_metadata = parent.take_ready(grant(0, 0, 0)).unwrap().unwrap();
            let mut child_metadata = child.take_ready(grant(0, 0, 0)).unwrap().unwrap();
            assert_eq!(parent_metadata.parts().unwrap().transaction, transaction);
            assert_eq!(child_metadata.parts().unwrap().transaction, transaction);
            assert_eq!(parent_metadata.parts().unwrap().group_id.as_deref(), group_id);
            assert_eq!(child_metadata.parts().unwrap().group_id.as_deref(), group_id);
            for _ in 0..33 {
                if !parent_metadata.terminal_is_empty() { let bytes = parent_metadata.next_close_byte_demand(); parent_metadata.close_granted(grant(0, 0, bytes)).unwrap(); }
                if !child_metadata.terminal_is_empty() { let bytes = child_metadata.next_close_byte_demand(); child_metadata.close_granted(grant(0, 0, bytes)).unwrap(); }
            }
            assert!(parent_metadata.terminal_is_empty() && child_metadata.terminal_is_empty());
        }
    }
    println!("[DEBUG] common parent/child group identities: nine optional transaction/group combinations; fields4/31 original; copy64; same-turn allocator birth parity; no synthesized transaction");
}
