// 🧷️ Tests of the cli domain: hook, configure and micro-commit commands.

package cli

import (
	bytes "bytes"
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"

	hooks "github.com/usalu/semio/repo/hooks"
	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestTrackHookInOpenTicketUsesStableSessionIDs(t *testing.T) {
	tmpDir := t.TempDir()
	run := func(name string, field ...string) {
		cmd := exec.Command(name, field...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("run %s %v failed: %v\nOutput: %s", name, field, err, out)
		}
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}
	goal, err := ticketspkg.OpenGoal("Hook Goal", "Hook Goal Desc", "Hook Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", "", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}
	ticket, err := ticketspkg.OpenTicket("🪝️", "Hook Ticket", "Hook Ticket Prompt", "gemini-3-pro", "", "copilot-chat", "", false, goal.ID, "", true, "", providers.McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("expected 0 agents at open (no synthetic agent), got %d", len(ticket.Agents))
	}
	stableInput := json.RawMessage(`{"session_id":"agent-session-123","request_id":"req-1","second":"2026-02-23T00:00:00Z"}`)
	hooks.RunHook(model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-23T00:00:00Z",
		RepoRoot: tmpDir,
		Input:    stableInput,
	})
	ticketData, err := os.ReadFile(ticket.JsonPath)
	if err != nil {
		t.Fatalf("read ticket.json: %v", err)
	}
	var saved model.Ticket
	if err := json.Unmarshal(ticketData, &saved); err != nil {
		t.Fatalf("unmarshal ticket.json: %v", err)
	}
	if len(saved.Agents) != 0 {
		t.Fatalf("expected no persisted agents, got %d", len(saved.Agents))
	}
	if len(saved.Sessions) != len(ticket.Sessions) {
		t.Fatalf("hook should not change sessions length: before=%d after=%d", len(ticket.Sessions), len(saved.Sessions))
	}
	for i := range ticket.Sessions {
		if saved.Sessions[i] != ticket.Sessions[i] {
			t.Fatalf("hook should not change sessions: before=%+v after=%+v", ticket.Sessions, saved.Sessions)
		}
	}
	requestOnlyInput := json.RawMessage(`{"request_id":"req-2","second":"2026-02-23T00:00:01Z"}`)
	hooks.RunHook(model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T00:00:01Z",
		RepoRoot: tmpDir,
		ToolName: "Read",
		Input:    requestOnlyInput,
	})
	ticketData, err = os.ReadFile(ticket.JsonPath)
	if err != nil {
		t.Fatalf("read ticket.json after request-only hook: %v", err)
	}
	if err := json.Unmarshal(ticketData, &saved); err != nil {
		t.Fatalf("unmarshal ticket.json after request-only hook: %v", err)
	}
	if len(saved.Agents) != 0 {
		t.Fatalf("request-only hooks should not persist agents, got %d", len(saved.Agents))
	}
	if len(saved.Sessions) != len(ticket.Sessions) {
		t.Fatalf("request-only hook should not change sessions length: before=%d after=%d", len(ticket.Sessions), len(saved.Sessions))
	}
	for i := range ticket.Sessions {
		if saved.Sessions[i] != ticket.Sessions[i] {
			t.Fatalf("request-only hook should not change sessions: before=%+v after=%+v", ticket.Sessions, saved.Sessions)
		}
	}
}

func TestRunHookForRejectsGenericKind(t *testing.T) {
	err := RunHookFor(providers.McpClientGeneric, "stop", nil)
	if err == nil || !strings.Contains(strings.ToLower(err.Error()), "hooks are not available") {
		t.Fatalf("expected hooks-unavailable error for generic kind, got: %v", err)
	}
}

