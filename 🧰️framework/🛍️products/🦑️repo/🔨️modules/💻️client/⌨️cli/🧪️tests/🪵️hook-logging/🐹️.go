// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Repo logging config and hook logging.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func testLoggingConfigFull() LoggingConfig {
	return LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "full"}
}

func testLoggingConfigSession() LoggingConfig {
	return LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "standard"}
}

func writeRepoLoggingConfig(t *testing.T, root string, lg LoggingConfig) {
	t.Helper()
	repoDir := filepath.Join(root, ".🧬semio", "🦑️repo")
	if err := os.MkdirAll(repoDir, 0o755); err != nil {
		t.Fatalf("mkdir repo metadata directory: %v", err)
	}
	content := fmt.Sprintf("[logging]\nsession = %t\noperations = %t\nplan = %t\ndetail = \"%s\"\n",
		lg.Session, lg.Operations, lg.Plan, lg.Detail)
	if err := os.WriteFile(filepath.Join(repoDir, "📋️config.toml"), []byte(content), 0o644); err != nil {
		t.Fatalf("write config.toml: %v", err)
	}
}

func TestRepoConfig(t *testing.T) {
	t.Run("defaults when file missing", func(t *testing.T) {
		cfg := LoadRepoConfig(t.TempDir())
		if cfg.Logging.Session {
			t.Error("expected session logging off by default")
		}
		if !cfg.Logging.Operations || !cfg.Logging.Plan {
			t.Error("expected operations and plan enabled by default")
		}
		if cfg.Logging.Detail != "standard" {
			t.Errorf("expected detail standard, got %q", cfg.Logging.Detail)
		}
	})
	t.Run("parse logging table", func(t *testing.T) {
		tmpDir := t.TempDir()
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{
			Session: true, Operations: false, Plan: false, Detail: "minimal",
		})
		cfg := LoadRepoConfig(tmpDir)
		if !cfg.Logging.Session || cfg.Logging.Operations || cfg.Logging.Plan {
			t.Errorf("unexpected logging config: %+v", cfg.Logging)
		}
		if cfg.Logging.Detail != "minimal" {
			t.Errorf("expected detail minimal, got %q", cfg.Logging.Detail)
		}
	})
	t.Run("session off by default", func(t *testing.T) {
		tmpDir := t.TempDir()
		RunHook(HookContext{
			Event:    HookAgentStarted,
			Client:   "claude-code",
			Second:   time.Now().UTC().Format(time.RFC3339),
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"off-by-default"}`),
		})
		assertNoHookLogFiles(t, tmpDir)
	})
	t.Run("detail levels", func(t *testing.T) {
		tmpDir := t.TempDir()
		logDir := sessionLogDir(tmpDir, 2026, 5, 30, "detail-sess")
		if err := os.MkdirAll(logDir, 0o755); err != nil {
			t.Fatal(err)
		}
		hctx := HookContext{Event: HookAgentStarted, Client: "claude-code", Input: json.RawMessage(`{"x":1}`)}
		result := HookResultBase{Allowed: true}
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{Session: true, Detail: "minimal"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", LoadRepoConfig(tmpDir).Logging)
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 event, got %d", len(meta.Events))
		}
		if meta.Events[0].Native != nil {
			t.Error("minimal must not include native")
		}
		if meta.Events[0].Response != nil {
			t.Error("minimal must not include response")
		}
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{Session: true, Detail: "full"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", LoadRepoConfig(tmpDir).Logging)
		data, _ = os.ReadFile(filepath.Join(logDir, "session.json"))
		json.Unmarshal(data, &meta)
		if len(meta.Events) < 2 {
			t.Fatalf("expected 2 events after full detail append")
		}
		last := meta.Events[len(meta.Events)-1]
		if last.Native == nil || last.Response == nil {
			t.Error("full detail must include native and response")
		}
	})
	t.Run("plan off", func(t *testing.T) {
		tmpDir := t.TempDir()
		writeRepoLoggingConfig(t, tmpDir, LoggingConfig{Session: true, Plan: false, Detail: "standard"})
		RunHook(HookContext{
			Event:    HookAgentToolPlanUpdatingEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"plan-off","tool_input":{"todoList":[{"title":"A","status":"pending"}]}}`),
		})
		logFiles := getLogFiles(t, tmpDir)
		if len(logFiles) != 1 {
			t.Fatalf("expected session.json, got %v", logFiles)
		}
		var meta SessionMeta
		data, _ := os.ReadFile(logFiles[0])
		json.Unmarshal(data, &meta)
		if meta.Plan != nil {
			t.Error("expected no plan when plan=false")
		}
	})
}

