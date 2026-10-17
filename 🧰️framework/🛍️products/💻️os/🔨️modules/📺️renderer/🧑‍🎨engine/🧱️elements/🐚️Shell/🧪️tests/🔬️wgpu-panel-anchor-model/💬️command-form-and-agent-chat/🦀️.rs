/// 💬️ Command form and agent chat: the staged command form and the agent chat transcript, cancel and composer laws.

//#region 🎛️StagedCommandForm
/// 🎛️ `expanded_command_id`'s READER — the gap `📓️w1c` recorded. The expanded command's staged form is
/// a section of its own carrying one control per argument plus Execute and Reset, and that command is
/// filtered out of the list section below it (React's `buildCommandCategoryTree`).
#[test]
fn the_command_dock_opens_the_expanded_commands_staged_form() {
    let mut shell = fresh_state();
    let entry = shell.docs.resolved_commands(&shell.chrome, &shell.plugins).into_iter().find(|entry| entry.definition.in_palette && !entry.definition.args.is_empty()).expect("an arg-carrying os command");
    let key = command_address_stable_key(&entry.address);
    let category = entry.definition.category.clone();
    shell.commands.expanded_command_id = Some(key.clone());
    let UiNode::Tree(panel) = SessionCtx::of(&mut shell).build_command_category_ui(&category) else { panic!("expected a Tree root") };
    let form = panel.sections.first().expect("the expanded form is the FIRST section, so a bottom anchor paints it above the list");
    assert_eq!(form.id, format!("command.category.{category}.form"));
    assert_eq!(form.default_open, None, "React TreeDataSection defaults closed");
    let toolbar = form.header_toolbar.as_ref().expect("form header toolbar");
    let buttons: Vec<&str> = toolbar.children.iter().filter_map(|node| if let UiNode::Button(button) = node { button.id.as_deref() } else { None }).collect();
    assert_eq!(buttons.len(), 2, "🎛️ Execute and Reset, React's own two section actions");
    assert!(buttons[0].ends_with("-execute") && buttons[1].ends_with("-reset"));
    let arg_rows = form.items.iter().filter(|row| row.control.is_some()).count();
    assert_eq!(arg_rows, entry.definition.args.len(), "🎛️ one control per declared argument");
    let list = panel.sections.iter().find(|section| section.id == "command.category.list");
    let listed: Vec<&str> = list.map(|section| section.items.iter().map(|row| row.id.as_str()).collect()).unwrap_or_default();
    let expanded_id = format!("command.{}", key.replace(':', "."));
    assert!(!listed.contains(&expanded_id.as_str()), "🎛️ the expanded command is not also listed");
    let records = panel_ui_records("command.fixture", &UiNode::Tree(panel)).expect("command Tree projects");
    let form_record = records.iter().find(|record| record.key.as_str() == format!("command.fixture/command.category.{category}.form")).expect("form record");
    let ui_contract::Component::TreeSection(props) = &form_record.component else { panic!("form is a TreeSection") };
    let toolbar_id = props.header_toolbar.expect("TreeSection carries its header Toolbar relation");
    let toolbar_record = records.iter().find(|record| record.id == toolbar_id).expect("toolbar record");
    assert!(matches!(&toolbar_record.component, ui_contract::Component::Container(props) if props.role == ui_contract::ContainerRole::Toolbar));
    assert_eq!(toolbar_record.children.len(), 2);
    assert!(toolbar_record.children.iter().all(|id| records.iter().find(|record| record.id == *id).is_some_and(|record| matches!(&record.component, ui_contract::Component::Button(_)))));
}

