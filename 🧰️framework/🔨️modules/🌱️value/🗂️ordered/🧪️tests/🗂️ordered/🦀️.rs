//! 🧪️ Exact ordered-map ownership and byte-frontier laws against std BTreeMap and committed fixtures.

use super::*;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

fn release_grant<V>(cursor:&Retirement<V>,grant:Grant)->RetainedCloneGrant{RetainedCloneGrant {maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_bytes,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().unwrap(),maximum_depth:cursor.next_depth_demand()}}

struct MixedReceiptPayload {source:Option<Vec<u8>>,replacement:Option<Vec<u8>>,replacement_capacity:usize,complete:bool}
impl crate::retirement::RetireOwned for MixedReceiptPayload {
    fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl crate::retirement::RetirementCursor for MixedReceiptPayload {
    fn close_step(&mut self,grant:RetainedCloneGrant)->crate::retirement::RetirementStep {
        use crate::{retirement::RetirementStep as Step,retained_clone::RetainedCloneProgress};
        if self.complete{return Step::Complete;}
        if grant.maximum_items==0{return Step::BudgetExhausted;}
        if grant.maximum_depth==0{return Step::Failure(crate::ValueError::literal(crate::ValueRefusalKind::DepthLimit,"mixed payload requires admitted depth"));}
        if let Some(source)=self.source.as_ref(){
            if grant.maximum_copy_bytes<1||grant.maximum_capacity_bytes<self.replacement_capacity||grant.maximum_release_bytes<source.capacity(){return Step::BudgetExhausted;}
            let released_bytes=source.capacity();let mut replacement=Vec::with_capacity(self.replacement_capacity);replacement.push(source[0]);let retained_capacity_bytes=replacement.capacity();
            drop(self.source.take());self.replacement=Some(replacement);
            return Step::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:1,retained_capacity_bytes,released_bytes});
        }
        if let Some(replacement)=self.replacement.as_ref(){
            let bytes=replacement.capacity();if grant.maximum_release_bytes<bytes{return Step::BudgetExhausted;}
            assert_eq!(replacement[0],7);drop(self.replacement.take());return Step::Bytes(bytes);
        }
        self.complete=true;Step::Complete
    }
    fn terminal_is_empty(&self)->bool{self.complete&&self.source.is_none()&&self.replacement.is_none()}
    fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(usize::from(self.source.is_some()))}
    fn next_close_byte_demand(&self)->Option<usize>{Some(self.source.as_ref().or(self.replacement.as_ref()).map_or(0,Vec::capacity))}
    fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(if self.source.is_some(){self.replacement_capacity}else{0})}
    fn terminal_release_bytes(&self)->Option<usize>{self.complete.then_some(std::mem::size_of::<Self>())}
}

#[test]
fn original_ordered_typed_owner_preserves_mixed_physical_receipt() {
    use crate::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneProgress};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/📦️full-receipt/🔣️.json")).unwrap();
    let source_capacity=fixture["sourceCapacityBytes"].as_u64().unwrap()as usize;let replacement_capacity=fixture["replacementCapacityBytes"].as_u64().unwrap()as usize;
    for maximum_copy_bytes in fixture["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let((source,pointer),(born,freed))=crate::observe_retirement_allocations(||{let mut bytes=Vec::with_capacity(source_capacity);bytes.push(fixture["payloadByte"].as_u64().unwrap()as u8);let pointer=bytes.as_ptr();(OrderedMap::from([(fixture["key"].as_str().unwrap().to_owned(),MixedReceiptPayload {source:Some(bytes),replacement:None,replacement_capacity,complete:false})]),pointer)});
        let source_bytes=born-freed;
        let(mut owner,allocation)=crate::observe_retirement_allocations(||ControlledRetirement::new(source).unwrap_or_else(|_|panic!("original generic OrderedMap owner is supported")));assert_eq!(allocation,(0,0));
        assert_eq!(owner.original().unwrap().get(fixture["key"].as_str().unwrap()).unwrap().source.as_ref().unwrap().as_ptr(),pointer);
        let mut mismatch=None;let mut total=RetainedCloneProgress::default();let mut physical_release=0;let mut actual_birth=0;let mut turns=0;let mut mixed_turns=0;
        while !owner.terminal_is_empty(){
            turns+=1;assert!(turns<20000);
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes,maximum_capacity_bytes:owner.next_capacity_byte_demand(maximum_copy_bytes).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap().max(source_capacity),maximum_depth:owner.next_depth_demand().unwrap()};
            let(paused,allocation)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(paused.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));
            if grant.maximum_capacity_bytes!=0{let(paused,allocation)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(paused.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));}
            let(step,(born,freed))=crate::observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));
            if born==replacement_capacity&&freed==source_capacity{mixed_turns+=1;assert_eq!(progress.copied_bytes,1);}
            if (born,freed)!=(progress.retained_capacity_bytes,progress.released_bytes)&&mismatch.is_none(){mismatch=Some((turns,born,freed,progress.retained_capacity_bytes,progress.released_bytes));}
            total=total.checked_add(progress).unwrap();actual_birth+=born;physical_release+=freed;
        }
        let(_,allocation)=crate::observe_retirement_allocations(||drop(owner));assert_eq!(allocation,(0,0));assert_eq!(mixed_turns,1);assert_eq!(physical_release,source_bytes+actual_birth);
        eprintln!("[DEBUG] original Ordered mixed copy={maximum_copy_bytes} source={source_bytes} admitted={actual_birth} physical={physical_release} reported={} turns={turns} mixedTurns={mixed_turns} terminalDropFree=0 mismatch={mismatch:?}",total.released_bytes);
        assert_eq!(mismatch,None,"original Ordered typed owner must preserve simultaneous copy, capacity birth and release");assert_eq!(total.released_bytes,physical_release);
    }
}

