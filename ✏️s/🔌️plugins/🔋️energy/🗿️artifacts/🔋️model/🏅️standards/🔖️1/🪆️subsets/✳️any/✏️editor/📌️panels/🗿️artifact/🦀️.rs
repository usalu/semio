//! 🗿️ Energy model editor panel — the artifact tree (outliner): every document entity as one row,
//! in the order a building modeller reads a model (site, then the zones with their spaces, surfaces
//! and windows, then the free-standing shading, then the catalogues those surfaces reference, then
//! the loads, controls, plant and schedules that drive them).
//!
//! 🕹️ The tree is bound to the framework-owned `"energyModel"` interaction domain and every row's key
//! is the RAW `EntityId` (`energy_target_id`), so a row click, a 3d viewport pick and the inspector
//! address the ONE selection. A pick row carries NO argument map of its own: it declares its
//! `granularity` and the tree carries ONE `interactionSelect` binding for all of them, which is what
//! keeps a whole document's worth of rows off the process-wide `UiValue` argument arena.
//!
//! 🪟️ The tree is VIRTUALISED, never paged: every container (each section, each zone, each surface)
//! stamps the full `total` of its logical child list and materialises only the window the host asked
//! for (`TreeWindows::for_body`, threaded in from the editor's `render_body`). There is no `+N` row,
//! no row quota and no starvation — a closed container costs one stamped `total` and nothing else, and
//! scrolling or expanding asks the host for the next slice.
//!
//! 🪪️ A family `energy_entity_kind` does not resolve (humidistats, air/plant loops, schedules — the
//! last in its own `ScheduleId` space, which would collide with `EntityId` on the wire) is rendered
//! READ-ONLY, so the tree never hands the inspector a target it cannot resolve back.

use crate::editor_interaction::{
    energy_target_id, EnergyModelInteractionSnapshot, ENERGY_GRANULARITY_CONSTRUCTION, ENERGY_GRANULARITY_FENESTRATION, ENERGY_GRANULARITY_GAS_MATERIAL, ENERGY_GRANULARITY_GLAZING_MATERIAL, ENERGY_GRANULARITY_HVAC, ENERGY_GRANULARITY_LOAD,
    ENERGY_GRANULARITY_MATERIAL, ENERGY_GRANULARITY_SHADING, ENERGY_GRANULARITY_SPACE, ENERGY_GRANULARITY_SURFACE, ENERGY_GRANULARITY_THERMOSTAT, ENERGY_GRANULARITY_ZONE, ENERGY_MODEL_INTERACTION_DOMAIN,
};
use crate::editor_commands::{energy_model_action, surface_class_id, ui_label, ENERGY_MODEL_EDITOR_CONTROLLER_ID};
use crate::model::{Construction, Fenestration, GasMaterial, GlazingMaterial, Material, ShadingSurface, Space, Surface, Zone};
use crate::snapshot::state::EnergyRowsView;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::Buildable;
use semio_framework_plugin::plugin_app_close_prelude::BuiltNode;
use semio_framework_plugin::plugin_app_close_prelude::HasBase;
use semio_framework_ui_contract::Label as UiLabel;
use semio_framework_plugin::activation_target;
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::tree_window_indexed_item;
use semio_framework_plugin::ActionId;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "energy.model.artifact";
/// 🌳️ Id namespace every SECTION key is minted under; entity row keys stay RAW `EntityId`s.
pub const TREE_NAMESPACE: &str = "energy-model-artifact";
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

fn ui_text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| capacity_error("energy artifact UI text admission failed"))
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

pub fn site_description(model: &EnergyRowsView<'_>) -> String {
    format!("{}° / {}° · {} m · UTC{:+}", scalar(model.site.latitude_deg), scalar(model.site.longitude_deg), scalar(model.site.elevation_m), model.site.time_zone_hours)
}
//#endregion 🔖️Formatting

//#region 🔖️Rows
/// 📍️ The site has no `EntityId`, so it cannot be a domain target. Its row instead dispatches the
/// framework's own CLEARING pick — `merge: replace` with an EMPTY target list, which the framework
/// documents as "clears selection while hover remains" — and the inspector's no-selection body is
/// exactly where the site's editable form lives. So clicking "Site" opens the site form.
fn clear_selection_action() -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    let targets = semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(Vec::new()));
    let args = ui_value_map([("domainId", ui_value_text(ENERGY_MODEL_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)])?;
    energy_model_action(INTERACTION_SELECT_ACTION_ID, Some(args))
}

