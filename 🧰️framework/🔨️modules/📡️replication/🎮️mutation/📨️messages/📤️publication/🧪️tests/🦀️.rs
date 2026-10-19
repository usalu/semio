use super::*;
use semio_framework_diagnostic::Severity;
use semio_framework_trace::observe_heap_allocations_on_this_thread;

#[test]
fn message_borrowed_publication_copies_exact_committed_sources_and_closes_partial_ownership() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retire/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["borrowCases"].as_array().unwrap() {
        let source: Vec<_> = fixture["cases"][case["sourceCase"].as_u64().unwrap() as usize]["messages"].as_array().unwrap().iter().map(|row| MutationMessage { level: match row["level"].as_str().unwrap() { "warning" => Severity::Warning, "error" => Severity::Error, "fatal" => Severity::Fatal, _ => Severity::Info }, code: row["code"].as_str().unwrap().to_owned().into(), message: row["text"].as_str().unwrap().to_owned(), target: row["target"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect(), op_index: Some(0) }).collect();
        let originals: Vec<_> = source.iter().map(|row| row.message.as_ptr()).collect();
        let mut owner = MutationMessageBorrowedPublication::default();
        for turn in 0..100000 {
            if case["cancelAt"].as_u64() == Some(turn) { break; }
            if owner.ready() { break; }
            let demand = owner.demands(&source).unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            for axis in 0..5 {
                let denied = match axis { 0 => RetainedCloneGrant { maximum_items: 0, ..grant }, 1 if demand.copy_bytes != 0 => RetainedCloneGrant { maximum_copy_bytes: 0, ..grant }, 2 if demand.capacity_bytes != 0 => RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant }, 3 if demand.release_bytes != 0 => RetainedCloneGrant { maximum_release_bytes: 0, ..grant }, 4 => RetainedCloneGrant { maximum_depth: 0, ..grant }, _ => continue };
                let (step, heap) = observe_heap_allocations_on_this_thread(|| owner.advance(&source, denied).unwrap()); assert_eq!(step.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (step, heap) = observe_heap_allocations_on_this_thread(|| owner.advance(&source, grant).unwrap()); assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            let substitute = source.clone(); let (error, heap) = observe_heap_allocations_on_this_thread(|| owner.demands(&substitute)); assert!(error.is_err()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        }
        if case["cancelAt"].is_null() {
            assert!(owner.ready()); let (mut publication, receipt) = owner.take(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<MutationMessagePublication>(), maximum_depth: 1, ..Default::default() }).unwrap(); assert_eq!(receipt.copied_bytes, size_of::<MutationMessagePublication>());
            assert_eq!(publication.original_messages(), source); assert_eq!(publication.copied_messages(), source);
            for (original, copy) in source.iter().zip(publication.original_messages()) { if !original.message.is_empty() { assert_ne!(original.message.as_ptr(), copy.message.as_ptr()); } if !original.code.0.is_empty() { assert_ne!(original.code.0.as_ptr(), copy.code.0.as_ptr()); } }
            publication.begin_close(); while !publication.terminal_is_empty() { let demand = publication.close_demands().unwrap(); publication.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }).unwrap(); }
        }
        owner.begin_close();
        for _ in 0..100000 { if owner.terminal_is_empty() { break; } let demand = owner.close_demands().unwrap(); let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }; let (step, heap) = observe_heap_allocations_on_this_thread(|| owner.close_step(grant).unwrap()); assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes)); }
        assert!(owner.terminal_is_empty()); assert_eq!(source.iter().map(|row| row.message.as_ptr()).collect::<Vec<_>>(), originals);
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(owner)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] Diagnostic committed-prefix case={} originalRows={} terminalDrop=0", case["id"], source.len());
    }
}