#[test]
fn original_controlled_retirement_exposes_simultaneous_physical_demands() {
    use crate::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneProgress};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/📦️full-receipt/🔣️.json")).unwrap();
    let expected=&fixture["expectedMixedDemand"];let source_capacity=expected["releaseBytes"].as_u64().unwrap()as usize;let replacement_capacity=expected["capacityBytes"].as_u64().unwrap()as usize;
    let(source,(born,freed))=crate::observe_retirement_allocations(||{let mut source=Vec::with_capacity(source_capacity);source.push(7);MixedReceiptPayload {source:Some(source),replacement:None,replacement_capacity,complete:false}});assert_eq!((born,freed),(source_capacity,0));
    let pointer=source.source.as_ref().unwrap().as_ptr();let(mut owner,allocation)=crate::observe_retirement_allocations(||ControlledRetirement::new(source).unwrap_or_else(|_|panic!("original controlled payload authority")));assert_eq!(allocation,(0,0));assert_eq!(owner.original().unwrap().source.as_ref().unwrap().as_ptr(),pointer);
    let mut mixed_demand=None;let mut total=RetainedCloneProgress::default();let mut physical=0;let mut turns=0;
    while !owner.terminal_is_empty(){
        turns+=1;assert!(turns<10000);let copy=owner.next_copy_byte_demand().unwrap();let capacity=owner.next_capacity_byte_demand(1).unwrap();let release=owner.next_release_byte_demand().unwrap();let depth=owner.next_depth_demand().unwrap();
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:1,maximum_capacity_bytes:capacity,maximum_release_bytes:release.max(source_capacity),maximum_depth:depth};
        if copy==1&&capacity==replacement_capacity{mixed_demand=Some((copy,capacity,release,depth));let(paused,allocation)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant {maximum_release_bytes:source_capacity-1,..grant}).unwrap());assert_eq!(paused.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));}
        let(step,(born,freed))=crate::observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!(born,progress.retained_capacity_bytes);assert_eq!(freed,progress.released_bytes);assert!(progress.fits(grant));total=total.checked_add(progress).unwrap();physical+=freed;
    }
    let(_,allocation)=crate::observe_retirement_allocations(||drop(owner));assert_eq!(allocation,(0,0));assert_eq!(physical,source_capacity+total.retained_capacity_bytes);
    eprintln!("[DEBUG] original Controlled simultaneous demand={mixed_demand:?} expectedCopy=1 expectedCapacity=96 expectedRelease=32 expectedDepth=2 physical={physical} terminalDropFree=0");
    assert_eq!(mixed_demand,Some((expected["copyBytes"].as_u64().unwrap()as usize,replacement_capacity,source_capacity,expected["depth"].as_u64().unwrap()as usize)),"independent physical release demand must remain visible while logical work is pending");
}

#[test]
fn original_ordered_deque_declares_physical_handoff_demand() {
    use crate::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneProgress};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/📦️full-receipt/🔣️.json")).unwrap();let row=&fixture["deque"];
    for copy in fixture["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let((source,pointer),(born,freed))=crate::observe_retirement_allocations(||{let mut source=std::collections::VecDeque::with_capacity(row["containerCapacity"].as_u64().unwrap()as usize);for value in row["source"].as_array().unwrap(){let mut text=String::with_capacity(row["payloadCapacity"].as_u64().unwrap()as usize);text.push_str(value.as_str().unwrap());source.push_back(text);}let pointer=source.as_slices().0.as_ptr();(OrderedMap::from([("deque".to_owned(),source)]),pointer)});assert_eq!(freed,0);let source_bytes=born;assert_eq!(serde_json::to_string(source.get("deque").unwrap()).unwrap(),row["canonicalSource"].as_str().unwrap());
        let(mut owner,heap)=crate::observe_retirement_allocations(||ControlledRetirement::new(source).unwrap_or_else(|_|panic!("original ordered deque supports controlled retirement")));assert_eq!(heap,(0,0));assert_eq!(owner.original().unwrap().get("deque").unwrap().as_slices().0.as_ptr(),pointer);
        let mut missing=0;let mut turns=0;let mut total=RetainedCloneProgress::default();let mut physical=0;
        while !owner.terminal_is_empty(){
            turns+=1;assert!(turns<20000);let(demand,heap)=crate::observe_retirement_allocations(||owner.next_release_byte_demand());assert_eq!(heap,(0,0));
            let release=match demand {Ok(bytes)=>bytes,Err(error)=>{assert_eq!(error.kind,crate::ValueRefusalKind::InvariantViolated);missing+=1;0}};
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};
            let(step,(born,freed))=crate::observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(born,progress.retained_capacity_bytes);assert_eq!(freed,progress.released_bytes);total=total.checked_add(progress).unwrap();physical+=freed;
        }
        let(_,heap)=crate::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));assert_eq!(physical,source_bytes+total.retained_capacity_bytes);
        eprintln!("[DEBUG] original Controlled deque copy={copy} missingReleaseDemands={missing} source={source_bytes} admitted={} physical={physical} turns={turns} terminalDropFree=0",total.retained_capacity_bytes);
        assert_eq!(missing,row["expectedMissingReleaseDemands"].as_u64().unwrap()as usize,"original deque must explicitly declare zero physical release for its admitted child handoff");
    }
}

