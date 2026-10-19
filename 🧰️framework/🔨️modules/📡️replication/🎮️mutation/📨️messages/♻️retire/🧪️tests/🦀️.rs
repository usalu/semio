//! ♻️ Physical message ledger capacity releases are measured independently from payload work.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use semio_framework_diagnostic::Severity;
fn fixture()->serde_json::Value { serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap() }
fn owners(row:&serde_json::Value)->(String,Vec<MutationMessage>,usize) {
    let mut context=String::with_capacity(row["contextCapacity"].as_u64().unwrap() as usize);context.push_str(row["context"].as_str().unwrap());
    let mut messages=Vec::with_capacity(row["rowCapacity"].as_u64().unwrap() as usize);
    for row in row["messages"].as_array().unwrap() {
        messages.push(MutationMessage{level:match row["level"].as_str().unwrap(){"fatal"=>Severity::Fatal,"error"=>Severity::Error,"warning"=>Severity::Warning,_=>Severity::Info},code:row["code"].as_str().unwrap().to_owned().into(),message:row["text"].as_str().unwrap().to_owned(),target:row["target"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect(),op_index:Some(7)});
    }
    let held=context.capacity()+messages.capacity()*size_of::<MutationMessage>()+messages.iter().map(|message|message.code.0.capacity()+message.message.capacity()+message.target.capacity()*size_of::<String>()+message.target.iter().map(String::capacity).sum::<usize>()).sum::<usize>();
    (context,messages,held)
}
/// 📤️ Neutral transfer grants move the original payload once and keep denied ownership unchanged.
#[test]
fn message_ledger_transfer_is_quoted_terminal_safe_and_keeps_original_allocations() {
    let law = fixture();
    for case in law["transferCases"].as_array().unwrap() {
        let (context, messages, _) = owners(&law["cases"][0]);
        let pointer = messages.as_ptr();
        let capacity = messages.capacity();
        let mut owner = MutationMessageLedgerRetirement::new(context, messages);
        let exact = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: owner.transfer_copy_byte_demand(), maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
        if case["closeFirst"] == true {
            let grant = RetainedCloneGrant { maximum_release_bytes: owner.next_release_byte_demand().unwrap(), ..exact };
            assert!(owner.close_step(grant).unwrap().progress().copied_items > 0);
        }
        let grant = RetainedCloneGrant {
            maximum_items: case["items"].as_u64().unwrap() as usize,
            maximum_copy_bytes: match case["copy"].as_str().unwrap() { "short" => exact.maximum_copy_bytes - 1, "zero" => 0, _ => exact.maximum_copy_bytes },
            ..exact
        };
        let (result, heap) = observe_heap_allocations_on_this_thread(|| owner.take_messages_controlled(grant));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        match case["expected"].as_str().unwrap() {
            "transferred" => {
                let (messages, receipt) = result.unwrap().unwrap();
                assert_eq!(messages.as_ptr(), pointer);
                assert_eq!(messages.capacity(), capacity);
                assert_eq!(receipt.copied_bytes, exact.maximum_copy_bytes);
                assert!(receipt.fits(grant));
                assert!(owner.take_messages_controlled(exact).is_err());
                let mut transferred = MutationMessageLedgerRetirement::new(String::new(), messages);
                while !transferred.terminal_is_empty() { transferred.close_step(RetainedCloneGrant { maximum_copy_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX, ..exact }).unwrap(); }
            }
            "blocked" => {
                assert!(result.unwrap().is_none());
                assert_eq!(owner.retained_messages().as_ptr(), pointer);
                assert_eq!(owner.retained_messages().len(), law["cases"][0]["expectedRows"].as_u64().unwrap() as usize);
            }
            "refused" => assert!(result.is_err()),
            _ => unreachable!(),
        }
        while !owner.terminal_is_empty() { owner.close_step(RetainedCloneGrant { maximum_copy_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX, ..exact }).unwrap(); }
    }
}
#[test]
fn message_ledger_retirement_releases_actual_capacity_under_separate_axes() {
    let law=fixture();
    for row in law["cases"].as_array().unwrap() {
        for copy in law["copyGrants"].as_array().unwrap() {
            for release in law["releaseGrants"].as_array().unwrap() {
                let(context,messages,held)=owners(row);
                let(mut cursor,heap)=observe_heap_allocations_on_this_thread(||MutationMessageLedgerRetirement::new(context,messages));
                assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                let mut returned=0;
                for turn in 0..1000 {
                    let ((copied,freed,depth),heap)=observe_heap_allocations_on_this_thread(||(cursor.next_copy_byte_demand(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap()));
                    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    let exact=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:(copy.as_u64().unwrap() as usize).max(copied),maximum_capacity_bytes:0,maximum_release_bytes:(release.as_u64().unwrap() as usize).max(freed),maximum_depth:depth};
                    for grant in [RetainedCloneGrant{maximum_items:0,..exact},RetainedCloneGrant{maximum_copy_bytes:copied.saturating_sub(1),..exact},RetainedCloneGrant{maximum_release_bytes:freed.saturating_sub(1),..exact},RetainedCloneGrant{maximum_depth:depth.saturating_sub(1),..exact}] {
                        if cursor.terminal_is_empty(){break;}
                        if grant.maximum_items!=0 && grant.maximum_depth>=depth && grant.maximum_copy_bytes>=copied && grant.maximum_release_bytes>=freed {continue;}
                        let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(grant).unwrap());
                        assert_eq!(step.progress(),RetainedCloneProgress::default());
                        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    }
                    let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(exact).unwrap());let progress=step.progress();
                    assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,progress.released_bytes,"{} turn{turn}",row["name"]);assert!(progress.fits(exact));
                    returned+=progress.released_bytes;
                    if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;}
                    assert!(turn<999);
                }
                assert_eq!(returned,held,"{}",row["name"]);
                let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
        }
    }
}

