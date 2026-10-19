use super::*;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:d.depth}}
/// 🧪️ Exact denied pages and original scalar buffers obey the independent portable byte corpus.
#[test]
fn retained_slice_equality_matches_portable_bytes_without_allocations(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for law in corpus["cases"].as_array().unwrap(){
  let left:Vec<u8>=law["left"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let right:Vec<u8>=law["right"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect();let pointers=(left.as_ptr(),right.as_ptr());
  let(mut cursor,heap)=crate::observe_retirement_allocations(RetainedSliceEquality::new);assert_eq!(heap,(0,0));let mut turns=0;
  while !cursor.ready(){
   let demand=cursor.demand(&left,&right,RetainedSliceEquality::fixed_copy_bytes()+2).unwrap();let funded=grant(demand);let before=cursor;
   for denied in[RetainedCloneGrant{maximum_items:0,..funded},RetainedCloneGrant{maximum_copy_bytes:RetainedSliceEquality::fixed_copy_bytes()-1,..funded},RetainedCloneGrant{maximum_depth:0,..funded}]{let(step,heap)=crate::observe_retirement_allocations(||cursor.advance(&left,&right,denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(cursor,before);}
   let(step,heap)=crate::observe_retirement_allocations(||cursor.advance(&left,&right,funded).unwrap());assert_eq!(heap,(0,0));assert!(step.progress().copied_bytes<=funded.maximum_copy_bytes);turns+=1;assert!(turns<left.len()+4);
  }
  assert_eq!(cursor.equal(),law["expected"].as_bool());assert_eq!((left.as_ptr(),right.as_ptr()),pointers);assert_eq!(crate::observe_retirement_allocations(||std::hint::black_box(cursor)).1,(0,0));
 }
 let mut cursor=RetainedSliceEquality::new();let demand=cursor.demand(&[1u8],&[1u8],0).unwrap();assert!(demand.copy_bytes>RetainedSliceEquality::fixed_copy_bytes());let p=cursor.advance(&[1u8],&[1u8],RetainedCloneGrant{maximum_copy_bytes:RetainedSliceEquality::fixed_copy_bytes(),..grant(demand)}).unwrap().progress();assert_eq!(p,RetainedCloneProgress::default());assert!(!cursor.ready());
 eprintln!("[DEBUG] eight shared slice equality laws match neutral SQLite-validated identities; denied item/copy/depth pages preserve state and native constructor/comparison perform zero heap activity without retaining original payload");
}
#[derive(Copy)]
struct InlinePoint{coordinates:[f64;3]}
impl Clone for InlinePoint{fn clone(&self)->Self{panic!("scalar comparison must never clone payload")}}
impl RetainedEqualityElement for InlinePoint{fn inline_eq(a:&Self,b:&Self)->bool{a.coordinates[0]==b.coordinates[0]&&a.coordinates[1]==b.coordinates[1]&&a.coordinates[2]==b.coordinates[2]}}
#[derive(Copy)]
struct Empty;
impl Clone for Empty{fn clone(&self)->Self{panic!("empty scalar comparison must never clone")}}
impl RetainedEqualityElement for Empty{fn inline_eq(_: &Self,_:&Self)->bool{true}}
/// ✂️ Wide inline point/byte buffers and zero-sized elements yield without retaining payload or invoking Clone.
#[test]
fn retained_slice_equality_wide_typed_pages_and_cancellation_are_bounded(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let length=corpus["wide"]["length"].as_u64().unwrap()as usize;let left=vec![0u8;length];let mut right=vec![0u8;length];right[length-1]=1;let pointers=(left.as_ptr(),right.as_ptr());
 for cut in[Some(0),Some(1),Some(7),Some(128),Some(25000),None]{
  let mut cursor=RetainedSliceEquality::new();let mut steps=0;while !cursor.ready()&&cut.is_none_or(|cut|steps<cut){let d=cursor.demand(&left,&right,RetainedSliceEquality::fixed_copy_bytes()+6).unwrap();let(step,heap)=crate::observe_retirement_allocations(||cursor.advance(&left,&right,grant(d)).unwrap());assert_eq!(heap,(0,0));assert!(step.progress().copied_bytes<=d.copy_bytes);steps+=1;assert!(steps<length+1);}
  if cut.is_none(){assert_eq!(cursor.equal(),Some(false));}assert_eq!((left.as_ptr(),right.as_ptr()),pointers);assert_eq!(crate::observe_retirement_allocations(||std::hint::black_box(cursor)).1,(0,0));
 }
 let left=[InlinePoint{coordinates:[1.0,2.0,3.0]},InlinePoint{coordinates:[4.0,5.0,6.0]}];let right=[InlinePoint{coordinates:[1.0,2.0,3.0]},InlinePoint{coordinates:[4.0,5.0,7.0]}];let mut cursor=RetainedSliceEquality::new();let mut turns=0;
 while !cursor.ready(){let d=cursor.demand(&left,&right,RetainedSliceEquality::fixed_copy_bytes()+size_of::<InlinePoint>()*2).unwrap();let(step,heap)=crate::observe_retirement_allocations(||cursor.advance(&left,&right,grant(d)).unwrap());assert_eq!(heap,(0,0));assert!(step.progress().copied_bytes<=d.copy_bytes);turns+=1;}assert_eq!(turns,2);assert_eq!(cursor.equal(),Some(false));
 let empty=[Empty;33];let mut cursor=RetainedSliceEquality::new();let mut turns=0;while !cursor.ready(){let before=cursor.offset;let d=cursor.demand(&empty,&empty,0).unwrap();let(_,heap)=crate::observe_retirement_allocations(||cursor.advance(&empty,&empty,grant(d)).unwrap());assert_eq!(heap,(0,0));assert_eq!(cursor.offset-before,1);turns+=1;}assert_eq!(turns,33);assert_eq!(cursor.equal(),Some(true));
 eprintln!("[DEBUG] actual65537-byte borrowed comparisons yield3-byte pages at five cancellation cuts plus completion; fixed inline point equality never calls Clone and33 zero-sized elements each consume a separate funded turn");
}
