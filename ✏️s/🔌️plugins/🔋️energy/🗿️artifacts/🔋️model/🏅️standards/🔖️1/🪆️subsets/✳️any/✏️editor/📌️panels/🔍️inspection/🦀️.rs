//! 🔍️ Energy model editor panel — the inspection panel: real editable controls for whatever the
//! framework-owned `"energyModel"` selection resolves to (a surface, a window, a zone, a material, a
//! glazing/gas material, a construction, a thermostat), or the document + site summary when nothing
//! is selected.
//!
//! ⚠️ The body is a `ui::column` of `ui::section`s whose rows are `field(label) > control` — the
//! shape the React `Interpreter` renders as a labelled form row. A `Component::Tree` is NOT that
//! shape: its renderer maps sections to `TreeDataItem`s and DROPS every non-tree child, so a control
//! nested in a tree section reaches the host and is never drawn (fem2d's verified defect, ticket
//! 26/09/16 `📓️w-d-inspector`). `carries_a_tree` pins this down in the tests.
//!
//! 🎛️ Every control binds `Trigger::Change` to one `set-*-property` command with the argument map
//! `{field, id}`; the host merges the control's own value under `value`, so one flat
//! `{field, id, value}` payload covers every field of every entity kind and no per-field action has
//! to be declared. `args_bridge::command_from_action` reads `field` back as `property` and `id` as
//! the entity id. Read-only rows (ids, areas, tilts, construction layers) are `field > ui::text`.

use crate::editor::model::interaction::{
    energy_entity_kind, energy_target_id, EnergyModelInteractionSnapshot, ENERGY_GRANULARITY_CONSTRUCTION, ENERGY_GRANULARITY_FENESTRATION, ENERGY_GRANULARITY_GAS_MATERIAL, ENERGY_GRANULARITY_GLAZING_MATERIAL, ENERGY_GRANULARITY_MATERIAL,
    ENERGY_GRANULARITY_SHADING, ENERGY_GRANULARITY_SURFACE, ENERGY_GRANULARITY_THERMOSTAT, ENERGY_GRANULARITY_ZONE,
};
use crate::editor::model::{
    energy_model_action, gas_kind_id, outside_boundary_kind_id, surface_class_id, ui_label, DELETE_SURFACE_ACTION_ID, DELETE_ZONE_ACTION_ID, GAS_KIND_IDS, OUTSIDE_BOUNDARY_KIND_IDS, SET_FENESTRATION_PROPERTY_ACTION_ID,
    SET_GAS_MATERIAL_PROPERTY_ACTION_ID, SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID, SET_MATERIAL_PROPERTY_ACTION_ID, SET_SITE_ACTION_ID, SET_SURFACE_PROPERTY_ACTION_ID, SET_THERMOSTAT_SETPOINTS_ACTION_ID, SET_ZONE_PROPERTY_ACTION_ID,
    SURFACE_CLASS_IDS,
};
use crate::model::{Construction, Fenestration, GasMaterial, GlazingMaterial, Material, Model, ShadingSurface, Surface, Thermostat, Zone};
use crate::EnergyModelSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind};
use semio_framework_plugin::{
    ActionId, BuiltNode, Locale, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, Trigger, UiAssemblyResult, UiFixedList, UiMapBuilder, UiText, UiValue, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "energy.model.inspection";
const ROOT: &str = "energy-model-inspection";
/// 🔽️ Options one reference select offers before it stops listing — a model with more constructions
/// than this is re-pointed from the tree, and `UiFixedList` admits 32 at most.
const SELECT_ITEMS_MAX: usize = 24;
/// 🧾️ Rows one nested listing (a construction's layers, a multi-selection header) materialises.
const LIST_ROWS_MAX: usize = 8;
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🗣️Language
fn say(locale: Locale, en: &'static str, de: &'static str) -> &'static str {
    if locale == Locale::De {
        de
    } else {
        en
    }
}
//#endregion 🗣️Language

//#region 🔖️Admission
fn ui_error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "energy inspector admission failed")
}

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| ui_error("ui.text"))
}

fn ui_value_text(value: impl AsRef<str>) -> UiAssemblyResult<UiValue> {
    ui_text(value).map(UiValue::Text)
}

fn ui_id<B: HasBase>(builder: B, id: impl AsRef<str>) -> UiAssemblyResult<B> {
    builder.try_id(id).map_err(|_| ui_error("ui.node.id"))
}

fn ui_build<B: Buildable>(builder: B) -> UiAssemblyResult<BuiltNode> {
    builder.try_build().map_err(|_| ui_error("ui.node.build"))
}

