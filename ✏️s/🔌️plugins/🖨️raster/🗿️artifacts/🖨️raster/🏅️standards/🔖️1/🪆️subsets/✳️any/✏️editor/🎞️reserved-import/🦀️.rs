//! 🎞️ The reserved wrapper retains original media, snapshot and completion; import work owns only private candidates.
use super::*;
#[path="🪜️base64/🦀️.rs"]
mod base64_ingress;
use semio_plugin::{ArtifactReservedToolInput,ArtifactReservedToolJobRequest,EditorApp,Emit,EphemeralEmit,MediaPayload};
use semio_plugin::tool_contract::ArtifactReservedWork;
use semio_framework_job::{StepContext,OriginalTurnFailure,InteractiveJobStep,InteractiveJobCloseStep,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeKind,JobOutcomeView};
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_framework_value::{RetirementDemand,RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,ValueError,retained_clone::RetainedCloneClose};
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use crate::config::RasterConfigMutation;
use semio_s_artifact_raster_raster_model::{RasterImagePayload,RasterLayerNode,SemioImageSnapshot};
pub(super) const RASTER_IMPORT_TOOL_ID:&str="import-media";
const RASTER_IMPORT_PORT:&str="image:in";
type Request=ArtifactReservedToolJobRequest<EditorApp<RasterPlayApp>>;
type Emission=(Result<Emit<RasterMutation,RasterConfigMutation>,semio_plugin::Fault>,EphemeralEmit<EditorApp<RasterPlayApp>>);
type Turn<'a>=Result<Option<JobOutcomeBorrow<'a>>,OriginalTurnFailure>;
type Candidate=(String,Option<SemioImageSnapshot>,RasterLayerNode);

