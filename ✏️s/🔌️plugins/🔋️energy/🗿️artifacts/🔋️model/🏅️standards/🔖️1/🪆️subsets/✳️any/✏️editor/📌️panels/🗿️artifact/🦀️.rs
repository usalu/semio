//! 🗿️ Energy model editor panel — the artifact tree (outliner): every document entity as one row,
//! in the order a building modeller reads a model (site, then the zones with their spaces, surfaces
//! and windows, then the free-standing shading, then the catalogues those surfaces reference, then
//! the loads, controls, plant and schedules that drive them).
//!
//! 🕹️ The tree is bound to the framework-owned `"energyModel"` interaction domain and every row's key
//! is the RAW `EntityId` (`energy_target_id`), so a row click, a 3d viewport pick and the inspector
//! address the ONE selection.
//!
//! 🪙️ One interactive-row page for the WHOLE tree, split max-min fair across the eleven sections
//! (`section_quotas`, ported from fem2d): a section that fits keeps all its rows and only the widest
//! truncate, each closing with a `+N` continuation row rather than failing an argument admission
//! mid-row. Rows carry exactly ONE action — their pick. Verbs (delete) live in the inspector as one
//! grouped action row, because a page of rows carrying two argument maps each is what exhausts the
//! process-wide `UiValue` argument arena and starves every panel rendered beside the tree.
//!
//! 🪪️ A family `energy_entity_kind` does not resolve (humidistats, air/plant loops, schedules — the
//! last in its own `ScheduleId` space, which would collide with `EntityId` on the wire) is rendered
//! READ-ONLY, so the tree never hands the inspector a target it cannot resolve back.

use crate::editor::model::interaction::{
    energy_target_id, EnergyModelInteractionSnapshot, ENERGY_GRANULARITY_CONSTRUCTION, ENERGY_GRANULARITY_FENESTRATION, ENERGY_GRANULARITY_GAS_MATERIAL, ENERGY_GRANULARITY_GLAZING_MATERIAL, ENERGY_GRANULARITY_HVAC, ENERGY_GRANULARITY_LOAD,
    ENERGY_GRANULARITY_MATERIAL, ENERGY_GRANULARITY_SHADING, ENERGY_GRANULARITY_SPACE, ENERGY_GRANULARITY_SURFACE, ENERGY_GRANULARITY_THERMOSTAT, ENERGY_GRANULARITY_ZONE, ENERGY_MODEL_INTERACTION_DOMAIN,
};
use crate::editor::model::{energy_model_action, surface_class_id, ui_label, ui_node_list};
use crate::model::{Construction, Fenestration, GasMaterial, GlazingMaterial, Material, Model, ShadingSurface, Space, Surface, Zone};
use crate::EnergyModelSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::{BuiltNode, Component, Label as UiLabel};
use semio_framework_plugin::{
    paged_panel_section, panel_page_rows, tree_item_desc, tree_item_with_action, ActionId, Locale, LocalizedLabel, PanelGroup, PanelRowBudget, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiFixedList,
    UiText, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID,
};

//#region 🔖️Constants
pub const BODY_KEY: &str = "energy.model.artifact";
/// 🌳️ Id namespace every SECTION key is minted under; entity row keys stay RAW `EntityId`s.
pub const TREE_NAMESPACE: &str = "energy-model-artifact";
/// 🧾️ Sections this tree assembles, in order — site, zones, shading, materials, glazing materials,
/// gas gaps, constructions, internal loads, controls, HVAC, schedules.
pub const SECTIONS: usize = 11;
/// 🎯️ Ids the builder records as selected/hovered — one `UiFixedList`, so a wider selection marks
/// its first page rather than refusing the whole render.
const MARKED_IDS_LIMIT: usize = semio_framework_ui_contract::UI_FIXED_LIST_ITEMS;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"), group: PanelGroup::Workbench, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🗣️Language
/// 🗣️ Picks one of the two authored languages for the OS-owned locale — the same helper the
/// simulation window uses, rather than a terminology roster this editor does not have.
fn say(locale: Locale, en: &'static str, de: &'static str) -> &'static str {
    if locale == Locale::De {
        de
    } else {
        en
    }
}
//#endregion 🗣️Language

