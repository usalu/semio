use super::*;
use crate::{retirement::controlled::ControlledRetirement,RetireOwned};

#[test]
fn factory_authority_keeps_original_typed_source_until_admitted_close_and_reports_all_physical_frees(){
    use crate::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneSource};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🔗️authority/🧫️fixtures/🔣️.json")).unwrap();
    use std::io::Write;let mut oracle=std::process::Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify([...new TextEncoder().encode(x.fixture.payload)]));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();oracle.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();let output=oracle.wait_with_output().unwrap();assert!(output.status.success());let reference:Vec<u8>=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(reference,fixture["payload"].as_str().unwrap().as_bytes());
    for work in fixture["copyGrants"].as_array().unwrap(){let work=work.as_u64().unwrap()as usize;
        let mut payload=Vec::with_capacity(fixture["payloadCapacity"].as_u64().unwrap()as usize);payload.extend_from_slice(&reference);let payload_capacity=payload.capacity();let factory=Arc::new(OwnedFactory{payload});let factory_extent=arc_extent::<OwnedFactory>();
        let original=Arc::new(String::from("source"));let original_bytes=original.capacity()+crate::retirement::shared::arc_bytes::<String>();
        let authority=FactoryAuthority::new(factory.clone());let pointer=Arc::as_ptr(&factory);
        let (refused,heap)=crate::observe_retirement_allocations(||RetainedCloneSource::admit(original,authority,Default::default()));assert_eq!(heap,(0,0));let (_,original,authority)=refused.err().unwrap();assert_eq!(authority.factory_address(),pointer as*const()as usize);
        let capacity=RetainedCloneSource::<String>::constructor_capacity_bytes::<FactoryAuthority>();let (admitted,heap)=crate::observe_retirement_allocations(||RetainedCloneSource::admit(original,authority,grant(capacity,0,1)));let(mut source,receipt)=admitted.unwrap_or_else(|_|panic!("source authority admission"));assert_eq!(heap,(receipt.retained_capacity_bytes,0));let mut born=receipt.retained_capacity_bytes;let mut physical=0;
        let mut cursor=String::retained_clone_cursor();cursor.advance(source.borrow(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}).unwrap();drop(factory);cursor.begin_close();
        for _ in 0..100000{if source.terminal_is_empty()&&cursor.terminal_is_empty(){break;}let grant=if !source.terminal_is_empty(){RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:source.next_close_capacity_byte_demand(work).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()}}else{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(work).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:64}};let(step,heap)=crate::observe_retirement_allocations(||if !source.terminal_is_empty(){source.close_step(grant).unwrap()}else{cursor.close_step(grant).unwrap()});let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;physical+=heap.1;}
        assert!(source.terminal_is_empty()&&cursor.terminal_is_empty());assert_eq!(original_bytes+factory_extent+payload_capacity+born,physical);println!("[DEBUG] Factory source authority work={work} original={} born={born} physical={physical}",original_bytes+factory_extent+payload_capacity);
    }
}

