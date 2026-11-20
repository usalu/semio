//! 🧲️ Original native decode and encode owners the stdio artifact tests run their operations under.

use super::{NATIVE_MAXIMUM_BYTES, original_grant};
use semio_framework_job::OriginalRetirementPump;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::retained_clone::OriginalEffectReply;
use semio_framework_value::{NativeEncodeControl, RetainedAllocationEffect, RetainedCloneGrant, RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl, NativeDecodeRetirementRecipient};
use semio_os_kernel::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};

/// 🪙️ Funds a test caller's original close turn.
pub(super) fn retirement_effect<T>(grant: RetainedCloneGrant, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> T) -> T {
    let mut turn = OriginalRetirementPump::new().turn(1 << 24, u64::MAX, || Some(0), grant).expect("original retirement turn");
    turn.with_effect(close).expect("original retirement work settles").expect("original retirement work admitted").0
}

pub(super) fn finish_decode(owner: &mut NativeSnapshotDecodeOwner<'_, '_>) {
    while owner.native().has_retirement_owner() {
        let grant = owner.remaining_grant();
        let poll = retirement_effect(grant, |effect| owner.native().close_retirement_recipient(grant.maximum_copy_bytes, effect));
        if let RetainedWorkPoll::Complete(turn) = poll {
            let progress = match &turn.result { Ok(step) => step.progress(), Err(error) => error.retained_progress() };
            owner.record_progress(progress).expect("original decode close is funded");
            turn.result.expect("original decode recipient closes");
        }
    }
}

pub(super) fn finish_encode(owner: &mut NativeSnapshotEncodeOwner<'_, '_>) {
    let mut answer = None;
    let mut pending = false;
    while owner.native().has_retirement_owner() {
        let grant = owner.remaining_grant();
        retirement_effect(grant, |effect| {
            let reply = OriginalEffectReply::borrow(&mut answer).expect("empty encoder close reply");
            let result = if pending {
                owner.native().recover_close_retirement_recipient_into(grant.maximum_copy_bytes, effect, reply)
            } else {
                owner.native().close_retirement_recipient_into(grant.maximum_copy_bytes, effect, reply)
            };
            result.expect("original encode recipient turn");
        });
        pending = answer.is_none();
        if let Some(result) = answer.take() {
            let progress = match &result { Ok(step) => step.progress(), Err(error) => error.retained_progress() };
            owner.record_progress(progress).expect("original encode close is funded");
            result.expect("original encode recipient closes");
        }
    }
}

/// 🛬️ Runs `operation` under an original native decode owner and closes every retired owner afterwards.
pub fn with_decoding<T>(operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> T) -> T {
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original decode recipient installs");
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, original_grant());
    let result = operation(&mut owner);
    finish_decode(&mut owner);
    result
}

/// 🛫️ Runs `operation` under an original native encode owner and closes every retired owner afterwards.
pub fn with_encoding<T>(operation: impl FnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let mut owner = NativeSnapshotEncodeOwner::new(&mut native, original_grant());
    let result = operation(&mut owner);
    finish_encode(&mut owner);
    result
}

/// 🕑️ Runs an async `operation` under an original native encode owner and closes every retired owner afterwards.
pub async fn with_encoding_async<T>(operation: impl AsyncFnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let mut owner = NativeSnapshotEncodeOwner::new(&mut native, original_grant());
    let result = operation(&mut owner).await;
    finish_encode(&mut owner);
    result
}
