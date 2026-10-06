//! 🩹️ Checked, cancellable RGBA8 region painting for the PNG image canvas.
use super::{PngEditCommand, PngEditor, PngNativeEditCommand};
use crate::schema::mutations::{PatchPixelsMutation, PngMutation};
use crate::schema::snapshot::PngSnapshot;
use crate::standards::v1_2::subsets::any::io::{png_layout, png_revision, project_png};
use semio_s_artifact_stdio_contract::editing::raster::{RasterRegion, RasterRegionError, RasterRegionLimits, RasterRegionPlan};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArgSchema;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_ui_locale::LocalizedLabel;

pub const ACTION_ID: &str = "set-pixel-region";
pub const PAYLOAD_SCHEMA: &str = "s.stdio.png.command.patch-pixel-region.v1";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
pub const MAXIMUM_RASTER_BYTES: usize = 4_096 * 2_160 * 4;
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

fn argument<'a>(args: Option<&'a semio_framework_value::DslValue>, name: &str) -> Option<&'a semio_framework_value::DslValue> {
    let Some(semio_framework_value::DslValue::Object(fields)) = args else { return None };
    fields.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}

fn integer(args: Option<&semio_framework_value::DslValue>, name: &'static str, maximum: u64) -> Result<u64, Fault> {
    let value = argument(args, name).ok_or_else(|| fault("stdio.png.pixel-region.missing-argument", format!("Missing {name}")))?;
    let semio_framework_value::DslValue::Number(number) = value else {
        return Err(fault("stdio.png.pixel-region.invalid-argument", format!("{name} must be an integer")));
    };
    let value = number.as_u64().ok_or_else(|| fault("stdio.png.pixel-region.invalid-argument", format!("{name} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("stdio.png.pixel-region.invalid-argument", format!("{name} exceeds {maximum}")))
}

impl PatchPixelRegion {
    pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
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

    fn validate(&self, snapshot: &PngSnapshot) -> Result<RasterRegionPlan, Fault> {
        let layout = png_layout(snapshot).map_err(|message| fault("stdio.png.pixel-region.invalid-snapshot", message))?;
        if layout.color_type != crate::schema::snapshot::PngColorType::Rgba || layout.bit_depth != 8 || layout.interlace {
            return Err(fault("stdio.png.pixel-region.profile-mismatch", "Pixel region painting requires an 8-bit non-interlaced RGBA PNG"));
        }
        let projection = project_png(&snapshot.bytes).map_err(|message| fault("stdio.png.pixel-region.invalid-snapshot", message))?;
        RasterRegionPlan::new(layout.width, layout.height, projection.pixels.len(),
            RasterRegion { x: self.x, y: self.y, width: self.width, height: self.height, color: [self.red, self.green, self.blue, self.alpha] },
            RasterRegionLimits { maximum_raster_bytes: MAXIMUM_RASTER_BYTES, maximum_patch_bytes: PATCH_PAYLOAD_BYTES, maximum_patches: CAPACITY.invertible_items() }
        ).map_err(region_fault)
    }
}

fn region_fault(error: RasterRegionError) -> Fault {
    let code = match error {
        RasterRegionError::Empty => "stdio.png.pixel-region.empty",
        RasterRegionError::InvalidBudget => "stdio.png.pixel-region.invalid-budget",
        RasterRegionError::ExtentOverflow => "stdio.png.pixel-region.extent-overflow",
        RasterRegionError::NoncanonicalRaster => "stdio.png.pixel-region.noncanonical-raster",
        RasterRegionError::RasterTooLarge => "stdio.png.pixel-region.raster-too-large",
        RasterRegionError::BoundsOverflow => "stdio.png.pixel-region.bounds-overflow",
        RasterRegionError::OutOfBounds => "stdio.png.pixel-region.out-of-bounds",
        RasterRegionError::TooManyPatches => "stdio.png.pixel-region.too-many-patches",
        RasterRegionError::InvalidOrdinal => "stdio.png.pixel-region.invalid-ordinal",
    };
    fault(code, format!("Pixel region cannot be applied: {error}"))
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
    plan: Option<RasterRegionPlan>,
    revision: Option<String>,
    cursor: usize,
    complete: bool,
    closing: bool,
}

impl ArtifactCommandWork<EditorApp<PngEditor>> for PatchPixelRegionWork {
    fn tool_id(&self) -> &'static str {
        ACTION_ID
    }

    fn extent(&self, command: &PngEditCommand, snapshot: &PngSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PngEditor>>>) -> Option<usize> {
        let PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(command)) = command else { return None };
        CAPACITY.rows_for_items(command.validate(snapshot).ok()?.patch_count())
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<PngEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<PngEditor>>, Fault> {
        if self.closing || self.complete {
            return Err(fault("stdio.png.pixel-region.work-closed", "Pixel region work is already closed"));
        }
        let PngEditCommand::Native(PngNativeEditCommand::PatchPixelRegion(command)) = input.command else {
            return Err(fault("stdio.png.pixel-region.route-mismatch", "Pixel region work received another command"));
        };
        if self.plan.is_none() {
            self.plan = Some(command.validate(input.snapshot)?);
            self.revision = Some(png_revision(input.snapshot));
            return Ok(ArtifactCommandWorkStep::Progress { stage: "png-pixel-region-prepare", preview: br#"{"en":"Preparing pixel region","de":"Pixelbereich wird vorbereitet"}"# });
        }
        let plan = self.plan.expect("plan was prepared");
        if self.cursor < plan.patch_count() {
            self.cursor += 1;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "png-pixel-region-patch", preview: br#"{"en":"Painting pixel region","de":"Pixelbereich wird gemalt"}"# });
        }
        self.complete = true;
        let revision = self.revision.take().expect("revision was prepared");
        Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![PngMutation::PatchPixels(PatchPixelsMutation {
            revision,
            x: command.x,
            y: command.y,
            width: command.width,
            height: command.height,
            red: command.red,
            green: command.green,
            blue: command.blue,
            alpha: command.alpha,
        })])))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 { return InteractiveJobCloseStep::Blocked; }
        self.plan = None;
        self.revision = None;
        self.cursor = 0;
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.plan.is_none() && self.revision.is_none() && self.cursor == 0
    }
}
