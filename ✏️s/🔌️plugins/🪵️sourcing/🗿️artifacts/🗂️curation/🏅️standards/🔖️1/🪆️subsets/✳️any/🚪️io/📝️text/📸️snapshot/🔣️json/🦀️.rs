//! 🔣 Native Curation JSON transport delegates record admission to the schema owner.
use crate::CurationSnapshot;
/// 📤 Emits the persisted Curation record.
pub fn encode_curation_snapshot_json(snapshot: &CurationSnapshot) -> String { semio_framework_pack_json::to_json_string(snapshot) }
/// 📥 Admits one exact persisted Curation record.
pub fn decode_curation_snapshot_json(text: &str) -> Result<CurationSnapshot, String> { semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string()) }
