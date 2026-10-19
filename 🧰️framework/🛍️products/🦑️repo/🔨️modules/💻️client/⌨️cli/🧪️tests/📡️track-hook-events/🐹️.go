// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Hook tracking, session IDs, and logged events.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

func TestExtractSessionIDFromInput(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"empty", ``, ""},
		{"invalid json", `{invalid}`, ""},
		{"trajectory_id", `{"trajectory_id": "test-1"}`, "test-1"},
		{"trajectoryId", `{"trajectoryId": "test-2"}`, "test-2"},
		{"sessionId", `{"sessionId": "test-3"}`, "test-3"},
		{"session_id", `{"session_id": "test-4"}`, "test-4"},
		{"conversationId", `{"conversationId": "test-5"}`, "test-5"},
		{"conversation_id", `{"conversation_id": "test-6"}`, "test-6"},
		{"agent_id", `{"agent_id": "test-7"}`, "test-7"},
		{"agentId", `{"agentId": "test-8"}`, "test-8"},
		{"nested conversation id", `{"native":{"event":{"conversation_id":"test-9"}}}`, "test-9"},
		{"transcript basename fallback", `{"native":{"event":{"transcript_path":"/tmp/transcripts/test-10.jsonl"}}}`, "test-10"},
		{"whitespace", `{"sessionId": " test-9 "}`, "test-9"},
		{"missing", `{"other": "value"}`, ""},
		{"wrong type", `{"sessionId": 123}`, ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := extractSessionIDFromInput(json.RawMessage(tt.input))
			if result != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, result)
			}
		})
	}
}

func TestExtractTranscriptAndLLMFromNestedInput(t *testing.T) {
	input := json.RawMessage(`{
		"native": {
			"event": {
				"model": "composer-1.5",
				"transcript_path": "/home/vscode/.cursor/projects/workspaces-compose/agent-transcripts/session-1/session-1.jsonl"
			}
		}
	}`)

	if got := extractTranscriptFromInput(input); got != "/home/vscode/.cursor/projects/workspaces-compose/agent-transcripts/session-1/session-1.jsonl" {
		t.Fatalf("expected nested transcript path, got %q", got)
	}

	if got := extractLLMFromInput(input); got != "composer-1.5" {
		t.Fatalf("expected nested model, got %q", got)
	}
}

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
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}
	goal, err := OpenGoal("Hook Goal", "Hook Goal Desc", "Hook Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", "", true)
	if err != nil {
		t.Fatalf("OpenGoal failed: %v", err)
	}
	ticket, err := OpenTicket("🪝️", "Hook Ticket", "Hook Ticket Prompt", "gemini-3-pro", "", "copilot-chat", "", false, goal.ID, "", true, "", McpClientGeneric, "", "")
	if err != nil {
		t.Fatalf("OpenTicket failed: %v", err)
	}
	if len(ticket.Agents) != 0 {
		t.Fatalf("expected 0 agents at open (no synthetic agent), got %d", len(ticket.Agents))
	}
	stableInput := json.RawMessage(`{"session_id":"agent-session-123","request_id":"req-1","second":"2026-02-23T00:00:00Z"}`)
	RunHook(HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-23T00:00:00Z",
		RepoRoot: tmpDir,
		Input:    stableInput,
	})
	ticketData, err := os.ReadFile(ticket.JsonPath)
	if err != nil {
		t.Fatalf("read ticket.json: %v", err)
	}
	var saved Ticket
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
	RunHook(HookContext{
		Event:    HookAgentToolStarting,
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

func readTicketJSON(t *testing.T, ticketJSON string) map[string]interface{} {
	t.Helper()
	data, err := os.ReadFile(ticketJSON)
	if err != nil {
		t.Fatalf("cannot read ticket.json: %v", err)
	}
	var result map[string]interface{}
	if err := json.Unmarshal(data, &result); err != nil {
		t.Fatalf("invalid ticket.json: %v\n%s", err, string(data))
	}
	return result
}

func getLogFiles(t *testing.T, tmpDir string) []string {
	t.Helper()
	outBase := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "⚡️cache")
	var logFiles []string
	filepath.WalkDir(outBase, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return nil
		}
		if !d.IsDir() && strings.HasSuffix(d.Name(), ".json") {
			logFiles = append(logFiles, path)
		}
		return nil
	})
	return logFiles
}

