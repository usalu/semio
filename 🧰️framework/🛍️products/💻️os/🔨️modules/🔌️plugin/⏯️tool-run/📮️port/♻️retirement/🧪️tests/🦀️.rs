//! 📮️ Native original port lease, atomic final queue and fixed-grant custody laws.
use std::sync::Arc;
use semio_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_plugin_store_lifecycle::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(test)]
use semio_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Mutation, MutationDiff};
use protocol::{OpBinary, OpText};
#[cfg(test)]
use semio_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_ui_scene::SceneDoc;
use semio_plugin_decl_views::tool_run_contract::{ToolRunJobPort, ToolRunRetargetableJob};
use crate::{ToolRunMemberEmissionOwner, ToolRunMemberOwners};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_plugin_window_config::WindowConfigOwner;
use semio_ui_contract::*;
use semio_framework::kernel::RequestId;
use semio_framework::Effect;
use semio_framework_value::{RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
use semio_plugin_decl_views::tool_run_contract::OriginalToolRunJobPortRetirement;
use std::mem::ManuallyDrop;
use semio_framework_value::RetirementDemand;

#[test]
fn original_tool_run_port_preserves_atomic_final_queue_and_full_grants(){
    let mut bank=crate::test_effect::bank();let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
    for reserve in[0,1,17]{let(mut source,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let port=ToolRunJobPort::default();port.inner.effects.write().unwrap().reserve(reserve);Some(port)});let mut born=heap.requested_bytes;let mut released=heap.released_bytes;let pointer=Arc::as_ptr(&source.as_ref().unwrap().inner);let quote=source.as_ref().unwrap().original_close_demands().unwrap();for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ToolRunJobPort::admit_original_close(&mut source,denied));assert!(result.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(Arc::as_ptr(&source.as_ref().unwrap().inner),pointer);}
        let((mut owner,progress),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ToolRunJobPort::admit_original_close(&mut source,grant).unwrap().unwrap());assert!(source.is_none());assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(Arc::as_ptr(&owner.source.as_ref().unwrap().inner),pointer);let mut turns=0;while !owner.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.demands().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}for grant in denied{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::test_effect::cursor_step(&mut bank,grant,|effect|owner.close_step(0,effect)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::test_effect::cursor_step(&mut bank,grant,|effect|owner.close_step(0,effect)).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);}assert_eq!(born,released);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
}

#[test]
fn original_tool_run_port_quotes_a_sole_queued_lease_and_refuses_only_live_weak_backing(){
    let port=ToolRunJobPort::default();let weak=Arc::downgrade(&port.inner);assert!(port.original_close_demands().is_err());drop(weak);
    port.dispatch(queued(3));assert_eq!(port.original_close_demands().unwrap(),RetirementDemand{copy_bytes:0,depth:1,..Default::default()});
}

fn queued(request:u64)->Effect{Effect::DispatchAction{req:RequestId(request),action:"original".into(),args:None,delay_ms:0}}

fn advance(bank:&mut semio_framework_value_testing_admission::EffectBank,grant:RetainedCloneGrant,owner:&mut OriginalToolRunJobPortRetirement)->(RetirementStep,(usize,usize)){
    let(outcome,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::test_effect::settled(bank,grant,|effect|{let step=owner.close_step(0,effect);let work=effect.performed_work();RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<_,()>(step),work})}));
    (outcome.result.unwrap(),(heap.requested_bytes,heap.released_bytes))
}

fn sole_owner(effects:u64)->(OriginalToolRunJobPortRetirement,Vec<Effect>,RetainedCloneGrant){
    let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
    let port=ToolRunJobPort::default();let expected:Vec<Effect>=(0..effects).map(queued).collect();for effect in expected.iter().cloned(){port.dispatch(effect);}
    let mut source=Some(port);let(owner,_)=ToolRunJobPort::admit_original_close(&mut source,grant).unwrap().unwrap();(owner,expected,grant)
}

fn retire_lease(bank:&mut semio_framework_value_testing_admission::EffectBank,grant:RetainedCloneGrant,owner:&mut OriginalToolRunJobPortRetirement){
    let(step,_)=advance(bank,grant,owner);assert!(matches!(step,RetirementStep::Progress(..)));assert!(owner.source.is_none());
}

