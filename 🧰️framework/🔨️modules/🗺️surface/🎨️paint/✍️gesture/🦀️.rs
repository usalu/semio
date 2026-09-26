//! 🖌️ Bounded stroke intent in intrinsic pixel coordinates; publication belongs to the editor.
use super::{Affine, LayerNode, Point, RasterHost, Vec2};

#[derive(Clone, Debug, PartialEq)]
pub struct PixelStrokeCommand {
    pub layer_id: String,
    pub expected_image_key: Option<String>,
    pub operation: String,
}

pub(super) struct PixelGesture {
    pub command: PixelStrokeCommand,
    pub world: Affine,
    pub inverse: Affine,
    pub points: Vec<[f64; 2]>,
}

fn target(layers: &[LayerNode], id: &str, parent: Affine) -> Option<(Affine, u32, u32, Option<String>)> {
    for layer in layers {
        match layer {
            LayerNode::Pixel { id: layer_id, visible: true, transform, width, height, image_key, .. } if layer_id == id => {
                let world = parent * *transform * Affine::IDENTITY.translate(Vec2::new(-f64::from(*width) / 2.0, -f64::from(*height) / 2.0));
                return Some((world, *width, *height, image_key.clone()));
            }
            LayerNode::Group { visible: true, transform, children, .. } => {
                if let Some(value) = target(children, id, parent * *transform) { return Some(value); }
            }
            _ => {}
        }
    }
    None
}

impl PixelGesture {
    pub fn begin(host: &RasterHost, point: Point) -> Option<Self> {
        let id = host.selected_ids.first()?;
        let (world, width, height, image_key) = target(&host.document.layers, id, Affine::IDENTITY)?;
        let extent = image_key.as_ref().and_then(|key| host.images.get(key)).map(|image| (image.width(), image.height())).unwrap_or((width, height));
        let matrix = world * Affine::new([f64::from(width) / f64::from(extent.0.max(1)), 0.0, 0.0, f64::from(height) / f64::from(extent.1.max(1)), 0.0, 0.0]);
        let [a, b, c, d, e, f] = matrix.as_coeffs();
        let determinant = a * d - b * c;
        if !determinant.is_finite() || determinant.abs() < 1e-12 { return None; }
        let inverse = Affine::new([d / determinant, -b / determinant, -c / determinant, a / determinant, (c * f - d * e) / determinant, (b * e - a * f) / determinant]);
        let mut gesture = Self { command: PixelStrokeCommand { layer_id: id.clone(), expected_image_key: image_key, operation: String::new() }, world: matrix, inverse, points: Vec::with_capacity(2048) };
        gesture.push(point);
        Some(gesture)
    }

    pub fn push(&mut self, world: Point) {
        let local = self.inverse * world;
        if !local.x.is_finite() || !local.y.is_finite() || local.x.abs() > 1_000_000.0 || local.y.abs() > 1_000_000.0 || self.points.len() >= 2048 { return; }
        let point = [(local.x * 1000.0).round() / 1000.0, (local.y * 1000.0).round() / 1000.0];
        if self.points.last() != Some(&point) { self.points.push(point); }
    }

    pub fn finish(mut self, size: f32, opacity: f32, color: [u8; 4], hardness: f32, erase: bool) -> Option<PixelStrokeCommand> {
        if self.points.is_empty() { return None; }
        self.command.operation = serde_json::json!({"kind":"stroke", "points":self.points, "size":size, "opacity":opacity, "color":color, "hardness":hardness, "erase":erase}).to_string();
        Some(self.command)
    }

    pub fn close_step(&mut self) -> bool {
        self.points.pop().is_none() && self.command.close_step()
    }
}

impl PixelStrokeCommand {
    pub fn close_step(&mut self) -> bool {
        if self.layer_id.pop().is_some() || self.operation.pop().is_some() || self.expected_image_key.as_mut().is_some_and(|key| key.pop().is_some()) { return false; }
        self.expected_image_key = None;
        true
    }
}
