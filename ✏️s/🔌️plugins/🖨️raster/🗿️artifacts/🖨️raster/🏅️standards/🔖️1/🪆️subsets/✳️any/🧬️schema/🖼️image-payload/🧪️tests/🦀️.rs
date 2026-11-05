//! 🖼️ Image payloads preserve original pixels, array wire and funded shared-owner closure.
use super::*;
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;
use semio_framework_value::{ToValue,RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneSource};
use semio_framework_value::retirement::{RetireOwned,arc_custody::SharedControlledRetirement};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree,ArtifactCanonicalJsonTreeCursor};
use std::sync::Arc;

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn stock()->RetainedCloneGrant{let f=fixture();let p=&f["stock"];RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:p["maximumDepth"].as_u64().unwrap() as usize}}
fn caller()->OriginalWorkFixture{OriginalWorkFixture::with_source(semio_framework_job::OperationId(994),semio_framework_job::Generation(17),stock(),||Some(1),u64::MAX,1<<24)}
fn loan()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn image(width:u32,height:u32,pixels:Vec<u8>)->SemioImageSnapshot{crate::semio_image_from_rgba8(width,height,pixels)}
fn owned(caller:&mut OriginalWorkFixture,image:SemioImageSnapshot)->RasterImagePayload{
    let mut output=None;caller.turn(loan(),|cx|{let turn=RasterImagePayload::from_owned(image,cx.retained_grant());let work=turn.work;let (payload,progress)=turn.result.unwrap_or_else(|(error,_)|panic!("payload fixture admission: {error}"));assert_eq!(progress.retained_capacity_bytes,RasterImagePayload::owned_birth_demands().capacity_bytes);output=Some(payload);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work})?;Ok(())}).unwrap();output.unwrap()
}
fn alias(caller:&mut OriginalWorkFixture,payload:&RasterImagePayload)->Arc<SemioImageSnapshot>{
    let mut output=None;caller.turn(loan(),|cx|{let turn=payload.owner(cx.retained_grant());let work=turn.work;let (owner,progress)=turn.result.unwrap();assert_eq!((progress.retained_capacity_bytes,progress.copied_bytes),(0,0));output=Some(owner);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work})?;Ok(())}).unwrap();output.unwrap()
}
fn close_alias(caller:&mut OriginalWorkFixture,owner:Arc<SemioImageSnapshot>)->(usize,usize){
    let mut owner=SharedControlledRetirement::lease(owner);let mut turns=0;let mut released=0;
    while !owner.terminal_is_empty(){caller.turn(loan(),|cx|{let demand=owner.next_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=owner.step(grant);if let Ok(step)=&result{released+=step.progress().released_bytes;}RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;Ok(())}).unwrap();turns+=1;assert!(turns<100_000);}(turns,released)
}
fn source<T:RetireOwned+Sync>(caller:&mut OriginalWorkFixture,value:T)->RetainedCloneSource<T>{
    let mut admission=RetainedCloneSource::<T>::admission_owned(value,());let mut output=None;
    while output.is_none(){caller.turn(loan(),|cx|{let turn=if admission.phase()=="ready"{admission.take_output(&mut output,cx.retained_grant())}else{admission.advance(cx.retained_grant())};semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();}assert!(admission.terminal_is_empty());output.unwrap()
}
fn close_source<T:RetireOwned+Sync>(caller:&mut OriginalWorkFixture,source:&mut RetainedCloneSource<T>){while !source.terminal_is_empty(){caller.turn(loan(),|cx|{let demand=source.next_close_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|source.close_step(grant))?;Ok(())}).unwrap();}}
fn canonical<T:ArtifactCanonicalJsonTree+RetireOwned>(caller:&mut OriginalWorkFixture,value:T)->Vec<u8>{
    let mut source=source(caller,value);let mut projection=None;
    caller.turn(loan(),|cx|{let turn=source.project_owned(0,|value|value as&dyn ArtifactCanonicalJsonTree,cx.retained_grant());let work=turn.work;let (value,progress)=turn.result.unwrap();projection=Some(value);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work})?;Ok(())}).unwrap();
    let mut cursor=None;caller.turn(loan(),|cx|{let (value,progress)=ArtifactCanonicalJsonTreeCursor::admit(projection.take().unwrap(),cx.retained_grant()).unwrap_or_else(|(error,_)|panic!("payload canonical fixture: {error}"));cursor=Some(value);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work:1})?;Ok(())}).unwrap();let mut cursor=cursor.unwrap();
    let mut output=Vec::new();let mut turns=0;
    while !cursor.terminal_is_empty(){caller.turn(loan(),|cx|{let mut bytes=[0;64];let step=cursor.advance(&mut bytes,cx.retained_grant())?;output.extend_from_slice(&bytes[..step.written_bytes]);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:Ok(step.ownership),work:cursor.last_turn_work()})?;Ok(())}).unwrap();turns+=1;assert!(turns<100_000);}assert!(cursor.is_complete());close_source(caller,&mut source);output
}

