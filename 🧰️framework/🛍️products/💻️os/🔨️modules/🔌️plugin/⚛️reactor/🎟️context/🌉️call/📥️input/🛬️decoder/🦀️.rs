//! 🛬️ Concrete JSON receive and take steps for an actor's original typed input.
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_pack_json_output_receiving::{OriginalJsonOutputOutcome, OriginalJsonOutputSource};
use semio_plugin_abi::owned_abi::ActorInput;

/// 📥️ Advances the caller's original receive until it can take the exact typed result.
pub fn advance(source: &mut OriginalJsonOutputSource<ActorInput>, wire: &[u8], input: &mut Option<ActorInput>, cx: &mut StepContext<'_>) -> Result<Option<OriginalJsonOutputOutcome>, OriginalTurnFailure> {
    if source.ready() { source.take_step(input, cx) } else { source.receive_step(wire, cx) }
}
