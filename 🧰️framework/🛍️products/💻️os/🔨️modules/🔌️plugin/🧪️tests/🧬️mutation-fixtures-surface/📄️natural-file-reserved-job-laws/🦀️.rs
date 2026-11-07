/// 📄️ Natural file reserved job laws of the surface fixture: the reserved job builder with its policies, step, outcome and close helpers and the accounting, cancellation, monolithic decoder and controlled decoder laws.

fn surface_reserved_request(
    snapshot_read: store::os_store::snapshot_read::SnapshotRead<SurfaceSnapshot>,
    data: Vec<u8>,
    maximum_work_units_per_step: u64,
    cx:&mut semio_framework_job::StepContext<'_>,
) -> (
    ArtifactReservedToolJobRequest<EditorApp<SurfaceEditorFixture>>,
    ArtifactToolCompletion<EditorApp<SurfaceEditorFixture>>,
    std::sync::Arc<SurfaceSnapshot>,
    std::sync::Arc<HistoryView>,
) {
    let snapshot = std::sync::Arc::new(SurfaceSnapshot::default());
    let history = std::sync::Arc::new(HistoryView::empty());
    let completion = ArtifactToolCompletion::<EditorApp<SurfaceEditorFixture>>::new();
    let request = ArtifactReservedToolJobRequest {
        source_binding:semio_framework::action_bus::OriginalToolSourceBinding::borrow(cx),
        retained: SURFACE_ONE_UNIT_POLICY,
        controller_id: SURFACE_CONTROLLER_ID.into(),
        tool_id: "framework.reserved.import-media".into(),
        payload_schema_id: "semio.testkit-surface-natural/v1".into(),
        contract: ToolExecutionContract::resumable(8_192, 8_192, maximum_work_units_per_step, 8_192, 7_500, 1, 1),
        app_instance_id: 7,
        ui_axes: None,
        parent_document_id: "surface-natural-file-fixture".into(),
        canonical_base_revision: [0; 32],
        authoring_seed: "surface-natural-file-fixture-admission".into(),
        snapshot: snapshot.clone(),
        snapshot_read,
        config: std::sync::Arc::new(NoConfig::default()),
        history: history.clone(),
        children: ChildContentView::EMPTY,
        raw_wire: Vec::new(),
        input: Some(ArtifactReservedToolInput::Media {
            port: NATURAL_FILE_PORT.into(),
            media: Media {
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                payload: MediaPayload::Intrinsic {
                    schema: "semio.testkit-surface-natural/v1".into(),
                    value: semio_framework_value::DslValue::Bytes(data),
                },
            },
        }),
        completion: completion.clone(),
    };
    (request,completion,snapshot,history)
}

fn surface_natural_file_reserved_job(snapshot_read:store::os_store::snapshot_read::SnapshotRead<SurfaceSnapshot>,data:Vec<u8>,maximum_work_units_per_step:u64,cx:&mut semio_framework_job::StepContext<'_>)->(ArtifactReservedToolJob,ArtifactToolCompletion<EditorApp<SurfaceEditorFixture>>,std::sync::Arc<SurfaceSnapshot>,std::sync::Arc<HistoryView>){
    let(request,completion,snapshot,history)=surface_reserved_request(snapshot_read,data,maximum_work_units_per_step,cx);
    let mut request=Some(request);let mut body=None;let mut job=None;
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
        if job.is_some(){break}
        cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{if body.is_none(){<SurfaceEditorFixture as ArtifactEditor>::build_reserved_tool_job(&mut request,&mut body,same)?;}else{ArtifactReservedToolJob::admit_boxed_original(&mut body,&mut job,same)?;}Ok(())}).expect("original natural-file retained admission");
    }
    let job=job.expect("mounted natural-file codec owns its original reserved route");assert!(request.is_none()&&body.is_none());
    (job, completion, snapshot, history)
}

#[derive(Default)]
struct SurfaceRequestCustodyWork{
    input:Option<ArtifactReservedToolInput>,
    retirement:semio_framework_value::retained_clone::RetainedCloneClose,
    consumed:bool,
    closing:bool,
}

