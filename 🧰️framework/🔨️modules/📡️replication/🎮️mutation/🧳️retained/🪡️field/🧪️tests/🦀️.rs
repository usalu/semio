use semio_framework_test_fixtures::fixture_text;
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;

fn observe<T>(operation: impl FnOnce() -> T) -> (T, (usize,usize)) {
    let (result,requested,released)=crate::test_allocation::observe_backing(operation);
    (result,(requested,released))
}
fn quote(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn source(text: String) -> RetainedCloneSource<String> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_release_bytes: 4096, maximum_depth: 4096 };
    let mut admission = RetainedCloneSource::admission_owned(text, ()); let mut output = None;let mut actual=None; for _ in 0..64 { if admission.terminal_is_empty() { break; } if admission.phase() == "ready" { admission.take_output(&mut output, grant).result.unwrap(); } else { admission.advance(grant,&mut actual).result.unwrap();actual=None; } } output.expect("original field source admitted")
}

fn close(field: &mut RetainedFieldPreparation<String>, source: &mut RetainedCloneSource<String>) {
    let turn=field.begin_close_paid(quote(field.begin_close_demand()));assert_eq!(turn.work,1);turn.result.unwrap();
    while !field.terminal_is_empty() {
        let demand = field.close_demand(4096).unwrap();
        let (step, heap) = observe(|| field.close_step(Default::default()).result.unwrap());
        assert_eq!(step.progress(), Default::default()); assert_eq!(heap,(0,0));
        let grant = quote(demand);
        let (step, heap) = observe(|| field.close_step(grant).result.unwrap());
        assert!(step.progress().fits(grant));
        assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
    }
    while !source.terminal_is_empty() {
        let grant = quote(source.next_close_demand(4096).unwrap());
        assert!(source.close_step(grant).result.unwrap().progress().fits(grant));
    }
}

fn field_test_work(body:usize)->u64{body.max(2) as u64}

/// 🪡️ SQLite-verified Unicode and empty setters use bounded original field custody and owned exchange.
#[test]
fn retained_field_exchange_matches_neutral_strings_and_physical_receipts() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../🪡️field/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let mut original = source(vector["authored"].as_str().unwrap().into());
        let mut field = RetainedFieldPreparation::<String>::default();
        let mut unpublished = vector["before"].as_str().unwrap().to_owned();
        loop {
            let demand = field.demand(original.borrow(), 4, field_test_work(4)).unwrap();
            let grant = quote(demand);
            let (denied,heap) = observe(|| field.advance(original.borrow(), 4, field_test_work(4), Default::default()).result.unwrap());
            assert_eq!(denied.progress(),Default::default()); assert_eq!(heap,(0,0));
            let (step,heap) = observe(|| field.advance(original.borrow(),4,field_test_work(4),grant).result.unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            if matches!(step,RetainedCloneStep::Complete(_)) { break; }
        }
        assert_eq!(unpublished,vector["before"].as_str().unwrap());
        assert_eq!(field.value().unwrap(),vector["authored"].as_str().unwrap());
        let grant = quote(field.exchange_demand().unwrap());
        let (denied,heap) = observe(|| field.install(&mut unpublished,semio_framework_mutation_diff::testing_apply_capability(),Default::default()).unwrap());
        assert_eq!(denied,Default::default()); assert_eq!(heap,(0,0));
        let (receipt,heap) = observe(|| field.install(&mut unpublished,semio_framework_mutation_diff::testing_apply_capability(),grant).unwrap());
        assert!(receipt.fits(grant)); assert_eq!(heap,(0,0));
        assert_eq!(unpublished,vector["expected"].as_str().unwrap());
        close(&mut field,&mut original);
    }
}

/// 🛑️ Cancelling before each text preparation frontier returns every source alias and staged field.
#[test]
fn retained_field_cancellation_closes_every_typed_frontier() {
    for frontier in 0..9 {
        let mut original = source("Feldgröße 🧱️".into());
        let mut field = RetainedFieldPreparation::<String>::default();
        for _ in 0..frontier {
            let grant = quote(field.demand(original.borrow(),4,field_test_work(4)).unwrap());
            let _ = field.advance(original.borrow(),4,field_test_work(4),grant).result.unwrap();
            if field.value().is_some() { break; }
        }
        close(&mut field,&mut original);
        assert!(field.terminal_is_empty()); assert!(original.terminal_is_empty());
    }
}

