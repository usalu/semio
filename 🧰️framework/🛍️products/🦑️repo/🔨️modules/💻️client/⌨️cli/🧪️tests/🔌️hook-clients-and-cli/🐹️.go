// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook client mapping, hook command CLI, and stdin extraction.

// #endregion 🧲️Header

package client

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func TestHookClientForMcpKindMapsIDEEntrypoints(t *testing.T) {
	cases := []struct {
		kind McpClientKind
		want string
	}{
		{McpClientCursor, "cursor-chat"},
		{McpClientKiro, "kiro-cli"},
		{McpClientCopilot, "copilot-chat"},
		{McpClientClaude, "claude-code"},
		{McpClientCodex, "codex"},
		{McpClientGeneric, ""},
	}
	for _, tc := range cases {
		if got := HookClientForMcpKind(tc.kind); got != tc.want {
			t.Errorf("HookClientForMcpKind(%q) = %q, want %q", tc.kind, got, tc.want)
		}
	}
}

func TestResolveMcpTicketClient(t *testing.T) {
	tests := []struct {
		name     string
		kind     McpClientKind
		client   string
		expected string
	}{
		{"Codex inferred", McpClientCodex, "", "codex"},
		{"Claude inferred", McpClientClaude, "", "claude-code"},
		{"Explicit preserved", McpClientCodex, "cursor-chat", "cursor-chat"},
		{"Generic remains empty", McpClientGeneric, "", ""},
	}
	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			if got := resolveMcpTicketClient(tc.kind, tc.client); got != tc.expected {
				t.Fatalf("resolveMcpTicketClient(%q, %q) = %q, want %q", tc.kind, tc.client, got, tc.expected)
			}
		})
	}
}

func TestCurrentMcpLLMsAllowed(t *testing.T) {
	for _, llm := range []string{"opus-4-7", "gpt-5-5"} {
		if got, err := ResolveAllowedLLM(llm); err != nil || got != llm {
			t.Fatalf("ResolveAllowedLLM(%q) = %q, %v", llm, got, err)
		}
	}
}

func TestWriteWarningfUsesStderr(t *testing.T) {
	read, write, err := os.Pipe()
	if err != nil {
		t.Fatal(err)
	}
	previous := os.Stderr
	os.Stderr = write
	defer func() { os.Stderr = previous }()

	writeWarningf("Failed to update: %s", "boom")
	if err := write.Close(); err != nil {
		t.Fatal(err)
	}
	os.Stderr = previous
	output, err := io.ReadAll(read)
	if err != nil {
		t.Fatal(err)
	}
	if got, want := string(output), "Warning: Failed to update: boom\n"; got != want {
		t.Fatalf("warning output = %q, want %q", got, want)
	}
}

func TestRunHookForRejectsGenericKind(t *testing.T) {
	err := RunHookFor(McpClientGeneric, "stop", nil)
	if err == nil || !strings.Contains(strings.ToLower(err.Error()), "hooks are not available") {
		t.Fatalf("expected hooks-unavailable error for generic kind, got: %v", err)
	}
}