fn push(rows: &mut UiFixedList<BuiltNode>, node: UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<()> {
    rows.try_push(node?).map_err(|_| ui_error("ui.fixed-capacity"))
}
//#endregion 🔖️Admission

//#region 🔖️Layout
/// 🗂️ One collapsible group of form rows — `ui::section`, never a tree section.
fn section(id: &str, label: &str, rows: UiFixedList<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui_id(ui::section(ui_label(label)?), id)?.default_open(true);
    ui_build(builder.try_children(rows).map_err(|_| ui_error("ui.section.children"))?)
}

/// 📋️ The panel body: its sections stacked vertically.
fn column(sections: UiFixedList<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui_id(ui::column(), ROOT)?;
    ui_build(builder.try_children(sections).map_err(|_| ui_error("ui.column.children"))?)
}

/// 🧾️ One `field(label) > control` form row — the shape the React `Interpreter` renders as a
/// labelled control rather than as a tree leaf.
fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let row = ui_id(ui::field(ui_label(label)?), row_id)?;
    ui_build(row.try_child(control).map_err(|_| ui_error("ui.node.child"))?)
}

fn text_node(id: &str, value: impl AsRef<str>) -> UiAssemblyResult<BuiltNode> {
    ui_build(ui_id(ui::text(ui_label(value)?), id)?)
}

/// 🔘️ One `Trigger::Activate` button — the verbs (delete), never a `Change`-bound control.
fn action_button(id: &str, label: &str, action: (ActionId, Option<UiValue>)) -> UiAssemblyResult<BuiltNode> {
    let (action, args) = action;
    let control = ui_id(ui::button(ui_label(label)?), id)?;
    let control = match args {
        Some(args) => control.try_on_with(Trigger::Activate, action, args).map_err(|_| ui_error("ui.control.binding"))?,
        None => control.try_on(Trigger::Activate, action).map_err(|_| ui_error("ui.control.binding"))?,
    };
    ui_build(control)
}
//#endregion 🔖️Layout

