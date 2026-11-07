//! 🧪️ A fallible authored mapping admits valid input and refuses marked input, retiring the received owner through funded closure either way.
use super::*;
use semio_framework_value_native_owned::{*,native_decoding::NativeDecodeControl};
use semio_framework_value_native_owned::retirement::{RetireOwned,controlled::ControlledRetirement};
use semio_framework_value_native_owned::retained_clone::{RetainedCloneRef,RetainedCloneSource};
use semio_framework_value_testing_admission as source_fixture;
struct Bank{remaining:RetainedCloneGrant,fuel:u64}
impl Bank{fn new()->Self{let f:serde_json::Value=serde_json::from_str(include_str!("../../🔄️map/🧪️tests/🎟️work/📐️.json")).unwrap();Self{remaining:RetainedCloneGrant{maximum_items:f["items"].as_u64().unwrap()as usize,maximum_copy_bytes:f["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:f["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:f["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:f["depth"].as_u64().unwrap()as usize},fuel:f["fuel"].as_u64().unwrap()}}fn offer(&self)->RetainedCloneGrant{assert!(self.fuel>0);RetainedCloneGrant{maximum_items:self.remaining.maximum_items.min(1),maximum_copy_bytes:self.remaining.maximum_copy_bytes.min(4096),..self.remaining}}fn quoted(&self,d:RetirementDemand)->RetainedCloneGrant{let loan=self.offer();assert!(semio_framework_value_native_owned::retained_clone::retained_clone_grant_admits(loan,d));RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth,..loan}}fn post(&mut self,p:RetainedCloneProgress,work:u64){assert!(p.fits(self.remaining));self.remaining.maximum_items-=p.copied_items;self.remaining.maximum_copy_bytes-=p.copied_bytes;self.remaining.maximum_capacity_bytes-=p.retained_capacity_bytes;self.remaining.maximum_release_bytes-=p.released_bytes;self.fuel=self.fuel.checked_sub(work).unwrap();}}
fn actual<R>(run:impl FnOnce()->R)->(R,(usize,usize)){semio_framework_value_testing_allocator::observe_retirement_allocations(run)}
fn check(p:RetainedCloneProgress,h:(usize,usize),g:RetainedCloneGrant){assert!(p.fits(g));assert_eq!((p.retained_capacity_bytes,p.released_bytes),h)}
fn drain<T:RetireOwned>(value:T,bank:&mut Bank){let mut close=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();for _ in 0..100000{if close.terminal_is_empty(){return}let g=bank.quoted(close.next_demand(4096).unwrap());let(step,h)=actual(||close.step(g));let work=close.last_turn_work();let p=step.unwrap().progress();check(p,h,g);bank.post(p,work);}panic!("original native map cleanup stalled")}
#[derive(semio_framework_value_derive::RetireOwned)]#[value(crate="semio_framework_value_native_owned")]
struct Checked(usize);
impl NativeReceiveTryMap<String> for Checked{fn try_from_native(v:&String,c:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{c.checkpoint()?;if v.contains('X'){return Err(ValueError::literal(semio_framework_value_native_owned::ValueRefusalKind::InvalidValue,"refused"))}Ok(Self(v.len()))}}
fn run(text:String){{
 let mut bank=Bank::new();let mut source=source_fixture::capture(DslValue::String(text.clone()),&mut bank,Bank::offer,|bank,p,work,h,g|{check(p,h,g);bank.post(p,work)});
 let mut receiver=NativeTryMappedReceiver::<DslValue,String,Checked>::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(1<<24,&mut callback);let mut refused=false;
 for _ in 0..100000{if receiver.output_ready(){break}let g=bank.quoted(receiver.demands(source.borrow(),4096).unwrap());let(turn,h)=actual(||receiver.advance(source.borrow(),&mut control,4096,g));let work=turn.work;match turn.result{Ok(s)=>{check(s.progress(),h,g);bank.post(s.progress(),work)}Err(_)=>{refused=true;break}}}
 assert_eq!(refused,text.contains('X'));
 if !refused{let g=bank.quoted(receiver.take_demand().unwrap());let(output,p)=receiver.take(g).result.unwrap().unwrap();bank.post(p,1);assert_eq!(output.0,text.len());drain(output,&mut bank);}
 receiver.begin_close();for _ in 0..100000{if receiver.terminal_is_empty(){break}let g=bank.quoted(receiver.close_demands(4096).unwrap());let(turn,h)=actual(||receiver.close_step(g));let p=turn.result.unwrap().progress();check(p,h,g);bank.post(p,turn.work);}assert!(receiver.terminal_is_empty());drop(receiver);
 for _ in 0..100000{if source.terminal_is_empty(){break}let g=bank.quoted(source.next_close_demand(4096).unwrap());let(turn,h)=actual(||source.close_step(g));let p=turn.result.unwrap().progress();check(p,h,g);bank.post(p,turn.work);}assert!(source.terminal_is_empty());}}
#[test]fn fallible_mapping_admits_valid_text(){run("λ🙂".repeat(2049))}
#[test]fn fallible_mapping_refuses_marked_text_with_funded_cleanup(){run("λ🙂X".repeat(3))}
