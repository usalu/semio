use super::*;
use crate::schema::IdmlPart;
use semio_framework_value::{retained_clone::RetainedCloneSource,retirement::RetireOwned};
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn underfunded(d:RetirementDemand)->Vec<RetainedCloneGrant>{let g=grant(d);let mut rows=vec![RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:0,..g}];if d.copy_bytes!=0{rows.push(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g});}if d.capacity_bytes!=0{rows.push(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..g});}if d.release_bytes!=0{rows.push(RetainedCloneGrant{maximum_release_bytes:d.release_bytes-1,..g});}rows}
fn close(owner:&mut RetainedIdmlAdmission){owner.begin_close();for _ in 0..100000{if owner.terminal_is_empty(){return;}let demand=owner.close_demand(4096).unwrap();for g in underfunded(demand){assert_eq!(owner.close_step(g).unwrap().progress(),Default::default());}owner.close_step(grant(demand)).unwrap();}panic!("original admission close stalled");}
fn close_source<T:RetireOwned+Sync+'static>(source:&mut RetainedCloneSource<T>){for _ in 0..100000{if source.terminal_is_empty(){return;}let d=RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()};source.close_step(grant(d)).unwrap();}panic!("original source close stalled");}
fn source<T:RetireOwned+Sync+'static>(snapshot:T)->RetainedCloneSource<T>{let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096};match RetainedCloneSource::admit_owned(snapshot,(),g){Ok((source,_))=>source,Err((error,_,_))=>panic!("{error}")}}
#[test]
fn genuine_original_admission_has_no_underfunded_work_and_closes_every_frontier(){
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🧪️fixtures/🔣️.json")).unwrap();
    let lifecycle:serde_json::Value=serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap(){
        let snapshot=IdmlSnapshot{dom_version:row["version"].as_str().unwrap().into(),parts:row["parts"].as_array().unwrap().iter().map(|row|{let text=row[1].as_str().unwrap();IdmlPart{path:row[0].as_str().unwrap().into(),bytes:if text=="$mime"{crate::PACKAGE_MIME.as_bytes().into()}else{text.as_bytes().into()}}}).collect()};
        let before=snapshot.clone();let mut original=source(snapshot);let mut owner=RetainedIdmlAdmission::new();let mut turns=0;
        loop{let demand=owner.demand(original.borrow(),4096).unwrap();for g in underfunded(demand){assert_eq!(owner.advance(original.borrow(),4096,g).unwrap().progress(),Default::default());}let step=owner.advance(original.borrow(),4096,grant(demand)).unwrap();assert_eq!(original.borrow().get(),&before);turns+=1;assert!(turns<100000);if let RetainedIdmlAdmissionAdvance::Ready(_,valid)=step{assert_eq!(valid,row["valid"].as_bool().unwrap(),"{}",row["id"]);if valid{assert!(owner.version().is_some());}else{assert!(owner.refusal().is_some());}break;}}
        close(&mut owner);
        for cut in lifecycle["cancellationCuts"].as_array().unwrap(){let mut owner=RetainedIdmlAdmission::new();for _ in 0..cut.as_u64().unwrap(){let d=owner.demand(original.borrow(),4096).unwrap();if matches!(owner.advance(original.borrow(),4096,grant(d)).unwrap(),RetainedIdmlAdmissionAdvance::Ready(..)){break;}}close(&mut owner);assert_eq!(original.borrow().get(),&before);}
        close_source(&mut original);
    }
}

#[test]
fn replacement_admission_keeps_both_genuine_original_issuers_live(){
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
    for row in corpus["replacements"].as_array().unwrap(){
        let snapshot=IdmlSnapshot{dom_version:"8.0".into(),parts:vec![IdmlPart{path:"mimetype".into(),bytes:crate::PACKAGE_MIME.as_bytes().into()},IdmlPart{path:"designmap.xml".into(),bytes:b"<Document DOMVersion=\"8.0\"/>".into()}]};let saved=snapshot.clone();let mut original=source(snapshot);let bytes=row["bytes"].as_str().unwrap().as_bytes().to_vec();let saved_bytes=bytes.clone();let mut replacement=source(bytes);let mut owner=RetainedIdmlAdmission::replacement(row["index"].as_u64().unwrap() as usize);
        loop{let d=owner.replacement_demand(original.borrow(),replacement.borrow(),4096).unwrap();for g in underfunded(d){assert_eq!(owner.advance_replacement(original.borrow(),replacement.borrow(),4096,g).unwrap().progress(),Default::default());}if let RetainedIdmlAdmissionAdvance::Ready(_,valid)=owner.advance_replacement(original.borrow(),replacement.borrow(),4096,grant(d)).unwrap(){assert_eq!(valid,row["valid"].as_bool().unwrap());break;}}
        assert_eq!(original.borrow().get(),&saved);assert_eq!(replacement.borrow().get(),&saved_bytes);
        if row["valid"].as_bool().unwrap(){let mut base=RetainedIdmlAdmission::new();loop{let d=base.demand(original.borrow(),4096).unwrap();if matches!(base.advance(original.borrow(),4096,grant(d)).unwrap(),RetainedIdmlAdmissionAdvance::Ready(_,true)){break;}}
            let mut cursor=crate::io::admission::change::IdmlChangeCursor::new();let mut events=Vec::new();{let base_source=base.admitted_source(original.borrow(),None).unwrap();let next_source=owner.admitted_source(original.borrow(),Some(replacement.borrow())).unwrap();loop{let step=cursor.advance(base_source,next_source,base.admitted_index().unwrap(),owner.admitted_index().unwrap(),1);assert!(step.work<=1);assert!(step.error.is_none());if let Some(event)=step.event{events.push(match event{crate::io::admission::change::IdmlChangeEvent::Version=>serde_json::json!({"kind":"version"}),crate::io::admission::change::IdmlChangeEvent::Part{next,before}=>serde_json::json!({"kind":"part","next":next,"before":before}),_=>panic!("unexpected replacement change")});}if step.done{break;}}}assert_eq!(serde_json::Value::Array(events),row["events"]);cursor.close();close(&mut base);}
        close(&mut owner);close_source(&mut replacement);close_source(&mut original);
    }
}