#[test]
fn ordered_partial_mutations_compose_typed_retirement_at_every_neutral_cancel_frontier() {
    use crate::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneProgress};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🩹️update/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){for maximum_copy_bytes in fixture["grants"].as_array().unwrap().iter().map(|grant|grant.as_u64().unwrap()as usize){for cut in fixture["cancelAfterSteps"].as_array().unwrap().iter().map(|cut|cut.as_u64().unwrap()as usize){
        for operation in row["operations"].as_array().unwrap(){
            let((map,mut cursor),(birth,free))=crate::observe_retirement_allocations(||{
                let map:OrderedMap<String>=row["initial"].as_array().unwrap().iter().map(|entry|(entry[0].as_str().unwrap().to_owned(),entry[1].as_i64().unwrap().to_string())).collect();
                let key=operation["key"].as_str().unwrap().to_owned();let cursor=if operation["kind"]=="set"{map.begin_set(key,operation["value"].as_i64().unwrap().to_string())}else{map.begin_remove(key)};(map,cursor)
            });let mut owned=birth-free;
            for _ in 0..cut{if cursor.is_complete(){break;}let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes,maximum_capacity_bytes:cursor.next_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()};let(step,(birth,free))=crate::observe_retirement_allocations(||cursor.advance(grant).unwrap());assert_eq!(birth,step.progress().retained_capacity_bytes);assert_eq!(free,0);owned+=birth;}
            let(mut owner,allocation)=crate::observe_retirement_allocations(||ControlledRetirement::new((map,cursor)).unwrap_or_else(|_|panic!("partial ordered mutation requires typed ownership composition")));assert_eq!(allocation,(0,0));let mut total=RetainedCloneProgress::default();let mut turns=0;
            while !owner.terminal_is_empty(){turns+=1;assert!(turns<200000);let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes,maximum_capacity_bytes:owner.next_capacity_byte_demand(maximum_copy_bytes).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
                let(step,allocation)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));
                let(step,(birth,free))=crate::observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(birth,progress.retained_capacity_bytes);assert_eq!(free,progress.released_bytes);total=total.checked_add(progress).unwrap();
            }
            assert_eq!(total.released_bytes,owned+total.retained_capacity_bytes);
        }
    }}}
    eprintln!("[DEBUG] actual partial ordered updates preserve all inputs, mutation births, payload backing and scaffold conservation at every neutral cancellation frontier");
}

#[test]
fn ordered_mutations_admit_exact_births_and_preserve_obsolete_roots_until_close() {
    use crate::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🩹️update/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){for maximum_copy_bytes in fixture["grants"].as_array().unwrap().iter().map(|grant|grant.as_u64().unwrap()as usize){
        let initial:Vec<(String,i64)>=row["initial"].as_array().unwrap().iter().map(|entry|(entry[0].as_str().unwrap().to_owned(),entry[1].as_i64().unwrap())).collect();
        let mut oracle:BTreeMap<_,_>=initial.iter().cloned().collect();let mut map:OrderedMap<_>=initial.into_iter().collect();let mut total_birth=0;let mut turns=0;
        for operation in row["operations"].as_array().unwrap(){
            let key=operation["key"].as_str().unwrap();let mut cursor=if operation["kind"]=="set"{map.begin_set(key.to_owned(),operation["value"].as_i64().unwrap())}else{map.begin_remove(key.to_owned())};
            while !cursor.is_complete(){turns+=1;assert!(turns<100000);let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes,maximum_capacity_bytes:cursor.next_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()};
                let demand=cursor.next_copy_byte_demand();
                for denied in [RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant {maximum_copy_bytes:demand.saturating_sub(1),..grant}]{
                    if denied.maximum_items!=0&&denied.maximum_capacity_bytes==grant.maximum_capacity_bytes&&denied.maximum_copy_bytes>=demand{continue;}
                    let(step,allocation)=crate::observe_retirement_allocations(||cursor.advance(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));assert_eq!(cursor.next_capacity_byte_demand().unwrap(),grant.maximum_capacity_bytes);assert_eq!(cursor.next_copy_byte_demand(),demand);
                }
                if grant.maximum_depth!=0{let(error,allocation)=crate::observe_retirement_allocations(||cursor.advance(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant}).unwrap_err());assert_eq!(error.kind,crate::ValueRefusalKind::DepthLimit);assert_eq!(allocation,(0,0));}
                let(step,(birth,free))=crate::observe_retirement_allocations(||cursor.advance(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(birth,grant.maximum_capacity_bytes);assert_eq!(birth,progress.retained_capacity_bytes);assert_eq!(free,0);assert_eq!(progress.released_bytes,0);assert!(progress.copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)));total_birth+=birth;
            }
            let removed=cursor.take_removed();let result=cursor.take_result().unwrap();let displaced=std::mem::replace(&mut map,result);retire(displaced,Grant {maximum_items:1,maximum_bytes:maximum_copy_bytes});close(&mut cursor,Grant {maximum_items:1,maximum_bytes:maximum_copy_bytes});
            let removed=removed.and_then(super::release_cold);let expected=if operation["kind"]=="set"{oracle.insert(key.to_owned(),operation["value"].as_i64().unwrap())}else{oracle.remove(key)};assert_eq!(removed,expected);assert!(map.iter().map(|(key,value)|(key,*value)).eq(oracle.iter().map(|(key,value)|(key,*value))));check_tree(&map.root);
        }
        retire(map,Grant {maximum_items:1,maximum_bytes:maximum_copy_bytes});eprintln!("[DEBUG] exact ordered mutation {} copy{} turns{} birth{} preserved rotation and successor ownership until granted close",row["id"].as_str().unwrap(),maximum_copy_bytes,turns,total_birth);
    }}
}

#[test]
fn ordered_retirement_composes_real_generic_owner_under_independent_category_grants() {
    use crate::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneProgress,RetainedCloneStep}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){for maximum_copy_bytes in fixture["grants"].as_array().unwrap().iter().map(|grant|grant.as_u64().unwrap()as usize){
        let mut key=String::with_capacity(row["keyCapacity"].as_u64().unwrap()as usize);key.push_str(row["key"].as_str().unwrap());
        let mut value=String::with_capacity(row["valueCapacity"].as_u64().unwrap()as usize);value.push_str(row["value"].as_str().unwrap());let payload_bytes=key.capacity()+value.capacity();
        let(source,(source_birth,source_free))=crate::observe_retirement_allocations(||OrderedMap::from([(key,value)]));assert_eq!(source_free,0);
        let(mut owner,allocation)=crate::observe_retirement_allocations(||ControlledRetirement::new(source).unwrap_or_else(|_|panic!("ordered field owner requires defining typed retirement")));assert_eq!(allocation,(0,0));
        let mut total=RetainedCloneProgress::default();let mut physical_free=0;let mut turns=0;
        while !owner.terminal_is_empty(){turns+=1;assert!(turns<20000);let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes,maximum_capacity_bytes:owner.next_capacity_byte_demand(maximum_copy_bytes).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            let(paused,allocation)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(paused.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));
            for denied in [RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant},RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant}] {
                if denied.maximum_capacity_bytes==grant.maximum_capacity_bytes&&denied.maximum_release_bytes==grant.maximum_release_bytes{continue;}
                let(paused,allocation)=crate::observe_retirement_allocations(||owner.step(denied).unwrap());assert_eq!(paused.progress(),RetainedCloneProgress::default());assert_eq!(allocation,(0,0));
            }
            let(step,(birth,free))=crate::observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(birth,progress.retained_capacity_bytes);assert_eq!(free,progress.released_bytes);total=total.checked_add(progress).unwrap();physical_free+=free;assert!(matches!(step,RetainedCloneStep::Progress(_)|RetainedCloneStep::Complete(_)));
        }
        assert_eq!(physical_free,source_birth+payload_bytes+total.retained_capacity_bytes);assert_eq!(total.released_bytes,physical_free);
        eprintln!("[DEBUG] actual ordered generic field {} copy{} turns{} source{} scaffold{} released{} independently admitted birth/release",row["id"].as_str().unwrap(),maximum_copy_bytes,turns,source_birth+payload_bytes,total.retained_capacity_bytes,physical_free);
    }}
}