func TestHookLogging(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess-log")
	payload := json.RawMessage(`{"session_id":"sess-log","second":"2026-02-20T10:00:00Z","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "claude-code",
		Second:   "2026-02-20T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	if !result.IsAllowed() {
		t.Fatalf("expected allowed=true, got: %s", result.GetMessage())
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected only session.json for session events, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json to be the only file, got %s", entries[0].Name())
	}
	metaBytes, err := os.ReadFile(filepath.Join(logDir, "session.json"))
	if err != nil {
		t.Fatalf("expected session.json to exist, got error: %v", err)
	}
	var meta SessionMeta
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("expected valid JSON in session.json, got: %v", err)
	}
	if len(meta.Events) != 1 {
		t.Fatalf("expected exactly 1 session event, got %d", len(meta.Events))
	}
	if meta.ID == "" {
		t.Errorf("expected id in session.json, got: %v", meta.ID)
	}
	if meta.URI == "" {
		t.Errorf("expected uri in session.json, got: %v", meta.URI)
	}
	if meta.Contributor == "" {
		t.Errorf("expected contributor in session.json, got: %v", meta.Contributor)
	}
	if meta.Client != "claude-code" {
		t.Errorf("expected client claude-code in session.json, got: %v", meta.Client)
	}
	if meta.Second == "" {
		t.Errorf("expected second in session.json, got: %v", meta.Second)
	}
	if meta.Transcript != "/tmp/transcript.jsonl" {
		t.Errorf("expected transcript in session.json, got: %v", meta.Transcript)
	}
	entry := meta.Events[0]
	if entry.Native.Event == nil {
		t.Error("expected native.event to be populated")
	}
	var evt map[string]interface{}
	if err := json.Unmarshal(entry.Event, &evt); err != nil {
		t.Fatalf("cannot unmarshal event: %v", err)
	}
	if evt["kind"] != string(HookAgentStarted) {
		t.Errorf("expected event.kind agent.started in log, got: %v", evt["kind"])
	}
	if evt["client"] != "claude-code" {
		t.Errorf("expected event.client claude-code in log, got: %v", evt["client"])
	}
	if _, ok := evt["contributor"]; !ok {
		t.Error("expected event.contributor to be present in log entry")
	}
	expectedSession := resolveEventSessionID("sess-log")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s in log, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := resolveEventSecondID("2026-02-20T10:00:00Z")
	if evt["second"] != expectedSecond {
		t.Errorf("expected event.second %s from input, got: %v", expectedSecond, evt["second"])
	}
	if evt["transcript"] != "/tmp/transcript.jsonl" {
		t.Errorf("expected event.transcript in log, got: %v", evt["transcript"])
	}
	if entry.Response.Blocked != nil {
		t.Error("expected response.blocked to be nil for allowed event")
	}
	if len(meta.Events) > 0 && len(meta.Events[0].Event) == 0 {
		t.Error("expected non-empty serialized event payload in session event")
	}
	for _, forbidden := range []string{"uuid", "started_at", "first_event"} {
		metaMap := map[string]interface{}{}
		if err := json.Unmarshal(metaBytes, &metaMap); err != nil {
			t.Fatalf("expected valid JSON map in session.json, got: %v", err)
		}
		if _, ok := metaMap[forbidden]; ok {
			t.Errorf("expected %s to be absent in session.json", forbidden)
		}
	}
	for _, forbidden := range []string{"native", "event", "response"} {
		metaMap := map[string]interface{}{}
		if err := json.Unmarshal(metaBytes, &metaMap); err != nil {
			t.Fatalf("expected valid JSON map in session.json, got: %v", err)
		}
		if _, ok := metaMap[forbidden]; ok {
			t.Errorf("expected %s to be absent at root of session.json", forbidden)
		}
	}
	if len(meta.Events) == 0 {
		t.Fatalf("expected session.json to be written alongside agent events")
	}
}

func TestHookLoggingToolBlocked(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	now := time.Now().UTC()
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "unknown")
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   time.Now().UTC().Format(time.RFC3339),
		RepoRoot: tmpDir,
		ToolName: "bash",
		ToolArgs: "git checkout main",
	}
	result := RunHook(hctx)
	if result.IsAllowed() {
		t.Error("expected blocked tool to be denied")
	}
	entries, err := os.ReadDir(logDir)
	if err != nil {
		t.Fatalf("expected log dir to exist: %v", err)
	}
	if len(entries) != 1 {
		t.Fatalf("expected only session.json, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json, got %s", entries[0].Name())
	}
	data, err := os.ReadFile(filepath.Join(logDir, "session.json"))
	if err != nil {
		t.Fatalf("cannot read session.json: %v", err)
	}
	var meta SessionMeta
	if err := json.Unmarshal(data, &meta); err != nil {
		t.Fatalf("expected valid session.json: %v", err)
	}
	if len(meta.Events) != 1 {
		t.Fatalf("expected 1 event in session.json, got %d", len(meta.Events))
	}
	entry := meta.Events[0]
	if entry.Response.Blocked == nil || !*entry.Response.Blocked {
		t.Error("expected response.blocked=true for blocked tool")
	}
	if !strings.Contains(entry.Response.Reason, "blocked") {
		t.Errorf("expected blocked reason in log, got: %s", entry.Response.Reason)
	}
}

func TestHookLoggingStdinInput(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	logDir := sessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "abc123")
	payload := json.RawMessage(`{"session_id":"abc123","second":"2026-02-20T12:00:00Z","tool_name":"Bash","tool_input":{"command":"ls"},"transcript_path":"/tmp/t.jsonl"}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "claude-code",
		Second:   "2026-02-20T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	RunHook(hctx)
	entries, _ := os.ReadDir(logDir)
	if len(entries) != 1 {
		t.Fatalf("expected only session.json, got %d files", len(entries))
	}
	if entries[0].Name() != "session.json" {
		t.Fatalf("expected session.json, got %s", entries[0].Name())
	}
	data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
	var meta SessionMeta
	json.Unmarshal(data, &meta)
	if len(meta.Events) != 1 {
		t.Fatalf("expected 1 event in session.json, got %d", len(meta.Events))
	}
	entry := meta.Events[0]
	if len(entry.Native.Event) == 0 {
		t.Error("expected native.event from HookContext.Input")
	}
	var wantMap, gotMap map[string]interface{}
	json.Unmarshal(payload, &wantMap)
	json.Unmarshal(entry.Native.Event, &gotMap)
	wantBytes, _ := json.Marshal(wantMap)
	gotBytes, _ := json.Marshal(gotMap)
	if string(gotBytes) != string(wantBytes) {
		t.Errorf("expected raw %s in log, got: %s", wantBytes, gotBytes)
	}
	var evt map[string]interface{}
	json.Unmarshal(entry.Event, &evt)
	expectedSession := resolveEventSessionID("abc123")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := resolveEventSecondID("2026-02-20T12:00:00Z")
	if evt["second"] != expectedSecond {
		t.Errorf("expected event.second %s from input, got: %v", expectedSecond, evt["second"])
	}
	if evt["transcript"] != "/tmp/t.jsonl" {
		t.Errorf("expected event.transcript, got: %v", evt["transcript"])
	}
	if evt["client"] != "claude-code" {
		t.Errorf("expected event.client claude-code, got: %v", evt["client"])
	}
}
