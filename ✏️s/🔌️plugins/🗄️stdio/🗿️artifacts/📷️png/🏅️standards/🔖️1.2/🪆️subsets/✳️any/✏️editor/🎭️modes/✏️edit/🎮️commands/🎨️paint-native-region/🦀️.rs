//! 🎨️ Checked, cancellable native PNG sample painting.

use super::{PngEditCommand, PngEditor, PngNativeEditCommand};
use crate::schema::operations::{validate_native_paint_target, owned_validation::PngOwnedValidationWork};
use crate::schema::snapshot::{PngNativePaint,PngNativeProfile,PngRegion};
use crate::schema::mutations::{PaintNativeSamplesMutation, PngMutation};
use crate::schema::snapshot::PngSnapshot;
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionKind, ArgSchema, EditorApp, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_ui_locale::LocalizedLabel;

pub const INDEXED_ACTION_ID: &str = "paint-index-region";
pub const GRAYSCALE_ACTION_ID: &str = "paint-grayscale-region";
pub const GRAYSCALE_ALPHA_ACTION_ID: &str = "paint-grayscale-alpha-region";
pub const RGB_ACTION_ID: &str = "paint-rgb-native-region";
pub const RGBA_ACTION_ID: &str = "paint-rgba-native-region";
pub const ACTION_IDS: &[&str] = &[INDEXED_ACTION_ID, GRAYSCALE_ACTION_ID, GRAYSCALE_ALPHA_ACTION_ID, RGB_ACTION_ID, RGBA_ACTION_ID];
pub const PAYLOAD_SCHEMA: &str = "s.stdio.png.command.paint-native-region.v1";
pub const MAXIMUM_RAW_BYTES: usize = 8_192;
pub const CAPACITY: ArtifactRetainedWorkCapacity = ArtifactRetainedWorkCapacity::for_invertible_items(128);

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaintNativeRegion {
    pub region: PngRegion,
    pub paint: PngNativePaint,
}

fn fault(code: &'static str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

fn argument<'a>(args: Option<&'a semio_framework_value::DslValue>, name: &str) -> Option<&'a semio_framework_value::DslValue> {
    let Some(semio_framework_value::DslValue::Object(fields)) = args else { return None };
    fields.iter().find(|(key, _)| key == name).map(|(_, value)| value)
}

fn integer(args: Option<&semio_framework_value::DslValue>, name: &'static str, maximum: u64) -> Result<u64, Fault> {
    let value = argument(args, name).ok_or_else(|| fault("stdio.png.native-region.missing-argument", format!("Missing {name}")))?;
    let semio_framework_value::DslValue::Number(number) = value else { return Err(fault("stdio.png.native-region.invalid-argument", format!("{name} must be an integer"))); };
    let value = number.as_u64().ok_or_else(|| fault("stdio.png.native-region.invalid-argument", format!("{name} must be a non-negative integer")))?;
    (value <= maximum).then_some(value).ok_or_else(|| fault("stdio.png.native-region.invalid-argument", format!("{name} exceeds {maximum}")))
}

impl PaintNativeRegion {
    pub fn from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self, Fault> {
        let width = integer(args, "width", u32::MAX.into())? as u32;
        let height = integer(args, "height", u32::MAX.into())? as u32;
        if width == 0 || height == 0 { return Err(fault("stdio.png.native-region.empty", "Native sample region must be nonempty")); }
        let region = PngRegion {
            x: integer(args, "x", u32::MAX.into())? as u32,
            y: integer(args, "y", u32::MAX.into())? as u32,
            width,
            height,
        };
        let sample = |name| integer(args, name, u16::MAX.into()).map(|value| value as u16);
        let paint = match action {
            INDEXED_ACTION_ID => PngNativePaint::indexed(sample("index")?),
            GRAYSCALE_ACTION_ID => PngNativePaint::grayscale(sample("gray")?),
            GRAYSCALE_ALPHA_ACTION_ID => PngNativePaint::grayscale_alpha(sample("gray")?, sample("alpha")?),
            RGB_ACTION_ID => PngNativePaint::rgb(sample("red")?, sample("green")?, sample("blue")?),
            RGBA_ACTION_ID => PngNativePaint::rgba(sample("red")?, sample("green")?, sample("blue")?, sample("alpha")?),
            _ => return Err(fault("stdio.png.native-region.unknown-action", format!("Unknown native PNG paint action {action}"))),
        };
        Ok(Self { region, paint })
    }

