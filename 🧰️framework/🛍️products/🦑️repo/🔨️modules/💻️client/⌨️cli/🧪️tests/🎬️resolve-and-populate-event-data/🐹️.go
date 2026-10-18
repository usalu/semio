// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook event resolution and event data population for agent and tool events.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"testing"
)

func TestResolveHookEvent(t *testing.T) {
	cases := []struct {
		name      string
		eventStr  string
		client    string
		toolName  string
		expectEvt HookEvent
		expectPar string
		expectErr bool
	}{
		{"neutral agent.started", "agent.started", "copilot-chat", "", HookAgentStarted, "", false},
		{"neutral agent.tool.code.edit.starting", "agent.tool.code.edit.starting", "copilot-chat", "", HookAgentToolCodeEditStarting, "", false},
		{"copilot SessionStart", "SessionStart", "copilot-chat", "", HookAgentStarted, "", false},
		{"copilot Stop", "Stop", "copilot-chat", "", HookAgentEnded, "", false},
		{"copilot SubagentStart", "SubagentStart", "copilot-chat", "", HookAgentStarted, "subagent", false},
		{"copilot SubagentStop", "SubagentStop", "copilot-chat", "", HookAgentEnded, "subagent", false},
		{"copilot UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", HookAgentPromptSubmitting, "", false},
		{"copilot PreCompact", "PreCompact", "copilot-chat", "", HookAgentCompacting, "", false},
		{"copilot PreToolUse generic", "PreToolUse", "copilot-chat", "runSubagent", HookAgentToolStarting, "", false},
		{"copilot PreToolUse read_file", "PreToolUse", "copilot-chat", "read_file", HookAgentToolSearchStarting, "", false},
		{"copilot PreToolUse create_file", "PreToolUse", "copilot-chat", "create_file", HookAgentToolCodeEditStarting, "", false},
		{"copilot PreToolUse run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", HookAgentToolTerminalStarting, "", false},
		{"copilot PreToolUse manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", HookAgentToolPlanUpdatingStarting, "", false},
		{"copilot PostToolUse read_file", "PostToolUse", "copilot-chat", "read_file", HookAgentToolSearchEnded, "", false},
		{"copilot PostToolUse create_file", "PostToolUse", "copilot-chat", "create_file", HookAgentToolCodeEditEnded, "", false},
		{"copilot PostToolUse run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", HookAgentToolTerminalEnded, "", false},
		{"copilot PostToolUse generic", "PostToolUse", "copilot-chat", "runSubagent", HookAgentToolEnded, "", false},
		{"cursor sessionStart", "sessionStart", "cursor-chat", "", HookAgentStarted, "", false},
		{"cursor sessionEnd", "sessionEnd", "cursor-chat", "", HookAgentEnded, "", false},
		{"cursor subagentStart", "subagentStart", "cursor-chat", "", HookAgentStarted, "subagent", false},
		{"cursor beforeReadFile", "beforeReadFile", "cursor-chat", "", HookAgentToolSearchStarting, "", false},
		{"cursor afterFileEdit", "afterFileEdit", "cursor-chat", "", HookAgentToolCodeEditEnded, "", false},
		{"cursor beforeShellExecution", "beforeShellExecution", "cursor-chat", "", HookAgentToolTerminalStarting, "", false},
		{"cursor afterShellExecution", "afterShellExecution", "cursor-chat", "", HookAgentToolTerminalEnded, "", false},
		{"cursor beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", HookAgentToolStarting, "", false},
		{"cursor afterMCPExecution", "afterMCPExecution", "cursor-chat", "", HookAgentToolEnded, "", false},
		{"cursor afterAgentResponse", "afterAgentResponse", "cursor-chat", "", HookAgentEnded, "", false},
		{"cursor afterAgentThought", "afterAgentThought", "cursor-chat", "", HookAgentThinkingEnded, "", false},
		{"cursor beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", HookAgentToolSearchStarting, "", false},
		{"cursor afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", HookAgentToolCodeEditEnded, "", false},
		{"windsurf pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", HookAgentPromptSubmitting, "", false},
		{"windsurf post_cascade_response", "post_cascade_response", "windsurf-chat", "", HookAgentEnded, "", false},
		{"windsurf post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", HookAgentStarted, "", false},
		{"windsurf pre_read_code", "pre_read_code", "windsurf-chat", "", HookAgentToolSearchStarting, "", false},
		{"windsurf pre_write_code", "pre_write_code", "windsurf-chat", "", HookAgentToolCodeEditStarting, "", false},
		{"windsurf post_write_code", "post_write_code", "windsurf-chat", "", HookAgentToolCodeEditEnded, "", false},
		{"windsurf pre_run_command", "pre_run_command", "windsurf-chat", "", HookAgentToolTerminalStarting, "", false},
		{"windsurf post_run_command", "post_run_command", "windsurf-chat", "", HookAgentToolTerminalEnded, "", false},
		{"windsurf pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", HookAgentToolStarting, "", false},
		{"windsurf post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", HookAgentToolEnded, "", false},
		{"claude SessionStart", "SessionStart", "claude-code", "", HookAgentStarted, "", false},
		{"claude SessionEnd", "SessionEnd", "claude-code", "", HookAgentEnded, "", false},
		{"claude SubagentStart", "SubagentStart", "claude-code", "", HookAgentStarted, "subagent", false},
		{"claude SubagentStop", "SubagentStop", "claude-code", "", HookAgentEnded, "subagent", false},
		{"claude TaskCompleted", "TaskCompleted", "claude-code", "", HookAgentToolPlanUpdatingEnded, "", false},
		{"claude PermissionRequest", "PermissionRequest", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude TeammateIdle", "TeammateIdle", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude Notification", "Notification", "claude-code", "", HookAgentToolStarting, "", false},
		{"claude PreToolUse Bash", "PreToolUse", "claude-code", "Bash", HookAgentToolTerminalStarting, "", false},
		{"claude PostToolUse Bash", "PostToolUse", "claude-code", "Bash", HookAgentToolTerminalEnded, "", false},
		{"claude PreToolUse Read", "PreToolUse", "claude-code", "Read", HookAgentToolSearchStarting, "", false},
		{"claude PreToolUse Glob", "PreToolUse", "claude-code", "Glob", HookAgentToolSearchStarting, "", false},
		{"claude PostToolUse Glob", "PostToolUse", "claude-code", "Glob", HookAgentToolSearchEnded, "", false},
		{"claude PreToolUse Edit", "PreToolUse", "claude-code", "Edit", HookAgentToolCodeEditStarting, "", false},
		{"claude PostToolUse Edit", "PostToolUse", "claude-code", "Edit", HookAgentToolCodeEditEnded, "", false},
		{"droid PreToolUse", "PreToolUse", "droid", "Bash", HookAgentToolTerminalStarting, "", false},
		{"codex PreToolUse", "PreToolUse", "codex", "Read", HookAgentToolSearchStarting, "", false},
		{"antigravity PreToolUse", "PreToolUse", "antigravity-chat", "Task", HookAgentToolPlanUpdatingStarting, "", false},
		{"unknown client defaults to claude-compatible", "SessionStart", "unknown-client", "", HookAgentStarted, "", false},
		{"kiro agentSpawn", "agentSpawn", "kiro-cli", "", HookAgentStarted, "", false},
		{"kiro userPromptSubmit", "userPromptSubmit", "kiro-cli", "", HookAgentPromptSubmitting, "", false},
		{"kiro preToolUse fs_read", "preToolUse", "kiro-cli", "fs_read", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse fs_write", "preToolUse", "kiro-cli", "fs_write", HookAgentToolCodeEditStarting, "", false},
		{"kiro preToolUse execute_bash", "preToolUse", "kiro-cli", "execute_bash", HookAgentToolTerminalStarting, "", false},
		{"kiro preToolUse code", "preToolUse", "kiro-cli", "code", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse grep", "preToolUse", "kiro-cli", "grep", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse glob", "preToolUse", "kiro-cli", "glob", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse web_search", "preToolUse", "kiro-cli", "web_search", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse web_fetch", "preToolUse", "kiro-cli", "web_fetch", HookAgentToolSearchStarting, "", false},
		{"kiro preToolUse use_subagent", "preToolUse", "kiro-cli", "use_subagent", HookAgentToolStarting, "", false},
		{"kiro postToolUse fs_write", "postToolUse", "kiro-cli", "fs_write", HookAgentToolCodeEditEnded, "", false},
		{"kiro postToolUse execute_bash", "postToolUse", "kiro-cli", "execute_bash", HookAgentToolTerminalEnded, "", false},
		{"kiro stop", "stop", "kiro-cli", "", HookAgentEnded, "", false},
		{"kiro invalid event", "UnknownEvent", "kiro-cli", "", "", "", true},
		{"invalid copilot event", "UnknownEvent", "copilot-chat", "", "", "", true},
		{"invalid cursor event", "UnknownEvent", "cursor-chat", "", "", "", true},
		{"invalid windsurf event", "UnknownEvent", "windsurf-chat", "", "", "", true},
		{"invalid claude event", "UnknownEvent", "claude-code", "", "", "", true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			event, parent, err := ResolveHookEvent(tc.eventStr, tc.client, tc.toolName, nil)
			if tc.expectErr {
				if err == nil {
					t.Errorf("expected error, got event=%s parent=%s", event, parent)
				}
				return
			}
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if event != tc.expectEvt {
				t.Errorf("expected event %s, got %s", tc.expectEvt, event)
			}
			if parent != tc.expectPar {
				t.Errorf("expected parent %q, got %q", tc.expectPar, parent)
			}
		})
	}
}

