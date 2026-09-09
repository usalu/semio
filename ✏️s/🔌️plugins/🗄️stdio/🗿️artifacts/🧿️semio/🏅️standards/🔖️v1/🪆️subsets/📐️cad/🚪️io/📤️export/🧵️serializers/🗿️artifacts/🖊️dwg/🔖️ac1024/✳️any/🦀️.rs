//! 📤️ `s.stdio.semio/v1/cad` → `dwg` (ac1024). The bridge remains unsupported until the CAD
//! topology model has a complete mapping to the logical DWG entity model; it never retains or
//! fabricates source bytes.

use crate::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_dwg::DwgSnapshot;

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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