//#region 🔖️UiValues
fn capacity_error(what: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", what)
}

fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value.as_ref()).map(UiValue::Text).ok_or_else(|| capacity_error("energy artifact UI text admission failed"))
}

/// 🗺️ `UiMapBuilder` admits keys in ascending order only, so every caller lists them sorted.
fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| capacity_error("energy artifact UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| capacity_error("energy artifact UI map entry admission failed"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

fn icon_text(icon: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(icon).ok_or_else(|| capacity_error("energy artifact icon admission failed"))
}
//#endregion 🔖️UiValues

//#region 🔖️Formatting
/// 🔢️ Display spelling of one scalar: two decimals, typographic minus, no exponent — every quantity
/// a building model shows in a tree row (m², m³, W/m²K) is a human-scale number.
fn scalar(value: f64) -> String {
    let text = format!("{value:.2}");
    match text.strip_prefix('-') {
        Some(rest) => format!("−{rest}"),
        None => text,
    }
}

pub fn zone_label(zone: &Zone) -> String {
    format!("{} · {} m³", zone.name, scalar(zone.volume_m3))
}

pub fn space_label(space: &Space) -> String {
    format!("{} · {} m²", space.name, scalar(space.floor_area_m2))
}

pub fn surface_label(surface: &Surface) -> String {
    format!("{} · {}", surface.name, surface_class_id(surface.class))
}

pub fn fenestration_label(fenestration: &Fenestration) -> String {
    format!("{} · {} m²", fenestration.name, scalar(fenestration.area_m2))
}

pub fn shading_label(shading: &ShadingSurface) -> String {
    format!("{} · {} pt", shading.name, shading.vertices_m.len())
}

pub fn material_label(material: &Material) -> String {
    format!("{} · {} m · {} W/mK", material.name, scalar(material.thickness_m), scalar(material.conductivity_w_m_k))
}

pub fn glazing_material_label(material: &GlazingMaterial) -> String {
    format!("{} · {} m", material.name, scalar(material.thickness_m))
}

pub fn gas_material_label(material: &GasMaterial) -> String {
    format!("{} · {} m · {:?}", material.name, scalar(material.thickness_m), material.gas)
}

pub fn construction_label(construction: &Construction) -> String {
    format!("{} · {} layers", construction.name, construction.layer_material_ids.len())
}

pub fn site_description(model: &Model) -> String {
    format!("{}° / {}° · {} m · UTC{:+}", scalar(model.site.latitude_deg), scalar(model.site.longitude_deg), scalar(model.site.elevation_m), model.site.time_zone_hours)
}
//#endregion 🔖️Formatting

//#region 🔖️Rows
/// 🕹️ A tree-row pick in the `"energyModel"` domain — the very `interactionSelect` a 3d viewport
/// pick dispatches, so a row click and a scene click land in ONE selection.
fn pick_action(granularity: &str, id: &str) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    let target = protocol::InteractionTarget { granularity: granularity.into(), id: id.into() };
    let targets = protocol::json::to_json_string(&protocol::DslValue::Array(vec![protocol::ToValue::to_value(&target)]));
    let args = ui_value_map([("domainId", ui_value_text(ENERGY_MODEL_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)])?;
    energy_model_action(INTERACTION_SELECT_ACTION_ID, Some(args))
}

/// 🌳️ One selectable entity row: keyed by the raw `EntityId`, picking through the framework domain.
/// It authors ONE argument map (its pick) and no row actions — see this module's own doc.
fn entity_row(id: &str, granularity: &str, label: &str, description: &str, icon: &str, dimmed: bool) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item_with_action(id, UiLabel(UiText::clipped(label)), None, pick_action(granularity, id)?)?;
    let Component::TreeItem(props) = &mut item.component else {
        return Err(capacity_error("energy artifact row is a tree item"));
    };
    props.icon = Some(icon_text(icon)?);
    props.description = Some(UiText::clipped(description));
    props.dimmed = Some(dimmed);
    props.default_open = Some(false);
    Ok(item)
}

