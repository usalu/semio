//! 🧩️ 🧩️ Sourcing curation app commands command — `set-contributions`.

use crate::{op::SourcingMutation, CurationSnapshot};
use crate::editor::sourcing::component::SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "contributions")]
pub struct SetContributions {
    pub json: String,
}

/// 🧩️ Retains the INSTALLABLE share of the host pack, never the pack itself: the host cuts its pack
/// from the whole loaded closure, while this app's retained config lane is a fixed envelope
/// (`SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES`). `schema::installable_contributions` keeps the
/// `sourcing.module` entries this app can actually act on and drops the rest, so an oversized or
/// foreign pack installs what fits instead of being refused whole.
pub fn handle(payload: &SetContributions, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    let json = crate::schema::installable_contributions(&payload.json, SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES);
    Ok(Emit::config(vec![SourcingCurationConfigMutation::SetContributions { json }]))
}
