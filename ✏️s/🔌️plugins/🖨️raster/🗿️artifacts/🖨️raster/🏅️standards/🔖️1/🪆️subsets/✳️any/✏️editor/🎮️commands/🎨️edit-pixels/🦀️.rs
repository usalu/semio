//! 🎨️ Authoritative pixel editing: validate the base revision, compute privately, publish an undoable image.
use crate::editor::raster::{RasterCommand, RasterPlayApp};
use crate::editor::raster::config::{RasterConfig, RasterConfigMutation};
use crate::standards::v1::subsets::any::schema::{find_layer, flatten_raster_layers, layer_node_id, locate_layer};
use crate::{RasterImageAsset, RasterLayerNode, RasterMutation, RasterSnapshot};
use dsl::os_pack::json::Value;
use semio_framework_pixels::{editing::{validate_extent, PixelBrush, PixelEditJob, PixelOperation}, RasterImage};
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
        Some("stroke") => {
            let points = value["points"].as_array().ok_or_else(|| fault("Missing stroke points"))?;
            if !(1..=2048).contains(&points.len()) { return Err(fault("Stroke requires 1–2048 points")); }
            let points = points.iter().map(|point| {
                let point = point.as_array().filter(|point| point.len() == 2).ok_or_else(|| fault("Invalid stroke point"))?;
                Ok([point[0].as_f64().ok_or_else(|| fault("Invalid stroke point"))?, point[1].as_f64().ok_or_else(|| fault("Invalid stroke point"))?])
            }).collect::<Result<Vec<_>, Fault>>()?;
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

fn parse_selection(json: Option<&str>, count: usize) -> Result<Option<Vec<u8>>, Fault> {
    let Some(json) = json else { return Ok(None) };
    let value = dsl::os_pack::json::parse(json).map_err(|_| fault("Invalid selection JSON"))?;
    let spans = value.as_array().ok_or_else(|| fault("Selection must contain spans"))?;
    let mut mask = vec![0; count];
    let mut previous = 0;
    for span in spans {
        let values = span.as_array().ok_or_else(|| fault("Invalid selection span"))?;
        if values.len() != 3 { return Err(fault("Invalid selection span")); }
        let mut triple = [0_usize; 3];
        for (i, item) in values.iter().enumerate() {
            let number = item.as_f64().ok_or_else(|| fault("Invalid selection span"))?;
            if !number.is_finite() || number.fract() != 0.0 || number < 0.0 || number > count.max(255) as f64 { return Err(fault("Invalid selection span")); }
            triple[i] = number as usize;
        }
        let [start, length, coverage] = triple;
        let end = start.checked_add(length).ok_or_else(|| fault("Selection overflow"))?;
        if start < previous || length == 0 || end > count || coverage > 255 { return Err(fault("Selection spans overlap or exceed image")); }
        mask[start..end].fill(coverage as u8);
        previous = end;
    }
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

fn prepare(command: &EditPixels, document: &RasterSnapshot) -> Result<(PixelEditJob, RasterLayerNode, Option<String>, usize), Fault> {
    let layer = find_layer(&document.layers, &command.layer_id).ok_or_else(|| fault("Select a pixel layer"))?;
    let RasterLayerNode::Pixel { width, height, image_key, visible, .. } = layer else { return Err(fault("This layer has no editable pixels")); };
    if !visible || visible_path(&document.layers, &command.layer_id, true) != Some(true) { return Err(fault("Show the layer and its groups before editing its pixels")); }
    if image_key != &command.expected_image_key { return Err(fault("The image changed while this edit was being prepared; retry on the current image")); }
    let image = if let Some(key) = image_key {
        let asset = document.assets.get(key).ok_or_else(|| fault("Layer image is missing"))?;
        let image = asset.local_owner::<crate::SemioImageSnapshot>().ok_or_else(|| fault("Layer image is not available locally"))?;
        let frame = image.frames.first().ok_or_else(|| fault("Layer image has no frame"))?;
        RasterImage { width: image.width, height: image.height, pixels: frame.rgba8.clone() }
    } else {
        let (width, height) = (width.unwrap_or(512), height.unwrap_or(512));
        validate_extent(width, height).map_err(|error| fault(error.to_string()))?;
        RasterImage::new(width, height)
    };
    let operation = parse_operation(&command.operation)?;
    let selection = parse_selection(command.selection.as_deref(), image.pixels.len() / 4)?;
    let mut layer = layer.clone();
    if let RasterLayerNode::Pixel { width, height, transform, .. } = &mut layer {
        transform.scale_x *= f64::from(width.unwrap_or(image.width)) / f64::from(image.width);
        transform.scale_y *= f64::from(height.unwrap_or(image.height)) / f64::from(image.height);
        if let PixelOperation::Crop { x, y, width, height } = &operation {
            let dx = (f64::from(*x) + f64::from(*width) / 2.0 - f64::from(image.width) / 2.0) * transform.scale_x;
            let dy = (f64::from(*y) + f64::from(*height) / 2.0 - f64::from(image.height) / 2.0) * transform.scale_y;
            let (sin, cos) = transform.rotation.to_radians().sin_cos();
            transform.x += cos * dx - sin * dy;
            transform.y += sin * dx + cos * dy;
        }
    }
    let job = PixelEditJob::new(image, operation, selection).map_err(|error| fault(error.to_string()))?;
    let (parent, index) = locate_layer(&document.layers, &command.layer_id).ok_or_else(|| fault("Layer tree address is missing"))?;
    Ok((job, layer, parent, index))
}

fn publish(image: EncodedPngImage, layer: RasterLayerNode, _parent_id: Option<String>, _index: usize, document: &RasterSnapshot) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    use crate::mutations::{add_layer_asset, change_layer_pixels, remove_layer_asset};
    let layer_id = layer_node_id(&layer).to_string();
    let RasterLayerNode::Pixel { image_key: previous_key, transform, .. } = layer else { return Err(fault("This layer has no editable pixels")); };
    let Some(RasterLayerNode::Pixel { width, height, image_key, .. }) = find_layer(&document.layers, &layer_id) else { return Err(fault("The edited layer was removed")); };
    if image_key != &previous_key { return Err(fault("The layer image changed during editing")); }
    let original_size = previous_key.as_ref().and_then(|key| document.assets.get(key)).and_then(|child| child.local_owner::<crate::SemioImageSnapshot>()).map(|image| (image.width, image.height)).unwrap_or((width.unwrap_or(512), height.unwrap_or(512)));
    let resized = original_size != (image.width, image.height);
    let key = format!("pixels-{layer_id}-{:016x}", image.content_hash);
    if previous_key.as_ref() == Some(&key) { return Ok(Emit::default()); }
    let mut operations = Vec::new();
    if !document.assets.contains_key(&key) {
        operations.push(RasterMutation::AddLayerAsset(add_layer_asset::AddLayerAsset { asset_id: key.clone(), asset: RasterImageAsset { mime: "image/png".into(), data: image.data } }));
    }
    operations.push(RasterMutation::ChangeLayerPixels(change_layer_pixels::ChangeLayerPixels {
        layer_id: layer_id.clone(), expected_image_key: previous_key.clone(),
        content: crate::RasterPixelContent { image_key: Some(key), width: if resized { Some(image.width) } else { *width }, height: if resized { Some(image.height) } else { *height } },
        transform: resized.then_some(transform),
    }));
    if let Some(previous) = previous_key {
        let shared = flatten_raster_layers(&document.layers).iter().any(|node| {
            if layer_node_id(node) == layer_id { return false; }
            match node {
                RasterLayerNode::Pixel { image_key, .. } => image_key.as_deref() == Some(previous.as_str()),
                _ => false,
            }
        });
        if !shared && document.assets.contains_key(&previous) {
            operations.push(RasterMutation::RemoveLayerAsset(remove_layer_asset::RemoveLayerAsset { asset_id: previous }));
        }
    }
    Ok(Emit::mutations(operations))
}

pub fn handle(_payload: &EditPixels, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
    Err(fault("Pixel editing requires the cancellable retained command route"))
}

#[derive(Default)]
pub struct PixelEditWork {
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
            self.prepared = Some(prepare(command, input.snapshot)?);
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
        self.prepared = None;
        self.encoding = None;
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool { self.prepared.is_none() && self.encoding.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