/// 🎛️ An Execute is DISABLED while a required argument is unstaged — React's `missing.length > 0` gate.
#[test]
fn execute_is_disabled_until_every_required_argument_is_staged() {
    let mut shell = fresh_state();
    let entry = shell.docs.resolved_commands(&shell.chrome, &shell.plugins).into_iter().find(|entry| entry.definition.in_palette && resolved_execute_args(&entry.definition.args, &serde_json::Map::new()).is_none()).and_then(|entry| {
        let arg = entry.definition.args.iter().find(|arg| arg.required && arg.default.is_none())?;
        let value = match arg.control() {
            semio_framework::ActionArgControl::Select { options } => serde_json::Value::String(options.first()?.value.clone()),
            semio_framework::ActionArgControl::Toggle => serde_json::Value::Bool(true),
            semio_framework::ActionArgControl::Number { min, .. } => serde_json::json!(min.unwrap_or(0.0)),
            semio_framework::ActionArgControl::Slider { min, .. } => serde_json::json!(min),
            _ => serde_json::Value::String("x".into()),
        };
        Some((command_address_stable_key(&entry.address), entry.definition.category.clone(), arg.id.clone(), value))
    });
    let Some((key, category, arg_id, value)) = entry else {
        return;
    };
    shell.commands.expanded_command_id = Some(key.clone());
    let disabled = |shell: &ShellState| {
        let UiNode::Tree(panel) = SessionCtx::of(&mut shell).build_command_category_ui(&category) else { panic!("Tree root") };
        let form = panel.sections.first().expect("form section");
        form.header_toolbar
            .as_ref()
            .expect("header toolbar")
            .children
            .iter()
            .any(|node| matches!(node, UiNode::Button(button) if button.id.as_deref().is_some_and(|id| id.ends_with("-execute")) && matches!(button.presence.state, semio_framework_ui_layout::layout::UiState::Disabled)))
    };
    assert!(disabled(&shell), "🎛️ unstaged required argument blocks Execute");
    shell.commands.staged_command_args.entry(key).or_default().insert(arg_id, value);
    assert!(!disabled(&shell), "🎛️ staging it enables Execute");
}
//#endregion 🎛️StagedCommandForm

//#region 💬️AgentChatTranscript
/// 🌉️ Opens the bridge the way a transport does — a `Welcome` frame — so the panel is in its
/// connected state and the composer is live.
fn welcomed_shell() -> ShellState {
    let mut shell = fresh_state();
    let welcome = semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::GatewayToShell::Welcome { bridge_version: semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::BRIDGE_VERSION, connection: "connection-1".into(), principal: "agent:local".into() };
    shell.chrome.apply_agent_bridge_frame(&welcome.encode()).expect("a Welcome frame decodes");
    shell
}

/// 💬️ Every text line the chat panel's own subtree paints, in order — the feed's readable content.
fn chat_panel_lines(node: &UiNode, out: &mut Vec<String>) {
    match node {
        UiNode::Text(text) => out.push(text.value.as_str().to_string()),
        UiNode::Stack(stack) => stack.children.iter().for_each(|child| chat_panel_lines(child, out)),
        _ => {}
    }
}

/// 🆔️ The ids of the conversation rows inside the panel's own FEED — React's
/// `<ol id="framework.chat.feed">`, which the wgpu body wraps its rows in so the transcript is one
/// addressable, labelled thing rather than rows loose beside the composer.
fn chat_entry_ids(root: &UiNode) -> Vec<String> {
    let UiNode::Stack(panel) = root else { panic!("💬️ the chat panel root is a stack") };
    let feed = panel
        .children
        .iter()
        .find_map(|child| match child {
            UiNode::Stack(stack) if stack.id.as_deref() == Some(FRAMEWORK_CHAT_FEED_ID) => Some(stack),
            _ => None,
        })
        .expect("💬️ the chat panel publishes its feed");
    feed.children.iter().filter_map(|child| if let UiNode::Stack(stack) = child { stack.id.clone() } else { None }).collect()
}

fn chat_button_action(root: &UiNode, id: &str) -> Option<ActionDescriptor> {
    match root {
        UiNode::Button(button) if button.id.as_deref() == Some(id) => Some(button.action.clone()),
        UiNode::Stack(stack) => stack.children.iter().find_map(|child| chat_button_action(child, id)),
        _ => None,
    }
}

