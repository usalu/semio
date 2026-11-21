//! 🖌️ Browser replay of logical canvas scene commands through the platform Canvas2D context.

use super::{Affine, BlendMode, Cap, Color, FillRule, Paint, PathEl, RasterImage, RecordedShape, Rgba8, Scene, SceneCommand, Stroke};
use std::collections::HashMap;
use std::sync::{Arc, Weak};
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, CanvasWindingRule, HtmlCanvasElement, ImageData};

struct CanvasImage {
    data: Weak<Vec<u8>>,
    canvas: HtmlCanvasElement,
    used: bool,
}

/// 🖼️ Replays a scene on a single canvas and retains browser image buffers across consecutive frames.
pub struct Canvas2dSession {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    images: HashMap<(*const Vec<u8>, u32, u32), CanvasImage>,
}

impl Canvas2dSession {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Canvas2D context unavailable"))?
            .dyn_into::<CanvasRenderingContext2d>()?;
        Ok(Self { canvas, context, images: HashMap::new() })
    }

    pub fn resize_surface(&self, width: u32, height: u32) {
        if self.canvas.width() != width {
            self.canvas.set_width(width);
        }
        if self.canvas.height() != height {
            self.canvas.set_height(height);
        }
    }

    pub fn render_frame(&mut self, scene: &Scene, clear: Color, dpr: f64) -> Result<(), JsValue> {
        let width = f64::from(self.canvas.width());
        let height = f64::from(self.canvas.height());
        let context = self.context.clone();
        let clear = clear.to_rgba8();
        context.save();
        let result = (|| {
            apply_transform(&context, Affine::IDENTITY)?;
            context.clear_rect(0.0, 0.0, width, height);
            if clear.a > 0 {
                context.set_fill_style_str(&css_color(clear));
                context.fill_rect(0.0, 0.0, width, height);
            }
            self.replay(scene, Affine::IDENTITY.scale(dpr))
        })();
        self.context.restore();
        self.images.retain(|_, image| std::mem::take(&mut image.used));
        result
    }

    fn replay(&mut self, scene: &Scene, base: Affine) -> Result<(), JsValue> {
        let context = self.context.clone();
        let mut depth = 0;
        let mut transform = None;
        let mut fill_color = None;
        let mut stroke_style: Option<(&Stroke, Rgba8)> = None;
        let result = (|| {
            for command in &scene.0 {
                let next_transform = match command {
                    SceneCommand::Fill { transform, .. }
                    | SceneCommand::Stroke { transform, .. }
                    | SceneCommand::DrawImage { transform, .. }
                    | SceneCommand::PushLayer { transform, .. }
                    | SceneCommand::PushClipLayer { transform, .. } => Some(*transform),
                    _ => None,
                };
                if let Some(next) = next_transform.filter(|next| Some(*next) != transform) {
                    apply_transform(&context, base * next)?;
                    transform = Some(next);
                }
                match command {
                    SceneCommand::Fill { rule, paint: Paint::Solid(color), shape, .. } => {
                        let color = color.to_rgba8();
                        if color.a == 0 {
                            continue;
                        }
                        if fill_color != Some(color) {
                            context.set_fill_style_str(&css_color(color));
                            fill_color = Some(color);
                        }
                        if let RecordedShape::Rect(rect) = shape {
                            context.fill_rect(rect.x0(), rect.y0(), rect.width(), rect.height());
                        } else {
                            trace_shape(&context, shape)?;
                            context.fill_with_canvas_winding_rule(winding(*rule));
                        }
                    }
                    SceneCommand::Stroke { stroke, paint: Paint::Solid(color), shape, .. } => {
                        let color = color.to_rgba8();
                        if stroke.width <= 0.0 || color.a == 0 {
                            continue;
                        }
                        if stroke_style.is_none_or(|(previous, _)| previous.width != stroke.width) {
                            context.set_line_width(stroke.width);
                        }
                        if stroke_style.is_none_or(|(previous, _)| previous.start_cap != stroke.start_cap) {
                            context.set_line_cap(match stroke.start_cap {
                                Cap::Butt => "butt",
                                Cap::Round => "round",
                                Cap::Square => "square",
                            });
                        }
                        if stroke_style.is_none() {
                            context.set_line_join("round");
                        }
                        if stroke_style.is_none_or(|(previous, _)| previous.dash_pattern != stroke.dash_pattern) {
                            let dash = js_sys::Array::new();
                            for value in &stroke.dash_pattern {
                                dash.push(&JsValue::from_f64(*value));
                            }
                            context.set_line_dash(&dash)?;
                        }
                        if stroke_style.is_none_or(|(previous, _)| previous.dash_offset != stroke.dash_offset) {
                            context.set_line_dash_offset(stroke.dash_offset);
                        }
                        if stroke_style.is_none_or(|(_, previous)| previous != color) {
                            context.set_stroke_style_str(&css_color(color));
                        }
                        stroke_style = Some((stroke, color));
                        match shape {
                            RecordedShape::Rect(rect) if stroke.dash_pattern.is_empty() && rect.width() > 0.0 && rect.height() > 0.0 => {
                                context.stroke_rect(rect.x0(), rect.y0(), rect.width(), rect.height());
                            }
                            _ => {
                                trace_shape(&context, shape)?;
                                context.stroke();
                            }
                        }
                    }
                    SceneCommand::DrawImage { image, .. } => {
                        let source = self.image_source(image)?;
                        context.draw_image_with_html_canvas_element(source, 0.0, 0.0)?;
                    }
                    SceneCommand::PushClipLayer { rule, clip, .. } | SceneCommand::PushLayer { rule, blend: BlendMode::Normal, alpha: 1.0, clip, .. } => {
                        context.save();
                        depth += 1;
                        trace_shape(&context, clip)?;
                        context.clip_with_canvas_winding_rule(winding(*rule));
                    }
                    SceneCommand::PopLayer if depth > 0 => {
                        context.restore();
                        depth -= 1;
                        transform = None;
                        fill_color = None;
                        stroke_style = None;
                    }
                    SceneCommand::PopLayer => {}
                    SceneCommand::PushLayer { .. } | SceneCommand::VelloFragment { .. } => {
                        return Err(JsValue::from_str("scene requires an unsupported Canvas2D operation"));
                    }
                }
            }
            Ok(())
        })();
        for _ in 0..depth {
            context.restore();
        }
        result
    }

    fn image_source(&mut self, image: &RasterImage) -> Result<&HtmlCanvasElement, JsValue> {
        let key = (Arc::as_ptr(&image.data), image.width(), image.height());
        let entry = match self.images.entry(key) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => {
                let document = self.canvas.owner_document().ok_or_else(|| JsValue::from_str("canvas has no document"))?;
                let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
                canvas.set_width(image.width());
                canvas.set_height(image.height());
                let context = canvas.get_context("2d")?.ok_or_else(|| JsValue::from_str("Canvas2D image context unavailable"))?.dyn_into::<CanvasRenderingContext2d>()?;
                let pixels = ImageData::new_with_u8_clamped_array_and_sh(Clamped(image.data.as_slice()), image.width(), image.height())?;
                context.put_image_data(&pixels, 0.0, 0.0)?;
                entry.insert(CanvasImage { data: Arc::downgrade(&image.data), canvas, used: false })
            }
        };
        debug_assert!(Weak::ptr_eq(&entry.data, &Arc::downgrade(&image.data)));
        entry.used = true;
        Ok(&entry.canvas)
    }
}

