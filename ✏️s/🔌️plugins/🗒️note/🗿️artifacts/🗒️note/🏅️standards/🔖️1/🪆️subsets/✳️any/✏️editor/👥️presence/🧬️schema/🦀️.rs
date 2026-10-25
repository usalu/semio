//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue, ArtifactSchema, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.note.note.presence")]
pub struct NotePresence {
    #[state(presence)]
    pub camera_x: f64,
    #[state(presence)]
    pub camera_y: f64,
    #[state(presence)]
    pub camera_zoom: f64,
}

impl store::ArtifactPresenceSnapshot for NotePresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, semio_framework_value::ValueError> {
        store::decode_presence_json(bytes, owner)
    }
}
