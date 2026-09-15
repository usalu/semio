//! 🏊️ Sourcing curation app — the pool window: the full stock catalogue with filter chrome + drag source.

use crate::schema::{available_modules, curated_count, filtered_stock, typology_flatten};
use crate::{CurationSnapshot, ObjectKind, SortDirection};
use crate::editor::sourcing::config::SourcingCurationConfig;
use crate::editor::sourcing::terminology::SourcingLabels;
use crate::editor::sourcing::{sourcing_action, sourcing_table, sourcing_table_action, sourcing_table_row, ui_value_bool, ui_value_map, ui_value_text};
use semio_framework_plugin::plugin_app_close_prelude::{self as ui, InputKind, Label};
use semio_framework_plugin::{Buildable, BuiltNode, HasBase, HasChildren, LocalizedLabel, PluginAssemblyError, SurfaceKind, TableCell, Trigger, UiAssemblyResult, UiFixedList, UiText, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const SOURCING_CURATION_WINDOW_POOL: &str = "sourcing-pool";
pub const SOURCING_CURATION_BODY_POOL: &str = "sourcing.pool";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: SOURCING_CURATION_WINDOW_POOL.into(),
        label: LocalizedLabel::native("Pool", "Pool"),
        body_key: SOURCING_CURATION_BODY_POOL.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "library".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        interactions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
const SOURCING_CURATION_SURFACE_POOL: &str = "sourcing.pool.table";

fn pool_kinds(document: &CurationSnapshot, cfg: &SourcingCurationConfig) -> Vec<ObjectKind> {
    let mut filtered = filtered_stock(document, &cfg.filters);
    if let Some(sort) = &cfg.filters.sort {
        filtered.sort_by(|a, b| {
            let ordering = match sort.column_id.as_str() {
                "availability" => a.availability.cmp(&b.availability),
                "module" => a.module_id.cmp(&b.module_id),
                _ => a.name.cmp(&b.name),
            };
            if sort.direction == SortDirection::Desc {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }
    filtered
}

fn pool_row(document: &CurationSnapshot, kind: &ObjectKind) -> protocol::DslValue {
    sourcing_table_row(
        &kind.id,
        vec![
            ("name", TableCell::Text { value: kind.name.clone() }),
            ("module", TableCell::Text { value: kind.module_id.clone() }),
            ("typology", TableCell::Text { value: kind.typology_path.join(" / ") }),
            ("availability", TableCell::Number { value: kind.availability as f64 }),
            ("curated", TableCell::Stepper { value: curated_count(document, &kind.id) as f64, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_table_action("curationSetCount", Some(&kind.id)) }),
        ],
    )
}

fn filter_error(stage: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("sourcing pool filter admission failed at {stage}"))
}

fn ui_text(value: impl AsRef<str>) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value.as_ref()).ok_or_else(|| filter_error("text"))
}

fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<Label> {
    Label::try_from(value.as_ref().to_string()).map_err(|_| filter_error("label"))
}

/// 🔍️ The pool's filter chrome: free-text query, one toggle per module, the typology path, and the
/// minimum availability — each bound to its session-only config command.
fn filter_bar(cfg: &SourcingCurationConfig, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    let filters = &cfg.filters;
    let mut children = UiFixedList::<BuiltNode>::default();
    let (query_action, _) = sourcing_action("setFilterQuery", None)?;
    let query = ui::input(InputKind::Text).value(ui_text(&filters.query)?).placeholder(ui_label(labels.search_placeholder.as_str())?);
    children.try_push(query.try_id("sourcing-filter-query").map_err(|_| filter_error("query-id"))?.try_on(Trigger::Change, query_action).map_err(|_| filter_error("query-binding"))?.try_build().map_err(|_| filter_error("query-build"))?).map_err(|_| filter_error("children"))?;
    let modules = available_modules(&cfg.contributions_json);
    for module in &modules {
        let pressed = filters.module_ids.iter().any(|id| id == &module.module_id);
        let (action, args) = sourcing_action("setFilterModule", Some(ui_value_map([("moduleId", ui_value_text(&module.module_id)?), ("enabled", ui_value_bool(!pressed))])?))?;
        let toggle = ui::toggle(pressed).icon(ui_text("layers")?).text(ui_label(&module.label)?).try_id(format!("sourcing-filter-module-{}", module.module_id)).map_err(|_| filter_error("module-id"))?;
        let toggle = match args {
            Some(args) => toggle.try_on_with(Trigger::Change, action, args),
            None => toggle.try_on(Trigger::Change, action),
        }
        .map_err(|_| filter_error("module-binding"))?;
        children.try_push(toggle.try_build().map_err(|_| filter_error("module-build"))?).map_err(|_| filter_error("children"))?;
    }
    let (typology_action, _) = sourcing_action("setFilterTypology", None)?;
    let mut typology = ui::select(ui_text(filters.typology_path.join("/"))?).try_item(ui_text("")?, ui_label(labels.all_typologies.as_str())?).map_err(|_| filter_error("typology-all"))?;
    for module in &modules {
        for (path, label) in typology_flatten(&module.typology) {
            typology = typology.try_item(ui_text(path.join("/"))?, ui_label(label)?).map_err(|_| filter_error("typology-item"))?;
        }
    }
    children.try_push(typology.try_id("sourcing-filter-typology").map_err(|_| filter_error("typology-id"))?.try_on(Trigger::Change, typology_action).map_err(|_| filter_error("typology-binding"))?.try_build().map_err(|_| filter_error("typology-build"))?).map_err(|_| filter_error("children"))?;
    let (availability_action, _) = sourcing_action("setFilterMinAvailability", None)?;
    let availability = ui::input(InputKind::Number).value(ui_text(filters.min_availability.to_string())?);
    children.try_push(availability.try_id("sourcing-filter-min-availability").map_err(|_| filter_error("availability-id"))?.try_on(Trigger::Change, availability_action).map_err(|_| filter_error("availability-binding"))?.try_build().map_err(|_| filter_error("availability-build"))?).map_err(|_| filter_error("children"))?;
    ui::row().try_id("sourcing-pool-filters").map_err(|_| filter_error("row-id"))?.try_children(children).map_err(|_| filter_error("row-children"))?.try_build().map_err(|_| filter_error("row-build"))
}

pub fn render(document: &CurationSnapshot, cfg: &SourcingCurationConfig, labels: &SourcingLabels) -> UiAssemblyResult<BuiltNode> {
    let columns = [
        ("name", labels.col_name.as_str(), true),
        ("module", labels.col_module.as_str(), true),
        ("typology", labels.col_typology.as_str(), false),
        ("availability", labels.col_availability.as_str(), true),
        ("curated", labels.col_curated.as_str(), false),
    ];
    let rows = pool_kinds(document, cfg).iter().map(|kind| pool_row(document, kind)).collect();
    let table = sourcing_table(SOURCING_CURATION_SURFACE_POOL, &columns, rows, "dropOnPool", cfg.filters.sort.as_ref())?;
    let mut children = UiFixedList::<BuiltNode>::default();
    children.try_push(filter_bar(cfg, labels)?).map_err(|_| filter_error("children"))?;
    children.try_push(table).map_err(|_| filter_error("children"))?;
    ui::column().try_id("sourcing-pool").map_err(|_| filter_error("column-id"))?.try_children(children).map_err(|_| filter_error("column-children"))?.try_build().map_err(|_| filter_error("column-build"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
