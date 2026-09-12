//! 🧹️ 🧹️ Remodeling play app commands command — `clear-sparse`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::replace_sparse;
use crate::op::RemodelingMutation;
use crate::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "clear-sparse")]
pub struct ClearSparse {}

pub fn handle(_payload: &ClearSparse, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![replace_sparse(None)]))
}