/// ➖️ How a row reports the framework-owned selection/hover it belongs to. `PanelTreeBuilder::selected`
/// / `::highlighted` are DECLARED on the built tree but do not paint in this SDK wave (lane C verified
/// that), so the state also rides in the row's own label: `●` for selected, `○` for hovered. That is
/// what makes tree ⇄ 3d agreement visible to a reader today; the prefix goes away the moment the
/// builder's own marking reaches the renderer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarkedAs {
    Unmarked,
    Hovered,
    Selected,
}

impl MarkedAs {
    /// 🔄️ The marker this state prefixes a row label with — the empty string when unmarked, so an
    /// unmarked tree reads exactly as it did before.
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Unmarked => "",
            Self::Hovered => "○ ",
            Self::Selected => "● ",
        }
    }

    fn of(interaction: &EnergyModelInteractionSnapshot, id: &str) -> Self {
        if interaction.selected_ids.iter().any(|marked| marked == id) {
            Self::Selected
        } else if interaction.hovered_ids.iter().any(|marked| marked == id) {
            Self::Hovered
        } else {
            Self::Unmarked
        }
    }
}

/// 👆️ One selectable entity row, unbuilt: keyed by the raw `EntityId` and declaring the granularity
/// it picks at. It authors NO argument map and no row actions — the ONE `interactionSelect` the
/// tree itself carries turns a click on this key into a pick of `{granularity, id}`.
fn entity_item(id: &str, granularity: &str, label: &str, description: &str, icon: &str, dimmed: bool, marked: MarkedAs) -> UiAssemblyResult<ui::TreeItemBuilder> {
    let label = format!("{}{label}", marked.prefix());
    let item = ui::tree_item(UiLabel(UiText::clipped(&label))).icon(ui_text(icon)?).description(UiText::clipped(description)).dimmed(dimmed).granularity(ui_text(granularity)?);
    item.try_id(id).map_err(|_| capacity_error("energy artifact row id admission failed"))
}

/// 🍃️ A leaf entity row — an [`entity_item`] with no children of its own.
fn entity_row(id: &str, granularity: &str, label: &str, description: &str, icon: &str, dimmed: bool, marked: MarkedAs) -> UiAssemblyResult<BuiltNode> {
    entity_item(id, granularity, label, description, icon, dimmed, marked)?.default_open(false).try_build().map_err(|_| capacity_error("energy artifact row assembly failed"))
}

/// 📖️ One read-only row: a family no interaction granularity resolves (a schedule, an air loop). It
/// binds NO action and declares NO granularity, so it is never a pick target.
fn read_row(id: String, label: &str, description: &str) -> UiAssemblyResult<BuiltNode> {
    tree_item_desc(id, UiLabel(UiText::clipped(label)), Some(description.to_string()))
}

/// 🧾️ One row of a flat catalogue section before it is built: `(key, label, description, icon,
/// granularity)`. A `None` granularity is a family the interaction resolver does not name
/// (humidistat, air/plant loop, schedule kind) and renders read-only.
type RowEntry = (String, String, String, &'static str, Option<&'static str>);

fn entry_row(entry: &RowEntry, interaction: &EnergyModelInteractionSnapshot) -> UiAssemblyResult<BuiltNode> {
    let (id, label, description, icon, granularity) = entry;
    match *granularity {
        Some(granularity) => entity_row(id, granularity, label, description, icon, false, MarkedAs::of(interaction, id)),
        None => read_row(id.clone(), label, description),
    }
}
//#endregion 🔖️Rows

//#region 🔖️Sections
fn section_label(noun: &str, count: usize) -> UiAssemblyResult<UiLabel> {
    ui_label(format!("{noun} ({count})"))
}

