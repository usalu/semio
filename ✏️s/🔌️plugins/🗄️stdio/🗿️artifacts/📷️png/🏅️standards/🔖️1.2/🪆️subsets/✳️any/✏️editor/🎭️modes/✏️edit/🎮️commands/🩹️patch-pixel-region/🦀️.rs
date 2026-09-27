//! 🩹️ Checked, cancellable RGBA8 region painting for the PNG image canvas.
use super::{PngEditCommand, PngEditor, PngNativeEditCommand};
use crate::schema::mutations::{PatchPixelsMutation, PngMutation};
use crate::schema::snapshot::PngSnapshot;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, ArgSchema, EditorApp, Emit, Fault, FaultCode, FaultOrigin, LocalizedLabel};

pub const ACTION_ID: &str = "set-pixel-region";
pub const PAYLOAD_SCHEMA: &str = "s.stdio.png.command.patch-pixel-region.v1";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
pub const MAXIMUM_RASTER_BYTES: usize = 32 * 1_024 * 1_024;
pub const PATCH_PAYLOAD_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES / 4;
pub const CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(128);

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PatchPixelRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

fn fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

fn argument<'a>(args: Option<&'a dsl::DslValue>, name: &str) -> Option<&'a dsl::DslValue> {
    let Some(dsl::DslValue::Object(fields)) = args else { return None };
    fields.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}

