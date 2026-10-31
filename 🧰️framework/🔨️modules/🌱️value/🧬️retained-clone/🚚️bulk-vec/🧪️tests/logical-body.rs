//! 📦️ Public bulk body laws preserve actual funding through all custody frontiers.
use semio_framework_value_retained_clone::*;
use semio_framework_value_testing_allocator::observe_retirement_allocations as heap;
use std::{fmt::Debug,mem::size_of};
use semio_framework_value_derive::{RetainedClone,RetireOwned};
struct Report{turns:usize,progress:Vec<RetainedCloneProgress>,binding:usize}
struct Bank{left:RetainedCloneGrant}
impl Bank{
 fn new(law:&serde_json::Value)->Self{let s=&law["stock"];Self{left:RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:s["release"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize}}}
 fn offer(&self,d:RetirementDemand)->RetainedCloneGrant{let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(g.maximum_items<=self.left.maximum_items&&g.maximum_copy_bytes<=self.left.maximum_copy_bytes&&g.maximum_capacity_bytes<=self.left.maximum_capacity_bytes&&g.maximum_release_bytes<=self.left.maximum_release_bytes&&g.maximum_depth<=self.left.maximum_depth);g}
 fn debit(&mut self,w:u64,p:RetainedCloneProgress,h:(usize,usize)){assert!(w<=self.left.maximum_items as u64);assert!(p.fits(self.left));assert_eq!(h,(p.retained_capacity_bytes,p.released_bytes));self.left.maximum_items-=w as usize;self.left.maximum_copy_bytes-=p.copied_bytes;self.left.maximum_capacity_bytes-=p.retained_capacity_bytes;self.left.maximum_release_bytes-=p.released_bytes;}
}
fn funded_drive<T:RetainedClone+Clone+PartialEq+Debug>(input:T,body:usize,law:&serde_json::Value,encode:impl Fn(&T)->serde_json::Value)->Report{
  let expected=input.clone();let encoded=encode(&expected);let mut bank=Bank::new(law);let limit=law["maximumTurns"].as_u64().unwrap();
  let mut admission=RetainedCloneSource::admission_owned(input,());let mut published=None;for _ in 0..limit{if admission.terminal_is_empty(){break}let g=bank.offer(admission.next_demand().unwrap());let(turn,h)=heap(||if admission.phase()=="ready"{admission.take_output(&mut published,g)}else{admission.advance(g)});let p=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};bank.debit(turn.work,p,h);turn.result.unwrap();}assert!(admission.terminal_is_empty());let mut source=published.expect("same-bank staged bulk source publication");let original=source.borrow().get()as*const T;let binding=source.borrow().binding_copy_bytes();let mut progress=Vec::new();
  let mut cursor=T::retained_clone_cursor();let mut output=None;let mut advance_turns=0;
  for turn in 0..limit{
   let d=cursor.advance_demands(source.borrow(),body).unwrap();let g=bank.offer(d);let denied=RetainedCloneGrant{maximum_items:0,..g};
   let(result,h)=heap(||cursor.advance(source.borrow(),body,denied));assert_eq!(result.work,law["deniedItemsWork"].as_u64().unwrap());assert_eq!(result.result.unwrap().progress(),Default::default());assert_eq!(h,(0,0));assert_eq!(cursor.advance_demands(source.borrow(),body).unwrap(),d);
   let(result,h)=heap(||cursor.advance(source.borrow(),body,g));advance_turns+=1;let w=result.work;let step=result.result.unwrap();let p=step.progress();progress.push(p);assert!(w<=g.maximum_items as u64);assert!(p.fits(g));assert!(p.copied_bytes<=d.copy_bytes);bank.debit(w,p,h);
   assert_eq!(source.borrow().get()as*const T,original);
   if matches!(step,RetainedCloneStep::Complete(_)){let g=bank.offer(cursor.take_demand().unwrap());let(result,h)=heap(||cursor.take(g));let w=result.work;let(value,p)=result.result.unwrap().unwrap();output=Some(value);bank.debit(w,p,h);break}assert!(turn<limit-1);
  }
  assert_eq!(output.as_ref(),Some(&expected));assert_eq!(encode(output.as_ref().unwrap()),encoded);assert_eq!(source.borrow().get(),&expected);cursor.begin_close();
  for turn in 0..limit{if cursor.terminal_is_empty(){break}let d=cursor.close_demands(body).unwrap();let g=bank.offer(d);let(result,h)=heap(||cursor.close_step(g));let w=result.work;let p=result.result.unwrap().progress();assert!(w<=g.maximum_items as u64);assert!(p.fits(g));bank.debit(w,p,h);assert!(turn<limit-1)}
  assert!(cursor.terminal_is_empty());assert_eq!(heap(||drop(cursor)).1,(0,0));
  let mut output=output.take();let mut close=RetainedCloneClose::default();
  for turn in 0..limit{if output.is_none()&&close.is_empty(){break}let d=close.next_owner_demands::<T>(output.is_some(),&None,body).unwrap();let g=bank.offer(d);let(result,h)=heap(||{if !close.is_empty(){close.step_granted(g)}else{let t=close.begin_granted(&mut output,g);RetainedWorkOutcome{work:t.work,result:t.result.map(|v|v.unwrap())}}});let w=result.work;let p=result.result.unwrap().progress();assert!(w<=g.maximum_items as u64);assert!(p.fits(g));bank.debit(w,p,h);assert!(turn<limit-1)}
  assert!(output.is_none()&&close.is_empty());assert_eq!(heap(||drop(close)).1,(0,0));
  for turn in 0..limit{if source.terminal_is_empty(){break}let d=source.next_close_demand(body).unwrap();let g=bank.offer(d);let(result,h)=heap(||source.close_step(g));let w=result.work;let p=result.result.unwrap().progress();assert!(w<=g.maximum_items as u64);assert!(p.fits(g));bank.debit(w,p,h);assert!(turn<limit-1)}
  assert!(source.terminal_is_empty());assert_eq!(heap(||drop(source)).1,(0,0));Report{turns:advance_turns,progress,binding}
}
#[test]
fn bulk_clone_logical_body_survives_expanded_physical_headers(){
 let law=funding_law();
 for body in law["bodies"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;funded_drive("α🙂abc".to_owned(),body,&law,|v|serde_json::to_value(v).unwrap());funded_drive((0..257u16).collect::<Vec<_>>(),body,&law,|v|serde_json::to_value(v).unwrap());funded_drive([37u8;257],body,&law,|v|serde_json::to_value(v.as_slice()).unwrap());}
}


