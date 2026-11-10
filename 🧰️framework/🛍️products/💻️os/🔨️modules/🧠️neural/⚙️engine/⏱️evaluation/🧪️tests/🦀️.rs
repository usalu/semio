use super::*;
use semio_os_kernel_neural_engine_model::testkit::EffectCaller;
use semio_os_kernel_neural_engine_registry::RegistryRetirement;

const WORK:u64=1<<16;
fn observe<T>(operation:impl FnOnce()->T)->(T,usize,usize){semio_os_kernel_neural_engine_model::testkit::observe_ownership(operation)}
fn grant_of(copy:usize,demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

#[derive(semio_framework_value::RetireOwned)]
struct FinishedChannels(EvalChannels);

type Sources=(SharedOwner<Tree>,SharedOwner<HistoryFoldIndex<String,Dictionary>>,HistoryFoldIndex<String,()>,SharedRegistry,RegistryRetirement);
/// 🌱️ Builds the preborn tree, seeds, dirty set and registry of one fixture row.
fn sources(row:&serde_json::Value)->Sources{
 let tree=SharedOwner::from_cold(Tree{neurons:row["nodes"].as_array().unwrap().iter().map(|node|crate::Neuron::with_kind(node["id"].as_str().unwrap(),node["kind"].as_str().unwrap(),serde_json::from_value::<Dictionary>(node["params"].clone()).unwrap())).collect(),synapses:row["edges"].as_array().unwrap().iter().map(|edge|crate::Synapse{id:String::new(),from:edge[0].as_str().unwrap().into(),to:edge[1].as_str().unwrap().into(),from_port:edge.get(2).and_then(serde_json::Value::as_str).unwrap_or_default().into(),to_port:edge.get(3).and_then(serde_json::Value::as_str).unwrap_or_default().into()}).collect()});
 let seeds=SharedOwner::from_cold(row["seeds"].as_object().unwrap().iter().map(|(key,value)|(key.clone(),serde_json::from_value::<Dictionary>(value.clone()).unwrap())).collect());
 let dirty=tree.neurons.iter().map(|node|(node.id.clone(),())).collect();
 let(registry,retirement)=SharedRegistry::new(crate::Registry::new());
 (tree,seeds,dirty,registry,retirement)
}
/// 🧹️ Drains a controlled owner through funded effects and returns its physical births and releases.
fn drain<T:RetireOwned>(caller:&mut EffectCaller,owner:&mut ControlledRetirement<T>,copy:usize)->(usize,usize){
 let(mut births,mut releases,mut stall)=(0,0,0);
 for _ in 0..1000000{
  if owner.terminal_is_empty(){break}let copy=copy.max(owner.next_copy_byte_demand().unwrap());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
  let(step,born,free)=observe(||caller.poll(grant,WORK,|effect|owner.step(copy,effect)).unwrap());assert!(step.progress().fits(grant));assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=born;releases+=free;
  if step.progress()==RetainedCloneProgress::default(){stall+=1}else{stall=0}assert!(stall<64,"retirement drain stalled without progress");
 }
 assert!(owner.terminal_is_empty());(births,releases)
}
/// 🧹️ Drains the registry supervisor through funded effects and returns its physical births and releases.
fn drain_registry(caller:&mut EffectCaller,owner:&mut RegistryRetirement,copy:usize)->(usize,usize){
 let(mut births,mut releases,mut stall)=(0,0,0);
 for _ in 0..1000000{
  if owner.terminal_is_empty(){break}let copy=copy.max(owner.next_copy_byte_demand().unwrap());let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
  let(step,born,free)=observe(||caller.poll(grant,WORK,|effect|owner.close_step(copy,effect)).unwrap());assert!(step.progress().fits(grant));assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=born;releases+=free;
  if step.progress()==RetainedCloneProgress::default(){stall+=1}else{stall=0}assert!(stall<64,"retirement drain stalled without progress");
 }
 assert!(owner.terminal_is_empty());(births,releases)
}

#[test]
fn original_budgeted_eval_normal_boundary_turns_match_oracle_and_cancel_same_source(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for copy in [1,3,64]{for cancel_at in [None,Some(0),Some(9),Some(37),Some(101)]{
  let((tree,seeds,dirty,registry,mut registry_retirement),born,free)=observe(||sources(row));let original=born-free;let source=tree.as_ptr();
  let(infos,born,free)=observe(||SharedOwner::from_cold(HistoryFoldIndex::new()));let original=original+born-free;
  let((mut state,registry),born,free)=observe(||{let authority=registry.clone();(BudgetedEvalState::new(tree,seeds,dirty,infos,registry,None),authority)});assert_eq!((born,free),(0,0));
  let(mut births,mut releases,mut result,mut idle,mut caller)=(0,0,None,0,EffectCaller::new());
  for turn in 0..1000000{
   if cancel_at==Some(turn){break}let demand=state.next_step_demands(copy).unwrap();let grant=grant_of(copy,demand);
   for _ in 0..2{let(step,born,free)=observe(||caller.turn(RetainedCloneGrant{maximum_items:0,..grant},WORK,|cx|state.step(cx)).unwrap());assert!(matches!(step,BudgetedEvalStep::Working));assert_eq!(state.normal_step_progress(),Default::default());assert_eq!((born,free),(0,0));}
   let(step,born,free)=observe(||caller.turn(grant,WORK,|cx|state.step(cx)).unwrap());let progress=state.normal_step_progress();
   assert_eq!(progress,caller.last());assert!(progress.fits(grant),"phase={} grant={grant:?} progress={progress:?}",state.phase);assert_eq!((born,free),(progress.retained_capacity_bytes,progress.released_bytes),"phase={} turn={turn}",state.phase);
   births+=born;releases+=free;assert_eq!(state.source().unwrap().as_ptr(),source);
   if let BudgetedEvalStep::Complete(output)=step{result=Some(output);break}
   if progress==RetainedCloneProgress::default(){idle+=1}else{idle=0}assert!(idle<64,"normal stall phase={} turn={turn} copy={copy} demand={demand:?}",state.phase);
  }
  if cancel_at.is_none(){
   let output=result.as_ref().expect("normal evaluation did not complete");
   let inputs:std::collections::BTreeMap<_,_>=output.channels.inputs.iter().map(|(key,value)|(key,serde_json::to_value(value).unwrap())).collect();let outputs:std::collections::BTreeMap<_,_>=output.channels.outputs.iter().map(|(key,value)|(key,serde_json::to_value(value).unwrap())).collect();
   assert_eq!(serde_json::to_value(inputs).unwrap(),row["expectedInputs"]);assert_eq!(serde_json::to_value(outputs).unwrap(),row["expectedOutputs"]);
  }
  let(mut output,born,free)=observe(||result.map(|output|{assert!(output.remaining.is_empty()&&output.pending_extensions.is_empty());ControlledRetirement::new(FinishedChannels(output.channels)).unwrap_or_else(|_|unreachable!())}));assert_eq!((born,free),(0,0));
  let(_,born,free)=observe(||{state.begin_close();drop(registry)});assert_eq!((born,free),(0,0));
  let mut close_idle=0;
  for close_turn in 0..1000000{
   if state.terminal_is_empty(){break}let demand=state.next_close_demands(copy).unwrap();let grant=grant_of(copy,demand);
   let(step,born,free)=observe(||caller.poll(grant,WORK,|effect|state.close_step(grant.maximum_copy_bytes,effect)).unwrap());assert_eq!(state.normal_step_progress(),step.progress());assert!(step.progress().fits(grant));assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
   births+=born;releases+=free;if step.progress()==RetainedCloneProgress::default(){close_idle+=1}else{close_idle=0}
   assert!(close_idle<64,"original normal-close stalled id={} cancel={cancel_at:?} copy={copy} turn={close_turn} phase={} owner={} retirement={} demand={demand:?} progress={:?}",row["id"],state.phase,state.owners.is_some(),state.retirement.is_some(),step.progress());
  }
  assert!(state.terminal_is_empty());let(_,born,free)=observe(||drop(state));assert_eq!((born,free),(0,0));
  if let Some(owner)=output.as_mut(){let(b,r)=drain(&mut caller,owner,copy);births+=b;releases+=r;}let(_,born,free)=observe(||drop(output));assert_eq!((born,free),(0,0));
  let(b,r)=drain_registry(&mut caller,&mut registry_retirement,copy);births+=b;releases+=r;
  assert_eq!(original+births,releases);let(_,born,free)=observe(||drop(registry_retirement));assert_eq!((born,free),(0,0));
 }}}
}

