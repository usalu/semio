//! 📥️ 📥️ Remodeling play app commands command — `remove-stream`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::delete_stream;
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "remove-stream")]
pub struct RemoveStream {
    pub stream_id: String,
}

pub fn handle(payload: &RemoveStream, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![delete_stream(payload.stream_id.clone())]))
}