fn css_color(rgba: Rgba8) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", rgba.r, rgba.g, rgba.b, rgba.a)
}

fn winding(rule: FillRule) -> CanvasWindingRule {
    match rule {
        FillRule::NonZero => CanvasWindingRule::Nonzero,
        FillRule::EvenOdd => CanvasWindingRule::Evenodd,
    }
}

fn apply_transform(context: &CanvasRenderingContext2d, transform: Affine) -> Result<(), JsValue> {
    let [a, b, c, d, e, f] = transform.as_coeffs();
    context.set_transform(a, b, c, d, e, f)
}

fn trace_shape(context: &CanvasRenderingContext2d, shape: &RecordedShape) -> Result<(), JsValue> {
    context.begin_path();
    match shape {
        RecordedShape::Rect(rect) => context.rect(rect.x0(), rect.y0(), rect.width(), rect.height()),
        RecordedShape::Circle(circle) => {
            let center = circle.center();
            context.arc(center.x, center.y, circle.radius().max(0.0), 0.0, std::f64::consts::TAU)?;
            context.close_path();
        }
        RecordedShape::Line(line) => {
            context.move_to(line.p0().x, line.p0().y);
            context.line_to(line.p1().x, line.p1().y);
        }
        RecordedShape::CubicBez(curve) => {
            context.move_to(curve.p0.x, curve.p0.y);
            context.bezier_curve_to(curve.p1.x, curve.p1.y, curve.p2.x, curve.p2.y, curve.p3.x, curve.p3.y);
        }
        RecordedShape::RoundedRect(rounded) => trace_path(context, &rounded.path_elements(0.1)),
        RecordedShape::Arc(arc) => trace_path(context, &arc.path_elements(0.1)),
        RecordedShape::BezPath(path) => trace_path(context, path.elements_slice()),
    }
    Ok(())
}

fn trace_path(context: &CanvasRenderingContext2d, elements: &[PathEl]) {
    for element in elements {
        match element {
            PathEl::MoveTo(p) => context.move_to(p.x, p.y),
            PathEl::LineTo(p) => context.line_to(p.x, p.y),
            PathEl::QuadTo(c, p) => context.quadratic_curve_to(c.x, c.y, p.x, p.y),
            PathEl::CurveTo(c1, c2, p) => context.bezier_curve_to(c1.x, c1.y, c2.x, c2.y, p.x, p.y),
            PathEl::ClosePath => context.close_path(),
        }
    }
}
