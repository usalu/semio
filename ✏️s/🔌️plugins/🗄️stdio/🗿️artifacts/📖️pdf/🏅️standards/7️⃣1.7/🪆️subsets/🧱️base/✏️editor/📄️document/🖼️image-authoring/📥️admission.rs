//! 📂 Uses the existing shell's bounded local picker for PNG authoring.

use super::{ImagePlacement, MAX_DATA_URL, PNG_PREFIX};
use semio_plugin::Fault;
use semio_plugin::__value::DslValue;

pub const IMPORT: &str = "import-local-image";
pub const INSERT: &str = "insert-image";
pub const REPLACE: &str = "replace-image";
const BINARY_PREFIX: &str = "data:application/octet-stream;base64,";
pub const INSERT_OPERATOR: u32 = u32::MAX;
pub const WIRE_BYTES: usize = 48 * 1024;
pub const CONTRACT: semio_plugin::ToolExecutionContract =
    semio_plugin::ToolExecutionContract::resumable(WIRE_BYTES, 4096, 16384, 16 * 1024 * 1024, 7500, 1, 1);

fn fault(message: &str) -> Fault {
    Fault::new(semio_plugin::FaultOrigin::App, semio_plugin::FaultCode::new("stdio.pdf.image-import"), message)
}

fn number(args: Option<&DslValue>, field: &str) -> Result<f64, Fault> {
    semio_s_artifact_stdio_contract::window_kit_text_argument(args, &[field], "")
        .parse::<f64>().ok().filter(|value| value.is_finite()).ok_or_else(|| fault("Invalid image placement coordinates"))
}

fn index(args: Option<&DslValue>, field: &str) -> Result<u32, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_index_argument(args, field)
}

fn text(args: Option<&DslValue>, field: &str) -> Result<String, Fault> {
    semio_s_artifact_stdio_contract::window_kit_required_text_argument(args, field)
}

pub fn admits_png_action(action: &str) -> bool {
    matches!(action, INSERT | REPLACE | IMPORT)
}