func assertNoHookLogFiles(t *testing.T, tmpDir string) {
	t.Helper()
	logFiles := getLogFiles(t, tmpDir)
	if len(logFiles) != 0 {
		t.Fatalf("expected no hook log files under .🧬semio/🦑️repo/⚡️cache, got %v", logFiles)
	}
}

func TestTrackHookAllEventsLogged(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow track hook all-events test in short mode")
	}
	tmpDir, ticketJSON := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	bindRootForTest(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"test-session-1","llm":"opus-4-6","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Fix the bug",
	}
	RunHook(hctx)
	ticket := readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	agentEvents := []struct {
		name  string
		event HookEvent
		input json.RawMessage
	}{
		{"agent.started", HookAgentStarted, sessionInput},
		{"agent.tool.starting", HookAgentToolStarting, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.file.read.starting", HookAgentToolSearchStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"hello","includePattern":"src/**"}}`)},
		{"agent.file.read.ended", HookAgentToolSearchEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"world"}}`)},
		{"agent.tool.code.edit.starting", HookAgentToolCodeEditStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.code.edit.ended", HookAgentToolCodeEditEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.terminal.starting", HookAgentToolTerminalStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"command":"go test ./..."}}`)},
		{"agent.tool.terminal.ended", HookAgentToolTerminalEnded, json.RawMessage(`{"session_id":"test-session-1","command":"go test ./...","pid":"12345","stdout":"PASS"}`)},
		{"agent.tool.test.starting", HookAgentToolTestStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"files":["/tmp/test.go"],"timeout":"30000"}}`)},
		{"agent.tool.test.ended", HookAgentToolTestEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["TestA"],"failed":["TestB"]}}`)},
		{"agent.tool.build.starting", HookAgentToolBuildStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"bundles":["compose/js"]}}`)},
		{"agent.tool.build.ended", HookAgentToolBuildEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["compose/js"]}}`)},
		{"agent.tool.ended", HookAgentToolEnded, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.tool.plan.updating.starting", HookAgentToolPlanUpdatingStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"todoList":[{"title":"Step 1","status":"completed"},{"title":"Step 2","status":"in-progress"}]}}`)},
		{"agent.ended", HookAgentEnded, sessionInput},
		{"agent.thinking.starting", HookAgentThinkingStarting, json.RawMessage(`{"session_id":"test-session-1","text":"Planning the approach"}`)},
		{"agent.thinking.ended", HookAgentThinkingEnded, json.RawMessage(`{"session_id":"test-session-1","text":"Decided to use X"}`)},
	}
	for _, ae := range agentEvents {
		hctx := HookContext{
			Event:    ae.event,
			Client:   "copilot-chat",
			Second:   "2026-02-23T10:01:00Z",
			RepoRoot: tmpDir,
			Input:    ae.input,
		}
		RunHook(hctx)
	}

	logFiles := getLogFiles(t, tmpDir)

	// After removing redundant files, only session.json should exist
	// All events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for all events, got %d", sessionCount)
	}

	// 🪪️Verify that all 16 events are in the session.json file
	var sessionFile string
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionFile = f
			break
		}
	}

	data, err := os.ReadFile(sessionFile)
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}

	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("cannot unmarshal session.json: %v", err)
	}

	// Should have 18 events (1 prompt + 15 agent events + 2 derived events from search operations)
	expectedEventCount := 18
	if len(meta.Events) != expectedEventCount {
		t.Fatalf("expected %d events in session.json, got %d", expectedEventCount, len(meta.Events))
	}

	ticket = readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	if _, hasAgents := ticket["agents"]; hasAgents {
		t.Fatal("agents must not be persisted in ticket.json")
	}
}

func TestTrackHookSearchingPattern(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	bindRootForTest(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"search-session"}`)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Search test",
	})
	RunHook(HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"search-session","tool_input":{"query":"findMe","includePattern":"src/**/*.ts"}}`),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for search-session, got %d", sessionCount)
	}

	// Check that session.json contains the search event
	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			if strings.Contains(string(data), "findMe") {
				found = true
				break
			}
		}
	}
	if !found {
		t.Error("expected search query in session.json")
	}
}

