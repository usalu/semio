//! 📒️ `shell-directory` staging of `settings-and-dock-persistence`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
impl DirectoryState {
    /// 💬️ Sends the draft to the connected agent — React's `sendAgentMessage`: one
    /// `ShellToGateway::AgentMessage` queued on the bridge outbox plus the identical turn echoed into
    /// the conversation, so the feed is never behind the human's own typing. A blank draft, or a draft
    /// typed while no socket is open, queues nothing and KEEPS the text rather than swallowing it.
    pub(crate) fn send_agent_chat_draft(&mut self, chrome: &mut ChromeState) -> bool {
        if !matches!(chrome.chrome_build.agent.status, crate::agent_bridge::AgentBridgeStatus::Open) {
            return false;
        }
        let trimmed = self.agent_chat_draft.trim().to_string();
        let session = self.shell_session_id.clone();
        if !chrome.chrome_build.agent.send_agent_message(&session, &trimmed) {
            return false;
        }
        self.agent_chat_draft.clear();
        true
    }

    /// 💬️ The `framework.chat` leaf's BODY — the LIVE agent conversation, the exact twin of React's
    /// `💬️AgentChatPanel/🟦️.tsx` feed: one row per `AgentConversationEntry` carrying a translated role
    /// noun and state chip, the tool name and its real arguments for a tool call, the result summary
    /// once it lands, and the parked approval's summary and decision. Nothing on this surface is
    /// generated locally — with no bridge attached the feed is empty and the composer is disabled,
    /// which is the honest state, not a simulated one. Entry ids mirror React's `entryElementId`.
    ///
    /// 🧾️ This is the packet's content-projection consumer: the panel body is assembled by the SHELL
    /// out of shell state rather than rendered by a guest, published through {@link panel_ui_records}
    /// like every other shell-owned leaf.
    ///
    /// 💬️ The FEED is its own node, under React's own id, rather than rows loose in the panel:
    /// that is what lets the transcript be addressed, labelled and paged as one thing — React's
    /// `<ol id="framework.chat.feed" aria-label={transcriptLabel} aria-live="polite">`.
    ///
    /// 📃️ React's feed SCROLLS and its effect pins it to the bottom on every new entry, so what
    /// the human sees is the newest window of the conversation. The wgpu panel body is a
    /// fixed-credit `MountedLayout` tree with no scroll of its own here, so the same window is
    /// produced by emitting the LAST `AGENT_CHAT_VISIBLE_ENTRIES` rows — the newest ones, in
    /// order. The bridge retains up to `AGENT_CONVERSATION_MAX_ENTRIES` (200) behind them, exactly
    /// as React's array does; nothing is discarded, only unpainted.
    ///
    /// ⌨️ React's composer submits on a bare Enter (Shift+Enter keeps the newline), which the
    /// retained input expresses as `commit: "enter"` plus an `on_submit` verb — the same
    /// `sendChatDraft` the Send button dispatches, so both routes are one code path.
    pub(crate) fn build_agent_chat_ui(&self, chrome: &ChromeState, prefs: &PrefsState) -> UiNode {
        let is_de = prefs.locale_id == "de";
        let locale = prefs.active_locale();
        let connected = matches!(chrome.chrome_build.agent.status, crate::agent_bridge::AgentBridgeStatus::Open);
        let mut children = Vec::new();
        let mut feed = Vec::new();
        if chrome.chrome_build.agent.conversation.is_empty() {
            feed.push(settings_text_row(shell_chrome_string("chat.empty", is_de)));
        }
        let conversation = &chrome.chrome_build.agent.conversation;
        let window = conversation.len().saturating_sub(AGENT_CHAT_VISIBLE_ENTRIES);
        for entry in conversation.iter().skip(window) {
            feed.push(agent_chat_entry_node(entry, is_de, locale));
        }
        children.push(UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some(FRAMEWORK_CHAT_FEED_ID.into()),
            children: feed,
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        }));
        if !connected {
            children.push(settings_text_row(shell_chrome_string("chat.disconnected", is_de)));
        }
        children.push(UiNode::Input(UiInputNode {
            id: "framework.chat.draft".into(),
            input_kind: "longText".into(),
            value: self.agent_chat_draft.clone(),
            placeholder: Some(Label::data(shell_chrome_string("chat.placeholder", is_de))),
            accessibility_label: Some(Label::data(shell_chrome_string("chat.draftLabel", is_de))),
            commit: Some("enter".into()),
            min: None,
            max: None,
            step: None,
            accept: None,
            precision: None,
            snaps: Vec::new(),
            on_change: ActionDescriptor { controller_id: "framework".into(), action: "setChatDraft".into(), args: None },
            on_submit: Some(ActionDescriptor { controller_id: "framework".into(), action: "sendChatDraft".into(), args: None }),
            on_abort: None,
            on_repeat_last: None,
            presence: UiPresence { state: if connected { ui_wgpu::wgpu::component::ui::UiState::Normal } else { ui_wgpu::wgpu::component::ui::UiState::Disabled }, ..UiPresence::default() },
            menu: None,
            ..Default::default()
        }));
        children.push(UiNode::Button(UiButtonNode {
            id: Some("framework.chat.send".into()),
            icon_id: IconName::ArrowRight,
            label: Label::data(shell_chrome_string("chat.send", is_de)),
            action: ActionDescriptor { controller_id: "framework".into(), action: "sendChatDraft".into(), args: None },
            style: None,
            presence: UiPresence { state: if connected && !self.agent_chat_draft.trim().is_empty() { ui_wgpu::wgpu::component::ui::UiState::Normal } else { ui_wgpu::wgpu::component::ui::UiState::Disabled }, ..UiPresence::default() },
            menu: None,
        }));
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: Some("framework.chat.panel".into()), children, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None })
    }
}

//#endregion ShellLifecycle