func TestResolvePreToolUse(t *testing.T) {
	cases := []struct {
		kind   ToolKind
		expect HookEvent
	}{
		{ToolKindPlan, HookAgentToolPlanUpdatingStarting},
		{ToolKindCodeSearch, HookAgentToolSearchStarting},
		{ToolKindCodeEdit, HookAgentToolCodeEditStarting},
		{ToolKindTest, HookAgentToolTestStarting},
		{ToolKindBuild, HookAgentToolBuildStarting},
		{ToolKindTerminal, HookAgentToolTerminalStarting},
		{ToolKindGeneric, HookAgentToolStarting},
	}
	for _, tc := range cases {
		t.Run(string(tc.kind), func(t *testing.T) {
			result := resolvePreToolUse(tc.kind)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestResolvePostToolUse(t *testing.T) {
	cases := []struct {
		kind   ToolKind
		expect HookEvent
	}{
		{ToolKindCodeSearch, HookAgentToolSearchEnded},
		{ToolKindCodeEdit, HookAgentToolCodeEditEnded},
		{ToolKindTest, HookAgentToolTestEnded},
		{ToolKindBuild, HookAgentToolBuildEnded},
		{ToolKindTerminal, HookAgentToolTerminalEnded},
		{ToolKindGeneric, HookAgentToolEnded},
		{ToolKindPlan, HookAgentToolPlanUpdatingEnded},
	}
	for _, tc := range cases {
		t.Run(string(tc.kind), func(t *testing.T) {
			result := resolvePostToolUse(tc.kind)
			if result != tc.expect {
				t.Errorf("expected %s, got %s", tc.expect, result)
			}
		})
	}
}

func TestPopulateEventDataAgentStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-abc","parent":"subagent"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T10:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Session != "sess-abc" {
		t.Errorf("expected session=sess-abc, got %s", res.Session)
	}
	if res.Second != "2026-02-19T10:00:00Z" {
		t.Errorf("expected second=2026-02-19T10:00:00Z, got %s", res.Second)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
	if res.Parent != "" {
		t.Errorf("expected empty parent, got %s", res.Parent)
	}
	if res.Raw == nil {
		t.Error("expected raw to be populated")
	}
}

func TestPopulateEventDataAgentStartingSubagentParentFromContextUsesSession(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-sub"}`)
	hctx := HookContext{
		Event:      HookAgentStarted,
		Client:     "codex",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "sess-sub" {
		t.Errorf("expected parent=sess-sub, got %s", res.Parent)
	}
}

func TestPopulateEventDataAgentStartingSubagentUsesAgentIDAsSession(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-parent","agent_id":"agent-child"}`)
	hctx := HookContext{
		Event:      HookAgentStarted,
		Client:     "copilot-chat",
		Second:     "2026-03-06T19:54:55.558Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Session != "agent-child" {
		t.Errorf("expected session=agent-child, got %s", res.Session)
	}
	if res.Parent != "sess-parent" {
		t.Errorf("expected parent=sess-parent, got %s", res.Parent)
	}
}

func TestPopulateEventDataAgentStartingParentFromContext(t *testing.T) {
	hctx := HookContext{
		Event:      HookAgentStarted,
		Client:     "claude-code",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		ParentInfo: "parent-agent",
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "parent-agent" {
		t.Errorf("expected parent=parent-agent, got %s", res.Parent)
	}
	if res.Client != "claude-code" {
		t.Errorf("expected client=claude-code, got %s", res.Client)
	}
}

func TestPopulateEventDataAgentEnded(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-end","tool_info":{"response":"final report text"}}`)
	hctx := HookContext{
		Event:    HookAgentEnded,
		Client:   "cursor-chat",
		Second:   "2026-02-19T11:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentEnded, got %T", result)
	}
	if res.Session != "sess-end" {
		t.Errorf("expected session=sess-end, got %s", res.Session)
	}
	if res.Second != "2026-02-19T11:00:00Z" {
		t.Errorf("expected second, got %s", res.Second)
	}
	if res.Client != "cursor-chat" {
		t.Errorf("expected client=cursor-chat, got %s", res.Client)
	}
	if res.Report != "final report text" {
		t.Errorf("expected report to be extracted, got %q", res.Report)
	}
}

func TestPopulateEventDataPromptSubmitting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-p","prompt":"Fix the bug in main.go"}`)
	hctx := HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T12:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentPromptSubmitting)
	if !ok {
		t.Fatalf("expected HookResultAgentPromptSubmitting, got %T", result)
	}
	if res.Session != "sess-p" {
		t.Errorf("expected session=sess-p, got %s", res.Session)
	}
	if res.Prompt != "Fix the bug in main.go" {
		t.Errorf("expected prompt, got %s", res.Prompt)
	}
	if res.Client != "copilot-chat" {
		t.Errorf("expected client=copilot-chat, got %s", res.Client)
	}
}

func TestPopulateEventDataCompacting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-c","chat":"previous conversation context"}`)
	hctx := HookContext{
		Event:    HookAgentCompacting,
		Client:   "claude-code",
		Second:   "2026-02-19T13:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentCompacting)
	if !ok {
		t.Fatalf("expected HookResultAgentCompacting, got %T", result)
	}
	if res.Session != "sess-c" {
		t.Errorf("expected session=sess-c, got %s", res.Session)
	}
	if res.Chat != "previous conversation context" {
		t.Errorf("expected chat content, got %s", res.Chat)
	}
}

func TestPopulateEventDataToolStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-t","tool_name":"runSubagent","tool_input":{"prompt":"do something"}}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:00:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Session != "sess-t" {
		t.Errorf("expected session=sess-t, got %s", res.Session)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	var inputData map[string]interface{}
	if err := json.Unmarshal(res.Input, &inputData); err != nil {
		t.Fatalf("expected valid JSON input, got: %v", err)
	}
	if inputData["prompt"] != "do something" {
		t.Errorf("expected prompt in input, got %v", inputData["prompt"])
	}
}

func TestPopulateEventDataToolEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-te","tool_name":"runSubagent","tool_input":{"prompt":"do something"},"tool_output":"done"}`)
	hctx := HookContext{
		Event:    HookAgentToolEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:30:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolEnded, got %T", result)
	}
	if res.Name != "runSubagent" {
		t.Errorf("expected name=runSubagent, got %s", res.Name)
	}
	if res.Input == nil {
		t.Error("expected input to be populated")
	}
	if res.Response == nil {
		t.Error("expected response to be populated")
	}
}