fn integer(args: Option<&dsl::DslValue>, name: &'static str, maximum: u64) -> Result<u64, Fault> {
    let value = argument(args, name).ok_or_else(|| fault("stdio.png.pixel-region.missing-argument", format!("Missing {name}")))?;
    let dsl::DslValue::Number(number) = value else {
        return Err(fault("stdio.png.pixel-region.invalid-argument", format!("{name} must be an integer")));
    };
    let value = number.as_u64().ok_or_else(|| fault("stdio.png.pixel-region.invalid-argument", format!("{name} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("stdio.png.pixel-region.invalid-argument", format!("{name} exceeds {maximum}")))
}

impl PatchPixelRegion {
    pub fn from_action(args: Option<&dsl::DslValue>) -> Result<Self, Fault> {
        let width = integer(args, "width", u32::MAX.into())?;
        let height = integer(args, "height", u32::MAX.into())?;
        if width == 0 || height == 0 {
            return Err(fault("stdio.png.pixel-region.empty", "Pixel region width and height must be positive"));
        }
        Ok(Self {
            x: integer(args, "x", u32::MAX.into())? as u32,
            y: integer(args, "y", u32::MAX.into())? as u32,
            width: width as u32,
            height: height as u32,
            red: integer(args, "red", u8::MAX.into())? as u8,
            green: integer(args, "green", u8::MAX.into())? as u8,
            blue: integer(args, "blue", u8::MAX.into())? as u8,
            alpha: integer(args, "alpha", u8::MAX.into())? as u8,
        })
    }

    fn color(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    fn validate(&self, snapshot: &PngSnapshot) -> Result<RegionPlan, Fault> {
        let row_bytes = usize::try_from(snapshot.width).ok().and_then(|width| width.checked_mul(4)).ok_or_else(|| fault("stdio.png.pixel-region.extent-overflow", "PNG row byte count overflows this platform"))?;
        let expected = row_bytes.checked_mul(snapshot.height as usize).ok_or_else(|| fault("stdio.png.pixel-region.extent-overflow", "PNG raster byte count overflows this platform"))?;
        if expected != snapshot.pixels.len() {
            return Err(fault("stdio.png.pixel-region.noncanonical-raster", format!("PNG raster has {} bytes; expected {expected}", snapshot.pixels.len())));
        }
        if expected > MAXIMUM_RASTER_BYTES {
            return Err(fault("stdio.png.pixel-region.raster-too-large", format!("PNG raster exceeds the {MAXIMUM_RASTER_BYTES}-byte interactive editing ceiling")));
        }
        let right = self.x.checked_add(self.width).ok_or_else(|| fault("stdio.png.pixel-region.bounds-overflow", "Pixel region x + width overflows"))?;
        let bottom = self.y.checked_add(self.height).ok_or_else(|| fault("stdio.png.pixel-region.bounds-overflow", "Pixel region y + height overflows"))?;
        if right > snapshot.width || bottom > snapshot.height {
            return Err(fault("stdio.png.pixel-region.out-of-bounds", format!("Pixel region {}/{}/{}×{} exceeds image {}×{}", self.x, self.y, self.width, self.height, snapshot.width, snapshot.height)));
        }
        let region_row_bytes = self.width as usize * 4;
        let (kind, patch_count) = if row_bytes <= PATCH_PAYLOAD_BYTES {
            let rows_per_patch = (PATCH_PAYLOAD_BYTES / row_bytes).max(1);
            let patch_count = (self.height as usize).div_ceil(rows_per_patch);
            (RegionPlanKind::Rows { rows_per_patch }, patch_count)
        } else {
            let chunks_per_row = region_row_bytes.div_ceil(PATCH_PAYLOAD_BYTES);
            let patch_count = (self.height as usize).checked_mul(chunks_per_row).ok_or_else(|| fault("stdio.png.pixel-region.patch-count-overflow", "Pixel region patch count overflows"))?;
            (RegionPlanKind::WideRows { chunks_per_row }, patch_count)
        };
        if patch_count == 0 || patch_count > CAPACITY.invertible_items() {
            return Err(fault("stdio.png.pixel-region.too-many-patches", format!("Pixel region needs {patch_count} patches; maximum is {}", CAPACITY.invertible_items())));
        }
        Ok(RegionPlan { row_bytes, region_row_bytes, patch_count, kind })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegionPlanKind {
    Rows { rows_per_patch: usize },
    WideRows { chunks_per_row: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RegionPlan {
    row_bytes: usize,
    region_row_bytes: usize,
    patch_count: usize,
    kind: RegionPlanKind,
}

impl RegionPlan {
    fn mutation(&self, command: &PatchPixelRegion, snapshot: &PngSnapshot, ordinal: usize) -> Option<PngMutation> {
        let color = command.color();
        let (index, mut pixels) = match self.kind {
            RegionPlanKind::Rows { rows_per_patch } => {
                let relative_row = ordinal * rows_per_patch;
                let rows = rows_per_patch.min(command.height as usize - relative_row);
                let row = command.y as usize + relative_row;
                let index = row * self.row_bytes;
                let mut pixels = snapshot.pixels[index..index + rows * self.row_bytes].to_vec();
                let left = command.x as usize * 4;
                for local_row in 0..rows {
                    let start = local_row * self.row_bytes + left;
                    for pixel in pixels[start..start + self.region_row_bytes].chunks_exact_mut(4) {
                        pixel.copy_from_slice(&color);
                    }
                }
                (index, pixels)
            }
            RegionPlanKind::WideRows { chunks_per_row } => {
                let relative_row = ordinal / chunks_per_row;
                let chunk = ordinal % chunks_per_row;
                let offset = chunk * PATCH_PAYLOAD_BYTES;
                let bytes = PATCH_PAYLOAD_BYTES.min(self.region_row_bytes - offset);
                let index = (command.y as usize + relative_row) * self.row_bytes + command.x as usize * 4 + offset;
                let mut pixels = vec![0; bytes];
                for pixel in pixels.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&color);
                }
                (index, pixels)
            }
        };
        let current = &snapshot.pixels[index..index + pixels.len()];
        if current == pixels {
            return None;
        }
        Some(PngMutation::PatchPixels(PatchPixelsMutation { index: index as u64, remove_count: pixels.len() as u64, pixels: std::mem::take(&mut pixels), move_to: None }))
    }
}

fn integer_arg(id: &'static str, en: &'static str, de: &'static str, minimum: f64, maximum: f64, default: u32) -> ActionArgDef {
    let mut argument = ActionArgDef::number(id, LocalizedLabel::native(en, de)).required().default_value(&default);
    if let ArgSchema::Number { min, max, step, integer, .. } = &mut argument.schema {
        *min = Some(minimum);
        *max = Some(maximum);
        *step = Some(1.0);
        *integer = true;
    }
    argument
}

pub fn action() -> ActionDefinition {
    ActionDefinition::bounded_catalog(ACTION_ID, LocalizedLabel::native("Paint Pixel Region", "Pixelbereich malen"), ActionKind::Mutation).with_args(args())
}

pub fn args() -> Vec<ActionArgDef> {
    vec![
        integer_arg("x", "Left (x)", "Links (x)", 0.0, u32::MAX as f64, 0),
        integer_arg("y", "Top (y)", "Oben (y)", 0.0, u32::MAX as f64, 0),
        integer_arg("width", "Width", "Breite", 1.0, u32::MAX as f64, 1),
        integer_arg("height", "Height", "Höhe", 1.0, u32::MAX as f64, 1),
        integer_arg("red", "Red", "Rot", 0.0, 255.0, 0),
        integer_arg("green", "Green", "Grün", 0.0, 255.0, 0),
        integer_arg("blue", "Blue", "Blau", 0.0, 255.0, 0),
        integer_arg("alpha", "Alpha", "Alpha", 0.0, 255.0, 255),
    ]
}

#[derive(Default)]
pub struct PatchPixelRegionWork {
    plan: Option<RegionPlan>,
    cursor: usize,
    mutations: Vec<PngMutation>,
    complete: bool,
    closing: bool,
}

impl ArtifactCommandWork<EditorApp<PngEditor>> for PatchPixelRegionWork {
    fn tool_id(&self) -> &'static str {
        ACTION_ID
    }

    fn extent(&self, command: &PngEditCommand, snapshot: &PngSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PngEditor>>>) -> Option<usize> {
        let PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(command)) = command else { return None };
        CAPACITY.rows_for_items(command.validate(snapshot).ok()?.patch_count)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<PngEditor>>) -> Result<ArtifactCommandWorkStep<EditorApp<PngEditor>>, Fault> {
        if self.closing || self.complete {
            return Err(fault("stdio.png.pixel-region.work-closed", "Pixel region work is already closed"));
        }
        let PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(command)) = input.command else {
            return Err(fault("stdio.png.pixel-region.route-mismatch", "Pixel region work received another command"));
        };
        if self.plan.is_none() {
            self.plan = Some(command.validate(input.snapshot)?);
            return Ok(ArtifactCommandWorkStep::Progress { stage: "png-pixel-region-prepare", preview: br#"{"en":"Preparing pixel region","de":"Pixelbereich wird vorbereitet"}"# });
        }
        let plan = self.plan.expect("plan was prepared");
        if self.cursor < plan.patch_count {
            if let Some(mutation) = plan.mutation(command, input.snapshot, self.cursor) {
                self.mutations.push(mutation);
            }
            self.cursor += 1;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "png-pixel-region-patch", preview: br#"{"en":"Painting pixel region","de":"Pixelbereich wird gemalt"}"# });
        }
        self.complete = true;
        let mutations = std::mem::take(&mut self.mutations);
        let description = (!mutations.is_empty()).then(|| format!("Paint PNG region {}/{}/{}×{}", command.x, command.y, command.width, command.height));
        Ok(ArtifactCommandWorkStep::Complete(Emit {
            artifact_mutations: mutations,
            description,
            ..Default::default()
        }))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Blocked;
        }
        let released_items = maximum_items.min(self.mutations.len());
        for _ in 0..released_items {
            self.mutations.pop();
        }
        if !self.mutations.is_empty() {
            return InteractiveJobCloseStep::Pending { released_items, released_bytes: 0 };
        }
        self.plan = None;
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.plan.is_none() && self.mutations.is_empty()
    }
}