fn law()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn number(value:&serde_json::Value,name:&str)->usize{value[name].as_u64().unwrap()as usize}
fn samples(count:usize)->Vec<u16>{let law=law();let(m,o,n)=(number(&law["sample"],"multiplier"),number(&law["sample"],"offset"),number(&law["sample"],"modulo"));(0..count).map(|i|((i*m+o)%n)as u16).collect()}
fn clone_equals<T:RetainedClone+Clone+PartialEq+Debug+serde::Serialize>(value:Vec<T>)->usize{let law=funding_law();funded_drive(value,1<<20,&law,|v|serde_json::to_value(v).unwrap()).turns}
fn clone_equals_wide<T:RetainedClone+Clone+PartialEq+Debug+serde::Serialize>(value:Vec<T>)->usize{let law=funding_law();funded_drive(value,1<<16,&law,|v|serde_json::to_value(v).unwrap()).turns}
#[test]
fn bulk_vector_clone_equals_the_clone_oracle_for_every_bitwise_element_shape() {
    let law = law();
    for count in law["sampleCounts"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        clone_equals(samples(count));
    }
    clone_equals((0..70_000u32).map(|index| index.wrapping_mul(2_654_435_761)).collect::<Vec<u32>>());
    clone_equals((0..9_000u64).map(|index| index.wrapping_mul(0x9E37_79B9_7F4A_7C15)).collect::<Vec<u64>>());
    clone_equals((0..300_000usize).map(|index| (index % 251) as u8).collect::<Vec<u8>>());
    clone_equals((0..4_000i128).map(|index| -index * 0x1_0000_0000_0001).collect::<Vec<i128>>());
    clone_equals((0..5_000).map(|index| f64::from(index) * 0.25 - 7.0).collect::<Vec<f64>>());
    clone_equals((0..20_000).map(|index| index % 3 == 0).collect::<Vec<bool>>());
    clone_equals((0..9_000u32).filter_map(|index| char::from_u32(index + 0x400)).collect::<Vec<char>>());
    clone_equals((0..30_000u32).map(|index| [index as u8, (index >> 8) as u8, (index >> 16) as u8, 7]).collect::<Vec<[u8; 4]>>());
    clone_equals((0..9_000u16).map(|index| [[index, index ^ 1, index ^ 2], [index ^ 3, index ^ 4, index ^ 5]]).collect::<Vec<[[u16; 3]; 2]>>());
    assert_eq!(clone_equals(vec![(); number(&law,"zeroSizedSampleCount")]), number(&law,"overheadTurns") + number(&law,"zeroSizedSampleCount").div_ceil(number(&law,"maximumZeroSizedElementsPerTurn")));
    assert_eq!(clone_equals(Vec::<u16>::new()), 3);
}

