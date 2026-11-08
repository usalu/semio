use super::*;
use semio_framework_value::{AuthoringWallet,RetainedAllocationEffect,retained_clone::RetainedCloneGrant as Grant};

/// 🏦️ Drives one step under an explicit finite authoring wallet equal to the granted stock: no job turn exists in these laws.
fn funded<R>(grant:Grant,step:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->R{AuthoringWallet::new(grant,1<<20).run(grant,1<<16,step).unwrap().expect("work quote admitted").output}
/// 🏁️ Reads a completed retirement poll as its funded step; a pending poll made no progress.
fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error),semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()))}}

/// 🔁️ Steps the retention job under `grant`; a refused quote retries once with copy and capacity widened to the job's exact quote, and the grant actually used is returned.
fn narrow_job(grant:Grant,job:&mut BrepRetentionJob,kernel:&mut Brep)->(semio_framework_value::retained_clone::RetainedCloneStep,Grant){
    let first=funded(grant,|effect|job.step(kernel,effect)).unwrap();
    if first.progress()!=Default::default()||job.terminal_is_empty(){return(first,grant)}
    let copy=grant.maximum_copy_bytes.max(job.next_copy_byte_demand(kernel).unwrap());
    let wide=Grant{maximum_copy_bytes:copy,maximum_capacity_bytes:grant.maximum_capacity_bytes.max(job.next_capacity_byte_demand(kernel,copy).unwrap()),..grant};
    (funded(wide,|effect|job.step(kernel,effect)).unwrap(),wide)
}

/// 🧹️ Original live handles, roots and body slots retire in exact funded turns without a copied directory.
#[test]
fn original_brep_retention_borrows_live_rows_and_restarts_changed_source(){
    use crate::test_support::observe_tessellation_system as observe;
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../📸️representation/🕸️topology/🧪️tests/🧫️fixtures/🎟️reachability/🔣️.json")).unwrap();
    for copy_grant in [1,3,64]{for changing in [false,true]{
        let((mut kernel,kept,discard),source)=observe(||{let mut kernel=Brep::new();let kept=kernel.box_prim_sync(1.,1.,1.).unwrap();let discard=kernel.box_prim_sync(2.,2.,2.).unwrap();(kernel,kept,discard)});
        let (mut job,heap)=observe(||BrepRetentionJob::new(&kernel));assert_eq!(heap,(0,0));let(mut born,mut freed,mut turns,mut changed)=(source.0,source.1,0,false);let mut new=None;
        while !job.terminal_is_empty(){
            if changing&&!changed&&turns==20{let(handle,heap)=observe(||kernel.box_prim_sync(3.,3.,3.).unwrap());born+=heap.0;freed+=heap.1;new=Some(handle);changed=true;}
            if let Some(handle)=job.candidate_handle(&kernel){let decision=handle==kept.as_str()||new.as_ref().is_some_and(|source|source.as_str()==handle);let(result,heap)=observe(||job.decide(&kernel,decision));result.unwrap();assert_eq!(heap,(0,0));}
            let copy=job.next_copy_byte_demand(&kernel).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy_grant.max(copy),maximum_capacity_bytes:job.next_capacity_byte_demand(&kernel,copy_grant).unwrap(),maximum_release_bytes:job.next_release_byte_demand(&kernel).unwrap(),maximum_depth:job.next_depth_demand(&kernel).unwrap()};
            let(step,heap)=observe(||funded(RetainedCloneGrant{maximum_items:0,..grant},|effect|job.step(&mut kernel,effect)).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let((step,grant),heap)=observe(||narrow_job(grant,&mut job,&mut kernel));assert!(step.progress().fits(grant));assert!(step.progress().copied_items<=1);assert!(step.progress().copied_items!=0||job.terminal_is_empty(),"original retained frontier stalls at turn {turns}: copyDemand={copy} grant={grant:?} receipt={:?} originalPhysical={born}/{freed} actualSystem={heap:?}",step.progress());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);assert!(kernel.live.get(kept.as_str()).is_some());
        }
        assert!(kernel.live.get(discard.as_str()).is_none());assert_eq!(kernel.body.entity_counts().solids,law["retention"]["keptBoxes"].as_u64().unwrap()as usize+usize::from(changing));let(_,heap)=observe(||drop(job));assert_eq!(heap,(0,0));
        let(mut handles,heap)=observe(||ControlledRetirement::new((kept.0,discard.0,new.map(|handle|handle.0))).unwrap_or_else(|(error,_)|panic!("original retained caller handles: {error}")));assert_eq!(heap,(0,0));while !handles.terminal_is_empty(){let copy=handles.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:handles.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:handles.next_release_byte_demand().unwrap(),maximum_depth:handles.next_depth_demand().unwrap()};let(step,heap)=observe(||settled(funded(grant,|effect|handles.step(effect))).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}let(_,heap)=observe(||drop(handles));assert_eq!(heap,(0,0));
        let(mut owner,heap)=observe(||ControlledRetirement::new(kernel).unwrap_or_else(|(error,_)|panic!("original kernel close: {error}")));assert_eq!(heap,(0,0));while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||settled(funded(grant,|effect|owner.step(effect))).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
    }}
}

/// 🧹️ An open mesh import blocks retention with a typed refusal, and releasing it unblocks retention.
#[test]
fn open_mesh_import_blocks_retention_with_a_typed_refusal() {
    let mut kernel=Brep::new();kernel.box_prim_sync(1.,1.,1.).unwrap();
    let mut cursor=MeshImportCursor::new(vec![0.,0.,0.,1.,0.,0.,0.,1.,0.],Vec::new(),vec![0,1,2],1e-6).unwrap();
    kernel.step_mesh_import_sync(&mut cursor,1).unwrap();
    let mut job=BrepRetentionJob::new(&kernel);
    let grant=Grant{maximum_items:1,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<16,maximum_release_bytes:1<<16,maximum_depth:1<<8};
    assert!(funded(grant,|effect|job.step(&mut kernel,effect)).is_err());
    drop(cursor);
    assert!(funded(grant,|effect|job.step(&mut kernel,effect)).is_ok());
}
