use super::*;
use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,controlled::ControlledRetirement}};
use std::{alloc::{GlobalAlloc,Layout,System},cell::Cell};

thread_local!{static ALLOCATION:Cell<Option<(usize,usize)>>=const{Cell::new(None)};}
struct LabelAllocator;
#[global_allocator]
static ALLOCATOR:LabelAllocator=LabelAllocator;
unsafe impl GlobalAlloc for LabelAllocator{
 unsafe fn alloc(&self,layout:Layout)->*mut u8{let pointer=unsafe{System.alloc(layout)};if !pointer.is_null(){let _=ALLOCATION.try_with(|value|if let Some((a,r))=value.get(){value.set(Some((a+layout.size(),r)));});}pointer}
 unsafe fn dealloc(&self,pointer:*mut u8,layout:Layout){let _=ALLOCATION.try_with(|value|if let Some((a,r))=value.get(){value.set(Some((a,r+layout.size())));});unsafe{System.dealloc(pointer,layout);}}
}
fn observed<T>(operation:impl FnOnce()->T)->(T,(usize,usize)){ALLOCATION.with(|value|{assert!(value.get().is_none());value.set(Some((0,0)));});let result=operation();let physical=ALLOCATION.with(|value|value.replace(None).unwrap());(result,physical)}
fn grant(owner:&ControlledRetirement<LocalizedLabel>)->RetainedCloneGrant{let copy=owner.next_copy_byte_demand().unwrap();RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}}
fn original(row:&serde_json::Value)->LocalizedLabel{
 let mut cells=std::array::from_fn(|_|std::array::from_fn(|_|Cow::Borrowed("")));
 for(index,input)in row["cells"].as_array().unwrap().iter().enumerate(){let text=input["text"].as_str().unwrap();cells[index/Locale::COUNT][index%Locale::COUNT]=if input["owned"]==true{let mut owned=String::with_capacity(input["capacity"].as_u64().unwrap()as usize);owned.push_str(text);Cow::Owned(owned)}else{Cow::Borrowed(match text{"English"=>"English","Deutsch"=>"Deutsch","Reuse"=>"Reuse","Wiederverwenden"=>"Wiederverwenden",""=>"",_=>panic!("undeclared borrowed fixture")})};}
 LocalizedLabel{cells}
}
#[test]
fn localized_label_original_backing_retirement_obeys_all_physical_grants(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🏷️label/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
 assert_eq!(Terminology::COUNT*Locale::COUNT,4);
 for row in law["cases"].as_array().unwrap(){
  let label=original(row);let oracle:LocalizedLabel=serde_json::from_value(serde_json::to_value(&label).unwrap()).unwrap();assert_eq!(label,oracle);
  let pointers:Vec<_>=label.cells.iter().flatten().map(|cell|cell.as_ptr()).collect();let original_bytes:usize=label.cells.iter().flatten().map(|cell|match cell{Cow::Owned(text)=>text.capacity(),_=>0}).sum();
  let(mut owner,physical)=observed(||ControlledRetirement::new(label).unwrap_or_else(|(error,_)|panic!("{error}")));assert_eq!(physical,(0,0));
  let mut born=0;let mut released=0;let mut denials=0;
  for _ in 0..1000{
   if owner.terminal_is_empty(){break;}
   let funding=grant(&owner);let(zero,physical)=observed(||owner.step(RetainedCloneGrant{maximum_items:0,..funding}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!(physical,(0,0));
   if let Some(label)=owner.original(){for(cell,pointer)in label.cells.iter().flatten().zip(&pointers){assert_eq!(cell.as_ptr(),*pointer);}assert_eq!(label,&oracle);}
   if funding.maximum_capacity_bytes>0{let(result,physical)=observed(||owner.step(RetainedCloneGrant{maximum_capacity_bytes:funding.maximum_capacity_bytes-1,..funding}));assert_eq!(physical,(0,0));assert!(result.is_err()||result.unwrap().progress()==RetainedCloneProgress::default());denials+=1;}
   if funding.maximum_release_bytes>0{let(result,physical)=observed(||owner.step(RetainedCloneGrant{maximum_release_bytes:funding.maximum_release_bytes-1,..funding}));assert_eq!(physical,(0,0));assert!(result.is_err()||result.unwrap().progress()==RetainedCloneProgress::default());denials+=1;}
   if funding.maximum_depth>0{let(result,physical)=observed(||owner.step(RetainedCloneGrant{maximum_depth:funding.maximum_depth-1,..funding}));assert_eq!(physical,(0,0));assert!(result.is_err()||result.unwrap().progress()==RetainedCloneProgress::default());denials+=1;}
   let(step,physical)=observed(||owner.step(funding).unwrap());let receipt=step.progress();assert!(receipt.fits(funding));assert_eq!(physical,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=physical.0;released+=physical.1;
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,original_bytes+born);let(_,physical)=observed(||drop(owner));assert_eq!(physical,(0,law["required"]["terminalDropBytes"].as_u64().unwrap()as usize));assert!(denials>0);
  eprintln!("[DEBUG] Localized label {} cells={} original={} born={} released={} terminalDrop=0",row["name"],pointers.len(),original_bytes,born,released);
 }
}

#[test]
fn localized_label_original_backing_direct_cursor_preserves_every_denied_cell(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🏷️label/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){
  let mut cursor=LocalizedLabelRetirement::new(original(row));
  for index in 0..Terminology::COUNT*Locale::COUNT{
   let pointer=cursor.current().unwrap().as_ptr();let capacity=cursor.next_close_byte_demand().unwrap();let funding=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:capacity,maximum_depth:1,..Default::default()};
   for denied in [RetainedCloneGrant{maximum_items:0,..funding},RetainedCloneGrant{maximum_depth:0,..funding}].into_iter().chain((capacity>0).then_some(RetainedCloneGrant{maximum_release_bytes:capacity.saturating_sub(1),..funding})){
    let(_,physical)=observed(||cursor.close_step(denied));assert_eq!(physical,(0,0));assert_eq!(cursor.index,index);assert_eq!(cursor.current().unwrap().as_ptr(),pointer);assert_eq!(cursor.next_close_byte_demand(),Some(capacity));
   }
   let(step,physical)=observed(||cursor.close_step(funding));assert_eq!(physical,(0,capacity));let RetirementStep::Progress(receipt)=step else{panic!("original cell must produce one physical receipt")};assert_eq!(receipt.copied_items,1);assert_eq!(receipt.released_bytes,capacity);assert!(receipt.fits(funding));
  }
  assert!(cursor.terminal_is_empty());let(_,physical)=observed(||drop(cursor));assert_eq!(physical,(0,0));
 }
}
