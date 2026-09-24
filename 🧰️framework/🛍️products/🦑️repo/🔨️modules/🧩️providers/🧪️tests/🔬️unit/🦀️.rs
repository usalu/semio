use super::*;

fn recorded(exchanges: Vec<ProcessExchange>) -> ProcessRunners {
    ProcessRunners::from(RecordedProcessRunner::new(ProcessTranscript { exchanges }))
}

#[test]
fn recorded_runner_replays_by_argv_and_logs_every_call() {
    let runner = RecordedProcessRunner::new(ProcessTranscript {
        exchanges: vec![ProcessExchange { argv: vec!["gh".into(), "api".into(), "user".into(), "--jq".into(), ".login".into()], stdin: None, stdout: "usalu\n".into(), stderr: String::new(), status: 0 }],
    });
    let outcome = runner.run(&ProcessRequest::new("gh", &["api".into(), "user".into(), "--jq".into(), ".login".into()], ""));
    assert_eq!(outcome.stdout, "usalu\n");
    assert_eq!(runner.run(&ProcessRequest::new("gh", &["nope".into()], "")).status, 127);
    assert_eq!(runner.issued().len(), 2);
}

#[test]
fn github_provider_issues_the_documented_argv_and_parses_the_issue() {
    let provider = GitHubManagementProvider::new(recorded(vec![ProcessExchange {
        argv: vec!["gh".into(), "issue".into(), "view".into(), "https://github.com/usalu/semio/issues/7".into(), "--json".into(), "url,state,milestone,labels,title,body".into()],
        stdin: None,
        stdout: r#"{"url":"https://github.com/usalu/semio/issues/7","state":"OPEN","title":"T","body":"B","milestone":{"number":3,"title":"M"},"labels":[{"name":"ticket"}]}"#.into(),
        stderr: String::new(),
        status: 0,
    }]));
    let issue = provider.get_issue_details("https://github.com/usalu/semio/issues/7").unwrap().unwrap();
    assert_eq!(issue.state, "OPEN");
    assert_eq!(issue.milestone.unwrap().number, 3);
    assert_eq!(issue.labels[0].name, "ticket");
    assert_eq!(provider.issued()[0][1], "issue");
}

#[test]
fn missing_milestone_leaves_create_issue_without_the_flag() {
    let provider = GitHubManagementProvider::new(recorded(vec![ProcessExchange {
        argv: vec!["gh".into(), "issue".into(), "create".into(), "--title".into(), "T".into(), "--body".into(), "B".into(), "--label".into(), "ticket".into()],
        stdin: None,
        stdout: "https://github.com/usalu/semio/issues/9\n".into(),
        stderr: String::new(),
        status: 0,
    }]));
    assert_eq!(provider.create_issue("T", "B", Some(4)).unwrap(), "https://github.com/usalu/semio/issues/9");
}

#[test]
fn extract_issue_url_reads_both_bare_and_embedded_forms() {
    assert_eq!(extract_issue_url("https://github.com/usalu/semio/issues/1\n"), "https://github.com/usalu/semio/issues/1");
    assert_eq!(extract_issue_url("Creating issue https://github.com/usalu/semio/issues/2 done"), "https://github.com/usalu/semio/issues/2");
    assert_eq!(extract_issue_url("no url here"), "");
}

#[test]
fn git_provider_reads_the_branch_and_the_staged_files() {
    let provider = GitVersionControlProvider::new(recorded(vec![
        ProcessExchange { argv: vec!["git".into(), "rev-parse".into(), "--abbrev-ref".into(), "HEAD".into()], stdin: None, stdout: "main\n".into(), stderr: String::new(), status: 0 },
        ProcessExchange { argv: vec!["git".into(), "diff".into(), "--cached".into(), "--name-only".into()], stdin: None, stdout: "a.txt\r\nb.txt\n".into(), stderr: String::new(), status: 0 },
    ]));
    assert_eq!(provider.current_branch("/repo").unwrap(), "main");
    assert_eq!(provider.staged_files("/repo").unwrap(), vec!["a.txt".to_string(), "b.txt".to_string()]);
}

