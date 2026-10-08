use crate::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,RetainedCloneStep};
use crate::value::observe_retirement_allocations;
use std::sync::Arc;

#[test]
fn retained_clone_owned_source_birth_is_funded_before_original_owner_transfer(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
    let((mut owner,mut authority),original)=observe_retirement_allocations(||{
        let mut owner=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(law["owner"].as_str().unwrap());
        let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(owner,authority)
    });
    let owner_pointer=owner.as_ptr();let authority_pointer=authority.as_ptr();
    let(bytes,heap)=observe_retirement_allocations(||RetainedCloneSource::<String>::owned_constructor_capacity_bytes::<String>());assert_eq!(heap,(0,0));
    let grant=RetainedCloneGrant{maximum_items:law["birth"]["items"].as_u64().unwrap()as usize,maximum_capacity_bytes:bytes,maximum_depth:law["birth"]["depth"].as_u64().unwrap()as usize,..Default::default()};
    for refusal in law["refusals"].as_array().unwrap(){
        let short=refusal["shortBy"].as_u64().unwrap()as usize;
        let denied=match refusal["currency"].as_str().unwrap(){"items"=>RetainedCloneGrant{maximum_items:grant.maximum_items-short,..grant},"capacity"=>RetainedCloneGrant{maximum_capacity_bytes:bytes-short,..grant},"depth"=>RetainedCloneGrant{maximum_depth:grant.maximum_depth-short,..grant},_=>unreachable!()};
        let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,authority,denied));let(error,returned,returned_authority)=result.err().unwrap();owner=returned;authority=returned_authority;
        assert_eq!(heap,(0,0));assert_eq!(format!("{:?}",error.kind),refusal["kind"].as_str().unwrap());assert_eq!(owner.as_ptr(),owner_pointer);assert_eq!(authority.as_ptr(),authority_pointer);
    }
    let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,authority,grant));let(mut source,receipt)=result.map_err(|(error,_,_)|error).unwrap();assert_eq!(heap,(bytes,0));assert_eq!(receipt.retained_capacity_bytes,bytes);assert_eq!(receipt.copied_items,1);assert_eq!(receipt.copied_bytes,law["birth"]["copyBytes"].as_u64().unwrap()as usize);assert_eq!(receipt.released_bytes,law["birth"]["releaseBytes"].as_u64().unwrap()as usize);assert_eq!(source.borrow().get().as_ptr(),owner_pointer);
    let(mut born,mut released)=(bytes,0usize);
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){
        if source.terminal_is_empty(){break;}
        let copy=source.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};
        let(step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
    }
    assert!(source.terminal_is_empty());assert_eq!(released,original.0+born);assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
    println!("[DEBUG] owned source measured birth={bytes} preserves original mutation and authority on every currency refusal; original={} born={born} release={released}",original.0);
}

struct RefusingCopyDemand { owner: Option<u64> }

#[test]
fn retained_clone_owned_source_closes_its_actual_shared_authority_lease(){
    use crate::ErasedSnapshotRetirement;
    use crate::retirement::shared::SharedControlledRetirement;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️owned-birth/🔣️.json")).unwrap();
    let((owner,authority),original)=observe_retirement_allocations(||{let mut owner=String::with_capacity(law["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(law["owner"].as_str().unwrap());let mut authority=String::with_capacity(law["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(law["authority"].as_str().unwrap());(owner,Arc::new(authority))});
    let pointer=Arc::as_ptr(&authority);let lease=SharedControlledRetirement::lease(Arc::clone(&authority));let demand=RetainedCloneSource::<String>::owned_constructor_demand::<SharedControlledRetirement<String>>();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};let(result,heap)=observe_retirement_allocations(||RetainedCloneSource::admit_owned(owner,lease,grant));let(mut source,receipt)=result.map_err(|(error,_,_)|error).unwrap();assert_eq!(heap,(receipt.retained_capacity_bytes,0));let(mut born,mut released)=heap;
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if source.terminal_is_empty(){break;}let copy=source.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
    assert!(source.terminal_is_empty());assert_eq!(Arc::strong_count(&authority),1);assert_eq!(Arc::as_ptr(&authority),pointer);assert_eq!(authority.as_str(),law["authority"].as_str().unwrap());assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
    let mut authority=SharedControlledRetirement::new(authority);
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){if authority.terminal_is_empty(){break;}let copy=authority.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:authority.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:authority.next_release_byte_demand().unwrap(),maximum_depth:authority.next_depth_demand().unwrap()};let(step,heap)=observe_retirement_allocations(||authority.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;}
    assert!(authority.terminal_is_empty());assert_eq!(released,original.0+born);assert_eq!(observe_retirement_allocations(||drop(authority)).1,(0,0));println!("[DEBUG] original shared source authority survives source closure with same pointer; original={} born={born} release={released}",original.0);
}

