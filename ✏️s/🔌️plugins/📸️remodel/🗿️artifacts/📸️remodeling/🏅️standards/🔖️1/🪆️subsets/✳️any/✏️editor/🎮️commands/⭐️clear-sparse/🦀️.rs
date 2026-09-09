//! 🧹️ 🧹️ Remodeling play app commands command — `clear-sparse`.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::mutations::replace_sparse;
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "clear-sparse")]
pub struct ClearSparse {}

pub fn handle(_payload: &ClearSparse, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_sparse(None)]))
}