    fn validate(&self, snapshot: &PngSnapshot) -> Result<usize, Fault> {
        validate_native_paint_target(snapshot, self.region, self.paint).map_err(|message| fault("stdio.png.native-region.invalid", message))
    }
}

fn integer_arg(id: &'static str, en: &'static str, de: &'static str, minimum: f64, maximum: f64, default: u32) -> ActionArgDef {
    let mut argument = ActionArgDef::number(id, LocalizedLabel::native(en, de)).required().default_value(&default);
    if let ArgSchema::Number { min, max, step, integer, .. } = &mut argument.schema {
        *min = Some(minimum); *max = Some(maximum); *step = Some(1.0); *integer = true;
    }
    argument
}

fn region_args() -> Vec<ActionArgDef> {
    vec![
        integer_arg("x", "Left (x)", "Links (x)", 0.0, u32::MAX as f64, 0),
        integer_arg("y", "Top (y)", "Oben (y)", 0.0, u32::MAX as f64, 0),
        integer_arg("width", "Width", "Breite", 1.0, u32::MAX as f64, 1),
        integer_arg("height", "Height", "Höhe", 1.0, u32::MAX as f64, 1),
    ]
}

fn sample_arg(id: &'static str, en: &'static str, de: &'static str, default: u32) -> ActionArgDef {
    integer_arg(id, en, de, 0.0, u16::MAX as f64, default)
}

fn action(id: &'static str, en: &'static str, de: &'static str, samples: Vec<ActionArgDef>) -> ActionDefinition {
    let mut args = region_args(); args.extend(samples);
    ActionDefinition::bounded_catalog(id, LocalizedLabel::native(en, de), ActionKind::Mutation).with_args(args)
}

pub fn actions() -> Vec<ActionDefinition> {
    vec![
        action(INDEXED_ACTION_ID, "Paint Palette Index Region", "Palettenindexbereich malen", vec![sample_arg("index", "Palette index", "Palettenindex", 0)]),
        action(GRAYSCALE_ACTION_ID, "Paint Grayscale Region", "Graustufenbereich malen", vec![sample_arg("gray", "Gray sample", "Grauwert", 0)]),
        action(GRAYSCALE_ALPHA_ACTION_ID, "Paint Grayscale Alpha Region", "Graustufen-Alpha-Bereich malen", vec![sample_arg("gray", "Gray sample", "Grauwert", 0), sample_arg("alpha", "Alpha sample", "Alphawert", u8::MAX.into())]),
        action(RGB_ACTION_ID, "Paint Native RGB Region", "Nativen RGB-Bereich malen", vec![sample_arg("red", "Red sample", "Rotwert", 0), sample_arg("green", "Green sample", "Grünwert", 0), sample_arg("blue", "Blue sample", "Blauwert", 0)]),
        action(RGBA_ACTION_ID, "Paint Native RGBA Region", "Nativen RGBA-Bereich malen", vec![sample_arg("red", "Red sample", "Rotwert", 0), sample_arg("green", "Green sample", "Grünwert", 0), sample_arg("blue", "Blue sample", "Blauwert", 0), sample_arg("alpha", "Alpha sample", "Alphawert", u8::MAX.into())]),
    ]
}

pub fn action_id(profile: PngNativeProfile) -> &'static str {
    match profile {
        PngNativeProfile::Indexed => INDEXED_ACTION_ID,
        PngNativeProfile::Grayscale => GRAYSCALE_ACTION_ID,
        PngNativeProfile::GrayscaleAlpha => GRAYSCALE_ALPHA_ACTION_ID,
        PngNativeProfile::Rgb => RGB_ACTION_ID,
        PngNativeProfile::Rgba => RGBA_ACTION_ID,
    }
}

