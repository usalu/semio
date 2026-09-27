//! 🖌️ Prepare mask pixels in bounded grants and publish one reversible coverage edit.
use crate::editor::raster::{RasterCommand,RasterPlayApp};
use crate::editor::raster::config::{RasterConfig,RasterConfigMutation};
use crate::standards::v1::subsets::any::schema::{find_layer,flatten_raster_layers,layer_node_id};
use crate::{RasterImageAsset,RasterLayerMask,RasterLayerNode,RasterMutation,RasterSnapshot};
use semio_framework_pixels::{RasterImage,editing::{validate_extent,PixelEditJob,PixelOperation},png_encoding::{EncodedPngImage,PngEncodeJob}};
use semio_framework_plugin::{ArtifactView,ConfigView,EditorApp,Emit,Fault};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs,ArtifactCommandWork,ArtifactCommandWorkStep};
use semio_framework_value_derive::{FromValue,ToValue};

#[derive(Clone,Debug,PartialEq,ToValue,FromValue,dsl::DslRecord)]
#[dsl(keyword="edit-mask")]
pub struct EditMask {
    pub layer_id:String,
    pub expected_mask:String,
    pub operation:String,
    pub selection:Option<String>,
}

fn layer_mask(layer:&RasterLayerNode)->Option<&RasterLayerMask> {
    match layer {RasterLayerNode::Pixel {mask,..}|RasterLayerNode::Group {mask,..}=>mask.as_ref(),_=>None}
}

struct Candidate {
    layer_id:String,
    mask:RasterLayerMask,
    image:RasterImage,
    operation:Option<PixelOperation>,
    selection:Option<Vec<u8>>,
    spans:Vec<(usize,usize,u8)>,
    span:usize,
}
impl Candidate {
    fn advance(&mut self,document:&RasterSnapshot,maximum:usize)->Result<bool,Fault> {
        let start=self.image.pixels.len()/4;
        let total=self.image.width as usize*self.image.height as usize;
        let end=start.saturating_add(maximum.min(32768)).min(total);
        if let Some(key)=&self.mask.image_key {
            let image=document.assets.get(key).and_then(|asset|asset.local_owner::<crate::SemioImageSnapshot>()).ok_or_else(||Fault::from("raster.mask-image-unavailable"))?;
            let frame=image.frames.first().ok_or_else(||Fault::from("raster.mask-frame-missing"))?;
            let bytes=frame.rgba8.get(start*4..end*4).ok_or_else(||Fault::from("raster.mask-pixels-invalid"))?;
            for pixel in bytes.chunks_exact(4) {
                let coverage=semio_framework_pixels::compositing::layers::mask_coverage([pixel[0],pixel[1],pixel[2],pixel[3]]);
                self.image.pixels.extend_from_slice(&[255,255,255,coverage]);
            }
        } else {self.image.pixels.resize(end*4,255);}
        if let Some(selection)=self.selection.as_mut() {
            for index in start..end {
                while self.span<self.spans.len()&&index>=self.spans[self.span].1 {self.span+=1;}
                selection.push(self.spans.get(self.span).filter(|span|index>=span.0).map_or(0,|span|span.2));
            }
        }
        Ok(end==total)
    }
    fn take_job(&mut self)->Result<PixelEditJob,Fault> {
        let image=std::mem::replace(&mut self.image,RasterImage {width:0,height:0,pixels:Vec::new()});
        let operation=self.operation.take().ok_or_else(||Fault::from("raster.mask-operation-missing"))?;
        PixelEditJob::new(image,operation,self.selection.take()).map_err(|_|Fault::from("raster.mask-operation-invalid"))
    }
}

fn prepare(command:&EditMask,document:&RasterSnapshot)->Result<Candidate,Fault> {
    let layer=find_layer(&document.layers,&command.layer_id).ok_or_else(||Fault::from("raster.mask-layer-missing"))?;
    let mask=layer_mask(layer).ok_or_else(||Fault::from("raster.mask-missing"))?;
    if command.expected_mask.len()>8000 {return Err(Fault::from("raster.mask-revision-budget"));}
    let expected:RasterLayerMask=dsl::json::from_json_str(&command.expected_mask).map_err(|_|Fault::from("raster.mask-revision-invalid"))?;
    if mask!=&expected {return Err(Fault::from("raster.mask-revision-conflict"));}
    if command.operation.len()>100000 {return Err(Fault::from("raster.mask-operation-budget"));}
    let operation=super::edit_pixels::parse_operation(&command.operation)?;
    if !matches!(operation,PixelOperation::AlphaStroke(_)|PixelOperation::AlphaFill {..}) {return Err(Fault::from("raster.mask-requires-alpha-operation"));}
    let (width,height)=if let Some(key)=&mask.image_key {
        let image=document.assets.get(key).and_then(|asset|asset.local_owner::<crate::SemioImageSnapshot>()).ok_or_else(||Fault::from("raster.mask-image-unavailable"))?;
        (image.width,image.height)
    } else {
        let (width,height)=match layer {RasterLayerNode::Pixel {width,height,..}=>(width.unwrap_or(512),height.unwrap_or(512)),_=>(512,512)};
        (mask.width.unwrap_or(width),mask.height.unwrap_or(height))
    };
    let count=validate_extent(width,height).map_err(|_|Fault::from("raster.mask-extent-invalid"))?;
    let spans=command.selection.as_deref().map(|json|super::edit_pixels::selection_spans(json,count)).transpose()?.unwrap_or_default();
    Ok(Candidate {layer_id:command.layer_id.clone(),mask:mask.clone(),image:RasterImage {width,height,pixels:Vec::with_capacity(count*4)},operation:Some(operation),selection:command.selection.as_ref().map(|_|Vec::with_capacity(count)),spans,span:0})
}

