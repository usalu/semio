//! 🧹️ 🧹️ Remodeling play app commands command — `clear-tracks`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::replace_tracks;
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "clear-tracks")]
pub struct ClearTracks {}

pub fn handle(_payload: &ClearTracks, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_tracks(Vec::new())]))
}
