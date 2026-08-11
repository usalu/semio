//! 📤️ `s.stdio.semio/v1/cad` → `dwg` (ac1024) — honest unsupported direction, mirroring the
//! import leaf. `encode_dwg` (dwg's own real codec) only ever RE-EMITS `DwgSnapshot.bytes`
//! verbatim (it is not a synthesizer — see that engine's own `encode_dwg` body: it validates the
//! existing byte header and returns `snap.bytes.clone()`, never constructs new DWG binary
//! structure). There is therefore no real, honest way to synthesize a NEW `.dwg` file from CAD
//! entities without reimplementing the DWG binary writer from scratch — explicitly out of scope
//! ("zero codec reimplementation"). Per the recipe's "error out" allowance for a genuine
//! impedance mismatch, this leaf always returns a real, documented `PackError` rather than
//! fabricating placeholder bytes.

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

    fn serialize(_from: &Self::From) -> Result<Self::Into, store::PackError> {
        Err(store::PackError::Schema(
            "semio/cad→dwg: unsupported — the ac1024 codec's encode_dwg only re-emits pre-existing \
             DWG bytes verbatim (no entity-level DWG binary writer exists at this codec's D1/D2 decode \
             depth); synthesizing a new .dwg file from CAD entities would require reimplementing the \
             DWG binary format from scratch, out of this bridge's scope (documented unsupported \
             direction, not a fabricated result)."
                .into(),
        ))
    }
}
//#endregion 🔖️Serializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documents_unsupported_direction_as_a_real_error_not_fabricated_bytes() {
        let err = SemioCadToDwg::serialize(&SemioCadSnapshot::default()).unwrap_err();
        match err {
            store::PackError::Schema(msg) => assert!(msg.contains("unsupported")),
            other => panic!("expected PackError::Schema, got {other:?}"),
        }
    }
}
//#endregion 🔖️Tests
