//! 🧪️ Full shared factory grants preserve original aliases and every physical allocation.
use super::*;
use crate::{close_factory_ticket,factory_ticket_demands,OwnedValueRetirementFactory,OriginalRetirementFailure,SharedOriginalStorage,allocation_testing::observe_retirement_allocations,driven::{poll,replied,grant_of,CPU}};

fn turn(owner:&mut Option<Box<dyn ErasedSnapshotRetirement>>,copy:usize)->(usize,usize) {
    let demand=factory_ticket_demands(owner.as_ref().unwrap(),copy).unwrap();let work=if owner.as_ref().unwrap().terminal_is_empty(){1}else{owner.as_ref().unwrap().next_computational_work(copy).unwrap()};
    let g=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(copy),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    let(step,heap)=poll(RetainedCloneGrant {maximum_items:0,..g},work,|effect|close_factory_ticket(owner,copy,effect));assert_eq!(step.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));
    for denied in [(demand.copy_bytes!=0).then_some(RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..g}),(demand.capacity_bytes!=0).then_some(RetainedCloneGrant {maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..g}),(demand.release_bytes!=0).then_some(RetainedCloneGrant {maximum_release_bytes:demand.release_bytes.saturating_sub(1),..g})].into_iter().flatten(){
        let(step,heap)=poll(denied,work,|effect|close_factory_ticket(owner,copy,effect));assert_eq!(step.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));
    }
    let(step,heap)=poll(g,work,|effect|close_factory_ticket(owner,copy,effect));let step=step.result.unwrap();assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().copied_items!=0||owner.is_none(),"exact full factory owner stalled");heap
}

/// 🔗️ Admits one issued alias with its issuer into the shared owner frame under a funded reply and reports its settled outcome.
fn admit_shared(original:&mut Option<SharedOriginalStorage<String>>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<String>>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,g:RetainedCloneGrant)->(RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>,(usize,usize),Option<usize>){replied(g,CPU,|effect,reply|FactorySharedRetirement::admit_original_into(original,factory,output,effect,reply))}

#[test]
fn original_factory_shared_aliases_cancel_with_exact_full_grants_and_physical_receipts() {
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(law["ownershipMoves"],serde_json::json!({"items":1}));
    for text in law["texts"].as_array().unwrap(){for count in law["aliases"].as_array().unwrap(){for copy in law["copyBytes"].as_array().unwrap(){for cancel in law["cancelAt"].as_array().unwrap(){
        let text=text.as_str().unwrap();let count=count.as_u64().unwrap()as usize;let copy=copy.as_u64().unwrap()as usize;let cancel=cancel.as_u64().unwrap()as usize;
        let(root,heap)=observe_retirement_allocations(||SharedOriginalStorage::from_cold(text.to_owned()));let mut original=heap.0;let pointer=root.identity();
        let(factory,heap)=observe_retirement_allocations(||Arc::new(OwnedValueRetirementFactory::<String>::default()));original+=heap.0;
        let exact:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=factory;
        let mut resident=Some(root);let g=grant_of(FactorySharedRetirement::<String>::original_admission_demands(&resident));let root=resident.take().unwrap();let bytes=FactorySharedRetirement::<String>::constructor_capacity_bytes();assert_eq!(g.maximum_capacity_bytes,bytes);
        let mut aliases=vec![root];for _ in 1..count{let next=aliases[0].alias();aliases.push(next);}
        let(mut births,mut released)=(0,0);
        let mut owners:Vec<Option<Box<dyn ErasedSnapshotRetirement>>>=Vec::new();
        for alias in aliases{
            let mut held=Some(alias);let mut issuer=Some(exact.clone());let mut output=None;
            for denied in [RetainedCloneGrant {maximum_items:0,..g},RetainedCloneGrant {maximum_capacity_bytes:bytes-1,..g},RetainedCloneGrant {maximum_depth:0,..g}]{let(turn,heap,_)=admit_shared(&mut held,&mut issuer,&mut output,denied);assert_eq!(turn.result.unwrap().progress(),Default::default());assert!(output.is_none());assert_eq!(held.as_ref().unwrap().identity(),pointer);assert!(Arc::ptr_eq(issuer.as_ref().unwrap(),&exact));assert_eq!(heap,(0,0));assert_eq!(serde_json::to_value(held.as_ref().unwrap().get()).unwrap(),text);}
            let(turn,heap,_)=admit_shared(&mut held,&mut issuer,&mut output,g);let p=turn.result.unwrap().progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,0));births+=heap.0;assert!(held.is_none()&&issuer.is_none());owners.push(output);
        }
        let(_,heap)=observe_retirement_allocations(||drop(exact));assert_eq!(heap,(0,0));
        for owner in &mut owners{for _ in 0..cancel{if owner.is_none(){break;}let(b,r)=turn(owner,copy);births+=b;released+=r;}}
        for owner in &mut owners{for turn_index in 0..100000{if owner.is_none(){break;}let(b,r)=turn(owner,copy);births+=b;released+=r;assert!(turn_index<99999,"shared factory full closure exceeded bounded original law");}assert!(owner.is_none());}
        assert_eq!(released,original+births);
    }}}}
}

#[path="🔐️custody/🦀️.rs"]
mod custody;

#[test]
fn original_shared_birth_reports_actual_action_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../⛽️birth/fixture.json")).unwrap();let mut original=Some(SharedOriginalStorage::from_cold(String::from("actual shared owner")));let pointer=original.as_ref().unwrap().identity();let mut factory:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<String>>>=Some(Arc::new(OwnedValueRetirementFactory::<String>::default()));let grant=grant_of(FactorySharedRetirement::<String>::original_admission_demands(&original));let denied=RetainedCloneGrant{maximum_items:0,..grant};let mut output=None;
 assert_eq!(FactorySharedRetirement::<String>::admit_original_source_work_demand(&original),law["allocationRefusal"].as_u64().unwrap());
 let(outcome,heap,_)=admit_shared(&mut original,&mut factory,&mut output,denied);assert_eq!(outcome.work,u128::from(law["denied"].as_u64().unwrap()));assert_eq!(outcome.result.unwrap().progress(),Default::default());assert!(output.is_none());assert_eq!(heap,(0,0));assert_eq!(original.as_ref().unwrap().identity(),pointer);
 let(outcome,heap,_)=admit_shared(&mut original,&mut factory,&mut output,grant);assert_eq!(outcome.work,u128::from(law["success"].as_u64().unwrap()));let progress=outcome.result.unwrap().progress();assert!(original.is_none());assert_eq!(FactorySharedRetirement::<String>::admit_original_source_work_demand(&original),law["denied"].as_u64().unwrap());assert_eq!(heap,(progress.retained_capacity_bytes,0));let mut owner=output;for _ in 0..100000{if owner.is_none(){break}turn(&mut owner,4096);}assert!(owner.is_none());
}
