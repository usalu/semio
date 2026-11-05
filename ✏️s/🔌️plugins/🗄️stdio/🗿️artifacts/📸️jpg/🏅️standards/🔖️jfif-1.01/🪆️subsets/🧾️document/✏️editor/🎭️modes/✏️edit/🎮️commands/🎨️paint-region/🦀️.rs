//! 🎨️ Checked JPEG region painting with retained progress and cancellation.

use super::{JpgAnyEditCommand, JpgAnyEditor};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::{EditorApp, Emit, Fault, FaultCode, FaultOrigin};
use super::main::MAXIMUM_INTERACTIVE_ROWS;
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot as Snapshot;
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::JpgMutation as Mutation;

pub const ACTION_ID: &str = "paint-region";
pub const TOOL_IDS: &[&str] = &[ACTION_ID];
pub const PAYLOAD_SCHEMA: &str = "s.stdio.jpg.any.command.paint-region.v1";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
const ROWS_PER_TURN: usize = 64;
pub const CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintRegionCommand {
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
    let value = argument(args, name).ok_or_else(|| fault("stdio.jpg.paint-region.missing-argument", format!("Missing {name}")))?;
    let semio_framework_value::DslValue::Number(number) = value else { return Err(fault("stdio.jpg.paint-region.invalid-argument", format!("{name} must be an integer"))); };
    let value = number.as_u64().ok_or_else(|| fault("stdio.jpg.paint-region.invalid-argument", format!("{name} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("stdio.jpg.paint-region.invalid-argument", format!("{name} exceeds {maximum}")))
}

impl PaintRegionCommand {
    pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
        let width = integer(args, "width", u32::MAX.into())? as u32;
        let height = integer(args, "height", u32::MAX.into())? as u32;
        if width == 0 || height == 0 { return Err(fault("stdio.jpg.paint-region.empty", "Paint region width and height must be positive")); }
        if height > MAXIMUM_INTERACTIVE_ROWS { return Err(fault("stdio.jpg.paint-region.too-many-rows", format!("Paint region height exceeds {MAXIMUM_INTERACTIVE_ROWS}"))); }
        Ok(Self {
            x: integer(args, "x", u32::MAX.into())? as u32,
            y: integer(args, "y", u32::MAX.into())? as u32,
            width,
            height,
            red: integer(args, "red", u8::MAX.into())? as u8,
            green: integer(args, "green", u8::MAX.into())? as u8,
            blue: integer(args, "blue", u8::MAX.into())? as u8,
            alpha: integer(args, "alpha", u8::MAX.into())? as u8,
        })
    }
}

fn validate(command: PaintRegionCommand, snapshot: &Snapshot) -> Result<String, Fault> {
    let image = &snapshot.image;
    let right = command.x.checked_add(command.width).ok_or_else(|| fault("stdio.jpg.paint-region.out-of-bounds", "Paint region overflows the image"))?;
    let bottom = command.y.checked_add(command.height).ok_or_else(|| fault("stdio.jpg.paint-region.out-of-bounds", "Paint region overflows the image"))?;
    if right > image.width || bottom > image.height { return Err(fault("stdio.jpg.paint-region.out-of-bounds", format!("Paint region {}x{} at {},{} exceeds the {}x{} image", command.width, command.height, command.x, command.y, image.width, image.height))); }
    if image.pixels.len() != image.width as usize * image.height as usize * 4 { return Err(fault("stdio.jpg.paint-region.invalid-snapshot", "The decoded raster does not match the image extent")); }
    Ok(format!("{}x{}", image.width, image.height))
}

pub struct PaintRegionWork {
    revision: Option<String>,
    samples: Option<Vec<u8>>,
    cursor: usize,
    rows: usize,
    complete: bool,
    closing: bool,
}

impl PaintRegionWork {
    pub fn new() -> Self {
        Self { revision: None, samples: None, cursor: 0, rows: 0, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<JpgAnyEditor>> for PaintRegionWork {
    fn tool_id(&self) -> &'static str { ACTION_ID }

    fn extent(&self, command: &JpgAnyEditCommand, snapshot: &Snapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<JpgAnyEditor>>>) -> Option<usize> {
        let JpgAnyEditCommand::PaintRegion(command) = command else { return None };
        if command.width == 0 || command.height == 0 || snapshot.image.pixels.is_empty() { return None; }
        CAPACITY.rows_for_items(1)
    }

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<JpgAnyEditor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: input.snapshot.image.pixels.len().max(size_of::<Mutation>()), depth: 1, ..Default::default() })
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<JpgAnyEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<JpgAnyEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.closing || self.complete { return Err(fault("stdio.jpg.paint-region.work-closed", "Paint region work is already closed")); }
        let JpgAnyEditCommand::PaintRegion(command) = input.command else { return Err(fault("stdio.jpg.paint-region.route-mismatch", "Paint work received another command")); };
        if self.revision.is_none() {
            self.revision = Some(validate(*command, input.snapshot)?);
            self.samples = Some(input.snapshot.image.pixels.clone());
            self.rows = command.height as usize;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "jpg-paint-region-prepare", preview: br#"{"en":"Preparing region paint","de":"Bereichsmalen wird vorbereitet"}"# });
        }
        if self.cursor < self.rows {
            let samples = self.samples.as_mut().ok_or_else(|| fault("stdio.jpg.paint-region.work-closed", "Paint region work lost its sample buffer"))?;
            let end = (self.cursor + ROWS_PER_TURN).min(self.rows);
            while self.cursor < end {
            let command = *command;
            let stride = input.snapshot.image.width as usize * 4;
            let row = command.y as usize + self.cursor;
            let start = row * stride + command.x as usize * 4;
            for column in 0..command.width as usize { samples[start + column * 4..start + column * 4 + 4].copy_from_slice(&[command.red, command.green, command.blue, command.alpha]); }
                self.cursor += 1;
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "jpg-paint-region-row", preview: br#"{"en":"Painting rows","de":"Zeilen werden gemalt"}"# });
        }
        self.revision.take();
        let samples = self.samples.take().ok_or_else(|| fault("stdio.jpg.paint-region.work-closed", "Paint region work lost its sample buffer"))?;
        self.complete = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![Mutation::ReplacePixels(crate::standards::v_jfif_1_01::subsets::document::schema::mutations::ReplacePixelsMutation { pixels: samples })])))
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            if !self.closing { return semio_framework_value::RetainedWorkOutcome { result: Ok(InteractiveJobCloseStep::Blocked), work: 0 }; }
            self.rows = 0;
            self.cursor = 0;
            self.samples = None;
            semio_framework_value::RetainedWorkOutcome { result: Ok(semio_s_artifact_stdio_contract::editing::close_revision_turn(&mut self.revision, grant)), work: 1 }
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(semio_s_artifact_stdio_contract::editing::revision_close_demand(&self.revision).release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(semio_s_artifact_stdio_contract::editing::revision_close_demand(&self.revision).depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.revision.is_none() && self.samples.is_none() && self.rows == 0 && self.cursor == 0
    }
}
