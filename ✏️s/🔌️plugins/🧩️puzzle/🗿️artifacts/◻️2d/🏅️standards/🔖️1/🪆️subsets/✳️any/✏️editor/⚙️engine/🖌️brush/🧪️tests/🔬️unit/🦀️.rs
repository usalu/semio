use semio_framework_os_infinite::board_set_camera;
mod tests {
    use crate::editor::puzzle2d::engine::board_host::testkit::*;
    use crate::editor::puzzle2d::engine::canvas::Point;
    use crate::editor::puzzle2d::engine::handle_position_on_circle;
    use crate::editor::puzzle2d::engine::BoardHost;
    use crate::editor::puzzle2d::engine::HandleDescriptor;
    use semio_framework_os_infinite::board::NodeDescriptor;
    use crate::editor::puzzle2d::engine::SceneDescriptor;
    use crate::editor::puzzle2d::engine::BoardFillCaptureStep;
    use crate::editor::puzzle2d::engine::BoardFillCheckpoint;
    use crate::editor::puzzle2d::engine::BoardFillCommitCandidate;
    use crate::editor::puzzle2d::engine::BoardFillJob;
    use crate::editor::puzzle2d::engine::BoardFillPlacement;
    use semio_framework_job::InteractiveJob;
    use semio_framework_job::InteractiveJobCloseStep;
    use semio_framework_job::InteractiveStage;
    use semio_framework_job::JobOutcomeView;
    use semio_framework_job::Operation;
    use semio_framework_job::RetainedCloneGrant;
    use semio_framework_job::RetainedCloneProgress;
    use semio_framework_job::RetainedCloneStep;
    use semio_framework_job::RetirementDemand;
    use semio_framework_job::StepBudget;
use semio_framework_job_worker::WorkerJobPoll;
    use semio_framework_job_worker_authority::{BatchDriveConfig,WorkerJobCloseStep};
    use semio_s_plugin_puzzle_jobs::testing::{OriginalFixture,fixture_stock};
    use semio_framework_pack_json::json;

    include!("🧪️fill-fixtures-and-mounted-session-helpers/🦀️.rs");
    include!("🖌️brush-slot-laws/🦀️.rs");
    include!("🧱️fill-frontier-checkpoint-and-job-laws/🦀️.rs");
    include!("🔎️candidate-acceptance-preview-and-proximity/🦀️.rs");
    include!("🚪️fill-base-core-tambour-and-pointer-slot-laws/🦀️.rs");
    include!("📚️kind-catalog-and-free-handle-laws/🦀️.rs");
}
