fn close_fold_and_check_original_grants(job:&mut HistoryFoldJob<'_,()>){
    let completed=job.completed();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.request_cancel());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for _ in 0..100000{if job.terminal_is_empty(){break;}let grant=job.next_step_grant(1,7).unwrap();let demand=(job.next_capacity_byte_demand(7).unwrap(),job.next_release_byte_demand().unwrap(),job.next_depth_demand().unwrap());
        let mut refusals=vec![RetainedCloneGrant{maximum_items:0,..grant}];if demand.0>0{refusals.push(RetainedCloneGrant{maximum_capacity_bytes:0,..grant});refusals.push(RetainedCloneGrant{maximum_capacity_bytes:demand.0-1,..grant});}if demand.1>0{refusals.push(RetainedCloneGrant{maximum_release_bytes:0,..grant});refusals.push(RetainedCloneGrant{maximum_release_bytes:demand.1-1,..grant});}if demand.2>0{refusals.push(RetainedCloneGrant{maximum_depth:0,..grant});}
        for denied in refusals{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(denied));if let Ok(step)=step{assert_eq!(step.progress(),RetainedCloneProgress::default());}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((job.next_capacity_byte_demand(7).unwrap(),job.next_release_byte_demand().unwrap(),job.next_depth_demand().unwrap()),demand);}
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap.requested_bytes,progress.retained_capacity_bytes);assert_eq!(heap.released_bytes,progress.released_bytes);assert_eq!(job.completed(),completed);
    }assert!(job.terminal_is_empty());
}

#[test]
fn bounded_history_fold_physical_retirement_preserves_whole_queued_owners_and_terminal_frames(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();assert_eq!(law["independentCredits"].as_array().unwrap().len(),5);
    for extent in law["extents"].as_array().unwrap(){let extent=extent.as_u64().unwrap()as usize;let mut job=HistoryFoldJob::<()>::new(|_|std::future::pending());let mut original=Vec::with_capacity(extent);if extent!=0{original.push(37u8);}assert_eq!(original.capacity(),extent);assert_eq!(original.len(),extent.min(law["logicalItems"].as_u64().unwrap()as usize));let birth=owned_retirement_birth_bytes::<Vec<u8>>();job.control.0.retirement_capacity.store(birth,Ordering::Relaxed);job.control.0.retirement_items.store(1,Ordering::Relaxed);job.control.retire(original);let expected=job.control.demands(7).unwrap();assert_eq!(job.next_capacity_byte_demand(7).unwrap(),expected.capacity_bytes);assert_eq!(job.next_release_byte_demand().unwrap(),expected.release_bytes);close_fold_and_check_original_grants(&mut job);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] fold extent={extent} independent capacity/release/depth refusals preserve original queued owners; trace equals receipts; terminalDrop0heap");}
}

#[test]
fn bounded_history_fold_cancellation_preserves_tracked_future_and_error_backings(){
    for extent in [1usize,8194,65536,262144]{let original=String::from_utf8(vec![37u8;extent]).unwrap();let pointer=original.as_ptr()as usize;let mut job=HistoryFoldJob::new(move|control|async move{let original=control.track(original).unwrap_or_else(|_| unreachable!()).await;assert_eq!(original.as_ptr()as usize,pointer);std::future::pending::<()>().await;drop(original);Ok(())});assert!(matches!(job.step(job.next_step_grant(1,1).unwrap(),&mut||false).unwrap(),HistoryFoldJobStep::Pending{released_bytes:0,..}));close_fold_and_check_original_grants(&mut job);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let detail=String::from_utf8(vec![37u8;extent]).unwrap();let mut job=HistoryFoldJob::<()>::new(move|_|std::future::ready(Err(crate::ProtocolError::Io(detail))));assert!(matches!(job.step(job.next_step_grant(1,1).unwrap(),&mut||false).unwrap(),HistoryFoldJobStep::Pending{released_bytes:0,..}));close_fold_and_check_original_grants(&mut job);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] tracked fold future/error extent={extent} preserve originals on independent undergrants; birth/release trace agrees; terminalDrop0heap");}
}

