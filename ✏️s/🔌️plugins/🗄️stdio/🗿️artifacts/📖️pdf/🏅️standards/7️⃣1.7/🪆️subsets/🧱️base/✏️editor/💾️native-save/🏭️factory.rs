//! 🏭️ PDF 1.7 Editor's exact owned registration for the normal native binary output.

use semio_plugin::{
    ActionDefinition, AppIo, ArtifactOwnedToolJobFactory, ArtifactPresentation, ArtifactReservedToolJob,
    ArtifactToolPublicationContract, ArtifactToolPublicationLane, LocalizedLabel, ArtifactApp,
    InteractiveJobClassification, MediaPortDirection, MediaPortSpec, PortMultiplicity, ToolExecutionContract, ArtifactOutputChunks,
};
use semio_plugin::plugin_app_close_prelude::{EXPORT_MEDIA_FILE_ACTION_ID, media_file_export_action_definition};
use semio_plugin::retained_command::__action_bus::{ToolFactoryKey, ToolJobFactory};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use crate::standards::v1_7::subsets::base::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA;
use super::producer::{FORMAT, MEDIA_TYPE, MIME, PORT, TOOL};
const PAYLOAD_SCHEMA: &str = "stdio.pdf.1.7.native-export.v1";
pub const CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(
    4_096, 4_096, 4_096, ArtifactOutputChunks::MAXIMUM_TOTAL_BYTES, 2_000, 64, 1,
);

/// 📄️ Declares a native PDF document port to the revision-bound media download shell action.
pub fn app_io() -> AppIo {
    AppIo {
        artifact_schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MEDIA_TYPE,
        ports: vec![MediaPortSpec {
            id: PORT.into(),
            label: LocalizedLabel::native("PDF document", "PDF-Dokument"),
            direction: MediaPortDirection::Out,
            media_type: MEDIA_TYPE,
            kind_id: Some(FORMAT.into()),
            required: false,
            multiplicity: PortMultiplicity::Many,
        }],
        export_formats: vec![FORMAT.into()],
        import_formats: vec![FORMAT.into()],
        artifact: ArtifactPresentation {
            id: "stdio.pdf".into(),
            name: "PDF Document".into(),
            dimension: "page".into(),
            component_kind: "document".into(),
        },
    }
}

/// 💾️ Declares the native Save on an app definition: its `artifact:native` output port ([`app_io`]) and the `exportMediaFile` action naming that port. The shell resolves Save to the revision-bound segmented export only when the action's port is a declared output port (`declaredMediaFileExportV1`); without the port it falls back to the reference route, which re-encodes the whole document and never appends to an opened file.
pub fn declare_native_save(definition: &mut semio_plugin::AppDefinition) {
    definition.io = app_io();
    definition.actions.push(export_action());
}

/// 🗂️ Keeps the shell's explicit and normal Save routes on the same native representation.
pub fn export_action() -> ActionDefinition {
    let mut action = media_file_export_action_definition(
        EXPORT_MEDIA_FILE_ACTION_ID, LocalizedLabel::native("Export PDF File", "PDF-Datei exportieren"),
        PORT, "document.pdf", MIME, FORMAT,
    );
    action.in_palette = false;
    action
}

pub struct PdfNativeExportFactory<A: ArtifactApp<Snapshot = PdfSnapshot>> {
    owner: std::marker::PhantomData<fn() -> A>,
    keys: [ToolFactoryKey; 1],
}

impl<A: ArtifactApp<Snapshot = PdfSnapshot>> PdfNativeExportFactory<A> {
    pub fn new(controller: &str) -> Self {
        Self { owner: std::marker::PhantomData, keys: [ToolFactoryKey::new(controller, TOOL)] }
    }
}

impl<A: ArtifactApp<Snapshot = PdfSnapshot>> ToolJobFactory for PdfNativeExportFactory<A> {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;

    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { CONTRACT }

    semio_plugin::retained_command::reserved_tool_job_factory_admission!();
}

impl<A: ArtifactApp<Snapshot = PdfSnapshot>> ArtifactOwnedToolJobFactory for PdfNativeExportFactory<A> {
    type Owner = A;
    const TOOL_IDS: &'static [&'static str] = &[TOOL];
    const DOCUMENT_SCHEMA: &'static str = STDIO_PDF17_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract {
            tool_id: TOOL,
            lanes: &[ArtifactToolPublicationLane::HostOnly],
        },
    ];
}
