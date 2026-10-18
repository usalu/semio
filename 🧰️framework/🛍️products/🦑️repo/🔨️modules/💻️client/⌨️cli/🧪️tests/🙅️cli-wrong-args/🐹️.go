// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// CLI wrong-argument behaviors per command.

// #endregion 🧲️Header

package client

import (
	"strings"
	"testing"
)

// 🎫️#region 🎬️Wrong Argument
func TestCliWrongArgs_TicketOpen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing emoji", []string{"ticket", "open", "--goal", "TEST", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing title", []string{"ticket", "open", "--emoji", "🎫️", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"ticket", "open", "--emoji", "🎫️", "--goal", "TEST", "--title", "Valid Title", "--opus-4-5", "--no-management"}},
		{"missing goal", []string{"ticket", "open", "--emoji", "🎫️", "--title", "Valid Title", "--copilot-chat", "--opus-4-5", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketClose(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}},
		{"missing summary", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--files", "f"}},
		{"missing files", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "NONEXISTENT", "--summary", "s"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketReopen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"invalid path format", []string{"ticket", "reopen", "bad-path", "prompt", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing prompt", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"ticket", "reopen", "2025/01/01/NONEXISTENT", "prompt", "--opus-4-5", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TicketChange(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"invalid path format", []string{"ticket", "change", "bad-path", "--no-management"}},
		{"nonexistent ticket", []string{"ticket", "change", "9999/01/01/NONEXISTENT", "--title", "New Title", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalOpen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing title", []string{"goal", "open", "--no-management"}},
		{"missing description", []string{"goal", "open", "Valid Title", "--no-management", "--copilot-chat", "--opus-4-5", "--due-date", "2026-02-15"}},
		{"missing client", []string{"goal", "open", "Valid Title", "desc", "prompt", "--opus-4-5", "--due-date", "2026-02-15", "--no-management"}},
		{"missing llm", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--due-date", "2026-02-15", "--no-management"}},
		{"missing due-date", []string{"goal", "open", "Valid Title", "desc", "prompt", "--copilot-chat", "--opus-4-5", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalClose(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing id", []string{"goal", "close", "--no-management"}},
		{"missing summary", []string{"goal", "close", "NONEXISTENT-GOAL", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalReopen(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing id", []string{"goal", "reopen", "--no-management"}},
		{"missing prompt", []string{"goal", "reopen", "NONEXISTENT-GOAL", "--copilot-chat", "--opus-4-5", "--no-management"}},
		{"missing client", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--opus-4-5", "--no-management"}},
		{"missing llm", []string{"goal", "reopen", "NONEXISTENT-GOAL", "prompt", "--copilot-chat", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GoalChange(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing slug", []string{"goal", "change", "--no-management"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_PolicyCheck(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing policy id", []string{"policy", "check"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_FolderOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing path", []string{"folder", "create"}},
		{"move missing args", []string{"folder", "move"}},
		{"move missing target", []string{"folder", "move", "src"}},
		{"delete missing path", []string{"folder", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_FileOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing path", []string{"file", "create"}},
		{"move missing args", []string{"file", "move"}},
		{"move missing target", []string{"file", "move", "src"}},
		{"delete missing path", []string{"file", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_SectionOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing args", []string{"section", "create"}},
		{"move missing args", []string{"section", "move"}},
		{"delete missing args", []string{"section", "delete"}},
		{"extract missing args", []string{"section", "extract"}},
		{"integrate missing args", []string{"section", "integrate"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_DefinitionOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_ContributorOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"add missing github", []string{"contributor", "add"}},
		{"remove missing github", []string{"contributor", "remove"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_GraphQL(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"missing query", []string{"graphql"}},
		{"invalid query syntax", []string{"graphql", "{ invalid @@@ }"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TodoOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"create missing all", []string{"todo", "create"}},
		{"create missing name", []string{"todo", "create", "some-parent"}},
		{"change missing id", []string{"todo", "change"}},
		{"delete missing id", []string{"todo", "delete"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_TopLevelOperations(t *testing.T) {
	tests := []struct {
		name string
		args []string
	}{
		{"move missing all args", []string{"move"}},
		{"move missing target", []string{"move", "source"}},
		{"extract missing all", []string{"extract"}},
		{"integrate missing all", []string{"integrate"}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			stdout, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s, got stdout: %s", tt.name, stdout)
			}
			if stdout != "" {
				t.Errorf("expected empty stdout on error, got: %s", stdout)
			}
			_ = stderr
		})
	}
}

func TestCliWrongArgs_ErrorMessages(t *testing.T) {
	tests := []struct {
		name        string
		args        []string
		expectedErr string
	}{

		{"ticket open missing emoji", []string{"ticket", "open", "--goal", "TEST", "--title", "T", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing emoji"},
		{"ticket open missing title", []string{"ticket", "open", "--emoji", "🎫️", "--goal", "TEST", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing title"},
		{"ticket open missing goal", []string{"ticket", "open", "--emoji", "🎫️", "--title", "T", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing goal"},
		{"ticket close missing path", []string{"ticket", "close", "--no-management", "--summary", "s", "--files", "f"}, "missing ticket path"},
		{"ticket close missing summary", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "X", "--files", "f"}, "missing summary"},
		{"ticket close missing files", []string{"ticket", "close", "--no-management", "--year", "2025", "--month", "1", "--day", "1", "--slug", "X", "--summary", "s"}, "missing files"},
		{"ticket reopen missing path", []string{"ticket", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing ticket path"},

		{"goal open missing title", []string{"goal", "open", "--no-management"}, "missing title"},
		{"goal close missing id", []string{"goal", "close", "--no-management"}, "missing goal id"},
		{"goal close missing summary", []string{"goal", "close", "NONEXISTENT", "--no-management"}, "missing summary"},
		{"goal reopen missing id", []string{"goal", "reopen", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing goal id"},
		{"goal reopen missing prompt", []string{"goal", "reopen", "NONEXISTENT", "--copilot-chat", "--opus-4-5", "--no-management"}, "missing prompt"},
		{"goal reopen missing client", []string{"goal", "reopen", "NONEXISTENT", "prompt", "--opus-4-5", "--no-management"}, "missing client"},
		{"goal reopen missing llm", []string{"goal", "reopen", "NONEXISTENT", "prompt", "--copilot-chat", "--no-management"}, "missing llm"},

		{"todo create missing parent or name", []string{"todo", "create"}, "missing parent-id or name"},
		{"todo create missing name only", []string{"todo", "create", "parent"}, "missing parent-id or name"},
		{"todo change missing id", []string{"todo", "change"}, "missing id"},
		{"todo delete missing id", []string{"todo", "delete"}, "missing id"},

		{"folder create missing path", []string{"folder", "create"}, "missing path"},
		{"folder move missing args", []string{"folder", "move"}, "missing"},
		{"folder delete missing path", []string{"folder", "delete"}, "missing path"},

		{"file create missing path", []string{"file", "create"}, "missing path"},
		{"file move missing args", []string{"file", "move"}, "missing"},
		{"file delete missing path", []string{"file", "delete"}, "missing path"},

		{"section create missing args", []string{"section", "create"}, "missing"},
		{"section move missing args", []string{"section", "move"}, "missing"},
		{"section delete missing args", []string{"section", "delete"}, "missing file or name"},
		{"section extract missing args", []string{"section", "extract"}, "missing source file, source section, or target file"},
		{"section integrate missing args", []string{"section", "integrate"}, "missing source, target section, or target file"},
		{"contributor remove missing github", []string{"contributor", "remove"}, "missing"},

		{"graphql missing query", []string{"graphql"}, "missing query"},

		{"extract missing args", []string{"extract"}, "missing file, section, or target-file"},
		{"integrate missing args", []string{"integrate"}, "missing file, target-file, or target-section"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, stderr, err := executeCommand(tt.args...)
			if err == nil {
				t.Fatalf("expected error for %s", tt.name)
			}
			if !strings.Contains(strings.ToLower(stderr), strings.ToLower(tt.expectedErr)) {
				t.Errorf("expected stderr to contain %q, got: %s", tt.expectedErr, stderr)
			}
		})
	}
}
