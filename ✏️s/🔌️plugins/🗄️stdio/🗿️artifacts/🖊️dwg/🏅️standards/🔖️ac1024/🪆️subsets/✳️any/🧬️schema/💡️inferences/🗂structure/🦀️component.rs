//! 🗂 `structure` — ac1024's honest structural statistic. This standard's `DwgSnapshot` (see
//! `📸️snapshot/🦀️component.rs`) performs real D1/D2 structural decode (section+page location,
//! per-page decompression) but never decodes any geometric entity out of the decoded bytes (D3-D4
//! bitcode/header-variable parsing is out of this ticket's scope), so a bounding-box inference
//! (dxf's `📦bounds/`) would be dishonest here; `structure` is the closest honest derived
//! statistic — real byte/section/page counts folded over `sections[].pages[]`, richer than
//! ac1018's own flat byte/section-name-only struct because THIS standard's snapshot genuinely has
//! more decoded structure. A pure whole-snapshot fold (no per-entity `InferredField` semantics —
//! sections/pages are a flat structural list, not a DAG) — no `InferredField` needed.

use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Structure
/// 🗂️ Dwg (ac1024) structural byte/section/page statistics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DwgStructure {
    pub byte_count: u32,
    pub section_count: u32,
    pub page_count: u32,
    pub decoded_page_count: u32,
    pub error_page_count: u32,
    pub declared_total_size: u64,
    pub codepage: u16,
    pub version: String,
}

/// 🩹 Hand-rolled: matches `compute_dwg_structure(&DwgSnapshot::default())` exactly
/// (`DwgSnapshot::default()`'s `source`/`sections` are empty, `codepage` is `0`, `version` is
/// `String::new()` — verified directly against `📸️snapshot/🦀️component.rs`'s own `Default` impl,
/// not assumed).
impl Default for DwgStructure {
    fn default() -> Self {
        Self {
            byte_count: 0,
            section_count: 0,
            page_count: 0,
            decoded_page_count: 0,
            error_page_count: 0,
            declared_total_size: 0,
            codepage: 0,
            version: String::new(),
        }
    }
}

/// 🗂️ Computes [`DwgStructure`] by folding `sections[].pages[]` (page/decoded/error counts,
/// declared-size sum) plus a few O(1) top-level field reads.
pub fn compute_dwg_structure(snapshot: &DwgSnapshot) -> DwgStructure {
    let mut page_count = 0u32;
    let mut decoded_page_count = 0u32;
    let mut error_page_count = 0u32;
    let mut declared_total_size = 0u64;

    for section in &snapshot.sections {
        declared_total_size += section.declared_size;
        for page in &section.pages {
            page_count += 1;
            if !page.decoded.is_empty() {
                decoded_page_count += 1;
            }
            if page.error.is_some() {
                error_page_count += 1;
            }
        }
    }

    DwgStructure {
        byte_count: snapshot.sections.iter().flat_map(|section| &section.pages).map(|page| page.decoded.len() as u64).sum::<u64>().min(u32::MAX as u64) as u32,
        section_count: snapshot.sections.len() as u32,
        page_count,
        decoded_page_count,
        error_page_count,
        declared_total_size,
        codepage: snapshot.codepage,
        version: snapshot.version.clone(),
    }
}
//#endregion 🔖️Structure

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::{DwgSection, DwgSectionPage};

    #[test]
    fn structure_matches_hand_built_sections_and_pages() {
        let snapshot = DwgSnapshot {
            schema: "s.stdio.dwg".into(),
            version: "AC1024".into(),
            maintenance_version: 3,
            codepage: 30,
            drawing: Default::default(),
            section_names: vec!["AcDb:Header".into(), "AcDb:Classes".into()],
            sections: vec![
                DwgSection {
                    name: "AcDb:Header".into(),
                    compressed: true,
                    declared_size: 100,
                    pages: vec![
                        DwgSectionPage { page_number: 0, start_offset: 0x100, decompressed_size: 40, decoded: vec![1, 2, 3], error: None },
                        DwgSectionPage { page_number: 1, start_offset: 0x140, decompressed_size: 30, decoded: Vec::new(), error: Some("bad crc".into()) },
                    ],
                    ..Default::default()
                },
                DwgSection {
                    name: "AcDb:Classes".into(),
                    compressed: false,
                    declared_size: 50,
                    pages: vec![DwgSectionPage { page_number: 0, start_offset: 0x200, decompressed_size: 50, decoded: vec![9], error: None }],
                    ..Default::default()
                },
            ],
            decode_status: crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::DwgDecodeStatus::SectionsLocated,
            physical: Default::default(),
        };
        let structure = compute_dwg_structure(&snapshot);
        assert_eq!(structure.byte_count, 4);
        assert_eq!(structure.section_count, 2);
        assert_eq!(structure.page_count, 3);
        assert_eq!(structure.decoded_page_count, 2);
        assert_eq!(structure.error_page_count, 1);
        assert_eq!(structure.declared_total_size, 150);
        assert_eq!(structure.codepage, 30);
        assert_eq!(structure.version, "AC1024");
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = DwgSnapshot {
            schema: "s.stdio.dwg".into(),
            version: "AC1024".into(),
            maintenance_version: 0,
            codepage: 30,
            drawing: Default::default(),
            section_names: vec!["AcDb:Header".into()],
            sections: vec![DwgSection {
                name: "AcDb:Header".into(),
                compressed: true,
                declared_size: 10,
                pages: vec![DwgSectionPage { page_number: 0, start_offset: 0, decompressed_size: 5, decoded: vec![1], error: None }],
                ..Default::default()
            }],
            decode_status: crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::DwgDecodeStatus::SectionsDecompressed,
            physical: Default::default(),
        };
        assert_eq!(compute_dwg_structure(&snapshot), compute_dwg_structure(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_dwg_structure(&DwgSnapshot::default()), DwgStructure::default());
    }
}
//#endregion 🧪️Tests
