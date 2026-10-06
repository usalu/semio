//! 💾️ Own14 original path operation bytes, not a snapshot carrier.
use crate::standards::v1_4::subsets::base::schema::mutations::PatchSnapshot;
use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
pub const TAG:u8=dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"),"patch-snapshot");
pub fn encode(mutation:&PdfMutation)->Option<Result<Vec<u8>,String>>{let PdfMutation::PatchSnapshot(payload)=mutation else{return None};Some(protocol::OpBinary::encode_op(&payload.patch).map_err(|error|error.to_string()))}
pub fn decode(bytes:&[u8])->Result<PdfMutation,String>{semio_s_artifact_stdio_contract::editing::snapshot_patch_from_bytes(bytes).map(|patch|PdfMutation::PatchSnapshot(PatchSnapshot{patch}))}
