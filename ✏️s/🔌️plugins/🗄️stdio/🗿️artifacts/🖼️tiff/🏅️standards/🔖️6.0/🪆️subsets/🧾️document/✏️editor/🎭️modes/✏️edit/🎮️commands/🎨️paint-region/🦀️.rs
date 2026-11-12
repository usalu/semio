//! 🎨️ Checked TIFF tiled-region authoring with retained progress and cancellation.

use super::{TiffAnyEditCommand, TiffAnyEditor};
use crate::editor::tiff_any::config::selected_ifd;
use crate::schema::mutations::paint_region::samples::{tiff_revision,validate_tiff_region_paint,TiffRegion};
use crate::schema::mutations::{PaintRegionMutation, TiffMutation};
use semio_framework_job::InteractiveJobCloseStep;
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_plugin::{EditorApp, Emit, Fault, FaultCode, FaultOrigin};

#[path = "📜️declaration/🦀️.rs"]
mod declaration;
pub use declaration::{action, ACTION_ID, MAXIMUM_INTERACTIVE_ROWS};
pub const TOOL_IDS: &[&str] = &[ACTION_ID];
pub const PAYLOAD_SCHEMA: &str = "s.stdio.tiff.command.paint-region.v1";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
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
    let value = argument(args, name).ok_or_else(|| fault("stdio.tiff.paint-region.missing-argument", format!("Missing {name}")))?;
    let semio_framework_value::DslValue::Number(number) = value else { return Err(fault("stdio.tiff.paint-region.invalid-argument", format!("{name} must be an integer"))); };
    let value = number.as_u64().ok_or_else(|| fault("stdio.tiff.paint-region.invalid-argument", format!("{name} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("stdio.tiff.paint-region.invalid-argument", format!("{name} exceeds {maximum}")))
}

impl PaintRegionCommand {
    pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
        let width = integer(args, "width", u32::MAX.into())? as u32;
        let height = integer(args, "height", u32::MAX.into())? as u32;
        if width == 0 || height == 0 { return Err(fault("stdio.tiff.paint-region.empty", "Paint region width and height must be positive")); }
        if height > MAXIMUM_INTERACTIVE_ROWS { return Err(fault("stdio.tiff.paint-region.too-many-rows", format!("Paint region height exceeds {MAXIMUM_INTERACTIVE_ROWS}"))); }
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

    fn region(self) -> TiffRegion {
        TiffRegion { x: self.x, y: self.y, width: self.width, height: self.height }
    }

    fn color(self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

fn validate(command: PaintRegionCommand, snapshot: &crate::TiffSnapshot, ifd_index: usize) -> Result<String, Fault> {
    validate_tiff_region_paint(snapshot, ifd_index, command.region(), command.color()).map_err(|error| fault("stdio.tiff.paint-region.invalid-snapshot", error))?;
    Ok(tiff_revision(snapshot))
}

pub struct PaintRegionWork {
    revision: Option<String>,
    complete: bool,
    closing: bool,
}

impl PaintRegionWork {
    pub fn new() -> Self {
        Self { revision: None, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<TiffAnyEditor>> for PaintRegionWork {
    fn terminal_frame_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn tool_id(&self) -> &'static str { ACTION_ID }

    fn extent(&self, command: &TiffAnyEditCommand, snapshot: &crate::TiffSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<TiffAnyEditor>>>) -> Option<usize> {
        let TiffAnyEditCommand::PaintRegion(command) = command else { return None };
        if snapshot.ifds.is_empty() || command.width == 0 || command.height == 0 { return None; }
        CAPACITY.rows_for_items(1)
    }

    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<TiffAnyEditor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<TiffMutation>(), depth: 1, ..Default::default() })
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<TiffAnyEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<TiffAnyEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.closing || self.complete { return Err(fault("stdio.tiff.paint-region.work-closed", "Paint region work is already closed")); }
        let TiffAnyEditCommand::PaintRegion(command) = input.command else { return Err(fault("stdio.tiff.paint-region.route-mismatch", "Paint work received another command")); };
        let ifd_index = selected_ifd(input.config, input.snapshot).ok_or_else(|| fault("stdio.tiff.paint-region.no-page", "TIFF document has no selectable image page"))?;
        let revision = validate(*command, input.snapshot, ifd_index)?;
        let mutation = TiffMutation::PaintRegion(PaintRegionMutation {
            revision,
            ifd_index,
            x: command.x,
            y: command.y,
            width: command.width,
            height: command.height,
            red: command.red,
            green: command.green,
            blue: command.blue,
            alpha: command.alpha,
        });
        self.complete = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![mutation])))
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            if !self.closing { return semio_framework_value::RetainedWorkOutcome::<InteractiveJobCloseStep, semio_framework_job::OriginalTurnPrimary> { result: Ok(InteractiveJobCloseStep::Blocked), work: 0 }; }
            semio_framework_value::RetainedWorkOutcome { result: Ok(semio_s_artifact_stdio_contract::editing::close_revision_turn(&mut self.revision, grant)), work: 1 }
        })
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(semio_s_artifact_stdio_contract::editing::revision_close_demand(&self.revision).release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(semio_s_artifact_stdio_contract::editing::revision_close_demand(&self.revision).depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.revision.is_none()
    }
}

