//! 📄️ Puzzle 5d play app panel — the document tree: parts (with their grips nested) and fasteners,
//! each row selecting its entity — bound to the `vortex` interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so the framework paints selected/hovered
//! presence after render.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    find_part_by_grip_full_id, puzzle5d_grip_full_id, ui_label, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dPart, Puzzle5dScene, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_INTERACTION_DOMAIN,
    PUZZLE5D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, HasChildren, Trigger};
use semio_framework_plugin::{ActionFactory, InteractionTarget, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
/// 🏷️ A part's display label: its flat text, else its volume label, else its kind.
pub fn part_label(part: &Puzzle5dPart) -> String {
    if !part.part_2d.text.is_empty() {
        return part.part_2d.text.clone();
    }
    part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone())
}

fn fastener_label(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> String {
    let side = |full_id: &str| find_part_by_grip_full_id(document, full_id).map_or_else(|| full_id.to_string(), |(part, _)| part_label(part));
    format!("{} → {}", side(&fastener.source), side(&fastener.target))
}

//#endregion 🔖️Rows

//#region 🔖️Render
fn ui_text_value(value: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value)
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.text", "fixed action text admission failed"))
}

fn ui_map_value(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.action.map", "fixed action map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.action.map.entry", "fixed action map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

fn select_action(granularity: &str, id: &str) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>)> {
    let targets = serde_json::to_string(&[InteractionTarget { granularity: granularity.into(), id: id.into() }])
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.action.targets", "selection target encoding failed"))?;
    let args = ui_map_value([
        ("domainId", ui_text_value(PUZZLE5D_INTERACTION_DOMAIN)?),
        ("merge", ui_text_value("replace")?),
        ("method", ui_text_value("pick")?),
        ("targets", ui_text_value(&targets)?),
    ])?;
    ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action(INTERACTION_SELECT_ACTION_ID, Some(args))
}

fn selectable_item(
    id: impl AsRef<str>,
    label: impl AsRef<str>,
    icon: &str,
    action: semio_framework_plugin::UiAssemblyResult<(semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::TreeItemBuilder> {
    let (action_id, args) = action?;
    let builder = ui::tree_item(ui_label(label)?)
        .try_id(id.as_ref())
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.id", "tree item id admission failed"))?
        .icon(semio_framework_plugin::UiText::try_from_str(icon).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.icon", "tree item icon admission failed"))?);
    match args {
        Some(args) => builder.try_on_with(Trigger::Activate, action_id, args),
        None => builder.try_on(Trigger::Activate, action_id),
    }
    .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.action", "tree item action admission failed"))
}

pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut part_items = semio_framework_plugin::UiFixedList::default();
    for part in &envelope.document.parts {
        let mut grip_items = semio_framework_plugin::UiFixedList::<BuiltNode>::default();
        for grip in &part.grips {
            let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
            let grip_item = selectable_item(full_id.clone(), format!("{} ({})", grip.id, grip.grip_kind), "circle-dot", select_action(PUZZLE5D_GRANULARITY_GRIP, &full_id))?
                .try_build()
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.grip", "grip row admission failed"))?;
            grip_items
                .try_push(grip_item)
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.grips", "grip list admission failed"))?;
        }
        let part_item = selectable_item(part.id.clone(), part_label(part), "box", select_action(PUZZLE5D_GRANULARITY_PART, &part.id))?
            .description(semio_framework_plugin::UiText::try_from_string(part.part_kind.clone()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.part-description", "part description admission failed"))?)
            .try_children(grip_items)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.part-children", "part child admission failed"))?
            .try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.part", "part row admission failed"))?;
        part_items
            .try_push(part_item)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.parts", "part list admission failed"))?;
    }
    let mut fastener_items = semio_framework_plugin::UiFixedList::default();
    for fastener in &envelope.document.fasteners {
        let fastener_item = selectable_item(
            fastener.id.clone(),
            fastener_label(&envelope.document, fastener),
            "link",
            select_action(PUZZLE5D_GRANULARITY_FASTENER, &fastener.id),
        )?
        .try_build()
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.fastener", "fastener row admission failed"))?;
        fastener_items
            .try_push(fastener_item)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.fasteners", "fastener list admission failed"))?;
    }
    PanelTreeBuilder::new("puzzle5d-play-document")?
        .section_or_placeholder("puzzle5d-play-document.parts", Some(ui_label(labels.parts.as_str())?), true, part_items, ui_label(labels.none.as_str())?)?
        .section_or_placeholder("puzzle5d-play-document.fasteners", Some(ui_label(labels.fasteners.as_str())?), false, fastener_items, ui_label(labels.none.as_str())?)?
        .interaction_domain(PUZZLE5D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle5d::testkit::*;

    #[test]
    fn document_tree_lists_the_seeded_parts_section() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-document.parts"));
    }
}
//#endregion 🧪️Tests
