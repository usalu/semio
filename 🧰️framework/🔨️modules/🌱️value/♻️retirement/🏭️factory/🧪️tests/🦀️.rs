use super::*;
use crate::{retirement::controlled::ControlledRetirement,RetireOwned};

use crate::original_factory_fixture::*;
fn admit<T:FactoryPayloadRetirement>(owner:Arc<T>)->(Box<dyn FactoryRetirementTicket>,usize) {
 let birth=owner.factory_retirement_birth_bytes();let depth=owner.factory_retirement_depth_demand();let pointer=Arc::as_ptr(&owner)as*const()as usize;
 let admission=owner.retirement_admission();let mut source:Option<Arc<dyn FactoryRetirement>>=Some(owner);let mut target=None;let mut actual=None;
 let (refused,heap)=crate::observe_retirement_allocations(||admission.admit(&mut source,&mut target,grant(birth-1,0,depth),&mut actual));
 assert_eq!(refused.result.unwrap().progress(),Default::default());assert!(target.is_none());assert!(actual.is_none());assert_eq!(Arc::as_ptr(source.as_ref().unwrap())as*const()as usize,pointer);assert_eq!(heap,(0,0));
 let (admitted,heap)=crate::observe_retirement_allocations(||admission.admit(&mut source,&mut target,grant(birth,0,depth),&mut actual));
 let progress=admitted.result.unwrap().progress();assert!(source.is_none());assert_eq!(actual,Some(birth));assert_eq!(heap,(birth,0));assert_eq!(progress.retained_capacity_bytes,birth);
 (target.unwrap(),birth)
}
fn drain(mut slot:Option<Box<dyn FactoryRetirementTicket>>,maximum_turns:usize,payload:usize)->(usize,usize,usize,usize) {
    let (mut born,mut freed,mut copies,mut winners)=(0,0,0,0);
    for _ in 0..maximum_turns {
        let Some(ticket)=slot.as_ref()else{break;};
        let demand=factory_ticket_demands(ticket,262144).unwrap();
        let full=grant(demand.capacity_bytes,demand.release_bytes,demand.depth);
        let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,RetainedCloneGrant {maximum_items:0,..full}).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
        for denied in [demand.capacity_bytes.checked_sub(1).map(|value|RetainedCloneGrant {maximum_capacity_bytes:value,..full}),demand.release_bytes.checked_sub(1).map(|value|RetainedCloneGrant {maximum_release_bytes:value,..full})].into_iter().flatten(){
            let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,denied).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
        }
        let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,full).result.unwrap());let progress=step.progress();
        assert!(progress.fits(full));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));
        born+=heap.0;freed+=heap.1;assert!(progress.released_bytes==0||progress.copied_bytes==0,"terminal metadata handoff is copy0 beside its physical release");copies+=progress.copied_bytes;winners+=usize::from(payload!=0&&heap.1==payload);
    }
    assert!(slot.is_none(),"factory original custody did not reach terminal in its original turn bound");
    (born,freed,copies,winners)
}

#[test]
fn factory_preborn_tickets_consume_concurrent_aliases_and_report_original_physical_bytes() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let count=fixture["aliasCount"].as_u64().unwrap()as usize;
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let original=Arc::new(OwnedFactory {payload:vec![42;extent]});
        let birth=original.factory_retirement_birth_bytes();assert!(birth<=fixture["maximumAdmissionBytes"].as_u64().unwrap()as usize);
        assert_eq!(birth,factory_retirement_frame_bytes::<OwnedFactory>()+original.close_state_birth_bytes());
        let tickets:Vec<_>=(0..count).map(|_|admit(original.clone()).0).collect();
        assert_eq!(crate::observe_retirement_allocations(||drop(original)).1,(0,0));
        let handles:Vec<_>=tickets.into_iter().map(|ticket|std::thread::spawn(move||drain(Some(ticket),16,extent))).collect();
        let receipts:Vec<_>=handles.into_iter().map(|handle|handle.join().unwrap()).collect();
        let born:usize=receipts.iter().map(|row|row.0).sum();let freed:usize=receipts.iter().map(|row|row.1).sum();
        assert_eq!(receipts.iter().map(|row|row.3).sum::<usize>(),fixture["winnerCount"].as_u64().unwrap()as usize);
        assert_eq!(receipts.iter().map(|row|row.2).sum::<usize>(),extent);
        assert_eq!(freed,extent+factory_arc_birth_bytes::<OwnedFactory>()+birth*count+born);
    }
}

#[test]
fn factory_preborn_tickets_never_wait_on_weak_aliases_and_copy_only_body_truthful() {
    let owner=Arc::new(ScalarFactory {tag:7,name:"factory"});assert_eq!((owner.tag,owner.name),(7,"factory"));
    let weak=Arc::downgrade(&owner);let (ticket,birth)=admit(owner.clone());drop(owner);let mut slot=Some(ticket);
    let demand=factory_ticket_demands(slot.as_ref().unwrap(),262144).unwrap();
    let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,grant(demand.capacity_bytes,demand.release_bytes,demand.depth)).result.unwrap());
    assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert!(weak.upgrade().is_some());drop(weak);
    let (born,freed,copied,winners)=drain(slot,8,0);assert_eq!((born,copied,winners),(0,0,0));
    assert_eq!(freed,arc_extent::<ScalarFactory>()+birth);
}

