//! 🥞️ Render visible artwork privately, then replace the layer tree in one undoable operation.
use crate::RasterCommand;
use crate::commands::work_app::RasterWorkApp;
use crate::config::{RasterConfig, RasterConfigMutation};
use semio_s_artifact_raster_raster_model::layer_node_id;
use semio_s_artifact_raster_raster_model::create_pixel_layer;
use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot;
use semio_s_artifact_raster_raster_model::RasterLayerNode;
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_io::RasterStackPreparation;
use semio_framework_pixels::{compositing::layers::RasterStackJob,png_encoding::{EncodedPngImage,PngEncodeJob}};
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs,ArtifactCommandWork,ArtifactCommandWorkStep};
use semio_framework_value_derive::{FromValue,ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword="flatten-layers")]
pub struct FlattenLayers {pub name:String}

pub fn prepare(command:&FlattenLayers,document:&RasterSnapshot)->Result<RasterStackPreparation,Fault> {
    if command.name.trim().is_empty()||command.name.chars().count()>120 {return Err(Fault::from("raster.flatten-name-invalid"));}
    for layer in &document.layers {semio_s_artifact_raster_raster_model::require_layer_edit(&document.layers,layer_node_id(layer),true).map_err(Fault::from)?;}
    RasterStackPreparation::new(document).map_err(Fault::from)
}
pub fn publish(image:EncodedPngImage,origin:[f64;2],name:&str,document:&RasterSnapshot)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {
    use semio_s_artifact_raster_raster_mutations_pixels::{add_layer_asset, remove_layer_asset};
    use semio_s_artifact_raster_raster_mutations_layers::{create_layer, delete_layer};
    let (layer,key,asset)=baked_layer(image,origin,name,"flatten");
    let previous=asset_keys_except(&document.layers,&Default::default());
    let mut mutations=Vec::new();
    mutations.extend(document.layers.iter().map(|layer|RasterMutation::DeleteLayer(delete_layer::DeleteLayer {layer_id:layer_node_id(layer).to_owned()})));
    mutations.extend(previous.into_iter().filter(|prior|prior!=&key&&document.assets.contains_key(prior)).map(|asset_id|RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset {asset_id})));
    mutations.push(RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:key,asset}));
    mutations.push(RasterMutation::CreateLayer(create_layer::CreateLayer {parent_id:None,index:0,layer:Box::new(layer)}));
    Ok(Emit::mutations(mutations))
}
pub fn baked_layer(image:EncodedPngImage,origin:[f64;2],name:&str,prefix:&str)->(RasterLayerNode,String,SemioImageSnapshot) {
    let mut layer=create_pixel_layer(name,image.width,image.height);
    let key=format!("{prefix}-{}-{:016x}",layer_node_id(&layer),image.content_hash);
    if let RasterLayerNode::Pixel {image_key,transform,..}=&mut layer {
        *image_key=Some(key.clone());transform.x=origin[0]+f64::from(image.width)/2.0;transform.y=origin[1]+f64::from(image.height)/2.0;
    }
    (layer,key,semio_s_artifact_raster_raster_io::semio_image_from_png_bytes(&image.data).expect("baked PNG decodes"))
}
pub fn asset_keys_except(layers:&[RasterLayerNode],excluded:&std::collections::BTreeSet<&str>)->std::collections::BTreeSet<String> {
    let mut keys=std::collections::BTreeSet::new();
    for layer in layers {
        if excluded.contains(layer_node_id(layer)) {continue;}
        let mask=match layer {
            RasterLayerNode::Pixel {image_key,mask,..}=>{if let Some(key)=image_key {keys.insert(key.clone());}mask.as_ref()},
            RasterLayerNode::Group {children,mask,..}=>{keys.extend(asset_keys_except(children,excluded));mask.as_ref()},
            _=>None,
        };
        if let Some(key)=mask.and_then(|mask|mask.image_key.as_ref()) {keys.insert(key.clone());}
    }
    keys
}
pub fn handle(_payload:&FlattenLayers,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {Err(Fault::from("raster.flatten-requires-retained-work"))}

#[derive(Default)]
pub struct LayerBakeWork<const MERGE_DOWN:bool> {pub preparing:Option<RasterStackPreparation>,compositing:Option<RasterStackJob>,encoding:Option<PngEncodeJob>,pub origin:[f64;2],complete:bool}
impl<A: RasterWorkApp, const MERGE_DOWN:bool> ArtifactCommandWork<A> for LayerBakeWork<MERGE_DOWN> {
    fn tool_id(&self)->&'static str {if MERGE_DOWN {"mergeDown"} else {"flattenLayers"}}
    fn extent(&self,command:&RasterCommand,_snapshot:&RasterSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<A>>)->Option<usize> {(matches!((MERGE_DOWN,command),(false,RasterCommand::FlattenLayers(_))|(true,RasterCommand::MergeDown(_)))).then_some(1)}
    fn work_demands(&self,_input:&ArtifactCommandInputs<'_,A>,_maximum_copy_bytes:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        let stage=if self.encoding.is_some() {size_of::<PngEncodeJob>()+size_of::<Emit<RasterMutation,RasterConfigMutation>>()} else if self.compositing.is_some() {size_of::<RasterStackJob>()+size_of::<PngEncodeJob>()} else {size_of::<RasterStackPreparation>()+size_of::<RasterStackJob>()};
        Ok(semio_framework_value::RetirementDemand {copy_bytes:stage,depth:1,..Default::default()})
    }
    fn step(&mut self,input:&ArtifactCommandInputs<'_,A>, _cx: &mut semio_framework_job::StepContext<'_>)->Result<ArtifactCommandWorkStep<A>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.complete {return Err(Fault::from("raster.flatten-work-complete"));}
        if !matches!((MERGE_DOWN,input.command),(false,RasterCommand::FlattenLayers(_))|(true,RasterCommand::MergeDown(_))) {return Err(Fault::from("raster.bake-work-mismatch"));}
        if let Some(encoder)=self.encoding.as_mut() {
            if !encoder.advance().map_err(|error|Fault::from(error.to_string()))?.done {return Ok(ArtifactCommandWorkStep::Progress {stage:"flatten-encode",preview:br#"{"en":"Encoding image","de":"Bild wird kodiert"}"#});}
            let image=self.encoding.take().unwrap().into_result().map_err(|error|Fault::from(error.to_string()))?;self.complete=true;
            return match input.command {
                RasterCommand::FlattenLayers(command)=>publish(image,self.origin,&command.name,input.snapshot),
                RasterCommand::MergeDown(command)=>super::merge_down::publish(image,self.origin,command,input.snapshot),
                _=>Err(Fault::from("raster.bake-work-mismatch")),
            }.map(ArtifactCommandWorkStep::Complete);
        }
        if let Some(job)=self.compositing.as_mut() {
            if job.advance(4096).map_err(|error|Fault::from(error.to_string()))?.done {
                let result=self.compositing.take().unwrap().into_result().map_err(|error|Fault::from(error.to_string()))?;
                if result.empty {return Err(Fault::from("raster.flatten-no-visible-pixels"));}
                self.origin=result.origin;self.encoding=Some(PngEncodeJob::new(result.image).map_err(|error|Fault::from(error.to_string()))?);
            }
            return Ok(ArtifactCommandWorkStep::Progress {stage:"flatten-composite",preview:br#"{"en":"Rendering layers","de":"Ebenen werden gerendert"}"#});
        }
        if self.preparing.is_none() {self.preparing=Some(match input.command {
            RasterCommand::FlattenLayers(command)=>prepare(command,input.snapshot),
            RasterCommand::MergeDown(command)=>super::merge_down::prepare(command,input.snapshot),
            _=>Err(Fault::from("raster.bake-work-mismatch")),
        }?);}
        if self.preparing.as_mut().unwrap().advance(input.snapshot,32768).map_err(Fault::from)? {self.compositing=Some(self.preparing.take().unwrap().into_job().map_err(Fault::from)?);}
        Ok(ArtifactCommandWorkStep::Progress {stage:"flatten-prepare",preview:br#"{"en":"Preparing layers","de":"Ebenen werden vorbereitet"}"#})
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
    fn begin_close(&mut self) {if let Some(job)=self.preparing.as_mut(){job.cancel();}if let Some(job)=self.compositing.as_mut(){job.cancel();}if let Some(job)=self.encoding.as_mut(){job.cancel();}}
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            if self.terminal_is_empty() {return Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:Default::default()});}
            if grant.maximum_items==0||grant.maximum_depth==0 {return Ok(semio_framework_job::InteractiveJobCloseStep::Pending {progress:Default::default()});}
            self.preparing=None;self.compositing=None;self.encoding=None;
            Ok(semio_framework_job::InteractiveJobCloseStep::Complete {progress:semio_framework_value::retained_clone::RetainedCloneProgress {copied_items:1,..Default::default()}})
        })
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(0)}
    fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,semio_framework_value::ValueError> {Ok(0)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(0)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError> {Ok(usize::from(!self.terminal_is_empty()))}
    fn terminal_is_empty(&self)->bool {self.preparing.is_none()&&self.compositing.is_none()&&self.encoding.is_none()}
}
