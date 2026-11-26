use super::*;
use crate::{driven::{poll,leaf,CPU},FactoryProducerReply,FactoryProducerReplyComplete,OriginalRetirementFailure,OwnedValueRetirementFactory};
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering};

#[derive(Debug,crate::RetireOwned)]
#[value(crate = "crate")]
struct Row { id: usize, text: String }

#[derive(Default,crate::FactoryPayloadRetirement)]
#[value(crate = "crate")]
struct Factory {
    #[factory_owned]
    attempts:AtomicUsize,
    #[factory_owned]
    order:AtomicUsize,
    #[factory_owned]
    refuse:AtomicBool,
}

impl ArtifactOwnedValueRetirementFactory<Row> for Factory {
    fn retire_owned_work_demand(&self,_:&Row)->Result<u64,ValueError>{Ok(if self.refuse.load(Ordering::SeqCst){2}else{4})}
    fn retirement_birth_bytes(&self,_:&Row)->usize {semio_framework_value_retirement::owned_retirement_birth_bytes::<Row>()}
    fn retire_owned_demands(&self,row:&Row)->Result<RetirementDemand,ValueError>{OwnedValueRetirementFactory::<Row>::default().retire_owned_demands(row)}
    fn retire_owned<'reply>(&self,value:&mut Option<Row>,effect:&mut RetainedAllocationEffect<'_>,mut reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{
        let boundary=effect.performed_boundary();
        self.attempts.fetch_add(1,Ordering::SeqCst);assert!(effect.performed(1).unwrap());assert!(effect.performed(1).unwrap());
        if self.refuse.swap(false,Ordering::SeqCst){return reply.complete_observed(effect,boundary,Err(OriginalRetirementFailure::from_primary_and_work(ValueError::literal(ValueRefusalKind::OwnershipLimit,"exact vector issuer refused first row"),ValueError::literal(ValueRefusalKind::WorkLimit,"original settlement refused"))));}
        let id=value.as_ref().expect("vector row is resident").id;self.order.fetch_update(Ordering::SeqCst,Ordering::SeqCst,|order|Some(order*10+id)).unwrap();assert!(effect.performed(1).unwrap());
        crate::driven::settle_owned(value,effect,reply,boundary)
    }
    fn recover_retire_owned<'reply>(&self,_:&mut Option<Row>,_:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{reply.pending()}
}

fn grant(demand:RetirementDemand)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
/// 🪑️ Captures one resident vector into its original retirement owner under a funded turn.
fn capture(rows:&mut Option<Vec<Row>>,g:RetainedCloneGrant)->(Option<FactoryVectorRetirement<Row>>,RetainedWorkOutcome<RetainedCloneStep,ValueError>,(usize,usize)){let mut output=None;let(turn,heap,_)=leaf(g,CPU,|effect|FactoryVectorRetirement::admit(rows,&mut output,effect));let RetainedWorkPoll::Complete(turn)=turn else{panic!("vector capture stayed pending")};(output,turn,heap)}

