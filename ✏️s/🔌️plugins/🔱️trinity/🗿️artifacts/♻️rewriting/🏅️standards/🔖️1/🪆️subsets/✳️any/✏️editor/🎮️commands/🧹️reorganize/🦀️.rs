//! 👁️ 👁️ Trinity Rewriting app command — `reorganize`.

use crate::standards::v1::subsets::any::schema::mutations::text::RewriteRuleMutation;
use crate::editor::rewriting::config::RewritingConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn reorganize(reorganize_epoch: u64) -> Emit<RewriteRuleMutation, RewritingConfigMutation> {
    Emit::config(vec![RewritingConfigMutation::SetReorganizeEpoch(crate::editor::rewriting::config::SetReorganizeEpoch { value: reorganize_epoch + 1 })])
}
