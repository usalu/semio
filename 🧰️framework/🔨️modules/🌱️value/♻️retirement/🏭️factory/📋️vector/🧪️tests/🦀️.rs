use super::*;
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
    fn retire_owned(&self,row:Row,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(crate::OriginalRetirementFailure,Row)> {let mut work=0;let result=(||{
        self.attempts.fetch_add(1,Ordering::SeqCst);work+=1;
        work+=1;if self.refuse.swap(false,Ordering::SeqCst){return Err((crate::OriginalRetirementFailure::from_primary_and_work(ValueError::literal(ValueRefusalKind::OwnershipLimit,"exact vector issuer refused first row"),ValueError::literal(ValueRefusalKind::WorkLimit,"original settlement refused")),row));}
        self.order.fetch_update(Ordering::SeqCst,Ordering::SeqCst,|value|Some(value*10+row.id)).unwrap();work+=1;
        {let turn=semio_framework_value_retirement::admit_owned_retirement(row,grant);work+=turn.work;turn.result.map_err(|(error,value)|(error.into(),value))}
    })();crate::RetainedWorkOutcome{result,work}}
}

fn grant(demand:RetirementDemand)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

/// 🧫️ Neutral original rows settle in declared order through only their borrowed exact issuer.
#[test]
fn factory_vector_retirement_preserves_original_capacity_order_and_refused_row(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let mut rows=Vec::with_capacity(7);
    for row in fixture["rows"].as_array().unwrap(){rows.push(Row{id:row["id"].as_u64().unwrap()as usize,text:row["text"].as_str().unwrap().into()});}
    let pointers=rows.iter().map(|row|row.text.as_ptr()as usize).collect::<Vec<_>>();
    let allocation=rows.as_ptr()as usize;let capacity=rows.capacity()*size_of::<Row>();
    let((denied,original),heap)=crate::observe_retirement_allocations(||match FactoryVectorRetirement::admit(rows,RetainedCloneGrant::default()){Err((error,rows))=>(error,rows),Ok(_)=>panic!("unfunded original capture")});
    assert_eq!(denied.kind,ValueRefusalKind::OwnershipLimit);assert_eq!(heap,(0,0));assert_eq!(original.as_ptr()as usize,allocation);
    let((mut owner,receipt),heap)=crate::observe_retirement_allocations(||FactoryVectorRetirement::admit(original,grant(FactoryVectorRetirement::<Row>::constructor_demand())).unwrap());
    assert_eq!(heap,(0,0));assert_eq!(receipt.copied_items,1);assert_eq!(owner.allocation_address(),allocation);assert_eq!(owner.allocation_bytes(),capacity);
    let factory=Factory::default();factory.refuse.store(fixture["refuseFirst"].as_bool().unwrap(),Ordering::SeqCst);
    assert_eq!(FactoryIssuerIdentity::capture(&factory).matches(&factory),fixture["issuerSame"]["admitted"].as_bool().unwrap());
    let mut refusals=0;let mut turns=0;
    while !owner.terminal_is_empty(){
        turns+=1;assert!(turns<10_000,"exact factory vector must complete under quoted grants");
        let(demand,heap)=crate::observe_retirement_allocations(||owner.demands(&factory,4096).unwrap());assert_eq!(heap,(0,0));
        let quote=grant(demand);
        let before=owner.pending().map(|row|(row.id,row.text.as_ptr()as usize));
        let(step,heap)=crate::observe_retirement_allocations(||owner.close_step(&factory,RetainedCloneGrant::default()).result.unwrap());assert_eq!(heap,(0,0));assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(owner.pending().map(|row|(row.id,row.text.as_ptr()as usize)),before);
        let work=owner.close_work_demand(&factory,4096).unwrap();
        if let Some(expected)=fixture["initialWork"].as_array().unwrap().get(turns-1){assert_eq!(work,expected.as_u64().unwrap());}
        let(result,heap)=crate::observe_retirement_allocations(||owner.close_step(&factory,quote));
        assert!(result.work<=work);
        match result.result{
            Ok(step)=>{assert!(step.progress().fits(quote));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));},
            Err(error)=>{refusals+=1;assert_eq!(error.primary().unwrap().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(error.work_refusal().unwrap().kind,ValueRefusalKind::WorkLimit);assert!(error.retained_progress().fits(quote));assert_eq!(heap,(0,0));let row=owner.pending().expect("provider refusal restores exact pending row");assert_eq!(row.id,1);assert_eq!(row.text.as_ptr()as usize,pointers[0]);},
        }
        assert_eq!(owner.allocation_address(),allocation);assert_eq!(owner.allocation_bytes(),capacity);
    }
    assert_eq!(refusals,1);assert_eq!(factory.attempts.load(Ordering::SeqCst),fixture["expectedAttempts"].as_u64().unwrap()as usize);
    let order=fixture["expectedOrder"].as_array().unwrap().iter().fold(0,|value,row|value*10+row.as_u64().unwrap()as usize);assert_eq!(factory.order.load(Ordering::SeqCst),order);
    let(_,heap)=crate::observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));
}