#[test]
fn ordered_shared_input_preserves_unique_pointer_and_refuses_foreign_alias_custody() {
    use crate::retained_clone::RetainedCloneGrant;
    let original=Arc::new(String::from("雪😀"));let pointer=Arc::as_ptr(&original);let weak=Arc::downgrade(&original);
    let((error,original),allocation)=crate::observe_retirement_allocations(||SharedOwner::admit_arc(original).err().expect("foreign weak custody must refuse"));
    assert_eq!(error.kind,crate::ValueRefusalKind::UnsupportedOwner);assert_eq!(allocation,(0,0));assert_eq!(Arc::as_ptr(&original),pointer);drop(weak);
    let alias=Arc::clone(&original);let((_,original),allocation)=crate::observe_retirement_allocations(||SharedOwner::admit_arc(original).err().expect("foreign strong custody must refuse"));assert_eq!(allocation,(0,0));assert_eq!(Arc::as_ptr(&original),pointer);drop(alias);
    let(mut owner,allocation)=crate::observe_retirement_allocations(||SharedOwner::admit_arc(original).unwrap_or_else(|_|panic!("unique admitted input")));assert_eq!(allocation,(0,0));assert_eq!(owner.as_ptr(),pointer);
    let mut alias=owner.clone();let extent=SharedOwner::<String>::allocation_bytes();
    for grant in [RetainedCloneGrant::default(),RetainedCloneGrant::one_release_turn(extent-1,1)] {let(step,allocation)=crate::observe_retirement_allocations(||owner.release_step(grant).unwrap());assert!(step.value.is_none());assert_eq!(step.progress,crate::retained_clone::RetainedCloneProgress::default());assert_eq!(allocation,(0,0));assert_eq!(owner.as_ptr(),pointer);}
    let(step,allocation)=crate::observe_retirement_allocations(||owner.release_step(RetainedCloneGrant::one_release_turn(extent,1)).unwrap());assert!(step.value.is_none());assert_eq!(allocation,(0,0));assert_eq!(step.progress.released_bytes,0);assert!(owner.terminal_is_empty());
    let(step,allocation)=crate::observe_retirement_allocations(||alias.release_step(RetainedCloneGrant::one_release_turn(extent,1)).unwrap());let payload=step.value.expect("exact final input payload");assert_eq!(allocation,(0,extent));assert_eq!(step.progress.released_bytes,extent);assert!(alias.terminal_is_empty());assert_eq!(payload,"雪😀");
    eprintln!("[DEBUG] first-party shared input preserved admitted pointer, refused foreign strong/Weak custody without allocation, and final header released only under its exact independent grant");
}

#[test]
fn ordered_retirement_physical_receipts_match_actual_allocator_and_whole_grants() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        for maximum_bytes in fixture["grants"].as_array().unwrap().iter().map(|grant|grant.as_u64().unwrap()as usize) {
            let mut key=String::with_capacity(row["keyCapacity"].as_u64().unwrap()as usize);key.push_str(row["key"].as_str().unwrap());
            let mut value=String::with_capacity(row["valueCapacity"].as_u64().unwrap()as usize);value.push_str(row["value"].as_str().unwrap());let original_pointer=value.as_ptr();
            let oracle=BTreeMap::from([(key.clone(),value.clone())]);let source=OrderedMap::from([(key,value)]);assert_eq!(serde_json::to_vec(&source).unwrap(),serde_json::to_vec(&oracle).unwrap());
            let mut cursor=source.retire();let mut observations=Vec::new();let mut transferred=Vec::new();let mut complete=false;
            for _ in 0..20000 {
                let demand=cursor.next_close_byte_demand().unwrap();let grant=release_grant(&cursor,Grant{maximum_items:1,maximum_bytes});
                if !cursor.is_empty(){for denied in [RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_release_bytes:demand.saturating_sub(1),..grant}] {
                    if demand==0&&denied.maximum_items!=0{continue;}
                    let(step,allocation)=crate::observe_retirement_allocations(||cursor.advance(denied));assert!(matches!(step,RetirementStep::Blocked));assert_eq!(allocation,(0,0));assert_eq!(cursor.next_close_byte_demand().unwrap(),demand);
                }}
                let(step,(birth,free))=crate::observe_retirement_allocations(||cursor.advance(grant));
                let reported=match step {RetirementStep::Progress{released_bytes,..}=>released_bytes,RetirementStep::ProcessedBytes(bytes)=>{assert!(bytes<=maximum_bytes);0},RetirementStep::OwnedValue(value)=>{assert_eq!(value.as_ptr(),original_pointer);transferred.push(value);0},RetirementStep::Complete=>{complete=true;0},RetirementStep::Blocked=>0,RetirementStep::Failure(error)=>panic!("{error}")};
                observations.push((demand,birth,free,reported));if complete{break;}
            }
            assert!(complete&&cursor.terminal_is_empty());assert_eq!(transferred.len(),1);drop(cursor);
            eprintln!("[DEBUG] actual ordered retirement {} byte grant{} {} closed observations: {:?}",row["id"].as_str().unwrap(),maximum_bytes,observations.len(),observations.iter().filter(|(_,_,free,reported)|free!=reported).collect::<Vec<_>>());
            for(demand,birth,free,reported)in observations {assert_eq!(birth,0);assert_eq!(free,reported,"exact physical release receipt; next demand {demand}");assert!(free<=demand,"whole physical release fits its separate caller grant");}
        }
    }
}

