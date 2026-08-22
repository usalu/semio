//! 📄️ Puzzle 5d play app panel — the document tree: parts (with their grips nested) and fasteners,
//! each row selecting its entity — bound to the `vortex` interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so the framework paints selected/hovered
//! presence after render.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    find_part_by_grip_full_id, puzzle5d_grip_full_id, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dPart, Puzzle5dScene, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_INTERACTION_DOMAIN,
    PUZZLE5D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, BuiltNode, HasBase, HasChildren, Trigger};
use semio_framework_plugin::{ActionFactory, InteractionTarget, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract as ui;
use serde_json::json;

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
fn select_action(granularity: &str, id: &str) -> (semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>) {
    ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID).action(
        INTERACTION_SELECT_ACTION_ID,
        Some(json!({ "domainId": PUZZLE5D_INTERACTION_DOMAIN, "targets": serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default(), "merge": "replace", "method": "pick" })),
    )
}

fn selectable_item(
    id: impl Into<String>,
    label: impl Into<semio_framework_ui_contract::Label>,
    icon: &str,
    action: (semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>),
) -> semio_framework_ui_contract::TreeItemBuilder {
    let (action_id, args) = action;
    let builder = ui::tree_item(label).id(id).icon(icon);
    match args {
        Some(args) => builder.on_with(Trigger::Activate, action_id, args),
        None => builder.on(Trigger::Activate, action_id),
    }
}

pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> BuiltNode {
    let part_items: Vec<BuiltNode> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            let grip_items: Vec<BuiltNode> = part
                .grips
                .iter()
                .map(|grip| {
                    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
                    selectable_item(full_id.clone(), format!("{} ({})", grip.id, grip.grip_kind), "circle-dot", select_action(PUZZLE5D_GRANULARITY_GRIP, &full_id)).build()
                })
                .collect();
            selectable_item(part.id.clone(), part_label(part), "box", select_action(PUZZLE5D_GRANULARITY_PART, &part.id)).description(part.part_kind.clone()).children(grip_items).build()
        })
        .collect();
    let fastener_items: Vec<BuiltNode> =
        envelope.document.fasteners.iter().map(|fastener| selectable_item(fastener.id.clone(), fastener_label(&envelope.document, fastener), "link", select_action(PUZZLE5D_GRANULARITY_FASTENER, &fastener.id)).build()).collect();
    PanelTreeBuilder::new("puzzle5d-play-document")
        .section_or_placeholder("puzzle5d-play-document.parts", Some(labels.parts.as_str().into()), true, part_items, labels.none.as_str())
        .section_or_placeholder("puzzle5d-play-document.fasteners", Some(labels.fasteners.as_str().into()), false, fastener_items, labels.none.as_str())
        .interaction_domain(PUZZLE5D_INTERACTION_DOMAIN)
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