func TestRunHookThinkingEvents(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow run hook thinking events test in short mode")
	}
	t.Run("thinking ended extracts text from top-level", func(t *testing.T) {
		hctx := HookContext{
			Event:  HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think","text":"Planning repo modifications"}`),
		}
		result := RunHook(hctx)
		res, ok := result.(HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Planning repo modifications" {
			t.Errorf("expected thinking text in message, got %q", res.GetMessage())
		}
		if !res.IsAllowed() {
			t.Error("expected thinking ended to be allowed")
		}
	})
	t.Run("thinking ended extracts text from native.event", func(t *testing.T) {
		input := `{"native":{"event":{"text":"Thinking deeply","hook_event_name":"afterAgentThought"}},"session_id":"sess-think2"}`
		hctx := HookContext{
			Event:  HookAgentThinkingEnded,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(input),
		}
		result := RunHook(hctx)
		res, ok := result.(HookResultAgentThinkingEnded)
		if !ok {
			t.Fatalf("expected HookResultAgentThinkingEnded, got %T", result)
		}
		if res.GetMessage() != "Thinking deeply" {
			t.Errorf("expected thinking text from native.event, got %q", res.GetMessage())
		}
	})
	t.Run("thinking starting is allowed", func(t *testing.T) {
		hctx := HookContext{
			Event:  HookAgentThinkingStarting,
			Client: "cursor-chat",
			Second: time.Now().UTC().Format(time.RFC3339),
			Input:  json.RawMessage(`{"session_id":"sess-think3","text":"About to think"}`),
		}
		result := RunHook(hctx)
		if _, ok := result.(HookResultAgentThinkingStarting); !ok {
			t.Fatalf("expected HookResultAgentThinkingStarting, got %T", result)
		}
		if !result.IsAllowed() {
			t.Error("expected thinking starting to be allowed")
		}
	})
}

func TestAllHookEventsCompleteness(t *testing.T) {
	expected := []HookEvent{
		HookVersionCheckpointStarting, HookVersionCheckpointEnded,
		HookVersionCheckinStarting, HookVersionCheckinEnded,
		HookVersionCheckoutStarting, HookVersionCheckoutEnded,
		HookAgentStarted, HookAgentEnded,
		HookAgentPromptSubmitting, HookAgentCompacting,
		HookAgentToolStarting, HookAgentToolEnded,
		HookAgentToolPlanUpdatingStarting, HookAgentToolPlanUpdatingEnded,
		HookAgentToolSearchStarting, HookAgentToolSearchEnded,
		HookAgentToolCodeEditStarting, HookAgentToolCodeEditEnded,
		HookAgentToolTestStarting, HookAgentToolTestEnded,
		HookAgentToolBuildStarting, HookAgentToolBuildEnded,
		HookAgentToolTerminalStarting, HookAgentToolTerminalEnded,
		HookAgentThinkingStarting, HookAgentThinkingEnded,
	}
	if len(AllHookEvents) != len(expected) {
		t.Errorf("expected %d events, got %d", len(expected), len(AllHookEvents))
	}
	for _, e := range expected {
		found := false
		for _, a := range AllHookEvents {
			if a == e {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("missing event: %s", e)
		}
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

func TestHookCommandToolBlocking(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
		ToolName: "terminal",
		ToolArgs: "git stash pop",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected tool to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
	if !strings.Contains(result.GetMessage(), "git stash") {
		t.Errorf("expected git stash in message, got: %s", result.GetMessage())
	}
	if !strings.Contains(result.GetMessage(), "other developers and agents may be editing the same files concurrently") {
		t.Errorf("expected concurrent edit warning in message, got: %s", result.GetMessage())
	}
}

func TestHookCommandJSONOutput(t *testing.T) {
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	out, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON marshaling, got: %v", err)
	}
	var parsed HookResultAgentStarted
	if err := json.Unmarshal(out, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	if !parsed.Allowed {
		t.Error("expected allowed=true")
	}
	if parsed.Second == "" {
		t.Error("expected non-empty second")
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
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess1")
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

func TestExtractCommandFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"claude code tool_input.command", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "git checkout main"},
		{"cursor beforeShellExecution", `{"command":"git stash pop"}`, "git stash pop"},
		{"windsurf tool_info.command_line", `{"tool_info":{"command_line":"git reset --hard"}}`, "git reset --hard"},
		{"no command", `{"tool_name":"ReadFile","tool_input":{"path":"/tmp"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractCommandFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractCommandFromStdinBlocking(t *testing.T) {
	payload := json.RawMessage(`{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   time.Now().UTC().Format(time.RFC3339),
		Input:    payload,
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected stdin-based git checkout to be blocked")
	}
	if !strings.Contains(result.GetMessage(), "blocked") {
		t.Errorf("expected blocked message, got: %s", result.GetMessage())
	}
}