/// 💬️ LAW: a bridge `AgentToolCall` frame — the one the gateway emits from its own `tools/call`
/// dispatch — becomes a conversation row in the wgpu panel model, and the matching `AgentToolResult`
/// folds its summary into that SAME row rather than opening a second one. This is the parity the wgpu
/// renderer owed React's `💬️AgentChatPanel` feed; nothing here is generated locally.
#[test]
fn a_bridge_tool_call_frame_becomes_a_conversation_row_in_the_chat_panel() {
    let mut shell = welcomed_shell();
    assert!(chat_entry_ids(&shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs)).is_empty(), "💬️ no bridge activity, no rows");

    let call = semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::GatewayToShell::AgentToolCall { invocation_id: "inv_7".into(), tool_name: "artifact_open".into(), arguments: "{\"artifactId\":\"doc-1\"}".into() };
    shell.chrome.apply_agent_bridge_frame(&call.encode()).expect("an AgentToolCall frame decodes");
    let running = shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs);
    assert_eq!(chat_entry_ids(&running), vec!["framework.chat.entry.toolCall.inv_7".to_string()], "💬️ the tool call is one addressable row");
    let mut lines = Vec::new();
    chat_panel_lines(&running, &mut lines);
    assert!(lines.contains(&"Tool call".to_string()), "💬️ the translated role noun leads the row: {lines:?}");
    assert!(lines.contains(&"Running…".to_string()), "💬️ a call with no result yet is running: {lines:?}");
    assert!(lines.contains(&"artifact_open".to_string()), "💬️ the real tool name: {lines:?}");
    assert!(lines.contains(&"{\"artifactId\":\"doc-1\"}".to_string()), "💬️ and its real arguments: {lines:?}");
    assert!(panel_ui_records(FRAMEWORK_CHAT_PANEL_ID, &running).is_ok(), "💬️ the conversation projects as publishable panel records");

    let result = semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::GatewayToShell::AgentToolResult { invocation_id: "inv_7".into(), tool_name: "artifact_open".into(), ok: true, summary: "opened doc-1".into() };
    shell.chrome.apply_agent_bridge_frame(&result.encode()).expect("an AgentToolResult frame decodes");
    let settled = shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs);
    assert_eq!(chat_entry_ids(&settled), vec!["framework.chat.entry.toolCall.inv_7".to_string()], "💬️ one row per invocation, never two");
    let mut settled_lines = Vec::new();
    chat_panel_lines(&settled, &mut settled_lines);
    assert!(settled_lines.contains(&"Done".to_string()), "💬️ the state chip settles: {settled_lines:?}");
    assert!(settled_lines.contains(&"Result: opened doc-1".to_string()), "💬️ the summary folds into the call's own row: {settled_lines:?}");
}

/// 🛑️ The actual retained button carries the gateway's invocation id into Shell dispatch, sends one
/// cancel frame, retires itself while cancellation is pending, and trusts only the result to settle.
#[test]
fn the_chat_cancel_control_drives_the_shared_invocation_lifecycle_end_to_end() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🔗️AgentBridge/🧪️tests/🧫️fixtures/🛑️cancellation/🔣️.json")).expect("cancellation fixture");
    let invocation_id = fixture["invocation"]["id"].as_str().expect("invocation id");
    let tool_name = fixture["invocation"]["toolName"].as_str().expect("tool name");
    let arguments = fixture["invocation"]["arguments"].as_str().expect("arguments");
    let mut shell = welcomed_shell();
    shell.chrome.apply_agent_bridge_frame(&semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::GatewayToShell::AgentToolCall { invocation_id: invocation_id.into(), tool_name: tool_name.into(), arguments: arguments.into() }.encode()).expect("tool call frame");
    let control_id = format!("framework.chat.cancel.{invocation_id}");
    let action = chat_button_action(&shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs), &control_id).expect("a running call publishes its cancel control");
    assert_eq!(action.action, "cancelToolCall");
    assert_eq!(action.args.as_ref().and_then(|args| args.get("invocationId")).and_then(DslValue::as_str), Some(invocation_id));
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(action)).expect("framework owns cancellation");
    let frames = shell.chrome.take_agent_bridge_outbox();
    assert_eq!(frames.len(), 1);
    assert_eq!(semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::ShellToGateway::decode(&frames[0]).expect("cancel frame"), semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::ShellToGateway::AgentCancel { invocation_id: invocation_id.into() });
    let cancelling = shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs);
    assert!(chat_button_action(&cancelling, &control_id).is_none(), "a cancelling row cannot send a duplicate control action");
    let mut cancelling_lines = Vec::new();
    chat_panel_lines(&cancelling, &mut cancelling_lines);
    assert!(cancelling_lines.contains(&"Cancelling…".to_string()), "optimistic state is visible: {cancelling_lines:?}");

    shell.chrome
        .apply_agent_bridge_frame(
            &semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::GatewayToShell::AgentToolResult {
                invocation_id: invocation_id.into(),
                tool_name: tool_name.into(),
                ok: fixture["terminalResult"]["ok"].as_bool().expect("terminal ok"),
                summary: fixture["terminalResult"]["summary"].as_str().expect("terminal summary").into(),
            }
            .encode(),
        )
        .expect("terminal result frame");
    let settled = shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs);
    assert!(chat_button_action(&settled, &control_id).is_none());
    let mut settled_lines = Vec::new();
    chat_panel_lines(&settled, &mut settled_lines);
    assert!(settled_lines.contains(&"Failed".to_string()) && settled_lines.iter().any(|line| line.contains("cancelled by user")), "the gateway result settles the row: {settled_lines:?}");

    let mut offline = welcomed_shell();
    offline.chrome.apply_agent_bridge_frame(&semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::GatewayToShell::AgentToolCall { invocation_id: invocation_id.into(), tool_name: tool_name.into(), arguments: arguments.into() }.encode()).expect("tool call frame");
    offline.chrome.chrome_build.agent.note_socket_closed();
    let offline_action = chat_button_action(&offline.directory.build_agent_chat_ui(&offline.chrome, &offline.prefs), &control_id).expect("React leaves a running row's control mounted while its sender can refuse");
    semio_framework_async::block_on(ActionsCtx::of(&mut offline).dispatch_action(offline_action)).expect("framework owns cancellation");
    assert!(offline.chrome.take_agent_bridge_outbox().is_empty(), "a disconnected cancel changes and sends nothing");
    assert!(matches!(&offline.chrome.chrome_build.agent.conversation[0], semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentConversationEntry::ToolCall { state: semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::AgentToolCallState::Running, .. }));
}