struct OwnedFactory {payload:Vec<u8>}
#[derive(crate::FactoryPayloadRetirement)]
struct ScalarFactory {tag:u64,name:&'static str}
impl FactoryPayloadRetirement for OwnedFactory {
    type CloseState=Option<ControlledRetirement<Vec<u8>>>;
    fn close_state_birth_bytes(&self)->usize {0}
    fn close_state_constructor_depth(&self)->usize {0}
    fn close_state_constructor_copy_bytes(&self)->usize {0}
    fn close_state_preparation_demands(&self,_:&Self::CloseState,_:usize)->Result<crate::RetirementDemand,crate::ValueError>{Ok(Default::default())}
    fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:crate::RetainedCloneGrant)->Result<crate::RetainedCloneStep,crate::ValueError>{Ok(crate::RetainedCloneStep::Complete(Default::default()))}
    fn close_state_preparation_is_complete(_:&Self::CloseState)->bool{true}
    fn prepare_close_state(&self)->Self::CloseState {None}
    fn transfer_payload(value:Self,state:&mut Self::CloseState) {*state=Some(ControlledRetirement::new(value.payload).unwrap_or_else(|_|panic!("Vec payload supports controlled retirement")));}
    fn close_state_demands(state:&Self::CloseState,copy:usize)->Result<RetirementDemand,ValueError> {
        state.as_ref().map_or(Ok(RetirementDemand::default()),|owner|Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?}))
    }
    fn close_state_step(state:&mut Self::CloseState,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let Some(owner)=state.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        if owner.terminal_is_empty(){if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}drop(state.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()}));}
        owner.step(grant)
    }
    fn close_state_terminal_is_empty(state:&Self::CloseState)->bool {state.is_none()}
}
fn grant(capacity:usize,release:usize,depth:usize)->RetainedCloneGrant {RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:262144,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}}
fn admit<T:FactoryPayloadRetirement>(owner:Arc<T>)->(Box<dyn FactoryRetirementTicket>,usize) {
    let birth=owner.factory_retirement_birth_bytes();
    let depth=owner.factory_retirement_depth_demand();
    let pointer=Arc::as_ptr(&owner);
    let (refused,heap)=crate::observe_retirement_allocations(||owner.preborn_factory_retirement(grant(birth-1,0,depth)));
    let error=refused.err().expect("factory frame must refuse one-short birth");assert!(error.ticket.is_none());assert_eq!(error.progress,Default::default());let owner=error.original.unwrap();
    assert_eq!(Arc::as_ptr(&owner) as *const (),pointer as *const ());assert_eq!(heap,(0,0));
    let (admitted,heap)=crate::observe_retirement_allocations(||owner.preborn_factory_retirement(grant(birth,0,depth)));
    let (ticket,progress)=admitted.unwrap_or_else(|error|panic!("{}",error.error));
    assert_eq!(heap,(birth,0));assert_eq!(progress.retained_capacity_bytes,birth);
    (ticket,birth)
}
fn drain(mut slot:Option<Box<dyn FactoryRetirementTicket>>,maximum_turns:usize,payload:usize)->(usize,usize,usize,usize) {
    let (mut born,mut freed,mut copies,mut winners)=(0,0,0,0);
    for _ in 0..maximum_turns {
        let Some(ticket)=slot.as_ref()else{break;};
        let demand=factory_ticket_demands(ticket,262144).unwrap();
        let full=grant(demand.capacity_bytes,demand.release_bytes,demand.depth);
        let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,RetainedCloneGrant {maximum_items:0,..full}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
        for denied in [demand.capacity_bytes.checked_sub(1).map(|value|RetainedCloneGrant {maximum_capacity_bytes:value,..full}),demand.release_bytes.checked_sub(1).map(|value|RetainedCloneGrant {maximum_release_bytes:value,..full})].into_iter().flatten(){
            let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
        }
        let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,full).unwrap());let progress=step.progress();
        assert!(progress.fits(full));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));
        born+=heap.0;freed+=heap.1;copies+=progress.copied_bytes;winners+=usize::from(payload!=0&&heap.1==payload);
    }
    assert!(slot.is_none(),"factory original custody did not reach terminal in its original turn bound");
    (born,freed,copies,winners)
}

#[test]
fn factory_preborn_tickets_consume_concurrent_aliases_and_report_original_physical_bytes() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
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
        eprintln!("[DEBUG] full factory concurrent aliases={count} payload={extent} close births={born} physical={freed} original unique winner");
    }
}

#[test]
fn factory_preborn_tickets_keep_weak_wait_and_copy_only_body_truthful() {
    let owner=Arc::new(ScalarFactory {tag:7,name:"factory"});assert_eq!((owner.tag,owner.name),(7,"factory"));
    let weak=Arc::downgrade(&owner);let (ticket,birth)=admit(owner.clone());drop(owner);let mut slot=Some(ticket);
    let demand=factory_ticket_demands(slot.as_ref().unwrap(),262144).unwrap();
    let (step,heap)=crate::observe_retirement_allocations(||close_factory_ticket(&mut slot,grant(demand.capacity_bytes,demand.release_bytes,demand.depth)).unwrap());
    assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert!(weak.upgrade().is_some());drop(weak);
    let (born,freed,copied,winners)=drain(slot,8,0);assert_eq!((born,copied,winners),(0,0,0));
    assert_eq!(freed,arc_extent::<ScalarFactory>()+birth);
    eprintln!("[DEBUG] full factory original weak lease froze custody; scalar body physical={freed}");
}

#[derive(crate::FactoryPayloadRetirement)]
struct NestedFactory {#[factory_child] child:Arc<dyn FactoryRetirement>,tag:u64}
#[test]
fn factory_preborn_tickets_retire_nested_original_children_without_close_birth() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let child:Arc<dyn FactoryRetirement>=Arc::new(OwnedFactory {payload:vec![17;extent]});
        let middle:Arc<dyn FactoryRetirement>=Arc::new(NestedFactory {child,tag:1});
        let original=Arc::new(NestedFactory {child:middle,tag:2});assert_eq!(original.tag,fixture["nestedDepth"].as_u64().unwrap());
        let (ticket,birth)=admit(original.clone());assert_eq!(crate::observe_retirement_allocations(||drop(original)).1,(0,0));
        let (born,freed,copies,winners)=drain(Some(ticket),32,extent);assert_eq!((copies,winners),(extent,1));
        assert_eq!(freed,extent+arc_extent::<OwnedFactory>()+2*arc_extent::<NestedFactory>()+birth+born);
        eprintln!("[DEBUG] full factory nested depth2 payload={extent} admitted tickets={birth} typed close births={born} physical={freed}");
    }
}