func TestTrackHookBlockedEvent(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	bindRootForTest(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"blocked-session"}`)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Do something",
	})
	RunHook(HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"blocked-session","tool_input":{"command":"git checkout main"}}`),
		ToolName: "run_in_terminal",
		ToolArgs: "git checkout main",
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for blocked-session, got %d", sessionCount)
	}
	// Check that session.json contains the blocked event
	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					if entry.Response != nil && entry.Response.Blocked != nil && *entry.Response.Blocked {
						found = true
						break
					}
				}
			}
		}
	}
	if !found {
		t.Error("expected blocked event in session.json")
	}
}

func TestTrackHookTerminalEvents(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	bindRootForTest(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"terminal-session"}`)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Run command",
	})
	RunHook(HookContext{
		Event:    HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"terminal-session","tool_input":{"command":"npm test"}}`),
	})
	RunHook(HookContext{
		Event:    HookAgentToolTerminalEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:02:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"terminal-session","command":"npm test","pid":"999","stdout":"all passed"}`),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// All events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for terminal-session, got %d", sessionCount)
	}
	// Check that session.json contains terminal events
	startFound := false
	endFound := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					var evt map[string]interface{}
					if err := json.Unmarshal(entry.Event, &evt); err == nil {
						if kind, ok := evt["kind"].(string); ok {
							if kind == "agent.tool.terminal.starting" {
								startFound = true
							} else if kind == "agent.tool.terminal.ended" {
								endFound = true
							}
						}
					}
				}
			}
		}
	}
	if !startFound {
		t.Error("expected agent-tool-terminal-starting event in session.json")
	}
	if !endFound {
		t.Error("expected agent-tool-terminal-ended event in session.json")
	}
}

func TestTrackHookTranscriptInSession(t *testing.T) {
	tmpDir, ticketJSON := setupTicketDir(t)
	bindRootForTest(t, tmpDir)
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"transcript-session","transcript_path":"/home/user/.vscode/transcripts/abc.jsonl"}`),
		ToolArgs: "test",
	})
	ticket := readTicketJSON(t, ticketJSON)
	if sessions, ok := ticket["sessions"].([]interface{}); ok && len(sessions) > 0 {
		t.Fatalf("expected hooks to not persist sessions, got %+v", sessions)
	}
	if _, hasAgents := ticket["agents"]; hasAgents {
		t.Error("agents must not be persisted in ticket.json")
	}
}

func TestTrackHookCodeEditedLogsToFile(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	bindRootForTest(t, tmpDir)
	tsContent := "// #region \U0001F516Functions\n\n// doWork MUST work.\nexport function doWork(): void {}\n\n// #endregion \U0001F516Functions\n"
	tsFile := filepath.Join(tmpDir, "proj", "kit", "file.ts")
	if err := os.MkdirAll(filepath.Dir(tsFile), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(tsFile, []byte(tsContent), 0644); err != nil {
		t.Fatal(err)
	}
	RunHook(HookContext{
		Event:    HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"def-id-session"}`),
		ToolArgs: "Edit file",
	})
	payload := fmt.Sprintf(`{"session_id":"def-id-session","tool_input":{"filePath":%q,"oldString":"","newString":"export function doWork(): void {}"}}`, tsFile)
	RunHook(HookContext{
		Event:    HookAgentToolCodeEditEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(payload),
	})

	logFiles := getLogFiles(t, tmpDir)
	// After removing redundant files, only session.json should exist
	// Both events use the same session_id, so they should be in one session.json file
	sessionCount := 0
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			sessionCount++
		}
	}
	if sessionCount != 1 {
		t.Fatalf("expected exactly 1 session.json file for def-id-session, got %d", sessionCount)
	}

	found := false
	for _, f := range logFiles {
		if strings.HasSuffix(f, "session.json") {
			data, err := os.ReadFile(f)
			if err != nil {
				t.Fatalf("cannot read session.json: %v", err)
			}
			var meta SessionMeta
			if err := json.Unmarshal(data, &meta); err == nil {
				for _, entry := range meta.Events {
					var evt map[string]interface{}
					if err := json.Unmarshal(entry.Event, &evt); err == nil {
						if evt["kind"] == "agent.tool.code.edit.ended" {
							found = true
							break
						}
					}
				}
			}
		}
	}
	if !found {
		t.Error("expected agent-tool-code-edit-ended event in session.json")
	}
}
