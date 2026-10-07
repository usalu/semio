//! 🎨️ Checked indexed and direct BMP region authoring with retained progress.

use super::{BmpEditCommand, BmpEditor, BmpNativeEditCommand};
use crate::schema::operations::{BmpPaintWorkOperation,BmpPaintWorkStep,BmpRetainedPaint};
use crate::schema::snapshot::BmpRegion;
use crate::schema::mutations::{BmpMutation, SetSnapshot};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, ArgSchema, EditorApp, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_ui_locale::LocalizedLabel;

pub const INDEXED_ACTION_ID: &str = "paint-indexed-region";
pub const DIRECT_ACTION_ID: &str = "paint-direct-region";
pub const TOOL_IDS: &[&str] = &[INDEXED_ACTION_ID, DIRECT_ACTION_ID];
pub const PAYLOAD_SCHEMA: &str = "s.stdio.bmp.command.paint-region.v1";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
pub const MAXIMUM_INTERACTIVE_ROWS: u32 = 65_536;
pub const CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintIndexedRegionCommand {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub palette_index: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintDirectRegionCommand {
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
    let value = argument(args, name).ok_or_else(|| fault("stdio.bmp.paint-region.missing-argument", format!("Missing {name}")))?;
    let semio_framework_value::DslValue::Number(number) = value else {
        return Err(fault("stdio.bmp.paint-region.invalid-argument", format!("{name} must be an integer")));
    };
    let value = number.as_u64().ok_or_else(|| fault("stdio.bmp.paint-region.invalid-argument", format!("{name} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("stdio.bmp.paint-region.invalid-argument", format!("{name} exceeds {maximum}")))
}

fn region(args: Option<&semio_framework_value::DslValue>) -> Result<BmpRegion, Fault> {
    let width = integer(args, "width", u32::MAX.into())?;
    let height = integer(args, "height", u32::MAX.into())?;
    if width == 0 || height == 0 {
        return Err(fault("stdio.bmp.paint-region.empty", "Paint region width and height must be positive"));
    }
    if height > u64::from(MAXIMUM_INTERACTIVE_ROWS) {
        return Err(fault("stdio.bmp.paint-region.too-many-rows", format!("Paint region height exceeds {MAXIMUM_INTERACTIVE_ROWS}")));
    }
    Ok(BmpRegion { x: integer(args, "x", u32::MAX.into())? as u32, y: integer(args, "y", u32::MAX.into())? as u32, width: width as u32, height: height as u32 })
}

impl PaintIndexedRegionCommand {
    pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
        let region = region(args)?;
        Ok(Self { x: region.x, y: region.y, width: region.width, height: region.height, palette_index: integer(args, "paletteIndex", u8::MAX.into())? as u8 })
    }

    fn region(self) -> BmpRegion {
        BmpRegion { x: self.x, y: self.y, width: self.width, height: self.height }
    }
}

impl PaintDirectRegionCommand {
    pub fn from_action(args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
        let region = region(args)?;
        Ok(Self {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
            red: integer(args, "red", u8::MAX.into())? as u8,
            green: integer(args, "green", u8::MAX.into())? as u8,
            blue: integer(args, "blue", u8::MAX.into())? as u8,
            alpha: integer(args, "alpha", u8::MAX.into())? as u8,
        })
    }

    fn region(self) -> BmpRegion {
        BmpRegion { x: self.x, y: self.y, width: self.width, height: self.height }
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

fn region_args() -> Vec<ActionArgDef> {
    vec![
        integer_arg("x", "Left (x)", "Links (x)", 0.0, u32::MAX as f64, 0),
        integer_arg("y", "Top (y)", "Oben (y)", 0.0, u32::MAX as f64, 0),
        integer_arg("width", "Width", "Breite", 1.0, u32::MAX as f64, 1),
        integer_arg("height", "Height", "Höhe", 1.0, f64::from(MAXIMUM_INTERACTIVE_ROWS), 1),
    ]
}

pub fn indexed_action() -> ActionDefinition {
    let mut args = region_args();
    args.push(integer_arg("paletteIndex", "Palette index", "Palettenindex", 0.0, 255.0, 0));
    ActionDefinition::bounded_catalog(INDEXED_ACTION_ID, LocalizedLabel::native("Paint Indexed Region", "Indexbereich malen"), ActionKind::Mutation).with_args(args)
}

pub fn direct_action() -> ActionDefinition {
    let mut args = region_args();
    args.extend([integer_arg("red", "Red", "Rot", 0.0, 255.0, 0), integer_arg("green", "Green", "Grün", 0.0, 255.0, 0), integer_arg("blue", "Blue", "Blau", 0.0, 255.0, 0), integer_arg("alpha", "Alpha", "Alpha", 0.0, 255.0, 255)]);
    ActionDefinition::bounded_catalog(DIRECT_ACTION_ID, LocalizedLabel::native("Paint Direct Region", "Direktfarbbereich malen"), ActionKind::Mutation).with_args(args)
}

fn checked_bounds(layout: &crate::schema::snapshot::BmpImage, region: BmpRegion) -> Result<(), Fault> {
    if region.width == 0 || region.height == 0 {
        return Err(fault("stdio.bmp.paint-region.empty", "Paint region width and height must be positive"));
    }
    if region.height > MAXIMUM_INTERACTIVE_ROWS {
        return Err(fault("stdio.bmp.paint-region.too-many-rows", format!("Paint region height exceeds {MAXIMUM_INTERACTIVE_ROWS}")));
    }
    let end_x = region.x.checked_add(region.width).ok_or_else(|| fault("stdio.bmp.paint-region.bounds-overflow", "Paint region x extent overflows"))?;
    let end_y = region.y.checked_add(region.height).ok_or_else(|| fault("stdio.bmp.paint-region.bounds-overflow", "Paint region y extent overflows"))?;
    if end_x > layout.width || end_y > layout.height {
        return Err(fault("stdio.bmp.paint-region.out-of-bounds", format!("Paint region exceeds {}×{} bitmap", layout.width, layout.height)));
    }
    Ok(())
}

fn validate(command: &BmpNativeEditCommand, snapshot: &crate::schema::snapshot::BmpSnapshot) -> Result<usize, Fault> {
    snapshot.image.validate_header().map_err(|error| fault("stdio.bmp.paint-region.invalid-snapshot", error))?;
    let layout = &snapshot.image;
    match command {
        BmpNativeEditCommand::PaintIndexedRegion(command) => {
            if !layout.profile.is_indexed() {
                return Err(fault("stdio.bmp.paint-region.profile-mismatch", "Indexed paint requires a 1-, 4-, or 8-bit indexed BMP"));
            }
            if usize::from(command.palette_index) >= layout.palette.len() || usize::from(command.palette_index) >= (1usize << layout.profile.bits_per_pixel()) {
                return Err(fault("stdio.bmp.paint-region.palette-index", format!("Palette index {} is outside the checked palette", command.palette_index)));
            }
            checked_bounds(&layout, command.region())?;
            Ok(command.height as usize)
        }
        BmpNativeEditCommand::PaintDirectRegion(command) => {
            if !layout.profile.is_direct() {
                return Err(fault("stdio.bmp.paint-region.profile-mismatch", "Direct paint requires a direct-color BMP"));
            }
            checked_bounds(&layout, command.region())?;
            Ok(command.height as usize)
        }
        BmpNativeEditCommand::SetActiveExample { .. } => Err(fault("stdio.bmp.paint-region.route-mismatch", "Paint work received another command")),
    }
}

pub struct PaintRegionWork {
    tool_id: &'static str,
    operation: Option<BmpPaintWorkOperation<'static>>,
    complete: bool,
    closing: bool,
}

impl PaintRegionWork {
    pub fn new(tool_id: &'static str) -> Self {
        Self { tool_id, operation: None, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<BmpEditor>> for PaintRegionWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &BmpEditCommand,
        snapshot: &crate::schema::snapshot::BmpSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BmpEditor>>>,
    ) -> Option<usize> {
        let BmpEditCommand::Native(command) = command else { return None };
        let id = match command {
            BmpNativeEditCommand::PaintIndexedRegion(_) => INDEXED_ACTION_ID,
            BmpNativeEditCommand::PaintDirectRegion(_) => DIRECT_ACTION_ID,
            BmpNativeEditCommand::SetActiveExample { .. } => return None,
        };
        if id != self.tool_id || validate(command, snapshot).is_err() {
            return None;
        }
        CAPACITY.rows_for_items(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BmpEditor>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BmpEditor>>, Fault> {
        if self.closing || self.complete {return Err(fault("stdio.bmp.paint-region.work-closed", "Paint work is already closed"));}
        let BmpEditCommand::Native(command)=input.command else {return Err(fault("stdio.bmp.paint-region.route-mismatch", "Paint work received another command"));};
        let reader=input.snapshot_owner.ok_or_else(||fault("stdio.bmp.paint-region.reader", "Paint work requires an immutable snapshot reader"))?;
        if self.operation.is_none() {
            validate(command,input.snapshot)?;
            let (region,paint)=match command {
                BmpNativeEditCommand::PaintIndexedRegion(command) if self.tool_id==INDEXED_ACTION_ID=>(command.region(),BmpRetainedPaint::Indexed(command.palette_index)),
                BmpNativeEditCommand::PaintDirectRegion(command) if self.tool_id==DIRECT_ACTION_ID=>(command.region(),BmpRetainedPaint::Direct(crate::schema::snapshot::BmpColor {red:command.red,green:command.green,blue:command.blue,alpha:command.alpha})),
                _=>return Err(fault("stdio.bmp.paint-region.route-mismatch", "Paint action and command profile differ")),
            };
            self.operation=Some(BmpPaintWorkOperation::try_new_retained(std::sync::Arc::clone(reader),region,paint,512*1024*1024).map_err(|error|fault("stdio.bmp.paint-region.prepare",error))?);
        }
        let operation=self.operation.as_mut().expect("paint operation prepared");
        if !operation.retained_reader_matches(reader) {return Err(fault("stdio.bmp.paint-region.reader-drift", "Paint source reader changed"));}
        match operation.advance(cx).map_err(|error|fault("stdio.bmp.paint-region.work",error))? {
            BmpPaintWorkStep::Yield {painting:false,..}=>Ok(ArtifactCommandWorkStep::Progress {stage:"bmp-paint-region-copy",preview:br#"{"en":"Copying owned bitmap samples","de":"Eigene Bitmap-Abtastwerte werden kopiert"}"#}),
            BmpPaintWorkStep::Yield {painting:true,..}=>Ok(ArtifactCommandWorkStep::Progress {stage:"bmp-paint-region-paint",preview:br#"{"en":"Painting owned bitmap samples","de":"Eigene Bitmap-Abtastwerte werden gemalt"}"#}),
            BmpPaintWorkStep::Cancelled=>Err(fault("stdio.bmp.paint-region.cancelled", "Paint work was cancelled")),
            BmpPaintWorkStep::Complete=>{let snapshot=operation.take_result().map_err(|error|fault("stdio.bmp.paint-region.result",error))?;self.complete=true;Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![BmpMutation::SetSnapshot(SetSnapshot {snapshot})])))},
        }
    }
    fn begin_close(&mut self) {self.closing=true;if let Some(operation)=&mut self.operation {operation.begin_close();}}
    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->InteractiveJobCloseStep {
        if !self.closing {return InteractiveJobCloseStep::Blocked;}
        let Some(operation)=&mut self.operation else {return InteractiveJobCloseStep::Complete};
        let step=operation.close_step(maximum_items,maximum_bytes);if step==InteractiveJobCloseStep::Complete {self.operation=None;}step
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.operation.is_none()
    }
}