#[test]
fn bounded_history_fold_new_local_admission_retains_original_across_same_poll_refusal(){
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for extent in law["extents"].as_array().unwrap() {
        let extent = extent.as_u64().unwrap() as usize;
        let mut original = Vec::with_capacity(extent);
        if extent != 0 { original.push(37u8); }
        let pointer = original.as_ptr() as usize;
        let witness = AtomicUsize::new(0);
        let witness_ref = &witness;
        let mut job = HistoryFoldJob::new(move |control| async move {
            let original = control.track(original).unwrap_or_else(|_| unreachable!()).await;
            witness_ref.store(original.as_ptr() as usize, Ordering::Relaxed);
            drop(original);
            Ok(())
        });
        let grant = job.next_step_grant(1, 7).unwrap();
        assert_eq!(grant.maximum_capacity_bytes, 0);
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.step(grant, &mut || false).unwrap());
        assert!(matches!(step, HistoryFoldJobStep::Pending { .. }));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(witness.load(Ordering::Relaxed), 0);
        let birth = owned_retirement_birth_bytes::<Vec<u8>>();
        assert_eq!(job.next_capacity_byte_demand(7).unwrap(), birth);
        let grant = job.next_step_grant(1, 7).unwrap();
        for capacity in [0, birth - 1] {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.step(RetainedCloneGrant { maximum_capacity_bytes: capacity, ..grant }, &mut || false).unwrap());
            match step { HistoryFoldJobStep::Pending { progress, .. } => assert_eq!(progress, RetainedCloneProgress::default()), _ => panic!("unfunded admission transferred its original") }
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(witness.load(Ordering::Relaxed), 0);
            assert_eq!(job.next_capacity_byte_demand(7).unwrap(), birth);
        }
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.step(grant, &mut || false).unwrap());
        let HistoryFoldJobStep::Pending { progress, .. } = step else { panic!("owner retirement precedes handoff") };
        assert_eq!(witness.load(Ordering::Relaxed), pointer);
        assert_eq!(progress.retained_capacity_bytes, birth);
        assert_eq!(heap.requested_bytes, birth);
        assert_eq!(heap.released_bytes, 0);
        close_fold_and_check_original_grants(&mut job);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(job));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] new fold local extent={extent} waits for independent frame capacity; original pointer retained; same-poll drop and terminal trace equal genuine receipts");
    }
}

#[test]
fn bounded_history_fold_unsupported_local_refusal_returns_original_before_registration(){
    struct Unsupported(String);
    impl RetireOwned for Unsupported { fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> { panic!("unsupported original must never construct retirement") } }
    let control = HistoryFoldControl::new();
    let original = Unsupported(String::from("original owner"));
    let pointer = original.0.as_ptr();
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| control.track(original));
    let (error, original) = match result { Err(refusal) => refusal, Ok(_) => panic!("unsupported owner was admitted") };
    assert_eq!(error.kind, ValueRefusalKind::UnsupportedOwner);
    assert_eq!(original.0.as_ptr(), pointer);
    assert_eq!(control.0.pending_retirement_birth.load(Ordering::Relaxed), 0);
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    println!("[DEBUG] unsupported fold local returns original pointer with zero birth, release, or registered demand");
}

#[test]
fn bounded_history_fold_cancellation_keeps_multiple_preborn_locals_and_waiting_original(){
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let extents: Vec<usize> = law["mixedLocalExtents"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize).collect();
    for admitted in law["prebornLocalCounts"].as_array().unwrap() {
        let admitted = admitted.as_u64().unwrap() as usize;
        let (first_extent, second_extent, third_extent) = (extents[0], extents[1], extents[2]);
        let mut job = HistoryFoldJob::new(move |control| async move {
            let first = control.track(String::from_utf8(vec![37u8; first_extent]).unwrap()).unwrap_or_else(|_| unreachable!()).await;
            let second = control.track(vec![41u8; second_extent]).unwrap_or_else(|_| unreachable!()).await;
            let third = control.track(Some(String::from_utf8(vec![43u8; third_extent]).unwrap())).unwrap_or_else(|_| unreachable!()).await;
            std::future::pending::<()>().await;
            drop((first, second, third));
            Ok(())
        });
        for _ in 0..=admitted { let _ = job.step(job.next_step_grant(1, 7).unwrap(), &mut || false).unwrap(); }
        close_fold_and_check_original_grants(&mut job);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(job));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] fold cancellation after {admitted} preborn locals retains mixed originals; queued frames transfer without allocation; waiting original birth, future release, nested backing release, and terminal drop obey independent receipts");
    }
}

