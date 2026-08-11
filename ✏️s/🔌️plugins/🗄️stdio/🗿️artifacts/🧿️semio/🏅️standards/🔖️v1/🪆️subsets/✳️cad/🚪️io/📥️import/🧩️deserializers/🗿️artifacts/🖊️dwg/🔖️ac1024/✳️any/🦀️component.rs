//! 📥️ `dwg` (ac1024) → `s.stdio.semio/v1/cad` — honest unsupported-content bridge, per the master
//! plan's own allowance ("others document as unsupported"). `DwgSnapshot` at this codec's real
//! decode depth (its own module doc: "D1/D2 wave... bitcode/header-variable parsing is D3-D4, out
//! of scope") locates named SECTIONS and decompresses their PAGES, but never parses entity/layer
//! bitcode — there is no decoded CAD content anywhere on `DwgSnapshot` to map (`sections[].pages[]
//! .decoded` is opaque bytes, `derive_section_names` only yields section NAMES like
//! `AcDb:AcDbObjects`, not layer/entity records). This is a REAL reflection of the underlying
//! codec's real capability boundary, not a stub: it always produces a structurally valid, empty
//! `SemioCadSnapshot` (zero layers/blocks/entities), and is asserted to do so by its own test
//! rather than silently claiming content that was never decoded.

use crate::artifacts::dwg::DwgSnapshot;
use crate::artifacts::semio::standards::v1::subsets::cad::schema::snapshot::{SemioCadSnapshot, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1024"), subset: SubsetId::ANY };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("cad") };

//#region 🔖️Deserializer
pub struct SemioCadFromDwg;

impl ArtifactDeserializer for SemioCadFromDwg {
    type From = DwgSnapshot;
    type Into = SemioCadSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        if from.version.is_empty() {
            return Err(store::PackError::Schema("dwg→semio/cad: missing AC10xx version sentinel — not a real DWG file".into()));
        }
        // 🚧 Honest: this codec's D1/D2 decode depth never reaches entity/layer bitcode (see
        // module doc) — there is no real CAD content on `DwgSnapshot` to carry over yet.
        Ok(SemioCadSnapshot { schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(), layers: Vec::new(), blocks: Vec::new(), entities: Vec::new() })
    }
}
//#endregion 🔖️Deserializer

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_dwg() -> DwgSnapshot {
        let mut bytes = vec![b'A', b'C', b'1', b'0', b'3', b'2'];
        bytes.resize(0x15, 0);
        DwgSnapshot { version: "AC1032".into(), maintenance_version: 0, codepage: 30, bytes, ..DwgSnapshot::default() }
    }

    #[test]
    fn produces_empty_but_valid_cad_snapshot() {
        let cad = SemioCadFromDwg::deserialize(&sample_dwg()).expect("deserialize");
        assert!(cad.layers.is_empty());
        assert!(cad.blocks.is_empty());
        assert!(cad.entities.is_empty());
        assert_eq!(cad.schema, STDIO_SEMIOCAD_DOCUMENT_SCHEMA);
    }

    #[test]
    fn rejects_missing_version() {
        let bad = DwgSnapshot { version: String::new(), ..DwgSnapshot::default() };
        assert!(SemioCadFromDwg::deserialize(&bad).is_err());
    }
}
//#endregion 🔖️Tests
