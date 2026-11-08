use semio_framework_test_fixtures::fixture_text;
use super::*;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:d.depth}}
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
fn test_bank(turns:usize)->EffectBank{let turns=turns.checked_add(1).unwrap();let work=u64::try_from(turns).unwrap().checked_mul(4096).unwrap();let stock=RetainedCloneGrant{maximum_items:turns,maximum_copy_bytes:turns.checked_mul(8192).unwrap(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};EffectBank::new(Wallet::new(stock,work))}
fn leaf<T:RetainedEqualityElement>(bank:&mut EffectBank,cursor:&mut RetainedSliceEquality,left:&[T],right:&[T],body:usize,maximum:u64,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let(poll,actual)=bank.run(grant,maximum,|effect|{let before=effect.performed_work();let mut result=None;let reply=semio_framework_value_control_contract::OriginalEffectReply::borrow(&mut result).unwrap();drop(cursor.advance_into(left,right,body,maximum,effect,reply).unwrap());semio_framework_value_control_contract::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:result.take().unwrap(),work:effect.performed_work()-before})});assert_eq!(actual,None);let semio_framework_value_control_contract::RetainedWorkPoll::Complete(turn)=poll else{panic!("native equality leaf pending")};assert_ne!(bank.take_admitted(),Some(false));if let Ok(step)=&turn.result{bank.post(step.progress(),grant,(0,0))}turn}
fn native_step(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome)->crate::RetainedWorkOutcome{let(poll,actual)=bank.run(grant,maximum,|effect|semio_framework_value_control_contract::RetainedWorkPoll::Complete(operation(effect)));assert_eq!(actual,None);let semio_framework_value_control_contract::RetainedWorkPoll::Complete(turn)=poll else{panic!("native equality step pending")};assert_ne!(bank.take_admitted(),Some(false));bank.post(turn.result.as_ref().map(|step|step.progress()).unwrap_or_default(),grant,(0,0));turn}
fn native_verify(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<bool>>)->crate::RetainedWorkOutcome<Option<bool>>{let(poll,actual)=bank.run(grant,maximum,|effect|semio_framework_value_control_contract::RetainedWorkPoll::Complete(operation(effect)));assert_eq!(actual,None);let semio_framework_value_control_contract::RetainedWorkPoll::Complete(turn)=poll else{panic!("native verification pending")};assert_ne!(bank.take_admitted(),Some(false));bank.post(Default::default(),grant,(0,0));turn}
/// ⛽️ The actual compared prefix cannot exceed the original caller's selected work.
#[test]
fn retained_slice_equality_obeys_neutral_work_vectors(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/⛽️work.json")).unwrap();let mut bank=test_bank(rows.as_array().unwrap().len()*2);
 for row in rows.as_array().unwrap(){
  let left:Vec<u8>=row["left"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u8).collect();let right:Vec<u8>=row["right"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u8).collect();
  let mut cursor=RetainedSliceEquality::new();let maximum=row["maximumWork"].as_u64().unwrap();let body=row["payloadBodyBytes"].as_u64().unwrap()as usize;let demand=cursor.demand(&left,&right,body,maximum).unwrap();let quoted=cursor.work_demand(&left,&right,body,maximum).unwrap();let g=grant(demand);
  let (denied,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&left,&right,4096,maximum,RetainedCloneGrant{maximum_items:0,..g}));assert_eq!(denied.work,0);assert_eq!(heap,(0,0));
  let (turn,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&left,&right,body,maximum,g));assert_eq!(heap,(0,0));assert!(turn.work<=u128::from(quoted));assert_eq!(turn.work,u128::from(row["work"].as_u64().unwrap()));let step=turn.result.unwrap();assert!(step.progress().fits(g));assert_eq!(cursor.ready(),row["ready"].as_bool().unwrap());assert_eq!(cursor.equal(),row["equal"].as_bool());
 }
}
/// 🧪️ Exact denied pages and original scalar buffers obey the independent portable byte corpus.
#[test]
fn retained_slice_equality_matches_portable_bytes_without_allocations(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../../../📋️list/🟰️equal/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let turns=corpus["cases"].as_array().unwrap().iter().map(|row|4*(row["left"].as_array().unwrap().len()+4)).sum::<usize>()+1;let mut bank=test_bank(turns);
 for law in corpus["cases"].as_array().unwrap(){
  let left:Vec<u8>=law["left"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let right:Vec<u8>=law["right"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let pointers=(left.as_ptr(),right.as_ptr());
  let(mut cursor,heap)=crate::observe_retirement_allocations(RetainedSliceEquality::new);assert_eq!(heap,(0,0));let mut turns=0;
  while !cursor.ready(){
   let body=2;let demand=cursor.demand(&left,&right,body,4096).unwrap();let funded=grant(demand);let before=cursor;
   for denied in[Some(RetainedCloneGrant{maximum_items:0,..funded}),demand.copy_bytes.checked_sub(1).map(|maximum_copy_bytes|RetainedCloneGrant{maximum_copy_bytes,..funded}),Some(RetainedCloneGrant{maximum_depth:0,..funded})].into_iter().flatten(){let(step,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&left,&right,body,4096,denied).result.unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(cursor,before);}
   let(step,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&left,&right,body,4096,funded).result.unwrap());assert_eq!(heap,(0,0));assert!(step.progress().copied_bytes<=funded.maximum_copy_bytes);turns+=1;assert!(turns<left.len()+4);
  }
  assert_eq!(cursor.equal(),law["expected"].as_bool());assert_eq!((left.as_ptr(),right.as_ptr()),pointers);assert_eq!(crate::observe_retirement_allocations(||std::hint::black_box(cursor)).1,(0,0));
 }
 let mut cursor=RetainedSliceEquality::new();let demand=cursor.demand(&[1u8],&[1u8],2,4096).unwrap();assert!(demand.copy_bytes>RetainedSliceEquality::fixed_copy_bytes());let p=leaf(&mut bank,&mut cursor,&[1u8],&[1u8],2,4096,RetainedCloneGrant{maximum_copy_bytes:RetainedSliceEquality::fixed_copy_bytes(),..grant(demand)}).result.unwrap().progress();assert_eq!(p,RetainedCloneProgress::default());assert!(!cursor.ready());
}
#[derive(Copy)]
struct InlinePoint{coordinates:[f64;3]}
impl Clone for InlinePoint{fn clone(&self)->Self{panic!("scalar comparison must never clone payload")}}
unsafe impl RetainedEqualityElement for InlinePoint{const WORK:u64=3;fn inline_eq(a:&Self,b:&Self)->bool{(a.coordinates[0]==b.coordinates[0])&(a.coordinates[1]==b.coordinates[1])&(a.coordinates[2]==b.coordinates[2])}}
#[derive(Copy)]
struct Empty;
impl Clone for Empty{fn clone(&self)->Self{panic!("empty scalar comparison must never clone")}}
unsafe impl RetainedEqualityElement for Empty{const WORK:u64=1;fn inline_eq(_: &Self,_:&Self)->bool{true}}
/// ✂️ Wide inline point/byte buffers and zero-sized elements yield without retaining payload or invoking Clone.
#[test]
fn retained_slice_equality_wide_typed_pages_and_cancellation_are_bounded(){
 let corpus:serde_json::Value=serde_json::from_str(fixture_text!("../../../../📋️list/🟰️equal/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let length=corpus["wide"]["length"].as_u64().unwrap()as usize;let mut bank=test_bank(6*(length+1)+36);let left=vec![0u8;length];let mut right=vec![0u8;length];right[length-1]=1;let pointers=(left.as_ptr(),right.as_ptr());
 for cut in[Some(0),Some(1),Some(7),Some(128),Some(25000),None]{
  let mut cursor=RetainedSliceEquality::new();let mut steps=0;while !cursor.ready()&&cut.is_none_or(|cut|steps<cut){let body=6;let d=cursor.demand(&left,&right,body,4096).unwrap();let(step,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&left,&right,body,4096,grant(d)).result.unwrap());assert_eq!(heap,(0,0));assert!(step.progress().copied_bytes<=d.copy_bytes);steps+=1;assert!(steps<length+1);}
  if cut.is_none(){assert_eq!(cursor.equal(),Some(false));}assert_eq!((left.as_ptr(),right.as_ptr()),pointers);assert_eq!(crate::observe_retirement_allocations(||std::hint::black_box(cursor)).1,(0,0));
 }
 let left=[InlinePoint{coordinates:[1.0,2.0,3.0]},InlinePoint{coordinates:[4.0,5.0,6.0]}];let right=[InlinePoint{coordinates:[1.0,2.0,3.0]},InlinePoint{coordinates:[4.0,5.0,7.0]}];let mut cursor=RetainedSliceEquality::new();let mut turns=0;
 while !cursor.ready(){let body=size_of::<InlinePoint>()*2;let d=cursor.demand(&left,&right,body,4096).unwrap();let(step,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&left,&right,body,4096,grant(d)).result.unwrap());assert_eq!(heap,(0,0));assert!(step.progress().copied_bytes<=d.copy_bytes);turns+=1;}assert_eq!(turns,2);assert_eq!(cursor.equal(),Some(false));
 let empty=[Empty;33];let mut cursor=RetainedSliceEquality::new();let mut turns=0;while !cursor.ready(){let before=cursor.offset;let d=cursor.demand(&empty,&empty,0,4096).unwrap();let(_,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&empty,&empty,0,4096,grant(d)).result.unwrap());assert_eq!(heap,(0,0));assert_eq!(cursor.offset-before,1);turns+=1;}assert_eq!(turns,33);assert_eq!(cursor.equal(),Some(true));
}