#[test]
fn non_bitwise_vector_clones_keep_the_per_element_path() {
    assert!(!String::BITWISE && !<Vec<u8>>::BITWISE && !<Option<u8>>::BITWISE);
    let strings: Vec<String> = (0..40).map(|index| format!("row-{index}")).collect();
    let turns = clone_equals_wide(strings);
    assert!(turns > 40 * 3, "elements clone one by one: {turns}");
    clone_equals_wide((0..16).map(|index| (0..index * 300).map(|value| value as u8).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>());
    clone_equals_wide((0..64u32).map(|index| (index % 4 != 0).then_some(index)).collect::<Vec<Option<u32>>>());
}

fn funding_law()->serde_json::Value{let law:serde_json::Value=serde_json::from_str(include_str!("logical-body.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("logical-body.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}law}

fn ownership_copy()->usize{let law:serde_json::Value=serde_json::from_str(include_str!("../../🧬️contract/📜️clone-and-cursor-traits/🧪️tests/take.json")).unwrap();number(&law,"ownershipCopyBytes")}
#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, RetainedClone, RetireOwned)]
#[value(crate="semio_framework_value_retained_clone")]
#[retained_clone(bitwise)]
struct Texel { red: u32, green: u32, blue: u32, alpha: u32, reserved: u32 }

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize, RetainedClone, RetireOwned)]
#[value(crate="semio_framework_value_retained_clone")]
struct PlainTexel { red: u32, green: u32 }


#[test]
fn derived_bitwise_structs_clone_in_pages_and_plain_structs_stay_elementwise() {
    assert!(Texel::BITWISE && <[Texel; 3]>::BITWISE && !PlainTexel::BITWISE);
    let texels: Vec<Texel> = (0..5_000u32).map(|index| Texel { red: index, green: index ^ 7, blue: index.wrapping_mul(3), alpha: 255, reserved: 0 }).collect();
    let page_elements = RETAINED_CLONE_BULK_PAGE_BYTES / size_of::<Texel>();
    let profile=funding_law();let report=funded_drive(texels.clone(),1<<20,&profile,|v|{assert_eq!(v,&texels);serde_json::to_value(v).unwrap()});let turns=&report.progress;
    assert_eq!(turns.len(), number(&law(),"overheadTurns") + texels.len().div_ceil(page_elements));
    assert_eq!(turns.iter().map(|turn| turn.copied_bytes).sum::<usize>(), report.binding + texels.len() * size_of::<Texel>() + ownership_copy());
    let plain: Vec<PlainTexel> = (0..40u32).map(|index| PlainTexel { red: index, green: index + 1 }).collect();
    assert!(clone_equals_wide(plain) > 40 * 3);
}
#[test]
fn bulk_run_elements_obeys_every_grant_axis() {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
    assert_eq!(bulk_run_elements(grant, 2), RETAINED_CLONE_BULK_PAGE_BYTES / 2);
    assert_eq!(bulk_run_elements(RetainedCloneGrant { maximum_copy_bytes: 1001, ..grant }, 2), 500);
    assert_eq!(bulk_run_elements(RetainedCloneGrant { maximum_copy_bytes: 1, ..grant }, 2), 0);
    assert_eq!(bulk_run_elements(RetainedCloneGrant { maximum_items: 0, ..grant }, 2), 0);
    assert_eq!(bulk_run_elements(grant, 0), number(&law(),"maximumZeroSizedElementsPerTurn"));
    for row in law()["grantCases"].as_array().unwrap() {
        let grant=RetainedCloneGrant {maximum_items:number(row,"items"),maximum_copy_bytes:number(row,"copyBytes"),maximum_depth:1,..Default::default()};
        assert_eq!(bulk_run_elements(grant,number(row,"width")),number(row,"elements"));
    }
    assert_eq!(bulk_run_progress(500, 2), RetainedCloneProgress { copied_items: 1, copied_bytes: 1000, ..Default::default() });
}