/// ⛏️ The site: one read-only row. The site is a singleton with no `EntityId`, so it is picked from
/// the inspector's own document summary rather than from a domain target.
fn site_row(model: &EnergyRowsView<'_>, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{TREE_NAMESPACE}.site.row");
    let item = ui::tree_item(UiLabel(UiText::clipped(say(locale, "Site", "Standort")))).icon(ui_text("globe")?).description(UiText::clipped(&site_description(model))).dimmed(false).default_open(false);
    let item = item.try_id(&row_id).map_err(|_| capacity_error("energy artifact site row id admission failed"))?.target(activation_target(clear_selection_action()?)?);
    item.try_build().map_err(|_| capacity_error("energy artifact site row assembly failed"))
}

/// 🏘️ Everything one zone owns, as the ONE logical child list its container stamps a `total` over:
/// its spaces first, then its surfaces — each of which nests its own windows. Only row positions are held.
enum ZoneChild {
    Space(usize),
    Surface(usize),
}

fn zone_children(model: &EnergyRowsView<'_>, zone: &Zone) -> Vec<ZoneChild> {
    let id = zone.id;
    let spaces = model.spaces.positions(|space| space.zone_id == id).into_iter().map(ZoneChild::Space);
    let surfaces = model.surfaces.positions(|surface| surface.zone_id == id).into_iter().map(ZoneChild::Surface);
    spaces.chain(surfaces).collect()
}

fn missing_row() -> PluginAssemblyError {
    capacity_error("energy artifact row vanished from the native state")
}

/// 👶️ One zone row and its window of children. The zone reports how many spaces and surfaces it
/// owns whatever the host materialises, so the scrollbar spans the whole zone from first paint.
fn zone_row(windows: &TreeWindows<'_>, model: &EnergyRowsView<'_>, interaction: &EnergyModelInteractionSnapshot, locale: Locale, zone: &Zone) -> UiAssemblyResult<BuiltNode> {
    let zone_id = energy_target_id(zone.id);
    let item = entity_item(&zone_id, ENERGY_GRANULARITY_ZONE, &zone_label(zone), say(locale, "Zone", "Zone"), "box", !zone.conditioned, MarkedAs::of(interaction, &zone_id))?;
    let children = zone_children(model, zone);
    tree_window_indexed_item(windows, item, &zone_id, true, children.len(), |index| match children.get(index).ok_or_else(missing_row)? {
        ZoneChild::Space(at) => {
            let space = model.spaces.get(*at).ok_or_else(missing_row)?;
            let space_id = energy_target_id(space.id);
            entity_row(&space_id, ENERGY_GRANULARITY_SPACE, &space_label(&space), say(locale, "Space", "Raum"), "layout-grid", false, MarkedAs::of(interaction, &space_id))
        }
        ZoneChild::Surface(at) => surface_row(windows, model, interaction, locale, &model.surfaces.get(*at).ok_or_else(missing_row)?),
    })
}

/// 🟫️ One surface row with its own windows nested inside it — the third level of the hierarchy, and
/// its own window container, so a wall with a hundred openings costs the first paint one row.
fn surface_row(windows: &TreeWindows<'_>, model: &EnergyRowsView<'_>, interaction: &EnergyModelInteractionSnapshot, locale: Locale, surface: &Surface) -> UiAssemblyResult<BuiltNode> {
    let construction_id = surface.construction_id;
    let dangling = !model.constructions.holds(|entry| entry.id == construction_id);
    let surface_id = energy_target_id(surface.id);
    let item = entity_item(&surface_id, ENERGY_GRANULARITY_SURFACE, &surface_label(surface), say(locale, "Surface", "Fläche"), "square", dangling, MarkedAs::of(interaction, &surface_id))?;
    let owner = surface.id;
    let openings = model.fenestrations.positions(|window| window.surface_id == owner);
    tree_window_indexed_item(windows, item, &surface_id, true, openings.len(), |index| {
        let window = model.fenestrations.get(*openings.get(index).ok_or_else(missing_row)?).ok_or_else(missing_row)?;
        let window_id = energy_target_id(window.id);
        entity_row(&window_id, ENERGY_GRANULARITY_FENESTRATION, &fenestration_label(&window), say(locale, "Window", "Fenster"), "app-window", false, MarkedAs::of(interaction, &window_id))
    })
}

/// 🧭️ Maps one flat index over consecutive families to `(family, position)`.
fn locate(counts: &[usize], mut index: usize) -> Option<(usize, usize)> {
    for (family, count) in counts.iter().enumerate() {
        if index < *count {
            return Some((family, index));
        }
        index -= count;
    }
    None
}

