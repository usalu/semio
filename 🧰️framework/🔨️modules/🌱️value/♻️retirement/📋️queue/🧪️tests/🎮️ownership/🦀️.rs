//! 🧪️ Queue admission returns original owners and all frame/page births and releases match the allocator.

use crate::{RetirementQueue,retained_clone::{RetainedCloneGrant,RetainedCloneStep},observe_retirement_allocations,queue_bank::{bank,grant,kind,poll,quoted,sync}};

struct Unsupported(String);
/// 🛑️ Deliberately refuses controlled retirement so the queue admission returns the original frame owner.
impl crate::retirement::RetireOwned for Unsupported {
    type RetirementCursor=RetirementQueue;
    fn controlled_retirement_supported() -> bool { false }
    fn retirement_seed(self)->RetirementQueue {panic!("unsupported original owner must return before retirement construction")}
}

fn reserve_grant(queue:&RetirementQueue)->RetainedCloneGrant{let d=queue.reserve_demand().unwrap();RetainedCloneGrant{maximum_depth:d.depth.max(queue.len()+1),..grant(d)}}

#[test]
fn retirement_queue_admits_original_frames_and_releases_exact_terminal_backings() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {for count in fixture["ownerCounts"].as_array().unwrap() {for copy in fixture["copyGrants"].as_array().unwrap() {
        let mut bank=bank();let mut queue=RetirementQueue::default();let mut original=0;let mut births=0;let mut released=0;let mut copied=0;let mut budget=0;
        for _ in 0..count.as_u64().unwrap() {
            let (mut value,(a,r))=observe_retirement_allocations(||String::with_capacity(row["reservedCapacity"].as_u64().unwrap() as usize));value.push_str(row["text"].as_str().unwrap());original+=a-r;let pointer=value.as_ptr();
            while !queue.has_reserved_slot() {
                let g=reserve_grant(&queue);
                let ((zero,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,RetainedCloneGrant {maximum_items:0,..g},1,|effect|queue.reserve_step(effect)));assert_eq!(zero.result.unwrap().copied_items,0);assert_eq!((a,r),(0,0));
                let ((small,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,RetainedCloneGrant {maximum_capacity_bytes:g.maximum_capacity_bytes-1,..g},1,|effect|queue.reserve_step(effect)));assert_eq!(small.result.unwrap().copied_items,0);assert_eq!((a,r),(0,0));
                let ((step,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,g,1,|effect|queue.reserve_step(effect)));let step=step.result.unwrap();assert_eq!(step.retained_capacity_bytes,a);assert_eq!(r,0);bank.post(step,g,(a,r));births+=a;
            }
            let mut slot=Some(value);let g=grant(queue.admit_owned_demand(slot.as_ref().unwrap()).unwrap());
            for denied in [RetainedCloneGrant {maximum_capacity_bytes:g.maximum_capacity_bytes-1,..g},RetainedCloneGrant {maximum_depth:0,..g}] {let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,denied,2,|effect|queue.admit_owned(&mut slot,effect)));assert!(turn.result.unwrap().is_none());assert_eq!(turn.work,0);assert_eq!(slot.as_ref().unwrap().as_ptr(),pointer);assert_eq!((a,r),(0,0));}
            let mut unsupported=Some(Unsupported(slot.take().unwrap()));let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,g,2,|effect|queue.admit_owned(&mut unsupported,effect)));assert_eq!(turn.result.unwrap_err().kind,crate::ValueRefusalKind::UnsupportedOwner);assert_eq!(unsupported.as_ref().unwrap().0.as_ptr(),pointer);assert_eq!((a,r),(0,0));slot=unsupported.map(|owner|owner.0);
            let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,g,2,|effect|queue.admit_owned(&mut slot,effect)));let step=turn.result.unwrap().unwrap();assert_eq!(step.retained_capacity_bytes,a);assert_eq!(r,0);bank.post(step,g,(a,r));births+=a;assert!(slot.is_none());
        }
        assert_eq!(queue.len(),count.as_u64().unwrap() as usize);
        for turn in 0..1000000 {
            if queue.terminal_is_empty(){break;}
            let body=copy.as_u64().unwrap() as usize;let g=quoted(body,|b|queue.next_demand(b).unwrap());let units=queue.next_computational_work(g.maximum_copy_bytes).unwrap();budget+=queue.next_demand(g.maximum_copy_bytes).unwrap().copy_bytes;
            let ((zero,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant {maximum_items:0,..g},units,|effect|queue.step(effect)));assert_eq!(zero.result.unwrap().progress().copied_items,0);assert_eq!((a,r),(0,0));
            let ((denied,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant {maximum_depth:g.maximum_depth-1,..g},units,|effect|queue.step(effect)));assert_eq!(kind(&denied.result.unwrap_err()),crate::ValueRefusalKind::DepthLimit);assert_eq!((a,r),(0,0));
            if g.maximum_capacity_bytes>0 {let ((small,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant {maximum_capacity_bytes:g.maximum_capacity_bytes-1,..g},units,|effect|queue.step(effect)));assert_eq!(small.result.unwrap().progress().copied_items,0);assert_eq!((a,r),(0,0));}
            if g.maximum_release_bytes>0 && queue.next_copy_byte_demand().unwrap()==0 {let ((small,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant {maximum_release_bytes:g.maximum_release_bytes-1,..g},units,|effect|queue.step(effect)));assert_eq!(small.result.unwrap().progress().copied_items,0);assert_eq!((a,r),(0,0));}
            let ((step,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,g,units,|effect|queue.step(effect)));let step=step.result.unwrap();let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(progress.copied_items<=g.maximum_items);assert!(progress.copied_bytes<=g.maximum_copy_bytes);assert!(a<=g.maximum_capacity_bytes);assert!(r<=g.maximum_release_bytes);bank.post(progress,g,(a,r));births+=a;released+=r;copied+=progress.copied_bytes;
            assert!(progress.copied_items!=0 || matches!(step,RetainedCloneStep::Complete(_)),"exact queue grant blocked on turn {turn}");
        }
        assert!(queue.terminal_is_empty());assert_eq!(released,original+births);assert!(copied<=budget);
    }}}
}