/// 🔢️ Bounded inline scalar pages follow native IEEE equality without cloning or allocation.
#[test]
fn retained_floating_equality_matches_portable_ieee_words() {
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🟰️float.json")).unwrap();let mut bank=test_bank(corpus.as_array().unwrap().len());
 for case in corpus.as_array().unwrap(){
  let expected=case["expected"].as_bool().unwrap();
  if case["width"]==64 {
   let left=f64::from_bits(u64::from_str_radix(case["left"].as_str().unwrap(),16).unwrap());let right=f64::from_bits(u64::from_str_radix(case["right"].as_str().unwrap(),16).unwrap());
   assert_eq!(<f64 as RetainedEqualityElement>::inline_eq(&left,&right),expected);
   let mut cursor=RetainedSliceEquality::new();let d=cursor.demand(&[left],&[right],1024,4096).unwrap();let(_,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&[left],&[right],1024,4096,grant(d)).result.unwrap());assert_eq!(heap,(0,0));assert_eq!(cursor.equal(),Some(expected));
  } else {
   let left=f32::from_bits(u32::from_str_radix(case["left"].as_str().unwrap(),16).unwrap());let right=f32::from_bits(u32::from_str_radix(case["right"].as_str().unwrap(),16).unwrap());
   assert_eq!(<f32 as RetainedEqualityElement>::inline_eq(&left,&right),expected);
   let mut cursor=RetainedSliceEquality::new();let d=cursor.demand(&[left],&[right],1024,4096).unwrap();let(_,heap)=crate::observe_retirement_allocations(||leaf(&mut bank,&mut cursor,&[left],&[right],1024,4096,grant(d)).result.unwrap());assert_eq!(heap,(0,0));assert_eq!(cursor.equal(),Some(expected));
  }
 }
}

