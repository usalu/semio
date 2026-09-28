//! 🎨️ Authoritative pixel editing: validate the base revision, compute privately, publish an undoable image.
use crate::editor::raster::{RasterCommand, RasterPlayApp};
use crate::editor::raster::selection::selection_spans;
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::standards::v1::subsets::any::schema::{find_layer, flatten_raster_layers, layer_node_id, locate_layer};
use crate::{RasterImageAsset, RasterLayerNode, RasterMutation, RasterSnapshot};
use dsl::os_pack::json::Value;
use semio_framework_pixels::{editing::{validate_extent, PixelAlphaBrush, PixelBrush, PixelEditJob, PixelOperation}, RasterImage};
use semio_framework_pixels::png_encoding::{EncodedPngImage, PngEncodeJob};
use semio_framework_plugin::{ArtifactView, ConfigView, EditorApp, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "edit-pixels")]
pub struct EditPixels {
    pub layer_id: String,
    pub expected_image_key: Option<String>,
    pub operation: String,
    pub selection: Option<String>,
}

fn fault(message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("raster.pixel-edit"), message.into())
}

fn integer(value: &Value, key: &str) -> Result<u32, Fault> {
    let number = value[key].as_f64().ok_or_else(|| fault(format!("Missing {key}")))?;
    if !number.is_finite() || number.fract() != 0.0 || !(0.0..=f64::from(u32::MAX)).contains(&number) {
        return Err(fault(format!("Invalid {key}")));
    }
    Ok(number as u32)
}

pub fn parse_operation(json: &str) -> Result<PixelOperation, Fault> {
    let value = dsl::os_pack::json::parse(json).map_err(|_| fault("Invalid pixel operation JSON"))?;
    let amount = || value["value"].as_f64().ok_or_else(|| fault("Missing filter value"));
    Ok(match value["kind"].as_str() {
        Some("invert") => PixelOperation::Invert,
        Some("grayscale") => PixelOperation::Grayscale,
        Some("clear") => PixelOperation::Clear,
        Some("flipHorizontal") => PixelOperation::FlipHorizontal,
        Some("flipVertical") => PixelOperation::FlipVertical,
        Some("rotateClockwise") => PixelOperation::RotateClockwise,
        Some("rotateCounterclockwise") => PixelOperation::RotateCounterclockwise,
        Some("brightness") => PixelOperation::Brightness(amount()?),
        Some("contrast") => PixelOperation::Contrast(amount()?),
        Some("saturation") => PixelOperation::Saturation(amount()?),
        Some("gamma") => PixelOperation::Gamma(amount()?),
        Some("threshold") => PixelOperation::Threshold(amount()?),
        Some("posterize") => PixelOperation::Posterize(u16::try_from(integer(&value, "value")?).map_err(|_| fault("Invalid posterization levels"))?),
        Some("blur") => PixelOperation::Blur(u8::try_from(integer(&value, "value")?).map_err(|_| fault("Invalid blur radius"))?),
        Some("sharpen") => PixelOperation::Sharpen(amount()?),
        Some("crop") => PixelOperation::Crop { x: integer(&value, "x")?, y: integer(&value, "y")?, width: integer(&value, "width")?, height: integer(&value, "height")? },
        Some("resize") => PixelOperation::Resize { width: integer(&value, "width")?, height: integer(&value, "height")?, bilinear: match value["sampling"].as_str() { Some("nearest") => false, Some("bilinear") => true, _ => return Err(fault("Invalid resize sampling")) } },
        Some("stroke" | "alphaStroke") => {
            let points = value["points"].as_array().ok_or_else(|| fault("Missing stroke points"))?;
            if !(1..=2048).contains(&points.len()) { return Err(fault("Stroke requires 1–2048 points")); }
            let points = points.iter().map(|point| {
                let point = point.as_array().filter(|point| point.len() == 2).ok_or_else(|| fault("Invalid stroke point"))?;
                Ok([point[0].as_f64().ok_or_else(|| fault("Invalid stroke point"))?, point[1].as_f64().ok_or_else(|| fault("Invalid stroke point"))?])
            }).collect::<Result<Vec<_>, Fault>>()?;
            if value["kind"].as_str()==Some("alphaStroke") {
                return Ok(PixelOperation::AlphaStroke(PixelAlphaBrush {points,
                    size:value["size"].as_f64().ok_or_else(||fault("Missing brush size"))?,
                    opacity:value["opacity"].as_f64().ok_or_else(||fault("Missing brush opacity"))?,
                    hardness:value["hardness"].as_f64().ok_or_else(||fault("Missing brush hardness"))?,
                    alpha:u8::try_from(integer(&value,"alpha")?).map_err(|_|fault("Invalid brush alpha"))?,
                }));
            }
            let color_value = format!("{{\"kind\":\"fill\",\"color\":{}}}", value["color"]);
            let PixelOperation::Fill(color) = parse_operation(&color_value)? else { return Err(fault("Invalid brush color")); };
            PixelOperation::Stroke(PixelBrush {
                points, color,
                size: value["size"].as_f64().ok_or_else(|| fault("Missing brush size"))?,
                opacity: value["opacity"].as_f64().ok_or_else(|| fault("Missing brush opacity"))?,
                hardness: value["hardness"].as_f64().ok_or_else(|| fault("Missing brush hardness"))?,
                erase: value["erase"].as_bool().ok_or_else(|| fault("Missing eraser flag"))?,
            })
        }
        Some("alphaFill") => PixelOperation::AlphaFill {alpha:u8::try_from(integer(&value,"alpha")?).map_err(|_|fault("Invalid fill alpha"))?,opacity:value["opacity"].as_f64().filter(|value|value.is_finite()&&(0.0..=1.0).contains(value)).ok_or_else(||fault("Invalid fill opacity"))?},
        Some("fill") => {
            let values = value["color"].as_array().ok_or_else(|| fault("Missing fill color"))?;
            if values.len() != 4 { return Err(fault("Color requires four byte channels")); }
            let mut color = [0; 4];
            for (i, item) in values.iter().enumerate() {
                let channel = item.as_f64().ok_or_else(|| fault("Invalid color channel"))?;
                if channel.fract() != 0.0 || !(0.0..=255.0).contains(&channel) { return Err(fault("Invalid color channel")); }
                color[i] = channel as u8;
            }
            PixelOperation::Fill(color)
        }
        _ => return Err(fault("Unknown pixel operation")),
    })
}

