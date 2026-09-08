//! 🗂 `structure` — logical drawing statistics derived only from modeled layers and entities.

use crate::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot;

//#region 🔖️Structure
/// 🗂️ Dwg (ac1024) logical drawing statistics.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(Default)]
pub struct DwgStructure {
    pub layer_count: u32,
    pub entity_count: u32,
    pub geometry_value_count: u32,
    pub geometry_index_count: u32,
    pub text_character_count: u32,
    pub codepage: u16,
    pub version: String,
}


/// 🗂️ Computes [`DwgStructure`] from standard logical drawing concepts.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