func TestHookCommandCLI(t *testing.T) {
	factory := func(cfg Config) (*Engine, error) {
		return nil, nil
	}
	config := &Config{Format: "json", Repo: t.TempDir()}
	cmd := hookCommand(factory, config)
	cases := []struct {
		name    string
		args    []string
		wantErr bool
	}{
		{"neutral agent starting", []string{"agent.started", "copilot-chat"}, false},
		{"neutral agent prompt submitting", []string{"agent.prompt.submitting", "cursor-chat"}, false},
		{"neutral agent tool terminal starting", []string{"agent.tool.terminal.starting", "windsurf-chat"}, false},
		{"native copilot SessionStart", []string{"SessionStart", "copilot-chat"}, false},
		{"native copilot PreToolUse", []string{"PreToolUse", "copilot-chat"}, false},
		{"native copilot PreCompact", []string{"PreCompact", "copilot-chat"}, false},
		{"native cursor sessionStart", []string{"sessionStart", "cursor-chat"}, false},
		{"native cursor beforeReadFile", []string{"beforeReadFile", "cursor-chat"}, false},
		{"native windsurf pre_user_prompt", []string{"pre_user_prompt", "windsurf-chat"}, false},
		{"native windsurf pre_read_code", []string{"pre_read_code", "windsurf-chat"}, false},
		{"native claude SessionStart", []string{"SessionStart", "claude-code"}, false},
		{"native claude PreToolUse", []string{"PreToolUse", "claude-code"}, false},
		{"native droid PreToolUse", []string{"PreToolUse", "droid"}, false},
		{"invalid event no client", []string{"invalid.event"}, true},
		{"invalid native no client", []string{"UnknownEvent", "copilot-chat"}, true},
		{"no args", []string{}, true},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			var buf bytes.Buffer
			cmd.SetOut(&buf)
			cmd.SetErr(&buf)
			cmd.SetIn(strings.NewReader(""))
			cmd.SetArgs(tc.args)
			err := cmd.Execute()
			if tc.wantErr && err == nil {
				t.Error("expected error")
			}
			if !tc.wantErr && err != nil {
				t.Errorf("unexpected error: %v", err)
			}
		})
	}
}

func TestConfigureCommandDoesNotGenerateConfigFiles(t *testing.T) {
	repoRoot := t.TempDir()
	hooksDir := filepath.Join(repoRoot, ".git", "hooks")
	hookSourcesDir := filepath.Join(repoRoot, "🧰️framework", "🛍️products", "🦑️repo", "🪝️hooks")
	if err := os.MkdirAll(hooksDir, 0o755); err != nil {
		t.Fatalf("mkdir hooks: %v", err)
	}
	if err := os.MkdirAll(hookSourcesDir, 0o755); err != nil {
		t.Fatalf("mkdir hook sources: %v", err)
	}
	for _, hookName := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		if err := os.WriteFile(filepath.Join(hookSourcesDir, hookName), []byte("#!/bin/sh\nexit 0\n"), 0o755); err != nil {
			t.Fatalf("write hook source %s: %v", hookName, err)
		}
	}
	for _, hookName := range []string{"pre-commit", "post-commit"} {
		hookPath := filepath.Join(hooksDir, hookName)
		if err := os.WriteFile(hookPath, []byte("#!/bin/sh\nexit 1\n"), 0o755); err != nil {
			t.Fatalf("write hook %s: %v", hookName, err)
		}
	}
	var out bytes.Buffer
	cmd := configureCommand(nil, &Config{Repo: repoRoot})
	cmd.SetOut(&out)
	cmd.SetArgs(nil)
	if err := cmd.Execute(); err != nil {
		t.Fatalf("configure should succeed: %v", err)
	}
	output := out.String()
	if !strings.Contains(output, "config generation is disabled") {
		t.Fatalf("expected no-operation message, got %q", output)
	}
	if !strings.Contains(output, "git hooks removed") {
		t.Fatalf("expected git hook removal message, got %q", output)
	}
	if !strings.Contains(output, "micro-commit hooks installed") {
		t.Fatalf("expected micro-commit hooks install message, got %q", output)
	}
	for _, hookName := range []string{"prepare-commit-msg", "post-commit", "post-checkout", "post-merge", "post-rewrite"} {
		hookPath := filepath.Join(hooksDir, hookName)
		if st, err := os.Stat(hookPath); err != nil || st.IsDir() {
			t.Fatalf("expected micro-commit hook at %s: %v", hookPath, err)
		}
	}
	if _, err := os.Stat(filepath.Join(hooksDir, "pre-commit")); err == nil {
		t.Fatal("expected legacy pre-commit hook removed")
	}
	for _, path := range []string{
		filepath.Join(repoRoot, ".github", "hooks", "repo.json"),
		filepath.Join(repoRoot, ".cursor", "hooks.json"),
		filepath.Join(repoRoot, ".claude", "settings.json"),
		filepath.Join(repoRoot, ".kiro", "agents", "repo.json"),
		filepath.Join(hooksDir, "pre-commit"),
	} {
		if _, err := os.Stat(path); err == nil || !os.IsNotExist(err) {
			t.Fatalf("configure unexpectedly left or created %s", path)
		}
	}
}