#[cfg(test)]
fn parse_selection(json: Option<&str>, count: usize) -> Result<Option<Vec<u8>>, Fault> {
    let Some(json) = json else { return Ok(None) };
    let mut mask=vec![0;count];
    for (start,end,coverage) in selection_spans(json,count)? {mask[start..end].fill(coverage);}
    Ok(Some(mask))
}


fn visible_path(layers: &[RasterLayerNode], id: &str, ancestors_visible: bool) -> Option<bool> {
    for layer in layers {
        let visible = ancestors_visible && crate::standards::v1::subsets::any::schema::layer_visible(layer);
        if layer_node_id(layer) == id { return Some(visible); }
        if let RasterLayerNode::Group { children, .. } = layer {
            if let Some(found) = visible_path(children, id, visible) { return Some(found); }
        }
    }
    None
}

struct PreparingEdit {
    image:RasterImage,
    operation:PixelOperation,
    selection:Option<Vec<u8>>,
    spans:Vec<(usize,usize,u8)>,
    span:usize,
    layer:RasterLayerNode,
    parent:Option<String>,
    index:usize,
}
impl PreparingEdit {
    fn advance(&mut self,document:&RasterSnapshot,maximum:usize)->Result<bool,Fault> {
        let start=self.image.pixels.len()/4;
        let total=self.image.width as usize*self.image.height as usize;
        let end=start.saturating_add(maximum.min(32768)).min(total);
        let RasterLayerNode::Pixel {image_key,..}=&self.layer else {return Err(fault("This layer has no editable pixels"));};
        if let Some(key)=image_key {
            let image=document.assets.get(key).and_then(|asset|asset.local_owner::<crate::SemioImageSnapshot>()).ok_or_else(||fault("Layer image is unavailable"))?;
            let frame=image.frames.first().ok_or_else(||fault("Layer image has no frame"))?;
            if frame.rgba8.len()!=total*4 {return Err(fault("RGBA8 length does not match image extent"));}
            self.image.pixels.extend_from_slice(&frame.rgba8[start*4..end*4]);
        } else {self.image.pixels.resize(end*4,0);}
        if let Some(selection)=self.selection.as_mut() {
            for index in start..end {
                while self.span<self.spans.len()&&index>=self.spans[self.span].1 {self.span+=1;}
                selection.push(self.spans.get(self.span).filter(|span|index>=span.0).map_or(0,|span|span.2));
            }
        }
        Ok(end==total)
    }
    fn into_job(self)->Result<(PixelEditJob,RasterLayerNode,Option<String>,usize),Fault> {
        let job=PixelEditJob::new(self.image,self.operation,self.selection).map_err(|error|fault(error.to_string()))?;
        Ok((job,self.layer,self.parent,self.index))
    }
}

