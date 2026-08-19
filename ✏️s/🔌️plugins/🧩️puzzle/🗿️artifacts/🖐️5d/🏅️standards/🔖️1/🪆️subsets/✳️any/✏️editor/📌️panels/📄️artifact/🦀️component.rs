//! 📄️ Puzzle 5d play app panel — the document tree: parts (with their grips nested) and fasteners,
//! each row selecting its entity — bound to the `vortex` interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so the framework paints selected/hovered
//! presence after render.

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{find_part_by_grip_full_id, puzzle5d_grip_full_id, puzzle5d_interaction_select, tree_info_item, tree_item_with_action, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dPart, Puzzle5dScene, PUZZLE5D_GRANULARITY_FASTENER, PUZZLE5D_GRANULARITY_GRIP, PUZZLE5D_GRANULARITY_PART, PUZZLE5D_INTERACTION_DOMAIN};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
pub async fn part_label(part: &Puzzle5dPart) -> String {
    if !part.part_2d.text.is_empty() {
        return part.part_2d.text.clone();
    }
    part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone())
}

async fn fastener_label(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> String {
    let side = |full_id: &str| find_part_by_grip_full_id(document, full_id).map_or_else(|| full_id.to_string(), |(part, _)| part_label(part));
    format!("{} → {}", side(&fastener.source), side(&fastener.target))
}

//#endregion 🔖️Rows

//#region 🔖️Render
pub async fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    let part_items: Vec<UiTreeItemNode> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            let grip_items: Vec<UiTreeItemNode> = part
                .grips
                .iter()
                .map(|grip| {
                    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
                    tree_item_with_action(full_id.clone(), format!("{} ({})", grip.id, grip.grip_kind), Some("circle-dot"), puzzle5d_interaction_select(PUZZLE5D_GRANULARITY_GRIP, &full_id))
                })
                .collect();
            let mut item = tree_item_with_action(part.id.clone(), part_label(part), Some("box"), puzzle5d_interaction_select(PUZZLE5D_GRANULARITY_PART, &part.id));
            item.description = Some(part.part_kind.clone());
            if !grip_items.is_empty() {
                item.items = Some(grip_items);
            }
            item
        })
        .collect();
    let fastener_items: Vec<UiTreeItemNode> = envelope
        .document
        .fasteners
        .iter()
        .map(|fastener| tree_item_with_action(fastener.id.clone(), fastener_label(&envelope.document, fastener), Some("link"), puzzle5d_interaction_select(PUZZLE5D_GRANULARITY_FASTENER, &fastener.id)))
        .collect();
    let sections = vec![
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "puzzle5d-play-document.parts".into(),
            label: Some(labels.parts.into()),
            default_open: Some(true),
            items: if part_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.parts.empty", labels.none, None)] } else { part_items },
        },
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "puzzle5d-play-document.fasteners".into(),
            label: Some(labels.fasteners.into()),
            default_open: Some(false),
            items: if fastener_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.fasteners.empty", labels.none, None)] } else { fastener_items },
        },
    ];
    UiNode::Tree(UiTreeNode { presence: UiPresence::default(), sections, drop_action: None, menu: None, interaction_domain: Some(PUZZLE5D_INTERACTION_DOMAIN.into()) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle5d::testkit::*;

    #[test]
    async fn document_tree_lists_the_seeded_parts_section() {
        let mut app = app();
        assert!(render_body(&mut app, BODY_KEY).contains("puzzle5d-play-document.parts"));
    }
}
//#endregion 🧪️Tests