//#region 🔖️Controls
/// 🎛️ The argument map every property control carries: the edited field and the entity it addresses.
/// `UiMapBuilder` admits keys in ascending order only (`field` < `id`); the control's own value
/// arrives as `value` from the host, so it is deliberately ABSENT here — authoring one would be
/// overwritten by the merge.
fn patch_args(field_name: &str, id: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    args.push("field".into(), ui_value_text(field_name)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    args.push("id".into(), ui_value_text(id)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    Ok(UiValue::Map(args.finish()))
}

fn bind<B: HasBase>(builder: B, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<B> {
    let (action, args) = energy_model_action(action, Some(patch_args(field_name, id)?))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| ui_error("ui.control.binding")),
        None => builder.try_on(Trigger::Change, action).map_err(|_| ui_error("ui.control.binding")),
    }
}

fn read_only_row(suffix: &str, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    control_row(&row_id, label, text_node(&format!("{row_id}.value"), value.to_string())?)
}

fn number_row(suffix: &str, label: &str, value: f64, step: f64, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::input(InputKind::Number).value(ui_text(format!("{value}"))?).step(step), format!("{row_id}.input"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn text_row(suffix: &str, label: &str, value: &str, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::input(InputKind::Text).value(ui_text(value)?), format!("{row_id}.input"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn slider_row(suffix: &str, label: &str, value: f64, bounds: (f64, f64, f64), action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let (minimum, maximum, step) = bounds;
    let control = ui_id(ui::slider(value).min(minimum).max(maximum).step(step), format!("{row_id}.slider"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn toggle_row(suffix: &str, label: &str, on: bool, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::toggle(on).text(ui_label(label)?), format!("{row_id}.toggle"))?;
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}

fn select_row(suffix: &str, label: &str, value: &str, options: Vec<(String, String)>, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let mut control = ui_id(ui::select(ui_text(value)?), format!("{row_id}.select"))?;
    for (option, option_label) in options.into_iter().take(SELECT_ITEMS_MAX) {
        control = control.try_item(ui_text(option)?, ui_label(option_label)?).map_err(|_| ui_error("ui.select.item"))?;
    }
    control_row(&row_id, label, ui_build(bind(control, action, field_name, id)?)?)
}
//#endregion 🔖️Controls

//#region 🔖️Options
fn construction_options(model: &Model) -> Vec<(String, String)> {
    model.constructions.iter().map(|construction| (energy_target_id(construction.id), construction.name.clone())).collect()
}

/// 🧊️ A window's glazing stack is optional, so its select carries an explicit empty option that
/// clears the binding — an empty `value` is exactly what `set_fenestration_property` reads as
/// `clear-fenestration-glazing-construction`.
fn glazing_construction_options(model: &Model, locale: Locale) -> Vec<(String, String)> {
    let mut options = vec![(String::new(), say(locale, "None (simple glazing)", "Keine (einfache Verglasung)").to_string())];
    options.extend(construction_options(model));
    options
}

fn surface_options(model: &Model) -> Vec<(String, String)> {
    model.surfaces.iter().map(|surface| (energy_target_id(surface.id), surface.name.clone())).collect()
}

fn class_options() -> Vec<(String, String)> {
    SURFACE_CLASS_IDS.iter().map(|id| ((*id).to_string(), (*id).to_string())).collect()
}

fn boundary_options() -> Vec<(String, String)> {
    OUTSIDE_BOUNDARY_KIND_IDS.iter().map(|id| ((*id).to_string(), (*id).to_string())).collect()
}

fn schedule_options(model: &Model) -> Vec<(String, String)> {
    let schedules = &model.schedules;
    let mut options: Vec<(String, String)> = Vec::new();
    for schedule in &schedules.constants {
        options.push((schedule.id.0.to_string(), format!("{} · {}", schedule.id.0, schedule.value)));
    }
    for id in schedules.daily.iter().map(|schedule| schedule.id).chain(schedules.weekly.iter().map(|schedule| schedule.id)).chain(schedules.annual.iter().map(|schedule| schedule.id)).chain(schedules.time_series.iter().map(|schedule| schedule.id))
    {
        options.push((id.0.to_string(), id.0.to_string()));
    }
    options
}
//#endregion 🔖️Options

//#region 🔖️Geometry
/// 📐️ A surface's plan area, tilt and azimuth — read-only, derived from its own polygon, so the
/// inspector reports the geometry the 3d window draws instead of inventing a second source.
fn surface_geometry(model: &Model, surface: &Surface) -> (f64, f64, f64) {
    let area = crate::geometry::surface_area_m2(&surface.vertices_m);
    let orientation = crate::geometry::surface_tilt_azimuth(crate::geometry::polygon_normal(&surface.vertices_m), model.site.north_axis_deg);
    (area, orientation.tilt_deg, orientation.azimuth_deg)
}
//#endregion 🔖️Geometry

//#region 🔖️Sections
fn surface_rows(model: &Model, surface: &Surface, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = energy_target_id(surface.id);
    let (area, tilt, azimuth) = surface_geometry(model, surface);
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("surface.id", say(locale, "Id", "Id"), &id))?;
    push(&mut rows, text_row("surface.name", say(locale, "Name", "Bezeichnung"), &surface.name, SET_SURFACE_PROPERTY_ACTION_ID, "name", &id))?;
    push(&mut rows, select_row("surface.class", say(locale, "Class", "Klasse"), surface_class_id(surface.class), class_options(), SET_SURFACE_PROPERTY_ACTION_ID, "class", &id))?;
    push(
        &mut rows,
        select_row(
            "surface.boundary",
            say(locale, "Boundary", "Randbedingung"),
            outside_boundary_kind_id(surface.outside_boundary_condition.kind()),
            boundary_options(),
            SET_SURFACE_PROPERTY_ACTION_ID,
            "boundary",
            &id,
        ),
    )?;
    if let Some(partner) = surface.outside_boundary_condition.interzone_partner() {
        let name = model.surfaces.iter().find(|entry| entry.id == partner).map_or_else(|| partner.0.to_string(), |entry| entry.name.clone());
        push(&mut rows, read_only_row("surface.partner", say(locale, "Interzone partner", "Nachbarfläche"), name))?;
    }
    push(
        &mut rows,
        select_row(
            "surface.construction",
            say(locale, "Construction", "Konstruktion"),
            &energy_target_id(surface.construction_id),
            construction_options(model),
            SET_SURFACE_PROPERTY_ACTION_ID,
            "construction",
            &id,
        ),
    )?;
    push(&mut rows, toggle_row("surface.sun-exposed", say(locale, "Sun exposed", "Besonnt"), surface.sun_exposed, SET_SURFACE_PROPERTY_ACTION_ID, "sunExposed", &id))?;
    push(&mut rows, toggle_row("surface.wind-exposed", say(locale, "Wind exposed", "Windexponiert"), surface.wind_exposed, SET_SURFACE_PROPERTY_ACTION_ID, "windExposed", &id))?;
    push(&mut rows, number_row("surface.multiplier", say(locale, "Multiplier", "Multiplikator"), f64::from(surface.multiplier), 1.0, SET_SURFACE_PROPERTY_ACTION_ID, "multiplier", &id))?;
    push(&mut rows, read_only_row("surface.area", say(locale, "Area (m²)", "Fläche (m²)"), format!("{area:.2}")))?;
    push(&mut rows, read_only_row("surface.tilt", say(locale, "Tilt (°)", "Neigung (°)"), format!("{tilt:.1}")))?;
    push(&mut rows, read_only_row("surface.azimuth", say(locale, "Azimuth (°)", "Azimut (°)"), format!("{azimuth:.1}")))?;
    Ok(rows)
}

fn fenestration_rows(model: &Model, window: &Fenestration, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = energy_target_id(window.id);
    let action = SET_FENESTRATION_PROPERTY_ACTION_ID;
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("fenestration.id", say(locale, "Id", "Id"), &id))?;
    push(&mut rows, text_row("fenestration.name", say(locale, "Name", "Bezeichnung"), &window.name, action, "name", &id))?;
    let host = model.surfaces.iter().find(|surface| surface.id == window.surface_id);
    push(&mut rows, read_only_row("fenestration.surface", say(locale, "Surface", "Fläche"), host.map_or_else(|| window.surface_id.0.to_string(), |surface| surface.name.clone())))?;
    push(&mut rows, number_row("fenestration.u-value", say(locale, "U-value (W/m²K)", "U-Wert (W/m²K)"), window.u_value_w_m2k, 0.1, action, "uValueWM2K", &id))?;
    push(&mut rows, slider_row("fenestration.shgc", say(locale, "SHGC", "g-Wert"), window.shgc, (0.0, 1.0, 0.01), action, "shgc", &id))?;
    push(&mut rows, slider_row("fenestration.vlt", say(locale, "VLT", "Lichttransmission"), window.vlt, (0.0, 1.0, 0.01), action, "vlt", &id))?;
    push(&mut rows, number_row("fenestration.area", say(locale, "Area (m²)", "Fläche (m²)"), window.area_m2, 0.1, action, "areaM2", &id))?;
    push(&mut rows, number_row("fenestration.height", say(locale, "Height (m)", "Höhe (m)"), window.height_m, 0.05, action, "heightM", &id))?;
    push(&mut rows, number_row("fenestration.sill-height", say(locale, "Sill height (m)", "Brüstungshöhe (m)"), window.sill_height_m, 0.05, action, "sillHeightM", &id))?;
    push(&mut rows, number_row("fenestration.frame", say(locale, "Frame conductance (W/K)", "Rahmenleitwert (W/K)"), window.frame_conductance_w_k, 0.1, action, "frameConductanceWK", &id))?;
    push(&mut rows, number_row("fenestration.divider", say(locale, "Divider conductance (W/K)", "Sprossenleitwert (W/K)"), window.divider_conductance_w_k, 0.1, action, "dividerConductanceWK", &id))?;
    push(&mut rows, number_row("fenestration.overhang-depth", say(locale, "Overhang depth (m)", "Auskragung Tiefe (m)"), window.overhang_depth_m, 0.05, action, "overhangDepthM", &id))?;
    push(&mut rows, number_row("fenestration.overhang-offset", say(locale, "Overhang offset (m)", "Auskragung Abstand (m)"), window.overhang_offset_m, 0.05, action, "overhangOffsetM", &id))?;
    push(&mut rows, number_row("fenestration.fin-depth", say(locale, "Fin depth (m)", "Seitenblende Tiefe (m)"), window.fin_depth_m, 0.05, action, "finDepthM", &id))?;
    push(&mut rows, number_row("fenestration.fin-offset", say(locale, "Fin offset (m)", "Seitenblende Abstand (m)"), window.fin_offset_m, 0.05, action, "finOffsetM", &id))?;
    let glazing = window.glazing_construction_id.map(energy_target_id).unwrap_or_default();
    push(&mut rows, select_row("fenestration.glazing", say(locale, "Glazing construction", "Verglasungsaufbau"), &glazing, glazing_construction_options(model, locale), action, "glazingConstruction", &id))?;
    Ok(rows)
}

fn zone_rows(zone: &Zone, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = energy_target_id(zone.id);
    let action = SET_ZONE_PROPERTY_ACTION_ID;
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("zone.id", say(locale, "Id", "Id"), &id))?;
    push(&mut rows, text_row("zone.name", say(locale, "Name", "Bezeichnung"), &zone.name, action, "name", &id))?;
    push(&mut rows, number_row("zone.volume", say(locale, "Volume (m³)", "Volumen (m³)"), zone.volume_m3, 1.0, action, "volumeM3", &id))?;
    push(&mut rows, number_row("zone.multiplier", say(locale, "Multiplier", "Multiplikator"), f64::from(zone.multiplier), 1.0, action, "multiplier", &id))?;
    push(&mut rows, toggle_row("zone.conditioned", say(locale, "Conditioned", "Konditioniert"), zone.conditioned, action, "conditioned", &id))?;
    push(&mut rows, toggle_row("zone.floor-area", say(locale, "Part of floor area", "Teil der Nettofläche"), zone.part_of_total_floor_area, action, "partOfTotalFloorArea", &id))?;
    Ok(rows)
}

/// 🧱️ A material's seven SI scalars. `set-material-property` carries a NUMBER, so the material's
/// own `name`/`roughness` have no editor verb yet — they are reported read-only rather than offered
/// as a control that would silently do nothing.
fn material_rows(material: &Material, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = energy_target_id(material.id);
    let action = SET_MATERIAL_PROPERTY_ACTION_ID;
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("material.id", say(locale, "Id", "Id"), &id))?;
    push(&mut rows, read_only_row("material.name", say(locale, "Name", "Bezeichnung"), &material.name))?;
    push(&mut rows, read_only_row("material.roughness", say(locale, "Roughness", "Rauigkeit"), format!("{:?}", material.roughness)))?;
    push(&mut rows, number_row("material.thickness", say(locale, "Thickness (m)", "Dicke (m)"), material.thickness_m, 0.01, action, "thicknessM", &id))?;
    push(&mut rows, number_row("material.conductivity", say(locale, "Conductivity (W/mK)", "Leitfähigkeit (W/mK)"), material.conductivity_w_m_k, 0.01, action, "conductivityWMK", &id))?;
    push(&mut rows, number_row("material.density", say(locale, "Density (kg/m³)", "Rohdichte (kg/m³)"), material.density_kg_m3, 10.0, action, "densityKgM3", &id))?;
    push(&mut rows, number_row("material.specific-heat", say(locale, "Specific heat (J/kgK)", "Wärmekapazität (J/kgK)"), material.specific_heat_j_kg_k, 10.0, action, "specificHeatJKgK", &id))?;
    push(&mut rows, slider_row("material.thermal-absorptance", say(locale, "Thermal absorptance", "Emissionsgrad"), material.thermal_absorptance, (0.0, 1.0, 0.01), action, "thermalAbsorptance", &id))?;
    push(&mut rows, slider_row("material.solar-absorptance", say(locale, "Solar absorptance", "Solarer Absorptionsgrad"), material.solar_absorptance, (0.0, 1.0, 0.01), action, "solarAbsorptance", &id))?;
    push(&mut rows, slider_row("material.visible-absorptance", say(locale, "Visible absorptance", "Visueller Absorptionsgrad"), material.visible_absorptance, (0.0, 1.0, 0.01), action, "visibleAbsorptance", &id))?;
    Ok(rows)
}

/// 🧊️ Glazing material. The six fields `change-glazing-material-*`/`rename-glazing-material` name
/// are editable; the four reflectances and the infrared transmittance have NO mutation kind, so they
/// are reported read-only rather than offered as a control that would refuse on change.
fn glazing_material_rows(material: &GlazingMaterial, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = energy_target_id(material.id);
    let action = SET_GLAZING_MATERIAL_PROPERTY_ACTION_ID;
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("glazing.id", say(locale, "Id", "Id"), &id))?;
    push(&mut rows, text_row("glazing.name", say(locale, "Name", "Bezeichnung"), &material.name, action, "name", &id))?;
    push(&mut rows, number_row("glazing.thickness", say(locale, "Thickness (m)", "Dicke (m)"), material.thickness_m, 0.001, action, "thicknessM", &id))?;
    push(&mut rows, number_row("glazing.conductivity", say(locale, "Conductivity (W/mK)", "Leitfähigkeit (W/mK)"), material.conductivity_w_m_k, 0.01, action, "conductivityWMK", &id))?;
    push(&mut rows, slider_row("glazing.solar-transmittance", say(locale, "Solar transmittance", "Solare Transmission"), material.solar_transmittance, (0.0, 1.0, 0.01), action, "solarTransmittance", &id))?;
    push(&mut rows, slider_row("glazing.visible-transmittance", say(locale, "Visible transmittance", "Lichttransmission"), material.visible_transmittance, (0.0, 1.0, 0.01), action, "visibleTransmittance", &id))?;
    push(&mut rows, slider_row("glazing.emissivity-front", say(locale, "Emissivity front", "Emissionsgrad außen"), material.infrared_emissivity_front, (0.0, 1.0, 0.01), action, "infraredEmissivityFront", &id))?;
    push(&mut rows, slider_row("glazing.emissivity-back", say(locale, "Emissivity back", "Emissionsgrad innen"), material.infrared_emissivity_back, (0.0, 1.0, 0.01), action, "infraredEmissivityBack", &id))?;
    push(&mut rows, read_only_row("glazing.solar-reflectance", say(locale, "Solar reflectance (front/back)", "Solare Reflexion (außen/innen)"), format!("{} / {}", material.solar_reflectance_front, material.solar_reflectance_back)))?;
    push(&mut rows, read_only_row("glazing.infrared-transmittance", say(locale, "Infrared transmittance", "Infrarot-Transmission"), material.infrared_transmittance))?;
    Ok(rows)
}

/// 💨️ Gas gap: thickness, fill gas and name — the record's whole addressable surface.
fn gas_material_rows(material: &GasMaterial, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let id = energy_target_id(material.id);
    let action = SET_GAS_MATERIAL_PROPERTY_ACTION_ID;
    let gases = GAS_KIND_IDS.iter().map(|gas| ((*gas).to_string(), (*gas).to_string())).collect();
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("gas.id", say(locale, "Id", "Id"), &id))?;
    push(&mut rows, text_row("gas.name", say(locale, "Name", "Bezeichnung"), &material.name, action, "name", &id))?;
    push(&mut rows, number_row("gas.thickness", say(locale, "Thickness (m)", "Dicke (m)"), material.thickness_m, 0.001, action, "thicknessM", &id))?;
    push(&mut rows, select_row("gas.kind", say(locale, "Gas", "Gas"), gas_kind_id(material.gas), gases, action, "gas", &id))?;
    Ok(rows)
}

/// 🧱️ A construction and its layer stack — the layers are reported, not edited: layer membership is
/// an `add`/`remove`/`reorder` list edit, not a field patch, and this editor declares no verb for it.
fn construction_rows(model: &Model, construction: &Construction, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("construction.id", say(locale, "Id", "Id"), energy_target_id(construction.id)))?;
    push(&mut rows, read_only_row("construction.name", say(locale, "Name", "Bezeichnung"), &construction.name))?;
    for (index, layer) in construction.layer_material_ids.iter().take(LIST_ROWS_MAX).enumerate() {
        let name = model
            .materials
            .iter()
            .find(|material| material.id == *layer)
            .map(|material| material.name.clone())
            .or_else(|| model.glazing_materials.iter().find(|material| material.id == *layer).map(|material| material.name.clone()))
            .or_else(|| model.gas_materials.iter().find(|material| material.id == *layer).map(|material| material.name.clone()))
            .unwrap_or_else(|| format!("? {}", layer.0));
        push(&mut rows, read_only_row(&format!("construction.layer.{index}"), &format!("{} {}", say(locale, "Layer", "Schicht"), index + 1), name))?;
    }
    if construction.layer_material_ids.len() > LIST_ROWS_MAX {
        push(&mut rows, read_only_row("construction.layers.more", say(locale, "More layers", "Weitere Schichten"), construction.layer_material_ids.len() - LIST_ROWS_MAX))?;
    }
    Ok(rows)
}

