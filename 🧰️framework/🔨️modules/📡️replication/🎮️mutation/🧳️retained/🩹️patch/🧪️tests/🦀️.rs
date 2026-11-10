use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::retained::retained_mutation_demand_fits;
use crate::retained::harness::{Harness,Recorded,observe};
#[path="🎟️demand/🦀️.rs"]mod close_demand;
use semio_framework_value::retained_clone::{RetainedCloneSource,RetainedCloneStep};
#[derive(semio_framework_value::RetireOwned)]
struct Row{name:String,value:f64,untouched:Vec<u64>}
#[derive(Default,Debug,PartialEq,serde::Deserialize,serde::Serialize,semio_framework_value::RetireOwned)]
struct Patch{#[serde(skip_serializing_if="Option::is_none")]name:Option<String>,#[serde(skip_serializing_if="Option::is_none")]value:Option<f64>}
crate::retained_empty_option_patch!(Patch{name,value});
crate::retained_row_patch!(pub inverse_fields for Patch => Row{name:String,value:f64});
type Fields=inverse_fields::Fields;
fn patch_test_work()->u64{inverse_fields::Fields::new_work_demand().unwrap()+1}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn adv(h:&mut Harness,f:&mut Fields,a:&RetainedCloneSource<Patch>,o:&RetainedCloneSource<Row>,g:RetainedCloneGrant)->Recorded{h.turn(g,patch_test_work(),&mut|e,r,rec|f.advance_reply(a.borrow(),o.borrow(),4,patch_test_work(),e,r,rec))}
fn stage(h:&mut Harness,x:&mut RetainedRowPatchApply<Row,Patch>,a:&RetainedCloneSource<Patch>,o:&RetainedCloneSource<Row>,g:RetainedCloneGrant)->Recorded{h.turn(g,patch_test_work(),&mut|e,r,rec|if rec{x.recover_stage_into(a.borrow(),o.borrow(),4,patch_test_work(),e,r)}else{x.stage_into(a.borrow(),o.borrow(),4,patch_test_work(),e,r)})}
fn begin_fields(h:&mut Harness,f:&mut Fields){let g=grant(f.begin_close_demand());let(start,work,_)=h.native(g,1,|e|f.begin_close_paid(e));assert_eq!(work,1);assert_eq!(start.work,1);start.result.unwrap();}
fn close(h:&mut Harness,cursor:&mut Fields){begin_fields(h,cursor);let mut turns=0;while !cursor.terminal_is_empty(){turns+=1;assert!(turns<1000);let quote=grant(cursor.close_demand(4096).unwrap());let units=cursor.close_work_demand(4096).unwrap();let recorded=h.turn(quote,units,&mut|e,r,rec|cursor.close_reply(None,4096,e,r,rec));let step=recorded.result.unwrap();assert!(step.progress().fits(quote));}}
fn close_apply(h:&mut Harness,apply:&mut RetainedRowPatchApply<Row,Patch>,turns:&mut usize){let g=grant(apply.begin_close_demand());let(start,work,_)=h.native(g,1,|e|apply.begin_close_paid(e));assert_eq!(work,1);assert_eq!(start.work,1);start.result.unwrap();while !apply.terminal_is_empty(){let quote=grant(apply.close_demand(4096).unwrap());let units=apply.close_work_demand(4096).unwrap();let recorded=h.turn(quote,units,&mut|e,r,rec|apply.close_reply(None,4096,e,r,rec));assert!(recorded.result.unwrap().progress().fits(quote));*turns+=1;assert!(*turns<2000);}}
fn row(value:&serde_json::Value)->Row{Row{name:value["name"].as_str().unwrap().into(),value:value["value"].as_f64().unwrap(),untouched:vec![42;1024]}}

/// 🧫️ Shared retained sparse inverses agree with the language-neutral setters and preserve sibling edits.
#[test]
fn retained_row_patch_inverse_and_owned_exchange_match_neutral_sparse_fields(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🩹️patch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus.as_array().unwrap(){
  let mut h=Harness::new();let mut original=h.source(row(&case["before"]));let mut authored=h.source(serde_json::from_value::<Patch>(case["patch"].clone()).unwrap());let mut inverse=inverse_fields::Fields::new(RetainedPatchSource::Inverse).0;let mut turns=0;
  loop{turns+=1;assert!(turns<1000);let quote=grant(inverse.demand(authored.borrow(),original.borrow(),4,patch_test_work()).unwrap());
   let denied=adv(&mut h,&mut inverse,&authored,&original,RetainedCloneGrant::default());assert_eq!(denied.heap,(0,0));assert_eq!(denied.result.unwrap().progress(),Default::default());
   let recorded=adv(&mut h,&mut inverse,&authored,&original,quote);let step=recorded.result.unwrap();assert!(step.progress().fits(quote));assert_eq!(recorded.heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break;}
  }
  let quote=grant(RetirementDemand{copy_bytes:std::mem::size_of::<Patch>()*2,depth:1,..Default::default()});let(mut undo,progress)=inverse.take(quote).unwrap();assert!(progress.fits(quote));assert_eq!(undo.as_ref().unwrap(),&serde_json::from_value::<Patch>(case["inverse"].clone()).unwrap());
  let mut authored_fields=inverse_fields::Fields::new(RetainedPatchSource::Authored).0;
  loop{let quote=grant(authored_fields.demand(authored.borrow(),original.borrow(),4,patch_test_work()).unwrap());let recorded=adv(&mut h,&mut authored_fields,&authored,&original,quote);let step=recorded.result.unwrap();assert!(step.progress().fits(quote));assert_eq!(recorded.heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break;}}
  let(mut forward,receipt)=authored_fields.take(quote).unwrap();assert!(receipt.fits(quote));let mut forward=forward.take().unwrap();assert_eq!(forward,serde_json::from_value::<Patch>(case["patch"].clone()).unwrap());
  let mut post=row(&case["before"]);let pointer=post.untouched.as_ptr();let name=post.name.as_ptr();
  for field in 0..Patch::FIELD_COUNT{let quote=grant(forward.exchange_demand(field).unwrap());assert!(retained_mutation_demand_fits(forward.exchange_demand(field).unwrap(),quote));let(progress,heap)=observe(||forward.exchange_field(&mut post,field,semio_framework_mutation_diff::testing_apply_capability(),quote).unwrap());assert!(progress.fits(quote));assert_eq!(heap,(0,0));}
  assert_eq!(post.name,case["after"]["name"].as_str().unwrap());assert_eq!(post.value,case["after"]["value"].as_f64().unwrap());assert_eq!(post.untouched.as_ptr(),pointer);
  if case["patch"].get("name").is_none(){assert_eq!(post.name.as_ptr(),name);}
  let sibling=case["patch"].get("value").is_none();if sibling{post.value=99.;}
  let undo=undo.as_mut().unwrap();for field in 0..Patch::FIELD_COUNT{let quote=grant(undo.exchange_demand(field).unwrap());undo.exchange_field(&mut post,field,semio_framework_mutation_diff::testing_apply_capability(),quote).unwrap();}
  assert_eq!(post.name,case["before"]["name"].as_str().unwrap());assert_eq!(post.value,if sibling{99.}else{case["before"]["value"].as_f64().unwrap()});assert_eq!(post.untouched.as_ptr(),pointer);
  close(&mut h,&mut inverse);close(&mut h,&mut authored_fields);h.retire(forward);h.retire(std::mem::take(undo));h.close_source(authored);h.close_source(original);
 }
}

/// 🛑️ Cancellation closes partial sparse inverse fields before the original immutable sources.
#[test]
fn retained_row_patch_inverse_cancellation_closes_every_original_field(){
 for cancel in 0..24{
  let mut h=Harness::new();let original=h.source(Row{name:"original🧱".into(),value:1.,untouched:vec![]});let authored=h.source(Patch{name:Some("new".into()),value:Some(2.)});let mut inverse=inverse_fields::Fields::new(RetainedPatchSource::Inverse).0;
  for _ in 0..cancel{let quote=grant(inverse.demand(authored.borrow(),original.borrow(),4,patch_test_work()).unwrap());if matches!(adv(&mut h,&mut inverse,&authored,&original,quote).result.unwrap(),RetainedCloneStep::Complete(_)){break;}}
  close(&mut h,&mut inverse);let(_,heap)=observe(||drop(inverse));assert_eq!(heap,(0,0));h.close_source(authored);h.close_source(original);
 }
}

/// 🚚️ Generic authored-field application keeps original sources and displaced fields through funded closure.
#[test]
fn retained_row_patch_apply_preserves_portable_rows_and_every_cancelled_frontier(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🩹️patch/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus.as_array().unwrap(){
  for cut in (0..40).map(Some).chain([None]){
   let mut h=Harness::new();let original=h.source(row(&case["before"]));let authored=h.source(serde_json::from_value::<Patch>(case["patch"].clone()).unwrap());let mut post=row(&case["before"]);let pointer=post.untouched.as_ptr();let name=post.name.as_ptr();
   let(mut apply,heap)=observe(||RetainedRowPatchApply::<Row,Patch>::new());assert_eq!(heap,(0,0));let mut turns=0;
   while !apply.staged()&&cut.is_none_or(|limit|turns<limit){
    let quote=grant(apply.stage_demand(authored.borrow(),original.borrow(),4,patch_test_work()).unwrap());
    for denied in[RetainedCloneGrant{maximum_items:0,..quote},RetainedCloneGrant{maximum_copy_bytes:quote.maximum_copy_bytes.saturating_sub(1),..quote},RetainedCloneGrant{maximum_depth:0,..quote}]{let recorded=stage(&mut h,&mut apply,&authored,&original,denied);assert_eq!(recorded.heap,(0,0));assert_eq!(recorded.result.unwrap().progress(),Default::default());}
    if quote.maximum_capacity_bytes>0{let recorded=stage(&mut h,&mut apply,&authored,&original,RetainedCloneGrant{maximum_capacity_bytes:0,..quote});assert_eq!(recorded.heap,(0,0));assert_eq!(recorded.result.unwrap().progress(),Default::default());}
    let recorded=stage(&mut h,&mut apply,&authored,&original,quote);let step=recorded.result.unwrap();assert!(step.progress().fits(quote));assert_eq!(recorded.heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;assert!(turns<1000);assert_eq!(post.name,case["before"]["name"].as_str().unwrap());assert_eq!(post.value,case["before"]["value"].as_f64().unwrap());
   }
   while apply.staged()&&!apply.complete()&&cut.is_none_or(|limit|turns<limit){
    let quote=grant(apply.exchange_demand().unwrap());let before=(post.name.clone(),post.value);for denied in[RetainedCloneGrant{maximum_items:0,..quote},RetainedCloneGrant{maximum_copy_bytes:quote.maximum_copy_bytes.saturating_sub(1),..quote},RetainedCloneGrant{maximum_depth:0,..quote}]{let(step,heap)=observe(||apply.exchange(&mut post,semio_framework_mutation_diff::testing_apply_capability(),denied).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!((&post.name,post.value),(&before.0,before.1));}
    let(step,heap)=observe(||apply.exchange(&mut post,semio_framework_mutation_diff::testing_apply_capability(),quote).result.unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(0,0));turns+=1;
   }
   if cut.is_none(){assert!(apply.complete());assert_eq!(post.name,case["after"]["name"].as_str().unwrap());assert_eq!(post.value,case["after"]["value"].as_f64().unwrap());if case["patch"].get("name").is_none(){assert_eq!(post.name.as_ptr(),name);}}
   assert_eq!(post.untouched.as_ptr(),pointer);assert_eq!(original.borrow().get().name,case["before"]["name"].as_str().unwrap());assert_eq!(original.borrow().get().value,case["before"]["value"].as_f64().unwrap());
   close_apply(&mut h,&mut apply,&mut turns);
   assert_eq!(observe(||drop(apply)).1,(0,0));h.close_source(authored);h.close_source(original);
  }
 }
}

/// 📏️ The apply header holds its staged field owner behind a charged allocation, so a retained diff's birth quote never carries the field owners inline.
#[test]
fn the_sparse_apply_header_stays_independent_of_the_field_owner_size(){
 assert!(std::mem::size_of::<RetainedRowPatchApply<Row,Patch>>()<std::mem::size_of::<inverse_fields::Fields>()+std::mem::size_of::<Option<Patch>>());
}

#[test]
fn patch_apply_paid_close_start_denial_and_repeat_preserve_the_original_owner(){let mut h=Harness::new();let mut owner=RetainedRowPatchApply::<Row,Patch>::new();let demand=owner.begin_close_demand();assert_eq!(demand.copy_bytes,std::mem::size_of::<bool>());assert_eq!(owner.begin_close_work_demand(),1);let(denied,_,heap)=h.native(RetainedCloneGrant::default(),1,|e|owner.begin_close_paid(e));assert_eq!(heap,(0,0));assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap(),RetainedCloneProgress::default());assert!(!owner.close_started());let(admitted,_,heap)=h.native(grant(demand),1,|e|owner.begin_close_paid(e));assert_eq!(heap,(0,0));assert_eq!(admitted.work,1);assert_eq!(admitted.result.unwrap().copied_bytes,std::mem::size_of::<bool>());assert!(owner.close_started());let(repeated,_,_)=h.native(RetainedCloneGrant::default(),1,|e|owner.begin_close_paid(e));assert_eq!(repeated.work,0);assert_eq!(repeated.result.unwrap(),RetainedCloneProgress::default());assert!(owner.terminal_is_empty());}
