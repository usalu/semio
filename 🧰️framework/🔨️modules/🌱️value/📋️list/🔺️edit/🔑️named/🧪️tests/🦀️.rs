use super::*;
use crate::{IndexedOutputCursor,IndexedOutputWork};
struct Source{keys:[Vec<String>;4],indices:Vec<usize>}
impl NamedEditSource for Source{
 fn shape(&self)->IndexedEditShape{IndexedEditShape{base_len:self.keys[0].len(),removed:self.keys[1].len(),modified:self.keys[2].len(),added:self.keys[3].len()}}
 fn key_demand(&self,_:NamedEditLane,_:usize)->Result<RetirementDemand,ValueError>{Ok(contiguous_named_key_demand(self))}
 fn observe_key(&self,lane:NamedEditLane,ordinal:usize,grant:RetainedCloneGrant)->Result<Option<(&str,RetainedCloneProgress)>,ValueError>{Ok(observe_contiguous_named_key(self.key_demand(lane,ordinal)?,grant,||&self.keys[match lane{NamedEditLane::Base=>0,NamedEditLane::Removed=>1,NamedEditLane::Modified=>2,NamedEditLane::Added=>3}][ordinal]))}
 fn added_index(&self,ordinal:usize)->usize{self.indices[ordinal]}
}
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn close(plan:&mut NamedEditPlan,born:usize){
 plan.begin_close();let mut births=born;let mut releases=0;let mut turns=0;let limit=128+(plan.shape.base_len+plan.shape.added+plan.shape.removed+plan.shape.modified)*32;
 while !plan.terminal_is_empty(){
  let demand=plan.close_demand(32768).unwrap();let funded=grant(demand);
  for denied in[RetainedCloneGrant{maximum_items:0,..funded},RetainedCloneGrant{maximum_copy_bytes:0,..funded},RetainedCloneGrant{maximum_depth:0,..funded}]{let(step,heap)=crate::observe_retirement_allocations(||plan.close_step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}
  if demand.capacity_bytes>0{let(step,heap)=crate::observe_retirement_allocations(||plan.close_step(RetainedCloneGrant{maximum_capacity_bytes:0,..funded}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}
  if demand.release_bytes>0{let(step,heap)=crate::observe_retirement_allocations(||plan.close_step(RetainedCloneGrant{maximum_release_bytes:0,..funded}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}
  let(step,heap)=crate::observe_retirement_allocations(||plan.close_step(funded).unwrap());let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.copied_bytes<=funded.maximum_copy_bytes);births+=heap.0;releases+=heap.1;turns+=1;assert!(turns<limit);
 }
 assert_eq!(births,releases);
}
fn pointers(source:&Source)->Vec<(*const u8,usize)>{source.keys.iter().flat_map(|rows|rows.iter().map(|key|(key.as_ptr(),key.len()))).collect()}
/// 🧪️ Original named coordinates, refusals and physical receipts match the neutral binary-identity corpus.
#[test]
fn named_edit_plan_matches_portable_keys_and_physical_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for law in fixture["cases"].as_array().unwrap(){
  let source=Source{keys:["base","removed","modified","added"].map(|name|law[name].as_array().unwrap().iter().map(|key|key.as_str().unwrap().to_owned()).collect()),indices:law["indices"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize).collect()};
  let original=pointers(&source);let(mut plan,heap)=crate::observe_retirement_allocations(||NamedEditPlan::new(source.shape()));assert_eq!(heap,(0,0));let mut born=0;let mut turns=0;
  while !plan.settled(){
   let demand=plan.demand(&source,NamedEditPlan::fixed_copy_bytes()+256).unwrap();let funded=grant(demand);let state=(plan.phase,plan.cursor,plan.low,plan.high,plan.searching,plan.compare.offset);
   for denied in[RetainedCloneGrant{maximum_items:0,..funded},RetainedCloneGrant{maximum_copy_bytes:0,..funded},RetainedCloneGrant{maximum_depth:0,..funded}]{let(step,heap)=crate::observe_retirement_allocations(||plan.advance(&source,denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(state,(plan.phase,plan.cursor,plan.low,plan.high,plan.searching,plan.compare.offset));}
   if demand.capacity_bytes>0{let(step,heap)=crate::observe_retirement_allocations(||plan.advance(&source,RetainedCloneGrant{maximum_capacity_bytes:0,..funded}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));}
   let(step,heap)=crate::observe_retirement_allocations(||plan.advance(&source,funded).unwrap());let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.copied_bytes<=funded.maximum_copy_bytes);born+=heap.0;turns+=1;assert!(turns<2000);
  }
  let expected=&law["expected"];assert_eq!(plan.ready(),!expected.is_null());
  if plan.ready(){
   for ordinal in 0..source.shape().modified{
    let demand=plan.modified_index_demand(ordinal).unwrap();assert_eq!(demand.capacity_bytes,fixture["observation"]["capacityBytes"].as_u64().unwrap()as usize);assert_eq!(demand.release_bytes,fixture["observation"]["releaseBytes"].as_u64().unwrap()as usize);let funded=grant(demand);
    for currency in fixture["observation"]["deniedCurrencies"].as_array().unwrap(){let denied=match currency.as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:0,..funded},"copy"=>RetainedCloneGrant{maximum_copy_bytes:0,..funded},"depth"=>RetainedCloneGrant{maximum_depth:0,..funded},_=>panic!("undeclared coordinate observation currency")};let(output,heap)=crate::observe_retirement_allocations(||plan.observe_modified_index(ordinal,denied).unwrap());assert!(output.is_none());assert_eq!(heap,(0,0));}
    let(output,heap)=crate::observe_retirement_allocations(||plan.observe_modified_index(ordinal,funded).unwrap());let(index,p)=output.unwrap();assert_eq!(index,expected["modified"][ordinal].as_u64().unwrap()as usize);assert_eq!(heap,(0,0));assert_eq!(p.copied_bytes,demand.copy_bytes);assert_eq!(p.copied_items,1);
   }
   assert!(plan.modified_index_demand(source.shape().modified).is_err());
   let mut cursor=IndexedOutputCursor::default();let mut output=Vec::new();loop{match cursor.next(plan.plan().unwrap()).unwrap(){IndexedOutputWork::Original{index}=>output.push(source.keys[0][index].as_str()),IndexedOutputWork::Removed{..}=>(),IndexedOutputWork::Added{ordinal}=>output.push(source.keys[3][ordinal].as_str()),IndexedOutputWork::Complete=>break}cursor.advance(plan.plan().unwrap()).unwrap();}assert_eq!(serde_json::to_value(output).unwrap(),expected["output"]);
  }else{assert!(plan.fault().is_some());}
  assert_eq!(pointers(&source),original);close(&mut plan,born);assert_eq!(crate::observe_retirement_allocations(||drop(plan)).1,(0,0));assert_eq!(pointers(&source),original);
 }
 eprintln!("[DEBUG] shared named planner matches15 portable exact UTF8/key/coordinate laws; real paid directories close with equal physical receipts and original text identity at every refusal");
}
/// ✂️ Long common prefixes compare only granted pages and close actual coordinates at every selected frontier.
#[test]
fn named_edit_plan_long_prefixes_and_cancellation_are_bounded(){
 let prefix="k".repeat(65537);let source=Source{keys:[vec![format!("{prefix}é"),format!("{prefix}ö")],vec![format!("{prefix}é")],vec![format!("{prefix}ö")],vec![format!("{prefix}é")]],indices:vec![1]};let original=pointers(&source);
 for cut in[Some(0),Some(1),Some(7),Some(128),Some(25000),None]{
  let mut plan=NamedEditPlan::new(source.shape());let mut born=0;let mut steps=0;
  while !plan.settled()&&cut.is_none_or(|cut|steps<cut){let demand=plan.demand(&source,NamedEditPlan::fixed_copy_bytes()+Compare::fixed()+6).unwrap();let funded=grant(demand);let(step,heap)=crate::observe_retirement_allocations(||plan.advance(&source,funded).unwrap());let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert!(p.copied_bytes<=funded.maximum_copy_bytes);born+=heap.0;steps+=1;assert!(steps<65537*16);}
  if cut.is_none(){assert!(plan.ready());assert_eq!(plan.modified_index(0).unwrap(),1);assert_eq!(plan.plan().unwrap().entries(IndexedEditKind::Removed).unwrap()[0].index,0);}
  close(&mut plan,born);assert_eq!(crate::observe_retirement_allocations(||drop(plan)).1,(0,0));assert_eq!(pointers(&source),original);
 }
 let mut compare=Compare::default();let demand=compare.demand("a","b",0).unwrap();assert!(demand.copy_bytes>Compare::fixed());let(step,heap)=crate::observe_retirement_allocations(||compare.advance("a","b",RetainedCloneGrant{maximum_copy_bytes:Compare::fixed(),..grant(demand)}).unwrap());assert_eq!(step.1,RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(compare.offset,0);
 eprintln!("[DEBUG] shared named planner compares65537-byte UTF8 prefixes in funded3-byte pages, preserves positive work demand and original strings, and returns every directory at five cancellation cuts plus completion");
}