/// 🪢️ Joined batch owners move every original string exactly once and close at all tested frontiers.
#[test]
fn message_publication_join_preserves_payload_allocations_and_funded_cancellation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retire/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["joinCases"].as_array().unwrap() {
        let mut batch = MutationMessagePublicationBatch::default();
        let count = case["rows"].as_array().unwrap().len();
        let capacity = count * size_of::<MutationMessagePublication>();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: capacity, maximum_depth: 1, ..Default::default() };
        let (receipt, heap) = observe_heap_allocations_on_this_thread(|| batch.reserve(count, grant).unwrap());
        assert_eq!(heap.requested_bytes, receipt.retained_capacity_bytes); assert!(receipt.fits(grant));
        let mut pointers = Vec::new();
        for index in case["rows"].as_array().unwrap() {
            let rows: Vec<_> = fixture["cases"][index.as_u64().unwrap() as usize]["messages"].as_array().unwrap().iter().map(|row| MutationMessage { level: match row["level"].as_str().unwrap() { "warning" => Severity::Warning, "error" => Severity::Error, "fatal" => Severity::Fatal, _ => Severity::Info }, code: row["code"].as_str().unwrap().to_owned().into(), message: row["text"].as_str().unwrap().to_owned(), target: row["target"].as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_owned()).collect(), op_index: Some(0) }).collect();
            pointers.extend(rows.iter().map(|row| row.message.as_ptr()));
            let mut owner = MutationMessagePublication::new(rows);
            while !owner.ready() { let demand = owner.demands().unwrap(); owner.advance(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }).unwrap(); }
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<MutationMessagePublication>(), maximum_depth: 1, ..Default::default() };
            match batch.append(owner, grant) { Ok(receipt) => assert!(receipt.fits(grant)), Err(mut owner) => { owner.begin_close(); while !owner.terminal_is_empty() { let demand = owner.close_demands().unwrap(); owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }).unwrap(); } panic!("exact batch owner slot unexpectedly refused"); } }
        }
        assert_eq!(batch.rows().count(), case["expectedRows"].as_u64().unwrap() as usize);
        let mut join = MutationMessagePublicationJoin::new(batch, pointers.len());
        for turn in 0..100000 {
            if case["cancelAt"].as_u64() == Some(turn) { join.begin_close(); break; }
            if join.ready() { break; }
            let demand = join.demands().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (blocked, heap) = observe_heap_allocations_on_this_thread(|| join.advance(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
            assert_eq!(blocked.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (step, heap) = observe_heap_allocations_on_this_thread(|| join.advance(grant).unwrap());
            assert!(step.progress().fits(grant)); assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes); assert_eq!(heap.released_bytes, step.progress().released_bytes);
        }
        if case["cancelAt"].is_null() {
            assert!(join.ready());
            let (mut owner, receipt) = join.take(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<MutationMessagePublication>(), maximum_depth: 1, ..Default::default() }).unwrap().unwrap();
            assert_eq!(receipt.copied_bytes, size_of::<MutationMessagePublication>());
            assert_eq!(owner.original_messages().iter().map(|row| row.message.as_ptr()).collect::<Vec<_>>(), pointers);
            assert_eq!(owner.original_messages(), owner.copied_messages());
            owner.begin_close(); while !owner.terminal_is_empty() { let demand = owner.close_demands().unwrap(); owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }).unwrap(); }
        }
        join.begin_close();
        for _ in 0..100000 {
            if join.terminal_is_empty() { break; }
            let demand = join.close_demands().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (step, heap) = observe_heap_allocations_on_this_thread(|| join.close_step(grant).unwrap());
            assert!(step.progress().fits(grant)); assert_eq!(heap.requested_bytes, 0); assert_eq!(heap.released_bytes, step.progress().released_bytes);
        }
        assert!(join.terminal_is_empty());
    }
}

/// 🪪️ Batch and transaction rows retain exact indices under denied and admitted metadata grants.
#[test]
fn message_publication_stamps_actual_operation_offsets_without_payload_copy() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retire/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["stampCases"].as_array().unwrap() {
        let rows = case["indices"].as_array().unwrap().iter().map(|index| MutationMessage { level: Severity::Warning, code: "mutation.notice".into(), message: "Owned warning ä🐚".into(), target: vec!["field".into()], op_index: Some(u32::try_from(index.as_u64().unwrap()).unwrap()) }).collect();
        let mut owner = MutationMessagePublication::new(rows);
        while !owner.ready() {
            let demand = owner.demands().unwrap();
            owner.advance(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }).unwrap();
        }
        for (index, expected) in case["expected"].as_array().unwrap().iter().enumerate() {
            let pointer = owner.original_messages()[index].message.as_ptr();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 2 * size_of::<Option<u32>>(), maximum_depth: 1, ..Default::default() };
            let offset = case["offset"].as_u64().unwrap() as usize;
            let (denied, heap) = observe_heap_allocations_on_this_thread(|| owner.stamp_row(index, offset, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - 1, ..grant }).unwrap());
            assert_eq!(denied, RetainedCloneProgress::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (receipt, heap) = observe_heap_allocations_on_this_thread(|| owner.stamp_row(index, offset, grant).unwrap());
            assert!(receipt.fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(owner.original_messages()[index].op_index, Some(u32::try_from(expected.as_u64().unwrap()).unwrap()));
            assert_eq!(owner.copied_messages()[index].op_index, owner.original_messages()[index].op_index);
            assert_eq!(owner.original_messages()[index].message.as_ptr(), pointer);
        }
        owner.begin_close();
        while !owner.terminal_is_empty() { let demand = owner.close_demands().unwrap(); owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }).unwrap(); }
    }
}

