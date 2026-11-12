//! 🚪️ Native artifact representation codecs.
use super::*;

impl protocol::OpBinary for WorkflowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_os_kernel::os_pack::pack_rt::encode_pack_value(&semio_framework_value::ToValue::to_value(self)))
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |detail: String| protocol::ProtocolError::Malformed { what: "workflow-mutation", offset: 0, detail };
        let value = semio_os_kernel::os_pack::pack_rt::decode_pack_value(bytes).map_err(|error| malformed(error.to_string()))?;
        semio_framework_value::FromValue::from_value(semio_os_kernel::os_pack::pack_rt::renormalize_whole_number_floats(value)).map_err(|error| malformed(error.to_string()))
    }
}

dsl::original_pack_value_operation_receiving!(WorkflowMutation);
dsl::artifact_persisted_binary_operation!(WorkflowMutation);