#[test]
fn archive_branch_is_zero_padded() {
    assert_eq!(archive_branch_name("ueli", (2026, 9, 6)), "ueli/2026/09/06");
    assert_eq!(civil_from_days(0), (1970, 1, 1));
}

#[test]
fn copilot_wraps_the_permission_decision_and_others_do_not() {
    let denied = HookResult::new(false, "blocked: git checkout");
    let copilot = CopilotEditorProvider.format_hook_output("PreToolUse", &denied);
    let parsed: Value = serde_json::from_str(&copilot).unwrap();
    assert_eq!(parsed["hookSpecificOutput"]["permissionDecision"], "deny");
    assert_eq!(parsed["hookSpecificOutput"]["permissionDecisionReason"], "blocked: git checkout");
    let cursor: Value = serde_json::from_str(&CursorEditorProvider.format_hook_output("PreToolUse", &denied)).unwrap();
    assert!(cursor.get("hookSpecificOutput").is_none());
}

#[test]
fn every_editor_resolves_and_names_events() {
    assert_eq!(all_editor_providers().len(), 8);
    assert_eq!(get_editor_provider("kiro-cli").unwrap().kind(), "kiro-cli");
    assert!(get_editor_provider("nope").is_none());
    assert_eq!(CopilotEditorProvider.resolve_native_event("SubagentStart", ToolKind::Generic).unwrap(), (HookEvent::AgentStarted, "subagent".to_string()));
    assert_eq!(CopilotEditorProvider.native_event_from_hook_event(HookEvent::AgentStarted, "subagent"), "SubagentStart");
    assert_eq!(CursorEditorProvider.native_event_from_hook_event(HookEvent::AgentStarted, "subagent"), "");
    assert!(KiroEditorProvider.resolve_native_event("SessionStart", ToolKind::Generic).is_err());
}

#[test]
fn shell_events_fall_back_to_terminal_only_for_generic_tools() {
    assert_eq!(resolve_shell_pre_tool_use(ToolKind::Generic), HookEvent::AgentToolTerminalStarting);
    assert_eq!(resolve_shell_pre_tool_use(ToolKind::Test), HookEvent::AgentToolTestStarting);
    assert_eq!(resolve_shell_post_tool_use(ToolKind::Generic), HookEvent::AgentToolTerminalEnded);
}

#[test]
fn mcp_client_kinds_round_trip_through_their_slugs() {
    assert_eq!(parse_mcp_client_kind("  CODEX ").unwrap(), McpClientKind::Codex);
    assert_eq!(parse_mcp_client_kind("client").unwrap(), McpClientKind::Generic);
    assert!(parse_mcp_client_kind("unknown").is_err());
    assert_eq!(mcp_server_name(McpClientKind::Kiro), "repo");
    assert_eq!(hook_client_for_mcp_kind(McpClientKind::Claude), "claude-code");
    assert_eq!(mcp_kind_from_resolved_client("cursor-chat"), McpClientKind::Cursor);
}

#[test]
fn the_null_provider_never_reaches_a_process() {
    let provider = NullManagementProvider;
    assert_eq!(provider.kind(), "none");
    assert_eq!(provider.create_issue("a", "b", None).unwrap(), "");
    assert!(provider.get_issue_details("x").unwrap().is_none());
}

#[test]
fn ports_are_reachable_from_the_provider_implementations() {
    let formatter: &dyn HookFormatter = &EditorProviders::from(CursorEditorProvider);
    assert_eq!(formatter.native_event_from_hook_event(HookEvent::AgentEnded, ""), "");
    assert_eq!(formatter.format_hook_output("PreToolUse", &HookResult::new(true, "")), "{\"allowed\":true}");
    let tracker = default_management_provider(recorded(Vec::new()));
    assert!(IssueTracker::list_repo_labels(&tracker).is_err());
    assert!(IssueTracker::find_milestone_by_title(&tracker, "  ").unwrap().is_none());
}
