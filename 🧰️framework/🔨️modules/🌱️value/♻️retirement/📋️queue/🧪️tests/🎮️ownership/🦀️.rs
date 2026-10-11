//! 🧪️ Queue admission returns original owners and all frame/page births and releases match the allocator.

use crate::{retirement::queue::RetirementQueue,retained_clone::{RetainedCloneGrant,RetainedCloneStep},observe_retirement_allocations};

struct Unsupported(String);
impl crate::retirement::RetireOwned for Unsupported {
    fn retirement(self)->Box<dyn crate::retirement::RetirementCursor> {panic!("unsupported original owner must return before retirement construction")}
}

#[test]
fn retirement_queue_admits_original_frames_and_releases_exact_terminal_backings() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {for count in fixture["ownerCounts"].as_array().unwrap() {for copy in fixture["copyGrants"].as_array().unwrap() {
        let mut queue=RetirementQueue::default();let mut original=0;let mut births=0;let mut released=0;let mut copied=0;
        for _ in 0..count.as_u64().unwrap() {
            let (mut value,(a,r))=observe_retirement_allocations(||String::with_capacity(row["reservedCapacity"].as_u64().unwrap() as usize));value.push_str(row["text"].as_str().unwrap());original+=a-r;let pointer=value.as_ptr();
            while !queue.has_reserved_slot() {
                let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:queue.next_reserve_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:queue.len()+1};
                let (zero,(a,r))=observe_retirement_allocations(||queue.reserve_step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.copied_items,0);assert_eq!((a,r),(0,0));
                let (small,(a,r))=observe_retirement_allocations(||queue.reserve_step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(small.copied_items,0);assert_eq!((a,r),(0,0));
                let (step,(a,r))=observe_retirement_allocations(||queue.reserve_step(grant).unwrap());assert_eq!(step.retained_capacity_bytes,a);assert_eq!(r,0);births+=a;
            }
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:RetirementQueue::frame_birth_bytes::<String>(),maximum_release_bytes:0,maximum_depth:queue.len()+1};
            let (denied,(a,r))=observe_retirement_allocations(||queue.admit_owned(value,RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}));let (error,returned)=denied.unwrap_err();assert_eq!(error.kind,crate::ValueRefusalKind::OwnershipLimit);assert_eq!(returned.as_ptr(),pointer);assert_eq!((a,r),(0,0));value=returned;
            let (denied,(a,r))=observe_retirement_allocations(||queue.admit_owned(value,RetainedCloneGrant {maximum_depth:0,..grant}));let (error,returned)=denied.unwrap_err();assert_eq!(error.kind,crate::ValueRefusalKind::DepthLimit);assert_eq!(returned.as_ptr(),pointer);assert_eq!((a,r),(0,0));
            let (denied,(a,r))=observe_retirement_allocations(||queue.admit_owned(Unsupported(returned),grant));let (error,returned)=denied.unwrap_err();assert_eq!(error.kind,crate::ValueRefusalKind::UnsupportedOwner);assert_eq!(returned.0.as_ptr(),pointer);assert_eq!((a,r),(0,0));
            let (step,(a,r))=observe_retirement_allocations(||queue.admit_owned(returned.0,grant).unwrap());assert_eq!(step.retained_capacity_bytes,a);assert_eq!(r,0);births+=a;
        }
        assert_eq!(queue.len(),count.as_u64().unwrap() as usize);
        for turn in 0..1000000 {
            if queue.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:queue.next_capacity_byte_demand(copy.as_u64().unwrap() as usize).unwrap(),maximum_release_bytes:queue.next_release_byte_demand().unwrap(),maximum_depth:queue.next_depth_demand().unwrap()};
            let (zero,(a,r))=observe_retirement_allocations(||queue.step(RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress().copied_items,0);assert_eq!((a,r),(0,0));
            let (denied,(a,r))=observe_retirement_allocations(||queue.step(RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant}));assert_eq!(denied.unwrap_err().kind,crate::ValueRefusalKind::DepthLimit);assert_eq!((a,r),(0,0));
            if grant.maximum_capacity_bytes>0 {let (small,(a,r))=observe_retirement_allocations(||queue.step(RetainedCloneGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(small.progress().copied_items,0);assert_eq!((a,r),(0,0));}
            if grant.maximum_release_bytes>0 && queue.next_copy_byte_demand().unwrap()==0 {let (small,(a,r))=observe_retirement_allocations(||queue.step(RetainedCloneGrant {maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(small.progress().copied_items,0);assert_eq!((a,r),(0,0));}
            let (step,(a,r))=observe_retirement_allocations(||queue.step(grant).unwrap());let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(progress.copied_items<=grant.maximum_items);assert!(progress.copied_bytes<=grant.maximum_copy_bytes);assert!(a<=grant.maximum_capacity_bytes);assert!(r<=grant.maximum_release_bytes);births+=a;released+=r;copied+=progress.copied_bytes;
            assert!(progress.copied_items!=0 || matches!(step,RetainedCloneStep::Complete(_)),"exact queue grant blocked on turn {turn}");
        }
        assert!(queue.terminal_is_empty());assert_eq!(released,original+births);assert_eq!(copied,0);
        eprintln!("[DEBUG] Generic admitted retirement queue count={} copy={} original={original} births={births} physical={released}",count,copy);
    }}}
}

#[test]
fn partial_retirement_queue_transfers_to_the_actual_generic_retained_owner() {
    use crate::retirement::controlled::ControlledRetirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {for cut in fixture["cancelCuts"].as_array().unwrap() {for copy in fixture["copyGrants"].as_array().unwrap() {
        let mut queue=RetirementQueue::default();let mut original=0;let mut births=0;let mut released=0;let mut copied=0;
        for _ in 0..257 {
            let (mut value,(a,r))=observe_retirement_allocations(||String::with_capacity(row["reservedCapacity"].as_u64().unwrap() as usize));value.push_str(row["text"].as_str().unwrap());original+=a-r;
            while !queue.has_reserved_slot() {
                let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:queue.next_reserve_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:queue.len()+1};
                let (progress,(a,r))=observe_retirement_allocations(||queue.reserve_step(grant).unwrap());assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));births+=a;released+=r;
            }
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:RetirementQueue::frame_birth_bytes::<String>(),maximum_release_bytes:0,maximum_depth:queue.len()+1};
            let (progress,(a,r))=observe_retirement_allocations(||queue.admit_owned(value,grant).unwrap());assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));births+=a;released+=r;
        }
        for _ in 0..cut.as_u64().unwrap() {
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:queue.next_capacity_byte_demand(copy.as_u64().unwrap() as usize).unwrap(),maximum_release_bytes:queue.next_release_byte_demand().unwrap(),maximum_depth:queue.next_depth_demand().unwrap()};
            let (step,(a,r))=observe_retirement_allocations(||queue.step(grant).unwrap());assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));births+=a;released+=r;copied+=step.progress().copied_bytes;
        }
        let mut owner=ControlledRetirement::new(queue).map_err(|(error,_)|error).unwrap();
        for turn in 0..1000000 {
            if owner.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap() as usize,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy.as_u64().unwrap() as usize).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            let (step,(a,r))=observe_retirement_allocations(||owner.step(grant).unwrap());let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(a<=grant.maximum_capacity_bytes);assert!(r<=grant.maximum_release_bytes);assert!(progress.copied_bytes<=grant.maximum_copy_bytes);births+=a;released+=r;copied+=progress.copied_bytes;
            assert!(progress.copied_items!=0 || matches!(step,RetainedCloneStep::Complete(_)),"nested exact queue grant blocked on turn {turn}");
        }
        assert!(owner.terminal_is_empty());assert_eq!(released,original+births);assert_eq!(copied,0);
        eprintln!("[DEBUG] Partial Generic retirement queue transfer cutoff={cut} copy={copy} original={original} births={births} physical={released}");
    }}}
}