/// ⚡️ Internal loads: the four families that share the `load` granularity, flattened into one
/// section because each is tiny and they read as one list. Only the addressed row is projected.
fn load_total(model: &EnergyRowsView<'_>) -> usize {
    model.people.len() + model.lighting.len() + model.equipment.len() + model.infiltrations.len()
}

fn load_entry(model: &EnergyRowsView<'_>, locale: Locale, index: usize) -> Option<RowEntry> {
    let (family, at) = locate(&[model.people.len(), model.lighting.len(), model.equipment.len(), model.infiltrations.len()], index)?;
    Some(match family {
        0 => {
            let people = model.people.get(at)?;
            (energy_target_id(people.id), format!("{} · {} p/m²", say(locale, "People", "Personen"), scalar(people.people_per_area)), say(locale, "People", "Personen").to_string(), "users", Some(ENERGY_GRANULARITY_LOAD))
        }
        1 => {
            let lighting = model.lighting.get(at)?;
            (energy_target_id(lighting.id), format!("{} · {} W/m²", say(locale, "Lighting", "Beleuchtung"), scalar(lighting.watts_per_area)), say(locale, "Lighting", "Beleuchtung").to_string(), "lightbulb", Some(ENERGY_GRANULARITY_LOAD))
        }
        2 => {
            let equipment = model.equipment.get(at)?;
            (energy_target_id(equipment.id), format!("{} · {} W/m²", say(locale, "Equipment", "Geräte"), scalar(equipment.watts_per_area)), say(locale, "Equipment", "Geräte").to_string(), "plug", Some(ENERGY_GRANULARITY_LOAD))
        }
        _ => {
            let infiltration = model.infiltrations.get(at)?;
            (energy_target_id(infiltration.id), format!("{} · {} ACH", say(locale, "Infiltration", "Infiltration"), scalar(infiltration.design_flow_ach)), say(locale, "Infiltration", "Infiltration").to_string(), "wind", Some(ENERGY_GRANULARITY_LOAD))
        }
    })
}

/// 🌡️ Controls: thermostats pick at their own granularity; humidistats are read-only because
/// `energy_entity_kind` resolves no `humidistat` family, and a row must never hand the inspector a
/// target it cannot resolve back.
fn control_total(model: &EnergyRowsView<'_>) -> usize {
    model.thermostats.len() + model.humidistats.len()
}

fn control_entry(model: &EnergyRowsView<'_>, locale: Locale, index: usize) -> Option<RowEntry> {
    let section_id = format!("{TREE_NAMESPACE}.controls");
    let (family, at) = locate(&[model.thermostats.len(), model.humidistats.len()], index)?;
    Some(match family {
        0 => {
            let thermostat = model.thermostats.get(at)?;
            (
                energy_target_id(thermostat.id),
                format!("{} {} · ±{} K", say(locale, "Thermostat", "Thermostat"), thermostat.id.0, scalar(thermostat.heating_throttle_range_k)),
                say(locale, "Thermostat", "Thermostat").to_string(),
                "thermometer",
                Some(ENERGY_GRANULARITY_THERMOSTAT),
            )
        }
        _ => {
            let humidistat = model.humidistats.get(at)?;
            (format!("{section_id}.humidistat.{}", humidistat.id.0), format!("{} {}", say(locale, "Humidistat", "Hygrostat"), humidistat.id.0), say(locale, "Humidistat", "Hygrostat").to_string(), "droplet", None)
        }
    })
}

/// 🌬️ HVAC: ideal-loads systems pick at the `hvac` granularity; air and plant loops are read-only
/// until the shared resolver names them.
fn hvac_total(model: &EnergyRowsView<'_>) -> usize {
    model.ideal_loads.len() + model.air_loops.len() + model.plant_loops.len()
}