#[test]
fn original_reserved_preparation_pages_metadata_moves_arguments_and_cancels_partial_sources(){
    type Prep=semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation<EditorApp<SurfaceEditorFixture>>;
    for cancel_phase in 0..=4{
        let text="argument 💧/Ä".repeat(80);let pointer=text.as_ptr();let expected=serde_json::to_vec(&("copy",Some(&text))).unwrap();let mut args=Some(semio_framework_value::DslValue::String(text));
        let mut caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original=caller.context();let cx=&mut original;let mut preparation=None;
        for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if preparation.is_some(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|Prep::admit(&mut preparation,[0;32],same)).unwrap();}
        let owner=preparation.as_mut().expect("original preparation admitted");
        if cancel_phase>=1{
            let page=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:1,..SURFACE_ONE_UNIT_POLICY};let mut complete=false;
            for turn in 0..32{complete=cx.with_retained_loan(page,|same|owner.copy_metadata(1,["copy",""],same)).unwrap();if cancel_phase==1&&owner.metadata.as_ref().unwrap()[1].len()==1{break}if complete{break}assert!(turn<31);}
            if cancel_phase>1{assert!(complete);assert_eq!(owner.metadata.as_ref().unwrap()[1],b"copy");}
        }
        if cancel_phase>=2{assert!(cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|owner.begin_action(&mut args,same)).unwrap());assert!(args.is_none());}
        if cancel_phase>=3{
            for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
                if cancel_phase==3&&owner.action_wire.as_ref().unwrap().phase()=="initialize"||cancel_phase==4&&owner.raw_wire.is_some(){break}
                cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|owner.advance_action_wire(8192,same)).unwrap();
            }
            if cancel_phase==3{assert_eq!(owner.action_wire.as_ref().unwrap().phase(),"initialize");}
            else{assert_eq!(owner.raw_wire.as_deref().unwrap(),expected);let actual=owner.action_returned.as_ref().unwrap().1.as_ref().unwrap().as_str().unwrap();assert_eq!(actual.as_ptr(),pointer);}
        }
        cx.original_cancel_token().cancel_now();
        for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if owner.terminal_is_empty(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|owner.close_step(same)).unwrap();}
        assert!(owner.terminal_is_empty());let demand=Prep::slot_close_demands(&preparation,32768).unwrap();let denied=semio_framework_value::RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..SURFACE_ONE_UNIT_POLICY};assert_eq!(cx.with_retained_loan(denied,|same|Prep::close_slot(&mut preparation,same)).unwrap().progress(),Default::default());assert!(preparation.is_some());cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|Prep::close_slot(&mut preparation,same)).unwrap();assert!(preparation.is_none());let fuel=cx.fuel_remaining();drop(original);caller.retire(fuel).unwrap();
    }
}

#[test]
fn original_completion_birth_denies_preserves_and_closes_each_partial_native_root(){
    type Birth=semio_plugin_decl_app::OriginalCompletionBirth<EditorApp<SurfaceEditorFixture>>;
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../📜️declaration/🧩️app/🛬️lifted/♻️retirement/🌱️birth/🧪️tests/🔣️.json")).unwrap();assert_eq!(law["completionBirth"]["phases"].as_array().unwrap().len(),3);
    for cancel_phase in 0..=3{
        let mut caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original=caller.context();let cx=&mut original;let mut birth=Birth::default();let mut output=None;
        if cancel_phase==1{cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|birth.advance(&mut output,same)).unwrap();}
        if cancel_phase>=2{for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if cancel_phase==2&&birth.output_ready()||cancel_phase==3&&output.is_some(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|birth.advance(&mut output,same)).unwrap();}if cancel_phase==2{assert!(birth.output_ready());}}
        if cancel_phase<3{assert!(output.is_none());}
        else{assert!(output.is_some());assert!(birth.terminal_is_empty());}
        cx.original_cancel_token().cancel_now();
        for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if birth.terminal_is_empty(){break}let demand=birth.close_demands();let denied=semio_framework_value::RetainedCloneGrant{maximum_items:0,..SURFACE_ONE_UNIT_POLICY};assert_eq!(cx.with_retained_loan(denied,|same|birth.close_step(same)).unwrap().progress(),Default::default());assert_eq!(birth.close_demands(),demand);cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|birth.close_step(same)).unwrap();}
        assert!(birth.terminal_is_empty());if let Some(output)=output{assert!(output.take_emit().unwrap().is_none());crate::app_testing::mutation_fixture::job_close::retire_completion(output,cx,SURFACE_JOB_POLICY);}
        let fuel=cx.fuel_remaining();drop(original);caller.retire(fuel).unwrap();
    }
}

