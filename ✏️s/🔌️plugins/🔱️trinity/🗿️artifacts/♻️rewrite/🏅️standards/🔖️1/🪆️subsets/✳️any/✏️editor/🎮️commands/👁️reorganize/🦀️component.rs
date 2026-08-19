//! 👁️ 👁️ Trinity Rewrite app command — `reorganize`.

use crate::editor::rewrite::config::RewriteConfigMutation;
use crate::artifacts::rewrite::op::RewriteRuleMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) async fn reorganize(reorganize_epoch: u64) -> Result<Emit<RewriteRuleMutation, RewriteConfigMutation>, Fault> {
    Ok(Emit::config(vec![RewriteConfigMutation::SetReorganizeEpoch { value: reorganize_epoch + 1 }]))
}
