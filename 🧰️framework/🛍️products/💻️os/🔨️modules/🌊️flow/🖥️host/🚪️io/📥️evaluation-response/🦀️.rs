//! 📥️ Explicit host response JSON admission.
use neural_engine::Dictionary;
/// 📥️ Decodes a host extension response before typed cache ingestion.
pub fn decode_flow_node_output_json(source: &str) -> Result<Dictionary, semio_framework_value::ValueError> { semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject) }
