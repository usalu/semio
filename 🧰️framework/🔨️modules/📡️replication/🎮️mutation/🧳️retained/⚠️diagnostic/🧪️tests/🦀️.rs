use semio_framework_test_fixtures::fixture_text;
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observe<T>(op:impl FnOnce()->T)->(T,(usize,usize)){let(result,born,released)=crate::test_allocation::observe_backing(op);(result,(born,released))}
fn source(value:Vec<String>)->RetainedCloneSource<Vec<String>>{match RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}){Ok((source,_))=>source,Err((error,_,()))=>panic!("original semantic target source:{error}")}}
fn close_source(source:&mut RetainedCloneSource<Vec<String>>){while !source.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap(),capacity_bytes:source.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});assert!(source.close_step(quote).unwrap().progress().fits(quote));}}
fn close<const N:usize>(owner:&mut RetainedDiagnosticPreparation<N>){owner.begin_close();let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let quote=grant(owner.close_demand(4096).unwrap());let(step,heap)=observe(||owner.close_step(quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}}
fn targets(source:&RetainedCloneSource<Vec<String>>)->[RetainedCloneRef<'_,String>;2]{[source.borrow().project(0,|rows|&rows[0]),source.borrow().project(1,|rows|&rows[1])]}
fn spec(case:&serde_json::Value)->RetainedDiagnosticSpec{match case["level"].as_str().unwrap(){"error"=>RetainedDiagnosticSpec{level:Severity::Error,code:"mutation.target-missing",message:"The addressed entity is missing."},"fatal"=>RetainedDiagnosticSpec{level:Severity::Fatal,code:"mutation.invariant",message:"The authored field is inadmissible."},_=>panic!("declared semantic fixture severity")}}

/// 🧫️ Refusal diagnostics build the exact schema rows from immutable original targets without cold outcomes.
#[test]
fn retained_semantic_diagnostics_match_neutral_original_code_level_text_and_target(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../⚠️diagnostic/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in corpus.as_array().unwrap(){
  let mut source=source(case["target"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect());let mut owner=RetainedDiagnosticPreparation::<2>::default();
  let(progress,heap)=observe(||owner.start(spec(case),RetainedCloneGrant::default()).unwrap());assert_eq!(heap,(0,0));assert_eq!(progress,Default::default());
  let quote=grant(RetainedDiagnosticPreparation::<2>::start_demand());let(progress,heap)=observe(||owner.start(spec(case),quote).unwrap());assert_eq!(heap,(0,0));assert!(progress.fits(quote));
  let mut turns=0;loop{turns+=1;assert!(turns<1000);let quote=grant(owner.demand(targets(&source),4).unwrap());
   let(step,heap)=observe(||owner.advance(targets(&source),4,RetainedCloneGrant::default()).unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),Default::default());
   let(step,heap)=observe(||owner.advance(targets(&source),4,quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){break;}
  }
  let quote=grant(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()});let(rows,receipt)=owner.take(quote).unwrap().unwrap();assert!(receipt.fits(quote));assert_eq!(rows.len(),1);let actual=&rows[0];assert_eq!(actual.level,spec(case).level);assert_eq!(actual.code.0,case["code"].as_str().unwrap());assert_eq!(actual.message,case["message"].as_str().unwrap());assert_eq!(serde_json::to_value(&actual.target).unwrap(),case["target"]);assert_eq!(actual.op_index,None);assert!(owner.take(quote).is_err());
  let mut ledger=MutationMessageLedgerRetirement::new(String::new(),rows);while !ledger.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(4096).unwrap(),release_bytes:ledger.next_release_byte_demand().unwrap(),depth:ledger.next_depth_demand().unwrap()});let(step,heap)=observe(||ledger.close_step(quote).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
  close(&mut owner);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));close_source(&mut source);
 }
}

/// 🛑️ Cancellation settles each partial original target, literal, row backing and publication frontier.
#[test]
fn retained_semantic_diagnostic_cancellation_closes_all_original_construction_frontiers(){
 let spec=RetainedDiagnosticSpec{level:Severity::Error,code:"mutation.target-missing",message:"The addressed entity is missing."};
 for cancel in 0..38{
  let mut source=source(vec!["original-🧱".into(),"target-🧩".into()]);let mut owner=RetainedDiagnosticPreparation::<2>::default();owner.start(spec,grant(RetainedDiagnosticPreparation::<2>::start_demand())).unwrap();
  for _ in 0..cancel{let quote=grant(owner.demand(targets(&source),4).unwrap());if matches!(owner.advance(targets(&source),4,quote).unwrap(),RetainedCloneStep::Complete(_)){break;}}
  close(&mut owner);close_source(&mut source);
 }
}
/// 🏷️ Declared literal target rows share funded address custody and cancellation semantics.
#[test]
fn retained_literal_diagnostic_targets_match_portable_outputs_and_partial_cancellation(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../⚠️diagnostic/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let case=corpus.as_array().unwrap().iter().find(|row|row["id"]=="declared-literal-targets").unwrap();let targets=[RetainedDiagnosticTarget::Literal("compression-params"),RetainedDiagnosticTarget::Literal("field-🧱")];
 for cut in std::iter::once(None).chain((0..38).map(Some)){let mut owner=RetainedDiagnosticPreparation::<2>::default();owner.start(spec(case),grant(RetainedDiagnosticPreparation::<2>::start_demand())).unwrap();let mut turns=0;let mut complete=false;while cut.is_none_or(|cut|turns<cut){let quote=grant(owner.target_demand(targets,128).unwrap());let(step,heap)=observe(||owner.advance_targets(targets,128,RetainedCloneGrant{maximum_items:0,..quote}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let(step,heap)=observe(||owner.advance_targets(targets,128,quote).unwrap());assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;if owner.literal_source.is_some()&&owner.phase==0{let changed=[RetainedDiagnosticTarget::Literal("compression-params-alt"),targets[1]];assert!(owner.target_demand(changed,128).is_err());}if matches!(step,RetainedCloneStep::Complete(_)){complete=true;break;}}
  if cut.is_none(){assert!(complete);let quote=grant(RetirementDemand{copy_bytes:size_of::<Vec<MutationMessage>>(),depth:1,..Default::default()});let(rows,p)=owner.take(quote).unwrap().unwrap();assert!(p.fits(quote));assert_eq!(serde_json::to_value(&rows[0].target).unwrap(),case["target"]);let mut ledger=MutationMessageLedgerRetirement::new(String::new(),rows);while !ledger.terminal_is_empty(){let quote=grant(RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(128).unwrap(),release_bytes:ledger.next_release_byte_demand().unwrap(),depth:ledger.next_depth_demand().unwrap()});let(s,heap)=observe(||ledger.close_step(quote).unwrap());assert_eq!(heap,(s.progress().retained_capacity_bytes,s.progress().released_bytes));}}
  close(&mut owner);
 }
}