#[derive(crate::FactoryPayloadRetirement)]
struct InlineFactory {#[factory_owned] payload:OwnedFactory,tag:u64}
#[test]
fn factory_preborn_tickets_move_inline_original_payload_into_prefunded_state() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();assert_eq!(fixture["inlineOwnedPayload"],true);
    for extent in fixture["payloadBytes"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let original=Arc::new(InlineFactory {payload:OwnedFactory {payload:vec![31;extent]},tag:7});assert_eq!(original.tag,7);
        let (ticket,birth)=admit(original.clone());assert_eq!(crate::observe_retirement_allocations(||drop(original)).1,(0,0));
        let (born,freed,copies,winners)=drain(Some(ticket),16,extent);assert_eq!((copies,winners),(extent,1));
        assert_eq!(freed,extent+arc_extent::<InlineFactory>()+birth+born);
        eprintln!("[DEBUG] full factory inline payload={extent} frame={birth} typed close births={born} physical={freed}");
    }
}

trait OriginalProtocolTicket: ErasedSnapshotRetirement { fn role(&self) -> &'static str; }
struct OriginalProtocolOwner { original: ControlledRetirement<String> }
impl OriginalProtocolTicket for OriginalProtocolOwner { fn role(&self) -> &'static str { "original-protocol" } }
impl ErasedSnapshotRetirement for OriginalProtocolOwner {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.original.step(grant) }
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
            let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap());
            assert_eq!(heap, (0, 0));
            assert_eq!(step.progress(), Default::default());
            assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn OriginalProtocolTicket as *const (), pointer);
            if demand.capacity_bytes != 0 {
                let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..grant }).unwrap());
                assert_eq!(heap, (0, 0));
                assert_eq!(step.progress(), Default::default());
                assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn OriginalProtocolTicket as *const (), pointer);
            }
            if terminal {
                assert_eq!(demand.release_bytes, shell);
                let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, RetainedCloneGrant { maximum_release_bytes: shell - 1, ..grant }).unwrap());
                assert_eq!(heap, (0, 0));
                assert_eq!(step.progress(), Default::default());
                assert_eq!(slot.as_ref().unwrap().as_ref() as *const dyn OriginalProtocolTicket as *const (), pointer);
            }
            let (step, heap) = crate::observe_retirement_allocations(|| close_factory_ticket(&mut slot, grant).unwrap());
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
        println!("[DEBUG] original protocol ticket case={} original={source_pointer:?} backing={backing} actual-shell={shell} admitted-scaffold={born} physical={freed} zero/one-below0 body4096 terminalDrop0", row["id"]);
    }
}
#[test]
fn original_factory_metadata_handoffs_preserve_fixed_copy_and_all_physical_currency(){
 for levels in [0,1,3]{for copy in [1,3,64]{let((original,extent),heap)=crate::observe_retirement_allocations(||{let mut payload=Vec::with_capacity(137);payload.extend_from_slice("original factory payload 雪🌳️".as_bytes());let leaf=Arc::new(OwnedFactory{payload});let mut extent=arc_extent::<OwnedFactory>()+leaf.payload.capacity();let mut source:Arc<dyn FactoryRetirement>=leaf;for tag in 0..levels{source=Arc::new(NestedFactory{child:source,tag});extent+=arc_extent::<NestedFactory>();}(source,extent)});assert_eq!(heap,(extent,0));let pointer=Arc::as_ptr(&original)as*const()as usize;let(mut owner,heap)=crate::observe_retirement_allocations(||FactoryAuthority::new(original));assert_eq!(heap,(0,0));assert_eq!(owner.factory_address(),pointer);let mut born=0;let mut released=0;let mut idle=0;for turn in 0..1000000{if owner.terminal_is_empty(){break}let d=owner.demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};for _ in 0..2{let(step,heap)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));}if turn==0{assert_eq!(d.copy_bytes,0);let(step,heap)=crate::observe_retirement_allocations(||owner.step(RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(owner.factory_address(),pointer);}let(step,heap)=crate::observe_retirement_allocations(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;released+=heap.1;if step.progress()==RetainedCloneProgress::default(){idle+=1}else{idle=0}assert!(idle<64,"original factory metadata stalled copy={copy} turn={turn} demand={d:?}");}assert!(owner.terminal_is_empty());assert_eq!(extent+born,released);assert_eq!(crate::observe_retirement_allocations(||drop(owner)).1,(0,0));eprintln!("[DEBUG] Original factory metadata levels={levels} copy={copy} extent={extent} born={born} release={released} terminalDrop=0");}
}
}
