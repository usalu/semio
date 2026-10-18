// 🧬️ Tests of the extraction of fields from a hook input.

package hooks

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	strings "strings"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
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
			result := ExtractSessionIDFromInput(json.RawMessage(tt.input))
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

	if got := ExtractTranscriptFromInput(input); got != "/home/vscode/.cursor/projects/workspaces-compose/agent-transcripts/session-1/session-1.jsonl" {
		t.Fatalf("expected nested transcript path, got %q", got)
	}

	if got := extractLLMFromInput(input); got != "composer-1.5" {
		t.Fatalf("expected nested model, got %q", got)
	}
}

func TestExtractToolNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"vscode tool_name", `{"hookEventName":"PreToolUse","tool_name":"run_in_terminal","tool_input":{"command":"ls"}}`, "run_in_terminal"},
		{"claude code tool_name", `{"tool_name":"Bash","tool_input":{"command":"git checkout main"}}`, "Bash"},
		{"no tool_name", `{"tool_input":{"command":"ls"}}`, ""},
		{"empty object", `{}`, ""},
		{"invalid json", `not json`, ""},
		{"empty input", ``, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractToolNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
	}
}

func TestExtractHookEventNameFromStdin(t *testing.T) {
	cases := []struct {
		name   string
		input  string
		expect string
	}{
		{"PreToolUse", `{"hookEventName":"PreToolUse","tool_name":"editFiles"}`, "PreToolUse"},
		{"PostToolUse", `{"hookEventName":"PostToolUse","tool_name":"editFiles"}`, "PostToolUse"},
		{"SessionStart", `{"hookEventName":"SessionStart","source":"new"}`, "SessionStart"},
		{"no hookEventName", `{"tool_name":"Bash"}`, ""},
		{"empty", `{}`, ""},
		{"invalid json", `bad`, ""},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			result := extractHookEventNameFromStdin(json.RawMessage(tc.input))
			if result != tc.expect {
				t.Errorf("expected %q, got %q", tc.expect, result)
			}
		})
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
	hctx := model.HookContext{
		Event:    model.HookAgentToolStarting,
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

func TestExtractSearchDefinitionReadsFromInput(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "sample.go")
	content := "package main\n\nfunc First() {\n\tprintln(\"alpha\")\n}\n\nfunc Second() {\n\tprintln(\"beta\")\n}\n"
	if err := os.WriteFile(filePath, []byte(content), 0o644); err != nil {
		t.Fatalf("failed to write sample file: %v", err)
	}
	input := json.RawMessage(`{"tool_info":{"command_line":"grep -n \"beta\" sample.go","cwd":"` + filepath.ToSlash(tmpDir) + `"}}`)
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
		result := extractCheckpointSHAFromInput(nil)
		if result == "" {
			t.Skip("no git repo available")
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