//#region 🔣️Fixture
fn key(value: &serde_json::Value) -> String {
    value["prefix"].as_str().unwrap().repeat(value["repetitions"].as_u64().unwrap() as usize) + value["suffix"].as_str().unwrap()
}

fn advance_update<V>(cursor:&mut UpdateCursor<V>,grant:Grant)->Step {
    let step=cursor.advance(RetainedCloneGrant {maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_bytes,maximum_capacity_bytes:if grant.maximum_bytes==0{0}else{cursor.next_capacity_byte_demand().unwrap()},maximum_release_bytes:0,maximum_depth:cursor.next_depth_demand()}).unwrap();
    let progress=step.progress();assert_eq!(progress.released_bytes,0);
    if progress.copied_items!=0{Step::Progress {completed_items:progress.copied_items,completed_bytes:progress.copied_bytes}}else if matches!(step,RetainedCloneStep::Complete(_)){Step::Complete}else{Step::Blocked}
}

fn update<V>(cursor: &mut UpdateCursor<V>, grant: Grant) -> usize {
    let mut compared = 0;
    for _ in 0..1_000_000 {
        if cursor.is_complete() { return compared; }
        match advance_update(cursor,grant) {
            Step::Progress { completed_items, completed_bytes } => { assert!(completed_items <= 1 && completed_bytes <= grant.maximum_bytes); compared += completed_bytes; }
            Step::Complete => return compared,
            Step::Blocked => panic!("positive grant blocked ordered-map update"),
        }
    }
    panic!("ordered-map update did not finish")
}

fn close<V>(cursor: &mut UpdateCursor<V>, grant: Grant) -> Vec<V> {
    cursor.begin_close(); let mut values = Vec::new();
    for _ in 0..1_000_000 {
        let physical=RetainedCloneGrant {maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_bytes,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand()};
        match cursor.close_step(physical) {
            RetirementStep::Progress { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= physical.maximum_release_bytes),
            RetirementStep::ProcessedBytes(bytes)=>assert!(bytes<=physical.maximum_copy_bytes),
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => { assert!(cursor.terminal_is_empty()); return values; }
            RetirementStep::Blocked => panic!("positive grant blocked ordered-map close"),
            RetirementStep::Failure(error)=>panic!("{error}"),
        }
    }
    panic!("ordered-map close did not finish")
}

fn retire<V>(map: OrderedMap<V>, grant: Grant) -> Vec<V> {
    let mut retirement = map.retire(); let mut values = Vec::new();
    for _ in 0..1_000_000 {
        let physical=release_grant(&retirement,grant);
        match retirement.advance(physical) {
            RetirementStep::Progress { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= physical.maximum_release_bytes),
            RetirementStep::ProcessedBytes(bytes)=>assert!(bytes<=physical.maximum_copy_bytes),
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => { assert!(retirement.is_empty()); return values; }
            RetirementStep::Blocked => panic!("positive grant blocked ordered-map retirement"),
            RetirementStep::Failure(error)=>panic!("{error}"),
        }
    }
    panic!("ordered-map retirement did not finish")
}

fn lookup<V>(cursor: &mut LookupCursor<V>, grant: Grant) -> usize {
    let mut compared = 0;
    for _ in 0..1_000_000 {
        if cursor.is_complete() { return compared; }
        match cursor.advance(grant) {
            Step::Progress { completed_items, completed_bytes } => { assert!(completed_items <= 1 && completed_bytes <= grant.maximum_bytes); compared += completed_bytes; }
            Step::Complete => return compared,
            Step::Blocked => panic!("positive grant blocked ordered-map lookup"),
        }
    }
    panic!("ordered-map lookup did not finish")
}

fn close_lookup<V>(cursor: &mut LookupCursor<V>, grant: Grant) -> Vec<V> {
    cursor.begin_close(); let mut values = Vec::new();
    for _ in 0..1_000_000 {
        let physical=RetainedCloneGrant {maximum_items:grant.maximum_items,maximum_copy_bytes:grant.maximum_bytes,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand()};
        match cursor.close_step(physical) {
            RetirementStep::Progress { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= physical.maximum_release_bytes),
            RetirementStep::ProcessedBytes(bytes)=>assert!(bytes<=physical.maximum_copy_bytes),
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => { assert!(cursor.terminal_is_empty()); return values; }
            RetirementStep::Blocked => panic!("positive grant blocked ordered-map lookup close"),
            RetirementStep::Failure(error)=>panic!("{error}"),
        }
    }
    panic!("ordered-map lookup close did not finish")
}

fn check_tree<V>(root: &Root<V>) -> (usize, usize) {
    let Some(root) = root else { return (0, 0); };
    let (left_height, left_len) = check_tree(&root.left); let (right_height, right_len) = check_tree(&root.right);
    assert!(left_height.abs_diff(right_height) <= 1);
    assert!(root.height <= MAX_AVL_HEIGHT);
    assert_eq!(root.height, left_height.max(right_height) + 1); assert_eq!(root.len, left_len + right_len + 1);
    (root.height, root.len)
}
//#endregion 🔣️Fixture