/// 💬️ The composer: a turn typed here leaves as a real `ShellToGateway::AgentMessage` and is echoed
/// into the same feed, a blank draft sends nothing, and with no socket open the draft is KEPT rather
/// than silently swallowed — React's `sendAgentMessage` contract, returning `false`.
#[test]
fn the_chat_composer_sends_one_agent_message_per_turn_and_keeps_an_unsendable_draft() {
    let mut offline = fresh_state();
    offline.directory.agent_chat_draft = "hello".into();
    assert!(!offline.directory.send_agent_chat_draft(&mut offline.chrome), "💬️ no socket, nothing sent");
    assert_eq!(offline.directory.agent_chat_draft, "hello", "💬️ and the human's text is not discarded");

    let mut shell = welcomed_shell();
    let _ = shell.chrome.take_agent_bridge_outbox();
    assert!(!shell.directory.send_agent_chat_draft(&mut shell.chrome), "💬️ a blank draft sends nothing");
    shell.directory.agent_chat_draft = "  move it  ".into();
    assert!(shell.directory.send_agent_chat_draft(&mut shell.chrome), "💬️ a connected shell sends");
    assert!(shell.directory.agent_chat_draft.is_empty(), "💬️ the draft clears on send");
    let outbox = shell.chrome.take_agent_bridge_outbox();
    assert_eq!(outbox.len(), 1, "💬️ exactly one frame per turn");
    match semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::ShellToGateway::decode(&outbox[0]).expect("the outbox frame decodes") {
        semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::ShellToGateway::AgentMessage { text, .. } => assert_eq!(text, "move it", "💬️ trimmed, React's own `trimmed`"),
        other => panic!("💬️ expected an AgentMessage, got {other:?}"),
    }
    let panel = shell.directory.build_agent_chat_ui(&shell.chrome, &shell.prefs);
    assert!(chat_entry_ids(&panel).iter().any(|id| id.starts_with("framework.chat.entry.userMessage.")), "💬️ the turn is echoed into the feed");
    let mut lines = Vec::new();
    chat_panel_lines(&panel, &mut lines);
    assert!(lines.contains(&"You".to_string()) && lines.contains(&"move it".to_string()), "💬️ under the translated role noun: {lines:?}");
    let UiNode::Stack(root) = &panel else { panic!("💬️ the chat panel root is a stack") };
    assert!(root.children.iter().any(|node| matches!(node, UiNode::Input(input) if input.id == "framework.chat.draft")), "💬️ the draft box is part of the published body");
    assert!(root.children.iter().any(|node| matches!(node, UiNode::Button(button) if button.id.as_deref() == Some("framework.chat.send"))), "💬️ so is Send");
    assert!(panel_ui_records(FRAMEWORK_CHAT_PANEL_ID, &panel).is_ok(), "💬️ and the whole body projects");
}
//#endregion 💬️AgentChatTranscript