#[test]
fn bounded_history_manual_protocol_variants_declare_exact_native_birth_and_release(){
    fn check<T:RetireOwned>(value:T){
        let mut job=HistoryFoldJob::<()>::new(|_|std::future::pending());
        let grant=RetainedCloneGrant::one_capacity_turn(owned_retirement_birth_bytes::<T>(),1);
        let (admitted,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_owned_retirement(value,grant));
        let(owner,progress)=match admitted{Ok(admitted)=>admitted,Err((error,_))=>panic!("manual protocol owner refused {error}")};
        assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,0));
        job.control.0.retirements.lock().push_back(owner);
        close_fold_and_check_original_grants(&mut job);
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for variant in law["manualProtocolVariants"].as_array().unwrap(){match variant.as_str().unwrap(){
        "input"=>check(crate::InputReplacement::Input{schema:"owned-schema".into(),payload:vec![1u8,2,3]}),
        "withdrawn"=>check(crate::InputReplacement::Withdrawn),
        "owner"=>check(crate::MutationOrigin::Owner),
        "contributed"=>check(crate::MutationOrigin::Contributed{plugin_id:"owned-plugin".into(),mutation_id:crate::SchemaId("owned-mutation".into()),payload_hash:crate::PayloadHash([7;32])}),
        "transaction"=>check(crate::MutationOrigin::Transaction{initiator:crate::ForeignTarget{artifact_id:"owned-artifact".into(),artifact_kind:"owned-kind".into(),dialect:Some("owned-dialect".into())}}),
        "quarantined"=>check(crate::ConflictKind::Quarantined{envelopes:Vec::new()}),
        "degraded"=>check(crate::ConflictKind::Degraded{edit_ids:vec!["owned-edit".into()]}),
        "set"=>check(crate::MapEntryOperation::Set("owned-value".to_owned())),
        "remove"=>check(crate::MapEntryOperation::<String>::Remove),
        "reject"=>check(crate::MapEntryOperation::<String>::Reject),
        _=>panic!("unknown neutral protocol variant"),
    }println!("[DEBUG] manual protocol variant={} actual native frame/scaffold/backing birth and release equal genuine receipts; every independent undergrant preserves original custody",variant.as_str().unwrap());}
}

#[test]
fn bounded_history_multiple_waiting_originals_refuse_independent_item_undergrant(){
    let first=String::from_utf8(vec![37u8;8194]).unwrap();let second=vec![41u8;65536];
    let mut job=HistoryFoldJob::<()>::new(move|control|{let first=control.track(first).unwrap_or_else(|_|unreachable!());let second=control.track(second).unwrap_or_else(|_|unreachable!());async move{let first=first.await;let second=second.await;drop((first,second));Ok(())}});
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let expected=law["waitingOriginalItemDemand"].as_u64().unwrap() as usize;assert_eq!(job.next_local_admission_item_demand(),expected);
    job.request_cancel();let grant=job.next_step_grant(expected,7).unwrap();
    for items in 0..expected{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(RetainedCloneGrant{maximum_items:items,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(job.next_local_admission_item_demand(),expected);}
    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_step(grant).unwrap());let progress=step.progress();assert_eq!(progress.copied_items,expected);assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(job.next_local_admission_item_demand(),0);close_fold_and_check_original_grants(&mut job);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(job));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] two waiting fold originals require exactly two real frame items; zero/subexact item grants preserve original custody and heap; funded atomic future release reports both actual births");
}