#[test]
fn retained_slice_header_inspection_obeys_neutral_admission(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔎️inspection.json")).unwrap();let mut bank=test_bank(cases.as_array().unwrap().len()*10);
 for case in cases.as_array().unwrap(){
  let left=[1u8,2];let right=[1u8,3];let mut stage=RetainedSliceInspection::new();let demand=stage.inspect_demand().unwrap();let mut g=grant(demand);if !case["physical"].as_bool().unwrap(){g.maximum_copy_bytes=0;}
  let denied=native_step(&mut bank,RetainedCloneGrant{maximum_depth:0,..grant(demand)},3,|effect|stage.inspect(&left,&right,effect));assert_eq!(denied.work,0);assert!(!stage.ready());let (turn,heap)=crate::observe_retirement_allocations(||native_step(&mut bank,g,case["work"].as_u64().unwrap(),|effect|stage.inspect(&left,&right,effect)));assert_eq!(heap,(0,0));
  let progress=turn.result.unwrap().progress();assert_eq!(stage.ready(),case["inspected"].as_bool().unwrap());assert_eq!(turn.work,if stage.ready(){3}else{0});assert!(progress.fits(g));
  if stage.ready(){assert_eq!(native_verify(&mut bank,grant(demand),2,|effect|stage.verify(&left,&right,effect)).work,0);let matched=native_verify(&mut bank,grant(demand),3,|effect|stage.verify(&left,&right,effect));assert_eq!(matched.work,3);assert_eq!(matched.result.unwrap(),Some(true));assert_eq!(native_verify(&mut bank,grant(demand),3,|effect|stage.verify(&right,&left,effect)).result.unwrap(),Some(false));}
  let start=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};native_step(&mut bank,grant(start),1,|effect|stage.begin_close(effect)).result.unwrap();while !stage.terminal_is_empty(){let d=stage.close_demand().unwrap();let g=grant(d);assert_eq!(native_step(&mut bank,g,0,|effect|stage.close_step(effect)).work,0);assert_eq!(native_step(&mut bank,RetainedCloneGrant{maximum_items:0,..g},1,|effect|stage.close_step(effect)).work,0);assert_eq!(native_step(&mut bank,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},1,|effect|stage.close_step(effect)).work,0);let turn=native_step(&mut bank,g,1,|effect|stage.close_step(effect));assert_eq!(turn.work,1);assert!(turn.result.unwrap().progress().fits(g));}
 }
}