/// 🪡️ Actual native copy receipts match independent physical allocation and terminal cancellation.
#[test]
fn message_publication_copies_original_rows_under_independent_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../♻️retire/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["publicationCases"].as_array().unwrap() {
        let rows: Vec<_> = case["rows"].as_array().unwrap().iter().flat_map(|index| fixture["cases"][index.as_u64().unwrap() as usize]["messages"].as_array().unwrap()).map(|row| MutationMessage { level: match row["level"].as_str().unwrap() { "warning" => Severity::Warning, "error" => Severity::Error, "fatal" => Severity::Fatal, _ => Severity::Info }, code: row["code"].as_str().unwrap().to_owned().into(), message: row["text"].as_str().unwrap().to_owned(), target: row["target"].as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_owned()).collect(), op_index: Some(0) }).collect();
        let original_pointer = rows.as_ptr();
        let expected = rows.clone();
        let mut cursor = MutationMessagePublication::new(rows);
        for turn in 0..100000 {
            if case["cancelAt"].as_u64() == Some(turn) { cursor.begin_close(); break; }
            if cursor.ready() { break; }
            let demand = cursor.demands().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes.saturating_sub(1), ..grant }, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes.saturating_sub(1), ..grant }] {
                if denied.maximum_items != 0 && denied == grant { continue; }
                let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.advance(denied).unwrap());
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.advance(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes);
            assert_eq!(heap.released_bytes, step.progress().released_bytes);
        }
        if case["cancelAt"].is_null() {
            assert!(cursor.ready());
            assert_eq!(cursor.copied_messages().len(), case["expectedCopies"].as_u64().unwrap() as usize);
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 2 * size_of::<Vec<MutationMessage>>(), maximum_depth: 1, ..Default::default() };
            let (original, report, receipt) = cursor.take(grant).unwrap().unwrap();
            assert_eq!(original.as_ptr(), original_pointer);
            assert_eq!(original, expected); assert_eq!(report, expected); assert_ne!(original.as_ptr(), report.as_ptr()); assert!(receipt.fits(grant));
            for messages in [original, report] {
                let mut owner = MutationMessageLedgerRetirement::new(String::new(), messages);
                while !owner.terminal_is_empty() { owner.close_step(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX, ..Default::default() }).unwrap(); }
            }
        }
        cursor.begin_close();
        assert!(cursor.take(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: usize::MAX, maximum_depth: usize::MAX, ..Default::default() }).is_err());
        for _ in 0..100000 {
            if cursor.terminal_is_empty() { break; }
            let demand = cursor.close_demands().unwrap();
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (step, heap) = observe_heap_allocations_on_this_thread(|| cursor.close_step(grant).unwrap());
            assert!(step.progress().fits(grant)); assert_eq!(heap.requested_bytes, 0); assert_eq!(heap.released_bytes, step.progress().released_bytes);
        }
        assert!(cursor.terminal_is_empty());
    }
}

#[test]
fn message_publication_final_selection_preserves_neutral_policy_and_partial_custody() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../✂️clamp/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for case in law["cases"].as_array().unwrap(){
        let make=||case["rows"].as_array().unwrap().iter().enumerate().map(|(index,row)|MutationMessage{level:[Severity::Info,Severity::Warning,Severity::Error,Severity::Fatal][row["level"].as_u64().unwrap()as usize],code:"mutation.invariant".to_string().into(),message:row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize),target:row["targets"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect(),op_index:Some(index as u32)}).collect::<Vec<_>>();
        for cancel in std::iter::once(None).chain(law["cancelAt"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize))){
            let original=make();let pointers=original.iter().map(|row|row.message.as_ptr()).collect::<Vec<_>>();let mut owner=MutationMessagePublication::from_copied_rows(original,make());let bytes=case["editId"].as_str().unwrap().len();
            for turn in 0..100000{
                if cancel==Some(turn)||owner.selection_ready(bytes){break;}let demand=owner.selection_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
                for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant}]{if denied==grant{continue;}let(step,heap)=observe_heap_allocations_on_this_thread(||owner.select(bytes,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
                let(step,heap)=observe_heap_allocations_on_this_thread(||owner.select(bytes,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999);
            }
            if owner.selection_ready(bytes){let originals=owner.original_messages();assert_eq!(originals,owner.copied_messages());let kept=originals.iter().filter(|row|row.code.0!="mutation.cascade").map(|row|row.op_index.unwrap()as usize).collect::<Vec<_>>();assert_eq!(serde_json::to_value(kept).unwrap(),case["expectedKept"]);for(original,report)in originals.iter().zip(owner.copied_messages()){if !original.message.is_empty(){assert_ne!(original.message.as_ptr(),report.message.as_ptr());}if case["dropped"].as_u64()==Some(0){assert_eq!(original.message.as_ptr(),pointers[original.op_index.unwrap()as usize]);}}
                let extent=originals.iter().fold(bytes,|extent,row|extent+crate::MUTATION_MESSAGE_OWNER_BYTES+row.code.0.len()+row.message.len()+row.target.iter().map(|target|crate::MUTATION_MESSAGE_TARGET_BYTES+target.len()).sum::<usize>());assert!(extent<=crate::MUTATION_MESSAGE_ENTRY_BYTES);
            }else{assert!(cancel.is_some());}
            owner.begin_close();for turn in 0..100000{if owner.terminal_is_empty(){break;}let demand=owner.close_demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999);}assert!(owner.terminal_is_empty());let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        }
        println!("[DEBUG] shared publication final selection case={} actualIdentityBytes={} independentReport=true allCancellationFrontiers=true",case["name"],case["editId"].as_str().unwrap().len());
    }
}
