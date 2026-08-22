//! 📄️ Puzzle 3d play app panel — the document tree: objects (with their vortices nested), reference
//! planes, target volumes and attractions, each row selecting its entity and carrying inline
//! hide/lock actions. The rendered sections are memoized by `Puzzle3dPlayApp` against the fixture's
//! geometry fingerprint, so this builder only reruns when the document actually changes.

use crate::editor::puzzle3d::terminology::Puzzle3dLabels;
use crate::editor::puzzle3d::{
    puzzle3d_vortex_full_id, Puzzle3dFixture, PUZZLE3D_GRANULARITY_ATTRACTION, PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_REFERENCE, PUZZLE3D_GRANULARITY_TARGET_VOLUME, PUZZLE3D_GRANULARITY_VORTEX, PUZZLE3D_INTERACTION_DOMAIN,
    PUZZLE3D_PLAY_CONTROLLER_ID,
};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, HasBase, HasChildren, Label, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{ActionFactory, InteractionTarget, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, INTERACTION_SELECT_ACTION_ID};
use semio_framework_ui_contract as ui;
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
fn action(action: &str, args: Option<Value>) -> (semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>) {
    ActionFactory::new(PUZZLE3D_PLAY_CONTROLLER_ID).action(action, args)
}

fn select_action(granularity: &str, id: &str) -> (semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>) {
    let targets = serde_json::to_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]).unwrap_or_default();
    action(INTERACTION_SELECT_ACTION_ID, Some(json!({ "domainId": PUZZLE3D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

fn binding((action, args): (semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>)) -> ActionBinding {
    ActionBinding { trigger: Trigger::Activate, action, args, capability: None }
}

fn selectable_item(id: impl Into<String>, label: impl Into<Label>, icon: &str, action: (semio_framework_ui_contract::ActionId, Option<semio_framework_ui_contract::UiValue>)) -> semio_framework_ui_contract::TreeItemBuilder {
    let (action_id, args) = action;
    let builder = ui::tree_item(label).id(id).icon(icon);
    match args {
        Some(args) => builder.on_with(Trigger::Activate, action_id, args),
        None => builder.on(Trigger::Activate, action_id),
    }
}

fn hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle3dLabels, flag_args: impl Fn(&str) -> Value) -> Vec<RowAction> {
    vec![
        RowAction {
            icon: if hidden { "eye-off".into() } else { "eye".into() },
            label: Some(if hidden { labels.show.as_str().into() } else { labels.hide.as_str().into() }),
            action: binding(action("setSelectionFlag", Some(flag_args("hidden")))),
            placement: RowActionPlacement::Row,
        },
        RowAction {
            icon: if locked { "lock".into() } else { "lock-open".into() },
            label: Some(if locked { labels.unlock.as_str().into() } else { labels.lock.as_str().into() }),
            action: binding(action("setSelectionFlag", Some(flag_args("locked")))),
            placement: RowActionPlacement::Row,
        },
    ]
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🌳️ The four document sections, memoized by the app against the fixture's geometry fingerprint.
pub fn render(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> BuiltNode {
    let object_items: Vec<BuiltNode> = fixture
        .objects
        .iter()
        .map(|object| {
            let vortex_items: Vec<BuiltNode> = object
                .vortices
                .iter()
                .map(|vortex| {
                    let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                    selectable_item(full_id.clone(), vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()), "circle-dot", select_action(PUZZLE3D_GRANULARITY_VORTEX, &full_id)).build()
                })
                .collect();
            let flag_args = {
                let id = object.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "object", "ids": [id.clone()] })
            };
            selectable_item(object.id.clone(), object.object_kind.clone().unwrap_or_else(|| object.id.clone()), "box", select_action(PUZZLE3D_GRANULARITY_OBJECT, &object.id))
                .default_open(false)
                .row_actions(hide_lock_actions(object.hidden, object.locked, labels, flag_args))
                .dimmed(object.hidden)
                .children(vortex_items)
                .build()
        })
        .collect();
    let reference_items: Vec<BuiltNode> = fixture
        .references
        .iter()
        .map(|reference| {
            let flag_args = {
                let id = reference.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "reference", "ids": [id.clone()] })
            };
            selectable_item(reference.id.clone(), reference.id.clone(), "globe", select_action(PUZZLE3D_GRANULARITY_REFERENCE, &reference.id))
                .description(reference.source.url.clone())
                .row_actions(hide_lock_actions(reference.hidden, reference.locked, labels, flag_args))
                .dimmed(reference.hidden)
                .build()
        })
        .collect();
    let target_volume_items: Vec<BuiltNode> = fixture
        .target_volumes
        .iter()
        .map(|volume| {
            let flag_args = {
                let id = volume.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "targetVolume", "ids": [id.clone()] })
            };
            selectable_item(volume.id.clone(), volume.id.clone(), "cylinder", select_action(PUZZLE3D_GRANULARITY_TARGET_VOLUME, &volume.id)).row_actions(hide_lock_actions(volume.hidden, volume.locked, labels, flag_args)).dimmed(volume.hidden).build()
        })
        .collect();
    let attraction_items: Vec<BuiltNode> =
        fixture.attractions.iter().map(|attraction| selectable_item(attraction.id.clone(), format!("{} → {}", attraction.attracting, attraction.attracted), "link", select_action(PUZZLE3D_GRANULARITY_ATTRACTION, &attraction.id)).build()).collect();
    PanelTreeBuilder::new("puzzle3d-play-document")
        .section("puzzle3d-play-document.objects", Some(labels.objects.as_str().into()), true, object_items)
        .section("puzzle3d-play-document.references", Some(labels.references.as_str().into()), false, reference_items)
        .section("puzzle3d-play-document.target-volumes", Some(labels.target_volumes.as_str().into()), false, target_volume_items)
        .section("puzzle3d-play-document.attractions", Some(labels.attractions.as_str().into()), false, attraction_items)
        .interaction_domain(PUZZLE3D_INTERACTION_DOMAIN)
        .build()
}
//#endregion 🔖️Render
