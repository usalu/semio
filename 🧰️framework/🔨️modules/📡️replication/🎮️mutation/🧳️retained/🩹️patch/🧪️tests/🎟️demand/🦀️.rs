use semio_framework_test_fixtures::fixture_text;
use super::*;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_value::retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect};
use std::{mem::size_of,sync::atomic::{AtomicUsize,Ordering}};
static VISITS:AtomicUsize=AtomicUsize::new(0);
static BODY:AtomicUsize=AtomicUsize::new(usize::MAX);
#[derive(semio_framework_value::RetireOwned)]struct QuoteRow{value:u64}
#[derive(Default)]struct QuotePatch{value:Option<u64>}
struct QuoteCursor{value:Option<QuotePatch>}
crate::retained_empty_option_patch!(QuotePatch{value});
impl RetireOwned for QuotePatch{
 type RetirementCursor=QuoteCursor;
 fn retirement_seed(self)->QuoteCursor{QuoteCursor{value:Some(self)}}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for QuoteCursor{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{VISITS.fetch_add(1,Ordering::Relaxed);BODY.store(body,Ordering::Relaxed);Ok(RetirementDemand{copy_bytes:if self.value.is_some(){2*size_of::<QuotePatch>()}else{0},depth:usize::from(self.value.is_some()),..Default::default()})}
 fn next_computational_work(&self,_:usize)->Result<u64,ValueError>{Ok(u64::from(self.value.is_some()))}
 fn close_into<'r>(&mut self,_:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{let g=effect.grant();let d=self.next_demand(g.maximum_copy_bytes)?;if !retained_mutation_demand_fits(d,g){return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}assert!(effect.performed(1)?);self.value.take();Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},1))))}
 fn terminal_is_empty(&self)->bool{self.value.is_none()}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
crate::retained_row_patch!(pub quote_fields for QuotePatch=>QuoteRow{value:u64});

/// 🎟️ One original sparse-patch close quote visits its genuine retained child exactly once with the caller body span.
#[test]
fn retained_row_patch_close_quote_visits_original_child_once(){
 let f:serde_json::Value=serde_json::from_str(fixture_text!("../../🩹️patch/🧪️tests/🧫️fixtures/🎟️close/🔣️.json")).unwrap();let mut harness=Harness::new();let((mut owner,birth),h)=observe(||quote_fields::Fields::new(RetainedPatchSource::Inverse));assert_eq!(h,(0,0));assert_eq!(birth,quote_fields::Fields::new_work_demand().unwrap());let(start,_,_)=harness.native(grant(owner.begin_close_demand()),1,|e|owner.begin_close_paid(e));assert_eq!(start.work,1);start.result.unwrap();let mut reached=false;
 for _ in 0..64{VISITS.store(0,Ordering::Relaxed);let d=owner.close_demand(4096).unwrap();if VISITS.load(Ordering::Relaxed)>0{reached=true;break;}let g=grant(d);let u=owner.close_work_demand(4096).unwrap();let s=harness.turn(g,u,&mut|e,r,rec|owner.close_reply(None,4096,e,r,rec)).result.unwrap();assert!(s.progress().fits(g));}assert!(reached);
 for body in f["bodyHints"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;VISITS.store(0,Ordering::Relaxed);let(d,h)=observe(||owner.close_demand(body).unwrap());assert_eq!(h,(0,0));assert_eq!(VISITS.load(Ordering::Relaxed),f["visits"].as_u64().unwrap()as usize,"original sparse close recursively re-queried its child");assert_eq!(BODY.load(Ordering::Relaxed),body);assert!(d.depth>0);}
 while !owner.terminal_is_empty(){let g=grant(owner.close_demand(4096).unwrap());let u=owner.close_work_demand(4096).unwrap();let s=harness.turn(g,u,&mut|e,r,rec|owner.close_reply(None,4096,e,r,rec)).result.unwrap();assert!(s.progress().fits(g));}assert_eq!(observe(||drop(owner)).1,(0,0));
}
