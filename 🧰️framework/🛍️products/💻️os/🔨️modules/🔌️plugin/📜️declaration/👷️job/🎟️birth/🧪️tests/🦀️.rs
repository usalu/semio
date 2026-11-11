/// 🧪️ Original optional producer inputs survive denial and every physical frame returns to their issuer.
use super::*;
use semio_framework_job::{InteractiveJob,OriginalJobFixtureCaller,OriginalJobDecodeOwner,OriginalConstructionDemand,OperationId,Generation,original_construction_settled_turn_quote};
use semio_framework_value_testing_allocator as _;

struct Body {terminal:bool,bytes:[u8;31]}
impl InteractiveJob for Body {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(JobOutcomeBorrow::admit_yield(cx).map_err(Into::into))}
    fn borrow_outcome<'a>(&'a self,_:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"producer birth specimen has no retained output"))}
    fn begin_close(&mut self){}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{if self.terminal{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:1,depth:1,..Default::default()},|_|{self.terminal=true;semio_framework_value::RetainedWorkOutcome::<RetainedCloneStep,ValueError>{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()})),work:1}})}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal))}
    fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal))}
    fn terminal_is_empty(&self)->bool{self.terminal}
}
impl ArtifactReservedJob for Body {}
fn original_now()->Option<u64>{Some(1000)}

fn turn<R>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<R,semio_framework_job::OriginalTurnFailure>)->R{caller.turn_with_decoder(grant,decoder,run).unwrap()}
fn heap<R>(run:impl FnOnce()->R)->(R,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}