fn publish(image:EncodedPngImage,candidate:&Candidate,document:&RasterSnapshot)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {
    use crate::mutations::{add_layer_asset,change_layer_mask,remove_layer_asset};
    let current=find_layer(&document.layers,&candidate.layer_id).and_then(layer_mask);
    if current!=Some(&candidate.mask) {return Err(Fault::from("raster.mask-revision-conflict"));}
    let key=format!("mask-{}-{:016x}",candidate.layer_id,image.content_hash);
    if candidate.mask.image_key.as_ref()==Some(&key) {return Ok(Emit::default());}
    let mut replacement=candidate.mask.clone();replacement.image_key=Some(key.clone());
    let mut operations=Vec::new();
    if !document.assets.contains_key(&key) {operations.push(RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:key,asset:RasterImageAsset {mime:"image/png".into(),data:image.data}}));}
    operations.push(RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask {layer_id:candidate.layer_id.clone(),expected:Some(candidate.mask.clone()),mask:Some(replacement)}));
    if let Some(previous)=&candidate.mask.image_key {
        let shared=flatten_raster_layers(&document.layers).iter().any(|layer| {
            matches!(layer,RasterLayerNode::Pixel {image_key:Some(key),..} if key==previous)
                || (layer_node_id(layer)!=candidate.layer_id&&layer_mask(layer).and_then(|mask|mask.image_key.as_ref())==Some(previous))
        });
        if !shared {operations.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset {asset_id:previous.clone()}));}
    }
    Ok(Emit::mutations(operations))
}

pub fn handle(_payload:&EditMask,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {Err(Fault::from("raster.mask-requires-retained-work"))}

#[derive(Default)]
pub struct EditMaskWork {candidate:Option<Candidate>,job:Option<PixelEditJob>,encoding:Option<PngEncodeJob>,complete:bool}
impl ArtifactCommandWork<EditorApp<RasterPlayApp>> for EditMaskWork {
    fn tool_id(&self)->&'static str {"editMask"}
    fn extent(&self,command:&RasterCommand,_snapshot:&RasterSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>)->Option<usize> {matches!(command,RasterCommand::EditMask(_)).then_some(1)}
    fn step(&mut self,input:&ArtifactCommandInputs<'_,EditorApp<RasterPlayApp>>)->Result<ArtifactCommandWorkStep<EditorApp<RasterPlayApp>>,Fault> {
        if self.complete {return Err(Fault::from("raster.mask-work-complete"));}
        let RasterCommand::EditMask(command)=input.command else {return Err(Fault::from("raster.mask-work-mismatch"));};
        if let Some(encoder)=self.encoding.as_mut() {
            if !encoder.advance().map_err(|_|Fault::from("raster.mask-encoding-failed"))?.done {return Ok(ArtifactCommandWorkStep::Progress {stage:"mask-encode",preview:br#"{"en":"Encoding mask","de":"Maske wird kodiert"}"#});}
            let image=self.encoding.take().unwrap().into_result().map_err(|_|Fault::from("raster.mask-encoding-failed"))?;
            let candidate=self.candidate.take().unwrap();self.complete=true;
            return publish(image,&candidate,input.snapshot).map(ArtifactCommandWorkStep::Complete);
        }
        if let Some(job)=self.job.as_mut() {
            if job.advance(job.recommended_grant()).map_err(|_|Fault::from("raster.mask-edit-failed"))?.done {
                let image=self.job.take().unwrap().into_result().map_err(|_|Fault::from("raster.mask-edit-failed"))?;
                self.encoding=Some(PngEncodeJob::new(image).map_err(|_|Fault::from("raster.mask-encoding-failed"))?);
            }
            return Ok(ArtifactCommandWorkStep::Progress {stage:"mask-paint",preview:br#"{"en":"Editing mask","de":"Maske wird bearbeitet"}"#});
        }
        if self.candidate.is_none() {self.candidate=Some(prepare(command,input.snapshot)?);}
        let candidate=self.candidate.as_mut().unwrap();
        if candidate.advance(input.snapshot,32768)? {self.job=Some(candidate.take_job()?);}
        Ok(ArtifactCommandWorkStep::Progress {stage:"mask-prepare",preview:br#"{"en":"Preparing mask","de":"Maske wird vorbereitet"}"#})
    }
    fn begin_close(&mut self) {if let Some(job)=self.job.as_mut(){job.cancel();}if let Some(encoder)=self.encoding.as_mut(){encoder.cancel();}}
    fn close_step(&mut self,maximum_items:usize,_maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep {
        if maximum_items==0 {return semio_framework_job::InteractiveJobCloseStep::Blocked;}
        self.candidate=None;self.job=None;self.encoding=None;semio_framework_job::InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self)->bool {self.candidate.is_none()&&self.job.is_none()&&self.encoding.is_none()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