#[test]
fn staged_slice_retains_and_closes_its_inspection(){
 let rows:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/⛽️work.json")).unwrap();let mut bank=test_bank(rows.as_array().unwrap().len()*9);
 for row in rows.as_array().unwrap(){
  let left:Vec<u8>=row["left"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let right:Vec<u8>=row["right"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let mut c=RetainedStagedSliceEquality::new();
  let g=grant(c.demand::<u8>(4096,4096).unwrap());assert_eq!(native_step(&mut bank,g,0,|effect|c.advance(&left,&right,4096,0,effect)).work,0);assert!(native_step(&mut bank,g,3,|effect|c.advance(&left,&right,4096,3,effect)).result.unwrap().progress().fits(g));
  let work=row["maximumWork"].as_u64().unwrap()+4;let body=row["payloadBodyBytes"].as_u64().unwrap()as usize;let d=c.demand::<u8>(body,work).unwrap();let quoted=c.work_demand::<u8>(body,work).unwrap();let g=grant(d);let denied=native_step(&mut bank,RetainedCloneGrant{maximum_items:0,..g},work,|effect|c.advance(&left,&right,body,work,effect));assert_eq!(denied.work,0);if quoted>0&&left.as_ptr()!=right.as_ptr(){let foreign=native_step(&mut bank,g,work,|effect|c.advance(&right,&left,body,work,effect));assert_eq!(foreign.work,3);assert!(foreign.result.is_err());}
  let turn=native_step(&mut bank,g,work,|effect|c.advance(&left,&right,body,work,effect));assert!(turn.work<=u128::from(quoted));assert!(turn.result.unwrap().progress().fits(g));assert_eq!(c.ready(),row["ready"].as_bool().unwrap());assert_eq!(c.equal(),row["equal"].as_bool());if row["work"].as_u64().unwrap()>0{assert_eq!(turn.work,u128::from(row["work"].as_u64().unwrap()+4));}else{assert_eq!(turn.work,0);}
  let start=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};native_step(&mut bank,grant(start),1,|effect|c.begin_close(effect)).result.unwrap();while !c.terminal_is_empty(){let g=grant(c.close_demand().unwrap());assert_eq!(native_step(&mut bank,g,0,|effect|c.close_step(effect)).work,0);let turn=native_step(&mut bank,g,1,|effect|c.close_step(effect));assert!(turn.result.unwrap().progress().fits(g));}
 }
}
