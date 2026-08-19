//! 📤️ `s.stdio.semio/v1/cad` → `dwg` (ac1024). The bridge remains unsupported until the CAD
//! topology model has a complete mapping to the logical DWG entity model; it never retains or
//! fabricates source bytes.

use crate::artifacts::dwg::DwgSnapshot;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };

//#region 🔖️Serializer
pub struct SemioCadToDwg;

impl ArtifactSerializer for SemioCadToDwg {
    type From = SemioCadSnapshot;
    type Into = DwgSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(_from: &Self::From) -> Result<Self::Into, store::PackError> {
        Err(store::PackError::Schema("semio/cad→dwg: unsupported until every CAD topology value has a defined logical DWG entity mapping".into()))
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documents_unsupported_direction_as_a_real_error_not_fabricated_bytes() {
        let err = semio_framework_plugin::resolve_ready(SemioCadToDwg::serialize(&SemioCadSnapshot::default())).unwrap_err();
        match err {
            store::PackError::Schema(msg) => assert!(msg.contains("unsupported")),
            other => panic!("expected PackError::Schema, got {other:?}"),
        }
    }
}
//#endregion 🔖️Tests