fn prepare(command: &EditPixels, document: &RasterSnapshot) -> Result<PreparingEdit, Fault> {
    crate::standards::v1::subsets::any::schema::require_layer_edit(&document.layers,&command.layer_id,false).map_err(Fault::from)?;
    let layer = find_layer(&document.layers, &command.layer_id).ok_or_else(|| fault("Select a pixel layer"))?;
    let RasterLayerNode::Pixel { width, height, image_key, visible, .. } = layer else { return Err(fault("This layer has no editable pixels")); };
    if !visible || visible_path(&document.layers, &command.layer_id, true) != Some(true) { return Err(fault("Show the layer and its groups before editing its pixels")); }
    if image_key != &command.expected_image_key { return Err(fault("The image changed while this edit was being prepared; retry on the current image")); }
    if command.operation.len()>100000 {return Err(fault("Pixel operation exceeds transport budget"));}
    let (image_width,image_height)=if let Some(key)=image_key {
        let image=document.assets.get(key).and_then(|asset|asset.local_owner::<crate::SemioImageSnapshot>()).ok_or_else(||fault("Layer image is unavailable"))?;
        (image.width,image.height)
    } else {(width.unwrap_or(512),height.unwrap_or(512))};
    let count=validate_extent(image_width,image_height).map_err(|error|fault(error.to_string()))?;
    let image=RasterImage {width:image_width,height:image_height,pixels:Vec::with_capacity(count*4)};
    let operation=parse_operation(&command.operation)?;
    let spans=command.selection.as_deref().map(|json|selection_spans(json,count)).transpose()?.unwrap_or_default();
    let selection=command.selection.as_ref().map(|_|Vec::with_capacity(count));
    let mut layer = layer.clone();
    if let RasterLayerNode::Pixel { width, height, transform, .. } = &mut layer {
        let scale_x=f64::from(width.unwrap_or(image.width))/f64::from(image.width);
        let scale_y=f64::from(height.unwrap_or(image.height))/f64::from(image.height);
        transform.a*=scale_x;transform.b*=scale_x;transform.c*=scale_y;transform.d*=scale_y;
        if let PixelOperation::Crop {x,y,width,height}=&operation {
            let dx=f64::from(*x)+f64::from(*width)/2.0-f64::from(image.width)/2.0;
            let dy=f64::from(*y)+f64::from(*height)/2.0-f64::from(image.height)/2.0;
            transform.x+=transform.a*dx+transform.c*dy;
            transform.y+=transform.b*dx+transform.d*dy;
        }
    }
    let (parent, index) = locate_layer(&document.layers, &command.layer_id).ok_or_else(|| fault("Layer tree address is missing"))?;
    Ok(PreparingEdit {image,operation,selection,spans,span:0,layer,parent,index})
}

fn publish(mut image: EncodedPngImage, layer: RasterLayerNode, _parent_id: Option<String>, _index: usize, document: &RasterSnapshot) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    use crate::mutations::{add_layer_asset, change_layer_pixels, remove_layer_asset};
    let layer_id = layer_node_id(&layer).to_string();
    let RasterLayerNode::Pixel { image_key: previous_key, transform, .. } = layer else { return Err(fault("This layer has no editable pixels")); };
    let Some(RasterLayerNode::Pixel { width, height, image_key, .. }) = find_layer(&document.layers, &layer_id) else { return Err(fault("The edited layer was removed")); };
    if image_key != &previous_key { return Err(fault("The layer image changed during editing")); }
    let original_size = previous_key.as_ref().and_then(|key| document.assets.get(key)).and_then(|child| child.local_owner::<crate::SemioImageSnapshot>()).map(|image| (image.width, image.height)).unwrap_or((width.unwrap_or(512), height.unwrap_or(512)));
    let resized = original_size != (image.width, image.height);
    let key = format!("pixels-{layer_id}-{:016x}", image.content_hash);
    if previous_key.as_ref() == Some(&key) { return Ok(Emit::default()); }
    use crate::editor::raster::asset_replacement::{replacement_steps,Step};
    let removable=previous_key.as_ref().is_some_and(|previous|document.assets.contains_key(previous)&&!flatten_raster_layers(&document.layers).iter().any(|node|match node {
        RasterLayerNode::Pixel {image_key,mask,..}=>(layer_node_id(node)!=layer_id&&image_key.as_ref()==Some(previous))||mask.as_ref().and_then(|mask|mask.image_key.as_ref())==Some(previous),
        RasterLayerNode::Group {mask,..}=>mask.as_ref().and_then(|mask|mask.image_key.as_ref())==Some(previous),
        _=>false,
    }));
    let plan=replacement_steps(document.assets.len(),crate::RASTER_OWNED_MAP_CAPACITY,removable,document.assets.contains_key(&key)).map_err(Fault::from)?;
    let mut expected=previous_key.clone();let mut operations=Vec::new();
    for step in plan {
        match step {
            Step::Detach=>{
                operations.push(RasterMutation::ChangeLayerPixels(change_layer_pixels::ChangeLayerPixels {layer_id:layer_id.clone(),expected_image_key:expected.take(),content:crate::RasterPixelContent {image_key:None,width:*width,height:*height},transform:None}));
            }
            Step::Remove=>operations.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset {asset_id:previous_key.as_ref().unwrap().clone()})),
            Step::Add=>operations.push(RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset {asset_id:key.clone(),asset:RasterImageAsset {mime:"image/png".into(),data:std::mem::take(&mut image.data)}})),
            Step::Replace=>operations.push(RasterMutation::ChangeLayerPixels(change_layer_pixels::ChangeLayerPixels {
                layer_id:layer_id.clone(),expected_image_key:expected.clone(),
                content:crate::RasterPixelContent {image_key:Some(key.clone()),width:if resized {Some(image.width)}else{*width},height:if resized {Some(image.height)}else{*height}},
                transform:resized.then(||transform.clone()),
            })),
        }
    }
    Ok(Emit::mutations(operations))
}

