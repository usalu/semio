use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::retained::retained_mutation_demand_fits;
#[path="🎟️demand/🦀️.rs"]mod close_demand;
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::RetainedCloneSource};
#[derive(semio_framework_value::RetireOwned)]
struct Row{name:String,value:f64,untouched:Vec<u64>}
#[derive(Default,Debug,PartialEq,serde::Deserialize,serde::Serialize,semio_framework_value::RetireOwned)]
struct Patch{#[serde(skip_serializing_if="Option::is_none")]name:Option<String>,#[serde(skip_serializing_if="Option::is_none")]value:Option<f64>}
crate::retained_row_patch!(pub inverse_fields for Patch => Row{name:String,value:f64});
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(op:impl FnOnce()->T)->(T,(usize,usize)){let(result,born,released)=crate::test_allocation::observe_backing(op);(result,(born,released))}
fn source<T:RetireOwned+Sync>(value:T)->RetainedCloneSource<T>{match RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}){Ok((source,_))=>source,Err((error,_,()))=>panic!("sparse original source:{error}")}}
fn close_source<T:RetireOwned+Sync>(source:&mut RetainedCloneSource<T>){while !source.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});assert!(source.close_step(quote).unwrap().progress().fits(quote));}}
fn close(cursor:&mut inverse_fields::Fields){cursor.begin_close();let mut turns=0;while !cursor.terminal_is_empty(){turns+=1;assert!(turns<1000);let quote=grant(cursor.close_demand(4096).unwrap());let(step,heap)=observe(||cursor.close_step(quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}
fn retire<T:RetireOwned+std::fmt::Debug>(value:T){let mut close=ControlledRetirement::new(value).unwrap();while !close.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:close.next_copy_byte_demand().unwrap(),capacity_bytes:close.next_capacity_byte_demand(4096).unwrap(),release_bytes:close.next_release_byte_demand().unwrap(),depth:close.next_depth_demand().unwrap()});assert!(close.step(quote).unwrap().progress().fits(quote));}}
fn row(value:&serde_json::Value)->Row{Row{name:value["name"].as_str().unwrap().into(),value:value["value"].as_f64().unwrap(),untouched:vec![42;1024]}}