/// 📖️ One read-only row: a family no interaction granularity resolves (a schedule, an air loop), or
/// the singleton site. It binds NO action, so it costs nothing of the argument arena.
fn read_row(id: String, label: &str, description: &str) -> UiAssemblyResult<BuiltNode> {
    tree_item_desc(id, UiLabel(UiText::clipped(label)), Some(description.to_string()))
}
//#endregion 🔖️Rows

//#region 🔖️Sections
fn section_label(noun: &str, count: usize) -> UiAssemblyResult<UiLabel> {
    ui_label(format!("{noun} ({count})"))
}

/// 📍️ The site: one read-only row. The site is a singleton with no `EntityId`, so it is picked from
/// the inspector's own document summary rather than from a domain target.
fn site_section(model: &Model, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    ui_node_list([read_row(format!("{TREE_NAMESPACE}.site.row"), say(locale, "Site", "Standort"), &site_description(model))])
}

/// 🏘️ Zones and everything they own: each zone row nests its spaces and its surfaces, and each
/// surface row nests its own windows.
fn zones_section(model: &Model, interaction: &EnergyModelInteractionSnapshot, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let section_id = format!("{TREE_NAMESPACE}.zones");
    let rows = budget.remaining();
    paged_panel_section(&section_id, &model.zones, rows, budget, |zone, nested| {
        let mut item = entity_row(&energy_target_id(zone.id), ENERGY_GRANULARITY_ZONE, &zone_label(zone), say(locale, "Zone", "Zone"), "box", !zone.conditioned)?;
        let spaces: Vec<&Space> = model.spaces.iter().filter(|space| space.zone_id == zone.id).collect();
        let surfaces: Vec<&Surface> = model.surfaces.iter().filter(|surface| surface.zone_id == zone.id).collect();
        let space_rows = nested.remaining();
        let space_items = paged_panel_section(&format!("{section_id}.{}.spaces", zone.id.0), &spaces, space_rows, nested, |space, _| {
            entity_row(&energy_target_id(space.id), ENERGY_GRANULARITY_SPACE, &space_label(space), say(locale, "Space", "Raum"), "layout-grid", false)
        })?;
        let surface_items = surfaces_with_windows(&section_id, model, &surfaces, interaction, locale, nested)?;
        for row in space_items.into_iter().chain(surface_items) {
            item.children.try_push(row).map_err(|_| capacity_error("energy artifact zone child row admission failed"))?;
        }
        if let Component::TreeItem(props) = &mut item.component {
            props.default_open = Some(true);
        }
        Ok(item)
    })
}

/// 🎯️ True when this id is the framework-owned selection's or the pointer hover's.
fn is_marked(interaction: &EnergyModelInteractionSnapshot, id: &str) -> bool {
    interaction.selected_ids.iter().any(|marked| marked == id) || interaction.hovered_ids.iter().any(|marked| marked == id)
}