#[derive(Clone,Copy,crate::FactoryPayloadRetirement)]
#[value(crate = "crate")]
struct ZstOriginal;
#[derive(Clone,Copy,crate::FactoryPayloadRetirement)]
#[value(crate = "crate")]
struct ZstOther;
union SameIssuerSlot{original:ZstOriginal,other:ZstOther}
thread_local!{static OTHER_CALLS:std::cell::Cell<usize>=const{std::cell::Cell::new(0)};}
impl ArtifactOwnedValueRetirementFactory<Row> for ZstOriginal{fn retire_owned_work_demand(&self,_:&Row)->Result<u64,crate::ValueError>{Ok(1)}
 fn retirement_birth_bytes(&self,_:&Row)->usize{semio_framework_value_retirement::owned_retirement_birth_bytes::<Row>()}
 fn retire_owned(&self,row:Row,g:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(crate::OriginalRetirementFailure,Row)>{let turn=(||{let mut work=0;let result=(||{{let turn=semio_framework_value_retirement::admit_owned_retirement(row,g);work+=turn.work;turn.result}})();crate::RetainedWorkOutcome{result,work}})();crate::RetainedWorkOutcome{result:turn.result.map_err(|(error,value)|(error.into(),value)),work:turn.work}}
}
impl ArtifactOwnedValueRetirementFactory<Row> for ZstOther{fn retire_owned_work_demand(&self,_:&Row)->Result<u64,crate::ValueError>{Ok(1)}
 fn retirement_birth_bytes(&self,_:&Row)->usize{semio_framework_value_retirement::owned_retirement_birth_bytes::<Row>()}
 fn retire_owned(&self,row:Row,_:RetainedCloneGrant)->crate::RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(crate::OriginalRetirementFailure,Row)>{let turn=(||{let mut work=0;let result=(||{OTHER_CALLS.set(OTHER_CALLS.get()+1);work+=1;Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"foreign zero-sized issuer must not receive this original"),row))})();crate::RetainedWorkOutcome{result,work}})();crate::RetainedWorkOutcome{result:turn.result.map_err(|(error,value)|(error.into(),value)),work:turn.work}}
}
/// 🔑️ Distinct zero-sized implementations at the same actual address cannot substitute their issuer.
#[test]
fn factory_vector_retirement_rejects_same_data_different_implementation_issuer(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let law=&fixture["issuerAlias"];let slot=SameIssuerSlot{original:ZstOriginal};let original=unsafe{&slot.original};let foreign=unsafe{&slot.other};assert_eq!(std::ptr::from_ref(original).cast::<()>(),std::ptr::from_ref(foreign).cast::<()>());let identity=FactoryIssuerIdentity::capture(original);assert!(!identity.matches(foreign));OTHER_CALLS.set(0);
 let rows=vec![Row{id:law["pendingId"].as_u64().unwrap()as usize,text:"original zero-sized issuer row".into()}];let pointer=rows[0].text.as_ptr();let allocation=rows.as_ptr()as usize;let(mut owner,_)=FactoryVectorRetirement::admit(rows,grant(FactoryVectorRetirement::<Row>::constructor_demand())).unwrap();
 let capture=grant(owner.demands(original,4096).unwrap());let turn=owner.close_step(original,capture);assert_eq!(turn.work,1);assert!(turn.result.unwrap().progress().fits(capture));assert!(owner.pending().is_none());let q=grant(owner.demands(original,4096).unwrap());let turn=owner.close_step(original,q);assert_eq!(turn.work,1);assert!(turn.result.unwrap().progress().fits(q));assert_eq!(owner.pending().unwrap().text.as_ptr(),pointer);
 let(result,heap)=crate::observe_retirement_allocations(||owner.close_step(foreign,q));assert!(result.result.is_err());assert_eq!(heap,(0,0));assert_eq!(owner.pending().unwrap().id,law["pendingId"].as_u64().unwrap()as usize);assert_eq!(owner.pending().unwrap().text.as_ptr(),pointer);assert_eq!(owner.allocation_address(),allocation);assert_eq!(OTHER_CALLS.get(),law["providerCalls"].as_u64().unwrap()as usize);
 for turn in 0..1000{if owner.terminal_is_empty(){break;}let g=grant(owner.demands(original,4096).unwrap());owner.close_step(original,g).result.unwrap();assert!(turn<999);}
 assert!(owner.terminal_is_empty());
}
