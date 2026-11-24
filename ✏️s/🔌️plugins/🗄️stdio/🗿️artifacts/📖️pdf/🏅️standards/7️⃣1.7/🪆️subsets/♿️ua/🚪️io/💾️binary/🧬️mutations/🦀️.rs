//! 🧾️ Semio Pack representation for PDF/UA mutations.

use crate::standards::v1_7::subsets::ua::schema::mutations::PdfUaMutation;
use protocol::{OpBinary, ProtocolError};
use semio_framework_value::{FromValue, ToValue};
use store::os_pack::pack_rt::{decode_pack_value, encode_pack_value, renormalize_whole_number_floats};

impl OpBinary for PdfUaMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(encode_pack_value(&self.to_value()))
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let malformed = |detail| ProtocolError::Malformed { what: "PdfUaMutation", offset: 0, detail };
        let value = decode_pack_value(bytes).map_err(|error| malformed(error.to_string()))?;
        Self::from_value(renormalize_whole_number_floats(value)).map_err(|error| malformed(error.to_string()))
    }
}