impl crate::ErasedSnapshotRetirement for RefusingCopyDemand {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}let items=usize::from(self.owner.take().is_some());Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:items,..Default::default()}))}
    fn terminal_is_empty(&self)->bool{self.owner.is_none()}
    fn next_copy_byte_demand(&self)->Result<usize,crate::ValueError>{Err(crate::ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"copy demand has no authority"))}
    fn next_capacity_byte_demand(&self,_:usize)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_release_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
    fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(0)}
}

#[test]
fn retained_clone_erased_copy_demand_propagates_original_owner_refusal(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();
    let owner=RefusingCopyDemand{owner:Some(law["owner"].as_u64().unwrap())};
    let(result,heap)=observe_retirement_allocations(||crate::ErasedSnapshotRetirement::next_copy_byte_demand(&owner));
    assert_eq!(heap,(0,0));assert_eq!(format!("{:?}",result.unwrap_err().kind),law["copyDemandRefusal"].as_str().unwrap());assert_eq!(owner.owner,Some(law["owner"].as_u64().unwrap()));
    let mut ticket:Option<Box<dyn crate::ErasedSnapshotRetirement>>=Some(Box::new(owner));
    let(result,heap)=observe_retirement_allocations(||crate::factory_ticket_demands(ticket.as_ref().unwrap(),0));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,crate::ValueRefusalKind::UnsupportedOwner);assert!(!ticket.as_ref().unwrap().terminal_is_empty());
    let step=crate::close_factory_ticket(&mut ticket,RetainedCloneGrant{maximum_items:1,..Default::default()}).unwrap();assert!(matches!(step,RetainedCloneStep::Progress(_)));assert!(ticket.as_ref().unwrap().terminal_is_empty());
    let demand=crate::factory_ticket_demands(ticket.as_ref().unwrap(),0).unwrap();assert_eq!(demand.depth,1);
    let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};let(step,heap)=observe_retirement_allocations(||crate::close_factory_ticket(&mut ticket,grant).unwrap());assert_eq!(heap,(0,step.progress().released_bytes));assert!(ticket.is_none());assert!(matches!(step,RetainedCloneStep::Progress(_)));assert!(matches!(crate::close_factory_ticket(&mut ticket,Default::default()).unwrap(),RetainedCloneStep::Complete(_)));
    println!("[DEBUG] erased copy demand propagates UnsupportedOwner without observing allocation or consuming original scalar");
}

#[test]
fn retained_clone_close_depth_observes_pending_birth_and_preserves_owner_on_refusal() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🪜️close-demands/🔣️.json")).unwrap();
    let mut cursor=crate::retained_clone::ScalarCursor::<u64>::default();
    cursor.value=Some(law["owner"].as_u64().unwrap());
    assert_eq!(cursor.next_close_depth_demand().unwrap(),law["idleDepth"].as_u64().unwrap()as usize);
    cursor.begin_close();
    let ((depth,capacity),heap)=observe_retirement_allocations(||(cursor.next_close_depth_demand().unwrap(),cursor.next_close_capacity_byte_demand(0).unwrap()));
    assert_eq!(heap,(0,0));assert_eq!(depth,law["pendingBirthDepth"].as_u64().unwrap()as usize);
    let denied=RetainedCloneGrant{maximum_items:law["refusedBirth"]["maximumItems"].as_u64().unwrap()as usize,maximum_capacity_bytes:capacity,maximum_depth:law["refusedBirth"]["maximumDepth"].as_u64().unwrap()as usize,..Default::default()};
    let(result,heap)=observe_retirement_allocations(||cursor.close_step(denied));
    assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,crate::ValueRefusalKind::DepthLimit);assert_eq!(cursor.value,Some(law["owner"].as_u64().unwrap()));
    let birth=RetainedCloneGrant{maximum_items:law["fundedBirth"]["maximumItems"].as_u64().unwrap()as usize,maximum_capacity_bytes:capacity,maximum_depth:depth,..Default::default()};
    let(step,heap)=observe_retirement_allocations(||cursor.close_step(birth).unwrap());
    assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(birth));assert!(cursor.value.is_none());
    for _ in 0..law["maximumTurns"].as_u64().unwrap(){
        if cursor.terminal_is_empty(){break;}
        let copy=cursor.next_close_copy_byte_demand().unwrap();let release=cursor.next_close_release_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(copy.max(release)).unwrap(),maximum_release_bytes:release,maximum_depth:cursor.next_close_depth_demand().unwrap()};
        let(step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
    }
    assert!(cursor.terminal_is_empty());assert_eq!(cursor.next_close_depth_demand().unwrap(),law["terminalDepth"].as_u64().unwrap()as usize);
    println!("[DEBUG] retained clone close depth keeps original scalar on refused birth and reports exact funded scaffold closure");
}