/// 🟫️ One zone's surfaces with their windows nested inside them — hand-paged instead of
/// `paged_panel_section`, because that helper reserves exactly ONE row per remaining sibling surface
/// and hands the rest to the current one. Under a tight page (the arena headroom shrinks while the
/// inspector holds a form of bound controls) that leaves the FIRST surface too little for its own
/// windows, and they collapse into a `…windows.more` marker no tree row can expand — a user could
/// not reach a window while its host wall was selected (first browser probe of this ticket).
///
/// 🎯️ So the allowance is handed out in two passes: the surface the domain currently marks — picked
/// in the 3d viewport, hovered, or owning a marked window — claims its whole demand FIRST, and the
/// rest follow in document order. Rows are then BUILT in document order, so the tree never reshuffles
/// under the reader's cursor; only who survives a truncation changes.
fn surfaces_with_windows(section_id: &str, model: &Model, surfaces: &[&Surface], interaction: &EnergyModelInteractionSnapshot, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let windows_of = |surface: &Surface| model.fenestrations.iter().filter(|window| window.surface_id == surface.id).collect::<Vec<_>>();
    let marked = |surface: &Surface| is_marked(interaction, &energy_target_id(surface.id)) || windows_of(surface).iter().any(|window| is_marked(interaction, &energy_target_id(window.id)));

    // 1️⃣ BREADTH first: one row per surface, in document order, so a truncation never hides a whole
    // wall behind the windows of the wall before it.
    let mut allowance = vec![0usize; surfaces.len()];
    let mut left = budget.remaining();
    for slot in allowance.iter_mut() {
        if left == 0 {
            break;
        }
        *slot = 1;
        left -= 1;
    }
    // 2️⃣ DEPTH second: what the surface rows left over buys windows — the MARKED surface's first, so
    // a picked wall always shows its own windows, then the rest in document order.
    let mut order: Vec<usize> = (0..surfaces.len()).collect();
    order.sort_by_key(|index| usize::from(!marked(surfaces[*index])));
    for index in order {
        if left == 0 {
            break;
        }
        if allowance[index] == 0 {
            continue;
        }
        let give = windows_of(surfaces[index]).len().min(left);
        allowance[index] += give;
        left -= give;
    }

    let mut items = UiFixedList::default();
    let mut placed = 0;
    for (index, surface) in surfaces.iter().enumerate() {
        let share = allowance[index];
        if share == 0 || !budget.spend() {
            continue;
        }
        let dangling = !model.constructions.iter().any(|entry| entry.id == surface.construction_id);
        let row = entity_row(&energy_target_id(surface.id), ENERGY_GRANULARITY_SURFACE, &surface_label(surface), say(locale, "Surface", "Fläche"), "square", dangling);
        let mut row = match row {
            Ok(row) => row,
            Err(error) if error.code == "ui.fixed-capacity" => break,
            Err(error) => return Err(error),
        };
        let windows = windows_of(surface);
        let window_items = paged_panel_section(&format!("{section_id}.{}.windows", surface.id.0), &windows, share - 1, budget, |window, _| {
            entity_row(&energy_target_id(window.id), ENERGY_GRANULARITY_FENESTRATION, &fenestration_label(window), say(locale, "Window", "Fenster"), "app-window", false)
        })?;
        for window in window_items {
            row.children.try_push(window).map_err(|_| capacity_error("energy artifact window row admission failed"))?;
        }
        if let Component::TreeItem(props) = &mut row.component {
            props.default_open = Some(true);
        }
        if items.try_push(row).is_err() {
            break;
        }
        placed += 1;
    }
    if placed < surfaces.len() {
        if let Ok(more) = semio_framework_plugin::panel_continuation_row(&format!("{section_id}.surfaces"), surfaces.len() - placed) {
            let _ = items.try_push(more);
        }
    }
    Ok(items)
}

fn shading_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.shading"), &model.shading_surfaces, rows, budget, |shading, _| {
        entity_row(&energy_target_id(shading.id), ENERGY_GRANULARITY_SHADING, &shading_label(shading), say(locale, "Shading", "Verschattung"), "umbrella", shading.vertices_m.len() < 3)
    })
}

fn materials_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.materials"), &model.materials, rows, budget, |material, _| {
        entity_row(&energy_target_id(material.id), ENERGY_GRANULARITY_MATERIAL, &material_label(material), say(locale, "Material", "Material"), "layers", false)
    })
}

fn glazing_materials_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.glazing-materials"), &model.glazing_materials, rows, budget, |material, _| {
        entity_row(&energy_target_id(material.id), ENERGY_GRANULARITY_GLAZING_MATERIAL, &glazing_material_label(material), say(locale, "Glazing", "Verglasung"), "panel-top", false)
    })
}

fn gas_materials_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.gas-materials"), &model.gas_materials, rows, budget, |material, _| {
        entity_row(&energy_target_id(material.id), ENERGY_GRANULARITY_GAS_MATERIAL, &gas_material_label(material), say(locale, "Gas gap", "Gasfüllung"), "wind", false)
    })
}

