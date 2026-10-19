use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread;

#[test]
fn store_diagnostic_stage_preserves_originals_and_independent_report_through_funded_close() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let policy = &corpus["grant"];
    let axis = |name: &str| policy[name].as_u64().unwrap() as usize;
    let grant = RetainedCloneGrant { maximum_items: axis("items"), maximum_copy_bytes: axis("copy"), maximum_capacity_bytes: axis("capacity"), maximum_release_bytes: axis("release"), maximum_depth: axis("depth") };
    for case in corpus["cases"].as_array().unwrap() {
        let rows: Vec<_> = case["rows"].as_array().unwrap().iter().map(|row| semio_framework_os_spr::MutationMessage { level: semio_framework_diagnostic::Severity::from_value(row["level"].clone().into()).unwrap(), code: row["code"].as_str().unwrap().to_owned().into(), message: row["message"].as_str().unwrap().to_owned(), target: row["target"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().to_owned()).collect(), op_index: Some(row["opIndex"].as_u64().unwrap() as u32) }).collect();
        let originals: Vec<_> = rows.iter().map(|row| row.message.as_ptr()).collect();
        let mut publication = MutationMessagePublication::new(rows);
        while !publication.ready() { publication.advance(grant).unwrap(); }
        let mut batch = MutationMessagePublicationBatch::default();
        batch.reserve(1, grant).unwrap();
        match batch.append(publication, grant) { Ok(_) => {}, Err(_) => panic!("fixture batch reservation must retain its original") }
        let mut stage = ArtifactStoreDiagnosticStage::default(); stage.batch = Some(batch); stage.rows = originals.len();
        let mut turns = 0;
        while !stage.prepared() {
            if case["cancelAt"].as_u64().is_some_and(|phase| stage.phase == phase as u8) { break; }
            let (blocked, heap) = observe_heap_allocations_on_this_thread(|| stage.advance(case["editId"].as_str().unwrap(), RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
            assert_eq!(blocked.progress(), Default::default()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (step, heap) = observe_heap_allocations_on_this_thread(|| stage.advance(case["editId"].as_str().unwrap(), grant).unwrap());
            assert!(step.progress().fits(grant)); assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes); assert_eq!(heap.released_bytes, step.progress().released_bytes);
            turns += 1; assert!(turns < 10000, "unchanged diagnostic authority must progress");
        }
        if case["cancelAt"].is_null() {
            assert!(stage.prepared()); assert_eq!(stage.original_messages().iter().map(|row| row.message.as_ptr()).collect::<Vec<_>>(), originals);
            let (durable, report, worst, receipt) = stage.take(grant).unwrap().unwrap(); assert!(receipt.fits(grant));
            let durable=durable.into_entry();
            assert_eq!(durable.edit_id, case["editId"].as_str().unwrap()); assert_eq!(report.len(), 1); assert_eq!(durable, report[0]);
            assert_eq!(worst.map(|level| serde_json::Value::from(level.to_value())).unwrap_or(serde_json::Value::Null), case["worst"]);
            assert_eq!(durable.messages.iter().map(|row| row.message.as_ptr()).collect::<Vec<_>>(), originals);
            for (original, copy) in durable.messages.iter().zip(report[0].messages.iter()) { assert_ne!(original.message.as_ptr(), copy.message.as_ptr()); }
            let mut owner = semio_framework_os_spr::command::EditMessageLedgerRetirement::new(vec![durable]);
            while !owner.terminal_is_empty() { owner.close_step(grant).unwrap(); }
            let mut owner = semio_framework_os_spr::command::EditMessageLedgerRetirement::new(report);
            while !owner.terminal_is_empty() { owner.close_step(grant).unwrap(); }
        }
        stage.begin_close();
        while !stage.terminal_is_empty() {
            let (step, heap) = observe_heap_allocations_on_this_thread(|| stage.close_step(grant).unwrap());
            assert!(step.progress().fits(grant)); assert_eq!(heap.requested_bytes, step.progress().retained_capacity_bytes); assert_eq!(heap.released_bytes, step.progress().released_bytes);
            turns += 1; assert!(turns < 20000, "diagnostic cancellation must reach terminal ownership");
        }
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(stage)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        println!("[DEBUG] Store diagnostic case={} turns={} terminalDrop=0", case["id"], turns);
    }
}

#[test]
fn store_diagnostic_stage_applies_final_neutral_bound_before_actual_context_publication() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();assert_eq!(corpus["finalSelection"]["entryBound"],4096);
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🔨️modules/📡️replication/🎮️mutation/📨️messages/✂️clamp/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:32768,maximum_release_bytes:32768,maximum_depth:24};
    for case in law["cases"].as_array().unwrap(){
        if !law["publication"]["copyableCases"].as_array().unwrap().contains(&case["name"]){continue;}
        let rows=case["rows"].as_array().unwrap().iter().enumerate().map(|(index,row)|semio_framework_os_spr::MutationMessage{level:[semio_framework_diagnostic::Severity::Info,semio_framework_diagnostic::Severity::Warning,semio_framework_diagnostic::Severity::Error,semio_framework_diagnostic::Severity::Fatal][row["level"].as_u64().unwrap()as usize],code:"mutation.invariant".to_string().into(),message:row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap()as usize),target:row["targets"].as_array().unwrap().iter().map(|target|target.as_str().unwrap().to_owned()).collect(),op_index:Some(index as u32)}).collect();
        let mut publication=MutationMessagePublication::new(rows);while !publication.ready(){publication.advance(grant).unwrap();}let mut stage=ArtifactStoreDiagnosticStage::from_publication(Some(publication));let identity=case["editId"].as_str().unwrap();
        for turn in 0..100000{if stage.prepared(){break;}let(step,heap)=observe_heap_allocations_on_this_thread(||stage.advance(identity,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999);}
        assert!(stage.prepared());let(record,report,worst,receipt)=stage.take(grant).unwrap().unwrap();assert!(receipt.fits(grant));assert!(record.bytes()<=4096);assert_eq!(record.bytes(),record.entry().messages.iter().fold(identity.len(),|bytes,row|bytes+ARTIFACT_EDIT_MESSAGE_OWNER_BYTES+row.code.0.len()+row.message.len()+row.target.iter().map(|target|ARTIFACT_EDIT_MESSAGE_TARGET_BYTES+target.len()).sum::<usize>()));
        let durable=record.into_entry();assert_eq!(durable.edit_id,identity);assert_eq!(report.len(),1);assert_eq!(durable,report[0]);assert_eq!(serde_json::to_value(durable.messages.iter().filter(|row|row.code.0!="mutation.cascade").map(|row|row.op_index.unwrap()as usize).collect::<Vec<_>>()).unwrap(),case["expectedKept"]);assert_eq!(worst,durable.messages.iter().map(|row|row.level).max());
        for(original,copy)in durable.messages.iter().zip(&report[0].messages){if !original.message.is_empty(){assert_ne!(original.message.as_ptr(),copy.message.as_ptr());}}
        let mut original=semio_framework_os_spr::command::EditMessageLedgerRetirement::new(vec![durable]);while !original.terminal_is_empty(){original.close_step(grant).unwrap();}let mut report=semio_framework_os_spr::command::EditMessageLedgerRetirement::new(report);while !report.terminal_is_empty(){report.close_step(grant).unwrap();}stage.begin_close();while !stage.terminal_is_empty(){stage.close_step(grant).unwrap();}let(_,heap)=observe_heap_allocations_on_this_thread(||drop(stage));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        println!("[DEBUG] Store final diagnostic selection case={} actualEdit={} measuredBound=4096 independentReport=true terminalDrop=0",case["name"],identity);
    }
}
