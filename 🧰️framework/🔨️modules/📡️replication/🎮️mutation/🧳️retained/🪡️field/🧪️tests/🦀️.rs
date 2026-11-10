use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::field::harness::{Harness,Recorded,observe};
use semio_framework_value::RetirementDemand;
use semio_framework_value::retained_clone::{OriginalEffectReply,RetainedCloneGrant,RetainedCloneSource};
fn quote(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}
fn field_test_work(_:usize)->u64{4096}
fn advance(h:&mut Harness,field:&mut RetainedFieldPreparation<String>,source:&RetainedCloneSource<String>,body:usize,g:RetainedCloneGrant)->Recorded{h.turn(g,field_test_work(body),&mut |e,r,rec|if rec{field.recover_advance_into(source.borrow(),body,field_test_work(body),e,r)}else{field.advance_into(source.borrow(),body,field_test_work(body),e,r)})}
fn close(h:&mut Harness,field:&mut RetainedFieldPreparation<String>,source:RetainedCloneSource<String>) {
    let(started,work,heap)=h.native(quote(field.begin_close_demand()),1,|e|field.begin_close_paid(e));assert_eq!(work,1);assert_eq!(heap,(0,0));started.result.unwrap();
    while !field.terminal_is_empty() {
        let demand = field.close_demand(4096).unwrap();let work=field.close_work_demand(4096).unwrap();
        let denied = h.turn(RetainedCloneGrant::default(),work,&mut |e,r,rec|if rec{field.recover_close_into(4096,e,r)}else{field.close_into(4096,e,r)});
        assert_eq!(denied.result.unwrap().progress(), Default::default()); assert_eq!(denied.heap,(0,0));
        let grant = quote(demand);
        let step = h.turn(grant,work,&mut |e,r,rec|if rec{field.recover_close_into(4096,e,r)}else{field.close_into(4096,e,r)});
        assert!(step.result.unwrap().progress().fits(grant));
    }
    h.close_source(source);
}

/// 🪡️ SQLite-verified Unicode and empty setters use bounded original field custody and owned exchange.
#[test]
fn retained_field_exchange_matches_neutral_strings_and_physical_receipts() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../🪡️field/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for vector in vectors.as_array().unwrap() {
        let mut h = Harness::new();
        let original = h.source(vector["authored"].as_str().unwrap().into());
        let mut field = RetainedFieldPreparation::<String>::default();
        for _ in 0..1000 {
            let demand = field.demand(original.borrow(), 4, field_test_work(4)).unwrap();
            let grant = quote(demand);
            let denied = advance(&mut h,&mut field,&original,4,Default::default());
            assert_eq!(denied.result.unwrap().progress(),Default::default()); assert_eq!(denied.heap,(0,0));
            let step = advance(&mut h,&mut field,&original,4,grant).result.unwrap();
            assert!(step.progress().fits(grant));
            if matches!(step,RetainedCloneStep::Complete(_)) { break; }
        }
        assert_eq!(field.value().unwrap(),vector["authored"].as_str().unwrap());
        let grant = quote(field.exchange_demand().unwrap());
        let mut output = None;
        let(denied,_,heap) = h.native(RetainedCloneGrant::default(),1,|e|{let mut slot=None;drop(field.take_into(&mut output,e,OriginalEffectReply::borrow(&mut slot).unwrap()).unwrap());slot.take().unwrap().unwrap()});
        assert_eq!(denied.progress(),Default::default()); assert_eq!(heap,(0,0)); assert!(output.is_none());
        let(receipt,_,heap) = h.native(grant,1,|e|{let mut slot=None;drop(field.take_into(&mut output,e,OriginalEffectReply::borrow(&mut slot).unwrap()).unwrap());slot.take().unwrap().unwrap()});
        assert!(receipt.progress().fits(grant)); assert_eq!(heap,(0,0));
        assert_eq!(output.take().unwrap(),vector["expected"].as_str().unwrap());
        close(&mut h,&mut field,original);
    }
}