#[test]
fn reserved_argument_wire_returns_actual_original_allocation_after_independent_encoding(){
    use semio_framework_value::{RetainedCloneStep,RetainedWorkOutcome};
    let mut caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original=caller.context();let cx=&mut original;
    let input="original 💧/Ä".repeat(300);let pointer=input.as_ptr();let expected=serde_json::to_vec(&input).unwrap();let mut owner=semio_framework_pack_json::ArtifactCanonicalJsonTreeWireCursor::new(input);let mut returned=None;let mut output=None;
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
        if output.is_some(){break}
        cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{
            if owner.phase()=="sourceClose"{
                let demand=owner.original_return_demands(same.retained_grant().maximum_copy_bytes)?;
                let denied=semio_framework_value::RetainedCloneGrant{maximum_items:0,..same.retained_grant()};let denied_turn=owner.return_original(&mut returned,denied);assert_eq!(denied_turn.work,0);assert_eq!(denied_turn.result.unwrap().progress(),Default::default());assert!(returned.is_none());assert_eq!(owner.original_return_demands(same.retained_grant().maximum_copy_bytes)?,demand);
                let turn=owner.return_original(&mut returned,same.retained_grant());semio_framework_job::receive_original_retained_work(same,turn)?;
            }else{
                let result=owner.advance(same.retained_grant());let work=owner.last_turn_work();let result=result.map(|step|{output=step.output;RetainedCloneStep::Progress(step.progress)});semio_framework_job::receive_original_retained_work(same,RetainedWorkOutcome{result,work})?;
            }
            Ok(())
        }).unwrap();
    }
    assert!(owner.terminal_is_empty());let returned=returned.expect("wire hands back the actual argument owner");assert_eq!(returned.as_ptr(),pointer);let output=output.unwrap();assert_eq!(output,expected);
    let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::retain((returned,output));
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if close.terminal_is_empty(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{let demand=close.selected_demand(same.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(same,demand,|grant|{let result=close.step(grant);RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())}).unwrap();}
    assert!(close.terminal_is_empty());let fuel=cx.fuel_remaining();drop(original);caller.retire(fuel).unwrap();
}

impl crate::tool_contract::ArtifactReservedWork<EditorApp<SurfaceEditorFixture>> for SurfaceRequestCustodyWork{
    fn step<'a>(&'a mut self,request:&'a mut ArtifactReservedToolJobRequest<EditorApp<SurfaceEditorFixture>>,cx:&mut semio_framework_job::StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{
        let result=if self.consumed{
            assert!(request.input.is_none(),"retained Work resumes after donating its actual input");
            semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)
        }else{
            let bytes=std::mem::size_of::<ArtifactReservedToolInput>();let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return semio_framework_job::InteractiveJobStep::from_callback(Ok(None))}
            assert!(request.source_binding.matches(cx));assert!(!request.controller_id.is_empty()&&!request.tool_id.is_empty()&&!request.payload_schema_id.is_empty()&&!request.parent_document_id.is_empty()&&!request.authoring_seed.is_empty());
            self.input=request.input.take();assert!(self.input.is_some());self.consumed=true;cx.consume_fuel(1);
            cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).map(|_|None).map_err(Into::into)
        };
        semio_framework_job::InteractiveJobStep::from_callback(result)
    }
    fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{descriptor.complete(None,None)}
    fn begin_close(&mut self){self.closing=true;}
    fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.retirement.next_owner_demands::<ArtifactReservedToolInput>(self.input.is_some(),&None,body)}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let demand=<Self as crate::tool_contract::ArtifactReservedWork<EditorApp<SurfaceEditorFixture>>>::close_demands(self,cx.retained_grant().maximum_copy_bytes)?;
        semio_framework_job::close_original_retained_step(cx,demand,|grant|if self.retirement.is_empty(){let turn=self.retirement.begin_granted(&mut self.input,grant);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(semio_framework_value::RetainedCloneStep::Complete(Default::default()))),work:turn.work}}else{self.retirement.step_granted(grant)})
    }
    fn terminal_is_empty(&self)->bool{self.closing&&self.input.is_none()&&self.retirement.is_empty()}
}

