//! 🧪️ Actual admitted payload pages, unused backing and operation ledger survive every denied turn.
use semio_framework_job::testkit::*;
use semio_framework_job::*;
use std::sync::Arc;
#[path="../../../📬️outcome/📦️builder/🧪️testing/🦀️.rs"]
mod payload_case;
use payload_case::PayloadCase;
use semio_framework_test_fixtures::fixture_text;
fn observe_retirement_allocations<T>(body:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,(heap.requested_bytes,heap.released_bytes))}

fn admitted(pages:usize,stream:JobPayloadStream)->(RetainedJobPayload,Arc<JobPayloadOperationLedger>,PayloadCase){
 let mut case=PayloadCase::new(OperationId(95001),Generation(41));let mut b=case.initialize(stream);
 for index in 0..pages{case.page(&mut b,if index%2==0{b""}else{b"original\0"});}
 let payload=case.finish(&mut b);let(ledger,case)=case.into_ledger();(payload,ledger,case)
}
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn denied_grants(full:RetainedCloneGrant,demand:RetirementDemand)->[Option<RetainedCloneGrant>;4]{[Some(RetainedCloneGrant{maximum_items:0,..full}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..full}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..full}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..full})]}
fn retained_step(outcome:&mut StepOutcome,case:&mut PayloadCase)->(RetainedCloneStep,(usize,usize)){
    let(demand,heap)=observe_retirement_allocations(||outcome.retirement_demands().unwrap());assert_eq!(heap,(0,0));let full=grant(demand);
    let pointer=match outcome{StepOutcome::PreviewReady(p)=>p.page(0).map(|page|page.as_ptr()),StepOutcome::CheckpointReady(p)=>p.state.page(0).map(|page|page.as_ptr()),StepOutcome::Complete(p)=>p.state.page(0).or_else(||p.output.page(0)).map(|page|page.as_ptr()),StepOutcome::Fault(p)=>p.detail.page(0).map(|page|page.as_ptr()),_=>None};
    for under in denied_grants(full,demand).into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||case.step(under,|_|outcome.close_step(under)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let same=match outcome{StepOutcome::PreviewReady(p)=>p.page(0).map(|page|page.as_ptr()),StepOutcome::CheckpointReady(p)=>p.state.page(0).map(|page|page.as_ptr()),StepOutcome::Complete(p)=>p.state.page(0).or_else(||p.output.page(0)).map(|page|page.as_ptr()),StepOutcome::Fault(p)=>p.detail.page(0).map(|page|page.as_ptr()),_=>None};assert_eq!(same,pointer);}
    let(step,heap)=observe_retirement_allocations(||case.step(full,|_|outcome.close_step(full)));assert!(step.progress().fits(full));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);(step,heap)
}
#[test]
fn job_payload_retirement_original_variants_page_and_ledger_release_match_actual_heap(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../♻️retirement/📄️payload/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in law["pages"].as_array().unwrap(){for variant in law["variants"].as_array().unwrap(){for shared in [false,true]{for cut in law["cancellationCuts"].as_array().unwrap(){
        let pages=row.as_u64().unwrap()as usize;let(mut outcome,setup)=observe_retirement_allocations(||{if variant=="yield"{return(StepOutcome::Yield,None,PayloadCase::detached(OperationId(95001),Generation(41)));}if variant=="cancelled"{return(StepOutcome::Cancelled,None,PayloadCase::detached(OperationId(95001),Generation(41)));}let(payload,ledger,mut case)=admitted(pages,JobPayloadStream::Preview);let held=shared.then(||case.alias(&ledger));case.close_ledger(&mut Some(ledger));let outcome=match variant.as_str().unwrap(){"preview"=>StepOutcome::PreviewReady(payload),"checkpoint"=>StepOutcome::CheckpointReady(Checkpoint{state:payload,applied_progress:3}),"fault"=>StepOutcome::Fault(JobFault{detail:payload}),"complete"=>StepOutcome::Complete(CommitCandidate{state:payload,output:RetainedJobPayload::empty(JobPayloadStream::CommitOutput)}),_=>unreachable!()};(outcome,held,case)});
        let mut released=0;let mut turns=0;let ledger_bytes=semio_framework_value::shared_retirement_allocation_bytes::<JobPayloadOperationLedger>();
        while !outcome.0.terminal_is_empty(){let(step,heap)=retained_step(&mut outcome.0,&mut outcome.2);released+=heap.1;turns+=1;if turns==cut.as_u64().unwrap()as usize{let(same,heap)=observe_retirement_allocations(||outcome);outcome=same;assert_eq!(heap,(0,0));}assert!(turns<=pages+2);assert!(step.progress().copied_items>0);}
        assert_eq!(turns,if matches!(variant.as_str().unwrap(),"yield"|"cancelled"){0}else{pages+usize::from(pages>0)});
        let(mut held,heap)=observe_retirement_allocations(||outcome.1.take());assert_eq!(heap,(0,0));if held.is_some(){let demand=ledger_demands(&held);let(step,heap)=observe_retirement_allocations(||outcome.2.step(grant(demand),|_|close_ledger(&mut held,grant(demand))));assert_eq!(heap,(0,ledger_bytes));assert_eq!(step.progress().released_bytes,ledger_bytes);released+=heap.1;}
        assert_eq!(released,setup.0-setup.1);let(_,heap)=observe_retirement_allocations(||drop(outcome));assert_eq!(heap,(0,0));
    }}}}
}

