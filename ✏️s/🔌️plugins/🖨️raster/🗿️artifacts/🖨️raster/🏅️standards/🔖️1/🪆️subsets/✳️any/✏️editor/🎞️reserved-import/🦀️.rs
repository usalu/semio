//! 🎞️ Raster editor reserved import: the one resumable media importer with its action machine and interactive job.

use super::*;
use semio_framework_job::InteractiveJob;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_job_publication::{RetainedJobPublication, JobPublicationKind};
use semio_s_artifact_raster_raster_mutations::RasterMutation;
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_framework_job::{InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, StepContext};
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaPayload;

//#region 🎞️ReservedImport
pub(super) const RASTER_IMPORT_TOOL_ID: &str = "import-media";

const RASTER_IMPORT_PORT: &str = "image:in";

/// 🎞️ The ONE concrete resumable importer this app owns. The framework registers the reserved
/// `import-media` factory for every app but never a concrete job, so `dispatch_import_media` →
/// `build_artifact_reserved_media_job` fails closed with `interactive-job.missing-reserved-builder`
/// until the app hands one back from `build_reserved_tool_job` — `ArtifactApp::import_media`'s
/// unbounded one-shot seam is no longer on any live route. Two bounded steps: decode the incoming
/// base64 PNG into this artifact's own `(asset_id, asset, layer)` triple, then publish the two real
/// semantic mutations (`add-layer-asset` then `create-layer`, in dependency order) through the
/// completion authority. The decode itself is `crate::standards::v1::subsets::any::io::raster_image_layer_and_asset` — the SAME
/// function the pure seam used — so the import's meaning lives in one place.
pub(super) struct RasterImportJob {
    pub(super) port: Option<String>,
    media_json: Option<String>,
    pub(super) snapshot: Option<std::sync::Arc<RasterSnapshot>>,
    pub(super) mutations: Option<Vec<RasterMutation>>,
    pub(super) decoded: bool,
    pub(super) completed: bool,
    pub(super) closing: bool,
    pub(super) completion: Option<ArtifactToolCompletion<EditorApp<RasterPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<RasterPlayApp>>>,
    pub(super) publication: RetainedJobPublication,
    pub(super) publishing: Option<JobPublicationKind>,
    pub(super) source: Vec<u8>,
    pub(super) delivered: bool,
    pub(super) active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
}

enum RasterImportAction {
    Publish,
    Complete,
}

impl RasterImportJob {
    pub(super) fn new(request: ArtifactReservedToolJobRequest<EditorApp<RasterPlayApp>>, port: String, media: Media) -> Self {
        let media_json = match media.payload {
            MediaPayload::Structured { json, .. } => Some(json),
            MediaPayload::Binary { .. } | MediaPayload::Intrinsic { .. } => None,
        };
        Self {
            port: Some(port),
            media_json,
            snapshot: Some(request.snapshot),
            mutations: Some(Vec::new()),
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
            publication: RetainedJobPublication::new(),
            publishing: None,
            source: Vec::new(),
            delivered: false,
            active: None,
        }
    }

    /// 🧯️ Stages one bounded fault description for the next publication turns.
    pub(super) fn stage_fault(&mut self, detail: &str) {
        let bytes = detail.as_bytes();
        self.source = bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec();
        self.publishing = Some(JobPublicationKind::Fault);
    }

    pub(super) fn decode(&mut self) -> bool {
        if self.port.as_deref() != Some(RASTER_IMPORT_PORT) {
            self.stage_fault("raster import only implements image:in");
            return false;
        }
        let decoded = {
            let (Some(media_json), Some(snapshot)) = (self.media_json.as_ref(), self.snapshot.as_ref()) else {
                self.stage_fault("raster image:in only accepts a Structured (base64 PNG) payload over its snapshot authority");
                return false;
            };
            let index = snapshot.layers.len();
            semio_s_artifact_raster_raster_io::raster_image_layer_and_asset(media_json).map(|(asset_id, asset, layer)| {
                vec![
                    RasterMutation::AddLayerAsset(semio_s_artifact_raster_raster_mutations_pixels::add_layer_asset::mutation::AddLayerAsset { asset_id, asset }),
                    RasterMutation::CreateLayer(semio_s_artifact_raster_raster_mutations_layers::create_layer::mutation::CreateLayer { parent_id: None, index, layer: Box::new(layer) }),
                ]
            })
        };
        match decoded {
            Ok(mutations) => {
                self.mutations = Some(mutations);
                self.decoded = true;
                true
            }
            Err(error) => {
                self.stage_fault(&error);
                false
            }
        }
    }

