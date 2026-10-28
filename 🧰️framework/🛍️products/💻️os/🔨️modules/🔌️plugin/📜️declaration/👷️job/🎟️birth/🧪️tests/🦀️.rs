/// 🧪️ Original optional producer inputs survive denial and every physical frame returns to their issuer.
use super::*;
#[cfg(test)]
#[global_allocator]
static ORIGINAL_RESERVED_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_job::{InteractiveJob, OriginalCallerSourceParent, OperationId, Generation, StepBudget};
use semio_framework_job_payload::{JobPayloadAuthority};

struct Body {terminal:bool,bytes:[u8;31]}
impl InteractiveJob for Body {
    fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->Result<Option<JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
    fn borrow_outcome<'a>(&'a self,_:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"producer birth specimen has no retained output"))}
    fn begin_close(&mut self){}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{if self.terminal{return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}semio_framework_job::close_original_retained_step(cx,RetirementDemand{copy_bytes:1,depth:1,..Default::default()},|_|{self.terminal=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()}))})}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal))}
    fn next_close_capacity_byte_demand(&self,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(!self.terminal))}
    fn terminal_is_empty(&self)->bool{self.terminal}
}
impl ArtifactReservedJob for Body {}
fn original_now()->Option<u64>{Some(1000)}

/// 🎟️ Runs both original body ports and exact body/shared frame close under the one issuing context.
#[cfg_attr(test,test)]
pub fn original_optional_producer_birth_and_complete_physical_return(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();assert_eq!(fixture["receiverRetainsPostedOwner"],true);
    let stock=RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:16777216,maximum_capacity_bytes:16777216,maximum_release_bytes:16777216,maximum_depth:4096};
    let((mut parent,mut recipient),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let(cancel,a)=semio_framework_job::CancelToken::admit_root(stock).unwrap().unwrap();let(authority,b)=JobPayloadAuthority::admit(OperationId(fixture["sourceIdentity"][0].as_u64().unwrap()),Generation(fixture["sourceIdentity"][1].as_u64().unwrap()),stock).unwrap().unwrap();let(parent,c)=OriginalCallerSourceParent::admit(authority,cancel,stock).ok().unwrap();(parent,a.checked_add(b).unwrap().checked_add(c).unwrap())});
    assert_eq!((heap.requested_bytes,heap.released_bytes),(recipient.retained_capacity_bytes,recipient.released_bytes));let(mut born,mut released)=(heap.requested_bytes,heap.released_bytes);let mut sequence=0;let mut decoder=semio_framework_job::OriginalJobDecodeOwner::empty();let mut cx=parent.borrow_context(StepBudget::new(32768,1500,stock),original_now,&mut sequence,&mut recipient).unwrap().with_original_decoder(&mut decoder);let recipient_identity=cx.original_recipient_identity();
    for port in 0..3{
        let boxed=port==1;let mounted=port==2;
        let mut typed=Some(Body{terminal:false,bytes:[71;31]});let mut body:Option<Box<dyn ArtifactReservedJob+Send>>=None;let mut receiving=None;
        if boxed{let before=cx.retained_progress();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{body=Some(Box::new(typed.take().unwrap()));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Body>(),retained_capacity_bytes:size_of::<Body>(),..Default::default()}).unwrap();cx.consume_fuel(1)});let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));born+=heap.requested_bytes;}
        let demand=if boxed{ArtifactReservedToolJob::original_boxed_birth_demands()}else{ArtifactReservedToolJob::original_birth_demands::<Body>()};let paid=RetainedCloneGrant{maximum_items:if boxed{1}else{2},maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:65536,maximum_depth:1};let original_pointer=body.as_ref().map(|value|(&**value)as*const _ as*const() as usize);
        for row in fixture["cases"].as_array().unwrap().iter().filter(|row|row["port"]==(if boxed{"admitted-box"}else{"typed-body"})&&row["admitted"]==false){
            let denied=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:demand.copy_bytes.checked_add_signed(row["copyDelta"].as_i64().unwrap()as isize).unwrap(),maximum_capacity_bytes:demand.capacity_bytes.checked_add_signed(row["capacityDelta"].as_i64().unwrap()as isize).unwrap(),maximum_depth:row["depth"].as_u64().unwrap()as usize,..paid};
            let before=cx.retained_progress();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();
            let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),deadline,|work|work.with_retained_loan(denied,|same|if boxed{ArtifactReservedToolJob::admit_boxed_original(&mut body,&mut receiving,same)}else{ArtifactReservedToolJob::admit_original(&mut typed,&mut receiving,same)})));
            assert!(!result.unwrap());assert!(receiving.is_none());assert!(if boxed{body.is_some()}else{typed.as_ref().is_some_and(|body|body.bytes==[71;31])});assert_eq!(body.as_ref().map(|value|(&**value)as*const _ as*const()as usize),original_pointer);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!(cx.deadline_us(),deadline);
        }
        let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(paid,|same|if boxed{ArtifactReservedToolJob::admit_boxed_original(&mut body,&mut receiving,same)}else{ArtifactReservedToolJob::admit_original(&mut typed,&mut receiving,same)}));assert!(result.unwrap());let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,0));assert_eq!(heap.requested_bytes,demand.capacity_bytes);born+=heap.requested_bytes;
        let mut owner=receiving.take().unwrap();if let Some(pointer)=original_pointer{let gate=owner.state.as_ref().unwrap().try_lock().unwrap();assert_eq!((&**gate.inner.as_ref().unwrap())as*const _ as*const() as usize,pointer)}
        let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let gate=owner.state.as_ref().unwrap().try_lock().unwrap();let body:&dyn std::any::Any=&**gate.inner.as_ref().unwrap();assert_eq!(body.downcast_ref::<Body>().unwrap().bytes,[71;31])});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let declared=&fixture["businessOperation"];let mut business=Some(semio_framework_job::Operation::new(OperationId(declared[0].as_u64().unwrap()),semio_framework_job::RevisionId(declared[1].as_u64().unwrap()),Generation(declared[2].as_u64().unwrap()),declared[3].as_u64().unwrap()));assert_ne!(business.as_ref().unwrap().operation,cx.operation());
        let mut mounted_owner=None;
        if mounted{
            let identity=owner.test_source_identity();let mut source=Some(owner);let demand=ArtifactReservedToolJob::mounted_birth_demands();
            for row in fixture["cases"].as_array().unwrap().iter().filter(|row|row["port"]=="mounted-job"&&row["admitted"]==false){
                let denied=RetainedCloneGrant{maximum_items:row["items"].as_u64().unwrap()as usize,maximum_copy_bytes:demand.copy_bytes.checked_add_signed(row["copyDelta"].as_i64().unwrap()as isize).unwrap(),maximum_capacity_bytes:demand.capacity_bytes.checked_add_signed(row["capacityDelta"].as_i64().unwrap()as isize).unwrap(),maximum_release_bytes:0,maximum_depth:row["depth"].as_u64().unwrap()as usize};
                let before=cx.retained_progress();let fuel=cx.fuel_remaining();let deadline=cx.deadline_us();
                let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),deadline,|work|work.with_retained_loan(denied,|same|ArtifactReservedToolJob::admit_mounted(&mut source,&mut mounted_owner,business.as_ref().unwrap(),same))));
                assert!(!result.unwrap());assert!(mounted_owner.is_none());assert_eq!(source.as_ref().unwrap().test_source_identity(),identity);assert_eq!(source.as_ref().unwrap().test_bound_operation(),None);assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
            let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ArtifactReservedToolJob::admit_mounted(&mut source,&mut mounted_owner,business.as_ref().unwrap(),&mut cx));assert!(result.unwrap());assert!(source.is_none());let after=cx.retained_progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,0));assert_eq!(heap.requested_bytes,demand.capacity_bytes);born+=heap.requested_bytes;
            assert_eq!(size_of_val(mounted_owner.as_ref().unwrap().retained_job().unwrap()),size_of::<ArtifactReservedToolJob>());owner=ArtifactReservedToolJob{state:ManuallyDrop::new(None)};
        }else{
        let denied=RetainedCloneGrant{maximum_items:0,..stock};let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(denied,|same|owner.bind_original_operation(&mut business,same)));assert!(!result.unwrap());assert!(business.is_some());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress(),before);
        let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.bind_original_operation(&mut business,&mut cx));assert!(result.unwrap());assert!(business.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(cx.retained_progress().copied_bytes-before.copied_bytes,size_of::<semio_framework_job::Operation>());
        }
        let closing:&mut dyn InteractiveJob=if mounted{mounted_owner.as_mut().unwrap()}else{&mut owner};
        for turn in 0..16{if closing.terminal_is_empty(){break}let demand=RetirementDemand{copy_bytes:closing.next_close_copy_byte_demand().unwrap(),capacity_bytes:closing.next_close_capacity_byte_demand(65536).unwrap(),release_bytes:closing.next_close_release_byte_demand().unwrap(),depth:closing.next_close_depth_demand().unwrap()};let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let before=cx.retained_progress();let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_retained_loan(grant,|same|closing.close_step(same)).unwrap());let progress=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap());assert_eq!(cx.original_recipient_identity(),recipient_identity);born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<15,"original shared producer physical close stalled")}
        assert!(closing.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop((owner,mounted_owner)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
    drop(cx);while !decoder.terminal_is_empty(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||decoder.close_step(stock).unwrap());let p=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}for turn in 0..128{if parent.terminal_is_empty(){break}let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||parent.close_step(stock).unwrap());let progress=step.progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<127)}assert!(parent.terminal_is_empty());assert_eq!(born,released);
}