/// 🧳️ Every original input lane remains in the caller-bound request through publication and paid retirement.
#[derive(Default)]
pub(super) struct RasterImportJob{
 codec_failure:semio_framework_job::OriginalDomainReservation<base64_ingress::CodecFailure>,
 ingress:base64_ingress::Base64Ingress,
 png:base64_ingress::PngIngress,
 builder:base64_ingress::RasterCandidateBuilder,
 candidate:Option<Candidate>,
 asset:Option<RasterImagePayload>,
 mutations:Option<Vec<RasterMutation>>,
 emission:Option<Emission>,
 decode_error:Option<String>,
 checkpoint:RetainedJobPublication,
 fault:RetainedJobPublication,
 retirement:RetainedCloneClose,
 fault_text:Option<&'static str>,
 decoded:bool,
 checkpoint_delivered:bool,
 completed:bool,
 closing:bool,
}
fn import_invalid()->ValueError{ValueError::invariant("Raster original reserved import lost its required media or candidate")}
impl RasterImportJob{
 fn fault_outcome<'a>(&'a mut self,cx:&mut StepContext<'_>)->Turn<'a>{
  let source=if let Some(error)=self.decode_error.as_ref(){error.as_bytes()}else{self.fault_text.unwrap_or("Raster import refused").as_bytes()};
  self.fault.advance_from_source(JobPublicationKind::Fault,&source[..source.len().min(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)],cx)
 }
 fn decode_step(&mut self,request:&mut Request,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  if self.ingress.failure.is_some(){let source=&mut self.ingress.failure;self.codec_failure.capture_original(cx,||base64_ingress::CodecFailure::Base64(source.take().unwrap()))?;return Ok(())}
  if self.png.failure.is_some(){let source=&mut self.png.failure;self.codec_failure.capture_original(cx,||base64_ingress::CodecFailure::Png(source.take().unwrap()))?;return Ok(())}
  if self.codec_failure.pending(){if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.codec_failure,cx)?{return Err(failure)}return Ok(())}
  if !self.codec_failure.ready(){self.codec_failure.prepare(cx)?;return Ok(())}
  let Self{ingress,png,builder,candidate,fault_text,codec_failure,..}=self;
  codec_failure.with_body(cx,|same,_slot|{
   let result=(||{
    if !ingress.started(){
     let Some(ArtifactReservedToolInput::Media{port,media})=request.input.as_mut()else{return Err(import_invalid().into())};
     if port!=RASTER_IMPORT_PORT{*fault_text=Some("Raster import only implements image:in");return Ok(())}
     let MediaPayload::Structured{schema,json}=&mut media.payload else{*fault_text=Some("Raster image:in requires Structured base64 PNG media");return Ok(())};
     if !<EditorApp<RasterPlayApp> as semio_plugin::ArtifactApp>::accepts_original_structured_media(port,schema,&media.media_type){*fault_text=Some("Raster image:in requires the declared 2d.image raster wire");return Ok(())}
     ingress.begin(json,same)?;return Ok(())
    }
    if !png.started()&&ingress.bytes.is_none(){ingress.step(same)?;return Ok(())}
    if !png.ready(){png.step(&mut ingress.bytes,same)?;return Ok(())}
    builder.step(png,candidate,same)?;Ok(())
   })();
   result
  })?;Ok(())
 }
 fn advance_image_payload(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.asset.is_some(){return Ok(true)}
  if cx.fuel_remaining()<2{return Ok(false)}
  let demand=RasterImagePayload::owned_birth_demands();let bytes=std::mem::size_of::<Option<SemioImageSnapshot>>()+std::mem::size_of::<RasterImagePayload>();let original=cx.retained_grant();
  let candidate=self.candidate.as_mut().ok_or_else(import_invalid)?;
  semio_framework_job::admit_original_construction(cx,&mut self.asset,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:demand.capacity_bytes,release_bytes:0,depth:demand.depth},|_|{
   let image=match candidate.1.take(){Some(image)=>image,None=>return RetainedWorkOutcome{result:Err(import_invalid()),work:1}};
   let turn=RasterImagePayload::from_owned(image,original);
   RetainedWorkOutcome{work:turn.work+1,result:match turn.result{Ok((asset,mut progress))=>{progress.copied_bytes+=bytes;Ok((asset,progress))},Err((error,image))=>{candidate.1=Some(image);Err(error)}}}
  })
 }
 fn assemble_mutations(&mut self,index:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.decoded{return Ok(true)}
  if self.candidate.as_ref().is_none_or(|candidate|candidate.1.is_some())||self.asset.is_none(){return Err(import_invalid().into())}
  let rows=2*std::mem::size_of::<RasterMutation>();let layer=std::mem::size_of::<RasterLayerNode>();let bytes=std::mem::size_of::<Candidate>()+std::mem::size_of::<Option<RasterImagePayload>>()+rows+layer;
  let mut done=None;
  semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:rows+layer,release_bytes:0,depth:1},|_|{
   let(asset_id,_,layer)=self.candidate.take().unwrap();let asset=self.asset.take().unwrap();
   self.mutations=Some(vec![
    RasterMutation::AddLayerAsset(semio_s_artifact_raster_raster_mutations_pixels::add_layer_asset::mutation::AddLayerAsset{asset_id,asset}),
    RasterMutation::CreateLayer(semio_s_artifact_raster_raster_mutations_layers::create_layer::mutation::CreateLayer{parent_id:None,index,layer:Box::new(layer)}),
   ]);self.decoded=true;
   RetainedWorkOutcome{result:Ok(((),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:rows+std::mem::size_of::<RasterLayerNode>(),..Default::default()})),work:1}
  })
 }
 fn step_value<'a>(&'a mut self,request:&mut Request,cx:&mut StepContext<'_>)->Turn<'a>{
  if cx.is_cancelled(){return JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)}
  if cx.should_yield(){return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
  if self.decode_error.is_some()||self.fault_text.is_some(){return self.fault_outcome(cx)}
  if !self.decoded{
   cx.set_stage("raster-import-decode");
   if self.candidate.is_none(){self.decode_step(request,cx)?;}
   else if self.asset.is_none(){self.advance_image_payload(cx)?;}
   else{self.assemble_mutations(request.snapshot.layers.len(),cx)?;}
   return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)
  }
  if !self.checkpoint_delivered{
   let outcome=self.checkpoint.advance_from_source(JobPublicationKind::Checkpoint{applied_progress:1},&[1],cx)?;
   if outcome.is_some(){self.checkpoint_delivered=true;}return Ok(outcome)
  }
  cx.set_stage("raster-import-publish");
  if !self.completed{
   if !request.completion.has_mounted_consumer(){self.fault_text=Some("Raster import completion consumer is absent");return self.fault_outcome(cx)}
   if self.emission.is_none(){
    let bytes=std::mem::size_of::<Option<Vec<RasterMutation>>>()+std::mem::size_of::<Option<Emission>>();
    let mut moved=None;
    semio_framework_job::admit_original_construction(cx,&mut moved,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{
     let result=(||{let mutations=self.mutations.take().ok_or_else(import_invalid)?;self.emission=Some((Ok(Emit{artifact_mutations:mutations,ui_scope:semio_framework::kernel::UiDirtyScope::Full,..Default::default()}),EphemeralEmit::default()));Ok(((),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))})();
     RetainedWorkOutcome{result,work:1}
    })?;
    return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)
   }
   if !request.completion.complete_emit_original(&mut self.emission,&mut self.completed,cx)?{return JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
  }
  JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)
 }
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.fault.terminal_is_empty(){return self.fault.retirement_demands()}
  if !self.checkpoint.terminal_is_empty(){return self.checkpoint.retirement_demands()}
  if !self.codec_failure.terminal_is_empty(){return self.codec_failure.retirement_demands(body)}
  if !self.builder.empty(){return self.builder.close_demands(body)}
  if !self.png.empty(){return self.png.close_demands(body)}
  if !self.ingress.empty(){return self.ingress.close_demands(body)}
  macro_rules! demand{($field:ident,$ty:ty)=>{if self.$field.is_some()||!self.retirement.is_empty(){return self.retirement.next_owner_demands::<$ty>(self.$field.is_some(),&None,body)}};}
  demand!(candidate,Candidate);demand!(asset,RasterImagePayload);demand!(mutations,Vec<RasterMutation>);demand!(emission,Emission);demand!(decode_error,String);Ok(Default::default())
 }
 fn close_work(&self,body:usize)->Result<u64,ValueError>{
  if !self.fault.terminal_is_empty(){return self.fault.retirement_work()}if !self.checkpoint.terminal_is_empty(){return self.checkpoint.retirement_work()}
  if !self.codec_failure.terminal_is_empty(){return self.codec_failure.retirement_work(body)}
  if !self.builder.empty(){return self.builder.close_work(body)}if !self.png.empty(){return self.png.close_work(body)}if !self.ingress.empty(){return self.ingress.close_work(body)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<Candidate>(false,&None,body)}
  macro_rules! quote{($field:ident,$ty:ty)=>{if self.$field.is_some(){return self.retirement.next_owner_work::<$ty>(true,&None,body)}};}
  quote!(candidate,Candidate);quote!(asset,RasterImagePayload);quote!(mutations,Vec<RasterMutation>);quote!(emission,Emission);quote!(decode_error,String);Ok(0)
 }
 fn close_turn(&mut self,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{
  if !self.fault.terminal_is_empty(){let turn=self.fault.close_step(grant);return RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
  if !self.checkpoint.terminal_is_empty(){let turn=self.checkpoint.close_step(grant);return RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
  if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}
  if !self.codec_failure.terminal_is_empty(){let turn=self.codec_failure.close_step(grant);return RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
  if !self.builder.empty(){return self.builder.close_step(grant)}
  if !self.png.empty(){return self.png.close_step(grant)}
  if !self.ingress.empty(){return self.ingress.close_step(grant)}
  macro_rules! retire{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,grant);return RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work}}};}
  retire!(candidate);retire!(asset);retire!(mutations);retire!(emission);retire!(decode_error);
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
}
impl ArtifactReservedWork<EditorApp<RasterPlayApp>> for RasterImportJob{
 fn step<'a>(&'a mut self,request:&'a mut Request,cx:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->InteractiveJobStep<'a>{InteractiveJobStep::from_callback(self.step_value(request,cx))}
 fn borrow_outcome<'a>(&'a self,d:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{
  match d.kind(){JobOutcomeKind::Yield=>d.yielded(),JobOutcomeKind::Cancelled=>d.cancelled(),JobOutcomeKind::Complete=>d.complete(None,None),JobOutcomeKind::CheckpointReady{..}=>self.checkpoint.borrow_outcome(d),JobOutcomeKind::Fault=>self.fault.borrow_outcome(d),JobOutcomeKind::PreviewReady=>Err(import_invalid())}
 }
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.begin_close();let demand=self.demands(cx.retained_grant().maximum_copy_bytes)?;
  let work=self.close_work(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_quoted_retained_step(cx,demand,work,|grant|self.close_turn(grant))?;
  Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.codec_failure.terminal_is_empty()&&self.ingress.empty()&&self.png.empty()&&self.builder.empty()&&self.candidate.is_none()&&self.asset.is_none()&&self.mutations.is_none()&&self.emission.is_none()&&self.decode_error.is_none()&&self.retirement.is_empty()&&self.checkpoint.terminal_is_empty()&&self.fault.terminal_is_empty()}
}
#[cfg(test)]
mod original_close_laws{
 use super::*;
 use semio_plugin::tool_contract::ArtifactReservedWork;
 use semio_plugin::artifact_app_laws::OriginalWorkFixture;
 #[test]
 fn original_image_payload_admission_preserves_denied_source_and_moves_pixel_storage(){
  let mut caller=OriginalWorkFixture::new();let grant=OriginalWorkFixture::stock();
  let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(1,1,vec![0,0,0,255]);let pointer=image.frames[0].rgba8.as_ptr();
  let layer=semio_s_artifact_raster_raster_model::create_pixel_layer("Imported Image",1,1);
  let mut work=RasterImportJob{candidate:Some(("original-image".into(),Some(image),layer)),..Default::default()};
  for axis in ["items","copy","capacity","depth","work"]{
   let mut denied=grant;match axis{"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes=0,"capacity"=>denied.maximum_capacity_bytes=0,"depth"=>denied.maximum_depth=0,_=>{}}
   caller.work_turn(denied,if axis=="work"{0}else{32},|cx|{assert!(!work.advance_image_payload(cx)?);Ok(())}).unwrap();
   assert_eq!(work.candidate.as_ref().unwrap().1.as_ref().unwrap().frames[0].rgba8.as_ptr(),pointer);assert!(work.asset.is_none());
  }
  caller.turn(grant,|cx|{assert!(work.advance_image_payload(cx)?);Ok(())}).unwrap();
  assert!(work.candidate.as_ref().unwrap().1.is_none());assert_eq!(work.asset.as_ref().unwrap().image().frames[0].rgba8.as_ptr(),pointer);
  for axis in ["items","copy","capacity","depth","work"]{
   let mut denied=grant;match axis{"items"=>denied.maximum_items=0,"copy"=>denied.maximum_copy_bytes=0,"capacity"=>denied.maximum_capacity_bytes=0,"depth"=>denied.maximum_depth=0,_=>{}}
   caller.work_turn(denied,if axis=="work"{0}else{32},|cx|{assert!(!work.assemble_mutations(3,cx)?);Ok(())}).unwrap();
   assert!(work.candidate.is_some()&&work.asset.is_some()&&work.mutations.is_none());assert_eq!(work.asset.as_ref().unwrap().image().frames[0].rgba8.as_ptr(),pointer);
  }
  caller.turn(grant,|cx|{assert!(work.assemble_mutations(3,cx)?);Ok(())}).unwrap();
  assert!(work.candidate.is_none()&&work.asset.is_none());let rows=work.mutations.as_ref().unwrap();assert_eq!(rows.len(),2);
  let RasterMutation::AddLayerAsset(added)=&rows[0]else{panic!("first mutation attaches the real image")};assert_eq!(added.asset.image().frames[0].rgba8.as_ptr(),pointer);
  let RasterMutation::CreateLayer(created)=&rows[1]else{panic!("second mutation creates its layer")};assert_eq!(created.index,3);
  work.begin_close();for _ in 0..10000{if work.terminal_is_empty(){break}caller.turn(grant,|cx|{work.close_step(cx)?;Ok(())}).unwrap();}
  assert!(work.terminal_is_empty());caller.finish();
 }
 #[test]
 fn original_structured_wire_matches_the_declared_raster_input(){
  let corpus:serde_json::Value=serde_json::from_str(include_str!("🧪️structured-wire/cases.json")).unwrap();
  for row in corpus["cases"].as_array().unwrap(){
   let media_type=semio_plugin::MediaType{class:match row["mediaClass"].as_str().unwrap(){"twoD"=>semio_plugin::MediaClass::TwoD,_=>semio_plugin::MediaClass::Data},form:match row["form"].as_str().unwrap(){"raster"=>semio_plugin::MediaForm::Raster,"vector"=>semio_plugin::MediaForm::Vector,_=>semio_plugin::MediaForm::Value}};
   assert_eq!(<EditorApp<RasterPlayApp> as semio_plugin::ArtifactApp>::accepts_original_structured_media(row["port"].as_str().unwrap(),row["schema"].as_str().unwrap(),&media_type),row["accepted"].as_bool().unwrap());
  }
 }
 #[test]
 fn decode_fault_owner_survives_denied_close_and_retires_on_its_original_caller(){
  let mut caller=OriginalWorkFixture::new();let grant=OriginalWorkFixture::stock();
  let mut work=RasterImportJob{decode_error:Some("original malformed PNG diagnostic".to_owned()),..Default::default()};
  let pointer=work.decode_error.as_ref().unwrap().as_ptr();
  work.begin_close();
  caller.work_turn(grant,0,|cx|{let step=work.close_step(cx)?;assert!(!matches!(step,InteractiveJobCloseStep::Complete{..}));Ok(())}).unwrap();
  assert_eq!(work.decode_error.as_ref().unwrap().as_ptr(),pointer);
  for _ in 0..10000{if work.terminal_is_empty(){break}caller.turn(grant,|cx|{work.close_step(cx)?;Ok(())}).unwrap();}
  assert!(work.terminal_is_empty());caller.finish();
 }
 #[test]
 fn original_reserved_wrapper_imports_real_media_and_returns_completion_on_same_caller(){
  use semio_plugin::{App,EditorApp,PluginApp};use semio_plugin::artifact_app_laws::{new_app_with_registry,fixture_identity,meta,OriginalFixtureScope,FixtureStepOutcome,fixture_step,close_registered_fixture_app};use semio_framework_job::InteractiveJob;
  fn manifest()->App{App{definition:crate::create_raster_app(),examples:Vec::new()}}
  fn identity(r:&Request)->([usize;13],RetainedCloneGrant,semio_framework::ToolExecutionContract,[u8;32]){let Some(ArtifactReservedToolInput::Media{media,..})=r.input.as_ref()else{panic!("original input")};let MediaPayload::Structured{json,..}=&media.payload else{panic!("original text")};([r.controller_id.as_ptr()as usize,r.tool_id.as_ptr()as usize,r.payload_schema_id.as_ptr()as usize,r.parent_document_id.as_ptr()as usize,r.authoring_seed.as_ptr()as usize,std::sync::Arc::as_ptr(&r.snapshot)as usize,r.snapshot_read.get()as*const _ as usize,std::sync::Arc::as_ptr(&r.config)as usize,std::sync::Arc::as_ptr(&r.history)as usize,r.children.root.as_ref().map_or(0,|root|std::sync::Arc::as_ptr(root)as usize),r.raw_wire.as_ptr()as usize,json.as_ptr()as usize,&r.completion as*const _ as usize],r.retained,r.contract,r.canonical_base_revision)}
  let policy=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;let mut scope=OriginalFixtureScope::admit();let mut app=semio_framework_async::poll::resolve_ready(new_app_with_registry::<EditorApp<RasterPlayApp>>(manifest,semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()),policy,&mut fixture_identity()));semio_framework_async::poll::resolve_ready(app.bind_instance_id(1));semio_framework_async::poll::resolve_ready(app.render_ctx().refresh_cache()).unwrap();
  let cases:serde_json::Value=serde_json::from_str(include_str!("🧪️structured-wire/image-cases.json")).unwrap();let text=cases["cases"][0]["base64"].as_str().unwrap().to_owned();let pointer=text.as_ptr();let mut media=Some(semio_plugin::Media{media_type:semio_plugin::MediaType{class:semio_plugin::MediaClass::TwoD,form:semio_plugin::MediaForm::Raster},payload:MediaPayload::Structured{schema:"2d.image".into(),json:text}});let meta=meta("local");let mut witnessed=false;let mut denied=false;
  for _ in 0..200000{let done={let cx=scope.cx();let deadline=cx.deadline_us();cx.with_original_work_loan(4096,deadline,|same|app.render_ctx().advance_reserved_media("image:in",&mut media,&meta,same)).unwrap()};if let Some(prep)=app.pending.reserved_preparation.as_ref(){if let Some(request)=prep.request.as_ref(){let(_,snapshot,config,history)=app.history.cache.as_ref().unwrap();assert!(std::sync::Arc::ptr_eq(&request.snapshot,snapshot));assert!(std::sync::Arc::ptr_eq(&request.config,config));assert!(std::sync::Arc::ptr_eq(&request.history,history));assert!(std::ptr::eq(request.snapshot_read.get(),request.snapshot.as_ref()));assert_eq!(request.tool_id,"import-media");assert_eq!(request.app_instance_id,1);assert!(!request.controller_id.is_empty());assert!(!request.payload_schema_id.is_empty());assert!(!request.parent_document_id.is_empty());assert!(!request.authoring_seed.is_empty());assert!(request.raw_wire.is_empty());assert!(request.source_binding.matches(scope.cx()));let Some(ArtifactReservedToolInput::Media{port,media})=request.input.as_ref()else{panic!("original media lane")};assert_eq!(port,"image:in");let MediaPayload::Structured{schema,json}=&media.payload else{panic!("original structured media")};assert_eq!(schema,"2d.image");assert_eq!(json.as_ptr(),pointer);assert!(request.ui_axes.is_none());assert_eq!(request.canonical_base_revision,app.stores.store.content_revision());assert!(request.completion.has_mounted_consumer());witnessed=true}}if witnessed&&!denied{let before=identity(app.pending.reserved_preparation.as_ref().unwrap().request.as_ref().unwrap());let cx=scope.cx();let deadline=cx.deadline_us();assert!(!cx.with_original_work_loan(0,deadline,|same|app.render_ctx().advance_reserved_media("image:in",&mut media,&meta,same)).unwrap());let after=identity(app.pending.reserved_preparation.as_ref().unwrap().request.as_ref().unwrap());assert_eq!(before,after);denied=true}if done{break}}
  assert!(witnessed&&denied);assert!(media.is_none());let mut origin=semio_framework_job::OriginalJobReturnOrigin::empty();for _ in 0..10000{if origin.body_ready(){break}origin.admit(scope.cx()).unwrap();}assert!(origin.body_ready());let mut checkpoint=false;let mut complete=false;
  for _ in 0..200000{let job=app.pending.reserved_preparation.as_mut().unwrap().job.as_mut().expect("original reserved wrapper");let outcome={let cx=scope.cx();let deadline=cx.deadline_us();let mut output=None;cx.with_original_work_loan(4096,deadline,|same|{output=Some(fixture_step(job,same,origin.receiver()?));Ok(())}).unwrap();output.unwrap()};match outcome{FixtureStepOutcome::Checkpoint{applied_progress,..}=>{assert_eq!(applied_progress,1);checkpoint=true},FixtureStepOutcome::Complete{..}=>{complete=true;break},FixtureStepOutcome::Idle|FixtureStepOutcome::Yield=>{},other=>panic!("unexpected Raster import outcome: {other:?}")}}
  assert!(checkpoint&&complete);let prep=app.pending.reserved_preparation.as_mut().unwrap();let completion=prep.completion.as_ref().unwrap();let mut emission=Some(completion.test_take_emit().unwrap().expect("original completion consumer"));let rows=&emission.as_ref().unwrap().0.as_ref().unwrap().artifact_mutations;assert_eq!(rows.len(),2);let RasterMutation::AddLayerAsset(added)=&rows[0]else{panic!("image mutation")};assert_eq!(added.asset.image().frames[0].rgba8,vec![10,20,30,255]);assert_eq!(added.asset.image().metadata.len(),2);assert!(matches!(&rows[1],RasterMutation::CreateLayer(_)));
  let mut retirement=RetainedCloneClose::default();for _ in 0..100000{if emission.is_none()&&retirement.is_empty(){break}let cx=scope.cx();let d=retirement.next_owner_demands::<Emission>(emission.is_some(),&None,cx.retained_grant().maximum_copy_bytes).unwrap();let work=retirement.next_owner_work::<Emission>(emission.is_some(),&None,cx.retained_grant().maximum_copy_bytes).unwrap();semio_framework_job::close_original_quoted_retained_step(cx,d,work,|grant|{if !retirement.is_empty(){return retirement.step_granted(grant)}let turn=retirement.begin_granted(&mut emission,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}}).unwrap();}assert!(emission.is_none()&&retirement.is_empty());
  let job=prep.job.as_mut().unwrap();InteractiveJob::begin_close(job);for _ in 0..100000{if InteractiveJob::terminal_is_empty(job){break}InteractiveJob::close_step(job,scope.cx()).unwrap();}assert!(InteractiveJob::terminal_is_empty(job));for _ in 0..10000{if origin.terminal_is_empty(){break}origin.close_step(scope.cx()).unwrap();}assert!(origin.terminal_is_empty());close_registered_fixture_app(&mut app,policy,scope.cx()).unwrap();
 }

}
