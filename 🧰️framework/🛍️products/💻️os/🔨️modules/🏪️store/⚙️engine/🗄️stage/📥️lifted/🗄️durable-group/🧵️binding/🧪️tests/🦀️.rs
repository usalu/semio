//! 🧪️ Native original candidates keep their issuer pair and addresses until funded cancellation.
use super::*;

#[global_allocator]
static ALLOCATOR:semio_framework_trace_memory_testing::RequestedAllocator=semio_framework_trace_memory_testing::RequestedAllocator;

fn close(owner:&mut DurableStorePreparedOwnerV1<bool,bool>,grant:RetainedCloneGrant)->bool{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.retire_originals_step(grant));let done=result.unwrap();assert!(owner.retirement_progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(owner.retirement_progress.retained_capacity_bytes,owner.retirement_progress.released_bytes));done}

#[test]
fn original_candidate_cancellation_retains_exact_issuers_and_refuses_unfunded_birth(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("./🔣️.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:fixture["maximumBodyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:fixture["maximumBodyBytes"].as_u64().unwrap()as usize,maximum_release_bytes:fixture["maximumBodyBytes"].as_u64().unwrap()as usize,maximum_depth:fixture["maximumDepth"].as_u64().unwrap()as usize};
    let actor=semio_framework_value::SharedUtf8::admit(fixture["actor"].as_str().unwrap().to_owned(),grant).unwrap_or_else(|_|panic!("funded fixture identity")).0;
    let authority=Arc::new(ArtifactStoreOneItemLiveAuthority::mint(semio_framework_job::OperationId(fixture["operation"].as_u64().unwrap()),semio_framework_job::Generation(fixture["generation"].as_u64().unwrap()),[fixture["revisionByte"].as_u64().unwrap()as u8;32],0,1,HybridLogicalTimestamp{actor:1,physical_ms:1,logical:0},actor,None,None,None));
    let edit=authority.next_edit(fixture["forward"].as_bool().unwrap(),vec![fixture["inverse"].as_bool().unwrap()]);
    let prepared=authority.prepare_one_item(edit,Arc::new(fixture["post"].as_bool().unwrap())).unwrap_or_else(|_|panic!("actual sealed fixture candidate"));
    drop(authority);
    let mut owner=DurableStorePreparedOwnerV1::<bool,bool>{retirement_progress:Default::default(),member_encoding:Default::default(),bound_encoding:Default::default(),original_retirement:None,issuer_retirement:None,prepared:Some(prepared),unbound:None,plan:None,bound:None,previous_authority:None,discarded_pack:None,mutation_retirement:Some(Arc::new(semio_framework_value::factory::OwnedValueRetirementFactory::<bool>::default())),snapshot_retirement:Some(Arc::new(semio_framework_value::factory::SharedValueRetirementFactory::<bool>::default()))};
    for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if owner.member_encoding.terminal_is_empty()&&owner.bound_encoding.terminal_is_empty(){break;}assert!(!close(&mut owner,grant));assert!(owner.retirement_progress.fits(grant));}
    assert!(owner.member_encoding.terminal_is_empty()&&owner.bound_encoding.terminal_is_empty());
    let address=owner.prepared().unwrap().edit()as *const Edit<bool>as usize;
    for axis in fixture["deniedBirthAxes"].as_array().unwrap(){
        let mut denied=grant;
        match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes=0,"capacity"=>denied.maximum_capacity_bytes=0,"depth"=>denied.maximum_depth=0,_=>panic!("declared birth axis")}
        assert!(!close(&mut owner,denied));assert_eq!(owner.retirement_progress,Default::default());assert_eq!(owner.prepared().unwrap().edit()as *const Edit<bool>as usize,address);assert!(owner.mutation_retirement.is_some()&&owner.snapshot_retirement.is_some());
    }
    let mut complete=false;
    for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){let done=close(&mut owner,grant);assert!(owner.retirement_progress.fits(grant));if done{complete=true;break;}}
    assert!(complete&&owner.originals_empty());
}
