package todos

import (
	json "encoding/json"
	fmt "fmt"
	os "os"
	exec "os/exec"
	filepath "path/filepath"
	testing "testing"
)

// #region 🔭️Hook Search Tests

func TestExtractPlanStepsFromInput(t *testing.T) {
	t.Run("todoList format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"todoList":[{"title":"Task A","status":"completed"},{"title":"Task B","status":"in-progress"}]}}`)
		steps := ExtractPlanStepsFromInput(input, "")
		if len(steps) != 2 {
			t.Fatalf("expected 2 steps, got %d", len(steps))
		}
		if steps[0].Name != "Task A" || steps[0].Status != "completed" {
			t.Errorf("unexpected step 0: %+v", steps[0])
		}
	})
	t.Run("steps format", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"steps":[{"name":"Build","status":"pending"}]}}`)
		steps := ExtractPlanStepsFromInput(input, "")
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "Build" {
			t.Errorf("expected name=Build, got %s", steps[0].Name)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		steps := ExtractPlanStepsFromInput(nil, `{"todoList":[{"title":"FromArgs","status":"done"}]}`)
		if len(steps) != 1 {
			t.Fatalf("expected 1 step, got %d", len(steps))
		}
		if steps[0].Name != "FromArgs" {
			t.Errorf("expected name=FromArgs, got %s", steps[0].Name)
		}
	})
	t.Run("empty", func(t *testing.T) {
		steps := ExtractPlanStepsFromInput(nil, "")
		if steps != nil {
			t.Errorf("expected nil steps, got %v", steps)
		}
	})
}

func TestExtractSearchFromInput(t *testing.T) {
	t.Run("grep_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"hookCommand","includePattern":"*.go"}}`)
		pages, ranges := ExtractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("file_search style", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"query":"**/*.ts"}}`)
		pages, ranges := ExtractSearchFromInput(input, "")
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
		pages, ranges := ExtractSearchFromInput(input, "")
		if len(pages) != 0 {
			t.Errorf("expected no webpages, got %v", pages)
		}
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("from toolArgs", func(t *testing.T) {
		pages, ranges := ExtractSearchFromInput(nil, `{"query":"fromArgs"}`)
		if len(pages) != 0 {
			t.Errorf("expected no webpages from non-url query, got %v", pages)
		}
		if len(ranges) != 0 {
			t.Errorf("expected no ranges, got %v", ranges)
		}
	})
	t.Run("webpages only", func(t *testing.T) {
		input := json.RawMessage(`{"tool_input":{"url":"https://example.com/docs","pages":["https://compose.dev","not-a-url"]}}`)
		pages, ranges := ExtractSearchFromInput(input, "")
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
		_, ranges := ExtractSearchFromInput(input, "")
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
		_, ranges := ExtractSearchFromInput(input, "")
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
		_, ranges := ExtractSearchFromInput(input, "")

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
		_, ranges := ExtractSearchFromInput(input, "")
		want := tempFile + "#L1-L3"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
	t.Run("grep tool with pattern ignores path for line range", func(t *testing.T) {
		tempDir := t.TempDir()
		input := json.RawMessage(fmt.Sprintf(`{"tool_input":{"pattern":"foo","path":%q}}`, tempDir))
		_, ranges := ExtractSearchFromInput(input, "")

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
		_, ranges := ExtractSearchFromInput(input, "")
		want := tempFile + "#L1-L2"
		if len(ranges) != 1 || ranges[0] != want {
			t.Errorf("expected ranges=[%s], got %v", want, ranges)
		}
	})
}

// 💾️initTestGitRepo creates a fresh git repo with signing disabled, an initial checkpoint, and returns the path.
func initTestGitRepo(t *testing.T, branch string) string {
	t.Helper()
	tmpDir := t.TempDir()
	if branch == "" {
		branch = "main"
	}
	run := func(args ...string) {
		t.Helper()
		cmd := exec.Command("git", args...)
		cmd.Dir = tmpDir
		out, err := cmd.CombinedOutput()
		if err != nil {
			t.Fatalf("git %v failed: %s\n%s", args, err, string(out))
		}
	}
	cmd := exec.Command("git", "init", "-b", branch, tmpDir)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("git init failed: %s\n%s", err, string(out))
	}
	run("config", "user.email", "test@test.com")
	run("config", "user.name", "Test")
	run("config", "commit.gpgsign", "false")
	run("config", "tag.gpgsign", "false")
	os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("hello"), 0644)
	run("add", "-A")
	run("commit", "-m", "initial")
	return tmpDir
}

// 🧲️#endregion 🌡️Technology Generate
func TestExtractSearchFromInputLineNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		toolArgs string
		expected string
	}{
		{
			name: "single line",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490,
					"endLine": 35490
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490",
		},
		{
			name: "line range",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490,
					"endLine": 35540
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490-L35540",
		},
		{
			name: "only start line",
			input: `{
				"tool_name": "read_file",
				"tool_input": {
					"filePath": "/workspaces/semio/repo/client/main.go",
					"startLine": 35490
				}
			}`,
			expected: "/workspaces/semio/repo/client/main.go#L35490",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, ranges := ExtractSearchFromInput(json.RawMessage(tt.input), tt.toolArgs)
			if len(ranges) != 1 || ranges[0] != tt.expected {
				t.Errorf("extractSearchFromInput() ranges = %v, want [%v]", ranges, tt.expected)
			}
		})
	}
}

func TestExtractSearchFromInputCompleteFileRange(t *testing.T) {
	tempFile := filepath.Join(t.TempDir(), "all.go")
	if err := os.WriteFile(tempFile, []byte("a\nb\nc"), 0o644); err != nil {
		t.Fatalf("failed to write temp file: %v", err)
	}

	input := fmt.Sprintf(`{
		"tool_name": "read_file",
		"tool_input": {
			"filePath": %q
		}
	}`, tempFile)

	pages, ranges := ExtractSearchFromInput(json.RawMessage(input), "")
	if len(pages) != 0 {
		t.Errorf("expected no webpages, got %v", pages)
	}
	expected := tempFile + "#L1-L3"
	if len(ranges) != 1 || ranges[0] != expected {
		t.Errorf("extractSearchFromInput() ranges = %v, want [%v]", ranges, expected)
	}
}

// #endregion 🔭️Hook Search Tests
