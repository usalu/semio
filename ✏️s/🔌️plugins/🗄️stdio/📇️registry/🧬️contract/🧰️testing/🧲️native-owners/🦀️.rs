//! 🧲️ Original native decode and encode owners the stdio artifact tests run their operations under.

use super::{NATIVE_MAXIMUM_BYTES, original_grant};
use semio_framework_os_kernel::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};
use semio_framework_value::native_decoding::NativeDecodeRetirementRecipient;
use semio_framework_value::native_encoding::NativeEncodeRetirementRecipient;
use semio_framework_value::{NativeDecodeControl, NativeDecodeProgress, NativeEncodeControl};

/// 🛬️ Runs `operation` under an original native decode owner and closes every retired owner afterwards.
pub fn with_decoding<T>(operation: impl FnOnce(&mut NativeSnapshotDecodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeDecodeRetirementRecipient::new();
    let mut native = NativeDecodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original decode recipient installs");
    let mut owner = NativeSnapshotDecodeOwner::new(&mut native, grant);
    let result = operation(&mut owner);
    drop(owner);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant).expect("original decode recipient closes");
    }
    result
}

/// 🛫️ Runs `operation` under an original native encode owner and closes every retired owner afterwards.
pub fn with_encoding<T>(operation: impl FnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let mut owner = NativeSnapshotEncodeOwner::new(&mut native, grant);
    let result = operation(&mut owner);
    drop(owner);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant).expect("original encode recipient closes");
    }
    result
}

/// 🛫️ Runs an async `operation` under an original native encode owner and closes every retired owner afterwards.
pub async fn with_encoding_async<T>(operation: impl AsyncFnOnce(&mut NativeSnapshotEncodeOwner<'_, '_>) -> T) -> T {
    let grant = original_grant();
    let mut callback = |_| true;
    let mut recipient = NativeEncodeRetirementRecipient::new();
    let mut native = NativeEncodeControl::new(NATIVE_MAXIMUM_BYTES, &mut callback);
    native.install_retirement_recipient(&mut recipient).expect("original encode recipient installs");
    let mut owner = NativeSnapshotEncodeOwner::new(&mut native, grant);
    let result = operation(&mut owner).await;
    drop(owner);
    while native.has_retirement_owner() {
        native.close_retirement_recipient(grant).expect("original encode recipient closes");
    }
    result
}

/// 🛬️ A bare native decode control with its original retirement recipient installed; the recipient is leaked so the control can be built inline at a call site.
pub fn decode_control<'a>(maximum_bytes: usize, callback: &'a mut dyn FnMut(NativeDecodeProgress) -> bool) -> NativeDecodeControl<'a> {
    let recipient: &'a mut NativeDecodeRetirementRecipient = Box::leak(Box::new(NativeDecodeRetirementRecipient::new()));
    let mut native = NativeDecodeControl::new(maximum_bytes, callback);
    native.install_retirement_recipient(recipient).expect("original decode recipient installs");
    native
}
