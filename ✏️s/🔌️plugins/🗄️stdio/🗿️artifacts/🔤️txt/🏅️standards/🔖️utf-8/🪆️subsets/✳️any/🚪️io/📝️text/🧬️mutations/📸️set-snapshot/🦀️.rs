//! 📝️ Canonical text payload codec for set-snapshot.

use crate::standards::v_utf_8::subsets::any::schema::mutations::SetSnapshotPayload;
use crate::schema::mutations::TxtMutation;

pub const TEXT_OPCODE: &str = "set-snapshot";

pub fn encode_payload(value: &SetSnapshotPayload) -> Result<String, String> {
    Ok(semio_framework_pack_json::to_json_string(value))
}

pub fn decode_payload(value: &str) -> Result<SetSnapshotPayload, String> {
    semio_framework_pack_json::from_json_str(value, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

pub fn try_encode(value: &TxtMutation) -> Option<Result<String, String>> {
    match value {
        TxtMutation::SetSnapshot(payload) => Some(encode_payload(payload)),
        _ => None,
    }
}

pub fn decode_mutation(value: &str) -> Result<TxtMutation, String> {
    decode_payload(value).map(TxtMutation::SetSnapshot)
}
