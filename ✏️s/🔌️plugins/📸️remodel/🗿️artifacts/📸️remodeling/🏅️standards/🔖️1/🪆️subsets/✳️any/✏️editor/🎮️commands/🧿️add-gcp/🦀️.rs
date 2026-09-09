//! 🎯️ 🎯️ Remodeling play app commands command — `add-gcp`.

use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use crate::mutations::create_gcp;
use crate::op::RemodelingMutation;
use crate::schema::next_remodeling_id;
use crate::{GroundControlPoint, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "add-gcp")]
pub struct AddGcp {
    pub name: String,
    pub world_x: f64,
    pub world_y: f64,
    pub world_z: f64,
}

pub fn handle(payload: &AddGcp, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    let id = next_remodeling_id("gcp");
    let gcp = GroundControlPoint { id, name: payload.name.clone(), world_position: [payload.world_x, payload.world_y, payload.world_z], observations: Vec::new() };
    Ok(Emit::mutations(vec![create_gcp(gcp)]))
}
