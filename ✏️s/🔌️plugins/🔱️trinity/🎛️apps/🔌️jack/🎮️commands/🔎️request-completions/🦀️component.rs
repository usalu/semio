//! 🔎️ 🔎️ Trinity Jack app command — `request-completions`.

use crate::apps::jack::config::JackConfigMutation;
use crate::artifacts::jack::op::TrinityGraphMutation;
use crate::artifacts::jack::JackSnapshot;
use crate::core;
use semio_framework_plugin::{Emit, Fault};
use serde_json::json;
use store::ArtifactDsl;

pub(crate) fn request_completions(revision: u64) -> Result<Emit<TrinityGraphMutation, JackConfigMutation>, Fault> {
    Ok(Emit::config(vec![JackConfigMutation::SetRevision { value: revision + 1 }]))
}
