//! ♻️ Physical message ledger capacity releases are measured independently from payload work.
use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread;
use semio_framework_diagnostic::Severity;
fn fixture()->serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn owners(row:&serde_json::Value)->(String,Vec<MutationMessage>,usize) {
    let mut context=String::with_capacity(row["contextCapacity"].as_u64().unwrap() as usize);context.push_str(row["context"].as_str().unwrap());
    let mut messages=Vec::with_capacity(row["rowCapacity"].as_u64().unwrap() as usize);
    for row in row["messages"].as_array().unwrap() {
        messages.push(MutationMessage{level:match row["level"].as_str().unwrap(){"fatal"=>Severity::Fatal,"error"=>Severity::Error,"warning"=>Severity::Warning,_=>Severity::Info},code:row["code"].as_str().unwrap().to_owned().into(),message:row["text"].as_str().unwrap().to_owned(),target:row["target"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect(),op_index:Some(7)});
    }
    let held=context.capacity()+messages.capacity()*size_of::<MutationMessage>()+messages.iter().map(|message|message.code.0.capacity()+message.message.capacity()+message.target.capacity()*size_of::<String>()+message.target.iter().map(String::capacity).sum::<usize>()).sum::<usize>();
    (context,messages,held)
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
    println!("[DEBUG] native message ledger constructor/demands0heap, undergrants0effects, exact physical capacity release and terminal0heap verified");
}

#[test]
fn message_ledger_retirement_nested_edits_release_original_scaffolds_under_separate_axes() {
    let law=fixture();
    for row in law["editLedgers"].as_array().unwrap() {
        for copy in law["copyGrants"].as_array().unwrap() {
            for release in law["releaseGrants"].as_array().unwrap() {
                let mut entries=Vec::with_capacity(row["capacity"].as_u64().unwrap() as usize);
                let mut held=entries.capacity()*size_of::<crate::EditMessages>();
                for index in row["rows"].as_array().unwrap() {
                    let(context,messages,bytes)=owners(&law["cases"][index.as_u64().unwrap() as usize]);
                    held+=bytes;entries.push(crate::EditMessages{edit_id:context,messages});
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
    println!("[DEBUG] native nested edit ledgers move original owners, require parent-plus-child depth, return every vector/string capacity and finish with0heap Drop");
}

#[test]
fn message_ledger_retirement_replay_outcome_releases_both_original_identities() {
    let law=fixture();
    for row in law["cases"].as_array().unwrap() {
        let(context,messages,bytes)=owners(row);
        let mut identity=String::with_capacity(law["outcomeIdentity"]["capacity"].as_u64().unwrap() as usize);identity.push_str(law["outcomeIdentity"]["text"].as_str().unwrap());
        let held=bytes+identity.capacity();
        let outcome=crate::MutationReplayOutcome{mutation_id:crate::ids::MutationId(identity),edit_id:context,op_index:7,worst:Some(Severity::Fatal),messages,superseded:true,withdrawn:false};
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
    println!("[DEBUG] native replay outcome constructor0heap, both identity capacities released under exact grants, original messages retired and terminal Drop0heap");
}
