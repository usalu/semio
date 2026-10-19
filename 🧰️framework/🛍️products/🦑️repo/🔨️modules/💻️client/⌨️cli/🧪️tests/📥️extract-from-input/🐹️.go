// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Search, code edit, report, chat, and checkpoint extraction from hook input.

// #endregion 🧲️Header

package client

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func TestRunHookSearchStartingIncludesDefinitions(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc Alpha() {\n\tprintln(\"one\")\n}\n\nfunc Beta() {\n\tprintln(\"needle\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	payload := json.RawMessage(`{"trajectory_id":"trace-1","tool_info":{"command_line":"grep -n \"needle\" sample.go","cwd":"` + tmpDir + `"}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected definitions in event, got %v", res.Definitions)
	}
	if res.Definitions[0].Loc <= 0 {
		t.Fatalf("expected loc > 0, got %d", res.Definitions[0].Loc)
	}
}

func TestBuildDefinitionReadsFullFileReturnsFileID(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "full.go")
	content := "package main\n\nfunc Alpha() {}\n\nfunc Beta() {}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	origRoot := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(origRoot)

	lineSet := map[int]struct{}{1: {}, 2: {}, 3: {}, 4: {}, 5: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) != 1 {
		t.Fatalf("expected exactly one file-id entry for full file read, got %d: %v", len(definitions), definitions)
	}
	if definitions[0].Loc != 0 {
		t.Errorf("expected loc=0 for file-id entry, got %d", definitions[0].Loc)
	}
	if definitions[0].ID == "" {
		t.Errorf("expected non-empty file ID")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	if strings.Contains(definitions[0].ID, absPrefix) {
		t.Errorf("file ID %q contains absolute tmpDir path %q", definitions[0].ID, absPrefix)
	}
}

func TestBuildDefinitionReadsUsesRepoRelativePaths(t *testing.T) {
	tmpDir := t.TempDir()
	subDir := filepath.Join(tmpDir, "pkg")
	if err := os.MkdirAll(subDir, 0o755); err != nil {
		t.Fatalf("failed to create subdir: %v", err)
	}
	filePath := filepath.Join(subDir, "util.go")
	content := "package pkg\n\nfunc Helper() {\n\tprintln(\"found\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	origRoot := GetRootDir()
	SetRootDir(tmpDir)
	defer SetRootDir(origRoot)
	lineSet := map[int]struct{}{3: {}}
	filesToLines := map[string]map[int]struct{}{filePath: lineSet}
	definitions := buildDefinitionReads(filesToLines)
	if len(definitions) == 0 {
		t.Fatalf("expected at least one definition, got none")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute tmpDir path %q", def.ID, absPrefix)
		}
	}
}

func TestRunHookPreReadCodeDefinitionIDsAreRepoRelative(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "cmd.go")
	content := "package main\n\nfunc Run() {\n\tprintln(\"run\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	payload := json.RawMessage(`{"tool_info":{"file_path":"` + filepath.ToSlash(filePath) + `"}}`)
	hctx := HookContext{
		Event:    HookAgentToolSearchStarting,
		Client:   "windsurf-chat",
		RepoRoot: tmpDir,
		Input:    payload,
	}
	result := RunHook(hctx)
	res, ok := result.(HookResultAgentToolSearchStarting)
	if !ok {
		t.Fatalf("expected HookResultAgentToolSearchStarting, got %T", result)
	}
	if len(res.Definitions) == 0 {
		t.Fatalf("expected at least one definition")
	}
	absPrefix := filepath.ToSlash(tmpDir)
	for _, def := range res.Definitions {
		if strings.Contains(def.ID, absPrefix) {
			t.Errorf("definition ID %q contains absolute path %q", def.ID, absPrefix)
		}
	}
}

func TestShouldExecuteSearchCommand(t *testing.T) {
	tests := []struct {
		name    string
		command string
		want    bool
	}{
		{name: "grep command", command: "grep -n needle file.go", want: true},
		{name: "cat command", command: "cat file.go", want: true},
		{name: "blocked redirection", command: "grep -n needle file.go > out.txt", want: false},
		{name: "edit command", command: "sed -i 's/a/b/' file.go", want: false},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := shouldExecuteSearchCommand(tt.command)
			if got != tt.want {
				t.Fatalf("shouldExecuteSearchCommand(%q) = %v, want %v", tt.command, got, tt.want)
			}
		})
	}
}

