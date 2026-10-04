//! 🎭️ Publish a selection as an image-backed layer mask without changing source pixels.
use crate::editor::raster::{RasterCommand, RasterPlayApp};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::standards::v1::subsets::any::schema::{find_layer, flatten_raster_layers, layer_node_id};
use crate::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterMutation, RasterSnapshot, RasterTransform};
use semio_framework_pixels::{editing::validate_extent, png_encoding::{EncodedPngImage, PngEncodeJob}, RasterImage};
use semio_framework_plugin::{ArtifactView, ConfigView, EditorApp, Emit, Fault};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "mask-from-selection")]
pub struct MaskFromSelection {
    pub layer_id: String,
    pub expected_image_key: Option<String>,
    pub selection: String,
}

struct Candidate {
    layer_id: String,
    expected_image_key: Option<String>,
    expected_mask: Option<RasterLayerMask>,
    image: RasterImage,
    spans: Vec<(usize,usize,u8)>,
    cursor: usize,
    span: usize,
}

impl Candidate {
    fn advance(&mut self, maximum: usize) -> bool {
        let count = self.image.width as usize * self.image.height as usize;
        let end = self.cursor.saturating_add(maximum).min(count);
        while self.cursor < end {
            while self.span < self.spans.len() && self.cursor >= self.spans[self.span].1 { self.span += 1; }
            let coverage = self.spans.get(self.span).filter(|span| self.cursor >= span.0).map_or(0, |span| span.2);
            self.image.pixels[self.cursor*4..self.cursor*4+4].copy_from_slice(&[255,255,255,coverage]);
            self.cursor += 1;
        }
        self.cursor == count
    }
}

fn prepare(command: &MaskFromSelection, document: &RasterSnapshot) -> Result<Candidate, Fault> {
    crate::standards::v1::subsets::any::schema::require_layer_edit(&document.layers,&command.layer_id,false).map_err(Fault::from)?;
    let Some(RasterLayerNode::Pixel { image_key, mask, width, height, .. }) = find_layer(&document.layers, &command.layer_id) else { return Err(Fault::from("raster.mask-requires-pixel-layer")); };
    if image_key != &command.expected_image_key { return Err(Fault::from("raster.mask-image-conflict")); }
    let (width,height) = if let Some(key) = image_key {
        let image = document.assets.get(key).and_then(|asset| asset.local_owner::<crate::SemioImageSnapshot>()).ok_or_else(|| Fault::from("raster.mask-image-unavailable"))?;
        (image.width,image.height)
    } else { (width.unwrap_or(512),height.unwrap_or(512)) };
    validate_extent(width,height).map_err(|_| Fault::from("raster.mask-extent-invalid"))?;
    let spans=crate::editor::raster::selection::selection_spans(&command.selection,width as usize*height as usize)?;
    Ok(Candidate { layer_id:command.layer_id.clone(),expected_image_key:image_key.clone(),expected_mask:mask.clone(),image:RasterImage::new(width,height),spans,cursor:0,span:0 })
}