#[test]
fn partial_retirement_queue_transfers_to_the_actual_generic_retained_owner() {
    use crate::retirement::controlled::ControlledRetirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {for cut in fixture["cancelCuts"].as_array().unwrap() {for copy in fixture["copyGrants"].as_array().unwrap() {
        let mut bank=bank();let mut queue=RetirementQueue::default();let mut original=0;let mut births=0;let mut released=0;let mut copied=0;let mut budget=0;
        for _ in 0..257 {
            let (mut value,(a,r))=observe_retirement_allocations(||String::with_capacity(row["reservedCapacity"].as_u64().unwrap() as usize));value.push_str(row["text"].as_str().unwrap());original+=a-r;
            while !queue.has_reserved_slot() {
                let g=reserve_grant(&queue);
                let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,g,1,|effect|queue.reserve_step(effect)));let progress=turn.result.unwrap();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));bank.post(progress,g,(a,r));births+=a;released+=r;
            }
            let mut slot=Some(value);let g=grant(queue.admit_owned_demand(slot.as_ref().unwrap()).unwrap());
            let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,g,2,|effect|queue.admit_owned(&mut slot,effect)));let progress=turn.result.unwrap().unwrap();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));bank.post(progress,g,(a,r));births+=a;released+=r;
        }
        for _ in 0..cut.as_u64().unwrap() {
            let body=copy.as_u64().unwrap() as usize;let g=quoted(body,|b|queue.next_demand(b).unwrap());let units=queue.next_computational_work(g.maximum_copy_bytes).unwrap();budget+=queue.next_demand(g.maximum_copy_bytes).unwrap().copy_bytes;
            let ((step,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,g,units,|effect|queue.step(effect)));let step=step.result.unwrap();assert_eq!((step.progress().retained_capacity_bytes,step.progress().released_bytes),(a,r));bank.post(step.progress(),g,(a,r));births+=a;released+=r;copied+=step.progress().copied_bytes;
        }
        let mut owner=ControlledRetirement::new(queue).map_err(|(error,_)|error).unwrap();
        for turn in 0..1000000 {
            if owner.terminal_is_empty(){break;}
            let body=copy.as_u64().unwrap() as usize;let g=quoted(body,|b|owner.next_demand(b).unwrap());let units=owner.next_computational_work(g.maximum_copy_bytes).unwrap();budget+=owner.next_demand(g.maximum_copy_bytes).unwrap().copy_bytes;
            let ((step,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,g,units,|effect|owner.step(effect)));let step=step.result.unwrap();let progress=step.progress();assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(a,r));assert!(a<=g.maximum_capacity_bytes);assert!(r<=g.maximum_release_bytes);assert!(progress.copied_bytes<=g.maximum_copy_bytes);bank.post(progress,g,(a,r));births+=a;released+=r;copied+=progress.copied_bytes;
            assert!(progress.copied_items!=0 || matches!(step,RetainedCloneStep::Complete(_)),"nested exact queue grant blocked on turn {turn}");
        }
        assert!(owner.terminal_is_empty());assert_eq!(released,original+births);assert!(copied<=budget);
    }}}
}


