//! 🚚️ Bitwise vector clones move whole pages per turn: equal to the `Clone` oracle, O(bytes/page) turns, exact capacity accounting and leak-free cancellation at every turn.
use semio_framework_test_fixtures::fixture_text;
use super::*;
use crate::observe_retirement_allocations as observe;
use std::{fmt::Debug, ops::Deref};
#[path="../../🧬️contract/🔤️string-cursor/🧪️tests/producer.rs"] mod producer;

const CLOSE_COPY_BYTES: usize = RETAINED_CLONE_BULK_PAGE_BYTES;
fn assert_actual_step<E:Debug>(outcome:crate::RetainedWorkOutcome<RetainedCloneStep,E>)->RetainedCloneStep{let work=outcome.work;let step=outcome.result.unwrap_or_else(|error|panic!("retained work {work} failed: {error:?}"));assert!(step.progress()==RetainedCloneProgress::default()||work>0,"retained effects have no actual work");step}


struct Held<T: RetainedClone>(FixtureSource<Vec<T>>);

impl<T: RetainedClone> Held<T> {
    fn new(owner: Vec<T>) -> Self {
        Self(funded_source(owner))
    }
}

impl<T: RetainedClone> Deref for Held<T> {
    type Target = RetainedCloneSource<Vec<T>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T: RetainedClone> Drop for Held<T> {
 fn drop(&mut self){if std::thread::panicking(){return}for _ in 0..1_000_000{if self.0.terminal_is_empty(){return}let grant=self.0.wallet().remaining();match self.0.close_with(CLOSE_COPY_BYTES,grant).expect("actual bulk source close"){semio_framework_value_clone_source_testing::FixtureCloseStep::Progress(progress)=>assert!(progress.fits(grant)),semio_framework_value_clone_source_testing::FixtureCloseStep::Complete|semio_framework_value_clone_source_testing::FixtureCloseStep::Pending=>{},other=>panic!("bulk source action did not settle: {other:?}")}}panic!("vector source did not close")}
}

fn law() -> serde_json::Value { serde_json::from_str(fixture_text!("../../../🚚️bulk-vec/🧪️tests/🧫️fixtures/🔣️.json")).unwrap() }

fn number(value: &serde_json::Value, name: &str) -> usize { value[name].as_u64().unwrap() as usize }

fn source_stock()->(RetainedCloneGrant,u64){let law=law();let stock=&law["sourceStock"];(RetainedCloneGrant{maximum_items:number(stock,"maximumItems"),maximum_copy_bytes:number(stock,"maximumCopyBytes"),maximum_capacity_bytes:number(stock,"maximumCapacityBytes"),maximum_release_bytes:number(stock,"maximumReleaseBytes"),maximum_depth:number(stock,"maximumDepth")},stock["work"].as_u64().unwrap())}
fn funded_with<T:RetainedClone>(owner:T,grant:RetainedCloneGrant,work:u64)->FixtureSource<T>{let mut bank=semio_framework_value_testing_effect_bank::EffectBank::new(semio_framework_value_testing_effect_bank::Wallet::new(grant,work));let source={let turn=semio_framework_value_clone_source_testing::admit_owned(&mut bank,owner,(),grant);match turn.result{Ok((source,_))=>source,Err(_)=>panic!("original bulk Source admission refused")}};match FixtureSource::from_admitted(source,bank){Ok(source)=>source,Err(_)=>panic!("original bulk Source bank remained active")}}
fn funded_source<T:RetainedClone>(owner:T)->FixtureSource<T>{let(grant,work)=source_stock();funded_with(owner,grant,work)}
fn samples(count: usize) -> Vec<u16> {
    let law = law();
    let (multiplier, offset, modulo) = (number(&law["sample"], "multiplier"), number(&law["sample"], "offset"), number(&law["sample"], "modulo"));
    (0..count).map(|index| ((index * multiplier + offset) % modulo) as u16).collect()
}

use producer::{Harness,Op};
fn exact_grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn action_with_limit<T:RetainedClone>(cursor:&mut Harness<T>,source:&mut FixtureSource<T>,op:Op,body:usize,grant:RetainedCloneGrant,maximum:u64)->(RetainedCloneStep,(usize,usize),u64){let(_,units)=cursor.quote(source,op,body,maximum);let before=source.wallet().remaining_work();let(step,heap)=cursor.drive(source,op,body,maximum,grant,units);let work=before.checked_sub(source.wallet().remaining_work()).expect("original bulk work stock regressed");assert!(work<=units);assert!(step.progress().fits(grant));if matches!(op,Op::Advance|Op::Close)&&step.progress()!=Default::default(){assert!(work>0,"retained effects have no actual work")} (step,heap,work)}
fn action<T:RetainedClone>(cursor:&mut Harness<T>,source:&mut FixtureSource<T>,op:Op,body:usize,grant:RetainedCloneGrant)->(RetainedCloneStep,(usize,usize),u64){action_with_limit(cursor,source,op,body,grant,4096)}
fn close_all<T:RetainedClone>(cursor:&mut Harness<T>,source:&mut FixtureSource<T>,limit:usize,require_progress:bool){
 if cursor.has_output(){let(d,_)=cursor.quote(source,Op::OutputStart,0,4096);let(step,heap,_)=action(cursor,source,Op::OutputStart,0,exact_grant(d));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));for _ in 0..limit{if cursor.output_close_empty(){break}let(d,_)=cursor.quote(source,Op::OutputClose,CLOSE_COPY_BYTES,4096);let(step,heap,_)=action(cursor,source,Op::OutputClose,CLOSE_COPY_BYTES,exact_grant(d));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}assert!(cursor.output_close_empty());}
 let(d,_)=cursor.quote(source,Op::Start,0,4096);action(cursor,source,Op::Start,0,exact_grant(d));for _ in 0..limit{if cursor.terminal(){cursor.release(source);return}let(d,_)=cursor.quote(source,Op::Close,CLOSE_COPY_BYTES,4096);let(step,heap,_)=action(cursor,source,Op::Close,CLOSE_COPY_BYTES,exact_grant(d));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));if require_progress{assert!(step.progress()!=Default::default()||cursor.terminal())}}panic!("bulk clone close did not terminate")
}
struct Turn{work:u64,progress:RetainedCloneProgress,heap:(usize,usize),grant:RetainedCloneGrant,demand_copy:usize}
fn turn_grant<T:RetainedClone>(cursor:&Harness<Vec<T>>,source:&Held<T>,_planned:usize,copy:usize)->RetainedCloneGrant{let maximum=law()["originalWorkCeilings"]["throughput"].as_u64().unwrap();let(d,_)=cursor.quote(&source.0,Op::Advance,copy,maximum);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(d.copy_bytes),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn advance_with<T:RetainedClone>(cursor:&mut Harness<Vec<T>>,source:&mut Held<T>,copy:usize,grant:RetainedCloneGrant)->(RetainedCloneStep,Turn){let maximum=law()["originalWorkCeilings"]["throughput"].as_u64().unwrap();let(d,_)=cursor.quote(&source.0,Op::Advance,copy,maximum);let(step,heap,work)=action_with_limit(cursor,&mut source.0,Op::Advance,copy,grant,maximum);(step,Turn{work,progress:step.progress(),heap,grant,demand_copy:d.copy_bytes})}
fn advance_one<T:RetainedClone>(cursor:&mut Harness<Vec<T>>,source:&mut Held<T>,planned:usize,copy:usize)->(RetainedCloneStep,Turn){let grant=turn_grant(cursor,source,planned,copy);advance_with(cursor,source,copy,grant)}
fn drive<T:RetainedClone>(source:&mut Held<T>,copy:usize)->(Vec<Turn>,Harness<Vec<T>>){let planned=source.borrow().get().len()*size_of::<T>();let mut cursor=Harness::new(&source.0);let mut turns=Vec::new();for _ in 0..1_000_000{let(step,turn)=advance_one(&mut cursor,source,planned,copy);turns.push(turn);if matches!(step,RetainedCloneStep::Complete(_)){let(d,quoted)=cursor.quote(&source.0,Op::Take,0,4096);let grant=exact_grant(d);let(step,_,work)=action(&mut cursor,&mut source.0,Op::Take,0,grant);assert_eq!(work,quoted);assert_eq!(work,1);assert!(law()["publication"]["matchesDemand"].as_bool().unwrap());assert_eq!(step.progress().copied_bytes,d.copy_bytes);assert_eq!(work,law()["publication"]["work"].as_u64().unwrap());assert!(cursor.has_output());return(turns,cursor)}}panic!("vector clone did not terminate")}

#[test]
fn bulk_vector_clone_capacity_accounting_is_exact() {
    let count = 100_003usize;
    let planned = count * size_of::<u16>();
    let mut source = Held::new(samples(count));
    let copy = 10_000usize;
    let (turns, mut cursor) = drive(&mut source, copy);
    cursor.inspect_output(|output|assert_eq!(output.unwrap().capacity()*size_of::<u16>(),planned));
    assert_eq!(turns[0].progress, RetainedCloneProgress { copied_items: 1, copied_bytes: source.borrow().binding_copy_bytes(), ..Default::default() });
    assert_eq!(turns[0].heap, (0, 0));
    assert_eq!(turns[1].progress, RetainedCloneProgress { copied_items: 1, copied_bytes:size_of::<Vec<u16>>()+size_of::<u8>(), retained_capacity_bytes: planned, ..Default::default() });
    assert_eq!(turns[1].heap, (planned, 0), "one reservation for the whole capacity");
    assert_eq!(turns[1].grant.maximum_capacity_bytes, planned);
    let chunks = &turns[2..turns.len()-1];
    assert_eq!(chunks.len(), planned.div_ceil(copy.min(RETAINED_CLONE_BULK_PAGE_BYTES) / 2 * 2));
    for turn in chunks {
        assert_eq!(turn.grant.maximum_capacity_bytes, 0);
        assert_eq!(turn.heap, (0, 0), "chunk copies never allocate");
        assert_eq!(turn.progress.retained_capacity_bytes, 0);
        assert_eq!(turn.progress.released_bytes, 0);
        assert_eq!(turn.progress.copied_items, 1);
    }
    assert_eq!(size_of::<Vec<u16>>(),number(&law(),"ownerHeaderBytes"));assert!(law()["publication"]["matchesDemand"].as_bool().unwrap());assert_eq!(turns.last().unwrap().progress.copied_bytes,turns.last().unwrap().demand_copy);
    assert_eq!(chunks.iter().map(|turn| turn.progress.copied_bytes-size_of::<Vec<u16>>()).sum::<usize>(), planned);
    assert_eq!(turns.iter().map(|turn| turn.progress.retained_capacity_bytes).sum::<usize>(), planned);
    cursor.inspect_output(|output|assert_eq!(output.unwrap(),&samples(count)));
    close_all(&mut cursor,&mut source.0,1_000_000,false);

    let mut cursor = Harness::<Vec<u16>>::new(&source.0);
    let bind = turn_grant(&cursor, &source, planned, copy);
    assert_eq!(advance_with(&mut cursor,&mut source,copy,bind).0.progress().copied_items,1);
    let admitted = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_capacity_bytes: planned - 1, ..admitted },
        RetainedCloneGrant { maximum_items: 0, ..admitted },
    ] {
        let (step,turn)=advance_with(&mut cursor,&mut source,copy,denied);let heap=turn.heap;
        assert_eq!(turn.work,0);assert_eq!(step.progress(),RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.inspect_cursor(|c|c.phase),0);
    }
    assert_eq!(advance_with(&mut cursor,&mut source,copy,admitted).0.progress().retained_capacity_bytes,planned);
    let copying = turn_grant(&cursor, &source, planned, copy);
    for denied in [
        RetainedCloneGrant { maximum_copy_bytes: size_of::<u16>() - 1, ..copying },
        RetainedCloneGrant { maximum_items: 0, ..copying },
        RetainedCloneGrant { maximum_depth: 0, ..copying },
    ] {
        let (step,turn)=advance_with(&mut cursor,&mut source,copy,denied);let heap=turn.heap;
        assert_eq!(turn.work,0);assert_eq!(step.progress(),RetainedCloneProgress::default());
        assert_eq!(heap, (0, 0));
        assert_eq!(cursor.inspect_cursor(|c|c.values.len()),0);
    }
    let one = RetainedCloneGrant { maximum_copy_bytes: size_of::<Vec<u16>>()+size_of::<u16>(), ..copying };
    assert_eq!(advance_with(&mut cursor,&mut source,size_of::<u16>(),one).0.progress(), RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Vec<u16>>()+size_of::<u16>(), ..Default::default() });
    assert_eq!(cursor.inspect_cursor(|c|c.values.len()),1);
    close_all(&mut cursor,&mut source.0,1_000_000,false);
}

