//! 🎨️ Checked GIF region painting with retained progress and cancellation.

use super::{Gif89aEditCommand, Gif89aEditor};
use semio_framework_job::InteractiveJobCloseStep;
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_plugin::{ActionArgDef, ActionDefinition, ActionKind, ArgSchema, EditorApp, Emit, Fault, FaultCode, FaultOrigin};
use semio_ui_locale::LocalizedLabel;
use crate::standards::v89a::subsets::any::schema::snapshot::GifSnapshot as Snapshot;
use crate::standards::v89a::subsets::any::schema::mutations::GifMutation as Mutation;

pub const ACTION_ID: &str = "paint-region";
pub const TOOL_IDS: &[&str] = &[ACTION_ID];
pub const PAYLOAD_SCHEMA: &str = "s.stdio.gif.89a.command.paint-region.v1";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
pub const MAXIMUM_INTERACTIVE_ROWS: u32 = 65_536;
const ROWS_PER_TURN: usize = 64;
pub const CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintRegionCommand {
    pub item: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub index: u8,
}

fn fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

fn argument<'a>(args: Option<&'a semio_framework_value::DslValue>, name: &str) -> Option<&'a semio_framework_value::DslValue> {
    let Some(semio_framework_value::DslValue::Object(fields)) = args else { return None };
    fields.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}

