//go:build exhaustive

// 🧮️ Exhaustive tests of the cli domain: statute, checkpoint and tool commands.

package cli

import (
	fmt "fmt"
	os "os"
	strings "strings"
	testing "testing"
	time "time"

	model "github.com/usalu/semio/repo/model"
	providers "github.com/usalu/semio/repo/providers"
	ticketspkg "github.com/usalu/semio/repo/tickets"
	workspace "github.com/usalu/semio/repo/workspace"
)

func TestExhaustiveStatuteCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow statute commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)

	t.Run("statute list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--json")
		if err != nil {
			t.Fatalf("statute list failed: %v", err)
		}
		if !strings.Contains(output, "statute") {
			t.Errorf("expected statute JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) < 5 {
			t.Errorf("expected multiple statutes, got %d lines", len(lines))
		}
	})

	t.Run("statute tree returns results", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--only-statute", "--text")
		if err != nil {
			t.Fatalf("statute tree failed: %v", err)
		}
		if !strings.Contains(output, "header") {
			t.Errorf("expected statute tree categories in output")
		}
	})

	t.Run("statute list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("statute tree markdown", func(t *testing.T) {
		output, err := executeTreeCommand("search", "--only-statute", "--md")
		if err != nil {
			t.Fatalf("statute tree md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})
}

func TestExhaustiveCheckpointCommands(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow checkpoint commands test in short mode")
	}
	cwd, _ := os.Getwd()
	repoRoot := findTestRepoRoot(cwd)
	workspace.SetRootDir(repoRoot)

	t.Run("checkpoint list returns results", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list failed: %v", err)
		}
		if !strings.Contains(output, "checkpoint") {
			t.Errorf("expected checkpoint JSON key in output")
		}
		lines := strings.Split(strings.TrimSpace(output), "\n")
		if len(lines) == 0 {
			t.Error("expected at least one checkpoint")
		}
	})

	t.Run("checkpoint list --query filters", func(t *testing.T) {
		allOutput, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "200")
		if err != nil {
			t.Fatalf("checkpoint list failed: %v", err)
		}
		allLines := strings.Split(strings.TrimSpace(allOutput), "\n")

		filteredOutput, err := executeTreeCommand("list", "--only-checkpoint", "--json", "--limit", "200", "--query", "merge")
		if err != nil {
			t.Fatalf("checkpoint list --query failed: %v", err)
		}
		filteredLines := strings.Split(strings.TrimSpace(filteredOutput), "\n")
		if len(filteredLines) >= len(allLines) && len(allLines) > 1 {
			t.Errorf("expected --query to reduce results: all=%d filtered=%d", len(allLines), len(filteredLines))
		}
	})

	t.Run("checkpoint list markdown", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--md", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list md failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty markdown output")
		}
	})

	t.Run("checkpoint list text", func(t *testing.T) {
		output, err := executeTreeCommand("list", "--only-checkpoint", "--text", "--limit", "5")
		if err != nil {
			t.Fatalf("checkpoint list text failed: %v", err)
		}
		if output == "" {
			t.Error("expected non-empty text output")
		}
	})
}

func TestExhaustiveToolTechnologyTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool technology tree test in short mode")
	}
	setupToolTest(t)
	result := ToolTechnologyTree()
	if result.Error != "" {
		t.Errorf("ToolTechnologyTree returned error: %s", result.Error)
	}
}

func TestExhaustiveToolTicketList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool ticket list test in short mode")
	}
	setupToolTest(t)
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolTicketList returned nil data")
	}
}

func TestExhaustiveToolFolderTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool folder tree test in short mode")
	}
	setupToolTest(t)
	result := ToolFolderTree("repo")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

func TestExhaustiveToolFileTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool file tree test in short mode")
	}
	setupToolTest(t)
	result := ToolFileTree("repo/client")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestExhaustiveToolTicketLifecycle(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket lifecycle test in short mode")
	}
	setupToolTest(t)
	title := fmt.Sprintf("Test Lifecycle Ticket %d", time.Now().UnixNano())

	result := ToolTicketOpen("🎫️", title, "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", providers.McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*model.Ticket)
	if !ok || ticket == nil {
		t.Fatal("ToolTicketOpen returned nil ticket")
	}

	readResult := ToolTicketRead(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	if readResult.Error != "" {
		t.Fatalf("ToolTicketRead returned error: %s", readResult.Error)
	}

	closeResult := ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Test summary", []string{"repo/client/package.json"}, "", true)
	if closeResult.Error != "" {
		t.Fatalf("ToolTicketClose returned error: %s", closeResult.Error)
	}

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen prompt", "sonnet-4-5", "", "windsurf-chat", "", "", "", "", true, providers.McpClientGeneric, "", "")
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen returned error: %s", reopenResult.Error)
	}

	ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Final close", []string{"repo/client/package.json"}, "", true)
	ticketPath := ticketspkg.GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	os.RemoveAll(ticketPath)
}
