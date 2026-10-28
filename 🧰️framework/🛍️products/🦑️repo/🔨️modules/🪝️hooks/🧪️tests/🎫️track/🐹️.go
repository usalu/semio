// 🎫️ Tests of the hook tracking inside an open ticket.

package hooks

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func setupTicketDir(t *testing.T) (string, string) {
	t.Helper()
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	ticketDir := filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets",
		model.FormatYearDir(now.Year()%100),
		model.FormatMonthDir(int(now.Month())),
		model.FormatDayDir(now.Day()),
		"TEST-TICKET")
	if err := os.MkdirAll(ticketDir, 0755); err != nil {
		t.Fatal(err)
	}
	ticketJSON := filepath.Join(ticketDir, "🎫️ticket.json")
	initialTicket := `{"title":"Test Ticket","status":"open","goal":"TEST/GOAL"}`
	if err := os.WriteFile(ticketJSON, []byte(initialTicket), 0644); err != nil {
		t.Fatal(err)
	}
	importantPath := filepath.Join(ticketDir, "📌️important", "📝️.md")
	if err := os.MkdirAll(filepath.Dir(importantPath), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(importantPath, nil, 0644); err != nil {
		t.Fatal(err)
	}
	return tmpDir, ticketJSON
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
		t.Fatalf("expected no hook log files under .🦑️repo/⚡️, got %v", logFiles)
	}
}

func TestTrackHookAllEventsLogged(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow track hook all-events test in short mode")
	}
	tmpDir, ticketJSON := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	sandbox.BindRoot(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"test-session-1","llm":"opus-4-6","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
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
		event model.HookEvent
		input json.RawMessage
	}{
		{"agent.started", model.HookAgentStarted, sessionInput},
		{"agent.tool.starting", model.HookAgentToolStarting, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.file.read.starting", model.HookAgentToolSearchStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"hello","includePattern":"src/**"}}`)},
		{"agent.file.read.ended", model.HookAgentToolSearchEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"query":"world"}}`)},
		{"agent.tool.code.edit.starting", model.HookAgentToolCodeEditStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.code.edit.ended", model.HookAgentToolCodeEditEnded, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"filePath":"/tmp/test.go","oldString":"old","newString":"new"}}`)},
		{"agent.tool.terminal.starting", model.HookAgentToolTerminalStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"command":"go test ./..."}}`)},
		{"agent.tool.terminal.ended", model.HookAgentToolTerminalEnded, json.RawMessage(`{"session_id":"test-session-1","command":"go test ./...","pid":"12345","stdout":"PASS"}`)},
		{"agent.tool.test.starting", model.HookAgentToolTestStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"files":["/tmp/test.go"],"timeout":"30000"}}`)},
		{"agent.tool.test.ended", model.HookAgentToolTestEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["TestA"],"failed":["TestB"]}}`)},
		{"agent.tool.build.starting", model.HookAgentToolBuildStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"bundles":["compose/js"]}}`)},
		{"agent.tool.build.ended", model.HookAgentToolBuildEnded, json.RawMessage(`{"session_id":"test-session-1","tool_output":{"succeeded":["compose/js"]}}`)},
		{"agent.tool.ended", model.HookAgentToolEnded, json.RawMessage(`{"session_id":"test-session-1","tool_name":"grep_search"}`)},
		{"agent.tool.plan.updating.starting", model.HookAgentToolPlanUpdatingStarting, json.RawMessage(`{"session_id":"test-session-1","tool_input":{"todoList":[{"title":"Step 1","status":"completed"},{"title":"Step 2","status":"in-progress"}]}}`)},
		{"agent.ended", model.HookAgentEnded, sessionInput},
		{"agent.thinking.starting", model.HookAgentThinkingStarting, json.RawMessage(`{"session_id":"test-session-1","text":"Planning the approach"}`)},
		{"agent.thinking.ended", model.HookAgentThinkingEnded, json.RawMessage(`{"session_id":"test-session-1","text":"Decided to use X"}`)},
	}
	for _, ae := range agentEvents {
		hctx := model.HookContext{
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

	var meta model.SessionMeta
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
	requireAgentSession(t, ticket, "test-session-1", "")
}

func requireAgentSession(t *testing.T, ticket map[string]interface{}, session string, transcript string) {
	t.Helper()
	agents, _ := ticket["agents"].([]interface{})
	if len(agents) != 1 {
		t.Fatalf("expected the durable agents member to hold exactly the hook session, got %+v", ticket["agents"])
	}
	agent, _ := agents[0].(map[string]interface{})
	if agent["session"] != session {
		t.Fatalf("expected agent session %q, got %+v", session, agent)
	}
	if transcript != "" && agent["transcript"] != transcript {
		t.Fatalf("expected agent transcript %q, got %+v", transcript, agent)
	}
}

func TestTrackHookSearchingPattern(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	sandbox.BindRoot(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"search-session"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Search test",
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
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
	sandbox.BindRoot(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"blocked-session"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Do something",
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolStarting,
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
			var meta model.SessionMeta
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
	sandbox.BindRoot(t, tmpDir)
	sessionInput := json.RawMessage(`{"session_id":"terminal-session"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    sessionInput,
		ToolArgs: "Run command",
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"terminal-session","tool_input":{"command":"npm test"}}`),
	})
	RunHook(model.HookContext{
		Event:    model.HookAgentToolTerminalEnded,
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
			var meta model.SessionMeta
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
	sandbox.BindRoot(t, tmpDir)
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
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
	requireAgentSession(t, ticket, "transcript-session", "/home/user/.vscode/transcripts/abc.jsonl")
}

func TestTrackHookCodeEditedLogsToFile(t *testing.T) {
	tmpDir, _ := setupTicketDir(t)
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	sandbox.BindRoot(t, tmpDir)
	tsContent := "// #region \U0001F516Functions\n\n// doWork MUST work.\nexport function doWork(): void {}\n\n// #endregion \U0001F516Functions\n"
	tsFile := filepath.Join(tmpDir, "proj", "kit", "file.ts")
	if err := os.MkdirAll(filepath.Dir(tsFile), 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(tsFile, []byte(tsContent), 0644); err != nil {
		t.Fatal(err)
	}
	RunHook(model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-23T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    json.RawMessage(`{"session_id":"def-id-session"}`),
		ToolArgs: "Edit file",
	})
	payload := fmt.Sprintf(`{"session_id":"def-id-session","tool_input":{"filePath":%q,"oldString":"","newString":"export function doWork(): void {}"}}`, tsFile)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolCodeEditEnded,
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
			var meta model.SessionMeta
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