fn constructions_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.constructions"), &model.constructions, rows, budget, |construction, _| {
        let dangling = construction.layer_material_ids.iter().any(|layer| !model.materials.iter().any(|material| material.id == *layer) && !model.glazing_materials.iter().any(|material| material.id == *layer) && !model.gas_materials.iter().any(|material| material.id == *layer));
        entity_row(&energy_target_id(construction.id), ENERGY_GRANULARITY_CONSTRUCTION, &construction_label(construction), say(locale, "Construction", "Konstruktion"), "bricks", dangling)
    })
}

/// ⚡️ Internal loads: the four families that share the `load` granularity, flattened into one
/// section because each is tiny and their rows compete for the same page.
fn loads_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let entries = load_entries(model, locale);
    let rows = budget.remaining();
    paged_panel_section(&format!("{TREE_NAMESPACE}.loads"), &entries, rows, budget, |(id, label, description, icon), _| entity_row(id, ENERGY_GRANULARITY_LOAD, label, description, icon, false))
}

fn load_entries(model: &Model, locale: Locale) -> Vec<(String, String, &'static str, &'static str)> {
    let mut entries = Vec::new();
    for people in &model.people {
        entries.push((energy_target_id(people.id), format!("{} · {} p/m²", say(locale, "People", "Personen"), scalar(people.people_per_area)), say(locale, "People", "Personen"), "users"));
    }
    for lighting in &model.lighting {
        entries.push((energy_target_id(lighting.id), format!("{} · {} W/m²", say(locale, "Lighting", "Beleuchtung"), scalar(lighting.watts_per_area)), say(locale, "Lighting", "Beleuchtung"), "lightbulb"));
    }
    for equipment in &model.equipment {
        entries.push((energy_target_id(equipment.id), format!("{} · {} W/m²", say(locale, "Equipment", "Geräte"), scalar(equipment.watts_per_area)), say(locale, "Equipment", "Geräte"), "plug"));
    }
    for infiltration in &model.infiltrations {
        entries.push((energy_target_id(infiltration.id), format!("{} · {} ACH", say(locale, "Infiltration", "Infiltration"), scalar(infiltration.design_flow_ach)), say(locale, "Infiltration", "Infiltration"), "wind"));
    }
    entries
}

/// 🌡️ Controls: thermostats pick at their own granularity; humidistats are read-only because
/// `energy_entity_kind` resolves no `humidistat` family, and a row must never hand the inspector a
/// target it cannot resolve back.
fn controls_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let section_id = format!("{TREE_NAMESPACE}.controls");
    let rows = budget.remaining();
    let mut items = paged_panel_section(&section_id, &model.thermostats, rows, budget, |thermostat, _| {
        entity_row(
            &energy_target_id(thermostat.id),
            ENERGY_GRANULARITY_THERMOSTAT,
            &format!("{} {} · ±{} K", say(locale, "Thermostat", "Thermostat"), thermostat.id.0, scalar(thermostat.heating_throttle_range_k)),
            say(locale, "Thermostat", "Thermostat"),
            "thermometer",
            false,
        )
    })?;
    for humidistat in &model.humidistats {
        let row = read_row(format!("{section_id}.humidistat.{}", humidistat.id.0), &format!("{} {}", say(locale, "Humidistat", "Hygrostat"), humidistat.id.0), say(locale, "Humidistat", "Hygrostat"))?;
        if items.try_push(row).is_err() {
            break;
        }
    }
    Ok(items)
}