/// 🌡️ A thermostat's four fields, all carried by ONE `set-thermostat-setpoints` payload — so each
/// control has to restate the other three, which `patch_args` cannot express. They are therefore
/// bound with the full payload map instead of the flat `{field, id}` one.
fn thermostat_rows(model: &Model, thermostat: &Thermostat, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("thermostat.id", say(locale, "Id", "Id"), energy_target_id(thermostat.id)))?;
    let zone = model.zones.iter().find(|zone| zone.id == thermostat.zone_id);
    push(&mut rows, read_only_row("thermostat.zone", say(locale, "Zone", "Zone"), zone.map_or_else(|| thermostat.zone_id.0.to_string(), |zone| zone.name.clone())))?;
    let options = schedule_options(model);
    push(
        &mut rows,
        thermostat_select("thermostat.heating-schedule", say(locale, "Heating setpoint schedule", "Heiz-Sollwertprofil"), thermostat, "heatingSchedule", &thermostat.heating_setpoint_schedule_id.0.to_string(), options.clone()),
    )?;
    push(
        &mut rows,
        thermostat_select("thermostat.cooling-schedule", say(locale, "Cooling setpoint schedule", "Kühl-Sollwertprofil"), thermostat, "coolingSchedule", &thermostat.cooling_setpoint_schedule_id.0.to_string(), options),
    )?;
    push(&mut rows, thermostat_number("thermostat.heating-throttle", say(locale, "Heating throttle range (K)", "Heiz-Regelbereich (K)"), thermostat, "heatingThrottleRangeK", thermostat.heating_throttle_range_k))?;
    push(&mut rows, thermostat_number("thermostat.cooling-throttle", say(locale, "Cooling throttle range (K)", "Kühl-Regelbereich (K)"), thermostat, "coolingThrottleRangeK", thermostat.cooling_throttle_range_k))?;
    Ok(rows)
}

