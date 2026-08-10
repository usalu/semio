//! 📄️ Puzzle 3d play app panel — the document tree: objects (with their vortices nested), reference
//! planes, target volumes and attractions, each row selecting its entity and carrying inline
//! hide/lock actions. The rendered sections are memoized by `Puzzle3dPlayApp` against the fixture's
//! geometry fingerprint, so this builder only reruns when the document actually changes.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use crate::apps::puzzle3d::modes::edit::windows::main;
use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, puzzle3d_vortex_full_id, Puzzle3dFixture};
use semio_framework_plugin::{
    ActionDescriptor, IconName, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
    FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.document";
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
fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        presence: UiPresence::default(),
        id: id.into(),
        label: Label::data(label),
        description: None,
        icon_id: icon_id.map(IconName::from),
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}

fn hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle3dLabels, flag_args: impl Fn(&str) -> Value) -> Vec<UiTreeItemAction> {
    vec![
        UiTreeItemAction {
            icon_id: if hidden { "eye-off".into() } else { "eye".into() },
            label: Some(if hidden { labels.show.into() } else { labels.hide.into() }),
            action: puzzle3d_action("setSelectionFlag", Some(flag_args("hidden"))),
            placement: Some(UiTreeActionPlacement::Row),
        },
        UiTreeItemAction {
            icon_id: if locked { "lock".into() } else { "lock-open".into() },
            label: Some(if locked { labels.unlock.into() } else { labels.lock.into() }),
            action: puzzle3d_action("setSelectionFlag", Some(flag_args("locked"))),
            placement: Some(UiTreeActionPlacement::Row),
        },
    ]
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🌳️ The four document sections, memoized by the app against the fixture's geometry fingerprint.
pub fn sections(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> Vec<UiTreeSectionNode> {
    let object_items: Vec<UiTreeItemNode> = fixture
        .objects
        .iter()
        .map(|object| {
            let vortex_items: Vec<UiTreeItemNode> = object
                .vortices
                .iter()
                .map(|vortex| {
                    let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                    tree_item_with_action(
                        format!("puzzle3d-vortex:{full_id}"),
                        vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()),
                        Some("circle-dot"),
                        puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [full_id], "attractionIds": [] } }))),
                    )
                })
                .collect();
            let flag_args = {
                let id = object.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "object", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                presence: UiPresence::default(),
                id: format!("puzzle3d-object:{}", object.id),
                label: Label::data(object.object_kind.clone().unwrap_or_else(|| object.id.clone())),
                description: None,
                icon_id: Some("box".into()),
                default_open: Some(false),
                action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [object.id], "vortexIds": [], "attractionIds": [] } })))),
                hover_action: Some(puzzle3d_action("setHover", Some(json!({ "objectId": object.id })))),
                unhover_action: Some(puzzle3d_action("setHover", None)),
                actions: Some(hide_lock_actions(object.hidden, object.locked, labels, flag_args)),
                draggable: None,
                drag_data: None,
                items: if vortex_items.is_empty() { None } else { Some(vortex_items) },
                control: None,
                dimmed: Some(object.hidden),
                menu: None,
            }
        })
        .collect();
    let reference_items: Vec<UiTreeItemNode> = fixture
        .references
        .iter()
        .map(|reference| {
            let flag_args = {
                let id = reference.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "reference", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                presence: UiPresence::default(),
                id: format!("puzzle3d-reference:{}", reference.id),
                label: Label::data(reference.id.clone()),
                description: Some(reference.source.url.clone()),
                icon_id: Some("globe".into()),
                default_open: None,
                action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "referenceIds": [reference.id] } })))),
                hover_action: None,
                unhover_action: None,
                actions: Some(hide_lock_actions(reference.hidden, reference.locked, labels, flag_args)),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: Some(reference.hidden),
                menu: None,
            }
        })
        .collect();
    let target_volume_items: Vec<UiTreeItemNode> = fixture
        .target_volumes
        .iter()
        .map(|volume| {
            let flag_args = {
                let id = volume.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "targetVolume", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                presence: UiPresence::default(),
                id: format!("puzzle3d-target-volume:{}", volume.id),
                label: Label::data(volume.id.clone()),
                description: None,
                icon_id: Some("cylinder".into()),
                default_open: None,
                action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "targetVolumeIds": [volume.id] } })))),
                hover_action: None,
                unhover_action: None,
                actions: Some(hide_lock_actions(volume.hidden, volume.locked, labels, flag_args)),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: Some(volume.hidden),
                menu: None,
            }
        })
        .collect();
    let attraction_items: Vec<UiTreeItemNode> = fixture
        .attractions
        .iter()
        .map(|attraction| {
            tree_item_with_action(
                format!("puzzle3d-attraction:{}", attraction.id),
                format!("{} → {}", attraction.attracting, attraction.attracted),
                Some("link"),
                puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [attraction.id] } }))),
            )
        })
        .collect();
    vec![
        UiTreeSectionNode { id: "puzzle3d-play-document.objects".into(), label: Some(labels.objects.into()), default_open: Some(true), presence: UiPresence::default(), items: object_items },
        UiTreeSectionNode { id: "puzzle3d-play-document.references".into(), label: Some(labels.references.into()), default_open: Some(false), presence: UiPresence::default(), items: reference_items },
        UiTreeSectionNode { id: "puzzle3d-play-document.target-volumes".into(), label: Some(labels.target_volumes.into()), default_open: Some(false), presence: UiPresence::default(), items: target_volume_items },
        UiTreeSectionNode { id: "puzzle3d-play-document.attractions".into(), label: Some(labels.attractions.into()), default_open: Some(false), presence: UiPresence::default(), items: attraction_items },
    ]
}

/// 🌳️ Wraps memoized `sections` into the selection-aware tree node the panel body renders.
pub fn render(sections: Vec<UiTreeSectionNode>, selection: &Puzzle3dSelection) -> UiNode {
    UiNode::Tree(UiTreeNode {
        sections,
        presence: UiPresence::default(),
        selected_ids: Some(main::document_selected_ids(selection)),
        highlighted_ids: None,
        selection_change: Some(puzzle3d_action("setSelection", None)),
        drop_action: None,
        menu: None,
    })
}
//#endregion 🔖️Render