    /// ⏭️ Runs the next bounded unit of work and reports what the step must publish.
    pub(super) fn advance(&mut self) -> RasterImportAction {
        if self.pending_completion_rejection.is_some() {
            self.stage_fault("raster import completion remains rejected");
            return RasterImportAction::Publish;
        }
        if !self.decoded {
            if !self.decode() {
                return RasterImportAction::Publish;
            }
            self.source = vec![1];
            self.publishing = Some(JobPublicationKind::Checkpoint { applied_progress: 1 });
            return RasterImportAction::Publish;
        }
        if !self.completed {
            let mutations = self.mutations.take().unwrap_or_default();
            let Some(completion) = self.completion.as_ref() else {
                self.stage_fault("raster import lost its completion authority");
                return RasterImportAction::Publish;
            };
            if !completion.has_mounted_consumer() {
                self.stage_fault("raster import completion consumer is absent");
                return RasterImportAction::Publish;
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                self.stage_fault(&message);
                return RasterImportAction::Publish;
            }
            self.completed = true;
        }
        RasterImportAction::Complete
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<RasterRetirementDemand, semio_framework_value::ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if let Some(active) = self.active.as_ref() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if self.pending_completion_rejection.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_completion_rejection)?);
        }
        if self.mutations.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutations)?);
        }
        if self.media_json.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media_json)?);
        }
        if self.port.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.port)?);
        }
        if self.snapshot.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.snapshot)?);
        }
        if self.completion.is_some() {
            return raster_preparation_nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion)?);
        }
        Ok(RasterRetirementDemand { depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }
}

impl InteractiveJob for RasterImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if self.delivered {
            let step = self.publication.close_step(cx.retained_grant())?;
            cx.consume_retained(step.progress())?;
            if matches!(step, RetainedCloneStep::Complete(_)) {
                self.delivered = false;
            }
            return Ok(None);
        }
        if cx.is_cancelled() {
            return JobOutcomeBorrow::admit_cancelled(cx);
        }
        if self.publishing.is_none() {
            cx.set_stage(if self.decoded { "raster-import-publish" } else { "raster-import-decode" });
            if matches!(self.advance(), RasterImportAction::Complete) {
                return JobOutcomeBorrow::admit_complete(cx, None, None);
            }
            cx.consume_fuel(1);
            return Ok(None);
        }
        let Some(kind) = self.publishing else { return Ok(None) };
        let result = self.publication.advance_from_source(kind, &self.source, cx)?;
        if result.is_some() {
            self.delivered = true;
            if matches!(kind, JobPublicationKind::Checkpoint { .. }) {
                self.publishing = None;
            }
        }
        Ok(result)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete if self.completed => descriptor.complete(None, None),
            _ => self.publication.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        let refused = |error: semio_framework_value::ValueError| InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() };
        let demand = match self.close_demands(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return refused(error),
        };
        if self.terminal_is_empty() {
            return InteractiveJobCloseStep::Complete { progress: Default::default() };
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || grant.maximum_depth < demand.depth {
            return InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
        macro_rules! owned {
            ($slot:expr) => {
                if $slot.is_some() {
                    return match store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut $slot, &mut self.active, child) {
                        Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                        Err(error) => refused(error),
                    };
                }
            };
        }
        if !self.publication.terminal_is_empty() {
            return match self.publication.close_step(grant) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        if self.delivered {
            self.delivered = false;
            return InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } };
        }
        if self.active.is_some() {
            return match store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child) {
                Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
                Err(error) => refused(error),
            };
        }
        owned!(self.pending_completion_rejection);
        owned!(self.mutations);
        owned!(self.media_json);
        owned!(self.port);
        owned!(self.snapshot);
        owned!(self.completion);
        InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.copy_bytes)
    }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(body)?.capacity_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.release_bytes)
    }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && !self.delivered
            && self.publication.terminal_is_empty()
            && self.active.is_none()
            && self.port.is_none()
            && self.media_json.is_none()
            && self.snapshot.is_none()
            && self.mutations.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}

impl ArtifactReservedJob for RasterImportJob {}
//#endregion 🎞️ReservedImport
