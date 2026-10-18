// 📓️ Tests of the repo logging config, hook logging and repo operation logging.

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
	workspace "github.com/usalu/semio/repo/workspace"
)

func testLoggingConfigFull() workspace.LoggingConfig {
	return workspace.LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "full"}
}

func testLoggingConfigSession() workspace.LoggingConfig {
	return workspace.LoggingConfig{Session: true, Operations: true, Plan: true, Detail: "standard"}
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

func TestRepoConfig(t *testing.T) {
	t.Run("defaults when file missing", func(t *testing.T) {
		cfg := workspace.LoadRepoConfig(t.TempDir())
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
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{
			Session: true, Operations: false, Plan: false, Detail: "minimal",
		})
		cfg := workspace.LoadRepoConfig(tmpDir)
		if !cfg.Logging.Session || cfg.Logging.Operations || cfg.Logging.Plan {
			t.Errorf("unexpected logging config: %+v", cfg.Logging)
		}
		if cfg.Logging.Detail != "minimal" {
			t.Errorf("expected detail minimal, got %q", cfg.Logging.Detail)
		}
	})
	t.Run("session off by default", func(t *testing.T) {
		tmpDir := t.TempDir()
		RunHook(model.HookContext{
			Event:    model.HookAgentStarted,
			Client:   "claude-code",
			Second:   time.Now().UTC().Format(time.RFC3339),
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"off-by-default"}`),
		})
		assertNoHookLogFiles(t, tmpDir)
	})
	t.Run("detail levels", func(t *testing.T) {
		tmpDir := t.TempDir()
		logDir := SessionLogDir(tmpDir, 2026, 5, 30, "detail-sess")
		if err := os.MkdirAll(logDir, 0o755); err != nil {
			t.Fatal(err)
		}
		hctx := model.HookContext{Event: model.HookAgentStarted, Client: "claude-code", Input: json.RawMessage(`{"x":1}`)}
		result := model.HookResultBase{Allowed: true}
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{Session: true, Detail: "minimal"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", workspace.LoadRepoConfig(tmpDir).Logging, InertEnvironment{})
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
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
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{Session: true, Detail: "full"})
		writeSessionHookLog(hctx, result, logDir, "detail-sess", workspace.LoadRepoConfig(tmpDir).Logging, InertEnvironment{})
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
		writeRepoLoggingConfig(t, tmpDir, workspace.LoggingConfig{Session: true, Plan: false, Detail: "standard"})
		RunHook(model.HookContext{
			Event:    model.HookAgentToolPlanUpdatingEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"plan-off","tool_input":{"todoList":[{"title":"A","status":"pending"}]}}`),
		})
		logFiles := getLogFiles(t, tmpDir)
		if len(logFiles) != 1 {
			t.Fatalf("expected session.json, got %v", logFiles)
		}
		var meta model.SessionMeta
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
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess-log")
	payload := json.RawMessage(`{"session_id":"sess-log","second":"2026-02-20T10:00:00Z","transcript_path":"/tmp/transcript.jsonl"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
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
	var meta model.SessionMeta
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
	if evt["kind"] != string(model.HookAgentStarted) {
		t.Errorf("expected event.kind agent.started in log, got: %v", evt["kind"])
	}
	if evt["client"] != "claude-code" {
		t.Errorf("expected event.client claude-code in log, got: %v", evt["client"])
	}
	if _, ok := evt["contributor"]; !ok {
		t.Error("expected event.contributor to be present in log entry")
	}
	expectedSession := ResolveEventSessionID("sess-log")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s in log, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := ResolveEventSecondID("2026-02-20T10:00:00Z")
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

func TestSessionJsonTracksPlan(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigFull())
	now := time.Now().UTC()
	sessionID := "plan-track-session"
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), sessionID)
	startPayload := json.RawMessage(`{"session_id":"plan-track-session","second":"2026-03-02T10:00:00Z"}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "claude-code",
		Second:   "2026-03-02T10:00:00Z",
		RepoRoot: tmpDir,
		Input:    startPayload,
	})

	plan1Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"in-progress"},{"title":"Step B","status":"pending"}]}}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:01:00Z",
		RepoRoot: tmpDir,
		Input:    plan1Payload,
	})

	metaPath := filepath.Join(logDir, "session.json")
	metaBytes, err := os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after plan update, got error: %v", err)
	}
	var meta model.SessionMeta
	assertHasLifecycle := func(step model.TicketAgentPlanStep) {
		if step.Ideated == "" && step.Started == "" && step.Completed == "" && step.Abandoned == "" {
			t.Errorf("expected step %q to include at least one lifecycle timestamp, got %+v", step.Name, step)
		}
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 2 {
		t.Fatalf("expected 2 plan steps, got %v", meta.Plan)
	}
	stepA := meta.Plan.Steps[0]
	if stepA.Name != "Step A" {
		t.Errorf("expected Step A, got %s", stepA.Name)
	}
	if stepA.Ideated == "" {
		t.Errorf("expected Step A ideated timestamp, got empty")
	}
	if stepA.Started == "" {
		t.Errorf("expected Step A started timestamp, got empty")
	}
	if stepA.Completed != "" {
		t.Errorf("expected Step A not completed, got %s", stepA.Completed)
	}
	stepB := meta.Plan.Steps[1]
	if stepB.Name != "Step B" {
		t.Errorf("expected Step B, got %s", stepB.Name)
	}
	if stepB.Ideated == "" {
		t.Errorf("expected Step B ideated timestamp, got empty")
	}
	if stepB.Started != "" || stepB.Completed != "" {
		t.Errorf("expected Step B pending, got started=%s completed=%s", stepB.Started, stepB.Completed)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan2Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step A","status":"completed"},{"title":"Step B","status":"in-progress"},{"title":"Step C","status":"pending"}]}}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:02:00Z",
		RepoRoot: tmpDir,
		Input:    plan2Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after second plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after second update, got %v", meta.Plan)
	}
	stepA2 := meta.Plan.Steps[0]
	if stepA2.Name != "Step A" || stepA2.Completed == "" {
		t.Errorf("expected Step A completed, got %+v", stepA2)
	}
	if stepA2.Ideated != stepA.Ideated {
		t.Errorf("expected Step A ideated timestamp preserved, got %s vs %s", stepA2.Ideated, stepA.Ideated)
	}
	if stepA2.Started != stepA.Started {
		t.Errorf("expected Step A started timestamp preserved, got %s vs %s", stepA2.Started, stepA.Started)
	}
	stepB2 := meta.Plan.Steps[1]
	if stepB2.Name != "Step B" || stepB2.Started == "" {
		t.Errorf("expected Step B in-progress, got %+v", stepB2)
	}
	stepC := meta.Plan.Steps[2]
	if stepC.Name != "Step C" || stepC.Ideated == "" || stepC.Started != "" || stepC.Completed != "" {
		t.Errorf("expected Step C pending, got %+v", stepC)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}

	plan3Payload := json.RawMessage(`{"session_id":"plan-track-session","tool_input":{"todoList":[{"title":"Step B","status":"completed"},{"title":"Step C","status":"in-progress"}]}}`)
	RunHook(model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingEnded,
		Client:   "claude-code",
		Second:   "2026-03-02T10:03:00Z",
		RepoRoot: tmpDir,
		Input:    plan3Payload,
	})

	metaBytes, err = os.ReadFile(metaPath)
	if err != nil {
		t.Fatalf("expected session.json after third plan update: %v", err)
	}
	if err := json.Unmarshal(metaBytes, &meta); err != nil {
		t.Fatalf("invalid session.json JSON: %v", err)
	}
	if meta.Plan == nil || len(meta.Plan.Steps) != 3 {
		t.Fatalf("expected 3 plan steps after third update (Step A preserved as completed), got %v", meta.Plan)
	}
	stepB3 := meta.Plan.Steps[0]
	if stepB3.Name != "Step B" || stepB3.Completed == "" {
		t.Errorf("expected Step B completed, got %+v", stepB3)
	}
	if stepB3.Started != stepB2.Started {
		t.Errorf("expected Step B started timestamp preserved, got %s vs %s", stepB3.Started, stepB2.Started)
	}
	stepC2 := meta.Plan.Steps[1]
	if stepC2.Name != "Step C" || stepC2.Started == "" {
		t.Errorf("expected Step C in-progress, got %+v", stepC2)
	}
	stepACompleted := meta.Plan.Steps[2]
	if stepACompleted.Name != "Step A" || stepACompleted.Completed == "" || stepACompleted.Abandoned != "" {
		t.Errorf("expected Step A to remain completed and not abandoned, got %+v", stepACompleted)
	}
	for _, step := range meta.Plan.Steps {
		assertHasLifecycle(step)
	}
}

