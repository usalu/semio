//! 📥️ 📥️ Remodeling play app commands command — `set-stream-sync`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_remodel_remodeling_mut_media::standards::v1::subsets::any::schema::mutations::change_stream_sync::change_stream_sync;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::RemodelingMutation;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "stream-sync")]
pub struct SetStreamSync {
    pub stream_id: String,
    pub sync_offset_ms: f64,
}

pub fn handle(payload: &SetStreamSync, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    if !doc.snapshot.streams.iter().any(|stream| stream.id == payload.stream_id) {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(vec![change_stream_sync(payload.stream_id.clone(), payload.sync_offset_ms)]))
}
