//! 🧫️ Registered native formats preserve original custody and match the shared independent oracle.
use super::*;
use semio_os_kernel::io::io_mechanism::{io_register,io_begin_owned_serialization};
use semio_framework_job::{OperationId, Generation, OriginalJobFixtureCaller};
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_io::byte_writer::physical_close as close_owned;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

const TURN_GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 4096,
    maximum_copy_bytes: 1 << 20,
    maximum_capacity_bytes: 1 << 25,
    maximum_release_bytes: 1 << 25,
    maximum_depth: 256,
};
const SOURCE_STOCK: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1 << 22,
    maximum_copy_bytes: 1 << 28,
    maximum_capacity_bytes: 1 << 26,
    maximum_release_bytes: 1 << 27,
    maximum_depth: TURN_GRANT.maximum_depth,
};

fn serializer_caller(operation: u64, generation: u64) -> OriginalJobFixtureCaller {
    OriginalJobFixtureCaller::admit(OperationId(operation), Generation(generation), SOURCE_STOCK, || Some(0), u64::MAX, 1 << 28)
}

fn advance(job: &mut dyn OwnedSerializerJob, caller: &mut OriginalJobFixtureCaller, fuel: u64) -> OwnedSerializerStep {
    caller.turn(TURN_GRANT, |cx| {
        cx.with_original_work_loan(fuel, cx.original_absolute_deadline_us(), |cx| {
            let result = job.advance_serialization(cx);
            assert!(cx.fuel_remaining() <= fuel);
            Ok(result)
        })
    }).unwrap()
}
fn corpus()->serde_json::Value{serde_json::from_str(fixture_text!("../../📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn handoffs()->serde_json::Value{serde_json::from_str(fixture_text!("../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/📤️serialization/📦️owned/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn terminal_grant(row:&serde_json::Value)->RetainedCloneGrant{let g=&row["grant"];RetainedCloneGrant{maximum_items:g["items"].as_u64().unwrap()as usize,maximum_copy_bytes:g["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:g["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:g["release"].as_u64().unwrap()as usize,maximum_depth:g["depth"].as_u64().unwrap()as usize}}
fn target(format:&str)->ArtifactDialect{let(kind,standard)=match format{"png"=>("s.stdio.png","1.2"),"svg"=>("s.stdio.svg","1.1"),_=>("s.stdio.pdf","1.4")};ArtifactDialect{artifact_kind:kind.into(),standard:standard.into(),subset:"*".into()}}
fn document()->DrawingSnapshot{let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🎬️scene/📋️prepare/🔣️.json")).unwrap();let source=rows.as_array().unwrap().iter().find(|row|row["name"]=="rectangle").unwrap();let mut doc:DrawingSnapshot=serde_json::from_value(source["document"].clone()).unwrap();doc.artboard=Some(semio_s_artifact_draw_drawing_model::DrawingArtboard{width:2.0,height:1.0});doc}
type Store = store::ArtifactStore<DrawingSnapshot, semio_s_artifact_draw_drawing_mutations::DrawingMutation>;

fn retirement_grant(demand: semio_framework_value::RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: demand.copy_bytes,
        maximum_capacity_bytes: demand.capacity_bytes,
        maximum_release_bytes: demand.release_bytes,
        maximum_depth: demand.depth,
    }
}

fn effect_turn<T>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let mut turn = semio_framework_job::OriginalRetirementPump::new()
        .turn(1 << 24, u64::MAX, || Some(0), grant).unwrap();
    turn.with_effect(run).unwrap().unwrap().0
}

fn progress(poll: OriginalRetirementPoll) -> RetainedCloneProgress {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.unwrap().progress(),
        RetainedWorkPoll::Pending { .. } => Default::default(),
    }
}

fn pump_returns(issuer: &mut Store, active: &mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>) {
    if active.is_none() && issuer.returned_snapshot_read_count() == 0 { return; }
    let demand = if let Some(owner) = active.as_ref() {
        let body = owner.next_copy_byte_demand().unwrap().max(owner.next_release_byte_demand().unwrap());
        store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body).unwrap()
    } else {
        issuer.returned_snapshot_read_retirement_demand().unwrap()
    };
    let grant = retirement_grant(demand);
    let (receipt, physical) = observe(|| effect_turn(grant, |effect| {
        if active.is_some() {
            return progress(store::os_store::snapshot_read::artifact_retirement_box_close_step(active, grant.maximum_copy_bytes, effect));
        }
        let (owner, receipt) = issuer.take_returned_snapshot_read_retirement(effect).unwrap();
        *active = owner;
        receipt
    }));
    assert!(receipt.fits(grant));
    assert!(!physical.overflowed);
    assert_eq!((physical.requested_bytes, physical.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
}

fn close_job(mut job: Box<dyn OwnedSerializerJob>, issuer: &mut Store, caller: &mut OriginalJobFixtureCaller) {
    let mut returned = None;
    job.begin_close();
    for _ in 0..2_000_000 {
        pump_returns(issuer, &mut returned);
        if job.terminal_is_empty() { break; }
        let copy = job.next_close_copy_byte_demand().unwrap();
        let release = job.next_close_release_byte_demand().unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy.max(release)).unwrap(),
            maximum_release_bytes: release,
            maximum_depth: job.next_close_depth_demand().unwrap(),
        };
        for axis in 0..5 {
            let mut denied = grant;
            let limit = match axis {
                0 => &mut denied.maximum_items,
                1 => &mut denied.maximum_copy_bytes,
                2 => &mut denied.maximum_capacity_bytes,
                3 => &mut denied.maximum_release_bytes,
                _ => &mut denied.maximum_depth,
            };
            if *limit == 0 { continue; }
            *limit -= 1;
            caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
                let (result, heap) = observe(|| cx.with_retained_loan(denied, |cx| job.close_step(cx)));
                assert!(!heap.overflowed);
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                match result {
                    Ok(step) => assert_eq!(step.progress(), Default::default()),
                    Err(error) => assert_eq!(error.effects, Default::default()),
                }
                Ok(())
            })).unwrap();
        }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            let (result, heap) = observe(|| job.close_step(cx));
            let receipt = result?.progress();
            assert!(receipt.fits(grant));
            assert!(!heap.overflowed);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
            Ok(())
        })).unwrap();
    }
    assert!(job.terminal_is_empty());
    for _ in 0..100000 {
        if returned.is_none() && issuer.returned_snapshot_read_count() == 0 { break; }
        pump_returns(issuer, &mut returned);
    }
    assert!(returned.is_none() && issuer.returned_snapshot_read_count() == 0);
    let frame = job.terminal_frame_release_bytes().unwrap();
    let (_, physical) = observe(|| drop(job));
    assert_eq!((physical.requested_bytes, physical.released_bytes), (0, frame));
}