pub fn handle(_payload: &EditPixels, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Err(fault("Pixel editing requires the cancellable retained command route"))
}

#[derive(Default)]
pub struct PixelEditWork {
    preparing: Option<PreparingEdit>,
    prepared: Option<(PixelEditJob, RasterLayerNode, Option<String>, usize)>,
    encoding: Option<(PngEncodeJob, RasterLayerNode, Option<String>, usize)>,
    complete: bool,
}

impl ArtifactCommandWork<EditorApp<RasterPlayApp>> for PixelEditWork {
    fn tool_id(&self) -> &'static str { "editPixels" }

    fn extent(&self, command: &RasterCommand, _snapshot: &RasterSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<RasterPlayApp>>>) -> Option<usize> {
        matches!(command, RasterCommand::EditPixels(_)).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<RasterPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<RasterPlayApp>>, Fault> {
        if self.complete { return Err(fault("Pixel edit already completed")); }
        let RasterCommand::EditPixels(command) = input.command else { return Err(fault("Pixel edit route mismatch")); };
        if let Some((encoder, ..)) = self.encoding.as_mut() {
            if !encoder.advance().map_err(|error| fault(error.to_string()))?.done {
                return Ok(ArtifactCommandWorkStep::Progress { stage: "pixel-edit-encode", preview: br#"{"en":"Encoding image","de":"Bild wird kodiert"}"# });
            }
            let (encoder, layer, parent, index) = self.encoding.take().ok_or_else(|| fault("Pixel edit lost its encoded image"))?;
            self.complete = true;
            return publish(encoder.into_result().map_err(|error| fault(error.to_string()))?, layer, parent, index, input.snapshot).map(ArtifactCommandWorkStep::Complete);
        }
        if self.prepared.is_none() {
            if self.preparing.is_none() {self.preparing=Some(prepare(command,input.snapshot)?);}
            if self.preparing.as_mut().unwrap().advance(input.snapshot,32768)? {
                self.prepared=Some(self.preparing.take().unwrap().into_job()?);
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "pixel-edit-prepare", preview: br#"{"en":"Preparing image","de":"Bild wird vorbereitet"}"# });
        }
        let prepared = self.prepared.as_mut().ok_or_else(|| fault("Pixel edit lost its candidate"))?;
        let grant = prepared.0.recommended_grant();
        if !prepared.0.advance(grant).map_err(|error| fault(error.to_string()))?.done {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "pixel-edit-compute", preview: br#"{"en":"Editing pixels","de":"Pixel werden bearbeitet"}"# });
        }
        let (job, layer, parent, index) = self.prepared.take().ok_or_else(|| fault("Pixel edit lost its candidate"))?;
        let image = job.into_result().map_err(|error| fault(error.to_string()))?;
        self.encoding = Some((PngEncodeJob::new(image).map_err(|error| fault(error.to_string()))?, layer, parent, index));
        Ok(ArtifactCommandWorkStep::Progress { stage: "pixel-edit-encode", preview: br#"{"en":"Encoding image","de":"Bild wird kodiert"}"# })
    }

    fn begin_close(&mut self) {
        if let Some((job, ..)) = self.prepared.as_mut() { job.cancel(); }
        if let Some((job, ..)) = self.encoding.as_mut() { job.cancel(); }
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 { return semio_framework_job::InteractiveJobCloseStep::Blocked; }
        self.preparing = None;
        self.prepared = None;
        self.encoding = None;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool { self.preparing.is_none() && self.prepared.is_none() && self.encoding.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