fn measured_zst_retirement()->(RetainedCloneProgress,u64){let mut source=funded_source(());let mut cursor=Harness::<()>::new(&source);for _ in 0..128{let(d,_)=cursor.quote(&source,Op::Advance,0,4096);let(step,_,_)=action(&mut cursor,&mut source,Op::Advance,0,exact_grant(d));if matches!(step,RetainedCloneStep::Complete(_)){break}}assert!(cursor.ready());let(d,_)=cursor.quote(&source,Op::Take,0,4096);let(step,_,_)=action(&mut cursor,&mut source,Op::Take,0,exact_grant(d));assert!(matches!(step,RetainedCloneStep::Complete(_)));let mut progress=RetainedCloneProgress::default();let mut work=0u64;let(d,_)=cursor.quote(&source,Op::OutputStart,0,4096);let(step,heap,used)=action(&mut cursor,&mut source,Op::OutputStart,0,exact_grant(d));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));progress=progress.checked_add(step.progress()).unwrap();work=work.checked_add(used).unwrap();for _ in 0..128{if cursor.output_close_empty(){break}let(d,_)=cursor.quote(&source,Op::OutputClose,4096,4096);let(step,heap,used)=action(&mut cursor,&mut source,Op::OutputClose,4096,exact_grant(d));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));progress=progress.checked_add(step.progress()).unwrap();work=work.checked_add(used).unwrap()}assert!(cursor.output_close_empty());assert!(progress.retained_capacity_bytes>0);close_all(&mut cursor,&mut source,128,false);for _ in 0..10000{if source.terminal_is_empty(){break}match source.close_once().unwrap(){semio_framework_value_clone_source_testing::FixtureCloseStep::Progress(_)|semio_framework_value_clone_source_testing::FixtureCloseStep::Pending|semio_framework_value_clone_source_testing::FixtureCloseStep::Complete=>{},other=>panic!("ZST calibration Source close refused: {other:?}")}}assert!(source.terminal_is_empty());(progress,work)}
fn funded_array_source<const N:usize>(owner:[();N])->FixtureSource<[();N]>{let law=law();let scenario=&law["arrayRetirement"];let cases=number(scenario,"cutCases");let total=(0..cases).map(|cut|N.min(cut.saturating_sub(2)*number(&law,"maximumZeroSizedElementsPerTurn"))).sum::<usize>();assert_eq!(total,number(scenario,"maximumRetiredElements"));assert!(scenario["sameBankPerCut"].as_bool().unwrap());assert_eq!(number(scenario,"calibrationElements"),1);let(cost,cpu)=measured_zst_retirement();let(mut grant,work)=source_stock();grant.maximum_items=grant.maximum_items.checked_add(total.checked_mul(cost.copied_items).unwrap()).unwrap();grant.maximum_copy_bytes=grant.maximum_copy_bytes.checked_add(total.checked_mul(cost.copied_bytes).unwrap()).unwrap();grant.maximum_capacity_bytes=grant.maximum_capacity_bytes.checked_add(total.checked_mul(cost.retained_capacity_bytes).unwrap()).unwrap();grant.maximum_release_bytes=grant.maximum_release_bytes.checked_add(total.checked_mul(cost.released_bytes).unwrap()).unwrap();let work=work.checked_add(u64::try_from(total).unwrap().checked_mul(cpu).unwrap()).unwrap();funded_with(owner,grant,work)}
#[test]
fn bulk_array_zero_sized_work_is_bounded_and_every_prefix_closes(){
 const N:usize=8193;assert_eq!(N,number(&law(),"largeZeroSizedArrayCount"));let original=[();N];let oracle=original.clone();let mut source=funded_array_source(original);let expected=2+N.div_ceil(RETAINED_CLONE_BULK_PAGE_ITEMS);
 for cut in 0..=expected+1{let mut cursor=Harness::<[();N]>::new(&source);let mut rows=0;
  for turn in 0..expected{if turn==cut{break}let(d,_)=cursor.quote(&source,Op::Advance,RETAINED_CLONE_BULK_PAGE_BYTES,4096);let issued=exact_grant(d);let before=cursor.inspect_cursor(|c|c.prefix.initialized());let(step,heap,_)=action(&mut cursor,&mut source,Op::Advance,RETAINED_CLONE_BULK_PAGE_BYTES,issued);assert!(step.progress().fits(issued));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));let after=cursor.inspect_cursor(|c|c.prefix.initialized());if after>=before{assert!(after-before<=RETAINED_CLONE_BULK_PAGE_ITEMS);rows+=after-before}if matches!(step,RetainedCloneStep::Complete(_)){let(d,_)=cursor.quote(&source,Op::Take,0,4096);let issued=exact_grant(d);let(step,_,work)=action(&mut cursor,&mut source,Op::Take,0,issued);assert_eq!(work,1);assert!(step.progress().fits(issued));assert!(law()["publication"]["matchesDemand"].as_bool().unwrap());assert_eq!(step.progress().copied_bytes,d.copy_bytes);assert_eq!(work,law()["publication"]["work"].as_u64().unwrap());assert!(cursor.has_output());break}}
  if cursor.has_output(){cursor.inspect_output(|output|assert_eq!(output.unwrap(),&oracle));assert_eq!(rows,N)}close_all(&mut cursor,&mut source,200_000,true);assert_eq!(observe(||drop(cursor)).1,(0,0));assert_eq!(source.borrow().get(),&oracle);
 }
}