//#region ⚖️Oracle
#[test]
fn fixture_operations_match_btree_map_under_every_actual_grant() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️ordered-map.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        for maximum_bytes in fixture["grants"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
            let grant = Grant { maximum_items: 1, maximum_bytes }; let mut map = OrderedMap::new(); let mut oracle = BTreeMap::new();
            for operation in row["operations"].as_array().unwrap() {
                let key = key(&operation["key"]);
                let mut cursor = if operation["op"] == "set" {
                    let value = operation["value"].as_str().unwrap().to_owned(); oracle.insert(key.clone(), value.clone()); map.begin_set(key, value)
                } else { oracle.remove(&key); map.begin_remove(key) };
                update(&mut cursor, grant); let displaced = std::mem::replace(&mut map, cursor.take_result().unwrap()); retire(displaced, grant); close(&mut cursor, grant);
                check_tree(&map.root);
                assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
                assert_eq!(serde_json::to_vec(&map).unwrap(), serde_json::to_vec(&oracle).unwrap());
                assert_eq!(map.iter().rev().map(|(key, _)| key).collect::<Vec<_>>(), oracle.keys().rev().collect::<Vec<_>>());
            }
            let expected: BTreeMap<_, _> = row["expected"].as_array().unwrap().iter().map(|entry| (key(&entry["key"]), entry["value"].as_str().unwrap().to_owned())).collect();
            assert_eq!(oracle, expected);
            retire(map, grant);
        }
    }
}

#[test]
fn all_rotation_and_successor_shapes_match_btree_map() {
    let grant = Grant { maximum_items: 1, maximum_bytes: 7 }; let mut map = OrderedMap::new(); let mut oracle = BTreeMap::new(); let mut state = 0x517c_c1b7_u64;
    for index in 0..4096 {
        state ^= state << 13; state ^= state >> 7; state ^= state << 17; let key = format!("{:03}", state % 257);
        let mut cursor = if index % 3 == 0 { oracle.remove(&key); map.begin_remove(key) } else { oracle.insert(key.clone(), index); map.begin_set(key, index) };
        update(&mut cursor, grant); let displaced = std::mem::replace(&mut map, cursor.take_result().unwrap()); retire(displaced, grant); close(&mut cursor, grant); check_tree(&map.root);
        assert_eq!(map.iter().map(|(key, value)| (key.clone(), *value)).collect::<BTreeMap<_, _>>(), oracle);
    }
    retire(map, grant);
}

#[test]
fn sorted_construction_preserves_fixed_height_and_double_ended_rank_bound() {
    let mut map = OrderedMap::new();
    for index in 0..4096 { map.insert(format!("{index:04}"), index); check_tree(&map.root); }
    let mut iter = map.iter();
    for index in 0..2048 {
        assert_eq!(*iter.next().unwrap().1, index);
        assert_eq!(*iter.next_back().unwrap().1, 4095 - index);
        assert_eq!(iter.len(), 4094 - index * 2);
    }
    assert!(iter.next().is_none() && iter.next_back().is_none());
    retire(map, Grant { maximum_items: 1, maximum_bytes: 1 });
}

#[test]
fn cold_serde_preserves_duplicate_semantics_and_retires_failed_partial_roots() {
    let map: OrderedMap<i64> = serde_json::from_str(r#"{"b":2,"a":1,"b":3}"#).unwrap();
    let oracle: BTreeMap<String, i64> = serde_json::from_str(r#"{"b":2,"a":1,"b":3}"#).unwrap();
    assert_eq!(serde_json::to_vec(&map).unwrap(), serde_json::to_vec(&oracle).unwrap());
    retire(map, Grant { maximum_items: 1, maximum_bytes: 1 });
    assert!(serde_json::from_str::<OrderedMap<i64>>(r#"{"a":1,"b":"bad"}"#).is_err());
}
//#endregion ⚖️Oracle

//#region 🔎️Lookup
#[test]
fn lookup_matches_oracle_with_exact_long_key_comparison_bytes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️ordered-map.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        for row in fixture["lookupCases"].as_array().unwrap() {
            let grant = Grant { maximum_items: 1, maximum_bytes }; let mut map = OrderedMap::new();
            let source_value = row["sourceValue"].as_i64().unwrap();
            map.insert(key(&row["sourceKey"]), source_value);
            let mut cursor = map.begin_lookup(key(&row["query"]));
            assert_eq!(cursor.advance(Grant { maximum_items: 0, maximum_bytes }), Step::Blocked);
            assert_eq!(cursor.advance(Grant { maximum_items: 1, maximum_bytes: 0 }), Step::Blocked);
            assert_eq!(lookup(&mut cursor, grant), row["expectedComparedBytes"].as_u64().unwrap() as usize);
            assert_eq!(cursor.result().copied(), row["expected"].as_i64());
            assert!(retire(map, grant).is_empty());
            assert_eq!(close_lookup(&mut cursor, grant), [source_value]);
        }
    }
}

#[test]
fn lookup_cancellation_keeps_borrowed_payload_alive_until_owned_transfer() {
    for cancel in [0, 1, 9, 100, 20_000] {
        let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new();
        map.insert("🌊".repeat(2048), Payload(Arc::clone(&drops)));
        let mut cursor = map.begin_lookup("🌊".repeat(2048)); let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
        for _ in 0..cancel { cursor.advance(grant); }
        assert!(retire(map, grant).is_empty());
        let values = close_lookup(&mut cursor, grant);
        assert_eq!(drops.load(AtomicOrdering::SeqCst), 0); assert_eq!(values.len(), 1);
        drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
    }
}
//#endregion 🔎️Lookup

//#region 🧹️CancellationAndOwnership
struct Payload(Arc<AtomicUsize>);
impl Drop for Payload { fn drop(&mut self) { self.0.fetch_add(1, AtomicOrdering::SeqCst); } }

#[test]
fn cancellation_never_clones_payload_or_drops_final_value_inside_a_step() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️ordered-map.json")).unwrap();
    for cancel in fixture["cancelAfterSteps"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize) {
        let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new(); map.insert("🌊".repeat(2048) + "a", Payload(Arc::clone(&drops)));
        let mut cursor = map.begin_set("🌊".repeat(2048) + "b", Payload(Arc::clone(&drops)));
        let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
        for _ in 0..cancel { advance_update(&mut cursor,grant); }
        assert!(retire(map, grant).is_empty()); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
        let values = close(&mut cursor, grant); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
        assert_eq!(values.len(), 2); drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 2);
    }
}

