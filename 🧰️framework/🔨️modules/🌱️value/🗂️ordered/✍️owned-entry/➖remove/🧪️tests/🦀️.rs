use super::*;
fn denied(d:RetirementDemand,g:RetainedCloneGrant)->impl Iterator<Item=RetainedCloneGrant>{[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(active,g)|active.then_some(g))}
#[test]
fn original_owned_removal_preserves_aliases_finite_stock_denials_and_cancellation(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔣️ordered-map.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../🔣️.json")).unwrap();
 for case in corpus["ownedRemovalCases"].as_array().unwrap(){for cut in policy["cancelAfter"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap())).chain([None]){
  let mut source=funded_bank(&policy);let base:OrderedMap<u64>=serde_json::from_value::<Vec<(String,u64)>>(case["base"].clone()).unwrap().into_iter().collect();let mut alias=Some(base.clone());let identity=base.original_root_identity();let key=case["key"].as_str().unwrap().to_owned();
  let(work,heap)=crate::observe_retirement_allocations(||OriginalOwnedEntry::remove(base,key));assert_eq!(heap,(0,0));assert_eq!(work.fields.base.as_ref().unwrap().original_root_identity(),identity);
  let mut work=Some(work);let mut output=None;let mut close=RetainedCloneClose::default();let mut output_close=RetainedCloneClose::default();let mut alias_close=RetainedCloneClose::default();let mut h=Harness::new_with_alias(&mut work,&mut output,&mut close,&mut output_close,&mut alias,&mut alias_close,&source);
  for turn in 0..policy["maximumTurns"].as_u64().unwrap(){
   if cut==Some(turn){break}let(d,units)=h.quote(Op::Advance);let g=grant(d);assert!(retained_clone_grant_admits(source.wallet().remaining(),d));
   for denied in denied(d,g){let step=h.run(&mut source,Op::Advance,denied,units);assert_eq!(h.last_work,0);assert_eq!(h.last_heap,(0,0));assert_eq!(step.progress(),Default::default());assert_eq!(h.quote(Op::Advance).0,d)}
   let step=h.run(&mut source,Op::Advance,g,units);assert!(step.progress().fits(g));if h.inspect(|p|p.output.is_some()){break}
  }
  if cut.is_none(){h.inspect(|p|{let actual:Vec<_>=p.output.as_ref().unwrap().iter().map(|(key,value)|(key.clone(),*value)).collect();let expected:Vec<(String,u64)>=serde_json::from_value(case["expected"].clone()).unwrap();assert_eq!(actual,expected)})}
  h.inspect(|p|{let actual:Vec<_>=p.extra.as_ref().unwrap().as_ref().unwrap().iter().map(|(key,value)|(key.clone(),*value)).collect();let expected:Vec<(String,u64)>=serde_json::from_value(case["base"].clone()).unwrap();assert_eq!(actual,expected)});
  for op in [Op::EntryStart,Op::EntryClose,Op::OutputStart,Op::OutputClose,Op::ExtraStart,Op::ExtraClose]{for _ in 0..10000{
   let empty=h.inspect(|p|match op{Op::EntryStart=>p.entry.is_none(),Op::EntryClose=>p.entry_close.is_empty(),Op::OutputStart=>p.output.is_none(),Op::OutputClose=>p.output_close.is_empty(),Op::ExtraStart=>p.extra.as_ref().unwrap().is_none(),Op::ExtraClose=>p.extra_close.as_ref().unwrap().is_empty(),_=>unreachable!()});if empty{break}
   let(d,units)=h.quote(op);let g=grant(d);assert!(retained_clone_grant_admits(source.wallet().remaining(),d));for denied in denied(d,g){let step=h.run(&mut source,op,denied,units);assert_eq!(h.last_work,0);assert_eq!(h.last_heap,(0,0));assert_eq!(step.progress(),Default::default());assert_eq!(h.quote(op).0,d)}h.run(&mut source,op,g,units);
  }}
  assert!(h.inspect(|p|p.empty()));h.release(&mut source)
 }}
}