/// 🧫️ Shared retained sparse inverses agree with the language-neutral setters and preserve sibling edits.
#[test]
fn retained_row_patch_inverse_and_owned_exchange_match_neutral_sparse_fields(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🩹️patch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus.as_array().unwrap(){
  let mut original=source(row(&case["before"]));let mut authored=source(serde_json::from_value::<Patch>(case["patch"].clone()).unwrap());let mut inverse=inverse_fields::Fields::new(RetainedPatchSource::Inverse);let mut turns=0;
  loop{turns+=1;assert!(turns<1000);let quote=grant(inverse.demand(authored.borrow(),original.borrow(),4).unwrap());
   let(step,heap)=observe(||inverse.advance(authored.borrow(),original.borrow(),RetainedCloneGrant::default()).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());
   let(step,heap)=observe(||inverse.advance(authored.borrow(),original.borrow(),quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break;}
  }
  let quote=grant(RetirementDemand{copy_bytes:std::mem::size_of::<Patch>()*2,depth:1,..Default::default()});let(mut undo,progress)=inverse.take(quote).unwrap();assert!(progress.fits(quote));assert_eq!(undo.as_ref().unwrap(),&serde_json::from_value::<Patch>(case["inverse"].clone()).unwrap());
  let mut authored_fields=inverse_fields::Fields::new(RetainedPatchSource::Authored);
  loop{let quote=grant(authored_fields.demand(authored.borrow(),original.borrow(),4).unwrap());let(step,heap)=observe(||authored_fields.advance(authored.borrow(),original.borrow(),quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break;}}
  let(mut forward,receipt)=authored_fields.take(quote).unwrap();assert!(receipt.fits(quote));let mut forward=forward.take().unwrap();assert_eq!(forward,serde_json::from_value::<Patch>(case["patch"].clone()).unwrap());
  let mut post=row(&case["before"]);let pointer=post.untouched.as_ptr();let name=post.name.as_ptr();
  for field in 0..Patch::FIELD_COUNT{let quote=grant(forward.exchange_demand(field).unwrap());assert!(retained_mutation_demand_fits(forward.exchange_demand(field).unwrap(),quote));let(progress,heap)=observe(||forward.exchange_field(&mut post,field,semio_framework_mutation_diff::testing_apply_capability(),quote).unwrap());assert!(progress.fits(quote));assert_eq!(heap,(0,0));}
  assert_eq!(post.name,case["after"]["name"].as_str().unwrap());assert_eq!(post.value,case["after"]["value"].as_f64().unwrap());assert_eq!(post.untouched.as_ptr(),pointer);
  if case["patch"].get("name").is_none(){assert_eq!(post.name.as_ptr(),name);}
  let sibling=case["patch"].get("value").is_none();if sibling{post.value=99.;}
  let undo=undo.as_mut().unwrap();for field in 0..Patch::FIELD_COUNT{let quote=grant(undo.exchange_demand(field).unwrap());undo.exchange_field(&mut post,field,semio_framework_mutation_diff::testing_apply_capability(),quote).unwrap();}
  assert_eq!(post.name,case["before"]["name"].as_str().unwrap());assert_eq!(post.value,if sibling{99.}else{case["before"]["value"].as_f64().unwrap()});assert_eq!(post.untouched.as_ptr(),pointer);
  close(&mut inverse);close(&mut authored_fields);retire(forward);retire(std::mem::take(undo));close_source(&mut authored);close_source(&mut original);
  eprintln!("[DEBUG] retained sparse patch {:?}: exact inverse fields, untouched1024-row pointer, sibling undo preserved",case["id"]);
 }
}

/// 🛑️ Cancellation closes partial sparse inverse fields before the original immutable sources.
#[test]
fn retained_row_patch_inverse_cancellation_closes_every_original_field(){
 for cancel in 0..24{
  let mut original=source(Row{name:"original🧱".into(),value:1.,untouched:vec![]});let mut authored=source(Patch{name:Some("new".into()),value:Some(2.)});let mut inverse=inverse_fields::Fields::new(RetainedPatchSource::Inverse);
  for _ in 0..cancel{let quote=grant(inverse.demand(authored.borrow(),original.borrow(),4).unwrap());if matches!(inverse.advance(authored.borrow(),original.borrow(),quote).unwrap(),RetainedCloneStep::Complete(_)){break;}}
  close(&mut inverse);let(_,heap)=observe(||drop(inverse));assert_eq!(heap,(0,0));close_source(&mut authored);close_source(&mut original);
 }
}

/// 🚚️ Generic authored-field application keeps original sources and displaced fields through funded closure.
#[test]
fn retained_row_patch_apply_preserves_portable_rows_and_every_cancelled_frontier(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🩹️patch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus.as_array().unwrap(){
  for cut in (0..40).map(Some).chain([None]){
   let mut original=source(row(&case["before"]));let mut authored=source(serde_json::from_value::<Patch>(case["patch"].clone()).unwrap());let mut post=row(&case["before"]);let pointer=post.untouched.as_ptr();let name=post.name.as_ptr();
   let(mut apply,heap)=observe(||RetainedRowPatchApply::<Row,Patch>::new());assert_eq!(heap,(0,0));let mut turns=0;
   while !apply.staged()&&cut.is_none_or(|limit|turns<limit){
    let quote=grant(apply.stage_demand(authored.borrow(),original.borrow(),4).unwrap());
    for denied in[RetainedCloneGrant{maximum_items:0,..quote},RetainedCloneGrant{maximum_copy_bytes:quote.maximum_copy_bytes.saturating_sub(1),..quote},RetainedCloneGrant{maximum_depth:0,..quote}]{let(step,heap)=observe(||apply.stage(authored.borrow(),original.borrow(),denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
    if quote.maximum_capacity_bytes>0{let(step,heap)=observe(||apply.stage(authored.borrow(),original.borrow(),RetainedCloneGrant{maximum_capacity_bytes:0,..quote}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}
    let(step,heap)=observe(||apply.stage(authored.borrow(),original.borrow(),quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;assert!(turns<1000);assert_eq!(post.name,case["before"]["name"].as_str().unwrap());assert_eq!(post.value,case["before"]["value"].as_f64().unwrap());
   }
   while apply.staged()&&!apply.complete()&&cut.is_none_or(|limit|turns<limit){
    let quote=grant(apply.exchange_demand().unwrap());let before=(post.name.clone(),post.value);for denied in[RetainedCloneGrant{maximum_items:0,..quote},RetainedCloneGrant{maximum_copy_bytes:quote.maximum_copy_bytes.saturating_sub(1),..quote},RetainedCloneGrant{maximum_depth:0,..quote}]{let(step,heap)=observe(||apply.exchange(&mut post,semio_framework_mutation_diff::testing_apply_capability(),denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!((&post.name,post.value),(&before.0,before.1));}
    let(step,heap)=observe(||apply.exchange(&mut post,semio_framework_mutation_diff::testing_apply_capability(),quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(0,0));turns+=1;
   }
   if cut.is_none(){assert!(apply.complete());assert_eq!(post.name,case["after"]["name"].as_str().unwrap());assert_eq!(post.value,case["after"]["value"].as_f64().unwrap());if case["patch"].get("name").is_none(){assert_eq!(post.name.as_ptr(),name);}}
   assert_eq!(post.untouched.as_ptr(),pointer);assert_eq!(original.borrow().get().name,case["before"]["name"].as_str().unwrap());assert_eq!(original.borrow().get().value,case["before"]["value"].as_f64().unwrap());
   apply.begin_close();while !apply.terminal_is_empty(){let quote=grant(apply.close_demand(4096).unwrap());let(step,heap)=observe(||apply.close_step(quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;assert!(turns<2000);}
   assert_eq!(observe(||drop(apply)).1,(0,0));close_source(&mut authored);close_source(&mut original);
  }
 }
 eprintln!("[DEBUG] common sparse apply four portable rows: original1024 sibling allocation preserved;41 cancellation frontiers each; exact physical staging/exchange/close receipts");
}