func TestHookLoggingToolBlocked(t *testing.T) {
	tmpDir := t.TempDir()
	writeRepoLoggingConfig(t, tmpDir, testLoggingConfigSession())
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "unknown")
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
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
	var meta model.SessionMeta
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
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "abc123")
	payload := json.RawMessage(`{"session_id":"abc123","second":"2026-02-20T12:00:00Z","tool_name":"Bash","tool_input":{"command":"ls"},"transcript_path":"/tmp/t.jsonl"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
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
	var meta model.SessionMeta
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
	expectedSession := ResolveEventSessionID("abc123")
	if evt["session"] != expectedSession {
		t.Errorf("expected event.session %s, got: %v", expectedSession, evt["session"])
	}
	expectedSecond := ResolveEventSecondID("2026-02-20T12:00:00Z")
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

func TestLogRepoOperationHookMCPTool(t *testing.T) {
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess1")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for mcp__repo__tree", func(t *testing.T) {
		input := json.RawMessage(`{"query":"contributors"}`)
		result := model.HookResultAgentToolStarting{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name:  "mcp__repo__tree",
			Input: input,
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
			Input:    json.RawMessage(`{"session_id":"sess1"}`),
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		hasSessionJSON := false
		for _, e := range after {
			if e.Name() == "session.json" {
				hasSessionJSON = true
				break
			}
		}
		if len(after) != len(before)+1 || !hasSessionJSON {
			t.Fatalf("expected session.json to be created for derived repo operation event")
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 derived event, got %d", len(meta.Events))
		}
		entry := meta.Events[0]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.tree.starting" {
			t.Errorf("expected kind agent.tree.starting, got: %v", evt["kind"])
		}
	})

	t.Run("ended event for mcp__repo__ticket_open", func(t *testing.T) {
		result := model.HookResultAgentToolEnded{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess1",
			},
			Name: "mcp__repo__ticket_open",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Fatalf("expected no extra files beyond session.json, got %d", len(after))
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 2 {
			t.Fatalf("expected 2 derived events, got %d", len(meta.Events))
		}
		entry := meta.Events[1]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.ticket.open.ended" {
			t.Errorf("expected kind agent.ticket.open.ended, got: %v", evt["kind"])
		}
	})

	t.Run("no extra file for non-compose tool", func(t *testing.T) {
		result := model.HookResultAgentToolStarting{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
			},
			Name: "Bash",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess1", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Errorf("expected no new files for Bash tool, got %d new files", len(after)-len(before))
		}
	})
}

func TestLogRepoOperationHookCLI(t *testing.T) {
	tmpDir := t.TempDir()
	now := time.Now().UTC()
	logDir := SessionLogDir(tmpDir, now.Year(), int(now.Month()), now.Day(), "sess2")
	if err := os.MkdirAll(logDir, 0755); err != nil {
		t.Fatal(err)
	}

	t.Run("starting event for CLI ticket open", func(t *testing.T) {
		result := model.HookResultAgentToolTerminalStarting{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "go run ./repo/client/mcp/go ticket open MY-GOAL 'My Title' claude-code sonnet-4-5",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolTerminalStarting,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess2", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		hasSessionJSON := false
		for _, e := range after {
			if e.Name() == "session.json" {
				hasSessionJSON = true
				break
			}
		}
		if len(after) != len(before)+1 || !hasSessionJSON {
			t.Fatalf("expected session.json creation for CLI repo operation event")
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 1 {
			t.Fatalf("expected 1 derived event, got %d", len(meta.Events))
		}
		entry := meta.Events[0]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.ticket.open.starting" {
			t.Errorf("expected kind agent.ticket.open.starting, got: %v", evt["kind"])
		}
	})

	t.Run("ended event for CLI goal close", func(t *testing.T) {
		result := model.HookResultAgentToolTerminalEnded{
			HookResultAgentBase: model.HookResultAgentBase{
				HookResultBase: model.HookResultBase{Allowed: true},
				Session:        "sess2",
			},
			Command: "/workspace/repo/client/client goal close MY-GOAL 'Summary'",
		}
		hctx := model.HookContext{
			Event:    model.HookAgentToolTerminalEnded,
			Client:   "claude-code",
			RepoRoot: tmpDir,
		}
		before, _ := os.ReadDir(logDir)
		logRepoOperationHook(hctx, result, logDir, now, "sess2", testLoggingConfigSession())
		after, _ := os.ReadDir(logDir)
		if len(after) != len(before) {
			t.Fatalf("expected no extra file beyond session.json, got %d", len(after))
		}
		data, _ := os.ReadFile(filepath.Join(logDir, "session.json"))
		var meta model.SessionMeta
		json.Unmarshal(data, &meta)
		if len(meta.Events) != 2 {
			t.Fatalf("expected 2 derived events, got %d", len(meta.Events))
		}
		entry := meta.Events[1]
		var evt map[string]interface{}
		json.Unmarshal(entry.Event, &evt)
		if evt["kind"] != "agent.goal.close.ended" {
			t.Errorf("expected kind agent.goal.close.ended, got: %v", evt["kind"])
		}
	})
}