#[semio_framework_async_macros::async_test]
async fn drawing_owned_serializer_original_routes(){
 io_register(semio_s_artifact_draw_drawing::io().entries).unwrap();let contract=corpus();
 for format in contract["formats"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()){
  for fuel in contract["workGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()){
   let mut issuer=store::ArtifactStore::new::<::semio_os_kernel::os_store::ArtifactSnapshotPublication>(store::os_store::document_materialize::create_document_envelope::<DrawingSnapshot,semio_s_artifact_draw_drawing_mutations::DrawingMutation>("drawing.drawing","captured-native-export",document(),None, &::semio_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer),protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();issuer.install_document_store_owners_exact(::semio_os_kernel::os_store::document_store_owners::funded_bounded_artifact_store_owners().expect("funded bounded document owners"));let generation=issuer.generation();let revision=issuer.content_revision_now();let read=issuer.snapshot_read().unwrap();let original=read.get()as*const DrawingSnapshot;let mut request=OwnedSerializerRequest{source:read.into_erased(),route:(semio_s_artifact_draw_drawing_model::DRAWING_DIALECT.into(),target(format)),generation,revision,options:DslValue::Object(vec![("width".into(),DslValue::uint(2)),("height".into(),DslValue::uint(1)),("transparent".into(),DslValue::Bool(true))]),children:None};let options=request.options.as_object().unwrap().as_ptr();let route=request.route.1.artifact_kind.as_ptr();let demand=serializer_demand(&request,match format{"png"=>"png","svg"=>"svg",_=>"pdf"}).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
   for axis in contract["deniedBirth"].as_array().unwrap(){let mut denied=grant;match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"capacity"=>denied.maximum_capacity_bytes-=1,_=>denied.maximum_depth=0}let(result,physical)=observe(||io_begin_owned_serialization(request,denied));let refusal=match result{Err(refusal)=>refusal,Ok(_)=>panic!("Denied constructor admitted")};assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(refusal.progress,Default::default());request=refusal.request;assert_eq!(request.source.get::<DrawingSnapshot>().unwrap()as*const DrawingSnapshot,original);assert_eq!(request.options.as_object().unwrap().as_ptr(),options);assert_eq!(request.route.1.artifact_kind.as_ptr(),route);}
   let(result,physical)=observe(||io_begin_owned_serialization(request,grant));let admission=match result{Ok(admission)=>admission,Err(refusal)=>panic!("Native route refused: {}",refusal.error)};assert_eq!(physical.requested_bytes,admission.progress.retained_capacity_bytes);assert_eq!(physical.released_bytes,0);assert_eq!(admission.progress.retained_capacity_bytes,demand.capacity_bytes);let mut job=admission.job;let mut caller=serializer_caller(17,generation);assert_eq!(advance(&mut *job,&mut caller,0),OwnedSerializerStep::Pending);let mut completed=false;for _ in 0..1_000_000{match advance(&mut *job,&mut caller,fuel){OwnedSerializerStep::Pending=>{},OwnedSerializerStep::Complete=>{completed=true;break;},other=>panic!("Native serializer failed: {other:?} {}",job.fault().map_or("",|fault|fault.message.as_str()))}}assert!(completed);let output=job.output().unwrap();let page=output.page(0).unwrap().as_ptr();for row in handoffs()["terminalHandoffs"].as_array().unwrap().iter().filter(|row|row["kind"]=="output"&&row["current"]==true&&row["finished"]==true&&row["available"]==true&&row["cancelled"]==false&&row["accepted"]==false){let denied=terminal_grant(row);let(result,physical)=observe(||job.take_output(denied));assert!(result.is_err());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(job.output().unwrap().page(0).unwrap().as_ptr(),page);}let(result,physical)=observe(||job.take_output(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}));let handoff=result.unwrap().unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(handoff.progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert_eq!(handoff.output.page(0).unwrap().as_ptr(),page);assert!(job.output().is_none());let bytes=(0..handoff.output.page_count()).flat_map(|index|handoff.output.page(index).unwrap().iter().copied()).collect::<Vec<_>>();
   if format=="png"{let decoder=png::Decoder::new(std::io::Cursor::new(&bytes));let mut reader=decoder.read_info().unwrap();let mut pixels=vec![0;reader.output_buffer_size()];let info=reader.next_frame(&mut pixels).unwrap();assert_eq!((info.width,info.height),(2,1));assert_eq!(pixels[..info.buffer_size()],contract["expected"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as u8).collect::<Vec<_>>());}
   if let Some(directory)=std::env::var_os("SEMIO_TEST_ARTIFACT_DIR"){let directory=std::path::PathBuf::from(directory);std::fs::create_dir_all(&directory).unwrap();std::fs::write(directory.join(format!("{format}-{fuel}.{format}")),&bytes).unwrap();}close_job(job,&mut issuer,&mut caller);caller.retire_exact();close_owned(handoff.output);if fuel==1{for stop in contract["cancelTurns"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()){let read=issuer.snapshot_read().unwrap();let request=OwnedSerializerRequest{source:read.into_erased(),route:(semio_s_artifact_draw_drawing_model::DRAWING_DIALECT.into(),target(format)),generation,revision,options:DslValue::Null,children:None};let mut cancelled=match io_begin_owned_serialization(request,grant){Ok(admission)=>admission.job,Err(refusal)=>panic!("{}",refusal.error)};let mut cancel_caller=serializer_caller(18,generation);for _ in 0..stop{if advance(&mut *cancelled,&mut cancel_caller,1)!=OwnedSerializerStep::Pending{break;}}if stop==40{for _ in 0..1_000_000{if cancelled.output().is_some(){break;}assert_ne!(advance(&mut *cancelled,&mut cancel_caller,4096),OwnedSerializerStep::Fault);}assert!(cancelled.output().is_some());}cancel_caller.cancel().cancel().await;assert_eq!(advance(&mut *cancelled,&mut cancel_caller,1),OwnedSerializerStep::Cancelled);assert!(cancelled.output().is_none());assert!(cancelled.take_output(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}).is_err());close_job(cancelled,&mut issuer,&mut cancel_caller);cancel_caller.retire_exact();}}
   if fuel==1&&format=="png"{let request=OwnedSerializerRequest{source:issuer.snapshot_read().unwrap().into_erased(),route:(semio_s_artifact_draw_drawing_model::DRAWING_DIALECT.into(),target(format)),generation,revision,options:DslValue::Object(vec![("width".into(),DslValue::uint(contract["fault"]["width"].as_u64().unwrap())),("height".into(),DslValue::uint(contract["fault"]["height"].as_u64().unwrap()))]),children:None};let mut failed=match io_begin_owned_serialization(request,grant){Ok(admission)=>admission.job,Err(refusal)=>panic!("{}",refusal.error)};let mut fault_caller=serializer_caller(19,generation);let mut observed=false;for _ in 0..10{if advance(&mut *failed,&mut fault_caller,1)==OwnedSerializerStep::Fault{observed=true;break;}}assert!(observed);let message=failed.fault().unwrap().message.as_ptr();assert!(failed.output().is_none());for row in handoffs()["terminalHandoffs"].as_array().unwrap().iter().filter(|row|row["kind"]=="fault"&&row["current"]==true&&row["finished"]==true&&row["available"]==true&&row["cancelled"]==false&&row["accepted"]==false){let denied=terminal_grant(row);let(result,physical)=observe(||failed.take_fault(denied));assert!(result.is_err());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(failed.fault().unwrap().message.as_ptr(),message);}let(result,physical)=observe(||failed.take_fault(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()}));let handoff=result.unwrap().unwrap();assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(handoff.fault.message.as_ptr(),message);assert_eq!(handoff.progress,RetainedCloneProgress{copied_items:1,..Default::default()});assert!(failed.fault().is_none()&&failed.output().is_none());close_job(failed,&mut issuer,&mut fault_caller);fault_caller.retire_exact();close_owned(handoff.fault);}for _ in 0..1_000_000{if issuer.close_owned_terminal_is_empty(){break;}let demand=issuer.close_owned_demands(4096).unwrap();effect_turn(retirement_grant(demand),|effect|progress(issuer.close_owned_step(effect)));}assert!(issuer.close_owned_terminal_is_empty());
  }
 }
}

#[path="🎟️font/🦀️.rs"]
mod original_font_block;