#[test]
fn retirement_queue_retains_original_snapshot_frames_and_closes_mixed_custody() {
    use crate::retirement::{admit_owned_retirement,owned_retirement_birth_bytes,controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for copy in fixture["copyGrants"].as_array().unwrap(){for cut in fixture["cancelCuts"].as_array().unwrap(){
        let mut queue=RetirementQueue::default();let mut originals=0;let mut births=0;let mut released=0;let mut copied=0;
        for kind in fixture["frameKinds"].as_array().unwrap(){
            let (mut input,(a,r))=observe_retirement_allocations(||String::with_capacity(fixture["snapshotCase"]["reservedCapacity"].as_u64().unwrap() as usize));input.push_str(fixture["snapshotCase"]["text"].as_str().unwrap());originals+=a-r;
            if kind=="snapshot"{
                let frame_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:owned_retirement_birth_bytes::<String>(),maximum_release_bytes:0,maximum_depth:1};
                let ((mut frame,p),(a,r))=observe_retirement_allocations(||admit_owned_retirement(input,frame_grant).map_err(|(error,_)|error).unwrap());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;
                let pointer=frame.as_ref() as *const dyn crate::ErasedSnapshotRetirement;
                let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:queue.len()+1};
                if !queue.has_reserved_slot(){let (result,heap)=observe_retirement_allocations(||queue.admit_retirement(frame,grant));let (error,original)=result.unwrap_err();assert_eq!(error.kind,crate::ValueRefusalKind::OwnershipLimit);assert!(std::ptr::addr_eq(pointer,original.as_ref() as *const dyn crate::ErasedSnapshotRetirement));assert_eq!(heap,(0,0));frame=original;}
                while !queue.has_reserved_slot(){let grant=RetainedCloneGrant{maximum_capacity_bytes:queue.next_reserve_capacity_byte_demand().unwrap(),..grant};let (p,(a,r))=observe_retirement_allocations(||queue.reserve_step(grant).unwrap());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;}
                for refused in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
                    let (result,heap)=observe_retirement_allocations(||queue.admit_retirement(frame,refused));let (_,original)=result.unwrap_err();assert!(std::ptr::addr_eq(pointer,original.as_ref() as *const dyn crate::ErasedSnapshotRetirement));assert_eq!(heap,(0,0));frame=original;
                }
                let (p,heap)=observe_retirement_allocations(||queue.admit_retirement(frame,grant).map_err(|(error,_)|error).unwrap());assert_eq!(p.copied_items,1);assert_eq!((p.copied_bytes,p.retained_capacity_bytes,p.released_bytes),(0,0,0));assert_eq!(heap,(0,0));
            }else{
                while !queue.has_reserved_slot(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:queue.next_reserve_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:queue.len()+1};let (p,(a,r))=observe_retirement_allocations(||queue.reserve_step(grant).unwrap());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;}
                let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:RetirementQueue::frame_birth_bytes::<String>(),maximum_release_bytes:0,maximum_depth:queue.len()+1};let (p,(a,r))=observe_retirement_allocations(||queue.admit_owned(input,grant).unwrap());assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));births+=a;released+=r;
            }
        }
        for _ in 0..cut.as_u64().unwrap(){
            if queue.terminal_is_empty(){break;}
            let body=copy.as_u64().unwrap() as usize;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:queue.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:queue.next_release_byte_demand().unwrap(),maximum_depth:queue.next_depth_demand().unwrap()};
            let (step,(a,r))=observe_retirement_allocations(||queue.step(grant).unwrap());let p=step.progress();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert!(p.fits(grant));assert!(!matches!(step,RetainedCloneStep::Complete(_))||queue.terminal_is_empty());births+=a;released+=r;copied+=p.copied_bytes;
        }
        let (mut owner,heap)=observe_retirement_allocations(||ControlledRetirement::new(queue).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));
        for turn in 0..1000000{
            if owner.terminal_is_empty(){break;}
            let body=copy.as_u64().unwrap() as usize;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body,maximum_capacity_bytes:owner.next_capacity_byte_demand(body).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};
            let (zero,heap)=observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!(heap,(0,0));
            let (denied,heap)=observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}));assert_eq!(denied.unwrap_err().kind,crate::ValueRefusalKind::DepthLimit);assert_eq!(heap,(0,0));
            if grant.maximum_capacity_bytes>0{let (small,heap)=observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(small.progress(),Default::default());assert_eq!(heap,(0,0));}
            if grant.maximum_release_bytes>0&&owner.next_copy_byte_demand().unwrap()==0{let (small,heap)=observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(small.progress(),Default::default());assert_eq!(heap,(0,0));}
            let (step,(a,r))=observe_retirement_allocations(||owner.step(grant).unwrap());let p=step.progress();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert!(p.fits(grant));assert!(p.copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)),"mixed original queue blocked on turn {turn}");births+=a;released+=r;copied+=p.copied_bytes;
        }
        assert!(owner.terminal_is_empty());assert_eq!(released,originals+births);assert_eq!(copied,0);let (_,heap)=observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));
        eprintln!("[DEBUG] Original mixed snapshot queue copy={copy} cancellation={cut} original={originals} births={births} physical={released} terminalDrop=0");
    }}
}