fn hvac_entry(model: &EnergyRowsView<'_>, locale: Locale, index: usize) -> Option<RowEntry> {
    let section_id = format!("{TREE_NAMESPACE}.hvac");
    let (family, at) = locate(&[model.ideal_loads.len(), model.air_loops.len(), model.plant_loops.len()], index)?;
    Some(match family {
        0 => {
            let system = model.ideal_loads.get(at)?;
            (energy_target_id(system.id), format!("{} {}", say(locale, "Ideal loads", "Idealanlage"), system.id.0), say(locale, "Ideal loads", "Idealanlage").to_string(), "fan", Some(ENERGY_GRANULARITY_HVAC))
        }
        1 => {
            let air_loop = model.air_loops.get(at)?;
            (format!("{section_id}.air-loop.{}", air_loop.id.0), air_loop.name, say(locale, "Air loop", "Luftkreis").to_string(), "route", None)
        }
        _ => {
            let plant_loop = model.plant_loops.get(at)?;
            (format!("{section_id}.plant-loop.{}", plant_loop.id.0), plant_loop.name, say(locale, "Plant loop", "Anlagenkreis").to_string(), "route", None)
        }
    })
}

/// 🗓️ Schedules, read-only and free: a `ScheduleId` lives in its OWN id space, so a schedule row
/// keyed by its raw number would collide with an `EntityId` on the interaction wire.
fn schedule_entries(model: &EnergyRowsView<'_>, locale: Locale) -> Vec<RowEntry> {
    let section_id = format!("{TREE_NAMESPACE}.schedules");
    let schedules = &model.schedules;
    [
        (say(locale, "Constant", "Konstant"), schedules.constants.len()),
        (say(locale, "Daily", "Täglich"), schedules.daily.len()),
        (say(locale, "Weekly", "Wöchentlich"), schedules.weekly.len()),
        (say(locale, "Annual", "Jährlich"), schedules.annual.len()),
        (say(locale, "Time series", "Zeitreihe"), schedules.time_series.len()),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (label, count))| (format!("{section_id}.{index}"), label.to_string(), count.to_string(), "calendar", None))
    .collect()
}
//#endregion 🔖️Sections

//#region 🔖️Render
fn marked_ids(ids: &[String]) -> Vec<String> {
    ids.iter().take(MARKED_IDS_LIMIT).cloned().collect()
}

/// 🔋️ The outliner: eleven windowed sections bound to the `"energyModel"` domain and marked from the
/// live interaction snapshot. Every container stamps its full `total`; the host decides which slice
/// of each is materialised, so the tree never truncates and never invents a `+N` row.
pub fn build_artifact_tree(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let latch = crate::snapshot::state::ProjectionFault::default();
    let view = snapshot.model.read();
    let rows = view.rows(&latch);
    let tree = build_tree_rows(&rows, interaction, locale, windows);
    latch.finish().map_err(|_| capacity_error("energy artifact row failed to project from the native state"))?;
    tree
}