func TestExtractCodeEditFromInput(t *testing.T) {
	t.Run("replace_string_in_file style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/file.go","oldString":"old","newString":"new"}}`)
		path, old, new_, all := extractCodeEditFromInput(input, "")
		if path != "/tmp/file.go" {
			t.Errorf("expected path=/tmp/file.go, got %s", path)
		}
		if old != "old" {
			t.Errorf("expected old=old, got %s", old)
		}
		if new_ != "new" {
			t.Errorf("expected new=new, got %s", new_)
		}
		if all {
			t.Error("expected all=false")
		}
	})
	t.Run("with replaceAll", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"filePath":"/tmp/f.go","oldString":"a","newString":"b","replaceAll":true}}`)
		_, _, _, all := extractCodeEditFromInput(input, "")
		if !all {
			t.Error("expected all=true")
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		path, _, _, _ := extractCodeEditFromInput(nil, `{"filePath":"/from/args.go"}`)
		if path != "/from/args.go" {
			t.Errorf("expected path=/from/args.go, got %s", path)
		}
	})
}

func TestExtractToolInputFromStdin(t *testing.T) {
	t.Run("with tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test","tool_input":{"key":"val"}}`)
		result := extractToolInputFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil tool input")
		}
		var data map[string]interface{}
		if err := json.Unmarshal(result, &data); err != nil {
			t.Fatal(err)
		}
		if data["key"] != "val" {
			t.Errorf("expected key=val, got %v", data["key"])
		}
	})
	t.Run("without tool_input", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := extractToolInputFromStdin(input)
		if result != nil {
			t.Error("expected nil tool input")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractToolInputFromStdin(nil)
		if result != nil {
			t.Error("expected nil for empty input")
		}
	})
}

func TestExtractToolResponseFromStdin(t *testing.T) {
	t.Run("tool_output", func(t *testing.T) {
		input := json.RawMessage(`{"tool_output":"response data"}`)
		result := extractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("tool_response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_response":"data"}`)
		result := extractToolResponseFromStdin(input)
		if result == nil {
			t.Fatal("expected non-nil response")
		}
	})
	t.Run("no response", func(t *testing.T) {
		input := json.RawMessage(`{"tool_name":"test"}`)
		result := extractToolResponseFromStdin(input)
		if result != nil {
			t.Error("expected nil response")
		}
	})
}

func TestExtractTerminalEndedFromInput(t *testing.T) {
	t.Run("full payload", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"command":"ls -la"},"pid":"999","terminated":true,"stdout":"file1\nfile2","stderr":""}`)
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(input)
		if command != "ls -la" {
			t.Errorf("expected command=ls -la, got %s", command)
		}
		if pid != "999" {
			t.Errorf("expected pid=999, got %s", pid)
		}
		if !terminated {
			t.Error("expected terminated=true")
		}
		if stdout != "file1\nfile2" {
			t.Errorf("expected stdout, got %s", stdout)
		}
		if stderr != "" {
			t.Errorf("expected empty stderr, got %s", stderr)
		}
	})
	t.Run("numeric pid", func(t *testing.T) {
		input := json.RawMessage(`{"pid":42}`)
		_, pid, _, _, _ := extractTerminalEndedFromInput(input)
		if pid != "42" {
			t.Errorf("expected pid=42, got %s", pid)
		}
	})
	t.Run("empty", func(t *testing.T) {
		command, pid, terminated, stdout, stderr := extractTerminalEndedFromInput(nil)
		if command != "" || pid != "" || terminated || stdout != "" || stderr != "" {
			t.Error("expected all empty for nil input")
		}
	})
}

func TestExtractChatFromInput(t *testing.T) {
	t.Run("chat string", func(t *testing.T) {
		input := json.RawMessage(`{"chat":"conversation context"}`)
		result := extractChatFromInput(input)
		if result != "conversation context" {
			t.Errorf("expected conversation context, got %s", result)
		}
	})
	t.Run("messages array", func(t *testing.T) {
		input := json.RawMessage(`{"messages":[{"role":"user","content":"hello"},{"role":"assistant","content":"hi"}]}`)
		result := extractChatFromInput(input)
		if result == "" {
			t.Error("expected non-empty chat from messages array")
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractChatFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractReportFromInput(t *testing.T) {
	t.Run("report field", func(t *testing.T) {
		input := json.RawMessage(`{"report":"agent summary"}`)
		result := extractReportFromInput(input)
		if result != "agent summary" {
			t.Errorf("expected agent summary, got %s", result)
		}
	})
	t.Run("tool_info response field", func(t *testing.T) {
		input := json.RawMessage(`{"tool_info":{"response":"from tool info"}}`)
		result := extractReportFromInput(input)
		if result != "from tool info" {
			t.Errorf("expected from tool info, got %s", result)
		}
	})
	t.Run("native event tool_info response field", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"tool_info":{"response":"nested response"}}}}`)
		result := extractReportFromInput(input)
		if result != "nested response" {
			t.Errorf("expected nested response, got %s", result)
		}
	})
	t.Run("empty input", func(t *testing.T) {
		result := extractReportFromInput(nil)
		if result != "" {
			t.Errorf("expected empty report, got %s", result)
		}
	})
}

func TestExtractCheckpointMessageFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"message":"feat: new feature"}`)
		result := extractCheckpointMessageFromInput(input, "/nonexistent")
		if result != "feat: new feature" {
			t.Errorf("expected feat: new feature, got %s", result)
		}
	})
	t.Run("from COMMIT_EDITMSG file", func(t *testing.T) {
		tmpDir := t.TempDir()
		gitDir := filepath.Join(tmpDir, ".git")
		os.MkdirAll(gitDir, 0755)
		os.WriteFile(filepath.Join(gitDir, "COMMIT_EDITMSG"), []byte("fix: bug fix"), 0644)
		result := extractCheckpointMessageFromInput(nil, tmpDir)
		if result != "fix: bug fix" {
			t.Errorf("expected fix: bug fix, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractCheckpointMessageFromInput(nil, "/nonexistent")
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestExtractCheckpointSHAFromInput(t *testing.T) {
	t.Run("from input json", func(t *testing.T) {
		input := json.RawMessage(`{"sha":"deadbeef123"}`)
		result := extractCheckpointSHAFromInput(input)
		if result != "deadbeef123" {
			t.Errorf("expected deadbeef123, got %s", result)
		}
	})
	t.Run("empty falls back to git", func(t *testing.T) {
		root := withExhaustiveRepo(t)
		want := strings.TrimSpace(string(execCommandWithTimeout(t, 30*time.Second, root, nil, "git", "rev-parse", "HEAD")))
		if result := extractCheckpointSHAFromInput(nil); result != want {
			t.Errorf("expected the fixture HEAD %q, got %q", want, result)
		}
	})
}

func TestExtractParentFromInput(t *testing.T) {
	t.Run("parent field", func(t *testing.T) {
		input := json.RawMessage(`{"parent":"parent-session"}`)
		result := extractParentFromInput(input)
		if result != "parent-session" {
			t.Errorf("expected parent-session, got %s", result)
		}
	})
	t.Run("nested event parent field", func(t *testing.T) {
		input := json.RawMessage(`{"event":{"parent":"parent-from-event"}}`)
		result := extractParentFromInput(input)
		if result != "parent-from-event" {
			t.Errorf("expected parent-from-event, got %s", result)
		}
	})
	t.Run("nested native event parent field", func(t *testing.T) {
		input := json.RawMessage(`{"native":{"event":{"parent":"parent-from-native-event"}}}`)
		result := extractParentFromInput(input)
		if result != "parent-from-native-event" {
			t.Errorf("expected parent-from-native-event, got %s", result)
		}
	})
	t.Run("sentinel parent field", func(t *testing.T) {
		input := json.RawMessage(`{"parent":"subagent"}`)
		result := extractParentFromInput(input)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
	t.Run("empty", func(t *testing.T) {
		result := extractParentFromInput(nil)
		if result != "" {
			t.Errorf("expected empty, got %s", result)
		}
	})
}

func TestHookResultJSONFields(t *testing.T) {
	result := HookResultAgentToolTerminalEnded{
		HookResultAgentBase: HookResultAgentBase{
			HookResultBase: HookResultBase{
				Allowed: true,
				Raw:     json.RawMessage(`{"raw":"data"}`),
			},
			Session:   "sess-1",
			Second:    "2026-02-19T10:00:00Z",
			Client:    "copilot-chat",
			MessageID: "msg-123",
		},
		Command:    "npm test",
		PID:        12345,
		Terminated: true,
		Stdout:     json.RawMessage(`"passed"`),
		Stderr:     json.RawMessage(`"warn"`),
	}
	data, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("expected valid JSON, got: %v", err)
	}
	var parsed map[string]interface{}
	if err := json.Unmarshal(data, &parsed); err != nil {
		t.Fatalf("expected valid JSON round-trip, got: %v", err)
	}
	expectedKeys := []string{"allowed", "message", "session", "second", "client", "command", "pid", "terminated", "stdout", "stderr"}
	for _, key := range expectedKeys {
		if _, ok := parsed[key]; !ok {
			t.Errorf("missing JSON key: %s", key)
		}
	}
}

func TestHookResultOmitEmpty(t *testing.T) {
	result := HookResultAgentStarted{
		HookResultAgentBase: HookResultAgentBase{HookResultBase: HookResultBase{Allowed: true}},
	}
	data, _ := json.Marshal(result)
	var parsed map[string]interface{}
	json.Unmarshal(data, &parsed)
	if _, ok := parsed["session"]; ok {
		t.Error("expected session to be omitted when empty")
	}
	if _, ok := parsed["raw"]; ok {
		t.Error("expected raw to be omitted when nil")
	}
}