fn publish(image: EncodedPngImage, candidate: &Candidate, document: &RasterSnapshot) -> Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {
    use crate::mutations::{add_layer_asset,change_layer_mask,remove_layer_asset};
    let Some(RasterLayerNode::Pixel { image_key,mask,width,height,.. })=find_layer(&document.layers,&candidate.layer_id) else { return Err(Fault::from("raster.mask-layer-missing")); };
    if image_key!=&candidate.expected_image_key || mask!=&candidate.expected_mask { return Err(Fault::from("raster.mask-revision-conflict")); }
    let width=width.unwrap_or(image.width);let height=height.unwrap_or(image.height);
    validate_extent(width,height).map_err(|_| Fault::from("raster.mask-extent-invalid"))?;
    let key=format!("mask-{}-{:016x}",candidate.layer_id,image.content_hash);
    let replacement=RasterLayerMask {enabled:true,linked:true,invert:false,width:Some(width),height:Some(height),image_key:Some(key.clone()),transform:RasterTransform::default()};
    if mask.as_ref()==Some(&replacement) { return Ok(Emit::default()); }
    let mut operations=Vec::new();
    if !document.assets.contains_key(&key) {
        operations.push(RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:key.clone(),asset:RasterImageAsset {mime:"image/png".into(),data:image.data}}));
    }
    operations.push(RasterMutation::ChangeLayerMask(change_layer_mask::ChangeLayerMask {layer_id:candidate.layer_id.clone(),expected:mask.clone(),mask:Some(replacement)}));
    if let Some(previous)=mask.as_ref().and_then(|mask|mask.image_key.as_ref()).filter(|previous|*previous!=&key) {
        let referenced=flatten_raster_layers(&document.layers).iter().any(|layer|match layer {
            RasterLayerNode::Pixel {image_key,mask,..} => image_key.as_ref()==Some(previous) || (layer_node_id(layer)!=candidate.layer_id && mask.as_ref().and_then(|mask|mask.image_key.as_ref())==Some(previous)),
            RasterLayerNode::Group {mask,..} => mask.as_ref().and_then(|mask|mask.image_key.as_ref())==Some(previous),
            _=>false,
        });
        if !referenced && document.assets.contains_key(previous) { operations.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset {asset_id:previous.clone()})); }
    }
    Ok(Emit::mutations(operations))
}

pub fn handle(_payload:&MaskFromSelection,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>) -> Result<Emit<RasterMutation,RasterConfigMutation>,Fault> {
    Err(Fault::from("raster.mask-requires-retained-work"))
}

#[derive(Default)]
pub struct MaskFromSelectionWork {
    candidate:Option<Candidate>,
    encoding:Option<PngEncodeJob>,
    complete:bool,
}

impl ArtifactCommandWork<EditorApp<RasterPlayApp>> for MaskFromSelectionWork {
    fn tool_id(&self)->&'static str { "maskFromSelection" }
    fn extent(&self,command:&RasterCommand,_snapshot:&RasterSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>) -> Option<usize> {
        matches!(command,RasterCommand::MaskFromSelection(_)).then_some(1)
    }
    fn step(&mut self,input:&ArtifactCommandInputs<'_,EditorApp<RasterPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<RasterPlayApp>>,Fault> {
        if self.complete { return Err(Fault::from("raster.mask-work-complete")); }
        let RasterCommand::MaskFromSelection(command)=input.command else { return Err(Fault::from("raster.mask-work-mismatch")); };
        if let Some(encoder)=self.encoding.as_mut() {
            if !encoder.advance().map_err(|_|Fault::from("raster.mask-encoding-failed"))?.done { return Ok(ArtifactCommandWorkStep::Progress {stage:"mask-encode",preview:br#"{"en":"Encoding mask","de":"Maske wird kodiert"}"#}); }
            let encoded=self.encoding.take().unwrap().into_result().map_err(|_|Fault::from("raster.mask-encoding-failed"))?;
            let candidate=self.candidate.take().unwrap();
            self.complete=true;
            return publish(encoded,&candidate,input.snapshot).map(ArtifactCommandWorkStep::Complete);
        }
        if self.candidate.is_none() { self.candidate=Some(prepare(command,input.snapshot)?); }
        let candidate=self.candidate.as_mut().unwrap();
        if candidate.advance(32768) {
            let image=std::mem::replace(&mut candidate.image,RasterImage {width:0,height:0,pixels:Vec::new()});
            self.encoding=Some(PngEncodeJob::new(image).map_err(|_|Fault::from("raster.mask-encoding-failed"))?);
        }
        Ok(ArtifactCommandWorkStep::Progress {stage:"mask-create",preview:br#"{"en":"Creating selection mask","de":"Auswahlmaske wird erstellt"}"#})
    }
    fn begin_close(&mut self) { if let Some(encoder)=self.encoding.as_mut() {encoder.cancel();} }
    fn close_step(&mut self,maximum_items:usize,_maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep {
        if maximum_items==0 {return semio_framework_job::InteractiveJobCloseStep::Blocked;}
        self.candidate=None;self.encoding=None;semio_framework_job::InteractiveJobCloseStep::Complete
    }
    fn terminal_is_empty(&self)->bool {self.candidate.is_none()&&self.encoding.is_none()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
