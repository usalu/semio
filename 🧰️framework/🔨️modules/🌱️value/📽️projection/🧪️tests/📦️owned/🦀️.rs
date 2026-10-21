use semio_framework_test_fixtures::fixture_text;
use super::*;

#[derive(crate::RetireOwned,crate::SnapshotProjection)]
#[value(crate="crate")]
struct OwnedRecord{schema:String,bytes:Vec<u8>,dictionary:Option<Vec<u8>>}
impl Clone for OwnedRecord{fn clone(&self)->Self{panic!("whole record projection must never clone")}}

fn project_owned(count:usize,schema:&str,octet:u8,optional:bool,cut:Option<usize>){
 let value=OwnedRecord{schema:schema.into(),bytes:vec![octet;count],dictionary:optional.then(||vec![octet;count])};
 let d=RetainedCloneSource::<OwnedRecord>::owned_constructor_demand::<()>();
 let grant=funded(RetirementDemand{copy_bytes:RetainedCloneSource::<OwnedRecord>::constructor_copy_bytes(),capacity_bytes:d.capacity_bytes,depth:d.depth,..Default::default()});
 let(mut source,_)=RetainedCloneSource::admit_owned(value,(),grant).map_err(|(e,..)|e).unwrap();
 let pointer=source.borrow().get().bytes.as_ptr();let optional_pointer=source.borrow().get().dictionary.as_ref().map(Vec::as_ptr);
 let((mut cursor,p),heap)=observe_retirement_allocations(||OwnedRecord::begin_projection(funded(OwnedRecord::projection_birth_demand())).unwrap());
 assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
 let mut turns=0;
 while !cursor.ready()&&cut.is_none_or(|cut|turns<cut){
  let d=cursor.demands(source.borrow(),4096).unwrap();let grant=funded(d);
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(d.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..grant}),(d.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..grant}),(d.depth>0).then_some(RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..grant})].into_iter().flatten(){
   let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,denied).unwrap());
   assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(cursor.demands(source.borrow(),4096).unwrap(),d);
  }
  let(step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),4096,grant).unwrap());
  assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));turns+=1;assert!(turns<count.saturating_mul(4).saturating_add(schema.len()).saturating_add(256));
 }
 if cursor.ready()&&cut.is_none(){
  let d=cursor.take_demand().unwrap();let grant=funded(d);
  assert!(cursor.take(RetainedCloneGrant{maximum_items:0,..grant}).unwrap().is_none());
  let(output,p)=cursor.take(grant).unwrap().unwrap();assert!(p.fits(grant));
  assert_eq!(output.schema,schema);assert_eq!(output.bytes,vec![octet;count]);assert_eq!(output.dictionary,optional.then(||vec![octet;count]));
  if count>0{assert_ne!(output.bytes.as_ptr(),pointer);}
  drain(output);
 }
 cursor.begin_close();for _ in 0..100000{
  if cursor.terminal_is_empty(){break;}let d=cursor.close_demands(4096).unwrap();let grant=funded(d);
  let(step,heap)=observe_retirement_allocations(||cursor.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
  let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
 }
 assert!(cursor.terminal_is_empty());assert_eq!(source.borrow().get().bytes.as_ptr(),pointer);assert_eq!(source.borrow().get().bytes.len(),count);assert_eq!(source.borrow().get().dictionary.as_ref().map(Vec::as_ptr),optional_pointer);
 assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
 for _ in 0..256usize.saturating_add(schema.len()).saturating_add(count.saturating_mul(if optional{32}else{16})){if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let g=funded(d);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);source.close_step(g).unwrap();}
 assert!(source.terminal_is_empty());
 eprintln!("[DEBUG] owned projection bytes={count} cut={cut:?} turns={turns} original stable, exact heap receipts, zero whole-record clones, terminal closure");
}

#[test]
fn owned_snapshot_projection_composes_real_string_and_byte_fields_with_exact_custody(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📦️owned/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){let count=row["count"].as_u64().unwrap()as usize;let schema=row["schema"].as_str().unwrap();let octet=row["octet"].as_u64().unwrap()as u8;let optional=row["optionalPayload"].as_bool().unwrap();project_owned(count,schema,octet,optional,None);for cut in corpus["cancelCuts"].as_array().unwrap(){project_owned(count,schema,octet,optional,Some(cut.as_u64().unwrap()as usize));}}
}

