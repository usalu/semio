//! 🧪️ Full shared factory grants preserve original aliases and every physical allocation.
use super::*;
use crate::{close_factory_ticket,factory_ticket_demands,OwnedValueRetirementFactory,allocation_testing::observe_retirement_allocations};

fn turn(owner:&mut Option<Box<dyn ErasedSnapshotRetirement>>,copy:usize)->(usize,usize) {
    let demand=factory_ticket_demands(owner.as_ref().unwrap(),copy).unwrap();
    assert!(demand.copy_bytes<=copy,"shared ownership metadata must not self-enlarge a narrow payload copy grant");
    let g=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
    let(step,heap)=observe_retirement_allocations(||close_factory_ticket(owner,RetainedCloneGrant {maximum_items:0,..g}).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
    for denied in [(demand.copy_bytes!=0).then_some(RetainedCloneGrant {maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..g}),(demand.capacity_bytes!=0).then_some(RetainedCloneGrant {maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..g}),(demand.release_bytes!=0).then_some(RetainedCloneGrant {maximum_release_bytes:demand.release_bytes.saturating_sub(1),..g})].into_iter().flatten(){
        let(step,heap)=observe_retirement_allocations(||close_factory_ticket(owner,denied).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
    }
    let(step,heap)=observe_retirement_allocations(||close_factory_ticket(owner,g).result.unwrap());assert!(step.progress().fits(g));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().copied_items!=0||owner.is_none(),"exact full factory owner stalled");heap
}

#[test]
fn original_factory_shared_aliases_cancel_with_exact_full_grants_and_physical_receipts() {
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    assert_eq!(law["ownershipMoves"],serde_json::json!({"items":1,"copyBytes":0}));
    for text in law["texts"].as_array().unwrap(){for count in law["aliases"].as_array().unwrap(){for copy in law["copyBytes"].as_array().unwrap(){for cancel in law["cancelAt"].as_array().unwrap(){
        let text=text.as_str().unwrap();let count=count.as_u64().unwrap()as usize;let copy=copy.as_u64().unwrap()as usize;let cancel=cancel.as_u64().unwrap()as usize;
        let(root,heap)=observe_retirement_allocations(||Arc::new(text.to_owned()));let mut original=heap.0;let pointer=Arc::as_ptr(&root);
        let(factory,heap)=observe_retirement_allocations(||Arc::new(OwnedValueRetirementFactory::<String>::default()));original+=heap.0;
        let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=factory;
        let bytes=FactorySharedRetirement::<String>::constructor_capacity_bytes();
        let g=RetainedCloneGrant {maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
        for denied in [RetainedCloneGrant {maximum_items:0,..g},RetainedCloneGrant {maximum_capacity_bytes:bytes-1,..g},RetainedCloneGrant {maximum_depth:0,..g}]{
            let((_,same,same_factory),heap)=observe_retirement_allocations(||FactorySharedRetirement::admit(root.clone(),factory.clone(),denied).result.err().unwrap());assert_eq!(Arc::as_ptr(&same),pointer);assert!(Arc::ptr_eq(&same_factory,&factory));assert_eq!(heap,(0,0));assert_eq!(serde_json::to_value(same.as_ref()).unwrap(),text);drop(same);drop(same_factory);
        }
        let(mut births,mut released)=(0,0);
        let mut owners:Vec<_>=(0..count).map(|_|{let((owner,p),heap)=observe_retirement_allocations(||FactorySharedRetirement::admit(root.clone(),factory.clone(),g).result.unwrap_or_else(|_|panic!("exact original shared factory admission")));assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,0));births+=heap.0;Some(owner)}).collect();
        let(_,heap)=observe_retirement_allocations(||{drop(root);drop(factory);});assert_eq!(heap,(0,0));
        for owner in &mut owners{for _ in 0..cancel{if owner.is_none(){break;}let(b,r)=turn(owner,copy);births+=b;released+=r;}}
        for owner in &mut owners{for turn_index in 0..100000{if owner.is_none(){break;}let(b,r)=turn(owner,copy);births+=b;released+=r;assert!(turn_index<99999,"shared factory full closure exceeded bounded original law");}assert!(owner.is_none());}
        assert_eq!(released,original+births);
    }}}}
}

#[path="🔐️custody/🦀️.rs"]
mod custody;

#[test]
fn original_shared_birth_reports_actual_action_work(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../⛽️birth/fixture.json")).unwrap();let mut original=Some(Arc::new(String::from("actual shared owner")));let pointer=Arc::as_ptr(original.as_ref().unwrap());let factory:Arc<dyn ArtifactOwnedValueRetirementFactory<String>>=Arc::new(OwnedValueRetirementFactory::<String>::default());let bytes=FactorySharedRetirement::<String>::constructor_capacity_bytes();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};let denied=RetainedCloneGrant{maximum_items:0,..grant};
 assert_eq!(FactorySharedRetirement::<String>::admit_original_source_work_demand(&original),law["allocationRefusal"].as_u64().unwrap());
 assert_eq!(FactorySharedRetirement::<String>::admit_original_work_demand(&original,denied),law["denied"].as_u64().unwrap());let(outcome,heap)=observe_retirement_allocations(||FactorySharedRetirement::admit_original(&mut original,&factory,denied));assert_eq!(outcome.work,law["denied"].as_u64().unwrap());assert!(outcome.result.is_err());assert_eq!(heap,(0,0));assert_eq!(Arc::as_ptr(original.as_ref().unwrap()),pointer);
 assert_eq!(FactorySharedRetirement::<String>::admit_original_work_demand(&original,grant),law["allocationRefusal"].as_u64().unwrap());let(outcome,heap)=observe_retirement_allocations(||FactorySharedRetirement::admit_original(&mut original,&factory,grant));assert_eq!(outcome.work,law["success"].as_u64().unwrap());let(owner,progress)=outcome.result.unwrap().unwrap();assert!(original.is_none());assert_eq!(FactorySharedRetirement::<String>::admit_original_source_work_demand(&original),law["denied"].as_u64().unwrap());assert_eq!(heap,(progress.retained_capacity_bytes,0));let mut owner=Some(owner);for _ in 0..100000{if owner.is_none(){break}turn(&mut owner,4096);}assert!(owner.is_none());
}