func testLoggingConfigFull() workspace.LoggingConfig {
	return workspace.LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "full"}
}

func writeRepoLoggingConfig(t *testing.T, root string, lg workspace.LoggingConfig) {
	t.Helper()
	repoDir := filepath.Join(root, ".🧬semio", "🦑️repo")
	if err := os.MkdirAll(repoDir, 0o755); err != nil {
		t.Fatalf("mkdir .🦑️repo: %v", err)
	}
	content := fmt.Sprintf("[logging]\nsession = %t\noperations = %t\nplan = %t\ndetail = \"%s\"\n",
		lg.Session, lg.Operations, lg.Plan, lg.Detail)
	if err := os.WriteFile(filepath.Join(repoDir, workspace.ConfigFileName), []byte(content), 0o644); err != nil {
		t.Fatalf("write %s: %v", workspace.ConfigFileName, err)
	}
}

func TestHookCommandStdinPiped(t *testing.T) {
	tmpDir := t.TempDir()
	payload := `{"session_id":"sess1","tool_name":"Read","tool_input":{"file_path":"/tmp/x"}}`
	cmd := exec.Command("./cli", "hook", "agent.tool.ended", "claude-code", "--tool-name", "Read")
	cmd.Stdin = strings.NewReader(payload)
	cmd.Env = append(os.Environ(), fmt.Sprintf("COMPOSE_REPO=%s", tmpDir))
	cmd.Dir = filepath.Dir(os.Args[0])
	out, err := cmd.CombinedOutput()
	_ = out
	_ = err
	now := time.Now().UTC()
	logDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache", "🤖️generated",
		fmt.Sprintf("%02d", now.Year()%100),
		fmt.Sprintf("%02d", int(now.Month())),
		fmt.Sprintf("%02d", now.Day()),
		"sess1")
	entries, readErr := os.ReadDir(logDir)
	if readErr != nil {
		t.Skip("cli binary not available for subprocess test")
	}
	if len(entries) == 0 {
		t.Fatal("expected at least one log file after piped hook invocation")
	}
	data, _ := os.ReadFile(filepath.Join(logDir, entries[0].Name()))
	if !strings.Contains(string(data), "sess1") {
		t.Errorf("expected stdin payload in log, got: %s", string(data))
	}
}

