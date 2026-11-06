//! 🧪️ Actual original Pack receiving preserves literal IFC ownership independently of STEP schema restrictions.
use super::*;
use semio_os_kernel::{ArtifactPackReceiving,NativeSnapshotDecodeOwner,NativeSnapshotFailure};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,RetireOwned};
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,OriginalTurnFailure,StepContext,testkit};
use semio_framework_dsl_record::receiving::OwnedSlot;
use std::{mem::size_of,sync::Arc};
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:2_000_000,maximum_copy_bytes:64*1024*1024,maximum_capacity_bytes:64*1024*1024,maximum_release_bytes:64*1024*1024,maximum_depth:512}}
struct OriginalIfcFixture{caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner}
impl OriginalIfcFixture{
 fn new()->Self{Self{caller:OriginalJobFixtureCaller::admit(OperationId(940),Generation(9),grant(),||Some(0),u64::MAX,10_000_000),decoder:OriginalJobDecodeOwner::empty()}}
 fn alias(&mut self)->Arc<semio_framework_async::CancelToken>{Arc::new(self.caller.admit_alias(grant()))}
 fn return_alias(&mut self,token:Arc<semio_framework_async::CancelToken>){self.caller.return_alias(Arc::try_unwrap(token).ok().expect("fixture observers released their external borrows"),grant());}
 fn receive<T>(&mut self,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->RetainedWorkOutcome<T,NativeSnapshotFailure>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{
  let mut output=None;
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn_with_decoder(grant(),&mut self.decoder,|cx|{
   let original_grant=cx.retained_grant();let result=cx.with_native_receiving::<_,ValueError>(|native|{output=Some(run(native,original_grant));Ok(())});
   let work=output.as_ref().map_or(0,|turn|turn.work);let progress=cx.receiving_child_progress();if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,progress)?;}cx.consume_fuel(work);
   if let Err(error)=result{assert!(output.is_none(),"outer receiving refusal cannot discard an installed native outcome");output=Some(RetainedWorkOutcome{result:Err(error.into()),work:0});}
   Ok(())
  }));
  result.unwrap();let p=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(self.caller.stock(),grant());output.unwrap()
 }
 fn retire_turn(&mut self,run:impl FnOnce(&mut StepContext<'_>)->Result<(),OriginalTurnFailure>){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn(grant(),|cx|cx.with_original_retirement(run)));result.unwrap();let p=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}
 fn close_value<T:RetireOwned>(&mut self,value:T){
  let mut value=Some(value);let mut slot=OwnedSlot::<T>::default();let demand=RetirementDemand{copy_bytes:size_of::<Option<T>>()+size_of::<OwnedSlot<T>>(),depth:1,..Default::default()};
  self.retire_turn(|cx|{semio_framework_job::close_original_retained_step(cx,demand,|_|{*slot.value=value.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}})?;Ok(())});
  for _ in 0..2_000_000{if slot.terminal_is_empty(){return}let demand=slot.close_demands(4096).unwrap();self.retire_turn(|cx|{semio_framework_job::close_original_retained_step(cx,demand,|g|{let turn=slot.close_step(g);RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}})?;Ok(())});}panic!("finite IFC fixture value retirement")
 }
 fn drain_decoder(&mut self){for _ in 0..2_000_000{if self.decoder.terminal_is_empty(){return}let decoder=&mut self.decoder;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||self.caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{decoder.close_on_source(cx)?;Ok(())})));result.unwrap();let p=self.caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));}panic!("finite original IFC decoder retirement")}
 fn finish(mut self){self.drain_decoder();self.caller.retire(grant());}
}
fn snapshot() -> IfcSnapshot {
    IfcSnapshot {
        schema: "stdio.ifc".into(),
        header: IfcHeader {
            file_description: vec![IfcValue::Aggregate(vec![IfcValue::String("ViewDefinition [ReferenceView]".into())]), IfcValue::String("2;1".into())],
            file_name: vec![IfcValue::String("original.ifc".into())],
            file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])],
        },
        entities: vec![IfcEntity {
            id: u64::MAX,
            name: String::new(),
            args: Vec::new(),
            complex: vec![IfcComplexType {
                name: "A".into(),
                args: vec![IfcValue::Binary("17".into()), IfcValue::TypedValue(IfcTypedValue { name: "B".into(), items: vec![IfcValue::Real(f64::from_bits(0x7ff8_0000_0000_0042)), IfcValue::Integer(i64::MIN), IfcValue::Reference(u64::MAX)] })],
            }],
        }],
    }
}

