use crate::board_set_camera;
mod tests {
    use crate::editor::puzzle2d::engine::board_host::testkit::*;
    use crate::editor::puzzle2d::engine::canvas::Point;
    use crate::editor::puzzle2d::engine::{handle_position_on_circle, BoardHost, HandleDescriptor, NodeDescriptor, SceneDescriptor};
    use crate::editor::puzzle2d::engine::{BoardFillCaptureStep, BoardFillCheckpoint, BoardFillCommitCandidate, BoardFillJob, BoardFillPlacement};
    use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep,InteractiveStage,JobOutcomeView,Operation,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,StepBudget};
use semio_framework_job_worker::{WorkerJobPoll};use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams,WorkerJobAdmissionContext,WorkerJobCloseStep};
    use semio_framework_pack_json::json;

    include!("🧪️fill-fixtures-and-mounted-session-helpers/🦀️.rs");
    include!("🖌️brush-slot-laws/🦀️.rs");
    include!("🧱️fill-frontier-checkpoint-and-job-laws/🦀️.rs");
    include!("🔎️candidate-acceptance-preview-and-proximity/🦀️.rs");
    include!("🚪️fill-base-core-tambour-and-pointer-slot-laws/🦀️.rs");
    include!("📚️kind-catalog-and-free-handle-laws/🦀️.rs");
}