/// 📏️ Original logical quantum survives an expanded physical field header grant on every real copy turn.
#[test]
fn retained_field_logical_body_survives_expanded_physical_headers() {
    let vectors:serde_json::Value=serde_json::from_str(fixture_text!("../../../📏️body/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in vectors["rows"].as_array().unwrap() {
        let body=row["body"].as_u64().unwrap() as usize;
        let expected="original field payload ".repeat(32);
        let mut original=source(expected.clone());
        let mut field=RetainedFieldPreparation::<String>::default();
        let mut turns=0;
        while field.value().is_none() {
            let demand=field.demand(original.borrow(),body,field_test_work(body)).unwrap();
            let exact=quote(demand);
            let grant=RetainedCloneGrant{maximum_copy_bytes:row["physical"].as_u64().unwrap() as usize,..exact};
            let (denied,heap)=observe(||field.advance(original.borrow(),body,field_test_work(body),RetainedCloneGrant{maximum_items:0,..grant}).result.unwrap());
            assert_eq!(denied.progress(),Default::default());assert_eq!(heap,(0,0));
            let (step,heap)=observe(||field.advance(original.borrow(),body,field_test_work(body),grant).result.unwrap());
            assert!(step.progress().fits(exact));
            assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
            turns+=1;assert!(turns<10000);
        }
        assert_eq!(field.value().unwrap(),&expected);
        assert_eq!(original.borrow().get(),&expected);
        close(&mut field,&mut original);
        assert_eq!(observe(||drop(field)).1,(0,0));
    }
}

#[test]
fn field_child_start_denies_zero_copy_then_posts_original_flag_once(){let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🛑️start.json")).unwrap();let mut field=RetainedFieldPreparation::<String>::default();let parent=field.begin_close_demand();let denied=field.begin_close_paid(RetainedCloneGrant::default());assert_eq!(denied.work,0);assert!(!field.close_started());let started=field.begin_close_paid(quote(parent));assert_eq!(started.work,1);assert_eq!(started.result.unwrap().copied_bytes,parent.copy_bytes);let repeat=field.begin_close_paid(quote(parent));assert_eq!(repeat.work,0);assert_eq!(repeat.result.unwrap(),RetainedCloneProgress::default());let demand=field.close_demand(4096).unwrap();assert_eq!(demand.copy_bytes,law["flagBytes"].as_u64().unwrap()as usize);assert_eq!(field.close_work_demand(4096).unwrap(),1);let mut denied=quote(demand);denied.maximum_copy_bytes=0;let turn=field.close_step(denied);assert_eq!(turn.work,law["denied"]["work"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress(),RetainedCloneProgress::default());assert!(!field.cursor.close_started());let turn=field.close_step(quote(demand));assert_eq!(turn.work,law["admitted"]["work"].as_u64().unwrap());assert_eq!(turn.result.unwrap().progress().copied_bytes,demand.copy_bytes);assert!(field.cursor.close_started());assert!(field.terminal_is_empty());}

/// 🔑️ The actual mutation capability publishes both original slots under one finite Source.
#[test]fn original_field_install_keeps_both_slots_on_denial(){use semio_framework_value::retained_clone::{OriginalEffectReply,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll};use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/staged.json")).unwrap();let mut row=law["rowBefore"].as_str().unwrap();let mut authored=Some(law["authored"].as_str().unwrap());let original=(row.as_ptr(),authored.unwrap().as_ptr());let mut displaced=None;let demand=super::field_install_demand::<&str>().unwrap();let stock=RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:demand.copy_bytes*2,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let mut bank=EffectBank::new(Wallet::new(stock,2));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};for denied in [true,false]{let selected=if denied{RetainedCloneGrant{maximum_items:0,..grant}}else{grant};let mut reply=None;let((poll,actual),heap)=crate::test_allocation::observe_retirement_allocations(||bank.run_with_source(selected,1,|effect,turn|{assert_eq!(effect.work_source_turn(),Some(turn.identity()));let before=effect.performed_work();let capability=semio_framework_mutation_diff::testing_apply_capability();drop(super::install_field_into(&mut authored,&mut row,&mut displaced,capability,effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})}));assert_eq!(actual,None);assert_eq!(heap,(0,0));let RetainedWorkPoll::Complete(turn)=poll else{panic!("native field exchange pending")};let step=turn.result.unwrap();if denied{assert_eq!(turn.work,u128::from(law["deniedWork"].as_u64().unwrap()));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((row.as_ptr(),authored.unwrap().as_ptr()),original);assert!(displaced.is_none())}else{assert_eq!(turn.work,u128::from(law["publishWork"].as_u64().unwrap()));assert_eq!(row,law["rowAfter"].as_str().unwrap());assert_eq!(displaced,law["displaced"].as_str());assert_eq!((row.as_ptr(),displaced.unwrap().as_ptr()),(original.1,original.0));assert!(authored.is_none())}assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),selected,heap)}}

/// 🧳️ The real row and slots stay borrowed by the original installation authority.
#[test]fn original_field_install_frame_keeps_destination_and_source(){use semio_framework_value::retained_clone::{OriginalEffectAuthority,OriginalEffectReply,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/staged.json")).unwrap();let mut row=law["rowBefore"].as_str().unwrap();let mut authored=Some(law["authored"].as_str().unwrap());let mut displaced=None;let original=(row.as_ptr(),authored.unwrap().as_ptr());let admission=super::RetainedFieldInstallFrame::<&str>::admission_demand();let transfer=OriginalEffectAuthority::<super::RetainedFieldInstallFrame<'_,&str>>::producer_transfer_demand().unwrap();let release=super::RetainedFieldInstallFrame::<&str>::release_demand();let install=super::field_install_demand::<&str>().unwrap();let bytes=admission.copy_bytes+transfer.copy_bytes+release.copy_bytes+install.copy_bytes;let stock=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let mut bank=EffectBank::new(Wallet::new(stock,4));let select=|d:semio_framework_value::RetirementDemand|RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let mut authority=None;let mut reply=None;let g=select(admission);let authored_slot=&mut authored;let row_slot=&mut row;let displaced_slot=&mut displaced;let((poll,actual),heap)=crate::test_allocation::observe_retirement_allocations(||bank.run_with_source(g,1,|effect,turn|{let before=effect.performed_work();let capability=semio_framework_mutation_diff::testing_apply_capability();drop(super::RetainedFieldInstallFrame::admit_into(authored_slot,row_slot,displaced_slot,capability,turn,&mut authority,effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})}));assert_eq!(actual,None);assert_eq!(heap,(0,0));let RetainedWorkPoll::Complete(admitted)=poll else{panic!("installation admission pending")};let step=admitted.result.unwrap();assert_eq!(admitted.work,1);assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,heap);assert!(!authority.as_ref().unwrap().producer().installed());for denied in [true,false]{let g=if denied{RetainedCloneGrant{maximum_items:0,..select(install)}}else{select(install)};let mut action=None;let mut output=None;let((poll,actual),heap)=crate::test_allocation::observe_retirement_allocations(||bank.run_with_source(g,1,|effect,turn|{let before=effect.performed_work();let mut context=super::RetainedFieldInstallContext::new(turn);let ready=authority.as_mut().unwrap().drive(&mut context,effect,&mut action).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:effect.performed_work()-before})}else{RetainedWorkPoll::Pending{work:effect.performed_work()-before}}}));assert_eq!(actual,None);assert_eq!(heap,(0,0));let RetainedWorkPoll::Complete(outcome)=poll else{panic!("installation unexpectedly pending")};assert!(action.as_mut().unwrap().transfer_reply(&mut output).unwrap());authority.as_mut().unwrap().finish(&mut action).unwrap();let step=output.take().unwrap().unwrap();if denied{assert_eq!(outcome.work,u128::from(law["deniedWork"].as_u64().unwrap()));assert_eq!(step.progress(),RetainedCloneProgress::default());assert!(!authority.as_ref().unwrap().producer().installed())}else{assert_eq!(outcome.work,u128::from(law["publishWork"].as_u64().unwrap()));assert!(matches!(step,RetainedCloneStep::Complete(_)));assert!(authority.as_ref().unwrap().producer().installed())}assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,heap)}let mut returned=None;let g=select(transfer);let(poll,actual)=bank.run_with_source(g,1,|effect,turn|{let before=effect.performed_work();let context=super::RetainedFieldInstallContext::new(turn);let result=OriginalEffectAuthority::transfer_producer(&mut authority,&context,&mut returned,effect).result;RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:effect.performed_work()-before})});assert_eq!(actual,None);let RetainedWorkPoll::Complete(turn)=poll else{panic!("frame transfer pending")};let step=turn.result.unwrap();assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,(0,0));let g=select(release);let(poll,actual)=bank.run(g,1,|effect|{let before=effect.performed_work();drop(super::RetainedFieldInstallFrame::release_into(&mut returned,effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})});assert_eq!(actual,None);let RetainedWorkPoll::Complete(turn)=poll else{panic!("frame release pending")};let step=turn.result.unwrap();assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,(0,0));drop(returned);drop(authority);assert!(authored.is_none());assert_eq!(row,law["rowAfter"].as_str().unwrap());assert_eq!(displaced,law["displaced"].as_str());assert_eq!((row.as_ptr(),displaced.unwrap().as_ptr()),(original.1,original.0));}
