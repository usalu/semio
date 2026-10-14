//! 🎯️ 🎯️ Remodeling play app commands command — `add-gcp`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_calib::standards::v1::subsets::any::schema::mutations::create_gcp::create_gcp;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_schema::standards::v1::subsets::any::schema::mint_remodeling_id;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GroundControlPoint;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
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
