//! 🧹️ 🧹️ Remodeling play app commands command — `clear-tracks`.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::mutations::replace_tracks;
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "clear-tracks")]
pub struct ClearTracks {}

pub fn handle(_payload: &ClearTracks, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_tracks(Vec::new())]))
}
