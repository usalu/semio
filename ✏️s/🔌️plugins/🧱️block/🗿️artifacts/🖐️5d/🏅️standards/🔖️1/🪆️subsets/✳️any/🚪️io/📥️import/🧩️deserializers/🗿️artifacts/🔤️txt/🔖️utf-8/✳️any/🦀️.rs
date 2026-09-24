//! txt import via framework `io_mechanism::deserialize_dsl_txt` (UTF-8 DSL carrier).

use store::ArtifactDsl;
use crate::Block5dSnapshot;
use semio_framework::io::io_mechanism::{deserialize_dsl_txt, Deserializer};
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct TxtIntoBlock5d;

/// 🏗 Parses UTF-8 DSL text into this subset snapshot — also used by the zip container leaf.
pub fn from_dsl_text(text: &str) -> Result<Block5dSnapshot, IoError> {
    <Block5dSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError { message: error.to_string(), diagnostics: Vec::new() })
}

impl Deserializer<Block5dSnapshot> for TxtIntoBlock5d {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<Block5dSnapshot> {
        deserialize_dsl_txt(payload)
    }
}
