//! 🧵️ Retains cancellable path simplification until one semantic geometry mutation is ready.
use super::*;
use crate::schema::geometry::editing::{PathEdit,simplify::PathSimplifyJob};
use semio_framework_plugin::retained_command::{ArtifactCommandWork,ArtifactCommandInputs,ArtifactCommandWorkStep};
use semio_framework_value::{list::PagedList,retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep},ValueError};
use crate::DrawingLayerNode;
type App=semio_framework_plugin::EditorApp<DrawingPlayApp>;
#[derive(semio_framework_value::RetireOwned)]
struct Workspace {source:PagedList<crate::PathSegment,{usize::MAX}>,job:Option<PathSimplifyJob>}
struct Work {workspace:Option<ControlledRetirement<Workspace>>,identity:Option<usize>,complete:bool,closing:bool}
impl Work {
    fn new()->Self {Self {workspace:None,identity:None,complete:false,closing:false}}
}
impl ArtifactCommandWork<App> for Work {
    fn terminal_frame_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
    fn tool_id(&self)->&'static str {"editPath"}
    fn work_demands(&self,_input:&ArtifactCommandInputs<'_,App>,_maximum_copy_bytes:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {Ok(semio_framework_value::RetirementDemand {copy_bytes:std::mem::size_of::<Self>()+std::mem::size_of::<crate::PathSegment>(),depth:1,..Default::default()})}
    fn extent(&self,command:&DrawingCommand,snapshot:&DrawingSnapshot,interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<App>>)->Option<usize> {
        let DrawingCommand::EditPath(payload)=command else {return None};
        if let PathEdit::Simplify {tolerance}=payload.edit.as_ref() {if !tolerance.is_finite()||!(1e-6..=1e6).contains(tolerance) {return None;}}
        let DrawingLayerNode::Path(path)=crate::schema::find_drawing_layer(snapshot,&payload.layer_id)? else {return None};
        (!self.complete&&path.segments.len()<=65536&&snapshot.layers.len()+snapshot.assets.len()<=4096&&interaction.selection.values().all(|value|value.ids.len()<=4096)).then_some(1)
    }
    fn step(&mut self,input:&ArtifactCommandInputs<'_,App>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<App>,Fault> {
        if self.complete||self.closing {return Err(Fault::from("drawing-path-work-terminal"));}
        if cx.is_cancelled() {return Err(Fault::from("Path simplification cancelled"));}
        let DrawingCommand::EditPath(payload)=input.command else {return Err(Fault::from("drawing-path-command-required"));};
        let PathEdit::Simplify {tolerance}=payload.edit.as_ref() else {
            let mut work=DrawingWindowCommandWork::new("editPath");
            let result=work.step(input,cx)?;self.complete=true;return Ok(result);
        };
        let identity=input.snapshot as*const DrawingSnapshot as usize;
        if self.identity.is_some_and(|previous|previous!=identity) {return Err(Fault::from("Path simplification source changed"));}
        self.identity=Some(identity);
        if crate::schema::drawing_layer_is_locked(input.snapshot,&payload.layer_id) {return Err(Fault::from("Unlock the path before editing"));}
        let Some(DrawingLayerNode::Path(path))=crate::schema::find_drawing_layer(input.snapshot,&payload.layer_id) else {return Err(Fault::from("Select a path to edit"));};
        let owner=self.workspace.get_or_insert_with(||ControlledRetirement::new(Workspace {source:Default::default(),job:None}).unwrap_or_else(|_|unreachable!()));
        let workspace=owner.original_mut().ok_or_else(||Fault::from("drawing-path-retirement-started"))?;
        while !cx.should_yield() {
            if cx.is_cancelled() {return Err(Fault::from("Path simplification cancelled"));}
            if workspace.job.is_none() {
                if workspace.source.len()<path.segments.len() {
                    cx.set_stage("path-copy");
                    workspace.source.try_push(path.segments.get(workspace.source.len()).unwrap().clone()).map_err(|error|Fault::from(error.to_string()))?;
                    cx.consume_fuel(1);continue;
                }
                workspace.job=Some(PathSimplifyJob::new_paged(std::mem::take(&mut workspace.source),*tolerance).map_err(Fault::from)?);
                cx.consume_fuel(1);continue;
            }
            let job=workspace.job.as_mut().unwrap();
            let progress=job.advance(1).map_err(Fault::from)?;
            cx.set_stage(match progress.phase {crate::schema::geometry::editing::simplify::PathSimplifyPhase::Scanning=>"path-scan",crate::schema::geometry::editing::simplify::PathSimplifyPhase::Reducing=>"path-simplify",crate::schema::geometry::editing::simplify::PathSimplifyPhase::Building=>"path-build",crate::schema::geometry::editing::simplify::PathSimplifyPhase::Complete=>"path-publish"});
            cx.consume_fuel(1);
            if !progress.done {continue;}
            self.complete=true;
            if job.result().map_err(Fault::from)?.len()==path.segments.len() {return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));}
            let segments=job.take_result().map_err(Fault::from)?;
            let mut emit=Emit::mutations(vec![crate::mutations::update_path_geometry(path.base.id.clone(),segments)]);
            if let Some(selection)=input.interaction.selection.get(DRAWING_POINT_DOMAIN) {
                let retained=selection.ids.iter().filter(|id|crate::editor::drawing::interaction::points::parse_point_id(id).is_some_and(|point|point.layer_id!=payload.layer_id)).cloned().collect::<Vec<_>>();
                emit.effects.push(crate::editor::drawing::commands::canvas_pointer_down::point_selection_effect(&retained));
            }
            return Ok(ArtifactCommandWorkStep::Complete(emit));
        }
        Ok(ArtifactCommandWorkStep::Progress {stage:"path-simplify",preview:b""})
    }
    fn begin_close(&mut self) {self.closing=true;}
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        if !self.closing {return InteractiveJobCloseStep::Blocked;}
        let Some(owner)=self.workspace.as_mut() else {return InteractiveJobCloseStep::Complete {progress:Default::default()};};
        match owner.step(grant) {Ok(RetainedCloneStep::Complete(progress))=>InteractiveJobCloseStep::Complete {progress},Ok(RetainedCloneStep::Progress(progress))=>InteractiveJobCloseStep::Pending {progress},Err(error)=>InteractiveJobCloseStep::Refused{kind:error.kind,progress:error.retained_progress()}}
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError> {self.workspace.as_ref().map(|owner|owner.next_copy_byte_demand()).unwrap_or(Ok(0))}
    fn next_close_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError> {self.workspace.as_ref().map(|owner|owner.next_capacity_byte_demand(copy)).unwrap_or(Ok(0))}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError> {self.workspace.as_ref().map(|owner|owner.next_release_byte_demand()).unwrap_or(Ok(0))}
    fn next_close_depth_demand(&self)->Result<usize,ValueError> {self.workspace.as_ref().map(|owner|owner.next_depth_demand()).unwrap_or(Ok(0))}
    fn terminal_is_empty(&self)->bool {self.closing&&self.workspace.as_ref().is_none_or(|owner|owner.terminal_is_empty())}
}
pub(super) struct DrawingPathCommandJobFactory {keys:Vec<semio_framework::ToolFactoryKey>}
impl DrawingPathCommandJobFactory {pub(super) fn new(controller:&str)->Self {Self {keys:vec![semio_framework::ToolFactoryKey::new(controller,"editPath")]}}}
impl semio_framework::ToolJobFactory for DrawingPathCommandJobFactory {
    type Payload=semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<App>;
    type Job=semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<App>;
    fn keys(&self)->&[semio_framework::ToolFactoryKey] {&self.keys}
    fn payload_schema_id(&self)->&str {DRAWING_BOUNDED_PAYLOAD_SCHEMA}
    fn classification(&self)->semio_framework::InteractiveJobClassification {semio_framework::InteractiveJobClassification::Migrated}
    fn execution_contract(&self)->semio_framework::ToolExecutionContract {semio_framework::ToolExecutionContract::resumable(65536,4096,1,262144,7500,1,1)}
    fn create_job(&mut self,_operation:semio_framework_job::Operation,payload:Self::Payload)->Result<Self::Job,semio_framework::ToolJobFactoryError> {Ok(Self::Job::new(payload))}
    fn create_job_from_wire_pages_with_payload(&mut self,_operation:semio_framework_job::Operation,payload:Self::Payload,input:semio_framework::action_bus::RetainedToolWireInput,checkpoint:Option<semio_framework::action_bus::RetainedToolWireInput>)->Result<Self::Job,(semio_framework::ToolJobFactoryError,semio_framework::action_bus::RetainedToolWireInput,Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some()||input.declared_bytes()>65536 {return Err((semio_framework::ToolJobFactoryError::new("Path editing ingress rejects oversized wire or checkpoint"),input,checkpoint));}
        Ok(Self::Job::from_wire(payload,input))
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for DrawingPathCommandJobFactory {
    type Owner=App;
    const TOOL_IDS:&'static[&'static str]=DRAWING_PATH_TOOL_IDS;
    const DOCUMENT_SCHEMA:&'static str=DRAWING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS:&'static[semio_framework_plugin::ArtifactToolPublicationContract]=DRAWING_PATH_PUBLICATION_CONTRACTS;
}
pub(super) fn build(request:semio_framework_plugin::ArtifactOwnedToolJobRequest<App>)->Result<semio_framework::ToolOperationSpec,Fault> {
    let work=Work::new();
    if request.tool_id!="editPath"||work.extent(&request.command,&request.snapshot,&request.interaction_state,Some(&request.context))!=Some(1) {return Err(Fault::from("Path editing exceeds its registered source capacity"));}
    let operation=semio_framework_plugin::AppOperationContext {app_instance_id:request.app_instance_id,parent_document_id:request.parent_document_id.clone(),operation_id:request.operation.operation.0,generation:request.operation.generation.0,canonical_base_revision:request.canonical_base_revision,retained: request.retained,authoring_seed:request.authoring_seed.clone()};
    let payload=semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {command:*request.command,snapshot:request.snapshot,config:request.config,history:request.history,interaction_state:request.interaction_state,interaction_hover:request.interaction_hover,context:Some(request.context),operation,completion:request.completion},DrawingCommand::command_id,65536,4096,Box::new(work));
    Ok(semio_framework::ToolOperationSpec::new(request.controller_id,request.tool_id,request.payload_schema_id,payload,request.operation))
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