#[test]
fn job_payload_retirement_staged_writer_preserves_original_ledger_after_page_release(){
    for shared in [false,true]{let((mut writer,mut held,mut case),setup)=observe_retirement_allocations(||{let mut case=PayloadCase::new(OperationId(95002),Generation(41));let mut writer=case.initialize(JobPayloadStream::Preview);case.stage(&mut writer,b"original\0");let(ledger,mut case)=case.into_ledger();let held=shared.then(||case.alias(&ledger));case.close_ledger(&mut Some(ledger));(writer,held,case)});let pointer=writer.staged_backing_identity().unwrap();let mut physical=0;
        let demand=writer.retirement_demands().unwrap();let full=grant(demand);let denied=RetainedCloneGrant{maximum_release_bytes:JOB_PAYLOAD_PAGE_BYTES-1,..full};let(step,heap)=observe_retirement_allocations(||case.step(denied,|_|writer.close_step_granted(denied)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(writer.staged_backing_identity().unwrap(),pointer);
        for _ in 0..10{if writer.terminal_is_empty(){break;}let(demand,heap)=observe_retirement_allocations(||writer.retirement_demands().unwrap());assert_eq!(heap,(0,0));let full=grant(demand);let(step,heap)=observe_retirement_allocations(||case.step(full,|_|writer.close_step_granted(full)));assert!(step.progress().fits(full));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),heap);physical+=heap.1;}
        assert!(writer.terminal_is_empty());if held.is_some(){let demand=ledger_demands(&held);let(step,heap)=observe_retirement_allocations(||case.step(grant(demand),|_|close_ledger(&mut held,grant(demand))));assert_eq!(step.progress().released_bytes,heap.1);physical+=heap.1;}assert_eq!(physical,setup.0-setup.1);let(_,heap)=observe_retirement_allocations(||drop((writer,held)));assert_eq!(heap,(0,0));
    }
}

