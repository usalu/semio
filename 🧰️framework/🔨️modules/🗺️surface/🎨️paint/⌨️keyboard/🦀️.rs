//! ⌨️ Keyboard operation of the pixel tools: the pixel cursor, the marquee, stamping, the camera keys and the stroke length limit,
//! the same functions and payload shapes as `🧱️elements/🖌️Paint2dHost/✍️editing/⌨️keyboard/🟦️.ts`.
use super::{camera, gesture, Affine, PaintTarget, RasterHost};

/// 🌊️ Points a stroke buffers before it streams the buffer as one history-neutral segment.
pub const STROKE_POINT_CAP: usize = 2048;
/// ⚠️ The count from which the user is told the stroke is nearing the segment boundary.
pub const STROKE_WARNING_POINTS: usize = STROKE_POINT_CAP * 9 / 10;
const ZOOM_FACTOR: f64 = 1.25;
const REPEAT_STRIDE: u32 = 8;
const CAMERA_SCREEN_STRIDE: f64 = 40.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PixelCursor {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelExtent {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelMarquee {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrokeLimit {
    Ok,
    Warn,
    Full,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StrokeRollover {
    pub batch: Vec<[f64; 2]>,
    pub points: Vec<[f64; 2]>,
    pub sent: usize,
}

fn arrow(key: &str) -> Option<(i64, i64)> {
    match key {
        "ArrowLeft" => Some((-1, 0)),
        "ArrowRight" => Some((1, 0)),
        "ArrowUp" => Some((0, -1)),
        "ArrowDown" => Some((0, 1)),
        _ => None,
    }
}

/// 🧭️ Moves the cursor by `stride` pixels along an arrow key and clamps it to the image; other keys answer `None`.
pub fn step_keyboard_cursor(cursor: PixelCursor, key: &str, extent: PixelExtent, stride: u32) -> Option<PixelCursor> {
    let (dx, dy) = arrow(key)?;
    let clamp = |value: i64, size: u32| value.clamp(0, i64::from(size.max(1)) - 1) as u32;
    let stride = i64::from(stride);
    Some(PixelCursor { x: clamp(i64::from(cursor.x) + dx * stride, extent.width), y: clamp(i64::from(cursor.y) + dy * stride, extent.height) })
}

/// 🖋️ A stamp is a two-point stroke through the pixel center, the exact payload shape a pointer click produces.
pub fn keyboard_stamp_path(cursor: PixelCursor) -> [[f64; 2]; 2] {
    let point = [f64::from(cursor.x) + 0.5, f64::from(cursor.y) + 0.5];
    [point, point]
}

/// ▭ The inclusive rectangle spanned by the anchor pixel and the cursor pixel.
pub fn keyboard_marquee(anchor: PixelCursor, cursor: PixelCursor) -> PixelMarquee {
    PixelMarquee { x: anchor.x.min(cursor.x), y: anchor.y.min(cursor.y), width: anchor.x.abs_diff(cursor.x) + 1, height: anchor.y.abs_diff(cursor.y) + 1 }
}

/// 📣️ The live-region text for the cursor, one-based and localized (`de`, otherwise English).
pub fn cursor_announcement(locale: &str, cursor: PixelCursor, extent: PixelExtent) -> String {
    let (column, row) = (cursor.x + 1, cursor.y + 1);
    if locale == "de" {
        format!("Pixel Spalte {column} von {}, Zeile {row} von {}", extent.width, extent.height)
    } else {
        format!("Pixel column {column} of {}, row {row} of {}", extent.width, extent.height)
    }
}

/// 🔍️ Zoom (`+`, `=`, `-`), reset (`0`) and pan (arrows, `screen_stride` screen pixels) when no tool owns the arrows.
pub fn keyboard_camera_step(camera: [f64; 3], key: &str, screen_stride: f64) -> Option<[f64; 3]> {
    let zoom = camera::clamp_zoom;
    let [x, y, z] = camera;
    match key {
        "+" | "=" => Some([x, y, zoom(z * ZOOM_FACTOR)]),
        "-" => Some([x, y, zoom(z / ZOOM_FACTOR)]),
        "0" => Some([x, y, 1.0]),
        _ => arrow(key).map(|(dx, dy)| [x + dx as f64 * screen_stride / z, y + dy as f64 * screen_stride / z, z]),
    }
}

/// 🚦️ How close a buffered stroke is to its segment boundary.
pub fn stroke_limit_state(count: usize) -> StrokeLimit {
    if count >= STROKE_POINT_CAP {
        StrokeLimit::Full
    } else if count >= STROKE_WARNING_POINTS {
        StrokeLimit::Warn
    } else {
        StrokeLimit::Ok
    }
}

/// ✂️ At the cap, hands out the unsent points as one stream batch and restarts the buffer at the last point so the gesture stays one history step.
pub fn rollover_stroke(points: &[[f64; 2]], sent: usize, cap: usize) -> Option<StrokeRollover> {
    (points.len() >= cap).then(|| StrokeRollover { batch: points[sent.min(points.len())..].to_vec(), points: points.last().copied().into_iter().collect(), sent: 1 })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PixelKeyTool {
    Layers,
    Brush,
    Eraser,
    Bucket,
    Wand,
    Rectangle,
    Ellipse,
    Lasso,
}

impl PixelKeyTool {
    /// 🧰️ The tool a raster utility id puts the keyboard in, `Layers` for every utility that owns no pixel tool.
    pub fn from_utility(utility: &str) -> Self {
        match utility {
            "paintBrush" => Self::Brush,
            "paintEraser" => Self::Eraser,
            "paintBucket" => Self::Bucket,
            "selectWand" => Self::Wand,
            "selectMarquee" => Self::Rectangle,
            "selectEllipse" => Self::Ellipse,
            "selectLasso" => Self::Lasso,
            _ => Self::Layers,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PixelKeyInput<'a> {
    pub key: &'a str,
    pub space: bool,
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub repeat: bool,
}

/// 🎹️ What one key press asks of the caller: nothing, a camera to publish, a published edit already queued on the host, or a pixel selection to compute.
#[derive(Clone, Debug, PartialEq)]
pub enum PixelKeyOutcome {
    Ignored,
    Consumed,
    Camera([f64; 3]),
    Edit,
    Select { method: &'static str, path: Vec<[f64; 2]> },
}

#[derive(Clone, Debug, Default)]
pub struct PixelKeyboard {
    pub cursor: Option<PixelCursor>,
    pub anchor: Option<PixelCursor>,
    pub announcement: String,
    context: Option<(String, PixelExtent, &'static str, String)>,
}

impl PixelKeyboard {
    fn bind(&mut self, context: Option<(String, PixelExtent, &'static str, String)>) {
        if self.context != context {
            *self = Self { context, ..Self::default() };
        }
    }
}

impl RasterHost {
    fn keyboard_binding(&self) -> Option<(String, PixelExtent, &'static str, String)> {
        let id = self.selected_ids.first()?;
        let (_, (width, height)) = gesture::target(self, &self.document.layers, id, Affine::IDENTITY, true)?;
        Some((id.clone(), PixelExtent { width, height }, if self.paint_target == PaintTarget::Pixels { "pixels" } else { "mask" }, self.active_utility.clone()))
    }

    /// 🧭️ Drops the cursor, anchor and announcement once the layer, its extent, the target or the utility they belong to changed.
    pub fn sync_keyboard(&mut self) {
        let binding = self.keyboard_binding();
        self.keyboard.bind(binding);
    }

    pub fn keyboard(&self) -> &PixelKeyboard {
        &self.keyboard
    }

    /// ⌨️ One key press of the pixel keyboard, the port of `keyboardPaint` in `🖌️Paint2dHost/✍️editing/🟦️.tsx`; `ready` is whether no selection or edit work is in flight, `locale` the announcement language and `merge` the selection merge the `selectPixels` of a marquee carries.
    pub fn pixel_key_down(&mut self, input: &PixelKeyInput<'_>, locale: &str, ready: bool) -> PixelKeyOutcome {
        if input.ctrl || input.meta || input.alt {
            return PixelKeyOutcome::Ignored;
        }
        self.sync_keyboard();
        let tool = PixelKeyTool::from_utility(&self.active_utility);
        let view = [self.camera.x, self.camera.y, self.camera.zoom];
        let zooming = matches!(input.key, "+" | "=" | "-" | "0");
        if zooming || tool == PixelKeyTool::Layers {
            return match keyboard_camera_step(view, input.key, CAMERA_SCREEN_STRIDE) {
                Some(next) => {
                    self.set_camera(next[0], next[1], next[2]);
                    PixelKeyOutcome::Camera(next)
                }
                None => PixelKeyOutcome::Ignored,
            };
        }
        let Some(extent) = self.keyboard.context.as_ref().map(|context| context.1) else { return PixelKeyOutcome::Ignored };
        let origin = self.keyboard.cursor.unwrap_or(PixelCursor { x: extent.width / 2, y: extent.height / 2 });
        if let Some(moved) = step_keyboard_cursor(origin, input.key, extent, if input.repeat { REPEAT_STRIDE } else { 1 }) {
            self.keyboard.anchor = input.shift.then_some(self.keyboard.anchor.unwrap_or(origin));
            self.keyboard.cursor = Some(moved);
            self.keyboard.announcement = cursor_announcement(locale, moved, extent);
            return PixelKeyOutcome::Consumed;
        }
        let brush = matches!(tool, PixelKeyTool::Brush | PixelKeyTool::Eraser);
        if input.key != "Enter" && !(input.space && brush) {
            return PixelKeyOutcome::Ignored;
        }
        if input.repeat || !ready {
            return PixelKeyOutcome::Consumed;
        }
        let at = self.keyboard.cursor.unwrap_or(origin);
        if self.keyboard.cursor.is_none() {
            self.keyboard.cursor = Some(at);
            self.keyboard.announcement = cursor_announcement(locale, at, extent);
        }
        let center = [f64::from(at.x) + 0.5, f64::from(at.y) + 0.5];
        match tool {
            PixelKeyTool::Brush | PixelKeyTool::Eraser => {
                let name = if tool == PixelKeyTool::Brush { "brush" } else { "eraser" };
                if let Some(edit) = gesture::PaintGesture::keyboard_edit(self, name, keyboard_stamp_path(at).to_vec()) {
                    self.paint_edit = Some(edit);
                    return PixelKeyOutcome::Edit;
                }
                PixelKeyOutcome::Consumed
            }
            PixelKeyTool::Bucket => match gesture::PaintGesture::keyboard_edit(self, "bucket", vec![center]) {
                Some(edit) if edit.points.first().is_some_and(|point| point[0] < f64::from(extent.width) && point[1] < f64::from(extent.height)) => {
                    self.paint_edit = Some(edit);
                    PixelKeyOutcome::Edit
                }
                _ => PixelKeyOutcome::Consumed,
            },
            PixelKeyTool::Wand => PixelKeyOutcome::Select { method: "wand", path: vec![center] },
            PixelKeyTool::Rectangle | PixelKeyTool::Ellipse | PixelKeyTool::Lasso => {
                let marquee = keyboard_marquee(self.keyboard.anchor.unwrap_or(at), at);
                self.keyboard.anchor = None;
                let (x, y) = (f64::from(marquee.x), f64::from(marquee.y));
                PixelKeyOutcome::Select { method: if tool == PixelKeyTool::Ellipse { "ellipse" } else { "rectangle" }, path: vec![[x, y], [x + f64::from(marquee.width), y + f64::from(marquee.height)]] }
            }
            PixelKeyTool::Layers => PixelKeyOutcome::Ignored,
        }
    }

    /// ⎋️ Escape drops the keyboard marquee anchor; `true` while one was set.
    pub fn cancel_keyboard_marquee(&mut self) -> bool {
        self.keyboard.anchor.take().is_some()
    }

    /// 🟨️ The pixel cursor and the marquee as image-pixel rectangles with the layer's pixel-to-world placement, for the overlay a paint pass draws.
    pub(super) fn keyboard_overlay(&self) -> Option<(Affine, Option<PixelMarquee>, PixelCursor)> {
        let cursor = self.keyboard.cursor?;
        let id = self.selected_ids.first()?;
        let (world, _) = gesture::target(self, &self.document.layers, id, Affine::IDENTITY, true)?;
        Some((world, self.keyboard.anchor.map(|anchor| keyboard_marquee(anchor, cursor)), cursor))
    }
}