/// 📌 Validated target fields used by both picker and completion commands.
pub struct PngTarget {
    pub page: u32,
    pub operator: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl PngTarget {
    pub fn placement(&self) -> ImagePlacement {
        if self.operator == INSERT_OPERATOR {
            ImagePlacement::Insert {
                page: self.page as usize,
                matrix: [self.width, 0.0, 0.0, self.height, self.x, self.y],
            }
        } else {
            ImagePlacement::Replace { page: self.page as usize, operator: self.operator as usize }
        }
    }
}

pub enum PngInvocation {
    Pick { target: PngTarget, guard: String },
    Import { target: PngTarget, revision: String, payload: String },
}

pub fn png_invocation(action: &str, args: Option<&DslValue>) -> Result<PngInvocation, Fault> {
    let page = index(args, "page")?;
    if action == REPLACE {
        let operator = index(args, "operator")?;
        if operator == INSERT_OPERATOR { return Err(fault("Invalid PDF image operator")); }
        return Ok(PngInvocation::Pick { target: PngTarget { page, operator, x: 0.0, y: 0.0, width: 0.0, height: 0.0 }, guard: text(args, "text")? });
    }
    let geometry = || Ok::<_, Fault>((number(args, "x")?, number(args, "y")?, number(args, "width")?, number(args, "height")?));
    let (x, y, width, height) = geometry()?;
    if action == INSERT {
        if width <= 0.0 || height <= 0.0 { return Err(fault("Image width and height must be positive")); }
        return Ok(PngInvocation::Pick { target: PngTarget { page, operator: INSERT_OPERATOR, x, y, width, height }, guard: String::new() });
    }
    let operator = index(args, "operator")?;
    let revision = text(args, "revision")?;
    let wire = text(args, "payload")?;
    let encoded = wire.strip_prefix(BINARY_PREFIX).or_else(|| wire.strip_prefix(PNG_PREFIX))
        .ok_or_else(|| fault("Choose a PNG within the local image import limits"))?;
    if revision.len() != 64 || encoded.is_empty() || encoded.len() > MAX_DATA_URL - PNG_PREFIX.len() {
        return Err(fault("Choose a PNG within the local image import limits"));
    }
    let payload = format!("{PNG_PREFIX}{encoded}");
    Ok(PngInvocation::Import { target: PngTarget { page, operator, x, y, width, height }, revision, payload })
}


pub fn png_actions() -> [semio_plugin::ActionDefinition; 3] {
    use semio_plugin::{ActionArgDef, ActionDefinition, ActionKind, InteractiveJobClassification};
    use semio_plugin::LocalizedLabel;
    let label = LocalizedLabel::native;
    let page = || ActionArgDef::number("page", label("Page", "Seite")).required();
    let operator = || ActionArgDef::number("operator", label("Image placement", "Bildposition")).required();
    let geometry = || [
        ActionArgDef::number("x", label("X", "X")).required(),
        ActionArgDef::number("y", label("Y", "Y")).required(),
        ActionArgDef::number("width", label("Width", "Breite")).required(),
        ActionArgDef::number("height", label("Height", "Höhe")).required(),
    ];
    let action = |id, en, de, args| {
        let mut definition = ActionDefinition::bounded_catalog(id, label(en, de), ActionKind::Mutation).with_args(args);
        definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
        definition
    };
    [
        action(INSERT, "Insert PNG Image", "PNG-Bild einfügen", [vec![page()], geometry().to_vec()].concat()),
        action(REPLACE, "Replace Image with PNG", "Bild durch PNG ersetzen", vec![page(), operator(), ActionArgDef::text("text", label("Revision", "Revision")).min_length(16).required()]).in_palette(false),
        action(IMPORT, "Import PNG Samples", "PNG-Bildpunkte importieren", [
            vec![page(), operator(),
                ActionArgDef::text("revision", label("Revision", "Revision")).min_length(64).required(),
                ActionArgDef::text("payload", label("PNG data", "PNG-Daten")).min_length(1).required(),
                ActionArgDef::text("name", label("File name", "Dateiname")).min_length(1),
            ],
            geometry().to_vec(),
        ].concat()).in_palette(false),
    ]
}

/// 🏭️ Declares the concrete factory joined to the PDF editor's retained PNG job proof.
#[macro_export]
macro_rules! pdf_local_image_import_factory {
    ($factory:ident, $app:ty) => {
        pub struct $factory {
            keys: [semio_plugin::retained_command::__action_bus::ToolFactoryKey; 1],
        }

        impl $factory {
            pub fn new(controller: &str) -> Self {
                use semio_plugin::retained_command::__action_bus::ToolFactoryKey;
                Self { keys: [ToolFactoryKey::new(controller, $crate::IMPORT)] }
            }
        }

        impl semio_plugin::retained_command::__action_bus::ToolJobFactory for $factory {
            type Payload = semio_plugin::retained_command::ArtifactRetainedCommandPayload<$app>;
            type Job = semio_plugin::retained_command::ArtifactRetainedCommandJob<$app>;

            fn keys(&self) -> &[semio_plugin::retained_command::__action_bus::ToolFactoryKey] { &self.keys }
            fn payload_schema_id(&self) -> &str { "stdio.pdf.local-png-import.v1" }
            fn classification(&self) -> semio_plugin::InteractiveJobClassification { semio_plugin::InteractiveJobClassification::Migrated }
            fn execution_contract(&self) -> semio_plugin::ToolExecutionContract { $crate::CONTRACT }

            semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: $crate::WIRE_BYTES, checkpoint_resume: false);
        }

        impl semio_plugin::ArtifactOwnedToolJobFactory for $factory {
            type Owner = $app;
            const TOOL_IDS: &'static [&'static str] = &[$crate::IMPORT];
            const DOCUMENT_SCHEMA: &'static str = <$app as semio_plugin::ArtifactApp>::DOCUMENT_SCHEMA;
            const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = &[
                semio_plugin::ArtifactToolPublicationContract { tool_id: $crate::IMPORT, lanes: &[semio_plugin::ArtifactToolPublicationLane::Artifact] },
            ];
        }
    };
}