#[test]
fn message_ledger_retirement_nested_edits_release_original_scaffolds_under_separate_axes() {
    let law=fixture();
    for row in law["editLedgers"].as_array().unwrap() {
        for copy in law["copyGrants"].as_array().unwrap() {
            for release in law["releaseGrants"].as_array().unwrap() {
                let mut entries=Vec::with_capacity(row["capacity"].as_u64().unwrap() as usize);
                let mut held=entries.capacity()*size_of::<semio_framework_replication_conflict::EditMessages>();
                for index in row["rows"].as_array().unwrap() {
                    let(context,messages,bytes)=owners(&law["cases"][index.as_u64().unwrap() as usize]);
                    held+=bytes;entries.push(semio_framework_replication_conflict::EditMessages{edit_id:context,messages});
                }
                let original=entries.as_ptr();
                let(mut cursor,heap)=observe_heap_allocations_on_this_thread(||EditMessageLedgerRetirement::new(entries));
                assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                let mut returned=0;
                for turn in 0..4000 {
                    let ((copied,freed,depth),heap)=observe_heap_allocations_on_this_thread(||(cursor.next_copy_byte_demand().unwrap(),cursor.next_release_byte_demand().unwrap(),cursor.next_depth_demand().unwrap()));
                    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    if !cursor.entries.is_empty(){assert_eq!(cursor.entries.as_ptr(),original);}
                    let exact=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:(copy.as_u64().unwrap() as usize).max(copied),maximum_capacity_bytes:0,maximum_release_bytes:(release.as_u64().unwrap() as usize).max(freed),maximum_depth:depth};
                    for grant in [RetainedCloneGrant{maximum_items:0,..exact},RetainedCloneGrant{maximum_copy_bytes:copied.saturating_sub(1),..exact},RetainedCloneGrant{maximum_release_bytes:freed.saturating_sub(1),..exact},RetainedCloneGrant{maximum_depth:depth.saturating_sub(1),..exact}] {
                        if cursor.terminal_is_empty(){break;}
                        if grant.maximum_items!=0 && grant.maximum_depth>=depth && grant.maximum_copy_bytes>=copied && grant.maximum_release_bytes>=freed {continue;}
                        let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(grant).unwrap());
                        assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    }
                    let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(exact).unwrap());let progress=step.progress();
                    assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,progress.released_bytes,"{} turn{turn}",row["name"]);assert!(progress.fits(exact));
                    returned+=progress.released_bytes;
                    if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;}
                    assert!(turn<3999);
                }
                assert_eq!(returned,held,"{}",row["name"]);
                let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
        }
    }
}

#[test]
fn message_ledger_retirement_replay_outcome_releases_both_original_identities() {
    let law=fixture();
    for row in law["cases"].as_array().unwrap() {
        let(context,messages,bytes)=owners(row);
        let mut identity=String::with_capacity(law["outcomeIdentity"]["capacity"].as_u64().unwrap() as usize);identity.push_str(law["outcomeIdentity"]["text"].as_str().unwrap());
        let held=bytes+identity.capacity();
        let outcome=semio_framework_replication_conflict::MutationReplayOutcome{mutation_id:semio_framework_replication_ids::MutationId(identity),edit_id:context,op_index:7,worst:Some(Severity::Fatal),messages,superseded:true,withdrawn:false};
        let(mut cursor,heap)=observe_heap_allocations_on_this_thread(||MutationMessageLedgerRetirement::from_replay_outcome(outcome));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let mut returned=0;
        for turn in 0..1000 {
            let copied=cursor.next_copy_byte_demand();let freed=cursor.next_release_byte_demand().unwrap();let depth=cursor.next_depth_demand().unwrap();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copied,maximum_capacity_bytes:0,maximum_release_bytes:freed,maximum_depth:depth};
            if freed>0 { let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(RetainedCloneGrant{maximum_release_bytes:freed-1,..grant}).unwrap()); assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0)); }
            let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.close_step(grant).unwrap());assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,step.progress().released_bytes);assert!(step.progress().fits(grant));returned+=heap.released_bytes;
            if matches!(step,RetainedCloneStep::Complete(_)){assert!(cursor.terminal_is_empty());break;}
            assert!(turn<999);
        }
        assert_eq!(returned,held);
        let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
}