#[test]
fn original_budgeted_eval_state_preserves_complete_source_roster_and_closes_with_physical_receipts(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for copy in [1,3,64]{
  let((tree,seeds,dirty,registry,mut registry_retirement,infos,previous),born,free)=observe(||{
   let(tree,seeds,dirty,registry,retirement)=sources(row);
   let infos=SharedOwner::from_cold([(String::from("preborn-kind"),OperatorInfo{id:"same original metadata".into(),..Default::default()})].into_iter().collect());
   let previous=SharedOwner::from_cold(EvalChannels{inputs:row["expectedInputs"].as_object().unwrap().iter().map(|(key,value)|(key.clone(),serde_json::from_value::<Dictionary>(value.clone()).unwrap())).collect(),outputs:row["expectedOutputs"].as_object().unwrap().iter().map(|(key,value)|(key.clone(),serde_json::from_value::<Dictionary>(value.clone()).unwrap())).collect()});
   (tree,seeds,dirty,registry,retirement,infos,Some(previous))
  });let original=born-free;let source=tree.as_ptr();
  let(mut state,born,free)=observe(||BudgetedEvalState::new(tree,seeds,dirty,infos,registry,previous));assert_eq!((born,free),(0,0));assert_eq!(state.source().unwrap().as_ptr(),source);
  let(_,born,free)=observe(||state.begin_close());assert_eq!((born,free),(0,0));
  let(mut births,mut releases,mut idle,mut caller)=(0,0,0,EffectCaller::new());
  for turn in 0..1000000{
   if state.terminal_is_empty(){break}let demand=state.next_close_demands(copy).unwrap();let grant=grant_of(copy,demand);
   for _ in 0..2{let(step,born,free)=observe(||caller.poll(RetainedCloneGrant{maximum_items:0,..grant},WORK,|effect|state.close_step(grant.maximum_copy_bytes,effect)).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(state.normal_step_progress(),step.progress());assert_eq!((born,free),(0,0));}
   let(step,born,free)=observe(||caller.poll(grant,WORK,|effect|state.close_step(grant.maximum_copy_bytes,effect)).unwrap());assert!(step.progress().fits(grant));assert_eq!(state.normal_step_progress(),step.progress());assert_eq!((born,free),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
   births+=born;releases+=free;if step.progress()==RetainedCloneProgress::default(){idle+=1}else{idle=0}assert!(idle<64,"original state close stalled copy={copy} turn={turn} demand={demand:?}");
  }
  assert!(state.terminal_is_empty());let(_,born,free)=observe(||drop(state));assert_eq!((born,free),(0,0));
  let(b,r)=drain_registry(&mut caller,&mut registry_retirement,copy);births+=b;releases+=r;
  assert_eq!(original+births,releases);let(_,born,free)=observe(||drop(registry_retirement));assert_eq!((born,free),(0,0));
 }}
}

//#region 🧠️CacheRetirement
/// 🪙️ Builds the exact one-item grant of the next cache close action.
fn cache_grant(owner:&crate::NeuralCacheRetirement,copy:usize)->(RetainedCloneGrant,usize){
 let copy=copy.max(owner.next_demand(copy).unwrap().copy_bytes);
 (grant_of(copy,owner.next_demand(copy).unwrap()),copy)
}
#[test]
fn cache_retirement_preserves_shared_roots_and_drains_replaced_nested_values() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧵️retirement/🧪️tests/🧫️fixtures/🗃️cache-retirement/🔣️.json")).unwrap();
 let seeds:Vec<_>=fixture["operations"].as_array().unwrap().iter().filter(|value|value["op"]=="seed").collect();
 let payload:usize=seeds.iter().map(|seed|seed["field"].as_str().unwrap().len()+seed["text"].as_str().unwrap().len()*seed["repeat"].as_u64().unwrap()as usize).sum();assert_eq!(payload,fixture["expected"]["finalReleasedBytes"].as_u64().unwrap()as usize);
 for copy in [1,64,4096] {
  let mut caller=EffectCaller::new();
  let (_,born,freed)=observe(|| {
   let cache=SharedOwner::from_cold(crate::NeuralCache::new());
   for seed in &seeds {cache.seed(fixture["key"].as_u64().unwrap(),Dictionary::new().insert(seed["field"].as_str().unwrap(),Value::Atom(Atom::String(seed["text"].as_str().unwrap().repeat(seed["repeat"].as_u64().unwrap()as usize))))).unwrap();}
   let mut shared=crate::NeuralCacheRetirement::new(cache.clone());
   for _ in 0..100 {if shared.terminal_is_empty(){break;}let(grant,body)=cache_grant(&shared,copy);let(step,a,r)=observe(||caller.poll(grant,WORK,|effect|shared.close_step(body,effect)).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));}
   assert!(shared.terminal_is_empty());assert_eq!(cache.len(),1);
   let mut owner=crate::NeuralCacheRetirement::new(cache);
   for _ in 0..1000000 {
    if owner.terminal_is_empty(){break;}let(grant,body)=cache_grant(&owner,copy);
    let (zero,a,r)=observe(||caller.poll(RetainedCloneGrant {maximum_items:0,..grant},WORK,|effect|owner.close_step(body,effect)).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!((a,r),(0,0));
    let (step,a,r)=observe(||caller.poll(grant,WORK,|effect|owner.close_step(body,effect)).unwrap());assert!(step.progress().fits(grant));assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));
   }
   assert!(owner.terminal_is_empty());
  });assert_eq!(born,freed);
 }
}
#[test]
fn cache_live_final_drop_is_guarded_instead_of_recursively_destroying_dictionaries() {
 assert!(std::panic::catch_unwind(||{let cache=crate::NeuralCache::new();cache.seed(1,Dictionary::new().insert("label",Value::Atom(Atom::String("guarded".into())))).unwrap();drop(cache);}).is_err());
}
//#endregion 🧠️CacheRetirement

/// 🔒️ Static law: every normal evaluation action that moves retained currency or fails charges at least one fuel, so a one-fuel loan can never chain a second retained action behind its quote.
#[test]
fn every_retained_evaluation_action_charges_at_least_one_fuel(){
 let source=include_str!("../🦀️.rs");
 assert!(source.contains("if performed==0&&moved{cx.consume_fuel(1)}"));
 assert!(!source.contains("if closing_job{cx.consume_fuel(self.last_work)}"));
}
