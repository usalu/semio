//! 📥️ txt import via framework `io_mechanism::deserialize_dsl_txt` (UTF-8 DSL carrier).

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{deserialize_dsl_txt, Deserializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};

pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;

pub struct TxtIntoBlock2d;

/// 🏗 Parses UTF-8 DSL text into this subset snapshot — also used by the zip container leaf.
pub fn from_dsl_text(text: &str) -> Result<Block2dSnapshot, IoError> {
    <Block2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(error.kind, error.to_string())))
}

impl Deserializer<Block2dSnapshot> for TxtIntoBlock2d {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<Block2dSnapshot> {
        deserialize_dsl_txt(payload)
    }
}
