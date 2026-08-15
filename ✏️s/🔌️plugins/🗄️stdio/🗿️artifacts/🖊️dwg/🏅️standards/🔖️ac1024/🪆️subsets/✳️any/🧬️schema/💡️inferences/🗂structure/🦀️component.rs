//! 🗂 `structure` — logical drawing statistics derived only from modeled layers and entities.

use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Structure
/// 🗂️ Dwg (ac1024) logical drawing statistics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DwgStructure {
    pub layer_count: u32,
    pub entity_count: u32,
    pub geometry_value_count: u32,
    pub geometry_index_count: u32,
    pub text_character_count: u32,
    pub codepage: u16,
    pub version: String,
}

impl Default for DwgStructure {
    fn default() -> Self {
        Self { layer_count: 0, entity_count: 0, geometry_value_count: 0, geometry_index_count: 0, text_character_count: 0, codepage: 0, version: String::new() }
    }
}

/// 🗂️ Computes [`DwgStructure`] from standard logical drawing concepts.
pub fn compute_dwg_structure(snapshot: &DwgSnapshot) -> DwgStructure {
    let entities = snapshot.drawing.entities();
    DwgStructure {
        layer_count: snapshot.drawing.layers.len() as u32,
        entity_count: entities.len() as u32,
        geometry_value_count: entities.iter().map(|entity| entity.geometry.values.len() as u32).sum(),
        geometry_index_count: entities.iter().map(|entity| entity.geometry.indices.len() as u32).sum(),
        text_character_count: entities.iter().map(|entity| entity.geometry.text.chars().count() as u32).sum(),
        codepage: snapshot.codepage,
        version: snapshot.version.clone(),
    }
}
//#endregion 🔖️Structure

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::snapshot::{DwgEntityBody, DwgEntityCommon, DwgLineEntity, DwgLogicalDrawing, DwgLogicalLayer, DwgLogicalObject, DwgLogicalObjectBody, DwgObjectCategory};

    #[test]
    fn structure_matches_hand_built_logical_drawing() {
        let snapshot = DwgSnapshot {
            schema: "s.stdio.dwg".into(),
            version: "AC1024".into(),
            maintenance_version: 3,
            codepage: 30,
            drawing: DwgLogicalDrawing {
                layers: vec![DwgLogicalLayer { name: "0".into(), color: 7 }],
                objects: vec![DwgLogicalObject {
                    handle: 1,
                    type_code: 19,
                    class_name: "LINE".into(),
                    category: DwgObjectCategory::Entity,
                    body: Some(DwgLogicalObjectBody::Entity(DwgEntityBody::Line(DwgLineEntity {
                        common: DwgEntityCommon { linetype_scale: 1.0, lineweight: 29, ..Default::default() },
                        start: vec![1.0, 2.0, 3.0],
                        end: vec![4.0, 5.0, 6.0],
                        thickness: 0.0,
                        extrusion: vec![0.0, 0.0, 1.0],
                    }))),
                    ..Default::default()
                }],
                ..Default::default()
            },
            ..Default::default()
        };
        let structure = compute_dwg_structure(&snapshot);
        assert_eq!(structure.layer_count, 1);
        assert_eq!(structure.entity_count, 1);
        assert_eq!(structure.geometry_value_count, 6);
        assert_eq!(structure.geometry_index_count, 0);
        assert_eq!(structure.text_character_count, 0);
        assert_eq!(structure.codepage, 30);
        assert_eq!(structure.version, "AC1024");
    }

    #[test]
    fn inference_determinism_law() {
        let snapshot = DwgSnapshot { schema: "s.stdio.dwg".into(), version: "AC1024".into(), codepage: 30, ..Default::default() };
        assert_eq!(compute_dwg_structure(&snapshot), compute_dwg_structure(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_dwg_structure(&DwgSnapshot::default()), DwgStructure::default());
    }
}
//#endregion 🧪️Tests
