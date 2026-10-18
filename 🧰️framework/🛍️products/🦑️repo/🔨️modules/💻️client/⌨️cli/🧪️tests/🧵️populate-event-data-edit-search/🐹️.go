// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Event data population for edit, terminal, and checkpoint events plus input extraction.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestCodeEditEndedRunsFormatter(t *testing.T) {
	// 📋️Track whether the formatter was invoked.
	var formatterCalled bool
	var formatterBinary string
	origRun := formatterCommandRun
	formatterCommandRun = func(binary string, args []string, workDir string) error {
		formatterCalled = true
		formatterBinary = binary
		return nil
	}
	defer func() { formatterCommandRun = origRun }()

	// Stub binary lookup so the formatter plan is considered available.
	origLookup := formatterBinaryLookup
	formatterBinaryLookup = func(file string) (string, error) { return "/usr/bin/" + file, nil }
	defer func() { formatterBinaryLookup = origLookup }()

	tmpDir := t.TempDir()
	// Create requirement files so the plan is available (prettier needs .prettierrc.json and node_modules/.bin/prettier).
	prettierBin := filepath.Join(tmpDir, "node_modules", ".bin")
	_ = os.MkdirAll(prettierBin, 0o755)
	_ = os.WriteFile(filepath.Join(prettierBin, "prettier"), []byte("#!/bin/sh\n"), 0o755)
	_ = os.WriteFile(filepath.Join(tmpDir, ".prettierrc.json"), []byte("{}"), 0o644)

	origRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = origRoot }()

	relPath := "src/example.ts"
	payload := json.RawMessage(`{"sessionId":"sess-fmt","tool_input":{"filePath":"` + relPath + `","oldString":"old","newString":"new"}}`)
	hctx := HookContext{
		Event:    HookAgentToolCodeEditEnded,
		Client:   "claude-code",
		Second:   "2026-03-16T12:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := dispatchHook(hctx)
	res, ok := result.(HookResultAgentToolCodeEditEnded)
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
	hctx := HookContext{
		Event:    HookAgentToolTerminalStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolTerminalStarting)
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
	hctx := HookContext{
		Event:    HookAgentToolTerminalEnded,
		Client:   "copilot-chat",
		Second:   "2026-02-19T19:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolTerminalEnded)
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
	hctx := HookContext{
		Event:    HookVersionCheckpointEnded,
		Client:   "",
		Second:   "2026-02-19T20:00:00Z",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultVersionCheckpointEnded)
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
	hctx := HookContext{
		Event:    HookVersionCheckpointStarting,
		Client:   "",
		RepoRoot: tmpDir,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultVersionCheckpointStarting)
	if !ok {
		t.Fatalf("expected HookResultVersionCheckpointStarting, got %T", result)
	}
	if res.Description != "fix: resolve issue" {
		t.Errorf("expected description=fix: resolve issue, got %s", res.Description)
	}
}

func TestPopulateEventDataRawField(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"raw-test","some":"data"}`)
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T21:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
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
	hctx := HookContext{
		Event:    HookAgentStarted,
		Client:   "copilot-chat",
		Second:   "2026-02-19T22:00:00Z",
		RepoRoot: t.TempDir(),
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentStarted)
	if !ok {
		t.Fatalf("expected HookResultAgentStarted, got %T", result)
	}
	if res.Raw != nil {
		t.Error("expected raw to be nil when no input")
	}
}

func TestPopulateEventDataToolNameFromStdin(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-tn","tool_name":"mcp_custom_tool","tool_input":{"arg":"val"}}`)
	hctx := HookContext{
		Event:    HookAgentToolStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:00:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolStarting, got %T", result)
	}
	if res.Name != "mcp_custom_tool" {
		t.Errorf("expected name=mcp_custom_tool, got %s", res.Name)
	}
}

func TestPopulateEventDataCodeSearchWithExclude(t *testing.T) {
	payload := json.RawMessage(`{"sessionId":"sess-ex","tool_input":{"query":"test","include":["*.ts","*.tsx"],"exclude":["node_modules"]}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "copilot-chat",
		Second:   "2026-02-19T23:30:00Z",
		RepoRoot: t.TempDir(),
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Pages) != 0 {
		t.Errorf("expected no pages for non-web search input, got %v", res.Pages)
	}
}

func TestExtractPlanStepsFromInput(t *testing.T) {
	t.Run("todoList format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"todoList":[{"title":"Task A","status":"completed"},{"title":"Task B","status":"in-progress"}]}}`)
		steps := extractPlanStepsFromInput(input, "")
		if len(steps) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(steps))
		}
		if steps[0].Name != "Task A" || steps[0].Status != "completed" {
			t.Errorf("unexpected step 0: %+v", steps[0])
		}
	})
	t.Run("steps format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"steps":[{"name":"Build","status":"pending"}]}}`)
		steps := extractPlanStepsFromInput(input, "")
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "Build" {
			t.Errorf("expected name=Build, got %s", steps[0].Name)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		steps := extractPlanStepsFromInput(nil, `{"todoList":[{"title":"FromArgs","status":"done"}]}`)
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "FromArgs" {
			t.Errorf("expected name=FromArgs, got %s", steps[0].Name)
		}
	})
	t.Run("empty", func(t *testing.T) {
		steps := extractPlanStepsFromInput(nil, "")
		if steps != nil {
			t.Errorf("expected nil steps, got %v", steps)
		}
	})
}