#[test]
fn job_outcome_slot_retirement_keeps_original_header_until_separate_admitted_metadata(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../♻️retirement/📄️payload/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let policy=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:0,maximum_release_bytes:262144,maximum_depth:64};
    for body in [32768,4096]{let policy=RetainedCloneGrant{maximum_copy_bytes:body,..policy};
    for pages in law["pages"].as_array().unwrap(){for variant in law["variants"].as_array().unwrap(){
        let pages=pages.as_u64().unwrap()as usize;
        let((mut slot,mut case),setup)=observe_retirement_allocations(||{let mut issuer=None;let outcome=match variant.as_str().unwrap(){"yield"=>StepOutcome::Yield,"cancelled"=>StepOutcome::Cancelled,name=>{let(payload,ledger,mut case)=admitted(pages,JobPayloadStream::Preview);case.close_ledger(&mut Some(ledger));issuer=Some(case);match name{"preview"=>StepOutcome::PreviewReady(payload),"checkpoint"=>StepOutcome::CheckpointReady(Checkpoint{state:payload,applied_progress:7}),"fault"=>StepOutcome::Fault(JobFault{detail:payload}),"complete"=>StepOutcome::Complete(CommitCandidate{state:payload,output:RetainedJobPayload::empty(JobPayloadStream::CommitOutput)}),_=>unreachable!()}}};(JobOutcomeSlot::from_outcome(outcome),issuer.unwrap_or_else(||PayloadCase::detached(OperationId(95003),Generation(41))))});
        let physical=setup.0-setup.1;let mut released=0;let mut metadata_turns=0;let mut turns=0;
        while !slot.is_empty(){
            turns+=1;assert!(turns<=32);
            let(demand,heap)=observe_retirement_allocations(||step_outcome_slot_retirement_demands(&slot).unwrap());assert_eq!(heap,(0,0));
            let metadata=slot.original().unwrap().terminal_is_empty();
            let pointer=slot.original().map(|outcome|outcome as*const StepOutcome);
            for denied in denied_grants(policy,demand).into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||case.step(denied,|_|close_step_outcome_slot(&mut slot,denied)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(slot.original().map(|outcome|outcome as*const StepOutcome),pointer);}
            let(step,heap)=observe_retirement_allocations(||case.step(policy,|_|close_step_outcome_slot(&mut slot,policy)));let progress=step.progress();assert!(progress.fits(policy));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);released+=progress.released_bytes;
            if metadata{metadata_turns+=1;assert!(slot.is_empty());assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(progress.copied_bytes,0);assert_eq!(progress.copied_items,1);}else{assert!(!slot.is_empty());assert!(matches!(step,RetainedCloneStep::Progress(_)));}
        }
        assert_eq!(metadata_turns,1);assert_eq!(released,physical);
        let(step,heap)=observe_retirement_allocations(||case.step(policy,|_|close_step_outcome_slot(&mut slot,policy)));assert_eq!(step,RetainedCloneStep::Complete(Default::default()));assert_eq!(heap,(0,0));let(_,heap)=observe_retirement_allocations(||drop(slot));assert_eq!(heap,(0,0));
    }}}
}

#[test]
fn job_payload_retirement_original_fault_slot_closes_in_place_under_fixed_body(){
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../../♻️retirement/📄️payload/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let policy=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
    for pages in law["pages"].as_array().unwrap(){let pages=pages.as_u64().unwrap()as usize;
        let((mut slot,mut case),setup)=observe_retirement_allocations(||{let(payload,ledger,mut case)=admitted(pages,JobPayloadStream::Fault);case.close_ledger(&mut Some(ledger));(JobPayloadSlot::from_payload(payload),case)});
        let mut physical=0;let mut presence=0;let mut turns=0;
        while !slot.is_empty(){turns+=1;assert!(turns<=pages+3);let(demand,heap)=observe_retirement_allocations(||slot.retirement_demands().unwrap());assert_eq!(heap,(0,0));let pointer=slot.original().map(|original|original as*const RetainedJobPayload);let metadata=slot.original().unwrap().terminal_is_empty();
            for denied in denied_grants(policy,demand).into_iter().flatten(){let(step,heap)=observe_retirement_allocations(||case.step(denied,|_|slot.close_step(denied)));assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(slot.original().map(|original|original as*const RetainedJobPayload),pointer);}
            let(step,heap)=observe_retirement_allocations(||case.step(policy,|_|slot.close_step(policy)));let progress=step.progress();assert!(progress.fits(policy));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),heap);physical+=progress.released_bytes;
            if metadata{presence+=1;assert!(slot.is_empty());assert!(matches!(step,RetainedCloneStep::Complete(_)));assert_eq!(progress.copied_bytes,law["faultSlotPresenceBytes"].as_u64().unwrap()as usize);}else{assert!(!slot.is_empty());assert!(matches!(step,RetainedCloneStep::Progress(_)));assert_eq!(slot.original().map(|original|original as*const RetainedJobPayload),pointer);}
        }
        assert_eq!(presence,1);assert_eq!(physical,setup.0-setup.1);let(step,heap)=observe_retirement_allocations(||case.step(policy,|_|slot.close_step(policy)));assert_eq!(step,RetainedCloneStep::Complete(Default::default()));assert_eq!(heap,(0,0));let(_,heap)=observe_retirement_allocations(||drop(slot));assert_eq!(heap,(0,0));
    }
}