#[test]
fn original_tool_run_port_retires_every_queued_final_effect_through_the_paid_cursor(){
    let mut bank=crate::test_effect::bank();let(mut owner,expected,grant)=sole_owner(3);retire_lease(&mut bank,grant,&mut owner);
    for effect in &expected{
        let quote=owner.demands().unwrap();assert_eq!(quote,semio_framework_value::retirement::retirement_cursor_birth_demand(effect).unwrap());assert_eq!(owner.next_demand(0).unwrap(),quote);
        let before=owner.effects.as_ref().unwrap().len();let(step,heap)=advance(&mut bank,grant,&mut owner);
        let RetirementStep::ChildProgress(child,progress,_)=step else{panic!("a queued effect retires as a funded child cursor")};std::mem::forget(child);
        assert!(progress.fits(grant));assert_eq!(progress.retained_capacity_bytes,quote.capacity_bytes);assert_eq!(heap.0,quote.capacity_bytes);
        assert_eq!(owner.effects.as_ref().unwrap().len(),before-1);assert!(owner.staged.is_none());
    }
    assert!(!owner.terminal_is_empty());let quote=owner.demands().unwrap();assert!(quote.release_bytes>0);
    let(step,_)=advance(&mut bank,grant,&mut owner);assert!(matches!(step,RetirementStep::Progress(progress,_) if progress.released_bytes==quote.release_bytes));assert!(owner.terminal_is_empty());
    let(step,heap)=advance(&mut bank,grant,&mut owner);assert!(matches!(step,RetirementStep::Complete(0)));assert_eq!(heap,(0,0));
}

#[test]
fn original_tool_run_port_quotes_effect_demands_exactly_before_any_move(){
    let mut bank=crate::test_effect::bank();let(mut owner,expected,grant)=sole_owner(2);retire_lease(&mut bank,grant,&mut owner);
    let exact=semio_framework_value::retirement::retirement_cursor_birth_demand(&expected[0]).unwrap();
    let(quote,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.demands().unwrap());assert_eq!(quote,exact);assert_eq!(heap.requested_bytes+heap.released_bytes,0);assert_eq!(owner.effects.as_ref().unwrap().len(),2);
    let pointer=owner.effects.as_ref().unwrap().as_slices().0.as_ptr();
    for denied in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:exact.depth-1,..grant},RetainedCloneGrant{maximum_capacity_bytes:exact.capacity_bytes-1,..grant},RetainedCloneGrant{maximum_copy_bytes:exact.copy_bytes-1,..grant}]{
        let(step,heap)=advance(&mut bank,denied,&mut owner);assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!(heap,(0,0));
        assert_eq!(owner.effects.as_ref().unwrap().len(),2);assert!(owner.staged.is_none());assert_eq!(owner.effects.as_ref().unwrap().as_slices().0.as_ptr(),pointer);assert_eq!(owner.demands().unwrap(),exact);
    }
    let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:exact.copy_bytes,maximum_capacity_bytes:exact.capacity_bytes,maximum_depth:exact.depth,..grant};
    let(step,_)=advance(&mut bank,funded,&mut owner);let RetirementStep::ChildProgress(child,progress,_)=step else{panic!("an exactly quoted grant funds the effect")};std::mem::forget(child);assert!(progress.fits(funded));
    while let Some(effect)=owner.effects.as_mut().and_then(std::collections::VecDeque::pop_front){drop(effect);}let _=ManuallyDrop::new(owner);
}

#[test]
fn original_tool_run_port_cancellation_mid_retirement_leaves_a_resumable_state(){
    let mut bank=crate::test_effect::bank();let(mut owner,_,grant)=sole_owner(3);retire_lease(&mut bank,grant,&mut owner);
    let(step,_)=advance(&mut bank,grant,&mut owner);let RetirementStep::ChildProgress(child,..)=step else{panic!("first effect retires")};std::mem::forget(child);
    for cancelled in[RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
        let(step,heap)=advance(&mut bank,cancelled,&mut owner);assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!(heap,(0,0));
        assert_eq!(owner.effects.as_ref().unwrap().len(),2);assert!(owner.staged.is_none());assert!(!owner.terminal_is_empty());
    }
    let mut children=0;while !owner.terminal_is_empty(){let(step,_)=advance(&mut bank,grant,&mut owner);match step{RetirementStep::ChildProgress(child,..)=>{children+=1;std::mem::forget(child);}RetirementStep::Progress(..)=>{}other=>panic!("resumed retirement made no progress: {}",other.work())}assert!(children<=2);}
    assert_eq!(children,2);
}