#[test]
fn retirement_queue_retains_original_snapshot_frames_and_closes_mixed_custody() {
    use crate::retirement::{admit_owned_retirement,controlled::{ControlledRetirement,controlled_retirement_birth_demand}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for copy in fixture["copyGrants"].as_array().unwrap(){for cut in fixture["cancelCuts"].as_array().unwrap(){
        let mut bank=bank();let mut queue=RetirementQueue::default();let mut originals=0;let mut births=0;let mut released=0;let mut copied=0;let mut budget=0;
        for kind in fixture["frameKinds"].as_array().unwrap(){
            let (mut input,(a,r))=observe_retirement_allocations(||String::with_capacity(fixture["snapshotCase"]["reservedCapacity"].as_u64().unwrap() as usize));input.push_str(fixture["snapshotCase"]["text"].as_str().unwrap());originals+=a-r;
            if kind=="snapshot"{
                let frame_grant=grant(controlled_retirement_birth_demand(&input).unwrap());let mut source=Some(input);
                let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,frame_grant,1,|effect|admit_owned_retirement(&mut source,effect)));let (frame,p)=turn.result.unwrap().unwrap();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));bank.post(p,frame_grant,(a,r));births+=a;released+=r;assert!(source.is_none());
                let mut frame=Some(frame);let pointer=frame.as_deref().unwrap() as *const dyn crate::ErasedSnapshotRetirement;
                let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetirementQueue::placement_bytes::<dyn crate::ErasedSnapshotRetirement>(),maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:queue.placement_depth()};
                if !queue.has_reserved_slot(){let ((turn,_),heap)=observe_retirement_allocations(||sync(&mut bank,g,1,|effect|queue.admit_retirement(&mut frame,effect)));assert!(turn.result.unwrap().is_none());assert_eq!(turn.work,0);assert!(std::ptr::addr_eq(pointer,frame.as_deref().unwrap() as *const dyn crate::ErasedSnapshotRetirement));assert_eq!(heap,(0,0));}
                while !queue.has_reserved_slot(){let reserve=reserve_grant(&queue);let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,reserve,1,|effect|queue.reserve_step(effect)));let p=turn.result.unwrap();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));bank.post(p,reserve,(a,r));births+=a;released+=r;}
                let g=RetainedCloneGrant{maximum_depth:queue.placement_depth(),..g};
                for refused in [RetainedCloneGrant{maximum_items:0,..g},RetainedCloneGrant{maximum_depth:0,..g}]{
                    let ((turn,_),heap)=observe_retirement_allocations(||sync(&mut bank,refused,1,|effect|queue.admit_retirement(&mut frame,effect)));assert!(turn.result.unwrap().is_none());assert_eq!(turn.work,0);assert!(std::ptr::addr_eq(pointer,frame.as_deref().unwrap() as *const dyn crate::ErasedSnapshotRetirement));assert_eq!(heap,(0,0));
                }
                let ((turn,_),heap)=observe_retirement_allocations(||sync(&mut bank,g,1,|effect|queue.admit_retirement(&mut frame,effect)));let p=turn.result.unwrap().unwrap();assert_eq!(p.copied_items,1);assert_eq!((p.copied_bytes,p.retained_capacity_bytes,p.released_bytes),(g.maximum_copy_bytes,0,0));assert_eq!(heap,(0,0));assert!(frame.is_none());bank.post(p,g,heap);
            }else{
                while !queue.has_reserved_slot(){let reserve=reserve_grant(&queue);let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,reserve,1,|effect|queue.reserve_step(effect)));let p=turn.result.unwrap();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));bank.post(p,reserve,(a,r));births+=a;released+=r;}
                let mut slot=Some(input);let g=grant(queue.admit_owned_demand(slot.as_ref().unwrap()).unwrap());let ((turn,_),(a,r))=observe_retirement_allocations(||sync(&mut bank,g,2,|effect|queue.admit_owned(&mut slot,effect)));let p=turn.result.unwrap().unwrap();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));bank.post(p,g,(a,r));births+=a;released+=r;
            }
        }
        for _ in 0..cut.as_u64().unwrap(){
            if queue.terminal_is_empty(){break;}
            let body=copy.as_u64().unwrap() as usize;let g=quoted(body,|b|queue.next_demand(b).unwrap());let units=queue.next_computational_work(g.maximum_copy_bytes).unwrap();budget+=queue.next_demand(g.maximum_copy_bytes).unwrap().copy_bytes;
            let ((step,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,g,units,|effect|queue.step(effect)));let step=step.result.unwrap();let p=step.progress();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert!(p.fits(g));assert!(!matches!(step,RetainedCloneStep::Complete(_))||queue.terminal_is_empty());bank.post(p,g,(a,r));births+=a;released+=r;copied+=p.copied_bytes;
        }
        let (mut owner,heap)=observe_retirement_allocations(||ControlledRetirement::new(queue).map_err(|(error,_)|error).unwrap());assert_eq!(heap,(0,0));
        for turn in 0..1000000{
            if owner.terminal_is_empty(){break;}
            let body=copy.as_u64().unwrap() as usize;let g=quoted(body,|b|owner.next_demand(b).unwrap());let units=owner.next_computational_work(g.maximum_copy_bytes).unwrap();budget+=owner.next_demand(g.maximum_copy_bytes).unwrap().copy_bytes;
            let ((zero,_),heap)=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant{maximum_items:0,..g},units,|effect|owner.step(effect)));assert_eq!(zero.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));
            let ((denied,_),heap)=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g},units,|effect|owner.step(effect)));assert_eq!(kind(&denied.result.unwrap_err()),crate::ValueRefusalKind::DepthLimit);assert_eq!(heap,(0,0));
            if g.maximum_capacity_bytes>0{let ((small,_),heap)=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant{maximum_capacity_bytes:g.maximum_capacity_bytes-1,..g},units,|effect|owner.step(effect)));assert_eq!(small.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));}
            if g.maximum_release_bytes>0&&owner.next_copy_byte_demand().unwrap()==0{let ((small,_),heap)=observe_retirement_allocations(||poll(&mut bank,RetainedCloneGrant{maximum_release_bytes:g.maximum_release_bytes-1,..g},units,|effect|owner.step(effect)));assert_eq!(small.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));}
            let ((step,_),(a,r))=observe_retirement_allocations(||poll(&mut bank,g,units,|effect|owner.step(effect)));let step=step.result.unwrap();let p=step.progress();assert_eq!((p.retained_capacity_bytes,p.released_bytes),(a,r));assert!(p.fits(g));assert!(p.copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)),"mixed original queue blocked on turn {turn}");bank.post(p,g,(a,r));births+=a;released+=r;copied+=p.copied_bytes;
        }
        assert!(owner.terminal_is_empty());assert_eq!(released,originals+births);assert!(copied<=budget);let (_,heap)=observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));
    }}
}