/// 🌬️ HVAC: ideal-loads systems pick at the `hvac` granularity; air and plant loops are read-only
/// until the shared resolver names them.
fn hvac_section(model: &Model, locale: Locale, budget: &mut PanelRowBudget) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let section_id = format!("{TREE_NAMESPACE}.hvac");
    let rows = budget.remaining();
    let mut items = paged_panel_section(&section_id, &model.ideal_loads, rows, budget, |system, _| {
        entity_row(
            &energy_target_id(system.id),
            ENERGY_GRANULARITY_HVAC,
            &format!("{} {}", say(locale, "Ideal loads", "Idealanlage"), system.id.0),
            say(locale, "Ideal loads", "Idealanlage"),
            "fan",
            false,
        )
    })?;
    for air_loop in &model.air_loops {
        let row = read_row(format!("{section_id}.air-loop.{}", air_loop.id.0), &air_loop.name, say(locale, "Air loop", "Luftkreis"))?;
        if items.try_push(row).is_err() {
            return Ok(items);
        }
    }
    for plant_loop in &model.plant_loops {
        let row = read_row(format!("{section_id}.plant-loop.{}", plant_loop.id.0), &plant_loop.name, say(locale, "Plant loop", "Anlagenkreis"))?;
        if items.try_push(row).is_err() {
            return Ok(items);
        }
    }
    Ok(items)
}

/// 🗓️ Schedules, read-only and free: a `ScheduleId` lives in its OWN id space, so a schedule row
/// keyed by its raw number would collide with an `EntityId` on the interaction wire.
fn schedules_section(model: &Model, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let section_id = format!("{TREE_NAMESPACE}.schedules");
    let schedules = &model.schedules;
    let counts = [
        (say(locale, "Constant", "Konstant"), schedules.constants.len()),
        (say(locale, "Daily", "Täglich"), schedules.daily.len()),
        (say(locale, "Weekly", "Wöchentlich"), schedules.weekly.len()),
        (say(locale, "Annual", "Jährlich"), schedules.annual.len()),
        (say(locale, "Time series", "Zeitreihe"), schedules.time_series.len()),
    ];
    ui_node_list(counts.into_iter().enumerate().map(|(index, (label, count))| read_row(format!("{section_id}.{index}"), label, &count.to_string())))
}
//#endregion 🔖️Sections

//#region 🔖️Render
fn marked_ids(ids: &[String]) -> Vec<String> {
    ids.iter().take(MARKED_IDS_LIMIT).cloned().collect()
}

/// 🧾️ Interactive rows each section wants — the read-only families (site, schedules, humidistats,
/// air/plant loops) demand nothing, because they bind no argument map.
pub fn section_demands(model: &Model) -> [usize; SECTIONS] {
    [
        0,
        model.zones.len() + model.spaces.len() + model.surfaces.len() + model.fenestrations.len(),
        model.shading_surfaces.len(),
        model.materials.len(),
        model.glazing_materials.len(),
        model.gas_materials.len(),
        model.constructions.len(),
        model.people.len() + model.lighting.len() + model.equipment.len() + model.infiltrations.len(),
        model.thermostats.len(),
        model.ideal_loads.len(),
        0,
    ]
}

/// 🪙️ Max-min fair split of one interactive-row page across the sections: the quota is the highest
/// per-section ceiling the page affords, every section under it gets ALL its rows, and the spare goes
/// to the widest sections. Ported verbatim from fem2d's artifact panel — reserving "the rows my
/// siblings still need" instead (cad's symmetric panes) would let the zones section eat the whole
/// page and leave the catalogues one row each.
pub fn section_quotas(demands: [usize; SECTIONS], page: usize) -> [usize; SECTIONS] {
    let mut ceiling = 0;
    while ceiling < page && demands.iter().map(|demand| (*demand).min(ceiling + 1)).sum::<usize>() <= page {
        ceiling += 1;
    }
    let mut quotas = demands.map(|demand| demand.min(ceiling));
    let mut spare = page.saturating_sub(quotas.iter().sum::<usize>());
    while spare > 0 {
        let Some(index) = (0..SECTIONS).filter(|index| quotas[*index] < demands[*index]).max_by_key(|index| demands[*index] - quotas[*index]) else {
            break;
        };
        quotas[index] += 1;
        spare -= 1;
    }
    quotas
}

