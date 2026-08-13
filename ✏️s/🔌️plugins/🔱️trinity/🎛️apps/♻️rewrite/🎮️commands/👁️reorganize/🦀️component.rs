//! 👁️ 👁️ Trinity Rewrite app command — `reorganize`.

use crate::apps::rewrite::config::RewriteConfigMutation;
use crate::artifacts::jack::Camera;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn reorganize(reorganize_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetReorganizeEpoch { value: reorganize_epoch + 1 }]))
}