#[semio_framework_async_macros::async_test]
async fn reserved_request_keeps_all_lanes_on_denied_birth_and_continues_after_input_donation(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../📜️declaration/👷️job/🎟️request/♻️reserved/🧪️tests/🔣️.json")).unwrap();assert_eq!(fixture["ownedLanes"].as_array().unwrap().len(),13);
    for cancel in [0_u8,1,2]{
        let mut caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original=caller.context();let cx=&mut original;
        let mut app=new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
        let(mut request,completion,snapshot,history)=surface_reserved_request(app.stores.store.snapshot_read().unwrap(),vec![7;4096],64,cx);request.raw_wire=vec![3;1024];
        let mut request=Some(request);let mut body=None;let mut job=None;let source=request.as_ref().unwrap().authoring_seed.as_ptr();let snapshot_count=std::sync::Arc::strong_count(&snapshot);let history_count=std::sync::Arc::strong_count(&history);
        for axis in 0..4{
            let mut denied=SURFACE_ONE_UNIT_POLICY;match axis{0=>denied.maximum_items=0,1=>denied.maximum_copy_bytes=0,2=>denied.maximum_capacity_bytes=0,_=>denied.maximum_depth=0}
            assert!(!cx.with_retained_loan(denied,|same|crate::tool_contract::admit_reserved_work::<EditorApp<SurfaceEditorFixture>,SurfaceRequestCustodyWork>(&mut request,&mut body,same)).unwrap());
            let retained=request.as_ref().expect("denied birth keeps complete original request");assert_eq!(retained.authoring_seed.as_ptr(),source);assert!(body.is_none()&&retained.input.is_some());assert!(std::sync::Arc::ptr_eq(&retained.snapshot,&snapshot));assert_eq!(std::sync::Arc::strong_count(&snapshot),snapshot_count);assert_eq!(std::sync::Arc::strong_count(&history),history_count);
        }
        let mut no_release=SURFACE_ONE_UNIT_POLICY;no_release.maximum_release_bytes=0;for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if body.is_some(){break}cx.with_retained_loan(no_release,|same|crate::tool_contract::admit_reserved_work::<EditorApp<SurfaceEditorFixture>,SurfaceRequestCustodyWork>(&mut request,&mut body,same)).unwrap();}assert!(body.is_some(),"birth requires no fabricated release budget");
        let mut foreign_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut foreign=foreign_caller.context();assert!(foreign.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|semio_framework_job::InteractiveJob::close_step(&mut **body.as_mut().unwrap(),same)).is_err(),"foreign cancellation cannot mark or retire private Work");let foreign_fuel=foreign.fuel_remaining();drop(foreign);foreign_caller.retire(foreign_fuel).unwrap();
        for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
            if job.is_some(){break}
            cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{if body.is_none(){crate::tool_contract::admit_reserved_work::<EditorApp<SurfaceEditorFixture>,SurfaceRequestCustodyWork>(&mut request,&mut body,same)?;}else{ArtifactReservedToolJob::admit_boxed_original(&mut body,&mut job,same)?;}Ok(())}).unwrap();
        }
        assert!(request.is_none()&&body.is_none());let mut job=job.expect("original retained request body admitted");
        if cancel==1{cx.original_cancel_token().cancel_now();}
        else if cancel==2{assert!(matches!(surface_natural_file_step(&mut job,cx),crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle|crate::app_testing::artifact_app_laws::FixtureStepOutcome::Yield));cx.original_cancel_token().cancel_now();}
        let outcome=surface_natural_file_outcome(&mut job,cx);if cancel==0{assert!(matches!(outcome,crate::app_testing::artifact_app_laws::FixtureStepOutcome::Complete{..}));}else{assert!(matches!(outcome,crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled));}
        close_surface_natural_file_job(&mut job,cx);
        assert!(completion.take_emit().unwrap().is_none(),"custody Work publishes no replacement source");
        assert_eq!(std::sync::Arc::strong_count(&snapshot),snapshot_count-1);assert_eq!(std::sync::Arc::strong_count(&history),history_count-1);
        drop((snapshot,history));crate::app_testing::mutation_fixture::job_close::retire_completion(completion,cx,SURFACE_JOB_POLICY);
        close_registered_fixture_app(&mut app,crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).unwrap();let fuel=cx.fuel_remaining();drop(original);caller.retire(fuel).unwrap();
    }
}

