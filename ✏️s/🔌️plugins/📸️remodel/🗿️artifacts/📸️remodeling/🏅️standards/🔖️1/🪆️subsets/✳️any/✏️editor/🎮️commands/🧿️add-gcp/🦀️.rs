//! 🎯️ 🎯️ Remodeling play app commands command — `add-gcp`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::mutations::create_gcp;
use crate::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use crate::schema::mint_remodeling_id;
use crate::{GroundControlPoint, RemodelingSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "add-gcp")]
pub struct AddGcp {
    pub name: String,
    pub world_x: f64,
    pub world_y: f64,
    pub world_z: f64,
}

pub fn handle(payload: &AddGcp, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let id = mint_remodeling_id(doc.operation_optional(), "gcp");
    let gcp = GroundControlPoint { id, name: payload.name.clone(), world_position: [payload.world_x, payload.world_y, payload.world_z], observations: Vec::new() };
    Ok(Emit::mutations(vec![create_gcp(gcp)]))
}