/// 🪙️ Runs one section against its quota, reserving the whole rest of the page for its siblings —
/// and settling whatever it under-spends straight back into the shared budget.
fn with_quota<R>(budget: &mut PanelRowBudget, quota: usize, build: impl FnOnce(&mut PanelRowBudget) -> R) -> R {
    let reserved = budget.remaining().saturating_sub(quota);
    budget.nested(reserved, build)
}

/// 🌳️ The outliner: eleven sections over one interactive-row page, bound to the `"energyModel"`
/// domain and marked from the live interaction snapshot.
pub fn build_artifact_tree(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let model = &snapshot.model;
    let page = panel_page_rows();
    let quotas = section_quotas(section_demands(model), page);
    let budget = &mut PanelRowBudget::new(page);
    let site = site_section(model, locale)?;
    let zones = with_quota(budget, quotas[1], |share| zones_section(model, interaction, locale, share))?;
    let shading = with_quota(budget, quotas[2], |share| shading_section(model, locale, share))?;
    let materials = with_quota(budget, quotas[3], |share| materials_section(model, locale, share))?;
    let glazing = with_quota(budget, quotas[4], |share| glazing_materials_section(model, locale, share))?;
    let gases = with_quota(budget, quotas[5], |share| gas_materials_section(model, locale, share))?;
    let constructions = with_quota(budget, quotas[6], |share| constructions_section(model, locale, share))?;
    let loads = with_quota(budget, quotas[7], |share| loads_section(model, locale, share))?;
    let controls = with_quota(budget, quotas[8], |share| controls_section(model, locale, share))?;
    let hvac = with_quota(budget, quotas[9], |share| hvac_section(model, locale, share))?;
    let schedules = schedules_section(model, locale)?;
    let placeholder = ui_label(say(locale, "None", "Keine"))?;

    let builder = PanelTreeBuilder::new(TREE_NAMESPACE)?
        .section(format!("{TREE_NAMESPACE}.site"), Some(ui_label(say(locale, "Site", "Standort"))?), true, site)?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.zones"), Some(section_label(say(locale, "Zones", "Zonen"), model.zones.len())?), true, zones, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.shading"), Some(section_label(say(locale, "Shading surfaces", "Verschattungsflächen"), model.shading_surfaces.len())?), false, shading, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.materials"), Some(section_label(say(locale, "Materials", "Materialien"), model.materials.len())?), false, materials, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.glazing-materials"), Some(section_label(say(locale, "Glazing materials", "Verglasungsmaterialien"), model.glazing_materials.len())?), false, glazing, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.gas-materials"), Some(section_label(say(locale, "Gas gaps", "Gasfüllungen"), model.gas_materials.len())?), false, gases, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.constructions"), Some(section_label(say(locale, "Constructions", "Konstruktionen"), model.constructions.len())?), false, constructions, placeholder.clone())?
        .section_or_placeholder(format!("{TREE_NAMESPACE}.loads"), Some(section_label(say(locale, "Internal loads", "Innere Lasten"), load_entries(model, locale).len())?), false, loads, placeholder.clone())?
        .section_or_placeholder(
            format!("{TREE_NAMESPACE}.controls"),
            Some(section_label(say(locale, "Controls", "Regelung"), model.thermostats.len() + model.humidistats.len())?),
            false,
            controls,
            placeholder.clone(),
        )?
        .section_or_placeholder(
            format!("{TREE_NAMESPACE}.hvac"),
            Some(section_label(say(locale, "HVAC", "Anlagentechnik"), model.ideal_loads.len() + model.air_loops.len() + model.plant_loops.len())?),
            false,
            hvac,
            placeholder,
        )?
        .section(format!("{TREE_NAMESPACE}.schedules"), Some(ui_label(say(locale, "Schedules", "Zeitpläne"))?), false, schedules)?
        .interaction_domain(ENERGY_MODEL_INTERACTION_DOMAIN)?
        .selected(marked_ids(&interaction.selected_ids))?
        .highlighted(marked_ids(&interaction.hovered_ids))?;
    builder.build()
}

pub fn render(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    build_artifact_tree(snapshot, interaction, locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
