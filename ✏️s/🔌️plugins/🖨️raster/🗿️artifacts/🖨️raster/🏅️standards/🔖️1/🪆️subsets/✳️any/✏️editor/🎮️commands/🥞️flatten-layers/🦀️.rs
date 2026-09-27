//! 🥞️ Render visible artwork privately, then replace the layer tree in one undoable operation.
use crate::editor::raster::{RasterCommand,RasterPlayApp};
use crate::editor::raster::config::{RasterConfig,RasterConfigMutation};
use crate::standards::v1::subsets::any::schema::{create_pixel_layer,layer_node_id};
use crate::{RasterImageAsset,RasterLayerNode,RasterMutation,RasterSnapshot};
use crate::io::RasterStackPreparation;
use semio_framework_pixels::{compositing::layers::RasterStackJob,png_encoding::{EncodedPngImage,PngEncodeJob}};
use semio_framework_plugin::{ArtifactView,ConfigView,EditorApp,Emit,Fault};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs,ArtifactCommandWork,ArtifactCommandWorkStep};
use semio_framework_value_derive::{FromValue,ToValue};

#[derive(Clone,Debug,PartialEq,ToValue,FromValue,dsl::DslRecord)]
#[dsl(keyword="flatten-layers")]
pub struct FlattenLayers {pub name:String}

fn prepare(command:&FlattenLayers,document:&RasterSnapshot)->Result<RasterStackPreparation,Fault> {
    if command.name.trim().is_empty()||command.name.chars().count()>120 {return Err(Fault::from("raster.flatten-name-invalid"));}
    RasterStackPreparation::new(document).map_err(Fault::from)
}
fn publish(image:EncodedPngImage,origin:[f64;2],name:&str,document:&RasterSnapshot)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {
    use crate::mutations::{add_layer_asset,create_layer,delete_layer,remove_layer_asset};
    let (layer,key,asset)=baked_layer(image,origin,name,"flatten");
    let previous=asset_keys_except(&document.layers,&Default::default());
    let mut mutations=vec![RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:key.clone(),asset})];
    mutations.extend(document.layers.iter().map(|layer|RasterMutation::DeleteLayer(delete_layer::DeleteLayer {layer_id:layer_node_id(layer).to_owned()})));
    mutations.push(RasterMutation::CreateLayer(create_layer::CreateLayer {parent_id:None,index:0,layer:Box::new(layer)}));
    mutations.extend(previous.into_iter().filter(|prior|prior!=&key&&document.assets.contains_key(prior)).map(|asset_id|RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset {asset_id})));
    Ok(Emit::mutations(mutations))
}
pub(crate) fn baked_layer(image:EncodedPngImage,origin:[f64;2],name:&str,prefix:&str)->(RasterLayerNode,String,RasterImageAsset) {
    let mut layer=create_pixel_layer(name,image.width,image.height);
    let key=format!("{prefix}-{}-{:016x}",layer_node_id(&layer),image.content_hash);
    if let RasterLayerNode::Pixel {image_key,transform,..}=&mut layer {
        *image_key=Some(key.clone());transform.x=origin[0]+f64::from(image.width)/2.0;transform.y=origin[1]+f64::from(image.height)/2.0;
    }
    (layer,key,RasterImageAsset {mime:"image/png".into(),data:image.data})
}
pub(crate) fn asset_keys_except(layers:&[RasterLayerNode],excluded:&std::collections::BTreeSet<&str>)->std::collections::BTreeSet<String> {
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
pub struct LayerBakeWork<const MERGE_DOWN:bool> {preparing:Option<RasterStackPreparation>,compositing:Option<RasterStackJob>,encoding:Option<PngEncodeJob>,origin:[f64;2],complete:bool}
impl<const MERGE_DOWN:bool> ArtifactCommandWork<EditorApp<RasterPlayApp>> for LayerBakeWork<MERGE_DOWN> {
    fn tool_id(&self)->&'static str {if MERGE_DOWN {"mergeDown"} else {"flattenLayers"}}
    fn extent(&self,command:&RasterCommand,_snapshot:&RasterSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>)->Option<usize> {(matches!((MERGE_DOWN,command),(false,RasterCommand::FlattenLayers(_))|(true,RasterCommand::MergeDown(_)))).then_some(1)}
    fn step(&mut self,input:&ArtifactCommandInputs<'_,EditorApp<RasterPlayApp>>)->Result<ArtifactCommandWorkStep<EditorApp<RasterPlayApp>>,Fault> {
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
    }
    fn begin_close(&mut self) {if let Some(job)=self.preparing.as_mut(){job.cancel();}if let Some(job)=self.compositing.as_mut(){job.cancel();}if let Some(job)=self.encoding.as_mut(){job.cancel();}}
    fn close_step(&mut self,maximum_items:usize,_maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep {
        if maximum_items==0 {return semio_framework_job::InteractiveJobCloseStep::Blocked;}
        self.preparing=None;self.compositing=None;self.encoding=None;semio_framework_job::InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self)->bool {self.preparing.is_none()&&self.compositing.is_none()&&self.encoding.is_none()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
