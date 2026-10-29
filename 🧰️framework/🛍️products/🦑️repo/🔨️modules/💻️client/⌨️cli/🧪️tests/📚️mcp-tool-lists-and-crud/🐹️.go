// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// MCP tool list, tree, and CRUD tools.

// #endregion 🧲️Header

package client

import (
	"context"
	"fmt"
	"os"
	"testing"
	"time"
)

func TestToolTechnologyList(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolTechnologyList()
	if result.Error != "" {
		t.Errorf("ToolTechnologyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolTechnologyList returned nil data")
	}
	technologies, ok := result.Data.([]Technology)
	if !ok {
		t.Fatal("ToolTechnologyList data is not []Technology")
	}
	if len(technologies) == 0 {
		t.Error("ToolTechnologyList returned empty technologies")
	}
}

func TestExhaustiveToolTechnologyTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool technology tree test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolTechnologyTree(context.Background())
	if result.Error != "" {
		t.Errorf("ToolTechnologyTree returned error: %s", result.Error)
	}
}

func TestToolContributorList(t *testing.T) {
	setupToolTest(t)
	result := ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

func TestToolGoalList(t *testing.T) {
	setupToolTest(t)
	result := ToolGoalList()
	if result.Error != "" {
		t.Errorf("ToolGoalList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolGoalList returned nil data")
	}
}

func TestExhaustiveToolTicketList(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool ticket list test in short mode")
	}
	withExhaustiveRepo(t)
	result := ToolTicketList(nil, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolTicketList returned nil data")
	}
}

func TestToolDraftList(t *testing.T) {
	setupToolTest(t)
	result := ToolDraftList()
	if result.Error != "" {
		t.Errorf("ToolDraftList returned error: %s", result.Error)
	}
}

func TestToolFolderList(t *testing.T) {
	setupToolTest(t)
	result := ToolFolderList(".")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestExhaustiveToolFolderTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool folder tree test in short mode")
	}
	withMonorepoFixture(t)
	result := ToolFolderTree("repo")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

func TestToolFileList(t *testing.T) {
	setupToolTest(t)
	result := ToolFileList("repo/client")
	if result.Error != "" {
		t.Errorf("ToolFileList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFileList returned nil data")
	}
}

func TestExhaustiveToolFileTree(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow tool file tree test in short mode")
	}
	withMonorepoFixture(t)
	result := ToolFileTree("repo/client")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestToolSectionList(t *testing.T) {
	result := ToolSectionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
}

func TestToolSectionTree(t *testing.T) {
	withMonorepoFixture(t)
	result := ToolSectionTree("repo/client/main.go")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

func TestToolDefinitionList(t *testing.T) {
	result := ToolDefinitionList(withSectionFixture(t))
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

func TestToolPolicyList(t *testing.T) {
	setupToolTest(t)
	result := ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
}

func TestToolPolicyCheck(t *testing.T) {
	setupToolTest(t)
	result := ToolPolicyCheck("code", "repo/client")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestToolAnalyzeScope(t *testing.T) {
	setupToolTest(t)
	result := ToolAnalyze("repo/client", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestToolFolderCRUD(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	result := ToolFolderCreate("test-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderCreate returned error: %s", result.Error)
	}

	result = ToolFolderList(".")
	if result.Error != "" {
		t.Fatalf("ToolFolderList returned error: %s", result.Error)
	}

	result = ToolFolderMove("test-folder", "renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderMove returned error: %s", result.Error)
	}

	result = ToolFolderDelete("renamed-folder")
	if result.Error != "" {
		t.Fatalf("ToolFolderDelete returned error: %s", result.Error)
	}
}

func TestToolFileCRUD(t *testing.T) {
	setupToolTest(t)
	tmpDir := t.TempDir()
	oldRoot := rootDir
	rootDir = tmpDir
	defer func() { rootDir = oldRoot }()

	result := ToolFileCreate("test.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileCreate returned error: %s", result.Error)
	}

	result = ToolFileMove("test.txt", "renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileMove returned error: %s", result.Error)
	}

	result = ToolFileDelete("renamed.txt")
	if result.Error != "" {
		t.Fatalf("ToolFileDelete returned error: %s", result.Error)
	}
}

func TestExhaustiveToolTicketLifecycle(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping slow ticket lifecycle test in short mode")
	}
	withExhaustiveRepo(t)
	title := fmt.Sprintf("Test Lifecycle Ticket %d", time.Now().UnixNano())

	result := ToolTicketOpen("🎫️", title, "Test prompt", "sonnet-4-5", "", "windsurf-chat", "", true, "AI-OPTIMIZED-REPO", "", true, "", McpClientGeneric, "", "")
	if result.Error != "" {
		t.Fatalf("ToolTicketOpen returned error: %s", result.Error)
	}
	ticket, ok := result.Data.(*Ticket)
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

	reopenResult := ToolTicketReopen(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Reopen prompt", "sonnet-4-5", "", "windsurf-chat", "", "", "", "", true, McpClientGeneric, "", "")
	if reopenResult.Error != "" {
		t.Fatalf("ToolTicketReopen returned error: %s", reopenResult.Error)
	}

	ToolTicketClose(ticket.Year, ticket.Month, ticket.Day, ticket.Slug, "Final close", []string{"repo/client/package.json"}, "", true)
	ticketPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
	os.RemoveAll(ticketPath)
}
