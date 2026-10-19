// 🧷️ Tests of the event data population per hook event.

package hooks

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
	sandbox "github.com/usalu/semio/repo/workspace/sandbox"
)

func TestPopulateEventDataAgentStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-abc","parent":"subagent"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T10:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
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
	hctx := model.HookContext{
		Event:      model.HookAgentStarted,
		Client:     "codex",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Parent != "sess-sub" {
		t.Errorf("expected parent=sess-sub, got %s", res.Parent)
	}
}

func TestPopulateEventDataAgentStartingSubagentUsesAgentIDAsSession(t *testing.T) {
	payload := json.RawMessage(`{"session_id":"sess-parent","agent_id":"agent-child"}`)
	hctx := model.HookContext{
		Event:      model.HookAgentStarted,
		Client:     "copilot-chat",
		Second:     "2026-03-06T19:54:55.558Z",
		RepoRoot:   t.TempDir(),
		Input:      payload,
		ParentInfo: "subagent",
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
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
	hctx := model.HookContext{
		Event:      model.HookAgentStarted,
		Client:     "claude-code",
		Second:     "2026-02-19T10:00:00Z",
		RepoRoot:   t.TempDir(),
		ParentInfo: "parent-agent",
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
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
	hctx := model.HookContext{
		Event:    model.HookAgentEnded,
		Client:   "cursor-chat",
		Second:   "2026-02-19T11:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentEnded)
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
	hctx := model.HookContext{
		Event:    model.HookAgentPromptSubmitting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T12:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentPromptSubmitting)
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
	hctx := model.HookContext{
		Event:    model.HookAgentCompacting,
		Client:   "claude-code",
		Second:   "2026-02-19T13:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentCompacting)
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
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:00:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolStarting)
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
	hctx := model.HookContext{
		Event:    model.HookAgentToolEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T14:30:00Z",
		RepoRoot: t.TempDir(),
		ToolName: "runSubagent",
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolEnded)
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
	hctx := model.HookContext{
		Event:    model.HookAgentToolPlanUpdatingStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T15:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolPlanUpdating)
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
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T16:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
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
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T17:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditStarting)
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
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditStarting,
		Client:   "cursor-chat",
		Second:   "2026-02-19T17:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditStarting, got %T", result)
	}
	if !res.All {
		t.Error("expected all=true")
	}
}