/// 🌡️ The whole `set-thermostat-setpoints` payload MINUS the one key the edited control supplies —
/// the host merges the control's value under that key, so every other field has to travel with it or
/// the reducer would read a zero for it.
fn thermostat_args(thermostat: &Thermostat, edited: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    let entries: Vec<(&str, String)> = vec![
        ("coolingSchedule", thermostat.cooling_setpoint_schedule_id.0.to_string()),
        ("coolingThrottleRangeK", thermostat.cooling_throttle_range_k.to_string()),
        ("heatingSchedule", thermostat.heating_setpoint_schedule_id.0.to_string()),
        ("heatingThrottleRangeK", thermostat.heating_throttle_range_k.to_string()),
        ("thermostat", thermostat.id.0.to_string()),
        ("value", String::new()),
    ];
    for (key, value) in entries.into_iter().filter(|(key, _)| *key != edited) {
        args.push(key.to_owned(), ui_value_text(value)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

fn thermostat_select(suffix: &str, label: &str, thermostat: &Thermostat, field: &str, value: &str, options: Vec<(String, String)>) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let mut control = ui_id(ui::select(ui_text(value)?), format!("{row_id}.select"))?;
    for (option, option_label) in options.into_iter().take(SELECT_ITEMS_MAX) {
        control = control.try_item(ui_text(option)?, ui_label(option_label)?).map_err(|_| ui_error("ui.select.item"))?;
    }
    control_row(&row_id, label, ui_build(bind_full(control, thermostat, field)?)?)
}

fn thermostat_number(suffix: &str, label: &str, thermostat: &Thermostat, field: &str, value: f64) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::input(InputKind::Number).value(ui_text(format!("{value}"))?).step(0.1), format!("{row_id}.input"))?;
    control_row(&row_id, label, ui_build(bind_full(control, thermostat, field)?)?)
}