#[derive(crate::FactoryPayloadRetirement)]
#[value(crate="crate")]
struct NestedFactory {#[factory_child] child:Arc<dyn FactoryRetirement>,tag:u64}
#[test]
fn factory_preborn_tickets_retire_nested_original_children_without_close_birth() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let child:Arc<dyn FactoryRetirement>=Arc::new(OwnedFactory {payload:vec![17;extent]});
        let middle:Arc<dyn FactoryRetirement>=Arc::new(NestedFactory {child,tag:1});
        let original=Arc::new(NestedFactory {child:middle,tag:2});assert_eq!(original.tag,fixture["nestedDepth"].as_u64().unwrap());
        let (ticket,birth)=admit(original.clone());assert_eq!(crate::observe_retirement_allocations(||drop(original)).1,(0,0));
        let (born,freed,copies,winners)=drain(Some(ticket),32,extent);assert_eq!((copies,winners),(extent,1));
        assert_eq!(freed,extent+arc_extent::<OwnedFactory>()+2*arc_extent::<NestedFactory>()+birth+born);
    }
}

#[derive(crate::FactoryPayloadRetirement)]
#[value(crate="crate")]
struct InlineFactory {#[factory_owned] payload:OwnedFactory,tag:u64}
#[test]
fn factory_preborn_tickets_move_inline_original_payload_into_prefunded_state() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["inlineOwnedPayload"],true);
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let original=Arc::new(InlineFactory {payload:OwnedFactory {payload:vec![31;extent]},tag:7});assert_eq!(original.tag,7);
        let (ticket,birth)=admit(original.clone());assert_eq!(crate::observe_retirement_allocations(||drop(original)).1,(0,0));
        let (born,freed,copies,winners)=drain(Some(ticket),16,extent);assert_eq!((copies,winners),(extent,1));
        assert_eq!(freed,extent+arc_extent::<InlineFactory>()+birth+born);
    }
}