pub struct PaintNativeRegionWork {
    tool_id: &'static str,
    reader: Option<std::sync::Arc<PngSnapshot>>,
    validation: PngOwnedValidationWork,
    retirement: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    complete: bool,
    closing: bool,
}

impl PaintNativeRegionWork {
    pub fn new(tool_id: &'static str) -> Self { Self { tool_id, reader: None, validation: PngOwnedValidationWork::default(), retirement: None, complete: false, closing: false } }
}

impl ArtifactCommandWork<EditorApp<PngEditor>> for PaintNativeRegionWork {
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(&self, command: &PngEditCommand, snapshot: &PngSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<PngEditor>>>) -> Option<usize> {
        let PngEditCommand::Native(PngNativeEditCommand::PaintNativeRegion(command)) = command else { return None };
        if action_id(command.paint.profile) != self.tool_id { return None; }
        CAPACITY.rows_for_items(command.validate(snapshot).ok()?.min(CAPACITY.invertible_items()).max(1))
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<PngEditor>>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<PngEditor>>, Fault> {
        if self.closing || self.complete { return Err(fault("stdio.png.native-region.work-closed", "Native PNG paint work is already closed")); }
        let PngEditCommand::Native(PngNativeEditCommand::PaintNativeRegion(command)) = input.command else { return Err(fault("stdio.png.native-region.route-mismatch", "Native PNG paint work received another command")); };
        if action_id(command.paint.profile) != self.tool_id { return Err(fault("stdio.png.native-region.route-mismatch", "Native PNG paint action and profile differ")); }
        if cx.is_cancelled() {return Err(fault("stdio.png.native-region.cancelled", "Native PNG paint was cancelled"));}
        let reader=input.snapshot_owner.ok_or_else(||fault("stdio.png.native-region.reader", "Native paint requires an immutable snapshot reader"))?;
        if let Some(previous)=self.reader.as_ref() {
            if !std::sync::Arc::ptr_eq(previous,reader) {return Err(fault("stdio.png.native-region.reader-drift", "Native paint source reader changed"));}
        } else {
            command.validate(reader)?;
            self.reader=Some(std::sync::Arc::clone(reader));
        }
        while !cx.should_yield() {
            if cx.is_cancelled() {return Err(fault("stdio.png.native-region.cancelled", "Native PNG paint was cancelled"));}
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_depth:64, maximum_release_bytes: 4096 };
            let (done,progress)=self.validation.advance(reader,None,grant).map_err(|message|fault("stdio.png.native-region.work",message))?;
            cx.consume_fuel(progress.copied_items as u64);
            if done {
                let revision=self.validation.revision().ok_or_else(||fault("stdio.png.native-region.revision", "Native paint source revision is incomplete"))?;
                self.complete=true;
                return Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![PngMutation::PaintNativeSamples(PaintNativeSamplesMutation{revision,region:command.region,paint:command.paint})])));
            }
        }
        Ok(ArtifactCommandWorkStep::Progress{stage:"png-native-region-intent",preview:br#"{"en":"Preparing PNG paint intent","de":"PNG-Malabsicht wird vorbereitet"}"#})
    }

    fn begin_close(&mut self) {
        self.closing=true;
        if let Some(reader)=self.reader.take() {self.retirement=Some(semio_framework_value::retirement::shared_lease_retirement(reader));}
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {return InteractiveJobCloseStep::Blocked;}
        let Some(retirement)=self.retirement.as_mut() else {return InteractiveJobCloseStep::Complete;};
        match retirement.close_step(maximum_items,maximum_bytes) {
            Ok(semio_framework_value::SnapshotRetirementStep::Complete)=>{
                if !retirement.terminal_is_empty() {return InteractiveJobCloseStep::Blocked;}
                self.retirement=None;InteractiveJobCloseStep::Complete
            },
            Ok(semio_framework_value::SnapshotRetirementStep::Pending{released_items,released_bytes})=>InteractiveJobCloseStep::Pending{released_items,released_bytes},
            _=>InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {self.closing&&self.reader.is_none()&&self.retirement.is_none()}
}
