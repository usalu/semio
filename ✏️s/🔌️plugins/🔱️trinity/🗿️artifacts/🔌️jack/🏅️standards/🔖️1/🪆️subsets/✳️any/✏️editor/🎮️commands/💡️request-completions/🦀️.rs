//! 🔎️ 🔎️ Trinity Jack app command — `request-completions`.

use crate::standards::v1::subsets::any::schema::mutations::text::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::Emit;

pub(crate) fn request_completions(revision: u64) -> Emit<TrinityGraphMutation, JackConfigMutation> {
    Emit::config(vec![JackConfigMutation::SetRevision(crate::editor::jack::config::SetRevision { value: revision + 1 })])
}
