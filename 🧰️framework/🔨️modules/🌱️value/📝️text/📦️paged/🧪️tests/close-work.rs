//! 🧪️ Concrete byte pops and physical releases retain their actual CPU through natural retirement.
use super::*;
use crate::observe_retirement_allocations as heap;
struct Bank{work:u64,copy:usize,capacity:usize,release:usize,actual:Option<usize>}
impl Bank{
 fn new(law:&serde_json::Value)->Self{let s=&law["stock"];Self{work:s["work"].as_u64().unwrap(),copy:s["copy"].as_u64().unwrap()as usize,capacity:s["capacity"].as_u64().unwrap()as usize,release:s["release"].as_u64().unwrap()as usize,actual:None}}
 fn effect<R>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut crate::RetainedAllocationEffect<'_>)->R)->R{assert!(self.actual.is_none());let result=run(&mut crate::RetainedAllocationEffect::new(grant,self.work,&mut self.actual));assert!(self.actual.is_none());result}
 fn debit(&mut self,w:u64,p:RetainedCloneProgress,h:(usize,usize)){assert!(w<=self.work&&p.copied_bytes<=self.copy&&p.retained_capacity_bytes<=self.capacity&&p.released_bytes<=self.release);assert_eq!(h,(p.retained_capacity_bytes,p.released_bytes));self.work-=w;self.copy-=p.copied_bytes;self.capacity-=p.retained_capacity_bytes;self.release-=p.released_bytes;}
}
fn law()->serde_json::Value{let law:serde_json::Value=serde_json::from_str(include_str!("close-work.json")).unwrap();let schema:serde_json::Value=serde_json::from_str(include_str!("close-work.schema.json")).unwrap();for(k,v)in law.as_object().unwrap(){assert_eq!(&schema["properties"][k]["const"],v)}law}
fn text(length:usize,byte:u8,bank:&mut Bank)->PagedText<4097>{let mut owner=PagedText::empty();while owner.bytes.capacity()<length{let n=owner.bytes.next_exact_capacity_allocation_bytes(length).unwrap().unwrap();assert!(n<=bank.capacity);assert!(bank.actual.is_none());let(result,h)=heap(||owner.bytes.reserve_exact_capacity_one(length,n,&mut bank.actual));let p=result.unwrap();assert_eq!(bank.actual.take(),Some(p.allocated_bytes));bank.debit(p.work,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:p.allocated_bytes,..Default::default()},h);}for _ in 0..length{let(result,h)=heap(||owner.bytes.push_reserved(byte));result.unwrap();bank.debit(1,RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()},h);}owner}
#[test]
fn paged_text_close_returns_actual_byte_and_release_work_from_one_finite_bank(){
 let law=law();for length in law["lengths"].as_array().unwrap(){let length=length.as_u64().unwrap()as usize;for body in law["copyBodies"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;for items in law["itemOffers"].as_array().unwrap(){let items=items.as_u64().unwrap()as usize;let mut bank=Bank::new(&law);let mut owner=text(length,law["byte"].as_u64().unwrap()as u8,&mut bank);let actual:Vec<u8>=owner.bytes.iter().copied().collect();assert_eq!(serde_json::to_value(&actual).unwrap(),serde_json::json!(vec![120u8;length]));
  for turn in 0..law["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){break}let d=owner.next_demand(body).unwrap();let g=RetainedCloneGrant{maximum_items:items.min(bank.work as usize),maximum_copy_bytes:body.max(d.copy_bytes).min(bank.copy),maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..Default::default()};let before=(owner.bytes.len(),owner.allocated_bytes(),owner.bytes.backing_ptr(0));
   let denied=[Some(RetainedCloneGrant{maximum_items:0,..g}),d.copy_bytes.checked_sub(1).map(|maximum_copy_bytes|RetainedCloneGrant{maximum_copy_bytes,..g}),d.release_bytes.checked_sub(1).map(|maximum_release_bytes|RetainedCloneGrant{maximum_release_bytes,..g}),Some(RetainedCloneGrant{maximum_depth:0,..g})];
   for denied in denied.into_iter().flatten(){let(result,h)=heap(||bank.effect(denied,|effect|owner.close_step(effect)));assert_eq!(result.work,law["deniedWork"].as_u64().unwrap());assert_eq!(h,(0,0));match result.result{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>{assert_eq!(denied.maximum_depth,0);assert_eq!(error.retained_progress(),Default::default())}}assert_eq!((owner.bytes.len(),owner.allocated_bytes(),owner.bytes.backing_ptr(0)),before);}
   let(result,h)=heap(||bank.effect(g,|effect|owner.close_step(effect)));let w=result.work;let p=result.result.unwrap().progress();assert!(w<=g.maximum_items as u64);assert!(p.fits(g));let removed=before.0-owner.bytes.len();if removed>0{assert_eq!(w,removed as u64);assert_eq!(p.copied_bytes,removed)}bank.debit(w,p,h);assert!(turn<16383);
  }
  assert!(owner.terminal_is_empty());let cached=bank.effect(Default::default(),|effect|owner.close_step(effect));assert_eq!(cached.work,law["cachedWork"].as_u64().unwrap());assert_eq!(cached.result.unwrap().progress(),Default::default());assert_eq!(heap(||drop(owner)).1,(0,0));
 }}}
}
#[test]
fn paged_text_natural_wrapper_forwards_direct_work_before_result_matching(){
 use crate::retirement::RetirementCursor;
 let law=law();let mut bank=Bank::new(&law);let mut owner=PagedTextRetirement{owner:text(7,120,&mut bank)};let g=RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:7,maximum_depth:1,..Default::default()};let(step,h)=heap(||bank.effect(g,|effect|RetirementCursor::close_step(&mut owner,effect)));let w=step.work();assert_eq!(w,law["naturalFirstPayloadWork"].as_u64().unwrap());match step{crate::retirement::RetirementStep::Progress(p,_)=>bank.debit(w,p,h),_=>panic!("natural byte progress was discarded")}
 for _ in 0..16384{if owner.terminal_is_empty(){break}let d=owner.next_demand(3).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..Default::default()};let(step,h)=heap(||bank.effect(g,|effect|RetirementCursor::close_step(&mut owner,effect)));let w=step.work();match step{crate::retirement::RetirementStep::Progress(p,_)=>bank.debit(w,p,h),crate::retirement::RetirementStep::Complete(_)=>bank.debit(w,Default::default(),h),_=>panic!("unexpected natural cursor result")}}assert!(owner.terminal_is_empty());
}

#[test]
fn original_formatter_fragments_match_independent_unicode_and_extent_fixture(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../📋️list/🧪️tests/🧫️fixtures/🏷️formatted-exact-extent.json")).unwrap();let body="x".repeat(law["payloadBytes"].as_u64().unwrap()as usize);
 for(prefix,suffix,key)in[("Set label to ","","expectedEnBytes"),("Beschriftung auf "," setzen","expectedDeBytes")]{let formatter=FormatterRead{prefix,body:&body,suffix,body_len:body.len(),total:prefix.len()+body.len()+suffix.len()};let bytes=(0..formatter.byte_len()).map(|index|formatter.byte_at(index).unwrap()).collect::<Vec<_>>();assert_eq!(bytes.len(),law[key].as_u64().unwrap()as usize);assert_eq!(bytes,format!("{prefix}{body}{suffix}").into_bytes());assert!(formatter.byte_at(bytes.len()).is_none());}
}