#[test]
fn original_pack_receiving_preserves_owned_literal_values(){
 let source=snapshot();let wire=encode_pack(&source,&Default::default()).unwrap();let mut fixture=OriginalIfcFixture::new();
 let mut output=None;let turn=fixture.receive(|native,original_grant|{let mut owner=NativeSnapshotDecodeOwner::new(native,original_grant);<IfcSnapshot as ArtifactPackReceiving>::receive_pack(&wire,&mut output,&mut owner)});assert!(turn.work>0);turn.result.unwrap();let received=output.take().unwrap();
 assert!(crate::standards::v4::subsets::any::schema::snapshot::ifc_arguments_identical(&source.entities[0].complex[0].args,&received.entities[0].complex[0].args));assert_eq!(source.header,received.header);assert_eq!(source.entities[0].id,received.entities[0].id);
 fixture.close_value(received);fixture.drain_decoder();fixture.close_value(wire);fixture.close_value(source);fixture.finish();
}
#[test]
fn original_pack_receiving_does_not_replace_the_callers_cancelled_decoder(){
 let source=snapshot();let wire=encode_pack(&source,&Default::default()).unwrap();let mut fixture=OriginalIfcFixture::new();let token=fixture.alias();token.cancel_now();
 let mut output=None;let turn=fixture.receive(|native,original_grant|{let mut owner=NativeSnapshotDecodeOwner::new(native,original_grant);<IfcSnapshot as ArtifactPackReceiving>::receive_pack(&wire,&mut output,&mut owner)});let error=turn.result.unwrap_err();assert!(output.is_none());assert_eq!(error.kind(),ValueRefusalKind::Canceled);
 fixture.close_value(error);fixture.return_alias(token);fixture.drain_decoder();fixture.close_value(wire);fixture.close_value(source);fixture.finish();
}
#[test]
fn original_ifc_pack_cancelled_prefixes_drain_under_the_same_original_recipient(){
 use semio_os_kernel::{OriginalArtifactPackReceiving,OriginalArtifactSnapshotReceiving};
 for prefix in [0,1,8,64,512,4096]{let source=snapshot();let wire=encode_pack(&source,&Default::default()).unwrap();let mut fixture=OriginalIfcFixture::new();let token=fixture.alias();
  let mut output=None;let turn=fixture.receive(|native,original_grant|{let mut owner=NativeSnapshotDecodeOwner::new(native,original_grant);owner.receive_nested::<super::owned::IfcOriginalPackReceiving,IfcSnapshot,NativeSnapshotFailure>(&mut output,&mut |slot,produced,_marker,_unwind_failure,original|{
   let mut work=0;let result=(||{let g=original.remaining_grant();let(rx,p)=IfcSnapshot::admit_original_pack(&wire,Default::default(),original.native(),g)?.unwrap();work+=1;*slot=Some(rx);original.record_progress(p)?;
    for _ in 0..prefix{let rx=slot.as_mut().unwrap();if rx.is_ready(){break}let g=original.remaining_grant();let before=rx.demands(&wire,4096)?;let denied=rx.step(&wire,original.native(),4096,RetainedCloneGrant{maximum_items:0,..g});assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap(),Default::default());assert_eq!(rx.demands(&wire,4096)?,before);let turn=rx.step(&wire,original.native(),4096,g);work+=turn.work;original.record_progress(turn.result.map_err(NativeSnapshotFailure::from)?)?;}
    token.cancel_now();original.native().checkpoint()?;unreachable!()
   })();RetainedWorkOutcome{result,work}
  })});let error=turn.result.unwrap_err();assert!(output.is_none());assert_eq!(error.kind(),ValueRefusalKind::Canceled);fixture.close_value(error);fixture.return_alias(token);fixture.drain_decoder();fixture.close_value(wire);fixture.close_value(source);fixture.finish();
 }
}
fn node(children:Vec<usize>)->Node{Node{kind:if children.is_empty(){"unset"}else{"aggregate"}.into(),integer:None,real:None,text:None,reference:None,name:None,children}}
#[test]
fn invalid_flat_forests_refuse_without_publishing_a_graph(){
 for case in 0..4{
  let values=match case{0=>vec![node(vec![0])],1=>vec![node(vec![1,1]),node(Vec::new())],2=>vec![node(Vec::new()),node(Vec::new())],_=>(0..67).map(|i|node(if i==66{Vec::new()}else{vec![i+1]})).collect()};
  let frame=Frame{schema:"stdio.ifc".into(),file_description:vec![0],file_name:Vec::new(),file_schema:Vec::new(),entities:Vec::new(),values};
  let body=store::pack_rt::encode_document(&Frame::__dsl_spec(),&frame.__dsl_to_record(),&Default::default()).unwrap();let envelope=store::semio_format::SemioEnvelope::from_envelope_id(STDIO_IFC_DOCUMENT_SCHEMA,store::semio_format::Component::Pack,1).unwrap();let wire=store::semio_format::wrap_binary(&envelope,&body);let mut fixture=OriginalIfcFixture::new();
  let mut output=None;let turn=fixture.receive(|native,original_grant|{let mut owner=NativeSnapshotDecodeOwner::new(native,original_grant);<IfcSnapshot as ArtifactPackReceiving>::receive_pack(&wire,&mut output,&mut owner)});let error=turn.result.unwrap_err();assert!(output.is_none());assert_eq!(error.kind(),if case==3{ValueRefusalKind::DepthLimit}else{ValueRefusalKind::InvalidValue});
  fixture.close_value(error);fixture.drain_decoder();fixture.close_value(wire);fixture.close_value(body);fixture.close_value(frame);fixture.finish();
 }
}
#[test]
fn graph_completed_threshold_cancels_after_an_accepted_binary_turn_and_drains_original_custody(){
 use semio_os_kernel::{OriginalArtifactPackReceiving,OriginalArtifactSnapshotReceiving};use std::cell::Cell;
 let mut source=snapshot();source.entities[0].complex[0].args[0]=IfcValue::Binary(format!("0{}","F".repeat(8192)));let wire=encode_pack(&source,&Default::default()).unwrap();let mut fixture=OriginalIfcFixture::new();let token=fixture.alias();let observing=Arc::clone(&token);
 let armed=std::rc::Rc::new(Cell::new(false));let observer_armed=std::rc::Rc::clone(&armed);let completed=std::rc::Rc::new(Cell::new(0));let observer_completed=std::rc::Rc::clone(&completed);let was_binary=Cell::new(false);let returned=Cell::new(RetainedCloneProgress::default());let identity=Cell::new(None);
 let mut output=None;let turn=testkit::with_native_decode_progress(Box::new(move|progress|{let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{observer_completed.set(progress.completed);if observer_armed.get()&&progress.completed>=256{observing.cancel_now();}});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}),||fixture.receive(|native,original_grant|{
  identity.set(Some((native.ledger_identity(),native.retirement_recipient_identity())));let mut owner=NativeSnapshotDecodeOwner::new(native,original_grant);owner.receive_nested::<super::owned::IfcOriginalPackReceiving,IfcSnapshot,NativeSnapshotFailure>(&mut output,&mut |slot,produced,_marker,_unwind_failure,original|{
   let mut work=0;let result=(||{let g=original.remaining_grant();let(rx,p)=IfcSnapshot::admit_original_pack(&wire,Default::default(),original.native(),g)?.unwrap();work+=1;*slot=Some(rx);original.record_progress(p)?;
    for _ in 0..1_000_000{if slot.as_ref().unwrap().graph_phase().is_some(){break}let g=original.remaining_grant();let turn=slot.as_mut().unwrap().step(&wire,original.native(),4096,g);work+=turn.work;original.record_progress(turn.result.map_err(NativeSnapshotFailure::from)?)?;}
    assert_eq!(slot.as_ref().unwrap().graph_phase(),Some(false));let g=original.remaining_grant();let turn=slot.as_mut().unwrap().step(&wire,original.native(),1,g);work+=turn.work;original.record_progress(turn.result.map_err(NativeSnapshotFailure::from)?)?;
    let denied=slot.as_mut().unwrap().step(&wire,original.native(),1,RetainedCloneGrant{maximum_items:0,..g});assert_eq!(denied.work,0);assert_eq!(denied.result.unwrap(),Default::default());armed.set(true);
    let turn=original.drive_cursor::<super::owned::IfcOriginalPackReceiving,IfcSnapshot,NativeSnapshotFailure>(slot,produced,&mut |rx,_produced,native,g|{
     assert!(!rx.is_ready(),"graph must reach the original completed-work cancellation");let binary=rx.graph_phase()==Some(true);let turn=rx.step(&wire,native,1,g);let result=turn.result.map_err(|error|{was_binary.set(binary);returned.set(error.retained_progress());NativeSnapshotFailure::from(error)});RetainedWorkOutcome{result,work:turn.work}
    });work+=turn.work;turn.result
   })();RetainedWorkOutcome{result,work}
  })
 }));
 let error=turn.result.unwrap_err();assert!(output.is_none());assert_eq!(error.kind(),ValueRefusalKind::Canceled);assert_eq!(completed.get(),256);assert!(was_binary.get());assert_eq!(returned.get().copied_items,1);assert!(returned.get().copied_bytes>0);assert!(identity.get().unwrap().1.is_some());fixture.close_value(error);fixture.return_alias(token);fixture.drain_decoder();fixture.close_value(wire);fixture.close_value(source);fixture.finish();
}

#[test]
fn original_ifc_pack_publication_matches_the_independent_reference_codec() {
    let source = snapshot();
    let expected = encode_pack(&source, &Default::default()).unwrap();
    let mut observer = |_| true;
    let mut recipient = semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(64 * 1024 * 1024, &mut observer);
    native.install_retirement_recipient(&mut recipient).unwrap();
    let mut owner = semio_os_kernel::NativeSnapshotEncodeOwner::new(&mut native, grant());
    let turn = source.encode_exchange_pack(&mut owner);
    assert!(turn.work > 0);
    let wire = turn.result.unwrap();
    assert_eq!(wire, expected);
    assert!(owner.progress().copied_items > 0);
    drop(owner);
    close(wire);
    close(source);
}