func TestPopulateEventDataCodeEdited(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ced","tool_input":{"filePath":"/tmp/edited.ts","oldString":"before","newString":"after"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T18:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditEnded)
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

func TestCodeEditEndedRunsFormatter(t *testing.T) {
	// 📋️Track whether the formatter was invoked.
	var formatterCalled bool
	var formatterBinary string
	origRun := workspace.FormatterCommandRun
	workspace.FormatterCommandRun = func(binary string, args []string, workDir string) error {
		formatterCalled = true
		formatterBinary = binary
		return nil
	}
	defer func() { workspace.FormatterCommandRun = origRun }()

	// Stub binary lookup so the formatter plan is considered available.
	origLookup := workspace.FormatterBinaryLookup
	workspace.FormatterBinaryLookup = func(file string) (string, error) { return "/usr/bin/" + file, nil }
	defer func() { workspace.FormatterBinaryLookup = origLookup }()

	tmpDir := t.TempDir()
	// Create requirement files so the plan is available (prettier needs .prettierrc.json and node_modules/.bin/prettier).
	prettierBin := filepath.Join(tmpDir, "node_modules", ".bin")
	_ = os.MkdirAll(prettierBin, 0o755)
	_ = os.WriteFile(filepath.Join(prettierBin, "prettier"), []byte("#!/bin/sh\n"), 0o755)
	_ = os.WriteFile(filepath.Join(tmpDir, ".prettierrc.json"), []byte("{}"), 0o644)

	sandbox.BindRoot(t, tmpDir)

	relPath := "src/example.ts"
	payload := json.RawMessage(`{"sessionId":"sess-fmt","tool_input":{"filePath":"` + relPath + `","oldString":"old","newString":"new"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolCodeEditEnded,
		Client:   "claude-code",
		Second:   "2026-03-16T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := dispatchHook(hctx)
	res, ok := result.(model.HookResultAgentToolCodeEditEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolCodeEditEnded, got %T", result)
	}
	if res.Path != relPath {
		t.Errorf("expected path=%s, got %s", relPath, res.Path)
	}
	if !formatterCalled {
		t.Error("expected formatter to be called on code edit ended, but it was not")
	}
	if !strings.Contains(formatterBinary, "prettier") {
		t.Errorf("expected prettier formatter for .ts file, got %s", formatterBinary)
	}
}

func TestPopulateEventDataTerminalStarting(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ts","tool_input":{"command":"npm test"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolTerminalStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalStarting, got %T", result)
	}
	if res.Session != "sess-ts" {
		t.Errorf("expected session=sess-ts, got %s", res.Session)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
}

func TestPopulateEventDataTerminalEnded(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tse","tool_input":{"command":"npm test"},"pid":"12345","terminated":true,"stdout":"all passed","stderr":""}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolTerminalEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolTerminalEnded)
	if !ok {
		t.Fatalf("expected HookResultAgentToolTerminalEnded, got %T", result)
	}
	if res.Command != "npm test" {
		t.Errorf("expected command=npm test, got %s", res.Command)
	}
	if res.PID != 12345 {
		t.Errorf("expected pid=12345, got %d", res.PID)
	}
	if !res.Terminated {
		t.Error("expected terminated=true")
	}
	if res.Output != nil {
		var outputStr string
		if err := json.Unmarshal(res.Output, &outputStr); err != nil {
			t.Errorf("expected output to be JSON string, got error: %v", err)
		} else if outputStr != "all passed" {
			t.Errorf("expected stdout=all passed, got %s", outputStr)
		}
	}
}

func TestPopulateEventDataVersionCheckpointEnded(t *testing.T) {
	tmpDir := t.TempDir()
	payload := json.RawMessage(`{"sha":"abc123def","message":"feat: add hooks"}`)
	hctx := model.HookContext{
		Event:    model.HookVersionCheckpointEnded,
		Client:   "",
		Second:   "2026-02-19T20:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultVersionCheckpointEnded)
	if !ok {
		t.Fatalf("expected HookResultVersionCheckpointEnded, got %T", result)
	}
	if res.Checkpoint != "abc123def" {
		t.Errorf("expected checkpoint=abc123def, got %s", res.Checkpoint)
	}
	if res.Description != "feat: add hooks" {
		t.Errorf("expected description=feat: add hooks, got %s", res.Description)
	}
}

func TestPopulateEventDataVersionCheckpointStartingFromFile(t *testing.T) {
	tmpDir := t.TempDir()
	gitDir := filepath.Join(tmpDir, ".git")
	if err := os.MkdirAll(gitDir, 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: resolve issue"), 0644); err != nil {
		t.Fatal(err)
	}
	hctx := model.HookContext{
		Event:    model.HookVersionCheckpointStarting,
		Client:   "",
		RepoRoot: tmpDir,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultVersionCheckpointStarting)
	if !ok {
		t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
	}
	if res.Description != "fix: resolve issue" {
		t.Errorf("expected description=fix: resolve issue, got %s", res.Description)
	}
}

func TestPopulateEventDataRawField(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"raw-test","some":"data"}`)
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T21:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw == nil {
		t.Fatal("expected raw to be populated")
	}
	var rawData map[string]interface{}
	body, _ := json.Marshal(res.Raw)
	if err := json.Unmarshal(body, &rawData); err != nil {
		t.Fatalf("expected valid JSON raw, got: %v", err)
	}
	if rawData["sessionId"] != "raw-test" {
		t.Errorf("expected sessionId=raw-test in raw, got %v", rawData["sessionId"])
	}
	if rawData["some"] != "data" {
		t.Errorf("expected some=data in raw, got %v", rawData["some"])
	}
}

func TestPopulateEventDataNoInputNoRaw(t *testing.T) {
	hctx := model.HookContext{
		Event:    model.HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T22:00:00Z",
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw != nil {
		t.Error("expected raw to be nil when no input")
	}
}

func TestPopulateEventDataToolNameFromStdin(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tn","tool_name":"mcp_custom_tool","tool_input":{"arg":"val"}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Name != "mcp_custom_tool" {
		t.Errorf("expected name=mcp_custom_tool, got %s", res.Name)
	}
}

func TestPopulateEventDataCodeSearchWithExclude(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ex","tool_input":{"query":"test","include":["*.ts","*.tsx"],"exclude":["node_modules"]}}`)
	hctx := model.HookContext{
		Event:    model.HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(model.HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Pages) != 0 {
		t.Errorf("expected no pages for non-web search input, got %v", res.Pages)
	}
}