fn build_tree_rows(model: &EnergyRowsView<'_>, interaction: &EnergyModelInteractionSnapshot, locale: Locale, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let placeholder = ui_label(say(locale, "None", "Keine"))?;
    let schedules = schedule_entries(model, locale);

    let builder = PanelTreeBuilder::new(TREE_NAMESPACE)?
        .window_section(windows, &format!("{TREE_NAMESPACE}.site"), Some(ui_label(say(locale, "Site", "Standort"))?), true, std::slice::from_ref(&model.site), |_| site_row(model, locale))?;
    let builder = indexed_section(builder, windows, "zones", section_label(say(locale, "Zones", "Zonen"), model.zones.len())?, true, model.zones.len(), &placeholder, |index| {
        zone_row(windows, model, interaction, locale, &model.zones.get(index).ok_or_else(missing_row)?)
    })?;
    let builder = indexed_section(builder, windows, "shading", section_label(say(locale, "Shading surfaces", "Verschattungsflächen"), model.shading_surfaces.len())?, false, model.shading_surfaces.len(), &placeholder, |index| {
        let shading = model.shading_surfaces.get(index).ok_or_else(missing_row)?;
        let id = energy_target_id(shading.id);
        entity_row(&id, ENERGY_GRANULARITY_SHADING, &shading_label(&shading), say(locale, "Shading", "Verschattung"), "umbrella", shading.vertices_m.len() < 3, MarkedAs::of(interaction, &id))
    })?;
    let builder = indexed_section(builder, windows, "materials", section_label(say(locale, "Materials", "Materialien"), model.materials.len())?, false, model.materials.len(), &placeholder, |index| {
        let material = model.materials.get(index).ok_or_else(missing_row)?;
        let id = energy_target_id(material.id);
        entity_row(&id, ENERGY_GRANULARITY_MATERIAL, &material_label(&material), say(locale, "Material", "Material"), "layers", false, MarkedAs::of(interaction, &id))
    })?;
    let builder = indexed_section(builder, windows, "glazing-materials", section_label(say(locale, "Glazing materials", "Verglasungsmaterialien"), model.glazing_materials.len())?, false, model.glazing_materials.len(), &placeholder, |index| {
        let material = model.glazing_materials.get(index).ok_or_else(missing_row)?;
        let id = energy_target_id(material.id);
        entity_row(&id, ENERGY_GRANULARITY_GLAZING_MATERIAL, &glazing_material_label(&material), say(locale, "Glazing", "Verglasung"), "panel-top", false, MarkedAs::of(interaction, &id))
    })?;
    let builder = indexed_section(builder, windows, "gas-materials", section_label(say(locale, "Gas gaps", "Gasfüllungen"), model.gas_materials.len())?, false, model.gas_materials.len(), &placeholder, |index| {
        let material = model.gas_materials.get(index).ok_or_else(missing_row)?;
        let id = energy_target_id(material.id);
        entity_row(&id, ENERGY_GRANULARITY_GAS_MATERIAL, &gas_material_label(&material), say(locale, "Gas gap", "Gasfüllung"), "wind", false, MarkedAs::of(interaction, &id))
    })?;
    let builder = indexed_section(builder, windows, "constructions", section_label(say(locale, "Constructions", "Konstruktionen"), model.constructions.len())?, false, model.constructions.len(), &placeholder, |index| {
        let construction = model.constructions.get(index).ok_or_else(missing_row)?;
        let dangling = construction.layer_material_ids.iter().any(|layer| {
            !model.materials.holds(|material| material.id == *layer) && !model.glazing_materials.holds(|material| material.id == *layer) && !model.gas_materials.holds(|material| material.id == *layer)
        });
        let id = energy_target_id(construction.id);
        entity_row(&id, ENERGY_GRANULARITY_CONSTRUCTION, &construction_label(&construction), say(locale, "Construction", "Konstruktion"), "bricks", dangling, MarkedAs::of(interaction, &id))
    })?;
    let builder = indexed_section(builder, windows, "loads", section_label(say(locale, "Internal loads", "Innere Lasten"), load_total(model))?, false, load_total(model), &placeholder, |index| entry_row(&load_entry(model, locale, index).ok_or_else(missing_row)?, interaction))?;
    let builder = indexed_section(builder, windows, "controls", section_label(say(locale, "Controls", "Regelung"), control_total(model))?, false, control_total(model), &placeholder, |index| entry_row(&control_entry(model, locale, index).ok_or_else(missing_row)?, interaction))?;
    let builder = indexed_section(builder, windows, "hvac", section_label(say(locale, "HVAC", "Anlagentechnik"), hvac_total(model))?, false, hvac_total(model), &placeholder, |index| entry_row(&hvac_entry(model, locale, index).ok_or_else(missing_row)?, interaction))?;
    builder
        .window_section(windows, &format!("{TREE_NAMESPACE}.schedules"), Some(ui_label(say(locale, "Schedules", "Zeitpläne"))?), false, &schedules, |entry| entry_row(entry, interaction))?
        .interaction_domain(ENERGY_MODEL_EDITOR_CONTROLLER_ID, ENERGY_MODEL_INTERACTION_DOMAIN)?
        .selected(marked_ids(&interaction.selected_ids))?
        .highlighted(marked_ids(&interaction.hovered_ids))?
        .build()
}

/// 🪟️ One windowed section addressed by row index, with the empty-state placeholder an empty family shows.
fn indexed_section(builder: PanelTreeBuilder, windows: &TreeWindows<'_>, key: &str, label: UiLabel, default_open: bool, total: usize, placeholder: &UiLabel, row: impl FnMut(usize) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<PanelTreeBuilder> {
    let id = format!("{TREE_NAMESPACE}.{key}");
    match total {
        0 => builder.window_section_or_placeholder(windows, &id, Some(label), default_open, &[] as &[()], |_| Err(missing_row()), placeholder.clone()),
        _ => builder.window_indexed_section(windows, &id, Some(label), default_open, total, row),
    }
}

pub fn render(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    build_artifact_tree(snapshot, interaction, locale, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