const SURFACE_JOB_POLICY:semio_framework_job::RetainedCloneGrant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
#[test]
fn original_emission_handoff_moves_ok_payload_without_heap_and_accounts_busy_work(){
 type App=EditorApp<SurfaceEditorFixture>;type Birth=semio_plugin_decl_app::OriginalCompletionBirth<App>;
 let mut caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original=caller.context();let cx=&mut original;
 let mut birth=Birth::default();let mut completion=None;
 for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if completion.is_some(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|birth.advance(&mut completion,same)).unwrap();}
 let completion=completion.unwrap();assert!(birth.terminal_is_empty());
 let invocation=crate::ExtensionInvocation::new("original.extension","evaluate","{\"unicode\":\"💧\"}","complete");
 let pointer=invocation.request_json.as_ptr();
 let mut source=Some((Ok(Emit{extension_invocations:vec![invocation],..Default::default()}),EphemeralEmit::<App>::default()));let mut published=false;
 cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{
  let progress=same.retained_progress();let fuel=same.fuel_remaining();
  let(result,heap)=completion.with_busy_test_lock(||semio_framework_trace::observe_heap_allocations_on_this_thread(||completion.complete_emit_original(&mut source,&mut published,same)));
  assert!(!result?);assert_eq!(same.retained_progress(),progress);assert_eq!(fuel-same.fuel_remaining(),1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));Ok(())
 }).unwrap();
 assert_eq!(source.as_ref().unwrap().0.as_ref().ok().unwrap().extension_invocations[0].request_json.as_ptr(),pointer);
 cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{
  let before=same.retained_progress();let fuel=same.fuel_remaining();
  let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||completion.complete_emit_original(&mut source,&mut published,same));
  assert!(result?);assert_eq!(fuel-same.fuel_remaining(),1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(same.retained_progress().copied_bytes>before.copied_bytes);Ok(())
 }).unwrap();
 assert!(source.is_none()&&published);let accepted=completion.take_emit().unwrap().unwrap();assert_eq!(accepted.0.as_ref().ok().unwrap().extension_invocations[0].request_json.as_ptr(),pointer);
 let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::retain(accepted);
 for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if close.terminal_is_empty(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{let demand=close.selected_demand(same.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(same,demand,|grant|{let result=close.step(grant);semio_framework_value::RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())}).unwrap();}
 assert!(close.terminal_is_empty());crate::app_testing::mutation_fixture::job_close::retire_completion(completion,cx,SURFACE_JOB_POLICY);
 let fuel=cx.fuel_remaining();drop(original);caller.retire(fuel).unwrap();
}
#[test]
fn original_emission_handoff_preserves_denied_busy_and_occupied_sources(){
 type App=EditorApp<SurfaceEditorFixture>;
 type Birth=semio_plugin_decl_app::OriginalCompletionBirth<App>;
 let cases:serde_json::Value=serde_json::from_str(include_str!("../../../📜️declaration/👷️job/🎯️completion-and-owned-context/🧪️tests/emit-original.json")).unwrap();
 for row in cases["cases"].as_array().unwrap(){
  let mut caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original=caller.context();let cx=&mut original;
  let mut birth=Birth::default();let mut completion=None;
  for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if completion.is_some(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|birth.advance(&mut completion,same)).unwrap();}
  assert!(birth.terminal_is_empty());let completion=completion.unwrap();
  if row["receiver"].as_bool().unwrap(){
   let mut first=Some((Err(Fault::from("already accepted original fault")),EphemeralEmit::<App>::default()));let mut first_published=false;
   assert!(cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|completion.complete_emit_original(&mut first,&mut first_published,same)).unwrap());assert!(first.is_none());
  }
  let mut published=row["published"].as_bool().unwrap();
  let mut source=row["source"].as_bool().unwrap().then(||(Err(Fault::from("original emission fault 💧".repeat(40))),EphemeralEmit::<App>::default()));
  let pointer=source.as_ref().map(|(emit,_)|emit.as_ref().err().unwrap().message.as_ptr());
  let mut grant=SURFACE_ONE_UNIT_POLICY;if !row["items"].as_bool().unwrap(){grant.maximum_items=0;}if !row["copy"].as_bool().unwrap(){grant.maximum_copy_bytes=0;}if !row["depth"].as_bool().unwrap(){grant.maximum_depth=0;}
  if row["cancel"].as_bool().unwrap(){cx.original_cancel_token().cancel_now();}
  let mut outcome=None;let fuel=if row["fuel"].as_bool().unwrap(){8}else{0};
  cx.with_retained_loan(grant,|same|same.with_original_work_loan(fuel,same.deadline_us(),|same|{
   outcome=Some(if row["busy"].as_bool().unwrap(){completion.with_busy_test_lock(||completion.complete_emit_original(&mut source,&mut published,same))}else{completion.complete_emit_original(&mut source,&mut published,same)});Ok(())
  })).unwrap();
  let result=match outcome.unwrap(){Ok(true)=>"published",Ok(false)=>"pending",Err(_)=>"refused"};assert_eq!(result,row["result"].as_str().unwrap(),"{}",row["id"]);
  assert_eq!(source.is_some(),row["afterSource"].as_bool().unwrap(),"{}",row["id"]);assert_eq!(published,row["afterPublished"].as_bool().unwrap(),"{}",row["id"]);
  if let Some(source)=source.as_ref(){assert_eq!(source.0.as_ref().err().unwrap().message.as_ptr(),pointer.unwrap());}
  let taken=completion.take_emit().unwrap();assert_eq!(taken.is_some(),row["afterReceiver"].as_bool().unwrap(),"{}",row["id"]);
  if row["id"]=="publish"{assert_eq!(taken.as_ref().unwrap().0.as_ref().err().unwrap().message.as_ptr(),pointer.unwrap());}
  let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::retain((source,taken));
  for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if close.terminal_is_empty(){break}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|{let demand=close.selected_demand(same.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(same,demand,|grant|{let result=close.step(grant);semio_framework_value::RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())}).unwrap();}
  assert!(close.terminal_is_empty());crate::app_testing::mutation_fixture::job_close::retire_completion(completion,cx,SURFACE_JOB_POLICY);
  let fuel=cx.fuel_remaining();drop(original);caller.retire(fuel).unwrap();
 }
}
const SURFACE_ONE_UNIT_POLICY:semio_framework_job::RetainedCloneGrant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:262144,maximum_release_bytes:1048576,maximum_depth:4096};
thread_local!{static SURFACE_ORIGINS:std::cell::RefCell<std::collections::HashMap<usize,semio_framework_job::OriginalJobReturnOrigin>>=std::cell::RefCell::new(std::collections::HashMap::new());}
fn surface_prepare_origin(origin:&mut semio_framework_job::OriginalJobReturnOrigin,cx:&mut semio_framework_job::StepContext<'_>){
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if origin.body_ready(){return}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|origin.admit(same).map(|_|())).expect("original surface return origin admission")}
    assert!(origin.body_ready(),"original surface return origin admitted within its turn bound")
}
fn surface_close_origin(origin:&mut semio_framework_job::OriginalJobReturnOrigin,cx:&mut semio_framework_job::StepContext<'_>){
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){if origin.terminal_is_empty(){return}cx.with_retained_loan(SURFACE_ONE_UNIT_POLICY,|same|origin.close_step(same).map(|_|())).expect("original surface return origin close")}
    assert!(origin.terminal_is_empty(),"original surface return origin closed within its turn bound")
}
fn surface_natural_file_step(job:&mut ArtifactReservedToolJob,cx:&mut semio_framework_job::StepContext<'_>)->crate::app_testing::artifact_app_laws::FixtureStepOutcome{
    let key=job as *const ArtifactReservedToolJob as usize;
    SURFACE_ORIGINS.with(|origins|{let mut origins=origins.borrow_mut();let origin=origins.entry(key).or_insert_with(semio_framework_job::OriginalJobReturnOrigin::empty);surface_prepare_origin(origin,cx);
        let mut outcome=None;cx.with_retained_loan(SURFACE_JOB_POLICY,|same|{outcome=Some(crate::app_testing::artifact_app_laws::fixture_step(job,same,origin.receiver()?));Ok(())}).expect("original surface job event");outcome.expect("original surface event returned")})
}
fn surface_natural_file_outcome(job:&mut ArtifactReservedToolJob,cx:&mut semio_framework_job::StepContext<'_>)->crate::app_testing::artifact_app_laws::FixtureStepOutcome{
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){let outcome=surface_natural_file_step(job,cx);if !matches!(outcome,crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle|crate::app_testing::artifact_app_laws::FixtureStepOutcome::Yield){return outcome}}
    panic!("original surface job retained its outcome frontier")
}
fn close_surface_natural_file_job(job:&mut ArtifactReservedToolJob,cx:&mut semio_framework_job::StepContext<'_>){
    use semio_framework_job::{InteractiveJob as Job,InteractiveJobCloseStep as Step,RetainedCloneProgress};
    Job::begin_close(job);let zero=semio_framework_job::RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};assert_eq!(cx.with_retained_loan(zero,|same|Job::close_step(job,same)).unwrap().progress(),RetainedCloneProgress::default());
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
        if Job::terminal_is_empty(job){break}
        let demand=Job::next_close_release_byte_demand(job).expect("original Surface release demand");assert_eq!(Job::next_close_release_byte_demand(job).unwrap(),demand);
        if demand>64{let denied=semio_framework_job::RetainedCloneGrant{maximum_release_bytes:64,..SURFACE_JOB_POLICY};let step=cx.with_retained_loan(denied,|same|Job::close_step(job,same)).expect("original short-release close");assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(Job::next_close_release_byte_demand(job).unwrap(),demand);assert!(!Job::terminal_is_empty(job));}
        let step=cx.with_retained_loan(SURFACE_JOB_POLICY,|same|Job::close_step(job,same)).expect("original surface full close");assert!(step.progress().fits(SURFACE_JOB_POLICY));match step{Step::Pending{progress}=>assert!(progress!=RetainedCloneProgress::default(),"original Surface fixed policy made no progress"),Step::Complete{..}=>{},other=>panic!("original Surface fixed policy close retained its frontier: {other:?}")}
    }
    assert!(Job::terminal_is_empty(job),"original Surface exact terminal witness");
    let key=job as *const ArtifactReservedToolJob as usize;
    if let Some(mut origin)=SURFACE_ORIGINS.with(|origins|origins.borrow_mut().remove(&key)){surface_close_origin(&mut origin,cx)}
}