func TestPopulateEventDataPlanUpdating(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-plan","tool_input":{"todoList":[{"id":1,"title":"Step 1","status":"completed"},{"id":2,"title":"Step 2","status":"in-progress"},{"id":3,"title":"Step 3","status":"not-started"}]}}`)
	hctx := HookContext{
		Event:    HookAgentToolPlanUpdatingStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T15:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolPlanUpdating)
	if !ok {
		t.Fatalf("expected HookResultAgentToolPlanUpdating, got %T", result)
	}
	if res.Session != "sess-plan" {
		t.Errorf("expected session=sess-plan, got %s", res.Session)
	}
	if len(res.Steps) != 3 {
		t.Fatalf("expected 3 steps, got %d", len(res.Steps))
	}
	if res.Steps[0].Name != "Step 1" || res.Steps[0].Status != "completed" {
		t.Errorf("expected Step 1 completed, got %+v", res.Steps[0])
	}
	if res.Steps[1].Name != "Step 2" || res.Steps[1].Status != "in-progress" {
		t.Errorf("expected Step 2 in-progress, got %+v", res.Steps[1])
	}
	if res.Steps[2].Name != "Step 3" || res.Steps[2].Status != "not-started" {
		t.Errorf("expected Step 3 not-started, got %+v", res.Steps[2])
	}
}