#[test]
fn payload_admission_preserves_original_pixels_and_refused_ownership(){
    let mut caller=caller();let input=image(1,1,vec![1,2,3,4]);let pointer=input.frames[0].rgba8.as_ptr();
    let turn=RasterImagePayload::from_owned(input,RetainedCloneGrant::default());assert_eq!(turn.work,0);let (_,input)=turn.result.err().expect("unfunded owned payload must refuse");assert_eq!(input.frames[0].rgba8.as_ptr(),pointer);
    let payload=owned(&mut caller,input);assert_eq!(payload.image().frames[0].rgba8.as_ptr(),pointer);let owner=alias(&mut caller,&payload);let second=alias(&mut caller,&payload);assert!(Arc::ptr_eq(&owner,&second));
    let turn=RasterImagePayload::from_shared(second,RetainedCloneGrant::default());let (_,second)=turn.result.err().expect("unfunded shared payload must refuse");assert!(Arc::ptr_eq(&owner,&second));
    let mut shared=None;caller.turn(loan(),|cx|{let turn=RasterImagePayload::from_shared(second,cx.retained_grant());let work=turn.work;let (value,progress)=turn.result.unwrap_or_else(|(error,_)|panic!("payload shared fixture: {error}"));assert_eq!(progress.retained_capacity_bytes,0);shared=Some(value);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work})?;Ok(())}).unwrap();let shared=shared.unwrap();assert_eq!(shared.image().frames[0].rgba8.as_ptr(),pointer);caller.retire_owned(shared,loan());caller.retire_owned(payload,loan());assert_eq!(owner.frames[0].rgba8.as_ptr(),pointer);close_alias(&mut caller,owner);caller.finish();
}

#[test]
fn live_child_alias_does_not_stall_payload_close_and_last_owner_funds_pixels(){
    let mut caller=caller();let payload=owned(&mut caller,image(262144,1,vec![7;1048576]));let owner=alias(&mut caller,&payload);let other=alias(&mut caller,&payload);
    caller.retire_owned(payload,loan());assert_eq!(owner.frames[0].rgba8.len(),1048576);
    let (turns,_)=close_alias(&mut caller,other);assert!(turns<=fixture()["maximumLiveAliasCloseTurns"].as_u64().unwrap() as usize);
    let (turns,released)=close_alias(&mut caller,owner);assert!(turns>1);assert!(released>=1048576);caller.finish();
}

#[test]
fn payload_native_receiving_and_canonical_bytes_match_image_and_serde(){
    for row in fixture()["cases"].as_array().unwrap(){let mut caller=caller();let pixels:Vec<u8>=serde_json::from_value(row["rgba"].clone()).unwrap();let image=image(row["width"].as_u64().unwrap() as u32,row["height"].as_u64().unwrap() as u32,pixels);
        let expected=image.to_value();let oracle=serde_json::to_vec(&serde_json::Value::from(&expected)).unwrap();
        let payload:RasterImagePayload=caller.receive_native(expected.clone(),loan());assert_eq!(payload.to_value(),expected);assert_eq!(payload.image(),&image);assert_eq!(canonical(&mut caller,payload),oracle);assert_eq!(canonical(&mut caller,image),oracle);caller.finish();
    }
}

#[test]
fn retained_payload_clone_alias_capacity_is_independent_of_pixel_length(){
    let fixture=fixture();let mut capacities=Vec::new();
    for bytes in fixture["cloneSizes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize){let mut caller=caller();let payload=owned(&mut caller,image((bytes/4) as u32,1,vec![3;bytes]));let pointer=payload.image().frames[0].rgba8.as_ptr();let mut source=source(&mut caller,payload);let mut cursor=RasterImagePayload::retained_clone_cursor();let mut capacity=0;
        while !cursor.output_ready(){caller.turn(loan(),|cx|{let turn=cursor.advance(source.try_borrow()?,cx.retained_grant().maximum_copy_bytes,cx.retained_grant());if let Ok(step)=&turn.result{capacity+=step.progress().retained_capacity_bytes;}semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();}
        let mut output=None;caller.turn(loan(),|cx|{let turn=cursor.take(cx.retained_grant());let work=turn.work;let (value,progress)=turn.result.unwrap().unwrap();output=Some(value);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work})?;Ok(())}).unwrap();let output=output.unwrap();assert_eq!(output.image().frames[0].rgba8.as_ptr(),pointer);
        cursor.begin_close();while !cursor.terminal_is_empty(){caller.turn(loan(),|cx|{let demand=cursor.close_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|cursor.close_step(grant))?;Ok(())}).unwrap();}
        close_source(&mut caller,&mut source);caller.retire_owned(output,loan());caller.finish();capacities.push(capacity);assert!(capacity<=fixture["maximumAliasCapacityBytes"].as_u64().unwrap() as usize);
    }
    assert!(capacities.windows(2).all(|pair|pair[0]==pair[1]));
}

#[path="native.rs"]mod native;