#[test]
fn admitted_source_authority_preserves_original_owners_and_canceled_cursor_custody() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in law["vectors"].as_array().unwrap(){for work in law["workGrants"].as_array().unwrap(){
        let work=work.as_u64().unwrap()as usize;
        let ((owner,authority),original)=observe_retirement_allocations(||{
            let mut owner=String::with_capacity(row["ownerCapacity"].as_u64().unwrap()as usize);owner.push_str(row["owner"].as_str().unwrap());
            let mut authority=String::with_capacity(row["authorityCapacity"].as_u64().unwrap()as usize);authority.push_str(row["authority"].as_str().unwrap());
            (Arc::new(owner),authority)
        });
        let pointer=Arc::as_ptr(&owner);let authority_pointer=authority.as_ptr();
        let bytes=RetainedCloneSource::<String>::constructor_capacity_bytes::<String>();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
        let mut owner=owner;let mut authority=authority;
        for short in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:bytes-1,..grant}]{
            let (refused,heap)=observe_retirement_allocations(||RetainedCloneSource::admit(owner,authority,short));
            let (_,returned,returned_authority)=refused.err().unwrap();owner=returned;authority=returned_authority;
            assert_eq!(heap,(0,0));assert_eq!(Arc::as_ptr(&owner),pointer);assert_eq!(authority.as_ptr(),authority_pointer);
        }
        let (admitted,heap)=observe_retirement_allocations(||RetainedCloneSource::admit(owner,authority,grant));
        let (mut source,receipt)=admitted.map_err(|(error,_,_)|error).unwrap();
        assert_eq!(heap,(receipt.retained_capacity_bytes,0));assert_eq!(heap.0,bytes);
        let (mut born,mut released)=(heap.0,0usize);
        assert_eq!(source.borrow().get()as*const String,pointer);
        assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(),row["owner"]);
        let mut cursor=String::retained_clone_cursor();
        let turn=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:4096,maximum_release_bytes:0,maximum_depth:64};
        let (step,heap)=observe_retirement_allocations(||cursor.advance(source.borrow(),turn).unwrap());
        assert!(step.progress().fits(turn));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
        cursor.begin_close();
        for _ in 0..100000{
            if source.terminal_is_empty(){break;}
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:source.next_close_capacity_byte_demand(work).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};
            let (zero,heap)=observe_retirement_allocations(||source.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(zero.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));
            let (step,heap)=observe_retirement_allocations(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
        }
        assert!(source.terminal_is_empty());assert_eq!(observe_retirement_allocations(||drop(source)).1,(0,0));
        for _ in 0..100000{
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(work).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:64};
            let (step,heap)=observe_retirement_allocations(||cursor.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;
            if matches!(step,RetainedCloneStep::Complete(_)){break;}
        }
        assert!(cursor.terminal_is_empty());assert_eq!(released,original.0+born);assert_eq!(original.1,0);
        assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
        println!("[DEBUG] full admitted source={} work={work} original={} born={born} physically released={released} original pointer and early cancellation custody conserved",row["id"],original.0);
    }}
}