func TestExtractSearchFromInput(t *testing.T) {
	t.Run("grep_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
		pages, ranges := extractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("file_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"**/*.ts"}}`)
		pages, ranges := extractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("read_file style", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		if err := os.WriteFile(tempFile, []byte("one\ntwo\nthree\n"), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"filePath":%q}}`, tempFile))
		pages, ranges := extractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		pages, ranges := extractSearchFromInput(nil, `{"query":"fromArgs"}`)
		if len(pages) != 0 {
			t.Errorf("expected no webpages from non-url query, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("webpages only", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"url":"https://example.com/docs","pages":["https://compose.dev","not-a-url"]}}`)
		pages, ranges := extractSearchFromInput(input, "")
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
		if len(pages) != 2 || pages[0] != "https://example.com/docs" || pages[1] != "https://compose.dev" {
			t.Errorf("expected only valid webpages, got %v", pages)
		}
	})
	t.Run("read tool with limit only", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"file_path":%q,"limit":3}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("read tool with offset and limit", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"file_path":%q,"offset":2,"limit":3}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L2-L4"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("native claude code format with file_path and limit", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\nline4\nline5\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":%q,"limit":100}}}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")

		want := tempFile + "#L1-L100"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("native claude code format without limit reads full file", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\nline3\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":%q}}}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("grep tool with pattern ignores path for line range", func(t *testing.T) {
		tempDir := t.TempDir()
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"pattern":"foo","path":%q}}`, tempDir))
		_, ranges := extractSearchFromInput(input, "")

		if len(ranges) != 0 {
			t.Errorf("grep with pattern+path should produce no file ranges, got %v", ranges)
		}
	})
	t.Run("grep tool with pattern and file_path uses file_path for range", func(t *testing.T) {
		tempFile := filepath.Join(t.TempDir(), "test.go")
		content := "line1\nline2\n"
		if err := os.WriteFile(tempFile, []byte(content), 0o644); err != nil {
			t.Fatalf("failed to write temp file: %v", err)
		}
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"pattern":"foo","file_path":%q}}`, tempFile))
		_, ranges := extractSearchFromInput(input, "")
		want := tempFile + "#L1-L2"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
}

func TestExtractToolInputMapFromData(t *testing.T) {
	t.Run("direct tool_input", func(t *testing.T) {
		data := map[string]interface{}{"tool_input": map[string]interface{}{"key": "val"}}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "val" {
			t.Errorf("expected tool_input with key=val, got %v", ti)
		}
	})
	t.Run("native.event.tool_input", func(t *testing.T) {
		data := map[string]interface{}{
			"native": map[string]interface{}{
				"event": map[string]interface{}{
					"tool_input": map[string]interface{}{"key": "native"},
				},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "native" {
			t.Errorf("expected native tool_input with key=native, got %v", ti)
		}
	})
	t.Run("event.tool_input", func(t *testing.T) {
		data := map[string]interface{}{
			"event": map[string]interface{}{
				"tool_input": map[string]interface{}{"key": "event"},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "event" {
			t.Errorf("expected event tool_input with key=event, got %v", ti)
		}
	})
	t.Run("direct takes precedence over native", func(t *testing.T) {
		data := map[string]interface{}{
			"tool_input": map[string]interface{}{"key": "direct"},
			"native": map[string]interface{}{
				"event": map[string]interface{}{
					"tool_input": map[string]interface{}{"key": "native"},
				},
			},
		}
		ti := extractToolInputMapFromData(data)
		if ti == nil || ti["key"] != "direct" {
			t.Errorf("expected direct tool_input to take precedence, got %v", ti)
		}
	})
	t.Run("empty data returns nil", func(t *testing.T) {
		ti := extractToolInputMapFromData(map[string]interface{}{})
		if ti != nil {
			t.Errorf("expected nil for empty data, got %v", ti)
		}
	})
}

func TestExtractToolInputFromStdinNativeFormat(t *testing.T) {
	t.Run("native.event.tool_input extracted", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go","limit":100}}}}`)
		result := extractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input from native format")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["file_path"] != "/tmp/test.go" {
			t.Errorf("expected file_path=/tmp/test.go, got %v", data["file_path"])
		}
	})
	t.Run("direct tool_input still works", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"Read","tool_input":{"file_path":"/tmp/test.go"}}`)
		result := extractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
	})
}

func TestExtractSearchDefinitionReadsFromInput(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc First() {\n\tprintln(\"alpha\")\n}\n\nfunc Second() {\n\tprintln(\"beta\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	input := json.RawMessage(`{"tool_info":{"command_line":"grep -n \"beta\" sample.go","cwd":"` + tmpDir + `"}}`)
	definitions := extractSearchDefinitionReadsFromInput(input, "")
	if len(definitions) == 0 {
		t.Fatalf("expected at least one definition read, got %v", definitions)
	}
	foundSecond := false
	for _, definition := range definitions {
		if strings.Contains(strings.ToLower(definition.ID), "second") {
			foundSecond = true
			if definition.Loc <= 0 {
				t.Fatalf("expected positive loc for Second, got %d", definition.Loc)
			}
		}
	}
	if !foundSecond {
		t.Fatalf("expected Second definition in %v", definitions)
	}
}