fn bind_full<B: HasBase>(builder: B, thermostat: &Thermostat, field: &str) -> UiAssemblyResult<B> {
    let (action, args) = energy_model_action(SET_THERMOSTAT_SETPOINTS_ACTION_ID, Some(thermostat_args(thermostat, field)?))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(|_| ui_error("ui.control.binding")),
        None => builder.try_on(Trigger::Change, action).map_err(|_| ui_error("ui.control.binding")),
    }
}

/// 📍️ The site's five scalars, each carrying the other four so the singleton `set-site` payload
/// stays complete — the same "one wide payload, one edited key" shape the thermostat uses.
fn site_rows(model: &Model, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let site = &model.site;
    let mut rows = UiFixedList::default();
    push(&mut rows, site_number("site.latitude", say(locale, "Latitude (°)", "Breitengrad (°)"), site, "latitudeDeg", site.latitude_deg))?;
    push(&mut rows, site_number("site.longitude", say(locale, "Longitude (°)", "Längengrad (°)"), site, "longitudeDeg", site.longitude_deg))?;
    push(&mut rows, site_number("site.elevation", say(locale, "Elevation (m)", "Höhe (m)"), site, "elevationM", site.elevation_m))?;
    push(&mut rows, site_number("site.time-zone", say(locale, "Time zone (h)", "Zeitzone (h)"), site, "timeZoneHours", site.time_zone_hours))?;
    push(&mut rows, site_number("site.north-axis", say(locale, "North axis (°)", "Nordachse (°)"), site, "northAxisDeg", site.north_axis_deg))?;
    Ok(rows)
}

