//! 🧯️ Status builders: the empty state, error state, recovery panel and import drop zone builders. See [`super`].

use super::*;

// #region ui
//#region 🔖️UiNode
//#region 🔖️StatusBuilders
/** 🗂️ Builds an empty-state placeholder: a centered title, optional description text, and an
     * optional call-to-action button. */
pub fn ui_empty_state(id: &str, title: Label, description: Option<Label>, action: Option<UiButtonNode>) -> UiNode {
    let mut children = vec![UiNode::Text(UiTextNode { menu: None, value: title, emphasize: Some(true), data_attributes: None, presence: UiPresence::default() })];
    if let Some(description) = description {
        children.push(ui_text(description));
    }
    if let Some(action) = action {
        children.push(UiNode::Button(action));
    }
    UiNode::Stack(UiStackNode {
        menu: None,
        direction: "vertical".into(),
        gap: Some("standard".into()),
        padding: Some("standard".into()),
        id: Some(id.into()),
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children,
    })
}

/** ⚠️ Builds an error-state placeholder: an emphasized message and an optional retry button. */
pub fn ui_error_state(id: &str, message: Label, retry: Option<ActionDescriptor>) -> UiNode {
    let mut children = vec![UiNode::Text(UiTextNode { menu: None, value: message, emphasize: Some(true), data_attributes: None, presence: UiPresence::default() })];
    if let Some(retry) = retry {
        children.push(UiNode::Button(UiButtonNode {
            menu: None,
            id: Some(format!("{id}.retry")),
            icon_id: IconName::RotateCw,
            // 🚧️ Framework-level fallback copy (not app content); not yet routed through app_labels!
            // since this SDK-level builder predates the two-axis macro — flagged for a follow-up pass.
            label: Label::data("Retry"),
            action: retry,
            style: None,
            presence: UiPresence::default(),
        }));
    }
    UiNode::Stack(UiStackNode {
        menu: None,
        direction: "vertical".into(),
        gap: Some("standard".into()),
        padding: Some("standard".into()),
        id: Some(id.into()),
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children,
    })
}

/** 🩺️ Builds a plugin-recovery panel: bilingual (en/de) crash copy plus three fixed actions —
     * restart the app (`recovery.restartApp`), disable the offending program (`recovery.disablePlugin`), or
     * open diagnostics (`recovery.showDiagnostics`). `quarantined` swaps in the host-auto-disabled copy. */
pub fn ui_recovery_panel(plugin_id: &str, quarantined: bool, is_de: bool) -> UiNode {
    let title = if is_de { "Plugin-Wiederherstellung" } else { "Plugin Recovery" };
    let message = match (quarantined, is_de) {
        (true, true) => "Dieses Plugin wurde nach wiederholten Abstürzen unter Quarantäne gestellt.",
        (true, false) => "This program was quarantined after repeated crashes.",
        (false, true) => "Dieses Plugin ist abgestürzt.",
        (false, false) => "This program crashed.",
    };
    let restart_label = if is_de { "App neu starten" } else { "Restart App" };
    let disable_label = if is_de { "Plugin deaktivieren" } else { "Disable Plugin" };
    let diagnostics_label = if is_de { "Diagnose anzeigen" } else { "Show Diagnostics" };
    let args = Some(DslValue::Object(vec![("pluginId".into(), DslValue::String(plugin_id.to_string()))]));
    UiNode::Stack(UiStackNode {
        menu: None,
        direction: "vertical".into(),
        gap: Some("standard".into()),
        padding: Some("standard".into()),
        id: Some("recovery.panel".into()),
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children: vec![
            UiNode::Text(UiTextNode { menu: None, value: Label::data(title), emphasize: Some(true), data_attributes: None, presence: UiPresence::default() }),
            ui_text(Label::data(message)),
            UiNode::Button(UiButtonNode {
                menu: None,
                id: Some("recovery.restartApp".into()),
                icon_id: IconName::RotateCcw,
                label: Label::data(restart_label),
                action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.restartApp".into(), args: args.clone() },
                style: None,
                presence: UiPresence::default(),
            }),
            UiNode::Button(UiButtonNode {
                menu: None,
                id: Some("recovery.disablePlugin".into()),
                icon_id: IconName::Link2Off,
                label: Label::data(disable_label),
                action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.disablePlugin".into(), args: args.clone() },
                style: None,
                presence: UiPresence::default(),
            }),
            UiNode::Button(UiButtonNode {
                menu: None,
                id: Some("recovery.showDiagnostics".into()),
                icon_id: IconName::Info,
                label: Label::data(diagnostics_label),
                action: ActionDescriptor { controller_id: "recovery".into(), action: "recovery.showDiagnostics".into(), args },
                style: None,
                presence: UiPresence::default(),
            }),
        ],
    })
}

/** 📥️ Builds a drop-zone `Stack` for importing files: `drop_overlay` supplies the hover-state
     * title/hint/accept copy, `drop_action` fires once the drop completes. */
pub fn ui_import_drop_zone(id: &str, title: Label, hint: Label, accept: Option<&str>, drop_action: ActionDescriptor) -> UiNode {
    UiNode::Stack(UiStackNode {
        menu: None,
        direction: "vertical".into(),
        gap: Some("standard".into()),
        padding: Some("standard".into()),
        id: Some(id.into()),
        presence: UiPresence::default(),
        activate: None,
        drop_action: Some(drop_action),
        drop_overlay: Some(UiDropOverlaySpec { title: title.clone(), hint: hint.clone(), accept: accept.map(Into::into) }),
        children: vec![ui_text(title), ui_text(hint)],
    })
}
//#endregion 🔖️StatusBuilders
//#endregion 🔖️UiNode
// #endregion ui
