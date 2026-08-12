//! 🗂 `structure` — ac1018's honest structural statistic. This standard's `DwgSnapshot` (see
//! `📸️snapshot/🦀️component.rs`) never decoded any geometric entity — `bytes` is the opaque,
//! undecoded raw payload and `section_names` a substring-scanned label list, per that module's own
//! "deliberately frozen legacy shim" doc comment. Forcing a bounding box onto this format would be
//! dishonest (there is no decoded geometry to bound); a direct byte/section count read straight
//! off the snapshot's own persisted fields is the closest honest derived stat. A pure
//! whole-snapshot scalar (no per-record fold) — no `InferredField` needed.

use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Structure
/// 🗂️ Dwg (ac1018) structural byte/section statistics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DwgStructure {
    pub byte_count: u32,
    pub section_count: u32,
    pub codepage: u16,
    pub version: String,
}

/// 🩹 Hand-rolled: matches `compute_dwg_structure(&DwgSnapshot::default())` exactly
/// (`DwgSnapshot::default()`'s `bytes`/`section_names` are empty, `codepage` is `0`, `version` is
/// `String::new()` — verified directly against `📸️snapshot/🦀️component.rs`'s own `Default` impl,
/// not assumed).
impl Default for DwgStructure {
    fn default() -> Self {
        Self { byte_count: 0, section_count: 0, codepage: 0, version: String::new() }
    }
}

/// 🗂️ Computes [`DwgStructure`] directly from the snapshot's own persisted fields — a straight
/// read/count, no fold over any per-entity list (ac1018 has none).
pub fn compute_dwg_structure(snapshot: &DwgSnapshot) -> DwgStructure {
    DwgStructure {
        byte_count: snapshot.bytes.len() as u32,
        section_count: snapshot.section_names.len() as u32,
        codepage: snapshot.codepage,
        version: snapshot.version.clone(),
    }
}
//#endregion 🔖️Structure

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    #[test]
    fn structure_matches_hand_built_snapshot() {
        let snapshot = DwgSnapshot {
            schema: "s.stdio.dwg".into(),
            version: "AC1018".into(),
            maintenance_version: 2,
            codepage: 30,
            bytes: vec![0u8; 128],
            section_names: vec!["AcDb:Header".into(), "AcDb:Classes".into(), "AcDb:Handles".into()],
        };
        let structure = compute_dwg_structure(&snapshot);
        assert_eq!(structure.byte_count, 128);
        assert_eq!(structure.section_count, 3);
        assert_eq!(structure.codepage, 30);
        assert_eq!(structure.version, "AC1018");
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = DwgSnapshot {
            schema: "s.stdio.dwg".into(),
            version: "AC1018".into(),
            maintenance_version: 0,
            codepage: 30,
            bytes: vec![1, 2, 3],
            section_names: vec!["AcDb:Header".into()],
        };
        assert_eq!(compute_dwg_structure(&snapshot), compute_dwg_structure(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_dwg_structure(&DwgSnapshot::default()), DwgStructure::default());
    }
}
//#endregion 🧪️Tests