fn site_args(site: &crate::model::Site, edited: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    let entries: Vec<(&str, String)> = vec![
        ("elevationM", site.elevation_m.to_string()),
        ("latitudeDeg", site.latitude_deg.to_string()),
        ("longitudeDeg", site.longitude_deg.to_string()),
        ("northAxisDeg", site.north_axis_deg.to_string()),
        ("timeZoneHours", site.time_zone_hours.to_string()),
    ];
    for (key, value) in entries.into_iter().filter(|(key, _)| *key != edited) {
        args.push(key.to_owned(), ui_value_text(value)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

fn site_number(suffix: &str, label: &str, site: &crate::model::Site, field: &str, value: f64) -> UiAssemblyResult<BuiltNode> {
    let row_id = format!("{ROOT}.{suffix}");
    let control = ui_id(ui::input(InputKind::Number).value(ui_text(format!("{value}"))?).step(0.1), format!("{row_id}.input"))?;
    let (action, args) = energy_model_action(SET_SITE_ACTION_ID, Some(site_args(site, field)?))?;
    let control = match args {
        Some(args) => control.try_on_with(Trigger::Change, action, args).map_err(|_| ui_error("ui.control.binding"))?,
        None => control.try_on(Trigger::Change, action).map_err(|_| ui_error("ui.control.binding"))?,
    };
    control_row(&row_id, label, ui_build(control)?)
}

fn shading_rows(shading: &ShadingSurface, locale: Locale) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    push(&mut rows, read_only_row("shading.id", say(locale, "Id", "Id"), energy_target_id(shading.id)))?;
    push(&mut rows, read_only_row("shading.name", say(locale, "Name", "Bezeichnung"), &shading.name))?;
    push(&mut rows, read_only_row("shading.vertices", say(locale, "Vertices", "Eckpunkte"), shading.vertices_m.len()))?;
    Ok(rows)
}
//#endregion 🔖️Sections

//#region 🔖️Render
/// 📋️ What the `"energyModel"` domain shows when it selects nothing: what this document is, how much
/// of it there is, and the site — which is a singleton with no `EntityId` and therefore has no other
/// place to be edited from.
fn summary(model: &Model, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let mut counts = UiFixedList::default();
    push(&mut counts, read_only_row("summary.name", say(locale, "Model", "Modell"), &model.name))?;
    for (suffix, label, count) in [
        ("summary.zones", say(locale, "Zones", "Zonen"), model.zones.len()),
        ("summary.surfaces", say(locale, "Surfaces", "Flächen"), model.surfaces.len()),
        ("summary.fenestrations", say(locale, "Windows", "Fenster"), model.fenestrations.len()),
        ("summary.shading", say(locale, "Shading surfaces", "Verschattungsflächen"), model.shading_surfaces.len()),
        ("summary.materials", say(locale, "Materials", "Materialien"), model.materials.len()),
        ("summary.constructions", say(locale, "Constructions", "Konstruktionen"), model.constructions.len()),
        ("summary.thermostats", say(locale, "Thermostats", "Thermostate"), model.thermostats.len()),
    ] {
        push(&mut counts, read_only_row(suffix, label, count))?;
    }
    let mut sections = UiFixedList::default();
    push(&mut sections, section(&format!("{ROOT}.summary"), say(locale, "Document", "Dokument"), counts))?;
    push(&mut sections, section(&format!("{ROOT}.site"), say(locale, "Site", "Standort"), site_rows(model, locale)?))?;
    column(sections)
}

/// 🎯️ The selected entity's verbs — one grouped row, never one per form row, so a page of controls
/// costs the argument arena one map each and the tree keeps its own credit.
fn action_rows(id: &str, kind: &str, locale: Locale) -> UiAssemblyResult<Option<UiFixedList<BuiltNode>>> {
    let (action, key) = match kind {
        ENERGY_GRANULARITY_ZONE => (DELETE_ZONE_ACTION_ID, "zone"),
        ENERGY_GRANULARITY_SURFACE => (DELETE_SURFACE_ACTION_ID, "surface"),
        _ => return Ok(None),
    };
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_error("ui.value.map"))?;
    args.push(key.to_owned(), ui_value_text(id)?).map_err(|_| ui_error("ui.value.map.entry"))?;
    let mut rows = UiFixedList::default();
    push(&mut rows, action_button(&format!("{ROOT}.actions.delete"), say(locale, "Delete", "Löschen"), energy_model_action(action, Some(UiValue::Map(args.finish())))?))?;
    Ok(Some(rows))
}

/// 🔢️ One text row per selected id — the form below belongs to the FIRST resolvable one, which is
/// the id every viewport pick leaves at the head of the selection.
fn selection_rows(interaction: &EnergyModelInteractionSnapshot) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut rows = UiFixedList::default();
    for (index, id) in interaction.selected_ids.iter().take(LIST_ROWS_MAX).enumerate() {
        push(&mut rows, text_node(&format!("{ROOT}.selection.{index}"), id))?;
    }
    Ok(rows)
}

