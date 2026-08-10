//! 🧩️ Deserialize stdio.binary (raw/✳️any) from stdio.binary (raw/✳️any) — identity, the terminal
//! self-referential base case every other stdio artifact's DAG chain resolves through.

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use crate::artifacts::binary::BinarySnapshot;

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

pub struct BinaryFromBinaryRawAny;

impl ArtifactDeserializer for BinaryFromBinaryRawAny {
    type From = BinarySnapshot;
    type Into = BinarySnapshot;
    const FROM: Dialect = DIALECT;
    const INTO: Dialect = DIALECT;
    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(from.clone())
    }
}
