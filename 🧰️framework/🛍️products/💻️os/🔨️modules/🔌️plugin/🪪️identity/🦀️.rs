//! 🪪️ Plugin-owned entity-identity admission for every store authoring call.
use crate::store::EntityIdentityAuthority;
use semio_framework_value::native_encoding::NativeEncodeProgress;

/// 🚦️ Cumulative owned-byte ceiling one authoring call admits across all of its physical stages.
pub const ARTIFACT_IDENTITY_CEILING_BYTES: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;

/// 👁️ The plugin's identity observer: continues while owned bytes stay under the declared ceiling.
pub fn identity_observer() -> impl FnMut(NativeEncodeProgress) -> bool + Send {
    |progress| progress.owned_bytes <= ARTIFACT_IDENTITY_CEILING_BYTES
}

/// 🪪️ Admits one caller-owned identity authority over `observer` and runs `operation` under it.
/// The consuming ownership receipt returns to the caller's drop only after the call finished.
#[macro_export]
macro_rules! with_authoring_identity {
    (|$identity:ident| $operation:expr) => {{
        let mut observer = $crate::authoring_identity::identity_observer();
        let mut $identity = $crate::store::EntityIdentityAuthority::new($crate::authoring_identity::ARTIFACT_IDENTITY_CEILING_BYTES, &mut observer).unwrap_or_else(|_| unreachable!("plugin identity ceiling is a declared nonzero constant"));
        let result = $operation;
        drop($identity.pause().unwrap_or_else(|_| unreachable!("identity authority retains its receipt through one authoring call")));
        result
    }};
}
