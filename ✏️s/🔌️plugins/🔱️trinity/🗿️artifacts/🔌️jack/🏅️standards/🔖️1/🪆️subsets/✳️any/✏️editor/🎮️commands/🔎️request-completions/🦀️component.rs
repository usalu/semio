//! 🔎️ 🔎️ Trinity Jack app command — `request-completions`.

use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::editor::jack::config::JackConfigMutation;
use semio_framework_plugin::{Emit, Fault};

pub(crate) fn request_completions(revision: u64) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetRevision { value: revision + 1 }]))
}