trait OriginalProtocolTicket: ErasedSnapshotRetirement { fn role(&self) -> &'static str; }
struct OriginalProtocolOwner { original: ControlledRetirement<String> }
impl OriginalProtocolTicket for OriginalProtocolOwner { fn role(&self) -> &'static str { "original-protocol" } }
impl ErasedSnapshotRetirement for OriginalProtocolOwner {
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.original.next_demand(body)}
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let grant=effect.grant();let result=self.original.step(&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"));crate::RetainedWorkOutcome{result,work:self.original.last_turn_work()}}
    fn terminal_is_empty(&self) -> bool { self.original.terminal_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.original.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { self.original.next_capacity_byte_demand(body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.original.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.original.next_depth_demand() }
}
#[test]
fn original_protocol_ticket_retires_actual_subtrait_object_body_and_box_in_separate_grants() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("📦️protocol/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let mut original = String::with_capacity(row["capacity"].as_u64().unwrap() as usize);
        original.push_str(row["text"].as_str().unwrap());
        let backing = original.capacity();
        let source_pointer = original.as_ptr();
        let (original, heap) = crate::observe_retirement_allocations(|| ControlledRetirement::new(original).unwrap_or_else(|_| panic!("native String supports original controlled ownership")));
        assert_eq!(heap, (0, 0));
        assert_eq!(original.original().unwrap().as_ptr(), source_pointer);
        let mut slot: Option<Box<dyn OriginalProtocolTicket>> = Some(Box::new(OriginalProtocolOwner { original }));
        let shell = std::mem::size_of::<OriginalProtocolOwner>();
        let pointer = slot.as_ref().unwrap().as_ref() as *const dyn OriginalProtocolTicket as *const ();
        assert_eq!(slot.as_ref().unwrap().role(), "original-protocol");
        let mut freed = 0;
        let mut born = 0;
        let mut paid_shell = 0;
        for _ in 0..fixture["maximumTurns"].as_u64().unwrap() {
            let Some(owner) = slot.as_ref() else { break; };
            let terminal = owner.terminal_is_empty();
            let (demand, heap) = crate::observe_retirement_allocations(|| factory_ticket_demands(owner, 4096).unwrap());
            assert_eq!(heap, (0, 0));
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: fixture["maximumBodyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
            let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, RetainedCloneGrant { maximum_items: 0, ..grant }).result.unwrap());
            assert_eq!(heap, (0, 0));
            assert_eq!(step.progress(), Default::default());
            assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn OriginalProtocolTicket as *const (), pointer);
            if demand.capacity_bytes != 0 {
                let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }).result.unwrap());
                assert_eq!(heap, (0, 0));
                assert_eq!(step.progress(), Default::default());
                assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn OriginalProtocolTicket as *const (), pointer);
            }
            if terminal {
                assert_eq!(demand.release_bytes, shell);
                let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, RetainedCloneGrant { maximum_release_bytes: shell - 1, ..grant }).result.unwrap());
                assert_eq!(heap, (0, 0));
                assert_eq!(step.progress(), Default::default());
                assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn OriginalProtocolTicket as *const (), pointer);
            }
            let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, grant).result.unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(heap, (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            born += heap.0;
            freed += heap.1;
            if terminal { assert_eq!(heap.0, 0); assert!(slot.is_none()); paid_shell += heap.1; }
            else { assert!(slot.is_some()); }
        }
        assert!(slot.is_none());
        assert_eq!(paid_shell, shell);
        assert_eq!(freed, backing + shell + born);
        let (_, heap) = crate::observe_retirement_allocations(|| drop(slot));
        assert_eq!(heap, (0, 0));
    }
}
#[test]
fn original_factory_metadata_handoffs_preserve_fixed_copy_and_all_physical_currency(){
 let closing:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📬️closing.json")).unwrap();let terminal_copy=closing["headers"].as_array().unwrap().iter().find(|row|row["wordBytes"].as_u64().unwrap()as usize==std::mem::size_of::<usize>()).unwrap()["copyBytes"].as_u64().unwrap()as usize;assert_eq!(terminal_copy,0);
 for levels in [0,1,3]{for copy in [1,3,64]{let((original,extent),heap)=crate::observe_retirement_allocations(||{let mut payload=Vec::with_capacity(137);payload.extend_from_slice("original factory payload 雪🌳️".as_bytes());let leaf=Arc::new(OwnedFactory{payload});let mut extent=arc_extent::<OwnedFactory>()+leaf.payload.capacity();let mut source:Arc<dyn FactoryRetirement>=leaf;for tag in 0..levels{source=Arc::new(NestedFactory{child:source,tag});extent+=arc_extent::<NestedFactory>();}(source,extent)});assert_eq!(heap,(extent,0));let pointer=Arc::as_ptr(&original)as*const()as usize;let(mut owner,heap)=crate::observe_retirement_allocations(||FactoryAuthority::new(original));assert_eq!(heap,(0,0));assert_eq!(owner.factory_address(),pointer);let mut born=0;let mut released=0;let mut idle=0;for turn in 0..1000000{if owner.terminal_is_empty(){break}let metadata_terminal=owner.ticket.as_ref().is_some_and(|ticket|ticket.terminal_is_empty());let d=owner.demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};for _ in 0..2{let(step,heap)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}if turn==0{assert_eq!(d.copy_bytes,0);let(step,heap)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..grant}).result.unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.factory_address(),pointer);}let used_grant=grant;let(step,heap)=crate::observe_retirement_allocations(||owner.step(used_grant).result.unwrap());assert!(step.progress().fits(used_grant));if metadata_terminal{assert_eq!(step.progress().copied_bytes,terminal_copy);}assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;if step.progress()==RetainedCloneProgress::default(){idle+=1}else{idle=0}assert!(idle<64,"original factory metadata stalled copy={copy} turn={turn} demand={d:?}");}assert!(owner.terminal_is_empty());assert_eq!(extent+born,released);assert_eq!(crate::observe_retirement_allocations(||drop(owner)).1,(0,0));}
}
}

#[test]
fn factory_owned_frame_preserves_original_until_funded_birth() {
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🌱️birth/📐️.json")).unwrap();
 let factory=OwnedValueRetirementFactory::<String>::default();
 for row in fixture["cases"].as_array().unwrap() {
  let mut source=Some(String::from("actual original"));let pointer=source.as_ref().unwrap().as_ptr();let mut target=None;let mut actual=None;
  let d=retirement::owned_retirement_birth_demand::<String>().unwrap();
  let mut g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:0,maximum_depth:d.depth};
  match row["axis"].as_str().unwrap(){"items"=>g.maximum_items=0,"copy"=>g.maximum_copy_bytes-=1,"capacity"=>g.maximum_capacity_bytes-=1,"depth"=>g.maximum_depth-=1,"ready"|"allocator"=>{},_=>unreachable!()}
  let (turn,heap)=crate::observe_retirement_allocations(||if row["axis"]=="allocator"{let(turn,attempts)=semio_framework_value_testing_allocator::refuse_one_allocation_on_this_thread(||factory.retire_owned(&mut source,&mut target,g,&mut actual));assert_eq!(attempts,1);turn}else{factory.retire_owned(&mut source,&mut target,g,&mut actual)});if row["axis"]=="allocator"{assert!(turn.result.is_err());assert_eq!(turn.work,1);}else{turn.result.unwrap();}
  assert_eq!(source.is_some(),row["retained"].as_bool().unwrap());if let Some(source)=&source{assert_eq!(source.as_ptr(),pointer);assert_eq!(actual,if row["axis"]=="allocator"{Some(0)}else{None});assert_eq!(heap,(0,0));}else{assert_eq!(actual,Some(d.capacity_bytes));assert_eq!(heap,(d.capacity_bytes,0));}
  while let Some(owner)=target.as_ref(){let d=factory_ticket_demands(owner,usize::MAX).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};close_factory_ticket(&mut target,g,&mut None).result.unwrap();}
 }
}
