use super::*;

/// 🧹️ Every original arena slot and removed payload closes with an exact physical receipt while kept generations remain unchanged.
#[test]
fn original_body_compaction_turns_own_all_physical_effects() {
    use crate::brep::{operations::primitives::make_box,representation::topology::history};
    use crate::brep::queries::tessellation::tests::observe_tessellation_system as observe;
    use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️reachability/🔣️.json")).unwrap();
    for cancel in std::iter::once(None).chain(law["compaction"]["cancelTurns"].as_array().unwrap().iter().map(|turn|Some(turn.as_u64().unwrap()as usize))) {
        let ((mut body,solid,discard),source)=observe(||{let mut body=Body::new();let mut rec=history::OpRecorder::new();let solid=make_box(&mut body,1.,1.,1.,&mut rec).unwrap();let discard=make_box(&mut body,2.,2.,2.,&mut rec).unwrap();(body,solid,discard)});
        let(keep,membership)=observe(||body.reachable_from(&[EntityRef::Solid(solid)]));let kept=keep.solids.get(&solid).copied().unwrap();
        let(mut job,heap)=observe(||BodyCompactionJob::new(keep).unwrap_or_else(|(error,_)|panic!("original compact admission: {error}")));assert_eq!(heap,(0,0));
        let(mut born,mut freed,mut turns)=(source.0+membership.0,source.1+membership.1,0);
        while !job.terminal_is_empty() {
            if cancel==Some(turns){let(_,heap)=observe(||job.cancel());assert_eq!(heap,(0,0));}
            let copy=job.next_copy_byte_demand(&body).unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_capacity_byte_demand(&body,copy).unwrap(),maximum_release_bytes:job.next_release_byte_demand(&body).unwrap(),maximum_depth:job.next_depth_demand(&body).unwrap()};
            for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(grant.maximum_capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}),(grant.maximum_release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant}),(grant.maximum_depth>0).then_some(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant})].into_iter().flatten(){let(result,heap)=observe(||job.step(&mut body,denied));match result{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>{assert!(denied.maximum_depth<grant.maximum_depth);assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::DepthLimit);assert_eq!(error.retained_progress(),Default::default());}}assert_eq!(job.normal_step_progress(),Default::default());assert_eq!(heap,(0,0));}
            let(step,heap)=observe(||job.step(&mut body,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().copied_items<=1);born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);assert!(body.solids.is_live(kept));
        }
        if cancel.is_none(){assert!(!body.solids.is_live(discard));assert_eq!(body.entity_counts().vertices,law["box"]["vertices"].as_u64().unwrap()as usize);assert_eq!(job.freed().freed_solids,1);}
        let(_,heap)=observe(||drop(job));assert_eq!(heap,(0,0));
        let(mut owner,heap)=observe(||ControlledRetirement::new(body).unwrap_or_else(|(error,_)|panic!("original body closure: {error}")));assert_eq!(heap,(0,0));
        while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}
        assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));eprintln!("[DEBUG] Original compaction cancel={cancel:?} oneItem=true keptGenerations=true physical={freed} turns={turns} terminalDrop=0");
    }
}
