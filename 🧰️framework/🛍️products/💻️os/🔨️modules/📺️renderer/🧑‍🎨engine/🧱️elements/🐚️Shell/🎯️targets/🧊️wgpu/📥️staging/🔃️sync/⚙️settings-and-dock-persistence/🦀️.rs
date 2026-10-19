//! 🔃️ `shell-sync` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl SyncState {
    /// 🔄️ The retained body behind the always-present Sync leaf. Its three choice actions are the
    /// existing `framework.sync` controller routes; the body only projects that authority into the dock.
    pub fn build_sync_attach_ui(&self, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let active_kind = self.sync_backbone_uri.as_deref().and_then(|uri| {
            if uri.starts_with("file://") {
                Some("file")
            } else if uri.starts_with("folder://") {
                Some("folder")
            } else if uri.starts_with("remote://") {
                Some("remote")
            } else {
                None
            }
        });
        let mut children = Vec::new();
        for (kind, icon_id, label_key, action) in [("file", "file-json", "sync.file", "selectFile"), ("folder", "folder", "sync.folder", "selectFolder"), ("remote", "cloud", "sync.remote", "selectRemote")] {
            children.push(UiNode::Button(UiButtonNode {
                id: Some(format!("framework.sync.{kind}")),
                icon_id: IconName::from(icon_id),
                label: Label::data(shell_chrome_string(label_key, is_de)),
                action: ActionDescriptor { controller_id: "framework.sync".into(), action: action.into(), args: None },
                style: None,
                presence: UiPresence { selected: active_kind == Some(kind), ..UiPresence::default() },
                menu: None,
            }));
        }
        if let Some(kind) = self.sync_card_kind.as_deref().filter(|kind| matches!(*kind, "file" | "folder" | "remote")) {
            let placeholder = match kind {
                "remote" => "127.0.0.1:8787/studio-1/demo",
                "folder" => "/absolute/project/folder",
                _ => "/absolute/document.json",
            };
            children.push(UiNode::Input(UiInputNode {
                id: format!("framework.sync.{kind}.path"),
                input_kind: "text".into(),
                value: self.sync_card_draft.clone(),
                placeholder: Some(Label::data(placeholder)),
                accessibility_label: None,
                commit: None,
                min: None,
                max: None,
                step: None,
                accept: None,
                precision: None,
                snaps: Vec::new(),
                on_change: ActionDescriptor { controller_id: "framework.sync".into(), action: "setSyncDraft".into(), args: None },
                on_submit: Some(ActionDescriptor { controller_id: "framework.sync".into(), action: "attach".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "kind": kind }) }),
                on_abort: None,
                on_repeat_last: None,
                presence: UiPresence::default(),
                menu: None,
                ..Default::default()
            }));
            if kind != "remote" {
                children.push(UiNode::Button(UiButtonNode {
                    id: Some("framework.sync.browse".into()),
                    icon_id: IconName::from("folder-open"),
                    label: Label::data(shell_chrome_string("sync.browse", is_de)),
                    action: ActionDescriptor { controller_id: "framework.sync".into(), action: "browse".into(), args: None },
                    style: None,
                    presence: UiPresence::default(),
                    menu: None,
                }));
            }
            children.push(UiNode::Button(UiButtonNode {
                id: Some("framework.sync.attach".into()),
                icon_id: IconName::from("link"),
                label: Label::data(shell_chrome_string("sync.attach", is_de)),
                action: ActionDescriptor { controller_id: "framework.sync".into(), action: "attach".into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "kind": kind }) },
                style: None,
                presence: UiPresence { state: if self.sync_card_draft.trim().is_empty() { semio_framework_ui_declaration::ui::UiState::Disabled } else { semio_framework_ui_declaration::ui::UiState::Normal }, ..UiPresence::default() },
                menu: None,
            }));
        }
        if let Some(uri) = self.sync_backbone_uri.as_deref() {
            children.push(UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some("framework.sync.active".into()),
                children: vec![settings_text_row(uri)],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            }));
            children.push(UiNode::Button(UiButtonNode {
                id: Some("framework.sync.detach".into()),
                icon_id: IconName::X,
                label: Label::data(shell_chrome_string("sync.detach", is_de)),
                action: ActionDescriptor { controller_id: "framework.sync".into(), action: "detach".into(), args: None },
                style: None,
                presence: UiPresence::default(),
                menu: None,
            }));
        }
        if let Some(line) = self.sync_link.text(is_de) {
            children.push(UiNode::Stack(UiStackNode {
                direction: "column".into(),
                gap: None,
                padding: None,
                id: Some(format!("framework.sync.link.{}", self.sync_link.code())),
                children: vec![settings_text_row(line)],
                presence: UiPresence::default(),
                activate: None,
                drop_action: None,
                drop_overlay: None,
                menu: None,
            }));
        }
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("framework.sync.panel".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }
}

//#endregion ShellLifecycle

