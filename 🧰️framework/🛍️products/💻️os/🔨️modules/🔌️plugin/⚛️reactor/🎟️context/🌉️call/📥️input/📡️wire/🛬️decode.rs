//! 🛬️ The concrete parse and close steps of an original actor input source.
use semio_framework_actor::OriginalJobSource;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_job_native_decode::OriginalNativeDecodeOutcome;
use semio_framework_pack_json_output_receiving::{OriginalJsonOutputClose, OriginalJsonOutputSource};
use semio_framework_value::{RetainedCloneStep, retirement::controlled::ControlledRetirement};
use semio_plugin_abi::owned_abi::ActorInput;

pub fn close(source: &mut OriginalJsonOutputSource<ActorInput>, cx: &mut StepContext<'_>) -> Result<OriginalJsonOutputClose, OriginalTurnFailure> {
    source.close_step(cx)
}

pub fn close_actor(source: &mut OriginalJsonOutputSource<ActorInput>, owner: &mut ControlledRetirement<ActorInput>, cx: &mut StepContext<'_>) -> Result<Option<OriginalNativeDecodeOutcome<RetainedCloneStep>>, OriginalTurnFailure> {
    source.close_owned_step(owner, cx)
}

pub fn close_job(source: &mut OriginalJsonOutputSource<ActorInput>, owner: &mut ControlledRetirement<OriginalJobSource>, cx: &mut StepContext<'_>) -> Result<Option<OriginalNativeDecodeOutcome<RetainedCloneStep>>, OriginalTurnFailure> {
    source.close_owned_step(owner, cx)
}
