use super::*;
fn observe<T>(operation:impl FnOnce()->T)->(T,usize,usize){crate::registry::tests::observe_ownership(operation)}

#[test]
fn original_budgeted_input_merge_preserves_shared_payloads_and_physical_receipts(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for copy in [1,3,64]{
  let ((base,overlay),born,free)=observe(||(serde_json::from_value::<Dictionary>(row["base"].clone()).unwrap(),serde_json::from_value::<Dictionary>(row["overlay"].clone()).unwrap()));let original=born-free;
  let pointers:Vec<_>=overlay.iter().map(|(key,value)|(key.as_ptr(),value as*const Value)).collect();
  let (mut owner,born,free)=observe(||BudgetedInputMerge::new(base,overlay));assert_eq!((born,free),(0,0));
  let mut births=0;let mut releases=0;
  for turn in 0..100000{if owner.is_complete(){break;}let demand=owner.next_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   let (step,born,free)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((born,free),(0,0));
   if demand.capacity_bytes>0{let (step,born,free)=observe(||owner.step(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((born,free),(0,0));}
   let (step,born,free)=observe(||owner.step(grant).unwrap());assert_eq!(step.progress(),owner.step_progress());assert!(step.progress().fits(grant));assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes),"turn={turn} demand={demand:?}");births+=born;releases+=free;
  }
  assert!(owner.is_complete());let result=owner.output().unwrap();let actual=serde_json::to_value(result).unwrap();assert_eq!(actual,row["expected"]);
  for(index,(key,value))in owner.overlay().unwrap().iter().enumerate(){assert_eq!((key.as_ptr(),value as*const Value),pointers[index]);assert_eq!(result.get(key).unwrap()as*const Value,value as*const Value);}
  let (_,born,free)=observe(||owner.begin_close());assert_eq!((born,free),(0,0));let mut idle=0;
  for turn in 0..1000000{if owner.terminal_is_empty(){break;}let demand=owner.next_close_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,born,free)=observe(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress(),owner.step_progress());assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=born;releases+=free;if step.progress()==RetainedCloneProgress::default(){idle+=1;}else{idle=0;}assert!(idle<64,"input closure stall copy={copy} turn={turn} demand={demand:?}");}
  assert!(owner.terminal_is_empty());assert_eq!(original+births,releases);let(_,born,free)=observe(||drop(owner));assert_eq!((born,free),(0,0));eprintln!("[DEBUG] Original Neural input id={} copy={copy} original={original} born={births} release={releases} terminalDrop=0",row["id"]);
 }}
}

#[derive(semio_framework_value::RetireOwned)]
struct OwnedEntrySources{base:Option<Dictionary>,rows:Vec<(String,Value)>}
#[test]
fn original_owned_entry_update_funds_new_headers_and_preserves_preborn_payloads(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){for copy in [1,3,64]{let(mut sources,born,free)=observe(||OwnedEntrySources{base:Some(serde_json::from_value(row["base"].clone()).unwrap()),rows:row["overlay"].as_object().unwrap().iter().rev().map(|(key,value)|(key.clone(),serde_json::from_value(value.clone()).unwrap())).collect()});let original=born-free;let mut births=0;let mut releases=0;
 while let Some((key,value))=sources.rows.pop(){let pointer=key.as_ptr();let(mut cursor,born,free)=observe(||BudgetedOwnedEntry::new(sources.base.take().unwrap(),key,value));assert_eq!((born,free),(0,0));let mut idle=0;for _ in 0..1000000{let demand=cursor.next_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(result,born,free)=observe(||cursor.step(grant).unwrap());let progress=cursor.step_progress();assert!(progress.fits(grant));assert_eq!((born,free),(progress.retained_capacity_bytes,progress.released_bytes));births+=born;releases+=free;if let Some(output)=result{assert!(output.iter().any(|(key,_)|key.as_ptr()==pointer));sources.base=Some(output);break;}if progress==RetainedCloneProgress::default(){idle+=1}else{idle=0}assert!(idle<64,"original owned entry stalled phase={} demand={demand:?}",cursor.phase);}assert!(sources.base.is_some());cursor.begin_close();idle=0;for _ in 0..1000000{if cursor.terminal_is_empty(){break}let demand=cursor.next_close_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(step,born,free)=observe(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=born;releases+=free;if step.progress()==RetainedCloneProgress::default(){idle+=1}else{idle=0}assert!(idle<64);}assert!(cursor.terminal_is_empty());let(_,born,free)=observe(||drop(cursor));assert_eq!((born,free),(0,0));}
 assert_eq!(serde_json::to_value(sources.base.as_ref().unwrap()).unwrap(),row["expected"]);let(mut owner,born,free)=observe(||ControlledRetirement::new(sources).unwrap_or_else(|_|unreachable!()));assert_eq!((born,free),(0,0));for _ in 0..1000000{if owner.terminal_is_empty(){break}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,born,free)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=born;releases+=free;}assert!(owner.terminal_is_empty());assert_eq!(original+births,releases);let(_,born,free)=observe(||drop(owner));assert_eq!((born,free),(0,0));eprintln!("[DEBUG] Original owned entry id={} copy={copy} original={original} born={births} release={releases} terminalDrop=0",row["id"]);
 }}
}

