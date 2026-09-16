//! 🕹️ Energy model editor — the framework-owned interaction domain: its definition, the
//! selection/hover snapshot every render reads, and the one lookup that tells which entity family a
//! raw target id belongs to. A target id is the raw `EntityId` (`id.0.to_string()`), so the artifact
//! tree, the inspector and the 3d model window all agree on one vocabulary. Picking itself is
//! framework-owned: a World3d scene stamped with this domain lets the react host dispatch
//! `interactionSelect`/`interactionHover` without any plugin command.

use crate::EnergyModelSnapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, LocalizedLabel, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};

//#region 🔖️Constants
pub const ENERGY_MODEL_INTERACTION_DOMAIN: &str = "energyModel";
pub const ENERGY_GRANULARITY_ZONE: &str = "zone";
pub const ENERGY_GRANULARITY_SPACE: &str = "space";
pub const ENERGY_GRANULARITY_SURFACE: &str = "surface";
pub const ENERGY_GRANULARITY_FENESTRATION: &str = "fenestration";
pub const ENERGY_GRANULARITY_SHADING: &str = "shading";
pub const ENERGY_GRANULARITY_MATERIAL: &str = "material";
pub const ENERGY_GRANULARITY_GLAZING_MATERIAL: &str = "glazingMaterial";
pub const ENERGY_GRANULARITY_GAS_MATERIAL: &str = "gasMaterial";
pub const ENERGY_GRANULARITY_CONSTRUCTION: &str = "construction";
pub const ENERGY_GRANULARITY_THERMOSTAT: &str = "thermostat";
pub const ENERGY_GRANULARITY_LOAD: &str = "load";
pub const ENERGY_GRANULARITY_HVAC: &str = "hvac";
pub const ENERGY_GRANULARITY_SCHEDULE: &str = "schedule";
pub const ENERGY_POINTER_CHANNEL: &str = "pointer";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🕹️ The one energy interaction domain: every document entity family is a granularity.
pub fn energy_model_interaction_definition() -> InteractionDefinition {
    let granularity = |id: &str, en: &str, de: &str, icon: &str| GranularityDefinition { id: id.into(), label: LocalizedLabel::native(en, de), icon_id: icon.into() };
    InteractionDefinition {
        id: ENERGY_MODEL_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Energy model", "Energiemodell"),
        granularities: vec![
            granularity(ENERGY_GRANULARITY_ZONE, "Zone", "Zone", "box"),
            granularity(ENERGY_GRANULARITY_SPACE, "Space", "Raum", "layout-grid"),
            granularity(ENERGY_GRANULARITY_SURFACE, "Surface", "Fläche", "square"),
            granularity(ENERGY_GRANULARITY_FENESTRATION, "Window", "Fenster", "app-window"),
            granularity(ENERGY_GRANULARITY_SHADING, "Shading", "Verschattung", "umbrella"),
            granularity(ENERGY_GRANULARITY_MATERIAL, "Material", "Material", "layers"),
            granularity(ENERGY_GRANULARITY_GLAZING_MATERIAL, "Glazing", "Verglasung", "panel-top"),
            granularity(ENERGY_GRANULARITY_GAS_MATERIAL, "Gas gap", "Gasfüllung", "wind"),
            granularity(ENERGY_GRANULARITY_CONSTRUCTION, "Construction", "Konstruktion", "bricks"),
            granularity(ENERGY_GRANULARITY_THERMOSTAT, "Thermostat", "Thermostat", "thermometer"),
            granularity(ENERGY_GRANULARITY_LOAD, "Internal load", "Innere Last", "zap"),
            granularity(ENERGY_GRANULARITY_HVAC, "HVAC", "Anlagentechnik", "fan"),
            granularity(ENERGY_GRANULARITY_SCHEDULE, "Schedule", "Zeitplan", "calendar"),
        ],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}
//#endregion 🔖️Definition

//#region 🔖️Snapshot
/// 🕹️ The immutable interaction snapshot one render reads: selected and pointer-hovered entity ids.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnergyModelInteractionSnapshot {
    pub selected_ids: Vec<String>,
    pub hovered_ids: Vec<String>,
}

impl EnergyModelInteractionSnapshot {
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self { selected_ids: interaction.selection(ENERGY_MODEL_INTERACTION_DOMAIN).ids.clone(), hovered_ids: interaction.hover(ENERGY_MODEL_INTERACTION_DOMAIN, ENERGY_POINTER_CHANNEL).ids.clone() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Identity
/// 🪪️ One entity family's target id — the raw `EntityId`, spelled once here.
pub fn energy_target_id(id: crate::model::EntityId) -> String {
    id.0.to_string()
}

/// 🪪️ The granularity one raw target id belongs to, in a fixed precedence (geometry first, then
/// the catalogue families). `EntityId` is ONE shared space across every `Model` collection, so the
/// first family that owns the id wins.
pub fn energy_entity_kind(snapshot: &EnergyModelSnapshot, id: &str) -> Option<&'static str> {
    let Ok(raw) = id.parse::<u32>() else { return None };
    let id = crate::model::EntityId(raw);
    let model = &snapshot.model;
    if model.surfaces.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_SURFACE);
    }
    if model.fenestrations.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_FENESTRATION);
    }
    if model.shading_surfaces.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_SHADING);
    }
    if model.zones.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_ZONE);
    }
    if model.spaces.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_SPACE);
    }
    if model.materials.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_MATERIAL);
    }
    if model.glazing_materials.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_GLAZING_MATERIAL);
    }
    if model.gas_materials.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_GAS_MATERIAL);
    }
    if model.constructions.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_CONSTRUCTION);
    }
    if model.thermostats.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_THERMOSTAT);
    }
    if model.people.iter().any(|item| item.id == id) || model.lighting.iter().any(|item| item.id == id) || model.equipment.iter().any(|item| item.id == id) || model.infiltrations.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_LOAD);
    }
    if model.ideal_loads.iter().any(|item| item.id == id) {
        return Some(ENERGY_GRANULARITY_HVAC);
    }
    None
}
//#endregion 🔖️Identity

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