/// 🎟️ Runs both original body ports and exact body/shared frame close under the one issuing caller.
#[cfg_attr(test,test)]
pub fn original_optional_producer_birth_and_complete_physical_return(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(fixture["receiverRetainsPostedOwner"],true);
    let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(fixture["sourceIdentity"][0].as_u64().unwrap()),Generation(fixture["sourceIdentity"][1].as_u64().unwrap()),stock,original_now,u64::MAX,1<<24);let mut decoder=caller.admitted_decoder();
    let source_operation=turn(&mut caller,&mut decoder,stock,|cx|Ok(cx.operation()));
    for port in 0..3{
        let boxed=port==1;let mounted=port==2;
        let mut typed=Some(Body{terminal:false,bytes:[71;31]});let mut body:Option<Box<dyn ArtifactReservedJob+Send>>=None;let mut receiving=None;
        if boxed{caller.turn_with_decoder(stock,&mut decoder,|cx|{let before=cx.retained_progress();let(consumed,observed)=heap(||{body=Some(Box::new(typed.take().unwrap()));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Body>(),retained_capacity_bytes:size_of::<Body>(),..Default::default()})});consumed?;cx.consume_fuel(1);let after=cx.retained_progress();assert_eq!(observed,(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));Ok(())}).unwrap();}
        let demand=if boxed{ArtifactReservedToolJob::original_boxed_birth_demands()}else{ArtifactReservedToolJob::original_birth_demands::<Body>()};let paid=RetainedCloneGrant{maximum_items:if boxed{1}else{2},maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:65536,maximum_depth:1};let original_pointer=body.as_ref().map(|value|(&**value)as*const _ as*const() as usize);
        for row in fixture["cases"].as_array().unwrap().iter().filter(|row|row["port"]==(if boxed{"admitted-box"}else{"typed-body"})&&row["admitted"]==false){
            let denied=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:demand.copy_bytes.checked_add_signed(row["copyDelta"].as_i64().unwrap()as isize).unwrap(),maximum_capacity_bytes:demand.capacity_bytes.checked_add_signed(row["capacityDelta"].as_i64().unwrap()as isize).unwrap(),maximum_depth:row["depth"].as_u64().unwrap()as usize,..paid};
            turn(&mut caller,&mut decoder,stock,|cx|{
                let before=cx.retained_progress();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();
                let(result,heap)=heap(||cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),deadline,|work|work.with_retained_loan(denied,|same|if boxed{ArtifactReservedToolJob::admit_boxed_original(&mut body,&mut receiving,same)}else{ArtifactReservedToolJob::admit_original(&mut typed,&mut receiving,same)})));
                assert!(!result?);assert!(receiving.is_none());assert!(if boxed{body.is_some()}else{typed.as_ref().is_some_and(|body|body.bytes==[71;31])});assert_eq!(body.as_ref().map(|value|(&**value)as*const _ as*const()as usize),original_pointer);assert_eq!(heap,(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(cx.deadline_us(),deadline);Ok(())
            });
        }
        turn(&mut caller,&mut decoder,stock,|cx|{let before=cx.retained_progress();let(result,heap)=heap(||cx.with_retained_loan(paid,|same|if boxed{ArtifactReservedToolJob::admit_boxed_original(&mut body,&mut receiving,same)}else{ArtifactReservedToolJob::admit_original(&mut typed,&mut receiving,same)}));assert!(result?);let after=cx.retained_progress();assert_eq!(heap,(after.retained_capacity_bytes-before.retained_capacity_bytes,0));assert_eq!(heap.0,demand.capacity_bytes);Ok(())});
        let mut owner=receiving.take().unwrap();if let Some(pointer)=original_pointer{let gate=owner.state.as_ref().unwrap().try_lock().unwrap();assert_eq!((&**gate.inner.as_ref().unwrap())as*const _ as*const() as usize,pointer)}
        let(_,observed)=heap(||{let gate=owner.state.as_ref().unwrap().try_lock().unwrap();let body:&dyn std::any::Any=&**gate.inner.as_ref().unwrap();assert_eq!(body.downcast_ref::<Body>().unwrap().bytes,[71;31])});assert_eq!(observed,(0,0));
        let declared=&fixture["businessOperation"];let mut business=Some(semio_framework_job::Operation::new(OperationId(declared[0].as_u64().unwrap()),semio_framework_job::RevisionId(declared[1].as_u64().unwrap()),Generation(declared[2].as_u64().unwrap()),declared[3].as_u64().unwrap()));assert_ne!(business.as_ref().unwrap().operation,source_operation);
        let mut mounted_owner=None;
        if mounted{
            let identity=owner.test_source_identity();let mut source=Some(owner);let demand=ArtifactReservedToolJob::mounted_birth_demands();let mut construction=semio_framework::action_bus::ResidentConstruction::default();
            let business_demand=OriginalConstructionDemand{items:1,copy_bytes:demand.copy_bytes,capacity_bytes:demand.capacity_bytes,release_bytes:demand.release_bytes,depth:demand.depth};let(settled,_)=original_construction_settled_turn_quote::<semio_framework::action_bus::ErasedToolJob>(business_demand,1).unwrap();
            for row in fixture["cases"].as_array().unwrap().iter().filter(|row|row["port"]=="mounted-job"&&row["admitted"]==false){
                let denied=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:settled.maximum_copy_bytes.checked_add_signed(row["copyDelta"].as_i64().unwrap()as isize).unwrap(),maximum_capacity_bytes:settled.maximum_capacity_bytes.checked_add_signed(row["capacityDelta"].as_i64().unwrap()as isize).unwrap(),maximum_release_bytes:0,maximum_depth:row["depth"].as_u64().unwrap()as usize};
                turn(&mut caller,&mut decoder,stock,|cx|{
                    let before=cx.retained_progress();let deadline=cx.deadline_us();
                    let(result,observed)=heap(||cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),deadline,|work|work.with_retained_loan(denied,|same|ArtifactReservedToolJob::admit_mounted(&mut source,&mut construction,business.as_ref().unwrap(),same).map(|job|job.is_none()))));
                    assert!(result?);assert_eq!(source.as_ref().unwrap().test_source_identity(),identity);assert_eq!(source.as_ref().unwrap().test_bound_operation(),None);assert_eq!(cx.retained_progress(),before);assert_eq!(observed,(0,0));Ok(())
                });
            }
            let mut paid=(0,0);let mut observed=(0,0);for _ in 0..1000000{if mounted_owner.is_some(){break}let(job,heap)=heap(||turn(&mut caller,&mut decoder,settled,|cx|match ArtifactReservedToolJob::admit_mounted(&mut source,&mut construction,business.as_ref().unwrap(),cx){Err(e)=>{eprintln!("[DEBUG] outer={e:?}");Ok(None)},Ok(j)=>Ok(j)}));{let mut out=None;turn(&mut caller,&mut decoder,stock,|cx|{construction.admission().take_failure_into_step(&mut out,cx)?;Ok(())});if out.is_some(){eprintln!("[DEBUG] cause={out:?}");panic!("[DEBUG] stop")}}let receipt=caller.receipt();mounted_owner=job;paid=(paid.0+receipt.retained_capacity_bytes,paid.1+receipt.released_bytes);observed=(observed.0+heap.0,observed.1+heap.1);}assert_eq!(observed,paid);
            assert!(source.is_none());assert_eq!(size_of_val(mounted_owner.as_ref().unwrap().retained_job().unwrap()),size_of::<ArtifactReservedToolJob>());owner=ArtifactReservedToolJob{state:ManuallyDrop::new(None),held:ManuallyDrop::new(None)};
            while !construction.terminal_is_empty(){turn(&mut caller,&mut decoder,stock,|cx|construction.admission().retire_step(cx).map(drop));}
        }else{
            let denied=RetainedCloneGrant{maximum_items:0,..stock};turn(&mut caller,&mut decoder,stock,|cx|{let before=cx.retained_progress();let(result,observed)=heap(||cx.with_retained_loan(denied,|same|owner.bind_original_operation(&mut business,same)));assert!(!result?);assert!(business.is_some());assert_eq!(observed,(0,0));assert_eq!(cx.retained_progress(),before);Ok(())});
            turn(&mut caller,&mut decoder,stock,|cx|{let before=cx.retained_progress();let(result,observed)=heap(||owner.bind_original_operation(&mut business,cx));assert!(result?);assert!(business.is_none());assert_eq!(observed,(0,0));assert_eq!(cx.retained_progress().copied_bytes-before.copied_bytes,size_of::<semio_framework_job::Operation>());Ok(())});
        }
        let closing:&mut dyn InteractiveJob=if mounted{mounted_owner.as_mut().unwrap()}else{&mut owner};
        for step in 0..16{if closing.terminal_is_empty(){break}let demand=RetirementDemand{copy_bytes:closing.next_close_copy_byte_demand().unwrap(),capacity_bytes:closing.next_close_capacity_byte_demand(65536).unwrap(),release_bytes:closing.next_close_release_byte_demand().unwrap(),depth:closing.next_close_depth_demand().unwrap()};let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            turn(&mut caller,&mut decoder,stock,|cx|{let before=cx.retained_progress();let(step,observed)=heap(||cx.with_retained_loan(grant,|same|closing.close_step(same)));let progress=step?.progress();assert_eq!(observed,(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap());Ok(())});assert!(step<15,"original shared producer physical close stalled")}
        assert!(closing.terminal_is_empty());let(_,observed)=heap(||drop((owner,mounted_owner)));assert_eq!(observed,(0,0));
    }
    while !decoder.terminal_is_empty(){let(result,observed)=heap(||caller.turn(stock,|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})));let receipt=caller.receipt();assert_eq!(observed,(receipt.retained_capacity_bytes,receipt.released_bytes));result.unwrap();}
    caller.retire(stock);
}