/// 🔍️ The panel body: the first resolvable selected entity's editable fields, or the document +
/// site summary. Every group title is the entity's own noun, so the panel reads as "Window" /
/// "Construction" rather than as a generic "Properties".
pub fn render(snapshot: &EnergyModelSnapshot, interaction: &EnergyModelInteractionSnapshot, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let model = &snapshot.model;
    let Some((id, kind)) = interaction.selected_ids.iter().find_map(|id| energy_entity_kind(snapshot, id).map(|kind| (id.as_str(), kind))) else {
        return summary(model, locale);
    };
    let Ok(raw) = id.parse::<u32>() else { return summary(model, locale) };
    let entity = crate::model::EntityId(raw);
    let mut sections = UiFixedList::default();
    if interaction.selected_ids.len() > 1 {
        let title = format!("{} {}", interaction.selected_ids.len(), say(locale, "selected", "ausgewählt"));
        push(&mut sections, section(&format!("{ROOT}.selection"), &title, selection_rows(interaction)?))?;
    }
    match kind {
        ENERGY_GRANULARITY_SURFACE => {
            let surface = model.surfaces.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.surface"), say(locale, "Surface", "Fläche"), surface_rows(model, surface, locale)?))?;
        }
        ENERGY_GRANULARITY_FENESTRATION => {
            let window = model.fenestrations.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.fenestration"), say(locale, "Window", "Fenster"), fenestration_rows(model, window, locale)?))?;
        }
        ENERGY_GRANULARITY_ZONE => {
            let zone = model.zones.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.zone"), say(locale, "Zone", "Zone"), zone_rows(zone, locale)?))?;
        }
        ENERGY_GRANULARITY_MATERIAL => {
            let material = model.materials.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.material"), say(locale, "Material", "Material"), material_rows(material, locale)?))?;
        }
        ENERGY_GRANULARITY_GLAZING_MATERIAL => {
            let material = model.glazing_materials.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.glazing"), say(locale, "Glazing material", "Verglasungsmaterial"), glazing_material_rows(material, locale)?))?;
        }
        ENERGY_GRANULARITY_GAS_MATERIAL => {
            let material = model.gas_materials.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.gas"), say(locale, "Gas gap", "Gasfüllung"), gas_material_rows(material, locale)?))?;
        }
        ENERGY_GRANULARITY_CONSTRUCTION => {
            let construction = model.constructions.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.construction"), say(locale, "Construction", "Konstruktion"), construction_rows(model, construction, locale)?))?;
        }
        ENERGY_GRANULARITY_THERMOSTAT => {
            let thermostat = model.thermostats.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.thermostat"), say(locale, "Thermostat", "Thermostat"), thermostat_rows(model, thermostat, locale)?))?;
        }
        ENERGY_GRANULARITY_SHADING => {
            let shading = model.shading_surfaces.iter().find(|entry| entry.id == entity).ok_or_else(|| ui_error("ui.document"))?;
            push(&mut sections, section(&format!("{ROOT}.shading"), say(locale, "Shading surface", "Verschattungsfläche"), shading_rows(shading, locale)?))?;
        }
        _ => return summary(model, locale),
    }
    if let Some(rows) = action_rows(id, kind, locale)? {
        push(&mut sections, section(&format!("{ROOT}.actions"), say(locale, "Actions", "Aktionen"), rows))?;
    }
    column(sections)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