/// 🧵️ A mounted natural-file importer accounts owned input bytes across turns, observes
/// cancellation before decode, and refuses a monolithic decoder whose input exceeds its exact
/// per-turn grant. Both exits retire every retained owner with bounded positive progress.
#[semio_framework_async_macros::async_test]
async fn natural_file_reserved_job_accounts_cancels_refuses_and_retires_boundedly() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["registeredEditor"];
    let data = row["octets"].as_array().expect("registered editor octets").iter().map(|octet| octet.as_u64().expect("octet") as u8).collect::<Vec<_>>();
    let turn = row["accountBytesPerTurn"].as_u64().expect("account turn bytes");
    let cancel_after = row["cancelAfterAccountedBytes"].as_u64().expect("cancelled accounted bytes");
    let (mut cancelled, cancelled_completion, cancelled_snapshot, cancelled_history) = surface_natural_file_reserved_job(app.stores.store.snapshot_read().expect("surface store read"), data,turn,cx);
    let mut first = crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle;
    for _ in 0..64 {
        first = surface_natural_file_step(&mut cancelled,cx);
        if !matches!(first, crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle) { break; }
    }
    match first {
        crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { applied_progress, .. } => {
            assert_eq!(applied_progress, cancel_after);
        }
        other => panic!("first bounded natural-file turn must checkpoint accounted bytes, got {other:?}"),
    }
    cx.original_cancel_token().cancel_now();
    assert!(matches!(
        surface_natural_file_step(&mut cancelled,cx),
        crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled
    ));
    drop((cancelled_snapshot, cancelled_history));
    close_surface_natural_file_job(&mut cancelled,cx);
    crate::app_testing::mutation_fixture::job_close::retire_completion(cancelled_completion,cx,SURFACE_JOB_POLICY);

    close_registered_fixture_app(&mut app,crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).unwrap();
    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).unwrap();result
}