#[test]
fn queue_reservation_quote_matches_original_physical_allocator_receipt(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["reservation"];let mut owner=RetirementQueue::default();let mut bank=bank();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){if owner.has_reserved_slot(){break}let g=grant(owner.reserve_demand().unwrap());let((turn,_),actual)=observe_retirement_allocations(||sync(&mut bank,g,1,|effect|owner.reserve_step(effect)));let receipt=turn.result.unwrap();assert_eq!(receipt.retained_capacity_bytes,actual.0);assert_eq!(actual.1,0);assert_ne!(receipt.copied_items,0);bank.post(receipt,g,actual);}
 assert_eq!(owner.has_reserved_slot(),law["expectedSlot"].as_bool().unwrap());assert!(law["expectedPhysicalReceipt"].as_bool().unwrap());for _ in 0..1024{if owner.terminal_is_empty(){break}let g=quoted(0,|b|owner.next_demand(b).unwrap());let units=owner.next_computational_work(g.maximum_copy_bytes).unwrap();let((turn,_),actual)=observe_retirement_allocations(||poll(&mut bank,g,units,|effect|owner.step(effect)));let receipt=turn.result.unwrap();assert_eq!((receipt.progress().retained_capacity_bytes,receipt.progress().released_bytes),actual);bank.post(receipt.progress(),g,actual);}assert!(owner.terminal_is_empty());
}
