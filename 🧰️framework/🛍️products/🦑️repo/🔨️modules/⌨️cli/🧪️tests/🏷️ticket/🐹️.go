// 🏷️ Tests of the cli domain: ticket commands, paths and effort.

package cli

import (
	os "os"
	filepath "path/filepath"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestTicketOpenNoticketKeyword(t *testing.T) {
	result := ToolTicketOpen("🎫️", "Skip Ticket", "NOTICKET skip ticket creation", "gpt-5-mini", "", "codex", "", true, "", "", false, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	if result.Data != nil {
		t.Fatalf("expected no ticket data for NOTICKET keyword")
	}
}

func TestTicketOpenContinueKeyword(t *testing.T) {
	tmpDir := t.TempDir()
	run := func(name string, args ...string) {
		execCommandWithTimeout(t, 30*time.Second, tmpDir, nil, name, args...)
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}
	first := ToolTicketOpen("🌱️", "Seed Ticket", "Seed prompt", "gpt-5-mini", "", "codex", "", true, "TEST-GOAL", "", false, "", providers.McpClientGeneric, "", "")
	if first.Error != "" {
		t.Fatalf("failed to seed ticket: %s", first.Error)
	}
	seed, ok := first.Data.(*model.Ticket)
	if !ok || seed == nil {
		t.Fatalf("expected seeded ticket data")
	}
	second := ToolTicketOpen("🎫️", "Continue Ticket", "CONTINUE follow-up", "gpt-5-mini", "", "codex", "", true, "TEST-GOAL", "", false, "", providers.McpClientGeneric, "", "")
	if second.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", second.Error)
	}
	continued, ok := second.Data.(*model.Ticket)
	if !ok || continued == nil {
		t.Fatalf("expected continued ticket data")
	}
	if continued.Slug != seed.Slug {
		t.Fatalf("expected continued ticket %s, got %s", seed.Slug, continued.Slug)
	}
}

func TestParseTicketPath(t *testing.T) {
	tests := []struct {
		name    string
		path    string
		year    int
		month   int
		day     int
		slug    string
		wantErr bool
	}{
		{"two-digit year", "26/03/27/FIX-MCP", 26, 3, 27, "FIX-MCP", false},
		{"four-digit year normalized", "2026/03/27/FIX-MCP", 26, 3, 27, "FIX-MCP", false},
		{"nested slug", "26/03/27/PARENT/CHILD", 26, 3, 27, "PARENT/CHILD", false},
		{"too few parts", "26/03", 0, 0, 0, "", true},
		{"empty slug", "26/03/27/", 0, 0, 0, "", true},
		{"non-numeric year", "abc/03/27/SLUG", 0, 0, 0, "", true},
		{"non-numeric month", "26/abc/27/SLUG", 0, 0, 0, "", true},
		{"non-numeric day", "26/03/abc/SLUG", 0, 0, 0, "", true},
		{"empty string", "", 0, 0, 0, "", true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			y, m, d, s, err := parseTicketPath(tt.path)
			if (err != nil) != tt.wantErr {
				t.Fatalf("parseTicketPath(%q) error = %v, wantErr %v", tt.path, err, tt.wantErr)
			}
			if !tt.wantErr {
				if y != tt.year || m != tt.month || d != tt.day || s != tt.slug {
					t.Errorf("parseTicketPath(%q) = (%d,%d,%d,%q), want (%d,%d,%d,%q)", tt.path, y, m, d, s, tt.year, tt.month, tt.day, tt.slug)
				}
			}
		})
	}
}

// 🚩️TestExtractEffortFromArgs tests extracting reasoning effort from command line arguments.
func TestExtractEffortFromArgs(t *testing.T) {
	cases := []struct {
		args     []string
		expected string
	}{
		{[]string{"--effort", "high"}, "high"},
		{[]string{"--effort=low"}, "low"},
		{[]string{"--llm", "opus-4-7", "--effort", "max"}, "max"},
		{[]string{"--other", "val"}, ""},
	}
	for _, tc := range cases {
		cmd := &Command{}
		addEffortFlags(cmd)
		got, _ := extractEffortFromArgs(cmd, tc.args)
		if got != tc.expected {
			t.Errorf("extractEffortFromArgs(%v) = %q, want %q", tc.args, got, tc.expected)
		}
	}
}

// 🧪️TestTicketEffortLifecycle tests full lifecycle preservation of reasoning effort on tickets.
func TestTicketEffortLifecycle(t *testing.T) {
	tmpDir := t.TempDir()
	run := func(name string, args ...string) {
		execCommandWithTimeout(t, 30*time.Second, tmpDir, nil, name, args...)
	}
	run("git", "init")
	run("git", "config", "user.email", "test@test.com")
	run("git", "config", "user.name", "Test")
	run("git", "config", "commit.gpgsign", "false")
	run("git", "commit", "--allow-empty", "-m", "initial")
	oldRoot := workspace.RootDir
	workspace.RootDir = tmpDir
	defer func() { workspace.RootDir = oldRoot }()
	if err := os.MkdirAll(filepath.Join(tmpDir, ".🧬semio", "🦑️repo", "🎫️tickets"), 0755); err != nil {
		t.Fatal(err)
	}

	result := ToolTicketOpen("🎫️", "Test Effort Ticket", "Prompt with high effort", "opus-4-7", "high", "copilot-chat", "", true, "TEST-GOAL", "", false, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
	if !ok || ticket == nil {
		t.Fatal("expected valid ticket")
	}
	if ticket.GetEffort() != "high" {
		t.Errorf("ticket.GetEffort() = %q, want 'high'", ticket.GetEffort())
	}
	if ticket.GetLLM() != "opus-4-7" {
		t.Errorf("ticket.GetLLM() = %q, want 'opus-4-7'", ticket.GetLLM())
	}

	testFile := "work.txt"
	if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("done"), 0644); err != nil {
		t.Fatal(err)
	}
	run("git", "add", testFile)
	run("git", "commit", "-m", "work done")

	closeResult := ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Close effort ticket", []string{testFile}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose error: %s", closeResult.Error)
	}

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen with max effort", "gemini-3-7-pro", "max", "copilot-chat", "", "", "", "", true, providers.McpClientGeneric, "", "")
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen error: %s", reopenResult.Error)
	}
	reopenedTicket, ok := reopenResult.Data.(*model.Ticket)
	if !ok || reopenedTicket == nil {
		t.Fatal("expected valid reopened ticket")
	}
	if reopenedTicket.GetEffort() != "max" {
		t.Errorf("reopenedTicket.GetEffort() = %q, want 'max'", reopenedTicket.GetEffort())
	}
	if reopenedTicket.GetLLM() != "gemini-3-7-pro" {
		t.Errorf("reopenedTicket.GetLLM() = %q, want 'gemini-3-7-pro'", reopenedTicket.GetLLM())
	}
}