/// 🧫️ Neutral original rows settle in declared order through only their borrowed exact issuer.
#[test]
fn factory_vector_retirement_preserves_original_capacity_order_and_refused_row(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let mut rows=Vec::with_capacity(7);
    for row in fixture["rows"].as_array().unwrap(){rows.push(Row{id:row["id"].as_u64().unwrap()as usize,text:row["text"].as_str().unwrap().into()});}
    let pointers=rows.iter().map(|row|row.text.as_ptr()as usize).collect::<Vec<_>>();
    let allocation=rows.as_ptr()as usize;let capacity=rows.capacity()*size_of::<Row>();let mut original=Some(rows);
    let(output,denied,heap)=capture(&mut original,RetainedCloneGrant::default());
    assert!(output.is_none());assert_eq!(denied.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(original.as_ref().unwrap().as_ptr()as usize,allocation);
    let(output,admitted,heap)=capture(&mut original,grant(FactoryVectorRetirement::<Row>::constructor_demand()));let receipt=admitted.result.unwrap().progress();let mut owner=output.expect("funded original capture");
    assert_eq!(heap,(0,0));assert_eq!(receipt.copied_items,1);assert!(original.is_none());assert_eq!(owner.allocation_address(),allocation);assert_eq!(owner.allocation_bytes(),capacity);
    let factory=Arc::new(Factory::default());factory.refuse.store(fixture["refuseFirst"].as_bool().unwrap(),Ordering::SeqCst);let exact:Arc<dyn ArtifactOwnedValueRetirementFactory<Row>>=factory.clone();
    assert_eq!(owner.demands(&exact,4096).is_ok(),fixture["issuerSame"]["admitted"].as_bool().unwrap());
    let mut refusals=0;let mut turns=0;
    while !owner.terminal_is_empty(){
        turns+=1;assert!(turns<10_000,"exact factory vector must complete under quoted grants");
        let(demand,heap)=crate::observe_retirement_allocations(||owner.demands(&exact,4096).unwrap());assert_eq!(heap,(0,0));
        let quote=grant(demand);
        let before=owner.pending().map(|row|(row.id,row.text.as_ptr()as usize));
        let(step,heap)=poll(RetainedCloneGrant::default(),CPU,|effect|owner.close_step(&exact,4096,effect));assert_eq!(heap,(0,0));assert_eq!(step.result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(owner.pending().map(|row|(row.id,row.text.as_ptr()as usize)),before);
        let work=owner.close_work_demand(&exact,4096).unwrap();
        if let Some(expected)=fixture["initialWork"].as_array().unwrap().get(turns-1){assert_eq!(work,expected.as_u64().unwrap());}
        let(result,heap)=poll(quote,work,|effect|owner.close_step(&exact,4096,effect));
        assert!(result.work<=u128::from(work));
        match result.result{
            Ok(step)=>{assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));},
            Err(error)=>{refusals+=1;assert_eq!(error.primary().unwrap().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(error.work_refusal().unwrap().kind,ValueRefusalKind::WorkLimit);assert!(error.retained_progress().fits(quote));assert_eq!(heap,(0,0));let row=owner.pending().expect("provider refusal restores exact pending row");assert_eq!(row.id,1);assert_eq!(row.text.as_ptr()as usize,pointers[0]);},
        }
        assert_eq!(owner.allocation_address(),allocation);assert_eq!(owner.allocation_bytes(),capacity);
    }
    assert_eq!(refusals,1);assert_eq!(factory.attempts.load(Ordering::SeqCst),fixture["expectedAttempts"].as_u64().unwrap()as usize);
    let order=fixture["expectedOrder"].as_array().unwrap().iter().fold(0,|value,row|value*10+row.as_u64().unwrap()as usize);assert_eq!(factory.order.load(Ordering::SeqCst),order);
    let(_,heap)=crate::observe_retirement_allocations(||drop((owner,exact,factory)));assert_eq!(heap.0,0);
}

#[derive(Clone,Copy,crate::FactoryPayloadRetirement)]
#[value(crate = "crate")]
struct ZstOriginal;
#[derive(Clone,Copy,crate::FactoryPayloadRetirement)]
#[value(crate = "crate")]
struct ZstOther;
thread_local!{static OTHER_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
impl ArtifactOwnedValueRetirementFactory<Row> for ZstOriginal{
 fn retire_owned_work_demand(&self,row:&Row)->Result<u64,ValueError>{OwnedValueRetirementFactory::<Row>::default().retire_owned_work_demand(row)}
 fn retirement_birth_bytes(&self,_:&Row)->usize{semio_framework_value_retirement::owned_retirement_birth_bytes::<Row>()}
 fn retire_owned_demands(&self,row:&Row)->Result<RetirementDemand,ValueError>{OwnedValueRetirementFactory::<Row>::default().retire_owned_demands(row)}
 fn retire_owned<'reply>(&self,value:&mut Option<Row>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{OwnedValueRetirementFactory::<Row>::default().retire_owned(value,effect,reply)}
 fn recover_retire_owned<'reply>(&self,_:&mut Option<Row>,_:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{reply.pending()}
}
impl ArtifactOwnedValueRetirementFactory<Row> for ZstOther{
 fn retire_owned_work_demand(&self,_:&Row)->Result<u64,ValueError>{Ok(1)}
 fn retirement_birth_bytes(&self,_:&Row)->usize{semio_framework_value_retirement::owned_retirement_birth_bytes::<Row>()}
 fn retire_owned_demands(&self,row:&Row)->Result<RetirementDemand,ValueError>{OwnedValueRetirementFactory::<Row>::default().retire_owned_demands(row)}
 fn retire_owned<'reply>(&self,_:&mut Option<Row>,effect:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{let boundary=effect.performed_boundary();OTHER_CALLS.set(OTHER_CALLS.get()+1);assert!(effect.performed(1).unwrap());reply.complete_observed(effect,boundary,Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"foreign zero-sized issuer must not receive this original").into()))}
 fn recover_retire_owned<'reply>(&self,_:&mut Option<Row>,_:&mut RetainedAllocationEffect<'_>,reply:FactoryProducerReply<'reply,dyn ErasedSnapshotRetirement,OriginalRetirementFailure>)->FactoryProducerReplyComplete<'reply,OriginalRetirementFailure>{reply.pending()}
}
/// 🔑️ A distinct zero-sized issuer cannot substitute the exact issuer the vector captured.
#[test]
fn factory_vector_retirement_rejects_same_data_different_implementation_issuer(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["issuerAlias"];let original:Arc<dyn ArtifactOwnedValueRetirementFactory<Row>>=Arc::new(ZstOriginal);let foreign:Arc<dyn ArtifactOwnedValueRetirementFactory<Row>>=Arc::new(ZstOther);assert!(!Arc::ptr_eq(&original,&foreign));OTHER_CALLS.set(0);
 let rows=vec![Row{id:law["pendingId"].as_u64().unwrap()as usize,text:"original zero-sized issuer row".into()}];let pointer=rows[0].text.as_ptr();let allocation=rows.as_ptr()as usize;let mut resident=Some(rows);let(output,admitted,_)=capture(&mut resident,grant(FactoryVectorRetirement::<Row>::constructor_demand()));admitted.result.unwrap();let mut owner=output.expect("funded original capture");
 let capture_grant=grant(owner.demands(&original,4096).unwrap());let work=owner.close_work_demand(&original,4096).unwrap();let(turn,_)=poll(capture_grant,work,|effect|owner.close_step(&original,4096,effect));assert_eq!(turn.work,1);assert!(turn.result.unwrap().progress().fits(capture_grant));assert!(owner.pending().is_none());let q=grant(owner.demands(&original,4096).unwrap());let work=owner.close_work_demand(&original,4096).unwrap();let(turn,_)=poll(q,work,|effect|owner.close_step(&original,4096,effect));assert_eq!(turn.work,1);assert!(turn.result.unwrap().progress().fits(q));assert_eq!(owner.pending().unwrap().text.as_ptr(),pointer);
 let(result,heap)=poll(q,CPU,|effect|owner.close_step(&foreign,4096,effect));assert!(result.result.is_err());assert_eq!(heap,(0,0));assert_eq!(owner.pending().unwrap().id,law["pendingId"].as_u64().unwrap()as usize);assert_eq!(owner.pending().unwrap().text.as_ptr(),pointer);assert_eq!(owner.allocation_address(),allocation);assert_eq!(OTHER_CALLS.get(),law["providerCalls"].as_u64().unwrap()as usize);
 for turn in 0..1000{if owner.terminal_is_empty(){break;}let g=grant(owner.demands(&original,4096).unwrap());let work=owner.close_work_demand(&original,4096).unwrap();poll(g,work,|effect|owner.close_step(&original,4096,effect)).0.result.unwrap();assert!(turn<999);}
 assert!(owner.terminal_is_empty());
}