#[test]
fn every_update_phase_closes_unclaimed_result_and_aliases_without_payload_drop() {
    for remove in [false, true] {
        for cancel in 0..48 {
            let grant = Grant { maximum_items: 1, maximum_bytes: 1 }; let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new();
            for key in ["4", "2", "6", "1", "3", "5", "7"] { map.insert(key.into(), Payload(Arc::clone(&drops))); }
            let mut cursor = if remove { map.begin_remove("4".into()) } else { map.begin_set("4".into(), Payload(Arc::clone(&drops))) };
            assert_eq!(advance_update(&mut cursor,Grant { maximum_items: 0, maximum_bytes: 1 }), Step::Blocked);
            assert_eq!(advance_update(&mut cursor,Grant { maximum_items: 1, maximum_bytes: 0 }), Step::Blocked);
            for _ in 0..cancel { advance_update(&mut cursor,grant); }
            assert!(retire(map, grant).is_empty());
            let values = close(&mut cursor, grant); let expected = if remove { 7 } else { 8 };
            assert_eq!(values.len(), expected); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
            drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), expected);
        }
    }
}

#[test]
fn removed_payload_handoff_preserves_exact_last_owner() {
    let grant = Grant { maximum_items: 1, maximum_bytes: 1 }; let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new();
    map.insert("key".into(), Payload(Arc::clone(&drops)));
    let mut cursor = map.begin_remove("key".into()); update(&mut cursor, grant);
    let removed = cursor.take_removed().unwrap(); let result = cursor.take_result().unwrap();
    assert!(retire(map, grant).is_empty()); assert!(close(&mut cursor, grant).is_empty()); assert!(retire(result, grant).is_empty());
    assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    let mut removed=removed;let payload=removed.release_step(RetainedCloneGrant::one_release_turn(removed.next_release_byte_demand(),1)).unwrap().value.expect("removed payload must have one transferred owner");
    assert_eq!(drops.load(AtomicOrdering::SeqCst), 0); drop(payload); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
}

/// 🔒️ Deliberate contract violations retain their tiny test allocations instead of invoking recursive payload destruction.
#[test]
fn live_owner_drop_guards_never_destroy_payloads() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️ordered-map.json")).unwrap();
    assert_eq!(fixture["ownership"]["terminalOwners"], 0);
    for kind in 0..fixture["ownership"]["liveOwners"].as_array().unwrap().len() {
        let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new(); map.insert("key".into(), Payload(Arc::clone(&drops)));
        let guarded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match kind {
            0 => drop(map),
            1 => { let cursor = map.begin_set("next".into(), Payload(Arc::clone(&drops))); retire(map, Grant { maximum_items: 1, maximum_bytes: 1 }); drop(cursor); }
            2 => { let cursor = map.begin_lookup("key".into()); retire(map, Grant { maximum_items: 1, maximum_bytes: 1 }); drop(cursor); }
            _ => drop(map.retire()),
        }));
        assert!(guarded.is_err()); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn long_key_comparison_transfers_workers_and_retirement_counts_exact_key_bytes() {
    let prefix = "🌊".repeat(2048); let mut map = OrderedMap::new(); map.insert(prefix.clone() + "a", 1);
    let mut cursor = map.begin_set(prefix.clone() + "b", 2);
    let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
    assert!(retire(map, grant).is_empty());
    assert_eq!(advance_update(&mut cursor,grant), Step::Progress { completed_items: 1, completed_bytes: 1 });
    let mut cursor = std::thread::spawn(move || { let bytes = update(&mut cursor, grant); assert_eq!(bytes + 1, (8192 + 1) * 2); cursor }).join().unwrap();
    let map = cursor.take_result().unwrap(); close(&mut cursor, grant);
    let expected=map.keys().map(String::capacity).sum::<usize>()+map.len()*(SharedOwner::<Node<i32>>::allocation_bytes()+SharedOwner::<Entry<i32>>::allocation_bytes()+SharedOwner::<String>::allocation_bytes()+SharedOwner::<i32>::allocation_bytes());
    let mut retirement = map.retire(); let mut bytes = 0; let mut values = Vec::new();
    for _ in 0..100_000 {
        let physical=release_grant(&retirement,grant);
        match retirement.advance(physical) {
            RetirementStep::Progress { released_bytes, .. } => { assert!(released_bytes <= physical.maximum_release_bytes); bytes += released_bytes; }
            RetirementStep::ProcessedBytes(bytes)=>assert!(bytes<=physical.maximum_copy_bytes),
            RetirementStep::OwnedValue(value) => values.push(value),
            RetirementStep::Complete => break, RetirementStep::Blocked => panic!("retirement blocked"),
            RetirementStep::Failure(error)=>panic!("{error}"),
        }
    }
    assert!(retirement.is_empty()); assert_eq!(bytes, expected); values.sort(); assert_eq!(values, [1, 2]);
}
//#endregion 🧹️CancellationAndOwnership

//#region 📤️SharedOwnership
/// 🎟️ Original tiny logical grants remain separate from each exact whole physical backing grant.
#[test]
fn ordered_physical_retirement_uses_inline_frontier_and_charges_the_key_payload_down() {
    let mut key = String::with_capacity(8193);
    key.push_str("retained-key");
    let key_capacity = key.capacity();
    let key_payload = key.len();
    let mut map = OrderedMap::new();
    map.insert(key, ());
    let mut retirement = map.retire();
    assert_eq!(retirement.allocated_bytes(), 0);
    let mut released = 0usize;
    let mut saw_key = false;
    for _ in 0..MAX_AVL_HEIGHT + 16 + key_payload {
        if retirement.allocated_bytes() == key_capacity&&retirement.next_copy_byte_demand()==0 {
            saw_key = true;
            assert_eq!(retirement.next_close_byte_demand().unwrap(), key_capacity);
            assert!(matches!(retirement.advance(RetainedCloneGrant {maximum_items:0,maximum_release_bytes:key_capacity,maximum_depth:MAX_AVL_HEIGHT+3,..Default::default()}), RetirementStep::Blocked));
            for maximum_release_bytes in [0,1,key_capacity-1]{assert!(matches!(retirement.advance(RetainedCloneGrant::one_release_turn(maximum_release_bytes,MAX_AVL_HEIGHT+3)), RetirementStep::Blocked));}
            assert_eq!(retirement.allocated_bytes(), key_capacity);
        }
        let physical=release_grant(&retirement,Grant {maximum_items:1,maximum_bytes:1});
        match retirement.advance(physical) {
            RetirementStep::Progress { released_items, released_bytes } => {
                assert!(released_items <= 1 && released_bytes <= physical.maximum_release_bytes);
                released += released_bytes;
            }
            RetirementStep::OwnedValue(()) => {}
            RetirementStep::ProcessedBytes(bytes)=>assert!(bytes<=physical.maximum_copy_bytes),
            RetirementStep::Complete => break,
            RetirementStep::Blocked => panic!("positive ordered retirement grant blocked"),
            RetirementStep::Failure(error)=>panic!("{error}"),
        }
    }
    assert!(saw_key);
    assert_eq!(released, key_capacity+SharedOwner::<Node<()>>::allocation_bytes()+SharedOwner::<Entry<()>>::allocation_bytes()+SharedOwner::<String>::allocation_bytes()+SharedOwner::<()>::allocation_bytes());
    assert_eq!(retirement.allocated_bytes(), 0);
    assert!(retirement.terminal_is_empty());
}