/// 🫱️ Funded associated output handoff retains denied candidates and moves only their owned handle.
#[test]
fn owned_projection_field_output_handoff_preserves_candidate_and_projected_custody(){
 use crate::snapshot_projection::{ProjectionField,ProjectionOutput};
 use std::mem::size_of;
 let law:serde_json::Value=serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📦️owned/🔣️.json")).unwrap();assert_eq!(law["outputHandoffs"].as_array().unwrap().len(),8);
 let birth=RetainedCloneSource::<OwnedRecord>::owned_constructor_demand::<()>();let value=OwnedRecord{schema:"handoff".into(),bytes:vec![17;3],dictionary:None};
 let(mut original,_)=RetainedCloneSource::admit_owned(value,(),funded(RetirementDemand{copy_bytes:RetainedCloneSource::<OwnedRecord>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})).map_err(|(e,..)|e).unwrap();
 let original_pointer=original.borrow().get().bytes.as_ptr();let mut field=ProjectionField::<OwnedRecord>::default();assert!(field.take_output_demand().is_err());
 for _ in 0..10000{if field.ready(){break;}let d=field.demands(original.borrow(),4096).unwrap();let g=funded(d);let(step,heap)=observe_retirement_allocations(||field.advance(original.borrow(),4096,g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
 assert!(field.ready());let d=field.take_output_demand().unwrap();assert_eq!(d,RetirementDemand{copy_bytes:2*size_of::<<OwnedRecord as SnapshotProjection>::Output>(),depth:1,..Default::default()});let g=funded(d);
 for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(result,heap)=observe_retirement_allocations(||field.take_output(denied).unwrap());assert!(result.is_none());assert_eq!(heap,(0,0));assert!(field.ready());assert_eq!(field.take_output_demand().unwrap(),d);assert_eq!(original.borrow().get().bytes.as_ptr(),original_pointer);}
 let((mut output,receipt),heap)=observe_retirement_allocations(||field.take_output(g).unwrap().unwrap());assert!(receipt.fits(g));assert_eq!(heap,(0,0));assert!(!field.ready());assert!(field.take_output_demand().is_err());assert!(field.demands(original.borrow(),4096).is_err());let(result,heap)=observe_retirement_allocations(||field.advance(original.borrow(),4096,g));assert!(result.is_err());assert_eq!(heap,(0,0));let projected:&OwnedRecord=output.projected_value();let pointer=projected as*const OwnedRecord;assert_eq!(projected.bytes,[17;3]);let projected:&mut OwnedRecord=output.projected_value_mut();projected.bytes[0]=23;assert_eq!(original.borrow().get().bytes,[17;3]);
 type Source=RetainedCloneSource<OwnedRecord,<OwnedRecord as SnapshotProjection>::Output>;let birth=Source::constructor_demand::<()>();let grant=funded(RetirementDemand{copy_bytes:Source::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});let((mut intermediate,receipt),heap)=observe_retirement_allocations(||Source::admit_borrowed(output,|owner|owner.projected_value(),grant).map_err(|(e,_)|e).unwrap());assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(intermediate.borrow().get()as*const OwnedRecord,pointer);assert_eq!(intermediate.borrow().get().bytes,[23,17,17]);
 field.begin_close();assert!(field.terminal_is_empty());assert!(field.take_output_demand().is_err());drop(field);
 for source in [&mut intermediate]{for _ in 0..10000{if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let g=funded(d);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||source.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(source.terminal_is_empty());}
 for _ in 0..10000{if original.terminal_is_empty(){break;}let d=original.next_close_demand(4096).unwrap();let g=funded(d);assert_eq!(original.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||original.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(original.terminal_is_empty());
}

/// 🫧️ Zero-sized output handoff still requires item and depth authority without inventing payload bytes.
#[test]
fn zero_sized_projection_field_output_handoff_requires_only_its_real_axes(){
 use crate::snapshot_projection::ProjectionField;
 let birth=RetainedCloneSource::<()>::owned_constructor_demand::<()>();let(mut source,_)=RetainedCloneSource::admit_owned((),(),funded(RetirementDemand{copy_bytes:RetainedCloneSource::<()>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})).map_err(|(error,..)|error).unwrap();let mut field=ProjectionField::<()>::default();
 for _ in 0..128{if field.ready(){break;}let d=field.demands(source.borrow(),4096).unwrap();let g=funded(d);let(step,heap)=observe_retirement_allocations(||field.advance(source.borrow(),4096,g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(field.ready());let d=field.take_output_demand().unwrap();assert_eq!(d,RetirementDemand{depth:1,..Default::default()});let g=funded(d);
 for denied in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{let(result,heap)=observe_retirement_allocations(||field.take_output(denied).unwrap());assert!(result.is_none());assert_eq!(heap,(0,0));assert_eq!(field.take_output_demand().unwrap(),d);}
 let(((),receipt),heap)=observe_retirement_allocations(||field.take_output(g).unwrap().unwrap());assert!(receipt.fits(g));assert_eq!(heap,(0,0));field.begin_close();assert!(field.terminal_is_empty());drop(field);
 for _ in 0..128{if source.terminal_is_empty(){break;}let d=source.next_close_demand(4096).unwrap();let g=funded(d);assert_eq!(source.next_close_demand(g.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_retirement_allocations(||source.close_step(g).unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(source.terminal_is_empty());
}
