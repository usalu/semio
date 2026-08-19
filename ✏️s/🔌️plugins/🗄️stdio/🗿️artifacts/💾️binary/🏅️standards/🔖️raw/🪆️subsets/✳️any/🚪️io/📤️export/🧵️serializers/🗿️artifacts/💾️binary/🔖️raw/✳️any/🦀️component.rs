//! 🧵️ Serialize stdio.binary (raw/✳️any) to stdio.binary (raw/✳️any) — identity, the terminal
//! self-referential base case every other stdio artifact's DAG chain resolves through.

use crate::artifacts::binary::BinarySnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

pub struct BinaryToBinaryRawAny;

impl ArtifactSerializer for BinaryToBinaryRawAny {
    type From = BinarySnapshot;
    type Into = BinarySnapshot;
    const FROM: Dialect = DIALECT;
    const INTO: Dialect = DIALECT;
    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        Ok(from.clone())
    }
}