thread_local!{static SELECTED:std::cell::Cell<Option<usize>>=const{std::cell::Cell::new(None)};}
struct PlanFixtureOperator(usize);
impl crate::Operator for PlanFixtureOperator {
 fn evaluate(&self,input:&Dictionary)->Result<Dictionary,crate::EvalError>{Ok(input.clone())}
 fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(crate::OperatorPlanAdmission,RetainedCloneProgress),(crate::EvalError,Dictionary)>{let result=crate::OperatorPlanAdmission::immediate(input,grant);if result.is_ok(){SELECTED.with(|selected|selected.set(Some(self.0)));}result}
 fn next_plan_copy_byte_demand(&self,_:&Dictionary)->Result<usize,ValueError>{Ok(0)}
 fn next_plan_capacity_byte_demand(&self,_:&Dictionary,_:usize)->Result<usize,ValueError>{Ok(0)}
 fn next_plan_release_byte_demand(&self,_:&Dictionary)->Result<usize,ValueError>{Ok(0)}
 fn next_plan_depth_demand(&self,_:&Dictionary)->Result<usize,ValueError>{Ok(1)}
}
#[derive(semio_framework_value::RetireOwned)]
struct PlanSources{registry:crate::Registry,input:Dictionary}

#[test]
fn original_registry_plan_cursor_preserves_input_and_selects_exact_signature_without_births(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["plans"].as_array().unwrap(){for copy in [1,3,64]{
  SELECTED.with(|selected|selected.set(None));let kind=row["kind"].as_str().unwrap();
  let((registry,mut input),born,free)=observe(||{let mut registry=crate::Registry::new();registry.register_operator(crate::OperatorInfo{id:"preborn-decoy-original".into(),..Default::default()},vec![crate::OperatorImpl{schemas:Vec::new(),operator:Box::new(PlanFixtureOperator(usize::MAX))}],&[]);let channels=row["channels"].as_array().unwrap().iter().map(|name|crate::ChannelSpec{code:String::new(),abbreviation:String::new(),name:name.as_str().unwrap().into(),full_name:String::new(),operators:Vec::new(),value_types:Vec::new(),item_types:Vec::new(),default:None,label:None,cardinality:crate::Cardinality::ZeroOrOne}).collect();let implementations=row["signatures"].as_array().unwrap().iter().enumerate().map(|(index,signature)|crate::OperatorImpl{schemas:signature.as_array().unwrap().iter().map(|item|item.as_str().unwrap().into()).collect(),operator:Box::new(PlanFixtureOperator(index))}).collect();registry.register_operator(crate::OperatorInfo{id:kind.into(),inputs:channels,..Default::default()},implementations,&[]);(registry,Some(serde_json::from_value::<Dictionary>(row["input"].clone()).unwrap()))});let original=born-free;
  let pointer=input.as_ref().unwrap().entry_at_rank(0).map(|(_,value)|value as*const Value);let(mut cursor,born,free)=observe(crate::OperatorPlanCursor::new);assert_eq!((born,free),(0,0));let mut admitted=None;
  for turn in 0..10000{let demand=registry.next_dispatch_job_demands(kind,&input,&cursor,copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let(_,born,free)=observe(||registry.dispatch_job(kind,&mut input,&mut cursor,RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(cursor.step_progress(),RetainedCloneProgress::default());assert_eq!((born,free),(0,0));assert_eq!(input.as_ref().unwrap().entry_at_rank(0).map(|(_,value)|value as*const Value),pointer);
   let(step,born,free)=observe(||registry.dispatch_job(kind,&mut input,&mut cursor,grant).unwrap());assert!(cursor.step_progress().fits(grant));assert_eq!((born,free),(cursor.step_progress().retained_capacity_bytes,cursor.step_progress().released_bytes));assert_eq!((born,free),(0,0),"original plan selection birth turn={turn}");if let crate::OperatorPlanStep::Admitted(crate::OperatorPlanAdmission::Immediate(original))=step{admitted=Some(original);break;}
  }
  let input=admitted.expect("same original input admitted");assert_eq!(input.entry_at_rank(0).map(|(_,value)|value as*const Value),pointer);assert_eq!(SELECTED.with(std::cell::Cell::get).unwrap(),row["selected"].as_u64().unwrap()as usize);let(mut close,born,free)=observe(||ControlledRetirement::new(PlanSources{registry,input}).unwrap_or_else(|_|panic!("original registry fixture declares every field")));assert_eq!((born,free),(0,0));let mut births=0;let mut releases=0;let mut idle=0;
  for turn in 0..1000000{if close.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:close.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:close.next_release_byte_demand().unwrap(),maximum_depth:close.next_depth_demand().unwrap()};let(step,born,free)=observe(||close.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=born;releases+=free;if step.progress()==RetainedCloneProgress::default(){idle+=1}else{idle=0}assert!(idle<64,"registry closure stall copy={copy} turn={turn}");}
  assert!(close.terminal_is_empty());assert_eq!(original+births,releases);let(_,born,free)=observe(||drop(close));assert_eq!((born,free),(0,0));eprintln!("[DEBUG] Original plan id={} copy={copy} original={original} born={births} release={releases} terminalDrop=0",row["id"]);
 }}
}
