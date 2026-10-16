//! 🔐️ Original grants and the mounted owner policy every stdio artifact test caller runs under.

use semio_framework_plugin::MountedOwnerPolicyV1;
use semio_framework_value::RetainedCloneGrant;

/// 📏️ Native allocation ceiling the original test callers grant.
pub const NATIVE_MAXIMUM_BYTES: usize = 1 << 30;

/// 🎟️ The generous five-axis ownership grant of the original test caller.
pub const fn original_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1 << 24, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 4_096 }
}

/// ♾️ A grant no test step can exhaust, for jobs whose retained work the test does not meter.
pub const fn unbounded_grant() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX }
}

/// 🛂️ The explicit mounted owner policy every test instance runs under: the shipped stdio hub policy, whose item budget batches the interactive publication ladder and whose capacity admits one whole-snapshot apply of the largest shipped fixture.
pub const fn mounted_policy() -> MountedOwnerPolicyV1 {
    MountedOwnerPolicyV1::INTERACTIVE_COMPONENT
}
