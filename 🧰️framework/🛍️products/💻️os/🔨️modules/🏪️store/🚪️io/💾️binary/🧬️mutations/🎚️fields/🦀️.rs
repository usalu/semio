//! 🎚️ Explicit config field-set representation expansion.
#[macro_export]
macro_rules! field_set_mutations_binary {
    ($set:ident) => {
        impl $crate::OpBinary for $set {
            fn encode_op(&self) -> Result<Vec<u8>, $crate::ProtocolError> {
                Ok($crate::os_pack::pack_rt::encode_pack_value(&$crate::__value::ToValue::to_value(self)))
            }

            fn decode_op(bytes: &[u8]) -> Result<Self, $crate::ProtocolError> {
                let value = $crate::os_pack::pack_rt::decode_pack_value(bytes).map_err($crate::ProtocolError::from)?;
                $crate::__value::FromValue::from_value($crate::os_pack::pack_rt::renormalize_whole_number_floats(value)).map_err(|error| $crate::ProtocolError::from($crate::PackError::from(error)))
            }
        }
    };
}
