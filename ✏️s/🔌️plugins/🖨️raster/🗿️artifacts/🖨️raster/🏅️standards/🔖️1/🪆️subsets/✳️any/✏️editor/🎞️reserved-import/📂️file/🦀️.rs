//! 📂️ Uses the same retained PNG decoder and layer candidate as the image input port.
use super::*;
use crate::RASTER_RETAINED_PAYLOAD_SCHEMA;
use crate::commands::import_png::{MAX_PAYLOAD_BYTES, base64_offset};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_plugin::{ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolPublicationContract, ArtifactToolPublicationLane};
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use std::mem::size_of;

type App = EditorApp<RasterPlayApp>;
const TOOL: &str = "importPngFile";
pub(crate) const RAW_BYTES: usize = MAX_PAYLOAD_BYTES + 65_536;
const OUTPUT_BYTES: usize = 16_777_216 * 52 + 65_536;
const WORK_ITEMS: usize = semio_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(2).work_items();
const CLOSE_PAGE: usize = semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;

pub(crate) fn proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
    RasterPngFileProofs::bounded_first_step_tool_proofs()
}

struct RasterPngFileProofs;
impl RasterPngFileProofs {
    semio_plugin::bounded_first_step_tool_proofs! {
        owner: App,
        owner_file: "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.raster.raster@1/*#editor",
        artifact_schema: "raster.document",
        factory: "RasterPngFileJobFactory",
        factory_type: RasterPngFileJobFactory,
        tools: { "importPngFile" => ToolExecutionContract::resumable(RAW_BYTES, 4096, 4096, OUTPUT_BYTES, 7500, 64, 1) }
    }
}

pub(crate) struct RasterPngFileJobFactory {
    keys: [ToolFactoryKey; 1],
}

impl RasterPngFileJobFactory {
    pub(crate) fn new(controller_id: &str) -> Self {
        Self { keys: [ToolFactoryKey::new(controller_id, TOOL)] }
    }
}

impl ToolJobFactory for RasterPngFileJobFactory {
    type Payload = ArtifactRetainedCommandPayload<App>;
    type Job = ArtifactRetainedCommandJob<App>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        RASTER_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(RAW_BYTES, 4096, 4096, OUTPUT_BYTES, 7500, 64, 1)
    }
    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: RAW_BYTES, checkpoint_resume: false);
}

impl ArtifactOwnedToolJobFactory for RasterPngFileJobFactory {
    type Owner = App;
    const TOOL_IDS: &'static [&'static str] = &[TOOL];
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_raster_raster_model::RASTER_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: TOOL, lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Child] }];
}

pub(crate) fn build(request: &mut Option<ArtifactOwnedToolJobRequest<App>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
    if !request.as_ref().is_some_and(|request| request.tool_id == TOOL && matches!(&*request.command, RasterCommand::ImportPngFile(_))) {
        return Err(import_invalid().into());
    }
    semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, RasterCommand::command_id, RAW_BYTES, WORK_ITEMS, false, |_| RasterPngFileWork::default())
}

#[derive(Default)]
struct RasterPngFileWork {
    import: RasterImportJob,
}

impl ArtifactCommandWork<App> for RasterPngFileWork {
    fn tool_id(&self) -> &'static str {
        TOOL
    }

    fn original_wire_demands(&self, _: &[u8], _: usize) -> Result<Option<RetirementDemand>, ValueError> {
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "Raster PNG import requires the original staged file command"))
    }

    fn extent(&self, command: &RasterCommand, _: &RasterSnapshot, _: &protocol::InteractionState, _: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<App>>) -> Option<usize> {
        let RasterCommand::ImportPngFile(file) = command else { return None };
        base64_offset(&file.payload).map(|_| WORK_ITEMS)
    }

    fn receive_original_command(&mut self, command: &mut RasterCommand, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        let RasterCommand::ImportPngFile(file) = command else { return Err(import_invalid().into()) };
        let offset = base64_offset(&file.payload).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "Raster import requires a bounded PNG data URL"))?;
        self.import.ingress.begin_at(&mut file.payload, offset, cx)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, App>, cx: &mut StepContext<'_>) -> Result<ArtifactCommandWorkStep<App>, OriginalTurnFailure> {
        if !self.import.advance_candidate(None, input.snapshot.layers.len(), cx)? {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "raster-import-decode", preview: b"" });
        }
        let bytes = size_of::<Option<Vec<RasterMutation>>>() + size_of::<Emit<RasterMutation, RasterConfigMutation>>();
        let mut emit = None;
        admit_original_construction_settled(cx, &mut self.import.construction, &mut emit, OriginalConstructionDemand { items: 1, copy_bytes: bytes, capacity_bytes: 0, release_bytes: 0, depth: 1 }, 1, |_, _| {
            let result = self.import.mutations.take().ok_or_else(import_invalid).map(|artifact_mutations| {
                let emit = Emit { artifact_mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() };
                (emit, semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })
            });
            RetainedWorkOutcome { result, work: 1 }
        })?;
        Ok(emit.map_or(ArtifactCommandWorkStep::Pending { stage: "raster-import-publish", preview: b"" }, ArtifactCommandWorkStep::Complete))
    }

    fn begin_close(&mut self) {
        self.import.begin_close();
    }
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        self.import.close_step(cx)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.import.demands(CLOSE_PAGE)?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, bytes: usize) -> Result<usize, ValueError> {
        Ok(self.import.demands(bytes)?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.import.demands(CLOSE_PAGE)?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.import.demands(CLOSE_PAGE)?.depth)
    }
    fn terminal_is_empty(&self) -> bool {
        self.import.terminal_is_empty()
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
}