func TestNativeHookEventMappingWithRealData(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow native hook event mapping test in short mode")
	}
	cases := []struct {
		name        string
		nativeEvent string
		client      string
		toolName    string
		input       string
		expectEvent model.HookEvent
		expectPar   string
	}{
		{"copilot/SessionStart", "SessionStart", "copilot-chat", "", `{"hookEventName":"SessionStart","sessionId":"d765d480","second":"2026-02-19T10:44:08.112Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentStarted, ""},
		{"copilot/Stop", "Stop", "copilot-chat", "", `{"hookEventName":"Stop","sessionId":"2f1e87c2","second":"2026-02-18T18:51:54.315Z","stop_hook_active":false}`, model.HookAgentEnded, ""},
		{"copilot/SubagentStart", "SubagentStart", "copilot-chat", "", `{"hookEventName":"SubagentStart","sessionId":"ab58fc89","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentStarted, "subagent"},
		{"copilot/SubagentStop", "SubagentStop", "copilot-chat", "", `{"hookEventName":"SubagentStop","sessionId":"ab58fc89","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentEnded, "subagent"},
		{"copilot/UserPromptSubmit", "UserPromptSubmit", "copilot-chat", "", `{"hookEventName":"UserPromptSubmit","sessionId":"d765d480","second":"2026-02-19T10:44:16.328Z","transcript_path":"/tmp/t.jsonl","cwd":"/workspaces/semio"}`, model.HookAgentPromptSubmitting, ""},
		{"copilot/PreCompact", "PreCompact", "copilot-chat", "", `{"second":"2026-02-18T18:41:24.718Z","hookEventName":"PreCompact","sessionId":"2f1e87c2","transcript_path":"/tmp/t.jsonl","trigger":"auto","cwd":"/workspaces/semio"}`, model.HookAgentCompacting, ""},
		{"copilot/PreToolUse/read_file", "PreToolUse", "copilot-chat", "read_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"read_file","second":"2026-02-19T12:30:31.702Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/grep_search", "PreToolUse", "copilot-chat", "grep_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"grep_search","second":"2026-02-19T10:44:35.056Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/file_search", "PreToolUse", "copilot-chat", "file_search", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"file_search","second":"2026-02-19T10:50:09.443Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_dir", "PreToolUse", "copilot-chat", "list_dir", `{"second":"2026-02-19T10:44:16.328Z","hookEventName":"PreToolUse","sessionId":"d765d480","transcript_path":"/tmp/t.jsonl","tool_name":"list_dir","tool_input":{"path":"/workspaces/semio"}}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/list_code_usages", "PreToolUse", "copilot-chat", "list_code_usages", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"list_code_usages","second":"2026-02-19T11:31:35.416Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchStarting, ""},
		{"copilot/PreToolUse/replace_string_in_file", "PreToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"d765d480","hookEventName":"PreToolUse","tool_name":"replace_string_in_file","second":"2026-02-19T10:50:46.160Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/multi_replace_string_in_file", "PreToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:17.358Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/create_file", "PreToolUse", "copilot-chat", "create_file", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"create_file","second":"2026-02-19T12:25:00.000Z"}`, model.HookAgentToolCodeEditStarting, ""},
		{"copilot/PreToolUse/run_in_terminal", "PreToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"run_in_terminal","second":"2026-02-18T18:42:59.593Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolTerminalStarting, ""},
		{"copilot/PreToolUse/manage_todo_list", "PreToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PreToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:13.780Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolPlanUpdatingStarting, ""},
		{"copilot/PreToolUse/runSubagent", "PreToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PreToolUse","tool_name":"runSubagent","second":"2026-02-19T12:46:49.918Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolStarting, ""},
		{"copilot/PostToolUse/read_file", "PostToolUse", "copilot-chat", "read_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"read_file","second":"2026-02-18T18:38:51.951Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchEnded, ""},
		{"copilot/PostToolUse/replace_string_in_file", "PostToolUse", "copilot-chat", "replace_string_in_file", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"replace_string_in_file","second":"2026-02-18T18:37:05.471Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/multi_replace_string_in_file", "PostToolUse", "copilot-chat", "multi_replace_string_in_file", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"multi_replace_string_in_file","second":"2026-02-19T12:25:26.261Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolCodeEditEnded, ""},
		{"copilot/PostToolUse/run_in_terminal", "PostToolUse", "copilot-chat", "run_in_terminal", `{"sessionId":"d765d480","hookEventName":"PostToolUse","tool_name":"run_in_terminal","second":"2026-02-19T10:43:56.761Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolTerminalEnded, ""},
		{"copilot/PostToolUse/manage_todo_list", "PostToolUse", "copilot-chat", "manage_todo_list", `{"sessionId":"2f1e87c2","hookEventName":"PostToolUse","tool_name":"manage_todo_list","second":"2026-02-18T18:44:20.586Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolPlanUpdatingEnded, ""},
		{"copilot/PostToolUse/runSubagent", "PostToolUse", "copilot-chat", "runSubagent", `{"sessionId":"ab58fc89","hookEventName":"PostToolUse","tool_name":"runSubagent","second":"2026-02-19T12:48:58.829Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolEnded, ""},
		{"copilot/PostToolUse/grep_search", "PostToolUse", "copilot-chat", "grep_search", `{"sessionId":"8a40542e","hookEventName":"PostToolUse","tool_name":"grep_search","second":"2026-02-18T18:58:48.393Z","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchEnded, ""},
		{"cursor/sessionStart", "sessionStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:00Z"}`, model.HookAgentStarted, ""},
		{"cursor/sessionEnd", "sessionEnd", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, model.HookAgentEnded, ""},
		{"cursor/subagentStart", "subagentStart", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:01:00Z"}`, model.HookAgentStarted, "subagent"},
		{"cursor/subagentStop", "subagentStop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:29:00Z"}`, model.HookAgentEnded, "subagent"},
		{"cursor/stop", "stop", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:30:00Z"}`, model.HookAgentEnded, ""},
		{"cursor/beforeSubmitPrompt", "beforeSubmitPrompt", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:00:01Z","prompt":"Fix bug"}`, model.HookAgentPromptSubmitting, ""},
		{"cursor/preCompact", "preCompact", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, model.HookAgentCompacting, ""},
		{"cursor/preToolUse/read_file", "preToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:02:00Z","tool_name":"read_file"}`, model.HookAgentToolSearchStarting, ""},
		{"cursor/preToolUse/edit", "preToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:03:00Z","tool_name":"editfile"}`, model.HookAgentToolCodeEditStarting, ""},
		{"cursor/preToolUse/terminal", "preToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:04:00Z","tool_name":"terminal"}`, model.HookAgentToolTerminalStarting, ""},
		{"cursor/preToolUse/task", "preToolUse", "cursor-chat", "task", `{"sessionId":"cur-001","second":"2026-02-19T10:05:00Z","tool_name":"task"}`, model.HookAgentToolPlanUpdatingStarting, ""},
		{"cursor/postToolUse/read_file", "postToolUse", "cursor-chat", "read_file", `{"sessionId":"cur-001","second":"2026-02-19T10:06:00Z","tool_name":"read_file"}`, model.HookAgentToolSearchEnded, ""},
		{"cursor/postToolUse/editfile", "postToolUse", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:07:00Z","tool_name":"editfile"}`, model.HookAgentToolCodeEditEnded, ""},
		{"cursor/postToolUse/terminal", "postToolUse", "cursor-chat", "terminal", `{"sessionId":"cur-001","second":"2026-02-19T10:08:00Z","tool_name":"terminal"}`, model.HookAgentToolTerminalEnded, ""},
		{"cursor/postToolUseFailure/edit", "postToolUseFailure", "cursor-chat", "editfile", `{"sessionId":"cur-001","second":"2026-02-19T10:09:00Z","tool_name":"editfile"}`, model.HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeMCPExecution", "beforeMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:10:00Z"}`, model.HookAgentToolStarting, ""},
		{"cursor/afterMCPExecution", "afterMCPExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:11:00Z"}`, model.HookAgentToolEnded, ""},
		{"cursor/beforeReadFile", "beforeReadFile", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:12:00Z"}`, model.HookAgentToolSearchStarting, ""},
		{"cursor/afterFileEdit", "afterFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:13:00Z"}`, model.HookAgentToolCodeEditEnded, ""},
		{"cursor/beforeShellExecution", "beforeShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:14:00Z"}`, model.HookAgentToolTerminalStarting, ""},
		{"cursor/afterShellExecution", "afterShellExecution", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:15:00Z"}`, model.HookAgentToolTerminalEnded, ""},
		{"cursor/afterAgentResponse", "afterAgentResponse", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:16:00Z"}`, model.HookAgentEnded, ""},
		{"cursor/afterAgentThought", "afterAgentThought", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:17:00Z"}`, model.HookAgentThinkingEnded, ""},
		{"cursor/beforeTabFileRead", "beforeTabFileRead", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:18:00Z"}`, model.HookAgentToolSearchStarting, ""},
		{"cursor/afterTabFileEdit", "afterTabFileEdit", "cursor-chat", "", `{"sessionId":"cur-001","second":"2026-02-19T10:19:00Z"}`, model.HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_user_prompt", "pre_user_prompt", "windsurf-chat", "", `{"second":"2026-02-18T18:46:41.123Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentPromptSubmitting, ""},
		{"windsurf/post_cascade_response", "post_cascade_response", "windsurf-chat", "", `{"second":"2026-02-18T19:00:12.032Z","agent_action_name":"post_cascade_response","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentEnded, ""},
		{"windsurf/post_setup_worktree", "post_setup_worktree", "windsurf-chat", "", `{"second":"2026-02-18T18:45:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentStarted, ""},
		{"windsurf/pre_mcp_tool_use", "pre_mcp_tool_use", "windsurf-chat", "", `{"agent_action_name":"pre_mcp_tool_use","trajectory_id":"23e6dcf5","second":"2026-02-18T18:54:57.304Z","execution_id":"d9b64466","tool_info":{"mcp_server_name":"repo","mcp_tool_name":"tree"}}`, model.HookAgentToolStarting, ""},
		{"windsurf/post_mcp_tool_use", "post_mcp_tool_use", "windsurf-chat", "", `{"second":"2026-02-18T18:55:28.469Z","agent_action_name":"post_mcp_tool_use","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolEnded, ""},
		{"windsurf/pre_read_code", "pre_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:48.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolSearchStarting, ""},
		{"windsurf/post_read_code", "post_read_code", "windsurf-chat", "", `{"second":"2026-02-18T18:46:50.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolSearchEnded, ""},
		{"windsurf/pre_write_code", "pre_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:30.780Z","agent_action_name":"pre_write_code","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolCodeEditStarting, ""},
		{"windsurf/post_write_code", "post_write_code", "windsurf-chat", "", `{"second":"2026-02-18T18:57:35.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolCodeEditEnded, ""},
		{"windsurf/pre_run_command", "pre_run_command", "windsurf-chat", "", `{"second":"2026-02-18T18:54:00.000Z","trajectory_id":"23e6dcf5","execution_id":"d9b64466"}`, model.HookAgentToolTerminalStarting, ""},
		{"windsurf/post_run_command", "post_run_command", "windsurf-chat", "", `{"agent_action_name":"post_run_command","trajectory_id":"23e6dcf5","second":"2026-02-18T18:57:49.375Z","execution_id":"d9b64466","tool_info":{"command_line":"npm install","cwd":"/workspaces/semio"}}`, model.HookAgentToolTerminalEnded, ""},
		{"claude/SessionStart", "SessionStart", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionStart","source":"startup"}`, model.HookAgentStarted, ""},
		{"claude/SessionEnd", "SessionEnd", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","hook_event_name":"SessionEnd"}`, model.HookAgentEnded, ""},
		{"claude/SubagentStart", "SubagentStart", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStart"}`, model.HookAgentStarted, "subagent"},
		{"claude/SubagentStop", "SubagentStop", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"SubagentStop"}`, model.HookAgentEnded, "subagent"},
		{"claude/Stop", "Stop", "claude-code", "", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"Stop","stop_hook_active":false}`, model.HookAgentEnded, ""},
		{"claude/UserPromptSubmit", "UserPromptSubmit", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"UserPromptSubmit","prompt":"Fix the bug"}`, model.HookAgentPromptSubmitting, ""},
		{"claude/PreCompact", "PreCompact", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreCompact"}`, model.HookAgentCompacting, ""},
		{"claude/TaskCompleted", "TaskCompleted", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TaskCompleted"}`, model.HookAgentToolPlanUpdatingEnded, ""},
		{"claude/Notification", "Notification", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"Notification"}`, model.HookAgentToolStarting, ""},
		{"claude/TeammateIdle", "TeammateIdle", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"TeammateIdle"}`, model.HookAgentToolStarting, ""},
		{"claude/PermissionRequest", "PermissionRequest", "claude-code", "", `{"session_id":"167906cd","transcript_path":"/tmp/t.jsonl","hook_event_name":"PermissionRequest"}`, model.HookAgentToolStarting, ""},
		{"claude/PreToolUse/Bash", "PreToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"go test -v"},"tool_use_id":"toolu_01WKqqc5y27LZu1KTB5GsGDu"}`, model.HookAgentToolTestStarting, ""},
		{"claude/PreToolUse/Read", "PreToolUse", "claude-code", "Read", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","cwd":"/workspaces/semio","permission_mode":"bypassPermissions","hook_event_name":"PreToolUse","tool_name":"Read","tool_input":{"file_path":"/workspaces/semio/main.go"},"tool_use_id":"toolu_01Md976UFvJmmL5KaH4xzsx8"}`, model.HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/Edit", "PreToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl","tool_name":"Edit"}`, model.HookAgentToolCodeEditStarting, ""},
		{"claude/PreToolUse/Glob", "PreToolUse", "claude-code", "Glob", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"Glob","tool_input":{"pattern":"**/*.json"}}`, model.HookAgentToolStarting, ""},
		{"claude/PreToolUse/Grep", "PreToolUse", "claude-code", "Grep", `{"session_id":"e51a2976","transcript_path":"/tmp/t.jsonl","hook_event_name":"PreToolUse","tool_name":"Grep","tool_input":{"pattern":"BlockedToolPatterns"}}`, model.HookAgentToolSearchStarting, ""},
		{"claude/PreToolUse/mcp_tree", "PreToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl","hook_event_name":"PreToolUse","tool_name":"mcp__repo__tree","tool_input":{"query":"hooks"}}`, model.HookAgentToolStarting, ""},
		{"claude/PostToolUse/Bash", "PostToolUse", "claude-code", "Bash", `{"session_id":"167906cd-0550-4387-96af-2cc20cb48fe3","tool_name":"Bash","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/167906cd.jsonl"}`, model.HookAgentToolTerminalEnded, ""},
		{"claude/PostToolUse/Edit", "PostToolUse", "claude-code", "Edit", `{"session_id":"e51a2976-3fee-42db-a5cf-7b2f0a4c5b84","tool_name":"Edit","transcript_path":"/home/vscode/.claude/technologies/-workspaces-compose/e51a2976.jsonl"}`, model.HookAgentToolCodeEditEnded, ""},
		{"claude/PostToolUse/Read", "PostToolUse", "claude-code", "Read", `{"session_id":"167906cd","tool_name":"Read","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolSearchEnded, ""},
		{"claude/PostToolUse/TodoWrite", "PostToolUse", "claude-code", "TodoWrite", `{"session_id":"167906cd","tool_name":"TodoWrite","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolPlanUpdatingEnded, ""},
		{"claude/PostToolUse/mcp_tree", "PostToolUse", "claude-code", "mcp__repo__tree", `{"session_id":"167906cd","tool_name":"mcp__repo__tree","transcript_path":"/tmp/t.jsonl"}`, model.HookAgentToolEnded, ""},
		{"claude/PostToolUseFailure/Bash", "PostToolUseFailure", "claude-code", "Bash", `{"session_id":"167906cd","tool_name":"Bash","transcript_path":"/tmp/t.jsonl","error":"command failed"}`, model.HookAgentToolTerminalEnded, ""},
		{"droid/SessionStart", "SessionStart", "droid", "", `{"session_id":"droid-001","second":"2026-02-18T18:47:06.000Z"}`, model.HookAgentStarted, ""},
		{"droid/PreToolUse/Bash", "PreToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:06.000Z"}`, model.HookAgentToolTerminalStarting, ""},
		{"droid/PostToolUse/Bash", "PostToolUse", "droid", "Bash", `{"session_id":"droid-001","tool_name":"Bash","second":"2026-02-18T18:47:10.000Z"}`, model.HookAgentToolTerminalEnded, ""},
		{"codex/SessionStart", "SessionStart", "codex", "", `{"session_id":"codex-001","second":"2026-02-18T18:50:00.000Z"}`, model.HookAgentStarted, ""},
		{"codex/PreToolUse/Read", "PreToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:05.000Z"}`, model.HookAgentToolSearchStarting, ""},
		{"codex/PostToolUse/Read", "PostToolUse", "codex", "Read", `{"session_id":"codex-001","tool_name":"Read","second":"2026-02-18T18:50:10.000Z"}`, model.HookAgentToolSearchEnded, ""},
		{"antigravity/SessionStart", "SessionStart", "antigravity-chat", "", `{"session_id":"ag-001","second":"2026-02-18T18:55:00.000Z"}`, model.HookAgentStarted, ""},
		{"antigravity/PreToolUse/Task", "PreToolUse", "antigravity-chat", "Task", `{"session_id":"ag-001","tool_name":"Task","second":"2026-02-18T18:55:05.000Z"}`, model.HookAgentToolPlanUpdatingStarting, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			input := json.RawMessage(tc.input)
			event, parent, err := hooks.ResolveHookEvent(tc.nativeEvent, tc.client, tc.toolName, input)
			if err != nil {
				t.Fatalf("ResolveHookEvent error: %v", err)
			}
			if event != tc.expectEvent {
				t.Errorf("event: want %s, got %s", tc.expectEvent, event)
			}
			if parent != tc.expectPar {
				t.Errorf("parent: want %q, got %q", tc.expectPar, parent)
			}
			tmpDir := t.TempDir()
			writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
			inputSecond := extractSecondFromInput(string(input))
			var secondStr string
			if inputSecond == 0 {
				secondStr = time.Now().UTC().Format(time.RFC3339)
			} else {
				secondStr = fmt.Sprintf("%d", inputSecond)
			}
			hctx := model.HookContext{
				Event:      event,
				Client:     tc.client,
				Second:     secondStr,
				RepoRoot:   tmpDir,
				ToolName:   tc.toolName,
				Input:      input,
				ParentInfo: parent,
			}
			hooks.RunHook(hctx)
			sessionID := hooks.ExtractSessionIDFromInput(input)
			if sessionID == "" {
				sessionID = "unknown"
			}
			logNow := time.Now().UTC()
			logDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache", "🤖️generated",
				model.FormatYearDir(logNow.Year()%100),
				model.FormatMonthDir(int(logNow.Month())),
				model.FormatDayDir(logNow.Day()),
				sessionID)
			entries, err := os.ReadDir(logDir)
			if err != nil {
				t.Fatalf("log dir: %v", err)
			}
			if len(entries) != 1 || entries[0].Name() != "session.json" {
				names := make([]string, len(entries))
				for i, e := range entries {
					names[i] = e.Name()
				}
				t.Fatalf("want only session.json in session dir, got %v", names)
			}
			data, err := os.ReadFile(filepath.Join(logDir, "session.json"))
			if err != nil {
				t.Fatalf("read session.json: %v", err)
			}
			var meta model.SessionMeta
			if err := json.Unmarshal(data, &meta); err != nil {
				t.Fatalf("invalid session JSON: %v", err)
			}
			if len(meta.Events) == 0 {
				t.Fatalf("expected at least one event in session.json")
			}
			var entry *model.HookLogEntry
			for i := range meta.Events {
				var probe map[string]interface{}
				if err := json.Unmarshal(meta.Events[i].Event, &probe); err == nil && probe["kind"] == string(event) {
					entry = &meta.Events[i]
					break
				}
			}
			if entry == nil {
				t.Fatalf("could not find event kind %s in session.json events (%d total)", event, len(meta.Events))
			}
			var evt map[string]interface{}
			if err := json.Unmarshal(entry.Event, &evt); err != nil {
				t.Fatalf("cannot unmarshal event: %v", err)
			}
			if evt["kind"] != string(event) {
				t.Errorf("log kind: want %s, got %v", event, evt["kind"])
			}
			if evt["client"] != tc.client {
				t.Errorf("log client: want %s, got %v", tc.client, evt["client"])
			}
			if entry.Native.Event == nil {
				t.Error("log native.event: want non-nil")
			}
			wantSession := hooks.ResolveEventSessionID(hooks.ExtractSessionIDFromInput(input))
			if wantSession != "" && evt["session"] != wantSession {
				t.Errorf("log session: want %s, got %v", wantSession, evt["session"])
			}
			wantSecond := hooks.ResolveEventSecondID(secondStr)
			if wantSecond != "" && evt["second"] != wantSecond {
				t.Errorf("log second: want %s, got %v", wantSecond, evt["second"])
			}
			wantTranscript := hooks.ExtractTranscriptFromInput(input)
			if wantTranscript != "" && evt["transcript"] != wantTranscript {
				t.Errorf("log transcript: want %s, got %v", wantTranscript, evt["transcript"])
			}
			if entry.Response.Blocked != nil {
				t.Error("log response.blocked: want nil for non-blocked event")
			}
		})
	}
}

func TestMicroCommitCommandExists(t *testing.T) {
	root, _ := NewRootWithConfig(testEngineFactory)
	var found *Command
	for _, c := range root.Commands() {
		if c.Name() == "micro-commit" {
			found = c
			break
		}
	}
	if found == nil {
		t.Fatal("root command tree missing micro-commit")
	}
	if found.Use != "micro-commit [subcommand] [args...]" {
		t.Fatalf("unexpected micro-commit use: %q", found.Use)
	}
}
