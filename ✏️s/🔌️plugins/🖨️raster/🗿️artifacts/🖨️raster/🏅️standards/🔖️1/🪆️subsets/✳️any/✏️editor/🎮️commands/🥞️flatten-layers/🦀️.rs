//! 🥞️ Original retained layer baking publishes direct raw pixels through one funded private owner.
use crate::RasterCommand;
use crate::commands::work_app::RasterWorkApp;
use crate::config::{RasterConfig,RasterConfigMutation};
use semio_s_artifact_raster_raster_model::{RasterLayerNode,layer_node_id};
use semio_s_artifact_raster_raster_model::SemioImageSnapshot;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs,ArtifactCommandWork,ArtifactCommandWorkStep};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,retained_clone::RetainedCloneClose};
use semio_framework_value_derive::{FromValue,ToValue};
use std::mem::ManuallyDrop;
const BAKE_PREPARE:&[u8]=br#"{"en":"Preparing layers","de":"Ebenen werden vorbereitet"}"#;
const BAKE_RENDER:&[u8]=br#"{"en":"Baking layers","de":"Ebenen werden gerendert"}"#;

#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned)]
#[dsl(keyword="flatten-layers")]
pub struct FlattenLayers{pub name:String}
pub fn handle(_payload:&FlattenLayers,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault>{Err(Fault::from("raster.flatten-requires-retained-work"))}

#[path="🧵️work/🦀️.rs"]mod work;
pub use work::LayerBakeOwner;

pub struct LayerBakeWork<const MERGE:bool>{owner:ManuallyDrop<Option<LayerBakeOwner<MERGE>>>,pending:ManuallyDrop<Option<Emit<RasterMutation,RasterConfigMutation>>>,retirement:RetainedCloneClose,closing:bool}
impl<const MERGE:bool>Default for LayerBakeWork<MERGE>{fn default()->Self{Self{owner:ManuallyDrop::new(None),pending:ManuallyDrop::new(None),retirement:Default::default(),closing:false}}}
impl<const MERGE:bool>LayerBakeWork<MERGE>{
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.owner.as_ref(){return owner.close_demands(body)}self.retirement.next_owner_demands::<Emit<RasterMutation,RasterConfigMutation>>(self.pending.is_some(),&None,body)}
    fn close_owned(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{if let Some(owner)=self.owner.as_mut(){let turn=owner.close_step(grant);if owner.terminal_is_empty(){self.owner.take();}return turn}if !self.retirement.is_empty(){return self.retirement.step_granted(grant)}let turn=self.retirement.begin_granted(&mut self.pending,grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),work:turn.work}}
    pub fn terminal_is_empty(&self)->bool{self.owner.is_none()&&self.pending.is_none()&&self.retirement.is_empty()}
}
impl<A:RasterWorkApp,const MERGE:bool>ArtifactCommandWork<A>for LayerBakeWork<MERGE>{
    fn tool_id(&self)->&'static str{if MERGE{"mergeDown"}else{"flattenLayers"}}
    fn extent(&self,command:&RasterCommand,_snapshot:&RasterSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<A>>)->Option<usize>{matches!((MERGE,command),(false,RasterCommand::FlattenLayers(_))|(true,RasterCommand::MergeDown(_))).then_some(1)}
    fn work_demands(&self,_input:&ArtifactCommandInputs<'_,A>,body:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.owner.as_ref(){if !owner.output_ready(){return owner.demands(body.clamp(1,32768))}}Ok(RetirementDemand{depth:1,..Default::default()})}
    fn step(&mut self,input:&ArtifactCommandInputs<'_,A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<A>,semio_framework_job::OriginalTurnFailure>{
        if self.closing||cx.is_cancelled(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"raster bake is closing").into())}
        if self.owner.is_none(){if cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(ArtifactCommandWorkStep::Progress{stage:"bake-prepare",preview:BAKE_PREPARE})}*self.owner=Some(LayerBakeOwner::new(input.command,input.snapshot)?);cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);return Ok(ArtifactCommandWorkStep::Progress{stage:"bake-prepare",preview:BAKE_PREPARE})}
        let owner=self.owner.as_mut().unwrap();
        if owner.output_ready(){let turn=owner.take_emit(cx.retained_grant());let work=turn.work;let result=turn.result.map(|output|if let Some((emit,progress))=output{*self.pending=Some(emit);RetainedCloneStep::Progress(progress)}else{RetainedCloneStep::Progress(Default::default())});semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work})?;if let Some(emit)=self.pending.take(){return Ok(ArtifactCommandWorkStep::Complete(emit))}}
        else{let body=cx.retained_grant().maximum_copy_bytes.min(32768).max(1);let turn=owner.advance(input.command,input.snapshot,body,cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;}
        Ok(ArtifactCommandWorkStep::Progress{stage:"bake-layers",preview:BAKE_RENDER})
    }
    fn begin_close(&mut self){self.closing=true;if let Some(owner)=self.owner.as_mut(){owner.cancel();}}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}let demand=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_owned(grant))}
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.depth)}
    fn terminal_is_empty(&self)->bool{LayerBakeWork::terminal_is_empty(self)}
}
impl<const MERGE:bool>Drop for LayerBakeWork<MERGE>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"layer bake work retains original private custody until funded close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owner);ManuallyDrop::drop(&mut self.pending)}}}}