#[test]
fn shared_release_is_empty_or_transfers_the_exact_final_frontier() {
    assert!(OrderedMap::<Payload>::new().retire().is_empty());
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/👥️shared-owner/🔣️.json")).unwrap();
    for maximum_bytes in [1, 64, 4096] {
        let drops = Arc::new(AtomicUsize::new(0));
        let key = fixture["key"]["text"].as_str().unwrap().repeat(fixture["key"]["repetitions"].as_u64().unwrap() as usize);
        let mut map = OrderedMap::new(); map.insert(key, Payload(Arc::clone(&drops)));
        let expected=map.keys().map(String::capacity).sum::<usize>()+SharedOwner::<Node<Payload>>::allocation_bytes()+SharedOwner::<Entry<Payload>>::allocation_bytes()+SharedOwner::<String>::allocation_bytes()+SharedOwner::<Payload>::allocation_bytes();
        assert!(retire(map.clone(),Grant {maximum_items:1,maximum_bytes}).is_empty());
        let mut retirement = map.retire();
        assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
        let mut released_bytes = 0; let mut payloads = Vec::new();
        loop {
            let physical=release_grant(&retirement,Grant {maximum_items:1,maximum_bytes});
            match retirement.advance(physical) {
                RetirementStep::Progress { released_items, released_bytes: bytes } => { assert!(released_items <= 1 && bytes <= physical.maximum_release_bytes); released_bytes += bytes; }
                RetirementStep::ProcessedBytes(bytes)=>assert!(bytes<=physical.maximum_copy_bytes),
                RetirementStep::OwnedValue(value) => payloads.push(value),
                RetirementStep::Complete => break, RetirementStep::Blocked => panic!("positive grant blocked"),
                RetirementStep::Failure(error)=>panic!("{error}"),
            }
        }
        assert_eq!(released_bytes, expected); assert!(retirement.is_empty()); assert_eq!(payloads.len(), 1);
        assert_eq!(drops.load(AtomicOrdering::SeqCst), 0); drop(payloads); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
    }
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn racing_shared_releases_transfer_one_final_root_without_payload_destruction() {
    let drops = Arc::new(AtomicUsize::new(0)); let mut map = OrderedMap::new(); map.insert("🌊".repeat(4096), Payload(Arc::clone(&drops)));
    let barrier = Arc::new(std::sync::Barrier::new(8));
    let mut owners: Vec<_> = (0..7).map(|_| map.clone()).collect(); owners.push(map);
    let workers: Vec<_> = owners.into_iter().map(|owner| { let barrier = Arc::clone(&barrier); std::thread::spawn(move || { let mut retirement=owner.retire();let physical=release_grant(&retirement,Grant {maximum_items:1,maximum_bytes:1});barrier.wait();match retirement.advance(physical){RetirementStep::Progress {released_items,released_bytes}=>{assert_eq!(released_items,1);assert!(released_bytes<=physical.maximum_release_bytes);},_=>panic!("root alias release failed")};retirement }) }).collect();
    let mut shared = 0; let mut finals = Vec::new();
    for worker in workers {let frontier=worker.join().unwrap();if frontier.is_empty(){shared+=1;}else{finals.push(frontier);}}
    assert_eq!(shared, 7); assert_eq!(finals.len(), 1); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    let mut retirement = finals.pop().unwrap(); let mut values = Vec::new();
    loop { let physical=release_grant(&retirement,Grant {maximum_items:1,maximum_bytes:1});match retirement.advance(physical) { RetirementStep::OwnedValue(value) => values.push(value), RetirementStep::Complete => break, RetirementStep::Failure(error)=>panic!("{error}"),_ => {} } }
    assert!(retirement.is_empty()); assert_eq!(values.len(), 1); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
}

#[test]
fn shared_upsert_preserves_key_and_nonclone_payload_allocations() {
    let drops = Arc::new(AtomicUsize::new(0)); let map = OrderedMap::new();
    let key = Arc::new("🌊".repeat(4096)); let value = Arc::new(Payload(Arc::clone(&drops)));
    let key_pointer = key.as_ptr(); let value_pointer = Arc::as_ptr(&value);
    let key=SharedOwner::admit_arc(key).unwrap_or_else(|_|panic!("unique key allocation"));let value=SharedOwner::admit_arc(value).unwrap_or_else(|_|panic!("unique payload allocation"));
    let mut cursor = map.begin_set_shared(key, value); let grant = Grant { maximum_items: 1, maximum_bytes: 1 };
    update(&mut cursor, grant); let result = cursor.take_result().unwrap();
    let (key, value) = result.iter().next().unwrap(); assert_eq!(key.as_ptr(), key_pointer); assert_eq!(value as *const Payload, value_pointer);
    assert!(retire(map,grant).is_empty()); assert!(close(&mut cursor, grant).is_empty());
    let values = retire(result, grant); assert_eq!(values.len(), 1); assert_eq!(drops.load(AtomicOrdering::SeqCst), 0);
    drop(values); assert_eq!(drops.load(AtomicOrdering::SeqCst), 1);
}
//#endregion 📤️SharedOwnership
