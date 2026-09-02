//! 👁️ 👁️ Trinity Rewriting app command — `reorganize`.

use crate::artifacts::rewriting::op::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn reorganize(reorganize_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewritingConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewritingConfigMutation::SetReorganizeEpoch { value: reorganize_epoch + 1 }]))
}
