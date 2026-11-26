//! 📥️ Draw member JSON advances its original operation and typed receiver under the existing caller loan.

use semio_framework_pack_json::OriginalJsonReceiving;
use semio_framework_value::{
    RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll,
    native_decoding::owned::OriginalSourceTurnWitness,
};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl, NativeReceivingFailure};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;

/// 🪜️ Advances Draw's typed JSON receiver on the same source witness, with the store retaining its owner.
#[inline(never)]
pub fn advance(
    receiver: &mut OriginalJsonReceiving<DrawingMutation>,
    context: &mut NativeDecodeControl<'_>,
    witness: OriginalSourceTurnWitness<'_>,
    body: usize,
    work: u64,
    grant: RetainedCloneGrant,
) -> RetainedWorkPoll<RetainedCloneStep, NativeReceivingFailure> {
    receiver.advance(context, witness, body, work, grant)
}