fn integer(args: Option<&semio_framework_value::DslValue>, name: &'static str, maximum: u64) -> Result<u64, Fault> {
    let value = argument(args, name).ok_or_else(|| fault("stdio.gif.paint-region.missing-argument", format!("Missing {name}")))?;
    let semio_framework_value::DslValue::Number(number) = value else { return Err(fault("stdio.gif.paint-region.invalid-argument", format!("{name} must be an integer"))); };
    let value = number.as_u64().ok_or_else(|| fault("stdio.gif.paint-region.invalid-argument", format!("{name} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("stdio.gif.paint-region.invalid-argument", format!("{name} exceeds {maximum}")))
}

impl PaintRegionCommand {
    pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
        let width = integer(args, "width", u32::MAX.into())? as u32;
        let height = integer(args, "height", u32::MAX.into())? as u32;
        if width == 0 || height == 0 { return Err(fault("stdio.gif.paint-region.empty", "Paint region width and height must be positive")); }
        if height > MAXIMUM_INTERACTIVE_ROWS { return Err(fault("stdio.gif.paint-region.too-many-rows", format!("Paint region height exceeds {MAXIMUM_INTERACTIVE_ROWS}"))); }
        Ok(Self {
            item: integer(args, "item", u32::MAX.into())? as u32,
            x: integer(args, "x", u32::MAX.into())? as u32,
            y: integer(args, "y", u32::MAX.into())? as u32,
            width,
            height,
            index: integer(args, "index", u8::MAX.into())? as u8,
        })
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
    ActionDefinition::bounded_catalog(ACTION_ID, LocalizedLabel::native("Paint Frame Region", "Einzelbildbereich malen"), ActionKind::Mutation).with_args(vec![
        integer_arg("item", "Frame", "Einzelbild", 0.0, u32::MAX as f64, 0),
        integer_arg("x", "Left (x)", "Links (x)", 0.0, u32::MAX as f64, 0),
        integer_arg("y", "Top (y)", "Oben (y)", 0.0, u32::MAX as f64, 0),
        integer_arg("width", "Width", "Breite", 1.0, u32::MAX as f64, 1),
        integer_arg("height", "Height", "Höhe", 1.0, f64::from(MAXIMUM_INTERACTIVE_ROWS), 1),
        integer_arg("index", "Palette index", "Palettenindex", 0.0, 255.0, 0),
    ])
}

fn validate(command: PaintRegionCommand, snapshot: &Snapshot) -> Result<String, Fault> {
    let item = snapshot.frames.get(command.item as usize).ok_or_else(|| fault("stdio.gif.paint-region.no-item", format!("The document has no {} number {}", "frames", command.item)))?;
    let right = command.x.checked_add(command.width).ok_or_else(|| fault("stdio.gif.paint-region.out-of-bounds", "Paint region overflows the frame"))?;
    let bottom = command.y.checked_add(command.height).ok_or_else(|| fault("stdio.gif.paint-region.out-of-bounds", "Paint region overflows the frame"))?;
    if right > item.width || bottom > item.height { return Err(fault("stdio.gif.paint-region.out-of-bounds", format!("Paint region {}x{} at {},{} exceeds the {}x{} frame", command.width, command.height, command.x, command.y, item.width, item.height))); }
    if item.indices.len() != item.width as usize * item.height as usize { return Err(fault("stdio.gif.paint-region.invalid-snapshot", "The frame indices do not match the frame extent")); }
    Ok(format!("{}:{}x{}", command.item, item.width, item.height))
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

impl ArtifactCommandWork<EditorApp<Gif89aEditor>> for PaintRegionWork {
    fn terminal_frame_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn tool_id(&self) -> &'static str { ACTION_ID }

    fn extent(&self, command: &Gif89aEditCommand, snapshot: &Snapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gif89aEditor>>>) -> Option<usize> {
        let Gif89aEditCommand::PaintRegion(command) = command else { return None };
        if command.width == 0 || command.height == 0 || snapshot.frames.get(command.item as usize).is_none() { return None; }
        CAPACITY.rows_for_items(1)
    }

    fn work_demands(&self, input: &ArtifactCommandInputs<'_, EditorApp<Gif89aEditor>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: input.snapshot.frames.get(match input.command { Gif89aEditCommand::PaintRegion(command) => command.item as usize, _ => 0 }).map_or(0, |item| item.indices.len()).max(size_of::<Mutation>()), depth: 1, ..Default::default() })
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<Gif89aEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Gif89aEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.closing || self.complete { return Err(fault("stdio.gif.paint-region.work-closed", "Paint region work is already closed")); }
        let Gif89aEditCommand::PaintRegion(command) = input.command else { return Err(fault("stdio.gif.paint-region.route-mismatch", "Paint work received another command")); };
        if self.revision.is_none() {
            self.revision = Some(validate(*command, input.snapshot)?);
            self.samples = Some(input.snapshot.frames[command.item as usize].indices.clone());
            self.rows = command.height as usize;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "gif-paint-region-prepare", preview: br#"{"en":"Preparing region paint","de":"Bereichsmalen wird vorbereitet"}"# });
        }
        if self.cursor < self.rows {
            let samples = self.samples.as_mut().ok_or_else(|| fault("stdio.gif.paint-region.work-closed", "Paint region work lost its sample buffer"))?;
            let end = (self.cursor + ROWS_PER_TURN).min(self.rows);
            while self.cursor < end {
            let command = *command;
            let width = input.snapshot.frames[command.item as usize].width as usize;
            let start = (command.y as usize + self.cursor) * width + command.x as usize;
            samples[start..start + command.width as usize].fill(command.index);
                self.cursor += 1;
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "gif-paint-region-row", preview: br#"{"en":"Painting rows","de":"Zeilen werden gemalt"}"# });
        }
        self.revision.take();
        let samples = self.samples.take().ok_or_else(|| fault("stdio.gif.paint-region.work-closed", "Paint region work lost its sample buffer"))?;
        self.complete = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![Mutation::SetFramePixels(crate::standards::v89a::subsets::any::schema::mutations::set_frame_pixels::SetFramePixels { index: command.item as usize, indices: samples })])))
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|grant|{
            if !self.closing { return semio_framework_value::RetainedWorkOutcome::<_, semio_framework_value::ValueError> { result: Ok(InteractiveJobCloseStep::Blocked), work: 0 }; }
            self.rows = 0;
            self.cursor = 0;
            self.samples = None;
            semio_framework_value::RetainedWorkOutcome::<_, semio_framework_value::ValueError> { result: Ok(semio_s_artifact_stdio_contract::editing::close_revision_turn(&mut self.revision, grant)), work: 1 }
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