/// 🛑️ Cancelling before each text preparation frontier returns every source alias and staged field.
#[test]
fn retained_field_cancellation_closes_every_typed_frontier() {
    for frontier in 0..9 {
        let mut h = Harness::new();
        let original = h.source("Feldgröße 🧱️".into());
        let mut field = RetainedFieldPreparation::<String>::default();
        for _ in 0..frontier {
            let grant = quote(field.demand(original.borrow(),4,field_test_work(4)).unwrap());
            let _ = advance(&mut h,&mut field,&original,4,grant).result.unwrap();
            if field.value().is_some() { break; }
        }
        close(&mut h,&mut field,original);
        assert!(field.terminal_is_empty());
    }
}

/// 📏️ Original logical quantum survives an expanded physical field header grant on every real copy turn.
#[test]
fn retained_field_logical_body_survives_expanded_physical_headers() {
    let vectors:serde_json::Value=serde_json::from_str(fixture_text!("../../../📏️body/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    for row in vectors["rows"].as_array().unwrap() {
        let body=row["body"].as_u64().unwrap() as usize;
        let expected="original field payload ".repeat(32);
        let mut h = Harness::new();
        let original=h.source(expected.clone());
        let mut field=RetainedFieldPreparation::<String>::default();
        let mut turns=0;
        while field.value().is_none() {
            let demand=field.demand(original.borrow(),body,field_test_work(body)).unwrap();
            let exact=quote(demand);
            let grant=RetainedCloneGrant{maximum_copy_bytes:row["physical"].as_u64().unwrap() as usize,..exact};
            let denied=advance(&mut h,&mut field,&original,body,RetainedCloneGrant{maximum_items:0,..grant});
            assert_eq!(denied.result.unwrap().progress(),Default::default());assert_eq!(denied.heap,(0,0));
            let step=advance(&mut h,&mut field,&original,body,grant).result.unwrap();
            assert!(step.progress().fits(exact));
            turns+=1;assert!(turns<10000);
        }
        assert_eq!(field.value().unwrap(),&expected);
        assert_eq!(original.borrow().get(),&expected);
        close(&mut h,&mut field,original);
        assert_eq!(observe(||drop(field)).1,(0,0));
    }
}

/// 🔑️ The actual mutation capability publishes both original slots under one finite Source.
#[test]fn original_field_install_keeps_both_slots_on_denial(){use semio_framework_value::retained_clone::{OriginalEffectReply,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll};use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/staged.json")).unwrap();let mut row=law["rowBefore"].as_str().unwrap();let mut authored=Some(law["authored"].as_str().unwrap());let original=(row.as_ptr(),authored.unwrap().as_ptr());let mut displaced=None;let demand=super::field_install_demand::<&str>().unwrap();let stock=RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:demand.copy_bytes*2,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let mut bank=EffectBank::new(Wallet::new(stock,2));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};for denied in [true,false]{let selected=if denied{RetainedCloneGrant{maximum_items:0,..grant}}else{grant};let mut reply=None;let((poll,actual),heap)=crate::field::harness::observe(||bank.run_with_source(selected,1,|effect,turn|{assert_eq!(effect.work_source_turn(),Some(turn.identity()));let before=effect.performed_work();let capability=semio_framework_mutation_diff::testing_apply_capability();drop(super::install_field_into(&mut authored,&mut row,&mut displaced,capability,effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})}));assert_eq!(actual,None);assert_eq!(heap,(0,0));let RetainedWorkPoll::Complete(turn)=poll else{panic!("native field exchange pending")};let step=turn.result.unwrap();if denied{assert_eq!(turn.work,u128::from(law["deniedWork"].as_u64().unwrap()));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((row.as_ptr(),authored.unwrap().as_ptr()),original);assert!(displaced.is_none())}else{assert_eq!(turn.work,u128::from(law["publishWork"].as_u64().unwrap()));assert_eq!(row,law["rowAfter"].as_str().unwrap());assert_eq!(displaced,law["displaced"].as_str());assert_eq!((row.as_ptr(),displaced.unwrap().as_ptr()),(original.1,original.0));assert!(authored.is_none())}assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),selected,heap)}}

/// 🧳️ The real row and slots stay borrowed by the original installation authority.
#[test]fn original_field_install_frame_keeps_destination_and_source(){use semio_framework_value::retained_clone::{OriginalEffectAuthority,OriginalEffectReply,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError};use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/staged.json")).unwrap();let mut row=law["rowBefore"].as_str().unwrap();let mut authored=Some(law["authored"].as_str().unwrap());let mut displaced=None;let original=(row.as_ptr(),authored.unwrap().as_ptr());let admission=super::RetainedFieldInstallFrame::<&str>::admission_demand();let transfer=OriginalEffectAuthority::<super::RetainedFieldInstallFrame<'_,&str>>::producer_transfer_demand().unwrap();let release=super::RetainedFieldInstallFrame::<&str>::release_demand();let install=super::field_install_demand::<&str>().unwrap();let bytes=admission.copy_bytes+transfer.copy_bytes+release.copy_bytes+install.copy_bytes;let stock=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};let mut bank=EffectBank::new(Wallet::new(stock,4));let select=|d:semio_framework_value::RetirementDemand|RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let mut authority=None;let mut reply=None;let g=select(admission);let authored_slot=&mut authored;let row_slot=&mut row;let displaced_slot=&mut displaced;let((poll,actual),heap)=crate::field::harness::observe(||bank.run_with_source(g,1,|effect,turn|{let before=effect.performed_work();let capability=semio_framework_mutation_diff::testing_apply_capability();drop(super::RetainedFieldInstallFrame::admit_into(authored_slot,row_slot,displaced_slot,capability,turn,&mut authority,effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})}));assert_eq!(actual,None);assert_eq!(heap,(0,0));let RetainedWorkPoll::Complete(admitted)=poll else{panic!("installation admission pending")};let step=admitted.result.unwrap();assert_eq!(admitted.work,1);assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,heap);assert!(!authority.as_ref().unwrap().producer().installed());for denied in [true,false]{let g=if denied{RetainedCloneGrant{maximum_items:0,..select(install)}}else{select(install)};let mut action=None;let mut output=None;let((poll,actual),heap)=crate::field::harness::observe(||bank.run_with_source(g,1,|effect,turn|{let before=effect.performed_work();let mut context=super::RetainedFieldInstallContext::new(turn);let ready=authority.as_mut().unwrap().drive(&mut context,effect,&mut action).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,ValueError>(()),work:effect.performed_work()-before})}else{RetainedWorkPoll::Pending{work:effect.performed_work()-before}}}));assert_eq!(actual,None);assert_eq!(heap,(0,0));let RetainedWorkPoll::Complete(outcome)=poll else{panic!("installation unexpectedly pending")};assert!(action.as_mut().unwrap().transfer_reply(&mut output).unwrap());authority.as_mut().unwrap().finish(&mut action).unwrap();let step=output.take().unwrap().unwrap();if denied{assert_eq!(outcome.work,u128::from(law["deniedWork"].as_u64().unwrap()));assert_eq!(step.progress(),RetainedCloneProgress::default());assert!(!authority.as_ref().unwrap().producer().installed())}else{assert_eq!(outcome.work,u128::from(law["publishWork"].as_u64().unwrap()));assert!(matches!(step,RetainedCloneStep::Complete(_)));assert!(authority.as_ref().unwrap().producer().installed())}assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,heap)}let mut returned=None;let g=select(transfer);let(poll,actual)=bank.run_with_source(g,1,|effect,turn|{let before=effect.performed_work();let context=super::RetainedFieldInstallContext::new(turn);let result=OriginalEffectAuthority::transfer_producer(&mut authority,&context,&mut returned,effect).result;RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:effect.performed_work()-before})});assert_eq!(actual,None);let RetainedWorkPoll::Complete(turn)=poll else{panic!("frame transfer pending")};let step=turn.result.unwrap();assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,(0,0));let g=select(release);let(poll,actual)=bank.run(g,1,|effect|{let before=effect.performed_work();drop(super::RetainedFieldInstallFrame::release_into(&mut returned,effect,OriginalEffectReply::borrow(&mut reply).unwrap()).unwrap());RetainedWorkPoll::Complete(RetainedWorkOutcome{result:reply.take().unwrap(),work:effect.performed_work()-before})});assert_eq!(actual,None);let RetainedWorkPoll::Complete(turn)=poll else{panic!("frame release pending")};let step=turn.result.unwrap();assert_ne!(bank.take_admitted(),Some(false));bank.post(step.progress(),g,(0,0));drop(returned);drop(authority);assert!(authored.is_none());assert_eq!(row,law["rowAfter"].as_str().unwrap());assert_eq!(displaced,law["displaced"].as_str());assert_eq!((row.as_ptr(),displaced.unwrap().as_ptr()),(original.1,original.0));}