#[semio_framework_async_macros::async_test]
async fn natural_file_monolithic_decoder_refuses_and_retires_on_original_caller(){
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app=new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).unwrap();let row=&fixture["registeredEditor"];
    let oversized = usize::try_from(row["oversizedOctetCount"].as_u64().expect("oversized octet count")).expect("usize octet count");
    let maximum = u64::try_from(oversized - 1).expect("bounded maximum work");
    let (mut refused, refused_completion, refused_snapshot, refused_history) = surface_natural_file_reserved_job(app.stores.store.snapshot_read().expect("surface store read"), vec![0; oversized], maximum,cx);
    for expected in [maximum, u64::try_from(oversized).expect("oversized progress")] {
        match surface_natural_file_outcome(&mut refused,cx) {
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { applied_progress, .. } => {
                assert_eq!(applied_progress, expected);
            }
            other => panic!("natural-file accounting must checkpoint monotonically, got {other:?}"),
        }
    }
    match surface_natural_file_outcome(&mut refused,cx) {
        crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(bytes) => {
            let detail = String::from_utf8(bytes).expect("UTF-8 refusal");
            assert_eq!(detail, "natural-file codec requires a controlled decoder for this input size");
        }
        other => panic!("an oversized monolithic natural-file decoder must refuse deterministically, got {other:?}"),
    }
    close_surface_natural_file_job(&mut refused,cx);
    assert!(refused_completion.take_emit().expect("refused completion cell").is_none());
    drop((refused_snapshot, refused_history));
    crate::app_testing::mutation_fixture::job_close::retire_completion(refused_completion,cx,SURFACE_JOB_POLICY);
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🧭️ A format-owned decoder consumes a source larger than one scheduler grant over
/// monotonic checkpoints, publishes the independently computed value once, then retires its sole
/// source owner under the ordinary reserved-job close contract.
#[semio_framework_async_macros::async_test]
async fn natural_file_controlled_decoder_crosses_turns_and_retires_its_owner() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["controlledEditor"];
    let word_count = usize::try_from(row["wordCount"].as_u64().expect("controlled word count")).expect("usize word count");
    let last_word = i32::try_from(row["lastWord"].as_i64().expect("controlled last word")).expect("i32 last word");
    let work = row["maximumWorkUnitsPerStep"].as_u64().expect("controlled work grant");
    let expected_minimum_turns = usize::try_from(row["expectedMinimumTurns"].as_u64().expect("minimum turns")).expect("usize turns");
    let expected_decoded_items = usize::try_from(row["expectedDecodedItems"].as_u64().expect("decoded items")).expect("usize decoded items");
    assert_eq!(word_count, expected_decoded_items);
    let mut data = Vec::with_capacity(word_count * 4);
    for word in std::iter::repeat_n(0_i32, word_count - 1).chain(std::iter::once(last_word)) {
        data.extend_from_slice(&word.to_be_bytes());
    }
    let oracle = data.chunks_exact(4).map(|word| i32::from_be_bytes(word.try_into().expect("four-byte word"))).try_fold(0_i32, i32::checked_add).expect("independent checked sum");
    assert_eq!(oracle, last_word);

    let (mut job, completion, snapshot, history) = surface_natural_file_reserved_job(app.stores.store.snapshot_read().expect("surface store read"), data, work,cx);
    let mut checkpoints = 0;
    let mut last_progress = 0;
    loop {
        match surface_natural_file_step(&mut job,cx) {
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { applied_progress, .. } => {
                assert!(applied_progress >= last_progress, "controlled decoder checkpoints are monotonic");
                last_progress = applied_progress;
                checkpoints += 1;
            }
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Complete { .. } => {
                break;
            }
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Yield | crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle => {}
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled => panic!("controlled natural-file decode cancelled without a request"),
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(bytes) => {
                let detail = String::from_utf8_lossy(&bytes).into_owned();
                panic!("controlled natural-file decode faulted: {detail}");
            }
            other => panic!("unexpected controlled natural-file outcome: {other:?}"),
        }
    }
    assert!(checkpoints >= expected_minimum_turns, "fixture must cross many scheduler turns");
    let (emit, _) = completion.take_emit().expect("controlled completion cell").expect("controlled completion value");
    let emit = emit.expect("controlled import emit");
    assert!(emit.artifact_mutations.is_empty(), "natural-file Open is a document load: it emits no mutation");
    let loads = emit.effects.iter().filter_map(|effect| match effect { semio_framework::kernel::Effect::LoadDocument { pack, .. } => Some(pack), _ => None }).collect::<Vec<_>>();
    assert_eq!(loads.len(), 1, "the controlled decoder publishes exactly one document load");
    assert_eq!(<SurfaceSnapshot as pack::snapshot::ArtifactPack>::decode_pack(loads[0]).expect("loaded surface pack"), SurfaceSnapshot { count: oracle });
    drop((snapshot, history));
    close_surface_natural_file_job(&mut job,cx);
    crate::app_testing::mutation_fixture::job_close::retire_completion(completion,cx,SURFACE_JOB_POLICY);
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}