func TestPopulateEventDataCodeSearching(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cs","tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T16:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if res.Session != "sess-cs" {
		t.Errorf("expected session=sess-cs, got %s", res.Session)
	}
	if len(res.Pages) != 0 {
		t.Errorf("expected no pages for non-web search input, got %v", res.Pages)
	}
}

func TestPopulateEventDataCodeEditing(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ce","tool_input":{"filePath":"/workspaces/semio/test.go","oldString":"old code","newString":"new code"}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T17:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditStarting, got %T", result)
	}
	if res.Session != "sess-ce" {
		t.Errorf("expected session=sess-ce, got %s", res.Session)
	}
	if res.Path != "/workspaces/semio/test.go" {
		t.Errorf("expected path, got %s", res.Path)
	}
	if res.Old != "old code" {
		t.Errorf("expected old=old code, got %s", res.Old)
	}
	if res.New != "new code" {
		t.Errorf("expected new=new code, got %s", res.New)
	}
}

func TestPopulateEventDataCodeEditingWithAll(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-cea","tool_input":{"filePath":"/tmp/file.ts","oldString":"x","newString":"y","all":true}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditStarting,
		Client:   "cursor-chat",
		Second:   "2026-02-19T17:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditStarting, got %T", result)
	}
	if !res.All {
		t.Error("expected all=true")
	}
}

func TestPopulateEventDataCodeEdited(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ced","tool_input":{"filePath":"/tmp/edited.ts","oldString":"before","newString":"after"}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T18:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditEnded, got %T", result)
	}
	if res.Path != "/tmp/edited.ts" {
		t.Errorf("expected path=/tmp/edited.ts, got %s", res.Path)
	}
	if res.Old != "before" {
		t.Errorf("expected old=before, got %s", res.Old)
	}
	if res.New != "after" {
		t.Errorf("expected new=after, got %s", res.New)
	}
}
